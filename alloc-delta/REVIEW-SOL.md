# Sol review of the fallible-allocation delta: findings and resolutions

Status after the twelfth revision: **READY FOR OWNER FREEZE**, not frozen. Sol's last short read of the ninth revision found one blocking wording defect, corrected under "Tenth revision (inventory completeness)". The eleventh revision folds in the no_std requirement. Sol's short read of it is complete; its findings are applied under "Twelfth revision (wording)". The owner approved the freeze on 2026-11-12 pending the no_std work; the coordinator records the freeze with hashes.

An independent Sol review of the first draft of `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md` raised nine findings: four HIGH, four MEDIUM, one LOW. All nine are resolved in the revised texts. The largest change is to the theorem shape: the reservation log is now sealed in the guard module and records each reservation's actual result, so every `OutOfMemory` is tied by verification, not by the audit, to a reservation that failed. The second is an honest accounting of the frozen-named functions: they are now unverified wrappers, and their verified theorems are retired rather than "kept". Line references in the first column are to the first draft.

## 1. HIGH: the OOM theorem did not tie `OutOfMemory` to a failed reservation

The first draft's ghost trace recorded only `(site, count)` for each request. An implementation could reserve successfully and then return `OutOfMemory`, and clause 3 would still hold; the claim at `SPEC.md:48` that clause 3 rules this out was wrong. The audit strips ghost code, so it could not close the gap either.

Resolution (`SPEC.md` §2, §3; `THEOREMS-ALLOC-DELTA.md` §0):

- The log is a `tracked` `AllocLog` whose ghost field is private to `alloc_guard`. Only `new_log()` and the guard's reservation functions create or extend it, and each entry records `(site, k, ok)` with `ok == (r is Ok)`.
- `OutOfMemory`'s fields are private to `alloc_guard`, so only a guard call returning from `try_reserve_exact`'s `Err` arm constructs one (plus ICO's `from_dib`, which wraps DIB's).
- `G` clause 1: a non-OOM result has every entry `Ok` and `req(L) == Req(x)`. Clause 2: an `OutOfMemory` result has every entry but the last `Ok`, the last **not** `Ok`, and that entry's `(site, k)` equal to the returned value. This covers every concrete return site, whichever path reaches it.
- New progress theorem `Prog`: if every reservation returns `Ok`, the result is `lift(M(x))` and `req(L) == Req(x)`.
- Stated residual: the guard sets `ok` from its own result, and vstd does not constrain `try_reserve_exact`'s result, so "guard `Err` iff `try_reserve_exact` `Err`" rests on the hash-pinned guard text. Mutations A2/A2′ name the fault-injection test and the hash check, not the verifier, as their gates. New mutations A17 (stashed `o` after `Ok`), A18 (second `new_log`) and A19 (swallowed `Err`) target the new clauses.

## 2. HIGH: unverified wrappers retire the frozen public contracts

The first draft said that every existing obligation "stays in force" for the frozen names. Once those names are wrappers outside `verus!`, their in-Verus contracts no longer exist, and neither do the verified callers that relied on them (including ICO's cross-crate use of DIB's `decode_with_steps` postcondition). The draft also omitted ICO's public `parse_with_steps`.

Resolution (owner default D1: wrappers stay unverified, terminate via `handle_alloc_error`, reduced assurance boundary):

- `SPEC.md` §1.2 states that the verified API theorems on the frozen names are retired, and lists them: DIB T1, T3, T9–T13 and the policy-delta rows; TGA T1, T7–T11; ICNS T1, T3–T8, T10–T13; ICO T1–T11. Helper theorems stay verified.
- The replacement is the verified `try_*` theorem plus an unverified argument from the wrapper's text (`THEOREMS-ALLOC-DELTA.md` §0). Proof reports must list the wrappers as unverified source.
- `SPEC.md` §1.1 inventories every public function, with ICO's `parse_with_steps` (`ico.rs:844`) now included, and every verified caller of a frozen name that must be retargeted: DIB witnesses `dib.rs:8815–8962`, TGA witnesses `tga.rs:4571`/`:4609`/`:4939`, ICNS T-wrappers `icns.rs:1297–1483` and `combine`'s internal calls, ICO's public `t5_*`/`t6_*`/`t7_*`/`t9_*`/`t11_*` witnesses and `lib.rs::skeleton`, and ICO's DIB call at `ico.rs:1192`.
- DIB gains a public `try_decode_with_steps_logged` (A-DIB-8) so that ICO's T5 can cite a verified cross-crate postcondition again.
- The per-core tables in `THEOREMS-ALLOC-DELTA.md` mark each frozen row *helper, kept*, *retired; restated*, or *replaced*.

## 3. HIGH: physical allocation events are not equal to the request trace

`THEOREMS-ALLOC-DELTA.md:86–90` required allocator events equal to `Req(x)` in size and order. `Req` counts elements, not bytes; `try_reserve_exact` may produce more capacity than requested; a zero-element request (an ICNS file of 9–15 bytes) does not call the allocator; and `TryReserveError` can be `CapacityOverflow` with no allocator call. `SPEC.md:52`, `:97` and `TABLE.md:25` contradicted each other on the last point.

Resolution (`SPEC.md` §2–§3; `THEOREMS-ALLOC-DELTA.md` §5):

- Two traces: the logical reservation trace `L` (theorems), and the physical allocator-event trace (tests only).
- Zero-element reservations return `Ok` without calling `try_reserve_exact`, appear in `L` as `Ok`, and must produce no allocator event. ICNS `Req_P` now includes `(EntryList, 0)` for `8 ≤ |b| ≤ 15`.
- The counting test pairs `L⁺` (entries with `k ≥ 1`) one-to-one with `alloc` events and checks `align == align_of::<T_site>()`, `size ≥ k·size_of::<T_site>()` (equality expected and recorded on the pinned std), `capacity() ≥ k`, and no `realloc`.
- The guard requires `n·size_of::<T>() ≤ isize::MAX`, so capacity overflow is excluded by std's documented error conditions (A-STD-3). `OutOfMemory` is defined as "the reservation returned `Err`"; reading it as "allocator returned null" depends on A-STD-3, and the three places now say the same thing.
- Fault injection indexes the `j`-th **nonzero** reservation. A guard `Err` with no null-returning event fails the test.

## 4. HIGH: the three-layer audit could not establish "every heap allocation"

Regexes cannot decide whether `.clone()` is on a `Copy` type, or follow user helpers and transitive callees; rlib IR shows only external declarations for precompiled std.

Resolution (`SPEC.md` §6):

- The gate is a typed, transitive call-graph audit over post-monomorphisation MIR on the pinned rustc. It runs on every production configuration (release, default features, `cfg(test)` and `cfg(verus_only)` off, `x86_64` and `i686` Linux) from every public function.
- Every reachable dependency item must be on a per-core allowlist. Allocation-capable items are enumerated per caller (`try_reserve_exact` and `push` only inside `alloc_guard`; `handle_alloc_error`/`abort`/`Layout::array` only in `abort_oom`; ICO's single call to DIB's logged entry point), and anything unlisted fails.
- Dependency contracts: ICO admits the DIB entry point only together with DIB's passing audit for the same DIB source hash.
- The enumerated sites that must become unreachable are listed, including ICNS `String::new` + 8 pushes (`icns.rs:342–364`), the parser list (`:449`, `:579`), channels (`:692`), RGB (`:1023`), mask growth (`:1132–1142`), combine (`:1250`), and ICO `ico.rs:773`/`:817`; ICO's pixel allocation is reached only through DIB.
- The source lint, binary scan and runtime tests are falsification gates. Ghost-reading checks (`assume`, `Tracked::assume_new`, `new_log` count, hashes) are separate from the ghost-stripping allocation lint.
- The tool choice stays open (O2).

## 5. MEDIUM: A-STD and `try_with_room<T>` were stated too generically

Resolution (`SPEC.md` §3; owner default D4):

- A-STD is split into A-STD-1/2/3 and restricted to `Vec<u8>`, `Vec<icns::Entry>` and `Vec<ico::Entry>` with the `Global` allocator.
- `try_with_room<T>` requires `size_of::<T>() ≥ 1` and `n·size_of::<T>() ≤ isize::MAX`, discharged by a Verus `global layout` declaration, with a monomorphic fallback. Zero-sized types are excluded, and `n = 0` is handled without a reservation.
- `try_zeroed` and `try_copy` require `n ≥ 1`.
- Reservation failure (`Err`) is distinguished from allocator null (A-STD-3).
- Fills are `push` loops after the reservation, so A-STD rests on documented guarantees only. `resize` may join later if a measurement justifies it.
- The counting test is described as a regression check, not proof. Proof reports pin the rustc/std version and the allocator under test.

## 6. MEDIUM: error precedence, DIB Blank reachability, OOM-path work charge

Resolution (`SPEC.md` §1.2, §2; `THEOREMS-ALLOC-DELTA.md` A-DIB-2, A-DIB-6, A-ICNS-1; owner default D5):

- ICNS reserves `min(4096, (|b|−8)/8)` entries once, before the loop. `OutOfMemory` may preempt every in-loop parse error. The 16-byte file whose first entry declares length 7 is cited as the example, and the owner accepts this.
- `SPEC.md` §1.2 states that the frozen-name wrappers do **not** preserve today's behaviour exactly for malformed ICNS input under allocation failure (today `Vec::new()` allocates nothing until the first push).
- DIB `Reach` for the policy path is defined by actual reachability of the guard call. The Blank second pass runs after a failed `mh_policy` and returns the original error at `dib.rs:8243`, `:8249`, `:8255` and `:8258`. The required lemma equates passing all four with `mh_partial(...) is Some`, and mutation A20 targets the mask-size check.
- Policy quantification covers every `Policy` value, including those reachable only through the `cfg(test)` `decode_custom_policy` and its `try_` counterpart.
- The guard takes no step counter; the caller charges `4N` only after `Ok`, so the OOM path carries no initialisation charge. The `16·|b|` OOM bound stays optional (O5), with the Blank path's two header passes named as part of the pre-allocation work.

## 7. MEDIUM: `type_hex: [u8; 8]` touches `type_code` and glycin

Resolution (`SPEC.md` §4; `THEOREMS-ALLOC-DELTA.md` A-ICNS-8; `TABLE.md`; owner default D2):

- Allocation-free construction via `hex4_bytes(code) -> [u8; 8]`, with `ensures hex_str(a) == model::hex4(code)`.
- Model projection `hex_str(a) = Seq::new(8, |i| a[i] as char)`, injective on ASCII hex digits. The frozen model `Entry.type_hex: Seq<char>` is unchanged; `model.rs:167`/`:183` read through `hex_str`.
- `Entry::type_code` (`icns.rs:80–112`) keeps its contract and its `is_hex8` precondition (the field is public) and indexes the array directly.
- `Entry` can derive `Copy`. Semver: `icns_verus` 0.1.0 → 0.2.0.
- Downstream: the conformance test at `tests/conformance.rs:83`; glycin-icns `legacy_kind(&str)` (`lib.rs:55`, call at `:104`) and the test helper `side_of` (`tests/vectors.rs:48`, `:135`, `:154`) move to `&[u8; 8]` with byte-string literals.

## 8. MEDIUM: ICO "snapshots only" and "blocked" claims were false

The first draft claimed (`SPEC.md:113–114`; `THEOREMS-ALLOC-DELTA.md:74`, `:119`, `:129`) that ICO's source existed only as snapshots and that the step was blocked. `origin/main` tracks `ico/ico_verus/src/{ico.rs,lib.rs,model.rs,model_vectors.rs,witnesses.rs}` and a frozen `ico/THEOREMS.md`; its `ico/README.md` reports 303 verified obligations.

Resolution:

- All ICO references now point to `origin/main`. Sites were re-inventoried there: `Vec::with_capacity` at `ico.rs:773`, `push` at `:817`, the DIB call at `:1192`, public functions at `:719`, `:844`, `:969`, `:1263`, and public witnesses.
- ICO's `Safe` antecedent is cited from `THEOREMS.md` line 3.
- The ICO step now waits only on DIB's `try_decode_with_steps_logged` and on which branch ICO builds against (O6).

## 9. LOW: exact abort message and `handle_alloc_error` layout

Resolution (`SPEC.md` §1.2; `THEOREMS-ALLOC-DELTA.md` §5; `TABLE.md`):

- `abort_oom` builds the layout with checked `Layout::array::<T_site>(elements)`, which gives the right size and alignment for non-`u8` sites. On `Err` it calls `process::abort()`.
- `elements ≥ 1` is guaranteed, because zero-element reservations never fail.
- The promise is deliberate termination. The exact `memory allocation of N bytes failed` message plus SIGABRT is expected, and tested, only on the pinned std and target with the default alloc-error hook.

## Other owner defaults applied

- D3. Shims without an error channel write a fixed non-allocating message, then `process::abort()`.
- D6. `alloc_guard/mod.rs` is an identical per-core copy with a recorded SHA-256, plus a per-core `sites.rs` with its own hash. Both documents state that the hash authenticates text only.
- D7. gdk-pixbuf `fill_info` leaked `Vec`s become `static` arrays.
- D8. libglycin's existing `GLY_LOADER_ERROR_FAILED` mapping (`libglycin/src/error.rs:17–27`) is documented and gets a test.

## Remaining open owner decisions

Listed in `TABLE.md` under "Open": O1 (ICO public witness functions: keep `pub` with `TryError`, or make private), O2 (call-graph audit tool and allowlist reviewer), O3 (DIB/ICO version bumps), O4 (TGA policy decoder: `try_`-only or with a wrapper), O5 (DIB `16·|b|` bound on the OOM path: required or optional), O6 (branch on which ICO's step runs, given its path dependency on DIB).

## Second review

Sol re-checked the revised texts and raised three HIGH and three MEDIUM findings, plus one pre-freeze requirement. All are resolved in `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md`. The theorem shape changes in two places: ICO now imports DIB's log through a verified guard function, and every functional clause is stated through the frozen executable-to-model predicates, not through an equation. The audit gate is now a concrete `rustc_driver` tool with fail-closed rules and a reviewed per-target allowlist, and its prototype is a freeze gate. The owner settled O1 and O3–O6; O2 becomes gate F1. The open items above under "Remaining open owner decisions" are superseded by this section.

### 2.1 HIGH: ICO could not establish clause 2 for a DIB `OutOfMemory`

ICO creates its own sealed log, but DIB's reservation is recorded in DIB's log, whose elements have a different site type. The first revision's A-ICO-2 said ICO's `L` is DIB's log with sites mapped, but ICO's guard had no operation to import it. When DIB's `Output` reservation fails, ICO's log stays empty, so clause 2 (non-empty log whose last entry failed) cannot be proved. `from_dib` only built an error.

Resolution (`SPEC.md` §1, §3.1, §6; `THEOREMS-ALLOC-DELTA.md` §0, A-DIB-8, A-ICO-2, A21/A22):

- `from_dib` is removed. ICO's `alloc_guard/sites.rs` gains the verified exec function `import_dib(r, dlog, log)`, part of the hashed and reviewed guard text.
- Event mapping: `map_dib(s) = Seq::new(s.len(), |j| (Dib(s[j].0), s[j].1, s[j].2))`; ICO's site enum is `EntryList | Dib(dib::AllocSite)`.
- Contract: `requires` ICO's log empty and `dib::dib::log_tied(r, dlog.view())`; `ensures` ICO's log equals `map_dib(dlog.view())`, `Ok`/`Format` results pass through, and on `OutOfMemory(o)` it returns `o'` with `(o'.site(), o'.elements()) == (Dib(o.site()), o.elements())` and ICO's last log entry `(Dib(o.site()), o.elements(), false)`.
- DIB's A-DIB-8 now exports `log_tied` (the log part of clauses 1–2) and returns its log as `Tracked<dib::alloc_guard::AllocLog>`.
- The DIB log is consumed linearly, and the empty-log precondition excludes a second import. The ghost-reading audit checks one `import_dib` call on the one DIB call's result.
- A-ICO-2 now gives the clause 1 and clause 2 proofs step by step. New mutations: A21 (import a fresh empty DIB log) and A22 (skip the import).
- Cross-crate verifiability of this design is pre-freeze gate F3.

### 2.2 HIGH: `r == lift(M(x))` is not type-correct

Executables return `Vec`/`Decoded`, models return `Seq`/tuples, and ICNS's parse model has its own error enum (`MpError`), so an equation between them is not a valid Verus `ensures`.

Resolution (`SPEC.md` §1.2, §2; `THEOREMS-ALLOC-DELTA.md` §0, every A-row):

- New `fmt_part(r)` maps a non-`OutOfMemory` `try_*` result to the frozen result type.
- Each entry point's relation `Eq_f` is the frozen predicate: DIB `decode_mp_eq` (`dib.rs:6780`); DIB policy `decode_policy_mp_eq ∧ decode_policy_alpha_eq ∧ r.1 == recovered_policy`; TGA `decode_eq` (`tga.rs:281`); ICNS `parses_to`, `rle_result_eq`, `rgb_decoded_eq`, `mask_decoded_eq`, `combine_decoded_eq`; ICO `parse_eq`, `md_eq`.
- `G` clause 1, `Prog`, the `PreAlloc` corollary and the public `try_*` `ensures` are restated through `Eq_f`.
- Wrapper inheritance now reads: the wrapper returns `fmt_part(try_g(x))`, which satisfies `Eq_f`, the same predicate the retired theorem stated on `g`.
- ICO's T5 keeps `lemma_md_eq_dib` unchanged, fed with `decode_mp_eq(fmt_part(r0), …)`.
- Both documents state that every verifier kill citing clause 1, `Prog` or `PreAlloc` depends on `Eq_f` being the full frozen predicate.

### 2.3 HIGH: the typed audit had no mechanism or trust boundary

Resolution (`SPEC.md` §6, §8; `THEOREMS-ALLOC-DELTA.md` preamble, §7):

- Mechanism: `tools/alloc_audit`, a `rustc_driver` program on the pinned rustc 1.99.1 (commit `58cf8f9c3`, `rustc-dev` installed for x86_64), built with `RUSTC_BOOTSTRAP=1`. In `after_analysis` it walks post-monomorphisation MIR from `Instance::mono` roots. It resolves `Call` operands with `Instance::try_resolve` and `Drop` terminators with `resolve_drop_in_place`, then follows drop-glue shims, `Assert`/panic edges, reified function pointers and vtable methods.
- Fail-closed rules: precompiled non-generic std bodies without MIR, intrinsics, panic entry points and compiler-generated allocation paths are leaves that pass only by an allowlist entry. Calls through `fn` pointers or trait objects, FFI, inline/global assembly and `-Zshare-generics` fail. Panic edges are justified by citing T1/`Safe'`.
- Trust boundary: the allowlist is a manually reviewed, target-specific, hash-recorded transitive contract, with the rustc commit, vstd version, target and a named reviewer recorded in it. It is re-reviewed on any toolchain, std, vstd or target change, and the tool refuses a commit mismatch. The allowlist is now a named trust item in every proof report.
- 32-bit: no i686 `rust-std` (and no `rust-src` for `-Zbuild-std`) is installed, so the audit cannot compile the core for i686. Proof reports scope the audit claim to x86_64 and mark i686 unaudited until the component is added; i686 then gets its own allowlist. The README text in `TABLE.md` says so.
- The binary scan and counting tests remain falsification only.
- A prototype passing on DIB is pre-freeze gate F1. This closes O2's tool choice.

### 2.4 MEDIUM: A-DIB-7 required equal logs from independent calls

Two calls can see different allocator outcomes, for example under the fault injector.

Resolution (`THEOREMS-ALLOC-DELTA.md` A-DIB-7):

- (a) Model lemma: `Req_DIB(b,mode) == Req_DIBp(b, bmp_mode(mode), Strict)`. Since each `Req` has at most one element, clauses 1–2 give `req(L1) == req(L2)` for every outcome.
- (b) With the explicit antecedent that neither result is `OutOfMemory`: `L1 == L2`, each result satisfies its frozen predicate, and the frozen strict bridge (`policy/THEOREMS-POLICY-DELTA.md` line 28) makes them agree.
- No allocator coupling is assumed. A statement covering `OutOfMemory` would need "same reservation outcome" as an antecedent and is not part of the delta.

### 2.5 MEDIUM: the `type_hex` retargeting was incomplete

`icns_verus/src/model.rs` uses the executable field's `String` view in many places beyond `:167`/`:183`.

Resolution (`SPEC.md` §4; `THEOREMS-ALLOC-DELTA.md` §3, §7):

- Full inventory of executable-`Entry` occurrences, each replaced by `hex_str`: `model.rs` lines 167, 183, 184 (`entry_code`, `entry_inv`); 1037 (`entry_matches`); 1152 (`entry_eq_fields`, used by `parses_to`); 1358, 1659 (lemmas); 2251 (`entries_same_fields`); 2278, 2287 (`lemma_exact_entry_equivalence`); 3107 (`entry_type_code`); 3606, 3631 (`lemma_mc_ms_eq`, `lemma_ma_ms_eq`); 3928, 3948 (`lemma_mc_fields_eq`, `lemma_ma_fields_eq`). Plus `icns.rs:74`, `:84–125`, `:537–539`, `:557`, and `hex4` (`:342–364`) deleted.
- Unchanged, being on `MpEntry` or model sequences: `model.rs:335`, `:364`, `:702`, `:792`, `:949`, `:2117`, `:2219`, `:2224`, `:3405`, `:3426`, `:3607`, `:3632`, and all 39 occurrences in `model_vectors.rs`.
- The retargeting and the one new lemma (injectivity of `hex_str` on lowercase hex digits) are in the ICNS step's scope.

### 2.6 MEDIUM: ICNS peak memory omitted mismatched sides

`combine` holds both decoded buffers before the side check (`icns.rs:1240`).

Resolution (`SPEC.md` §4; `THEOREMS-ALLOC-DELTA.md` A-ICNS-6, §5):

- Three phases: `6·n_c` during the colour decode (channels plus `RgbOut`); `3·n_c + n_m` from the mask reservation until the side check, matched or not (16×16 colour + 128×128 mask = 17,152 bytes); `8n` after a matching check.
- The absolute bound is unchanged at 131,072 bytes (128 KiB).
- The peak is labelled an ownership fact, not a verified clause. The counting test checks it, including the mismatched case.

### 2.7 Pre-freeze: per-target `Entry` layout

Resolution (`SPEC.md` §3, §8 F2; `THEOREMS-ALLOC-DELTA.md` §5): the `global layout` declaration for `icns::Entry` and `ico::Entry`, or the monomorphic `try_with_room_entry` fallback, must be instantiated with concrete sizes and checked by rustc on each production target before any numeric bound that depends on `size_of::<Entry>()` is frozen. Test expectations read the sizes from that declaration.

### 2.8 Owner decisions recorded

Recorded as decided defaults in `SPEC.md` §7 and `TABLE.md`:

- O1: ICO witnesses stay `pub` with retargeted `TryError` contracts.
- O3: DIB 0.4.0, ICO 0.2.0, `icns_verus` 0.2.0.
- O4: the TGA policy API is `try_`-only; it gets an aborting wrapper only for an explicit compatibility requirement.
- O5: `steps_OOM ≤ 16·|b|` stays optional; the required bound holds for every result.
- O6: ICO is implemented only after DIB's verified logged export lands on the branch its path dependency resolves to, or on a merged branch with the DIB source hash recorded in ICO's audit.

Sequencing: the guard module and tooling come first; the TGA policy implementation (separate ongoing work editing `tga.rs`) must finish before the TGA allocation pilot.

Open items after the second review were F1–F3. F2 and F3 passed in the fourth revision; F1 was reopened by the final read (fifth revision), passed with the audit fix (sixth revision), and was reopened again by Sol's final read of that fix (seventh revision). The current list is at the end of this document.

## Third revision: owner decisions of 2026-11-12

No new review findings; these changes follow from owner decisions and are applied in `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md`. They supersede the wrapper design in §2 and §9 above and in the D1/O3/O4/D8 entries.

- **Frozen names removed (D1 revised).** The main crates (DIB, TGA, ICNS, ICO) export only the verified `try_*` functions returning `TryError { Format(Error), OutOfMemory(..) }`. No `decode`/`parse`/`combine` names and no aborting function remain in them. The retired theorems are replaced by the `try_*` theorems with no wrapper inheritance argument; main-crate proof reports list no unverified source (`SPEC.md` §1.2; `THEOREMS-ALLOC-DELTA.md` §0). The inventory of witnesses and verified callers to retarget is unchanged (`SPEC.md` §1.1).
- **Abort shim crates.** Aborting wrappers, if wanted, live in optional, unverified `<core>_abort_shim` crates outside the main crates' proof and audit boundary. `abort_oom`, the `Layout::array`/`handle_alloc_error` material, the ICNS abort-point caveat and the exact-message/SIGABRT promise (restricted to pinned std and target, default hook, no `-Zoom=panic`) move there (`SPEC.md` §1.3; `THEOREMS-ALLOC-DELTA.md` §5).
- **Audit.** Roots are the `try_*` functions and `*_logged` exports only. `handle_alloc_error`, `process::abort` and `Layout::array` have no allowlist entry except inside `Vec::push`'s contract in the guard; the source lint flags them, and new mutation A23 (reintroduced aborting entry point) is an audit kill (`SPEC.md` §6).
- **Consumer migration.** `SPEC.md` §1.4 lists every caller of a frozen name to move to `try_*` or a shim crate: gdkpixbuf-loaders, the glycin fork loaders and their tests, the core conformance and corpus tests, the leniency runners and the ICNS diff harness. No benchmark calls a core directly.
- **Breaking changes acceptable (D9).** Version bumps (O3) are notes, not gates. `type_hex: [u8; 8]` stays (D2).
- **D5 sharpened.** ICNS's single reservation of `min(4096, (|b|−8)/8)` entries may report `OutOfMemory` before a malformed-entry error even when enough memory would remain to report that error; the owner accepts this.
- **D7 sized.** BMP's static `fill_info` arrays are 48 + 32 + 16 = 96 bytes per module per process on 64-bit (`TABLE.md`).
- **D8 revised.** No upstreaming to glycin or libglycin; glycin-report wording removed. The fork loader mapping (`ProcessError::out_of_memory()`) stays; libglycin's C API keeps `GLY_LOADER_ERROR_FAILED`, documented.
- D3 (C-API shims without an error channel: fixed non-allocating message, then `process::abort()`) and D4 (push-loop fills under A-STD) are unchanged.

## Fourth revision (prototype gates)

No new review findings; the three pre-freeze gates ran as prototypes under `alloc-delta/proto/` and all passed, F1 and F2 on x86_64 only. (F1 was reopened by the final read and passed again after the audit fix; see "Fifth revision" and "Sixth revision".) The theorem shape is unchanged. The gates forced clause changes in `SPEC.md` §1, §3, §3.1, §6 and §8, mirrored in `THEOREMS-ALLOC-DELTA.md` (§0, A-DIB-8, A-ICO-2, §5, §6, §7) and `TABLE.md`. These changed passages are what the final Sol read should cover.

- **F3 (sealed log): passed.** `proto/guard_proto` (44 verified) and `proto/guard_proto_dep` (22 verified, cross-crate `import_dib`), 0 errors; four negative checks fail at the predicted gate (`proto/NOTES.md`, `NEGATIVE.tsv`). Clause changes:
  - `OutOfMemory`'s spec counterparts are `pub closed spec fn site_spec(&self)`/`elements_spec(&self)`, same-`impl` siblings of the exec accessors. Verus rejects an `open` body over a private field and a receiver-less counterpart (§1).
  - Three `tracked`-struct mechanics: `Tracked(log): Tracked<&mut AllocLog>` destructured in the parameter list; the append is `proof { log.ev = log.ev.push(..) }`; that one statement is repeated per reservation function, with no shared helper (§3).
  - A `global layout` axiom is usable only in its declaring module, so `sites.rs` exports `lemma_entry_layout` (§3).
  - `import_dib` consumes `r` and the DIB log, so it carries the DIB-arm part of A-ICO-2 in its own `ensures` over `out` and the final log. It is proved arm by arm, and the `OutOfMemory` literal is built only in ICO's `sites.rs` (§3.1). It now also requires DIB's exported `g_logged` (A-DIB-8).
  - The guard listing matches the prototype, which is the reference implementation, with its four SHA-256 values recorded in §3 (not the D6 production hashes).
- **F2 (`Entry` layout): passed on x86_64.** 28 B (ICNS) and 44 B (ICO), alignment 4, checked by rustc (`E0080` on a wrong size). The fallback was not needed. These are prototype struct shapes; each core's step re-declares its real `Entry`. i686 has no std, so it has no layout and no frozen i686 byte bound.
- **F1 (call-graph audit): passed on x86_64 release.** All 15 negative controls fail and the clean control passes. On DIB the tool reports exactly the one allocation site via `alloc_output_n` plus deallocation, and passes with a 21-entry allowlist (`proto/alloc_audit/NOTES.md`). The six edits proposed there are applied in §6/§8:
  - The audit is release-profile specific, because `share_generics()` is on below `opt-level=3` and the reachable set differs by profile.
  - The allowlist records the profile, and its tool-printed SHA-256 goes into the proof report.
  - Panic citations are enumerated per function.
  - `finish_grow` is reworded as "such as". The leaf set is toolchain-determined: `handle_error` is the observed leaf, and `finish_grow` was walked.
  - The tool is target-agnostic.
  - F1 requires a per-(target, profile) allowlist, and the DIB control's expected output is stated.
  - §6 also records the DIB review burden (21 entries; 19 panic edges over 18 functions needing T1/`Safe'` citations) and the blind spots: const-eval and static-initialiser allocations, panic-path allocation admitted by human citation, precompiled std without MIR pinned only by the commit hash, indirect-call candidate sets, and allocator identity. i686 is unaudited.
- **Caveat on F1's "named reviewer".** The F1 allowlist is a fixture with `reviewer = "prototype-fixture"`, not a human signature. §8 now reads F1 as producing a hash-recorded allowlist *for* a named reviewer to sign. The signed production allowlist arrives with DIB's step, after `try_zeroed` replaces the `vec!`. This is a reading of the original gate text that the final review should accept or reject.
- **Post-freeze hardening** (§8; not freeze blockers):
  - an automatic per-function panic-edge diff between reports, failing on a new edge;
  - an allowlist hash pinned in the tool instead of `sha256sum`;
  - a full TOML reader;
  - the profile field and its mismatch check;
  - i686 once its std is installed.

## Fifth revision (final read)

Sol's final read of the fourth-revision text raised one HIGH and two MEDIUM findings plus two notes. All are applied in `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md`. Gate F1 is reopened; F2 and F3 stand. The theorem shape is unchanged. `proto/` was not edited.

### 5.1 HIGH: the audit's authorisation is path-insensitive

The F1 prototype records one BFS parent per monomorphised instance (`proto/alloc_audit/src/walk.rs:253–256`) and authorises an allocation-capable instance by that one path. Instances are shared: `Vec::<u8>::try_reserve_exact` called from an unapproved helper is the same instance `try_zeroed` calls, so the helper is authorised through the guard's path. None of F1's fifteen negative controls calls a shared instance, so none exposed this.

Resolution:

- F1 is marked **NOT YET PASSED** in all three documents; the fix is in progress in `proto/alloc_audit`.
- `SPEC.md` §6 drops the claim that a typed call graph by itself establishes absence. The tool records every edge, and a new paragraph, *Authorisation on every path*, requires every reachable incoming path to an allocation-capable instance to pass through an admitted caller, failing closed otherwise. The allowlist rule for `try_reserve_exact`/`push` and the "anything unlisted fails" sentence say the same.
- F1's control set gains the same-instance negative control: a function outside `alloc_guard` calls the instance `try_zeroed` calls; the audit must fail and print that function's path.
- `TABLE.md`'s README text now says the audit shows every call path to an allocating std function passes through the guard, and that it rests on the signed allowlist.

### 5.2 MEDIUM: profile check and provenance validation are prerequisites, not hardening

Resolution: both move out of `SPEC.md` §8's post-freeze hardening into a new list, *Prerequisites for assurance claims*, which gates any DIB proof report and any README "proved"/"audit shows" text. The list is: corrected audit; production allowlist signed by a named human reviewer; profile-mismatch refusal; provenance validation (`source_hash` equals the audited sources' hash, `reviewer` non-empty and not a fixture value); production `try_zeroed` path with `#[inline(never)]` in the production guard. `SPEC.md` §6 states the provenance check in the allowlist contract. Post-freeze hardening keeps the panic-edge diff, the in-tool hash pin, the full TOML reader and i686.

### 5.3 MEDIUM: "header errors are never `OutOfMemory`" is false for ICNS

ICNS `try_parse` reserves the entry list before its loop, so a file with one complete entry and a trailing partial entry header can return `OutOfMemory` in place of `TruncatedEntryHeader`.

Resolution: `SPEC.md` §2, `THEOREMS-ALLOC-DELTA.md` §0 and the README text in `TABLE.md` now say "errors detected before the first reservation". The README text gives the ICNS example and names the assumptions behind "proved" and "audit shows": the reviewed, hash-checked guard body, A-STD and the signed allowlist.

### 5.4 Note: F1's gate wording and two unexercised obligations

- The revised F1 wording ("hash-recorded allowlist awaiting a named reviewer") is accepted as a **prototype feasibility** gate only. The human-signed production allowlist is required in the DIB step, before any assurance claim (§5.2).
- `import_dib`'s production signature returns the nested `Result<Result<dib::Decoded, dib::Error>, OutOfMemory>`; F3 returned ICO's `TryError` directly. The nested form adds a result-mapping step and a `g_dib_arm` proof over it that F3 did not exercise. It is listed in `SPEC.md` §8 as an implementation obligation of the DIB import; the code lands in ICO's step (`THEOREMS-ALLOC-DELTA.md` §7, step 4), which consumes DIB's export.
- F2 checked toy x86_64 struct shapes, not production `Entry` definitions. Each core's step re-declares and re-checks its real layout (`SPEC.md` §3, §8).

### 5.5 Note: F3 negative table

`neg2_edit_log_outside_guard`'s second route failed with an unrelated method-resolution error (`E0599`, `borrow_mut` not in scope; `proto/runs/123.log`) alongside the predicted `E0616` on the first route. The first route carries the evidence. The table entry in `proto/NOTES.md` should read "predicted failure plus an unrelated error in the second route, to be removed"; that file belongs to the agents working in `proto/` and was not edited here. `SPEC.md` §8 F3 records the caveat.

## Sixth revision (audit fix)

No new review findings. The fix for §5.1 landed in `proto/alloc_audit` (`NOTES.md`, "What this revision changed"), and this revision records it in `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md`. Gate F1 is marked **PASSED** as a prototype feasibility gate on x86_64 release. The delta is **not frozen**. The theorem shape and every non-audit clause are unchanged.

What the prototype now does:

- Every-path authorisation: `authorize_allocations` (`proto/alloc_audit/src/walk.rs:789`) enumerates every simple root-to-entry path, bounded at 20,000 paths per root and depth 80, failing closed with `path-bound` past either bound. Each path is judged by its last allocation-capable item and the nearest audited-crate function before it.
- Same-instance control `testcases/n11_shared_instance.rs`: passed (exit 0) before the fix; fails after it (exit 1, 12 `unauthorized-alloc-path` findings naming `unapproved_reserve` and `unapproved_grow`).
- Profile detection and mismatch refusal (`opt-level`, `debug-assertions`, `share-generics`), control `c1_prof_mismatch` plus a recorded-opt-level fixture.
- Provenance validation: `source_hash` recomputed from the audited sources; `reviewer` non-empty; `prototype-fixture` rejected unless `--prototype` / `ALLOC_AUDIT_PROTOTYPE`, which prints `mode: PROTOTYPE`.
- 21 negative controls fail and both toy positive controls pass; DIB passes at 91 instances, 277 edges, 0 findings, allowlist SHA-256 `1f9739e4d339ef2cbd8e4b41a54a72833fdf6196e9e18c16d3ae044eebcef2c8` (was `295c8bf4…`; the fixture gained `profile` and `source_hash`).
- Three new blind spots: the path-enumeration bound (fails closed), the source hash covering the crate's own files only, and authorisation constraining crate-local callers only.

Consequential edits: the profile check and provenance validation move out of §5.2's prerequisite list, since the prototype implements them; the prerequisites now read "production tool keeping those checks", "signed allowlist passing without `--prototype`", and "production `try_zeroed` with `#[inline(never)]`", none of which holds yet. The fourth revision's hardening item "the profile field and its mismatch check" (above) was already moved by §5.2 and is now done.

**Passages for the final Sol read** (changed audit text only):

- `SPEC.md` §6, *1. Call-graph audit*, first paragraph: the last sentence (the prototype as reference for edge resolution and authorisation).
- `SPEC.md` §6, *Authorisation on every path*: the new second paragraph (how the prototype realises the rule; bounds; std callers not judged). The first paragraph is unchanged.
- `SPEC.md` §6, *The allowlist*: the `[meta]` sentence, the four enforcement bullets (commit/target, profile, `source_hash`, reviewer and `--prototype`) and the fixture sentence that follows them.
- `SPEC.md` §6, *Review burden*: "identical before and after the every-path fix".
- `SPEC.md` §6, *What the audit cannot see*: the last three bullets (path-enumeration bound, source hash scope, authorisation scope).
- `SPEC.md` §7, O2.
- `SPEC.md` §8: the opening sentence, the F1 entry, *Before the delta is frozen*, and *Prerequisites for assurance claims*.
- `THEOREMS-ALLOC-DELTA.md` §7: the sequencing paragraph, step 0's table row, and the risk bullets "Path-insensitive authorisation" and "The audit's blind spots".
- `TABLE.md`: the last two sentences of the README text's first paragraph, the note after the README text, O2, the F1 gate entry, and the two paragraphs after the gate list.

Evidence Sol may check against the text: `proto/alloc_audit/NOTES.md` (control table, before/after table, blind spots 11–13), `reports/n11_shared_instance.txt`, `reports/dib_verus.txt`, `reports/c1_*.txt`.

## Seventh revision (final read of the audit fix)

Sol's final read of the sixth-revision text raised one HIGH, three MEDIUM and one LOW finding. All are applied to the text of `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md`. Gate F1 is **REOPENED, not passed**; F2 and F3 stand. The delta is **not frozen**. The theorem shape is unchanged. The tool fix is in progress in `proto/alloc_audit` by another agent; this revision did not touch `proto/`.

### 7.1 HIGH: allowlist caller matching by string suffix and prefix

The prototype admits a path's caller if the caller's path and the entry's `allow` value end with one another (`proto/alloc_audit/src/allowlist.rs:198`), and matches items by `starts_with`/`ends_with` (`:185`, `:197`). A lookalike local function, such as `not_alloc_guard::try_zeroed`, is then authorised by an entry for `alloc_guard::try_zeroed`. Every-path enumeration does not help when each path's judgement is loose.

Resolution:

- F1 is marked **REOPENED** in all three documents, until the fix and its controls land.
- `SPEC.md` §6 gains *Caller and item identity*: `allow` and item values match the fully qualified local def-path component by component with `::` boundaries (or by resolved `DefId`), never by suffix, prefix or substring; generic items name their generic arguments, so distinct instantiations are distinct entries; `"*"` is the only wildcard; an unresolvable or ambiguous value is a finding.
- Required controls: (a) a local function whose def-path ends with an admitted path; (b) a prefix or suffix fragment of an admitted path in another module. Both must fail with `unauthorized-alloc-path`.
- §6's prototype sentence now says the prototype is the reference for path enumeration, not for path matching.

### 7.2 MEDIUM: provenance fields not all required or validated

The prototype skips the commit, target and profile checks when the field is missing, accepts a commit prefix in either direction (`allowlist.rs:137`), and never reads `vstd_version`.

Resolution (`SPEC.md` §6, *The allowlist*; §8):

- All six `[meta]` fields (`rustc_commit`, `vstd_version`, `target`, `profile`, `source_hash`, `reviewer`) are REQUIRED; a missing or empty field is `allowlist-missing-<field>`.
- `rustc_commit`: exact, or an abbreviation of the running hash of at least 9 hex digits, recorded-is-prefix-of-running only.
- `vstd_version`: must equal the linked vstd's version, or the run fails (`allowlist-vstd`); a vstd change invalidates the allowlist.
- New required controls: one missing-field fixture per field, a vstd-change fixture, and a short-commit fixture.
- §8's prerequisites and the TABLE/THEOREMS step-0 text no longer describe provenance validation as implemented; they say it is incomplete until the fix lands.

### 7.3 MEDIUM: several admitted callers of one shared instance

Resolution (`SPEC.md` §6, *Caller and item identity*; allocation-capable bullet): one entry per (item instantiation, admitted caller); every path must match one of them, and an admitted caller admits no caller it does not name. Required control: two admitted callers of one shared instance pass, and an unapproved third caller of that instance fails with its path named.

### 7.4 MEDIUM: source-hash scope overstated

Resolution: `SPEC.md` §6 (the `source_hash` enforcement bullet and *What the audit cannot see*), `THEOREMS-ALLOC-DELTA.md` (preamble, §7 blind-spots risk) and `TABLE.md` (README first paragraph and the note after it) now say the hash covers source-map text only. It binds no `include_bytes!`/`include_str!` payload, environment or build input, `build.rs` or proc-macro output, or dependency. The production proof-report process pins or accounts for each, or the audit fails; that process is a new item under *Prerequisites for assurance claims*.

### 7.5 LOW: n11's `unapproved_grow` path

`unapproved_grow` reaches `resize -> extend_with -> reserve -> do_reserve_and_handle -> grow_amortized -> finish_grow`, not the guard's `push_mut` (`reports/n11_shared_instance.txt`). `SPEC.md` §8 F1 and `TABLE.md`'s F1 entry are corrected, and both now separate the archived post-fix report from the pre-fix exit 0, which is an observation recorded in `NOTES.md` with no archived report. The sixth-revision section above keeps its original wording as history.

**Passages for Sol to re-read after the tool fix lands** (changed audit contract text only):

- `SPEC.md` §6, *1. Call-graph audit*, first paragraph: the last two sentences (prototype as reference for enumeration, not matching).
- `SPEC.md` §6, *Caller and item identity*, its four bullets and the required-controls paragraph (all new).
- `SPEC.md` §6, *The allowlist*: the "every one of the six fields is required" sentence; the `rustc_commit`, `vstd_version` and `source_hash` bullets; the paragraph on what the prototype checks and the new controls; the allocation-capable `try_reserve_exact`/`push` bullet; the last sentence of "Anything reachable…".
- `SPEC.md` §6, *What the audit cannot see*: the source-hash-scope bullet.
- `SPEC.md` §7, O2.
- `SPEC.md` §8: the opening paragraph, the F1 entry (status, requirement, reopening paragraph, same-instance control bullet, the sentence after the evidence list), *Before the delta is frozen*, and the first three *Prerequisites for assurance claims* bullets.
- `THEOREMS-ALLOC-DELTA.md`: the preamble's trust-item sentences; §7 sequencing paragraph, step 0 row, and the risk bullets "Path-insensitive authorisation" and "The audit's blind spots".
- `TABLE.md`: the README text's allowlist and hash sentences in the first paragraph, the note after the README text, O2, the F1 gate entry, and the two paragraphs after the gate list.

Sol should check these against the fixed tool's code (`proto/alloc_audit/src/allowlist.rs`, `walk.rs`), its `NOTES.md` and the new collision, several-callers and provenance control reports.

## Eighth revision (audit fix landed)

No new review findings. The fix for §7.1–§7.5 landed in `proto/alloc_audit` (`NOTES.md`, "What the second revision changed"). This revision matches the audit contract in `SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md` to the implemented rules, applies the prototype's proposed SPEC edits 9–12, and marks gate F1 **PASSED** again as a prototype feasibility gate (x86_64, release). The delta is **not frozen**. The theorem shape and every non-audit clause are unchanged; this revision did not touch `proto/`.

Evidence: `run_tests.sh` runs 38 controls, 4 positive (`c1_clean`, `c1_rev_named`, `c2_two_guards`, `c3_vstd_ok`) and 34 negative; every archived verdict in `reports/` matches its expectation. DIB passes on `pinned-dib/`, a copy of the audited revision (91 instances, 277 edges, 0 findings, allowlist SHA-256 `1f9739e4…` unchanged); the live DIB crate is being rewritten by the `try_*` implementation, so the production DIB run is a later step. The pre-fix `n11` pass is archived as a labelled reconstruction (`reports/n11_pre_fix.txt`, `prefix-reconstruction/reproduce.sh`), which `SPEC.md` §8 now accepts as evidence (proposed edit 12).

Where the contract text changed to match the code rather than the other way round, Sol should judge whether the implemented rule is acceptable:

- **Generic arguments are optional in an entry.** The seventh revision required them and made an entry without them a finding. The tool instead lets an entry without generic arguments match every instantiation reached from its admitted caller (`RawVecInner::<A>::reserve_exact` covers `…<Global>`). The text now says so and requires the entry's contract to hold for each instantiation.
- **No `allowlist-entry` finding for an `allow` naming no function.** Such an entry admits nothing, so the paths it was meant to cover fail with `unauthorized-alloc-path` (still fail-closed), but the unused entry itself is not reported. Reporting it is now a post-freeze hardening item.
- **Commit rule.** The seventh revision said "a prefix of at least 9 hex digits". The tool accepts the compiler's 9-hex version-string token (`58cf8f9c3`) only by exact equality, the full 40-hex hash, or a prefix of that hash of at least 12 hex digits.
- **vstd version source.** The seventh revision said "read from the loaded crate's metadata". The tool reads `Cargo.lock` via `CARGO_MANIFEST_DIR`, or the `ALLOC_AUDIT_VSTD_VERSION` override, which takes precedence and is unchecked. Both limits are now listed under *What the audit cannot see*.
- **Missing-field finding names** are `allowlist-<field>-missing` (and `allowlist-reviewer` for the reviewer), not `allowlist-missing-<field>`.

**Passages for Sol to re-read** (changed audit contract text only):

- `SPEC.md` §6, *1. Call-graph audit*, first paragraph: the last sentence (the prototype as reference for matching and provenance).
- `SPEC.md` §6, *Authorisation on every path*, second paragraph: the sentence beginning "The tool consults every `alloc` entry".
- `SPEC.md` §6, *Caller and item identity*: the whole section (rewritten: six bullets and the controls paragraph).
- `SPEC.md` §6, *The allowlist*: the "Every `[meta]` field is required" sentence; the `rustc_commit` and `vstd_version` bullets; the scope-line clause added to the `source_hash` bullet; the new *Build inputs* bullet; the paragraph on the prototype's controls (replacing "Provenance validation is therefore not complete"); the last clause of the allocation-capable `try_reserve_exact`/`push` bullet.
- `SPEC.md` §6, *What the audit cannot see*: the two new bullets (build-input detection; vstd version source).
- `SPEC.md` §7, O2.
- `SPEC.md` §8: the opening sentence, the F1 entry (rewritten in full), *Before the delta is frozen*, the first *Prerequisites for assurance claims* bullet, and the new last post-freeze hardening item.
- `THEOREMS-ALLOC-DELTA.md`: the preamble's trust-item sentences; §7 sequencing paragraph; step 0 row; risk bullets "Path-insensitive authorisation" (last three sentences) and "The audit's blind spots" (the build-input-detection and vstd clauses).
- `TABLE.md`: the README text's hash sentence in the first paragraph; the note after the README text (prerequisite parenthesis and the audit-limit sentence); O2; the F1 gate entry (rewritten); the two paragraphs after the gate list.

Sol should check these against `proto/alloc_audit/src/allowlist.rs` (`def_path_matches`, `caller_matches`, the `[meta]` checks, `detect_build_inputs`), `src/walk.rs` (`judge_path`, finding names), `NOTES.md` (control table, items 13–14) and the reports for `n11`–`n15`, `c2_two_guards`, `c1_commit_*`, `c1_*_missing`, `c3_vstd_*`, `c1_vstd_undeclared`, `bs_build_script` and `dib_verus`.

## Ninth revision (contract narrowed)

Sol's re-read of the eighth revision found four defects in the prototype audit (two HIGH, two MEDIUM) and two further points (the vstd override; the meaning of "pins or lists" versus "accounts for"). Each is resolved by **narrowing** the contract, so the text no longer promises what the prototype does not do, and by making the fix a mandatory requirement of the production tool or the proof-report process. `proto/` was not touched (a parallel agent is finishing controls there), no Verus run was made, and no gate verdict changed: F1 stays PASSED, now stated as "a fail-closed transitive allocation audit is feasible, and these are its demonstrated mechanisms". The theorem shape and every non-audit clause are unchanged. The document set is **READY FOR OWNER FREEZE** after a last short Sol read; it is not frozen.

### 9.1 HIGH: node identity by printed def-path string

The prototype keys nodes, edges and paths by `def_path_str` plus printed generic arguments (`walk.rs:83–95`, `:446–457`, `:1186–1202`); local paths lack the crate prefix that external ones carry, so a local `guard::try_zeroed` and an external crate `guard`'s `try_zeroed` merge, and `allow = "guard::try_zeroed"` admits the external one (`allowlist.rs:480–523`, `judge_path` `walk.rs:1096–1122`).

Resolution: the production tool MUST key by `Instance`/`DefId`, use printed paths for display only, resolve each allowlist value to `DefId`s before the walk, and reject an ambiguous value (`allowlist-ambiguous`); local items are written `crate::…`. The prototype's def-path identity claim is limited to the single-crate toy setting. Required control `x1_extern_namesake` (local module plus same-named external crate). `SPEC.md` §6 *Caller and item identity*, §8 defect 1.

### 9.2 HIGH: `//` inside a string literal hides an include

`strip_line_comments` (`allowlist.rs:645–659`, used at `:673–685`) blanks from the first `//` on a line, so `let u = "https://x"; include_bytes!("p.bin")` is missed.

Resolution: production detection is compiler-resolved (expansion spans or the session's loaded-file list), and the proof-report process carries an independent build-input inventory (below). The categorical "undeclared include sites fail the audit" is removed for the prototype: it fails only on inputs its scan detects. The omission is added to the disclosed blind spots. Required control `x2_include_after_url` (plus renamed-import and foreign-`macro_rules!` variants). `SPEC.md` §6 *Build inputs*, *What the audit cannot see*; §8 defect 2.

### 9.3 MEDIUM: wildcard build-input declarations; `verus*` name prefix

`item = "*"` is accepted for build-script, proc-macro and include declarations (`allowlist.rs:335–342`, `:300–309`, `:709–713`), and `name.starts_with("verus")` admits `verus_evil` (`allowlist.rs:617–625`).

Resolution: `*` is only an `allow` value for allocation-caller authorisation; every build-input declaration names its exact input (`allowlist-wildcard-build-input` otherwise); production binds the actual Verus macro crates by name and artifact hash. `TABLE.md`'s README sentence ("each procedural-macro dependency must be declared") is rewritten as the production requirement, and the note after the README states what the prototype does instead. Required controls `x3_wildcard_build_input`, `x4_verus_prefix`. §8 defect 3.

### 9.4 MEDIUM: an absent allowlist does not fail

No allowlist yields `Allowlist::empty()` (`walk.rs:101–105`), skips every provenance check (`:123–313`), and `passed()` (`report.rs:153–159`) can return `Pass`.

Resolution: a proof-report run requires an allowlist file with every `[meta]` field; the production tool fails with `allowlist-absent` and prints `diagnostic: no allowlist`. Prototype runs without an allowlist are diagnostics only. `SPEC.md` §6's "every field is required" is qualified to runs that have an allowlist. Required control `x5_no_allowlist`. §8 defect 4.

### 9.5 vstd override and version binding

The override `ALLOC_AUDIT_VSTD_VERSION` is prohibited in proof-report runs (production refuses it outside `--prototype`, `vstd-override-forbidden`), and the version is bound to the linked rlib and its locked source checksum. The eighth revision's alternative ("or the report must record its value") is deleted: recording an override cannot show which vstd was linked. `c3_vstd_ok`, `c3_vstd_change` and `c3_vstd_missing` all set the override and are described as parser and comparison controls only. Required controls `x6_vstd_override`, `x7_vstd_artifact`. §8 defect 5.

### 9.6 "Pins or lists" versus "accounts for"

One enforceable meaning replaces both: the proof report lists every build input with a content hash (source-map files, `include_*!` targets, `env!` variables with values, the rustc command line, `build.rs` and its outputs, every dependency artifact with its locked checksum), and that list must equal an independent inventory built without the audit tool's detection code: rustc's `--emit=dep-info` file (every file read, plus `# env-dep:` lines), the cargo command line (`cargo build -v`), and `cargo metadata` with `Cargo.lock`. Any difference blocks the proof report. This assumes rustc's dep-info is complete for the pinned compiler, which the process tests with the `x2` controls. `SPEC.md` §6 *Build-input inventory*; `TABLE.md` README and note; `THEOREMS-ALLOC-DELTA.md` preamble.

### 9.7 F1 framing, known defects and prerequisites

`SPEC.md` §8 now lists **Known prototype defects the production tool must fix**: the four findings above, the vstd override, the build-script control injecting `OUT_DIR` into a crate with no `build.rs` (no real build script under cargo), and dylibs over-reported as proc-macros. *Prerequisites for assurance claims* (none met): the production tool fixing those defects and passing the required controls; the independent build-input inventory; a named-human-signed post-change allowlist passing without `--prototype` and without the vstd override; the production DIB guard path with `#[inline(never)]`. Production implementation may start now; claims may not. The missing `source_hash`/`reviewer` key fixtures no longer block the freeze, because the frozen text does not claim them of the prototype; they are on the production tool's required-control list.

**Passages for a last short Sol read** (changed text only):

- `SPEC.md` header line.
- `SPEC.md` §6, *1. Call-graph audit*, first paragraph: the last three sentences (prototype as reference for edge resolution and enumeration only; "the tool" means production).
- `SPEC.md` §6, *Caller and item identity*: the two new opening paragraphs (production `DefId` requirement; the prototype's string keying and single-crate limit) and the last two sentences of the controls paragraph (`x1_extern_namesake`).
- `SPEC.md` §6, *The allowlist*: the `allowlist-absent` sentences before "Within a run that has an allowlist"; the `vstd_version` bullet's first four sentences; the last two sentences of the `source_hash` bullet; the new *Build-input inventory* bullet; the *Build inputs* bullet (rewritten in full); the sentence beginning "The F1 prototype implements the commit"; the new *Required controls for the production tool* paragraph.
- `SPEC.md` §6, *What the audit cannot see*: the last sentence of the source-hash-scope bullet; the build-input-detection bullet (rewritten); the vstd bullet (rewritten); the new node-identity bullet.
- `SPEC.md` §7, O2.
- `SPEC.md` §8: the second and third sentences of the opening paragraph; the F1 entry's first paragraph (meaning and requirement); the caller-identity, provenance and build-input control bullets (their last sentences); the paragraph after the DIB control; *Known prototype defects the production tool must fix*; *Before the delta is frozen*; *Production implementation may start now; claims may not*; *Prerequisites for assurance claims*.
- `THEOREMS-ALLOC-DELTA.md`: header line; the preamble's trust-item sentences; §7 sequencing paragraph; step 0 row; the last three sentences of the "Path-insensitive authorisation" risk; the source-hash, build-input-detection and vstd clauses of "The audit's blind spots".
- `TABLE.md`: header line; README first paragraph, the two sentences after "That hash does not cover"; the note after the README text (prerequisite list and the sentences on what the prototype does); O2; the F1 gate entry (rewritten); the two paragraphs after the gate list.

Sol should check the defect descriptions against `proto/alloc_audit/src/{walk.rs,allowlist.rs,report.rs}` at the cited lines, and check that no passage still claims for the prototype what §8 lists as a defect.

## Tenth revision (inventory completeness)

Sol's last short read of the ninth revision raised one blocking wording defect. `SPEC.md` §6, *Build-input inventory*, said rustc's dep-info names every file the compilation read, and that dep-info, `cargo build -v`, `cargo metadata` and `Cargo.lock` together form a complete independent inventory of every build input (§9.6 above repeated it). They do not:

- a build script or proc macro can read files and environment variables through `std::fs`/`std::env` without any dep-info entry;
- `cargo metadata` describes packages, not the build units cargo executed or the outputs they generated;
- `cargo build -v` prints command lines, not the reads those commands perform;
- `Cargo.lock` content-hashes neither a mutable `path` dependency nor an already-built dependency artifact;
- comparing two incomplete lists does not make either complete.

Resolution, wording only (no gate verdict, theorem or `proto/` change; no Verus run). Dep-info is qualified as covering the file and macro-environment dependencies rustc **tracks**. The proof-report process must capture or prohibit every other read: build scripts and rustc (hosting the proc macros) run under sandboxed read tracing that records every file and environment read, with the environment reduced to the declared variables and unrecorded reads denied. It records the actual build-unit command lines and outputs, hashes every linked artifact and the source tree of every `path`/`git` dependency, and fails the report on any input it cannot account for. The mechanical comparison is restated as checking the report against that captured inventory, not as proving completeness. New required proof-report controls: `x9_build_script_untracked_read` and `x10_proc_macro_untracked_read`. In each, a real build script (or proc macro) reads an undeclared file and an undeclared environment variable through ordinary I/O. Each control must block the report, or the build must fail on the denied reads, and it must pass once both reads are declared. This supersedes §9.6's description of dep-info.

Changed passages: `SPEC.md` header; §6 *Build-input inventory* (rewritten from the dependency-crate clause on), the *Required controls* paragraph (new last two sentences), the *What the audit cannot see* source-hash-scope bullet's last sentence; §8 *Before the delta is frozen* and the inventory prerequisite. `THEOREMS-ALLOC-DELTA.md`: header; the preamble's inventory sentence; the source-hash clause of "The audit's blind spots". `TABLE.md`: header; the README's inventory sentence; the inventory item of the note after the README; the pre-freeze paragraph and the inventory prerequisite after the gate list.

The document set is **READY FOR OWNER FREEZE**, not frozen.

## Eleventh revision (no_std)

The owner wants the verified core libraries certified `no_std`. Evidence: `proto/nostd_probe/NOTES.md` (feasibility probe; audit-rule design in (d)); `tga/NO_STD.md` and `icns/NO_STD.md` (real conversions: TGA 947 and ICNS 315 verified, 0 errors, both unchanged, under vstd `default-features = false, features = ["alloc"]`; `cargo tree` closure `vstd [alloc]` → `verus_builtin`; `-Zls=root` on the release rlib lists only `core`, `alloc`, `compiler_builtins`, `vstd`, `verus_builtin` plus two `MacrosOnly` crates; lib builds on `x86_64-unknown-none`, `thumbv7m-none-eabi` and `i686-unknown-linux-gnu`). Both notes state as not certified: linking or running, a global allocator or panic handler, verification on any host but x86_64, release-profile or `panic=abort` bare-metal builds, and the std-using tests and examples. No theorem, gate verdict or `proto/` file changed, and no Verus run was made for this revision.

Changes:

- `SPEC.md` §9 (new): crate form; feature unification (every crate in the graph sets `default-features = false` on vstd); the ghost-only `Vec` import that `cargo build` cannot validate; the guard builds with `core` + `alloc` only; the audit's crate-origin rule (`foreign-crate-instance`, `std-instance`, `kind = "crate"` entries, ICO admitting `dib_verus`; controls `x11`/`x12`), a production requirement not yet implemented; the certification checks with what each does not show; consumer obligations (`#[global_allocator]`, panic handler; in `no_std`, `handle_alloc_error` panics, which matters only if A-STD-2 fails); bare-metal and i686 builds as compile evidence only; abort shims, gdk-pixbuf and glycin loaders outside the boundary; prerequisites for a published claim.
- `SPEC.md` §3: the prototype's `AllocSite` is in `sites.rs` (`guard_proto/src/alloc_guard/sites.rs:13`), re-exported by `mod.rs:20`. That re-export list names prototype element types, so the production `mod.rs` re-exports only `AllocSite` to stay byte-identical across cores. New finding in this revision; the earlier text did not state it.
- `SPEC.md` §6 mechanism step 4 (cross-reference to §9).
- i686: the std component is installed now, so F1/F2 on i686 are no longer blocked by the toolchain. The i686 allowlist and layout declaration are still to be done. "unaudited" becomes "installable, not yet audited" in `SPEC.md` §3, §6 *Targets*, the §6 closing line, §8 F1 and F2, and post-freeze hardening; in `THEOREMS-ALLOC-DELTA.md` §5 and §7; and in `TABLE.md`'s README first paragraph and its F1/F2 entries.
- `THEOREMS-ALLOC-DELTA.md` §8 (new) and `TABLE.md` README: a no_std paragraph claiming only build-time facts. `TABLE.md` gains a note on when the paragraph may be published (TGA and ICNS now; DIB and ICO after conversion) and on why the "no std reachable" wording waits for the audit rule.

**Passages for a short Sol read** (changed text only):

- `SPEC.md`: header line; §3, the four sentences after "D6's hash rule applies to that identical text" and the last sentences of the `global layout` paragraph and of the toolchain paragraph (i686); §6 mechanism step 4; §6 *Targets and profile*, the i686 sentences; the line after *What the audit cannot see*; §8 F1's last sentence, F2's last two sentences, *Before the delta is frozen*, the i686 hardening bullet; **§9 in full**.
- `THEOREMS-ALLOC-DELTA.md`: header line; §5 *Expected sizes per target*, last sentence; §7, the "What remains before the owner's freeze" sentence and the last two risks (i686, no_std drift); **§8 in full**.
- `TABLE.md`: header line; the abort-shim row's first cell; README first paragraph, the "audited targets" parenthesis; the README **no_std** paragraph; the publication note after the README note; F1's last sentence; F2's last sentence; the pre-freeze paragraph; "Remaining open items".

Sol should check:

- that no passage claims more than `tga/NO_STD.md`/`icns/NO_STD.md` measured, in particular that neither a bare-metal link nor any non-x86_64 verification is implied;
- that the table in §9 states correctly what each check does not show;
- that the §9 audit rule is clearly marked as an unimplemented production requirement;
- that the `handle_alloc_error` sentence in §9 *Consumer obligations* is right for the pinned 1.99.1 `alloc`.

The document set is **READY FOR OWNER FREEZE**, not frozen; the freeze is the owner's act.

## Twelfth revision (wording)

Sol's short read of the eleventh revision raised one HIGH, three MEDIUM and one LOW finding, plus one audit-rule gap. All are applied as wording edits. No theorem, gate verdict or `proto/` file changed, and no Verus run was made.

1. HIGH: the publishable no_std paragraph (`THEOREMS-ALLOC-DELTA.md` §8; `TABLE.md` README) said the allocation audit ran on 64-bit x86 Linux. No call-graph audit has run on the converted TGA or ICNS cores: ICNS has no audit script, and F1's call-graph run was on pre-delta DIB. Both paragraphs now say only the verifier and the tests ran there, and that the production call-graph allocation audit has not run on the crate on any target. `TABLE.md`'s publication note states why the paragraph names no audit.
2. MEDIUM: `TABLE.md`'s no_std paragraph said "`OutOfMemory` then means that allocator returned null", which none of §9's five build checks shows. The sentence is deleted there; it stays in the allocation-failure claim (*When "graceful" actually happens*), publishable only once that claim's prerequisites hold, and the publication note says so.
3. MEDIUM: `SPEC.md` §6 *Targets and profile* and §9 *Targets* said the Verus theorems quantify over 32- and 64-bit `usize` and that 32-bit behaviour is covered by the width-generic proofs. Both now say the theorem statements contain no explicit width-specific layout assumption, but 32-bit verification and behaviour remain unchecked: the verifier ran on x86_64 only, and the bare-metal and i686 builds establish compilation, not proof or execution (`tga/NO_STD.md:48`, `icns/NO_STD.md:49`). §9 *Targets* also stops listing the call-graph audit among checks run on x86_64 for these cores: it has run only as the F1 prototype on pre-delta DIB.
4. MEDIUM: `SPEC.md` §9 table, `#![no_std]` row: *Shows* now says the attribute removes the implicit std prelude and `extern crate std`, so unimported `std::` paths fail to resolve; *Does not show* says an unguarded `extern crate std` re-admits std, which a distinct source check catches.
5. LOW: `SPEC.md` §9 *Consumer obligations*: "in a `no_std` build" becomes "in a final binary without std and without a custom allocation-error handler, under the ordinary panic configuration", where the default handler panics through `panic_nounwind_fmt` into the consumer's panic handler. A `no_std` core inside a std-linked binary gets std's aborting default, a custom handler can differ, and in no configuration does an A-STD-2 violation produce `OutOfMemory`.
6. Audit rule: `SPEC.md` §9 now requires the `kind = "crate"` exception to fail with `allowlist-ambiguous-crate` when the crate name resolves to more than one crate number (two versions, or a renamed dependency sharing the name), so the exception never reaches a namesake; control `x12` gains that ambiguity variant.

Status lines updated in all four files (`SPEC.md` header and §8 *Before the delta is frozen*; `THEOREMS-ALLOC-DELTA.md` header and §7; `TABLE.md` header and pre-freeze paragraph; this file's opening and *Remaining open items* 1a).

The document set is **READY FOR OWNER FREEZE**, not frozen. The owner approved the freeze on 2026-11-12 pending the no_std work; the coordinator records the freeze with hashes.

## Remaining open items

Before the freeze:

1. Done: Sol's last read of the ninth revision; its one blocking correction is applied in the tenth revision. The document set is READY FOR OWNER FREEZE; the freeze is the owner's act.
1a. Done: Sol's short read of the eleventh-revision (no_std) passages; its findings are applied in the twelfth revision.
2. Rerun `prefix-reconstruction/reproduce.sh` against the final tool source: `reports/n11_pre_fix.txt` was written at 07:01:42, before the last `src/` edit at 07:04:44, while every other archived report postdates that edit. (Evidence hygiene; it does not change the F1 meaning stated in the ninth revision.)

The prototype controls still missing at the eighth revision (absent `source_hash` key, absent `reviewer` key, `allowlist-vstd-undetermined`, `undeclared-proc-macro`) no longer block the freeze; the first two are on the production tool's required-control list, and controls added in `proto/` after the eighth revision are evidence for the prototype only.

After the freeze (production implementation may start now; claims may not):

- Prerequisites before any DIB proof report or README assurance claim (`SPEC.md` §8), none of which holds yet: the production audit tool fixing the seven known prototype defects and passing §6's required controls (`x1`–`x8` and the missing-key fixtures); the independently captured build-input inventory in the proof-report process (dep-info for rustc-tracked dependencies, read tracing with denial for build scripts and proc macros, build-unit records, artifact and source-tree hashes), passing `x9`/`x10`; a production allowlist signed by a named human reviewer, with every `[meta]` field and the post-change DIB `source_hash`, passing without `--prototype` and without the vstd override; production `try_zeroed` with `#[inline(never)]`.
- Implementation obligations (`SPEC.md` §8): nested-`Result` `import_dib` mapping and `g_dib_arm` proof; per-core production `Entry` layout re-check.
- `neg2`'s second route fixed or removed in `proto/`.
- Post-freeze audit hardening (`SPEC.md` §8), now including a report of unused allowlist entries.
- The production DIB audit run on the `try_*` roots, once the live `dib/dib_verus` rewrite builds, with a signed allowlist (the F1 DIB run used `pinned-dib/`).
- i686: the std component is installed; write and review the i686 allowlist, declare and check the i686 `Entry` layouts, and run F1/F2 there. Until then i686 is installable, not yet audited.
- no_std (`SPEC.md` §9): convert DIB, and ICO once its source is live, each with an unchanged verifier count and the §9 checks; implement the audit's crate-origin rule with controls `x11`/`x12`; optionally link and run a bare-metal binary, which no current evidence covers.
- Consumer migration to `try_*` or a `<core>_abort_shim` crate (`SPEC.md` §1.4): gdkpixbuf-loaders `bmp.rs`; glycin fork `glycin-dib`, `glycin-tga`, `glycin-icns` and their tests; DIB/TGA/ICNS/ICO conformance and corpus tests; `leniency/bmp`, `leniency/shared`; `tools/diff-harness`.
