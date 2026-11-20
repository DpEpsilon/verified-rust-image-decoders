# Proposed ERRATA entries from the council's M1b-Q rulings (2026-11-20)

Two notation entries for `ERRATA.md`, written in its format and marked PROPOSED, ready to append as E3 and E4 (renumber if other entries land first). Both are notation only: neither changes a statement of `THEOREMS.md`. The rulings are unanimous (council report M1BQ, blocks Q5a and Q5b); the text below is the union of the four seats' round-3 wordings. `ERRATA.md`, `THEOREMS.md` and `spec-frozen/` were not edited.

Index rows to add:

| Erratum | Status |
|---|---|
| E3 | PROPOSED (2026-11-20, council M1b-Q5a) |
| E4 | PROPOSED (2026-11-20, council M1b-Q5b) |

---

## E3 — 2026-11-20: T13, the samples of `Still16::rgba16` (notation) — PROPOSED

**Frozen text.** `THEOREMS.md:124` (T13) states its RGBA16 clauses over samples: "`|rgba16| = 4·W·H` samples, `8·W·H ≤ 2·MAX_OUTPUT` bytes", and L-16-8 relates "every 8-bit sample" to "the 16-bit sample `x`". `THEOREMS.md:60` gives `MS16(b)` the field `rgba16`. Neither fixes how the executable lays out a 16-bit sample.

**Implementation.** `png_verus`'s `Still16::rgba16` is a `Vec<u8>` of `8·W·H` bytes: four samples per pixel in R, G, B, A order, each sample two bytes, most significant first (big-endian), on every host. The council kept this representation (M1b-Q5a, option (a)) because the shared, hash-pinned `alloc_guard` reserves bytes and stays byte-identical across cores (a `Vec<u16>` would need a second element type in the guard), and native-order bytes would make the output depend on the host.

**Proposed erratum (notation, no statement changes).** T13's model sample `i` of `rgba16` corresponds to `256·bytes[2i] + bytes[2i+1]` of the executable's `Still16::rgba16`, and `|bytes| = 8·W·H`, representing `4·W·H` samples. T13 for the executable is proved through this relation: the output byte length is `8·W·H`, and decoding adjacent byte pairs as big-endian yields exactly P13.3's sample sequence. The `Output` allocation remains byte-counted and the shared allocation guard remains unchanged. This records the representation relation without changing T13's sample semantics.

**Effect on users.** None on decoded values. Consumer glue converts when its destination needs another layout; on a little-endian host, reinterpreting the bytes as native `u16` is not a conversion (`BRIEF.md`).

**If rejected.** T13's 16-bit clauses have no stated link to the public byte field, and the M6 proof must fix one anyway.

---

## E4 — 2026-11-20: `Frames` constructor names (notation) — PROPOSED

**Frozen text.** `THEOREMS.md:94` (the request table) and `:143` (T-ITER's setup) write `Frames::try_new(b, p)`; `:119` (T9) writes `c = Frames::try_new(b, Strict)`; `:100` and `:157` (A-PNG-2) write the logged form `Frames::try_new_logged(b, p)`. `BRIEF.md` briefed the single constructor `try_new(b, p)`.

**Implementation.** Following D1 (`policy-draft/OPTIONS.md`: stable entry points are Strict, profiled ones take a named profile) and PNG's still entries and the GIF, DIB and TGA constructors, the council renamed the constructor (M1b-Q5b, option (b)): `Frames::try_new(b)` constructs Strict decoding and `Frames::try_new_with_policy(b, p)` takes a named profile. Both use the same constructor implementation; `try_new(b)` is defined as `try_new_with_policy(b, Profile::Strict)`, including identical errors and guard requests. `Frames` still does not implement `Iterator`, because each returned frame view borrows the decoder's reusable canvas. `png_verus`, its tests, its probe example and the leniency `verified` runner use the new names.

**Proposed erratum (notation, no statement changes).** In `THEOREMS.md`, `Frames::try_new(b, p)` denotes the executable `Frames::try_new_with_policy(b, p)`, and `Frames::try_new(b, Strict)` denotes the Strict wrapper `Frames::try_new(b)`. This naming correspondence changes neither `MWF_p`, nor allocation order, nor frame semantics. Provisional callers and unfrozen API documentation are updated consistently.

*Addition for the owner's decision (not in the council's wording):* the logged form `Frames::try_new_logged(b, p)` (`THEOREMS.md:100`, `:157`) is read the same way, as the logged form of `try_new_with_policy(b, p)`, with `try_new_logged(b, Strict)` the logged form of `try_new(b)`. No logged entry exists in the M1b code; the sentence keeps the two spellings from diverging when M3–M6 add them.

**Effect on users.** Callers of the provisional two-argument `try_new` change their source; decoding and allocation behaviour are unchanged.

**If rejected.** `Frames` stays the one constructor whose unqualified name takes a profile, contrary to D1, and the code reverts to `try_new(b, p)`.

---

## Not errata: council wordings addressed to `THEOREMS.md` and recorded elsewhere

Some seats addressed other text to `THEOREMS.md`, which is frozen. These are new obligations or Stage-2 model text, not corrections of a frozen statement, so they are recorded where the project keeps such items and need no erratum now:

- Policy form of L-REWALK (M1b-Q2; fable, kimi and opus45 wrote it for `THEOREMS.md` at P1, astra for `LEDGER.md`): recorded as an M3 obligation in `PLAN.md` (M3 gates), `png_verus/LEDGER.md` and `policy-draft/DECISIONS.md`.
- D16's model text (M1b-Q5d; opus45 wrote it for `THEOREMS.md` directly, the other seats for `DECISIONS.md` as the adopted proposal entered in `THEOREMS.md` at the Stage-2 freeze): recorded in `policy-draft/DECISIONS.md` for the Stage-2 freeze.
