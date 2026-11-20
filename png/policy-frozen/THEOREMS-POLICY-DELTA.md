# PNG policy theorem delta (draft, for the Stage-2 freeze)

This is a proposed extension of the policy rows of `../THEOREMS.md` (P-BRIDGE, P-T9, P-E, P-R, P-UNIV, T-CHUNKS and the D14–D16 working proposals), in the form of `../../dib/policy/THEOREMS-POLICY-DELTA.md`. It is **not a completed proof** and does not edit `THEOREMS.md`, which is frozen at Stage 1. It records how the RecommendedBeta field set fixed before the Stage-2 freeze changes those rows: the owner's decisions PNG-Q1–Q15 (2026-11-16), the council's M1b-Q rulings and the council's RB-S2 rulings (both 2026-11-20; `DECISIONS.md`). The Stage-2 freeze enters it, as amended then, next to the frozen manifest.

## Model and executable boundary

The 30-field record is internal; `Profile` is `#[non_exhaustive]` with the named profiles `Strict` and `RecommendedBeta` (the compatibility profiles GdkCompat-2.42.12 and ImageCrateCompat-0.25.10 follow at P2). Arbitrary records are constructible only under `#[cfg(test)]` (`png_verus`: behind the `test-records` feature). The stable entry points stay Strict; the profiled ones are `try_decode_still_with_policy`, `try_decode_still16_with_policy` (D16, provisional), `Frames::try_new_with_policy` (council M1b-Q5b; `THEOREMS.md`'s `Frames::try_new(b, p)`, ERRATA E4), and at P1 `try_icc_profile_with_policy` (D14) and `chunks` (D15).

RecommendedBeta differs from Strict in **thirteen** fields: twelve full relaxations and the narrowed `zlib_tail`.

| Field | RecommendedBeta | Source |
|---|---|---|
| `crc_ancillary` | `SkipExceptFdat` | PNG-Q2; a bad-CRC fcTL in the frames walk is structural (M1b-Q2) |
| `iend` | `Lenient` | PNG-Q1; kept, RB-S2 Q1 |
| `trailing` | `Ignore` | PNG-Q1; kept, not narrowed, RB-S2 Q4a |
| `ancillary_order` | `Drop` | PNG-Q1; kept, not narrowed, RB-S2 Q4b |
| `ancillary_duplicate` | `FirstWins` | PNG-Q1 |
| `ancillary_values` | `Drop` | PNG-Q1 |
| `plte_unexpected` | `Ignore` | PNG-Q1; kept, RB-S2 Q3b |
| `palette_size` | `Allow` | PNG-Q1; kept, RB-S2 Q3a |
| `palette_index` | `Black` | PNG-Q1, Q6 |
| `window` | `Ignore` | PNG-Q1 |
| `apng_in_still` | `Ignore` | PNG-Q1 (D22) |
| `apng_errors` | `Static` | PNG-Q11 |
| `zlib_tail` | `IgnoreTail` (narrowed) | PNG-Q1; exactly-`R` output only, M1b-Q1 |

`missing_iend` and `trns_invalid` take Strict's `Reject` in RecommendedBeta (RB-S2 Q2 and Q3c, 3–1). Their values `AtEof` and `Drop` remain in the record for GdkCompat-2.42.12, so their branches and proofs move to P2.

## Theorem inventory

| Claim | Required policy change |
|---|---|
| P-BRIDGE | Unchanged: `M*_p(b, STRICT) = M*` at spec level; the executable pairs as stated, with `Frames::try_new(b)` defined as `Frames::try_new_with_policy(b, Strict)` (identical errors and guard requests, M1b-Q5b). |
| P-T9 | T9, T10, T13 and T-ITER for every record whose fields take only Strict or RecommendedBeta values: **2^13 records** (was 2^15). T13's pixel clauses hold for these records except that under `palette_index = Black` a colour type 3 pixel may be `(0,0,0,255)` with an index `≥ E`, and, where a tRNS was dropped (through `crc_ancillary`, `ancillary_order` or `ancillary_duplicate`), the tRNS clauses refer to the tRNS chunk the record kept. The clause's `trns_invalid = Drop` case no longer arises for these records. Under `zlib_tail = IgnoreTail` P11 under policy renames no inflate error (M1b-Q1), so P-T9 carries no renaming arm. |
| P-E | Unchanged in outcome: under `STRICT` no event occurs; under RecommendedBeta R2 and R5 never occur, and R1, R3 and R4 are the reachable events. R1 is reached only through `crc_ancillary`, `ancillary_order` and `ancillary_duplicate` (no longer through `trns_invalid`). |
| P-R | Obligation R and lemma L-CONS as stated, with **thirteen inertness lemmas**, one per RecommendedBeta value: `crc_ancillary = SkipExceptFdat`, `iend = Lenient`, `trailing = Ignore`, `ancillary_order = Drop`, `ancillary_duplicate = FirstWins`, `ancillary_values = Drop`, `plte_unexpected = Ignore`, `palette_size = Allow`, `palette_index = Black`, `window = Ignore`, `apng_in_still = Ignore`, `apng_errors = Static` and `zlib_tail = IgnoreTail` (acts only after an Adler-32 that matched an output of exactly `R` bytes, where Strict either ends or returns `TrailingZlibData`). `missing_iend = AtEof` and `trns_invalid = Drop` are removed from the list. `apng_errors = Static` is inert on Strict-accepted inputs also for the structural `CrcMismatch` of M1b-Q2, because Strict returns the mandatory `CrcMismatch` at the same fcTL. |
| P-UNIV | Unchanged: T1, T-SEG, A-PNG-1–7, T11 and T12 for every record value, including `AtEof` and `Drop`, which stay in the record type. |
| L-REWALK, policy form (M3) | New (M1b-Q2): for successful fdAT animation selection under every Strict/RecommendedBeta-valued record, every fcTL bounding a selected frame has a walk-verified CRC; the re-walk reads only lengths and types; the `Static` fallback selects no fdAT stream. The Strict lemma does not establish it. |
| L-WALK-FIRST (M3) | Clarified (M1b-Q2): a fcTL whose CRC does not verify is a fcTL that failed its checks; see `../PLAN.md` M3. |
| L-INFO-RANGE (M3) | New (M1b-Q5c): every returned `Meta::Present((o, n))` satisfies `o ≤ |b|`, `n ≤ |b| − o` and denotes `MW_p(b)`'s range without copying. |
| MS16_p, MI_p, MChunks_p | D16's model text is adopted for the Stage-2 freeze (M1b-Q5d, `DECISIONS.md`); D14 and D15 stay open and gate their entry points at P1 (M1b-Q4). |

## Effect on proof size

Two fewer inertness lemmas and a P-T9 record space a quarter of the size. The `AtEof` branch (EOF at a chunk boundary after the first IDAT acting as IEND) and the `Drop` branch of `trns_invalid` are still implemented for GdkCompat-2.42.12 and are proved at P2 with that profile; P1 needs only that RecommendedBeta takes Strict's branch at both sites.
