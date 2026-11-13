# ICO core — working notes

## M1 (plain implementation)

`ico_verus/src/ico.rs` implements R1–R10 as ordinary Rust outside `verus!`.
`src/lib.rs` keeps the cross-crate DIB skeleton (still verifying) and declares
`pub mod ico;`. `tests/conformance.rs` runs all 217 vectors: `parse` vectors
compare the whole R1 record list (typed record rendered back to JSON, so every
field — including absent optionals as `null` — is compared); `decode` vectors run
`parse` first unless `entries_override` is supplied, then `decode_entry`, and
compare exact dimensions and RGBA bytes or the exact error name (`Dib(ErrorName)`
for the DIB wrap).

Arithmetic: `dir_end = 6 + 16*count` and `offset + size` are computed in `u64`;
the range test is `offset > |b| || size > |b| - offset`, so a `u32` wrap
(`wide-sum-wrap`) cannot pass. Reads go through guarded helpers that bounds-check
before indexing; no `unsafe`, `unwrap`, `expect` or `panic!` is reachable.

Two vectors expand past the 64 MiB cap (`file-size-67108865`,
`too-large-before-bad-header`). R2 tests the length before reading, so the
harness materialises a zeroed buffer of the exact expanded length and requires
`FileTooLarge` from both entry points; it never builds the under-cap 64 MiB
fixture (`file-size-67108864`), which is decided by the `len > MAX_INPUT` test
alone.

Result: `conformance: 217/217 vectors passed`.

## M2 (verus! with contracts and proof)

`ico_verus/src/ico.rs` is now inside `verus!`; `src/ico.rs.m1` preserves the M1
plain file. `ico_verus/src/model.rs` holds the independent reference model
`H`/`ME`/`MC` (over `Seq<u8>` and mathematical integers), the equality model
(`opt_u16_eq`/`status_eq`/`entry_eq`), the decode-side decision `md_ico` and the
arithmetic lemmas; nothing in it calls the executable. `lib.rs` declares
`pub mod model;`.

Proved: T1 (safety of `parse` and the ICO side of `decode_entry`), T2 (`parse`
equals `MC` as the whole `Result`), T3 (record validity), T4 (signature
classification on valid entries), T7 (arithmetic and output bounds) and their
witness lemmas (`t1_decode_ico`, `t1_index_u32`, `t2_parse_exact`,
`t3_record_validity`, `t4_signature_classification`, `t7_arithmetic_bounds`).
Deferred to M3: T5, T6, T8, the model `MD` and the DIB composition. The
`decode_entry` postcondition is `model::decode_pre`, which fixes the whole
ICO-controlled decision exactly and the DIB-result shape (success or
`Dib(_)`), but not the DIB bytes.

Executable behaviour is byte-identical (`cargo test --release` →
`conformance: 217/217 vectors passed`). Three proof-driven edits: the
`Option`-returning reads became preconditioned `rd8`/`rd16`/`rd32`; the M1
defensive `dir_end > u32::MAX` branch was dropped as provably dead; `reconstruct`
returns `Entry` and `parse` has no unreachable arm.

Clean full reverify: `verification results:: 79 verified, 0 errors` (after
`rm -rf ico_verus/target`). See `PROOF-REPORT-M2.md`.

## M3 (T5, T6, T8)

`ico_verus/src/model.rs` gains the total decode model `MD(b,E,i)` (R8/R11: the
ICO-controlled `md_ico` plus, on the selected valid DIB branch,
`dib_verus::model::mp(b.subrange(start,end), IcoEntryDibSlack)` mapped to
`Ok(dib_wrap)`/`Err`), the exact statement `md_eq`, the composition discharger
`lemma_md_eq_dib`, the T6 precedence predicates/lemmas and the T8/T7 lemmas.

`decode_entry`'s postcondition is now `model::md_eq`: exact for header/guard
errors, `IndexOutOfRange`, the forged-record `InvalidEntryRecord` check, the
entry defects, `PngDelegation`, and the composition with the DIB model on the
validated declared slice. The selected-DIB branch slices exactly
`[payload[0], payload[1])`, proves `slice@ == bytes@.subrange(start,end)`, calls

    let (r0, _steps) = dib_verus::decode_with_steps(slice, Mode::IcoEntryDibSlack);

once, and discharges `md_eq` from the returned `decode_mp_eq`. This is the DIB
1.1 public tracked call M4a's bound needs, called now so M4a leaves the executable
alone; `_steps` (a `Ghost<nat>`) is discarded and erases, so behaviour is
byte-identical.

Proved: T5 (exact total `decode_entry` incl. the composition), T6 (exact
precedence on both public calls; decode reads only `entries.len()` and
`entries[index]`), T8 (variants and the count-cap-returns-before-loop fact), plus
the T7 clause M2 deferred (`4*w*h <= 67,108,864` on a DIB success). Witnesses:
`t5_decode_exact`, `t6_precedence`, `t6_precedence_full`,
`t6_parse_directory_priority`, `t6_decode_ordering`, `t7_dib_output_bound`,
`t8_termination`; `t1_decode_ico` now discharges from `md_eq` via
`lemma_md_eq_implies_decode_pre`.

Three clean sensitivity kills (disposable copies): skip the selected-record
comparison, skip the list-length check, and pass `bytes[start..]` instead of the
declared slice. See `PROOF-REPORT-M3.md`.

Clean full reverify: `verification results:: 101 verified, 0 errors` (after
`cargo clean`). Executable behaviour byte-identical (`cargo test --release` →
`conformance: 217/217 vectors passed`).

## M4a (T9, T10, T11)

A ghost step counter (`&mut Ghost<nat>`) is charged on every ICO-controlled loop
body: the record loop, the eight-byte signature comparison, and the bounded list
initialisation (`parse_step_bound`); the wrapper charges its single signature
comparison. `parse_steps`/`parse_tracked` and `decode_entry`/`decode_entry_impl`
are split so the public bodies stay `parse`/`decode_entry` (now thin wrappers) with
unchanged postconditions; new public tracked exports `parse_with_steps` and
`decode_entry_with_steps` state `steps <= 16*(|b|+1)` for every `b,E,i`. The
combined bound `t9_step_cap = wrapper_own_bound + dib_steps_bound` is stated on the
executable, using the DIB 1.1 `decode_with_steps` published bound
`steps_DIB <= 16*(|slice|+N)` on the same call; off the selected-DIB decision no DIB
call is made.

A ghost allocation counter charges `Vec::with_capacity(count)` once in
`parse_tracked` (zero on a directory error) and nothing in the wrapper; labelled
honestly that it cannot catch an uncharged allocation. `tools/audit_alloc.sh`
(grep of the allowed site) is the enforcement and fails on injected
`vec!`/`Vec::with_capacity`/`to_vec`/`clone`/`collect`.

T11: `src/model_vectors.rs` (generated by `tools/gen_model_vectors.py`, expected
values from `spec-frozen/VECTORS.json` only) proves `mc`/`md` equal to the fixture
`Result` for every small vector, closed by `by (compute)`; 41 DIB-branch decode
vectors are excluded by the manifest's "no concrete DIB MP" rule and stay runtime.
`src/witnesses.rs` proves three public `decode_entry` calls (valid selected beside
invalid unselected → `PngDelegation`; forged selected → `InvalidEntryRecord`;
forged unselected ignored). All 31 named non-vacuity witnesses have proof items;
`src/model.rs` proves the symbolic L1-L4 lemmas.

Clean full reverify: `verification results:: 303 verified, 0 errors` (after
`cargo clean`). Executable behaviour byte-identical (`cargo test --release` →
`conformance: 217/217 vectors passed`). See `PROOF-REPORT-M4a.md`.

## M4b (executable-only mutation campaign)

All 33 manifest mutations (M1-M33) were run on the implementation
(`tools/mutations.py` edits ONLY `src/ico.rs` in a disposable copy rebuilt from
`mutation/base/ico_verus`, a pristine byte-identical copy of the live crate;
driver `tools/run_mutations.sh`; one unattended background batch). Result:
**32 verifier kills** (postcondition / precondition / assertion / loop
invariant, never a rustc error), **1 audit kill** (m21: an uncharged `vec!` the
verifier accepts at 303/0 but `tools/audit_alloc.sh` fails), and **1 weak kill**
(m17: rlimit-only). M17 was reworked as m34 (a concrete wrong DIB slice end),
killed on a real precondition. Live crate unchanged; clean reverify 303/0; tests
217/217. See `PROOF-REPORT-M4b.md`.

## alloc-delta (fallible allocation, no_std, ICO step)

Working tree: `dib-policy`, uncommitted. The complete ICO source was copied in
from `origin/main`; the `*.m1`/`*.m2`/`*.m3`/`*.m4b` files are historical
snapshots and are not built. Backups of the pre-change sources are the
`*.alloc-0` files beside each source.

### Baseline

The unmodified crate depends on the OLD `dib_verus` API (`decode_with_steps`),
which the DIB crate no longer exports (it migrated to `try_*`). To confirm the
baseline count I built a disposable copy at `~/projects/vd-baseline/` with the
committed pre-migration DIB (`~/projects/vd-commit/dib/dib_verus`) as the path
dependency: **run 1, 303 verified, 0 errors** (164 s), matching `LEDGER.md` and
`PROOF-REPORT-M4b.md`. The live tree is then migrated to the new DIB.

### Step A: no_std + alloc conversion

`Cargo.toml` sets `vstd = { version = "=0.0.0-2026-11-01-0251",
default-features = false, features = ["alloc"] }`. `src/lib.rs` starts with
`#![no_std]`, `extern crate alloc;`, and `#[cfg(test)] #[macro_use] extern
crate std;`. `src/ico.rs` and `src/model.rs` import `alloc::vec::Vec` (the
`Vec` named only in spec/proof signatures still needs the import; a plain
`cargo build` erases ghost code and would not catch its absence). No verified
item, spec or proof changed.

Verified in a disposable copy against the old DIB: **run 1, 303 verified, 0
errors** (174 s), identical to the baseline. Applied to the live tree.

### Step B: the production `alloc_guard` for ICO

`src/alloc_guard/mod.rs` is a byte-identical copy of DIB's and TGA's
(SHA-256 `018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6`).
It carries the sealed `AllocLog`, `new_log`, `OutOfMemory` with private fields
and `when_used_as_spec` accessors, `all_ok`, `req_of_log`, `MAX_BYTES`, and
`#[inline(never)] try_zeroed`.

`src/alloc_guard/sites.rs` is ICO's per-core child module. It carries
`AllocSite { EntryList, Dib(dib::AllocSite) }`, `TryError { Format(Error),
OutOfMemory(..) }` with `name_spec`/`name`, `fmt_part`/`fmt_part_decoded`,
`log_tied`/`log_tied_decoded`, the request models `req_p`/`req_d`, the theorem
shapes `g_parse`/`prog_parse`/`g_decode`/`prog_decode`, the `map_dib` lemmas,
`import_dib` with the production nested-`Result` return type, and the arm
lemmas. Two design points:

- **`try_with_room` and `Reserved<T>` live in `sites.rs`, not `mod.rs`.** The
  shipped `mod.rs` (hash-pinned, byte-identical across DIB and TGA) carries only
  `try_zeroed`: DIB and TGA each make one byte-oriented reservation, while ICO's
  entry list is a `Vec<Entry>`. Putting the element-list reservation in the
  per-core child module keeps `mod.rs` byte-identical (D6) and keeps every
  allocation inside `alloc_guard` (SPEC §3, §6). `MAX_ROOM = 4,096` is declared
  there too.
- **`global layout Entry is size == 44, align == 4;`** is declared in
  `sites.rs` and checked by rustc as a static assertion. `Entry` has no
  pointer-sized field, so the layout is the same on x86_64, i686, thumbv7m and
  x86_64-unknown-none (measured with rustc on all four). The numbers leave the
  module through `lemma_entry_layout`.

`import_dib` consumes DIB's `Tracked<AllocLog>`, writes `map_dib(dlog.view())`
into ICO's log, and returns the nested
`Result<Result<Decoded, dib::Error>, OutOfMemory>`. Its `ensures` carries
`g_dib_arm`/`prog_dib_arm` over `out` and the final log only, because `r` and
`dlog` are consumed. The proof is arm by arm (`lemma_g_ico_ok`,
`lemma_g_ico_format`, `lemma_g_ico_oom`), as the prototype required.

### Step C: the `try_*` rewrite of `ico.rs`

The frozen names are gone (decision D1). The crate exports `try_parse`,
`try_parse_with_steps`, `try_decode_entry`, `try_decode_entry_with_steps`.

- `try_parse_logged` reserves the entry list with
  `try_with_room::<Entry>(EntryList, header.count)` after the directory guard,
  pushes each record through `Reserved::push` (the `len < room` precondition
  follows from the loop bound `i < count == room`), and returns the log. The
  ghost allocation counter is replaced by the log: `r.2@ == L.len() <= 1`.
- `try_decode_entry_logged` calls `dib_verus::try_decode_with_steps_logged`
  once on the validated slice and passes its result and log to `import_dib`.
  The DIB-arm part of A-ICO-2 is `import_dib`'s `ensures`; the ICO-controlled
  arms (directory error, index, bad list, defect, PNG) use
  `lemma_g_decode_off_dib` with an empty `Req_D`.
- The public `try_*` functions create the log with `new_log()`, run the logged
  body, and discard the log. Their contracts state `G` existentially, the
  `OutOfMemory` site/size, and `md_eq`/`parse_eq` on `fmt_part` when the result
  is not `OutOfMemory`.
- The public witnesses `t5_decode_exact`, `t6_*`, `t7_dib_output_bound`,
  `t9_*`, `t10_allocation_bound`, `witnesses::t11_*` and `lib.rs::skeleton`
  stay `pub` with `TryError` contracts (O1). `skeleton` now calls
  `try_decode_with_steps` and states `decode_mp_eq` on `fmt_part`.

Verified on the live tree: **run 164, 343 verified, 0 errors** (174 s).

### Step D: tests

`tests/conformance.rs` is migrated to `try_parse`/`try_decode_entry` through
`fmt_part_parse`/`fmt_part_decode` helpers that treat `OutOfMemory` as a test
failure (the tests run without a fault-injecting allocator). All 217 vectors
pass.

`tests/fault_injection.rs` (with `tests/support/fault_alloc.rs`) is modelled on
DIB's serial fault harness. It runs every vector under eight allocation
schedules, at ICO's `EntryList` reservation (through `try_parse`) and at DIB's
`Output` reservation inside an ICO decode (through `try_decode_entry`, the
`Dib(Output)` event). It checks the exact OOM site and size, no partial output,
no retry, no reallocation, equal uninjected results, and zero live measured bytes
after drop. Observed: 217 cases, 117 allocating, largest reservation 45,056
bytes, 44 decode cases returning `OutOfMemory` at `Dib(Output)`. The 65,536 and
67,108,864 byte thresholds are no-failure controls (no vector reaches a DIB
output that large).

### Step E: audit tooling

`tools/audit_alloc.sh` is extended from DIB's/TGA's: it scans every production
module except `alloc_guard/` with `tools/alloc_scan.py` (proof-region stripping),
pins `alloc_guard/mod.rs` to the cross-core hash and `alloc_guard/sites.rs` to
ICO's own, and requires exactly one `try_with_room` call site, one `import_dib`
call site, one `dib_verus::try_decode_with_steps_logged` call site, and zero
`vec!` outside the guard. `Reserved::push`/`Reserved::into_vec` are whitelisted
by the receiver name `res` inside guard-using functions only (SPEC §6; see the
fixes section below). A mutation that injects an uncharged
`vec![0u8; 4]` into `try_parse_logged` is caught (scan exit 1).

### Step F: no_std builds

`cargo build --offline --lib` succeeds for `x86_64-unknown-none`,
`thumbv7m-none-eabi` and `i686-unknown-linux-gnu`. `cargo tree -e
normal,no-proc-macro` lists only `dib_verus`, `vstd [alloc]` and
`verus_builtin`. `rustc -Zls=root` on the release rlib lists `core`, `alloc`,
`compiler_builtins`, `vstd`, `verus_builtin`, `dib_verus` and two `MacrosOnly`
crates, no `std`. See `NO_STD.md` and `ico_verus/alloc-a5-dependencies.log`.

### Claim and lint fixes after Sol review

- `README.md`: the opening no longer says no implementation or proof exists; it
  states the verified Verus core (343 verified, run 167). The allocation
  paragraph no longer says every allocation passes through the guard as an
  established fact: it lists what ran (343 obligations, implementation and
  source-lint checks), what has not been produced (production call-graph
  allocation audit, independent build-input inventory, signed allowlist) and the
  trust boundary (A-STD, hash-reviewed guard text, future signed allowlist), per
  alloc-delta SPEC §8.
- `NO_STD.md`: run 164 updated to run 167 (343 verified, 0 errors, per
  `runs/167.log` and `RUNLOG.tsv`); crate version in the `cargo tree` excerpt
  corrected to 0.2.0.
- `tools/alloc_scan.py`: `res.push`/`res.into_vec` are suppressed under
  `--allow-reserved` only inside a function body that binds `res` from the guard
  (`Ok(res)`/`Ok(mut res)` or `let res = ..try_with_room(..)`, with a
  `try_with_room` call in that body). A `res: &mut Vec<u8>` parameter is flagged.
  New self-test controls: unrelated `res` parameter flagged (push and
  into_vec), local `res` Vec beside a stray `try_with_room` flagged, guard-bound
  `res` unflagged, and the real `ico.rs` clean under `--allow-reserved` yet
  flagged without it. Still a falsification gate, not enforcement: an unrelated
  second variable named `res` in the same guard-using function would be missed.
- `tests/fault_injection.rs`: the test already asserted `EntryList` on the parse
  path and `Dib(Output)` on the decode path (plus size). It cannot observe ICO's
  imported logical log, which is ghost; the header now says that mapping is
  covered by the verified `import_dib` postcondition and the runtime test checks
  the returned site and size only. Header "seven schedules" corrected to eight.

## Sol review fix — the two LOW findings (2026-11-12)

Two findings from Sol's review of the alloc-delta delivery. Both are
contract-strengthening with no executable change: the `name` bodies and the
`try_*` bodies are byte-identical, so conformance and fault injection are
unaffected.

### Finding 1: `Error::name` / `TryError::name` had no postcondition

Neither executable spelling accessor was tied to its `name_spec` table, so a
changed spelling would not have failed any of the 343 obligations (the spelling
is exercised only by the test binaries, in exec position).

Fix: an `ensures` on each accessor of the form `s@ == self.name_spec()@`, for
every variant except the one that delegates to another crate:

- `ico.rs::Error::name` — the tie holds for ICO's own 14 variants. The
  `Error::Dib(e)` arm delegates to `dib_verus::Error::name`, which exports no
  contract, so DIB's spelling is DIB's own R15 obligation and the `Dib` arm is
  left unconstrained (`match self { Error::Dib(_) => true, _ => s@ == .. }`).
- `alloc_guard/sites.rs::TryError::name` — same split; `Format(Error::Dib(_))`
  delegates through `Error::name`, so it is the one unconstrained arm.

No `#[verifier::when_used_as_spec]` was added, per the brief and the DIB
precedent (the attribute caused an unrelated per-function rlimit in DIB's
`raw_sample_scaled`; `dib/STUCK.md`, `dib/NOTES.md` alloc-delta). The `ensures`
is the spec-position tie instead.

Sensitivity, both against disposable copies (`VERIFY_CRATE`), each with the
copy's `target/` removed first:

- changing `FileTooLarge` to `FileTooLargeX` in `Error::name`: run 170, **346
  verified, 1 error**, postcondition not satisfied at `ico.rs:176`.
- changing `"OutOfMemory"` to `"OutOfMemoryX"` in `TryError::name`: run 171,
  **346 verified, 1 error**, postcondition not satisfied at
  `alloc_guard/sites.rs:69`.

### Finding 2: the exported PreAlloc corollaries asserted only `not OutOfMemory`

Frozen `THEOREMS-ALLOC-DELTA.md` §0 asks for a named corollary
`PreAlloc(x) ⇒ r is not OutOfMemory ∧ Eq_f(x, r) ∧ L == []`. The two exported
corollaries proved only the first conjunct.

Fix, in `alloc_guard/sites.rs`: the corollaries now state and prove all three
conjuncts, and two bridge lemmas connect the public `try_*` contracts (which
state `G` existentially) to them:

- `lemma_g_parse_prealloc(b, r, l)`: from `g_parse` and `Req_P(b) == []`, gives
  `!OutOfMemory`, `parse_eq(b, fmt_part(r))` (`Eq_f` for `parse`), and
  `l == Seq::empty()`.
- `lemma_g_decode_prealloc(b, es, index, r, l)`: the same three for
  `try_decode_entry`, with `md_eq` as `Eq_f`.
- `lemma_public_parse_prealloc(b, r)` / `lemma_public_decode_prealloc(b, es,
  index, r)`: take the public functions' existential `G` clause plus
  `Req == []` and yield the corollary's two result facts (`Eq_f` and
  `not OutOfMemory`); the log is existential in the public contract and is not
  returned, so the bridge states only what a caller can observe.

`L == []` follows from `req(L) == Req_P`/`Req_D` (clause 1) with `Req` empty and
`req_of_log(l).len() == l.len()`. The existing `*_prealloc_not_oom` lemmas are
kept and now called by the strengthened corollaries, so their callers
(`lemma_g_parse_format`, `lemma_g_decode_format`, the `witnesses.rs` entries) are
unchanged.

Verified: run 168, **347 verified, 0 errors** (343 + the four new function
obligations: two strengthened corollaries and two bridges; the `name` `ensures`
adds no new function, so it does not raise the count), conformance 217/217, fault
injection passing. `tools/audit_alloc.sh` OK with the new `sites.rs` pin below.

Final clean full reverify of the live tree, **run 172: 347 verified, 0 errors**,
exit 0, 168 s (`runs/172.log`). The machine was shared with the ICNS agent's
ongoing verifier campaign for the whole of this session's reverify; because that
agent's runs were contiguous, the clean reverify was run concurrently rather than
waiting indefinitely. Verus's per-function resource limit is a deterministic SMT
counter, not wall-clock, so a concurrent load slows a run but cannot change its
verdict; the counts match run 168 exactly.

### `alloc_guard/sites.rs` hash re-pin (D6)

The hash pin in `tools/audit_alloc.sh` moved with the two edits to `sites.rs`:

- old `EXPECTED_SITES_SHA` `b2cf3ee525a9f0cfeebffb2a6363120524b7fdb93bb7b5ff520b39ca8b2b5174`
- new `EXPECTED_SITES_SHA` `5c7b24a05052823eb33fdc22576dc97a84faa6aac2e17c742790b239e0767bc1`

`alloc_guard/mod.rs` is unchanged and stays at its cross-core hash
`018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6` (ERRATA E1).
The old sites hash remains recorded in `alloc-delta/ERRATA.md` E5 (frozen-adjacent,
append-only, not edited here).

Green-state backups (this session's `*.alloc-f<n>` convention, `n` = the green
run): `src/ico.rs.alloc-f172` (sha256
`6b60b834e53c75b62453860cf92a4887672a70916d148ac2df48419e6ec24025`) and
`src/alloc_guard/sites.rs.alloc-f172` (sha256 `5c7b24a0..`, the pin above).
`ico.rs` changes only in the `Error::name` doc and `ensures`: the exec body is
byte-identical to the pre-fix tree, and the alloc-delta `try_*` bodies are
untouched.
