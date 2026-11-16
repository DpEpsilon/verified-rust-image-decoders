# Release gates

Items to raise with the owner before any release declaration.

1. Raise the shared alloc guard `MAX_BYTES` (and GIF `MAX_OUTPUT`, currently limited to 64 MiB by it). The owner intends to; it needs an alloc-delta erratum, re-verification of all cores, a re-run audit and new guard hashes. Owner decision 2026-11-16.
   The PNG corpus pass decides the target value and the PNG output cap sizing (owner decision 2026-11-16): the 64 MiB output cap and the 16,384 per-side cap are provisional, held out of the PNG freeze, and tested against very large multi-monitor screenshots and long thin images; `MAX_BYTES` is not raised until that pass says what to raise it to. At the proposed PNG caps, `Raw` for 16-bit RGBA and the RGBA16 output already exceed one 64 MiB reservation (`png/spec-draft/SPEC.md` P20).

GIF integrated-proof sign-off gates (independent Astra review at checkpoint 606, `gif/REVIEW-ASTRA-INTEGRATED.md`, 2026-11-16). The review found no output mismatch and no `assume`/`admit`/`external_body`/`unsafe`; these gates are what remains before the frozen GIF manifest can be signed off. Errata E1 and E2 in `gif/ERRATA.md` are proposed and need your decision.

2. Independent witnesses: the 356 small-record proofs, the 11 large-record symbolic instantiations, the iterator witnesses (for example `anim-dispose-previous-middle`) and the integrated stress adapter for LZW inputs of at least 8 MiB. Six witness pairs exist today.
3. Mutation campaign: the 26 implementation-side proof mutations (for example 7, disposal 3 restores the whole canvas; 20, Saved reserved for every disposal-3 frame) and the alloc-delta mutation artifacts A1, A2, A2', A4, A5, A8, A9 and A17-A19, each with the failed-verifier diagnostic kept.
4. Production allocation call-graph audit for GIF: add the GIF configuration (`gif/tools/audit_alloc.sh:29-30` says there is none), the signed allowlist and the build-input inventory (A-GIF-5, `THEOREMS.md:99`, `:190`). This needs you to sign by the procedure in `alloc-delta/audit/SIGNOFF.md`.
5. Unfrozen policy obligations: L-PREFIX, L-TAIL/L-CONS, P-BRIDGE, P-E, P-R, and the P2 compatibility profiles, which are not implemented (the two ImageCrateCompat-labelled records run a RecommendedBeta plus `FirstSubBlockOnly` stand-in).
6. Fresh full verifier replay (the review did not replay Verus; checkpoint 606 reports 700 verified, 0 errors), recorded together with the source tree hash it was run on, so the verified count is bound to a source hash.
