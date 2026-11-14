# Rename survey: `Recommended` -> `RecommendedBeta` (DIB)

Owner decision 2026-11-14: every `Recommended` policy profile is renamed
`RecommendedBeta` ("provisional until real-world feedback"). Nobody uses the
project yet and no version was released, so this is a plain rename. It applies
to the already-frozen DIB policy layer (ICNS and ICO policy layers are deferred;
the new GIF and PNG drafts already use the new name). Semantic content — every
field value of the record — is unchanged.

This file is a survey of every occurrence, what was renamed, and what was
deliberately left. It is a new file, not covered by `FREEZE.txt`.

## Scope

Searched: `dib/`, `tga/`, `bench/`, `gdkpixbuf-loaders/`, `alloc-delta/audit/API-SURVEY.md`,
and (for the glycin fork) `~/projects/glycin-plan/glycin/glycin-loaders`.

Excluded (historical snapshots, run logs, generated run output — never edited):
`target/`, `runs/`, `alloc-wip-1..3/`, `mutation*/`, `spec-history/`,
`*.alloc-*`, `*.p2*`, `*.pol-*`, `*.pre-nostd`, `*.nostd-*`, `*.log`, `*.txt`,
`*.tsv`, and `alloc-delta/audit/runs/`.

Out of scope by the owner's decision (other formats / not this rename):
`gif/`, `png/` (already `RecommendedBeta`), `icns/` and `ico/` policy drafts
(ICNS/ICO policy layers deferred; their drafts say `Recommended` and are left).

## Counts per category (this directory, DIB)

Tokens renamed in this directory: **14** occurrences of the profile name, across
6 files. (The TGA directory has its own survey; see `tga/policy/RENAME-SURVEY.md`.)

| Category | File | Occurrences | What changed |
|---|---|---|---|
| Frozen spec text | `POLICY-SPEC.md` | 6 | `Recommended` -> `RecommendedBeta` in the profile table, the D2/D3 decisions, and prose |
| Frozen theorems | `THEOREMS-POLICY-DELTA.md` | 1 | the profile-selector list gains `RecommendedBeta` |
| Frozen oracle (Python) | `policy_oracle.py` | 3 | constant `RECOMMENDED` -> `RECOMMENDED_BETA`; `PROFILES` key `'Recommended'` -> `'RecommendedBeta'` |
| Frozen generator (Python) | `gen_policy_vectors.py` | 1 | `result['Recommended']` -> `result['RecommendedBeta']` |
| Frozen vectors | `POLICY-VECTORS.json` | 1 | top-level key `"Recommended"` -> `"RecommendedBeta"` |
| Frozen vectors | `CUSTOM-VECTORS.json` | 0 | unchanged (carries the field record, not the profile name) |
| Non-frozen docs | `README.md`, `REPORT.md`, `REVIEW.md` | 3 + 18 + 2 | profile name in prose (`REPORT.md` line 14 is ordinary English, kept) |

## Counts per category (Rust, DIB)

| Category | File | Occurrences | What changed |
|---|---|---|---|
| Enum variant | `dib/dib_verus/src/dib.rs` | 3 | `BmpProfile::Recommended` -> `BmpProfile::RecommendedBeta` (variant, two match arms) |
| Spec fn | `dib/dib_verus/src/dib.rs` | 3 | `recommended_policy` -> `recommended_beta_policy` (definition + 2 calls) |
| Proof fn | `dib/dib_verus/src/dib.rs` | 2 | `t1_policy_recovery_recommended` -> `t1_policy_recovery_recommended_beta` |
| Doc comments | `dib/dib_verus/src/dib.rs` | 4 | profile name in `///` prose |
| String key | `dib/dib_verus/src/policy_tests.rs` | 4 | `"Recommended"` -> `"RecommendedBeta"` and the match arm |
| String key | `dib/dib_verus/tests/policy_conformance.rs` | 5 | `PROFILES` table entries and the corpus acceptance table |
| Docs (non-frozen) | `BRIEF-POLICY.md` (4), `LEDGER.md` (2), `NOTES.md` (15), `NO_STD.md` (1), `PROOF-REPORT-P1.md` (3), `PROOF-REPORT-P2a.md` (2), `REVIEW-P2.md` (1), `STUCK.md` (1) | 29 | profile name in prose; `recommended_policy` in backticks |

## What was deliberately not renamed

| Occurrence | File | Why |
|---|---|---|
| "Recommended first implementation step" | `dib/policy/REPORT.md:14` | ordinary English, not the profile |
| "recommended: avoids consuming another entry" | `dib/spec-1.1-frozen/DESIGN-NOTES.md:30` | ordinary English (a recommendation in prose); also a frozen core-spec file, out of the rename |
| "recommended" strings inside `*.p2*`, `*.m11*`, `*.p1`, `spec-history/`, `mutation*/`, `alloc-wip-*` | many | historical snapshots; never edited |
| `Recommended` in `gif/`, `icns/policy-draft/`, `ico/policy-draft/`, `png/` sources | those dirs | other formats or deferred layers |
| `Recommended` in upstream spec text (`png-3e.txt`, `spec-gif89a.txt`, `rfc1951.txt`) | `*/sources/` | quoted specification prose, not this project's profile |

## Rename convention

- Rust identifiers and string keys: `Recommended` -> `RecommendedBeta`;
  `recommended_*` -> `recommended_beta_*`.
- Python constants: `RECOMMENDED` -> `RECOMMENDED_BETA`; the `PROFILES` dict key
  `'Recommended'` -> `'RecommendedBeta'`.
- Bench selector id (a hyphenated benchmark name, not a Rust identifier):
  `bmp-verified-policy-recommended` -> `bmp-verified-policy-recommended-beta`,
  likewise `tga-verified-policy-recommended-beta`. The result JSON files under
  `bench/results/` and the rendered `tables.md` were updated to match, so
  `bench.py` and `report.py` stay consistent.
