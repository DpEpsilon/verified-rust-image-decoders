# Errata against frozen POLICY-SPEC

This file records defects found in the frozen policy specification
(`POLICY-SPEC.md`, `THEOREMS-POLICY-DELTA.md`, `policy_oracle.py`,
`gen_policy_vectors.py`, `POLICY-VECTORS.json`, `CUSTOM-VECTORS.json`; see
`FREEZE.txt`). It is a new file, not covered by `FREEZE.txt`, and it never edits
a frozen file silently. Each entry names the frozen text, the applied change,
and the new hashes.

## E1 — 2026-11-14: owner-approved rename `Recommended` -> `RecommendedBeta`

Owner decision (2026-11-14): every `Recommended` policy profile is renamed
`RecommendedBeta`, because the field set is **provisional until real-world
feedback**. Nobody uses the project yet and no version was released, so this is
a plain rename, not a semantic change. It applies to this frozen DIB policy
layer; ICNS and ICO policy layers are deferred and were left untouched, and the
new GIF and PNG drafts already use the new name.

### What changed

The profile name only: the Rust selector variant, the spec/proof function
names, the string keys, and the prose. Semantic content — every field value of
the `RecommendedBeta` record (`file_size=Ignore`, `image_size=Ignore`, all other
fields at their Strict value) — is unchanged. Concretely:

- Rust identifiers: `BmpProfile::Recommended` -> `BmpProfile::RecommendedBeta`,
  `recommended_policy` -> `recommended_beta_policy`,
  `t1_policy_recovery_recommended` -> `t1_policy_recovery_recommended_beta`.
  (`CUSTOM-VECTORS.json` carries the field record, not the profile name, so it
  is unchanged and its hash is unchanged.)
- String keys and prose: `"Recommended"` -> `"RecommendedBeta"` in
  `POLICY-VECTORS.json`, `POLICY-SPEC.md`, `THEOREMS-POLICY-DELTA.md`, and the
  oracle's `PROFILES` map.
- The oracle's Python constant `RECOMMENDED` -> `RECOMMENDED_BETA`; the record
  it builds (`dict(STRICT, file_size='Ignore', image_size='Ignore')`) is
  unchanged.

### Evidence the rename is semantics-preserving

- `gen_policy_vectors.py`, after the rename, regenerates `POLICY-VECTORS.json`
  byte-identically to the file recorded in this erratum.
  `policy_oracle.py` then passes every vector and every custom probe
  (`102` policy vectors per profile, `66` custom probes).
- Verus: DIB `verify.sh` run 557 -> `1764 verified, 0 errors` (the count before
  the rename); ICO run 178 -> `346 verified, 0 errors`.
- `cargo test --release` in `dib_verus`: `policy_conformance_all_profiles`
  reports `RecommendedBeta: 102/102`; the corpus acceptance counts are unchanged
  (Strict 50, RecommendedBeta 52, GdkCompat 58, ImageCrateCompat 55 of 71).

### Which text controls

The field record is the frozen `CUSTOM-VECTORS.json` and the `RECOMMENDED_BETA`
record in `policy_oracle.py`; the name is a label on that record. The rename
changes the label everywhere and nothing else.

### Frozen hashes

The files below are frozen by `FREEZE.txt`. New hashes below replace the earlier
ones for the listed files; the earlier hashes are preserved in this erratum and
in the superseded block of `FREEZE.txt`.

| File | SHA-256 before | SHA-256 after |
|---|---|---|
| `POLICY-SPEC.md` | `5e1bad0579b4eb55721a5c570f7577ae589c2e96b4e5cb269cb951fb2f6d9ce4` | `d26c66a4d989294cd0a02413de5438950c3e27b9abcad1f0eedf68ebd5441e51` |
| `THEOREMS-POLICY-DELTA.md` | `c98c00701653911c9c8eb9f7b827bbf30f92bde8f0f0f2c8d84026127285d8cc` | `447017cf71411499cf69610e81e46d05f2a1eb51ba2a629403eee24c2833daad` |
| `policy_oracle.py` | `3935542bb744e3295005df85cf28a2c9c0081961e5ee0a962d8a99f95d7c7a07` | `ac35d1173bccbf046556eee9bc9e3fdeff485bc5e06e66c052f220aeb38e3fe5` |
| `gen_policy_vectors.py` | `2c25c25128d0313f42725837ff04defb1afeb735e55d34f6e7984bdb5d153867` | `3caa6b0702dc7ba8651a9308e9cb77b0e6db105064183c175e3982ce7ca18387` |
| `POLICY-VECTORS.json` | `bd82a5ce960a721c5ed3bd955eff666984aa7fc114618b10b518b3fb5647a02c` | `d796a0ffe563e9d6477bbdf473e4db286d4abefa3e3925baf45e244fb91f529a` |
| `CUSTOM-VECTORS.json` | `89119d99786f464da8c7543836babac0d61dadb2683f445ccdcfbbfcf94696c5` | `89119d99786f464da8c7543836babac0d61dadb2683f445ccdcfbbfcf94696c5` (unchanged) |

Non-frozen files in this directory that also changed (no erratum hash needed,
listed for completeness): `README.md`, `REPORT.md`, `REVIEW.md`.
