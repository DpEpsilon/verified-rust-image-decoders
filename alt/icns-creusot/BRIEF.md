# Shared brief: verified ICNS decoder core

Two projects implement the SAME frozen specification, one verified with Verus (`icns-verus`) and
one with Creusot (`icns-creusot`). The agents are the same model with the same instructions and
differ only in the verification tool. Your work is compared with the other project's, so follow
the rules exactly and report honestly, including failures.

## Specification (frozen, read-only)

`spec-frozen/SPEC.md` (ICNS-CORE-SPEC 1.1, requirements R1..R18), `spec-frozen/VECTORS.json`
(111 conformance vectors, schema in R18), `spec-frozen/SOURCES.md` (evidence). Do not edit, copy
over or work around them. The user has settled the strict rules in R4, R5, R10 and R11. If you
believe the spec is wrong or ambiguous, do not "fix" it; write the question in `QUESTIONS.md` and
continue with the most literal reading.

The public API is exactly the four functions of SPEC.md (`parse`, `decode_rgb`, `decode_mask`,
`combine`) with the records, integer types and error names from R1 and R17.

## Layout

- The crate is in `icns_verus/` or `icns_creusot/` (a working skeleton that already verifies).
- `./verify.sh "<what you are trying to prove>"` runs the verifier and logs the run to
  `RUNLOG.tsv` and `runs/NNN.log`. ALWAYS use it instead of calling cargo's verifier directly: the
  count of runs is a measured outcome. You may run `cargo test` and `cargo build` directly.
- `TOOLNOTES.md` has tool-specific hints and local documentation paths.
- Work only inside your project directory (plus reading `/opt/creusot-src`, `/opt/verus`). No
  network beyond what cargo needs for crates.io and what `git clone` of github.com repos gives you.

## Hard rules

1. No `unsafe`. No dependencies in the library beyond the verification support crate the
   skeleton already has. Test-only dependencies (for example `serde_json`) are fine in
   `[dev-dependencies]`.
2. Do not weaken, delete or bypass a specification or proof to make verification pass. Do not use
   assume, admit, trusted or external-body style escapes, `#[requires(false)]`-style vacuous
   contracts, or `todo`/`unimplemented` in finished code. If you think one is unavoidable, stop
   and write it in `STUCK.md` instead; do not add it.
3. Never change the vectors or the conformance harness expectations to match your code. A failing
   vector means your code or your reading of the spec is wrong.
4. Keep a short `NOTES.md` (decisions, what is done, what is not). Claims in your final report
   must be backed by commands you actually ran; state exact outputs (test counts, verifier lines).
5. Escalation: if the verifier fails five times on the same obligation, stop working on it,
   write `STUCK.md` (the obligation, the last error text, what you tried) and report. A person or
   a stronger model takes over from there.
6. Budget: stop and report if you have made more than the tool-call cap stated in your dispatch (M1: 150, M2: 400).

## Milestones (one dispatch each)

- M1: plain implementation of the whole spec as ordinary Rust (no proofs, no contracts yet; for
  Verus keep the code outside the `verus!` macro, for Creusot add no contracts). A test harness
  `tests/conformance.rs` runs ALL vectors from `spec-frozen/VECTORS.json` against the code and
  every vector passes (`cargo test`). Done when: 111/111 pass.
- M2, M3, M4: prove the theorems of `THEOREMS.md` (a read-only copy is in your project directory;
  it is the authority, and both projects prove the same statements). M2 = T1-T6, M3 = T7-T8,
  M4 = T9-T13. State each theorem formally with the exact quantification in the manifest; a
  weaker statement does not count. Define your own formal copy of the reference model (MP, MR,
  MC, MA, MJ) in your tool's logic, written independently of the executable code.
- Non-vacuity: agree with the manifest's non-vacuity and mutation protocol (102 small vectors by
  computation or proof, lemmas L1-L6 for the nine large ones, the 40 named witnesses, the ten
  mutations). Do the mutations in a disposable copy under `mutation/` inside your project,
  restore each before the next, and keep the failed verifier output as files. Mutations are the
  last step of M4.
- Each milestone ends with `PROOF-REPORT-Mn.md`: theorems proved, theorems not proved, the
  verifier version, the number of `./verify.sh` runs, and anything surprising.
Only the milestone named in a dispatch is requested.
