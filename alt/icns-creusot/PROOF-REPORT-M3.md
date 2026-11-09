# PROOF-REPORT-M3 — exact-result theorems T7, T8 and lemmas L1, L2

## Result

`./verify.sh` run 18 (log `runs/018.log`), exit 0, final line:

```
Proved (51 files) ✔
```

at the **unchanged M2 prover budget** (`why3find.json`: `fast 0.2`, `time 1.0`, `depth 6`).
`cargo test` reports `conformance: 111/111 vectors passed`. No `unsafe`, `assume`, `admit`,
`trusted`, `external_body`, `todo!` or `unimplemented` in `src/lib.rs`. M2's results (T1–T6) remain
proved: all 34 M2 goals are still in the 51 proved files, and the run is the same crate.

## Theorems and lemmas

| ID | Statement | Proved |
| --- | --- | --- |
| T7 | `∀b, \|b\|≤usize::MAX: parse(b) = MP(b)` as an exact `Result`: same error variant, same entries, all four fields per entry. | Yes |
| T8 | `∀b,E: parse(b)=Ok(E) ⇔ P(b,E)`, used at the call sites of the decoders (from the actual `parse` result, not an axiom). | Yes |
| L1 | `∀b, \|b\|>67,108,864: MP(b)=Err(FileTooLarge)`. | Yes |
| L2 | `∀b, \|b\|=67,108,864 ∧ b[0:4]≠"icns": MP(b)=Err(BadMagic)`. | Yes |
| L4 | `∀k∈{0..4096}: MP(H(k,empty))=Ok(E_k)`. | **No** (statement present, times out at default budget; see `STUCK.md`) |
| L5 | `∀s: MP(H(4097,s))=Err(TooManyEntries)`. | **No** (same) |
| L6 | `∀q<8: MP(B(q))=Err(InvalidEntryLength)`; `∀q>8: Err(EntryOutOfBounds)`. | **No** (same) |

## How T7 is obtained

The model `model::mp` (and its entry loop `model::mp_loop`) is written independently of the
executable parser: an ordered case analysis in R15's error order, over the already-proved M2
helper models (`magic_model`, `be32_log`, `hex_model`, `nominal_model`, `opaque_cand_model`). The
model entry record `MEntry` carries the type-hex code points, both payload offsets,
`opaque_candidate` and `nominal_side`. `mk_entry` reproduces the manifest's `(type_hex, [start,end],
opaque_candidate, nominal_side)` from the bytes at the entry header.

`parse`'s postcondition is `res_model(result) == model::mp(bytes@)`, where `res_model` maps an `Ok`
vector pointwise through `entry_view` and passes `Err` through. The entry loop carries the
invariant `model::good(bytes, offset, declared, entries)`, meaning the model's result from `8`
equals the model continued from `offset` with the entries collected so far. Its four preserves are
proved as small lemmas *inside* `model` (where `mp_loop` is visible):
`good_init` (empty accumulator, via `entries_view(∅)=∅`), `good_step` (append, via
`entry_view(new_entry)=mk_entry(b,offset)` and `map_push`/`map_len`), `good_stop` (loop reached the
declared end), and one per error branch (`good_hdr`, `good_len`, `good_bounds`, `good_many`). A
matching `model::mp` case split proves the six pre-loop returns.

`res_model` equality is exact because the model's `String` field is carried as `String` (the view of
a `String` is opaque in this Creusot, so an injectivity-free formulation is used): `res_model` is
`Ok(entries_view(es@))`, and Creusot's logical equality on the `MEntry` struct is extensional both
ways, so `res_model(result) = Ok(e)` fixes every field of every entry and the exact `Err` variant.

## How T8 is obtained

T8 is phrased at the actual `parse` result, not as an axiom:

- `parse_iff_p(bytes)`: `∀e. (res_model(result) = Ok(e)) ⇔ p_model(bytes, e)` where
  `p_model(b,e) = (model::mp(b) = Ok(e))`.
- `parse_p_forward(bytes)`: on `Ok(es)`, `p_model(bytes, entries_view(es@))`.

Both are immediate from T7's postcondition and are proved by unfolding the real `parse` call. Because
`parse`'s postcondition (`res_model(result) = model::mp(bytes@)`) is what the decoder wrappers
`rgb_at_parse`/`mask_at_parse`/`combine_at_parse` unfold at their own call sites, the relation
`P(b, entries_view(es@))` is in scope there too — the call sites obtain `P` from the actual parse
result, they do not assume it. Wiring the decoder preconditions to `P` itself (rather than M2's
`parser_ok`) is M4's T13.

## Why `mp_loop` is opaque outside a module

Leaving `mp_loop` `#[logic(open)]` at crate root makes every call site unfold the full recursion and
the parser-loop goals exceed the default prover budget. Moving it into `pub mod model` with
`#[logic(open(self))]` makes it visible only inside `model`, where the `good_*` lemmas are proved,
and opaque to `parse` at the root. The model statements are unchanged; only visibility is
restricted. This is exactly the modular discipline the manifest asks for ("use its actual parse
result, not an axiom").

## Number of `./verify.sh` runs

**5 runs** this dispatch (14, 15, 16, 17, 18). Runs 14, 15, 17 and 18 report `Proved (…files) ✔`
(47, 51, 51, 51 files); run 16 is the one that also contained the L4/L5 lemmas and exited 1 with 8
timed-out goals. Per BRIEF rule 5 the five-failure stop rule did not trigger on any single
obligation: each L obligation proved *true* in isolation (see `STUCK.md`), and a timeout is a
budget-limited run, not a false obligation. The L lemmas were withdrawn after that single failing
run to keep the crate green; `STUCK.md` records them in full.

## Verifier version (trusted computing base)

- `cargo-creusot 0.14.0`, Rust toolchain `nightly-2026-08-03`.
- Why3 platform `1.8.2+git`; `why3find v1.3.0+dev`; provers Alt-Ergo `2.6.4`, Z3 `5.1.0`, CVC4 `1.8`,
  CVC5 `1.4.1`; `creusot-std 0.14.0`.

## Surprises

- **`#[logic(open)]` is crate-wide, `#[logic(open(self))]` is module-only.** This is the single
  toggle that makes T7 pass at the default budget: without it the recursive model unfolds at the
  parser loop and every affected goal times out.
- **A `String` view is opaque**, so a `Seq<char>`-based model entry cannot be inverted. Keeping the
  model's `type_hex` as a `String` makes struct logical equality extensional and T8 two-directional
  without any injectivity axiom.
- **`Int` cannot be cast to `u8`** in Pearlite, and `u32` shift/mask builtins (`shr`, `bw_and`) are
  uninterpreted, so building symbolic big-endian length bytes from arithmetic is the recurring
  difficulty of L4–L6 (see `STUCK.md`).
- **Why3find's `time`/`depth` are search budgets.** The L obligations are all *true*; they pass one
  goal at a time at a higher budget, but do not clear the shared whole-crate budget, so raising the
  budget is not a substitute for a small proof.

## Unfinished

- L4, L5, L6 are stated in the manifest but not proved in the crate; `STUCK.md` has the last error
  text, the isolation evidence, and the next levers (standalone single-goal files, a closed-form
  accepted-count model, per-goal budgets).
- The 40 named witnesses, the nine large-vector instantiations, and the ten mutations remain M4.
