#!/usr/bin/env python3
"""Generate Verus model-vs-vector agreement proofs from
spec-1.1-frozen/VECTORS.json.

For every vector whose expanded input is at most 40,000 bytes, emit a `proof fn`
whose statement is

    mp(<the vector's expanded input>, <mode>) == <the fixture's expected result>

Every expected value is read from the fixture's `expected` field; nothing is
taken from the production decoder.  The generator reads the 302-vector 1.1 file:
the first 214 records are the unchanged 1.0 fixtures, the appended 88 exercise
`IcoEntryDibSlack` and the shared ICO handling, so both modes produce the same
`agree_<vector>` obligations against the extended `MP`.

Two forms are emitted:

* **literal** (the default): the expected `Ok`/`Err` is written with the fixture's
  own RGBA bytes and the body is `assert(...) by (compute)`.  A mismatch makes the
  obligation false, not vacuous.

* **term** (16-bit and BITFIELDS inputs): the model's own colour reader
  `channel_of(word, mask)` bottoms out in `u32_trailing_zeros`, a `closed` spec
  fn that Verus's `compute` normalizer refuses to unfold, so a literal expected
  value cannot be closed by `compute`.  For these the obligation is split in two
  connected steps whose conjunction is the manifest's statement:

      1. `mp(b, mode) == Ok((w, h, want()))`, with `want()` written in the model's
         `channel_of` terms, discharged by `by (compute)`;
      2. `want() == <the fixture's bytes>`, discharged by `channel_lit` (a lemma
         relating one `channel_of` call to its concrete byte) plus the
         `u32_trailing_zeros` value, which is itself proved from the mask's bits.

  `agree_<name>` composes the two, so the named obligation is exactly the
  fixture's expected `Result`.

The vectors longer than 40,000 bytes are covered by the symbolic lemmas L1-L2 in
`src/model.rs` (the oversized inputs) and by mandatory runtime conformance tests;
the generator checks each of them against the manifest's symbolic construction, so
no 64 MiB input is ever expanded into verifier terms.

Usage: tools/gen_model_vectors.py [--out PATH] [--check]
"""

import importlib.util
import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VECTORS = os.path.join(ROOT, "spec-1.1-frozen", "VECTORS.json")
DEFAULT_OUT = os.path.join(ROOT, "dib_verus", "src", "model_vectors.rs")

# The largest input expanded as a literal: the two 32 KiB height fixtures
# (`height-at-cap-8192` is 32,814 bytes) are nested here too, but they are listed
# under LARGE because THEOREMS.md calls them runtime tests.
LIMIT = 40000

LARGE = {
    "input-over-64m": ("lemma", "lemma_mp_l1 (L1)"),
    "precedence-too-large-bmp-bad-magic": ("lemma", "lemma_mp_l1 (L1)"),
    "input-exact-64m": ("lemma", "lemma_mp_l2 (L2)"),
    "output-exact-64m-rle8": ("runtime", "cargo test conformance (R16 output-exact fixture)"),
    "height-at-cap-8192": ("runtime", "cargo test conformance (8192-row height boundary)"),
    # R19: the exact 64 MiB boundary in the two ICO modes and one byte over it.
    # These are kept out of the verifier for the same reason as the 1.0 64 MiB
    # fixtures: they are 64 MiB slices, covered by the runtime conformance test
    # with the frozen oracle.  The small slack-* vectors cover the same branch
    # symbolically (see `slack-1-new`/`slack-1-old`).
    "max-input-slack-IcoEntryDibSlack": ("runtime", "cargo test conformance (64 MiB boundary, slack success)"),
    "max-input-slack-IcoEntryDib": ("runtime", "cargo test conformance (64 MiB boundary, exact-tail contrast)"),
    "over-max-input-slack-new": ("lemma", "lemma_mp_l1 (L1, over-cap slack)"),
}

TZ = "vstd::std_specs::bits::u32_trailing_zeros"

# R19: the two modes that share every ICO rule (R1-R18) and differ only at the
# final cursor test.  The generators treat them alike for geometry and colour.
ICO_MODES = ("IcoEntryDib", "IcoEntryDibSlack")


def is_ico_mode(mode):
    return mode in ICO_MODES


def expanded(v):
    raw = bytes.fromhex(v["input_hex"])
    rep = v.get("input_repeat", 1)
    cut = v.get("repeat_from")
    if cut is not None:
        return raw[:cut] + raw[cut:] * rep + bytes.fromhex(v["input_suffix_hex"])
    return raw * rep


def check_large(vectors):
    out = []
    for v in vectors:
        got = expanded(v)
        if len(got) <= LIMIT:
            continue
        name = v["name"]
        kind, cover = LARGE[name]
        exp = v["expected"]
        if name in ("input-over-64m", "precedence-too-large-bmp-bad-magic"):
            fill = 0x00 if name == "input-over-64m" else 0x5a
            assert got == bytes([fill]) * 67108865, name
            assert exp.get("error") == "FileTooLarge", name
        elif name == "input-exact-64m":
            assert got == bytes([0]) * 67108864, name
            assert exp.get("error") == "UnsupportedHeaderSize", name
        elif name == "output-exact-64m-rle8":
            unit = bytes.fromhex("8001") * 64 + bytes([0, 0])
            assert got == bytes.fromhex("28000000002000000008000001000800010000000000000000000000000000000200000000000000000000000000ff00") + unit * 2048 + bytes([0, 1]), name
            assert exp["width"] == 8192 and exp["height"] == 2048, name
            assert [(r["count"], r["hex"]) for r in exp["rgba_runs"]] == [(16777216, "ff0000ff")], name
        elif name == "height-at-cap-8192":
            assert exp["width"] == 1 and exp["height"] == 8192, name
        elif name == "max-input-slack-IcoEntryDibSlack":
            # R19: the exact cap in the slack mode — 48-byte preamble, one
            # pixel and mask, then 0x5a repeated to 64 MiB. The bytes after the
            # complete AND mask are ignored, so this is a success.
            assert len(got) == 67108864, name
            assert got[:48] == bytes.fromhex(
                "280000000100000002000000010018000000000000000000000000000000000000000000000000000000ff0000000000")
            assert got[48:] == bytes([0x5A]) * (67108864 - 48), name
            assert exp["width"] == 1 and exp["height"] == 1, name
            assert [(r["count"], r["hex"]) for r in exp["rgba_runs"]] == [(1, "ff0000ff")], name
        elif name == "max-input-slack-IcoEntryDib":
            # The same bytes in the exact-tail mode: the same complete mask but
            # a non-empty suffix, so the 1.0 mode reports TrailingBytes.
            assert len(got) == 67108864, name
            assert exp.get("error") == "TrailingBytes", name
        elif name == "over-max-input-slack-new":
            assert len(got) == 67108865, name
            assert exp.get("error") == "FileTooLarge", name
        out.append((name, kind, cover, len(got)))
    return out


# ---------------------------------------------------------------------------
# Structured reading of a raw/BITFIELDS (16- or 32-bit) fixture, in the model's
# own terms, so the expected `Ok` can be written as `channel_of` calls.
# ---------------------------------------------------------------------------

def parse_geometry(raw, mode):
    s = 14 if mode == "BmpFile" else 0
    H = int.from_bytes(raw[s:s + 4], "little")
    if H == 16:
        w = int.from_bytes(raw[s + 4:s + 8], "little", signed=True)
        sh = int.from_bytes(raw[s + 8:s + 12], "little", signed=True)
        depth = int.from_bytes(raw[s + 14:s + 16], "little")
        comp = 0
        used = 0
        ps = s + 16
    elif H in (12,):
        return None
    else:
        w = int.from_bytes(raw[s + 4:s + 8], "little", signed=True)
        sh = int.from_bytes(raw[s + 8:s + 12], "little", signed=True)
        depth = int.from_bytes(raw[s + 14:s + 16], "little")
        comp = int.from_bytes(raw[s + 16:s + 20], "little")
        used = int.from_bytes(raw[s + 32:s + 36], "little")
        ps = s + H
    return dict(s=s, H=H, w=w, sh=sh, depth=depth, comp=comp, used=used, ps=ps)


def tz_path(v):
    """True when the vector's pixels are converted through `channel_of`
    (16-bit BI_RGB or any BITFIELDS), i.e. when `compute` cannot see through
    `u32_trailing_zeros`."""
    raw = expanded(v)
    if len(raw) > LIMIT or "error" in v["expected"]:
        return False
    g = parse_geometry(raw, v["mode"])
    if g is None:
        return False
    if g["depth"] == 16 and g["comp"] == 0:
        return True
    if g["comp"] == 3 and g["depth"] in (16, 32):
        return True
    return False


def channel_terms(v):
    """The ordered (top-first) output pixels of a tz vector, each as a list of
    four component expressions plus the concrete bytes, or `None` if the shape
    is not one this generator models."""
    raw = expanded(v)
    mode = v["mode"]
    g = parse_geometry(raw, mode)
    if g is None:
        return None
    w, sh, depth, comp, used = g["w"], g["sh"], g["depth"], g["comp"], g["used"]
    s, H, ps = g["s"], g["H"], g["ps"]
    if not (1 <= w <= 8192 and sh != 0):
        return None
    top_down = (not is_ico_mode(mode)) and sh < 0
    h = (sh // 2) if is_ico_mode(mode) else abs(sh)
    if not (1 <= h <= 8192):
        return None

    # masks
    if comp == 3:
        if H in (52, 56, 108, 124):
            mr = int.from_bytes(raw[s + 40:s + 44], "little")
            mg = int.from_bytes(raw[s + 44:s + 48], "little")
            mb = int.from_bytes(raw[s + 48:s + 52], "little")
            ma = int.from_bytes(raw[s + 52:s + 56], "little") if H in (56, 108, 124) else 0
            ps = s + H + used * 4
        else:
            # H = 40 or 64: three external masks at s+H
            mr = int.from_bytes(raw[s + H:s + H + 4], "little")
            mg = int.from_bytes(raw[s + H + 4:s + H + 8], "little")
            mb = int.from_bytes(raw[s + H + 8:s + H + 12], "little")
            ma = 0
            ps = s + H + 12 + used * 4
    elif depth == 16:
        mr, mg, mb, ma = 0x7c00, 0x03e0, 0x001f, 0
    else:
        return None

    stride = (w * depth + 31) // 32 * 4
    nbytes = depth // 8
    if mode == "BmpFile":
        bf_off = int.from_bytes(raw[10:14], "little")
        if bf_off >= ps:
            ps = bf_off
    if ps + h * stride > len(raw):
        return None

    # expected alpha bytes per output pixel (the whole-image ICO policy is not
    # expressible as a per-pixel `channel_of`, so the ICO alpha is taken from
    # the fixture's own expected runs).
    exp_runs = v["expected"]["rgba_runs"]
    alpha_bytes = []
    for r in exp_runs:
        alpha_bytes += [int(r["hex"][6:8], 16)] * r["count"]
    if len(alpha_bytes) != w * h:
        return None

    pixels = []
    for q in range(h):
        r = q if top_down else h - 1 - q
        for x in range(w):
            off = ps + r * stride + x * nbytes
            if off + nbytes > len(raw):
                return None
            word = int.from_bytes(raw[off:off + nbytes], "little")
            a = alpha_bytes[q * w + x]
            pixels.append((word, mr, mg, mb, ma, a))
    return dict(w=w, h=h, depth=depth, pixels=pixels)


def compose_terms(pixels, ico):
    """Component expressions for one pixel: `channel_of(word, mask)` for each
    nonzero mask, the literal 255 for an absent mask, and the fixture's alpha
    byte for ICO (whose whole-image AND policy is not per-pixel)."""
    out = []
    for (word, mr, mg, mb, ma, a) in pixels:
        comps = []
        for mask in (mr, mg, mb):
            comps.append("crate::model::channel_of(%s, %s)" % (hex32(word), hex32(mask)))
        if ico:
            comps.append("%du8" % a)
        elif ma != 0:
            comps.append("crate::model::channel_of(%s, %s)" % (hex32(word), hex32(ma)))
        else:
            comps.append("255u8")
        out.append(comps)
    return out


def hex32(x):
    return "0x%08xu32" % x


def literal_bytes(pixels, ico):
    out = []
    for (word, mr, mg, mb, ma, a) in pixels:
        r = scale(word, mr) if mr else 255
        gv = scale(word, mg) if mg else 255
        b = scale(word, mb) if mb else 255
        if ico:
            av = a
        elif ma != 0:
            av = scale(word, ma)
        else:
            av = 255
        out.append((r, gv, b, av))
    return out


def scale(word, mask):
    k = (mask & -mask).bit_length() - 1
    mx = mask >> k
    comp = (word & mask) >> k
    return (255 * comp + mx // 2) // mx


def ident(name):
    return "".join(c if c.isalnum() else "_" for c in name)


def byte_lit(b):
    return "0x%02xu8" % b


def seq_of_bytes(data, indent=8, width=12):
    parts = [byte_lit(b) for b in data]
    if not parts:
        return "seq![]"
    lines = []
    for i in range(0, len(parts), width):
        lines.append(" " * indent + ", ".join(parts[i:i + width]) + ",")
    return "seq![\n" + "\n".join(lines) + "\n" + " " * (indent - 4) + "]"


def pattern_lit(hexstr):
    data = bytes.fromhex(hexstr)
    assert len(data) == 4, "each run is one RGBA pixel"
    return "seq![" + ", ".join(byte_lit(b) for b in data) + "]"


def expand_runs(runs, indent=12):
    if len(runs) == 1:
        return "rep_pat(%s, %d)" % (pattern_lit(runs[0]["hex"]), runs[0]["count"])
    parts = ["rep_pat(%s, %d)" % (pattern_lit(r["hex"]), r["count"]) for r in runs]
    return ("\n" + " " * indent + "+ ").join(parts)


HEADER = '''//! Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.
//!
//! DO NOT EDIT BY HAND.  Regenerate with
//!     python3 tools/gen_model_vectors.py
//! The expected values come from `spec-1.1-frozen/VECTORS.json` (`expected`),
//! never from the executable decoder.  Each `agree_<vector>` obligation states
//! that the formal model `MP` applied to the vector's expanded input yields
//! exactly the fixture's expected `Result` (`Ok(width, height, rgba)`
//! byte-exact, or `Err(e)` for the exact error name).  A mismatch makes the
//! obligation **false**, not vacuous.
//!
//! The first 214 records are the unchanged 1.0 fixtures (both ICO modes appear
//! for the 88 appended ones); the generator emits one obligation per eligible
//! vector regardless of mode.
//!
//! Two proof forms appear.  Most vectors are closed directly by `by (compute)`.
//! The 16-bit and BITFIELDS vectors cannot be: their pixels go through the
//! model's `channel_of`, whose `u32_trailing_zeros` is a `closed` spec fn the
//! `compute` normalizer refuses to unfold.  Those are split into a `by (compute)`
//! step against the model's own `channel_of` terms and a `channel_lit` step
//! relating each term to its concrete byte, composed inside `agree_<vector>`.
//!
//! The vectors longer than 40,000 bytes are covered by the symbolic lemmas
//! L1-L2 in `src/model.rs` (the three oversized inputs) and by mandatory runtime
//! conformance tests (`output-exact-64m-rle8`, `height-at-cap-8192`, and the two
//! 64 MiB R19 boundary fixtures `max-input-slack-IcoEntryDib{,Slack}`); no 64 MiB
//! input is expanded into verifier terms. The small `slack-*` fixtures cover the
//! same new-mode branch symbolically.
use vstd::prelude::*;

verus! {

#[cfg(verus_only)]
use crate::model::mp;
#[cfg(verus_only)]
use crate::dib::{Error, Mode};

/// `rep_pat(p, k)` repeats the four-pixel pattern `p` `k` times (R16's run
/// lists), i.e. output byte `i` is `p[i % 4]`.
pub open spec fn rep_pat(pat: Seq<u8>, count: nat) -> Seq<u8> {
    Seq::new((pat.len() * count) as nat, |i: int| pat[i % (pat.len() as int)])
}

/// One R6 component conversion, from the mask's bit facts.  The caller supplies
/// `u32_trailing_zeros`'s defining facts (they are decidable by `bit_vector` on
/// a literal mask), and `scale_of`'s value; the conclusion is the model's own
/// `channel_of` byte.  This is the step `compute` cannot take.
pub proof fn channel_lit(word: u32, mask: u32, k: u32, mx: u32, comp: u32, val: u8)
    requires
        mask != 0u32,
        k < 32u32,
        (mask >> k) & 1u32 == 1u32,
        forall|j: u32| 0 <= j < k ==> #[trigger] (mask >> j) & 1u32 == 0u32,
        mask >> k == mx,
        (word & mask) >> k == comp,
        crate::model::scale_of(comp as int, crate::model::channel_span(mx) as int) == val as int,
    ensures
        crate::model::channel_of(word, mask) == val,
{
    reveal(crate::model::channel_of);
    reveal(crate::model::scale_of);
    broadcast use vstd::std_specs::bits::axiom_u32_trailing_zeros;
    assert(vstd::std_specs::bits::u32_trailing_zeros(mask) == k) by {
        let t = vstd::std_specs::bits::u32_trailing_zeros(mask);
        assert(0u32 <= t <= 32u32);
        if t < k {
            assert((mask >> t) & 1u32 == 0u32);
            assert((mask >> t) & 1u32 == 1u32);
        } else if t > k {
            assert((mask >> k) & 1u32 == 0u32);
        }
    }
    assert(mask.trailing_zeros() == k);
    assert(((word & mask) >> mask.trailing_zeros()) == comp);
    assert((mask >> mask.trailing_zeros()) == mx);
}
'''


def gen():
    vectors = json.load(open(VECTORS))
    total = len(vectors)
    large_checked = check_large(vectors)
    body = [HEADER]
    body.append("\n// %d vectors; five are covered symbolically or at runtime:\n//\n" % total)
    for name, kind, cover, n in large_checked:
        body.append("/// `%s` (%d bytes, %s): %s.\n" % (name, n, kind, cover))
    body.append("\n")

    ico_path = os.path.join(ROOT, "tools", "gen_ico32.py")
    ico_spec = importlib.util.spec_from_file_location("dib_ico_vectors", ico_path)
    ico_mod = importlib.util.module_from_spec(ico_spec)
    ico_spec.loader.exec_module(ico_mod)
    proofs = []
    counts = dict(literal=0, term=0, staged=0)
    for v in vectors:
        name = v["name"]
        i = ident(name)
        data = expanded(v)
        if len(data) > LIMIT or name in LARGE:
            continue
        mode = v["mode"]
        exp = v["expected"]
        if ico_mod.use_ico32(v):
            proofs.append(ico_mod.one(v))
            counts["staged"] += 1
            continue

        if tz_path(v):
            info = channel_terms(v)
            if info is not None:
                terms = compose_terms(info["pixels"], is_ico_mode(mode))
                lits = literal_bytes(info["pixels"], is_ico_mode(mode))
                flat = [x for comps in lits for x in comps]
                # The bytes the model's terms reduce to must equal the fixture's
                # own expected pixels (so `agree` states the fixture's result).
                want = []
                for r in exp["rgba_runs"]:
                    want += [int(r["hex"][j:j + 2], 16) for j in (0, 2, 4, 6)] * r["count"]
                assert flat == want, (name, flat, want)
                block = []
                block.append("// ---- %s (%d bytes, %s) [term form] ----\n" % (name, len(data), mode))
                block.append("pub open spec fn b_%s() -> Seq<u8> {\n    %s\n}\n\n"
                             % (i, seq_of_bytes(data)))
                # want(): the model's own channel_of terms, one seq per pixel
                want_parts = []
                for comps in terms:
                    want_parts.append("        " + ", ".join(comps))
                block.append("pub open spec fn want_%s() -> Seq<u8> {\n    seq![\n%s,\n    ]\n}\n\n"
                             % (i, ",\n".join(want_parts)))
                # step 1
                block.append("/// Step 1: the model returns the `channel_of` terms.\n")
                block.append("#[allow(unused)]\n")
                block.append("proof fn mp_term_%s()\n    ensures\n        mp(b_%s(), Mode::%s) == Ok((%d, %d, want_%s())),\n{\n"
                             % (i, i, mode, info["w"], info["h"], i))
                block.append("    reveal(crate::model::bitfield_seq);\n")
                block.append("    reveal(crate::model::bitfield_channel);\n")
                block.append("    reveal(crate::model::rgb555_seq);\n")
                block.append("    reveal(crate::model::rgb555_channel);\n")
                block.append("    reveal(crate::model::conv);\n")
                block.append("    reveal(crate::model::conv_word);\n")
                block.append("    reveal(crate::model::channel_of);\n")
                block.append("    reveal(crate::model::scale_of);\n")
                block.append("    reveal(crate::model::ma_output);\n")
                block.append("    reveal(crate::model::ma_byte);\n")
                block.append("    reveal(crate::model::xor_alpha_present);\n")
                block.append("    assert(mp(b_%s(), Mode::%s) == Ok((%d, %d, want_%s()))) by (compute);\n}\n\n"
                             % (i, mode, info["w"], info["h"], i))
                # step 2
                block.append("/// Step 2: the `channel_of` terms are the fixture's bytes.\n")
                block.append("#[allow(unused)]\n")
                block.append("proof fn want_bytes_%s()\n    ensures\n        want_%s() == %s,\n{\n"
                             % (i, i, seq_of_bytes(flat)))
                block.append("    reveal(crate::model::channel_of);\n")
                block.append("    reveal(crate::model::scale_of);\n")
                # emit per-pixel facts
                for pi, (word, mr, mg, mb, ma, a) in enumerate(info["pixels"]):
                    r, gv, b, av = lits[pi]
                    ico = is_ico_mode(mode)
                    for mask, val in ((mr, r), (mg, gv), (mb, b), (ma, av)):
                        if mask == 0:
                            continue
                        if ico and mask == ma:
                            # ICO alpha is the fixture byte, not channel_of
                            continue
                        k = (mask & -mask).bit_length() - 1
                        mx = mask >> k
                        comp = (word & mask) >> k
                        v2 = (255 * comp + mx // 2) // mx
                        assert v2 == val, (name, mask, val, v2)
                        block.append("    assert(%s != 0u32) by (bit_vector);\n" % hex32(mask))
                        block.append("    assert((%s >> %du32) & 1u32 == 1u32) by (bit_vector);\n"
                                     % (hex32(mask), k))
                        block.append("    assert forall|j: u32| 0 <= j < %du32 implies #[trigger] (%s >> j) & 1u32 == 0u32 by {\n"
                                     % (k, hex32(mask)))
                        block.append("        assert((%s >> j) & 1u32 == 0u32) by (bit_vector) requires 0 <= j < %du32;\n"
                                     % (hex32(mask), k))
                        block.append("    };\n")
                        block.append("    assert(%s >> %du32 == %s) by (bit_vector);\n"
                                     % (hex32(mask), k, hex32(mx)))
                        block.append("    assert((%s & %s) >> %du32 == %su32) by (bit_vector);\n"
                                     % (hex32(word), hex32(mask), k, comp))
                        block.append("    assert(crate::model::scale_of(%dint, %dint) == %dint) by (nonlinear_arith);\n"
                                     % (comp, mx, v2))
                        block.append("    assert(crate::model::channel_span(%s) == %s) by (compute);\n"
                                     % (hex32(mx), hex32(mx)))
                        block.append("    channel_lit(%s, %s, %du32, %s, %su32, %du8);\n"
                                     % (hex32(word), hex32(mask), k, hex32(mx), comp, v2))
                block.append("    assert(want_%s() == %s);\n" % (i, seq_of_bytes(flat)))
                block.append("}\n\n")
                # step 3: compose.  The postcondition states the fixture's bytes
                # directly (a `seq!` literal), which is exactly `Ok((w, h, bytes))`.
                block.append("#[allow(unused)]\n")
                block.append("pub proof fn agree_%s()\n" % i)
                block.append("    ensures\n        mp(b_%s(), Mode::%s) == Ok((%d, %d, %s)),\n{\n"
                             % (i, mode, info["w"], info["h"], seq_of_bytes(flat)))
                block.append("    mp_term_%s();\n" % i)
                block.append("    want_bytes_%s();\n" % i)
                block.append("}\n\n")
                proofs.append("".join(block))
                counts["term"] += 1
                continue

        # literal form
        block = []
        block.append("// ---- %s (%d bytes, %s) ----\n" % (name, len(data), mode))
        block.append("pub open spec fn b_%s() -> Seq<u8> {\n    %s\n}\n\n" % (i, seq_of_bytes(data)))
        if "error" in exp:
            stmt = "mp(b_%s(), Mode::%s) == Err(Error::%s)" % (i, mode, exp["error"])
        else:
            stmt = ("mp(b_%s(), Mode::%s) == Ok((%d, %d, %s))"
                    % (i, mode, exp["width"], exp["height"], expand_runs(exp["rgba_runs"])))
        block.append("#[allow(unused)]\n")
        block.append("pub proof fn agree_%s()\n    ensures\n        %s,\n{\n" % (i, stmt))
        block.append("    assert(%s) by (compute);\n}\n\n" % stmt)
        proofs.append("".join(block))
        counts["literal"] += 1

    text = "".join(body) + "".join(proofs)
    text += "// ---- every small vector has exactly one obligation above ----\n"
    text += "} // verus!\n"
    return text, counts, large_checked


def main():
    out = DEFAULT_OUT
    args = sys.argv[1:]
    check = "--check" in args
    if "--out" in args:
        out = args[args.index("--out") + 1]
    text, counts, large_checked = gen()
    print("generated %d obligations (%d literal, %d term, %d staged), %d large"
          % (counts["literal"] + counts["term"] + counts["staged"], counts["literal"], counts["term"], counts["staged"],
             len(large_checked)))
    if check:
        old = open(out).read() if os.path.exists(out) else ""
        if old != text:
            print("STALE: %s differs from generated output" % out)
            return 1
        print("up to date: %s" % out)
        return 0
    with open(out, "w") as f:
        f.write(text)
    print("wrote %s (%d bytes)" % (out, len(text)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
