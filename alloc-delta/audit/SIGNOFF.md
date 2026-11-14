# Signing the four allocation allowlists: owner brief

Status 2026-11-14, branch `dib-policy`. Nothing here is signed. D3, D4 and
D8 are now resolved (D8 was a source change, D3/D4 were closed by the same
change; ERRATA E10). The remaining ICNS and ICO findings are the unsigned
reviewer, plus ICO's `dependency-contract` while DIB's report is unsigned.

**E11 (2026-11-14): the public-API reduction.** The four cores' public
surfaces were reduced to the library-user API (ERRATA E11; survey in
`API-SURVEY.md`). This changed every core's `source_hash`, so the four draft
allowlists were regenerated with `tools/draft_all.sh` and re-audited. The
`required_roots` are unchanged: all fifteen `try_*` roots remain exported
functions (the `*_logged` ones are `#[doc(hidden)] pub`, which the tool still
counts as exported; E12 later demoted ICNS's non-root `try_rle_logged`). Entry
counts are unchanged at 64 / 58 / 50 / 42. The
production run for this change (then `20261114T081621Z`, since deleted by
E12; the evidence then was `runs/20261114T084716Z`, since deleted by E13, whose
authoritative run at the time was `runs/20261114T094507Z`, later deleted by
E14) reports exactly the
expected findings: DIB,
TGA and ICNS only `allowlist-reviewer`; ICO `allowlist-reviewer` and
`dependency-contract` (D9), with DIB's report inside the ICO run also only
`allowlist-reviewer`. No `missing-root`, `mono-item-unwalked` or other finding
appeared, so hiding the proof modules did not orphan any codegen monomorphisation
the roots do not reach.

**E12 (2026-11-14): ICNS `try_rle_logged` demoted to `pub(crate)`.** Sol's
review of E11 found that a `#[doc(hidden)] pub` exec fn whose only size
restriction was a Verus precondition could be called with an out-of-range size
from another crate. ICNS's `try_rle_logged` was the one such item; it is now
`pub(crate)` (ERRATA E12). ICNS reverified (412/0, run 53) and ICNS tests pass
(111/111 conformance, 12/12 fault injection). The ICNS draft allowlist was
regenerated (new `source_hash` `cda2ebe7...`; entry set unchanged at 50) and
`PINS.sha256` rewritten. The authoritative production run was then
`runs/20261114T084716Z` (since deleted by E13, whose authoritative run is
`runs/20261114T094507Z`, itself later deleted by E14);
`runs/20261114T081621Z` is deleted. ICNS's walk is one
instance and three edges smaller; the other three cores are unchanged. See
`API-SURVEY.md` section 6 for the caller-controlled surfaces that remain public
(`alloc_guard::try_zeroed`, ICNS `try_copy`, and the ICNS `parsed_inv`
decoders), which are open design items.

**E13 (2026-11-14): the `Recommended` -> `RecommendedBeta` rename.** The DIB
and TGA policy profiles were renamed `RecommendedBeta` (owner decision; ERRATA
E13, and `dib/policy/ERRATA.md` E1, `tga/policy/ERRATA.md` E2). This changed
DIB's and TGA's `source_hash`, so `tools/draft_all.sh dib tga ico` regenerated
the DIB and TGA draft allowlists (only `source_hash` changed; ICNS and ICO were
byte-identical) and rewrote `PINS.sha256`. The four-core production audit was
re-run in one run, `runs/20261114T094507Z` (then authoritative; later deleted
by E14), which replaced `runs/20261114T084716Z`, which is deleted along with the
earlier `runs/20261114T081621Z`. It reports the same expected
findings (DIB, TGA and ICNS only `allowlist-reviewer`; ICO `allowlist-reviewer`
and `dependency-contract`), the same entry counts (64 / 58 / 50 / 42) and the
same instance and edge counts (DIB 421/712, TGA 364/622, ICNS 344/629, ICO
320/532). New `source_hash`: DIB `1176feef…`, TGA `627a1646…`; ICNS
`cda2ebe7…` and ICO `5e2660a5…` are unchanged.

**E14 (2026-11-14): the ICNS `Parsed` handle.** The ICNS decoders are now
methods on an opaque `Parsed<'a>` handle: private `bytes`/`entries` fields, a
Verus type invariant `parsed_inv(bytes@, entries@)`, and `try_parse` as the only
constructor. The three public decoders no longer take `(bytes, entries)`, so the
forged-entry surface recorded in `API-SURVEY.md` section 6 is gone (ERRATA E14).
This changed ICNS's `source_hash` to `0fb984b5…` and its `crate-svh` to
`e6dfd4a7…` (both changed again by E15: `964a0425…` and `266e0634…`), so
`tools/draft_all.sh icns` regenerated the ICNS draft allowlist
(entry set unchanged at 50; the no-MIR and panic-edge entries moved to the new
root names) and rewrote `PINS.sha256`. The `required_roots` changed: the decoders
are inherent methods, whose canonical path names the impl's lifetime parameter,
so the roots are now `crate::icns::try_parse` plus
`crate::icns::Parsed::<'a>::{entries, try_decode_rgb, try_decode_mask,
try_combine}` (five, not four). `Parsed::<'a>::bytes` is a sixth exported root;
it is allocation-free and stays in the walk. (An earlier revision of this
paragraph also named `Parsed::<'a>::entry_list_capacity`; no such root exists —
the accessor was removed before E14 landed, and the tool's root list has no
entry for it. Corrected by E15.) The four-core production run is
`runs/20261114T115040Z` (E15), which is now the authoritative evidence and
replaces `runs/20261114T111200Z` (deleted; that run replaced the
`runs/20261114T094507Z` E14 deleted). It reports the same expected findings (DIB,
TGA and ICNS only `allowlist-reviewer`; ICO `allowlist-reviewer` and
`dependency-contract`), the same entry counts (64 / 58 / 50 / 42), and the same
instance and edge counts and `source_hash` for DIB, TGA and ICO. ICNS's walk is
351 instances / 638 edges (was 344/629: the five new handle roots and their
calls); its guard call sites are unchanged at 1 `try_with_room`, 5 `try_zeroed`,
1 `try_copy`. ICNS reverified at 418 verified, 0 errors (run 63), and its tests
pass (111/111 conformance, 12/12 fault injection).

**Conclusion.** The mechanical checks I could redo without Verus or
cargo agree with the drafts: all 67 crate functions cited as the reason
a panic edge cannot fire exist, are exec functions inside `verus!`, and
carry no `external_body`, `assume`, `admit` or `assume_specification`.
Every cited A-STD item exists, every `guard-calls` count matches the
source, and every hash matches. I found no mismatch. What remains is
the signature itself and the decisions D1, D2, D5, D6, D7, D9 and D10
below; D3, D4 and D8 are done.

All paths below are relative to `alloc-delta/audit/` unless they start
with a core name.

## 1. What signing means

The tool proves one thing mechanically. In the release x86_64 build, every
call path in the MIR call graph, from every exported function that is not
excluded, to an allocator sink passes through an admitted guard function
(`try_zeroed`, `try_copy`, `try_with_room`, `Reserved::push`). Every edge
it cannot follow is either an allowlist entry or a finding.

Your signature asserts what the tool cannot check:
- the std contracts A-STD-1..4 hold (D5, D6);
- each panic edge listed is unreachable, because the cited function is
  Verus-verified (section 4 checks the "is verified" half only);
- each no-MIR leaf (a std function the compiler has no body for) and
  each intrinsic allocates nothing;
- the scope reductions D1, D2 and D7 are acceptable (D3 and D4 were
  closed by making the guard's element-list items `pub(crate)`; there is
  no generic-export entry left).

The tool refuses an allowlist whose `reviewer` is empty
(`allowlist-reviewer`). That is the only finding left for DIB and TGA,
and for ICNS. ICO also fails `dependency-contract` while DIB's report
does not PASS (D9).

## 2. Owner decisions

Answer by number: accept, reject, or change. "Lost" means what a
rejection costs.

### D1. Exclude `Decoded::clone` from the audited roots (DIB, TGA, ICNS)
- Effect of accepting: cloning a returned image allocates through
  `Vec::clone`, which aborts on out-of-memory. The audit says nothing
  about it. ICO has no entry because the type is DIB's, but ICO's
  public result is a DIB `Decoded`, so the same hole applies there
  (covered only by DIB's entry).
- Lost if rejected: the audit fails (`unauthorized-alloc-path`).
  The only fix is removing `derive(Clone)` from `Decoded` or adding a
  fallible `try_clone`, which changes the public API.
- Recommendation: accept; tell callers not to clone a decoded image
  where out-of-memory must not abort. File a follow-up for `try_clone`.

### D2. Exclude every `derive(Debug)` `fmt` root
- Effect: 30 (DIB), 20 (TGA), 6 (ICNS), 8 (ICO) entries. Printing a
  core type with `{:?}` goes through the caller's `dyn fmt::Write`;
  whatever that writer allocates is not covered.
- Lost if rejected: the audit reports `virtual-dispatch` for each.
  The only fix is removing `Debug` from the public types.
- Recommendation: accept. The allocation belongs to the writer, not to
  the decoder. `Decoded`'s `Debug` prints the whole pixel buffer, so
  warn callers about logging it.

### D3. Declare `try_with_room`, `Reserved::push`, `Reserved::into_vec` as generic exports (ICNS, ICO) — RESOLVED 2026-11-14

- Original effect: these were `pub` and generic, in the hash-pinned
  `sites.rs`. The audit covered them only at `T = Entry`, as the cores'
  own roots instantiate them; a downstream crate calling them with
  another `T` was not audited.
- Resolution: they are no longer `pub`. `Reserved<T>`, `Reserved::push`,
  `Reserved::into_vec` and `try_with_room<T>` are `pub(crate)` in both
  `sites.rs` files (ERRATA E10; `EXPECTED_SITES_SHA` re-pinned, the three
  `generic-export` entries removed). A downstream crate cannot reach
  them, so there is nothing left to declare: the tool no longer reports
  `generic-pub-fn` and no `generic-export` entry remains. The walk still
  audits their `Entry` instantiations as ordinary crate-internal callees.
- Residual trust: none beyond D4. `alloc_guard` stays a `pub` module
  (its `AllocSite`/`TryError` are public API), but the four items are
  crate-private.

### D4. Admit `Vec::push` at `Reserved::push` on a Verus precondition only (ICNS, ICO) — RESOLVED 2026-11-14

- Original effect: the entry says `Vec::push` never grows. Verus checks
  only `requires len < room` on `Reserved::push`. That `room <= capacity`
  is a doc comment ("Invariant A-STD-1"), not a Verus invariant: vstd
  models no capacity. The claim rests on `try_with_room` (A-STD-1) plus
  A-STD-2. A verified caller cannot violate `len < room`; an unverified
  downstream caller could, and `Vec::push` then grows with an infallible
  allocation (abort).
- Resolution: the unverified-caller hole is closed by D3's visibility
  change. `Reserved::push` is `pub(crate)`, so its only callers are
  crate-internal and Verus checks `len < room` at each. The wording
  defect is fixed: the `alloc` entry's citation now reads "documented
  invariant, not a Verus invariant: vstd models no capacity; callers are
  crate-internal and must prove len < room".
- Residual trust: `room <= capacity` is still only an A-STD-1/A-STD-2
  doc comment, not a Verus invariant and not in any contract. It is what
  makes the admitted `Vec::push` (and `Vec::set`, if reached) allocation-
  free; it is part of the A-STD signature (D5).

### D5. Trust A-STD-1, A-STD-2, A-STD-3 (documented std behaviour)
- Effect: A-STD-1 (`try_reserve_exact` `Ok` means capacity >= n),
  A-STD-2 (`push` does not allocate while `len < capacity`) and
  A-STD-3 (an `Err` means the allocator returned null; SPEC says no
  theorem depends on it). Verus cannot prove them; runtime
  counting-allocator tests are a regression check, not a proof.
- Lost if rejected: no alloc entry can be admitted; the audit cannot
  pass.
- Recommendation: accept; they are stated in std's own docs.
- Wording note: the `try_reserve_exact` contracts also say "never
  aborts". No A-STD item says that literally. It follows from the
  method's documented `Result` return, which is what A-STD-3 cites.

### D6. Trust A-STD-4 (observed, not documented)
- Effect: `Vec<u8>::resize(n, 0)` with `len == 0` and `capacity >= n`
  does not reallocate. Std's docs say bulk insertion "may reallocate,
  even when not necessary". The claim rests on the pinned std source
  (rustc 1.99.1, `58cf8f9c3`), a counting-allocator test and fault
  injection (ERRATA E6; logs in `astd4-logs/`). A reallocation would
  abort and cannot be detected afterwards.
- Lost if rejected: return to the push loop. E6 measured 47.8 ms
  against 6.6 ms for a 64 MiB output, a 41 ms regression per
  maximum-size decode.
- Recommendation: accept, since you approved the clause on
  2026-11-13, and re-review on any toolchain change. The rustc commit
  is pinned in `[meta]`, so a toolchain change fails the audit.

### D7. Admit deallocation anywhere (`allow = "*"` on `RawVecInner::deallocate`)
- Effect: dropping any `Vec` is admitted from any caller. The tool
  honours `*` for deallocation sinks only (ERRATA E7.3, control `x23`).
  Freeing cannot fail or allocate.
- Lost if rejected: one entry per dropping function.
- Recommendation: accept.

### D8. `guard-inline-never` finding (ICNS, ICO) — DONE 2026-11-14
- Fact: `Reserved::<T>::push` (and `into_vec`) in `sites.rs` lacked
  `#[inline(never)]`; `try_zeroed`, `try_copy`, `try_with_room` and
  `import_dib` had it. SPEC §3 requires it on every allocation-capable
  guard function. The tool reported `guard-inline-never` and no
  allowlist entry can waive it.
- Done: `#[inline(never)]` is now on `Reserved::<T>::push` and
  `Reserved::<T>::into_vec` in both files, placed as on the other guard
  functions (the line above the `fn`, before its `requires`/`ensures`).
  Consequences were done in order: Verus re-run (ICNS 412/0 run 52,
  ICO 346/0 runs 176 and 177; the count is unchanged), `EXPECTED_SITES_SHA`
  updated in `icns/tools/audit_alloc.sh` and `ico/tools/audit_alloc.sh`,
  and the two allowlists regenerated with `tools/draft_all.sh icns ico`
  (new `source_hash`, new pins). The `guard-inline-never` finding is gone;
  see ERRATA E10.

### D9. ICO relies on DIB's report (`crate`, `dependency`, two `no-mir` entries)
- Effect: ICO's walk stops at DIB's functions. ICO admits DIB's
  allocation on the contract "DIB's own report for the same crate
  hash passes". The tool enforces that contract (`dependency-contract`),
  so ICO fails whenever DIB does. The `no-mir` entry for
  `try_decode_with_steps` is admitted from `crate::skeleton`, a
  `pub` skeleton function in `ico_verus/src/lib.rs`.
- Recommendation: accept; sign DIB before ICO.

### D10. Roots that must exist (`required_roots`)
- Each core lists its public entry points, which must be among the
  roots (`missing-root` otherwise): DIB 4, TGA 3, ICNS 5, ICO 4. Read
  the list in `allowlist/decisions/<core>.toml`; confirm it is every
  entry point you consider public.
- ICNS's five (ERRATA E14) are `try_parse` and the four handle methods
  `Parsed::<'a>::{entries, try_decode_rgb, try_decode_mask, try_combine}`.
  The decoders moved from free functions to inherent methods when the
  opaque handle replaced `(bytes, entries)`, so their canonical paths
  carry the impl's lifetime parameter; `entries` is a root because it is
  the only way a caller reads an entry to choose an index.
- E11 check (2026-11-14): all fifteen roots are still exported functions
  after the public-API reduction. The DIB/TGA/ICNS roots are plain `pub`;
  ICO's `try_parse_with_steps` and `try_decode_entry_with_steps`, and
  TGA's and ICNS's `*_logged` roots, are `#[doc(hidden)] pub`, which the
  tool still counts as exported. No `missing-root` was reported at E11/E12
  (ERRATA E11/E12) or in E14's `20261114T111200Z` (now deleted by E15, whose
  authoritative run is `20261114T115040Z`). (E12 later demoted
  ICNS's `try_rle_logged` to `pub(crate)`; it is not a `required_root`.)
- Recommendation: accept after checking the lists.

### Other E7 deviations
E7's remaining items (inlining attribution, sink set, value resolution,
vstd binding, inventory) tighten the tool and need no owner decision.

## 3. Mechanical entries, by kind

Counts are DIB / TGA / ICNS / ICO. Total entries: 64 / 58 / 50 / 42.
(ICNS and ICO fell by 3 each when the `generic-export` entries were
removed, ERRATA E10.) Hand-written kinds (alloc, guard-calls, crate,
dependency, the clone exclusions) are covered in section 2 and listed
here only for the count.

**alloc** 3 / 3 / 7 / 5 (hand-written; D4-D7). Each names a std item,
the one guard function allowed to reach it, a contract and an A-STD
citation. Spot check: open `allowlist/decisions/<core>.toml`, read each
contract, and confirm the `allow` function in the source calls that std
method on that element type.

**root-exclude** 31 / 21 / 7 / 8. Of these, 1 / 1 / 1 / 0 are
`Decoded::clone` (D1) and 30 / 20 / 6 / 8 are `derive(Debug)` (D2).
Spot check: every non-clone item must end in `as core::fmt::Debug>::fmt`.
`grep -c 'Debug>::fmt' <file>` should give 30 / 20 / 6 / 8.

**generic-export** 0 / 0 / 0 / 0 (D3/D4, resolved 2026-11-14). The three
items (`try_with_room`, `Reserved::<T>::push`, `into_vec`) are no longer
exported, so no `generic-export` entry remains in any core. Spot check:
`grep -c '^kind = "generic-export"$'` over `allowlist/decisions/{icns,ico}.toml` and
the generated `*_verus.toml` gives zero. (A plain `grep generic-export` also
matches comments and counts wrongly.)

**guard-calls** 1 / 1 / 3 / 3. Each states how many times a guard function
is called in the source; the tool compares the count with the compiled
call graph. Example: ICNS `try_zeroed` sites = 5. Spot check: section 4.

**crate / dependency** 0 / 0 / 0 / 1 each (D9).

**proc-macro** 2 each: `verus_builtin_macros` and
`verus_state_machines_macros`, bound by SHA-256 of the `.so` the build
loaded. Asserts that the macro crates are the pinned Verus release.
Spot check: `sha256sum` the two `.so` files named in each `reason` and
compare with `sha256 =`.

**ffi** 1 each: `__rust_no_alloc_shim_is_unstable_v2`, a volatile read
of a marker symbol, no allocation. Spot check: it is the same entry in
all four files.

**intrinsic** 6 / 4 / 5 / 4. Compiler intrinsics from a fixed list:
`compare_bytes`, `cold_path`, `ctpop`, `write_bytes` in all cores;
DIB adds `cttz`, `ctlz`; ICNS and ICO add `raw_eq`. Each asserts the
intrinsic allocates nothing (memcmp, memset, bit counts). Spot check:
confirm no entry outside this list.

**no-mir** (diverging std functions) 12 / 12 / 13 / 13. All name a
panic function the compiler has no body for, one entry per calling
function, admitted only because the call is unreachable. Four
subgroups:
- `ub_checks` precondition checks, 10 each. Asserts the branch is dead
  because `debug-assertions=off`. This rests on the profile in
  `[meta]`, not on Verus. Example: `panic_nounwind_fmt` called from
  `unchecked_add::precondition_check`.
- `is_aligned_to`, 1 each, same argument.
- crate indexing or `unwrap`: 1 / 1 / 2 / 0 (`unwrap_failed` from
  `Option::unwrap` in DIB's `decode_raw_zero`; `slice_index_fail` from
  `Range` indexing in TGA, `Range` and `RangeFrom` in ICNS). Asserts
  Verus has proved the index or `Some`.
- DIB dependency contract entries, 0 / 0 / 0 / 2 (D9).

**panic** 8 / 14 / 12 / 4. Compiler-inserted `Assert` edges, one per
(function, kind): `PanicBoundsCheck` 6 / 12 / 12 / 4, `PanicDivZero`
2 / 1 / 0 / 0, `PanicRemZero` 0 / 1 / 0 / 0. Callers that are crate
functions: 6 / 13 / 10 / 4. Callers that are std index wrappers
(`<usize as SliceIndex>::index`): 2 / 1 / 2 / 0. Each asserts the check
cannot fire because the cited function is Verus-verified (T1 restated
as `Safe'`, `THEOREMS-ALLOC-DELTA.md` §1-4). Spot check: pick two
entries, open the function in the source, confirm the indexing there is
guarded (a `requires` or loop invariant bounds the index). Example:
`PanicBoundsCheck` in `crate::dib::rd16`, `dib.rs` line 813.

## 4. Verification of the generated citations

Method: a script in the scratchpad (not in the repo) parsed the four
allowlists with `tomllib`, collected every `crate::` path from the
`citation` and `allow` of every `panic` and `no-mir` entry, located each
function in the core's `src/`, and tested that it (a) exists, (b) is
`exec` (not spec or proof), (c) lies between `verus! {` and
`} // verus!`, (d) has no `external*`, `trusted`, `assume` or `admit`
attribute. The greps below cover all of `src/*.rs` and
`src/alloc_guard/*.rs` of each core. The script reads syntax; it does
not run Verus.

Cited functions found, per core: DIB 19 of 19, TGA 24 of 24, ICNS
15 of 15, ICO 9 of 9. Total 67 of 67 found, all exec, all inside
`verus!`. Of the 67, 9 are guard functions (`try_zeroed` in all four, plus
`try_copy`, `try_with_room` and `Reserved::push` where present) and 1
is `crate::skeleton` (`ico_verus/src/lib.rs` line 32).

Problems among the cited functions: none. `external_body`,
`assume_specification`, `admit`, `assume(` and `#[verifier::external]`
occur zero times in any of the four cores' source files. The only
`verifier::` attributes present are `opaque` (DIB, 3), `rlimit` (DIB, 2)
and `when_used_as_spec` (all). DIB and TGA also have a `policy_tests.rs`, outside `verus!`; it is
`cfg(test)`, and the tool refuses `cfg(test)` builds.

Two limits on that result:
- It shows the functions are eligible for verification. That they
  verify is the job of the proof reports (`dib/PROOF-REPORT-*.md`
  etc.), not of this check.
- Verus's std calls rest on vstd `assume_specification` entries
  (`Vec::push`, slice indexing, `Option::unwrap` with
  `requires option is Some`). vstd is bound by crate hash and by its
  locked source checksum, so it is pinned, but it is trusted. A panic
  entry is only as good as vstd's preconditions.

Other checks:
- **A-STD citations.** Every `alloc` entry cites only A-STD-1..4. All
  four exist: 1-3 in SPEC §3, 4 in ERRATA E6. The three `deallocate`
  entries cite none and need none. No missing item.
- **`THEOREMS-ALLOC-DELTA.md` §1-§4** exist (DIB, TGA, ICNS, ICO). Each
  core's `THEOREMS.md` has a T1 row (ICNS's is `**T1 · M2 · ...**`).
- **guard-calls counts** against the call sites in the source
  (`grep -n 'crate::alloc_guard::...' src/*.rs`, comments excluded):
  DIB `try_zeroed` 1 (`dib.rs:9194`); TGA `try_zeroed` 1
  (`tga.rs:8912`); ICNS `try_with_room` 1 (`icns.rs:920`), `try_zeroed`
  5 (`1508, 1519, 1530, 2743, 3988`), `try_copy` 1 (`3109`); ICO
  `try_with_room` 1 (`ico.rs:891`), `import_dib` 1 (`1523`),
  `dib_verus::try_decode_with_steps_logged` 1 (`1516`). All 7 counts
  match the entries. (A text grep cannot see macro-generated or
  inlined calls; the tool's walk does.)
- **PINS.sha256.** `sha256sum -c --strict PINS.sha256`: four of four OK.
- **`source_hash`.** I recomputed it as the tool does (SHA-256 over the
  sorted per-file SHA-256 lines of every non-test module file) for all
  four cores. After E13's rename (which changed DIB's and TGA's source and
  therefore DIB's and ICO's `source_hash`) the values are DIB
  `1176feef4f943a58c0073ba25f27f53c00b40a14d7b8812321ee91adb45bf62d`,
  TGA `627a1646805bd453cb6a8cc602b677cc6095cd647b8257c92d0ec121f987ccf7`,
  ICNS `cda2ebe74a1a747f9862bfb1e519294c9925515d88816cf01d237df2cea7b269`,
  ICO `5e2660a5d6a7af5205ca07fa64cf63504173f66e6bdab839fca5027b9816a530`,
  and four of four match the regenerated allowlists. (E11/E12 recorded DIB
  `10f5071a...`, TGA `387f37cb...`; E11 alone recorded ICNS
  as `bfcd14ae...`.) The method reproduces
  the pre-E11 values exactly (DIB `c0a03456...`, TGA `f9ca2824...`, ICNS
  `01f940ad...`, ICO `5265d477...`), which is the check that it is the
  tool's computation. The E14 reports are in run `20261114T111200Z` (deleted by
  E15, whose authoritative run is `20261114T115040Z`; that run in turn replaced
  the now-deleted `20261114T094507Z`, `20261114T084716Z` and
  `20261114T081621Z`); all earlier runs are deleted, so the E11-E13 findings
  are recorded here and in `../ERRATA.md` E11-E13 rather than in a run
  directory. DIB's
  `source_hash` inside the ICO run equals DIB's own.
- **Guard hashes.** `alloc_guard/mod.rs` is `26e4c832...` in all four
  (matches E6). Each `sites.rs` matches its pin: DIB `33465c74...`
  (`dib/tools/audit_alloc.py`), TGA `58e1fb28...`, ICNS `5dff9ec0...`,
  ICO `5918505c...`. The E11 change touched none of these files.

Mismatches found: none in hashes, counts or citations. Wording defects
found: D4 ("verified invariant") and D5 (the "never aborts" attribution).
The D4 wording is fixed in the ICNS and ICO decisions files and the
regenerated allowlists (ERRATA E10); D5 is unchanged.

## 5. What is not covered

The audit says nothing about:
- A-STD holding, or panics being unreachable (both are your signature);
- clones of results (D1) and `Debug` formatting (D2);
- the guard's element-list items at other instantiations or from
  downstream crates: no longer reachable (D3/D4 resolved by `pub(crate)`),
  so this is no longer a scope reduction;
- any target other than x86_64-unknown-linux-gnu (i686 is installed,
  not audited), or any profile other than release without
  debug-assertions and without share-generics;
- LLVM and binary level: calls LLVM introduces after MIR (the binary
  scan, SPEC §6 item 4, is not built);
- the allocator being glibc `malloc`;
- a no-MIR std leaf whose contract is wrong;
- std's own callers of a shared instance (the authorisation rule
  constrains crate callers only);
- the tool itself: Sol's review found seven problems over three rounds
  (five, then two, then none), all fixed with negative controls (98 of
  98 controls pass). Sol's last round approved the fixes and said that
  approval is not approval to claim production assurance. Sol did not
  test closures, allocating `Iterator::next` implementations or every
  nested-admission graph;
- build-input inventory gaps (README prerequisite 2: undeclared reads
  are blocked, not denied; `stat` and `mmap`-without-open are not
  traced).

Signing in spite of these means: you accept that a passing report shows
"no unauthorised allocation path in the audited root call paths of
this release build, given A-STD, vstd and your reading of the entries",
and nothing wider. The README says no "proved" or "audit shows" claim
may be made while prerequisites 1 and 2 are open (1 not claimed, 2
partly met). Signing closes prerequisite 3 only.

## 6. Checklist

Minimum to sign. D8 and the D3/D4 visibility change are done (ERRATA E10);
ICNS and ICO now fail only on the unsigned reviewer (ICO also on
`dependency-contract` while DIB is unsigned).

1. Answer D1, D2, D5, D6, D7, D9, D10. Reject any item and the matching
   entries must be removed or the source changed; send it back rather
   than signing. D3, D4 and D8 are resolved.
2. Read in full: `allowlist/decisions/dib.toml`, `icns.toml`, `ico.toml`
   (the TGA file differs from DIB's only in names and the root list).
   About 260 lines in all (ICNS and ICO are 3 entries shorter after E10).
3. The D4 wording is already fixed in the ICNS and ICO `decisions` files
   and in the regenerated `*_verus.toml` (ERRATA E10). Nothing to edit
   unless you change an entry.
4. Spot check, about 15 minutes:
   - 2 `panic` entries per core against the source (section 3);
   - the `guard-calls` counts with `grep -n` (section 4);
   - `grep -c '^\[\[entry\]\]'` gives 64 / 58 / 50 / 42;
   - `grep -c '^kind = "generic-export"$'` over the decisions and generated
     allowlists gives 0;
   - one `ub_checks` entry: confirm `profile` in `[meta]` reads
     `debug-assertions=off`.
5. In each of `allowlist/x86_64-unknown-linux-gnu/release/{dib,tga,icns,ico}_verus.toml`
   set line 18, `reviewer = ""`, to `reviewer = "Your Name"`. The tool
   accepts any non-empty value except its `prototype-fixture`
   placeholder. It checks no other signature field. Leave
   `drafted_by` as it is.
6. Re-pin, because the files changed. From `alloc-delta/audit/allowlist`:
   `find x86_64-unknown-linux-gnu -name '*.toml' | sort | xargs sha256sum > PINS.sha256`
   then `sha256sum -c --strict PINS.sha256`. This is the same line
   `tools/draft_all.sh` uses. Do not run `tools/draft_all.sh` after
   signing: it regenerates the files, resets `reviewer` and rewrites the
   pins.
7. Run `./run_audit.sh` from `alloc-delta/audit` (builds with `-j1`; it
   is slow on one CPU). Expected after signing: DIB and TGA PASS; ICNS
   PASS; ICO PASS once DIB passes in the same run (D9). The latest
   unsigned run is `runs/20261114T115040Z`, which covers all four cores
   after the E15 `try_parse` error clause (and the E14 ICNS handle, the E13
   rename, E11 public-API reduction and E12 `try_rle_logged` demotion before
   it):
   DIB, TGA and ICNS only
   `allowlist-reviewer`; ICO `allowlist-reviewer` and
   `dependency-contract`; the run directory under `runs/` is the
   evidence.
8. Commit the four allowlists, `PINS.sha256` and the run directory
   together. Any later edit to an allowlist breaks the pin check and
   means a new review.

Anything in the allowlists that looked wrong, beyond the two wording
defects (D4 now fixed, D5 open): nothing. The `reviewer` field is the
only blocker for DIB, TGA and ICNS; ICO also needs DIB signed (D9).
