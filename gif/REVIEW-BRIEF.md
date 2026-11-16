# Review brief: GIF `THEOREMS.md` (M0)

You are reviewing `gif/THEOREMS.md` before `spec-draft/` is frozen. Write your findings to `gif/REVIEW-SOL-THEOREMS.md` (or the reviewer's own name), each with a severity and the line it concerns. Do not edit `THEOREMS.md` or anything under `spec-draft/`; do not run Verus. Python with `-I` and `timeout` is fine.

Read, in order: `spec-draft/SPEC.md` (G1–G20, the target), `THEOREMS.md`, the precedents `../dib/THEOREMS.md`, `../tga/THEOREMS.md`, `../icns/THEOREMS.md`, the allocation shape `../alloc-delta/THEOREMS-ALLOC-DELTA.md` §0 and `../alloc-delta/SPEC.md` §2–§4, then `PLAN.md` (Milestones), `DECISIONS.md` and `policy-draft/OPTIONS.md` for the P rows.

## What to check

1. **The spec is stated exactly.** Every model in the models table names the right G section, and no theorem quietly narrows or reorders it: error precedence (G16), the label check before every other extension check (G6), rule A for NETSCAPE2.0 (G6.3), the two-phase image data (G9), the LZW loop order and its interleaving with the colour check (G10.2, G12.2; lemma L-INTERLEAVE), disposal order and the last frame's disposal (G12.3), the constructor's prefix walk (G15), `plays` (G20). Caps appear only by their G1 names.
2. **No theorem is weaker than its precedent without a stated reason.** Compare T1–T13 row by row with DIB's T1–T13, TGA's T1–T11 and ICNS's T1–T13: exactness including the first error and every output byte, both directions of T13, the 32/64-bit domain, integrated provenance (T9: same slice, no forged state), symbolic lemmas for every vector above 4,096 bytes, mutation kills tied to theorems. Each weakening must be listed with its reason, or it is a finding.
3. **The allocation shape matches alloc-delta.** `G` clauses 1–3, `Prog` and `PreAlloc` are used with alloc-delta's definitions unchanged; `Req_still`, `Req_new` and `Req_next` are written in model terms; clause 3 bounds are proved before the guard call. Say whether `ReqLzw = [(LzwTable, 20,480)]`, one byte-counted `try_zeroed` request (D4), and the `Saved` placement and fused state after its `OutOfMemory` (D8) are acceptable as G18 now states them.
4. **T-ITER covers all call sequences.** The statement must hold for every reachable decoder state and every finite interleaving of `info()` and `next_frame()`, including zero frames with a Trailer, an error at the first image (zero frames, error end), the fused state after an error, and a `Saved` `OutOfMemory`. Check that the base case is the state after `MP`, and that the `recovered` field is covered under every record.
5. **Nothing relies on an unlisted assumption.** Every premise used by a theorem must be either proved, a stated precondition of reachability, or one of the trusted items in the last-but-one section (A-STD-1..4, the guard's `Err` binding, the audit allowlist and build-input inventory, vstd, Verus, rustc).
6. **The constants and the work bound.** `C_CALL = 32`, `C_WORK = 64`, `C_FIXED = 41,984`, the step accounting and the per-call form of T12 (D5–D7, D18, D20): are they consistent with G18, achievable by a plain implementation, and strong enough to kill mutation 24? D20 is resolved: literal codes are decided by rule, with no per-frame initialisation; check that no text still assumes stored literal entries, and that every invariant uses `code_length` (1 for a literal code) rather than the raw `stored_length` accessor (R11).
7. **The discrepancy list.** Confirm or refute D1–D20 and the resolutions recorded against them, and add any the author missed. D11 can be rechecked with an instrumented run of `gif_oracle.py` on `lzw-m2-reaches-12-bits`.
8. **Obligation R's scope.** R binds RecommendedBeta and any record declared conservative in `policy-draft/OPTIONS.md` (every non-Strict value error-site-only), and nothing else: SPEC G14/G15, P-R and `OPTIONS.md` must say the same, and the compatibility profiles must not be claimed to preserve Strict acceptance or pixels. Error-site-only is defined as a simulation, not lockstep (R10): the relation `≈` compares named components (cursor, pending Graphic Control, `k`, `T`, `loop_count`, canvas, previous-frame state, returned frame records; inside a frame also the LZW state and event set) at three kinds of synchronisation point (block boundaries, LZW iterations up to raster completion, the frame's end). Check that this relation is strong enough for P-R's induction (lemma L-CONS) and implies R's three forms, that no compared component is missing (anything a later step reads must be compared at S1 and S3), that every value RecommendedBeta uses satisfies it, and that `lzw_tail=StopAtRaster` is the only value that needs the post-raster tail lemma L-TAIL (the other nine are pure error-site replacements).
9. **Allocation antecedents.** Every executable-to-model equivalence (T9, T10, T13, P-T9) must state its reverse direction and its error equivalences under `!matches!(r, Err(OutOfMemory(_)))`, as alloc-delta's clause 1 does; an unconditional "iff" between a `try_*` result and a model is a finding.

## Review focus: the iterator log (D9)

alloc-delta's shape (`THEOREMS-ALLOC-DELTA.md` §0) has each `*_logged` call create its log with `new_log()` and return it; it has no form for one log that spans a constructor and many later calls, and `../alloc-delta/` is not edited for GIF. A-GIF-4 is the extension. Check that it keeps the full strength of clauses 1 and 2 across calls, not only per call: (1) while no call has returned `OutOfMemory`, the requests logged so far are exactly `Req_new(b)` followed by `(Saved, 4·N)` if and only if the first disposal-3 frame has been reached, every entry `Ok`; (2) an `OutOfMemory` is returned only by the call that reserves `Saved`, its `(site, elements)` equal the log's last entry, which is the only failed entry, and the log never changes afterwards. A weaker reading (for example a log per call, or clause 2 without "the log never changes afterwards") is a finding.

## Freeze (after the review is applied and the owner approves; not before)

The `SPEC.md` items among D1–D20 and R1–R13 are resolved in `spec-draft/` (see `THEOREMS.md`, Discrepancies found); the owner settles any still marked Open, sets `SPEC.md`'s status line to the frozen version, and changes `THEOREMS.md`'s title from "M0 draft" to "frozen" (before hashing, since `FREEZE.txt` records its hash). Then, from `~/projects/verified-decoders/gif`:

```sh
set -eu
VERSION="GIF-CORE-SPEC 1.0"              # owner's choice
DATE=$(date +%F)
test ! -e spec-frozen                     # never overwrite an existing freeze
timeout 900 python3 -I spec-draft/check_vectors.py      # expect 373/373, 32/33 errors exercised
timeout 300 python3 -I spec-draft/check_plays.py        # expect 65537/65537
timeout 900 python3 -I policy-draft/validate.py         # Strict bridge and obligation R (RecommendedBeta), 0 violations
mkdir -p spec-frozen/sources
cp -p spec-draft/SPEC.md spec-draft/DESIGN-NOTES.md spec-draft/SOURCES.md spec-draft/VECTORS.json \
      spec-draft/gif_oracle.py spec-draft/gen_vectors.py spec-draft/check_vectors.py spec-draft/check_plays.py \
      spec-frozen/
cp -p spec-draft/sources/spec-gif89a.txt spec-frozen/sources/
diff -r -x __pycache__ spec-draft spec-frozen           # must print nothing
( cd spec-frozen && timeout 900 python3 -I check_vectors.py && timeout 300 python3 -I check_plays.py )
( cd spec-frozen
  { echo "# $VERSION (frozen $DATE); RecommendedBeta and policy-draft NOT frozen (Q9)"
    echo "# check_vectors.py imports ../policy-draft/policy_oracle.py, which imports ../spec-draft/gif_oracle.py;"
    echo "# both hashes at freeze are recorded below (D16)"
    sha256sum SPEC.md DESIGN-NOTES.md SOURCES.md VECTORS.json gif_oracle.py gen_vectors.py \
              check_vectors.py check_plays.py sources/spec-gif89a.txt \
              ../THEOREMS.md ../REVIEW-BRIEF.md ../policy-draft/policy_oracle.py ../spec-draft/gif_oracle.py
    ls ../REVIEW-SOL-THEOREMS*.md >/dev/null 2>&1 && sha256sum ../REVIEW-SOL-THEOREMS*.md
  } > FREEZE.txt
  sha256sum -c FREEZE.txt )
chmod a-w spec-frozen spec-frozen/* spec-frozen/sources spec-frozen/sources/*
```

There is one `FREEZE.txt`, in `spec-frozen/`; it lists every frozen file and the files outside `spec-frozen/` that the freeze depends on, and no other directory gets its own. `policy-draft/policy_oracle.py` is frozen together with the spec in the sense that its hash is recorded there (D16): the frozen `check_vectors.py` imports it from `../policy-draft/` for the 33 profile records, and it in turn imports `../spec-draft/gif_oracle.py`, so both are hashed, and a re-run of the frozen check reproduces the freeze only while `sha256sum -c FREEZE.txt` passes. RecommendedBeta and the rest of `policy-draft/` stay unfrozen (Q9). `gen_vectors.py` is frozen for provenance and must not be run inside `spec-frozen/` (it rewrites `VECTORS.json`). After the freeze, `BRIEF.md` gains the binding rules of `../dib/BRIEF.md`, `../dib/BRIEF-POLICY.md` and `../alloc-delta/SPEC.md`.
