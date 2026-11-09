# STUCK — L4, L5, L6 (M3): symbolic lemmas for the nine large vectors

Per BRIEF rule 5. The stop rule applies to *repeated failed prover runs on the same obligation*;
here each obligation is *true and separately checkable*, but they do not clear the project's
prover budget. Recorded in full so a stronger model or a person can finish them.

## The obligations (statements are in the model, not weakened)

Let `icns4 = bytes("icns")`, `e1 = bytes("none") · BE32(8)`, `e1k(k) = e1^k`,
`H(k, lb) = icns4 · lb · e1k(k)` (lb = the four declared-length bytes),
`mentry(j)`/`ek_entries(k)` = the manifest's `E_k` entries, and `mp_loop` the model entry loop.

- **L4** `∀k∈{0..4096}. mp_loop(H(k, lb), 8, 8+8k, ∅) = Ok(ek_entries(k))` (lb = BE32(8+8k)).
- **L5** `mp_loop(H(4097, lb), 8+8·4096, declared, ek_entries(4096)) = Err(TooManyEntries)`
  for any declared length covering the valid 4,097th header (and any suffix).
- **L6** `∀q<8. mp_loop(H(4096,lb) with 4097th header, …) = Err(InvalidEntryLength)`;
  `∀q>8` the `EntryOutOfBounds` variant. (L5's `q=8` case gives `TooManyEntries`.)

## Last error text (project default budget: fast 0.2 s / time 1.0 s / depth 6, run 16)

With the L lemmas in the crate, `./verify.sh` run 16 exited 1; why3find reported, per goal:

```
File ".../be32b_read.coma",  lines 30-31: proof failed
File ".../h_byte.coma",      lines 49-52: proof failed
File ".../h_lenfield.coma",  lines 51-52: proof failed
File ".../h_mk_entry.coma",  lines 180-181: proof failed
File ".../model/be32b_ok.coma", lines 34-36: proof failed
File ".../e1k_byte.coma",    lines 50-61: proof failed
File ".../model/l4_core.coma", lines 278-299: proof failed
File ".../model/l5_core.coma", lines 244-249: proof failed
```

The why3 session shows every failing subgoal as `result status="timeout"` for **all four provers**
(alt-ergo, z3, cvc5, cvc4) — i.e. the obligations are not *invalid*, the search is cut off.

## Evidence each obligation is true and provable

Run individually with `why3find prove -g <goal> -t 12 -d 14` (no cache), every one of them
reports `Proved`:

```
be32b_read   ✔     h_byte ✔     h_lenfield ✔    h_mk_entry ✔
be32b_ok     ✔     e1k_byte ✔   model/l4_core ✔  model/l5_core ✔
```

So the lemmas are true; the blocker is why3find's per-goal time budget under `./verify.sh`'s
default `why3find.json`.

## What was tried

1. `#[logic(open)]` on `mp_loop` at crate root → the parser-loop goals unfold the whole recursion and
   time out. Fixed by moving `mp_loop` into `pub mod model` with `#[logic(open(self))]` and proving
   the `good_*` step lemmas inside the module (this is what makes T7 pass at the default budget).
2. Independent copy of the model in a scratch crate; separate single-goal why3find runs to confirm
   each obligation is true (results above).
3. `#[logic(open, inline)]` on the small helper models (`e1k`, `H`, `mentry`, `ek_entries`, …) so they
   unfold cheaply → still timed out on the 8 goals above.
4. Parameterising `H` on the four declared-length **bytes** instead of a `u32` (to avoid the
   uninterpreted `shr`/`bw_and` builtins the M2 report already flagged) → same 6-8 goals still time
   out at the default budget; they pass at `time ≥ 6 s`.
5. Raising the prover budget (`why3find.json` `time`/`depth`, up to `fast 2.0 / time 20 / depth 16`):
   even with a very large shared budget the whole-crate run still reports times-out on these goals,
   because why3find splits the shared budget across the many sibling goals in each file. Single-goal
   invocations (`-g <goal>`) prove them. Raising a solver time limit changes no statement, but it
   also does not produce a small proof, so the budget was restored to the M2 default and the L
   lemmas left out of the crate rather than shipped as a slow or partly-timed-out run.

## Not attempted / next levers

- Proving the L lemmas as **standalone single-goal files** (each proves in isolation) and invoking
  them from the crate, instead of as many-goal files that share a budget.
- Replacing the entry-loop induction with a **closed-form** model of the accepted-entry count as a
  function of the parsed offsets, so L4–L6 reduce to arithmetic on lengths with no recursion.
- A per-file prover budget entry in `why3find.json` (why3find supports a per-goal `profile`), giving
  the L files more time without slowing the rest.

## Update by the orchestrator after the gpt-6-sol attempt (handoff #1, runs 19-25)

Sol (high effort) worked from the baseline `lib.rs.m3` and stopped STUCK after five failed prover
runs (runs 20-24) on the same obligation: the symbolic entry-classification goal
`model/entry_from_header.coma` lines 175-186 (`Goal Coma.vc_entry_from_header`), per
`runs/024.log`. It proved repeated sequence lengths and eight-byte slicing (run 19: 55 files of
preliminary constructors) but the exact `mk_entry` classification/equality goal timed out even after
splitting length, hexadecimal and nominal/opaque facts. It changed no prover settings and restored the
baseline (run 25: `Proved (51 files)`, lib.rs identical to lib.rs.m3). Sol's own instructions forbade
writing report files, so this note is mine. Next: handoff #2 to Fable.

## Resolution (handoff #2, Fable, runs 26-29)
L4, L5 and L6 are proved in manifest form at the default budget. See PROOF-REPORT-M3c.md.
