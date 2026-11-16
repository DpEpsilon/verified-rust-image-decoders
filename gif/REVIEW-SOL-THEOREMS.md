# GIF theorem statement review

Record written by the Sol reviewer and saved to this path verbatim by the coordinator (the reviewer's instructions prohibit it from writing the file). 2026-11-15. Independent review of the uncommitted M0 theorem manifest and its specification dependencies on branch `dib-policy`. The review covered `gif/THEOREMS.md`, the core and policy drafts, reference oracles, plan, API brief and freeze instructions, with the DIB, TGA, ICNS and alloc-delta theorem manifests as precedents. All inspection was read-only; Python experiments ran with isolation and bounded timeouts.

## Verdict

**Ready to freeze the theorem statements**, with the union-domain correction confirmed in THEOREMS.md's T-LZW representation lemma. This approves the statement of the proof target, not completion of the decoder proofs or implementation assurance gates. The owner still decides whether and when to freeze.

## Round 1

Three material findings:

- High, T9/T13 and PLAN M5: unconditional executable/model success equivalence contradicted permitted allocation failure. R1 now states executable success implies model success unconditionally, with reverse success and error equivalence under a non-OutOfMemory antecedent; G, Prog and PreAlloc govern allocation outcomes.
- High, SPEC G14/G15 versus P-R: the spec required every profile to preserve Strict acceptance and pixels, while the compatibility profiles deliberately reject or redraw some Strict-accepted inputs. R2 restricts preservation to RecommendedBeta and declared conservative records and expressly excludes compatibility profiles.
- Medium, T13: requiring every RGB byte to originate in a colour table excluded transparent-black initialization and disposal. R3 now distinguishes opaque palette-derived pixels from pixels exactly equal to transparent black and retains whole-canvas model equality.

The builder also recorded the following resolutions while applying the review: R4 applies D20's rule-based literals without per-frame initialization and removes the obsolete string-stack plan; R5 requires overflow-safe area and pixel-budget comparisons, with the u64 arithmetic lemma; R6 replaces an equivalent mutation with "skip a zero first sub-block and keep scanning"; R7 updates the review brief; R8 makes unused one-time initialization work an allowance within C_FIXED; R9 identifies RecommendedBeta as the profile actually checked by Python's obligation-R comparison. These items are not additional completed proof claims.

## Round 2

Two material findings:

- High, OPTIONS conservative-record definition and L-CONS: literal lockstep state equality was false for StopAtRaster. Strict reads a successful post-raster Clear*/EOI tail that RecommendedBeta skips. R10 replaces lockstep with a simulation at block, LZW and completed-frame synchronization points, together with L-TAIL; the completed-frame relation excludes resettable LZW scratch state.
- Medium, T-LZW/L-TABLE-VIEW: the invariant read raw stored lengths for literals after D20 had removed their initialization. R11 separates stored_length from semantic code_length and defines rule-based code_suffix/code_first, keeping raw-region separation on storage accessors.

R12 additionally fixes consumed-code-bit position conventions: failed reads do not advance the position; errors detected after reading a code report the position after that code. R13 corrects the full-table measurements: the clear-on-full and large-screen records also reach next=4096, but only the deferred-clear record reads non-Clear codes there. These were builder/proof-stream follow-up corrections, not findings that the reviewer independently established by a full new measurement campaign.

## Round 3

Medium, T-LZW representation lemma: the conjunction of the literal domain and dictionary-entry domain was empty, making the representation equalities vacuous. The coordinator replaced it with their explicit union. The reviewer re-read the applied correction: the lemma covers literal codes and valid dictionary entries and excludes Clear and EOI.

The focused re-check found no remaining material defect in the revised synchronization-point simulation, successful-tail characterization, semantic/storage accessor split or bit-position convention. At raster completion, the two walks can relate their states before StopAtRaster takes its direct return; a successful Strict tail changes no compared completed-frame component. Strict tail failure does not create another Strict frame that the preservation obligation must match.

## Checks and limits

The bounded Python checks passed during the review and the round-2 re-check:

- check_vectors.py: 373/373 records agree, comprising 340 Strict and 33 policy records; Strict exercises 32/33 errors, with PixelBudgetExceeded left symbolic.
- check_plays.py: 65,537/65,537 inputs satisfy the specified mapping, with injectivity and maximum total_plays 65,536.
- validate.py: Strict bridge 360/360; zero obligation-R violations over 163 accepted still inputs, 114 accepted streams and 131,270 Strict frames.

These policy comparisons use recorded incumbent results; no new incumbent decoder binaries were run. The entire suite was not rerun in round 3.

Small instrumented runs confirmed the width-11 behaviour of lzw-m2-reaches-12-bits, the differing Strict/RecommendedBeta post-raster positions, the compatibility-profile preservation counterexample and transparent-black output absent from its palette. A precedence experiment also confirmed that an integrated colour error can precede the standalone LZW error: on lzw-index-error-before-code-error, the colour check fails at consumed-code-bit position 6, while colour-independent LZW reaches InvalidLzwCode at position 9. Those positions must remain distinct in the proof models.

The allocation statements preserve the sealed accumulated log, exact successful request trace, failure-token binding and unchanged log after Saved failure. The byte-counted 20,480-byte LZW reservation and nonoverlapping regions are consistent at the statement level. The borrowed FrameView API has the appropriate Rust lifetime shape.

No Verus commands, builds or mutation campaign were run. The reviewer did not prove L1–L5, L-FULL, L-DISCARD, the arithmetic lemmas, the iterator invariant or the conservative-record simulation, and did not test Verus acceptance of the tracked-log iterator contract. Checker success and this review do not discharge those obligations.

## Subsequent LZW proof-stream report

The coordinator reported that the LZW Verus model had one conflict with the existing specification: under RecommendedBeta StopAtRaster, a string crossing the raster must be accepted, its excess indices discarded and E4 set. The proof stream fixed the model without changing the spec. The coordinator further reported that T-LZW now verifies with 184 verified obligations and the full observation: index sequence, error, consumed-code-bit position and E4. The reviewer did not independently run or audit that verification; these are attributed proof-stream results, separate from this statement review.

The theorem statements are ready to freeze; implementation and assurance gates remain separate.
