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

## E11 — 2026-11-14: the four cores' public API is reduced to what a library user needs

The owner asked that the public surface of all four decoder cores be only what a
library user needs. This is a visibility-only change: no `requires`, no
`ensures`, no proof text, no guard text and no `alloc_guard/mod.rs`/`sites.rs`
bytes changed. The `alloc_guard` hashes stay `mod.rs 26e4c832…`, DIB `sites.rs
33465c74…`, TGA `58e1fb28…`, ICNS `5dff9ec0…`, ICO `5918505c…` (the last two as
E10 left them).

**What was hidden, and how.** Two levers, because Verus refuses a `pub` item
whose contract names a non-`pub` item ("cannot refer to private function") and
refuses `pub(crate) open spec fn` at a public path:

- *Module visibility.* `model`, `model_vectors` and (TGA) `policy_model` and
  (ICO) `witnesses` became `pub(crate)`. This hides 1133 (DIB), 909 (TGA), 480
  (ICNS) and 491 (ICO) `pub` items at once. DIB's `model` is the exception: it
  stays a `pub` module but `#[doc(hidden)]`, because `ico_verus`'s allocation
  proof names `dib_verus::model::{mp, max_dimension, max_output,
  lemma_mp_ok_shape}` in spec position and Verus requires those to be `pub`.
  `alloc_guard`, `policy` (TGA) and the implementation module each stay `pub`.
- *Item demotion + `#[doc(hidden)]`.* Inside each implementation module the
  `pub` items no `pub` contract names were demoted to `pub(crate)`, and the rest
  (named by a `try_*` contract, or a `required_roots` entry, or the R15 `name`
  spelling) were marked `#[doc(hidden)]`. Counts (implementation module `pub`
  items, before → after, of which `#[doc(hidden)]`):
  - DIB `dib.rs` 91 → 62, 28 hidden.
  - TGA `tga.rs` 50 → 22, 17 hidden.
  - ICNS `icns.rs` 80 → 73, 61 hidden.
  - ICO `ico.rs` 40 → 21, 11 hidden.

**Public API after the change** (plain `pub`, documented): the `try_*`/`try_parse`
entry points (`try_decode`, `try_decode_with_policy`, `try_decode_with_steps`,
`try_decode_with_steps_logged`; `try_parse`, `try_decode_rgb`,
`try_decode_mask`, `try_combine`; `try_parse`, `try_decode_entry`), the result
types (`Decoded`, `Entry`, `Header`), `Error`, `TryError`, `AllocSite`,
`OutOfMemory`, the policy/profile types and constructors (`Mode`, `BmpMode`,
`BmpProfile`, `Policy` and its option enums; `Profile` and `policy::*_policy`),
`MAX_INPUT`/`MAX_OUTPUT`/`MAX_FILE_LEN`, and the `name()` methods.

**Verifier counts unchanged** (visibility changes no obligation): DIB 1764, TGA
974, ICNS 412, ICO 346 (with DIB 1764 in the same run), all 0 errors.
`cargo test --release -j1` passes in all four crates (DIB 302/302 vectors, TGA
169/169 + 9/9 fault injection, ICNS 111/111 + 12/12, ICO 217/217).

**Consumers.** `gdkpixbuf-loaders`, `bench` and `leniency/policy-probe` build
(exit 0, as before); `tools/diff-harness` still fails on the missing offline
`creusot-std` dependency (pre-existing). The glycin fork's `glycin-dib` and
`glycin-icns` loaders `cargo check --offline` clean; `glycin-tga` reads
`tga_verus::tga::MAX_INPUT`, which therefore stays plain `pub` and not
`#[doc(hidden)]`.

**Source hashes** change for all four cores (the source files changed; the guard
files did not): DIB `c0a03456…` → `10f5071a08964bb735ee266801c53d5c82926a36701cf4e01bbe0218b61e684c`,
TGA `f9ca2824…` → `387f37cbdee25ce4b6149ca10db8bc082918a6cfbda945cbdd9968916c7c0c34`,
ICNS `01f940ad…` → `bfcd14ae571cc5e1ecfd3e67a38b996b22cef339a59245bf91c03282de743d6f`,
ICO `5265d477…` → `5e2660a5d6a7af5205ca07fa64cf63504173f66e6bdab839fca5027b9816a530`
(the pre-change values are exactly what the shipped allowlists record, so the
computation reproduces them). The four allowlists were regenerated
(`tools/draft_all.sh`), `PINS.sha256` rewritten, and the production audit re-run
in `runs/20261114T081621Z` (since superseded and deleted by E12; the E11/E12
run `runs/20261114T084716Z` was itself later superseded and deleted by E13, whose
run `runs/20261114T094507Z` was in turn deleted by E14; E14's evidence is
`runs/20261114T111200Z`): DIB, TGA and ICNS report only `allowlist-reviewer`;
ICO reports `allowlist-reviewer` and `dependency-contract` (D9), with DIB's
report inside the ICO run also only `allowlist-reviewer`. No `missing-root` or
`mono-item-unwalked` appeared. Entry counts are unchanged (64/58/50/42) and the
`required_roots` are unchanged. See `alloc-delta/audit/SIGNOFF.md`.

The survey of every `pub` item, its external users and the per-item decision is
`alloc-delta/audit/API-SURVEY.md`.

**Correction (E12, 2026-11-14).** E11 and its survey said every item left
`#[doc(hidden)] pub` is required by a `pub` contract or an audit root. That is
not true of ICNS's `try_rle_logged`, which E12 demotes to `pub(crate)`, nor of
the three ICNS decode `*_logged` functions, which have a Verus-only
`requires parsed_inv`. The survey's DIB hidden-item list also wrongly named
`at_site_log`; E11 itself had demoted that one to `pub(crate)` in `dib.rs`, so it
was never hidden-public and the E11 list should not have included it. The survey
is corrected in place and the surfaces that remain public are recorded in its
section 6. The ICNS line "80 → 73, 61 hidden" above is the state at E11; E12
removes one more hidden-public item (`try_rle_logged`), making it 60. The other
three modules' hidden-public counts are unchanged (DIB 28, TGA 17, ICO 11). No
other E11 count changes.

## E12 — 2026-11-14: ICNS `try_rle_logged` demoted to `pub(crate)`; the E11 survey corrected

Sol's review of E11 found one real hole: an item could be left `pub` with
`#[doc(hidden)]` whose only size or shape restriction was a Verus precondition,
so ordinary Rust from another crate could violate that restriction and reach an
unchecked allocation. This entry closes the one such item E11 left, records the
E11 survey errors, and states the surfaces that remain. (E11 itself had already
demoted DIB's `at_site_log` to `pub(crate)`; the survey's DIB list was the only
place it was still recorded as public.)

**`try_rle_logged` is now `pub(crate)`.** In `icns/icns_verus/src/icns.rs`
(line 1822) the function's size restriction `1 <= n <= 128*128` was only a
`requires` clause, erased in the artifact. `icns_verus::icns::try_rle_logged(&[],
67_108_864)` with a path to the crate would reserve three 64 MiB channel
buffers. The function has no external caller in the repository (its only caller
is the private `t9_rle_exact` witness), is not a `required_roots` entry (ICNS's
are `try_parse`, `try_decode_rgb`, `try_decode_mask`, `try_combine`), and names
no `pub` item nor is named by one, so Verus does not require it to be `pub`.
`#[doc(hidden)]` was removed and the visibility set to `pub(crate)`; no
`requires`, `ensures` or proof text changed. Verus accepted the demotion (no
"cannot refer to private function" error), so the executable-check fallback was
not needed and no contract changed.

**Scan of the other hidden public functions.** Every `#[doc(hidden)] pub` exec
fn in the four cores' implementation modules was checked for the same shape (its
only size or shape restriction a Verus `requires`, no external user). The
results: TGA's `try_decode_with_steps_logged`, ICO's `try_parse_with_steps` and
`try_decode_entry_with_steps`, and ICNS's `try_parse_logged` have no `requires`
clause at all (their input caps are executable checks in the body), so they do
not match. ICNS's `try_decode_rgb_logged`, `try_decode_mask_logged` and
`try_combine_logged` do carry `requires parsed_inv(bytes@, entries@)` only, and
have no external user, but the task kept them public and recorded them as a
caller-controlled surface (section 6 of `API-SURVEY.md`), so they are not
demoted here. TGA's `try_decode_with_steps_logged` and ICO's two are
`required_roots`, so they must stay exported regardless. No other item required
a demotion.

**E11 survey corrections** (`alloc-delta/audit/API-SURVEY.md`):

- The survey claimed every hidden public function is required by a public
  contract or an audit root. That is false for `try_rle_logged` and for the
  three ICNS decode `*_logged` functions; the survey now says so.
- Its DIB hidden-item list included `at_site_log`, but E11 had made it
  `pub(crate)` in `dib.rs` (line 3806), so it was misrecorded; the list is
  corrected.
- Section 1 said the hidden model modules contribute nothing reachable. The
  documented exception is DIB's `#[doc(hidden)] pub mod model`, whose items ICO's
  spec code names and which therefore remain reachable from ICO; section 1 now
  records it.

**Verification and tests.** ICNS reverifies to **412 verified, 0 errors** (`icns`
run 53; a visibility change adds and removes no obligation). ICNS
`cargo test --release -j1` passes: 111/111 conformance vectors, 12/12 fault
injection. No other core's source changed, so no other verifier run was needed.

**Audit.** The ICNS draft allowlist was regenerated with `tools/draft_all.sh
icns` (its `source_hash` changed to
`cda2ebe74a1a747f9862bfb1e519294c9925515d88816cf01d237df2cea7b269`, was
`bfcd14ae…`; the entry set is unchanged at 50 and differs only in the order of
the `panic` entries). `PINS.sha256` was rewritten. The production
audit was `alloc-delta/audit/runs/20261114T084716Z` (since superseded and deleted
by E13, whose run was `20261114T094507Z` and is itself deleted by E14), which
replaced the
superseded
`runs/20261114T081621Z` (now deleted). It reports exactly the expected findings:
DIB, TGA and ICNS only `allowlist-reviewer`; ICO `allowlist-reviewer` and
`dependency-contract` (D9), with DIB's report inside the ICO run also only
`allowlist-reviewer`. The ICNS report shows the same allocator sinks (5), the
same guard call sites (1 `try_with_room`, 5 `try_zeroed`, 1 `try_copy`), the
same `allocator sinks` verdicts, and all four `required_roots` present. ICNS's
walk is one instance and three edges smaller (`icns_verus` crate instances 53 →
52) because `try_rle_logged` is no longer a root; the three
`try_zeroed` channel call sites in `rle_decode_tracked` are still reached, now
through `decode_rgb_tracked`. DIB, TGA and ICO have the same `source_hash` and
the same findings as in the superseded run (their source did not change; only the
absolute run-directory paths printed inside the reports differ).

**Source hashes** (the E11 values are unchanged; only ICNS changed):
DIB `10f5071a08964bb735ee266801c53d5c82926a36701cf4e01bbe0218b61e684c`,
TGA `387f37cbdee25ce4b6149ca10db8bc082918a6cfbda945cbdd9968916c7c0c34`,
ICNS `cda2ebe74a1a747f9862bfb1e519294c9925515d88816cf01d237df2cea7b269` (was
`bfcd14ae571cc5e1ecfd3e67a38b996b22cef339a59245bf91c03282de743d6f`),
ICO `5e2660a5d6a7af5205ca07fa64cf63504173f66e6bdab839fca5027b9816a530`. Each
matches its regenerated allowlist. The guard hashes are unchanged
(`alloc_guard/mod.rs` `26e4c832…`, ICNS `sites.rs` `5dff9ec0…`).

**Remaining caller-controlled surfaces** are listed, without change, in
`API-SURVEY.md` section 6: `alloc_guard::try_zeroed` in all four cores and
`alloc_guard::sites::try_copy` in ICNS (Verus-only size caps), and the ICNS
`try_decode_rgb`/`try_decode_mask`/`try_combine` family with their `_logged`
variants (`parsed_inv` required only in Verus, forgeable from public `Entry`
fields). They are open design items.

## E13 — 2026-11-14: the `Recommended` -> `RecommendedBeta` rename changes DIB's, TGA's and ICO's `source_hash`

Owner decision 2026-11-14: every `Recommended` policy profile is renamed
`RecommendedBeta` (provisional until real-world feedback). It applies to the
frozen DIB and TGA policy layers; GIF and PNG already use the new name, and the
ICNS and ICO policy layers are deferred. The rename touches the profile name
(Rust selector variant, spec/proof function names, string keys, prose) and no
field value; see `dib/policy/ERRATA.md` E1 and `tga/policy/ERRATA.md` E2 for the
per-layer old/new frozen hashes and the reproducibility evidence.

**Source hashes change for DIB and TGA** (their source files changed), and
therefore for ICO, whose `dib_verus` dependency's `source_hash` also changes.
ICNS's source is untouched, so its `source_hash` is unchanged; the ICNS
allowlist is unchanged.

| Core | `source_hash` before (E11/E12) | `source_hash` after |
|---|---|---|
| DIB | `10f5071a08964bb735ee266801c53d5c82926a36701cf4e01bbe0218b61e684c` | `1176feef4f943a58c0073ba25f27f53c00b40a14d7b8812321ee91adb45bf62d` |
| TGA | `387f37cbdee25ce4b6149ca10db8bc082918a6cfbda945cbdd9968916c7c0c34` | `627a1646805bd453cb6a8cc602b677cc6095cd647b8257c92d0ec121f987ccf7` |
| ICNS | `cda2ebe74a1a747f9862bfb1e519294c9925515d88816cf01d237df2cea7b269` | `cda2ebe74a1a747f9862bfb1e519294c9925515d88816cf01d237df2cea7b269` (unchanged) |
| ICO | `5e2660a5d6a7af5205ca07fa64cf63504173f66e6bdab839fca5027b9816a530` | `5e2660a5d6a7af5205ca07fa64cf63504173f66e6bdab839fca5027b9816a530` (ICO's own source; the DIB source it links changed to the new DIB value) |

The `dib_verus` `source_hash` inside the ICO run is DIB's own, so it also changed
from `10f5071a…` to `1176feef…`.

**Regeneration and re-audit.** `tools/draft_all.sh dib tga ico` regenerated the
DIB and TGA draft allowlists (only `source_hash` changed; ICNS and ICO drafts
were left byte-identical) and rewrote `PINS.sha256`. The production audit was
re-run over all four cores in one run, `runs/20261114T094507Z` (then the
authoritative run; deleted by E14), which replaced `runs/20261114T084716Z` (now
deleted). It reports exactly the expected
findings: DIB, TGA and ICNS only `allowlist-reviewer`; ICO `allowlist-reviewer`
and `dependency-contract` (D9), with DIB's report inside the ICO run also only
`allowlist-reviewer`. Entry counts are unchanged (64 / 58 / 50 / 42) and the
`required_roots` are unchanged. Instance and edge counts are unchanged for every
core (DIB 421/712, TGA 364/622, ICNS 344/629, ICO 320/532); the rename adds and
removes no item in the walk.

**New allowlist pins** (`allowlist/PINS.sha256`): DIB
`e0244373a5941588e9493adf05c11dfe4670efd17ede63310654967488f9f694` (was
`e85e9012…`), TGA
`fa18137315edf07a50abc82133c69837c4decd61a6109ea670c5d8c9f46d9d08` (was
`6faad7c8…`); ICNS
`1d95eaa7638d74e779771267c74dfa8b144ebc18681fd65260e167c9b744de6a` and ICO
`1070a98eb06ebb3af92cf2f6e47f40ef4f2e9496873ff094ae5f36232b7302b1` are unchanged.

**Verification and tests.** DIB reverifies (1764 verified, 0 errors, run 557),
TGA (974, 0, run 232), ICO (346, 0, run 178). `cargo test --release` passes in
`dib_verus`, `tga_verus` and `ico_verus`; the policy conformance tests report
`RecommendedBeta` 102/102 (DIB) and 302/302 (TGA).

## E14 — 2026-11-14: the ICNS public API becomes an opaque `Parsed` handle

Owner decision 2026-11-14: the ICNS core's public decoders must not accept
`(bytes, entries)` as two arguments. The problem is recorded in
`audit/API-SURVEY.md` section 6: `try_decode_rgb`, `try_decode_mask` and
`try_combine` require `crate::model::parsed_inv(bytes@, entries@)` only in
Verus, `Entry`'s fields are public, and the check is erased in the compiled
artifact, so ordinary safe Rust could build a `Vec<Entry>` with out-of-range
payloads and pass it to a decoder, which then indexes and slices on it
(`&entries[..]`, `payload_range`) with no executable check.

**The change.** `pub struct Parsed<'a>` now holds the input `bytes: &'a [u8]`
and the parsed `entries: Vec<Entry>` in **private** fields, with a Verus type
invariant `inv` stating `crate::model::parsed_inv(self.bytes@, self.entries@)`.
`try_parse` is the only constructor; it builds the handle in its `Ok` arm from
the entries `try_parse_logged` returned, discharging `parsed_inv` from
`lemma_parse_ok_gives_p` + `lemma_p_exec_inv`. The decoders are now inherent
methods on the handle:

- `try_parse(bytes: &'a [u8]) -> Result<Parsed<'a>, TryError>` (unchanged
  contract shape: the `Ok` value's entries are `parses_to`'s model list for
  `bytes`, the allocation log/`PreAlloc`/`G`/`Prog` theorem shape is unchanged);
- `Parsed::entries(&self) -> &[Entry]`, a shared view so a caller can choose an
  index but cannot mutate the list a decode will use;
- `Parsed::bytes(&self) -> &'a [u8]`, the slice its entry ranges index (needed by
  PNG-candidate classification);
- `Parsed::try_decode_rgb(&self, color_index) -> Result<Decoded, TryError>`;
- `Parsed::try_decode_mask(&self, mask_index) -> Result<Decoded, TryError>`;
- `Parsed::try_combine(&self, color_index: Option<u32>, mask_index: Option<u32>)
  -> Result<Decoded, TryError>`.

Each method has **no** `requires` clause: `use_type_invariant(self)` supplies
`parsed_inv(self.bytes@, self.entries@)`, which is exactly the internal
function's precondition. The `ensures` restates the internal postcondition
(model equality of the non-`OutOfMemory` result, the output bounds, the
allocation-log relation, and the named `OutOfMemory` request) in terms of
`self.spec_bytes()`/`self.spec_entries()`. Because `Parsed` has private fields
and no other constructor, safe Rust outside the crate cannot forge an entry list
or pair it with a different slice: provenance is a property of the type.

**Type invariant, not a manual spec fn.** `#[verifier::type_invariant]` worked
on this toolchain (verus `0.2026.11.02.8bf68ee`, V-1.99.1), so no manual
`inv(&self)` `requires`/`ensures` fallback was needed. Two facts about it are
worth recording. First, a `pub fn`'s `requires`/`ensures` may **not** name a
private field ("field expression for an opaque datatype … this field expression
must be well-formed everywhere"); the contract therefore speaks through the
`pub` closed spec accessors `spec_bytes`/`spec_entries`, which are `pub` +
`#[doc(hidden)]` (a `pub` contract naming a private spec fn is a hard error).
Second, the invariant is itself `pub` (again because a `pub` contract could name
it) and `#[doc(hidden)]`; `try_parse`'s `Ok` arm is the one place it is
established, so every other method may assume it.

**What did not change.** `Entry` keeps public fields; `Decoded`, `Error` and
`TryError` are untouched. The internal functions `try_parse_logged`,
`try_decode_rgb`/`_logged`, `try_decode_mask`/`_logged`, `try_combine`/`_logged`
and `parse_tracked`/`decode_rgb_tracked`/`decode_mask_tracked`/`combine_tracked`
keep their `requires parsed_inv` and their full theorem statements, and are now
`pub(crate)`. The allocation-log theorems (`G`/`Prog`/`at_site_log_*`, the
`PreAlloc` corollaries) are unchanged; the `*_logged` functions remain the
theorem-facing internals.

**Accessor the fault suite needed, and why it is not in the core.** The
alloc-delta fault suite checks `capacity * size_of::<Entry>()` against its
allocator trace. A slice cannot report capacity, and the entry list is not handed
out mutably, so a `Parsed::entry_list_capacity` accessor was added and then
**removed**: vstd has no specification for `Vec::capacity`, and this toolchain
refuses as unstable both the `A: Allocator` generic form and the allocator-free
form of an `assume_specification` for it. Rather than add an axiom for a value
no proof obligation names, the suite now takes the entry list's size from the
allocator's own trace (its `live` bytes) and the entry count from
`entries().len()`; the capacity is no longer read from the handle. No
`sites.rs` edit and no re-pin were needed.

**Tests.** `cargo test --release -j1` passes: 111/111 conformance, 12/12 fault
injection. The fault suite's reservation counts are unchanged (87/33/33/16/3/2
per reservation, 87 physical-allocation cases, 174 trace events for 111
vectors), and the frozen per-vector sites, order, sizes and the 128 KiB peak
bound are unchanged. Two mechanisms keep this so: a decode vector's handle is
parsed **outside** the armed window, so the entry list is the window's baseline
(not one of the decoder's reservations, matching SPEC.md §4's `Req`), and the
`arm`/`arm_threshold` helpers carry that baseline across the counter reset so
`live`, `peak` and `live_at_fail` report the decoder alone. `Outcome::Parsed`
needed an extra arm: `Parsed` does not implement `Debug` (its only honest derive
would print the opaque fields), and the former `Outcome::Parsed(Vec<Entry>)`
copy was dropped because copying entries out of the handle inside an armed window
would allocate and consume the injector's countdown.

**Verification.** ICNS reverifies at **418 verified, 0 errors** (run 62; was
412). The new and changed obligations are the handle's `entries`/`bytes` methods,
the three decoder wrappers (each discharges `parsed_inv` from the type
invariant), the rewritten `try_parse` construction site, and the retargeted
`parse_success_invariant` / `parse_with_t8` / `decode_rgb_after_parse` /
`decode_mask_after_parse` / `combine_after_parse` / `t13_combine_after_parse`
witnesses; no new lemma was needed beyond the two model lemmas below. No `rlimit`
was changed (the live ICNS source still has none).

**Model lemmas added.** `model::lemma_t8_view(b, es_actual, es)`: a handle's
entry view satisfies `entries_same_fields(es_actual, es) <==> p_exec(b, es)`,
stated directly on the sequence so a caller holding only `Seq<Entry>` (the view)
needs no `Result` wrapper; it reuses `lemma_exact_entry_equivalence`.
`model::parse_len(b)` and `model::lemma_parses_to_len(b, es)`: a successful
parse's view has the model list's length.

**Witness rewrites.** The T6/T7/T8/T9..T13 call-site witnesses in `icns.rs` and
`model.rs` were retargeted to the handle: `parse_success_invariant` and
`parse_with_t8` return `Result<Parsed, TryError>`, `decode_rgb_after_parse` /
`decode_mask_after_parse` / `combine_after_parse` call the methods through a
handle from `try_parse` (so no precondition is assumed anywhere in the chain),
and `t13_combine_after_parse` returns the `Parsed` handle as its first tuple
element. `t10`/`t11`/`t12` remain stated over the `parsed_inv` parts, which is
the statement the handle's invariant carries.

**Consumers updated.**

- `icns_verus/tests/conformance.rs`, `tests/fault_injection.rs`: adapted to the
  handle (above).
- `bench/src/main.rs`: parses once and calls `parsed.try_combine(..)`.
- `leniency/policy-probe/src/main.rs`: `icns_parsed(b)` + `p.entries()` +
  `p.try_combine(..)`.
- `leniency/shared/src/main.rs`: `icns_verus::try_parse`, `parsed.try_combine`,
  `glycin_icns::candidates(&parsed)`, `glycin_icns::entry_payload(&parsed, i)`
  (this runner was already uncompilable before the change, calling the removed
  frozen `parse`/`combine` names; it now names the new API, unverified).
- the local glycin fork `~/projects/glycin-plan/glycin/glycin-loaders/glycin-icns`
  (loader and tests): `candidates(parsed)` and `entry_payload(parsed, i)`, the
  decoders through the handle; `cargo check --offline -j1 -p glycin-icns` and
  `cargo test -p glycin-icns` pass (loader 23, robustness 4, vectors 1).
- `tools/diff-harness/src/main.rs` **not** fixed: it already calls the removed
  frozen names (`parse`, `decode_rgb`, `decode_mask`, `combine`) and does not
  compile; the task scopes it out. It now also predates the handle.

**Audit.** ICNS's `source_hash` changed to
`0fb984b50938524485db5b0e6a1548d9ff4c97a51513ada88387ec55c22f2d48` (was
`cda2ebe7…`) and its `crate-svh` to `e6dfd4a7253baef22fa39b192944e65e`. The ICNS
draft allowlist was regenerated with `tools/draft_all.sh icns`; the entry set is
unchanged at 50 (the no-MIR and panic-edge entries moved to the new root names)
and `PINS.sha256` was rewritten (ICNS
`d6f4dc578a8fea1495512e50c25e65a72f13921655de56f965436c1545f09e7b`). DIB, TGA
and ICO were left byte-identical; their `source_hash`, instance and edge counts
and findings are unchanged.

**`required_roots` (D10).** The decoders are inherent methods, whose canonical
path names the impl's lifetime parameter, so the ICNS list is now five entries,
spelled exactly as the tool reports them:
`crate::icns::try_parse`,
`crate::icns::Parsed::<'a>::entries`,
`crate::icns::Parsed::<'a>::try_decode_rgb`,
`crate::icns::Parsed::<'a>::try_decode_mask`,
`crate::icns::Parsed::<'a>::try_combine`.
`Parsed::<'a>::bytes` is a sixth exported root (it is `pub` and reachable);
it is allocation-free and stays in the walk, needing no exclusion. No
`_logged` function is public again: no theorem consumer in another crate needs
one for ICNS, and all of them are `pub(crate)`.

**Audit run.** The four-core production run is
`alloc-delta/audit/runs/20261114T111200Z`, which is the authoritative evidence and
replaces `runs/20261114T094507Z` (deleted). Expected findings, and what they
were: DIB and TGA only `allowlist-reviewer`; ICNS only `allowlist-reviewer`
(the former `guard-inline-never` for `Reserved::<T>::push` is gone since E10);
ICO `allowlist-reviewer` and `dependency-contract` (D9, while DIB is unsigned),
with DIB's report inside the ICO run also only `allowlist-reviewer`. Entry
counts are unchanged (64 / 58 / 50 / 42). ICNS's walk is now 351 instances /
638 edges (was 344/629: the new handle roots and their calls); its allocator
sinks (5) and guard call sites (1 `try_with_room`, 5 `try_zeroed`, 1 `try_copy`)
are unchanged, and rustc's collector still codegens no function the walk did not
reach (`mono-items … 0 not reached`).

**ICNS section-6 surface, resolved.** The forged-`entries` item in
`audit/API-SURVEY.md` section 6 is resolved by this change; the survey lists the
new public API and the remaining open item (the `try_zeroed`/`try_copy`
Verus-only size caps) is unchanged.

**Public API now (`crate::icns` and the crate root).** `try_parse`, `Parsed`
with `entries`, `bytes`, `try_decode_rgb`, `try_decode_mask`, `try_combine`;
`Entry` (public fields, for reading), `Decoded`, `Error`, `TryError`,
`MAX_FILE_LEN`, and the `#[doc(hidden)]` `MAX_ENTRIES`, `MAX_LEGACY_SIDE`,
`Entry::type_code`, `Error::name`, `Error::name_spec`, and the spec/proof items a
`pub` contract names. No decoder takes `(bytes, entries)` any more.

## E15 — 2026-11-14: `try_parse`'s rejection clause restored (corrects E14)

Sol's review of E14 (the ICNS opaque `Parsed` handle) found one material defect,
confirmed by the reviewer: the new public `try_parse` contract constrains the
successful handle but gave `Err(TryError::Format(_))` the postcondition `true`.
**E14's claim of an "unchanged contract shape" for `try_parse` (above) is
therefore wrong, and so is its statement that the method `ensures` restates "the
allocation-log relation": the `G`/`Prog`/`PreAlloc`/`at_site_log_*` clauses are
internal-theorem facts about `try_parse_logged`/`parse_tracked`, not method
postconditions.** This entry corrects both.

**What was lost.** Before E14 every non-`OutOfMemory` result of the public
parser satisfied `model::parses_to` on `fmt_part_entries(r)`, including the exact
first format error, and the `PreAlloc` clause carried the same relation. After
E14 the public boundary no longer established, for example, that bad magic
returns `BadMagic`, nor that an accepted input cannot return a format error.
HEAD's `parse_with_t8` public-call witness stated the full relation
(`!OutOfMemory ==> forall es. parse_returned(fmt_part_entries(r), es) <==>
p_exec(bytes, es)`), which covers rejection; the E14 rewrite constrained only
`Ok`, so it did not recover the obligation.

**The fix.** `try_parse`'s `ensures` again has an error-arm clause, in the same
`parses_to_s` form the pre-change contract used and derived from the logged
internal result:

```
Err(TryError::Format(e)) => crate::model::parses_to_s(bytes@, Err(e)),
```

The `Ok` arm (`p.spec_bytes() == bytes@ && parses_to_s(bytes@, Ok(p.spec_entries()))`),
the `pre_alloc_p` clause and the named-`OutOfMemory` clause are unchanged. The
body derives the error clause from `try_parse_logged`'s `g_p_clause1` (which for
a non-`OutOfMemory` result is `parses_to` on `fmt_part_entries`) via
`lemma_parses_to_err_view`, the existing small model lemma. The public-call
witness `parse_with_t8` is restored to cover rejection as well as successful
parses: its `ensures` is now
`match r { Ok(p) => forall es. entries_same_fields(p.spec_entries(), es) <==> p_exec(bytes@, es),
Err(Format(_)) => forall es. !p_exec(bytes@, es), Err(OutOfMemory(_)) => true }`,
discharged from the contract's new error clause plus `lemma_parse_err_gives_not_accepted`
(and `lemma_t8_view` on the `Ok` arm). No `assume`, `admit` or `external_body`
was added, and no new model lemma was needed. Also comment-only: the
`tests/fault_injection.rs` notes that said a decode vector parses *inside* its
injection window were wrong (it parses outside, see `check_vector`/`combine_probe`)
and now say so.

**Verification and tests.** ICNS reverifies at **418 verified, 0 errors** (run
63; the count is unchanged from E14's run 62 because the restored clause and the
widened witness add postconditions and assertions to functions that were already
counted verification units. A flat count does not show that no new logical
obligation was added; Sol's review (2026-11-14) read the clause and the proof path
directly and found the clause non-vacuous and fully discharged). `cargo
test --release -j1` passes: 111/111 conformance, 12/12 fault injection.

**Regeneration and re-audit.** ICNS's `source_hash` changed to
`964a04253c9d01e65bb59ab17256e0a85378155c75e01442ff27b79e242d709c` (was
`0fb984b5…`), so `tools/draft_all.sh icns` regenerated the ICNS draft allowlist
(entry set unchanged at 50; only `source_hash` and the entry ordering changed)
and rewrote `PINS.sha256` (ICNS
`b0a0cc0e306e6dfb0cd6497f3b2d9e83a6fe554c2b38077ee725be1c09593e71`). ICNS's
`crate-svh` is `266e0634352089beb02f512d3bd512e3`. The four-core production run
is `runs/20261114T115040Z`, which is now the authoritative evidence and replaces
E14's `runs/20261114T111200Z` (deleted). It reports the same expected findings
(DIB, TGA and ICNS only `allowlist-reviewer`; ICO `allowlist-reviewer` and
`dependency-contract`, with DIB's report inside the ICO run also only
`allowlist-reviewer`), the same entry counts (64 / 58 / 50 / 42), and the same
instance and edge counts and `source_hash` for DIB, TGA and ICO; ICNS's walk is
unchanged at 351 instances / 638 edges.

**Correction to E14's root list.** E14's SIGNOFF paragraph named
`Parsed::<'a>::entry_list_capacity` as an exported root. No such root exists: the
accessor was removed before E14 landed (E14 itself records why), and neither the
tool's root list nor `required_roots` has an entry for it. `Parsed::<'a>::bytes`
is the real sixth exported root, as E14 states elsewhere.

**Regression probe.** The review's scratch crate is kept as
`icns/api-probe/`, a compile-fail probe of the public surface: it checks that
`Parsed` cannot be constructed literally, cannot be `Default`/`Clone`/`Copy`,
cannot be mutated through `entries()`/`bytes()`, that the `_logged` internals and
the old `(bytes, entries)` decoders are not public, and that `spec_bytes`/
`spec_entries`/`inv` are not callable outside the crate. The expected compiler
error codes are in its `README.md`.
