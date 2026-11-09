#!/usr/bin/env python3
"""Generate Verus model-vs-vector agreement proofs from spec-frozen/VECTORS.json.

For each of the 102 vectors whose expanded input is at most 4,096 bytes, emit a
`proof fn` whose statement is

    <model application> == <the fixture's expected result>

and whose body discharges it by `assert(...) by (compute)`.  Every expected
value is read from the fixture's `expected` field; nothing is taken from the
production decoder.  The nine larger vectors are covered by the hand-written
lemmas L1-L6 in src/model.rs and are listed here only for the record.

Usage: tools/gen_model_vectors.py [--out PATH] [--check]
"""

import json
import sys
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VECTORS = os.path.join(ROOT, "spec-frozen", "VECTORS.json")
DEFAULT_OUT = os.path.join(ROOT, "icns_verus", "src", "model_vectors.rs")

LIMIT = 4096

# The nine vectors above the limit, with the lemma that discharges each.
LARGE = {
    "file-too-large": "lemma_l1_file_too_large (L1)",
    "file-size-at-limit": "lemma_l2_file_size_at_limit (L2)",
    "mask-t8mk": "lemma_l3_mask_t8mk (L3)",
    "entry-count-exactly-4096": "lemma_l4_entry_count_exactly_4096 (L4)",
    "entry-count-limit": "lemma_l5_vectors (L5)",
    "count-limit-before-stray-suffix": "lemma_l5_vectors (L5)",
    "entry-count-before-later-malformed-entry": "lemma_l5_vectors (L5)",
    "invalid-4097th-entry-before-limit": "lemma_l6_vectors (L6)",
    "entry-count-followed-by-overflowing-4097th": "lemma_l6_vectors (L6)",
}

PARSE_ERRORS = {
    "FileTooLarge", "TruncatedHeader", "BadMagic", "InvalidFileLength",
    "TruncatedFile", "TrailingFileBytes", "TruncatedEntryHeader",
    "InvalidEntryLength", "EntryOutOfBounds", "TooManyEntries",
}

E_BYTES = b"none" + (8).to_bytes(4, "big")   # `e`: the eight-byte empty entry


def h_string(k, suffix=b""):
    """The manifest's `H(k,s) = bytes("icns") · BE32(8+8k+|s|) · e^k · s`."""
    return b"icns" + (8 + 8 * k + len(suffix)).to_bytes(4, "big") + E_BYTES * k + suffix


def b_q(q):
    """The manifest's `B(q) = bytes("icns") · BE32(32,784) · e^4096 · "none" · BE32(q)`."""
    return (b"icns" + (32784).to_bytes(4, "big") + E_BYTES * 4096
            + b"none" + q.to_bytes(4, "big"))


def large_input_expected(name, v):
    """The byte string each large vector must equal, according to THEOREMS.md.

    Returns the lemma description (a string) and the bytes the manifest's
    symbolic form describes for that vector — the independent construction the
    fixture's `input_hex` is compared against.
    """
    if name == "file-too-large":                       # L1
        return "L1: |b| = 67,108,865 = 67,108,864 + 1", b"\x00" * 67108865
    if name == "file-size-at-limit":                   # L2
        return "L2: |b| = 67,108,864, b[0:4] != \"icns\"", b"\x00" * 67108864
    if name == "mask-t8mk":                            # L3
        a = b"\x7f" * 16384
        desc = ("L3: icns·BE32(16,400)·\"t8mk\"·BE32(16,392)·a, "
                "a = 0x7f repeated 16,384 times")
        return desc, (b"icns" + (16400).to_bytes(4, "big") + b"t8mk"
                      + (16392).to_bytes(4, "big") + a)
    if name == "entry-count-exactly-4096":             # L4
        return "L4 at k = 4,096: H(4096, empty)", h_string(4096)
    if name == "entry-count-limit":                    # L5, s = empty
        return "L5 with s = empty: H(4097, empty)", h_string(4097)
    if name == "count-limit-before-stray-suffix":      # L5, s = "BAD"
        return "L5 with s = bytes(\"BAD\")", h_string(4097, b"BAD")
    if name == "entry-count-before-later-malformed-entry":  # L5, s = "none"·BE32(7)
        return ("L5 with s = bytes(\"none\")·BE32(7)",
                h_string(4097, b"none" + (7).to_bytes(4, "big")))
    if name == "invalid-4097th-entry-before-limit":    # L6, q = 7
        return "L6 with q = 7 (< 8)", b_q(7)
    if name == "entry-count-followed-by-overflowing-4097th":  # L6, q = 2^32-1
        return "L6 with q = 4,294,967,295 (> 8)", b_q(0xFFFFFFFF)
    raise SystemExit("no symbolic description for large vector %r" % name)


def check_large(vectors):
    """Every large vector's bytes must equal the manifest's symbolic construction."""
    out = []
    for v in vectors:
        n = len(v["input_hex"]) // 2 * v.get("input_repeat", 1)
        if n <= LIMIT:
            continue
        desc, want = large_input_expected(v["name"], v)
        unit = bytes.fromhex(v["input_hex"])
        got = unit * v.get("input_repeat", 1)
        if got != want:
            raise SystemExit(
                "large vector %r does not match its lemma description (%s): "
                "fixture expands to %d bytes, symbolic form to %d"
                % (v["name"], desc, len(got), len(want)))
        out.append((v["name"], desc, len(got)))
    return out


def _hex4_chars(hex8):
    """The eight lowercase hex digits of a four-byte `type_hex` (R1's encoding)."""
    return list(hex8)


def ident(name):
    return "".join(c if c.isalnum() else "_" for c in name)


def byte_lit(b):
    return "0x%02xu8" % b


def seq_of_bytes(data, indent=8, width=12):
    """A `seq![...]` literal of the given bytes, wrapped for readability."""
    parts = [byte_lit(b) for b in data]
    lines = []
    for i in range(0, len(parts), width):
        lines.append(" " * indent + ", ".join(parts[i:i + width]) + ",")
    return "seq![\n" + "\n".join(lines) + "\n" + " " * (indent - 4) + "]"


def pattern_lit(hexstr):
    """The pixel bytes of one run as a `seq![...]` literal."""
    data = bytes.fromhex(hexstr)
    return "seq![" + ", ".join(byte_lit(b) for b in data) + "]"


def expand_runs(runs):
    """`rep_pat(pat, count) + ...` for a fixture run list (maximal runs kept)."""
    if len(runs) == 1:
        r = runs[0]
        return "rep_pat(%s, %d)" % (pattern_lit(r["hex"]), r["count"])
    parts = ["rep_pat(%s, %d)" % (pattern_lit(r["hex"]), r["count"]) for r in runs]
    return "\n            + ".join(parts)


def entries_lit(entries):
    """The fixture's entry list as `seq![MpEntry { ... }, ...]`."""
    if not entries:
        return "Seq::<MpEntry>::empty()"
    out = ["seq!["]
    for e in entries:
        chars = ", ".join("'%s'" % c for c in _hex4_chars(e["type_hex"]))
        ns = "None" if e["nominal_side"] is None else "Some(%du32)" % e["nominal_side"]
        out.append(
            "        MpEntry {\n"
            "            type_hex: seq![%s],\n"
            "            payload: (%du32, %du32),\n"
            "            opaque_candidate: %s,\n"
            "            nominal_side: %s,\n"
            "        }," % (chars, e["payload"][0], e["payload"][1],
                         "true" if e["opaque_candidate"] else "false", ns)
        )
    out.append("    ]")
    return "\n".join(out)


HEADER = '''//! Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.
//!
//! DO NOT EDIT BY HAND.  Regenerate with
//!     python3 tools/gen_model_vectors.py
//! The expected values come from `spec-frozen/VECTORS.json` (`expected`), never
//! from the executable decoder.  Each proof obligation below states that the
//! formal model (`mp`, and `mc_ms`/`ma_ms`/`mj_ms` for the decoding vectors)
//! applied to the vector's expanded input yields exactly the fixture's expected
//! result; the body discharges it by `by (compute)`.
//!
//! The nine vectors longer than 4,096 bytes are covered by the symbolic lemmas
//! L1-L6 in `src/model.rs` and are not expanded here.
use vstd::prelude::*;

verus! {

#[cfg(verus_only)]
use crate::model::{mp, mc_ms, ma_ms, mj_ms, MpEntry, MpError};
#[cfg(verus_only)]
use crate::icns::Error;

/// `rep_pat(p, k)` repeats the pattern `p` `k` times (R18 run lists).
pub open spec fn rep_pat(pat: Seq<u8>, count: nat) -> Seq<u8> {
    Seq::new((pat.len() * count) as nat, |i: int| pat[i % (pat.len() as int)])
}

'''


def gen():
    vectors = json.load(open(VECTORS))
    total = len(vectors)
    small = []
    large_seen = []
    for v in vectors:
        n = len(v["input_hex"]) // 2 * v.get("input_repeat", 1)
        if n <= LIMIT:
            small.append(v)
        else:
            large_seen.append(v["name"])
    for name in large_seen:
        if name not in LARGE:
            raise SystemExit("large vector %r has no lemma mapping" % name)
    large_checked = check_large(vectors)

    body = [HEADER]
    body.append("// %d vectors: %d at or below %d bytes, %d above.\n\n"
                % (total, len(small), LIMIT, len(large_seen)))
    body.append("// ---- The nine large vectors and the lemma that covers each ----\n")
    body.append("//\n// Each line was checked by the generator: the fixture's expanded\n"
                "// `input_hex`/`input_repeat` equals the manifest's symbolic construction.\n")
    for name, desc, n in large_checked:
        body.append("/// `%s` (%d bytes): %s; discharged by %s.\n"
                    % (name, n, desc, LARGE[name]))
    body.append("\n")

    proofs = []
    for v in small:
        name = v["name"]
        i = ident(name)
        data = bytes.fromhex(v["input_hex"])
        rep = v.get("input_repeat", 1)
        exp = v["expected"]
        op = v["operation"]

        block = []
        block.append("// ---- %s (%s, %d bytes) ----\n" % (name, op, len(data) * rep))
        if rep == 1:
            block.append("pub open spec fn b_%s() -> Seq<u8> {\n" % i)
            block.append("    " + seq_of_bytes(data) + "\n")
            block.append("}\n\n")
        else:
            block.append("pub open spec fn b_%s() -> Seq<u8> {\n" % i)
            block.append("    rep_pat(rep_pat(%s, %d), 1)\n" % (seq_of_bytes(data, indent=8), rep))
            block.append("}\n\n")

        # entries the model must produce
        if "entries" in exp:
            block.append("pub open spec fn e_%s() -> Seq<MpEntry> {\n" % i)
            block.append("    " + entries_lit(exp["entries"]) + "\n")
            block.append("}\n\n")

        if "error" in exp:
            err = exp["error"]
            if err in PARSE_ERRORS:
                stmt = "mp(b_%s()) == Err(MpError::%s)" % (i, err)
            else:
                if op == "rgb":
                    stmt = "mc_ms(b_%s(), ms, %d) == Err(Error::%s)" % (
                        i, v["color_index"], err)
                elif op == "mask":
                    stmt = "ma_ms(b_%s(), ms, %d) == Err(Error::%s)" % (
                        i, v["mask_index"], err)
                else:
                    ci = ("Some(%du32)" % v["color_index"]) if "color_index" in v else "None"
                    mi = ("Some(%du32)" % v["mask_index"]) if "mask_index" in v else "None"
                    stmt = "mj_ms(b_%s(), ms, %s, %s) == Err(Error::%s)" % (
                        i, ci, mi, err)
                # decoding vectors first run parse; make that explicit
                stmt = ("match mp(b_%s()) {\n            Ok(ms) => %s,\n"
                        "            Err(_) => false,\n        }" % (i, stmt))
        elif op == "parse":
            stmt = "mp(b_%s()) == Ok(e_%s())" % (i, i)
        else:
            side = exp["side"]
            if "rgb_runs" in exp:
                key, call = "rgb_runs", "mc_ms"
            elif "alpha_runs" in exp:
                key, call = "alpha_runs", "ma_ms"
            else:
                key, call = "rgba_runs", "mj_ms"
            out = expand_runs(exp[key])
            if key == "rgba_runs":
                ci = ("Some(%du32)" % v["color_index"]) if "color_index" in v else "None"
                mi = ("Some(%du32)" % v["mask_index"]) if "mask_index" in v else "None"
                inner = ("%s(b_%s(), ms, %s, %s) == Ok((%du32, %du32,\n                %s))"
                         % (call, i, ci, mi, side, side, out))
            else:
                idx = v.get("color_index", v.get("mask_index"))
                inner = ("%s(b_%s(), ms, %d) == Ok((%du32, %du32,\n                %s))"
                         % (call, i, idx, side, side, out))
            stmt = ("match mp(b_%s()) {\n            Ok(ms) => %s,\n"
                    "            Err(_) => false,\n        }" % (i, inner))

        block.append("#[allow(unused)]\n")
        block.append("pub proof fn agree_%s()\n" % i)
        block.append("    ensures\n        %s,\n{\n" % stmt)
        block.append("    assert(%s) by (compute);\n}\n\n" % stmt)
        proofs.append("".join(block))

    text = "".join(body) + "".join(proofs)
    text += "// ---- every small vector has exactly one obligation above ----\n"
    text += "}\n"
    return text, small, large_checked


def main():
    out = DEFAULT_OUT
    args = sys.argv[1:]
    check = "--check" in args
    if "--out" in args:
        out = args[args.index("--out") + 1]
    text, small, large_checked = gen()
    print("generated %d small-vector obligations, %d large (lemmas)"
          % (len(small), len(large_checked)))
    for name, desc, n in large_checked:
        print("  large: %-46s %7d bytes  %s" % (name, n, desc))
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