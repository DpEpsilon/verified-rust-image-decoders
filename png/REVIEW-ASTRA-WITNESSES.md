# Authored-witness pipeline review (GPT-6 Astra, 2026-11-18)

Scope: `git diff 1473bef 9c1f2f7 -- png/inflate_verus`. Read-only review; Astra ran no verifier, cargo, decoder or frozen encoder, and edited no file. Saved verbatim by the coordinating session, since the reviewer's own instructions forbade it from writing report files. Line references are to 9c1f2f7.

The committed 563/631 claim is supportable as a count of frozen stream instances, not distinct compressed streams or completed M2 gates. The expected outputs inspected are independent of decoding. The witness helpers invoke the real public entries and assert their concrete results. Two audit/evidence defects need correction before treating the automated coverage report as a self-contained regression gate.

## Findings, ranked by severity

### 1. Medium — the audit credits individual witnesses after their public calls have been deleted

At `inflate_verus/tools/audit_witnesses.py:118–137`, the audit recognizes the 48 individual witnesses using their documentation comment, function declaration and literal input array. It checks input equality, adds the name to `seen`, and then credits every name as covered. It never checks that the function invokes a public-entry harness, that its output limit equals the independent image R, or that its expected status/raw binding survives.

This affects new witnesses as well as the inherited subset. The real call for repeat-with-no-previous is `inflate_verus/src/witness_repeats.rs:35`. Removing that entire statement in an in-memory view of the committed source, the committed audit still passed and reproduced the byte-identical 563/631 inventory. The remaining function proves a model fact but no longer makes either concrete public call, which violates the coverage criterion while retaining credit.

The special/policy checks have a related binding gap. At `audit_witnesses.py:193–217`, the audit checks raw bytes, R, segmentation and the number of `check_policy_parts` strings, but never reads the manifest's `status`. Changing `empty_fdat.status` from `TruncatedZlib` to `Done` in an in-memory view of `tools/witness_special.json:17`, the audit again passed unchanged. The real source correctly proves TruncatedZlib at `src/witness_special.rs:29–34`. Similarly, `audit_witnesses.py:263–273` claims all four option combinations at R and R+1 based on eight call-string occurrences, without validating their argument tuples.

These are demonstrated audit false positives, not evidence that the current committed witnesses omit calls or use wrong statuses. Source inspection found the current calls and bindings intact.

Fix: give individual witnesses the same explicit manifest treatment as classes; bind each credited function to its exact input, independently computed R, expected raw/status and actual harness call. For policy families, validate the exact multiset of `(limit, enforce, ignore_tail, status, raw)` calls rather than their count. Add negative audit tests that remove a call, change its limit, duplicate an option combination, and change a manifest status; reject each mutation. A source parser or generated checked descriptor is preferable to additional unrestricted substring checks.

### 2. Medium — the new audit requires diagnostic files that the reviewed commit does not contain

`audit_witnesses.py:219–224` unconditionally reads each blocked item's diagnostic log (`tools/witness_blocked.json:13` onwards: runs 507 and 508). Neither `runs/0507.log` nor `runs/0508.log` is in the 9c1f2f7 tree; they match the ignore rule at `.gitignore:7`. The final verifier log `runs/0510.log` is also absent; its summary survives at `RUNLOG.tsv:511`.

A fresh checkout, after regenerating the intentionally untracked frozen vectors, therefore cannot complete the provenance audit: it fails opening run 507. The ledger's retained-evidence claim (`LEDGER.md:30–40`, `:59`) is true of this workspace, not of the committed checkpoint alone.

The local diagnostics are genuine: run 507 identifies the three named resource failures and run 508 the other two. Each blocked class's raw length and member list matches its saved manifest in the committed backup archives. This is an evidence-retention defect, not fabricated blocked accounting.

Fix: retain at least runs 507, 508 and 510 in the checkpoint, as tracked logs or a versioned evidence archive the audit can read, with hashes, plus a checkout-only replay test that every required evidence path is present. Do not replace historical failure diagnostics with an unrelated fresh successful run. (The frozen JSON files are a different case: their omission and deterministic regeneration are documented at `png/.gitignore:4–11`.)

## Independence

Generated expected raw bytes are not reconstructed by decoding compressed input. `tools/author_literal_classes.py:55–81` packs samples and computes forward filters/Adam7 traversal; `:98–114` reconstructs the seeded sample schedule. `tools/author_extended_classes.py:186–231` extends that schedule and `:245–266` authors APNG operands directly, without using composed frame expectations as frame inflate bytes. The original APNG operands are at `spec-frozen/gen_vectors.py:914–919` and `:979–1008`.

Astra independently reconstructed the scheduled raw bytes (separate read-only calculation, different packing formulation, explicit Paeth nearest-predictor). It matched 113 of 114 initial classes and all 128 extended classes; the remaining initial palette class uses the explicit six-byte seed `[0,0,1,1,2,255]` at `author_literal_classes.py:37–40`. No compressed bytes or frozen expected fields entered that reconstruction.

Recovering TPX by replaying encoder intent is legitimate: TPX is an input sample array, created at `gen_vectors.py:717` before `:718` passes it to `png`. An RNG-draw-only replay recovered `(201,73,63), (202,75,151), (225,26,23), (252,246,129)` without executing the encoder prefix; these yield the raw seed at `author_literal_classes.py:37`. This does not validate the whole PNG generator, which is outside an inflate raw-byte witness.

Forward encoding is a filter, not an output oracle. `author_literal_classes.py:110–114` receives an already constructed raw sequence; `author_extended_classes.py:167–184` tests a bounded set of greedy/lazy tokenizations of an already constructed raw sequence. Compressed equality chooses eligibility and a trace; it never changes raw bytes. The comparisons use the DEFLATE body `z[2:-4]`; concrete proofs separately check the header, final Adler and completion. "Complete forward encoding" in the ledger should be read as complete candidate DEFLATE-body encoding.

Class membership checks equal Z plus equal independently calculated R; the representative supplies the expectation. Incomplete categories (`audit_witnesses.py:232–245`) inspect first-block type/presence only and give no credit.

Repeat profiles, offsets, status and empty prefixes are explicit constants at `tools/author_repeat_witnesses.py:22–42`; the wire checks verify them rather than construct them. The overflow uses 286+2 slots, two successful 138-zero runs, then rejects the third; the generator's adjacent 257+2 comment is wrong, but the witness follows the actual profile.

## Proof strength

The public exports are `src/lib.rs:60–61`. Their contracts at `src/inflate.rs:1335–1393` constrain concrete returned status/count, written output prefix or measuring-ring contents, and the model result without a success-only antecedent. `status_eq` (`:1268–1275`) rejects OutsideDomain rather than accepting it vacuously.

`src/witnesses.rs:221–251` calls `inflate_zlib_into` and `inflate_zlib_measure` and asserts actual status, produced length and output/ring representation; `src/witness_policy.rs:5–22` does the same with original segments and the option tuple. Their model-equality preconditions are discharged by unconditional concrete caller bodies. Constructor `None` branches are proved impossible from the concrete byte bound, not early-return escapes.

The repeat bridge (`src/witness_dynamic.rs:71–89`, composition `:118–164`), fixed-match bridge (`src/witness_match.rs:48–68`, linked to model steps at `:83–139`) and Adler bridge (`src/witness_checksum.rs:15–55`) check what they claim. No new trusted items, false witness preconditions, unguarded public-result claims or ghost replacement of a real public call. The synthetic three-repeat-kind lemma (`witness_dynamic.rs:352–374`) is expressly partial and not credited. Every newly covered successful output fits in the measuring ring, so its checked representation covers all expected bytes.

## Class accounting and scope

The committed audit, run with inventory writes suppressed, regenerates a byte-identical inventory. 48 individual + 295 (114 initial classes) + 217 (128 extended classes) + 3 special = 563 covered, 68 incomplete of 631. Ten policy records are outside the numerator. 242 classes = 136 fixed-literal + 89 stored + 17 fixed-match. These agree with `LEDGER.md:24–28`.

The 68 incomplete: 12 blocked, 32 first-fixed-block trace gaps, 7 further fdAT gaps, 1 dynamic profile, 16 malformed/ambiguous image/frame-R. The five blocked class manifests match the committed snapshots and local diagnostics; no unattempted instance is labelled resource-blocked.

fdAT R is correct for credited instances: `audit_witnesses.py:78–101` uses the enclosing fcTL dimensions with the IHDR format and strips sequence numbers; `:183–190` requires equality with the representative's raw length. 33 newly covered successful fdAT instances plus two empty-fdAT instances at R=5.

Reusing a representative for byte-identical Z, equal R and equal options is sound for inflate status/output, since the public refinement depends on concatenation, not segmentation. It does not prove PNG container acceptance, APNG composition, an integrated reader's effects, or a named original-boundary gate; the ledger keeps those distinctions and leaves M2 sign-off open (`LEDGER.md:50–57`; gate requirement `THEOREMS.md:191–193`).
