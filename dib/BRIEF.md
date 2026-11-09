# Brief: verified DIB/BMP decoder core (Verus)

One project, `dib-verus`, implements the frozen DIB-CORE-SPEC 1.0 (BMP file / bare DIB / ICO-entry DIB; the ICO container is a later separate project) and proves the theorem manifest
`THEOREMS.md` with Verus. The same method produced the verified ICNS core (`../icns-verus` and `../tga-verus`, read them
for style: src/{tga.rs,model.rs,model_vectors.rs}, PROOF-REPORT-M*.md, NOTES.md, tools/).
Report honestly, including failures.

## Specification (frozen, read-only)

`spec-frozen/SPEC.md` (R1..R17), `spec-frozen/VECTORS.json` (214 vectors; read the schema in the
spec and in `spec-frozen/FREEZE.txt` hashes), `spec-frozen/SOURCES.md`, `spec-frozen/DESIGN-NOTES.md`
(the deliberate differences from gdk-pixbuf; owner-approved, do not "fix"). If you believe the
spec is wrong or ambiguous, do not change it: write the question in `QUESTIONS.md` and continue with
the most literal reading. The public API is exactly what SPEC.md defines (`decode`, with the records,
integer types and error names of R15 and R16/R17).

## Layout

- The crate is `dib_verus/` (a skeleton that already verifies).
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

## Lessons from the earlier projects (binding)

- Independent review of TGA found that a verifying proof can still be weaker than the manifest.
  Therefore: (a) state every theorem with the manifest's full quantification (arbitrary start states,
  all finite continuations, exact error AND ghost state where the manifest says so); a helper that is
  only proved for the initial state is not the theorem; (b) a "witness" lemma that assumes a
  projection hypothesis must come with the proved lemma that discharges that hypothesis from the
  public result, and be connected to the public proof; (c) mutations for the IMPLEMENTATION must
  change only the executable code (src/<crate>.rs), leaving the frozen model and vectors intact;
  model-side mutations are extra evidence, kept separately; (d) each mutation failure must be a
  verifier diagnostic (not a compile or type error) and must name the obligation the manifest says.
- EVERY verifier run, including exploratory ones, goes through ./verify.sh "<what>"
  (VERIFY_CRATE=<dir inside this project> for copies). Calling cargo verus verify directly or making
  scratch crates elsewhere is forbidden, and earlier agents who did so made their run counts useless.
- Do not add #[verifier::rlimit] or other solver-budget attributes: write STUCK.md instead.
- This project adds a LINEAR WORK bound (ghost step counter, T12) and an allocation-call bound (T3);
  they are part of the manifest, not optional.
- The volume has a 500,000-inode cap: delete target/ and regenerable caches in copies after use.
- `corpus/` has 84 real-world BMP/ICO test files (local testing only; run_corpus.py at the project
  root shows the oracle's expectation per file). From M1 on, also run your plain implementation over
  the corpus and make sure it agrees with the oracle's accepted/rejected result and, for accepted
  files, byte-identical RGBA.

## Milestones (one dispatch each)

- M1: plain implementation of the whole spec as ordinary Rust OUTSIDE `verus!`, plus
  `tests/conformance.rs` running ALL 214 vectors (including repeat-expanded large ones) and a corpus
  test. Done when every vector passes (`cargo test`).
- M2: T1, T2, T3 (safety, exact header, allocation bound). M3: T4, T5, T11 (conversion, row
  arithmetic and orientation, termination). M4a: T6, T7, T8 (exact raw, exact RLE, ICO AND mask).
  M4b: T9, T10, T12, T13 (integrated correctness, precedence, linear work, resource bounds), lemmas.
  M4c: vector agreement, witnesses, implementation-side mutations. Each ends with
  `PROOF-REPORT-Mn.md` (proved / not proved, verifier version, number of verify.sh runs, surprises).
  Only the milestone named in a dispatch is requested.
- Define reference models (MH, MP, ...) independently of the executable code, in model.rs.
