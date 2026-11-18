# Errata against the frozen PNG document set

Append-only. This file is **not** covered by the PNG freeze: it records
discrepancies between the frozen texts (`THEOREMS.md`, `spec-frozen/`, `PLAN.md`)
and the implementation. Nothing here edits a frozen file. New entries are appended
at the end with their date and the artifact they concern. An entry marked PROPOSED
has not been applied: it takes effect only after the owner approves it, and the
entry is then amended in place of a new one by appending an "Owner decision" line.

## Index and status (as of 2026-11-18)

| Erratum | Status |
|---|---|
| E1 | APPROVED (2026-11-18) |
| E2 | APPROVED WITH AMENDMENT (2026-11-18, council) |

## E1 — 2026-11-17: `inflate_verus` "no `alloc` dependency" (A-PNG-6) — APPROVED (2026-11-18)

**Frozen text.** `THEOREMS.md:161` (A-PNG-6): "`inflate_verus` has no `alloc`
dependency (build check, alloc-delta §9)". `PLAN.md:9` says the same of the
`inflate_verus` module list: "No allocation; no guard; no `alloc` dependency."

**Implementation.** The proof crate `png/inflate_verus` uses the same `vstd`
dependency declaration and feature set as `gif/gif_verus`, with the `alloc`
feature enabled, so that the pre-seeded verification artifact can be reused.
`vstd` is ghost and spec only: its model code is `cfg(verus_only)` and is erased,
so the shipped library contains no allocator references. Checked on the host and
`thumbv7m` release libraries, and on probes that instantiate both public entry
points (`inflate_verus/runs/0078.log`, `inflate_verus/runs/0060.log`).

**Proposed erratum.** Permit a ghost-only `vstd` dependency with the `alloc`
feature in the verification configuration, and require that production and
freestanding builds contain no `alloc::` paths and no `__rust_alloc`,
`__rust_dealloc` or `__rust_realloc` symbols, checked by instantiated-entry
probes.

**Effect on users.** None: the shipped code is unchanged and allocator-free.

**If rejected.** A `vstd` configuration without the `alloc` feature must be
built. Verus refuses that under `--no-cheating` in this sandbox (run 19:
`--is-vstd is not allowed with --no-cheating`; `inflate_verus/runs/0019.log`).

### E1 Status (2026-11-18)

APPROVED (2026-11-18).

Owner decision (2026-11-18): APPROVED. The verification configuration may use a ghost-only vstd dependency with the alloc feature; production and freestanding builds must remain allocator-free as checked by the instantiated-entry symbol probes.

## E2 — 2026-11-18: T-INFL(a) observable effects of generic `Segments` readers — APPROVED WITH AMENDMENT (2026-11-18, council)

**Frozen text.** `THEOREMS.md:112` (T-INFL(a)): "the bytes of `out` are the only observable effect besides `src`'s cursor, whose position after the call is unspecified but within `0..|Z|` with `concat()` unchanged". `THEOREMS.md:113` (T-SEG) imposes input-view, cursor, termination and work contracts, but no restriction on writes through other mutable references owned by a reader.

**Implementation.** The actual public entry points accept any `S: Segments`. A reader can set an unrelated caller-owned marker before delegating to `SegmentSlices::read`, preserving every T-SEG observation and adding no loop iteration. The reproducer `inflate_verus/review-astra/caller_effect.rs` changes a marker from 7 to 99 while an empty zlib stream returns `Done` with unchanged output. The verified implementation now preserves a typed `source_frame` through `read`, the bit reader and both public entries. For `SegmentSlices` this frame is the complete shared input borrow, and all other runtime source fields are cursor state; the source has no other mutable reference. Together with Rust's borrow rules and the public output/suffix contracts, this establishes the full effect restriction for `SegmentSlices`. A generic implementation selects its own frame: preservation of that frame cannot assert preservation of mutable referents it omits. Verus can express and verify a frame for a known representation, but its generic trait contract does not quantify over all reachable mutable memory or require a frame to enumerate every mutable referent. No new trusted heap/effect specification was added.

**Proposed erratum.** Replace the quoted effect clause with: "For `S = SegmentSlices`, the bytes of `out` are the only observable effect besides `src`'s cursor. For any other `S: Segments`, the inflate implementation itself writes only `out` and reader state; calls to `S::read` may additionally perform the effects permitted by that implementation's contract. Both cases preserve the reader's declared source frame and `concat()`, and leave its cursor within `0..|Z|`; the exact final cursor position is unspecified. Each production PNG reader must separately establish that it has no observable effect beyond its cursor, and the same restriction then holds for the instantiated inflate call." The analogous production-reader restriction applies to measuring calls with `ring` in place of `out`.

**Effect on users.** No decoded byte, status, limit, work bound or runtime behaviour changes. `SegmentSlices` callers retain the full effect restriction. Callers supplying a custom reader must account for that reader's effects; PNG's future IDAT, fdAT and iCCP readers acquire an explicit per-reader framing obligation rather than inheriting an unsupported generic guarantee.

**If rejected.** The generic effect clause remains unproved and M2 cannot be declared complete. A replacement reader interface must explicitly restrict and verify every reachable mutable referent, or the public theorem must be specialised to a closed set of readers whose representations admit that proof; no stronger fresh-cursor precondition repairs the present clause.

### E2 Status (2026-11-18)

APPROVED WITH AMENDMENT (2026-11-18). The amended text below replaces the "Proposed erratum" wording above, which is superseded.

Council decision (2026-11-18): APPROVED WITH AMENDMENT, 4 of 4 seats in all three rounds (sealed report `council/sealed/COUNCIL-REPORT-PNG-E2.md`, sha256 `a408ffce8f2fa60e8299eaac23782e07c30f8cec14211cb033ea30260a05f1c3`, kept outside this repository). Decided by the council under the project's delegation of minor specification calls. Of the four seats' texts, the coordinator adopted the most complete one, which keeps condition (iv) as a behavioural obligation (two seats' position) rather than only a recording obligation. Recorded dissent: one seat would allow readers holding mutable borrows when a complete representation-and-call argument establishes the same restriction; one seat notes L-SEG-FRAME rests on Rust's aliasing rules rather than a single Verus theorem.

**Amended replacement for the T-INFL(a) effect clause.** "For calls satisfying the verified entry contract, both entries preserve `concat()` and the reader's declared `source_frame()`, and leave the cursor within `0..|Z|`; the final cursor position is otherwise unspecified. Preservation of an implementation-selected frame does not establish preservation of omitted mutable referents or exclude other external effects. For `S = SegmentSlices`, `inflate_zlib_into` changes only output bytes and source cursor state. T-SEG alone gives no corresponding no-extra-effect guarantee for other implementations; the caller supplying another `S` must account for effects of `S::read` outside its cursor and declared frame. L-SEG-FRAME is a per-reader obligation, discharged at M3 together with L-SEG-PNG; M3 cannot be signed off while it is open for any IDAT, fdAT or iCCP reader. A reader type R satisfies L-SEG-FRAME when: (i) R and every type reachable from its fields contain no `&mut` reference, no raw pointer, no interior-mutability type (`Cell`, `RefCell`, atomics, `UnsafeCell` or a wrapper of one) and no I/O capability; (ii) R's `read` and every function it calls are safe Rust verified by Verus under `--no-cheating`, with no `unsafe`, `external_body`, `assume_specification`, static mutable state or I/O; (iii) `source_frame` returns every field of R that is not cursor state, the ledger lists each cursor-state field by name, and those fields implement only traversal or end-of-input bookkeeping; and (iv) for every reachable call to R's concrete `read`, including calls returning `None`, only those enumerated cursor-state fields change and no other externally observable effect occurs. The M3 evidence must record file and line for the type definition, concrete implementation, checked obligations and actual `inflate_zlib_into` and `inflate_zlib_measure` call sites, separately for IDAT, fdAT and iCCP, and must distinguish what Verus checks from what rests on inspection; the inspection argument must not be described as a Verus theorem. Once L-SEG-FRAME is discharged for R and connected to those call sites, the instantiated `inflate_zlib_into` call changes only output bytes and source cursor state. `SegmentSlices` is the first discharge of L-SEG-FRAME, recorded in the same form: Verus checks frame preservation, the structural conditions are checked against its type definition and verifier-enforced safe Rust, and the inference to no other observable effect rests on Rust's aliasing rules, which the project trusts and Verus does not prove. For `inflate_zlib_measure`, the same statements hold with `ring` in place of `out`; `ring`'s contents afterwards remain unspecified; this adds an effect restriction that frozen clause (b) did not state. Return values are excluded from these state-effect restrictions. Approval resolves only this statement mismatch: it neither completes M2 nor discharges the M3 reader obligations. No integrated PNG verification or release report may claim the no-extra-effect guarantee before the corresponding reader and call-site obligations are discharged, and no report may claim the original universal restriction for arbitrary T-SEG readers."
