# P2a proof report (BMP policy refactor, second slice — parties beyond P1)

Verifier: Verus 0.2026.11.02 (`/opt/verus`), Rust 1.99.1, via `./verify.sh`.
Crate 0.3.0. No commits. **Outcome: INCOMPLETE. The `dib_finish` →
`r6r7_policy` connection and the spec-level Strict bridge `lemma_r6r7_policy_strict`
did not close.** The delivered `dib_verus/src/dib.rs` and `model.rs` are the P1
baseline (run 280: `verification results:: 1594 verified, 0 errors`); the P2a
work is preserved in the `*.p2a-wip`/`*.p2a-wip2` backups (best state: 1633
verified, 3 errors).

## What the scope asked for

Add six of the frozen policy's 22 options as real typed fields in both the
executable and the spec model, with the frozen value sets and event/`recovered`
semantics, Strict byte-identical to the shipped profile, and the hard invariant
(limits, totality, memory safety, work bound) proved for every value:
`planes`, `reserved`, `offset`, `high_count`, `headers`, `masks`.

## What is built and proved (in `*p2a-wip2`)

**Types** (`dib.rs`): six new `#[derive]`ed enums — `PlanesOptions{Must1,
Ignore}`, `ReservedOptions{Zero, Ignore}`, `OffsetOptions{Declared, AllowAny,
Gdk}`, `HighCountOptions{BmpTable, Ignore, GdkBound}`, `HeadersOptions{All1_1,
No16, No16No64}`, `MasksOptions{Reject, Independent, DefaultRGB}` — and six new
`pub` fields on `Policy`. `strict_policy()`, `recommended_beta_policy()`,
`profile_policy` and `profile_policy_exec` carry all eight fields; `Strict` sets
every option to its bold (1.1) value, so `Strict` is byte-identical by
construction. The exhaustive matches mean a new value is a compile error, not a
silent default.

**Model** (`model.rs`): `r1_prefix_policy` extended with `reserved` and `offset`
(named predicates `r1_size_bad`/`r1_offset_bad`); new accessors
`width_i_of`/`stored_of`/`planes_of`/`depth_of`/`compression_of`/`used_of`/
`declared_of`/`comp_of`; `hsize_ok_policy` (headers), `gdk_high_bound`,
`table_count_of_policy`, `masks_ok_policy`, `default_rgb_masks`,
`masks_after_policy`, `raw_masks_policy`, `masks_ok_policy_rf`,
`masks_of_policy_rf`, `table_count_of_policy_rf`; a new `r2r5_policy` (planes,
headers, high_count) and `r6r7_policy`/`r6r7_result_policy` (masks, high_count,
offset); `offset_effective`, `gdk_blank`, `gdk_rgb_reject`; the closed-event
`mh_recovered_so_far`/`recovered_policy`; and `mp_raw_policy`/`mp_rle_policy`/
`mp_policy` extended with the Gdk blank/reject branches. Strict bridges
`lemma_r1_prefix_policy_strict`, `lemma_r2r5_policy_strict`,
`lemma_mh_policy_strict`, `lemma_hsize_ok_policy_strict`,
`lemma_table_count_of_policy_strict`, `lemma_masks_ok_policy_reject`,
`lemma_masks_after_policy_strict`, `lemma_mh_recovered_so_far_strict` all
verify, as do the per-branch lemmas `lemma_r2r5_policy_tail`,
`lemma_r2r5_policy_{planes,dims,ico,low_count,bare_high,gdk_count,output}_err`,
and the new `lemma_mp_policy_ok_shape` with the Gdk branch.

**Executable** (`dib.rs`): `dib_fields` now takes the `Policy` and honours
`headers`, `planes`, `high_count`; `dib_finish` takes it and honours `masks`
(via a new `policy_masks` helper with a model-exact postcondition),
`high_count` and `offset`; `bmp_prefix_policy` honours `reserved` and `offset`;
`parse_header`/`parse_header_policy` thread the policy; `decode_with_policy`
computes `from_gdk`/`from_gdk_rgb` and returns
`r.1 == model::recovered_policy(bytes, bm, profile_policy(profile))` for the
Gdk-blank success and the non-blank paths. `decode_raw` is parameterised by the
pixel start `p0` (proved total for any `p0` with `h.pixel_start <= p0 <= |b|`)
so the Gdk/AllowAny/Declared offset can select a different start. New model
lemmas `lemma_raw_truncated`, `lemma_rle_truncated`,
`lemma_mp_policy_{raw,rle}_trunc`, `lemma_mp_policy_gdk_rgb` cover the
past-EOF branches.

At the best WIP state (`*.p2a-wip2`, run adjacent to 280) **1633 obligations
verify, 3 error**: `dib.rs`'s `r6r7_policy == Ok(m)` assert, one guard
hypothesis on `lemma_r6r7_policy_ok`, and `lemma_r6r7_policy_strict`. The
earlier `*.p2a-wip` state had 1724 verified, 10 errors.

## Not done (the open obligation)

The record equality `r6r7_result_policy(.., raw_masks_policy(..), table_count_of(rf)) == r6r7_result(.., rf)`
does not normalize, so `lemma_r6r7_policy_strict` does not close, and the
executable's `dib_finish` cannot discharge `r6r7_policy` on its success path or
the BITFIELDS/plane exit. Two per-function resource-limit failures
(`dib_fields`, `dib_finish`) accompany it. See `STUCK.md` for the ten attempts
and the suggested next step (prove the record equality from the four mask-word
and table-count hypotheses, discharging them from `dib_finish`'s own `rd32`
reads, and split each proof block into a named `proof fn`).

## Conformance (baseline, run 280)

- `cargo test --release`: `conformance: 302/302 vectors passed`; `corpus: 84
  files checked, 58 accepted`; `policy conformance: Strict: 102/102` and
  `RecommendedBeta: 102/102`.
- `tools/audit_alloc.sh`: `OK` (`vec!` count 1).

The P2a conformance extension (per-option oracle probes from
`CUSTOM-VECTORS.json` plus generated vectors, and per-option corpus acceptance
counts) was **not** written, because the six options do not verify and the
policy-conformance harness cannot compile a `Policy` with them.

## Compliance

No `assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo!`/`unimplemented!`,
no `requires(false)`, no `#[verifier::rlimit]` anywhere in the delivered or WIP
sources. Frozen inputs (`policy/*.md`, `policy/FREEZE.txt`, `spec-frozen/`,
`spec-1.1-frozen/`, `THEOREMS.md`, the policy vectors) untouched. Corpus bytes
read locally only.

## verify.sh runs this session

122–280: 159 verifier runs, 3 green (122 baseline, 280 final baseline, plus one
intermediate). All P2a runs through `./verify.sh`; no direct
`cargo verus verify`. Full log in `RUNLOG.tsv` and `runs/`.
