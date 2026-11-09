# PROOF-REPORT-M4a — exact decoders T9, T10, T11 and lemma L3

## Result

`./verify.sh` run 114 (log `runs/114.log`), exit 0, final line:

```
Proved (152 files) ✔
```

at the **unchanged** default prover budget (`why3find.json`: `fast 0.2`, `time 1.0`, `depth 6`).
Clean-cache reproduction from a disposable copy (source only, no proof cache): run 109,
`Proved (152 files) ✔` in 35 s. `cargo test` reports `conformance: 111/111 vectors passed`. No
`unsafe`, `assume`, `admit`, `trusted`, `external_body`, `todo!` or `unimplemented!` in
`src/lib.rs`. M2 (T1–T6) and M3 (T7, T8, L1, L2, L4–L6) remain proved: their goal files are
unchanged and still present in the 152.

## Theorems and lemmas

| ID | Statement | Proved |
| --- | --- | --- |
| T9 | `∀s,n (\|s\|≤67,108,864, n∈{256,1024,2304,16384}): rle(s,n)=MR(s,n)` as the exact three-channel result or exact first RLE error. | Yes |
| T10 | `∀b,E,i:u32, P(b,E): rgb(b,E,i)=MC(b,E,i)` exactly (index/type/RLE precedence, optional zero-only `it32` prefix, every RGB byte). | Yes |
| T11 | `∀b,E,i:u32, P(b,E): mask(b,E,i)=MA(b,E,i)` exactly (index/type/length precedence, every alpha byte). | Yes |
| L3 | `\|a\|=16,384 ∧ b=bytes("icns")·BE32(16,400)·bytes("t8mk")·BE32(16,392)·a ∧ E=[E[0]] ⇒ MP(b)=Ok(E) ∧ MA(b,E,0)=Ok(128,128,a)`. | Yes |

## How each is obtained

**MR (the RLE model).** `rmodel::mr_ch` is the total recursive R12 channel decoder: it checks the
cursor before reading control (`RleTruncated`), computes `count`/`literal` from the control byte,
tests `count > n−produced` (`RleOverrun`) **before** the data-availability check, then requires
`count` bytes for a literal or one for a repeat (`RleTruncated`). The channel buffer is modelled
by a byte-at-index function (`Seq::create`), never by concat/repeat constructors — the
M3c-flagged encoding trap. `mr_model` runs channels R, G, B over one shared cursor and applies the
trailing-byte test last, so every error is the first in R12's order.

**T9.** `rle_channel` carries the loop invariant `mr_ch(payload, cursor, 0, n, out0) ==
mr_ch(payload, cur, produced, n, out)` — the model's state at the loop head equals the model
continued from the current cursor/produced with the buffer as written so far. The packet body
writes bytes through `mr_iw` (a partial packet write), tracked by the pixel-loop invariant
`out == mr_iw(payload, ctrl, literal, produced, j, old_out)`. Four single-case lemmas
(`ch_done`, `ch_trunc`, `ch_over`, `ch_data`, `ch_step`) unfold `mr_ch` once each, inside the
module. `rle_decode` chains the three channel calls with their exact model equalities and the
trailing-byte branch. `mr_iw_set` (writing byte j+1 equals `set` on the j-prefix) and
`rmodel::iw_zero` (partial write at `j=count` equals the whole-packet `mr_write`) are the two
byte-level facts that let the write loop meet the model's whole-packet step.

**The P(b,E) ⇒ parser_ok bridge.** T10/T11 are stated with the manifest hypothesis `P(b,E)` alone,
not M2's `parser_ok`. `model::loop_ok` (inside `model`, where `mp_loop` is visible) carries
`all_ok` through the entry loop; `mk_entry_ok` shows each appended model entry satisfies
`mentry_ok` by construction of `mk_entry`; `entry_ok_iff_mentry` and `entries_view_pointwise`
relate the concrete entry invariant to the model one. `mp_loop_done`, `mp_len_bound` and
`model::loop_len_bound` supply the length bounds. `p_parser_ok` assembles `parser_ok(b, es)` from
`P(b, entries_view(es))`, so the decoders' precondition is discharged from the hypothesis, never
assumed.

**MC, MA and T10/T11.** `mc_entry`/`ma_entry` dispatch one selected model entry; `mc_model`/
`ma_model` add the index check first. `mc_entry_c`/`ma_entry_c` are the same dispatch phrased on a
concrete `Entry` via `type_of`/`payload_of`; `mc_entry_view`/`ma_entry_view` prove the two
phrasings equal. `decode_rgb` is split into `decode_rgb_sel` (the dispatch on one selected entry,
proved against `mc_entry_c` exactly) and `decode_rgb` (the index selection plus the call). R11's
prefix is `strip_model`, driven by `is_it32_code` and `starts_zero4_prog`/`strip4_prog`, all
functionally equal to their model twins. The RLE+interleave step is `rle_interleave`, whose
postcondition is exactly `mr_model`'s channels and the `interleave3` result, so `decode_rgb_sel`'s
obligation never unfolds the RLE model and the interleave loop together. `t10_rgb`/`t11_mask` are
the blanket theorems: they establish `P(b,E) ⇒ parser_ok`, bridge `mc_model`/`ma_model` at the
selected index through `mc_model_at`/`ma_model_at`, and return the decoder result.

**L3.** `b` is defined byte by byte (`a3_byte`), matching L4–L6's arithmetic style, so no
concat/repeat constructor, shift/mask builtin or Int-to-u8 cast appears. `a3_magic`/`a3_len`
recover the two length fields with `be32_of_bytes`; `t8mk_hex`/`t8mk_nominal`/`t8mk_legacy`/
`t8mk_opaque` classify the type; `a3_mk_entry` builds `E[0]`; `model::step_l3` and
`model::loop_done` finish `MP`. `l3_ma` evaluates `MA` to `Ok(128,128,a)`. `build_a3` constructs
`b` executably with postcondition `is_a3`, and `witness_mask_t8mk` proves the hypothesis is
satisfiable (non-vacuous), matching the existing witness style.

## Non-vacuity of T10/T11

Deliberate negative proofs in a disposable copy (`VERIFY_CRATE=vac-check`), with all statements
enabled:

- Remove `decode_rgb`'s model postcondition `color_index < |E| ⇒ pix_res_model(result) =
  mc_entry_c(bytes, E[color_index])` (run 110): **only** `t10_rgb` fails
  (`Goal Coma.vc_t10_rgb: ✘ (6/7)`).
- Remove `decode_mask`'s analogous postcondition (run 113): **only** `t11_mask` fails
  (`Goal Coma.vc_t11_mask: ✘ (6/7)`).
- Weaken `decode_rgb_sel`'s Err clause to `true` (run 111): `decode_rgb` fails
  (`vc_decode_rgb: ✘ (10/11)`), so the error equality propagates up the real call chain.

Both blanket theorems therefore depend on the decoders' exact model postconditions and are not
vacuous.

## Number of `./verify.sh` runs

**85 runs** this dispatch (runs 30–114). They are an exploratory sequence: most early runs are
single-subgoal bisections of the `decode_rgb` model obligation (runs 45–99), which was the one
hard goal of this milestone. The final green runs are 100 (`decode_rgb_sel` with its Err clause),
102 (`decode_mask`), 105 (L3), 107 (L3 witness), 109 (clean-cache reproduction), 114 (final). The
five-failure stop rule did not trigger: the failures were successive encodings of one obligation
under a shared default budget, and every failing variant was replaced by a different encoding, not
retried. No prover setting was changed.

## Verifier version (trusted computing base)

- `cargo-creusot 0.14.0`, Rust toolchain `nightly-2026-08-03`, `creusot-rustc` from the bundled
  toolchain.
- Why3 platform `1.8.2+git`; `why3find v1.3.0+dev`; provers Alt-Ergo `2.6.4`, Z3 `5.1.0`,
  CVC4 `1.8`, CVC5 `1.4.1`; `creusot-std 0.14.0`.

## Surprises

- **`cargo build` does not check Pearlite bodies.** Only `./verify.sh` catches an ill-typed model
  clause, so each model edit costs a prover run. This dominated the run count.
- **A `#[logic(open)]` model function made total is cheaper than a preconditioned one.** The
  original `interleave3`/`zero_buf` guards (`n ≥ 0`, channel lengths) became obligations at every
  use site inside `decode_rgb_sel`; dropping them removed the failing subgoal.
- **The `decode_rgb` obligation exceeded one VC even after splitting `rle_decode` out.** The
  decisive fix was a two-level split (`decode_rgb_sel` for the dispatch, `decode_rgb` for the
  selection) plus `type_of_entry`'s postcondition `result@ == type_of(b, entry)`, which the M2
  contract had left out. Without that postcondition the model equality could not be closed at the
  call chain, and no amount of in-body `proof_assert!` substituting for it fixed the goal — the
  assertions themselves became the failing conjuncts.
- **Body `proof_assert!`s that restate a later ensures can fail where the ensures, on its own,
  passes.** Removing the intermediate `match out { … }` asserts from `decode_rgb` (run 96) turned a
  red run green; the ensures was provable directly from `rle_interleave`'s postcondition, but the
  extra assert created a harder goal.
- **`Seq::pop_back`/`push_back` congruence through a recursive `map` needs `forall`-form index
  lemmas.** Pointwise (`j`-parameterised) helper lemmas did not discharge the congruence; the
  `forall`-form `push_back_forall`/`pop_back_forall` did.

## Unfinished (not requested; M4b/M4c)

- T12, T13 (combine = MJ), the model-vs-vector agreement for the 102 small vectors, and the ten
  mutations are the next milestones.
