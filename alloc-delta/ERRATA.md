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

## E7 — 2026-11-13: the production audit tool (`alloc-delta/audit/`) deviates from `SPEC.md` §6 in nine places

The production tool lives at `alloc-delta/audit/tool`, not `tools/alloc_audit`, and its allowlists live at `alloc-delta/audit/allowlist/<target>/<profile>/<crate>.toml`. `alloc-delta/audit/README.md` states what it checks, the SPEC §8 prerequisite status, and the evidence runs. Every deviation below either makes the tool stricter than §6 or records a point where the frozen text could not be followed literally.

1. **Roots.** §6 step 1 takes every `pub` non-generic function as a root and says no core exports a generic function. The tool roots every exported function (trait-impl methods included) and `reachable_non_generics`. ICNS and ICO export generic guard functions from their hash-pinned `sites.rs` (`try_with_room<T>`, `Reserved::<T>::push`, `into_vec`). These fail with `generic-pub-fn` unless a `generic-export` entry declares them; a declared one is audited at every instantiation the roots reach. A downstream instantiation is not audited. Exported trait impls that cannot pass are left out only by `root-exclude` entries with a citation: every `derive(Debug)` `fmt` (it writes through a caller-supplied `dyn fmt::Write`), and `Decoded::clone` in DIB, TGA and ICNS (it clones the pixel `Vec` with an infallible allocation). **These exclusions are owner decisions.** The frozen REVIEW-SOL.md "Third revision" names the roots as the `try_*` and `*_logged` exports only; the tool makes each exclusion explicit and reviewable instead of implicit.
2. **Inlining is undone for attribution.** Optimised MIR has callees inlined. The tool rebuilds the call chain from rustc's inlined-scope records, so the caller judged on a path is the function whose source contains the call, not the function it was inlined into. §6 does not mention this. Without it, a local helper inlined into an admitted guard function would be admitted (control `x14_inlined_helper`).
3. **`allow = "*"`** admits deallocation sinks (`__rust_dealloc`) only, never an allocating one (control `x23_star_alloc`).
4. **The sink set** is exact canonical paths in the sysroot `alloc` crate (`__rust_alloc`, `__rust_alloc_zeroed`, `__rust_realloc`, `__rust_dealloc`, `handle_alloc_error`, `__rust_alloc_error_handler`, `raw_vec::handle_error`, `raw_vec::capacity_overflow`), plus any allocator-shaped foreign symbol. `handle_error` and `capacity_overflow` are judged as sinks rather than admitted as no-MIR leaves, so the guard's `resize`/`push` growth paths that end in them must be admitted by the `alloc` entries whose contracts cite A-STD-1/2/4.
5. **Allowlist value resolution** happens against the DefIds of the walked graph and the local crate, after the walk rather than before it. An external namesake that is never reached cannot be resolved, but it also cannot be matched. A value that resolves to no item is a finding (`allowlist-unresolved`), and so is an entry that admits nothing (`allowlist-unused-entry`). §8 lists the latter as post-freeze hardening; the tool enforces it now.
6. **`#[inline(never)]`** (§3, §8 prerequisite 4) is checked only for `alloc_guard` functions that reach an allocator sink. `OutOfMemory::site`/`elements` in the pinned `mod.rs` do not carry it and do not allocate. ICNS's and ICO's `Reserved::<T>::push` do not carry it and do allocate; the tool reports `guard-inline-never` for them. Neither file was edited (D6 hash pins).
7. **The vstd binding** uses the crate hash (SVH) of the linked rlib and checks the source tree that rlib was compiled from byte for byte against the registry `.crate` archive whose SHA-256 is the `Cargo.lock` checksum. Registry sources carry no `.cargo-checksum.json` (that file exists only for vendored sources, which the tool also accepts). The Verus macro crates are bound by name and by the SHA-256 of the loaded `.so`.
8. **Dependency contract.** ICO's `dependency` entry requires DIB's own report from the same run to say `verdict: PASS` with the same crate hash, target and profile. While DIB's allowlist is unsigned, ICO therefore also fails `dependency-contract`.
9. **The 42 prototype controls** are ported as `no_std` crates with the same items, so that §9's crate-origin rule does not fail them for an unrelated reason. Their fixtures are rewritten in the new allowlist format (`alloc-audit/2`, TOML parsed by the `toml` crate, unknown keys rejected). The controls run with every exported function as a root rather than the prototype's single named root, so some negative controls report more paths than the prototype archived. Every expected verdict is unchanged.

The build-input inventory (§6, §8 prerequisite 2) is implemented by detection plus blocking, which §6 allows as the alternative to denial. It does not deny reads. It attributes environment reads made inside a proc macro to the rustc process that hosts it. These limits are in the README's prerequisite table.

## E8 — 2026-11-13: SPEC §6's simple-path argument does not cover caller authorisation; the audit's scope reductions stated plainly

**Correction to `SPEC.md` §6, *What the audit cannot see*, "Path-enumeration bound".** The frozen text says that restricting to simple paths loses nothing, because a node reached through a cycle is also reached by the simple path that first enters it. That holds for **reachability** and fails for **caller authorisation**. A path is judged by the most recent crate-local caller before its last allowlisted allocation-capable item. A cycle can carry a different caller into a shared instance: a private generic `Clone` impl that calls `to_vec()` again, entered from a guard admitted for `to_vec`. The simple-path enumeration refuses to revisit the shared instance, so it never judges the walk on which the callback is the caller. Sol's review reproduced this as a PASS with zero findings (`alloc-delta/audit/controls/sol-repro/cycle-admitted.txt`). The production tool no longer enumerates paths. It computes a fixpoint over authorisation states: node, most recent crate-local caller, and the verdict of the last allowlisted item. Every walk is judged, including walks that repeat a node; the control is `x27_callback_cycle`. The 20,000-path and depth-80 bounds no longer exist; a 5,000,000-state cap fails closed (`state-bound`). The same review found that caller-side generic arguments in an `allow` value were dropped, so `allow = "crate::guard<u8>"` also admitted `guard::<u16>`. They are now compared exactly (control `x28_generic_caller_args`). Neither fix has been re-reviewed.

**Scope reductions (not proofs).** The draft allowlists make three choices that narrow what a passing audit would cover. A reader must not read past them:

- **`root-exclude` of `Decoded::clone`** (DIB, TGA, ICNS). Cloning a returned image allocates with an infallible `Vec::clone` and is **not covered**.
- **`root-exclude` of every `derive(Debug)` `fmt`.** Formatting a core type goes through the caller's `dyn fmt::Write`, and whatever that writer allocates is **not covered**.
- **`generic-export` of `try_with_room<T>`, `Reserved::<T>::push` and `Reserved::<T>::into_vec`** (ICNS, ICO). Only the instantiations the cores' own roots reach (`T = Entry`) are audited. The behaviour of these public guard functions at any other `T`, or when a downstream crate calls them, is **not covered**. In particular, `Reserved::push` relies on a Verus precondition (`len < room`), not a runtime capacity check. An unverified downstream caller that violates the precondition would reach `Vec::push`'s growth path, which allocates infallibly.

The assurance a signed, passing audit would give is therefore limited to the audited, verified root call paths: the `try_*` and `*_logged` entry points and the other exported non-generic functions not excluded. It does not cover every allocation reachable through the public API, cloning a returned image, the public guard API's behaviour, or arbitrary downstream monomorphisations. Each exclusion and declaration needs the owner's explicit acceptance before any allowlist is signed.

## E9 — 2026-11-13: an admitted item no longer lends its authorisation to crate code it calls back into; corrects E8 and the audit README

E8 says authorisation is decided by "the last allowlisted allocation-capable item" on a walk, judged against the crate caller before it. Sol's re-check showed that rule is too weak whenever an admitted std item calls back into crate code. A guard admitted for `to_vec` reaches `Vec::clone`, which calls a private crate `Clone` impl. That impl calls `alloc::alloc::alloc` directly. The verdict "admitted" from `to_vec` stayed in force across the callback, so the direct allocation inherited it, and the tool returned PRODUCTION, PASS and zero findings (`alloc-delta/audit/controls/sol-repro/recheck/callback-admitted.txt`).

Correction: entering any crate-local function now starts a new authorisation scope. The tool discards the inherited verdict, and an allocation that originates in that function needs its own admission: an `alloc` entry naming an item it calls, with that function as `allow`. Deallocation stays admitted only through explicit `allow = "*"` entries on deallocation items. Pinned std that invokes crate code under a callback-capable admission (`to_vec`, `clone`, iterator adaptors) therefore authorises nothing in that crate code.

- `x33_callback_direct_alloc` must FAIL, naming the callback.
- `x33b_callback_no_alloc` has the same shape with a non-allocating callback and must PASS.

Leaf and panic entries were checked and did not have this problem: each is matched against the immediate (inlining-undone) caller of the leaf or panic edge, not against an inherited state. `x34_leafpanic_caller_args` is a regression control for their caller-argument restriction. It instantiates a private generic at `u8` and `u16`, with intrinsic and bounds-panic admissions restricted to `u8`, and it must leave exactly the two `u16` edges unexplained.

The controls runner now also fails a run that selects no control (`x35_empty_selection_fails`). It keeps the separate exact full-suite count (98).

None of this has been re-reviewed.

## E10 — 2026-11-14: owner decisions D3, D4 and D8 are applied to ICNS's and ICO's `sites.rs`

The owner's three decisions in `alloc-delta/audit/SIGNOFF.md` are now source changes, made in the two hash-pinned per-core `sites.rs` files (`icns/icns_verus/src/alloc_guard/sites.rs`, `ico/ico_verus/src/alloc_guard/sites.rs`). Both edits are the same in the two files:

- **D8: `#[inline(never)]` added.** `Reserved::<T>::push` and `Reserved::<T>::into_vec` now carry `#[inline(never)]`, placed on the line immediately above the `pub(crate) fn` and above its `requires`/`ensures` clauses, the same way `try_zeroed`, `try_copy`, `try_with_room` and `import_dib` already carry it. With it, every allocation-capable `alloc_guard` function satisfies SPEC §3's requirement, so the tool's `guard-inline-never` finding is gone.
- **D3/D4: the heffalump trap is closed.** `Reserved<T>`, `Reserved::push`, `Reserved::into_vec` and `try_with_room<T>` are now `pub(crate)` instead of `pub`. That is the union of the two decisions as the owner resolved them: D3's narrower "declare them as generic exports" is refused in favour of making them non-exported, and D4's reliance on a caller honouring `Reserved::push`'s `requires len < room` is no longer reachable from another crate. The struct and its fields are otherwise unchanged; `AllocSite`, `TryError`, `fmt_part*`, `MAX_ROOM`, `log_tied*`, `name_spec`, the `global layout` declaration and the `alloc_guard` module itself stay `pub` (`lib.rs` re-exports `TryError`/`AllocSite`). An unverified downstream crate can no longer call `Reserved::push` and violate its precondition to trigger an infallible `Vec` growth.

New `sites.rs` hashes: ICNS `5dff9ec076a778b04b782b3dc2e8a290b92c333cee0b281ebdb4619ac037892b` (was `adcc09be…`), ICO `5918505c88282574c693481a557f3b0abd0c1022d4a3e11a2137891efd59bdcf` (was `5c7b24a0…`). `alloc_guard/mod.rs` stays `26e4c832…` (D6). The pins `EXPECTED_SITES_SHA` in `icns/tools/audit_alloc.sh` and `ico/tools/audit_alloc.sh` were updated to the new values, and the three `generic-export` entries were removed from `alloc-delta/audit/allowlist/decisions/{icns,ico}.toml` (the four items are no longer exported, so the tool no longer reports `generic-pub-fn` and the walk audits their `Entry` instantiations as ordinary crate-internal callees). ICNS/ICO allowlist totals fall from 53 to 50 and 45 to 42; the `decisions` files fall from 14 to 11 and 15 to 12.

Verification is unchanged in count and outcome: ICNS run 52 **412 verified, 0 errors** (run 51: 412), ICO run 176 (incremental) and run 177 (clean reverify of the crate and its DIB dependency) **346 verified, 0 errors** (run 175: 346). `#[inline(never)]` and a visibility reduction change no `requires`/`ensures`, so no obligation is added or removed. Cargo tests pass in both crates (`icns_verus`: 111/111 conformance vectors, 12 fault-injection tests; `ico_verus`: 217/217 vectors, the serial fault-injection test).

The D4 wording defect is fixed in both decisions files: `Reserved::push`'s `alloc` citation now says "documented invariant, not a Verus invariant: vstd models no capacity; callers are crate-internal and must prove len < room" instead of "verified invariant". The residual trust is unchanged: for crate-internal callers Verus checks `len < room`; `room <= capacity` remains an A-STD-1/A-STD-2 doc comment, not a Verus invariant, and is not in any contract.
