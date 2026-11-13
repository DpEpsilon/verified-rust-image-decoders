# PROOF-REPORT-M3 — theorems T5, T6, T8 (ICO core: total exact decode, precedence, termination)

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), toolchain
1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`, dependency
`dib_verus` (path `../dib/dib_verus`, version 0.3.0, unmodified).
**Final result line:** `verification results:: 101 verified, 0 errors`
(`runs/091.log`, a full reverify after `cargo clean`).
**Conformance:** `cargo test --release` →
`conformance: 217/217 vectors passed`, `test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 13 (runs 79–91 of `RUNLOG.tsv`). Seven
exited 0 (five intermediate green builds, one clean full reverify); two exited
101 on the `&&`-parenthesisation compile error (runs 84–85, fixed at run 86); and
four were disposable-copy sensitivity probes (runs 87–90). No obligation hit the
five-failure stop rule; no `STUCK.md`; `QUESTIONS.md` stays empty.
**Tool calls:** about 55 (my own count; the harness count is authoritative). Most
were verifier runs and `spec-frozen`/dependency lookups.
**Executable behaviour:** byte-identical. The one executable change is the DIB
call — `dib_verus::decode(slice, Mode::IcoEntryDibSlack)` became
`dib_verus::decode_with_steps(slice, Mode::IcoEntryDibSlack)` with the ghost step
count discarded, exactly the call M4a's combined work bound needs. The `(Result,
Ghost<nat>)` tuple's first component is the same value `decode` returns; the ghost
component erases. 217/217 still pass.

## What was built

`ico_verus/src/model.rs` (1,424 lines) gains the total model `MD` and the
statement it names, written from `spec-frozen/SPEC.md` R8/R11 independently of
the executable:

- `dib_wrap(r)`: R9's typed wrapping of the DIB result — `Ok(d) → Ok(d)`,
  `Err(e) → Err(Error::Dib(e))`.
- `md(b, es, index) -> Result<(int, int, Seq<u8>), Error>`: R8 in R10 order.
  The five ICO-controlled branches repeat `md_ico`; the selected-DIB branch is
  `match dib_verus::model::mp(b.subrange(start, end), Mode::IcoEntryDibSlack)`
  mapped to `Ok((w,h,rgba))` or `Err(Error::Dib(e))`. It is total for every typed
  `E` and every `u32` index; the tuple encoding means the statement carries the
  exact dimensions and every RGBA byte.
- `md_eq(b, es, index, r)`: the T5 statement as the complete `Result`; the
  success side is `decoded_matches(d, (w,h,rgba))` = `d.width == w &&
  d.height == h && d.rgba@ == rgba`.
- `lemma_md_eq_dib`: the composition discharger. Given `md_ico = DibSlice(start,end)`,
  the slice equality `slice == b.subrange(start,end)`, and
  `decode_mp_eq(r0, slice, Mode::IcoEntryDibSlack)`, it proves `md_eq(.., dib_wrap(r0))`.
- T6 predicates/lemmas (`t6_full_prec`, `lemma_t6_directory_priority`,
  `lemma_t6_index_before_list`, `lemma_t6_list_before_record`,
  `lemma_t6_record_before_defect`, `lemma_t6_defect_before_png_dib`,
  `lemma_t6_decode_reads_only_selected`, `lemma_t6_parse_defects_are_statuses`).
- T8 lemmas (`lemma_t8_variants`, `lemma_t8_count_cap_before_loop`,
  `lemma_t8_parse_iterates_count`).
- `lemma_t7_dib_output_bound`: the T7 clause M2 deferred.

`ico_verus/src/ico.rs` (1,391 lines): `decode_entry`'s postcondition is now
`model::md_eq` (was `model::decode_pre`). The selected-DIB branch slices exactly
`[payload[0], payload[1])`, proves `slice@ == bytes@.subrange(start, end)`, calls
`dib_verus::decode_with_steps` once and discharges `md_eq` from the returned
`decode_mp_eq` through `lemma_md_eq_dib`. The public witnesses `t5_decode_exact`,
`t6_precedence`, `t6_precedence_full`, `t6_parse_directory_priority`,
`t6_decode_ordering`, `t7_dib_output_bound` and `t8_termination` state the
manifest's obligations on the two public entry points. `model.rs.m2`/`ico.rs.m2`
preserve the M2 sources.

## Theorems proved

### T5 · exact total decode — proved at the manifest's quantification

`decode_entry` carries **no `requires`**. Its postcondition `md_eq(b,E,i,r)` holds
for every byte slice, every typed `Entry` slice and every `u32` index. Every R8
branch is decided by `md_ico` and the branch's proof discharges the matching
`md_eq` conjunct: `DirErr(e)` (the guard's error equals `h(b)`'s), `IndexOOR`
(`index >= count`), `BadList` (list length, then the selected-record mismatch),
`Defect(d)` (`Err(error_of_defect(d))`), `Png` (`Err(PngDelegation)`).

For a selected valid DIB entry the composition is discharged: `md_ico` names
`DibSlice(start,end)`, `reconstruct` re-proved `start <= end <= |b|` for this very
`bytes` (`lemma_valid_payload_in_bounds`), `bytes.get(start..end)` succeeds and
its view equals `bytes@.subrange(start,end)` (`slice@`), and
`decode_with_steps`'s postcondition `decode_mp_eq(r0, slice@, Mode::IcoEntryDibSlack)`
plus `lemma_md_eq_dib` give `md_eq(.., dib_wrap(r0))`. `MD`'s value on that branch
is `dib_verus::model::mp(b.subrange(start,end), IcoEntryDibSlack)` re-encoded, so
the executable equals the total model including every DIB error and every success
field/RGBA byte.

Note on the slice statement. The manifest asks to relate the call's slice to the
model's range: I prove `start = payload[0] as int` and `end = payload[1] as int`
(as `usize` in-bounds reads), and `slice@ == bytes@.subrange(start, end)` — the
`SliceIndex` postcondition. Because the executable call is on the literal
`bytes[start..end]`, this **is** `bytes@.subrange(a,b)` with `a,b` the model's
range. I did not state a separate proposition whose left side is the slice's
`@` view compared against a second `subrange` written with `as int` casts: the
exec call fixes the slice, so a separate witness would either be vacuous or name
a value no one calls. The composition uses the DIB contract on the same slice the
executable decodes, which is the load-bearing requirement.

### T6 · exact precedence — proved

Stated twice, on the model and on the public calls:

- `t6_full_prec` names the whole chain and `lemma_t6_full_prec` proves it from
  `md_eq`; `t6_precedence_full` is the public witness. R2/R3 directory priority
  first; then decode's index check before the list-length check
  (`lemma_t6_index_before_list`); the list/selected mismatch before the selected
  defect (`lemma_t6_list_before_record`, `lemma_t6_record_before_defect`);
  the selected defect before PNG/DIB (`lemma_t6_defect_before_png_dib`); PNG
  before any DIB error. The selected DIB's internal R14 precedence is delegated
  unchanged (the call's postcondition is the DIB's own `decode_mp_eq`).
- `t6_decode_reads_only_selected` proves `md_ico` depends only on `entries.len()`
  and `entries[index]`: two lists of equal length with equal selected element
  yield the same decision, so a bogus unselected supplied **or** wire record
  cannot change the selected result. This is R8's "only `entries.len()` and
  `entries[index]` are read" — a structural property of `md_ico`, not a test.
- `t6_parse_directory_priority` and `t6_decode_ordering` state the priorities on
  `parse` and `decode_entry`; `lemma_t6_parse_defects_are_statuses` records that
  `MC` fails only on `H`, so record defects appear as statuses.

An unselected invalid wire record or forged supplied record never changes a
selected result; decode checks only the list length and the selected record.

### T8 · termination — proved

`t8_termination(count, i, k)` proves, for `0 <= i < count` and `0 <= k <= 8`, that
`count - (i+1) < count - i`, `0 < count - i`, `8 - (k+1) < 8 - k` and
`0 <= 8 - k`: the parse loop's variant decreases once per complete record whether
valid or invalid (`reconstruct` never fails), and the eight-byte signature
comparison is bounded by eight. `lemma_t8_count_cap_before_loop` proves that a
wire count above 1,024 (`TooManyEntries`) is `H`'s result — checked before the
whole-table guard and before the record loop — so an input with count 65,535
returns from the guard and never iterates a record.
`lemma_t8_parse_iterates_count` proves `MC`'s list has length exactly `count`
(no iteration aborts), matching the loop invariant `acc.len() == i`.

`decode_entry` has no loop: the manifest's "decode revalidates only one record
then calls terminating DIB 1.1 once" is reflected in the executable directly —
there is a single `reconstruct` call, one index, and at most one
`decode_with_steps`. The only loop in the ICO decode path is the eight-byte
signature test inside `reconstruct`, whose variant `8 - k` `t8_termination`
bounds. Termination is *discharged* by Verus verifying these bodies (every loop
carries a `decreases` clause with a proved lower bound); the lemmas state the
manifest's exact variant arithmetic.

### T7 · the deferred DIB output clause — proved

M2 deferred "DIB output obeys the dependency's `4*w*h <= 67,108,864`
postcondition". `t7_dib_output_bound` now proves it on the public
`decode_entry`: on success, `1 <= w,h <= dib_verus::model::max_dimension()` (8,192),
`rgba.len() == 4*w*h` and `rgba.len() <= dib_verus::model::max_output()`
(67,108,864), discharged from `md_eq` through the DIB model's public
`lemma_mp_ok_shape` on the same slice.

## Executable changes

One change, behaviour-preserving (217/217 pass):

1. `decode_entry`'s selected-DIB branch calls
   `let (r0, _steps) = dib_verus::decode_with_steps(slice, Mode::IcoEntryDibSlack);`
   and matches `r0`. The function is the DIB 1.1 public tracked decoder the
   manifest's DIB T12/T9 work-bound plumbing exports; ICO calls it **now** so M4a
   does not change the executable. The ghost `Ghost<nat>` step count is discarded
   (`_steps`); the ghost return and its updates erase, so this is byte-for-byte
   the old `decode(slice, Mode::IcoEntryDibSlack)`. M4a will consume `_steps`
   through the already-proven `r.1@ <= 16*(|slice| + t12_checked_n(...))`.

## Sensitivity check (disposable copies, `VERIFY_CRATE=`, deleted afterwards)

Three implementation-only mutations, killed by a verifier diagnostic, not a test
(none hit an rlimit that hid the cause; the M30 probe below did, and is labelled):

- **M31** (skip the selected-record comparison, `if false && !record_matches`):
  `assertion failed` — 1 error (run 89). The `record_matches` branch's
  `md_eq` discharge needs the equality.
- **M-list** (skip the list-length check, `if false && entries.len() != ...`):
  `assertion failed` — 1 error (run 90). The later `InvalidEntryRecord`/`md_eq`
  discharge needs the length check.
- **M17** (pass `bytes[start..]` to EOF instead of the declared `[start, end)`):
  `precondition not satisfied` at the `decode_with_steps` call (its `bytes@` is
  no longer the declared slice) plus an rlimit on the same body — 2 errors
  (run 87). Both diagnostics name the slice.

Two further probes, both failing on verifier diagnostics but not reported as
clean kills because the diagnostic is an rlimit rather than the final assertion:

- **M30** (call `Mode::IcoEntryDib` instead of `IcoEntryDibSlack`): the
  `md_eq` discharge now needs `decode_mp_eq(.., IcoEntryDib)`, which the call's
  `decode_mp_eq(.., IcoEntryDibSlack)` postcondition does not give; the body
  hits `resource limit (rlimit) exceeded` — 1 error (run 88). This is a genuine
  proof failure (the obligation is unprovable), but I label it as such rather
  than a crisp assertion.

All are labelled audit-only proof-sensitivity demonstrations, not the M4b
campaign; M4b reruns the full set.

## Assurance boundary

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|
todo!|unimplemented!|verifier::external|verifier::rlimit|requires\(false\)|
unwrap\(\)|expect\(|panic!"` over `ico_verus/src/ico.rs`, `src/model.rs`,
`src/lib.rs` finds only two doc-comment matches (the rules list in the header and
the word "trusted" in a doc comment). No `#[verifier::rlimit]` or other
solver-budget attribute; no `STUCK.md`. The only trusted base is the verifier,
`vstd`'s contracts and the compiler. `spec-frozen/` (`SPEC.md`
`57b6dd8d…`, `VECTORS.json` `5ec052ad…`) and `../dib` are unmodified.

Honest caveats:

- **`as` casts** are unchanged from M2 and remain proved value-preserving.
- **T8 is stated, not a separate induction.** Termination is discharged by Verus
  verifying the loop bodies (`decreases` on `count-i` and `8-k`); the T8
  witnesses state the variant arithmetic the manifest names. This is the same
  assurance shape as TGA's `t10_termination`.
- **The DIB call's own work bound is M4a.** T5 binds the returned bytes/errors to
  the same slice; the step count `_steps` is carried but not yet bounded here.
- **32-bit target** is still an argument on the constants, not a second compiler
  run (unchanged from M2).
- **T9/T10/T11** (ICO-side linear work, allocation-call bound, mutation campaign)
  are M4a/M4b and untouched.

## Surprises

1. **The DIB call site is the only friction.** Everything else (the `MD` model,
   the composition lemma, all six T6 orderings, T8) proved within one or two
   verifier runs each because the M2 model already had `md_ico`, `me`,
   `entry_eq` and their iff-lemmas. The composition is short: `md_eq` unfolds to
   `mp(slice, Slack)`, `decode_mp_eq` unfolds to the same, and `dib_wrap`
   unfolds the wrap.
2. **`(index as int) < x` needs the parentheses inside `&&&`.** The parser read
   `index as int < hr.count` as a generic-argument list at run 85; wrapping the
   cast gave `((index as int) < hr.count)` and the compile error cleared.
3. **The giant inline `ensures`** for T6 was unwieldy and hit the same parse
   trap; naming the chain as `t6_full_prec` in `model.rs` both fixed it and kept
   the statement in one place.
4. **`decode_with_steps` returns `(Result, Ghost<nat>)`**, so the call is a `let`
   tuple destructure, not a `match` on a single value; matching `r0` afterwards
   is what keeps the branches shaped like the model.

## Deliverables

- `ico_verus/src/ico.rs` — `decode_entry` ensures `model::md_eq`; the DIB call is
  `decode_with_steps` on the exact validated slice; the T5/T6/T7/T8 public
  witnesses (1,391 lines).
- `ico_verus/src/model.rs` — adds `MD`, `md_eq`, the composition lemma, the T6
  predicates/lemmas and the T8/T7 lemmas (1,424 lines).
- `ico_verus/src/ico.rs.m2`, `src/model.rs.m2` — the preserved M2 sources.
