# PROOF-REPORT-M4b — L1-L4, the 165 model-vs-vector obligations, the 54 witnesses and the 20 mutations

**Milestone:** M4b, the last one. The four large-vector lemmas L1-L4, the compact
vector protocol, the named witnesses and the deliberate proof-sensitivity
mutations of `THEOREMS.md`.

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), release
profile, toolchain 1.99.1-x86_64-unknown-linux-gnu, vstd
`0.0.0-2026-11-01-0251` (the version in `TOOLNOTES.md`).

## Contents

1. What was added
2. Tick sheet: the four large-vector lemmas L1-L4
3. Tick sheet: the 165 model-vs-vector obligations
4. The 54 named witnesses, each by name with its proof item
5. Tick sheet: T1-T11
6. The 20 mutations
7. Assurance boundary
8. Things worth knowing
9. Runs, tool calls and the final lines

## 1. What was added

- `tga_verus/src/model.rs` (5,870 -> 6,800 lines) — L1-L4 and the helpers they
  need: `rep_byte`, `hdr18`, `rgba_pat`, `rep_pats`, `lemma_mh_prefix`,
  `lemma_pkt_at`, `lemma_pkt_control`, `lemma_pkt_window`,
  `lemma_packet_step_periodic`, `lemma_orient_const`, and the four instantiated
  lemmas.
- `tga_verus/src/model_vectors.rs` — generated, 2,954 lines, 165 proof
  obligations, one per small vector. Registered in `lib.rs` as
  `pub mod model_vectors;`.
- `tools/gen_model_vectors.py`, `tools/check_anchors.py`, `tools/mutations.py`,
  `tools/run_mutations.sh`, `tools/mutation_summary.py`,
  `tools/mutation_table.py` and the disposable copies under `mutation/`.

No `assume`, `admit`, `external_body`, `trusted`, `unsafe`, `todo!` or
`unimplemented!` appears in `src/model.rs`, `src/model_vectors.rs`, `src/tga.rs`
or `src/lib.rs`:

    grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external|requires\(false\)" tga_verus/src/

finds nothing. No solver-budget attribute was added; the only one in the crate
is M3's single `#[verifier::rlimit(80)]` on `lemma_packet_repetition`.

## 2. Tick sheet: the four large-vector lemmas L1-L4

Each general statement is proved from the model's own definitions (symbolic
sequence algebra, never a 64 MiB evaluation), and each is then instantiated on
the manifest's named vector.

| Lemma | Manifest statement | Where | Status |
| --- | --- | --- | --- |
| L1 | `forall b, \|b\|>67,108,864: MP(b)=Err(FileTooLarge)` | `lemma_l1_oversized` (model.rs:6049) | proved |
| L1 · `input-over-64m` | zero byte repeated 67,108,865 times | `lemma_l1_input_over_64m` (model.rs:6062) | proved |
| L2 | `forall b, \|b\|=67,108,864 and b[2]=0: MP(b)=Err(UnsupportedImageType)` | `lemma_l2_at_limit` (model.rs:6076) | proved |
| L2 · `input-at-64m` | zero byte repeated 67,108,864 times | `lemma_l2_input_at_64m` (model.rs:6094) | proved |
| L3 · `output-at-64m-8192x2048` | `b=H(11,8192,2048,8,32)+S^131072`; `\|b\|=262,162`, `MP(b)=Ok(8192,2048,(7f7f7fff)^16777216)`, `\|rgba\|=67,108,864` | `lemma_l3_output_at_64m_8192x2048` (model.rs:6507) | proved |
| L4 · `output-at-64m-4096-square` | `b=H(10,4096,4096,16,33)+C^131072`; `\|b\|=393,234`, `MP(b)=Ok(4096,4096,(ff0000ff)^16777216)`, `\|rgba\|=67,108,864` | `lemma_l4_output_at_64m_4096_square` (model.rs:6679) | proved |

The manifest's `S` is `bytes(0xff,0x7f)` (`l3_s`) and `C` is
`bytes(0xff,0x00,0xfc)` (`l4_c0`); `H(...)` is `hdr18`, a `Seq::new(18, ...)` with
the type, dimensions, depth and descriptor placed at the R1 offsets and every
other field zero. The generator checks that each fixture's expanded bytes equal
this construction and that its declared result equals the lemma's claim, and the
two `hdr18` byte sequences are equal to `tools/gen_model_vectors.py`'s `H(...)`
(diffed).

**How L3/L4 avoid the 64 MiB evaluation.** Neither lemma expands its 262 KB /
393 KB input or its 64 MiB output. The reasoning is:

- The input is `hdr18(t,w,h,d,a) + rep_pats(pats, 131072)`, and
  `lemma_pkt_*` read the control byte and the sample window of the `j`th packet
  from `pats` alone (period `|pats|` divides `j*|pats|`).
- `packet_step(hdr18+rep_pats(pats,1), ...)` — one packet — is decided by
  `assert(...) by (compute)`, a bounded 3-byte evaluation;
  `lemma_packet_step_periodic` lifts it to every one of the 131,072 periods by
  rewriting the absolute cursor and the produced count.
- L4's single packet is checked by `compute` on the literal byte sequence
  `0xff 0x00 0xfc`, which the model converts through `conv_sample`, `rgb555`,
  `rep5`, `blue5`/`green5`/`red5` to `0xff 0x00 0x00 0xff`; because
  `assert ... by (compute)` unfolds exactly those definitions, the one-packet
  obligation is decided without evaluating 16,777,216 pixels.
- `lemma_packet_repetition_mr` (M4a) sums the 131,072 steps to
  `MR = Ok(18 + 262144, 16777216, pix_pow(c, 16777216))`.
- `lemma_mr_cur_pix_proj` turns `MR` into the `mr_cur`/`mr_pix` projections the
  `MP` lemmas consume, and `lemma_mp_rle_ok` assembles `MP`'s `Ok`.
- `lemma_orient_const` proves `orient_bytes(px, ...)` for a constant pixel
  sequence `px` equals `rgba_pat(c, N)` — the orientation map is a bijection
  (`lemma_fcoord_inverse`), so every output cell is byte `i%4` of `c`, and no
  byte ordering inside a pixel changes. That is R16's single maximal run.

`lemma_l1_l2_lengths` supplies `|rep_byte(0, 67,108,865)|=67,108,865`,
`|rep_byte(0, 67,108,864)|=67,108,864 = MAX_INPUT`: the exact arithmetic that
makes L2 reject `>=` in place of `>`.

## 3. Tick sheet: the 165 model-vs-vector obligations

`tga_verus/src/model_vectors.rs` contains one `proof fn agree_<vector>` per
vector whose expanded input is at most 4,096 bytes: **165 obligations** over
169 vectors minus the 4 large ones, 3,957 input bytes in all. Each states

    MP(<the vector's expanded input>) == <the fixture's expected Result>

— `Ok((w, h, rgba))` byte-exact, or `Err(Error::X)` for the exact error name —
and each is discharged by `assert(...) by (compute)`. The expected values are
read only from `spec-frozen/VECTORS.json` (`expected`), never from the
executable decoder.

The verifier reports `578 verified, 0 errors` over the whole crate, of which the
165 vector obligations and the L1-L4 lemmas are the M4b addition (M4a was 373).

## 4. The 54 named witnesses

All 54 exist in `spec-frozen/VECTORS.json` at or below 4,096 bytes, and each is
discharged by the named obligation in section 3 (all of which verify). The
"expected" column is the fixture's own `expected` field.

### Header and precedence (16)

| Witness | Expected | Proof item |
| --- | --- | --- |
| `empty` | TruncatedHeader | `agree_empty` |
| `header-short-17` | TruncatedHeader | `agree_header_short_17` |
| `unsupported-type-0` | UnsupportedImageType | `agree_unsupported_type_0` |
| `invalid-map-flag-2` | InvalidColorMapFlag | `agree_invalid_map_flag_2` |
| `dimensions-0-1` | InvalidDimensions | `agree_dimensions_0_1` |
| `interleave-64` | UnsupportedInterleave | `agree_interleave_64` |
| `depth-type-2-depth-8` | UnsupportedPixelDepth | `agree_depth_type_2_depth_8` |
| `indexed-no-map` | InvalidColorMap | `agree_indexed_no_map` |
| `map-range-overflow` | ColorMapRange | `agree_map_range_overflow` |
| `map-invalid-depth-before-range` | InvalidColorMap | `agree_map_invalid_depth_before_range` |
| `attribute-2-24-1-0` | InvalidAttributeBits | `agree_attribute_2_24_1_0` |
| `output-too-large` | OutputTooLarge | `agree_output_too_large` |
| `type-before-flag-and-dimensions` | UnsupportedImageType | `agree_type_before_flag_and_dimensions` |
| `flag-before-dimensions` | InvalidColorMapFlag | `agree_flag_before_dimensions` |
| `attribute-before-output` | InvalidAttributeBits | `agree_attribute_before_output` |
| `output-before-id` | OutputTooLarge | `agree_output_before_id` |

### Payload and errors (10)

| Witness | Expected | Proof item |
| --- | --- | --- |
| `id-short` | TruncatedImageId | `agree_id_short` |
| `map-short` | TruncatedColorMap | `agree_map_short` |
| `raw-partial-pixel` | TruncatedPixels | `agree_raw_partial_pixel` |
| `rle-repeat-overrun-no-sample` | RleOverrun | `agree_rle_repeat_overrun_no_sample` |
| `rle-literal-overrun-no-sample` | RleOverrun | `agree_rle_literal_overrun_no_sample` |
| `rle-repeat-missing-sample` | RleTruncated | `agree_rle_repeat_missing_sample` |
| `index-below-first` | ColorIndexOutOfRange | `agree_index_below_first` |
| `index-rle-packet-truncation-before-index` | RleTruncated | `agree_index_rle_packet_truncation_before_index` |
| `rle-packet-missing-before-invalid-first-index` | RleTruncated | `agree_rle_packet_missing_before_invalid_first_index` |
| `trailing-incomplete-footer` | TrailingBytes | `agree_trailing_incomplete_footer` |

### Successful pixels (14)

| Witness | Expected | Proof item |
| --- | --- | --- |
| `raw-15-bit15-ignored` | Ok 1x1 | `agree_raw_15_bit15_ignored` |
| `raw-16-alpha-zero` | Ok 1x1 | `agree_raw_16_alpha_zero` |
| `raw-32-alpha-zero-ignores-byte` | Ok 1x1 | `agree_raw_32_alpha_zero_ignores_byte` |
| `gray16-alpha-eight` | Ok 1x1 | `agree_gray16_alpha_eight` |
| `palette-16-attr-1` | Ok 1x1 | `agree_palette_16_attr_1` |
| `palette-first-65535` | Ok 1x1 | `agree_palette_first_65535` |
| `rle-index16-literal` | Ok 2x1 | `agree_rle_index16_literal` |
| `rle-crosses-row` | Ok 2x2 | `agree_rle_crosses_row` |
| `origin-top-left` | Ok 2x2 | `agree_origin_top_left` |
| `origin-top-right` | Ok 2x2 | `agree_origin_top_right` |
| `origin-bottom-left` | Ok 2x2 | `agree_origin_bottom_left` |
| `origin-bottom-right` | Ok 2x2 | `agree_origin_bottom_right` |
| `footer-opaque-offsets` | Ok 1x1 | `agree_footer_opaque_offsets` |
| `rle-stops-before-footer` | Ok 1x1 | `agree_rle_stops_before_footer` |

### Revision-2 witnesses (14)

| Witness | Expected | Proof item |
| --- | --- | --- |
| `unused-map-type-2-entry-15-id-3` | Ok 1x1 | `agree_unused_map_type_2_entry_15_id_3` |
| `unused-map-type-11-entry-32-id-3` | Ok 1x1 | `agree_unused_map_type_11_entry_32_id_3` |
| `map-range-with-flag-zero` | InvalidColorMap | `agree_map_range_with_flag_zero` |
| `raw-16-mixed-0444` | Ok 1x1 | `agree_raw_16_mixed_0444` |
| `raw-15-mixed-0c63` | Ok 1x1 | `agree_raw_15_mixed_0c63` |
| `palette-15-mixed-0c63` | Ok 1x1 | `agree_palette_15_mixed_0c63` |
| `palette-16-mixed-0444` | Ok 1x1 | `agree_palette_16_mixed_0444` |
| `indexed-map-after-id-255` | Ok 1x1 | `agree_indexed_map_after_id_255` |
| `indexed-map-short-after-id` | TruncatedColorMap | `agree_indexed_map_short_after_id` |
| `palette-16-index-6-type-9` | Ok 1x1 | `agree_palette_16_index_6_type_9` |
| `palette-32-index-6-type-9` | Ok 1x1 | `agree_palette_32_index_6_type_9` |
| `rle-literal-crosses-row` | Ok 2x2 | `agree_rle_literal_crosses_row` |
| `gray-origin-bottom-right` | Ok 2x2 | `agree_gray_origin_bottom_right` |
| `attribute-rle-indexed-24-one` | InvalidAttributeBits | `agree_attribute_rle_indexed_24_one` |

54 witnesses, 54 distinct proof items, all among the 165 obligations of section
3. They are not a substitute for the other 111 obligations, which are checked
the same way.

## 5. Tick sheet: T1-T11

The eleven theorems are postconditions on the concrete functions named in
`THEOREMS.md`; every one is re-verified in the same run that checks the 165
obligations, L1-L4 and the mutations. M4b added no statement and weakened none.

| Theorem | Where it lives | Status |
| --- | --- | --- |
| T1 · safety | `t1_decode_safe` (tga.rs:4158); `decode`'s no-precondition safety postcondition | proved |
| T2 · header safety | `t2_header_bounds` (tga.rs:4133); `parse_header`'s postcondition + `lemma_mh_ok_bounds` | proved |
| T3 · packet safety/termination | `t3_rle_safe` (tga.rs:3902); `decode_rle`'s loop variant and `lemma_rle_count_range` | proved |
| T4 · conversion/orientation bounds | `t4_bounds` (tga.rs:3940), `t4_raw_total`; `lemma_colour_conversion`, `lemma_orientation_bijection` | proved |
| T5 · exact header semantics | `t5_header_exact` (tga.rs:4112); `parse_header` = `MH` | proved |
| T6 · exact packet semantics | `t6_rle_exact` (tga.rs:3727); `decode_rle`'s `rle_full_eq` | proved |
| T7 · total functional correctness | `t7_decode_exact` (tga.rs:3799); `decode`'s `decode_eq` | proved |
| T8 · success and resource bounds | `t8_output_cells` (tga.rs:3830); `decode_eq`'s iff | proved |
| T9 · precedence | `t9_precedence` (tga.rs:3782) | proved |
| T10 · termination | `t10_termination` (tga.rs:4070) | proved |
| T11 · integrated call path | the structure of T7's proof (`choose` over `parse_header`, one `bytes`/`map` slice) | proved |

## 6. The 20 mutations

Each mutation is applied to a disposable copy `mutation/mN/tga_verus` rebuilt
from the pristine `mutation/base/`, verified through
`VERIFY_CRATE=mutation/mN/tga_verus ./verify.sh "mutation N"`, and discarded
before the next. Every copy is the whole crate, so all of T1-T11, L1-L4 and the
165 model-vs-vector obligations are enabled in it. Where the manifest names a
fixture and a theorem that must both fail, the edit is made in `src/model.rs`
(the specification authority), which breaks the fixture-agreement obligation
*and* the executable's `decode = MP` postcondition (the executable is still the
correct decoder). The other seven mutations (8, 11, 13, 14, 15, 16, 17) are
edited in `src/tga.rs`, because each is one the manifest requires to fail on the
concrete code itself — an unguarded read, the loop's own produced count, the `N`
arithmetic, an offset or stride the executable computes — which a model edit
cannot exhibit.

All twenty mutations fail verification; not one passes. The "failing obligation"
column names the proof item the verifier blames for the manifest's named
theorem: where the mutation is in `src/model.rs`, the generated
`agree_<vector>` obligation and the executable's `decode = MP` postcondition
both fail (the executable is still the correct decoder); where the mutation is
in `src/tga.rs`, the named function's own postcondition fails. Every log is the
output of one `./verify.sh` run with all of T1-T11, L1-L4 and the 165 vector
obligations enabled.

| # | Mutation | Failing obligation(s) | Result | Log |
| --- | --- | --- | --- | --- |
| 1 | model `mh`: `|b|>MAX_INPUT` -> `>=` | `lemma_l2_at_limit`, `lemma_mh_prefix`, `parse_header`, `decode` | 575 verified, 3 errors | `mutation/m1/failure.log` |
| 2 | model `mh`: type check and flag check swapped | `agree_unsupported_type_0`, `lemma_l2_at_limit`, `parse_header` | 3 errors, compile aborted | `mutation/m2/failure.log` |
| 3 | model `max_dimension`: 8,192 -> 65,535 | `agree_dimensions_8193_1`, `lemma_mh_ok_bounds`, `parse_header` | 4 errors, compile aborted | `mutation/m3/failure.log` |
| 4 | model `conv_map`: map `first` ignored (`off = index*stride`) | `agree_palette_first_65535` + 11 more `agree_*` (all palette/RLE-indexed), `lemma_colour_conversion`, `convert_index` | 564 verified, 14 errors | `mutation/m4/failure.log` |
| 5 | model `conv_sample`: bit 15 as alpha for 16-bit attr 0 | `agree_raw_16_opaque_bit_off`, `convert_sample`, `lemma_colour_conversion` | 4 errors, compile aborted | `mutation/m5/failure.log` |
| 6 | model `rle_count`: `(control&0x7f)+1` -> `control&0x7f` | `agree_rle_literal_overrun_no_sample`, `lemma_l3_output_at_64m_8192x2048` | 2 errors, compile aborted | `mutation/m6/failure.log` |
| 7 | model `mr_at`/`mr_cur`/`mr_pix`: overrun/availability swapped | `agree_rle_repeat_overrun_no_sample`, `lemma_mr_cur_overrun`, `lemma_mr_at_progress` | 3 errors, compile aborted | `mutation/m7/failure.log` |
| 8 | executable `decode_rle`: `produced = 0` at each row | `decode_rle` (loop invariant), `decode` | 577 verified, 1 errors | `mutation/m8/failure.log` |
| 9 | model `oyr`: vertical flip reversed | `agree_rle_crosses_row`, `lemma_oyr_is_axis`, `lemma_offset_is_fcoord`, `decode_rle`, `decode_raw` | 7 errors, compile aborted | `mutation/m9/failure.log` |
| 10 | model `mp_suffix_ok`: any nonempty suffix accepted | `agree_trailing_one_byte`, `lemma_mp_suffix_short`, `lemma_mp_suffix_bad` | 4 errors, compile aborted | `mutation/m10/failure.log` |
| 11 | executable `decode`: footer offset dereferenced | `decode` (unmet index-in-bounds precondition, tga.rs:2146) | 577 verified, 1 errors | `mutation/m11/failure.log` |
| 12 | model `rep5`: `c*8+c/4` -> `(c*255+15)/31` | `agree_raw_15_mixed_0c63`, `expand5`, `lemma_rep5` | 3 errors, compile aborted | `mutation/m12/failure.log` |
| 13 | executable `decode`: `map_start = 18` (not `cursor`) | `decode` (map_end assertion, tga.rs:1927) | 577 verified, 1 errors | `mutation/m13/failure.log` |
| 14 | executable `convert_index`: fixed stride 3 | `convert_index` (stride equality assertion, tga.rs:1343) | 577 verified, 1 errors | `mutation/m14/failure.log` |
| 15 | executable `decode_rle`: whole-literal guard removed | `decode_rle` (slice bounds precondition, tga.rs:3283) | 577 verified, 1 errors | `mutation/m15/failure.log` |
| 16 | executable `decode`: `N` computed in `u16` | `decode` (arithmetic overflow, tga.rs:1833) | 577 verified, 1 errors | `mutation/m16/failure.log` |
| 17 | executable `index_in_range`: guard removed | `index_in_range` (range postcondition, tga.rs:1557) | 577 verified, 1 errors | `mutation/m17/failure.log` |
| 18 | model `conv_sample`: 32-bit A byte used at attr 0 | `agree_raw_32_alpha_zero_ignores_byte`, `convert_sample`, `lemma_colour_conversion` | 4 errors, compile aborted | `mutation/m18/failure.log` |
| 19 | model `conv_sample`: greyscale-16 second byte at attr 0 | `agree_gray16_alpha_zero`, `convert_sample`, `lemma_colour_conversion` | 4 errors, compile aborted | `mutation/m19/failure.log` |
| 20 | model `conv_sample`: bit 15 clear transparent at attr 1 | `agree_raw_16_alpha_zero`, `convert_sample`, `lemma_colour_conversion` | 3 errors, compile aborted | `mutation/m20/failure.log` |

Eleven mutations (2, 3, 5, 6, 7, 9, 10, 12, 18, 19, 20) report their errors and
then abort compilation, so their logs carry no crate-level `verification
results` line (only the support crate's `2159 verified, 0 errors`); the crate's
own errors are present and are what the manifest requires. The other nine carry
a crate line (`575`/`564`/`577` verified with the errors counted). In every case
the error is a **verifier** diagnostic with T1-T11 enabled, never a runtime
test.

None needed the manifest's fallback wording, and none was a vacuity finding: no
mutation verified. Two mutations were rebuilt after their first run because the
first attempt failed for the wrong reason (a type-inference error for mutation
13, and a too-broad edit for mutation 5); the corrected copies are the ones
above.

The mutation copies were built from `mutation/base/` as it stood during the
batch. After the last mutation run I corrected two doc comments on `l4_c0`/`l4_c`
in `src/model.rs` (the comments described the little-endian word wrongly; the
definitions were always right), so the copies' `model.rs` differs from the
current `base` by those comments as well as by their mutation. Comments carry no
proof content, so no mutation outcome changes; the crate reverified after the
fix (run 295) and again from clean (run 296).

## 7. Assurance boundary


No additional trust assumption was introduced. The trusted base is the verifier,
`vstd`'s contracts and the compiler's faithful execution of verified code — the
same base as M4a. The 165 `by (compute)` obligations are decided by Verus's
normalizer, whose soundness is part of that same trusted base; they introduce no
axiom and no escape. The L3/L4 lemmas use `compute` only on a single 2- or
3-byte packet.

## 8. Things worth knowing

1. **`by (compute)` is the right tool for 165 small obligations and the wrong one
   for 131,072 packets.** The small vectors total 3,957 input bytes with at most
   129 output pixels, so each `assert ... by (compute)` is cheap; the crate still
   verifies in about 25 s. For L3/L4, `compute` is used on exactly one packet and
   the periodicity lemma does the rest.

2. **`Seq::new` indices are `int`, lengths are `nat`.** `rep_pats`'s closure is
   `|i: int| pats[i % (pats.len() as int)]`; writing `pats.len()` directly makes
   the Euclidean-mod trait resolve with a `nat` RHS and the closure fails to
   typecheck (`expected int, found nat`). The same applies to `rgba_pat(c, n as
   nat)` at the call sites where `n` is an `int`.

3. **The model is the specification, so mutating both sides would be vacuous.**
   A mutation that changes both the model's `mh` and the executable's
   `parse_header` consistently leaves `decode = MP` true and T7 provable; only
   the L2 boundary lemma would fail. The manifest requires T7/T9 to fail, so
   mutation 1 changes the model alone, and the executable (still correct) no
   longer satisfies `decode_eq`. Thirteen of the twenty mutations are model-only
   for this reason (1, 2, 3, 4, 5, 6, 7, 9, 10, 12, 18, 19, 20); the other seven
   (8, 11, 13, 14, 15, 16, 17) are executable-only, because each is a mutation
   the manifest requires to fail *T1/T3/T4/T8 on the concrete code itself* — an
   unguarded read, the loop's own `produced`, `N` arithmetic — which a model
   edit cannot exhibit.

4. **`lemma_mh_prefix` and the `compute` headers.** `MH` reads only the first 18
   bytes and the total length, so `lemma_mh_prefix` proves `mh(hdr + rest) =
   mh(hdr)` for any in-cap `rest`; the small `hdr18` verdict is then decided by
   `compute`. Evaluating `mh` on a symbolic `hdr18` does not work — `compute`
   needs `hdr18` unfolded to its byte sequence — so the L3/L4 headers are also
   written byte by byte (`l3_hdr`, `l4_hdr`), and the generator's `H(...)` is
   checked equal to them.

5. **`lemma_mp_rle_ok` needs a `mr_pix` of exactly `N` pixels.** L3/L4 supply
   `pix_pow(c, 16777216)` and discharge `mp_pixels = pix_pow` through
   `lemma_mr_cur_pix_proj` plus the emptiness of `Seq::empty() + px`; that last
   step needs `lemma_seq_add_len`, since Verus will not unfold `Seq::add` on its
   own for the length equality.

## 9. Runs, tool calls and the final lines

### `./verify.sh` runs this milestone

M4b occupied runs **256-296**: 41 possible numbers, **40 rows in `RUNLOG.tsv`**
(run 278 has a log but no row — see below), **6 exited 0** (256, 258, 265, 294,
295, 296). Every run went through `./verify.sh`; no probe crate and no direct
`cargo verus verify` was used.

- 256-265 (10 runs): building L1-L4 in the real crate. 256 is the first build
  with the generated `model_vectors.rs`; 257-264 are the iterations of L1/L2 and
  L3/L4; 265 is the first 0-error build.
- 266-269 (4 runs): the mutation-1 and mutation-7 pilots and the mutation-1
  rebuild.
- 270-277, 279-292 (22 runs): the twenty mutation outcome runs of the first
  batch (270-277 = mutations 1-8) and the second batch (279-292 = mutations
  8-20, where 279 is mutation 8's definitive run, its 277 predecessor having
  been a first batch run that I deleted mid-flight).
- 293: mutation 5's corrected run (mutation 5's first edit was too broad; the
  refined edit targets 16-bit true-colour only).
- 294: the full reverify after `cargo clean`.
- 295: the reverify after I corrected two doc comments on `l4_c0`/`l4_c`
  (the comments, not the definitions, were wrong; no obligation changed).
- 296: the definitive full reverify after `cargo clean` on the final source
  (`578 verified, 0 errors`, 82 s) — the clean full-tick run.

Run 278 is not a result: it is the `verify.sh` invocation for mutation 9 that
the batch started and I killed (my own `pkill`) when my concurrent anchor-check
loop deleted `mutation/m9` mid-verify; that `verify.sh` never reached its
`RUNLOG.tsv` append, so the row is absent while the log survives. Mutation 9's
definitive run is 280. The first m1 runs (266-268) and the m7 pilot (269) are
superseded by the batch runs 270 and 276 respectively; they remain in the log
because no run log is ever deleted.

### Final result lines

- Real crate, final full reverify after `cargo clean`:
  `verification results:: 578 verified, 0 errors` (`runs/296.log`), 82 s.
- `cargo test --test conformance -- --nocapture`:
  `conformance: 169/169 vectors passed`,
  `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`.
- `tools/gen_model_vectors.py --check`: `up to date`.
- Frozen hashes: `sha256sum` of `spec-frozen/SPEC.md`, `VECTORS.json`,
  `SOURCES.md`, `DESIGN-NOTES.md` and `THEOREMS.md` equal `FREEZE.txt`.

### Tool calls

The harness count is authoritative; my own tally of distinct work is roughly
250 tool calls (reads, generator/mutation-script iterations, the mutation batch
and its result extractions), with a number of short polling calls while the
verifier ran (a mutation run is 80-95 s). I did not hit the 400 cap.

### Deliverables

- `tga_verus/src/model.rs` — 6,800 lines: the M4a model plus L1-L4 and their
  helpers.
- `tga_verus/src/model_vectors.rs` — 2,954 lines: the 165 generated obligations.
- `tga_verus/src/lib.rs` — registers `model_vectors`.
- `tools/gen_model_vectors.py` — the generator, with the large-vector checks.
- `tools/mutations.py`, `tools/run_mutations.sh`, `tools/mutation_summary.py`,
  `tools/mutation_table.py`, `tools/check_anchors.py` — the mutation protocol.
- `mutation/base/`, `mutation/m1`..`mutation/m20/` — the pristine copy, each
  mutation copy's `failure.log` and `result.txt`.
- `PROOF-REPORT-M4b.md`, updated `NOTES.md`.
- Backups `tga_verus/src/tga.rs.m4a`, `tga_verus/src/model.rs.m4a` (the M4a
  files, byte-identical to the pre-M4b state).

