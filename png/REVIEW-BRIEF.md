# Review brief: PNG `THEOREMS.md` (M0)

You are reviewing `png/THEOREMS.md` before the Stage-1 freeze of `spec-draft/`. Write your findings to `png/REVIEW-SOL-THEOREMS.md` (or the reviewer's own name), each with a severity and the line it concerns. Do not edit `THEOREMS.md`, anything under `spec-draft/`, `policy-draft/`, or `policy-draft/probes/`; do not run Verus. Python with `-I` and `timeout` is fine.

Read, in order: `spec-draft/SPEC.md` (P1–P21, the target), `THEOREMS.md`, the precedents `../gif/THEOREMS.md` (with `../gif/ERRATA.md`, which records what went wrong in GIF's manifest after its freeze) and `../dib/THEOREMS.md`, the allocation shape `../alloc-delta/THEOREMS-ALLOC-DELTA.md` §0 and `../alloc-delta/SPEC.md` §2–§4 (with `../alloc-delta/ERRATA.md` E16 for `MAX_BYTES`), then `PLAN.md` (Milestones), `policy-draft/DECISIONS.md` and `policy-draft/OPTIONS.md` for the P rows, and `spec-draft/png_oracle.py` where the SPEC text is silent.

## What to check

1. **The spec is stated exactly.** Every model in the two models tables names the right P section, and no theorem quietly narrows or reorders it: the first chunk's P3 checks before `MissingIhdr` (P4), P5's duplicate, ordering and fixed-length order, P14.1's fcTL and fdAT check order, P9 steps 1–6, P10.5's header checks, P10.6's limit raised before the byte would be written and before any later check, the inflate-then-filter-then-index order of P11 (N3), P14.3's disposal of frame 0 and of the last frame, P15–P17, and P18's 62 names. Caps appear only by their P1 names; format and work constants by their names in "Constants".
2. **No theorem is weaker than its precedent without a stated reason.** Compare T1–T13 row by row with GIF's and DIB's: exactness including the first error and every output byte, both directions of T13, the 32/64-bit domain, integrated provenance (T9: same slice, no forged state), symbolic lemmas for every record above 4,096 bytes, mutation kills tied to theorems. Each weakening must be listed with its reason, or it is a finding.
3. **The allocation shape matches alloc-delta.** `G` clauses 1–3, `Prog` and `PreAlloc` are used with alloc-delta's definitions unchanged; every `Req` row is written in model terms; clause-3 bounds are proved before the guard call from `caps_ok`. Say whether the site set `Raw | Output | Saved | IccWindow | Icc` is complete (the rejected candidates are listed under "Allocation shape"), whether `Req_icc`'s three cases (D7) and the empty-profile request (D8) are acceptable, and whether placing `Saved` at the start of call `SavedAt` (D9) is the right point.
4. **T-ITER covers all call sequences.** Every reachable decoder state and every finite interleaving of `info()` and `try_next()`, including a stream without acTL, a default image outside the animation, an error in frame 1, an R4 fallback under RecommendedBeta, the state after the end, and a `Saved` `OutOfMemory`. The base case is the state after `MWF_p` and the default image's decode. Item 3 depends on D3; say which reading you would freeze.
5. **Nothing relies on an unlisted assumption.** Every premise used by a theorem must be proved, a stated precondition of reachability, or one of the trusted items of "Not proved, and trusted".
6. **The constants and the work bound.** `C_IN = 512`, `C_OUT = 8`, `C_INFL_FIXED = 4,096`, `C_CALL = 1,024`, `C_WORK = 2,048`, `C_FIXED = 65,536`, the step accounting, the per-call form of T12 and L-HDR-BITS (45 bits): are they consistent with SPEC P20, achievable by a plain implementation (in particular a dynamic header within 2,048 steps), and strong enough to kill mutation 34? Check the arithmetic of the summed bound.
7. **The inflate model is independent (lesson 3) and the interface is the one frozen.** `Inflate` must not share the executable's representation: it appends to a sequence, defines a match on that sequence and a code by its canonical codewords; L-CANON, L-WINDOW and L-RING are the only bridges. Check T-INFL's observable effects (written prefix, untouched suffix, unspecified `src` position and ring contents) against what `inflate_verus`'s callers can see, the "output exactly full" versus "another output byte exists" distinction, and D4, D5, D6 and D20 (the places where the SPEC text, `png_oracle.py` and zlib disagree or are silent). Confirm that `png_verus` uses `inflate_verus::model::Inflate` by path and calls the same public entry points, so the GIF E2 gap cannot arise (D17).
8. **Obligation R's scope and proof shape.** R binds RecommendedBeta and any record declared conservative, and nothing else; the compatibility profiles must not be claimed to preserve Strict acceptance or pixels. P-R's relation `≈` and its four synchronisation points must be strong enough for L-CONS (anything a later step reads is compared), and `apng_in_still = Ignore` must be handled by the omission of the APNG components at S1 for the still entry (D22). `OPTIONS.md` has no definition of a conservative record yet (D21): say whether P-R's text should be copied there before Stage 2.
9. **Allocation antecedents.** Every executable-to-model equivalence (T9, T10, T13, P-T9, P-BRIDGE) states its reverse direction and its error equivalences under `!matches!(r, Err(OutOfMemory(_)))`; an unconditional "iff" between a `try_*` result and a model is a finding.
10. **Symbolic caps.** No theorem row, lemma or gate uses a cap's value; `caps_ok` lists every derived fact a proof needs, and nothing else; gate CAPS-SYM is specified so that it would have caught GIF's finding 6. The input-cap raise of 2026-11-17 (D2) needs no proof change beyond `caps_ok` (d).
11. **The discrepancy list.** Confirm or refute D1–D23 and add any the author missed. D20 can be rechecked with a three-line script (a fixed block that ends right after a distance-30 code; compare `png_oracle.inflate` with `FIXED_DIST` of 30 and of 32 lengths). The cap-dependent record list of D19 can be rechecked by re-running every record with each cap doubled and halved.
12. **Witness gates.** Every gate name exists in `VECTORS.json` with the outcome the gate needs; the gates cover all 62 errors and every colour type and depth; the four records above 4,096 bytes are carried by L1–L4.

## Review focus: the iterator log and the `Saved` placement

alloc-delta's shape (`THEOREMS-ALLOC-DELTA.md` §0) has each `*_logged` call create its log with `new_log()` and return it; it has no form for one log that spans a constructor and many later calls, and `../alloc-delta/` is not edited for PNG. A-PNG-4 is GIF's extension (`../gif/THEOREMS.md` A-GIF-4, D9). Check that it keeps the full strength of clauses 1 and 2 across calls: (1) while no call has returned `OutOfMemory`, the requests logged so far are exactly `Req_new_p(b)` followed by `(Saved, 4·N)` if and only if call `SavedAt_p(b)` has been made, every entry `Ok`; (2) an `OutOfMemory` is returned only by call `SavedAt_p(b)`, its `(site, elements)` equal the log's last entry, which is the only failed entry, and the log never changes afterwards. A weaker reading (a log per call, or clause 2 without "the log never changes afterwards") is a finding. Then check T10's displacement sentence against D9: with `Saved` reserved before frame `SavedAt`'s data is inflated, `OutOfMemory` may replace any error of that frame; say whether a later point (after that frame's inflate) would be preferable.

## Status (2026-11-17)

The review is done (`REVIEW-SOL-THEOREMS.md`) and applied, the council has decided D3, D4, D5, D9 and D12, and the pre-freeze split below is done (`THEOREMS.md`, "Review findings applied" and "Discrepancies found"). The freeze procedure was rewritten for the review's finding 6, so that Stage 1 does not depend on the cap values Stage 2 may still choose. Before Stage 1: a focused re-review of the corrected rows (the review's last paragraph) and the owner's approval.

## Pre-freeze split (done 2026-11-17; mechanical, no behaviour change)

The freeze holds out the cap values and RecommendedBeta (PNG-Q7). They used to sit inside files the freeze copies (D19); they are now split out:

1. `limits-draft/` (not frozen at Stage 1):
   - `LIMITS.md`: the six values, the decisions behind them and the permitted envelope (below). SPEC P1 keeps the names, the bounds and the `caps_ok` conditions.
   - `limits.py`: the six constants. `png_oracle.py` loads `../limits-frozen/limits.py` if that directory exists, else `../limits-draft/limits.py`, by path, and writes no value.
   - `gen_vectors_caps.py` and `VECTORS-CAPS.json`: the 13 cap-dependent Strict records (the 12 found by the perturbation run plus `limits/exact-max-input-trailing` of D18), generated with the encoder helpers of `spec-draft/gen_vectors.py` and the values of `limits.py`.
2. `policy-draft/gen_vectors_policy.py` and `policy-draft/VECTORS-POLICY.json` (not frozen at Stage 1): the 198 `policy/*` records, 6 of them cap-dependent.
3. `spec-draft/gen_vectors.py` keeps the other 599 records and writes only `spec-draft/VECTORS.json`. `check_vectors.py` imports `policy_oracle` only for a record whose `profile` is not Strict, so the frozen check does not depend on `policy-draft/`. `validate.py` reads all three files.
4. `.gitignore` lists the three JSON files and their frozen copies while PNG-Q16 is undecided (`plte/length-8`, the input that panics png 0.18.1, is in the frozen file; DESIGN-NOTES N16). Each is reproduced from its deterministic generator and checked against its `FREEZE.txt`. Tracked: the generators, `limits.py`, `LIMITS.md`; not tracked: every `VECTORS*.json`.

Acceptance (all hold on 2026-11-17, `timeout 20m python3 -I` each):

- A1: the three files hold, each once and as JSON values, the 805 records of the unsplit `VECTORS.json` generated just before the split (SHA-256 `4959b7410e16116e30a9fa042ec3e311ae1539c716d587cbf0c2455475286fb2`: the 796 earlier records and 9 added by the review), plus the 5 D4 witnesses added to `gen_vectors.py` after the split; no earlier record changed. Current files: `spec-draft/VECTORS.json` `641d9b0f…`, `limits-draft/VECTORS-CAPS.json` `0e80fe91…`, `policy-draft/VECTORS-POLICY.json` `c716bce6…` (full digests in `policy-draft/DECISIONS.md`, "Review and council update");
- A2: `check_vectors.py` passes on each file: 599/599 with 57 of 62 errors (the other five, `FileTooLarge`, `OutputTooLarge`, `TooManyFrames`, `PixelBudgetExceeded`, `IccProfileTooLarge`, only in `VECTORS-CAPS.json`), 13/13, 198/198; inflate versus zlib agreement on 631 + 140 + 410 = 1,181 vector streams;
- A3: `check_bounds.py` and `check_over.py` pass (4,261,412,864 tuples, 0 violations);
- A4: `validate.py`: Strict bridge 784 inputs, 0 differences; obligation R 375 inputs, 0 violations; RecommendedBeta 224 of 287;
- A5: `png_oracle.py` and `gen_vectors.py` contain no cap value; SPEC.md contains none outside P11's worked checks and P20's live-heap illustration;
- A6 (finding 6): the 599 frozen records pass at the envelope's lower corner and at its upper corner (`check_vectors.py --caps`, values in `limits-draft/LIMITS.md`).

## The permitted cap envelope (finding 6)

A cap edit must not invalidate the frozen Stage-1 fixtures, and the Stage-1 hashes must not cover a value Stage 2 may change. Both follow from fixing, at Stage 1, the domain within which Stage 2 may choose values:

- the **perturbation domain** is every cap tuple that satisfies `caps_ok` (`THEOREMS.md`, Constants; among others `MAX_DIMENSION ≤ PNG_WIRE_MAX`, so no permitted change sets `MAX_DIMENSION` above the wire maximum) and puts each cap at or above the largest quantity a frozen record compares with it (`MAX_INPUT ≥ 13,110`, `MAX_DIMENSION ≥ 402`, `MAX_OUTPUT ≥ 1,608`, `MAX_FRAMES ≥ 3`, `MAX_TOTAL_PIXELS ≥ 24`, `MAX_ICC ≥ 768`; `LIMITS.md` names the record behind each bound);
- every frozen record keeps its outcome on the whole domain: each cap enters the Strict model only through one test `quantity > cap` whose quantity does not depend on any cap, so raising a cap above a record's quantities changes nothing, and a record whose quantity exceeds every permitted value (`ihdr/width-2^31`) fails the same test everywhere. The two corners are checked by `--caps`, and each bound is tight (one less fails one to five records);
- a value outside the domain is a Stage-1 erratum (a new Stage-1 version), never a Stage-2 choice. A value inside it needs no Stage-1 change: no re-split, no re-hash.

The earlier rule ("the split is re-done after every cap change") is dropped. The review's example, `MAX_DIMENSION = 1`, lies outside the domain (`container/minimal-ok` is 2 × 2).

## Stage 1: Strict decode rules (after the re-review and the owner's approval; not before)

`SPEC.md`'s status line is set to the frozen version and `THEOREMS.md`'s title changes from "M0 draft, not frozen" to "frozen (Stage 1)" before hashing. Then, from `~/projects/verified-decoders/png`:

```sh
set -eu
VERSION="PNG-CORE-SPEC 1.0"              # owner's choice
DATE=$(date +%F)
LO="MAX_INPUT=13110,MAX_DIMENSION=402,MAX_OUTPUT=1608,MAX_FRAMES=3,MAX_TOTAL_PIXELS=24,MAX_ICC=768"
HI="MAX_INPUT=0x1000000000000000,MAX_DIMENSION=0x7fffffff,MAX_OUTPUT=429496728,MAX_FRAMES=0xffffffff,MAX_TOTAL_PIXELS=0x4000000000000000,MAX_ICC=0x40000000"
test ! -e spec-frozen                     # never overwrite an existing freeze
test -f limits-draft/limits.py && test -f limits-draft/VECTORS-CAPS.json && test -f policy-draft/VECTORS-POLICY.json   # split done
timeout 20m python3 -I spec-draft/check_vectors.py                                   # expect 599/599, 57/62 errors
timeout 20m python3 -I spec-draft/check_vectors.py spec-draft/VECTORS.json --usage | tail -1 | grep -qx "lower corner: --caps $LO"
timeout 20m python3 -I spec-draft/check_vectors.py spec-draft/VECTORS.json --fuzz 0 --caps "$LO"   # expect 599/599
timeout 20m python3 -I spec-draft/check_vectors.py spec-draft/VECTORS.json --fuzz 0 --caps "$HI"   # expect 599/599
timeout 20m python3 -I spec-draft/check_vectors.py limits-draft/VECTORS-CAPS.json    # expect 13/13
timeout 20m python3 -I spec-draft/check_vectors.py policy-draft/VECTORS-POLICY.json  # expect 198/198
timeout 20m python3 -I spec-draft/check_bounds.py
timeout 20m python3 -I spec-draft/check_over.py                                      # 4,261,412,864 tuples, 0 violations
( cd policy-draft && timeout 20m python3 -I validate.py )                            # Strict bridge and obligation R, 0 violations
mkdir spec-frozen
F="SPEC.md DESIGN-NOTES.md SOURCES.md VECTORS.json png_oracle.py gen_vectors.py check_vectors.py check_bounds.py check_over.py"
for f in $F; do cp -p "spec-draft/$f" spec-frozen/; cmp "spec-draft/$f" "spec-frozen/$f"; done
( cd spec-frozen && timeout 20m python3 -I check_vectors.py --fuzz 0 --caps "$LO" && timeout 20m python3 -I check_vectors.py --caps "$HI" \
                 && timeout 20m python3 -I check_bounds.py && timeout 20m python3 -I check_over.py )
( cd spec-frozen
  { echo "# $VERSION (frozen $DATE), Stage 1: Strict decode rules only."
    echo "# NOT frozen: the cap values (../limits-draft), RecommendedBeta and all of ../policy-draft (PNG-Q7),"
    echo "# and the P rows of ../THEOREMS.md (Stage 2, changes in ../THEOREMS-POLICY-DELTA.md)."
    echo "# Cap premise: VECTORS.json's outcomes hold for every cap tuple satisfying caps_ok with each cap at least"
    echo "#   $LO"
    echo "# checked here at that lower corner and at the upper corner"
    echo "#   $HI"
    echo "# png_oracle.py loads ../limits-frozen/limits.py or ../limits-draft/limits.py; no cap value is hashed here."
    echo "# Re-check: sha256sum -c FREEZE.txt, then check_vectors.py --caps <either corner> (independent of limits.py),"
    echo "# or plain check_vectors.py at any permitted values. VECTORS.json is git-ignored while PNG-Q16 is open;"
    echo "# regenerate it with gen_vectors.py in a scratch copy, never inside spec-frozen/."
    sha256sum $F ../THEOREMS.md ../REVIEW-BRIEF.md
    ls ../REVIEW-SOL-THEOREMS*.md >/dev/null 2>&1 && sha256sum ../REVIEW-SOL-THEOREMS*.md
  } > FREEZE.txt
  sha256sum -c FREEZE.txt )
chmod a-w spec-frozen spec-frozen/*
```

What is hashed at Stage 1, and nothing else: `SPEC.md` (P1 with names, bounds and `caps_ok`, no values), `DESIGN-NOTES.md`, `SOURCES.md`, `VECTORS.json` (the 599 cap-independent Strict records), `png_oracle.py`, `gen_vectors.py`, `check_vectors.py`, `check_bounds.py`, `check_over.py`, `../THEOREMS.md`, `../REVIEW-BRIEF.md` and the review file(s). `limits.py` is not hashed: the frozen check reproduces the freeze for any `limits.py` inside the envelope, and `--caps` with all six caps makes it independent of `limits.py` altogether (the file must exist to be loaded; its values are then overridden). `check_bounds.py` reads `MAX_OUTPUT` and `MAX_DIMENSION`; its cap-boundary shapes are derived from `MAX_OUTPUT`, so it passes at every permitted value (checked at both corners on 2026-11-17), though the numbers it prints are those of the values loaded. The PNG and RFC source texts are not copied (they are not committed; `SOURCES.md` pins them by SHA-256 and gives the re-fetch commands).

What stays out at Stage 1: `limits-draft/` (values, cap-dependent vectors and their generator); `policy-draft/` entirely (`OPTIONS.md`, `QUESTIONS.md`, `DECISIONS.md`, `policy_oracle.py`, `validate.py`, `necessity.py`, `VECTORS-POLICY.json`, `gen_vectors_policy.py`, `probes/`); `PLAN.md`, `BRIEF.md`, `GAPS-CLOSED.md`. `THEOREMS.md` is hashed whole, but its rows marked "not frozen" or "Stage 2" (P-BRIDGE, P-T9, P-E, P-R, P-UNIV's policy part, T-CHUNKS, the profiled request rows, and the `MS16_p`/`MI_p`/`MChunks_p` models) are not bound by Stage 1; changes to them go into `THEOREMS-POLICY-DELTA.md` (the DIB precedent `../dib/policy/THEOREMS-POLICY-DELTA.md`), never into the hashed file.

## Stage 2: cap values and RecommendedBeta (after the M0-corpus review is accepted by the owner; not before)

Append-only: Stage 2 creates `limits-frozen/` and `policy-frozen/`, each with its own `FREEZE.txt` that pins `../spec-frozen/FREEZE.txt`, and never edits `spec-frozen/`, whose hashes stay valid. Run only after the corpus pass of `PLAN.md` M0-corpus has been re-run under the values in `limits-draft/LIMITS.md`, its report reviewed, and the owner has accepted it (PNG-Q7). A value change it causes is made in `limits-draft/` first and must stay in the envelope; `gen_vectors_caps.py` is re-authored if its assertions fail. Then, from `~/projects/verified-decoders/png`:

```sh
set -eu
DATE=$(date +%F)
test -e spec-frozen && ( cd spec-frozen && sha256sum -c FREEZE.txt )     # Stage 1 intact
test ! -e limits-frozen && test ! -e policy-frozen
( cd limits-draft && timeout 20m python3 -I gen_vectors_caps.py )        # re-authored records for the chosen values
timeout 20m python3 -I spec-draft/check_vectors.py limits-draft/VECTORS-CAPS.json    # expect 13/13 (or the re-authored count)
timeout 20m python3 -I spec-draft/check_vectors.py policy-draft/VECTORS-POLICY.json  # expect 198/198 (or the new count)
( cd policy-draft && timeout 20m python3 -I validate.py )
mkdir limits-frozen policy-frozen
L="LIMITS.md limits.py gen_vectors_caps.py VECTORS-CAPS.json"
for f in $L; do cp -p "limits-draft/$f" limits-frozen/; cmp "limits-draft/$f" "limits-frozen/$f"; done
P="OPTIONS.md policy_oracle.py validate.py gen_vectors_policy.py VECTORS-POLICY.json"
for f in $P; do cp -p "policy-draft/$f" policy-frozen/; cmp "policy-draft/$f" "policy-frozen/$f"; done
( cd spec-frozen && timeout 20m python3 -I check_vectors.py && timeout 20m python3 -I check_bounds.py \
                 && timeout 20m python3 -I check_vectors.py ../limits-frozen/VECTORS-CAPS.json )   # now loads limits-frozen: the chosen values are in the envelope
( cd limits-frozen
  { echo "# PNG cap values (frozen $DATE), Stage 2, after the M0-corpus review; inside the Stage-1 envelope; pins Stage 1:"
    sha256sum $L ../spec-frozen/FREEZE.txt
  } > FREEZE.txt
  sha256sum -c FREEZE.txt )
( cd policy-frozen
  { echo "# PNG RecommendedBeta and the policy record (frozen $DATE), Stage 2; compatibility profiles NOT frozen (PNG-Q14);"
    echo "# policy_oracle.py imports ../spec-draft/png_oracle.py by path: its hash and Stage 1's are recorded"
    sha256sum $P ../spec-draft/png_oracle.py ../spec-frozen/FREEZE.txt ../limits-frozen/FREEZE.txt
    ls ../THEOREMS-POLICY-DELTA.md >/dev/null 2>&1 && sha256sum ../THEOREMS-POLICY-DELTA.md
  } > FREEZE.txt
  sha256sum -c FREEZE.txt )
chmod a-w limits-frozen limits-frozen/* policy-frozen policy-frozen/*
```

What is hashed at Stage 2: in `limits-frozen/FREEZE.txt`, the four files of `L` and Stage 1's `FREEZE.txt`; in `policy-frozen/FREEZE.txt`, the five files of `P`, `png_oracle.py` as `policy_oracle.py` imports it, both earlier `FREEZE.txt` files and `THEOREMS-POLICY-DELTA.md` if it exists. `policy-frozen/` freezes RecommendedBeta's field values and the `recovered` list as `OPTIONS.md` states them; the GdkCompat-2.42.12 and ImageCrateCompat-0.25.10 columns stay proposals until P2 (PNG-Q14). After Stage 1, `BRIEF.md` gains the binding rules of `../dib/BRIEF.md`, `../dib/BRIEF-POLICY.md` and `../alloc-delta/SPEC.md`.

## Questions for the owner that block a freeze

None of D1–D23 is open: the TECHNICAL items are resolved as the review recommended, the five POLICY items by the council (2026-11-17), and D14–D16, D21, D22 are Stage 2 working proposals that bind nothing at Stage 1. Still for the owner: approval of Stage 1 after the focused re-review; PNG-Q16 (whether frozen vectors may be committed); D23's allowed initialisation cost, recorded, not a question unless the owner objects.
