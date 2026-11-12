# Sol review of the fallible-allocation delta: findings and resolutions

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

Open items after the second review were F1–F3; the current list is at the end of this document.

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

## Remaining open items

- F1: audit prototype on DIB, x86_64, with a reviewed allowlist; i686 unaudited until its std component is installed.
- F2: per-target `Entry` layout declarations checked by rustc.
- F3: Verus accepts the sealed log across modules and ICO's cross-crate `import_dib`.
- Consumer migration to `try_*` or a `<core>_abort_shim` crate (`SPEC.md` §1.4): gdkpixbuf-loaders `bmp.rs`; glycin fork `glycin-dib`, `glycin-tga`, `glycin-icns` and their tests; DIB/TGA/ICNS/ICO conformance and corpus tests; `leniency/bmp`, `leniency/shared`; `tools/diff-harness`.
