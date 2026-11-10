# P1 proof report (policy refactor, first slice)

Verifier: Verus 0.2026.11.02 (binary `/opt/verus`), Rust 1.99.1, via `./verify.sh`.
Crate version 0.3.0 (was 0.2.0). Branch `dib-policy`. No commits.

## Result

- **Clean full reverify** (`cargo clean`, then `./verify.sh`): run 121,
  `verification results:: 1594 verified, 0 errors` (see below).
- **`cargo test --release`**: policy conformance Strict 102/102 and Recommended
  102/102; the unchanged `conformance.rs` 302/302; corpus 84 files / 58 accepted.
- **Frozen inputs intact**: `policy/FREEZE.txt` all six OK; `spec-1.1-frozen`
  FREEZE.txt OK. `THEOREMS.md`, `spec-frozen/`, `policy/*.md` and the policy
  vectors untouched.
- **Compliance**: no `assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo!`/
  `unimplemented!`, no `requires(false)`, no `#[verifier::rlimit]`. `audit_alloc.sh`
  OK (`vec!` count 1).
- **Purely additive to the shipped proof**: `dib.rs` +634 lines / 0 removed;
  `model.rs` +658 / 1 (the import line). `decode` and `decode_with_steps` bodies
  are byte-identical to the `*.p1` backups; `model_vectors.rs` and
  `tests/conformance.rs` are byte-identical.

## What P1 proves

Policy types (`dib_verus/src/dib.rs`): `MetadataOptions {Declared, Ignore}`, a
two-field `Policy {file_size, image_size}` (room for later option groups), the
`BmpProfile {Strict, Recommended}` selector with spec fn `profile_policy` and exec
wrapper `profile_policy_exec`, and `BmpMode {BmpFile, BareDib}` with
`bmp_mode_to_mode` / `bmp_mode_mode`. `BmpProfile` and `BmpMode` are
`#[non_exhaustive]`; `BmpProfile: Default` is `Strict`. The `profile_policy` match
is exhaustive, so a new profile is a compile error there, never a silent
default-to-Strict.

Policy model (`dib_verus/src/model.rs`): `r1_prefix_policy` (R1 with `file_size`),
`mh_policy` (R1–R7 with that prefix, the same `r2r7` chain and the same `MHRecord`
type, so the derived header record is one of `MH`'s two records unchanged),
`image_size_ok_policy` / `mp_image_size_ok_policy` (R12 with `image_size`),
`mp_mask_phase_policy`, `mp_raw_policy` / `mp_rle_policy`, and `mp_policy`.
`recovered` is not a model field: it is `false` for both P1 profiles, so it cannot
depend on input.

- **Strict bridge** (the delta's `MH_1_1 := MH(Strict)`, `MP_1_1 := MP(Strict)`,
  each a definitional reduction at `strict_policy()`, not an equality between
  independently written decoders): `lemma_r1_prefix_policy_strict`,
  `lemma_mh_policy_strict`, `lemma_image_size_ok_policy_strict`,
  `lemma_mp_image_size_ok_policy_strict`, `lemma_mp_mask_phase_policy_strict`,
  `lemma_mp_raw_policy_strict`, `lemma_mp_rle_policy_strict`, `lemma_mp_policy_strict`.
  No existing T-statement is weakened.
- **Profiled lemmas** the executable discharges: `lemma_mp_policy_mh_err`,
  `lemma_mp_policy_raw_err`, `lemma_mp_policy_rle_err`,
  `lemma_mp_policy_phase_err_raw`, `lemma_mp_policy_phase_err_rle`,
  `lemma_mp_policy_raw_assemble`, `lemma_mp_policy_rle_assemble`,
  `lemma_mp_policy_phase` (T9/T10 on both paths), `lemma_mp_mask_phase_policy_len`,
  `lemma_mp_policy_ok_shape` (T13), `lemma_mp_mask_phase_policy_bmp`.

Executable (`dib_verus/src/dib.rs`): `bmp_prefix_policy` (R1 with the option) and
`parse_header_policy` (reuse `dib_fields` + `dib_finish`, so they generalize to
any policy and the record is `MH_policy`'s exactly); `finish_mask_policy` (R12
with the option; no AND work in either BMP mode, and the proof shows the ICO arms
are unreachable at the call site); and
`pub fn decode_with_policy(bytes, bm, profile) -> (Result<Decoded, Error>, bool)`,
reusing `alloc_output` / `decode_raw` / `decode_rle`. Its postcondition is
`r.1 == false`, `decode_policy_mp_eq` (T9: the `Result` equals `MP_policy` on every
error, dimension and RGBA byte, for every slice and both modes), and the T1/T3/
T11/T12/T13 shape bounds. The `decoded` result is inside the pixel helpers, so
nothing about the pixel path was re-proved.

Statement witnesses at the project's usual quantification: `t9_policy`,
`t10_policy_precedence`, `t13_policy_success_bounds`, `t1_policy_totality`.

Conformance: `dib_verus/tests/policy_conformance.rs` runs
`policy/POLICY-VECTORS.json` for `Strict` and `Recommended` (102 each, `recovered`
checked), next to the unchanged 302-vector and corpus tests.

## Deviations and honest limits

1. **The `file_size` and `image_size` `Ignore` values are never read bounds.** In
   both the model and the executable, `bfSize` and `biSizeImage` are validated
   (or skipped) and otherwise unused: `r1_prefix_policy` and `bmp_prefix_policy`
   only test `bfSize`, and the R12 phase's `x_len` is always `pos - pixel_start`,
   the cursor the pixel phase actually reached — never the declaration. The
   universal `MAX_INPUT`/`MAX_OUTPUT` caps, exactly one `4N` allocation, the zero
   header-error allocations and `steps <= 16*(|b|+N)` are shared with `decode` and
   hold for every policy by the same code.
2. **`Policy`'s fields are `pub`.** Verus treats a struct with any non-public field
   as an opaque datatype and then rejects the spec-fn construction this crate uses
   (`disallowed: constructor for an opaque datatype`, run 119). POLICY-SPEC's rule
   that the field record is "internal and experimental, not a public constructor"
   is honoured at the API level instead: `decode_with_policy` accepts only a
   `BmpProfile`, so no caller can inject field values; the fields themselves are
   marked as internal in the doc comment.
3. **`recovered` is proved `false`, not the general closed-event definition.** P1
   has no option that can set a listed data-affecting event: the two `Ignore`
   values tolerate only `bfSize`/`biSizeImage`, which the spec explicitly excludes
   from the flag. `MP_policy` therefore has no recovery field, and
   `decode_with_policy`'s `recovered` postcondition is `r.1 == false` (and
   `r.1 == false` is stronger than the general flag semantics for these profiles).
4. **The other two frozen profiles and the other sixteen options are deferred.**
   They are not in the P1 types (they are added as new enum variants in a later
   milestone); the payload-computer design (`bmp_prefix_policy` /
   `parse_header_policy` / `finish_mask_policy` keyed on `Policy`) means adding
   them needs no rewrite of the `MH`→`MP` record.
5. **No implementation mutations were run for P1.** The P1 entry point adds one
   boolean path each to R1 and R12; mutant campaigns are a later-milestone
   activity per BRIEF-POLICY (P1 scope items 1–5). No mutant was executed and none
   is claimed.
6. **One obligation hit the per-function resource limit.** `decode_with_policy`'s
   first body (a larger proof block) exceeded the limit; the limit was resolved by
   removing a lemma application the proof did not need, **without** any
   `#[verifier::rlimit]` (STUCK.md was not needed: no single obligation failed five
   times).

## verify.sh runs this session (P1)

102 inc1 types, 103–104 model (name fixes), 105 model lemmas (1 error, fixed),
106–113 entry-point iterations, 114 (1 error, fixed), 115 (green 1589/0; 115 s),
116 clean full reverify (1589/0; 185 s), 117 witnesses (1593/0), 118 (private-field
experiment, rejected by Verus), 119 (opaque-datatype diagnostic), 120 (green
1594/0), 121 FINAL clean full reverify.

## Deferred (not P1)

`GdkCompat-2.42.12`, `ImageCrateCompat-0.25.10`; the other sixteen options; any
input that sets `recovered = true`; `#[cfg(test)]` custom policy records; the P1
implementation mutation campaign.
