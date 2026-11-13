# PROOF-REPORT-M4a — T9 (ICO-side linear work), T10 (allocation-call bound), T11 (witness bridge)

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), toolchain
1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`, dependency
`dib_verus` (path `../dib/dib_verus`, version 0.3.0, unmodified).
**Final result line:** `verification results:: 303 verified, 0 errors`
(`runs/126.log`, a full reverify after `cargo clean`).
**Conformance:** `cargo test --release -- --nocapture` →
`conformance: 217/217 vectors passed`, `test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 35 (runs 92–126 of `RUNLOG.tsv`; runs
92–98 were a throwaway feasibility probe crate at `ico/probe`, deleted afterwards).
Eighteen exited 0 (sixteen intermediate green builds, the T10 audit-only mutation
probe, and the final clean reverify); seventeen exited 101 (proof-development
iterations, one parse of a `&&&`-block form, and the T9 sensitivity probe). No
obligation hit the five-failure stop rule; no `STUCK.md`; `QUESTIONS.md` stays
empty.
**Tool calls:** about 150 (my own count; the harness count is authoritative). Most
were verifier runs, generator runs and the two probe crates.
**Executable behaviour:** byte-identical. The changes are (a) threading a ghost
`&mut Ghost<nat>` step counter and a ghost `&mut Ghost<nat>` allocation counter
through `parse`/`decode_entry`; (b) turning the two loops into tracked loops that
increment the ghost counter each body iteration; (c) recording the DIB call's
existing ghost return (previously `_steps`). Every ghost argument and update
erases, so `parse` and `decode_entry` compute exactly what they did at M3; the
public bodies stay `parse`/`decode_entry` (now thin wrappers over tracked
helpers). 217/217 still pass; M3's proofs are unchanged in strength.

## Tick sheet (T1–T11)

| ID | Status | Proof items |
| --- | --- | --- |
| T1 safety | proved (M2) | `parse`/`decode_entry` bodies, `t1_decode_ico`, `t1_index_u32` |
| T2 exact parse | proved (M2) | `parse` ensures `parse_eq`, `t2_parse_exact` |
| T3 record validity | proved (M2) | `t3_record_validity`, `lemma_parse_ok_well_formed` |
| T4 signature classification | proved (M2) | `t4_signature_classification`, `lemma_parse_ok_classified` |
| T5 exact total decode | proved (M3) | `decode_entry` ensures `md_eq`, `t5_decode_exact`, `lemma_md_eq_dib` |
| T6 exact precedence | proved (M3) | `t6_precedence`, `t6_precedence_full`, `t6_parse_directory_priority`, `t6_decode_ordering`, `lemma_t6_*` |
| T7 arithmetic/output bounds | proved (M2/M3) | `t7_arithmetic_bounds`, `t7_dib_output_bound` |
| T8 termination | proved (M3) | `t8_termination`, `lemma_t8_*` |
| T9 ICO-side linear work | **proved (M4a)** | `t9_parse_linear_work`, `t9_wrapper_linear_work`, `t9_wrapper_ignores_list`, `lemma_t9_parse_bound`, `lemma_t9_wrapper_bound`, `lemma_t9_total`, `parse_step_bound`, `wrapper_own_bound`, `dib_steps_bound`, `t9_step_cap` |
| T10 allocation-call bound | **proved + audited (M4a)** | `t10_allocation_bound`, `lemma_t10_parse_header_error_no_alloc`, `lemma_t10_parse_one_bounded`, `lemma_t10_decode_no_alloc`, `tools/audit_alloc.sh` |
| T11 witness bridge | **proved (M4a)** | `model_vectors.rs` (170 `agree_*`), `t11_valid_selected_beside_invalid_unselected`, `t11_forged_selected_invalid`, `t11_forged_unselected_ignored`; all 31 named witnesses below |

**Witnesses by name, with proof items (all 31 from the manifest's "Non-vacuity and
symbolic boundaries" list).** 30 are `agree_parse_<name>` / `agree_decode_<name>`
in `model_vectors.rs`; the last is a `witnesses.rs` public call. Each name below is
one generated proof item unless noted.

`short-dir-0`, `reserved-1`, `type-0`, `empty-directory-1`, `count-cap-1025`,
`short-table-15`, `dimensions-0-0-type1`, `words-65535-0-type2`,
`entry-reserved-1`, `size-0`, `directory-overlap-21`, `range-4294967295-1`,
`png-prefix-followed-by-rest-7`, `png-bit-0-parse`, `png-bit-7-parse`,
`png-shifted`, `table-boundary-record0-off22`, `table-boundary-record0-off38`,
`wide-size-10001`, `wide-offset-10016`, `straddling-ranges`,
`directory-order-large-first`, `valid-then-bad-oob`, `bad-then-valid-reserved`,
`forged-wrong-kind`, `wire-invalid-forged-valid`, `wire-invalid-forged-defect`,
`type-257`, `reserved-256`, `decode-index-priority` — `agree_*` obligations;
`forged-unselected-ignored` — `t11_forged_unselected_ignored`.

## What was built

- `ico_verus/src/model.rs` (1,900 lines) gains the M4a model: `parse_step_bound`,
  `wrapper_own_bound`, `dib_steps_bound`, `t9_step_cap`, `parse_alloc_calls`,
  `decode_alloc_calls`, the T9/T10 discharge lemmas, the `dib_steps_bound`
  non-negativity and off-DIB lemmas, the `md_eq` converse lemmas
  (`lemma_md_eq_to_err`/`_to_ok`), the subrange helpers `lemma_subrange_index_at`/
  `lemma_subrange_eq_lit`, and the symbolic boundary lemmas `lemma_l1_over_cap`,
  `lemma_l2_exact_cap`, `lemma_l3_record_induction`, `lemma_l4_valid_range`.
- `ico_verus/src/ico.rs` (1,675 lines): `parse`/`decode_entry` are unchanged public
  functions, now wrapping `parse_tracked`/`decode_entry_impl`; `has_png_signature`,
  `record_status`, `reconstruct`, `parse_tracked`, `decode_entry_impl` take and
  charge the ghost counters; new public tracked exports `parse_with_steps` and
  `decode_entry_with_steps`; new public witnesses `t9_parse_linear_work`,
  `t9_wrapper_linear_work`, `t9_wrapper_ignores_list`, `t10_allocation_bound`.
- `ico_verus/src/model_vectors.rs` (6,573 lines, generated): 170 model-vs-vector
  agreement obligations (116 `MC`, 54 `MD`), each closed by `by (compute)`.
- `ico_verus/src/witnesses.rs` (224 lines, hand-written): the three T11 public-call
  witnesses.
- `tools/gen_model_vectors.py`: reads `spec-frozen/VECTORS.json` only; `--check`
  is up to date.
- `tools/audit_alloc.sh`: the source-level allocation enforcement.

## T9 · ICO-side linear work — proved

Every ICO-controlled real loop iteration carries a ghost step:

- the record loop body in `reconstruct` charges **one** per `ME` iteration
  (`*steps = Ghost(steps@ + 1)` at the top of `reconstruct`);
- the eight-byte signature comparison in `has_png_signature` charges **one per
  comparison iteration** (at most eight, invariant `steps@ <= s0 + k`);
- the bounded list initialisation in `parse_tracked` (`Vec::with_capacity`)
  charges **one**;
- the directory guard has no loop, so a directory-wide error charges nothing;
- `decode_entry_impl` charges **one** for the selected-record revalidation's
  single signature comparison.

The bound is stated on the two public tracked exports at the manifest's
quantification (`∀ b, E, i`):

- `parse_with_steps(b)` ensures `r.1@ <= 16*(|b|+1)` and `r.1@ <= parse_step_bound(b)`,
  where `parse_step_bound(b) = 0` on `H`'s error and `1 + 9*count` otherwise.
  `lemma_t9_parse_bound` proves `parse_step_bound(b) <= 16*(|b|+1)` from
  `hr.dir_end = 6 + 16*count <= |b|` and `1 <= count <= 1024`
  (`1 + 9*count <= 16*(6 + 16*count + 1)`).
- `decode_entry_with_steps(b, E, i)` ensures `r.1@ <= 16*(|b|+1)` and
  `r.1@ <= t9_step_cap(b,E,i)`; off the selected-DIB decision `r.1@ <= wrapper_own_bound() = 16`.

The wrapper's constant work: `wrapper_own_bound() = 16`. The manifest's "counts at
most eight signature-byte checks" is satisfied — the selected-record work is one
`reconstruct` (whose interior loop is the eight-step signature comparison, already
charged inside) plus one explicit step; ten actual charges at most, 16 a safe
ceiling. **It does not scan `E` or parse all records:** the loop body runs once,
the list is read only at `entries[index]`, and `t9_wrapper_ignores_list` proves the
decision (hence the step cap) is a function of `|E|` and `E[index]`
(`lemma_t6_decode_reads_only_selected`).

### The combined statement (the manifest's T9 row, exact wording)

The manifest's T9 row reads: *"No in-crate theorem currently claims an end-to-end
numeric DIB+ICO bound … The DIB 1.1 proposal exports … public `decode_with_steps`
with `steps_DIB<=16*(|slice|+N)` … ICO calls that same function. Only after DIB
proves this new public contract can ICO derive a combined bound from both
counters."* That dependency is now real: the shipped `dib_verus::decode_with_steps`
carries the **proved** postcondition `r.1@ as int <= 16*(|bytes| + t12_checked_n(bytes, mode))`.
So the combined statement is stated and proved:

`dib_steps_bound(b,E,i)` = the DIB call's own bound `16*(|slice|+N)` on the
selected DIB (slice = `b.subrange(start,end)`, `N = dib_verus::dib::t12_checked_n`),
and `0` on every other decision. `t9_step_cap(b,E,i) = wrapper_own_bound() + dib_steps_bound(b,E,i)`.
`decode_entry_with_steps` ensures:

- `r.1@ <= t9_step_cap(b,E,i)` — the wrapper's total charged steps are within its
  own constant plus the DIB call's count;
- `r.2@ <= dib_steps_bound(b,E,i)` — the DIB call's returned ghost count obeys the
  dependency's published bound (this is the DIB half, taken straight from
  `decode_with_steps`'s postcondition on the exact validated slice);
- `!(md_ico(b,E,i) is DibSlice) ==> r.2@ == 0` — no DIB call off the selected-DIB
  decision (the manifest's "invokes DIB at most once"); `lemma_dib_steps_bound_zero_off_dib`
  and `lemma_dib_steps_bound_nonneg` discharge it.

`lemma_t9_total` is the named composition: from the wrapper's own-work bound and
the DIB call's published bound it yields `steps <= t9_step_cap`. **What is claimed
exactly:** the ICO-side counter (this crate's `16*(|b|+1)`) plus the DIB
`decode_with_steps` counter stay within the sum of the two crates' proved bounds —
not a single fused numeric constant, and only because ICO invokes the very
`decode_with_steps` whose postcondition is exported. The DIB 1.0 `decode` exports
no step count, so no such combination exists for it; ICO does not name a private
witness. The DIB half is the dependency's own proof, cited, not re-proved here.

## T10 · allocation-call bound and audit — proved (shape) + audited (enforcement)

`parse_with_steps(b)` ensures `r.2@ == parse_alloc_calls(b)` and `r.2@ <= 1`, where
`parse_alloc_calls(b) = 0` on `H`'s error (the guard returns before the single
reservation; `lemma_t10_parse_header_error_no_alloc`) and `1` otherwise
(`lemma_t10_parse_one_bounded`, `1 <= count <= 1024`). The one charge sits on the
literal `Vec::with_capacity(header.count)`. `decode_entry_with_steps` ensures
`r.3@ == 0` (`decode_alloc_calls = 0`, `lemma_t10_decode_no_alloc`), and the wrapper
calls DIB at most once.

**Honest label (the manifest's own words):** a Verus-verified ghost count *cannot
detect a new uncharged `vec!`, clone, `collect` or formatting call.* The source
audit is the enforcement. `tools/audit_alloc.sh` strips comments, counts
`Vec::with_capacity` (expected exactly 1, in `parse_tracked`) and `vec!` (expected
0), and fails on any other `Vec::`/`vec![`/`to_vec`/`clone`/`to_owned`/`String::`/
`format!`/`Box::new`/`from_iter`/`iter::repeat`/`repeat_n`/`into_boxed`/`resize`/
`reserve`/`extend`/`collect`/`Rc::`/`Arc::`/`to_string`. On the live source it
prints `OK`. On five mutated copies it **FAILS** (an added `vec![0u8;4]`,
`Vec::with_capacity(4)`, `to_vec()`, `clone()`, `collect()` each reported). This is
a finite grep heuristic, as the DIB review noted of its sibling; manual review
remains part of the gate. No physical heap-peak or `Vec`-capacity theorem is
claimed. DIB's own allocation audit stays separate.

## T11 · witness bridge — proved

- **`MC` vs expected records.** `model_vectors.rs` states `mc(b) == <fixture's
  expected parse Result>` for every parse vector (and `md(...) == <expected decode
  Result>` for every decode vector whose decision never reaches the DIB branch),
  discharged by `by (compute)`. This is the manifest's "small subset of
  independently authored expected records", taken from `expected`/`entries_override`
  in `spec-frozen/VECTORS.json` (never from the executable; the supplied non-override
  decode lists are transcribed independently from the wire by the generator).
- **All 31 named non-vacuity witnesses** in the manifest's "Non-vacuity and symbolic
  boundaries" list have a proof item: 30 are `agree_parse_<name>`/`agree_decode_<name>`
  obligations in `model_vectors.rs`; `forged-unselected-ignored` is
  `t11_forged_unselected_ignored` in `witnesses.rs`.
- **Valid selected beside invalid unselected.** `t11_valid_selected_beside_invalid_unselected`
  calls the public `decode_entry` on a 46-byte two-record file (record 0 valid PNG
  `[38,46)`, record 1 invalid out-of-file `EntryOutOfBounds`) and proves the result
  is exactly `Err(PngDelegation)`, so the invalid unselected record cannot change
  the selected result.
- **A forged selected record producing `InvalidEntryRecord`.**
  `t11_forged_selected_invalid` calls the public `decode_entry` on the
  `forged-wrong-kind` fixture (supplied kind `Png`, wire kind `Dib`) and proves
  `Err(InvalidEntryRecord)`.
- **`forged-unselected-ignored`.** `t11_forged_unselected_ignored` calls the public
  `decode_entry` with a *forged* unselected record 1 and proves
  `Err(PngDelegation)`, and (`by (compute)`) that `MD`'s decision is unchanged by
  the forged element.

All three public calls use the selected-PNG allowance the manifest gives
("`forged-unselected-ignored` can use a selected PNG signature"), so **no proof-gate
witness evaluates a concrete DIB `MP_1.1`**. The manifest's instruction is honoured:
in Verus, no `md` obligation reaches the DIB branch — 41 decode vectors whose expected
result is a DIB success or a `Dib(...)` error are excluded from the generated
obligations and remain runtime conformance vectors; the generic T5 slice-model lemma
(`lemma_md_eq_dib`, on the same `decode_with_steps` call) supplies the exact pixel
bridge.

### Symbolic boundary lemmas

- **L1** `lemma_l1_over_cap(b, E, i)`: every `|b| > MAX_INPUT` is `FileTooLarge` on
  `parse` and `decode_entry`, for every `E` and `i`.
- **L2** `lemma_l2_exact_cap(b)`: every slice of exactly `67,108,864` bytes whose
  first 30 bytes are `file-size-67108864`'s one-PNG-entry prefix yields
  `mc(b) == Ok(seq![l2_record()])` (one validated range `[22, 30)`). Proved from a
  30-byte prefix via `lemma_subrange_index_at`/`lemma_subrange_eq_lit` — the
  symbolic length lemma, **not** a 64 MiB verifier expansion.
- **L3** `lemma_l3_record_induction(b, hr, j)`: `MC` on a successful directory is the
  `count`-element list of `ME`, its `j`-th element exactly `ME(b,hr,j)`, no record
  aborts the list.
- **L4** `lemma_l4_valid_range(b, hr, i)`: a valid record satisfies
  `dir_end <= offset`, `size > 0`, `offset <= |b|`, `size <= |b| - offset`. The
  16-bit-narrowing and u32-wrap fixtures (`wide-size-10000`, `wide-offset-10016`,
  `wide-sum-wrap`) are covered as parse vectors.

## Executable changes

Ghost-only and behaviour-preserving (217/217 pass):

1. `parse` → thin wrapper over `parse_tracked(bytes, &mut steps, &mut alloc)`;
   `parse_tracked` is line-for-line the M3 `parse` plus the two `&mut Ghost<nat>`
   parameters, with `*steps = Ghost(steps@ + 1)` for the list initialisation and
   (inside `reconstruct`) the record body.
2. `decode_entry` → thin wrapper over `decode_entry_impl(bytes, entries, index, &mut steps, &mut dib_steps, &mut alloc)`; the DIB call's ghost return is now stored
   in `dib_steps` instead of `_steps`.
3. `has_png_signature`, `record_status`, `reconstruct` take and charge `steps`.
4. New public tracked exports `parse_with_steps`, `decode_entry_with_steps`, and
   the T9/T10/T11 public witnesses.

Every `Ghost` argument, local and update erases; the executable is byte-for-byte
M3's.

## Sensitivity check (disposable copies, `VERIFY_CRATE=`, deleted afterwards)

- **T9 charged rescan (an M20-like mutation):** an extra
  `*steps = Ghost(steps@ + (i as nat))` added in `parse_tracked`'s loop body before
  the `reconstruct` call (so `reconstruct`'s own one-step contract is untouched,
  but the loop's cumulative charge is no longer linear). Killed by a verifier
  diagnostic, `invariant not satisfied at end of loop body` — 1 error (run 120). A
  real verifier kill of the T9 obligation.
- **M21 uncharged `vec!` (audit-only):** an added `let _scratch: Vec<u8> = vec![0u8;4];`
  in `decode_entry_impl`. The verifier still reports **303 verified, 0 errors**
  (run 121) — the manifest's honest label: the ghost allocation counter **cannot**
  detect an uncharged allocation. `tools/audit_alloc.sh` **FAILS** on that copy
  (`vec! allocations: 1 (expected 0)`), which is the enforcement.

No other M4b mutations were run; that campaign is M4b.

## Assurance boundary

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|
todo!|unimplemented!|verifier::external|verifier::rlimit|requires\(false\)|
unwrap\(\)|expect\(|panic!|rlimit"` over `ico.rs`, `model.rs`, `lib.rs`,
`witnesses.rs`, `model_vectors.rs` finds nothing outside comments. No
`#[verifier::rlimit]` or other solver-budget attribute; no `STUCK.md`.
`spec-frozen/` hashes unchanged (`SPEC.md 57b6dd8d…`, `VECTORS.json 5ec052ad…`);
`../dib` unmodified.

Honest caveats:

- **T9 is a charged-step bound, not a physical-work theorem.** It bounds the
  charged iterations of the loops this crate owns; a new loop that escapes charging
  is invisible to it, exactly as T10's ghost count is invisible to an uncharged
  allocation. The loops in `ico.rs` are exactly two (`while` in `has_png_signature`
  and in `parse_tracked`), both charged.
- **The combined bound is the sum of two crates' counters**, valid only because
  ICO calls the DIB 1.1 `decode_with_steps` whose step postcondition is exported.
  The DIB arithmetic is the dependency's own proof, not re-derived here.
- **T10 enforcement is the source audit** (a finite grep), and the audit's own
  limits are the DIB review's (a heuristic, manual review still required).
- **The generator is a trusted tool run once**: its output (`model_vectors.rs`) is
  committed source whose obligations each close or fail on their own; `--check`
  guards regeneration. The expected values are the frozen oracle's, but the
  *transcription* (R12 expansion, R4-R6 reconstruction) is the generator's, itself
  subject to review.
- **32-bit target** remains an argument on the constants, unchanged from M2.
- **M4b** (the full executable-only mutation campaign) is untouched.

## Surprises

1. **`&&&` needs a block inside a `match` arm.** `IcoDecision::DibSlice(..) => &&& p
   &&& q` did not parse (`leading &&& must be inside a block`); wrapping the arm body
   in `{ ... }` fixed it.
2. **A slice/array exec-local `.view()` (`bs@` after `let bs: &[u8] = &bytes;`)
   trips a Verus internal AIR error** (`use of undeclared function …slice…view`),
   and `by (compute)` over a *parameter* seq panics the interpreter
   (`vir/src/interpreter.rs`). The T11 witnesses therefore pin the fixture with
   `requires bytes@ == <spec literal>` and compute on the **spec literal**, then
   connect to the executable result through `lemma_md_eq_to_err`. This is the same
   shape as the M3 public witnesses.
3. **`by (compute)` closes `mc`/`md` equalities including input-seq indexing** — the
   same assurance the DIB crate's `model_vectors.rs` uses. The DIB-branch exclusion
   is what keeps it inside the T11 allowance.
4. **Extending the public tracked return tuples** (adding `dib_steps`, `alloc`)
   rippled into every call site and the ensures; the four-tuple `decode_entry_with_steps`
   is the price of stating the combined bound on the executable.
5. **`reconstruct`'s charging is iteration-relative**: `old(steps)` inside the
   helper is its caller-relative entry value, and `parse_tracked`'s loop invariant
   `steps@ <= s0 + 1 + 9*i` composes cleanly only with a pinned `s0 == old(steps)@`
   ghost started after the list-initialisation charge.

## Deferred

T9's charging for caller-initiated repeated decoding of 1,024 aliases is the
caller's, not this core's (R13). A separate 32-bit verifier run is not installed.
M4b: the full implementation-only mutation campaign. `../dib` T12's own audit stays
separate.

## Deliverables

- `ico_verus/src/ico.rs` — tracked `parse`/`decode_entry`, their public tracked
  exports, the T9/T10/T11 public witnesses (1,675 lines).
- `ico_verus/src/model.rs` — the M4a step/alloc model, the T9/T10 discharge lemmas
  and the L1-L4 symbolic boundary lemmas (1,900 lines).
- `ico_verus/src/model_vectors.rs` — 170 generated `MC`/`MD` agreement obligations
  (6,573 lines).
- `ico_verus/src/witnesses.rs` — the three T11 public-call witnesses (224 lines).
- `ico_verus/src/{ico,model}.rs.m3` — the preserved M3 sources.
- `tools/gen_model_vectors.py`, `tools/audit_alloc.sh`.
