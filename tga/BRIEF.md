# Brief: verified TGA decoder core (Verus)

One project, `tga-verus`, implements the frozen TGA-CORE-SPEC 1.0 and proves the theorem manifest
`THEOREMS.md` with Verus. The same method produced the verified ICNS core (`../icns-verus`, read it
for style: `icns_verus/src/{icns.rs,model.rs,model_vectors.rs}`, `PROOF-REPORT-M4c.md`, `NOTES.md`).
Report honestly, including failures.

## Specification (frozen, read-only)

`spec-frozen/SPEC.md` (R1..R17), `spec-frozen/VECTORS.json` (169 vectors; read the schema in the
spec and in `spec-frozen/FREEZE.txt` hashes), `spec-frozen/SOURCES.md`, `spec-frozen/DESIGN-NOTES.md`
(the deliberate differences from gdk-pixbuf; owner-approved, do not "fix"). If you believe the
spec is wrong or ambiguous, do not change it: write the question in `QUESTIONS.md` and continue with
the most literal reading. The public API is exactly what SPEC.md defines (`decode`, with the records,
integer types and error names of R15 and R16/R17).

## Layout

- The crate is `tga_verus/` (a skeleton that already verifies).
- `./verify.sh "<what you are trying to prove>"` runs the verifier and logs the run to `RUNLOG.tsv`
  and `runs/NNN.log`. ALWAYS use it instead of calling cargo's verifier directly: the count of runs
  is a measured outcome. For a disposable copy use `VERIFY_CRATE=<dir inside this project>`;
  never make scratch/probe crates outside that mechanism. You may run `cargo test` and `cargo build`.
- `TOOLNOTES.md` has Verus notes. Work only inside this directory (plus reading `../icns-verus`,
  `/opt/verus`). No network beyond crates.io and git clone of github.com repos.

## Hard rules

1. No `unsafe`. No library dependencies beyond `vstd`. Test-only dependencies (`serde_json`) are fine in `[dev-dependencies]`.
2. Do not weaken, delete or bypass a specification or proof to make verification pass. No assume,
   admit, external_body, trusted, `requires(false)`-style vacuous contracts, `todo!`/`unimplemented!`
   in finished code. If one seems unavoidable, write it in `STUCK.md`; do not add it.
3. Never change the vectors or the harness expectations to match your code.
4. Keep a short `NOTES.md` (decisions, done, not done). Claims in your report must be backed by
   commands you ran; give exact outputs (test counts, verifier lines).
5. Escalation: five failed verifier runs on the same obligation -> stop, write `STUCK.md` (obligation,
   last error text, what you tried) and report. A stronger model takes over.
6. Budget: stop and report if you exceed the tool-call cap in your dispatch. Count honestly.

## Milestones (one dispatch each)

- M1: plain implementation of the whole spec as ordinary Rust OUTSIDE `verus!`, plus
  `tests/conformance.rs` running ALL vectors in `spec-frozen/VECTORS.json` (including the large
  `input_repeat` / output-repeat ones). Done when every vector passes (`cargo test`).
- M2: T1, T2, T5 (header). M3: T3, T4, T10 (packet safety, conversion/orientation, termination).
  M4a: T6-T9, T11 and lemmas L1-L4; M4b: the compact vector protocol, named witnesses and the ten+
  mutations in `mutation/` (restore each; keep failing logs). Each milestone ends with
  `PROOF-REPORT-Mn.md` (theorems proved / not, verifier version, number of `./verify.sh` runs,
  surprises). Only the milestone named in a dispatch is requested.
- State each theorem with the exact quantification in `THEOREMS.md`; a weaker statement does not
  count. Define your reference models (MH, MP, ...) independently of the executable code.
