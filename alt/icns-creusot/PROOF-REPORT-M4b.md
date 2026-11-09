# PROOF-REPORT-M4b — the compose model MJ, T12 and T13

## Result

`./verify.sh` run 127 (log `runs/127.log`), exit 0, final line:

```
Proved (162 files) ✔
```

at the **unchanged** default prover budget (`why3find.json`: `fast 0.2`, `time 1.0`, `depth 6`; the
file was not touched). Clean-cache reproduction from a disposable copy (source only, no proof cache)
is run 128, `Proved (162 files) ✔` in 97 s. `cargo test` reports `conformance: 111/111 vectors
passed`. No `unsafe`, `assume`, `admit`, `trusted`, `external_body`, `todo!`, `unimplemented!` or
`requires(false)` in `src/lib.rs`. M2 (T1–T6), M3 (T7, T8, L1–L6) and M4a (T9, T10, T11, L3) remain
proved; every one of their goal files is still present among the 162 and none has a failing subgoal.
`lib.rs` was backed up to `src/lib.rs.m4a` before editing.

## Theorems

| ID | Statement | Proved |
| --- | --- | --- |
| T12 | `∀b,E,ci,mi, P(b,E) ∧ ci,mi∈Option<u32>: combine(b,E,ci,mi)=MJ(b,E,ci,mi)` exactly — every RGBA byte, the `MissingColor`-before-`MissingMask` absent-index precedence, the colour-decode-before-mask-decode precedence for a present pair, and the side mismatch. | Yes |
| T13 | `∀b,E,ci,mi: parse(b)=Ok(E) ∧ ci,mi∈Option<u32> ⇒ combine(b,E,ci,mi)=MJ(b,E,ci,mi)`, with `P(b,E)` derived at the actual call site. | Yes |

Both are stated in the T10/T11 wrapper style: a public theorem function whose `#[requires]` is the
manifest hypothesis and whose `#[ensures]` is the exact model equality.

## The MJ model

Three new Pearlite functions, written independently of `combine` and pointwise (no `concat`/`repeat`
constructor, no `u32`-typed logic arithmetic — the M3c trap):

- `rgba4(rgb, alpha, n) = Seq::create(4*n, |i| if i%4==3 { alpha[i/4] } else { rgb[3*(i/4) + i%4] })`
  is R13's row-major RGBA layout, one byte-at-index function.
- `mj_pair(b, E, ci, mi)` is the total model for a present pair: colour index first
  (`IndexOutOfRange`), then `mc_entry`, then the mask index, then `ma_entry`, then the side
  comparison (`TypeMismatch`), then `Ok((d,d, rgba4(rgb,alpha,d²)))`. No other branch.
- `mj_model(b, E, ci, mi)` adds R14's absent-index order: `None` colour ⇒ `MissingColor`, else
  `None` mask ⇒ `MissingMask`, else `mj_pair`.

`mj_pair_c`/`mj_model_c` are the same two functions phrased on the concrete `Entry` list (the
selected decodes written `mc_entry_c`/`ma_entry_c`), exactly as M4a phrased `mc_entry`/`ma_entry`
for T10/T11. `mj_pair_view`/`mj_model_view` prove the two phrasings equal at every index pair — for
the out-of-range indices both sides reduce before any entry is selected, so the lemmas need no
in-range hypothesis (`mc_entry_at`/`ma_entry_at` supply the in-range case).

## How T12 is obtained

`combine` is split:

- `combine_pair(bytes, entries, ci, mi)` handles both indices present. Its postcondition is exactly
  `mj_pair_c` on the concrete list plus the T5 `4d²≤65,536` buffer bound. Its body is the R13 order
  in the same shape as the model: colour index check, `decode_rgb` with its exact `mc_entry_c`
  equality propagated on `Err`, then mask index check, `decode_mask` with its exact `ma_entry_c`
  equality, then the side test, then `zip_rgba` (the `vec![0u8; 4*n]` loop, whose postcondition is
  `rgba4`).
- `combine` keeps its public signature and T5 bound, and its body is the absent-index order:
  `MissingColor` when the colour index is `None`, then `MissingMask`, then `combine_pair`. Its model
  postcondition is `mj_model`.

`t12_combine` takes `p_model(bytes, entries_view(entries))`, discharges `parser_ok` through
`p_parser_ok` (the M4a bridge, never assumed), and calls `combine`.

## How T13 is obtained

`t13_combine_at_parse(bytes, ci, mi)` calls the **real** `parse`, matches its result, and in the `Ok`
branch reads `p_model(bytes, entries_view(es@))` straight off `parse`'s own T7 postcondition
(`res_model(result) == model::mp(bytes@)`, so `Ok(es)` gives `entries_view(es@) = MP(bytes@)`), then
discharges `parser_ok` and calls `combine`. The theorem's `#[ensures]` quantifies over every `e` with
`p_model(bytes@, e)`; since `MP` is a function, that `e` is exactly the parse result's entry list, so
the statement is the manifest's for the unique admissible `E` and vacuous only when `parse(b)` is an
`Err` — which is exactly when `P(b,·)` is unsatisfiable. No provenance hypothesis is posited: the two
witnesses below show the `Ok` branch is reachable and `P` is realisable.

## Non-vacuity of T12/T13

Two proved witnesses (both among the 162, no failing subgoal):

- `witness_t12_p_realisable()` builds the minimal `H(0, ε)` header with `build_h`, proves
  `MP(b) = Ok(empty)` through L4's `l4(b,0)`, and returns `p_model(result@, Seq::empty())`. So T12's
  hypothesis is satisfiable.
- `witness_t13_parse_succeeds()` builds the same header, proves `MP(b) = Ok(empty)`, calls the real
  `parse` and asserts the `Ok` branch is taken (the `Err` branch is proved unreachable), returning
  `p_model(result@, Seq::empty())`. So T13's call site is reachable.

Deliberate negative proofs in a disposable copy (`VERIFY_CRATE=vac-check`, runs 118–122), with all
T1–T13 statements left enabled:

- Weaken `combine`'s model `Ok` clause to the length/`w=h` shape while leaving `mc`/`ma` and
  `combine_pair` intact (run 119): **exactly** `t12_combine` and `t13_combine_at_parse` fail
  (`vc_t12_combine: ✘ (2/3)`, `vc_t13_combine_at_parse: ✘ (9/10)`). This is the mutation-10-style
  witness that the blanket theorems depend on the exact per-byte equality, not on a weaker bound.
- Weaken `combine_pair`'s `Ok` clause to just its length while leaving `combine`'s contract intact
  (run 118): only `vc_combine` fails, i.e. the equality propagates up the real call chain.
- Reverse the absent-index order so `MissingMask` is tested first (run 120): `vc_combine` fails.
- Move the mask index check before the colour decode (run 121): `vc_combine_pair` fails.
- Replace the side test with `if false` (run 122): `vc_combine_pair` fails.

Each mutation broke exactly the goal that should observe it; none passed by being derivable from
another premise.

## Modifications to existing code

Only `combine`'s body changed: the `ok_or(Error::MissingColor)?`/`ok_or(Error::MissingMask)?` chain
became explicit `match`es that pin each absent case to `mj_model_c`, and the RGBA write loop moved
into `zip_rgba`. The public signature, the T5 bound postcondition and the executable behaviour are
unchanged (111/111 vectors). The diff against `lib.rs.m4a` removes only the four loop invariants of
the moved loop and the two `ok_or` lines; every earlier theorem function is present with its
contract intact.

## Number of `./verify.sh` runs

**14 runs** this dispatch (runs 115–128). Green: 117 (`Proved (160 files)`), 123 (160, clean cache),
127 (162), 128 (162, clean cache). Red: 115–116 (the two single-subgoal iterations of the `4*n`
allocation bound and the stray `entries_view` assertion), 118–122 (the five mutations), 124–125 (a
`&[u8]`-vs-`&[u8]` Pearson-type slip in a witness `ensures`). The five-failure stop rule did not
trigger: 115 and 116 were different encodings of one obligation, and the mutations are deliberate.

## Verifier version (trusted computing base)

- `cargo-creusot 0.14.0`; Rust toolchain `nightly-2026-08-03` (`rustc 1.99.0-nightly 11177f223`,
  2026-08-02); `creusot-std 0.14.0`, `creusot-rustc` from that toolchain.
- Why3 platform `1.8.2+git`; `why3find v1.3.0+dev`; provers Alt-Ergo `2.6.4`, Z3 `5.1.0`,
  CVC5 `1.4.1`, CVC4 `1.8`.

## Surprises

- **The `vec![0u8; 4*n]` allocation bound is a separate goal.** `zip_rgba`'s first attempt failed on
  `in_bounds (4*n)` (subgoal 0 of 34), not on the byte equality: the `Vec` allocation needs
  `4*n ≤ usize::MAX`, which the length preconditions alone do not give. Adding
  `#[requires(n@ <= 16384)]` and a `n@ <= 16384` fact at the call site (from `mc_entry_c_bounds`)
  closed it in one run.
- **An intermediate `proof_assert` can be the failing conjunct.** `combine_pair`'s colour-error
  branch carried a redundant `entries_view(entries@)[color_index@] == entry_view(entries@[color_index@])`
  rewriting the concretely-phrased goal into an `entries_view` form the goal did not need; removing
  it (and keeping the goal in the `mj_pair_c` phrasing throughout) turned red to green. This repeats
  the M4a finding.
- **`combine`'s own contract already shielded T12 from the `combine_pair` mutation.** Mutation A
  (weakening `combine_pair`'s `Ok` clause) failed `vc_combine`, not `t12_combine`: a caller sees only
  the callee's contract. The mutation that reaches the blanket theorem is the one on `combine`'s own
  model postcondition (run 119), which fails both T12 and T13.

## Unfinished (not requested; M4c)

The model-vs-vector agreement for the 102 small vectors, the ten manifest mutations, and the
remaining L1–L6 instantiations are M4c.
