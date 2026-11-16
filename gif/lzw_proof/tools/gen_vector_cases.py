#!/usr/bin/env python3
"""Extract the LZW-relevant cases from gif/spec-draft/VECTORS.json into
tests/expected/vectors.json: those whose expanded image data reaches G10.

For each: the minimum code size `m`, the concatenated image data `D` (the
de-sub-blocked payload, which is what `lzw_decode` takes), the frame area `A`, and
the expected outcome from `gif_oracle.py`'s `lzw_decode` run on that `D` and `A`
(the oracle is the SPEC's reference; the vector's own expectation is the *frame*
result, which M3's parser turns into this call). Records whose profile is not
Strict skip the Strict instantiation test but are kept with their profile's
`lzw_min`, so the RecommendedBeta size-1 cases are exercised too.
"""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
GIF = ROOT.parent
sys.path.insert(0, str(GIF / "spec-draft"))
sys.path.insert(0, str(HERE))
from gif_oracle import lzw_decode, GifError  # noqa: E402
sys.path.insert(0, str(GIF / "policy-draft"))
import policy_oracle as po  # noqa: E402
from gen_witnesses import model_fold  # noqa: E402

import importlib.util as _ilu  # noqa: E402
_spec = _ilu.spec_from_file_location("gv", GIF / "spec-draft" / "gen_vectors.py")
gv = _ilu.module_from_spec(_spec)
_spec.loader.exec_module(gv)


def expand(rec):
    h = rec["input_hex"]
    b = bytes.fromhex(h)
    if "pad_to" in rec:
        b = b + b"\x00" * (rec["pad_to"] - len(b))
    if "repeat_hex" in rec:
        b = b + bytes.fromhex(rec["repeat_hex"]) * rec["repeat"]
    if "suffix_hex" in rec:
        b = b + bytes.fromhex(rec["suffix_hex"])
    return b


def find_image_data(b):
    """Locate the first image's minimum code size and its concatenated sub-block
    payload, or None. Enough of G5-G9 for the vectors under test: a linear walk that
    skips the global table, extensions and the descriptor."""
    if len(b) < 13:
        return None
    p = 13
    f = b[10]
    if f & 0x80:
        p += 3 * (2 << (f & 7))
    while p < len(b):
        if b[p] == 0x3B:
            return None
        if b[p] == 0x21:
            if p + 2 > len(b):
                return None
            p += 2
            while p < len(b) and b[p] != 0:
                p += 1 + b[p]
            if p < len(b):
                p += 1
            continue
        if b[p] == 0x2C:
            if p + 10 > len(b):
                return None
            packed = b[p + 9]
            left, top = int.from_bytes(b[p + 1:p + 3], "little"), int.from_bytes(b[p + 3:p + 5], "little")
            w, h = int.from_bytes(b[p + 5:p + 7], "little"), int.from_bytes(b[p + 7:p + 9], "little")
            q = p + 10
            if packed & 0x80:
                q += 3 * (2 << (packed & 7))
            if q >= len(b):
                return None
            m = b[q]
            q += 1
            subs = b""
            while q < len(b) and b[q] != 0:
                n = b[q]
                if q + 1 + n > len(b):
                    return None
                subs += b[q + 1:q + 1 + n]
                q += 1 + n
            return m, subs, w * h, left + w, top + h
        return None
    return None


def main():
    vs = json.loads((GIF / "spec-draft" / "VECTORS.json").read_text())
    recs = []
    for rec in vs:
        if rec.get("entry") != "still":
            continue
        if "input_hex" not in rec:
            continue
        try:
            b = expand(rec)
        except Exception:
            continue
        found = find_image_data(b)
        if found is None:
            continue
        m, D, A, _, _ = found
        if m > 8:
            # gif_oracle.py's `lzw_decode` cannot spell a literal above 255 (its
            # initial table builds `bytes([i])`), so sizes 9-11 are covered by the
            # policy_oracle vectors instead; the LZW core's range for them is the
            # `Range1to11Checked` branch, exercised in `witnesses.rs`.
            continue
        idx = []
        prof = rec.get("profile", "Strict")
        # The expectation must come from the oracle that shares the record's policy:
        # `gif_oracle.py` is Strict (its `lzw_decode` has no switches), and profile
        # records go through `policy_oracle.py` with that profile's record, so a
        # `StopAtRaster` record is not judged by Strict's tail rule.
        try:
            if prof == "Strict":
                lzw_decode(m, D, A, lambda q, v: idx.append(v))
            else:
                po.lzw_decode(m, D, A, lambda q, v: idx.append(v),
                              po.PROFILES[prof], set())
            err = None
        except GifError as e:
            err = e.name
        # The Strict range is 2..8; RecommendedBeta's is 1..11 with checked literals
        # and the size-1 rule. Records outside the profile's range are the parser's
        # `InvalidLzwMinCodeSize`, which the LZW core sees only as a policy branch.
        # A record whose `m` is outside its profile's range never reaches G10: the
        # parser returns `InvalidLzwMinCodeSize` at the minimum-code-size byte
        # (SPEC G9), which M3 proves, so the LZW core is not the place to check it.
        # The container vectors (e.g. `basic-4x3-gct`) all repeat the same tiny
        # image; keep one per distinct `(m, D, A)` so the file stays small, and drop
        # the two 8,192-pixel full-table vectors' payloads (they are tens of MB and
        # the same code paths run on the smaller ones; the full table is the symbolic
        # obligation L-FULL).
        lo, hi = (2, 8) if prof == "Strict" else (1, 11)
        if not (lo <= m <= hi):
            continue
        if prof == "RecommendedBeta" and m == 1 and A != 1:
            continue
        if A > 4096:
            continue
        recs.append(dict(name=rec["name"], profile=prof, m=m, a=A,
                         data=list(D), expected=idx, error=err))
    # One record per distinct call: the container vectors repeat one image.
    seen, uniq = set(), []
    for r in recs:
        k = (r["profile"], r["m"], r["a"], tuple(r["data"]))
        if k in seen:
            continue
        seen.add(k)
        uniq.append(r)
    out = ROOT / "tests" / "expected" / "vectors.json"
    out.write_text(json.dumps(uniq, indent=1) + "\n")
    recs = uniq
    print(f"wrote {out}: {len(recs)} distinct LZW vector calls")
    for r in recs:
        print(f"  {r['name']:<44} {r['profile']:<16} m={r['m']} A={r['a']} "
              f"D={r['data'][:12]}{'...' if len(r['data']) > 12 else ''} -> "
              f"{r['error'] or r['expected'][:8]}")


if __name__ == "__main__":
    main()
