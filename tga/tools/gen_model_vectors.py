#!/usr/bin/env python3
"""Generate Verus model-vs-vector agreement proofs from spec-frozen/VECTORS.json.

For each of the 165 vectors whose expanded input is at most 4,096 bytes, emit a
`proof fn` whose statement is

    MP(<the vector's expanded input>) == <the fixture's expected result>

and whose body discharges it by `assert(...) by (compute)`.  Every expected
value is read from the fixture's `expected` field; nothing is taken from the
production decoder.  The four larger vectors are covered by the hand-written
symbolic lemmas L1-L4 in src/model.rs and are listed here only for the record;
the generator checks that each one's expanded bytes equal the manifest's
symbolic construction before emitting the file.

Usage: tools/gen_model_vectors.py [--out PATH] [--check]
"""

import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VECTORS = os.path.join(ROOT, "spec-frozen", "VECTORS.json")
DEFAULT_OUT = os.path.join(ROOT, "tga_verus", "src", "model_vectors.rs")

LIMIT = 4096

# The four vectors above the limit, with the lemma that discharges each.  The
# names and the symbolic constructions are exactly those of THEOREMS.md's
# "Non-vacuity and compact vector protocol".
LARGE = {
    "input-over-64m": "lemma_l1_input_over_64m (L1)",
    "input-at-64m": "lemma_l2_input_at_64m (L2)",
    "output-at-64m-8192x2048": "lemma_l3_output_at_64m_8192x2048 (L3)",
    "output-at-64m-4096-square": "lemma_l4_output_at_64m_4096_square (L4)",
}


def H(t, w, h, d, a):
    """The manifest's R1 18-byte header `H(t,w,h,d,a)`: type `t`, dimensions
    `w,h`, depth `d`, descriptor `a`, all other fields zero."""
    b = bytearray(18)
    b[2] = t
    b[12] = w & 0xFF
    b[13] = (w >> 8) & 0xFF
    b[14] = h & 0xFF
    b[15] = (h >> 8) & 0xFF
    b[16] = d
    b[17] = a
    return bytes(b)


S = bytes([0xFF, 0x7F])                        # the manifest's `S`
C = bytes([0xFF, 0x00, 0xFC])                  # the manifest's `C`


def large_input_expected(name, v):
    """The byte string each large vector must equal, and the lemma description.

    Returns `(description, bytes)`; the bytes are the manifest's symbolic
    construction for that vector, the independent object the fixture's
    `input_hex`/`input_repeat` is compared against.
    """
    if name == "input-over-64m":                       # L1
        return ("L1: |b| = 67,108,865 = 67,108,864 + 1, a zero byte repeated",
                b"\x00" * 67108865)
    if name == "input-at-64m":                         # L2
        return ("L2: |b| = 67,108,864, b[2] = 0, a zero byte repeated",
                b"\x00" * 67108864)
    if name == "output-at-64m-8192x2048":              # L3
        return ("L3: H(11,8192,2048,8,32) + S^131072, S = bytes(0xff,0x7f)",
                H(11, 8192, 2048, 8, 32) + S * 131072)
    if name == "output-at-64m-4096-square":            # L4
        return ("L4: H(10,4096,4096,16,33) + C^131072, C = bytes(0xff,0x00,0xfc)",
                H(10, 4096, 4096, 16, 33) + C * 131072)
    raise SystemExit("no symbolic description for large vector %r" % name)


def large_expected(name, v):
    """The result THEOREMS.md claims for each large vector, as a string.

    This is checked against the fixture's own `expected` field (both the input
    bytes and the declared result), so a disagreement between the manifest and
    the frozen fixtures is caught before the generated file is written.
    """
    if name == "input-over-64m":                       # L1
        return "FileTooLarge"
    if name == "input-at-64m":                         # L2
        return "UnsupportedImageType"
    if name == "output-at-64m-8192x2048":              # L3
        return (8192, 2048, [(16777216, "7f7f7fff")])
    if name == "output-at-64m-4096-square":            # L4
        return (4096, 4096, [(16777216, "ff0000ff")])
    raise SystemExit("no manifest expectation for large vector %r" % name)


def check_large(vectors):
    """Every large vector's expanded bytes and expected result must equal the
    manifest's symbolic construction and its stated result."""
    out = []
    for v in vectors:
        n = len(v["input_hex"]) // 2 * v.get("input_repeat", 1)
        if n <= LIMIT:
            continue
        desc, want = large_input_expected(v["name"], v)
        got = bytes.fromhex(v["input_hex"]) * v.get("input_repeat", 1)
        if got != want:
            raise SystemExit(
                "large vector %r does not match its lemma description (%s): "
                "fixture expands to %d bytes, symbolic form to %d"
                % (v["name"], desc, len(got), len(want)))
        exp = large_expected(v["name"], v)
        e = v["expected"]
        if isinstance(exp, str):
            if "error" not in e or e["error"] != exp:
                raise SystemExit(
                    "large vector %r: manifest says Err(%s), fixture says %r"
                    % (v["name"], exp, e))
        else:
            w, h, runs = exp
            got_runs = [(r["count"], r["hex"]) for r in e.get("rgba_runs", [])]
            if ("error" in e or e["width"] != w or e["height"] != h
                    or got_runs != runs):
                raise SystemExit(
                    "large vector %r: manifest says Ok(%d,%d,%r), fixture says %r"
                    % (v["name"], w, h, runs, e))
        out.append((v["name"], desc, len(got)))
    return out


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
    """The four pixel bytes of one run as a `seq![...]` literal."""
    data = bytes.fromhex(hexstr)
    assert len(data) == 4, "each run is one RGBA pixel"
    return "seq![" + ", ".join(byte_lit(b) for b in data) + "]"


def expand_runs(runs):
    """`rep_pat(pat, count) + ...` over a fixture's maximal run list."""
    if len(runs) == 1:
        r = runs[0]
        return "rep_pat(%s, %d)" % (pattern_lit(r["hex"]), r["count"])
    parts = ["rep_pat(%s, %d)" % (pattern_lit(r["hex"]), r["count"]) for r in runs]
    return "\n            + ".join(parts)


HEADER = '''//! Model-vs-vector agreement, generated by `tools/gen_model_vectors.py`.
//!
//! DO NOT EDIT BY HAND.  Regenerate with
//!     python3 tools/gen_model_vectors.py
//! The expected values come from `spec-frozen/VECTORS.json` (`expected`), never
//! from the executable decoder.  Each proof obligation below states that the
//! formal model `MP` applied to the vector's expanded input yields exactly the
//! fixture's expected `Result` (`Ok(width, height, rgba)` byte-exact, or
//! `Err(e)` for the exact error name); the body discharges it by `by (compute)`.
//!
//! The four vectors longer than 4,096 bytes are covered by the symbolic lemmas
//! L1-L4 in `src/model.rs` and are not expanded here.
use vstd::prelude::*;

verus! {

#[cfg(verus_only)]
use crate::model::mp;
#[cfg(verus_only)]
use crate::tga::Error;

/// `rep_pat(p, k)` repeats the four-pixel pattern `p` `k` times (R16's run
/// lists), i.e. output byte `i` is `p[i % 4]`.  This is R9's oriented output
/// when every decoded pixel is `p`.
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
    body.append("// ---- The four large vectors and the lemma that covers each ----\n")
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
        assert rep == 1, "small vectors expand their hex exactly once"
        exp = v["expected"]

        block = []
        block.append("// ---- %s (%d bytes) ----\n" % (name, len(data)))
        block.append("pub open spec fn b_%s() -> Seq<u8> {\n" % i)
        block.append("    " + seq_of_bytes(data) + "\n")
        block.append("}\n\n")

        if "error" in exp:
            stmt = "mp(b_%s()) == Err(Error::%s)" % (i, exp["error"])
        else:
            w = exp["width"]
            h = exp["height"]
            out = expand_runs(exp["rgba_runs"])
            stmt = ("mp(b_%s()) == Ok((%d, %d,\n            %s))" % (i, w, h, out))

        block.append("#[allow(unused)]\n")
        block.append("pub proof fn agree_%s()\n" % i)
        block.append("    ensures\n        %s,\n{\n" % stmt)
        block.append("    assert(%s) by (compute);\n}\n\n" % stmt)
        proofs.append("".join(block))

    text = "".join(body) + "".join(proofs)
    text += "// ---- every small vector has exactly one obligation above ----\n"
    text += "} // verus!\n"
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
        print("  large: %-30s %9d bytes  %s" % (name, n, desc))
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
