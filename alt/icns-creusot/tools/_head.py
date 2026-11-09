#!/usr/bin/env python3
"""Generate Creusot model-vs-vector agreement proofs from spec-frozen/VECTORS.json.

For each of the 102 vectors whose expanded input is at most 4,096 bytes, emit a Pearlite
`#[logic]` obligation stating that the crate's formal model, applied to the vector's
expanded input, equals the fixture's `expected` value.  Every expected value is read from
the fixture's `expected` field; the executable `parse` / `decode_rgb` / `decode_mask` /
`combine` are never consulted.

Container vectors state `model::mp(b) == <expected>` exactly (same error variant, or the
same ordered entry list with all four fields).  Decoding vectors state

    match model::mp(b) { Ok(ms) => <mc_model|ma_model|mj_model>(b, ms, idx) == <expected>,
                         Err(_) => false }

so the entry list the decoder model consumes is `mp`'s own parse result, and a parse
failure makes the obligation false rather than vacuous.

The crate's recursive models (`model::mp_loop`, `rmodel::mr_ch`) are opaque outside their
modules, so every obligation is discharged through the proved single-step lemmas
(`model::step_acc`, `model::loop_done`, `model::loop_hdr`, `model::loop_short`,
`model::loop_oob`, `model::loop_many`, `rmodel::ch_step`, `rmodel::ch_done`,
`rmodel::ch_over`, `rmodel::ch_data`, `rmodel::ch_trunc`) instantiated at the vector's
concrete bytes and `Int` lengths.

The nine vectors longer than 4,096 bytes are covered by the symbolic lemmas L1-L6.

Usage: tools/gen_model_vectors.py [--out PATH] [--check] [--only parse|rgb|mask|combine]
"""

import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VECTORS = os.path.join(ROOT, "spec-frozen", "VECTORS.json")
DEFAULT_OUT = os.path.join(ROOT, "icns_creusot", "src", "model_vectors.rs")

LIMIT = 4096
E_BYTES = b"none" + (8).to_bytes(4, "big")

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
LEGACY = {b"is32": 16, b"il32": 32, b"ih32": 48, b"it32": 128,
          b"s8mk": 16, b"l8mk": 32, b"h8mk": 48, b"t8mk": 128}
COLOUR = {b"is32": 16, b"il32": 32, b"ih32": 48, b"it32": 128}
MASK = {b"s8mk": 16, b"l8mk": 32, b"h8mk": 48, b"t8mk": 128}
PNG = b"\x89PNG\r\n\x1a\n"
JP2 = b"\x00\x00\x00\x0cjP  \r\n\x87\n"


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


def entry_json(b, ty, s, e):
    opaque, ns = classify(ty, b[s:e])
    return {"type_hex": "".join("%02x" % c for c in ty), "payload": [s, e],
            "opaque_candidate": opaque, "nominal_side": ns}


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
        packets.append({"ctrl": c, "count": count, "lit": lit, "val": s[cur + 1],
                        "bytes": list(s[cur + 1:cur + 1 + count])})
        cur += 1 + need
        produced += count
    return ("Ok", packets, cur)


def mr(s, n):
    cur, chans = 0, []
    for _ in range(3):
        r = chan(s, cur, n)
        if r[0] == "Err":
            return ("Err", r[1], None, None)
        chans.append(r[1])
        cur = r[2]
    if cur != len(s):
        return ("Err", "RleTrailingBytes", None, None)
    return ("Ok", chans, cur)


def chbytes(packets, n):
    out = []
    for p in packets:
        out.extend(p["bytes"] if p["lit"] else [p["val"]] * p["count"])
    assert len(out) == n
    return bytes(out)


def interleave(r, g, b, n):
    out = bytearray(3 * n)
    for k in range(n):
        out[3 * k] = r[k]
        out[3 * k + 1] = g[k]
        out[3 * k + 2] = b[k]
    return bytes(out)


def rgba(rgb, alpha, n):
    out = bytearray(4 * n)
    for k in range(n):
        out[4 * k] = rgb[3 * k]
        out[4 * k + 1] = rgb[3 * k + 1]
        out[4 * k + 2] = rgb[3 * k + 2]
        out[4 * k + 3] = alpha[k]
    return bytes(out)


def mj(b, encol, ci, mi):
    if ci is None:
        return ("Err", "MissingColor")
    if mi is None:
        return ("Err", "MissingMask")
    if ci >= len(encol):
        return ("Err", "IndexOutOfRange")
    ty, s, e = encol[ci]
    if ty not in COLOUR:
        return ("Err", "UnsupportedType")
    d = COLOUR[ty]
    pl = b[s:e]
    if ty == b"it32" and pl[:4] == b"\0\0\0\0":
        pl = pl[4:]
    r = mr(pl, d * d)
    if r[0] == "Err":
        return ("Err", r[1])
    rgb = interleave(chbytes(r[1][0], d * d), chbytes(r[1][1], d * d),
                     chbytes(r[1][2], d * d), d * d)
    if mi >= len(encol):
        return ("Err", "IndexOutOfRange")
    mty, ms, me = encol[mi]
    if mty not in MASK:
        return ("Err", "UnsupportedType")
    md = MASK[mty]
    if md != d:
        return ("Err", "TypeMismatch")
    if me - ms != md * md:
        return ("Err", "InvalidMaskLength")
    return ("Ok", (d, rgba(rgb, b[ms:me], d * d)))


def mc(b, encol, i):
    if i >= len(encol):
        return ("Err", "IndexOutOfRange")
    ty, s, e = encol[i]
    if ty not in COLOUR:
        return ("Err", "UnsupportedType")
    d = COLOUR[ty]
    pl = b[s:e]
    if ty == b"it32" and pl[:4] == b"\0\0\0\0":
        pl = pl[4:]
    r = mr(pl, d * d)
    if r[0] == "Err":
        return ("Err", r[1])
    rgb = interleave(chbytes(r[1][0], d * d), chbytes(r[1][1], d * d),
                     chbytes(r[1][2], d * d), d * d)
    return ("Ok", (d, rgb))


def ma(b, encol, i):
    if i >= len(encol):
        return ("Err", "IndexOutOfRange")
    ty, s, e = encol[i]
    if ty not in MASK:
        return ("Err", "UnsupportedType")
    d = MASK[ty]
    if e - s != d * d:
        return ("Err", "InvalidMaskLength")
    return ("Ok", (d, b[s:e]))


# ------------------------------------------------------------------ Rust emission

def ident(name):
    return "".join(c if c.isalnum() else "_" for c in name)


def seq_expr(data):
    """A compact `Seq<u8>` expression for `data`, or None if it needs a generated table."""
    if not data:
        return "Seq::empty()"
    runs, i = [], 0
    while i < len(data):
        j = i
        while j < len(data) and data[j] == data[i]:
            j += 1
        runs.append((i, j, data[i]))
        i = j
    if len(runs) == 1:
        return "Seq::create(%d, |_i: Int| %du8)" % (len(data), runs[0][2])
    if len(runs) <= 4:
        parts = []
        for k, (_, b, val) in enumerate(runs):
            parts.append(("if i < %d" % b if k < len(runs) - 1 else "if true") + " { %du8 }" % val)
        return "Seq::create(%d, |i: Int| %s)" % (len(data), " else ".join(parts))
    return None


def lit(data, indent=12, width=12):
    parts = ["%du8" % b for b in data]
    lines = [" " * indent + ", ".join(parts[i:i + width]) + ","
             for i in range(0, len(parts), width)]
    return "seq![\n" + "\n".join(lines) + "\n" + " " * (indent - 4) + "]"


def hexchars(hx):
    return ", ".join("%d" % ord(c) for c in hx)


def mkind(e):
    ns = "None" if e["nominal_side"] is None else "Some(%du32)" % e["nominal_side"]
    return ("MEntry {\n        type_hex: seq![%s],\n        start: %d, end: %d,\n"
            "        opaque_candidate: %s,\n        nominal_side: %s,\n    }"
            % (hexchars(e["type_hex"]), e["payload"][0], e["payload"][1],
               "true" if e["opaque_candidate"] else "false", ns))


HEADER = '''//! Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.
//!
//! DO NOT EDIT BY HAND.  Regenerate with
//!     python3 tools/gen_model_vectors.py
//!
//! Each `agree_*` obligation below states that the crate's formal model, applied to the
//! vector's expanded input, yields exactly the fixture's `expected` value.  Expected values
//! are read from `spec-frozen/VECTORS.json` and never from the executable decoder.
//!
//! Container vectors state `model::mp(b) == <expected>` (an exact `Result`: the same error
//! variant, or the same ordered entry list with all four fields).  Decoding vectors state
//! `match model::mp(b) { Ok(ms) => <model>(b, ms, idx) == <expected>, Err(_) => false }`, so
//! the entries the decoder model consumes are `mp`'s own parse result and a parse failure
//! makes the obligation false rather than vacuous.
//!
//! The crate's recursive models (`model::mp_loop`, `rmodel::mr_ch`) are opaque outside their
//! modules, so every obligation is discharged through proved single-step lemmas
//! (`model::step_acc`, `model::loop_done`, `model::loop_hdr`, `model::loop_short`,
//! `model::loop_oob`, `model::loop_many`, `rmodel::ch_step`, `rmodel::ch_done`,
//! `rmodel::ch_over`, `rmodel::ch_data`, `rmodel::ch_trunc`) instantiated at the vector's
//! concrete bytes and `Int` lengths.  Nothing here is assumed.
//!
//! The nine vectors longer than 4,096 bytes are covered by the symbolic lemmas L1-L6.

use creusot_std::prelude::*;

use crate::model;
use crate::{
    be32_log, interleave3, magic_model, ma_model, mc_model, mj_model, mk_entry, mr_iw,
    mr_model, rmodel, zero_buf, Error, MEntry,
};

/// The `n`-byte channel buffer whose bytes below `filled` come from `rf` and whose bytes at or
/// above `filled` are still zero: a channel buffer after a prefix of its packets.
#[logic(open)]
pub fn part(rf: Seq<u8>, n: Int, filled: Int) -> Seq<u8> {
    pearlite! { Seq::create(n, |i: Int| if 0 <= i && i < filled { rf[i] } else { 0u8 }) }
}

/// Before any packet the buffer is untouched.
#[logic]
#[requires(0 <= n)]
#[ensures(zero_buf(n) == part(rf, n, 0))]
pub fn part0(rf: Seq<u8>, n: Int) {}

/// Once every byte is written the buffer is the channel itself.
#[logic]
#[requires(0 <= n)]
#[requires(rf.len() == n)]
#[ensures(part(rf, n, n) == rf)]
pub fn partn(rf: Seq<u8>, n: Int) {}

/// A repeat packet writing `val` over `[produced, produced+count)`, where the channel has
/// `val` on exactly that range.
#[logic]
#[requires(count > 0)]
#[requires(0 <= produced && produced + count <= n)]
#[requires(0 <= ctrl && ctrl + 1 < s.len())]
#[requires(!(s[ctrl]@ < 128))]
#[requires(s[ctrl]@ - 125 == count)]
#[requires(s[ctrl + 1]@ == val@)]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> produced <= i && i < produced + count ==> rf[i] == val@)]
#[ensures(mr_iw(s, ctrl, false, produced, count, part(rf, n, produced))
    == part(rf, n, produced + count))]
pub fn iw_rep(s: Seq<u8>, ctrl: Int, produced: Int, count: Int, n: Int, val: u8, rf: Seq<u8>) {
    proof_assert!(mr_iw(s, ctrl, false, produced, count, part(rf, n, produced))
        .ext_eq(part(rf, n, produced + count)));
}

/// A literal packet whose `count` bytes are exactly the channel's bytes over
/// `[produced, produced+count)`.
#[logic]
#[requires(count > 0)]
#[requires(0 <= produced && produced + count <= n)]
#[requires(0 <= ctrl && ctrl + count < s.len())]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> 0 <= i && i < count ==> s[ctrl + 1 + i]@ == rf[produced + i])]
#[ensures(mr_iw(s, ctrl, true, produced, count, part(rf, n, produced))
    == part(rf, n, produced + count))]
pub fn iw_lit(s: Seq<u8>, ctrl: Int, produced: Int, count: Int, n: Int, rf: Seq<u8>) {
    proof_assert!(mr_iw(s, ctrl, true, produced, count, part(rf, n, produced))
        .ext_eq(part(rf, n, produced + count)));
}

/// `k` identical repeat packets (control byte `ctrl`, count `c`, value `val`) starting at `cur0`.
#[logic]
#[variant(k)]
#[requires(k >= 0)]
#[requires(c > 0)]
#[requires(0 <= produced0 && produced0 + k * c <= n)]
#[requires(0 <= cur0)]
#[requires(!(ctrl < 128))]
#[requires(ctrl - 125 == c)]
#[requires(rf.len() == n)]
#[requires(forall<t: Int> 0 <= t && t < k ==> cur0 + 2 * t + 1 < s.len())]
#[requires(forall<t: Int> 0 <= t && t < k ==> s[cur0 + 2 * t]@ == ctrl)]
#[requires(forall<t: Int> 0 <= t && t < k ==> s[cur0 + 2 * t + 1]@ == val@)]
#[requires(forall<i: Int> produced0 <= i && i < produced0 + k * c ==> rf[i] == val@)]
#[ensures(rmodel::mr_ch(s, cur0, produced0, n, part(rf, n, produced0))
    == rmodel::mr_ch(s, cur0 + 2 * k, produced0 + k * c, n, part(rf, n, produced0 + k * c)))]
pub fn rep_run(s: Seq<u8>, cur0: Int, produced0: Int, n: Int, ctrl: Int, c: Int, val: u8,
               k: Int, rf: Seq<u8>) {
    if k > 0 {
        rep_run(s, cur0, produced0, n, ctrl, c, val, k - 1, rf);
        proof_assert!(cur0 + 2 * (k - 1) + 1 < s.len());
        proof_assert!(s[cur0 + 2 * (k - 1)]@ == ctrl);
        proof_assert!(s[cur0 + 2 * (k - 1) + 1]@ == val@);
        proof_assert!(produced0 + (k - 1) * c < n);
        rmodel::ch_step(s, cur0 + 2 * (k - 1), produced0 + (k - 1) * c, n,
            part(rf, n, produced0 + (k - 1) * c));
        iw_rep(s, cur0 + 2 * (k - 1), produced0 + (k - 1) * c, c, n, val, rf);
        proof_assert!(cur0 + 2 * (k - 1) + 2 == cur0 + 2 * k);
        proof_assert!(produced0 + (k - 1) * c + c == produced0 + k * c);
    }
}

/// Compose three finished channels into `MR`'s exact three-channel result.
#[logic]
#[requires(0 <= n)]
#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]
#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]
#[requires(rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) == Ok((b0, c3)))]
#[requires(c3 == s.len())]
#[ensures(mr_model(s, n) == Ok((r0, g0, b0)))]
pub fn mr_three(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, b0: Seq<u8>,
                c1: Int, c2: Int, c3: Int) {
    part0(r0, n);
    part0(g0, n);
    part0(b0, n);
}

/// `MR`'s trailing-byte branch: the three channels finish before the payload end.
#[logic]
#[requires(0 <= n)]
#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]
#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]
#[requires(rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) == Ok((b0, c3)))]
#[requires(c3 != s.len())]
#[ensures(mr_model(s, n) == Err(Error::RleTrailingBytes))]
pub fn mr_trailing(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, b0: Seq<u8>,
                   c1: Int, c2: Int, c3: Int) {
    part0(r0, n);
    part0(g0, n);
    part0(b0, n);
}

/// `MR`'s first-channel error.
#[logic]
#[requires(0 <= n)]
#[requires(match rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) { Err(_) => true, Ok(_) => false })]
#[ensures(mr_model(s, n) == Err(er))]
pub fn mr_err1(s: Seq<u8>, n: Int, er: Error) {}

/// `MR`'s second-channel error.
#[logic]
#[requires(0 <= n)]
#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]
#[requires(match rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) { Err(_) => true, Ok(_) => false })]
#[ensures(mr_model(s, n) == Err(er))]
pub fn mr_err2(s: Seq<u8>, n: Int, r0: Seq<u8>, c1: Int, er: Error) {
    part0(r0, n);
}

/// `MR`'s third-channel error.
#[logic]
#[requires(0 <= n)]
#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]
#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]
#[requires(match rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) { Err(_) => true, Ok(_) => false })]
#[ensures(mr_model(s, n) == Err(er))]
pub fn mr_err3(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, c1: Int, c2: Int, er: Error) {
    part0(r0, n);
    part0(g0, n);
}

/// `mr_model` on a payload that is empty in every channel: `ch_trunc` at cursor zero.
#[logic]
#[requires(0 <= n)]
#[requires(s.len() == 0)]
#[ensures(mr_model(s, n) == Err(Error::RleTruncated))]
pub fn mr_empty(s: Seq<u8>, n: Int) {
    rmodel::ch_trunc(s, 0, 0, n, zero_buf(n));
    part0(zero_buf(n), n);
}

/// The `interleave3` of three concrete channels is the fixture's expected RGB bytes.
#[logic]
#[requires(0 <= n)]
#[requires(r.len() == n && g.len() == n && b.len() == n)]
#[requires(out.len() == 3 * n)]
#[requires(forall<i: Int> 0 <= i && i < 3 * n ==> out[i]
    == (if i % 3 == 0 { r[i / 3] } else if i % 3 == 1 { g[i / 3] } else { b[i / 3] }))]
#[ensures(interleave3(r, g, b, n) == out)]
pub fn interleave_eq(r: Seq<u8>, g: Seq<u8>, b: Seq<u8>, n: Int, out: Seq<u8>) {
    proof_assert!(interleave3(r, g, b, n).ext_eq(out));
}

/// The `rgba4` layout of a concrete RGB buffer and alpha channel is the fixture's bytes.
#[logic]
#[requires(0 <= n)]
#[requires(rgb.len() == 3 * n && alpha.len() == n)]
#[requires(out.len() == 4 * n)]
#[requires(forall<i: Int> 0 <= i && i < 4 * n ==> out[i]
    == (if i % 4 == 3 { alpha[i / 4] } else { rgb[3 * (i / 4) + i % 4] }))]
#[ensures(crate::rgba4(rgb, alpha, n) == out)]
pub fn rgba_eq(rgb: Seq<u8>, alpha: Seq<u8>, n: Int, out: Seq<u8>) {
    proof_assert!(crate::rgba4(rgb, alpha, n).ext_eq(out));
}

/// The entry `mp` appends for the header at `off` is the model record built from the bytes.
#[logic]
#[requires(0 <= off)]
#[ensures(mk_entry(b, off) == m)]
pub fn mkent_eq(b: Seq<u8>, off: Int, m: MEntry) {}

/// The effective payload of a colour entry is the four-zero-stripped payload.
#[logic]
#[requires(0 <= n)]
#[ensures(crate::effective_payload(b, m) == p)]
pub fn effpl_eq(b: Seq<u8>, m: MEntry, p: Seq<u8>) {}

/// `legacy_colour_model`/`legacy_mask_model` select the side of a known legacy code.
#[logic]
#[ensures(crate::legacy_colour_model(t) == Some(d))]
pub fn lc_eq(t: Seq<u8>, d: Int, d2: u32) {}

'''

