#!/usr/bin/env python3
"""Generate tests/expected/random.json: ~200 random code streams with the expected
index sequence or first error, from gif_oracle.py's `lzw_decode` (the SPEC's
reference). The same Python mirror of `src/model.rs` (tools/gen_witnesses.py's
`model_fold`) is used to fill in this crate's bit-position convention, so the cargo
test can compare all three: the oracle's name, the mirror's name and position, and
(after the executable lands) the executable's result.

Kept as one file of a few hundred KB; the test reads it and does not need the bytes
in git (regenerate with this script).
"""
import json
import random
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
GIF = ROOT.parent
sys.path.insert(0, str(GIF / "spec-draft"))
sys.path.insert(0, str(HERE))
from gif_oracle import lzw_decode, GifError  # noqa: E402
from gen_vectors import lzw_codes, pack  # noqa: E402
from gen_witnesses import model_fold  # noqa: E402


def enc(indices, m, clear_every=None, clear_on_full=True):
    """Encode `indices` at `m` and pack it, using gen_vectors.py's encoder and the
    G10.2 width schedule (the same helper the witness gate uses)."""
    codes = lzw_codes(indices, m, clear_on_full=clear_on_full, clear_every=clear_every)
    ws, width, size, first = [], m + 1, (1 << m) + 2, True
    for c in codes:
        ws.append(width)
        if c == (1 << m):
            size, width, first = (1 << m) + 2, m + 1, True
            continue
        if c == (1 << m) + 1:
            continue
        if not first and size < 4096:
            size += 1
            if size == (1 << width) and width < 12:
                width += 1
        first = False
    return pack(codes, ws)


def main(n=200, seed=20261115):
    rng = random.Random(seed)
    recs = []
    for i in range(n):
        m = rng.choice([2, 2, 3, 4, 8, 8])
        A = rng.choice([1, 2, 3, 5, 8, 16, 40, 64, 100])
        if i % 2 == 0:
            # A valid stream: encode `A` random indices at `m`, sometimes with a
            # Clear every few codes so the reset path is exercised.
            idxs = [rng.randrange(1 << m) for _ in range(A)]
            every = rng.choice([None, None, 4, 8]) if A >= 8 else None
            D = enc(idxs, m, clear_every=every)
        else:
            # A random byte string, biased short so errors are common.
            ln = rng.choice([0, 1, 2, 3, 4, 6, 10, 20, 40])
            D = bytes(rng.randrange(256) for _ in range(ln))
        idx = []
        try:
            lzw_decode(m, D, A, lambda q, v: idx.append(v))
            err = None
        except GifError as e:
            err = e.name
        merr, mpos = model_fold(m, D, A)
        if merr is None:
            merr, mpos = None, None
        recs.append(dict(m=m, data=list(D), a=A, expected=idx, error=err,
                         mirror_error=merr, mirror_pos=mpos))
    out = ROOT / "tests" / "expected" / "random.json"
    out.write_text(json.dumps(recs) + "\n")
    agree = sum(1 for r in recs if r["error"] == r["mirror_error"] and
                (r["error"] is not None or r["expected"] == r["expected"]))
    names = {}
    for r in recs:
        k = (r["error"], r["mirror_error"])
        names[k] = names.get(k, 0) + 1
    print(f"wrote {out}: {len(recs)} records, {out.stat().st_size} bytes")
    print("oracle/mirror error-name agreement:", names)


if __name__ == "__main__":
    main()
