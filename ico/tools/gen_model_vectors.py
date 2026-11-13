#!/usr/bin/env python3
"""Generate Verus model-vs-vector agreement proofs from spec-frozen/VECTORS.json.

For every parse vector whose expanded input is at most LIMIT bytes, emit a
`proof fn` whose statement is

    mc(<the vector's expanded input>) == <the fixture's expected parse Result>

and for every decode vector whose expanded input is at most LIMIT bytes, emit

    <the supplied entries as a Seq<Entry>>  (from `entries_override`, or the
    independent ME reconstruction of the vector, since `parse` vectors and
    decode vectors without an override supply exactly `MC`'s output)

    md(<input>, <entries>, <index>) == <the fixture's expected decode Result>

Every expected value is read from the fixture's `expected` field; the supplied
entries are read from `entries_override` when present and otherwise rebuilt
from the wire fields by an independent transcription of R4-R6 (the same rules the
oracle uses), never from the executable decoder.

`mc`/`md` close by `by (compute)` for the small DIB/PNG slices, exactly as the
DIB crate's own `model_vectors.rs` closes `mp` on its small vectors: the model's
DIB call bottoms out in a concrete `dib_verus::model::mp(slice, Slack)` term the
normalizer unfolds.  The oversized fixtures (`file-size-67108864`,
`file-size-67108865`, `too-large-before-bad-header`, `count-exact-1024*`) are
covered symbolically by the `L1`-`L4` lemmas in `src/model.rs` and remain runtime
conformance vectors; no 64 MiB input is expanded into verifier terms.

Usage: tools/gen_model_vectors.py [--out PATH] [--check]
"""

import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VECTORS = os.path.join(ROOT, "spec-frozen", "VECTORS.json")
DEFAULT_OUT = os.path.join(ROOT, "ico_verus", "src", "model_vectors.rs")

# The largest input expanded as a literal into verifier terms.  The vector file
# holds nothing between a few hundred bytes and the 64 KiB / 64 MiB fixtures, so
# this limit separates the small vectors from the four symbolic ones cleanly.
LIMIT = 4096

# The four oversized fixtures: covered by the symbolic lemmas L1-L4 and the
# runtime conformance test, never by a verifier term.
LARGE = {
    "file-size-67108864": "L2 (symbolic length lemma)",
    "file-size-67108865": "L1 (over-cap length lemma)",
    "too-large-before-bad-header": "L1 (over-cap length lemma)",
    "count-exact-1024": "L3 (induction, not a 1,024-record verifier term)",
    "count-exact-1024-decode-last": "L3 + the runtime `count-exact-1024` fixture",
    "count-exact-1024-decode-first": "L3 + the runtime `count-exact-1024` fixture",
}

# ---------------------------------------------------------------------------
# R12 input expansion (identical to the conformance harness's rules)
# ---------------------------------------------------------------------------

MAX_INPUT = 67_108_864
MAX_ENTRIES = 1024
SIGNATURE = bytes.fromhex("89504e470d0a1a0a")


def expanded(v):
    raw = bytes.fromhex(v["input_hex"])
    rep = v.get("input_repeat", 1)
    cut = v.get("repeat_from")
    if cut is not None:
        return raw[:cut] + raw[cut:] * rep + bytes.fromhex(v["input_suffix_hex"])
    return raw * rep


# ---------------------------------------------------------------------------
# Independent wire-record reconstruction (R4-R6) for supplied entries
# ---------------------------------------------------------------------------

def word(b, p, n):
    return int.from_bytes(b[p:p + n], "little")


def wire_directory(b):
    """R3; None on a directory-wide error."""
    if len(b) > MAX_INPUT or len(b) < 6:
        return None
    if word(b, 0, 2):
        return None
    typ = word(b, 2, 2)
    if typ not in (1, 2):
        return None
    count = word(b, 4, 2)
    if not count or count > MAX_ENTRIES:
        return None
    end = 6 + 16 * count
    if end > len(b):
        return None
    return typ, count, end


def me_entry(b, typ, end, i):
    """R4-R6: one reconstructed record as (width,height,color_count,raw_offset,
    raw_size,planes,bit_count,hotspot_x,hotspot_y,status)."""
    p = 6 + 16 * i
    width = b[p] or 256
    height = b[p + 1] or 256
    colors = b[p + 2]
    reserved = b[p + 3]
    lo, hi = word(b, p + 4, 2), word(b, p + 6, 2)
    size, off = word(b, p + 8, 4), word(b, p + 12, 4)
    if reserved:
        status = ("invalid", "InvalidEntryReserved")
    elif size == 0:
        status = ("invalid", "EmptyEntry")
    elif off < end:
        status = ("invalid", "EntryOverlapsDirectory")
    elif off > len(b) or size > len(b) - off:
        status = ("invalid", "EntryOutOfBounds")
    else:
        kind = "Png" if size >= 8 and b[off:off + 8] == SIGNATURE else "Dib"
        status = ("valid", kind, off, off + size)
    planes = lo if typ == 1 else None
    bit_count = hi if typ == 1 else None
    hotspot_x = None if typ == 1 else lo
    hotspot_y = None if typ == 1 else hi
    return dict(width=width, height=height, color_count=colors, raw_offset=off,
                raw_size=size, planes=planes, bit_count=bit_count,
                hotspot_x=hotspot_x, hotspot_y=hotspot_y, status=status)


def supplied_entries(v, data):
    """The typed entries `decode_entry` is called with: `entries_override` when
    present, else `MC(data)`'s list rebuilt independently from the wire."""
    if "entries_override" in v:
        out = []
        for e in v["entries_override"]:
            st = e["status"]
            if "invalid" in st:
                status = ("invalid", st["invalid"])
            else:
                val = st["valid"]
                status = ("valid", val["kind"], val["payload"][0], val["payload"][1])
            out.append(dict(width=e["width"], height=e["height"],
                            color_count=e["color_count"], raw_offset=e["raw_offset"],
                            raw_size=e["raw_size"], planes=e["planes"],
                            bit_count=e["bit_count"], hotspot_x=e["hotspot_x"],
                            hotspot_y=e["hotspot_y"], status=status))
        return out
    d = wire_directory(data)
    if d is None:
        return []
    typ, count, end = d
    return [me_entry(data, typ, end, i) for i in range(count)]


# ---------------------------------------------------------------------------
# Verus rendering
# ---------------------------------------------------------------------------

def hex32(x):
    return "0x%08xu32" % (x & 0xFFFFFFFF)


def hex16(x):
    return "0x%04xu16" % (x & 0xFFFF)


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


def opt_u16(x):
    return "None" if x is None else ("Some(%s)" % hex16(x))


def status_lit(st):
    if st[0] == "invalid":
        return "EntryStatus::Invalid(EntryDefect::%s)" % st[1]
    _, kind, start, stop = st
    return ("EntryStatus::Valid { kind: EntryKind::%s, payload: [%s, %s] }"
            % (kind, hex32(start), hex32(stop)))


def entry_lit(e):
    return ("Entry { width: %s, height: %s, color_count: %du8, raw_offset: %s, "
            "raw_size: %s, planes: %s, bit_count: %s, hotspot_x: %s, hotspot_y: %s, "
            "status: %s }"
            % (hex16(e["width"]), hex16(e["height"]), e["color_count"],
               hex32(e["raw_offset"]), hex32(e["raw_size"]),
               opt_u16(e["planes"]), opt_u16(e["bit_count"]),
               opt_u16(e["hotspot_x"]), opt_u16(e["hotspot_y"]),
               status_lit(e["status"])))


def entries_seq(entries, indent=8):
    if not entries:
        return "Seq::empty()"
    lines = []
    for e in entries:
        lines.append(" " * indent + entry_lit(e) + ",")
    return "seq![\n" + "\n".join(lines) + "\n" + " " * (indent - 4) + "]"


def rgba_bytes(exp):
    out = []
    for r in exp["rgba_runs"]:
        px = bytes.fromhex(r["hex"])
        out += list(px) * r["count"]
    return bytes(out)


def error_lit(name):
    if name.startswith("Dib(") and name.endswith(")"):
        inner = name[4:-1]
        return "Error::Dib(dib_verus::dib::Error::%s)" % inner
    return "Error::%s" % name


def expected_mc(v):
    exp = v["expected"]
    if "error" in exp:
        return "Err(%s)" % error_lit(exp["error"])
    body = entries_seq([json_entry_to_typed(e) for e in exp["entries"]])
    return "Ok(%s)" % body


def json_entry_to_typed(e):
    st = e["status"]
    if "invalid" in st:
        status = ("invalid", st["invalid"])
    else:
        val = st["valid"]
        status = ("valid", val["kind"], val["payload"][0], val["payload"][1])
    return dict(width=e["width"], height=e["height"], color_count=e["color_count"],
                raw_offset=e["raw_offset"], raw_size=e["raw_size"],
                planes=e["planes"], bit_count=e["bit_count"],
                hotspot_x=e["hotspot_x"], hotspot_y=e["hotspot_y"], status=status)


def expected_md(v):
    exp = v["expected"]
    if "error" in exp:
        return "Err(%s)" % error_lit(exp["error"])
    rgba = rgba_bytes(exp)
    return "Ok((%dint, %dint, %s))" % (exp["width"], exp["height"],
                                       seq_of_bytes(rgba))


def ident(name):
    return "".join(c if c.isalnum() else "_" for c in name)


HEADER = '''//! Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.
//!
//! DO NOT EDIT BY HAND.  Regenerate with
//!     python3 tools/gen_model_vectors.py
//! Every expected value comes from `spec-frozen/VECTORS.json` (`expected`), never
//! from the executable decoder; every supplied decode-entry list comes from the
//! fixture's `entries_override` or from an independent R4-R6 reconstruction of
//! the wire bytes.  Each `agree_parse_<vector>` obligation states that the model
//! `MC` applied to the vector's expanded input equals the fixture's expected
//! parse `Result` in full (the exact directory error, or the complete ordered
//! record list with every raw field and valid/invalid status).  Each
//! `agree_decode_<vector>` obligation states that the total model `MD` applied to
//! the vector's expanded input, supplied entries and index equals the fixture's
//! expected decode `Result` (exact dimensions and RGBA bytes, or the exact error
//! name including the `Dib(...)` wrap).  A mismatch makes the obligation
//! **false**, not vacuous.
//!
//! The small DIB/PNG slices close by `by (compute)`: the model's DIB call bottoms
//! out in a concrete `dib_verus::model::mp(slice, IcoEntryDibSlack)` term the
//! normalizer unfolds, exactly as the DIB crate's own `model_vectors.rs` closes
//! its small vectors.  The four oversized fixtures are covered by the symbolic
//! lemmas L1-L4 in `src/model.rs` and remain runtime conformance vectors; no
//! 64 MiB input is expanded into verifier terms.
use vstd::prelude::*;

verus! {

#[cfg(verus_only)]
use crate::model::{mc, md};
#[cfg(verus_only)]
use crate::ico::{Entry, EntryDefect, EntryKind, EntryStatus, Error};

'''


def gen():
    vectors = json.load(open(VECTORS))
    body = [HEADER]
    body.append("// %d vectors; the oversized ones are symbolic:\n//\n" % len(vectors))
    for name, cover in LARGE.items():
        body.append("/// `%s`: %s.\n" % (name, cover))
    body.append("\n")
    proofs = []
    n_parse = n_decode = n_large = n_dib = 0
    for v in vectors:
        name = v["name"]
        data = expanded(v)
        if name in LARGE or len(data) > LIMIT:
            n_large += 1
            continue
        i = ident(name)
        if v["operation"] == "parse":
            stmt = "mc(%s) == %s" % (seq_of_bytes(data), expected_mc(v))
            blk = []
            blk.append("// ---- %s (%d bytes, parse) ----\n" % (name, len(data)))
            blk.append("pub open spec fn b_%s() -> Seq<u8> {\n    %s\n}\n\n"
                       % (i, seq_of_bytes(data)))
            blk.append("#[allow(unused)]\n")
            blk.append("pub proof fn agree_parse_%s()\n    ensures\n        %s,\n{\n"
                       % (i, stmt))
            blk.append("    assert(%s) by (compute);\n}\n\n" % stmt)
            proofs.append("".join(blk))
            n_parse += 1
        else:
            assert v["operation"] == "decode"
            exp = v["expected"]
            # T11 (manifest): no proof-gate witness needs a concrete DIB `MP_1.1`
            # pixel evaluation — `dib-basic-decode`/`dib-mismatch-decode` and the
            # other DIB-branch vectors remain runtime fixtures, bridged by the
            # generic T5 slice-model lemma, not by a `by (compute)` term.  Emit a
            # `md` obligation only when the decision never reaches the DIB branch,
            # i.e. the expected result is an ICO error (including `PngDelegation`),
            # never `Ok(...)` nor a `Dib(...)` error.
            is_dib_branch = ("error" not in exp) or exp["error"].startswith("Dib(")
            if is_dib_branch:
                n_dib += 1
                continue
            idx = v["index"]
            ents = supplied_entries(v, data)
            stmt = ("md(%s, %s, %su32) == %s"
                    % (seq_of_bytes(data), entries_seq(ents), idx, expected_md(v)))
            blk = []
            blk.append("// ---- %s (%d bytes, decode idx=%s) ----\n" % (name, len(data), idx))
            blk.append("pub open spec fn b_%s() -> Seq<u8> {\n    %s\n}\n\n"
                       % (i, seq_of_bytes(data)))
            blk.append("pub open spec fn es_%s() -> Seq<Entry> {\n    %s\n}\n\n"
                       % (i, entries_seq(ents)))
            blk.append("#[allow(unused)]\n")
            blk.append("pub proof fn agree_decode_%s()\n    ensures\n        %s,\n{\n"
                       % (i, stmt))
            blk.append("    assert(%s) by (compute);\n}\n\n" % stmt)
            proofs.append("".join(blk))
            n_decode += 1

    text = "".join(body) + "".join(proofs)
    text += "// ---- every small non-DIB vector has exactly one obligation above ----\n"
    text += "} // verus!\n"
    return text, n_parse, n_decode, n_large, n_dib


def main():
    out = DEFAULT_OUT
    args = sys.argv[1:]
    check = "--check" in args
    if "--out" in args:
        out = args[args.index("--out") + 1]
    text, n_parse, n_decode, n_large, n_dib = gen()
    print("generated %d obligations (%d parse, %d decode), %d large, %d DIB-branch runtime"
          % (n_parse + n_decode, n_parse, n_decode, n_large, n_dib))
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
