# M3 sign-off review (GPT-6 Astra, 2026-11-20)

Scope: committed state 8bdac4e of png/png_verus. Read-only review through Git objects; Astra ran no verifier, cargo or decoder and edited no file. Saved verbatim by the coordinating session.

**Do not sign off M3 at 8bdac4e under the currently frozen assurance gates.** The candidate supports its stated structural and allocation-prefix component claims, but the two budget witnesses remain mandatory, and the category-(b) ruling weakens the frozen witness requirement rather than merely interpreting it. These are sign-off obligations, not demonstrated decoder failures. This review used committed Git objects only and reran no verifier, Cargo command or decoder; all references below are at 8bdac4e.

## Findings

### Blocking — the two budget records remain required M3 witnesses

`/home/ws/projects/verified-decoders/png/THEOREMS.md:194` explicitly assigns both records to M3; lines 260–264 require proved witnesses **and** passing runtime conformance. `/home/ws/projects/verified-decoders/png/png_verus/evidence/final-candidate-m3-rows-0323.json:258–279` correctly records both omissions.

The missing obligations distinguish opposite sides of the same boundary:

- `apng-bad/pixel-budget` must prove that the actual public frame constructor returns `PixelBudgetExceeded` before inflating the already-truncated default stream.
- `apng-bad/pixel-budget-exactly-2^30-then-zlib` must establish structural acceptance at the exact budget; its frozen final result is `TruncatedZlib`, not `PixelBudgetExceeded`.

The authored expectations are at `/home/ws/projects/verified-decoders/png/limits-draft/gen_vectors_caps.py:53–68`. The universal walk theorem does not replace these separately required non-vacuity witnesses. The unsuccessful attempts supply no proof credit, as `/home/ws/projects/verified-decoders/png/png_verus/LEDGER.md:24–33` correctly states.

### Blocking — category-(b) projections do not discharge the frozen witness gate

`/home/ws/projects/verified-decoders/png/THEOREMS.md:189–191` requires agreement with each record’s expected result and an executable-versus-model call at the public entry point. The candidate instead declares all sixteen category-(b) final outcomes outside M3 at `/home/ws/projects/verified-decoders/png/png_verus/evidence/final-candidate-m3-rows-0323.json:310–315`.

For example, `/home/ws/projects/verified-decoders/png/png_verus/src/witness_m3_projection.rs:46–52` projects acceptance and the IHDR record; lines 109–122 check preparation and then discard the public decoder’s result. A continuation that returned `Format(InvalidFilterType)` for every structurally accepted input could still satisfy this witness for `container/minimal-ok`, despite violating that record’s expected pixels. Runtime conformance would catch that regression; this formal witness would not.

The boundary is sound **as proof staging**: category (a) checks the exact pre-inflate public error, while category (b) isolates walk success without claiming pixel correctness. `/home/ws/projects/verified-decoders/png/PLAN.md:33–37` supports leaving general scanline, conversion and integration equations to M4–M6. But that division does not override the explicit M3 fixture list and completion rule in `THEOREMS.md:194,260–263`. Accepting projections instead of the listed complete witnesses requires an approved clarification or erratum; the coordinator’s interpretation alone does not discharge the frozen gate.

I found no further defect in the scoped header/CRC/walk equations, allocation prefixes, Info representation, transport and separate E2 reader/call-site arguments, or weak continuation contracts; both interim findings are repaired, the trust scan found no new bypass, all 54 recorded proof-input hashes match the commit, and the archived host-323/i686-318 transcripts support the reported 1,448 verified and zero errors on identical source.

## Question 3 — may the budget records move to a later gate?

**Not under the current frozen text.** M3 sign-off must wait unless the authority responsible for the frozen manifest explicitly approves moving these obligations. Stack overflows, the fuel-slicing panic and the timeout explain why proofs are missing; they do not change the acceptance rule.

A permissible amendment should retain the universal M3 theorems and require all of the following:

- Assign a named owner and **M6 integration** as the deadline for both exact model-versus-frozen and public-entry obligations.
- Retain the unconditional pre-allocation `PixelBudgetExceeded` obligation for the overflow record.
- Retain structural acceptance and the exact `TruncatedZlib` outcome for the exact-budget record, with only the specified allocation-failure exception for the public call.
- Keep runtime conformance coverage of both records active throughout the deferral.
- Describe the intermediate state as “M3 component proofs complete; named witness gates deferred by approved amendment,” not as completion of the unchanged frozen M3 gate.

The existing runtime evidence already covers all thirteen cap records without failures (`/home/ws/projects/verified-decoders/png/png_verus/evidence/run-0324.log`, conformance section); that satisfies the interim testing condition, not the missing proof condition. Any amendment accepting category-(b) projections must also assign the other fifteen final outcomes to a definite gate rather than the current undifferentiated “M4–M6” obligation.
