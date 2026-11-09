# PROOF-REPORT-M3c: L4, L5, L6 (Creusot), by Fable (handoff #2)

Saved by the orchestrator from the agent's reply (its harness forbade writing report files).
Verified by the orchestrator: from a clean copy (proof cache, verif/ and target/ deleted)
`cargo creusot` printed `Proved (96 files)` in 68 s; `cargo test` 111/111; diff against
lib.rs.m3b is purely additive (616 lines); no escapes; why3find.json unchanged (fast 0.2,
time 1.0, depth 6).

Result: run 29, `Proved (96 files)`; runs this dispatch 26-29 (26 failed on two witness goals,
27-29 green; 28 was a clean-cache reproduction). 24 tool calls reported (harness: 45).

## Statements (lib.rs, section "L4, L5, L6")
Byte strings are defined arithmetically by the value of each byte at each index (be_byte,
e1_byte, prefix_byte), so no shift/mask builtin or Int-to-u8 cast appears in any goal.
- is_h(b,k,s): b = H(k,s); is_b(b,q): b = B(q); e_entry(j), e_list(k) = E_k.
- L4: 0<=k<=4096 and is_h(b,k,empty) => mp(b) = Ok(e_list(k)).
- L5: |s| <= 67,108,864 - 32,784 and is_h(b,4097,s) => mp(b) = Err(TooManyEntries).
- L6: q<8 and is_b(b,q) => InvalidEntryLength; q>8 => EntryOutOfBounds (also stated over Int);
  q=8 derived from L5 via B(8) = H(4097, empty).
- Six instantiations (inst_*) and six executable witnesses (witness_*, built by build_h/build_b,
  postcondition = exact model result), so no 32 KiB or 64 MiB sequence is expanded in the logic.

## Proof structure
One fact per lemma (be32_of_bytes, prefix_magic, prefix_hdr, prefix_rep, none_* classification
lemmas, none_entry: mk_entry(b, 8+8j) = e_entry(j)); five lemmas unfold mp_loop inside
`pub mod model` (step_none, loop_done, loop_many, loop_short, loop_oob); the induction over k
(rep_steps) runs at crate root where mp_loop is opaque.

## Surprise
With `proof_assert!{ lemma(v@, 7u32); true }` Why3 simplifies the call away, so the lemma is
instantiated only by trigger matching; a u32 parameter puts `t'int q` in the trigger, which never
matches a literal 7. Stating the lemma over Int fixed it.

## Why the earlier attempts failed
Not the truth of the lemmas but their encoding: concatenated/repeated Seq constructors and a
u32-typed BE32 forced index reasoning through concat/repeat and uninterpreted shift/mask builtins.
