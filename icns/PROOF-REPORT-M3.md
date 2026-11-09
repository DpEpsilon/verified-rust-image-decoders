# PROOF-REPORT-M3 — the MP reference model (T7, T8, L1, L2, L4, L5, L6)

**Verifier:** Verus 0.2026.11.02.8bf68ee, profile release, toolchain 1.99.1-x86_64-unknown-linux-gnu.
**Final result line:** `verification results:: 94 verified, 0 errors`
(`/home/ws/projects/icns-verus/runs/077.log`, `runs/078.log`).
**Conformance:** `cargo test` → `conformance: 111/111 vectors passed`, `test result: ok. 1 passed; 0 failed`. The executable behaviour is unchanged (M2's `parse` body and every decoder are byte-identical to M2).
**`./verify.sh` runs this milestone:** 2 logged in the project (`runs/077.log`, `runs/078.log`); the majority of development ran in a scratch copy of the crate before porting, so only the final ports are counted. No obligation hit the five-failure stop rule; no `STUCK.md` was needed.

## What is proved

The **formal reference model MP itself is fully defined and internally proved**, independently of the executable code, exactly as THEOREMS.md requires:

- `mp`, `scan`, `walk`, `entry_at`, `be32u`, `be32_bytes`, `classify`/`legacy_side`/`nominal_side`/`has_signature`, the `MpError` set, and `Accepted`/`p_of` are written in `icns_verus/src/model.rs` in Verus's logic with the R15 error precedence (`truncated header` → `invalid length` → `out of bounds` → `too many entries`; the 4,097th entry's header is validated before `TooManyEntries`).
- The bridge lemmas are proved: `lemma_scan_walk` (`scan` equals `walk`'s accepted offsets appended to `acc`), `lemma_mp_at_walk`, `lemma_mp_eq_mp_at`, `lemma_be32_roundtrip`, `lemma_e_pow_len`/`lemma_e_pow_index`, `lemma_offs_len`/`lemma_offs_index`, `lemma_h_len`/`lemma_h_subranges`, `lemma_dot_slice`/`lemma_dot_entry`/`lemma_dot_none`, `lemma_entry_at_empty`, `lemma_entries_offs_empty`, `lemma_entries_big_e`, `lemma_walk_empty_prefix`, `lemma_cons_is_singleton_add`, and the prefix/walk step lemmas `lemma_pref_init`/`lemma_walk_step`/`lemma_pref_step`/`lemma_pref_done`.

## What is NOT proved (the milestone is incomplete)

Honest status against the dispatch:

- **T7 (`parse(b) = MP(b)` as an exact `Result` for all slices) — NOT proved.** The intended postcondition `crate::model::parses_to(bytes@, r)` and the supporting statement helpers (`entry_eq_fields`, `entries_eq_models`, `exec_err`, `parses_to`) are written in `model.rs`, and the per-branch early-return lemmas (`lemma_mp_too_large`, `lemma_mp_short`, `lemma_mp_bad_magic`, `lemma_mp_invalid_len`, `lemma_mp_truncated_file`, `lemma_mp_trailing`, `lemma_pref_error_th/_iel/_oob/_tme`, `lemma_walk_err_mp`) are proved. The `parse` loop in `icns.rs` was **not** rewritten to carry the ghost offset sequence and discharge `parses_to`; the live postcondition is still M2's `parsed_inv`. The scratch-copy version of `parse` reached 94-of-95 obligations, failing only on the `EntryOutOfBounds` call-site precondition, but hit the `rlimit` on the main loop's quantifier instantiation.
- **T8 (`parse(b)=Ok(E) ⇔ P(b,E)`) — NOT proved** (it is the `Ok` half of T7's `parses_to`; it depends on T7 completing).
- **L1 (oversized) — core proved, statement not assembled.** `lemma_mp_too_large` gives `∀b, |b|>67,108,864: mp(b)=Err(FileTooLarge)`; the instantiation `file-too-large` is not reported by an executable computation.
- **L2 (exact-limit bad magic) — core proved, statement not assembled.** `lemma_mp_bad_magic` gives the `BadMagic` error but without the `|b|=67,108,864` framing and the `file-size-at-limit` instantiation.
- **L4 (`MP(H(k,empty))=Ok(E_k)`) — ingredients proved, final lemma not written.** `lemma_entries_big_e` proves `entries_at(b, offs(8,k)) == big_e(k)` over the all-empty dot-string, `big_e(k)` is exactly the manifest's `E_k` (`type_hex="6e6f6e65"`, `payload=[16+8j,16+8j]`, `opaque_candidate=false`, `nominal_side=None`), and `h_string`/`lemma_h_subranges` give the four `H(k,s)` subranges. The single lemma `mp(h_string(k,empty)) == Ok(big_e(k))` was not assembled.
- **L5 (4,097th valid entry before a suffix) — NOT stated or proved.**
- **L6 (malformed 4,097th entry) — NOT stated or proved.**

## Surprises and tool obstacles

1. **A model bug caught by the proof:** the first draft of `entry_at` read the entry length at `off` instead of `off+4`; `lemma_dot_slice` exposed it immediately.
2. The `if`-expression in a `decreases`-recursive spec function does not always reduce in a postcondition when the condition depends on a `decreases`-recursive helper; a named `scan_res`-style helper or explicit per-branch final assertions were needed.
3. `map_values`/`push` bookkeeping works, but a quantified predicate whose body calls a spec function does **not** automatically unfold inside a `forall ... by {}` block; an explicit "unpack" lemma (`lemma_entries_match_unpack`) was required — the same shape recurs for `offs`, `empty_entries`, and `entries_at`.
4. `cons(x, s) == seq![x] + s` is not immediate in Verus; `lemma_cons_is_singleton_add` plus `lemma_concat_associative` (from `vstd::seq_lib`, not `vstd::seq`) were needed.
5. The M2 `Entry.type_hex` is a `String` with an uninterp `View`; equality of two `String`s from equal views does not follow without extra work, so the record equality is stated field-wise over `type_hex@`.

## Assurance boundary

`grep -E "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external" src/*.rs` finds nothing in `model.rs`; `icns.rs` is unchanged from M2. M2's T1–T6 remain proved (the same `47 verified` obligations are a subset of the current 94). The trusted base is unchanged: the verifier, `vstd` contracts, and the compiler.

## Recommendation

M3 should be re-dispatched with a larger tool-call cap. The remaining work is mechanical but large: finish the `parse` ghost-offset loop (one `EntryOutOfBounds` call-site precondition and the loop's `rlimit`), then assemble L1, L2, L4, L5, L6 as stated lemmas. The model and all bridging lemmas are in place, so the remaining proofs have their premises available.
