# Errata against frozen POLICY-SPEC

This file records defects found in the frozen policy specification
(`POLICY-SPEC.md`, `THEOREMS-POLICY-DELTA.md`, `policy_oracle.py`,
`gen_policy_vectors.py`, `POLICY-VECTORS.json`, `QUESTIONS.md`; see `FREEZE.txt`).
It is a new file, not covered by `FREEZE.txt`, and it never edits a frozen file.
Each entry names the frozen text, the verified implementation behaviour, the
controlling text, and what a later revision should say.

## E1 — 8-bit greyscale with descriptor attribute count 8

### Frozen text

`POLICY-SPEC.md` line 24 (`attribute_bits`), the `ImageTable` sentence:

> allowed counts as R5 except 8-bit grey allows {0, 8} (8 is treated like 0) …

`THEOREMS-POLICY-DELTA.md` lines 9–12 add, to the policy header record, `eff`:

> the **effective attribute count** `eff` (equal to the descriptor count whenever
> R5 allows it; otherwise the `Lenient` value or `InvalidAttributeBits`) …

### Implemented behaviour

Three independent layers retain effective count 8 for an 8-bit greyscale image
whose descriptor attribute count is 8, rather than treating 8 like 0:

- the oracle (`policy_oracle.py` lines 59–61 and 153–162): `IMAGE_ATTR[('grey',
  8)] = (0, 8)` makes the count pass the `ImageTable` acceptance test, so `eff =
  count = 8`;
- the model (`policy_model.rs` lines 285–335 `image_attr_allowed`/`eff_attr`, and
  lines 1076–1090 `gov_depth`/`eff_count_img`): the accepted count is kept;
- the executable (`tga.rs` lines 5148–5165 `eff_attr_exec`): the accepted count
  is kept.

`MHModel` has no `eff` field: the effective count is derived from the record's
own fields (`image_type`, `pixel_depth`, `map_entry_depth`, `descriptor`) by
`eff_count_img`, so the delta's "adds to the 1.0 header record … `eff`" is not
how the model carries it. The header *record* is therefore the same 15 fields as
1.0; only the derived conversion projection `PktH` carries `attr = eff_count_img`.

### Effect on public output

None for the pixels. An 8-bit greyscale image is non-indexed with `kind = 2` and
`depth = 8`, so `sample_bytes = 1`; `conv_sample_policy` writes the byte to R, G
and B and alpha 255 unconditionally, reading no attribute byte. `stored_alpha`
is consulted only for 32-bit true-colour, 32-bit palette entries and 16-bit grey
(`policy_model.rs` `stored_alpha_policy`/`stored_alpha_img`), none of which is an
8-bit greyscale image. The effective count therefore reaches no alpha or pixel
decision on this element, and all four profiles decode the file to the same
RGBA8 bytes whichever of {0, 8} the count is given. The corpus counts
(Strict/RecommendedBeta/GdkCompat/ImageCrateCompat) are likewise unchanged, because
acceptance depends only on membership in the allowed set, which both readings
give.

### Which text controls

The listed layers are the proof obligation: `POLICY-SPEC.md` line 24's parenthetical
"(8 is treated like 0)" is superseded by the acceptance sentence that precedes it
("8-bit grey allows {0, 8}") and by the oracle, model and executable, which are
consistent with each other and are what the frozen `POLICY-VECTORS.json` was
generated from. The delta's `eff` description is a summary of intent, not a field
the record literally gained.

### What a revision should say

- `POLICY-SPEC.md`, `attribute_bits`/`ImageTable`: strike "(8 is treated like 0)".
  Write instead: "8-bit grey allows {0, 8}; an accepted count is kept as the
  effective count, as for every other element." If the intended behaviour really
  is to treat 8 like 0, the three implementation layers must change together and
  the frozen vectors regenerated — it is not a documentation-only fix.
- `THEOREMS-POLICY-DELTA.md` lines 9–12: say the effective count is a *derived*
  field of the conversion projection, not a new field of the header record:
  "`MH(b,P)` is the 1.0 header record unchanged; the conversion projection
  (`PktH`) carries `attr = eff`, the count the record's `attribute_bits` table
  accepts, which equals the descriptor count whenever that count is allowed."

No frozen file was modified to record this errata. No proof-visible behaviour
changes.

## E2 — 2026-11-14: owner-approved rename `Recommended` -> `RecommendedBeta`

Owner decision (2026-11-14): every `Recommended` policy profile is renamed
`RecommendedBeta`, because the field set is **provisional until real-world
feedback**. Nobody uses the project yet and no version was released, so this is
a plain rename, not a semantic change. It applies to this frozen TGA policy
layer; ICNS and ICO policy layers are deferred and were left untouched, and the
new GIF and PNG drafts already use the new name.

### What changed

The profile name only: the Rust selector variant, the spec/proof function
names, the string keys, and the prose. Semantic content — every field value of
the `RecommendedBeta` record (`trailing=Ignore`, `rle_overrun=ClipPacket`,
`attribute_bits=Lenient`, `map_header=IgnoreAbsent`, all other fields at their
Strict value) — is unchanged. Concretely:

- Rust identifiers: `Profile::Recommended` -> `Profile::RecommendedBeta`,
  `recommended_policy` -> `recommended_beta_policy`, the model lemmas
  `lemma_map_recommended_on_strict` -> `lemma_map_recommended_beta_on_strict`,
  `lemma_map_coherence_recommended` -> `lemma_map_coherence_recommended_beta`,
  `lemma_mh_policy_recommended_on_strict_ok` ->
  `lemma_mh_policy_recommended_beta_on_strict_ok`,
  `lemma_conv2_policy_recommended_on_strict` ->
  `lemma_conv2_policy_recommended_beta_on_strict`,
  `lemma_mp_policy_recommended_on_strict_ok` ->
  `lemma_mp_policy_recommended_beta_on_strict_ok`,
  `lemma_after_rle_shape_recommended` ->
  `lemma_after_rle_shape_recommended_beta`,
  `r_recommended_conservative` -> `r_recommended_beta_conservative`, and the
  generated witness names in `model_vectors.rs`
  (`policy_bytes_recommended_attr_32_count4` ->
  `policy_bytes_recommended_beta_attr_32_count4`, and likewise
  `..._map_absent_stray_depth`), with their generator
  `tools/gen_policy_header_witnesses.py` renamed in step.
- String keys and prose: `"Recommended"` -> `"RecommendedBeta"` in
  `POLICY-VECTORS.json`, `POLICY-SPEC.md`, `THEOREMS-POLICY-DELTA.md`,
  `QUESTIONS.md`, and the oracle's `PROFILES` map.
- The oracle's Python constant `RECOMMENDED` -> `RECOMMENDED_BETA`; the record
  it builds (`dict(STRICT, trailing='Ignore', rle_overrun='ClipPacket',
  attribute_bits='Lenient', map_header='IgnoreAbsent')`) is unchanged.

The oracle's `OPTIONS` map, the field record itself, and every expected vector
result are untouched. `tga/tools/policy_cases.py` uses `oracle.RECOMMENDED` as a
base record; the constant was renamed and its uses updated.

### Evidence the rename is semantics-preserving

- `gen_policy_vectors.py`, after the rename, regenerates `POLICY-VECTORS.json`
  byte-identically to the file recorded in this erratum.
  `policy_oracle.py` then passes all `169` frozen TGA-CORE-SPEC 1.0 vectors
  under Strict, all `302` vectors under each of the four profiles (including
  `RecommendedBeta`), and all `28` custom probes.
- Verus: TGA `verify.sh` run 232 -> `974 verified, 0 errors` (the count before
  the rename).
- `cargo test --release` in `tga_verus`:
  `policy_tests::frozen_policy_vectors` reports `RecommendedBeta: 302/302`; the
  corpus acceptance counts are unchanged (Strict 252, RecommendedBeta 266,
  GdkCompat 264, ImageCrateCompat 259 of 294).

### Which text controls

The field record is the frozen `POLICY-VECTORS.json` `custom` rows and the
`RECOMMENDED_BETA` record in `policy_oracle.py`; the name is a label on that
record. The rename changes the label everywhere and nothing else.

### Frozen hashes

The files below are frozen by `FREEZE.txt`. New hashes below replace the earlier
ones for the listed files; the earlier hashes are preserved in this erratum and
in the superseded block of `FREEZE.txt`.

| File | SHA-256 before | SHA-256 after |
|---|---|---|
| `POLICY-SPEC.md` | `7dd067627e0dc924ece9f8ea0f374ccc66386a436e5f87a2b3812965ca2d3b79` | `a14b9d0d38263b32609de8670bc8778690747f5fb1ef1a4de64f8fe4d02fd247` |
| `THEOREMS-POLICY-DELTA.md` | `a41deb479007ab7f0b98ef9e29c129e56afc5f7a4e79460a105844293e5b269c` | `3ad17f7e6a6ba02e9ab0259504586384d75868c14327bc06041fcba18ae3e4af` |
| `policy_oracle.py` | `818022eb3dba777133e546d5480bc9e1b83df97bfd78c126c34d2e6911a17c5` | `50939f1b1551a70bb4f248421d3b6b56afe772716a404a9bdd93e394441ceab3` |
| `gen_policy_vectors.py` | `9810f3e235eb294ff186e1a5dcc86f344a09656632afdb406c35917c3044d3e0` | `152b81417be85aa56b866c93ef567e34cef8f489dedb7d7059b77913000247ae` |
| `POLICY-VECTORS.json` | `6db237c003b8d4a595b22c1253b2eb9e23c3fba1005c93e7af75dc7dd03d77b2` | `852d20aca678e05c63ba8eb33e2d24f5564c37e23fd5584e77de10d1eb62c3a2` |
| `QUESTIONS.md` | `68681c6d0be63a158c73011a1e9d322a0b9a0e16ac2929a4c64a9166724b85b5` | `b02855569df6476e144b638cc4f1ee7cec841f048f398c5fd183e1fef44ad8a6` |

Non-frozen files in this directory that also changed (no erratum hash needed,
listed for completeness): `REPORT.md`, `REVIEW.md`, `validate.py`.
