# Errata against the frozen alloc-delta document set

Append-only. This file is **not** covered by `FREEZE.txt`: it records
discrepancies between the frozen texts (`SPEC.md`, `THEOREMS-ALLOC-DELTA.md`,
`TABLE.md`, `REVIEW-SOL.md`) and the implementation, plus naming drift. Nothing
here edits a frozen file. New entries are appended at the end with their date and
the artifact they concern.

## E1 — 2026-11-12: the canonical `alloc_guard/mod.rs` has a fourth `use` line

The canonical production `alloc_guard/mod.rs` includes `use alloc::vec::Vec;` for
all four cores including std-enabled cores. Once those cores adopt that import,
all four production copies must be byte-identical and pinned to the newly
reviewed common hash (TGA currently
`018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6`). Until then
D6 cross-core byte identity is not met.

The frozen `SPEC.md` §1.1/TABLE text that says the guard text is byte-identical
across the four cores predates the no_std conversion. A no_std crate's own
prelude does not re-export `Vec` (DIB's std prelude supplied it), so the guard's
`Result<Vec<u8>, OutOfMemory>` return type needs the explicit import. A std core
that has not converted yet does not carry the line; it must adopt it on
conversion so the four copies converge on the one hash above.

## E2 — 2026-11-12: `SPEC.md` §1.1's TGA inventory row is stale

`SPEC.md` §1.1 lists TGA's verified `try_*` API as `try_decode` alone. The real
set in `tga_verus/src/tga.rs` at the alloc-delta end state is:

- `try_decode` (public, 1.0 path),
- `try_decode_with_policy` (public, named-profile policy path),
- `try_decode_with_steps_logged` (public, the logged/counter entry point), and
- `try_decode_custom_policy` (test-only, `#[cfg(test)]`, arbitrary-record policy
  path).

So the frozen row's "Verified `try_*` API" cell and its "Frozen name, removed
from the main crate" cell (`decode` only) both understate the surface. The
frozen names that were removed are `decode`, `decode_with_policy` and
`decode_custom_policy`.

## E3 — 2026-11-12: A-TGA-4 naming, and the added charge obligation A-TGA-5

Frozen `THEOREMS-ALLOC-DELTA.md` §2 A-TGA-4 is **termination** (T10's argument
carried on `try_decode`; "the `OutOfMemory` branch has no loop"). The work/charge
bound — `steps <= 16*(|b|+N)` at the entry point, with the `4N` output
initialisation charged only after a *successful* reservation — is an **added**
TGA obligation, not frozen A-TGA-4. It is named **A-TGA-5** in the TGA code
comments and in `tga/NOTES.md`. Code that says "A-TGA-4 (charge bound)" is
mis-labelled; the correct label is A-TGA-5, and frozen A-TGA-4 stays the
termination theorem under that name.

## E4 — 2026-11-12: stale comment in `alloc_guard/mod.rs` (text frozen, not fixed)

`tga_verus/src/alloc_guard/mod.rs:3-4` still reads "This is the production copy
for `dib_verus`, adapted from the verified prototype ..." (the file is TGA's
copy, and DIB's own copy is the sibling), and lines 7-14 describe DIB's site
enumeration. The text is **not** changed here: it is hash-pinned by decision D6
(`EXPECTED_MOD_SHA` in `tools/audit_alloc.sh`) and any edit would invalidate the
reviewed common hash. The comment is cosmetic (it does not affect the sealed
body, the `#[inline(never)]` attribute or the log sealing); it is recorded here
instead of edited. When the four copies converge on one hash (E1), the shared
comment text should be made core-neutral in the same review that re-pins the
hash.

## E5 — 2026-11-12: ICO's `try_with_room`/`Reserved`/`MAX_ROOM` live in `sites.rs`, not `mod.rs`

`SPEC.md` §3 lists `Reserved<T>`, `try_with_room<T>` and `MAX_ROOM` in
`alloc_guard/mod.rs`, and §4's ICO row calls `try_with_room(EntryList, count)`.
The shipped production `mod.rs` (SHA-256
`018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6`, byte-identical
across DIB and TGA) carries only `try_zeroed`: DIB and TGA each make one
byte-oriented reservation, so the element-list reservation was dropped from the
shared text. ICO's entry list is a `Vec<Entry>`, so ICO's step declares
`Reserved<T>`, `try_with_room<T>` and `MAX_ROOM = 4,096` in its per-core
`alloc_guard/sites.rs` instead. This keeps `mod.rs` byte-identical (D6) and keeps
every allocation inside `alloc_guard` (SPEC §3, §6). The `sites.rs` hash
(`b2cf3ee525a9f0cfeebffb2a6363120524b7fdb93bb7b5ff520b39ca8b2b5174`) covers them.
The frozen §3 listing should be read as "the guard module (mod.rs plus the
per-core sites.rs)"; no theorem shape changes. ICNS's step, which also reserves
an entry list, will face the same choice.

## E6 — 2026-11-13: `try_zeroed` fills with `resize(n, 0)`; A-STD gains A-STD-4

Decision D4 froze the fill as a `push` loop so that A-STD rests on documented
guarantees, and allowed `resize` later if a measurement showed the loop matters.
It does: for a 64 MiB output the loop costs 47.8 ms against 6.6 ms for
`resize(n, 0)` (`leniency/bench/fillbench/RESULTS.md`), which is the 37 ms
regression the A/B benchmark measured against the old `vec![0; n]`. The user
approved extending A-STD on 2026-11-13. The guard body becomes
`try_reserve_exact(n)` then `v.resize(n, 0u8)`; `mod.rs` stays byte-identical
across the four cores, with new SHA-256
`26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7` (pins updated
in the four `tools/audit_alloc.*`). A-STD-4 (SPEC §3) is observed behaviour of
std 1.99.1, not a documented guarantee; a reallocation inside `resize` would be
infallible and would abort, so it cannot be detected after the fact. Earlier
references to the "push loop" in per-core notes describe the pre-E6 guard.

**A-STD-4 clause (amends SPEC §3; SPEC.md itself is frozen and unchanged).** **Observed, not documented.** `Vec::<u8>::resize(n, 0)` on a `Vec<u8>` with `len == 0` and `capacity() >= n` does not allocate or reallocate. `Vec`'s docs say bulk insertion methods "may reallocate, even when not necessary", so this clause rests on the pinned std source (rustc 1.99.1) and on the counting-allocator and fault-injection tests (one allocation, zero reallocations). It replaces the push loop in `try_zeroed`; A-STD-2 is no longer used by `try_zeroed`.
