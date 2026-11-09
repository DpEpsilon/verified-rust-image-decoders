# Draft 2 change log

This revision remains **draft**. Draft-one copies of all five spec inputs live alongside the revised files as `SPEC-draft1.md`, `VECTORS-draft1.json`, `SOURCES-draft1.md`, `DESIGN-NOTES-draft1.md`, and `check_vectors-draft1.py`; `THEOREMS-draft1.md` preserves the earlier theorem manifest. No freeze, hash or digest was created.

- **B1:** R4/R7 and the independent oracle now validate and skip maps attached to true-colour or greyscale images. Sixteen success vectors cover all four image types and four entry depths, with and without IDs; additional vectors cover truncated and out-of-range unused maps.
- **B2:** Six conversion vectors exercise 15/16-bit true-colour and palette entries with intermediate and distinct R/G/B values; mutation 12 replaces replication with rounded scaling.
- **S1:** Indexed-map vectors place the map after three-byte and 255-byte IDs and test truncation at the ID/map boundary; mutation 13 moves the map base.
- **S2:** Six vectors select the second entry of 15/16/32-bit maps in raw and RLE images, plus a 16-bit index selecting a second 16-bit entry; mutation 14 hard-codes the stride.
- **S3:** R4 makes flag-one coherence precede range for indexed and non-indexed maps; a flag-zero first=65,535 length=2 vector pins `InvalidColorMap`.
- **S4:** True-colour 16-bit count one is opaque even when bit 15 is clear; two existing expected results and the oracle changed. Map-entry 16-bit count one retains bit-controlled alpha; 32-bit count zero and 24-bit count eight retain strict policy. DESIGN-NOTES.md names each alpha difference, rationale and compatibility risk; no actual 24-bit count-eight file was established by the review.
- **S5:** DESIGN-NOTES.md states the baseline's final-RLE-packet clamping, this decoder's `RleOverrun`, and the compatibility consequence.
- **S6:** SOURCES.md and DESIGN-NOTES.md describe the baseline's 16-bit palette red/blue swap and missing bit replication, and its matching true-colour expansion.
- **S7:** MR takes the successful header record, checked map and starting cursor/count; success returns RGBA pixels plus cursor/count and failure carries ghost state for an exact packet comparison.
- **S8:** Mutation 10 is split into suffix acceptance and footer-offset dereference; mutations 12–20 address conversion, map placement/stride, literal safety, arithmetic width, palette bounds and alpha. Mutation 6 also names T3/T10. The review's proposed unchecked `u32` `4*N` mutant was **declined as an overflow example**: R2 limits `4*N` to 268,435,456, below `u32::MAX`, even on a 32-bit target. Mutation 16 instead uses unchecked `u16` for `N`, which really does overflow on valid dimensions.
- **S9:** SOURCES.md labels absolute first-index semantics, A1R5G5B5, descriptor-count semantics, extension alpha type and normative scanline-crossing claims RECALLED; footer layout is cited as READ.
- **S10:** The manifest separates colour conversion, states the four-orientation bijection and a general packet-repetition lemma for L3/L4, and requires 32-/64-bit width-aware proofs on actual Rust slice domains.
- **N1:** R5 says indexed descriptor counts depend on map entry depth, not index depth.
- **N2:** Vectors add image types 4/8/12/32, interleave `0x80`/`0xc0`, a literal packet crossing a row, grey/indexed non-default origins, and the previously unpinned dimension/interleave and depth/attribute orderings.
- **N3:** R6 permits lazy allocation or a cheap minimum-length test while preserving R14 error priority.
- **N4:** R7 derives its 262,140-byte map bound from the 16-bit length field and four-byte maximum entry.
- **N5:** L2 omits the redundant 18-byte-existence condition.
- **N6:** RLE indexed types with 15- and 24-bit map entries now test invalid attribute count one.
- **REVIEW2 N-1:** R5 says non-indexed descriptor policy follows image depth, not attached map depth; two new vectors test opposite 24/32-bit image/map combinations with attribute count eight. The existing oracle already selected image depth for non-indexed images and now states that choice explicitly.
- **REVIEW2 N-2:** The baseline's zero-length map allocation and zero-byte pull allow flag-one, zero-length maps when the rest of the payload is valid. DESIGN-NOTES.md records the compatibility difference; this draft retains `InvalidColorMap` because no real producer file was supplied.
- **REVIEW2 N-3:** The orientation proof requests named nonlinear lemmas rather than forbidding `by(nonlinear_arith)`.
- **REVIEW2 N-4:** The reviewer's independent rewrite agreed with all 167 pre-N-1 fixtures; neither it nor the earlier oracle has been rerun on the two N-1 fixtures.
- **REVIEW2 N-5:** Mutation 16 now assigns `u16` overflow detection to T1/T2/T8 verifier obligations; the large fixture is only a possible runtime witness.

The suite contains **169 vectors**, including 165 small expanded inputs and four large boundary cases. `python3 -I /home/ws/projects/tga/spec/check_vectors.py` reports **169 independent TGA conformance vectors passed**. The reviewer's separately written oracle agreed with all 167 draft-two vectors before the N-1 additions. No review finding was otherwise declined.
