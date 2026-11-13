#!/usr/bin/env python3
"""Emit the ICO 32-bit success obligations for `src/model_vectors.rs`.

Each ICO `depth == 32` success (in either mode: the 1.0 `IcoEntryDib` and the
1.1 `IcoEntryDibSlack`) is proved in stages, because the model's `ma_output`
(a `Seq::new` closure over the whole-image XOR-alpha rule) cannot be closed by
`by (compute)`:

  1. `mh_v()`               : `mh(b_v(), <mode>) == Ok(rec_v())` by `compute`.
                              This *validates* the concrete record: a wrong field
                              makes the obligation false.
  2. `raw_v()`              : `mr_raw_out(b_v(), rec_v(), ps) == Ok((end, xor_v()))`
                              by `compute`, `xor_v()` written in the model's
                              `conv_word` terms so `compute` can see through
                              `u32_trailing_zeros` for the BITFIELDS shapes.
  3. `xor_bytes_v()`        : `xor_v() == <the plane's RGBA bytes>`, by
                              `channel_lit` per mask (as in the term form).
  4. `ma_v()`               : `ma_output(b_v(), rec_v(), end, xor_v(), explicit_alpha)
                              == exp_v()`: `lemma_ma_explicit_alpha_keep` when the
                              XOR plane has a nonzero alpha byte, else a per-index
                              `ma_byte` evaluation (the AND fallback).
  5. `agree_v()`            : the fixture statement, composing the above through
                              `mp`'s ladder.

The record's `mode` field and every use of `Mode::<...>` in the emitted blocks are
the fixture's own mode, so the two ICO modes share this generator; only the final
cursor test differs between them, and that is inside `mp`.

Usage: tools/gen_ico32.py OUT.rs
"""

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
VECTORS = os.path.join(ROOT, "spec-1.1-frozen", "VECTORS.json")

# R19: the two ICO modes share every R1-R18 rule; only R12's final cursor test
# is governed by `exact_tail`, which lives in `mp`.
ICO_MODES = ("IcoEntryDib", "IcoEntryDibSlack")


def ident(name):
    return "".join(c if c.isalnum() else "_" for c in name)


def hex32(x):
    return "0x%08xu32" % x


def bytes_seq(data, indent=4):
    parts = ["0x%02xu8" % b for b in data]
    lines = []
    for i in range(0, len(parts), 12):
        lines.append(" " * (indent + 4) + ", ".join(parts[i:i + 12]) + ",")
    return "seq![\n" + "\n".join(lines) + "\n" + " " * indent + "]"


def use_ico32(v):
    if v["mode"] not in ICO_MODES or "error" in v["expected"]:
        return False
    raw = bytes.fromhex(v["input_hex"]) * v.get("input_repeat", 1)
    if len(raw) > 40000 or len(raw) < 20:
        return False
    H = int.from_bytes(raw[0:4], "little")
    return H >= 40 and raw[14] == 32


def fields(v):
    raw = bytes.fromhex(v["input_hex"]) * v.get("input_repeat", 1)
    H = int.from_bytes(raw[0:4], "little")
    w = int.from_bytes(raw[4:8], "little", signed=True)
    sh = int.from_bytes(raw[8:12], "little", signed=True)
    c = int.from_bytes(raw[16:20], "little")
    dec = int.from_bytes(raw[20:24], "little")
    used = int.from_bytes(raw[32:36], "little")
    h = sh // 2
    if c == 3 and H in (52, 56, 108, 124):
        masks = [int.from_bytes(raw[40 + 4 * j:44 + 4 * j], "little") for j in range(4)]
        ps = H + used * 4
        if H not in (56, 108, 124):
            masks = masks[:3] + [0]
    elif c == 3:
        masks = [int.from_bytes(raw[H + 4 * j:H + 4 * j + 4], "little") for j in range(3)] + [0]
        ps = H + 12 + used * 4
    else:
        masks = [0, 0, 0, 0]
        ps = H
    return dict(H=H, w=w, sh=sh, h=h, c=c, dec=dec, used=used, masks=masks, ps=ps)


def xor_terms(v, f):
    """The XOR plane in the model's RGBA byte order, per pixel, as `conv_word`
    expressions.  BI_RGB32 ICO reads the word as LE32; BITFIELDS32 uses the
    record's masks.  A nonzero A mask yields `conv_word`'s alpha; a zero A mask
    (BI_RGB32) yields the stored A byte (which `conv_word` takes from the word)."""
    raw = bytes.fromhex(v["input_hex"]) * v.get("input_repeat", 1)
    w, h, d, ps = f["w"], f["h"], 32, f["ps"]
    stride = (w * d + 31) // 32 * 4
    nbytes = 4
    terms = []
    words = []
    if f["c"] == 3:
        for q in range(h):
            r = h - 1 - q  # ICO is bottom-up; XOR rows are source order
            for x in range(w):
                off = ps + r * stride + x * nbytes
                word = int.from_bytes(raw[off:off + nbytes], "little")
                words.append(word)
                cs = ["crate::model::channel_of(0x%08xu32, 0x%08xu32)" % (word, f["masks"][j])
                      for j in range(4)]
                terms.append("seq![" + ", ".join(cs) + "]")
        return terms, words
    # BI_RGB32: the model's `rgb_byte` reads, which `compute` unfolds directly.
    for q in range(h):
        r = h - 1 - q
        for x in range(w):
            off = ps + r * stride + x * nbytes
            word = int.from_bytes(raw[off:off + nbytes], "little")
            words.append(word)
            a = (word >> 24) & 0xFF
            terms.append("seq![crate::model::rgb_byte(0x%08x, 0, %d), "
                         "crate::model::rgb_byte(0x%08x, 1, %d), "
                         "crate::model::rgb_byte(0x%08x, 2, %d), %du8]"
                         % (word, a, word, a, word, a, a))
    return terms, words


def xor_bytes(v, f, words):
    """The model's `conv_word` output bytes for each pixel."""
    out = []
    nbytes = 4
    for word in words:
        if f["c"] == 3:
            vals = []
            for j in range(4):
                m = f["masks"][j]
                if m == 0:
                    vals.append(255)
                else:
                    k = (m & -m).bit_length() - 1
                    vals.append(scale((word & m) >> k, m))
            out += vals
        else:
            out += [(word >> 16) & 0xFF, (word >> 8) & 0xFF, word & 0xFF, (word >> 24) & 0xFF]
    return out


def scale(comp, mask):
    k = (mask & -mask).bit_length() - 1
    mx = mask >> k
    return (255 * comp + mx // 2) // mx


def explicit_alpha(v, f, words):
    return f["c"] == 3 and f["masks"][3] != 0 or f["c"] == 0


def gen():
    V = json.load(open(VECTORS))
    blocks = []
    for v in V:
        if not use_ico32(v):
            continue
        blocks.append(one(v))
    return "".join(blocks)


def one(v):
    name = v["name"]
    i = ident(name)
    raw = bytes.fromhex(v["input_hex"]) * v.get("input_repeat", 1)
    f = fields(v)
    terms, words = xor_terms(v, f)
    xbytes = xor_bytes(v, f, words)
    exp = v["expected"]
    want = []
    for r in exp["rgba_runs"]:
        want += [int(r["hex"][j:j + 2], 16) for j in (0, 2, 4, 6)] * r["count"]
    assert len(want) == f["w"] * f["h"] * 4, name
    n = f["w"] * f["h"]
    a_bytes = f["h"] * (((f["w"] + 31) // 32) * 4)
    end = f["ps"] + f["w"] * f["h"] * 4
    masks = f["masks"]
    exp_alpha = explicit_alpha(v, f, words)
    alpha_present = any(xbytes[4 * j + 3] != 0 for j in range(n)) if exp_alpha else False
    keep = exp_alpha and alpha_present
    rec_fields = ("mode: Mode::%s, s: 0, hsize: %d, width: %d, height: %d, "
                  "stored_height: %d, planes: 1, depth: 32, compression: %d, "
                  "declared_image: %d, color_used: %d, count: 0, table_count: 0, "
                  "palette_stride: %d, palette_start: %d, palette_end: %d, pixel_start: %d, "
                  "mask_r: %s, mask_g: %s, mask_b: %s, mask_a: %s, has_masks: %s, "
                  "core: false, is_bmp: false, top_down: false, n: %d, stride: %d")
    pal_stride = 3 if f["H"] == 12 else 4
    rec = rec_fields % (
        v["mode"], f["H"], f["w"], f["h"], f["sh"], f["c"], f["dec"], f["used"],
        pal_stride, f["ps"], f["ps"], f["ps"],
        "0x%08x" % masks[0], "0x%08x" % masks[1], "0x%08x" % masks[2], "0x%08x" % masks[3],
        "true" if f["c"] == 3 else "false", n, ((f["w"] * 32 + 31) // 32) * 4,
    )
    b = []
    b.append("// ---- %s (%d bytes, %s, depth 32) [staged] ----\n" % (name, len(raw), v["mode"]))
    b.append("pub open spec fn b_%s() -> Seq<u8> {\n%s\n}\n\n" % (i, bytes_seq(raw)))
    b.append("pub open spec fn rec_%s() -> crate::model::MHRecord {\n"
             "    crate::model::MHRecord {\n        %s,\n    }\n}\n\n" % (i, rec))
    b.append("pub open spec fn xor_%s() -> Seq<u8> {\n%s\n}\n\n"
             % (i, "\n        + ".join(t for t in terms)))
    b.append("pub open spec fn want_%s() -> Seq<u8> {\n%s\n}\n\n" % (i, bytes_seq(want)))
    # stage 1
    b.append("/// Stage 1: the concrete record is the model's.\n")
    b.append("proof fn mh_%s()\n    ensures\n        crate::model::mh(b_%s(), Mode::%s) == Ok(rec_%s()),\n{\n"
             % (i, i, v["mode"], i))
    b.append("    reveal(crate::model::mh);\n    reveal(crate::model::r1_prefix);\n"
             "    reveal(crate::model::r2r7);\n    reveal(crate::model::r2r5);\n"
             "    reveal(crate::model::r6r7);\n    reveal(crate::model::r6r7_result);\n")
    b.append("    assert(crate::model::mh(b_%s(), Mode::%s) == Ok(rec_%s())) by (compute);\n}\n\n"
             % (i, v["mode"], i))
    # stage 2
    b.append("/// Stage 2: the raw phase produces exactly `xor_%s()`.\n" % i)
    b.append("proof fn raw_%s()\n    ensures\n        crate::model::mr_raw_out(b_%s(), rec_%s(), %d) == Ok((%d, xor_%s())),\n{\n"
             % (i, i, i, f["ps"], end, i))
    b.append("    reveal(crate::model::mr_raw_out);\n    reveal(crate::model::mr_raw);\n"
             "    reveal(crate::model::raw_full);\n    reveal(crate::model::raw_writes);\n"
             "    reveal(crate::model::conv);\n    reveal(crate::model::conv_word);\n"
             "    reveal(crate::model::bitfield_seq);\n    reveal(crate::model::rgb_seq);\n"
             "    reveal(crate::model::zero_bytes);\n    reveal(crate::model::bytes_replay);\n")
    b.append("    reveal(crate::model::channel_of);\n    reveal(crate::model::scale_of);\n")
    b.append("    reveal(crate::model::mh);\n    reveal(crate::model::r2r7);\n"
             "    reveal(crate::model::r6r7);\n    reveal(crate::model::r2r5);\n"
             "    reveal(crate::model::r6r7_result);\n    reveal(crate::model::r1_prefix);\n")
    b.append("    assert(crate::model::mh(b_%s(), Mode::%s) == Ok(rec_%s())) by (compute);\n"
             % (i, v["mode"], i))
    b.append("    assert(crate::model::mr_raw_out(b_%s(), rec_%s(), %d) == Ok((%d, xor_%s()))) by (compute);\n}\n\n"
             % (i, i, f["ps"], end, i))
    # stage 3: xor_ == xbytes
    b.append("/// Stage 3: `xor_%s()` is the plane's model output bytes.\n" % i)
    b.append("proof fn xor_bytes_%s()\n    ensures\n        xor_%s() == %s,\n{\n" % (i, i, bytes_seq(xbytes)))
    b.append("    reveal(crate::model::conv_word);\n    reveal(crate::model::bitfield_seq);\n"
             "    reveal(crate::model::rgb_seq);\n    reveal(crate::model::rgb_byte);\n")
    b.append("    reveal(crate::model::channel_of);\n    reveal(crate::model::scale_of);\n")
    for pi, word in enumerate(words):
        if f["c"] == 3:
            for j in range(4):
                m = f["masks"][j]
                if m == 0:
                    # `channel_of` with a zero mask is the literal 255.
                    continue
                k = (m & -m).bit_length() - 1
                mx = m >> k
                comp = (word & m) >> k
                val = scale(comp, m)
                b.append(plain_chan(word, m, k, mx, comp, val))
    # With every component's literal value established, `compute` closes the
    # whole sequence against the byte list.
    b.append("    assert(xor_%s() == %s);\n}\n\n" % (i, bytes_seq(xbytes)))
    # stage 4: ma
    b.append("/// Stage 4: the AND/alpha policy maps the plane to the fixture's bytes.\n")
    b.append("proof fn ma_%s()\n    ensures\n        crate::model::ma_output(b_%s(), rec_%s(), %d, xor_%s(), %s) == want_%s(),\n{\n"
             % (i, i, i, end, i, "true" if exp_alpha else "false", i))
    b.append("    reveal(crate::model::ma_output);\n    reveal(crate::model::ma_byte);\n"
             "    reveal(crate::model::and_bit);\n    reveal(crate::model::and_stride);\n"
             "    reveal(crate::model::stride_of);\n    reveal(crate::model::xor_alpha_present);\n")
    if keep:
        b.append("    xor_bytes_%s();\n" % i)
        b.append("    assert(xor_%s() == %s);\n" % (i, bytes_seq(xbytes)))
        pj = next(j for j in range(n) if xbytes[4 * j + 3] != 0)
        b.append("    assert(xor_%s()[%d] == %s);\n" % (i, 4 * pj + 3, "0x%02xu8" % xbytes[4 * pj + 3]))
        b.append("    assert(crate::model::xor_alpha_present(xor_%s(), %d)) by {\n" % (i, n))
        b.append("        assert(xor_%s()[4 * (%dint) + 3] == %s);\n"
                 % (i, pj, "0x%02xu8" % xbytes[4 * pj + 3]))
        b.append("    };\n")
        b.append("    crate::model::lemma_ma_explicit_alpha_keep(b_%s(), rec_%s(), %d, xor_%s());\n"
                 % (i, i, end, i))
        b.append("}\n\n")
    elif exp_alpha:
        # The alpha-presence quantifier is discharged once from the concrete
        # XOR plane. Per-byte computation then uses the quantifier-free fallback
        # definition, which is proved equal to MA's original ma_output.
        b.append("    xor_bytes_%s();\n" % i)
        b.append("    assert(!crate::model::xor_alpha_present(xor_%s(), %d)) by {\n" % (i, n))
        b.append("        if crate::model::xor_alpha_present(xor_%s(), %d) {\n" % (i, n))
        b.append("            let j = choose|j: int| 0 <= j < %d && #[trigger] xor_%s()[4 * j + 3] != 0u8;\n" % (n, i))
        b.append("            assert(xor_%s()[4 * j + 3] == 0u8);\n        }\n    };\n" % i)
        b.append("    crate::model::lemma_ma_explicit_alpha_zero_fallback(b_%s(), rec_%s(), %d, xor_%s());\n" % (i, i, end, i))
        b.append("    let f = Seq::new(xor_%s().len(), |k: int| crate::model::ma_fallback_byte(b_%s(), rec_%s(), %d, xor_%s(), k));\n" % (i, i, i, end, i))
        b.append("    assert(f =~= want_%s()) by {\n" % i)
        b.append("        assert forall|k: int| 0 <= k < %d implies #[trigger] f[k] == want_%s()[k] by {\n" % (len(want), i))
        for k, byte in enumerate(want):
            b.append("            if k == %d {\n" % k)
            b.append("                assert(crate::model::ma_fallback_byte(b_%s(), rec_%s(), %d, %s, %d) == 0x%02xu8) by (compute);\n" % (i, i, end, bytes_seq(xbytes), k, byte))
            b.append("                assert(crate::model::ma_fallback_byte(b_%s(), rec_%s(), %d, xor_%s(), %d) == 0x%02xu8);\n" % (i, i, end, i, k, byte))
            b.append("            }\n")
        b.append("        };\n    }\n}\n\n")
    else:
        # Concrete per-index lemmas: for each byte 0..len(want)-1, a lemma whose
        # index is a literal, so `ma_byte` reduces without a symbolic `k`.
        for k in range(len(want)):
            b.append("proof fn mb_%s_%d()\n    ensures\n"
                     "        crate::model::ma_byte(b_%s(), rec_%s(), %d, xor_%s(), %s, %d) == %s,\n{\n"
                     % (i, k, i, i, end, i, "true" if exp_alpha else "false", k,
                        "0x%02xu8" % want[k]))
            b.append("    reveal(crate::model::ma_byte);\n    reveal(crate::model::and_bit);\n"
                     "    reveal(crate::model::and_stride);\n    reveal(crate::model::stride_of);\n"
                     "    reveal(crate::model::xor_alpha_present);\n")
            b.append("    xor_bytes_%s();\n" % i)
            b.append("    assert(xor_%s() == %s);\n" % (i, bytes_seq(xbytes)))
            if exp_alpha:
                b.append("    assert(!crate::model::xor_alpha_present(xor_%s(), %d)) by {\n" % (i, n))
                b.append("        if crate::model::xor_alpha_present(xor_%s(), %d) {\n" % (i, n))
                b.append("            let ii = choose|ii: int| 0 <= ii < %d && #[trigger] xor_%s()[4 * ii + 3] != 0u8;\n" % (n, i))
                b.append("            assert(xor_%s()[4 * ii + 3] == 0u8);\n        }\n    };\n" % i)
            if k % 4 == 3:
                pi = k // 4
                x = pi % f["w"]
                yy = pi // f["w"]
                r = f["h"] - 1 - yy
                bit = (raw[end + r * (((f["w"] + 31) // 32) * 4) + x // 8] & (0x80 >> (x % 8))) != 0
                ab = raw[end + r * (((f["w"] + 31) // 32) * 4) + x // 8]
                b.append("    assert(b_%s()[%d] == 0x%02xu8) by (compute);\n"
                         % (i, end + r * (((f["w"] + 31) // 32) * 4) + x // 8, ab))
                if bit:
                    b.append("    assert(crate::model::and_bit(b_%s(), %d, %d, %d, %d)) by (compute);\n"
                             % (i, end, f["w"], x, yy))
                else:
                    b.append("    assert(crate::model::and_bit(b_%s(), %d, %d, %d, %d) == false) by (compute);\n"
                             % (i, end, f["w"], x, yy))
            else:
                b.append("    assert(xor_%s()[%d] == %s);\n" % (i, k, "0x%02xu8" % want[k]))
            b.append("    assert(crate::model::ma_byte(b_%s(), rec_%s(), %d, %s, %s, %d) "
                     "== %s);\n}\n\n"
                     % (i, i, end, bytes_seq(xbytes), "true" if exp_alpha else "false", k,
                        "0x%02xu8" % want[k]))
        b.append("    assert(crate::model::ma_output(b_%s(), rec_%s(), %d, xor_%s(), %s) =~= %s) by {\n"
                 % (i, i, end, i, "true" if exp_alpha else "false", bytes_seq(want)))
        b.append("        assert forall|k: int| 0 <= k < %d implies #[trigger] crate::model::ma_output("
                 "b_%s(), rec_%s(), %d, xor_%s(), %s)[k] == %s[k] by {\n"
                 % (len(want), i, i, end, i, "true" if exp_alpha else "false", bytes_seq(want)))
        for k in range(len(want)):
            b.append("            if k == %d { mb_%s_%d(); }\n" % (k, i, k))
        b.append("        };\n    }\n}\n\n")
    # stage 5: agree
    b.append("/// Stage 5: the fixture's result, composing the stages through `mp`.\n")
    b.append("#[allow(unused)]\n")
    b.append("pub proof fn agree_%s()\n    ensures\n        mp(b_%s(), Mode::%s) == Ok((%d, %d, want_%s())),\n{\n"
             % (i, i, v["mode"], f["w"], f["h"], i))
    b.append("    mh_%s();\n    raw_%s();\n    xor_bytes_%s();\n    ma_%s();\n" % (i, i, i, i))
    b.append("    assert(crate::model::image_size_ok(rec_%s().declared_image, %d - rec_%s().pixel_start, "
             "crate::model::and_bytes(rec_%s().width, rec_%s().height), true)) by (compute);\n"
             % (i, end, i, i, i))
    b.append("    assert(crate::model::and_bytes(rec_%s().width, rec_%s().height) == %d) by (compute);\n"
             % (i, i, a_bytes))
    b.append("    crate::model::lemma_mp_mask_phase_ico_ok(b_%s(), rec_%s(), %d, xor_%s(), want_%s());\n"
             % (i, i, end, i, i))
    b.append("    reveal(crate::model::mp);\n    reveal(crate::model::mp_raw);\n"
             "    reveal(crate::model::mp_decoded);\n")
    b.append("    assert(crate::model::mp_raw(b_%s(), rec_%s()) == Ok((want_%s(), %d)));\n"
             % (i, i, i, end + a_bytes))
    b.append("    assert(crate::model::mp_decoded(rec_%s(), crate::model::mp_raw(b_%s(), rec_%s())) "
             "== Ok((%d, %d, want_%s())));\n" % (i, i, i, f["w"], f["h"], i))
    b.append("}\n\n")
    return "".join(b)


def plain_chan(word, m, k, mx, comp, val):
    return ("    assert(%s != 0u32) by (bit_vector);\n"
            "    assert((%s >> %du32) & 1u32 == 1u32) by (bit_vector);\n"
            "    assert forall|j: u32| 0 <= j < %du32 implies #[trigger] (%s >> j) & 1u32 == 0u32 by {\n"
            "        assert((%s >> j) & 1u32 == 0u32) by (bit_vector) requires 0 <= j < %du32;\n"
            "    };\n"
            "    assert(%s >> %du32 == %s) by (bit_vector);\n"
            "    assert((%s & %s) >> %du32 == %su32) by (bit_vector);\n"
            "    assert(crate::model::scale_of(%dint, %dint) == %dint) by (nonlinear_arith);\n"
            "    assert(crate::model::channel_span(%s) == %s) by (compute);\n"
            "    channel_lit(%s, %s, %du32, %s, %su32, %du8);\n"
            % (hex32(m), hex32(m), k, k, hex32(m), hex32(m), k,
               hex32(m), k, hex32(mx), hex32(word), hex32(m), k, comp, comp, mx, val,
               hex32(mx), hex32(mx), hex32(word), hex32(m), k, hex32(mx), comp, val))


if __name__ == "__main__":
    sys.stdout.write(gen())
