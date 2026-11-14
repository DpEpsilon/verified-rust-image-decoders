# Production allocation audit (alloc-delta SPEC §6, §8, §9)

**Status, 2026-11-14: the tool is built and every control behaves as expected, but no core passes, and no "proved" or "audit shows" claim may be made.** Each core's production run fails on at least `allowlist-reviewer`: the four allowlists are unsigned drafts, and signing them is a human's job. (ICNS and ICO also failed on `Reserved::<T>::push` lacking `#[inline(never)]`, until ERRATA E10 added it.) ICO additionally fails its dependency contract, because DIB's report does not pass. **Prerequisite 1 is not claimed as met.** Sol's adversarial review (2026-11-13) found two authorisation holes in the tool, a callback cycle hiding the unauthorised caller and caller-side generic arguments being dropped, plus three process holes. All five are fixed, each with a negative control. Sol's re-check then found a sixth hole. An admitted std item that calls back into crate code lent its authorisation to allocations made directly inside that callback. That hole is fixed too (ERRATA E9). None of these fixes has been re-reviewed. Of the other prerequisites, 2 is partly met, 3 is not met, and 4 is met for DIB only; details below. Deviations from frozen SPEC §6 are recorded in `../ERRATA.md` E7. E8 records the correction to SPEC §6's simple-path argument, and the **scope reductions** the drafts rely on: the excluded `Decoded::clone` and `derive(Debug)` roots, and the declared generic guard exports.

Evidence for this revision is in `runs/`:

| Run | What it is | Result |
|---|---|---|
| `runs/20261114T115040Z/` | PRODUCTION: all four cores after the E15 restoration of `try_parse`'s rejection clause (ERRATA E15; ICNS's `source_hash` changed, so its allowlist was regenerated) | DIB, TGA and ICNS only `allowlist-reviewer`; ICO also `dependency-contract`. Same entry counts (64/58/50/42) and ICNS walk (351/638) as E14, the other three unchanged |
| `runs/20261113T081241Z/` | FINAL: production audit after the re-check fix (E9) | same findings: DIB and TGA only `allowlist-reviewer`; ICNS also `guard-inline-never`; ICO also `guard-inline-never` and `dependency-contract` |
| `runs/20261113T081403Z-controls/` | FINAL: 98 controls | 98 as expected, 0 failed, 98 result rows for 98 selected |
| `runs/20261113T081444Z-inventory/` | FINAL: inventory, plus the x9, x10 and x26 controls | four cores PASS, five controls as expected |
| `runs/20261113T065905Z/` | production audit after the review fixes | same findings as before the fixes: DIB and TGA only `allowlist-reviewer`; ICNS also `guard-inline-never`; ICO also `guard-inline-never` and `dependency-contract` |
| `runs/20261113T065917Z-controls/` | 94 controls after the review fixes | 94 as expected, 0 failed, 94 result rows (the runner requires exactly 94) |
| `runs/20261113T065949Z-inventory/` | inventory after the review fixes, plus the x9, x10 and x26 controls | four cores PASS, five controls as expected |
| `runs/20261113T063604Z/` | BEFORE the review fixes: production audit of all four cores (`run_audit.sh`) | exit 1; DIB and TGA: only `allowlist-reviewer`; ICNS: also `guard-inline-never`; ICO: also `guard-inline-never` and `dependency-contract` |
| `runs/20261113T063620Z-diagnostic/` | the same without allowlists (`--diagnostic`) | exit 1 by design (`allowlist-absent`); the full reachable set, for review |
| `runs/20261113T063626Z-controls/` | BEFORE the review fixes: 84 controls (`controls/run_controls.sh`) | 84 as expected, 0 failed (`CONTROLS.tsv`) |
| `runs/20261113T063902Z-inventory/` | BEFORE the review fixes: independent build-input inventory of the four cores, plus the x9/x10 controls (`inventory/run_inventory.sh`) | four cores PASS, four controls as expected |
| `runs/20261113T063657Z-inventory-SUPERSEDED-compare-defect/` | an earlier inventory run, kept as a record | superseded: `compare.py` counted the tool's read of its own allowlist as untracked (`SUPERSEDED.txt`) |
| `runs/20261113T064059Z/`, `runs/20261113T064141Z/` | `tga/tools/audit_alloc.sh` and `dib/tools/audit_alloc.sh` calling the production audit | as in the first row, for one core each |

## Reproduce

All commands run from `alloc-delta/audit/`. Build output goes to `/tmp/claude-0/alloc-audit` (override with `ALLOC_AUDIT_TARGET`), which is regenerable. Everything builds with `-j1`, and every step has a `timeout`. Verus is not used.

```
./run_audit.sh                    # production audit of dib tga icns ico; exit 0 only if all PASS
./run_audit.sh --diagnostic       # without allowlists, for drafting; always nonzero
./run_audit.sh tga                # one core
controls/run_controls.sh          # the 98 controls; nonzero if any control misbehaves, none is selected, or the full suite is not 98
inventory/run_inventory.sh        # independent inventory of the four cores + x9/x10
controls/make_fixtures.sh         # regenerate control fixtures (after a testcase changes)
tools/draft_all.sh                # regenerate the DRAFT allowlists and PINS.sha256 (discards any review)
```

The tool is a `rustc_driver` program built against the pinned rustc 1.99.1's `rustc-dev` component, with `#![feature(rustc_private)]` and `RUSTC_BOOTSTRAP=1`, as the F1 prototype was. `run_audit.sh` builds it and passes it to cargo as `RUSTC_WRAPPER`. Every compiler invocation that is not a crate named in the run's plan, including version probes, dependencies, build scripts and proc macros, is handed unchanged to the real rustc by `exec`. The audited crates (each core, and for ICO also the `dib_verus` it links) are compiled by the tool itself, in the release profile with `--target x86_64-unknown-linux-gnu`. So the audited compilation is the one that produces the artifact.

Each core's `tools/audit_alloc.sh` keeps its source lint and then calls `run_audit.sh <core>`, unless the script was given a source-path argument, as the ICO mutation runner gives one. `EXPECTED_MOD_SHA` and `EXPECTED_SITES_SHA` are unchanged.

## What the tool checks

Each item below is a check the tool performs, and each names the control that fails if the check stops working.

1. **Roots.** The roots are every exported function: free functions, inherent and trait-impl methods that are reachable under rustc's effective visibilities, plus `reachable_non_generics`. Every `required_roots` entry, such as the `try_*` functions, must be among them (`missing-root`). An exported function can be left out only by a `root-exclude` entry with a citation; the drafts exclude the `derive(Debug)` impls and `Decoded::clone`, which are owner decisions (E7). An exported generic function fails with `generic-pub-fn` unless a `generic-export` entry declares it, and a declared one must be reached at some instantiation (`generic-export-unreached`). Controls: `x15_generic_export`, `c1_mutated` (shows `missing-root`).
2. **Walk.** The walk covers post-monomorphisation MIR, `instance_mir` instantiated under `TypingEnv::fully_monomorphized()`, which is the MIR codegen uses. It follows calls (`Instance::try_resolve`, where `Ok(None)`/`Err` fail closed), drop glue (through the `DropGlue` lang item), fn items used as values or reified, vtable methods and vtable drop glue at unsizing sites, `SymFn` asm operands and intrinsic fallback bodies. It walks through every dependency body rustc has MIR for, including `core`, `alloc` and `vstd`.
3. **Node identity.** Nodes are keyed by `Instance` (DefId, generic arguments and instance kind), never by a printed string. Allowlist values are resolved to DefIds before matching. A local item must be written `crate::…`. A value that names two items, such as a local `guard::try_zeroed` and an external crate `guard`, is `allowlist-ambiguous` and matches neither. Controls: `x1a_ambiguous` and `x1b_extern_unauthorized` (defect 1); `n12`, `n13`.
4. **Undoing MIR inlining for attribution** (not in the SPEC; added). Optimised MIR has callees inlined. The tool reads rustc's inlined-scope records and adds the chain `owner -> inlined callee -> …` as edges, so a call is attributed to the function whose source contains it. Without this, a helper inlined into an admitted guard function would be admitted as the guard. Control: `x14_inlined_helper`.
5. **Every-path authorisation.** The allocator sinks are matched by exact canonical path in the sysroot `alloc` crate: `__rust_alloc`, `__rust_alloc_zeroed`, `__rust_realloc`, `__rust_dealloc`, `handle_alloc_error`, `__rust_alloc_error_handler`, `raw_vec::handle_error` and `raw_vec::capacity_overflow`. Any foreign symbol shaped like an allocator, `__rust_*alloc*` or `malloc`/`calloc`/`realloc`/`free`/`posix_memalign`/`aligned_alloc`/`mmap`, is also a sink, never an ffi leaf (`x25_extern_malloc`). Authorisation is decided by a fixpoint over states, not by enumerating paths. A state is a node plus the most recent crate-local function on the walk plus the verdict of the last allowlisted allocation-capable item on the walk (none, admitted, admitted for deallocation only, or not admitted). The fixpoint is explored from every root. Every walk is covered, including walks that repeat a node, so a callback cycle cannot hide a caller (Sol finding 1, control `x27_callback_cycle`). Entering any crate-local function resets the verdict, so a callback that pinned std makes from inside an admitted item (`to_vec`, `clone`, iterator adaptors) must earn its own admission for anything it allocates (E9; controls `x33_callback_direct_alloc` fails and `x33b_callback_no_alloc` passes). On a walk, the last node an `alloc` entry names (by DefId, and by generic arguments when the entry writes them) is judged against the most recent crate-local function at or before it. That function must be the entry's `allow`: DefId equality, and, when the `allow` value writes generic arguments, equality of the caller's printed arguments (Sol finding 2, control `x28_generic_caller_args`; leaf and panic entries use the same comparison, control `x34_leafpanic_caller_args`). `allow = "*"` admits deallocation sinks only (E7). States are capped at 5,000,000 (`state-bound`, fail closed). Each unauthorised state is reported with a witness walk. Controls: `n1`–`n9`, `n11`, `n14`, `c1_mutated`, `x23_star_alloc`, `x27`, `x28`, and the eleven seeded-core variants.
6. **Fail-closed leaves.** Leaves are no-MIR items, intrinsics and ffi. Each needs an entry with a `contract` and a `citation`. A diverging leaf (a panic-family function) needs one entry per calling function. Compiler panic edges (`Assert`, `UnwindTerminate`) need one entry per (function, panic kind). Indirect calls (`fn-ptr-call`), virtual calls (`virtual-dispatch`), unsizing to `dyn` (`vtable-unsize`), inline asm, statics and global asm in codegen units are findings, as are uninstantiable bodies and non-monomorphic inlined scopes. Controls: `n6`, `n7`, `n7b`, `n10`, `x19_const_fnptr`, `x20_const_dyn`, `x21_inline_asm`.
7. **Cross-check against rustc's own collector.** Every `MonoItem::Fn` in the local codegen units must have been reached by the walk from the roots or the excluded roots (`mono-item-unwalked`). `x19_const_fnptr` shows the check catching a function that is reachable only through a pointer in a `const` table.
8. **Crate origin (§9).** Every reached instance must come from `core`, `alloc` or `compiler_builtins` loaded from the sysroot, from `vstd`/`verus_builtin` (bound by crate hash in `[meta]`), or from a crate admitted by a `kind = "crate"` entry. An admitted crate also needs a `dependency` entry naming its own passing report with the same crate hash. Findings: `std-instance`, `foreign-crate-instance`, `allowlist-ambiguous-crate`. Controls: `x11`, `x12_foreign_crate`, `x12b_admitted`, `x12c_ambiguous_crate`.
9. **Provenance (`[meta]`).** Every field is required, and each missing field is its own finding. The fields are format, crate name, rustc commit (exact, or a ≥12-hex prefix of the full hash), target, profile (`share-generics` is refused), features, source hash, vstd version, reviewer (`prototype-fixture` only with `--prototype`), and in production mode `vstd_svh`, `vstd_source_checksum` and `verus_builtin_svh`. `cfg(test)` and `cfg(verus_only)` are refused. Controls: the 42 ported prototype controls, plus `x6`, `x7`.
10. **vstd bound to the linked artifact (defect 5).** The tool takes the crate hash of the vstd rlib it links. It locates the source tree that rlib was compiled from (its crate-root file) and checks that tree byte for byte against the registry `.crate` archive whose SHA-256 is the `Cargo.lock` checksum. It records the artifact hash. `ALLOC_AUDIT_VSTD_VERSION` is refused outside `--prototype`. Controls: `x6_vstd_override`, `x7_vstd_artifact`.
11. **Build inputs, compiler-resolved (defects 2, 3, 6, 7).** A file loaded by the compiler that is not a module file (`include!`, `include_str!`, `include_bytes!`, from the source map and dep-info) must be declared with its hash. So must an `env!`/`option_env!` read (dep-info), the build script (`OUT_DIR`, or cargo's package rule: `build.rs` or `package.build`), and each proc-macro crate (classified by crate kind and bound by artifact hash). `*` is refused in build-input declarations. A non-proc-macro dylib is `dylib-dependency`. Controls: `x2_include_after_url`, `x2b_renamed_import`, `x2c_foreign_macro`, `x2_declared`, `x3_wildcard_build_input`, `x4_verus_prefix`, `x8_real_build_script`, `x8b_build_script_declared`, `x13_dylib_not_proc_macro`, `x24_env_*`, `n15`, `n16`, `bs_build_script`.
12. **No allowlist (defect 4).** `allowlist-absent`, and the header says `diagnostic: no allowlist`. Control: `x5_no_allowlist`.
13. **Per-core shape.**
   - Guard call-site counts (`guard-calls` entries): DIB 1 `try_zeroed`; TGA 1 `try_zeroed`; ICNS 1 `try_with_room`, 5 `try_zeroed`, 1 `try_copy`; ICO 1 `try_with_room`, 1 `import_dib`, 1 call to `dib_verus::try_decode_with_steps_logged`.
   - Allocation-capable `alloc_guard` functions must carry `#[inline(never)]` (`x16_guard_inline_never`).
   - Crate code may not call `mem::forget`, `Box::leak` or `ManuallyDrop::new`, and `Rc`/`Arc` may not be reached (`x17_forbidden`).
   - An allowlist entry that admits nothing is `allowlist-unused-entry` (`x22_unused_entry`).

The seven prototype defects are SPEC §8 items 1–7. Their fixes and controls are rows 3, 11, 10 and 12 above, and defect 7 is `x13`. The 42 prototype controls are ported into `controls/testcases/legacy/`. They keep the same items, compiled `no_std` so that the crate-origin rule does not fail them for an unrelated reason, and their fixtures were rewritten in the new format. All 42 keep their expected verdicts, and each negative control is checked for its specific finding tag and for report text naming the seeded function, not only for a nonzero exit. A fixture that fails to parse is itself a failed control.

## Per-core results (run `20261113T063604Z`)

| Core | Instances / edges | Crates reached | Sinks | Allowlist entries (panic-edge entries) | Findings |
|---|---|---|---|---|---|
| DIB | 421 / 712 | alloc 65, core 170, dib_verus 178, verus_builtin 7, vstd 1 | 5, all judged OK | 64 (8) | `allowlist-reviewer` |
| TGA | 367 / 627 | alloc 64, core 163, tga_verus 134, verus_builtin 5, vstd 1 | 5, all OK | 58 (14) | `allowlist-reviewer` |
| ICNS | 345 / 633 | alloc 85, core 204, icns_verus 53, verus_builtin 3 | 5, all OK | 53 (12) | `allowlist-reviewer`, `guard-inline-never` (`Reserved::<T>::push`) |
| ICO | 329 / 542 | alloc 72, core 180, dib_verus 7, ico_verus 64, verus_builtin 6 | 5, all OK | 45 (4) | `allowlist-reviewer`, `guard-inline-never`, `dependency-contract` (DIB's report does not PASS: unsigned) |

In all four cores, rustc's collector codegens no function the walk did not reach (`mono-items: … 0 not reached`). Every allocator path ends at an admitted guard function, from `Vec::try_reserve_exact`/`resize`/`push` (pinned to their `u8` or `Entry` instantiations) or at `RawVecInner::deallocate`. The tool asserts this only relative to the unsigned allowlist; see the trusted base.

## Trusted base, and what a pass would and would not mean

A future PASS, with a signed allowlist, would establish this: in the release x86_64 compilation that produced the artifact, every path in the MIR call graph from every exported function that is not excluded to an allocator sink passes, at its last allowlisted allocation-capable item, through an admitted `alloc_guard` function. Every edge the tool could not follow is either a reviewed leaf entry or a finding. That claim rests on the following.

- **rustc 1.99.1 (`58cf8f9c3`) and its std.** That the MIR rustc hands the tool is what codegen uses, that rustc's inlined-scope records are accurate, and that `try_resolve` resolves correctly. LLVM after MIR is not audited, which is the binary scan's job (SPEC §6 item 4, not built here).
- **std behaviour, A-STD-1..4** (SPEC §3; E6 for A-STD-4). The tool shows that `Vec::push`/`resize` reach `finish_grow` and `handle_error`. Only A-STD-1/2/4 and the guard's verified preconditions say those growth paths are not taken. Each `alloc` entry's contract cites them; the tool does not check them.
- **The allowlist, and its human reviewer, who does not exist yet.** Each no-MIR leaf's `contract` is a human assertion, and so is each panic edge's unreachability citation (T1/`Safe'`). The drafts' citations were generated by rule, by `tools/draft_allowlist.py`. A reviewer must confirm, for each cited function, that it is Verus-verified and not `external_body`; the tool does not do the ghost-reading checks of SPEC §6 item 2. The allowlists are hash-pinned (`allowlist/PINS.sha256`, checked by `run_audit.sh`), and each report prints its allowlist's hash.
- **The `root-exclude` decisions.** The drafts exclude every `derive(Debug)` `fmt`, which formats through a caller-supplied `dyn fmt::Write`. They also exclude `Decoded::clone` in DIB, TGA and ICNS, which allocates infallibly when a caller clones a result. These are owner decisions recorded in E7; the frozen REVIEW-SOL.md "Third revision" already names the roots as the `try_*` and `*_logged` exports only.
- **The dependency contract for ICO.** ICO's walk stops at DIB's no-MIR entry points. DIB's own report for the same crate hash, produced in the same run, carries DIB's part.

What a pass would not show:
- that A-STD holds;
- that panics are unreachable;
- anything about the generic exports (`try_with_room`, `Reserved::push`/`into_vec`) at instantiations a downstream crate makes;
- anything on i686 or any target other than x86_64-unknown-linux-gnu; i686 is installable, not audited;
- anything about the debug profile;
- anything about LLVM-level or binary-level code;
- that the allocator is glibc `malloc` (allocator identity rests on A-STD and the proof report);
- anything about allocation inside a no-MIR std leaf whose contract a reviewer got wrong;
- anything about std's own callers of a shared instance: authorisation constrains crate callers only.

## SPEC §8 prerequisites

| # | Prerequisite | Status |
|---|---|---|
| 1 | Production audit tool, every-path authorisation and profile refusal kept, all 7 known defects fixed, passing §6's required controls plus the prototype's | **Not claimed.** The tool does everything in *What the tool checks*, and all 98 controls pass. Sol's review found that the earlier every-path authorisation missed callers hidden by cycles and ignored caller-side generic arguments. Both are fixed (`x27`, `x28`). The re-check found that the last-allowlisted-item rule let an admitted std item authorise crate callbacks; that is fixed too (`x33`, E9). None of these fixes has been re-reviewed. Deviations are in ERRATA E7, E8 and E9; E8's description of the authorisation rule is superseded by E9. Controls `x1`–`x8`, `x11`, `x12` (with the ambiguity variant) and both missing-key fixtures pass, as do the 42 prototype controls; `x9`/`x10` are process controls (row 2). |
| 2 | Independent build-input inventory: listed with hashes, rustc dep-info, traced build-script and proc-macro reads with unrecorded ones denied, build-unit records, artifact and path-dependency hashes, mechanical comparison with the report | **Partly met.** `inventory/capture.sh` builds from an empty target directory under `strace -f` (execs, opens, chdir, clone) with `env -i` reduced to the declared variables and an `LD_PRELOAD` getenv logger. `compare.py` first rejoins strace's `<unfinished ...>`/`<... resumed>` halves per task, and fails closed on an unmatched half (`strace-unmatched`) or an open record it cannot parse (`unparsed-open`), so every successful read open is classified (Sol finding 4, control `x26_concurrent_untracked_read`; write-only opens are not inputs and are skipped). It also classifies every env read, hashes every build unit's outputs and build-script `OUT_DIR` files, and compares the audited crate's dep-info inputs, rustc command line and `--extern` artifact hashes with the report from the same build. `x9`/`x10` block naming the file and the variable, and pass once both are declared; `x26` blocks on an undeclared FIFO whose open strace split. **Gaps:** (a) undeclared file reads are detected and block the report, but are not denied; §6 permits block-with-finding as an alternative. (b) Env reads made inside a proc macro are attributed to the rustc process, not to the macro, because stack attribution deadlocked in this sandbox; the read still blocks. (c) Absent-variable probes are admitted by a reviewed name list (`KNOWN-ENV-PROBES.txt`) that applies to any process. (d) `stat`, `readlink`, `getdents` and `mmap`-without-open are not traced. (e) The comparison with the report covers the audited crate's own inputs and linked artifacts; dependency source trees are checked through dep-info classification and vstd's archive binding, not compared hash by hash against a report. |
| 3 | Production allowlist signed by a named human, all `[meta]` fields, passing without `--prototype` and without the override | **Not met.** The four allowlists are complete drafts (every `[meta]` field, production mode, no override) with `reviewer = ""`. The tool refuses them with `allowlist-reviewer`, which is their only finding for DIB and TGA. |
| 4 | DIB's production `try_zeroed` path with `#[inline(never)]` on every exec function of the guard | **Met for DIB's allocation-capable guard function** (`try_zeroed` has it; the tool checks this). Not met by the letter of SPEC §3 for the `OutOfMemory::site`/`elements` accessors in the pinned `mod.rs`, which do not allocate. ICNS and ICO fail the tool's check on `Reserved::<T>::push`, whose `sites.rs` is pinned and was not edited. |

## Where the tool could miss an allocation path, and how each case is tested

| # | Way an allocation could escape | Handling | Test |
|---|---|---|---|
| 1 | Call through a function pointer, including one read from a `const`/`static` | `fn-ptr-call` at every `FnPtr` callee; candidate casts walked; mono cross-check catches unreached bodies | `n6`, `x19` (also trips `mono-item-unwalked`) |
| 2 | Trait-object call, vtable built elsewhere | `InstanceKind::Virtual` → `virtual-dispatch`; any unsizing to `dyn` → `vtable-unsize` | `n7`, `n7b`, `x20` |
| 3 | Allocation in a `Drop` impl or drop glue | `Drop` terminators resolved through the `DropGlue` lang item and walked | `n9`, seeded `box_new`/`push` (drops) |
| 4 | Allocation in a helper, closure or iterator adaptor | transitive walk; closures resolved through `Fn*` impls | `n5`, `n8`, seeded `collect` |
| 5 | Helper inlined into an admitted guard function | inlined-scope attribution (check 4) | `x14` |
| 6 | Unapproved caller of the same monomorphised instance as the guard | every walk judged by state fixpoint; DefId-keyed nodes | `n11`, `n12`, `n13`, `n14`, `x1b` |
| 6a | Unapproved caller hidden behind a cycle (a callback re-entering an admitted instance) | state fixpoint covers non-simple walks | `x27_callback_cycle` (PASSed before the fix: `controls/sol-repro/cycle-admitted.txt`) |
| 6b | Admission of one instantiation of a generic caller widened to all | caller-side generic arguments compared | `x28_generic_caller_args`, `x28b` (PASSed before the fix: `controls/sol-repro/generic-admitted.txt`); leaf and panic entries use the same comparison: `x34_leafpanic_caller_args` |
| 6c | Crate callback invoked by an admitted std item inheriting that item's admission | verdict reset on entry to every crate-local function | `x33_callback_direct_alloc` (PASSed before the fix: `controls/sol-repro/recheck/callback-admitted.txt`), `x33b` |
| 7 | Namesake external crate admitted by a path string | DefId resolution; ambiguity refused | `x1a`, `x1b` |
| 8 | Allocation inside a precompiled std body without MIR | leaf needs a reviewed entry; the drafting rules never admit a non-diverging no-MIR leaf | `n10`-style unexplained leaves; reviewer risk remains (trusted base) |
| 9 | Direct `__rust_alloc`/`malloc` FFI from crate code | allocator-shaped foreign symbols are sinks | `x25` |
| 10 | Wildcard admission | `*` admits deallocation sinks only; never in build inputs | `x23`, `x3` |
| 11 | Entry admitting an item at an unintended instantiation | the cores' entries pin generic arguments (`<u8,Global>`, `<Entry,Global>`) | no dedicated control (a stated gap); generic-argument matching itself is exercised by every core run |
| 12 | Exported function not reached from the roots, or a root left out | roots are all exported functions plus `reachable_non_generics`; exclusions explicit; rustc's collector cross-checked | `c1_mutated` (`missing-root`), `x19` |
| 13 | Generic export instantiated downstream | not auditable here; declared and reported | `x15`; a stated gap |
| 14 | Inline/global asm, statics, thread-locals | findings | `x21`; `static-item` and `global-asm` from the collector |
| 15 | Source or build input changed after review | source hash, include and env declarations with hashes, build script and proc-macro artifact hashes, independent inventory | `c1_hash_bad`, `x2*`, `x8*`, `x24*`, `x9`, `x10` |
| 16 | A different vstd/std/compiler | commit, vstd crate hash and archive binding, sysroot origin of core/alloc | `c1_commit_*`, `x7`, `x11` |
| 17 | Debug-profile or share-generics build | profile pinned; share-generics refused | `c1_prof_mismatch`, `c1_share_generics` |
| 18 | State explosion | the fixpoint is capped at 5,000,000 states and fails closed (`state-bound`) | not triggered by any control or core |
| 18a | Production run quietly in prototype mode, with narrowed roots or an overridden vstd | `run_audit.sh` refuses `ALLOC_AUDIT_*`, `RUSTC_WRAPPER`, `RUSTFLAGS` and similar variables if present, and accepts a report only with `mode: PRODUCTION`. The tool accepts only the value `1` for its switches and exits 2 otherwise. A dependency report must also be `mode: PRODUCTION` | `x29_prototype_flag_value`, `x30_runner_refuses_prototype`, `x30b_runner_refuses_roots`, `x31_dependency_not_production` |
| 18b | A control silently not run (for example, its seed generator broke) | a seed failure is a FAILED control; the runner requires at least one selected control, one result row per selected control, and exactly 98 for the full suite | `x32_failing_seed_generator_counted` (a nested run with a deliberately broken seed anchor), `x35_empty_selection_fails` |
| 19 | Panic path allocating in the panic handler | panic edges admitted only per function with a citation; the tool does not analyse the handler | trusted base |
| 20 | LLVM-introduced calls, and anything after MIR | not covered | binary scan (SPEC §6 item 4) not built |

The seeded-core controls are uneven: eight for TGA, one for DIB, one for ICNS and none for ICO, plus an unseeded copy of TGA as a baseline whose only finding is `allowlist-reviewer`. Each seed calls a plain function from the core's guard-reservation site, using `vec![0; n]`, `Vec::with_capacity`, `Box::new`, `push`, `String::from`, `format!`, `to_vec` or `collect`. Each variant must fail with `unauthorized-alloc-path` on a path naming `seeded_alloc`.

## Layout

- `tool/`: the rustc_driver tool (`src/{main,config,walk,allowlist,inputs,names,report}.rs`).
- `run_audit.sh`: the production run.
- `allowlist/x86_64-unknown-linux-gnu/release/<crate>.toml`: the DRAFT allowlists.
- `allowlist/PINS.sha256`: the allowlist pins.
- `allowlist/decisions/<core>.toml`: the hand-written alloc, root-exclude, generic-export, guard-calls and dependency entries.
- `tools/draft_allowlist.py`, `tools/draft_all.sh`: the drafting aids, whose rules are listed in the script header.
- `controls/`: testcases (`legacy/`, `x/`), fixtures and their generators, cargo-based controls (`cargo/x8`, `x9`, `x10`), the seeding script and `run_controls.sh`.
- `inventory/`: `capture.sh`, `compare.py`, the getenv shim, `KNOWN-ENV-PROBES.txt` and `run_inventory.sh`.
- `runs/`: evidence.
