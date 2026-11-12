# alloc_audit — prototype NOTES (gate F1)

A `rustc_driver` tool that walks monomorphised MIR from named public roots and
reports every path to an allocator entry point except through an allowlist.
Unresolved edges fail closed. This file records what was built, the control
table with expected vs actual, the review burden on DIB, the 32-bit story, and
the blind spots.

Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64-unknown-linux-gnu,
rustc-dev present (`librustc_driver-7ed0375ae352bd75.so` in the sysroot).
Tool source: this directory. Build: `cargo build --offline -j2`, with
`CARGO_TARGET_DIR=/tmp/claude-0/alloc-audit-target`, `RUSTC_BOOTSTRAP=1`.

## What was built

`src/main.rs` — a `rustc_driver` program. It implements
`Callbacks::after_analysis`, and also implements the `RUSTC_WORKSPACE_WRAPPER`
protocol (cargo invokes it as `<wrapper> <rustc> <args...>`; the leading rustc
path is dropped and the rest passed to `run_compiler`). Because it does not
return `Compilation::Stop`, the normal pipeline still emits the crate's
artifact: the wrapper is transparent to cargo. Configuration is by environment
variable, since a workspace wrapper cannot carry its own CLI:

- `ALLOC_AUDIT_ROOTS` — comma-separated roots; default is every exported
  non-generic `pub fn`.
- `ALLOC_AUDIT_ALLOWLIST` — allowlist TOML; default none (everything fails).
- `ALLOC_AUDIT_REPORT` — path to write the text report.
- `ALLOC_AUDIT_PROTOTYPE` — when set (the `--prototype` flag), a
  `prototype-fixture` reviewer is accepted and `mode: PROTOTYPE` is printed in
  the report header. Absent it, that reviewer value fails the audit.
- `ALLOC_AUDIT_EDGES` — when set, also emit the full reachable edge list.

Exit status: 0 pass, 1 findings, 2 tool error (rustc itself failed, giving no
verdict).

`src/walk.rs` — the traversal. From each root `Instance::mono`:

- `Call`/`TailCall` with `FnDef` operand: `Instance::try_resolve` under
  `TypingEnv::fully_monomorphized()`. `Ok(None)` and `Err` both fail closed.
- `Drop`: drop glue resolved through the `DropGlue` lang item with
  `try_resolve` (not `resolve_drop_glue`, which aborts the compiler on failure);
  a failure is a finding. Glue MIR is walked, so `Drop` impls and field drops
  are reached.
- `Assert`: the panic lang item is recorded as a panic leaf with a per-function
  identity. `BoundsCheck` and `MisalignedPointerDereference` are named directly,
  because `AssertKind::panic_function()` `bug!()`s on them.
- `Rvalue::Cast(PointerCoercion::ReifyFnPointer | ClosureFnPointer)`: edge to
  the pointed-to instance, so a body reached only through a fn pointer is still
  audited.
- `Rvalue::Cast(PointerCoercion::Unsize)` to a trait object: the `&T`/`Box<T>`
  wrapper is unpeeled to the concrete type, then `tcx.vtable_entries` yields
  every vtable method as an edge. The dynamic call itself is a finding.
- `InlineAsm`: a finding, plus edges for its `SymFn` operands.
- Every body is instantiated with the instance's args
  (`try_instantiate_mir_and_normalize_erasing_regions`); an instantiation
  failure is a leaf. Walking the un-instantiated generic body was the first bug
  found: it ICEs on `Box<dyn Any>` drop glue.
- Bodies with no available MIR (`is_mir_available` false) are leaves.
- Allocator entry points (`alloc::alloc::{alloc, alloc_zeroed, realloc, dealloc,
  exchange_malloc, handle_alloc_error}` and the `__rust_alloc*` / shim symbols,
  matched on def-path or symbol name) are the targets: each is reported with the
  full root-to-entry path.
- **Authorization is decided over every path, not the first one.** The walk
  records the whole edge set and every allocator entry reached; a post-pass
  (`authorize_allocations`) then enumerates every simple root-to-entry path and
  judges each one. An entry is `OK` only when *every* path to it is justified;
  any unjustified path makes it a finding, naming each offending path (bounded
  to 8 shown paths per entry, and to 20000 enumerated paths per root with a
  fail-closed `path-bound` finding past that). This replaces the first-BFS-parent
  rule that Sol's review found HIGH: the old rule kept only the shortest parent
  of each instance, so when an allowed guard and an unapproved helper both called
  the SAME monomorphised instance (`RawVecInner::<A>::reserve_exact<Global>` is
  parameterised by the allocator only, so a `Vec<u8>` and a `Vec<u32>` share it),
  the guard's path could be kept and the helper's never examined.
- A path is judged by the last allocation-capable item on it (matched against an
  `alloc` entry's `item`) and the nearest *audited-crate* function before that
  item. Nodes defined in the crate under compilation are marked at visit time
  (`DefId::is_local`), so free functions, impl methods and closures all count;
  std's internal callers between the item and the crate function are skipped,
  because they are pinned by the toolchain commit rather than by this allowlist.
  A path with no audited-crate caller before the item is admitted only by an
  `allow = "*"` entry.
- `sess.opts.share_generics()` is checked; if true, the whole audit fails.
- The compilation profile is detected (`sess.opts.optimize`,
  `debug_assertions`, `share_generics()`) into a canonical string
  `opt-level=<n>,debug-assertions=<on|off>,share-generics=<on|off>` plus a class
  (`release`/`debug`/`custom`); it is printed in the report header and compared
  against the allowlist's recorded `profile`.

## Provenance checks (task 4/5)

The allowlist records `profile` and the tool refuses a mismatch: a recorded
profile must equal the detected string exactly, else an `allowlist-profile`
finding fails the run closed. A release allowlist (opt-level 3) run against a
debug build (opt-level 0) fails on both `share-generics` and `allowlist-profile`.

The allowlist records `source_hash`, the SHA-256 over the audited crate's source
files. The tool computes it from the source map (every `FileName::Real` file not
under the sysroot), records each file's hash and the combined hash in the report
header, and fails closed if the recorded value is absent or differs. The
combined hash is the SHA-256 of the files' individual content hashes sorted
lexicographically, so it pins the source text and is independent of checkout
location.

**What the hash does not cover (stated plainly, and printed in the report header
as `source-hash scope: source-map text only`).** The hash covers the source-map
*text* of the crate's own files — nothing else. It does not cover:
`include_bytes!`/`include_str!` payloads (the embedded file's bytes are not in
the source map), a build script's output (`OUT_DIR` contents, `cargo:rustc-env`
values, `cargo:rustc-cfg`), proc-macro output (the tokens a foreign macro
generates are not the crate's source text), and dependency source text (only the
crate under audit is hashed; std is pinned by the commit, path-dependencies are
not pinned at all). Items (4) and (12) below record the consequence and the
detection added for it.

The `reviewer` field must be non-empty and must not be the literal
`prototype-fixture` unless the run sets `ALLOC_AUDIT_PROTOTYPE` (the
`--prototype` flag); the report header then prints `mode: PROTOTYPE (...)`. A
fixture allowlist therefore passes only under that flag, and a real allowlist
naming a human passes either way.

`src/allowlist.rs` — a small hand-rolled TOML subset reader (no external
crates). `[meta]` carries rustc commit, vstd version, target, **profile**,
reviewer and the declared **source_hash**. **Every `[meta]` field is required**;
a missing field is a distinct fail-closed finding (`allowlist-commit-missing`,
`allowlist-target-missing`, `allowlist-profile-missing`,
`allowlist-source-hash-missing`, `allowlist-vstd-missing`), not a silent pass.
`[[entry]]` has `item`, `kind`
(`intrinsic|no-mir|panic|ffi|alloc|build-script|proc-macro|include`), optional
`allow` (an admitted caller for `alloc` entries, or `*`), `reason` and
`reviewer`. A leaf passes when an entry matches its identity and kind. An
allocation passes when every path to the entry has its allocation-capable item
named by an `alloc` entry whose `allow` matches the nearest audited-crate caller.
The file's SHA-256 is computed with `sha256sum` and printed in the report.

Provenance comparisons, all fail-closed:

- **commit.** Required. The recorded value must equal the running compiler's
  reported commit token, or its full 40-hex hash, or be a *prefix* of that full
  hash of at least 12 hex characters. The comparison is one-directional: a
  recorded value **longer** than the actual hash is refused. The previous code
  accepted `rec.starts_with(run)`, so a 44-hex recorded value passed.
- **target.** Required; must equal the running target exactly.
- **profile.** Required; must equal the detected canonical profile string
  exactly.
- **source_hash.** Required; must equal the computed combined hash.
- **vstd.** Required. If the audited crate links vstd (detected from the crate
  graph: `vstd` is a loaded crate), the recorded `vstd_version` must equal the
  linked version, read from the crate's `Cargo.lock` (found via
  `CARGO_MANIFEST_DIR`) or from the `ALLOC_AUDIT_VSTD_VERSION` override. If the
  crate links vstd and the tool cannot determine the linked version, it fails
  closed (`allowlist-vstd-undetermined`). If the crate does **not** link vstd,
  the allowlist must declare `vstd = "none"` (or `vstd_version = "none"`);
  otherwise `allowlist-vstd-not-declared-none` fails the run. A crate that
  links vstd but declares `vstd = "none"` fails.
- **reviewer.** Required; non-empty, and not `prototype-fixture` unless the run
  sets `ALLOC_AUDIT_PROTOTYPE` (the `--prototype` flag); the header then prints
  `mode: PROTOTYPE (...)`.

`src/report.rs` — the verdict. Fail-closed: any refused edge, any unexplained
leaf, or any allocation not covered by an `alloc` entry fails. Justified
allocations are reported as `OK`, unexplained ones as `!!`, so a passing run
still shows every allocation it admits. The header prints the
`source-hash scope: source-map text only` line and every detected build input
(build script, foreign proc-macro crates, `include_*!` sites).

## Control table

Run with `run_tests.sh` (testcases) and `dib_audit.sh` (DIB). All testcases
built at `-Copt-level=2` so `share_generics` is off (see below).

| Control | Seeded construct | Expected | Actual |
|---|---|---|---|
| n1_push_unreserved | `Vec::new` + `push`, no reserve | fail; alloc via `Vec::push_mut` -> `grow_amortized` -> `finish_grow` | exit 1; 4 alloc entries (alloc/alloc_zeroed/realloc/dealloc), path `n1_push_unreserved -> Vec::push_mut -> grow_one -> grow_amortized -> finish_grow -> allocate` |
| n2_clone | `to_vec` then `clone` of a `Vec` | fail; alloc via `try_allocate_in` | exit 1; 3 alloc entries, path `n2_clone -> try_allocate_in -> allocate_zeroed` |
| n3_format | `format!` | fail; alloc via `String` growth from `to_owned` | exit 1; 2 alloc entries, path `n3_format -> str::to_owned -> to_vec -> with_capacity_in -> try_allocate_in` |
| n4_box | `Box::new` | fail; alloc via `box_new_uninit` | exit 1; 2 alloc entries, path `n4_box -> Box::new -> box_new_uninit -> alloc_impl_runtime` |
| n5_collect | `iter().map(..).collect()` | fail; alloc via `FromIterator` and `spec_extend`/`reserve` | exit 1; 4 alloc entries, paths through `spec_from_iter_nested::from_iter` and `spec_extend::spec_extend -> extend_trusted -> reserve -> do_reserve_and_handle -> grow_amortized` |
| n6_fnptr | `let f: fn()->Vec<u8> = alloc_helper; f()` | fail closed: `fn-ptr-call` finding | exit 1; `fn-ptr-call` finding, plus the 4 entries of the recovered candidate `alloc_helper` are also reported |
| n7_dyn | `&dyn Make` call, unsizing done by the caller | fail closed: `virtual-dispatch` | exit 1; `virtual-dispatch` finding; 0 alloc entries (the impl body is not reachable from this root) |
| n7b_dyn_local | `&dyn Make2` unsizing inside the root | fail closed: `vtable-unsize` + `virtual-dispatch`; vtable methods followed | exit 1; both findings; 4 alloc entries with path `n7b_dyn_local -> <Allocating2 as Make2>::make -> ... -> grow_amortized` |
| n8_helper | allocation in a user helper, called directly | fail; helper followed transitively | exit 1; 4 alloc entries, path `n8_caller -> helper_grow -> Vec::push_mut -> ...` |
| n9_drop | `Drop` impl that allocates | fail; drop glue followed into the `Drop` impl | exit 1; 4 alloc entries, path `n9_drop -> drop_glue<Dropper> -> <Dropper as Drop>::drop -> Vec::push_mut -> ...` |
| c1_clean | only `guard::try_zeroed` (reserve-then-push) allocates | pass with the allowlist | exit 0; 4 alloc entries, all `OK`; 0 findings; 0 unexplained leaves |
| c1_mutated | c1 plus a second `Vec::new`+`push` outside the guard | fail even with the c1 allowlist | exit 1; 4 entries all `!!`, each naming `c1_mutated_root -> scratch_mutated -> Vec::push_mut -> ...` as an `unauthorized-alloc-path` (6 such findings), plus one `[allowlist-source-hash]` because the mutated source no longer matches the hash recorded in the c1 allowlist; 7 fail-closed findings in all |
| n10_ffi | `extern "C" { fn getpid(); }` | fail closed: `ffi` leaf | exit 1; leaf `[ffi] getpid` |
| n11_shared_instance | allowed guard and unapproved helper call the SAME `reserve_exact<Global>` / `push_mut` instances | fail; the helper's paths are named | exit 1; 12 `unauthorized-alloc-path` findings naming `unapproved_reserve` and `unapproved_grow` paths (was `Pass` before the fix — see below) |
| n12_suffix_lookalike | `unapproved_guard::try_zeroed` — a caller whose def-path *ends with* the admitted `guard::try_zeroed` — calls the same shared `reserve_exact` instance | fail; the lookalike is refused while the real guard's path stays justified | exit 1; 6 `unauthorized-alloc-path` findings naming `unapproved_guard::try_zeroed`; the `guard::try_zeroed` path is `OK` |
| n13_root_lookalike | a root-level `try_zeroed` (def-path `try_zeroed`) calls the same shared instance | fail; the root-level name is refused | exit 1; 6 `unauthorized-alloc-path` findings naming `try_zeroed`; the `guard::try_zeroed` path is `OK` |
| c2_two_guards | two admitted guards (`guard_a`, `guard_b`) call the SAME `reserve_exact<Global>` instance, each with its own `[[entry]]` | pass | exit 0; 4 alloc entries all `OK`; 0 findings (with a single entry the run fails — the control is load-bearing) |
| n14_two_guards_plus_third | c2 plus a third, unapproved caller (`unapproved_third`) of the same instance | fail; the third caller is named | exit 1; 6 `unauthorized-alloc-path` findings naming `unapproved_third` |
| c1_commit_missing | allowlist omits `rustc_commit` | fail | exit 1; `[allowlist-commit-missing]` |
| c1_target_missing | allowlist omits `target` | fail | exit 1; `[allowlist-target-missing]` |
| c1_profile_missing | allowlist omits `profile` | fail | exit 1; `[allowlist-profile-missing]` |
| c1_source_hash_missing | c1_clean's allowlist with the `source_hash` key *absent* from `[meta]` (not merely wrong) | fail closed with the missing-source-hash finding | exit 1; `[allowlist-source-hash-missing]`, printing the hash the sources actually have (`5d90e4c0…`) |
| c1_reviewer_missing | c1_clean's allowlist with the `reviewer` key *absent* from `[meta]` (not merely empty: the key is gone, so no entry `reviewer` line can supply it) | fail closed with the reviewer finding | exit 1; `[allowlist-reviewer]` "reviewer is missing or empty (recorded None)" |
| c1_commit_toolong | allowlist records a 44-hex commit (actual hash with two extra hex chars) | fail | exit 1; `[allowlist-commit]` (the old `rec.starts_with(run)` accepted this) |
| c1_commit_short | allowlist records `58cf8f` (6 hex chars) | fail | exit 1; `[allowlist-commit]` (prefix must be ≥ 12 hex chars) |
| c3_vstd_ok | vstd-linked crate, correct recorded `vstd_version` | pass | exit 0 |
| c3_vstd_change | vstd-linked crate, recorded version `0.0.0-1999-01-01-0000` | fail | exit 1; `[allowlist-vstd]` |
| c3_vstd_missing | vstd-linked crate, no `vstd_version` recorded | fail | exit 1; `[allowlist-vstd-missing]` |
| c3_vstd_undet | vstd-linked crate, version recorded, but the linked version cannot be resolved: `ALLOC_AUDIT_VSTD_VERSION` unset and no readable `Cargo.lock` (the run unsets `ALLOC_AUDIT_VSTD_VERSION` and `CARGO_MANIFEST_DIR`) | fail closed: allowlist-vstd-undetermined | exit 1; `[allowlist-vstd-undetermined]`, `vstd_actual` printed as `None` |
| c1_vstd_undeclared | non-vstd toy crate whose allowlist never declares `vstd = "none"` | fail | exit 1; `[allowlist-vstd-not-declared-none]` |
| n15_include | `include_str!("n15_payload.txt")` of a non-source file | fail closed: `undeclared-include` | exit 1; `[undeclared-include]` naming `include_str!n15_payload.txt` (declaring `[[entry]] kind = "include" item = "include_str!n15_payload.txt"` makes it pass) |
| bs_build_script | c1_clean run with `OUT_DIR` set (the build-script probe) | fail closed: `undeclared-build-script` | exit 1; `[undeclared-build-script]` |
| n16_proc_macro | a crate that links a foreign proc-macro crate `pmtoy` (a toy proc macro, built offline, whose `passthrough!` generates the audited `n16_proc_macro` item) | fail closed: undeclared-proc-macro | exit 1; `[undeclared-proc-macro]` naming `pmtoy` (declaring `[[entry]] kind = "proc-macro" item = "pmtoy"` makes it pass) |
| commit mismatch (fixture) | allowlist records `deadbeef` | fail | exit 1; `[allowlist-commit]` |
| target mismatch (fixture) | allowlist records i686 | fail | exit 1; `[allowlist-target]` |
| profile mismatch (fixture) | release allowlist, debug build | fail | exit 1; `[share-generics]` + `[allowlist-profile]` |
| profile field mismatch (fixture) | allowlist records `opt-level=3`, run at `opt-level=2` | fail | exit 1; `[allowlist-profile]` |
| source_hash mismatch (fixture) | allowlist records a wrong hash | fail | exit 1; `[allowlist-source-hash]` (prints the actual hash) |
| empty reviewer (fixture) | `reviewer = ""` | fail | exit 1; `[allowlist-reviewer]` |
| `prototype-fixture` reviewer, no flag | c1_clean allowlist, no `--prototype` | fail | exit 1; `[allowlist-reviewer]` |
| named reviewer, no flag | `reviewer = "Ada …"`, no `--prototype` | pass | exit 0 |
| `-Zshare-generics=y` (`c1_share_generics`) | c1_clean at `-Copt-level=2` with `-Zshare-generics=y` (needs `RUSTC_BOOTSTRAP=1`, which `run_tests.sh` sets for this case); allowlist `allowlist/c1_share_generics.toml` is `c1_clean.toml` with `profile` recorded as `share-generics=on`, so the profile matches | fail | exit 1; `[share-generics]` is the only fail-closed finding (4 alloc entries allowlisted, 0 unexplained leaves); detected profile `opt-level=2,debug-assertions=off,share-generics=on [custom]` |

All 42 runs in `run_tests.sh` have archived reports
(`reports/<tag>.{txt,stdout,stderr}`). Four are positive controls expected to
pass — c1_clean, c1_rev_named, c2_two_guards, and c3_vstd_ok, i.e. **4 passing**
— and the remaining **38 are negative controls** expected to exit 1:

- allocation negatives: n1-n10 (10) + n7b (1) + n11 (1) + n14 (1) + n15 (1) +
  n12 (1) + n13 (1) = 16
- c1_mutated (1)
- provenance fixtures: c1_commit_mismatch, c1_target_mismatch, c1_prof_mismatch,
  c1_prof_bad, c1_hash_bad, c1_rev_empty, c1_rev_fixture (7)
- missing-field fixtures: c1_commit_missing, c1_target_missing,
  c1_profile_missing, c1_source_hash_missing, c1_reviewer_missing (5)
- commit-length fixtures: c1_commit_toolong, c1_commit_short (2)
- vstd fixtures: c3_vstd_change, c3_vstd_missing, c3_vstd_undet,
  c1_vstd_undeclared (4)
- build-input negatives: bs_build_script, n16_proc_macro (2)
- `-Zshare-generics=y` (c1_share_generics) (1)

16 + 1 + 7 + 5 + 2 + 4 + 2 + 1 = 38 negative, 4 positive = 42 runs.

The four controls added in this revision (`c1_source_hash_missing`,
`c1_reviewer_missing`, `c3_vstd_undet`, `n16_proc_macro`) were each verified to
fail for exactly their stated reason: each produces exactly one fail-closed
finding (the named one) and nothing else, so none is passing or failing for a
second, unrelated reason. Each is also load-bearing: the three field controls
pass under `c1_clean.toml`, and control (d) passes once `pmtoy` is declared with
an `[[entry]]`. The pre-existing 38 kept their verdicts byte-for-byte.

The DIB positive control (`dib_audit.sh`) is a further pass, and the pre-fix N11
evidence at `reports/n11_pre_fix.txt` is a reconstruction, not part of the 42.

Each collision control is load-bearing: n12 fails only because the whole-segment
matcher refuses `unapproved_guard::try_zeroed`, and under the old
`ends_with`/`ends_with` predicate it would have passed
(`"unapproved_guard::try_zeroed".ends_with("guard::try_zeroed")` is true); n13
fails only because the old reverse test `allow.ends_with(c)` no longer admits
the root-level `try_zeroed` (`"guard::try_zeroed".ends_with("try_zeroed")` is
true). c2 passes only because every matching entry is consulted: with a single
entry, the guard whose own entry is missing fails (verified by deleting the
`guard_b` entry, which turns c2 into a 6-finding failure).

The c1 mutation is the audit analogue of mutations A1 ("ignore the reservation
result") and A11 ("build the entry list with `Vec::new` + `push`"): adding an
allocation outside the guard turns a passing audit into a failing one, and the
new path is named in the report.

### Before/after the shared-instance fix

The table above is the *after* state. The *before* column is the tool as Sol
reviewed it (first-BFS-parent authorization only); the *after* column is the
tool after `authorize_allocations`. `n11` did not exist before; it was added to
exercise the defect.

| Control | Before (first-parent rule) | After (all-paths) |
|---|---|---|
| n11_shared_instance | n/a (new) — the defect it targets: the same crate passes as c1 would, exit 0, guard path only. **Reproduced** by `prefix-reconstruction/reproduce.sh`; report archived at `reports/n11_pre_fix.txt` (exit 0, `Pass`) | exit 1; 12 `unauthorized-alloc-path` findings, `unapproved_reserve`/`unapproved_grow` paths named (including `unapproved_grow -> Vec::push_mut`, the shared growth instance) |
| n1_push_unreserved | exit 1, 4 entries | exit 1, 4 entries (unchanged) |
| n2_clone | exit 1, 3 entries | exit 1, 3 entries (unchanged) |
| n3_format | exit 1, 2 entries | exit 1, 2 entries (unchanged) |
| n4_box | exit 1, 2 entries | exit 1, 2 entries (unchanged) |
| n5_collect | exit 1, 4 entries | exit 1, 4 entries (unchanged) |
| n6_fnptr | exit 1, 1 finding | exit 1, 1 finding (unchanged) |
| n7_dyn | exit 1, 1 finding | exit 1, 1 finding (unchanged) |
| n7b_dyn_local | exit 1, 2 findings | exit 1, 2 findings (unchanged) |
| n8_helper | exit 1, 4 entries | exit 1, 4 entries (unchanged) |
| n9_drop | exit 1, 4 entries | exit 1, 4 entries (unchanged) |
| n10_ffi | exit 1, 1 leaf | exit 1, 1 leaf (unchanged) |
| c1_clean (allowlist) | exit 0, 4 OK — but the guard's own `push_mut` growth path was *not* checked, only the reserve path its first parent pointed at | exit 0, 4 OK — now every path is checked; the fixture gains one `push_mut` entry (SPEC §6 requires it) so the guard's real growth path is named |
| c1_mutated (allowlist) | exit 1, 1 OK + 3 unexplained | exit 1, 4 `!!` (each entry now carries the `scratch_mutated` path as an `unauthorized-alloc-path` finding) |
| DIB positive control (allowlist) | Pass, 91 nodes / 277 edges, 3 OK | Pass, 91 nodes / 277 edges, 3 OK (unchanged) |

The c1_clean fixture changed for an honest reason, not to paper over the fix:
under the first-parent rule the report showed only the `reserve_exact` path and
never mentioned `guard::try_zeroed`'s own `push_mut -> grow_one ->
grow_amortized -> finish_grow` path, which is in the MIR and reaches the
allocator. The strict check surfaces it, and SPEC §6 already requires a `push`
entry admitted from the guard for exactly this reason. DIB needs no such entry:
its one site is `from_elem`, which is exact-size and never pushes.

## DIB positive control (gate F1)

Run in the **release** profile (the spec's production configuration; see the
share-generics note below), roots `decode_with_steps,decode,decode_with_policy`.
Run with `ALLOC_AUDIT_PROTOTYPE=1` (the allowlist is a fixture). The detected
profile is `opt-level=3,debug-assertions=off,share-generics=off` [release] and is
printed in the report header. The recorded `source_hash` is the combined hash
over `src/{dib.rs,lib.rs,model.rs,model_vectors.rs}`; the report prints each
file's hash and the combined value
(`cacb25c6fa40b84dd21335516d65bd57d744a42f25257a47355033e5b6280b42`).

Without an allowlist the run fails, as it must, and reports (release:
91 instances, 277 edges):

- **Allocator entry points: 3.**
  - `alloc::alloc::__rust_alloc_zeroed` and `__rust_alloc`, both on the path
    `dib::decode_with_steps -> dib::decode_tracked -> dib::alloc_output ->
    dib::alloc_output_n -> <u8 as SpecFromElem>::from_elem -> try_allocate_in ->
    allocate_zeroed -> alloc_impl_runtime`. This is exactly the one `vec!` site
    at `dib.rs:8210`, reached through `alloc_output_n`.
  - `alloc::alloc::__rust_dealloc`, on the drop path
    `decode_with_steps -> decode_tracked -> drop_glue<Vec<u8>> -> ... ->
    deallocate`. Deallocation cannot fail.
- **Fail-closed findings: 0** (all three roots resolved; no indirect call, no
  dyn call, no FFI, no asm, no unresolved edge, share-generics off). With *no
  allowlist* the strict all-paths check adds `unauthorized-alloc-path` findings
  for the `from_elem`/`with_capacity_in` paths (nothing is admitted), which is
  the intended fail-closed behaviour.
- **Unexplained leaves: 28** — 19 panic sites, 3 no-MIR std items, 5 intrinsics,
  1 FFI shim.

With a 21-entry prototype allowlist the run passes: all 3 allocations are `OK`
and the leaves are covered. The allowlist SHA-256 appears in the report
(`sha256=1f9739e4d339ef2cbd8e4b41a54a72833fdf6196e9e18c16d3ae044eebcef2c8`). The
hash changed from the pre-fix `295c8bf4…` because the fixture gained the
`profile` and a real `source_hash`; the entry count is unchanged at 21.

Note the `vec!` site is a `from_elem` with an exact size: it reaches
`try_allocate_in` and does not go through `finish_grow`. Under the delta it
becomes `alloc_guard::try_zeroed(4*n)`, and the release walk would then enter
`reserve_exact -> try_reserve_exact -> grow_exact -> finish_grow`
(`finish_grow`'s body **is** available in release for this generic
instantiation — confirmed in the debug/opt-2 runs of the toy crates, where
`finish_grow` appears on the path). The allowlist entry for the new site names
`alloc_guard::try_zeroed` as the admitted caller and must state the growth path
that ends in `handle_alloc_error`.

### Unresolved edges hit on DIB, and the review burden

Every item below is a leaf that the walk cannot enter; each needs an allowlist
entry with a reason and a named reviewer before the audit passes. This is the
realistic review burden for DIB on x86_64 release.

Non-allocating leaves (9 distinct identities, 9 entries):

| Kind | Identity | What the reviewer must establish |
|---|---|---|
| ffi | `alloc::alloc::__rust_no_alloc_shim_is_unstable_v2` | std alloc-shim stability probe; declares, does not allocate |
| no-mir | `alloc::raw_vec::handle_error` | capacity-overflow abort; excluded by the guard's `n <= isize::MAX` precondition (A-STD-3) |
| no-mir | `core::panicking::panic_fmt` | panic entry point; unreachable by T1/Safe' |
| no-mir | `core::panicking::panic_nounwind_fmt` | panic entry point; unreachable by T1/Safe' |
| intrinsic | `std::intrinsics::cold_path` | codegen hint |
| intrinsic | `std::intrinsics::ctlz<u32>` | leading-zero count |
| intrinsic | `std::intrinsics::cttz<u32>` | trailing-zero count |
| intrinsic | `std::intrinsics::ctpop<usize>` | population count |
| intrinsic | `std::intrinsics::write_bytes<u8>` | memset used by the zero fill |

Panic leaves (19 edges, 2 distinct identities: `PanicBoundsCheck`,
`PanicDivZero`). The tool records one leaf per (function, panic kind), so a new
panic edge in a changed function shows up as a diff. The 18 functions carrying
panic edges in release are: `Vec::set<u8>`,
`dib::{apply_and, bmp_prefix, bmp_prefix_policy, channel, colour, dib_fields,
finish_mask, masks_valid, masks_valid_policy, paint_abs, policy_packet_index,
raw_policy_word, raw_sample, rd32, rle_clip_absolute, rle_clip_command,
rle_clip_encoded, rle_command}`. In a production allowlist each carries its
T1/`Safe'` citation. The prototype covers them with two name-level entries; the
tool prints the per-function list so the reviewer can see exactly which edges
the citation must cover.

Allocation-capable items to admit (2 entries): the `from_elem` site from
`alloc_output_n`, and `RawVecInner::deallocate` from anywhere.

So the manual review burden for DIB release is **21 allowlist entries**: 1
allocation-capable caller, 1 deallocation, 9 non-allocating leaves, 2 panic
identities spread over 19 edges (18 functions). The panic citations are the bulk
of the reading: every bounds check and division in the reachable body needs its
unreachability argument. A production file would enumerate them per function so
that a changed function re-opens exactly its own citation.

In the **debug** configuration the same run reaches 130 instances / 671 edges
and 124 unexplained leaves (including the `PanicMulOverflow`,
`PanicAddOverflow`, `PanicSubOverflow`, `PanicShl/ShrOverflow`, `PanicNegOverflow`,
`PanicRemZero` identities that release optimises away). The audit is
profile-sensitive: the spec's rule "run on every production configuration" means
the allowlist is per (target, profile). The tool now **enforces** this: the
release allowlist records `profile = "opt-level=3,debug-assertions=off,share-generics=off"`
and a debug run fails on both `[share-generics]` and `[allowlist-profile]`
(confirmed with `run_tests.sh`'s `c1_prof_mismatch` control). The header always
records the profile the audit saw, allowlist or not.

## How it would work for the 32-bit target

The tool itself is target-agnostic: it reads `sess.opts.target_triple`, records
it in the report and in the allowlist comparison, and refuses an allowlist whose
recorded target differs. Everything it walks — `Instance`, `try_resolve`, drop
glue, `resolve_drop_glue`, `vtable_entries` — is defined in terms of the
compilation's target, so pointing it at i686 would produce the i686 walk.

It cannot be run here. There is no `rust-std` for `i686-unknown-linux-gnu` in
the pinned toolchain, and no `rust-src` for `-Zbuild-std`. A build for that
target fails before analysis with `E0463: can't find crate for std` (confirmed).
The verified claim "no allocation outside the guard is reachable" is therefore
**x86_64 only**; i686 is **unaudited** until the owner installs
`rust-std-i686-unknown-linux-gnu` for 1.99.1. When it is installed, i686 needs
its own allowlist file, reviewed separately: the resolved std instances differ
(32-bit `usize` changes which `Layout`/overflow branches are live, and `Entry`'s
layout differs), so the review is not transferable. The Verus theorems already
quantify over 32- and 64-bit `usize`, so the unaudited gap is exactly the
audit's scope, not the theorems'.

## What it cannot see (written plainly)

1. **Precompiled std bodies with no exported MIR.** A non-`#[inline]` std
   function is a leaf: the tool records it and needs an allowlist entry, but it
   cannot see inside. It does not prove the entry's contract; it records that a
   human asserted one. On DIB release the leaves in this class are
   `alloc::raw_vec::handle_error`, `core::panicking::panic_fmt` and
   `panic_nounwind_fmt`. The spec's predicted `alloc::raw_vec::finish_grow` was
   *not* a leaf in the configurations run here — its body was available — but in
   any configuration where a std generic is precompiled without MIR it becomes
   one. So the audit's coverage of std's internals depends on the toolchain's
   inline/MIR-export decisions, which is why the allowlist pins the commit.
2. **Indirect calls.** A call through a `fn` pointer is not statically resolved.
   The tool recovers candidate targets from the casting sites in the same body
   and audits those too, but it cannot prove the recovered set is complete, so
   it fails closed. There is no allowlist escape that lists "the full set of
   possible targets" in the prototype; the spec allows one and none is planned.
3. **Trait-object dispatch.** The tool follows the vtable methods when the
   unsizing is visible, but a `dyn` call whose vtable was built elsewhere still
   fails closed. The target set is not enumerated to a proof.
4. **FFI.** Any `extern` item fails closed. It cannot see what a foreign
   function allocates.
5. **Inline and global assembly.** A finding; the tool does not model it.
6. **`-Zshare-generics`.** Detected and fatal. Note the default is profile
   dependent: `share_generics()` returns true for `-Copt-level=0/1/s/2` and
   false for `-Copt-level=3`. A debug build therefore fails the audit by
   default; the audit is meaningful only in the release profile the spec names,
   or with `-Zshare-generics=n` set explicitly.
7. **Allocator identity.** The tool sees that `__rust_alloc` is reached; it does
   not see which `GlobalAlloc` is installed, nor that the `System` allocator is
   glibc `malloc`. A-STD, not the audit, is what ties `__rust_alloc` to the
   allocator under test.
8. **Const-eval and static-initialiser allocations.** The walk covers fn bodies;
   an allocation performed during const evaluation is not a MIR `Call` in a
   root body and is not reported. The tool would need a separate
   static-initialiser pass to catch it. None of the DIB roots has one, but this
   is a real gap for a crate that does.
9. **Panic-path allocation is admitted, not analysed.** The panic machinery in
   std allocates (the payload `String`). The tool admits panic edges only by a
   reviewer's T1/`Safe'` citation that the edge is unreachable — it does not
   prove the panic handler is non-allocating. If a panic edge is reachable, the
   audit's "no allocation outside the guard" claim is void for that edge, and
   only the citation (not the tool) carries the weight.
10. **The tool trusts `is_mir_available`/MIR as presented.** It audits the MIR
    rustc gives it. A `#[inline(always)]` body that rustc inlines and then
    discards is still visible as an edge here (the walk is over pre-codegen
    MIR), which is the conservative direction; but a body the compiler never
    gives MIR for is a leaf, per (1).
11. **The all-paths check bounds path enumeration.** It enumerates simple
    (non-repeating) root-to-entry paths up to 20000 per root and depth 80, and
    fails closed with a `path-bound` finding past either bound. A graph with more
    paths than the bound does not pass quietly; it fails, so the bound trades
    completeness of the *report* for a guaranteed fail-closed verdict, not for a
    false pass. Simple paths only means a cycle contributes nothing beyond the
    first visit, which is sound for reachability: any node reachable only through
    a cycle is also reachable by the simple path that first enters it.
12. **The combined source hash covers the crate's own files only.** Files under
    the sysroot are excluded (std and the `rustc-src` tree are pinned by the
    recorded rustc commit, not by a hash). A crate whose behaviour depends on a
    `path`-dependency's text is not covered unless that dependency is also
    audited and its hash recorded; the dependency-contract mechanism (SPEC §6,
    ICO admitting DIB's entry point with DIB's hash) is the intended place for
    that, and this prototype does not implement it.
13. **Authorization constrains the audited crate's callers only.** A path is
    judged by the nearest function defined in the crate under compilation. A
    caller inside std (for example a std generic that happens to call the same
    allocation-capable instance) is not matched against the allowlist, because
    the allowlist's `allow` values name crate functions and std's internals are
    pinned by the commit. This is the intended scope — the audit establishes "no
    allocation outside the guard in *this* crate" — but a reviewer must not read
    an `OK` as "no std caller reaches this instance either".
14. **Build inputs outside the source-map text are detected heuristically and
    must be declared.** The `source_hash` is over source-map text only (item
    12). Three inputs sit outside it, and the tool fails closed on each unless
    the allowlist declares it:
    - **Build script.** Detected by the presence of `OUT_DIR` in the
      environment, which cargo sets for the audited crate only when it has a
      `build.rs`. Declare `[[entry]] kind = "build-script" item = "build.rs"`.
      The detection is conservative: it over-reports (a `build.rs` that writes
      nothing cargo consumes still sets `OUT_DIR`), never under-reports within
      cargo. A crate compiled by hand without `OUT_DIR` that nonetheless has a
      `build.rs` would be missed; the production process must hold the build to
      cargo.
    - **Foreign proc-macro crates.** Detected as the crates in the crate graph
      whose `CrateSource` has a `dylib`. The Verus builtin macros
      (`verus_builtin_macros`, `verus_state_machines_macros`, `verus_builtin`,
      and any `verus*` crate) are admitted by construction, since the spec
      declares them; every other proc-macro crate must be declared with
      `[[entry]] kind = "proc-macro" item = "<crate>"`. The *output* of a
      proc macro (the tokens it generates) is still not hashed; only the
      dependency's presence is checked.
    - **`include!`/`include_str!`/`include_bytes!`.** Detected by scanning the
      crate's own source text for the macro names, with `//`-to-end-of-line
      comments blanked first. `include!("x.rs")` (source text, carried by the
      source map) is not flagged; every other site is. A non-literal argument
      (`concat!`, a `const`) is recorded as `(non-literal)` and must still be
      declared. The scan does not understand block comments or string literals,
      so a macro name inside a block comment can produce a spurious site; that
      over-reports and never hides a real one.
    Declaring an input admits its *presence*; it does not make the tool able to
    see inside it. The earlier `NOTES.md` claim that the source hash "pins the
    source text" is now qualified exactly here and in item 12.

## Verdict

A fail-closed transitive audit is **feasible on this toolchain**. `rustc-dev`
is installed and a `rustc_driver` binary links against it (only
`librustc_driver.so` is needed at run time; it pulls in LLVM and the system
libs). The tool walks real post-monomorphisation MIR, resolves direct calls,
drop glue, fn-pointer mentions and vtables, and fails closed on every case the
spec names. On DIB it reports exactly the one `vec!` site through
`alloc_output_n`, plus the deallocation, and fails closed on 28 std edges, all
of which are enumerable and justifiable.

The residual blind spots are those in the list above; the largest are the
panic-path citations (admitted by human assertion, not proof) and the
"precompiled std body with no MIR" boundary, which is invisible by construction
and pinned only by the commit hash. The manual review burden for DIB release is
21 allowlist entries, dominated by 18 functions carrying panic edges. i686
cannot be audited at all here (no std); the audit-backed claim is x86_64-only
and must be written as such in the proof report. The prototype is not a
production tool: it has no per-function panic edge-count diff against a previous
report, the allowlist hash is computed with `sha256sum` rather than pinned in
the tool, and the header-format reader accepts a TOML subset only. Those are
hardening steps, not feasibility blockers.

## What this revision changed (2026-11-12)

Sol's review found a HIGH defect: authorization was decided from each
monomorphised instance's *first BFS parent*, so a shared allocation-capable
instance (one called by both an allowed guard and an unapproved helper) was
reported as allowed from the guard path alone, and the helper's incoming path was
never examined. The fix and its tests:

1. **Reproduction (n11).** `testcases/n11_shared_instance.rs` has a guard-like
   `guard::try_zeroed` (allowed) and two unapproved helpers,
   `unapproved_reserve` (`Vec<u32>::reserve_exact`) and `unapproved_grow`
   (`Vec<u8>::resize`). `RawVecInner::<A>::reserve_exact<Global>` is
   parameterised by the allocator only, so the guard and `unapproved_reserve`
   reach the SAME instance; `unapproved_grow` reaches the same `push_mut`
   growth instance any `resize`/`push` uses. With an allowlist admitting only
   `guard::try_zeroed`, **the pre-fix tool reported `Pass`, exit 0** (report
   archived at `reports/n11_shared_instance.txt` is the post-fix failure; the
   pre-fix pass is the "Before" column above).
2. **Fix.** `AuditEngine::authorize_allocations` records the whole edge set,
   enumerates every simple root-to-entry path for each allocator entry, and
   requires *every* path to be justified by an `alloc` entry whose `allow`
   matches the nearest audited-crate caller. Bounded and fail-closed.
3. **Tests.** n11 fails with 12 named `unauthorized-alloc-path` findings; the 11
   existing negatives still fail; c1_clean passes (fixture gained the
   SPEC-required `push` entry); c1_mutated fails; DIB passes at 91/277 with the
   same 21 entries.
4. **Profile check.** `detect_profile` reads `opt-level`, `debug-assertions` and
   `share-generics`; the report header prints them and the allowlist's recorded
   `profile` must match exactly, else `[allowlist-profile]` fails the run.
5. **Provenance.** `source_hash` is recomputed from the source map and compared
   (fail closed on mismatch or absence, printing the actual hash); `reviewer`
   must be non-empty and not `prototype-fixture` unless `--prototype`
   (`ALLOC_AUDIT_PROTOTYPE`) is set, which the report header prints as
   `mode: PROTOTYPE`.

The remaining blind spots above are unchanged and now stated for the new code as
well (items 11–14 are new).

## What the second revision changed (Sol's second read, 2026-11-12)

Sol's final read found five defects. Each fix and its control:

1. **(HIGH) Allowlist def-path matching had no component boundary.**
   `walk.rs::judge_path` accepted a caller when
   `c.ends_with(allow) || allow.ends_with(c)`, and `item_matches` did the same,
   so with `allow = "guard::try_zeroed"` an unapproved
   `unapproved_guard::try_zeroed` matched by suffix and a root-local
   `try_zeroed` matched by the reverse test. **Fix:** `allowlist::def_path_matches`
   matches whole segments: the two strings must be equal, or the shorter must be
   a prefix of the longer whose only remainder is one balanced generic-argument
   group (`<...>`). `crate::` prefixing on the caller side is accepted
   (`caller_matches`). **Controls:** `n12_suffix_lookalike` and
   `n13_root_lookalike`; each fails with `unauthorized-alloc-path` while the
   real `guard::try_zeroed` path stays `OK`, and each would pass under the old
   predicate (load-bearing). Fresh source hashes in
   `allowlist/n12_suffix_lookalike.toml`, `n13_root_lookalike.toml`.
2. **(MEDIUM) Provenance was checked only when a field was present.**
   Deleting a field yielded no mismatch; `vstd_version` was never compared; a
   recorded commit longer than the compiler's passed via `rec.starts_with(run)`.
   **Fix:** every `[meta]` field is required, each with a distinct fail-closed
   finding when missing; commit is compared exactly or as a ≥12-hex prefix of
   the full 40-hex hash (never a longer value); `vstd_version` is compared
   against the version actually linked (from `Cargo.lock` via
   `CARGO_MANIFEST_DIR` or `ALLOC_AUDIT_VSTD_VERSION`), failing closed when
   vstd is linked and the version cannot be determined, and requiring an
   explicit `vstd = "none"` for a non-vstd crate. **Controls:**
   `c1_commit_missing`, `c1_target_missing`, `c1_profile_missing`,
   `c1_commit_toolong`, `c1_commit_short`, `c3_vstd_ok` (pass),
   `c3_vstd_change`, `c3_vstd_missing`, `c1_vstd_undeclared`.
3. **(MEDIUM) One entry per item, chosen by `entries.iter().find`.**
   Two guard functions calling the same instance could not each have an entry.
   **Fix:** `judge_path` consults every `(item, allow)` entry that names the
   node and admits the path if any of them does. **Controls:**
   `c2_two_guards` (passes; fails if either entry is removed) and
   `n14_two_guards_plus_third` (fails, naming the third caller).
4. **(MEDIUM) The source hash's scope was undocumented and unchecked.** The
   report header now prints `source-hash scope: source-map text only`, and the
   tool fails closed on an undeclared build script, foreign proc-macro crate, or
   `include_*!` site (item 14 above records what the detection can and cannot
   see). **Controls:** `n15_include` and `bs_build_script`.
5. **(LOW) n11's `unapproved_grow` did not reach the guard's `push_mut`
   instance.** It called `resize`, which reaches `reserve -> grow_amortized`.
   **Fix:** `unapproved_grow` now fills a `Vec<u8>` with `push`, reaching the
   *same* `Vec::<T, A>::push_mut<u8, Global>` instance the guard's own `push`
   loop uses; the report now shows `unapproved_grow -> Vec::<T, A>::push_mut`
   paths. The reserve-exact half is unchanged.

## What the follow-up revision added (2026-11-12)

Four controls that the table above recorded as missing, the rerun of the pre-fix
reconstruction against the final `src/`, and the DIB re-confirmation. No tool
source changed: the four controls exercise existing fail-closed paths, so this
revision adds tests and fixtures only.

**Pre-fix evidence, archived honestly.** The original `walk.rs` was not archived
and is not in git history, so the pre-fix `Pass` could not be re-run from the
original binary. It is now reproduced from the described rule:
`prefix-reconstruction/reproduce.sh` patches the current tool to judge only the
single shortest root-to-entry path (the first-BFS-parent rule) and runs the
pre-fix N11 testcase; the result (`Pass`, exit 0) is archived at
`reports/n11_pre_fix.txt`. This is a *reconstruction*, labelled as such in the
report header, not the original run. The report was re-archived after the last
tool change: the last edit to `src/` was `src/allowlist.rs` at
2026-11-12 07:04:44 UTC, and the re-archived `reports/n11_pre_fix.txt` (Pass,
exit 0, 62 nodes / 111 edges / 4 allocations all `OK`, 0 findings) was produced
at 2026-11-12 07:18:05 UTC, 13m21s later. So the reconstruction was rerun
against the final `src/` and is not stale; it remains a reconstruction of the
old rule, not the original binary's output.

**Four controls added for the findings recorded as missing.** Each was absent
from the table above (the "missing-field fixtures" had 3, the vstd fixtures 3,
and no proc-macro control existed). Each fails for exactly its stated reason and
nothing else, and each is load-bearing:

- `c1_source_hash_missing` — the `source_hash` key is removed from `[meta]`, not
  merely wrong. The tool fails closed with `allowlist-source-hash-missing` (a
  distinct finding from `allowlist-source-hash`) and prints the hash the sources
  actually have. An absent key must not read as a match.
- `c1_reviewer_missing` — the `reviewer` key is removed from `[meta]`. The
  finding is `allowlist-reviewer` "reviewer is missing or empty (recorded
  None)". This is distinct from `c1_rev_empty` (key present, value empty) and
  from `c1_rev_fixture` (placeholder present without `--prototype`).
- `c3_vstd_undet` — a vstd-linked crate with a version recorded, but the linked
  version cannot be resolved: `run_tests.sh` unsets both
  `ALLOC_AUDIT_VSTD_VERSION` and `CARGO_MANIFEST_DIR`, and the fixture stands
  outside any crate tree, so no `Cargo.lock` walk can find a `vstd` package. The
  tool fails closed with `allowlist-vstd-undetermined` (`vstd_actual` printed as
  `None`) rather than trusting the recorded value.
- `n16_proc_macro` — a crate that links a foreign proc-macro crate `pmtoy`. The
  toy macro builds offline: it depends only on `proc_macro`, which rustc
  provides from the sysroot, so `run_tests.sh` compiles it with a bare `rustc`
  (no cargo, no crates.io). `pmtoy::passthrough!` generates the audited
  `n16_proc_macro` item, so the expansion is a real build input and not a
  linked-but-unused dependency. The tool fails closed with
  `undeclared-proc-macro` naming `pmtoy`; adding
  `[[entry]] kind = "proc-macro" item = "pmtoy"` makes the run pass, which is
  what makes the control live rather than a fixture that always fails.

**DIB positive control.** The live `../dib/dib_verus` crate is under active
rewrite by another writer and does not currently compile, so `dib_audit.sh` now
defaults to the pinned revision at `pinned-dib/` — the exact four source files
(`dib.rs`, `lib.rs`, `model.rs`, `model_vectors.rs`) whose hashes the allowlist's
`source_hash` covers. The control passes: release profile, 91 nodes / 277 edges,
3 allocator entries all `OK`, 0 findings, 0 unexplained leaves; allowlist
SHA-256 `1f9739e4d339ef2cbd8e4b41a54a72833fdf6196e9e18c16d3ae044eebcef2c8`
(unchanged), re-confirmed on this revision. Set `DIB_CRATE` to the live crate to
audit it directly once it builds. (The wrapper audits only the crate named
`dib_verus`; the pinned crate's `Cargo.toml` carries that name, which is what
makes the wrapper fire on the pinned revision.)

## Proposed SPEC.md edits (do not apply without owner review)

1. §6, *Targets*: state the audit-backed claim is x86_64 (already implied) and
   add that the audit is **release-profile** specific, since `share_generics()`
   defaults on for `opt-level <= 2`. As written, "run on every production
   configuration: release profile" is right, but the reason (share-generics) is
   worth naming, because a debug-config run fails for a reason unrelated to the
   code.
2. §6, allowlist: the file records the rustc commit, vstd version, target and
   reviewer. The prototype now also records and **enforces** a `profile` (the
   canonical `opt-level=…,debug-assertions=…,share-generics=…` string) and a
   `source_hash` over the crate's own sources, and prints the file's SHA-256 in
   the report. §6 line 360 says "the F1 prototype refuses commit and target
   mismatches, with fixtures for both; the profile field and its check are added
   with the production allowlist format" — that sentence is now out of date: the
   profile and source-hash checks are implemented, with fixtures
   (`allowlist/c1_profile_mismatch.toml`, `c1_bad_hash.toml`,
   `c1_bad_reviewer.toml`). §8's post-freeze-hardening list includes "Add the
   profile field and its mismatch check" (line 435) and would drop that item.
3. §6, panic edges: the current text says "allowed only with the justification
   'unreachable by Verus T1/Safe' on this function', citing the verified
   theorem. The audit records the edge count per function." The prototype
   confirms this is the right shape and shows the burden: 18 functions on DIB,
   2 distinct panic identities. Consider requiring the per-function edge count
   to be diffed against the previous proof report automatically, so a new panic
   edge is a hard failure rather than a line in a report.
4. §6, `finish_grow`: the text names `alloc::raw_vec::finish_grow` as the
   canonical "precompiled body without MIR". On this toolchain it is *not* a
   leaf in the configurations run (its body is available); the canonical leaf
   observed is `alloc::raw_vec::handle_error`. Reword to "such as" and note the
   set is toolchain-determined, not fixed.
5. §6, i686: "the tool cannot compile the core for that target" is exactly what
   was observed (`E0463: can't find crate for std`). Keep; add that the tool
   itself is target-agnostic and would run unchanged once std is installed.
6. §6, allocation-capable items: the text says the `push` entry's contract names
   its growth path and why it is not taken. The strict all-paths check makes this
   mandatory in more cases than the text implies: *any* allocation-capable
   instance a guard path reaches — including `Vec::push_mut` behind a
   reserve-then-push fill — is a path that must be justified, because the path
   through `push_mut -> grow_one -> grow_amortized -> finish_grow` exists in the
   MIR whether or not the fill can actually regrow. The text is consistent with
   this (`Vec::<T>::push: only from alloc_guard functions`), but it is worth
   stating that the surface is the reachable instance, not the reachable
   *allocator call*: a helper and a guard that call the same monomorphised
   instance make every pair of callers relevant to that entry.
7. §6, *Authorisation on every path*: added in the concurrent SPEC revision (see
   the note below), and this prototype now implements it. The needed edit is the
   reverse of item 7's first draft: §6 line 336 says the prototype "is the
   reference for the edge resolution below, but not for authorisation: its walk
   keeps one BFS parent per monomorphised instance (`walk.rs:253–256`), so it
   does not meet the every-path rule, and gate F1 is not yet passed". That is no
   longer true: the walk records the whole edge set and `authorize_allocations`
   judges every simple path. §8 F1 ("NOT YET PASSED", line 426) should flip to
   PASSED, with the same-instance control it names now provided as
   `testcases/n11_shared_instance.rs`, and the DIB allowlist SHA-256 updated to
   `1f9739e4…` (the fixture gained `profile` and `source_hash`). This revision
   was made by another writer while this fix was in progress; I did not edit
   SPEC.md.
8. §8 F1: state that the reviewer must sign a per-(target, profile) allowlist,
   and that the DIB positive control is expected to report exactly one
   allocation site via `alloc_output_n` plus deallocation, which is what the
   prototype observed. Update the recorded DIB allowlist SHA-256 from
   `295c8bf4…` to `1f9739e4…`. Also state that the fixture's
   `reviewer = "prototype-fixture"` now requires the `--prototype` flag, which
   the report header prints.

Note on the concurrent SPEC revision: `SPEC.md` grew from 72295 to 78185 bytes
during this work, adding §6 *Authorisation on every path* (which requires exactly
the fix implemented here), the *Targets* profile paragraph, and the reconsidered
F1 entry that lists the same-instance control as the missing evidence. My reading
of §6's new text is that this prototype now satisfies it; the remaining SPEC edits
above are the F1 status flip and the hash update, and the allowlist `profile` /
`source_hash` / reviewer-flag format is already implemented here.

Additional SPEC.md edits needed after Sol's second read (do not apply without
owner review):

9. §6, allowlist identity: the text names the allowlisted caller and item but
   does not fix the *matching rule*. State that a def-path comparison is
   whole-segment (equal, or the shorter plus exactly one generic-argument
   group), never a substring/suffix match, so a lookalike module or a root-level
   name cannot be admitted; and that one item may carry several `[[entry]]`
   blocks, one per admitted caller, when two guard functions reach the same
   monomorphised instance.
10. §6, provenance: state that *every* recorded meta field (commit, target,
    profile, source_hash, vstd version, reviewer) is required and that a missing
    field is a fail-closed finding; that the commit is compared exactly or as a
    prefix of at least 12 hex characters of the compiler's full hash only; and
    that the vstd version is compared against the version actually linked
    (failing closed when it cannot be determined), with `vstd = "none"` marking
    a non-vstd crate.
11. §6, source-hash scope: add a sentence that the hash is over source-map text
    of the crate's own files only, and that `include_bytes!`/`include_str!`
    payloads, build-script output, proc-macro output and dependency text are not
    covered; require the audited crate to declare each such input (build script,
    foreign proc-macro crate, `include_*!`) in the allowlist, and require the
    build to run under cargo so `OUT_DIR` (the build-script signal) is set.
12. §8 F1: the pre-fix N11 evidence is now archived at
    `reports/n11_pre_fix.txt` as a labelled reconstruction (the original binary
    was not archived). If the SPEC's evidence rules require the original run,
    say so; otherwise accept the reconstruction and cite it.
