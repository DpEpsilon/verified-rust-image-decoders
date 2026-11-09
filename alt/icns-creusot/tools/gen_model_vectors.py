#!/usr/bin/env python3
"""Generate Creusot model-vs-vector agreement proofs from spec-frozen/VECTORS.json.

For each of the 102 vectors whose expanded input is at most 4,096 bytes, emit Pearlite
`#[logic]` obligations stating that the crate's formal model, applied to the vector's
expanded input, yields exactly the fixture's `expected` value.  Expected values are read
from `spec-frozen/VECTORS.json`; the executable decoder is never consulted.

Container vectors state `model::mp(b) == <expected>` exactly.  Decoding vectors state

    match model::mp(b) { Ok(ms) => <mc_model|ma_model|mj_model>(b, ms, idx) == <expected>,
                         Err(_) => false }

so the entry list the decoder model consumes is `mp`'s own parse result, and a parse
failure makes the obligation false rather than vacuous.

Creusot's recursive models (`model::mp_loop`, `rmodel::mr_ch`) are opaque outside their
modules, so each obligation is a chain of small lemmas instantiating the proved
single-step facts at concrete `Int` positions, following the M3c encoding rules
(byte-at-index functions via `Seq::create`, `Int` not `u32` in logic, no concat/repeat).

The nine large vectors are covered by the symbolic lemmas L1-L6.

Usage: tools/gen_model_vectors.py [--out PATH] [--check] [--only parse|rgb|mask|combine]
"""

import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VECTORS = os.path.join(ROOT, "spec-frozen", "VECTORS.json")
DEFAULT_OUT = os.path.join(ROOT, "icns_creusot", "src", "model_vectors.rs")

LIMIT = 4096

LARGE = {
    "file-too-large": "l1 (L1)",
    "file-size-at-limit": "l2 (L2)",
    "mask-t8mk": "inst_mask_t8mk (L3)",
    "entry-count-exactly-4096": "inst_entry_count_exactly_4096 (L4)",
    "entry-count-limit": "inst_entry_count_limit (L5)",
    "count-limit-before-stray-suffix": "inst_count_limit_before_stray_suffix (L5)",
    "entry-count-before-later-malformed-entry": "inst_entry_count_before_later_malformed_entry (L5)",
    "invalid-4097th-entry-before-limit": "inst_invalid_4097th_entry_before_limit (L6)",
    "entry-count-followed-by-overflowing-4097th": "inst_entry_count_followed_by_overflowing_4097th (L6)",
}

MODERN = {b"icp4": 16, b"icp5": 32, b"icp6": 64, b"ic07": 128, b"ic08": 256, b"ic09": 512,
          b"ic10": 1024, b"ic11": 32, b"ic12": 64, b"ic13": 256, b"ic14": 512,
          b"icsB": 36, b"sb24": 24, b"SB24": 48}
LEGACY = set(b"is32 il32 ih32 it32 s8mk l8mk h8mk t8mk".split())
COLOUR = {b"is32": 16, b"il32": 32, b"ih32": 48, b"it32": 128}
MASK = {b"s8mk": 16, b"l8mk": 32, b"h8mk": 48, b"t8mk": 128}
PNG = b"\x89PNG\r\n\x1a\n"
JP2 = b"\x00\x00\x00\x0cjP  \r\n\x87\n"

# Error names reachable as MR's first/only failure of a channel.
RLE_ERR = {"RleTruncated", "RleOverrun", "RleTrailingBytes"}


# ------------------------------------------------------------------ fixture model

def expanded(v):
    return bytes.fromhex(v["input_hex"]) * v.get("input_repeat", 1)


def mp(b):
    if len(b) > 67108864:
        return ("Err", "FileTooLarge")
    if len(b) < 8:
        return ("Err", "TruncatedHeader")
    if b[0:4] != b"icns":
        return ("Err", "BadMagic")
    L = int.from_bytes(b[4:8], "big")
    if L < 8:
        return ("Err", "InvalidFileLength")
    if L > len(b):
        return ("Err", "TruncatedFile")
    if L < len(b):
        return ("Err", "TrailingFileBytes")
    off, ent = 8, []
    while off < L:
        if L - off < 8:
            return ("Err", "TruncatedEntryHeader")
        ty = b[off:off + 4]
        q = int.from_bytes(b[off + 4:off + 8], "big")
        if q < 8:
            return ("Err", "InvalidEntryLength")
        if q > L - off:
            return ("Err", "EntryOutOfBounds")
        if len(ent) == 4096:
            return ("Err", "TooManyEntries")
        ent.append((ty, off + 8, off + q))
        off += q
    return ("Ok", ent)


def classify(ty, payload):
    ns = MODERN.get(ty)
    if ns is not None:
        return True, ns
    if ty in LEGACY:
        return False, None
    return payload.startswith(PNG) or payload.startswith(JP2), None


def chan(s, cur, n):
    produced, packets = 0, []
    while produced < n:
        if cur >= len(s):
            return ("Err", "RleTruncated", packets, cur)
        c = s[cur]
        count = c + 1 if c < 128 else c - 125
        lit = c < 128
        if count > n - produced:
            return ("Err", "RleOverrun", packets, cur)
        need = count if lit else 1
        if len(s) - (cur + 1) < need:
            return ("Err", "RleTruncated", packets, cur)
        packets.append({"cur": cur, "ctrl": c, "count": count, "lit": lit, "val": s[cur + 1],
                        "bytes": list(s[cur + 1:cur + 1 + count]), "prod": produced,
                        "next": cur + 1 + need})
        cur += 1 + need
        produced += count
    return ("Ok", packets, cur)


def interleave(r, g, b, n):
    o = bytearray(3 * n)
    for k in range(n):
        o[3 * k] = r[k]
        o[3 * k + 1] = g[k]
        o[3 * k + 2] = b[k]
    return bytes(o)


def rgba(rgb, alpha, n):
    o = bytearray(4 * n)
    for k in range(n):
        o[4 * k:4 * k + 3] = rgb[3 * k:3 * k + 3]
        o[4 * k + 3] = alpha[k]
    return bytes(o)


def runs_of(data):
    out, i = [], 0
    while i < len(data):
        j = i
        while j < len(data) and data[j] == data[i]:
            j += 1
        out.append((i, j, data[i]))
        i = j
    return out


def chbytes(packets, n):
    out = []
    for p in packets:
        out.extend(p["bytes"] if p["lit"] else [p["val"]] * p["count"])
    return bytes(out[:n])


def uniform_runs(packets):
    """Split the packet list into maximal runs with the same (literal, count, value)."""
    runs, i = [], 0
    while i < len(packets):
        p = packets[i]
        key = (p["lit"], p["count"], None if p["lit"] else p["val"])
        j = i
        if p["lit"]:
            # literals only group when the byte sequences are identical
            while (j < len(packets) and packets[j]["lit"]
                   and packets[j]["bytes"] == p["bytes"]):
                j += 1
        else:
            while (j < len(packets) and not packets[j]["lit"]
                   and packets[j]["count"] == p["count"] and packets[j]["val"] == p["val"]):
                j += 1
        runs.append(packets[i:j])
        i = j
    return runs


# ------------------------------------------------------------------ emission helpers

def ident(name):
    return "".join(c if c.isalnum() else "_" for c in name)


def table_data(n, data):
    if len(set(data)) == 1:
        return "Seq::create(%d, |_i: Int| %du8)" % (n, data[0])
    # try a small period
    for per in range(2, min(64, len(data)) + 1):
        if all(data[i] == data[i % per] for i in range(len(data))):
            pat = data[:per]
            parts = ["if i %% %d == %d { %du8 }" % (per, j, pat[j]) for j in range(per - 1)]
            body = " else ".join(parts)
            return "Seq::create(%d, |i: Int| %s else { %du8 })" % (n, body, pat[-1])
    # Long tables with a short exceptional prefix still have a compact pointwise model.
    # This covers the eight-zero it32 prefix and a mask whose first alpha differs.
    if len(data) > 32:
        for cut in range(1, min(16, len(data))):
            for per in range(1, 9):
                if all(data[j] == data[cut + (j - cut) % per] for j in range(cut, len(data))):
                    prefix = ["if i == %d { %du8 }" % (j, data[j]) for j in range(cut)]
                    tail = data[cut:cut + per]
                    periodic = ["if (i - %d) %% %d == %d { %du8 }" % (cut, per, j, tail[j])
                                for j in range(per - 1)]
                    return ("Seq::create(%d, |i: Int| %s else { %s })" %
                            (n, " else ".join(prefix),
                             " else ".join(periodic + ["{ %du8 }" % tail[-1]]) if periodic else "%du8" % tail[0]))
    return seqlit(data)


def table(n, runs):
    """A `Seq::create(n, |i| ...)` expression for `runs`; periodic data uses a modulo."""
    if len(runs) == 0:
        return "Seq::create(%d, |_i: Int| 0u8)" % n
    data = []
    for (a2, b2, val) in runs:
        data.extend([val] * (b2 - a2))
    return table_data(n, data)


def seqlit(data, indent=8, width=16):
    parts = ["%du8" % b for b in data]
    lines = [" " * (indent + 4) + ", ".join(parts[i:i + width]) + ","
             for i in range(0, len(parts), width)]
    return "seq![\n" + "\n".join(lines) + "\n" + " " * indent + "]"


def hexchars(hx):
    return ", ".join("%d" % ord(c) for c in hx)


def mkentry(ty, s, e, payload, ind="        "):
    opaque, ns = classify(ty, payload)
    nss = "None" if ns is None else "Some(%du32)" % ns
    return ("MEntry {\n%s    type_hex: seq![%s],\n%s    start: %d, end: %d,\n"
            "%s    opaque_candidate: %s,\n%s    nominal_side: %s,\n%s}"
            % (ind, hexchars("".join("%02x" % c for c in ty)), ind, s, e, ind,
               "true" if opaque else "false", ind, nss, ind))


def ms_expr(ents, b):
    if not ents:
        return "Seq::empty()"
    out = "Seq::empty()"
    for ty, s, e in ents:
        out += ".push_back(%s)" % mkentry(ty, s, e, b[s:e], ind="            ").replace(
            "\n            ", "\n                ")
    return out



HEADER = "// Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.\n//\n// DO NOT EDIT BY HAND.  Regenerate with\n//     python3 tools/gen_model_vectors.py\n//\n// Each `agree_*` obligation states that the crate's formal model, applied to the vector's\n// expanded input, yields exactly the fixture's `expected` value.  Expected values are read\n// from `spec-frozen/VECTORS.json` and never from the executable decoder.\n//\n// Container vectors state `model::mp(b) == <expected>` (an exact `Result`: the same error\n// variant, or the same ordered entry list with all four fields).  Decoding vectors state\n// `match model::mp(b) { Ok(ms) => <model>(b, ms, idx) == <expected>, Err(_) => false }`, so\n// the entries the decoder model consumes are `mp`'s own parse result and a parse failure\n// makes the obligation false rather than vacuous.\n//\n// Creusot's recursive models (`model::mp_loop`, `rmodel::mr_ch`) are opaque outside their\n// modules, so each obligation is a chain of small lemmas instantiating the proved\n// single-step facts (`model::step_acc`, `model::loop_done`, `model::loop_hdr`,\n// `model::loop_short`, `model::loop_oob`, `model::loop_many`, `rmodel::ch_step`,\n// `rmodel::ch_done`, `rmodel::ch_over`, `rmodel::ch_data`, `rmodel::ch_trunc`) at concrete\n// `Int` positions, following the M3c encoding rules: byte-at-index functions via\n// `Seq::create`, `Int` not `u32` in logic, no concat/repeat constructors.\n//\n// The nine vectors longer than 4,096 bytes are covered by the symbolic lemmas L1-L6.\n\nuse creusot_std::prelude::*;\n\nuse crate::model;\nuse crate::{\n    be32_log, interleave3, magic_model, ma_model, mc_entry, mc_model, mj_model, mk_entry,\n    mr_iw, mr_model, rgba4, rmodel, zero_buf, Error, MEntry,\n};\n\n/// The `n`-byte buffer whose bytes below `filled` come from `rf` and whose bytes at or above\n/// `filled` are still zero: a channel buffer after a prefix of its packets.\n#[logic(open)]\npub fn part(rf: Seq<u8>, n: Int, filled: Int) -> Seq<u8> {\n    pearlite! { Seq::create(n, |i: Int| if 0 <= i && i < filled { rf[i] } else { 0u8 }) }\n}\n\n/// A buffer that is `rf` up to `filled` and zero above is `rf` once `filled` reaches its length.\n#[logic]\n#[requires(0 <= n)]\n#[requires(rf.len() == n)]\n#[ensures(part(rf, n, n) == rf)]\npub fn partn(rf: Seq<u8>, n: Int) {}\n\n/// A repeat packet writing `val` over `[prod, prod+count)`, where `rf` is `val` on exactly\n/// that range.\n#[logic]\n#[requires(count > 0)]\n#[requires(0 <= prod && prod + count <= n)]\n#[requires(0 <= ctrl && ctrl + 1 < s.len())]\n#[requires(!(s[ctrl]@ < 128))]\n#[requires(s[ctrl]@ - 125 == count)]\n#[requires(s[ctrl + 1]@ == val@)]\n#[requires(rf.len() == n)]\n#[requires(forall<i: Int> prod <= i && i < prod + count ==> rf[i]@ == val@)]\n#[ensures(mr_iw(s, ctrl, false, prod, count, part(rf, n, prod)) == part(rf, n, prod + count))]\npub fn iw_rep(s: Seq<u8>, ctrl: Int, prod: Int, count: Int, n: Int, val: u8, rf: Seq<u8>) {\n    proof_assert!(mr_iw(s, ctrl, false, prod, count, part(rf, n, prod))\n        .ext_eq(part(rf, n, prod + count)));\n}\n\n/// A literal packet whose `count` bytes are exactly `rf`'s bytes over `[prod, prod+count)`.\n#[logic]\n#[requires(count > 0)]\n#[requires(0 <= prod && prod + count <= n)]\n#[requires(0 <= ctrl && ctrl + count < s.len())]\n#[requires(s[ctrl]@ < 128)]\n#[requires(s[ctrl]@ + 1 == count)]\n#[requires(rf.len() == n)]\n#[requires(forall<i: Int> 0 <= i && i < count ==> s[ctrl + 1 + i]@ == rf[prod + i]@)]\n#[ensures(mr_iw(s, ctrl, true, prod, count, part(rf, n, prod)) == part(rf, n, prod + count))]\npub fn iw_lit(s: Seq<u8>, ctrl: Int, prod: Int, count: Int, n: Int, rf: Seq<u8>) {\n    proof_assert!(mr_iw(s, ctrl, true, prod, count, part(rf, n, prod))\n        .ext_eq(part(rf, n, prod + count)));\n}\n\n/// One repeat packet step.\n#[logic]\n#[requires(count > 0)]\n#[requires(0 <= prod && prod + count <= n)]\n#[requires(0 <= cur && cur + 1 < s.len())]\n#[requires(!(s[cur]@ < 128))]\n#[requires(s[cur]@ - 125 == count)]\n#[requires(s[cur + 1]@ == val@)]\n#[requires(rf.len() == n)]\n#[requires(forall<i: Int> prod <= i && i < prod + count ==> rf[i]@ == val@)]\n#[ensures(rmodel::mr_ch(s, cur, prod, n, part(rf, n, prod))\n    == rmodel::mr_ch(s, cur + 2, prod + count, n, part(rf, n, prod + count)))]\npub fn step_rep(s: Seq<u8>, cur: Int, prod: Int, count: Int, n: Int, val: u8, rf: Seq<u8>) {\n    rmodel::ch_step(s, cur, prod, n, part(rf, n, prod));\n    iw_rep(s, cur, prod, count, n, val, rf);\n}\n\n/// One literal packet step.\n#[logic]\n#[requires(count > 0)]\n#[requires(0 <= prod && prod + count <= n)]\n#[requires(0 <= cur && cur + count < s.len())]\n#[requires(s[cur]@ < 128)]\n#[requires(s[cur]@ + 1 == count)]\n#[requires(rf.len() == n)]\n#[requires(forall<i: Int> 0 <= i && i < count ==> s[cur + 1 + i]@ == rf[prod + i]@)]\n#[ensures(rmodel::mr_ch(s, cur, prod, n, part(rf, n, prod))\n    == rmodel::mr_ch(s, cur + 1 + count, prod + count, n, part(rf, n, prod + count)))]\npub fn step_lit(s: Seq<u8>, cur: Int, prod: Int, count: Int, n: Int, rf: Seq<u8>) {\n    rmodel::ch_step(s, cur, prod, n, part(rf, n, prod));\n    iw_lit(s, cur, prod, count, n, rf);\n}\n\n/// The channel is full: `mr_ch` returns the buffer and the cursor unchanged.\n#[logic]\n#[requires(0 <= n)]\n#[ensures(rmodel::mr_ch(s, cur, n, n, rf) == Ok((rf, cur)))]\npub fn done(s: Seq<u8>, cur: Int, n: Int, rf: Seq<u8>) {\n    rmodel::ch_done(s, cur, n, n, rf);\n}\n\n/// `MR`'s first-channel error.\n#[logic]\n#[requires(0 <= n)]\n#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Err(er))]\n#[ensures(mr_model(s, n) == Err(er))]\npub fn mr_err1(s: Seq<u8>, n: Int, er: Error) {}\n\n/// `MR`'s second-channel error.\n#[logic]\n#[requires(0 <= n)]\n#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]\n#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Err(er))]\n#[ensures(mr_model(s, n) == Err(er))]\npub fn mr_err2(s: Seq<u8>, n: Int, r0: Seq<u8>, c1: Int, er: Error) {}\n\n/// `MR`'s third-channel error.\n#[logic]\n#[requires(0 <= n)]\n#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]\n#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]\n#[requires(rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) == Err(er))]\n#[ensures(mr_model(s, n) == Err(er))]\npub fn mr_err3(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, c1: Int, c2: Int, er: Error) {}\n\n/// `MR` composed of three finished channels.\n#[logic]\n#[requires(0 <= n)]\n#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]\n#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]\n#[requires(rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) == Ok((b0, c3)))]\n#[requires(c3 == s.len())]\n#[ensures(mr_model(s, n) == Ok((r0, g0, b0)))]\npub fn mr_three(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, b0: Seq<u8>,\n                c1: Int, c2: Int, c3: Int) {}\n\n/// `interleave3` of three channels equals a byte-at-index table.\n#[logic]\n#[requires(0 <= n)]\n#[requires(r.len() == n && g.len() == n && b.len() == n)]\n#[requires(out.len() == 3 * n)]\n#[requires(forall<i: Int> 0 <= i && i < 3 * n ==> out[i]\n    == (if i % 3 == 0 { r[i / 3] } else if i % 3 == 1 { g[i / 3] } else { b[i / 3] }))]\n#[ensures(interleave3(r, g, b, n) == out)]\npub fn interleave_eq(r: Seq<u8>, g: Seq<u8>, b: Seq<u8>, n: Int, out: Seq<u8>) {\n    proof_assert!(interleave3(r, g, b, n).ext_eq(out));\n}\n\n/// The `rgba4` layout of an RGB buffer and an alpha channel equals a byte-at-index table.\n#[logic]\n#[requires(0 <= n)]\n#[requires(rgb.len() == 3 * n && alpha.len() == n)]\n#[requires(out.len() == 4 * n)]\n#[requires(forall<i: Int> 0 <= i && i < 4 * n ==> out[i]\n    == (if i % 4 == 3 { alpha[i / 4] } else { rgb[3 * (i / 4) + i % 4] }))]\n#[ensures(rgba4(rgb, alpha, n) == out)]\npub fn rgba_eq(rgb: Seq<u8>, alpha: Seq<u8>, n: Int, out: Seq<u8>) {\n    proof_assert!(rgba4(rgb, alpha, n).ext_eq(out));\n}\n\n/// A one-entry model list.\n#[logic(open)]\npub fn one(m: MEntry) -> Seq<MEntry> {\n    pearlite! { Seq::empty().push_back(m) }\n}\n\n/// `Seq::create`'s read at an in-range index (uses the table's own axiom).\n#[logic]\n#[requires(0 <= i && i < len)]\n#[ensures(Seq::create(len, f)[i] == f.get(i))]\npub fn create_at<T>(len: Int, f: creusot_std::logic::Mapping<Int, T>, i: Int) {}"


# ------------------------------------------------------------------ decode vectors


# ------------------------------------------------------------------ parse vectors


def t_index(off):
    return off // 8 - 1


def acc(t, i):
    return "Seq::empty()" if t == 0 else "e_%s_%d()" % (i, t)


def err_acc(t, i):
    return "Seq::empty()" if t == 0 else "ea_%s_%d()" % (i, t)


def fixind(txt, n):
    lines = txt.split("\n")
    return lines[0] + "".join("\n" + (" " * n if l.strip() else l) + l for l in lines[1:])


def mkent_fact(i, off, b, fname):
    ty = b[off:off + 4]
    s, e = off + 8, off + int.from_bytes(b[off + 4:off + 8], "big")
    return ("#[logic]\n#[ensures(mk_entry(b_%s(), %d) == %s)]\npub fn %s() {}\n\n"
            % (i, off, mkentry(ty, s, e, b[s:e]).replace("        ", "    "), fname))


def emit_parse(v, o):
    name, i = v["name"], ident(v["name"])
    b, exp = expanded(v), v["expected"]
    o.append("// ---- %s (parse, %d bytes) ----\n" % (name, len(b)))
    o.append("#[logic(open)]\npub fn b_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n"
             % (i, seqlit(b)))
    o.append("#[logic]\n#[ensures(b_%s().len() == %d)]\npub fn blen_%s() {}\n\n" % (i, len(b), i))
    if "error" in exp:
        emit_parse_err(o, v, b, exp["error"], i)
        return
    emit_parse_ok(o, v, b, exp, i)


def emit_parse_err(o, v, b, err, i):
    facts = []
    if len(b) < 8 or len(b) > 67108864:
        facts.append("blen_%s()" % i)
    elif b[0:4] != b"icns":
        o.append("#[logic]\n#[ensures(!magic_model(b_%s()))]\npub fn m_%s() {}\n\n" % (i, i))
        facts.append("m_%s()" % i)
    else:
        L = int.from_bytes(b[4:8], "big")
        o.append("#[logic]\n#[ensures(magic_model(b_%s()))]\npub fn m_%s() {}\n\n" % (i, i))
        o.append("#[logic]\n#[ensures(be32_log(b_%s(), 4) == %d)]\npub fn h_%s() {}\n\n" % (i, L, i))
        facts.extend(["m_%s()" % i, "h_%s()" % i])
        if err in ("InvalidFileLength", "TruncatedFile", "TrailingFileBytes"):
            o.append("#[logic]\n#[ensures(model::mp(b_%s()) == Err(Error::%s))]\npub fn agree_%s() {\n"
                     "    blen_%s();\n    m_%s();\n    h_%s();\n}\n\n" % (i, err, i, i, i, i))
            return
        o.append("#[logic(open)]\npub fn ea_%s_0() -> Seq<MEntry> {\n    pearlite! { Seq::empty() }\n}\n\n" % i)
        off, t = 8, 0
        while True:
            if L - off < 8:
                facts.append("model::loop_hdr(b_%s(), %d, %d, %s)" % (i, off, L, err_acc(t, i)))
                break
            q = int.from_bytes(b[off + 4:off + 8], "big")
            o.append("#[logic]\n#[ensures(be32_log(b_%s(), %d) == %d)]\npub fn q_%s_%d() {}\n\n"
                     % (i, off + 4, q, i, off))
            fn = "loop_short" if q < 8 else ("loop_oob" if q > L - off else
                                             ("loop_many" if t == 4096 else None))
            if fn:
                facts.extend(["q_%s_%d()" % (i, off),
                              "model::%s(b_%s(), %d, %d, %s)" % (fn, i, off, L, err_acc(t, i))])
                break
            ty, s0, e0 = b[off:off + 4], off + 8, off + q
            o.append("#[logic(open)]\npub fn eme_%s_%d() -> MEntry {\n    pearlite! { %s }\n}\n\n"
                     % (i, t, fixind(mkentry(ty, s0, e0, b[s0:e0]), 4)))
            o.append("#[logic(open)]\npub fn ea_%s_%d() -> Seq<MEntry> {\n"
                     "    pearlite! { ea_%s_%d().push_back(eme_%s_%d()) }\n}\n\n"
                     % (i, t + 1, i, t, i, t))
            o.append(mkent_fact(i, off, b, "k_%s_%d" % (i, off)))
            facts.extend(["q_%s_%d()" % (i, off), "k_%s_%d()" % (i, off),
                          "model::step_acc(b_%s(), %d, %d, %s)" % (i, off, L, err_acc(t, i))])
            off += q
            t += 1
    o.append("#[logic]\n#[ensures(model::mp(b_%s()) == Err(Error::%s))]\npub fn agree_%s() {\n"
             % (i, err, i))
    o.append("    blen_%s();\n" % i)
    for f in facts:
        o.append("    %s;\n" % f)
    o.append("}\n\n")


def entry_lit(e):
    ns = "None" if e["nominal_side"] is None else "Some(%du32)" % e["nominal_side"]
    return ("MEntry {\n    type_hex: seq![%s],\n    start: %d, end: %d,\n"
            "    opaque_candidate: %s,\n    nominal_side: %s,\n}"
            % (hexchars(e["type_hex"]), e["payload"][0], e["payload"][1],
               "true" if e["opaque_candidate"] else "false", ns))


def emit_parse_ok(o, v, b, exp, i):
    entries = exp["entries"]
    _, ents = mp(b)
    L = int.from_bytes(b[4:8], "big")
    o.append("#[logic(open)]\npub fn e_%s_0() -> Seq<MEntry> {\n    pearlite! { Seq::empty() }\n}\n\n" % i)
    for t, e in enumerate(entries):
        o.append("#[logic(open)]\npub fn me_%s_%d() -> MEntry {\n    pearlite! { %s }\n}\n\n"
                 % (i, t, fixind(entry_lit(e), 4)))
        o.append("#[logic(open)]\npub fn e_%s_%d() -> Seq<MEntry> {\n"
                 "    pearlite! { e_%s_%d().push_back(me_%s_%d()) }\n}\n\n"
                 % (i, t + 1, i, t, i, t))
    o.append("#[logic]\n#[ensures(magic_model(b_%s()))]\npub fn m_%s() {}\n\n" % (i, i))
    o.append("#[logic]\n#[ensures(be32_log(b_%s(), 4) == %d)]\npub fn h_%s() {}\n\n" % (i, L, i))
    for t, e in enumerate(entries):
        off = ents[t][1] - 8
        o.append("#[logic]\n#[ensures(be32_log(b_%s(), %d) == %d)]\npub fn q_%s_%d() {}\n\n"
                 % (i, off + 4, (ents[t][2] - ents[t][1] + 8), i, off))
        o.append(mkent_fact(i, off, b, "k_%s_%d" % (i, off)))
    o.append("#[logic]\n#[ensures(model::mp(b_%s()) == Ok(e_%s_%d()))]\npub fn agree_%s() {\n"
             % (i, i, len(entries), i))
    o.append("    blen_%s();\n    m_%s();\n    h_%s();\n" % (i, i, i))
    for t in range(len(entries)):
        off = ents[t][1] - 8
        o.append("    q_%s_%d();\n    k_%s_%d();\n" % (i, off, i, off))
    for t in range(len(entries)):
        off = ents[t][1] - 8
        o.append("    model::step_acc(b_%s(), %d, %d, %s);\n" % (i, off, L, acc(t, i)))
    o.append("    model::loop_done(b_%s(), %d, %d, %s);\n" % (i, L, L, acc(len(entries), i)))
    o.append("}\n\n")


def emit_decode(v, o):
    if v["operation"] == "combine":
        emit_combine(v, o)
    else:
        emit_rgbish(o, v, v["operation"] == "mask")


def model_entries(b):
    st, ents = mp(b)
    assert st == "Ok", "decoding vector must parse"
    return ents


def emit_b_and_ms(o, v, i):
    """Emit b_<i>, the model entry list and the parse bridge; return (b, ents)."""
    b = expanded(v)
    ents = model_entries(b)
    L = int.from_bytes(b[4:8], "big")
    o.append("#[logic(open)]\npub fn b_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n" % (i, seqlit(b)))
    o.append("#[logic]\n#[ensures(b_%s().len() == %d)]\npub fn blen_%s() {}\n\n" % (i, len(b), i))
    o.append("#[logic(open)]\npub fn e_%s_0() -> Seq<MEntry> {\n    pearlite! { Seq::empty() }\n}\n\n" % i)
    for t, (ty, s, e) in enumerate(ents):
        o.append("#[logic(open)]\npub fn me_%s_%d() -> MEntry {\n    pearlite! { %s }\n}\n\n"
                 % (i, t, fixind(mkentry(ty, s, e, b[s:e]), 4)))
        o.append("#[logic(open)]\npub fn e_%s_%d() -> Seq<MEntry> {\n"
                 "    pearlite! { e_%s_%d().push_back(me_%s_%d()) }\n}\n\n"
                 % (i, t + 1, i, t, i, t))
    o.append("#[logic(open)]\npub fn ms_%s() -> Seq<MEntry> {\n    pearlite! { e_%s_%d() }\n}\n\n"
             % (i, i, len(ents)))
    o.append("#[logic]\n#[ensures(magic_model(b_%s()))]\npub fn m_%s() {}\n\n" % (i, i))
    o.append("#[logic]\n#[ensures(be32_log(b_%s(), 4) == %d)]\npub fn h_%s() {}\n\n" % (i, L, i))
    for t, (ty, s, e) in enumerate(ents):
        off = s - 8
        o.append("#[logic]\n#[ensures(be32_log(b_%s(), %d) == %d)]\npub fn q_%s_%d() {}\n\n"
                 % (i, off + 4, e - s + 8, i, off))
        o.append(mkent_fact(i, off, b, "k_%s_%d" % (i, off)))
    o.append("#[logic]\n#[ensures(model::mp(b_%s()) == Ok(ms_%s()))]\npub fn p_%s() {\n" % (i, i, i))
    o.append("    blen_%s();\n    m_%s();\n    h_%s();\n" % (i, i, i))
    for t in range(len(ents)):
        off = ents[t][1] - 8
        o.append("    q_%s_%d();\n    k_%s_%d();\n" % (i, off, i, off))
    for t in range(len(ents)):
        off = ents[t][1] - 8
        o.append("    model::step_acc(b_%s(), %d, %d, %s);\n" % (i, off, L, acc(t, i)))
    o.append("    model::loop_done(b_%s(), %d, %d, %s);\n" % (i, L, L, acc(len(ents), i)))
    o.append("}\n\n")
    return b, ents


def chfn(i, k):
    return "ch_%s_%d" % (i, k)


def chtab(i, k):
    return "ct_%s_%d" % (i, k)


def emit_channel_all(o, i, k, pl, n, packets, endcur, startcur, fail=False, err=None):
    """Emit `ct_<i>_<k>` (the finished buffer table) and `ch_<i>_<k>` (the mr_ch result).

    For a failing channel, `packets` holds the packets that succeeded before the failing one;
    the buffer those wrote is `ct_<i>_<k>` restricted to their total length, so the step
    functions of the failing channel read a prefix buffer.
    """
    # For a failing channel, `packets` holds the packets that succeeded before the failing one;
    # the buffer they wrote, zero-padded to `n`, is the reference `ct_` buffer for the steps.
    rf = chbytes(packets, sum(p["count"] for p in packets)) if fail else chbytes(packets, n)
    rf = rf + bytes(n - len(rf))
    o.append("#[logic(open)]\npub fn %s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n"
             % (chtab(i, k), table(n, runs_of(rf))))
    # per-packet read facts
    cur, prod = startcur, 0
    for j, p in enumerate(packets):
        o.append("#[logic]\n#[ensures(pl_%s()[%d]@ == %d)]\npub fn rd_%s_%d_%d() {}\n\n"
                 % (i, cur, p["ctrl"], i, k, j))
        if p["lit"]:
            for b2 in range(p["count"]):
                o.append("#[logic]\n#[ensures(pl_%s()[%d]@ == %s()[%d]@)]\npub fn rb_%s_%d_%d_%d() {}\n\n"
                         % (i, cur + 1 + b2, chtab(i, k), prod + b2, i, k, j, b2))
        else:
            o.append("#[logic]\n#[ensures(pl_%s()[%d]@ == %d)]\npub fn rv_%s_%d_%d() {}\n\n"
                     % (i, cur + 1, p["val"], i, k, j))
            o.append("#[logic]\n#[ensures(forall<q: Int> %d <= q && q < %d ==> %s()[q]@ == %d)]\n"
                     "pub fn rr_%s_%d_%d() {}\n\n" % (prod, prod + p["count"], chtab(i, k), p["val"], i, k, j))
        cur = p["next"]
        prod += p["count"]
    # one step function per packet
    cur, prod = startcur, 0
    for j, p in enumerate(packets):
        name = "st_%s_%d_%d" % (i, k, j)
        o.append("#[logic]\n#[requires(0 <= prod && prod + %d <= %d)]\n" % (p["count"], n))
        o.append("#[requires(0 <= cur && cur + 1 < pl_%s().len())]\n" % i)
        o.append("#[requires(pl_%s()[cur]@ == %d)]\n" % (i, p["ctrl"]))
        if p["lit"]:
            o.append("#[requires(pl_%s()[cur]@ < 128)]\n" % i)
            o.append("#[requires(forall<i2: Int> 0 <= i2 && i2 < %d ==> pl_%s()[cur + 1 + i2]@ == %s()[prod + i2]@)]\n"
                     % (p["count"], i, chtab(i, k)))
            o.append("#[ensures(rmodel::mr_ch(pl_%s(), cur, prod, %d, part(%s(), %d, prod))"
                     " == rmodel::mr_ch(pl_%s(), cur + 1 + %d, prod + %d, %d, part(%s(), %d, prod + %d)))]\n"
                     % (i, n, chtab(i, k), n, i, p["count"], p["count"], n, chtab(i, k), n, p["count"]))
            o.append("pub fn %s(cur: Int, prod: Int) {\n    step_lit(pl_%s(), cur, prod, %d, %d, %s());\n}\n\n"
                     % (name, i, p["count"], n, chtab(i, k)))
        else:
            o.append("#[requires(!(pl_%s()[cur]@ < 128))]\n" % i)
            o.append("#[requires(pl_%s()[cur]@ - 125 == %d)]\n" % (i, p["count"]))
            o.append("#[requires(pl_%s()[cur + 1]@ == %d)]\n" % (i, p["val"]))
            o.append("#[requires(forall<i2: Int> prod <= i2 && i2 < prod + %d ==> %s()[i2]@ == %d)]\n"
                     % (p["count"], chtab(i, k), p["val"]))
            o.append("#[ensures(rmodel::mr_ch(pl_%s(), cur, prod, %d, part(%s(), %d, prod))"
                     " == rmodel::mr_ch(pl_%s(), cur + 2, prod + %d, %d, part(%s(), %d, prod + %d)))]\n"
                     % (i, n, chtab(i, k), n, i, p["count"], n, chtab(i, k), n, p["count"]))
            o.append("pub fn %s(cur: Int, prod: Int) {\n    step_rep(pl_%s(), cur, prod, %d, %d, %du8, %s());\n}\n\n"
                     % (name, i, p["count"], n, p["val"], chtab(i, k)))
        cur = p["next"]
        prod += p["count"]
    # the chain
    if fail:
        o.append("#[logic]\n#[ensures(rmodel::mr_ch(pl_%s(), %d, 0, %d, zero_buf(%d)) == Err(Error::%s))]\n"
                 "pub fn %s() {\n" % (i, startcur, n, n, err, chfn(i, k)))
        o.append("    proof_assert!(zero_buf(%d) == part(%s(), %d, 0));\n" % (n, chtab(i, k), n))
    else:
        o.append("#[logic]\n#[ensures(rmodel::mr_ch(pl_%s(), %d, 0, %d, zero_buf(%d)) == Ok((%s(), %d)))]\n"
                 "pub fn %s() {\n" % (i, startcur, n, n, chtab(i, k), endcur, chfn(i, k)))
        o.append("    proof_assert!(zero_buf(%d) == part(%s(), %d, 0));\n" % (n, chtab(i, k), n))
    cur, prod = startcur, 0
    for j, p in enumerate(packets):
        o.append("    rd_%s_%d_%d();\n" % (i, k, j))
        if p["lit"]:
            for b2 in range(p["count"]):
                o.append("    rb_%s_%d_%d_%d();\n" % (i, k, j, b2))
        else:
            o.append("    rv_%s_%d_%d();\n    rr_%s_%d_%d();\n" % (i, k, j, i, k, j))
        o.append("    st_%s_%d_%d(%d, %d);\n" % (i, k, j, cur, prod))
        cur = p["next"]
        prod += p["count"]
    if fail:
        # the terminating error lemma for the failing packet, at `(cur, prod)`
        lem = ("rmodel::ch_trunc" if endcur >= len(pl) else
               "rmodel::ch_over" if err == "RleOverrun" else "rmodel::ch_data")
        o.append("    %s(pl_%s(), %d, %d, %d, part(%s(), %d, %d));\n"
                 % (lem, i, endcur, prod, n, chtab(i, k), n, prod))
    if not fail:
        o.append("    proof_assert!(part(%s(), %d, %d) == %s());\n" % (chtab(i, k), n, prod, chtab(i, k)))
        o.append("    partn(%s(), %d);\n" % (chtab(i, k), n))
        o.append("    done(pl_%s(), %d, %d, %s());\n" % (i, endcur, n, chtab(i, k)))
    o.append("}\n\n")


def emit_rgbish(o, v, is_mask):
    i = ident(v["name"])
    b, ents = emit_b_and_ms(o, v, i)
    exp = v["expected"]
    idx = v["mask_index"] if is_mask else v["color_index"]
    model = "ma_model" if is_mask else "mc_model"
    if "error" in exp:
        emit_decode_err(o, v, i, b, ents, idx, exp["error"], is_mask)
        return
    ty, s, e = ents[idx]
    d = exp["side"]
    if is_mask:
        alpha = expand_runs(exp["alpha_runs"], d * d)
        o.append("#[logic(open)]\npub fn out_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n" % (i, alpha))
        o.append("#[logic]\n#[ensures(crate::ma_entry(b_%s(), ms_%s()[%d]) == Ok((%d, %d, out_%s())))]\n"
                 "pub fn pay_%s() {\n"
                 "    proof_assert!(crate::legacy_mask_model(b_%s().subsequence(%d, %d)) == Some(%du32));\n"
                 "    proof_assert!(b_%s().subsequence(%d, %d) == out_%s());\n}\n\n"
                 % (i, i, idx, d, d, i, i, i, s - 8, s - 4, d, i, s, e, i))
        o.append("#[logic]\n#[ensures(%s(b_%s(), ms_%s(), %d) == Ok((%d, %d, out_%s())))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n"
                 "    proof_assert!(ms_%s().len() > %d);\n"
                 "    proof_assert!(%s(b_%s(), ms_%s(), %d) == crate::ma_entry(b_%s(), ms_%s()[%d]));\n"
                 "    pay_%s();\n}\n\n"
                 % (model, i, i, idx, d, d, i, i, i, i, idx, model, i, i, idx, i, i, idx, i))
        return
    emit_colour(o, v, i, b, ents, idx, d, exp)


def emit_colour_err(o, v, i, b, ents, idx, d):
    return emit_colour(o, v, i, b, ents, idx, d, {"error": v["expected"]["error"]})


def emit_colour(o, v, i, b, ents, idx, d, exp, final_agreement=True):
    ty, s, e = ents[idx]
    pl = b[s:e]
    if ty == b"it32" and pl[:4] == b"\0\0\0\0":
        pl = pl[4:]
    n = d * d
    o.append("#[logic(open)]\npub fn pl_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n" % (i, seqlit(pl)))
    o.append("#[logic]\n#[ensures(crate::mentry_type(b_%s(), ms_%s()[%d]) == b_%s().subsequence(%d, %d))]\n"
             "pub fn mt_%s() {}\n\n" % (i, i, idx, i, s - 8, s - 4, i))
    if ty == b"it32" and b[s:e][:4] == b"\0\0\0\0":
        o.append("#[logic]\n#[ensures(crate::effective_payload(b_%s(), ms_%s()[%d]) == pl_%s())]\n"
                 "pub fn eff_%s() {\n    mt_%s();\n"
                 "    proof_assert!(b_%s().subsequence(%d, %d) == pl_%s());\n}\n\n"
                 % (i, i, idx, i, i, i, i, s + 4, e, i))
    else:
        o.append("#[logic]\n#[ensures(crate::effective_payload(b_%s(), ms_%s()[%d]) == pl_%s())]\n"
                 "pub fn eff_%s() {\n    mt_%s();\n"
                 "    proof_assert!(b_%s().subsequence(%d, %d) == pl_%s());\n}\n\n"
                 % (i, i, idx, i, i, i, i, s, e, i))
    # channels
    cur, ok = 0, True
    chans = []
    for k in range(3):
        r = chan(pl, cur, n)
        chans.append(r)
        if r[0] == "Err":
            ok = False
            break
        cur = r[2]
    if ok and cur != len(pl):
        if exp.get("error") != "RleTrailingBytes":
            raise ValueError("fixture RLE error disagrees with model for " + v["name"])
        for k, r in enumerate(chans):
            emit_channel_all(o, i, k, pl, n, r[1], r[2], 0 if k == 0 else chans[k - 1][2])
        o.append("#[logic]\n#[ensures(mr_model(pl_%s(), %d) == Err(Error::RleTrailingBytes))]\npub fn mr_%s() {\n" % (i, n, i))
        for k in range(3):
            o.append("    %s();\n" % chfn(i, k))
        o.append("    crate::mr_trailing(pl_%s(), %d, %s(), %s(), %s(), %d, %d, %d);\n}\n\n"
                 % (i, n, chtab(i, 0), chtab(i, 1), chtab(i, 2), chans[0][2], chans[1][2], chans[2][2]))
        o.append("#[logic]\n#[ensures(mc_entry(b_%s(), ms_%s()[%d]) == Err(Error::RleTrailingBytes))]\n"
                 "pub fn msg_%s() {\n    eff_%s();\n    mr_%s();\n"
                 "    proof_assert!(crate::legacy_colour_model(crate::mentry_type(b_%s(), ms_%s()[%d])) == Some(%du32));\n}\n\n"
                 % (i, i, idx, i, i, i, i, i, idx, d))
        o.append("#[logic]\n#[ensures(mc_model(b_%s(), ms_%s(), %d) == Err(Error::RleTrailingBytes))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n    proof_assert!(ms_%s().len() > %d);\n"
                 "    proof_assert!(mc_model(b_%s(), ms_%s(), %d) == mc_entry(b_%s(), ms_%s()[%d]));\n"
                 "    msg_%s();\n}\n\n"
                 % (i, i, idx, i, i, i, idx, i, i, idx, i, i, idx, i))
        return
    if not ok:
        k = len(chans) - 1
        r = chans[k]
        if exp.get("error") != r[1]:
            raise ValueError("fixture RLE error disagrees with model for " + v["name"])
        for j in range(k):
            emit_channel_all(o, i, j, pl, n, chans[j][1], chans[j][2], 0 if j == 0 else chans[j - 1][2])
        emit_channel_all(o, i, k, pl, n, r[2], r[3], 0 if k == 0 else chans[k - 1][2],
                         fail=True, err=r[1])
        o.append("#[logic]\n#[ensures(mr_model(pl_%s(), %d) == Err(Error::%s))]\npub fn mr_%s() {\n"
                 % (i, n, r[1], i))
        c = 0
        for j in range(k):
            o.append("    %s();\n" % chfn(i, j))
            c = chans[j][2]
        o.append("    %s();\n}\n\n" % chfn(i, k))
        o.append("#[logic]\n#[ensures(crate::mc_entry(b_%s(), ms_%s()[%d]) == Err(Error::%s))]\n"
                 "pub fn msg_%s() {\n    eff_%s();\n    mr_%s();\n"
                 "    proof_assert!(crate::legacy_colour_model(crate::mentry_type(b_%s(), ms_%s()[%d])) == Some(%du32));\n}\n\n"
                 % (i, i, idx, r[1], i, i, i, i, i, idx, d))
        o.append("#[logic]\n#[ensures(mc_model(b_%s(), ms_%s(), %d) == Err(Error::%s))]\n"          # 4
                 "pub fn agree_%s() {\n    p_%s();\n    proof_assert!(ms_%s().len() > %d);\n"          # 4
                 "    proof_assert!(mc_model(b_%s(), ms_%s(), %d) == mc_entry(b_%s(), ms_%s()[%d]));\n" # 6
                 "    msg_%s();\n}\n\n"                                                                # 1
                 % (i, i, idx, exp["error"],   # 4
                    i, i, i, idx,       # 4
                    i, i, idx, i, i, idx,  # 6
                    i))                 # 1
        return
    for k, r in enumerate(chans):
        emit_channel_all(o, i, k, pl, n, r[1], r[2], 0 if k == 0 else chans[k - 1][2])
    o.append("#[logic(open)]\npub fn rgb_%s() -> Seq<u8> {\n"
             "    pearlite! { Seq::create(%d, |i: Int|\n"
             "        if i %% 3 == 0 { %s()[i / 3] } else if i %% 3 == 1 { %s()[i / 3] } else { %s()[i / 3] }) }\n}\n\n"
             % (i, 3 * n, chtab(i, 0), chtab(i, 1), chtab(i, 2)))
    o.append("#[logic]\n#[requires(mr_model(pl_%s(), %d) == Ok((%s(), %s(), %s())))]\n"
             "#[ensures(interleave3(%s(), %s(), %s(), %d) == rgb_%s())]\n"
             "pub fn ilv_%s() {\n"
             "    interleave_eq(%s(), %s(), %s(), %d, rgb_%s());\n"
             "    proof_assert!(forall<i2: Int> 0 <= i2 && i2 < %d ==> rgb_%s()[i2]@",
             )
    o[-1] = ("#[logic]\n#[requires(mr_model(pl_%s(), %d) == Ok((%s(), %s(), %s())))]\n"
             "#[ensures(interleave3(%s(), %s(), %s(), %d) == rgb_%s())]\n"
             "pub fn ilv_%s() {\n"
             "    proof_assert!(interleave3(%s(), %s(), %s(), %d).ext_eq(rgb_%s()));\n}\n\n"
             % (i, n, chtab(i, 0), chtab(i, 1), chtab(i, 2),
                chtab(i, 0), chtab(i, 1), chtab(i, 2), n, i, i,
                chtab(i, 0), chtab(i, 1), chtab(i, 2), n, i))
    o.append("#[logic]\n#[ensures(mr_model(pl_%s(), %d) == Ok((%s(), %s(), %s())))]\npub fn mr_%s() {\n"
             % (i, n, chtab(i, 0), chtab(i, 1), chtab(i, 2), i))
    o.append("    %s();\n    %s();\n    %s();\n" % (chfn(i, 0), chfn(i, 1), chfn(i, 2)))
    o.append("    mr_three(pl_%s(), %d, %s(), %s(), %s(), %d, %d, %d);\n}\n\n"
             % (i, n, chtab(i, 0), chtab(i, 1), chtab(i, 2), chans[0][2], chans[1][2], chans[2][2]))
    o.append("#[logic]\n#[ensures(crate::mc_entry(b_%s(), ms_%s()[%d]) == Ok((%d, %d, rgb_%s())))]\n"
             "pub fn msg_%s() {\n    eff_%s();\n"
             "    proof_assert!(crate::legacy_colour_model(crate::mentry_type(b_%s(), ms_%s()[%d])) == Some(%du32));\n"
             "    mr_%s();\n    ilv_%s();\n"
             "    crate::mc_entry_ok(b_%s(), ms_%s()[%d], %du32, pl_%s(), %s(), %s(), %s(), rgb_%s());\n}\n\n"
             % (i, i, idx, d, d, i, i, i, i, i, idx, d, i, i,
                i, i, idx, d, i, chtab(i, 0), chtab(i, 1), chtab(i, 2), i))
    if not final_agreement:
        return
    expected_rgb = expand_runs(exp["rgb_runs"], 3 * n)
    o.append("#[logic(open)]\npub fn expected_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n"
             % (i, expected_rgb))
    o.append("#[logic]\n#[ensures(rgb_%s() == expected_%s())]\n"
             "pub fn rgb_expected_%s() {\n    proof_assert!(rgb_%s().ext_eq(expected_%s()));\n}\n\n"
             % (i, i, i, i, i))
    o.append("#[logic]\n#[ensures(mc_model(b_%s(), ms_%s(), %d) == Ok((%d, %d, expected_%s())))]\n"
             "pub fn agree_%s() {\n    p_%s();\n    proof_assert!(ms_%s().len() > %d);\n"
             "    proof_assert!(mc_model(b_%s(), ms_%s(), %d) == mc_entry(b_%s(), ms_%s()[%d]));\n"
             "    msg_%s();\n    rgb_expected_%s();\n}\n\n"
             % (i, i, idx, d, d, i,
                i, i, i, idx,
                i, i, idx, i, i, idx,
                i, i))


def emit_decode_err(o, v, i, b, ents, idx, err, is_mask):
    model = "ma_model" if is_mask else "mc_model"
    pred = "crate::legacy_mask_model" if is_mask else "crate::legacy_colour_model"
    if err == "IndexOutOfRange":
        o.append("#[logic]\n#[ensures(%s(b_%s(), ms_%s(), %d) == Err(Error::IndexOutOfRange))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n    proof_assert!(ms_%s().len() <= %d);\n}\n\n"
                 % (model, i, i, idx, i, i, i, idx))
        return
    ty, s, e = ents[idx]
    if err == "IndexOutOfRange":
        o.append("#[logic]\n#[ensures(%s(b_%s(), ms_%s(), %d) == Err(Error::IndexOutOfRange))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n"
                 "    proof_assert!(ms_%s().len() <= %d);\n}\n\n"
                 % (model, i, i, idx, i, i, i, idx))
        return
    if err == "UnsupportedType":
        o.append("#[logic]\n#[ensures(%s(b_%s().subsequence(%d, %d)) == None)]\npub fn ut_%s() {}\n\n"
                 % (pred, i, s - 8, s - 4, i))
        o.append("#[logic]\n#[ensures(%s(b_%s(), ms_%s(), %d) == Err(Error::UnsupportedType))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n"
                 "    proof_assert!(crate::mentry_type(b_%s(), ms_%s()[%d]) == b_%s().subsequence(%d, %d));\n"
                 "    ut_%s();\n}\n\n"
                 % (model, i, i, idx, i, i, i, i, idx, i, s - 8, s - 4, i))
        return
    if err == "InvalidMaskLength":
        o.append("#[logic]\n#[ensures(b_%s().subsequence(%d, %d).len() != %d)]\npub fn iml_%s() {}\n\n"
                 % (i, s, e, MASK[ty] ** 2, i))
        o.append("#[logic]\n#[ensures(ma_model(b_%s(), ms_%s(), %d) == Err(Error::InvalidMaskLength))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n"
                 "    proof_assert!(crate::legacy_mask_model(b_%s().subsequence(%d, %d)) == Some(%du32));\n"
                 "    iml_%s();\n}\n\n"
                 % (i, i, idx, i, i, i, s - 8, s - 4, MASK[ty], i))
        return
    # MR error inside the colour payload
    emit_colour_err(o, v, i, b, ents, idx, COLOUR[ty])


def expand_runs(runs, total):
    """A `Seq::create(total, |k| ...)` for the fixture's run list, on the expanded bytes."""
    data, pos = [], 0
    for r in runs:
        pt = bytes.fromhex(r["hex"])
        data.extend(list(pt) * r["count"])
        pos += len(pt) * r["count"]
    assert pos == total, (pos, total)
    return table_data(total, data)


def emit_combine(v, o):
    i = ident(v["name"])
    b, ents = emit_b_and_ms(o, v, i)
    exp = v["expected"]
    ci, mi = v.get("color_index"), v.get("mask_index")
    cis = "None" if ci is None else "Some(%du32)" % ci
    mis = "None" if mi is None else "Some(%du32)" % mi
    if ci is None or mi is None or not ents:
        err = exp["error"]
        o.append("#[logic]\n#[ensures(mj_model(b_%s(), ms_%s(), %s, %s) == Err(Error::%s))]\n"
                 "pub fn agree_%s() {\n    p_%s();\n}\n\n" % (i, i, cis, mis, err, i, i))
        return
    colour_side = COLOUR[ents[ci][0]]
    if "error" not in exp and exp["side"] != colour_side:
        raise ValueError("fixture combine side disagrees with colour model for " + v["name"])
    emit_colour(o, v, i, b, ents, ci, colour_side, exp, final_agreement=False)
    ty, start, end = ents[mi]
    if ty in MASK:
        mask_side = MASK[ty]
        if end - start == mask_side * mask_side:
            o.append("#[logic(open)]\npub fn alpha_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n"
                     % (i, table_data(end-start, b[start:end])))
            mask_result = "Ok((%d, %d, alpha_%s()))" % (mask_side, mask_side, i)
            body = ("    proof_assert!(crate::legacy_mask_model(b_%s().subsequence(%d, %d)) == Some(%du32));\n"
                    "    proof_assert!(b_%s().subsequence(%d, %d) == alpha_%s());\n"
                    % (i, start-8, start-4, mask_side, i, start, end, i))
        else:
            raise ValueError("unexpected invalid combine mask")
    else:
        mask_result = "Err(Error::UnsupportedType)"
        body = ("    proof_assert!(crate::legacy_mask_model(b_%s().subsequence(%d, %d)) == None);\n"
                % (i, start-8, start-4))
    o.append("#[logic]\n#[ensures(crate::ma_entry(b_%s(), ms_%s()[%d]) == %s)]\n"
             "pub fn pay_%s() {\n%s}\n\n" % (i, i, mi, mask_result, i, body))
    pair_result = "Err(Error::%s)" % exp["error"] if "error" in exp else \
                  "Ok((%d, %d, out_%s()))" % (exp["side"], exp["side"], i)
    if "error" not in exp:
        rgba = expand_runs(exp["rgba_runs"], 4 * colour_side * colour_side)
        o.append("#[logic(open)]\npub fn out_%s() -> Seq<u8> {\n    pearlite! { %s }\n}\n\n" % (i, rgba))
        o.append("#[logic]\n#[ensures(rgba4(rgb_%s(), alpha_%s(), %d) == out_%s())]\n"
                 "pub fn rgba_%s() {\n    proof_assert!(rgba4(rgb_%s(), alpha_%s(), %d).ext_eq(out_%s()));\n}\n\n"
                 % (i, i, colour_side * colour_side, i, i, i, i, colour_side * colour_side, i))
    o.append("#[logic]\n#[ensures(crate::mj_pair(b_%s(), ms_%s(), %d, %d) == %s)]\n"
             "pub fn pair_%s() {\n    msg_%s();\n    pay_%s();\n%s}\n\n"
             % (i, i, ci, mi, pair_result, i, i, i,
                "    rgba_%s();\n" % i if "error" not in exp else ""))
    o.append("#[logic]\n#[ensures(mj_model(b_%s(), ms_%s(), %s, %s) == %s)]\n"
             "pub fn agree_%s() {\n    p_%s();\n    pair_%s();\n}\n\n"
             % (i, i, cis, mis, pair_result, i, i, i))


def gen(only=None):
    vectors = json.load(open(VECTORS))
    small = [v for v in vectors if len(expanded(v)) <= LIMIT]
    large = [v for v in vectors if len(expanded(v)) > LIMIT]
    for v in large:
        if v["name"] not in LARGE:
            raise SystemExit("large vector %r has no lemma mapping" % v["name"])
    o = [HEADER]
    o.append("// 111 vectors: %d at or below %d bytes, %d above.\n" % (len(small), LIMIT, len(large)))
    o.append("// The nine large vectors and the lemma covering each:\n")
    for v in large:
        o.append("//   %-46s %8d bytes  -> %s\n" % (v["name"], len(expanded(v)), LARGE[v["name"]]))
    o.append("\n")
    n = {}
    only_name = os.environ.get("ONLY_NAME")
    for v in small:
        if only_name and v["name"] != only_name:
            continue
        op = v["operation"]
        n[op] = n.get(op, 0) + 1
        if only and op != only:
            continue
        emit_parse(v, o) if op == "parse" else emit_decode(v, o)
    return "".join(o), n, small, large


def main():
    args = sys.argv[1:]
    only = args[args.index("--only") + 1] if "--only" in args else None
    out = args[args.index("--out") + 1] if "--out" in args else DEFAULT_OUT
    text, n, small, large = gen(only)
    print("counts:", n, "small:", len(small), "large:", len(large))
    if "--check" in args:
        old = open(out).read() if os.path.exists(out) else ""
        print("up to date" if old == text else "STALE")
        return 0 if old == text else 1
    open(out, "w").write(text)
    print("wrote %s (%d bytes, %d obligations)"
          % (out, len(text), text.count("pub fn agree_")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
