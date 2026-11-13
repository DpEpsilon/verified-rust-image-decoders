# ICO/CUR container core

`spec/` holds the frozen ICO-CORE-SPEC 1.0 (217 vectors, independent Python oracle, hashes in `spec/FREEZE.txt`); `THEOREMS-DRAFT.md` is the (frozen) manifest; `REVIEW*.md` are the independent reviews. A verified Verus core implements it (346 Verus obligations verified, 0 errors, run 175 on 2026-11-13): the ICO/CUR container core, built over the DIB core, `#![no_std]` with `alloc`. The core delegates DIB entries to `../dib` in `IcoEntryDibSlack` mode (spec `../dib/spec-1.1-frozen`).
Note: `spec/dib_oracle_pin.py` and the tools still point at the original workspace path of the DIB 1.1 oracle; the same file is `../dib/spec-1.1-frozen/check_vectors.py` (sha256 `214dbf12cd63e8967e1af401297cff5ac16922653d969998e7addc0d2bdd5488`). Edit the path to run the checker elsewhere.

## alloc-delta (fallible allocation, no_std)

`ico_verus` is `#![no_std]` and uses only `core` and `alloc`. Its one dependency
is `vstd`, built with `default-features = false, features = ["alloc"]`, plus the
`dib_verus` path dependency. The crate exports only verified `try_*` entry
points returning `TryError { Format(Error), OutOfMemory(..) }`; the frozen names
(`parse`, `parse_with_steps`, `decode_entry`, `decode_entry_with_steps`) are
removed. The crate's allocations are
intended to go through the sealed `alloc_guard`: ICO's entry list through
`try_with_room`, and DIB's output through
`dib_verus::try_decode_with_steps_logged` imported by `import_dib`. Established
so far: 346 Verus obligations verified (run 175). Run 172 had 347: the
alloc-delta delivery's 343 plus the two new PreAlloc corollaries and the two
bridge lemmas that connect them to the public contracts (the two
`name`/`name_spec` postconditions add no new function). The one-obligation drop
coincides with the guard change (the same drop appears in DIB, TGA and ICNS; the
obligation lists were not diffed): `try_zeroed` now fills with `resize(n, 0u8)` after
`try_reserve_exact(n)` instead of a `push` loop (ERRATA E6, trusted item A-STD-4,
observed behaviour of std 1.99.1 and not a documented guarantee). The shared
`alloc_guard/mod.rs` has SHA-256
`26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`, identical in
all four cores. Implementation and source-lint checks run. Not produced: the production call-graph allocation audit, the independent
build-input inventory and the signed allowlist. The trust boundary is A-STD, the
hash-reviewed guard text, and the future signed allowlist. See
`NO_STD.md` for the build-time evidence and `NOTES.md` for the step record.
