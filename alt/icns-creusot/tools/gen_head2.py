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


def table(n, runs):
    if len(runs) == 0:
        return "Seq::create(%d, |_i: Int| 0u8)" % n
    parts = []
    for k, (a, b, val) in enumerate(runs):
        cond = "i < %d" % b if k < len(runs) - 1 else "true"
        parts.append("if %s { %du8 }" % (cond, val))
    return "Seq::create(%d, |i: Int| %s)" % (n, " else ".join(parts))


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


HEADER = '''
//! Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.
//!
//! DO NOT EDIT BY HAND.  Regenerate with
//!     python3 tools/gen_model_vectors.py
//!
//! Each `agree_*` obligation states that the crate's formal model, applied to the vector's
//! expanded input, yields exactly the fixture's `expected` value.  Expected values are read
//! from `spec-frozen/VECTORS.json` and never from the executable decoder.
//!
//! Container vectors state `model::mp(b) == <expected>` (an exact `Result`: the same error
//! variant, or the same ordered entry list with all four fields).  Decoding vectors state
//! `match model::mp(b) { Ok(ms) => <model>(b, ms, idx) == <expected>, Err(_) => false }`, so
//! the entries the decoder model consumes are `mp`'s own parse result and a parse failure
//! makes the obligation false rather than vacuous.
//!
//! Creusot's recursive models (`model::mp_loop`, `rmodel::mr_ch`) are opaque outside their
//! modules, so each obligation is a chain of small lemmas instantiating the proved
//! single-step facts (`model::step_acc`, `model::loop_done`, `model::loop_hdr`,
//! `model::loop_short`, `model::loop_oob`, `model::loop_many`, `rmodel::ch_step`,
//! `rmodel::ch_done`, `rmodel::ch_over`, `rmodel::ch_data`, `rmodel::ch_trunc`) at concrete
//! `Int` positions, following the M3c encoding rules: byte-at-index functions via
//! `Seq::create`, `Int` not `u32` in logic, no concat/repeat constructors.
//!
//! The nine vectors longer than 4,096 bytes are covered by the symbolic lemmas L1-L6.

use creusot_std::prelude::*;

use crate::model;
use crate::{
    be32_log, interleave3, magic_model, ma_model, mc_model, mj_model, mk_entry, mr_iw,
    mr_model, rgba4, rmodel, zero_buf, Error, MEntry,
};

/// The `n`-byte buffer whose bytes below `filled` come from `rf` and whose bytes at or above
/// `filled` are still zero: a channel buffer after a prefix of its packets.
#[logic(open)]
pub fn part(rf: Seq<u8>, n: Int, filled: Int) -> Seq<u8> {
    pearlite! { Seq::create(n, |i: Int| if 0 <= i && i < filled { rf[i] } else { 0u8 }) }
}

/// A buffer that is `rf` up to `filled` and zero above is `rf` once `filled` reaches its length.
#[logic]
#[requires(0 <= n)]
#[requires(rf.len() == n)]
#[ensures(part(rf, n, n) == rf)]
pub fn partn(rf: Seq<u8>, n: Int) {}

/// A repeat packet writing `val` over `[prod, prod+count)`, where `rf` is `val` on exactly
/// that range.
#[logic]
#[requires(count > 0)]
#[requires(0 <= prod && prod + count <= n)]
#[requires(0 <= ctrl && ctrl + 1 < s.len())]
#[requires(!(s[ctrl]@ < 128))]
#[requires(s[ctrl]@ - 125 == count)]
#[requires(s[ctrl + 1]@ == val@)]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> prod <= i && i < prod + count ==> rf[i]@ == val@)]
#[ensures(mr_iw(s, ctrl, false, prod, count, part(rf, n, prod)) == part(rf, n, prod + count))]
pub fn iw_rep(s: Seq<u8>, ctrl: Int, prod: Int, count: Int, n: Int, val: u8, rf: Seq<u8>) {
    proof_assert!(mr_iw(s, ctrl, false, prod, count, part(rf, n, prod))
        .ext_eq(part(rf, n, prod + count)));
}

/// A literal packet whose `count` bytes are exactly `rf`'s bytes over `[prod, prod+count)`.
#[logic]
#[requires(count > 0)]
#[requires(0 <= prod && prod + count <= n)]
#[requires(0 <= ctrl && ctrl + count < s.len())]
#[requires(s[ctrl]@ < 128)]
#[requires(s[ctrl]@ + 1 == count)]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> 0 <= i && i < count ==> s[ctrl + 1 + i]@ == rf[prod + i]@)]
#[ensures(mr_iw(s, ctrl, true, prod, count, part(rf, n, prod)) == part(rf, n, prod + count))]
pub fn iw_lit(s: Seq<u8>, ctrl: Int, prod: Int, count: Int, n: Int, rf: Seq<u8>) {
    proof_assert!(mr_iw(s, ctrl, true, prod, count, part(rf, n, prod))
        .ext_eq(part(rf, n, prod + count)));
}

/// One repeat packet step.
#[logic]
#[requires(count > 0)]
#[requires(0 <= prod && prod + count <= n)]
#[requires(0 <= cur && cur + 1 < s.len())]
#[requires(!(s[cur]@ < 128))]
#[requires(s[cur]@ - 125 == count)]
#[requires(s[cur + 1]@ == val@)]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> prod <= i && i < prod + count ==> rf[i]@ == val@)]
#[ensures(rmodel::mr_ch(s, cur, prod, n, part(rf, n, prod))
    == rmodel::mr_ch(s, cur + 2, prod + count, n, part(rf, n, prod + count)))]
pub fn step_rep(s: Seq<u8>, cur: Int, prod: Int, count: Int, n: Int, val: u8, rf: Seq<u8>) {
    rmodel::ch_step(s, cur, prod, n, part(rf, n, prod));
    iw_rep(s, cur, prod, count, n, val, rf);
}

/// One literal packet step.
#[logic]
#[requires(count > 0)]
#[requires(0 <= prod && prod + count <= n)]
#[requires(0 <= cur && cur + count < s.len())]
#[requires(s[cur]@ < 128)]
#[requires(s[cur]@ + 1 == count)]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> 0 <= i && i < count ==> s[cur + 1 + i]@ == rf[prod + i]@)]
#[ensures(rmodel::mr_ch(s, cur, prod, n, part(rf, n, prod))
    == rmodel::mr_ch(s, cur + 1 + count, prod + count, n, part(rf, n, prod + count)))]
pub fn step_lit(s: Seq<u8>, cur: Int, prod: Int, count: Int, n: Int, rf: Seq<u8>) {
    rmodel::ch_step(s, cur, prod, n, part(rf, n, prod));
    iw_lit(s, cur, prod, count, n, rf);
}

/// `k` identical repeat packets advancing `(cur0, prod0)` by `(2k, k*count)`.
#[logic]
#[variant(k)]
#[requires(k >= 0)]
#[requires(count > 0)]
#[requires(0 <= prod0 && prod0 + k * count <= n)]
#[requires(0 <= cur0 && cur0 + 2 * k <= s.len())]
#[requires(forall<t: Int> 0 <= t && t < k ==> s[cur0 + 2 * t]@ == ctrl)]
#[requires(forall<t: Int> 0 <= t && t < k ==> s[cur0 + 2 * t + 1]@ == val@)]
#[requires(rf.len() == n)]
#[requires(forall<i: Int> prod0 <= i && i < prod0 + k * count ==> rf[i]@ == val@)]
#[ensures(rmodel::mr_ch(s, cur0, prod0, n, part(rf, n, prod0))
    == rmodel::mr_ch(s, cur0 + 2 * k, prod0 + k * count, n, part(rf, n, prod0 + k * count)))]
pub fn grp_rep(s: Seq<u8>, cur0: Int, prod0: Int, count: Int, n: Int, ctrl: Int, val: u8,
               k: Int, rf: Seq<u8>) {
    if k > 0 {
        grp_rep(s, cur0, prod0, count, n, ctrl, val, k - 1, rf);
        proof_assert!(s[cur0 + 2 * (k - 1)]@ == ctrl);
        proof_assert!(s[cur0 + 2 * (k - 1) + 1]@ == val@);
        proof_assert!(0 <= prod0 + (k - 1) * count && prod0 + (k - 1) * count + count <= n);
        step_rep(s, cur0 + 2 * (k - 1), prod0 + (k - 1) * count, count, n, val, rf);
        proof_assert!(cur0 + 2 * (k - 1) + 2 == cur0 + 2 * k);
        proof_assert!(prod0 + (k - 1) * count + count == prod0 + k * count);
    } else {
        proof_assert!(cur0 + 2 * k == cur0 && prod0 + k * count == prod0);
    }
}

/// The channel is full: `mr_ch` returns the buffer and the cursor unchanged.
#[logic]
#[requires(0 <= n)]
#[ensures(rmodel::mr_ch(s, cur, n, n, rf) == Ok((rf, cur)))]
pub fn done(s: Seq<u8>, cur: Int, n: Int, rf: Seq<u8>) {
    rmodel::ch_done(s, cur, n, n, rf);
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
pub fn mr_err2(s: Seq<u8>, n: Int, r0: Seq<u8>, c1: Int, er: Error) {}

/// `MR`'s third-channel error.
#[logic]
#[requires(0 <= n)]
#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]
#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]
#[requires(match rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) { Err(_) => true, Ok(_) => false })]
#[ensures(mr_model(s, n) == Err(er))]
pub fn mr_err3(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, c1: Int, c2: Int, er: Error) {}

/// `MR` composed of three finished channels.
#[logic]
#[requires(0 <= n)]
#[requires(rmodel::mr_ch(s, 0, 0, n, zero_buf(n)) == Ok((r0, c1)))]
#[requires(rmodel::mr_ch(s, c1, 0, n, zero_buf(n)) == Ok((g0, c2)))]
#[requires(rmodel::mr_ch(s, c2, 0, n, zero_buf(n)) == Ok((b0, c3)))]
#[requires(c3 == s.len())]
#[ensures(mr_model(s, n) == Ok((r0, g0, b0)))]
pub fn mr_three(s: Seq<u8>, n: Int, r0: Seq<u8>, g0: Seq<u8>, b0: Seq<u8>,
                c1: Int, c2: Int, c3: Int) {}

/// `interleave3` of three channels equals a byte-at-index table.
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

/// The `rgba4` layout of an RGB buffer and an alpha channel equals a byte-at-index table.
#[logic]
#[requires(0 <= n)]
#[requires(rgb.len() == 3 * n && alpha.len() == n)]
#[requires(out.len() == 4 * n)]
#[requires(forall<i: Int> 0 <= i && i < 4 * n ==> out[i]
    == (if i % 4 == 3 { alpha[i / 4] } else { rgb[3 * (i / 4) + i % 4] }))]
#[ensures(rgba4(rgb, alpha, n) == out)]
pub fn rgba_eq(rgb: Seq<u8>, alpha: Seq<u8>, n: Int, out: Seq<u8>) {
    proof_assert!(rgba4(rgb, alpha, n).ext_eq(out));
}

/// A one-entry model list.
#[logic(open)]
pub fn one(m: MEntry) -> Seq<MEntry> {
    pearlite! { Seq::empty().push_back(m) }
}

/// `Seq::create`'s read at an in-range index (uses the table's own axiom).
#[logic]
#[requires(0 <= i && i < len)]
#[ensures(Seq::create(len, f)[i] == f.get(i))]
pub fn create_at<T>(len: Int, f: creusot_std::logic::Mapping<Int, T>, i: Int) {}

'''
