# Second independent adversarial review (GPT-6 Astra), integrated GIF proofs

Date: 2026-11-17. Read-only review at checkpoint 689/690 (830 verified by default, 834 with the test-records cfg). Saved by the coordinator from the reviewer's condensed report. The first review is `REVIEW-ASTRA-INTEGRATED.md` (checkpoint 606).

## Verdict

Sign off with caveats on the integrated P1 proof additions at checkpoint 689/690, at the current caps and over the implemented policy type. Do not sign off on a release or on the complete frozen GIF manifest.

No new decoder-output mismatch, contradictory reachable precondition, circular executable-derived policy definition or production-vs-verification control-flow split was found.

This is a source review supported by verifier logs, source-hash checks, existing release-test executables and bounded Python experiments. It is not a Verus replay or a binary audit.

## Findings

High (carried forward from the first review, not new bugs):

- Witness coverage and proof mutations are incomplete: `model_vectors.rs:11-180` still holds six pairs; `THEOREMS.md:126-177`.
- The production allocation call-graph audit is unsigned and incomplete for GIF (`tools/audit_alloc.sh:5-8`, `:29-30`; `THEOREMS.md:190`; `RELEASE-GATES.md:12`). No extra allocation was found.

Medium:

- The universal T-LZW claim and the symbolic-cap claims still cannot be signed off. The E2 bridge and G8-G9 exactness are absent (`lzw.rs:2355-2366`, `ERRATA.md:187-193`).
- Finding 6: `parse.rs:1803-1804` bounds descriptors by the configurable screen cap, and `lzw.rs:2163`, `:2336`, `:2960` assume `MAX_TOTAL_PIXELS + TABLE_SIZE <= u32::MAX` (`ERRATA.md:195-207`, `:125-131`).

Low:

- E4's operative sentence at `ERRATA.md:217` says StopAtRaster does not read the post-raster code and then concludes "in every case such a code is rejected with an error". That is false for the excluded case (m = 9, area 1, payload [0, 0, 4], checked literals): Strict emits index 0 then `MissingEndOfInformation`; StopAtRaster returns after index 0 without consuming literal 256 (`string_model.rs:65-80`, `:434-443`). The final clause should say an excluded literal is never resolved, emitted or inserted, and that if consumed it returns an error before those operations. No code or proof change is needed. Corrected in `ERRATA.md` (E4 Correction block).

## Status of checkpoint 606 findings 1-6

1. Closed for the integrated executable on E4's readable domain (`string_model.rs:12-58`, `:304-311`, `:325-361`, `:373-394`; `lzw.rs:2442-2459`, `:2536-2540`, `:2576-2581`; witnesses `string_model.rs:399-428` and `lzw.rs:3237-3263`). This does not close the standalone/integrated agreement gate.
2. Open: six witness pairs; the 356 small and 11 large-record instantiations, the iterator witnesses, the integrated stress adapter for inputs of at least 8 MiB, and the proof mutations are all outstanding.
3. Open: production call graph, signed allowlist, build inventory.
4. Closed by approved E1 and its verified condition (`compose_new.rs:953-998`, `:1129-1138`; `frame_model.rs:124-131`; `gif.rs:1735-1738`).
5. Partly closed: scope corrected by E2; the independent string-table sub-obligation is proved; two gates remain open.
6. Open with partial remediation: the copied MAX_INPUT equality is gone and boundary evidence exists, but the unchanged-proofs promise is false or unestablished for the dimension and total-pixel caps.

## P1 semantics and independence

The policy model (`policy_model.rs:18-243`) is independent of the executable transitions. It uses pure header and sub-block functions, the independent string dictionary and pure compositor and disposal functions. `LzwParams` is a shared data type. Historical authorship independence cannot be established by source inspection.

Edge cases were checked against the Python oracle and the definitions: StopAtRaster, disposal 4-7, GCE consumers, ignored extension sizes, checked minima (the size-1 one-pixel rule), missing trailer and trailing-byte tolerances, Clip, and E1. A bounded Python transcription of the string model agreed with the oracle on 8,800 cases (code sizes 1-11, both tail choices; compared indices, error, and E4 behaviour, not bit positions).

- P-BRIDGE (`policy_model.rs:415-435`; `policy_proofs.rs:146-155`; `gif.rs:2380-2433`) is substantive.
- L-PREFIX (`prefix_model.rs:162-194`) matches the intended statement.
- L-TAIL, L-CONS and obligation R use synchronisation points (`conservative.rs:21-77`, `:110-143`, `:170-274`, `:276-365`): 1,024 combinations, FirstSubBlockOnly excluded.
- P-E is not circular (`policy_model.rs:440-477`, `:552-570`; `gif.rs:2494-2503`).
- P-T9 and P-UNIV are connected packages (`gif.rs:2515-2585`, `:507-577`, `:2663-2756`).
- The non-OOM qualifications are necessary, not vacuity devices.
- The full-record domain theorem (`policy.rs:105-137`) is exactly the 2,048 implemented records.

## Verified code versus shipped code

The same runtime decoder runs on both paths at the source/erasure boundary (`lib.rs:41-48` verus_only gates). The test-records feature adds record helpers (`policy.rs:205-215`) and wrappers (`gif.rs:865-870`, `:1707-1712`) that delegate to the same engines; run 690 verifies them. The ghost measured wrappers (`gif.rs:881-942`, `:1596-1657`) add no data-dependent branch, allocation or counted loop, but may change generated structure, so "unchanged runtime behaviour" does not mean byte-identical machine code. Reservation expressions are at `gif.rs:296`, `:320`, `:1823`. The default feature set omits the extra entry points; release tests enable them through the self dev-dependency (`Cargo.toml:19-22`). Binary identity is not established.

Evidence: `runs/689.log:629` and `runs/690.log:629`; all 29 files of `gif/runs/sol-20261117/p1-final-sources.sha256` matched the reviewed tree; 14 frozen hashes matched; the six release test binaries passed (7 lib/report, 7 conformance, 4 allocation-fault, the 1,024-combination test, 2 prefix/bridge, the 37,239-frame stress).

## Remaining gates

- Independent witness matrix and proof mutation campaign.
- Signed production allocation call-graph audit for GIF.
- Integrated T-LZW obligations: standalone/integrated model agreement (E2 (1)) and G8-G9 exactness (E2 (2)).
- Finding 6: cap-change limits for MAX_DIMENSION and MAX_TOTAL_PIXELS.
- P2 compatibility profiles and the policy freeze.
- Verus replay by someone other than the proof agent.

## Not verified

Verus replay, a new binary, a 32-bit build, machine-code equivalence, proof mutations, the independent witness matrix, an exhaustive audit of production loops and library calls, all catalogue and P2 policies, and incumbent compatibility.
