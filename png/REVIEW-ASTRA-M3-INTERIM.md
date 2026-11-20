# Interim M3 review (GPT-6 Astra, 2026-11-20)

Scope: committed state b07454b of png/png_verus (M3 commits 8e05689 to b07454b). Read-only review; Astra ran no verifier, cargo or decoder and edited no file. Saved verbatim by the coordinating session. Not an M3 sign-off review.

## Verdict

I found one representation-contract gap and one omitted mandatory gate in the M3 accounting. I did not find a concrete input for which the reviewed structural walk returns the wrong first error, an `OutsideDomain` escape on the proved executable entry paths, or a new trusted proof shortcut. The ledger correctly stops short of M3 sign-off and explicitly excludes most unfinished public-entry obligations.

All references below are to committed state **b07454b** in `/home/ws/projects/verified-decoders`. I read repository text through Git, did not use the concurrently changing working tree, and did not run Verus, Cargo or a decoder.

## Findings, ranked by severity

### 1. Medium — the claimed complete raw-Info representation does not carry the sBIT count’s representation invariant

The claim at `/home/ws/projects/verified-decoders/png/png_verus/LEDGER.md:135` and `/home/ws/projects/verified-decoders/png/png_verus/evidence/claims-m3-document.json:67` says the view represents every exposed field, excluding only unused sBIT/bKGD padding. The sBIT count is an exposed field, not padding: `/home/ws/projects/verified-decoders/png/png_verus/src/png.rs:268–269` exposes `Meta<([u8; 4], u8)>`.

However, `/home/ws/projects/verified-decoders/png/png_verus/src/info.rs:28` represents this pair only as `v.0@.subrange(0, v.1 as int)`. The representation needs `v.1 <= 4` before that subrange has its intended meaning. `Info::ranges_wf` at lines 14–17 checks only iCCP-name and eXIf ranges. `Doc::wf` at `/home/ws/projects/verified-decoders/png/png_verus/src/walk_state.rs:39–48` does not add an sBIT count invariant. The storage and whole-walk postconditions export the view equality and these weaker well-formedness predicates, not the missing count bound.

The local parser does prove the needed fact: `/home/ws/projects/verified-decoders/png/png_verus/src/metadata.rs:34–39` includes `n <= 4 && n == size` in `Value::wf`, and the actual store copies that count at `/home/ws/projects/verified-decoders/png/png_verus/src/info.rs:73`. The gap is that the storage contract discards this physical representation fact instead of carrying it through the walk invariant.

A concrete contract failure is a truecolour document with sBIT `[8, 8, 8]`: a downstream verified consumer needs to establish that the returned count is three, or at least at most four, before using the count to slice the four-byte array. The current returned-Doc invariant also admits `Meta::Present(([8, 8, 8, 0], 255))`; the view then uses an out-of-domain subrange rather than an explicitly invalid representation. View equality alone is not a substitute for proving the subrange’s domain. I found no current PNG input that actually produces count 255; this is an under-specified refinement contract, not an observed decoder bug.

Add an explicit Info representation predicate containing the sBIT count bound, preserve it through initialization/storage/discard, and include it in `Doc::wf`. With the bound available, sequence equality can establish the exact meaningful count. Until then, narrow the “all exposed fields” claim. This does not invalidate the separately proved iCCP/eXIf range bounds.

### 2. Low — the current remaining-gates inventory omits E2’s mandatory L-SEG-FRAME discharge

The latest ledger’s open-gate rows and resume instructions at `/home/ws/projects/verified-decoders/png/png_verus/LEDGER.md:17–19,31`, and the latest claim packet at `/home/ws/projects/verified-decoders/png/png_verus/evidence/claims-m3-source-lifecycle.json:124–159`, do not name L-SEG-FRAME. Neither the ledger nor any `claims-m3-*.json` contains a per-reader L-SEG-FRAME record.

Approved E2 at `/home/ws/projects/verified-decoders/png/ERRATA.md:70` requires this separately for IDAT, fdAT and iCCP before M3 sign-off. It requires named cursor fields, type/read definitions, actual inflate call sites, and a distinction between checked frame preservation and the inspected Rust-aliasing argument. Passing T-SEG or preserving an implementation-selected `source_frame` does not discharge that obligation by itself.

The material needed for the IDAT/fdAT inspection exists: `/home/ws/projects/verified-decoders/png/png_verus/src/reader.rs:51–65` identifies the runtime fields, and lines 144–230 contain the checked implementation. What is missing is the required ledger/evidence discharge and call-site connection. The actual image inflate is `/home/ws/projects/verified-decoders/png/png_verus/src/image.rs:337`; iCCP has two distinct instantiations at `/home/ws/projects/verified-decoders/png/png_verus/src/png.rs:659–663` and `678–682`.

The concrete assurance failure would be completing the latest listed resume tasks and then treating M3’s reader obligations as exhausted, without recording the iCCP measuring call or the concrete readers’ no-extra-effect argument. The current code does not exhibit an extra effect, and the ledger does not currently claim M3 completion; this is missing mandatory gate accounting, not evidence of an unsound Segments implementation. Add an explicit open L-SEG-FRAME row now, then discharge it in E2’s prescribed form.

## Claims checked and found sound within their stated scope

- **T2 and corrected L-CRC-TABLE.** The header model preserves P2/P3/P4 error order; the executable prefix has no validity precondition. The CRC model is bitwise and does not read the executable table. The ledger credits the remainder-table and finalized-message equations allowed by approved E5, not the false literal `TABLE[i] = Crc([i])` equation.
- **Internal whole-walk equality and termination.** `/home/ws/projects/verified-decoders/png/png_verus/src/walk.rs:108–145` has an unconditional executable-to-model equation, an explicit exclusion of `OutsideDomain`, and a decreasing remaining-input measure. The guards in `document_model::own` and `drive` therefore do not silently narrow this executable theorem to valid inputs. Fresh-event initialization is checked in `/home/ws/projects/verified-decoders/png/png_verus/src/callers.rs:18–44`.
- **Rule order examined.** Framing precedes CRC, CRC precedes duplicate/order/value handling, PLTE’s own ordering checks precede palette-value checks, APNG missing-control/sequence/presence checks follow the frozen order, and IEND checks image presence, APNG completion and trailing bytes in order. I found no divergence in these paths from SPEC and the frozen oracle.
- **Transport is not merely ghost tracing.** The walk’s erased IDAT/frame sequences are connected to the production readers by `DataReader::for_doc`, `DataReader::for_frame` and validated fcTL selection. The fdAT proof correctly treats intervening non-fdAT chunks as empty charged segments and claims only a lower bound from payload-range count. The active-frame proof checks fcTL CRC provenance; Static fallback proves a default-only document.
- **Source lifecycle is scoped honestly.** The checked transitions fuse the source state before returning a job, and the public iterator uses them. The ledger explicitly leaves public receiver composition, the ordinary `num_plays` update, allocation logs and disposal/image work open. `FrameJob::wf` is a source-selection predicate, not a compositor invariant.
- **Witness and public-entry limits are disclosed.** The new whole-input witnesses call production walk/reader/source-transition code, but use empty compressed streams and are not the frozen public-entry witness set. The ledger says so. The conditional cap guards and model-only `empty_fdat_trace` are not presented as complete frozen witness coverage. Runtime conformance is kept separate from proof credit.
- **Trust boundary.** I found no `assume`, `admit`, `external_body`, `assume_specification` or verifier-external declaration in the newly claimed walk/transport/source paths. The ordinary public allocation bodies still use `Tracked::assume_new()` and unverified `?`/conversion composition, but the ledger expressly excludes their allocation/public-entry theorems. Those bodies must remain outside any claim of complete A-PNG-2/A-PNG-7 or L-WALK-FIRST.

## Evidence consistency

The cited verifier totals match the retained transcripts and RUNLOG: runs 24/33/48/59/63 report 78/104/120/136/148; runs 79/98/106/124/142 report 207/240/252/290/301, each with zero errors. The cited intermediate counts also match, including run 134 at 300 and runs 138/140 at 301.

Run 142’s transcript reports **301 verified, 0 errors** at `/home/ws/projects/verified-decoders/png/png_verus/evidence/run-0142.txt:715`. All **45 source hashes** in `/home/ws/projects/verified-decoders/png/png_verus/evidence/checkpoint-0142.json` match the committed blobs at b07454b.

Run 143 reports **599 Strict + 13 cap + 236 policy = 848 records**, with **846 passed, zero failed and two skipped**, at `/home/ws/projects/verified-decoders/png/png_verus/evidence/run-0143.txt:370–373`; all seven council tests pass at lines 387–396. Earlier runtime packets correctly distinguish the 810-record and 846-record historical bundles. I found no inflated verifier count, hidden conversion of a skip to a pass, or runtime result credited as a completed formal milestone.