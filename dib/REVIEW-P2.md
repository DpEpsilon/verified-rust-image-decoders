# Sol soundness reviews of DIB policy P2a and P2b (condensed by coordinator)

## P2a (six options; run 293, 1644 verified)

No new functional or proof-soundness defect in the six option branches by static review. Checked: `decode_policy_tracked` has no `requires` and states its charged-work bound for every `Policy`; `fill_blank` charges one step per written byte; Gdk BI_RGB past-EOF rejection happens before `alloc_output`; the Gdk offset clamp is present; DefaultRGB recovery tests the original RGB mask words and `channel` uses `channel_span`; the Strict vector edits only strengthen literal-span assertions; the six policy freeze hashes match.

Two API-shape deviations from the frozen spec, both then addressed or flagged:
- Low: `Policy` has public mutable fields and `strict_policy_exec()` was public, so release callers could build custom records (spec wants an internal record). Resolved in P2b: Verus forces `Policy` fields to stay `pub` (opaque-datatype constraint), so the constructors became `pub(crate)` and `Policy` is no longer re-exported.
- Medium, predates P2a: the profiled entry returns `(Result<Decoded, Error>, bool)` rather than `Result<ProfiledDecoded, Error>` with `recovered` inside. Still open.

## P2b (all 22 options; run 437, 1712 verified)

Verdict: do not sign off as implementing the frozen four-profile API until GdkCompat and ImageCrateCompat are selectable.
- High: `BmpProfile` had only `Strict` and `RecommendedBeta`, so release callers could not select the compat profiles. The differential tests were real comparisons of the Rust custom-record decoder against the independent Python oracle (not the oracle against itself) but did not exercise the public selector, and `policy_conformance.rs` looped over only two profiles. Resolved in P2c (run 442, 1714 verified): both profiles selectable, 102/102 vectors per profile, corpus 50/52/58/55.
- Medium, inherited from the frozen package: `policy_oracle.py` and `gen_policy_vectors.py` read `policy/STRICT-VECTORS.json`, which does not exist (source is `spec-1.1-frozen/VECTORS.json`), so the oracle's advertised 302-vector gate is unrunnable from the policy package. The Rust crate's own 302-vector harness is unaffected. Not fixed (frozen files).

Other checks passed: no `assume`/`admit`/external/trusted/`unsafe`/`requires(false)`/rlimit in the active decoder or model; one allocation site (`vec![0u8; 4*n]`) with caps checked before it; RLE clip loops paint only the clipped count; ClampStop accepts equality without setting recovery; the non-palette prefill flag depends on unpainted pixels; Rgb2 restricted to H>=40 with high-first samples; Gdk repairs are BMP-only.
