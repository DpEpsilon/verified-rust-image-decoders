# PROOF-REPORT-M4b — the MJ reference model; T12 and T13

**Stopping reason:** Finished all requested obligations. Verus 0.2026.11.02.8bf68ee, release
profile, toolchain 1.99.1-x86_64-unknown-linux-gnu.
**Final result line:** `verification results:: 212 verified, 0 errors`
(`/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/runs/175.log`, a full reverify
after `cargo clean`).
**Conformance:** `cargo test --test conformance -- --nocapture` →
`conformance: 111/111 vectors passed`, `test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 10, numbered 166–175 (4 exited 0, 6 were intermediate
failing builds). No single obligation hit the five-failure stop rule; no `STUCK.md` was needed.
**Tool calls:** 96 by my count (well under the 400 cap).

## Theorems proved

- **T12 (proved).** `combine`'s postcondition is
  `combine_decoded_eq(r, bytes@, entries@, color_index, mask_index)`, i.e.
  `decoded_eq(r, MJ(bytes, entries, color_index, mask_index))` for every `bytes`, `entries` with
  `parsed_inv(bytes, entries)`, and every pair of `Option<u32>` indices. `MJ` is the R14 reference
  model in `model.rs`, written independently of the executable: an absent colour index gives
  `MissingColor`, then an absent mask index gives `MissingMask`, then `MC` runs and its error
  propagates, then `MA` runs and its error propagates, then the two `(width, height)` pairs are
  compared (`TypeMismatch` when unequal), otherwise the result is
  `Ok(width, height, interleave_rgba(rgb, alpha))` with
  `out[4k..4k+4] = [rgb[3k], rgb[3k+1], rgb[3k+2], alpha[k]]`, `k = y·d+x`, increasing x then y.
  The wrapper `t12_combine_exact_for_p` states it over `P(b,E)` (a stronger premise than needed:
  `P` implies `parsed_inv` through `lemma_p_exec_inv`).
- **T13 (proved).** `t13_combine_after_parse(bytes, ci, mi)` runs `parse(bytes)` and then
  `combine(bytes, &entries, ci, mi)` on the *same* bytes and the entries `parse` returned. Its
  postcondition is `match out.0 { Ok(entries) => combine_decoded_eq(out.1, bytes@, entries@, ci, mi),
  Err(_) => true }`: on every successful parse the combine result equals `MJ` over exactly the
  entries `parse` produced. The `P(bytes, entries)` premise is derived in the body from the parse
  result by `lemma_parse_ok_gives_p`, which is T7's `parses_to` postcondition fed through T8's
  `lemma_t8`; it is not assumed or posited. `lemma_p_exec_inv` then supplies `parsed_inv` for the
  `combine` call.

T1–T11 and L1–L6 remain proved (they are a subset of the 212 obligations). No M2, M3 or M4a
statement was weakened.

## The reference model MJ (independently written)

`model.rs` now defines, in Verus's logic and without calling any executable code:

- `interleave_rgba(rgb, alpha)` — R14's four-byte interleave, `Seq::new(4*|alpha|, ...)`;
- `mj(b, E, ci, mi)` — the total function returning the exact result or the first error in R14's
  order, `OutsideDomain` outside `P` being implicit in the fact that `mj` is only *used* under
  `P(b,E)` (it is a total function on all `b`, `E`, as the manifest requires);
- `combine_decoded_eq(r, b, E, ci, mi)` — the exact executable-vs-model result relation;
- case lemmas `lemma_mj_missing_color`, `lemma_mj_missing_mask`, `lemma_mj_mc_err`,
  `lemma_mj_ma_err`, `lemma_mj_mismatch`, `lemma_mj_ok`, and the interleave lemma
  `lemma_interleave_rgba_exact` proving a byte sequence with the right four per-pixel bytes equals
  `interleave_rgba`;
- `lemma_decoded_eq_ok` / `lemma_decoded_eq_err`, which unpack `decoded_eq` on a concrete
  executable `Ok`/`Err` result into the model's shape;
- `lemma_mc_fields_eq` / `lemma_ma_fields_eq` / `lemma_mj_fields_eq` for the model's dependence on
  only the R1 fields.

`combine`'s postcondition names `combine_decoded_eq`; the model is a separate definition, not a
copy of the executable's branches.

## What the executable now carries

- `combine` — the new postcondition `combine_decoded_eq(r, bytes@, entries@, color_index,
  mask_index)`, alongside the unchanged M2 bounds. Executable statements are unchanged except
  that the two `?` operators became explicit `match`es (so an error branch can carry its proof
  block) and the RGBA loop gained four ghost `forall` invariants recording which output bytes
  have been written. The `MissingColor`/`MissingMask` returns, the `decode_rgb` then `decode_mask`
  order, the side check and the byte-by-byte write are byte-for-byte the M4a code.
- No other executable changed. `rle_decode`, `decode_rgb` and `decode_mask` are untouched.

## Surprises and tool obstacles

1. Verus's multi-return syntax cannot spell `Result<Vec<Entry>, Error>` as a *named* field type:
   `-> (p: Result<Vec<Entry>, Error>, ...)` is parsed as the pattern/type boundary and fails
   ("expected -> (pattern: type)"). Naming a tuple field with a comma-bearing generic type does
   not work, so `ParseRes`/`CombineRes` type aliases carry those two types.
2. Verus multi-return fields cannot be pattern-matched on the `Ok` constructor directly
   (`match out.0 { Ok(entries) => ... }` fails); the `out.0`/`out.1` projections work in the
   `ensures` but the body must `let`-bind. The T13 statement was rewritten to
   `match out.0 { Ok(entries) => ..., Err(_) => true }`, which Verus handles and which is still an
   exact statement for every successful parse.
3. `nonlinear_arith` needs `k < alpha.len()` as a separate hypothesis to bound
   `3*k + (j % 4) < rgb.len()`; the single combined inequality
   `3*k + (j % 4) < rgb.len()` from `j < 4*alpha.len()`, `4*k <= j`, `k == j/4` did not discharge,
   while the split `k < alpha.len()` (from `4*k <= j < 4*alpha.len()`) then
   `3*k + c < 3*alpha.len()` did.
4. A stale `combine` postcondition mismatch in `t13_combine_after_parse` was avoided by phrasing
   the statement over the entries `parse` returned rather than over an existential list.

## Assurance boundary

`grep -nE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external" src/icns.rs src/model.rs` finds nothing. The trusted base is
unchanged: the verifier, `vstd` contracts and the compiler. No theorem statement was weakened.
The model-vs-vector agreement, the 40 named witnesses, the nine large-vector lemma
instantiations beyond L3 and the ten mutations are M4c, not this dispatch; this report does not
claim them.
