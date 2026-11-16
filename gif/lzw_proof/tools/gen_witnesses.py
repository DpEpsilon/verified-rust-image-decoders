#!/usr/bin/env python3
"""Emit src/witnesses.rs: the ~12-stream T-LZW witness gate.

Each witness is a Verus obligation that `model::ML` on a concrete `D` (the packed
code stream, no sub-block bytes) and a concrete `(m, A)` yields a *specific* result.
Expected values come from `gif_oracle.py`'s `lzw_decode` (the reference the SPEC
names) run on the same `D`, mapped through a small adapter that turns the oracle's
error name and bit position into the model's `ErrK` and position convention.

The model and the oracle must agree on the index sequence and the error *name*;
the oracle reports no bit position, so the position is this crate's convention
(see src/model.rs's header) and is recorded, not derived from the oracle.
"""
import sys, json
from pathlib import Path

HERE = Path(__file__).resolve().parent
GIF = HERE.parent.parent.parent
sys.path.insert(0, str(GIF / "spec-draft"))
from gif_oracle import lzw_decode, GifError  # noqa: E402


def codes_for(name):
    """The small streams of the witness gate. Each returns (m, D, A)."""
    def pack(codes, widths):
        acc = nb = 0
        out = bytearray()
        for c, w in zip(codes, widths):
            acc |= c << nb
            nb += w
            while nb >= 8:
                out.append(acc & 255)
                acc >>= 8
                nb -= 8
        if nb:
            out.append(acc & 255)
        return bytes(out)

    def widths_of(codes, m):
        """The G10.2 widths for a code list (the oracle's own schedule)."""
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
        return ws

    def enc(codes, m):
        return pack(codes, widths_of(codes, m))

    out = {}
    m2 = 2
    # 1. Clear then EOI, A = 1: too few pixels.
    out["clear_then_eoi"] = (m2, enc([4, 5], m2), 1)
    # 2. Clear, literal 1, EOI, A = 1.
    out["one_literal"] = (m2, enc([4, 1, 5], m2), 1)
    # 3. Two literals, A = 2.
    out["two_literals"] = (m2, enc([4, 1, 2, 5], m2), 2)
    # 4. KwKwK: Clear, 1, 6 (== next), ... A = 3.
    out["kwkwk"] = (m2, enc([4, 1, 6, 5], m2), 3)
    # 5. A code beyond next: Clear, 1, 6 (== next at that point) is KwKwK, so the
    #    first invalid code is 7 with A = 1 (after literal 1, next = 6).
    #    (A = 2: with A = 1 the raster completes first, so step 4's
    #    MissingEndOfInformation would precede step 6's InvalidLzwCode.)
    out["code_beyond_next"] = (m2, enc([4, 1, 7], m2), 2)
    # 6. Empty data: no bits at all, A = 1 -> TooFewPixels.
    out["empty"] = (m2, b"", 1)
    # 7. Too many pixels in one string: after literals 1, 2 the entry 6 = "12" is
    #    added; the code 6 then emits two indices with only one pixel left (A = 3).
    out["too_many_pixels"] = (m2, enc([4, 1, 2, 6], m2), 3)
    # 8. Data after EOI: Clear, 1, EOI, then a full extra byte.
    d8 = enc([4, 1, 5], m2) + b"\x00\x00"
    out["data_after_eoi"] = (m2, d8, 1)
    # 9. Missing EOI after the raster: Clear, 1, 2 with A = 1 (the second literal
    #    follows a complete raster without an End of Information).
    out["missing_eoi"] = (m2, enc([4, 1, 2], m2), 1)
    # 10. Width change 7 -> 8 entries at m = 2: entries 6 and 7 are added by the
    #     second and third literal codes, so the fourth code is read 4 bits wide.
    out["width_change"] = (m2, enc([4, 1, 2, 3, 1, 5], m2), 4)
    # 11. m = 1 (RecommendedBeta): Clear, one 2-bit literal, A = 1.
    m1 = 1
    out["m1_one_pixel"] = (m1, enc([2, 1, 3], m1), 1)
    # 12. m = 8: Clear, literal 200, EOI, A = 1.
    m8 = 8
    out["m8_one_literal"] = (m8, enc([256, 200, 257], m8), 1)
    return out


def model_fold(m, D, A, checked_literals=False, stop_at_raster=False):
    """Independent Python mirror of src/model.rs's `ML` (string table, chain walk),
    including the crate's bit-position convention: the position is the number of code
    bits consumed when the failing check runs (the pre-read exhaustion uses the
    position *before* the failed read). It is a small decoder of its own, so the
    expected values the Verus obligations pin are not the model restated; a mistake
    here shows up as a false obligation, not a vacuous one."""
    clear, eoi = 1 << m, (1 << m) + 1
    nbits = 8 * len(D)
    pos, width, size, prev, q = 0, m + 1, clear + 2, None, 0
    t = {i: bytes([i]) for i in range(clear)}   # literals by rule
    out = bytearray()

    def read(n):
        v = 0
        for k in range(n):
            i = pos + k
            v |= ((D[i >> 3] >> (i & 7)) & 1) << k
        return v

    while True:
        if stop_at_raster and q == A:
            return None, bytes(out)
        if pos + width > nbits:
            return ("TooFewPixels" if q < A else "MissingEndOfInformation"), pos
        c = read(width)
        after = pos + width
        if c == clear:
            pos, width, size, prev = after, m + 1, clear + 2, None
            continue
        if checked_literals and c < clear and c >= 256:
            return "ColorIndexOutOfRange", after
        if q == A and c != eoi:
            return "MissingEndOfInformation", after
        if c == eoi:
            if q < A:
                return "TooFewPixels", after
            if not stop_at_raster and (after + 7) // 8 < len(D):
                return "DataAfterEndOfInformation", after
            return None, bytes(out)
        if c < clear:
            s = bytes([c])
        elif c < size:
            s = t[c]
        elif c == size and prev is not None and size < 4096:
            s = t[prev] + t[prev][:1]
        else:
            return "InvalidLzwCode", after
        if prev is not None and size < 4096:
            t[size] = t[prev] + s[:1]
            size += 1
            if size == (1 << width) and width < 12:
                width += 1
        pos, prev = after, c
        if q + len(s) > A:
            return "TooManyPixels", after
        out += s
        q += len(s)


def main():
    cases = codes_for(None)
    recs = []
    for name, (m, D, A) in cases.items():
        idx = []
        err = None
        def emit(q, i):
            idx.append(i)
        try:
            lzw_decode(m, D, A, emit)
            err = None
        except GifError as e:
            err = e.name
        mpos = 0
        if m == 1:
            merr, mout = model_fold(m, D, A, checked_literals=True, stop_at_raster=True)
        else:
            merr, mout = model_fold(m, D, A)
        if merr is not None:
            assert merr == err, f"{name}: mirror says {merr}, oracle says {err}"
        else:
            assert list(mout) == idx, f"{name}: mirror {list(mout)} vs oracle {idx}"
        recs.append(dict(name=name, m=m, data=list(D), a=A, expected=idx, error=err))
    # The positions, read from the mirror once (they are this crate's convention).
    for r in recs:
        if r["error"] is not None:
            if r["m"] == 1:
                _, p = model_fold(r["m"], bytes(r["data"]), r["a"],
                                  checked_literals=True, stop_at_raster=True)
            else:
                _, p = model_fold(r["m"], bytes(r["data"]), r["a"])
            r["pos"] = p
    out = HERE.parent / "tests" / "witnesses.json"
    out.write_text(json.dumps(recs, indent=1) + "\n")
    print(f"wrote {out} ({len(recs)} witnesses)")
    for r in recs:
        print(r["name"], "m=", r["m"], "A=", r["a"], "data=", r["data"], "->",
              r["error"] or r["expected"])


if __name__ == "__main__":
    main()
