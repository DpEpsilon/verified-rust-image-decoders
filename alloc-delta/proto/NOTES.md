# Prototype verdict: gate F3 (sealed log across modules and crates) and gate F2 (`Entry` layout)

Workspace: `~/projects/verified-decoders/alloc-delta/proto/`. Every verifier run
goes through `verify.sh` and is numbered in `RUNLOG.tsv` (144 runs, 20 exit 0,
124 exit 101). Run logs are never deleted or renamed. Negative-check outcomes are
in `NEGATIVE.tsv`. Toolchain: rustc 1.99.1 (`58cf8f9c3`, 2026-10-13), Verus
`0.0.0-2026-11-01-0251`, vstd pinned at the same version, `cargo --offline`.
Nothing outside `alloc-delta/proto/` was edited. `proto/alloc_audit/` is a
concurrent agent's gate-F1 work; this prototype neither reads nor writes it.

## Verdict

**Gate F3 passes; SPEC.md §1 and §3 need the four small additions listed at the
end.** Verus accepts a
`tracked` struct whose single ghost field is private to the module, read from
outside through a `pub closed spec fn view`, and it accepts ICO's `import_dib`
consuming DIB's `Tracked<AllocLog>` across the crate boundary. Both crates verify
to zero errors:

| Crate | Contents | Verified |
|---|---|---|
| `guard_proto` | `alloc_guard/mod.rs` (sealed log, `OutOfMemory`, `TryError`, `try_zeroed`, `try_copy`, `try_with_room`, `Reserved<T>`) and `alloc_guard/sites.rs` (`AllocSite`, `Entry`, `IcoEntry`, `import_dib` shapes); `toy.rs` (DIB-shaped decoder, theorem shape `G`, `Prog`, `PreAlloc`); `toy_entries.rs` (ICNS `EntryList` shape, zero-entry case) | 44 verified, 0 errors |
| `guard_proto_dep` | ICO's own sealed `AllocLog`, `ICOSite = EntryList | Dib(..)`, `map_dib`, the three `map_dib` lemmas, A-ICO-2 as one lemma per arm, and `import_dib` | 22 verified, 0 errors |

**Gate F2 passes for x86_64 and is not available for i686.** `global layout`
declarations for `icns::Entry` (28, 4) and an `IcoEntry` (44, 4) are written,
accepted, and rustc checks them as static assertions (a deliberately wrong
declaration is rejected with `E0080: evaluation panicked: does not have the
expected size`). The i686 target is not installed: `rust-std-i686-unknown-linux-gnu`
is absent, so `--target i686-unknown-linux-gnu` fails with `E0463: can't find
crate for 'core'`. SPEC.md §6 already records this; it is reproduced here.

## The accessor clause SPEC.md needs

`OutOfMemory`'s accessors are specified in SPEC.md §1 as
`#[verifier::when_used_as_spec]` exec functions with spec counterparts, and §2
and `THEOREMS-ALLOC-DELTA.md` write `o.site()` and `o.elements()` inside `ensures`
clauses in this and other crates. That works, and this prototype uses it
throughout. Two mechanical facts about it are not in the spec and must be:

1. **The spec counterparts cannot be `open`.** A `pub open spec fn` body must be
   well-formed everywhere, which includes outside `alloc_guard`, where the fields
   are private. Verus rejects an `open` body that reads a private field with
   "disallowed: field expression for an opaque datatype", even when the function
   is declared inside the owning module. The counterparts must be `closed`, which
   is exactly the sealing §2 asks for. In this prototype they are the associated
   `pub closed spec fn site_spec(&self)` / `elements_spec(&self)`.
2. **The `when_used_as_spec` target must be a sibling item of the same `impl`
   block** ("unable to find sibling impl item ..."), and its parameters must match
   the exec function's receiver ("when_used_as_spec function should have the same
   parameters"). A receiver-less `pub closed spec fn site_spec(o: OutOfMemory)`
   does not satisfy this.

Consequence for §3's guard text: SPEC.md §1 should say the spec counterparts are
`closed` associated functions, and §3's listing of the accessor pair should show
them. No theorem shape changes: `o.site()` in a spec clause resolves to the
closed counterpart, proven inside the module where the field is visible.

## Two mechanical facts about `tracked` structs that SPEC.md's guard text should state

The guard bodies as SPEC.md §3 writes them are executable `fn`s that begin
`let tracked l; remember(...)`. Three spellings were needed; each is load-bearing.

1. **A reservation function takes `Tracked(log): Tracked<&mut AllocLog>`
   destructured in the parameter list**, not a named `log: Tracked<&mut AllocLog>`
   parameter. `old(log)` and `final(log)` then refer to the tracked *place*, and
   this is the only spelling that lets the function write the log or pass the
   reference on. With a named parameter, `old(log)` and `final(log)` are not
   even well-formed (`E0308: expected &mut _, found Tracked<..>`).
2. **Writing the private field must sit in a `proof { .. }` block:**
   `proof { log.ev = log.ev.push((site, n as nat, ok)); }`. A bare field
   assignment in the exec body is rejected with "cannot mutate through a
   spec-mode mutable reference".
3. **The append cannot be factored into a shared exec helper.** A helper taking
   `Tracked(log): Tracked<&mut AllocLog>` cannot be called with the caller's
   destructured reference (`E0308: expected Tracked<&mut ..>, found &mut ..`),
   and a helper taking `&mut AllocLog` cannot touch the proof-mode place ("cannot
   access proof-mode place in executable context"). So each reservation function
   repeats the one-line `proof` block. SPEC.md §3's `#[inline(never)]` on every
   exec fn still holds; the append is one statement per call site, not a shared
   helper.

## What proved clean, and what needed proof work

`try_zeroed`, `try_copy`, `try_with_room`, `Reserved::push`/`into_vec`, the
`G` clauses, `Prog`, the `PreAlloc` corollary, A-ICO-2, and `import_dib` all
verify. The theorem shape of `THEOREMS-ALLOC-DELTA.md` §0 is exactly stateable:
`G` clause 1 (`Eq_f ∧ req(L) = Req ∧ ∀j L[j].ok`), clause 2 (last entry failed
and equal to the returned `(site, elements)`), clause 3 (the request is within the
site cap), `Prog`, and the public `try_*` `ensures` all type-check as written.

Two things cost proof effort and are worth knowing before the real cores:

- **The cross-crate import needs per-arm lemmas.** `import_dib` consumes both
  `r` and the linear `Tracked<DibLog>`, so a caller cannot re-derive DIB's `G`
  about them afterwards. The ICO shape therefore has to be an `ensures` of
  `import_dib` itself, stated over `out` and `final(log).view()` only. Proving it
  in one shot did not go through; it went through as one `proof fn` per arm of
  DIB's result (`lemma_g_ico_ok`, `lemma_g_ico_format`, `lemma_g_ico_oom`),
  invoked in the corresponding `match` arm. The spec's A-ICO-2 proof sketch is
  arm-by-arm already, so this is a phrasing change, not a shape change.
- **`OutOfMemory`'s sealing bites the proof too.** The `OutOfMemory` literal may
  appear only inside `alloc_guard` (or `sites.rs`, its child). An arm lemma that
  wants to name the returned value cannot construct it; it has to take the value
  as a parameter and state its shape through `site_spec`/`elements_spec`, or live
  in `sites.rs`. That is the intended property, and it makes A5/A17/A22's
  build-level half real rather than aspirational.

## Negative-check table

Each check is a source file in `negative/`, copied into its crate by
`negative/run_negatives.sh`, run through `verify.sh`, then removed. "PASS" means
the check failed to verify as designed. The failure reason was read out of each
run log; a check that failed for an unrelated reason counts as no evidence and
was fixed rather than recorded.

| Check | Crate | Expectation | Run | Exit | Failure mode observed | Verdict |
|---|---|---|---|---|---|---|
| `neg1_construct_oom_outside_guard` | guard_proto | `OutOfMemory` has private fields; constructing one outside `alloc_guard` is a compile error | 122, 126, 145 | 101 | `disallowed: constructor for an opaque datatype` | PASS |
| `neg2_edit_log_outside_guard` | guard_proto | `AllocLog.ev` is private; appending/replacing outside `alloc_guard` is a compile error | 123, 127, 146 | 101 | exactly one rustc error (run 146): `E0616: field 'ev' of struct 'AllocLog' is private`. Runs 123/127 also had an unrelated E0599 (`borrow_mut` not in scope) from a second route; that route was removed | PASS |
| `neg3_return_oom_after_ok` | guard_proto | returning `OutOfMemory` after the last reservation returned `Ok` violates `G` clause 2 | 124, 128, 147 | 101 | one Verus `postcondition not satisfied` on `g`, at the stashed-OOM exit only | PASS-verifier |
| `neg4_skip_import` | guard_proto_dep | dropping `import_dib` leaves `req(L) ≠ Req_D` and has no ICO `OutOfMemory` constructor | 125, 129, 148 | 101 | two Verus postcondition errors on `g_ico`/`prog_ico`; the OOM arm has nowhere to build an ICO `OutOfMemory` | PASS-verifier |

Two of the four are compile errors, two are verifier errors, which is the split
`THEOREMS-ALLOC-DELTA.md` §6 predicts for A5/A17/A18/A19/A22.

## F2 results

| Item | Result |
|---|---|
| `global layout Entry is size == 28, align == 4;` | accepted; rustc checks it as a static assertion |
| `global layout IcoEntry is size == 44, align == 4;` | accepted |
| `size_of::<Entry>()`, `align_of::<Entry>()` real values (x86_64) | 28, 4 (measured with rustc; matches the declaration) |
| `size_of::<IcoEntry>()`, `align_of::<IcoEntry>()` | 44, 4 (ICO `EntryStatus` is 12 bytes, so `44 = 2+2+1+4+4+ 2+2+2+2 + 12 + padding`) |
| Wrong declaration (`size == 32` for `Entry`) | rejected: `E0080: evaluation panicked: does not have the expected size` |
| Numeric bound provable from the declaration | `n ≤ 4096 ⇒ n · size_of::<Entry>() ≤ 114,688 < isize::MAX`; `n ≤ 1024 ⇒ n · size_of::<IcoEntry>() ≤ 45,056 < isize::MAX`; both proved in `lemma_room_fits` / `lemma_entry_layout`, with the products discharged by `nonlinear_arith` |
| The layout axiom's scope | usable **only inside the module that declares it**; other modules and other crates must go through an exported `proof fn` (`lemma_entry_layout`). SPEC.md §3 says the declaration discharges the call-site preconditions; it must add that the numbers leave the declaring module only through such a lemma, or the fallback `try_with_room_entry` must state them directly |
| `i686-unknown-linux-gnu` | not installed (`rustup target list --installed` has no i686 entry; no i686 `rust-std`, so `E0463: can't find crate for 'core'`). Matches SPEC.md §6 |
| Fallback `try_with_room_entry` | not needed; the `global layout` declaration works on x86_64 |

`size_of::<Entry>()` depends on the target (`Option<u32>` and `[u32; 2]` are
target-independent, but `iced`/`usize` fields in the real `ICO Entry` are not),
so the declaration, or the fallback's assertion, still needs one instantiation
per production target, as SPEC.md §3 says.

## Run counts

148 runs in `RUNLOG.tsv`; 20 exit 0, 128 exit 101. Runs 143 and 144 re-confirm `guard_proto` at 44 verified and `guard_proto_dep` at 22 verified. Runs 145-148 are the four negative checks re-run after `neg2` lost its second route. Most of the non-zero runs are
the recorded build and proof iterations. Twenty runs are negative checks (each
check is run several times as its file is corrected): runs 109–129, 139–142 and 145–148.
Runs 135 and 136 are the clean 44-verified and 22-verified runs; runs 137 and 138
re-verify the same two crates, and cargo skips the checker because nothing
changed, so their `RUNLOG.tsv` line has exit 0 and an empty summary — the
evidence line for the clean state is 135/136.

## Per-core hashes

`alloc_guard/mod.rs` and `alloc_guard/sites.rs` are separate files per crate here,
as SPEC.md §3 decides. Their SHA-256 values (record the same way the real delta
will):

```
f8ac624bfa55e584761ce7462a9a6951b116cee19b4be6b214dcdcd403a64c12  guard_proto/src/alloc_guard/mod.rs
8158937bc55ba9ae43aea7e21a704bee7f49f505851de49fd2327c2f92f61ddb  guard_proto/src/alloc_guard/sites.rs
eda0c43034126bb860d090703326e7352a57b9a895c6d922f17e4317beacd8fc  guard_proto_dep/src/alloc_guard/mod.rs
dd07411b3a4668a828aecfbc4f48d298249e69acc4a32b1a2839f1531116250b  guard_proto_dep/src/alloc_guard/sites.rs
```

Not frozen, and the two crates' `mod.rs` files necessarily differ (the
prototype's `AllocSite` enum lives in `sites.rs`, so `mod.rs` names the
per-core type only through `pub use sites::...`; in the real delta the four
`mod.rs` files are byte-identical and each core's `sites.rs` carries its own
`AllocSite`). The hash rule of D6 applies to that identical text.

## SPEC.md clauses that need changing

- §1: say the `site()`/`elements()` spec counterparts are `closed` associated
  functions (`fn site_spec(&self)`), and that `#[verifier::when_used_as_spec]`
  requires same-`impl`-block, same-parameter counterparts. (Finding above,
  "The one clause SPEC.md needs".)
- §3: state the three `tracked`-struct mechanics: destructured `Tracked(log)`
  parameter, the append in a `proof` block, and no shared append helper.
- §3: state that a `global layout` axiom is usable only inside its declaring
  module, so the numbers are exported through a lemma (or the fallback states
  them directly).
- §3.1: state that `import_dib`'s `ensures` must carry ICO's `G`/`Prog` over
  `out` and `final(log).view()` alone, because `r` and `dlog` are consumed; and
  that the boxed `OutOfMemory` value is built by `import_dib` (in the guard), with
  the arm lemmas taking it as a parameter.
- §8 F2: record that the layout declaration is checked by rustc (E0080 on a wrong
  one) and that i686 is unavailable in this image.
- §8 F3: record this verdict, and that the cross-crate half needed the per-arm
  lemma split, not a shape change.
