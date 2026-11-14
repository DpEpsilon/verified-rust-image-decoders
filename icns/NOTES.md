# M2 notes — proofs of T1 to T6

## Status

All of T1–T6 are proved. `./verify.sh` reports `verification results:: 47 verified, 0 errors`
(run 72, forced full reverify). `cargo test` reports `conformance: 111/111 vectors passed`
and `test result: ok. 1 passed; 0 failed`. The executable behaviour is byte-identical to M1:
the only changes to the algorithms are (a) three std calls Verus cannot model, replaced by
hand-written equivalents, and (b) loop `invariant`/`decreases` clauses.

## Files

- `icns_verus/src/icns.rs` — the implementation, now inside `verus! { ... }`, with the M2
  contracts. The public API (`parse`, `decode_rgb`, `decode_mask`, `combine`), the records
  and the error names are unchanged.
- `icns_verus/src/model.rs` — the formal reference model written independently of the
  executable code: `hex4`/`hex_val`/`code_of`, R8 `legacy_side`, R9 `nominal_side`, the PNG
  and JP2 signature predicates, `classify`, `entry_inv`, `parsed_inv`, and the hex lemmas.
- `icns_verus/src/lib.rs` — unchanged except for `pub mod model;`.

## What the contracts say (and where each theorem is discharged)

- **T1** — `parse` has no `requires`, and its postcondition is `parsed_inv(bytes@, E@)` on
  success. Verus proves, for every `bytes` at every representable length, that no indexing,
  arithmetic or narrowing can fail: every read is guarded by the `declared`-vs-`len` equality
  and the per-entry bounds check, and every offset is computed in `u64` before narrowing. The
  entry loop has `decreases declared - offset`, and `offset` grows by `length >= 8`. The entry
  list is capped at `MAX_ENTRIES = 4096`, and each record's `type_hex` is eight hex digits, so
  the only allocation `parse` requests is bounded. `parse` never allocates a pixel buffer.
- **T2** — `rle_decode` has `requires n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE` (16384, which
  covers the required 256/1024/2304/16384) and allocates exactly three `n`-byte channel
  buffers. Every payload read is guarded by `cursor`/`payload.len()` checks, the per-packet
  copy is a helper (`rle_copy`) with its own bounds, `produced` increases by `count >= 1`, and
  the outer and inner loops carry `decreases` clauses. It never indexes past `payload`.
- **T3** — `decode_rgb` has `requires parsed_inv(bytes@, entries@)`; on success it returns
  `width == height <= 128` and `data.len() == width*width*3 <= 49152`. The `side <= 128` fact
  is a proved lemma over the R8 table (each legacy side is 16/32/48/128), so `n <= 16384` and
  the channel and output allocations are bounded.
- **T4** — `decode_mask`, same precondition; success gives `data.len() == width*width <= 16384`
  and `width <= 128`.
- **T5** — `combine`, same precondition; success gives `data.len() == width*width*4 <= 65536`.
  Its two internal calls `decode_rgb`/`decode_mask` each carry `parsed_inv`, and those
  preconditions are discharged by the same `parsed_inv(bytes@, entries@)` under which `combine`
  itself is verified — no axiom is introduced at the call sites.
- **T6** — the success invariant is `parsed_inv` (see `model.rs`): `|b| <= 67,108,864`,
  `|E| <= 4096`, and for every entry `8 <= start <= end <= |b|`, an eight-digit lowercase
  `type_hex`, `type_hex == hex4(type_code)`, and `opaque_candidate`/`nominal_side` equal to
  `classify` of the actual parsed bytes. `parse` proves it, and the three wrappers
  `parse_success_invariant`, `decode_rgb_after_parse`, `decode_mask_after_parse` and
  `combine_after_parse` show a concrete successful `parse` discharging each decoder's
  precondition. The empty file is covered (`E = []` satisfies the `forall`).

## Surprises and tool obstacles

1. Exec functions cannot be called from spec/proof code in Verus, so the model cannot be a
   "spec copy" that mirrors the executable and then be *called* on it. The executable instead
   carries a proven postcondition phrased in model terms, and spec-level reasoning stays inside
   the model. The bridge is `hex4(code)` built by executable `push` calls, with the postcondition
   `s@ == model::hex4(code)`.
2. `format!("{:02x}", ...)` carries no Verus postcondition, so R1's `type_hex` is built by
   pushing characters, and `parse`'s invariant is phrased over `hex4`. `Entry::type_code` is
   proved to recover the four bytes (`code_of(hex4(code)) == code`).
3. Three std calls are unsupported inside `verus!` and were replaced, keeping behaviour
   identical: `u32::from_be_bytes` → explicit shifts-free arithmetic; `String::as_bytes` →
   `String::as_str().as_bytes()`; `slice::to_vec` → a copy loop. Array `for` loops and
   byte-string `match` patterns on `[u8; 4]` are also unsupported, so loops are `while` with
   index variables and the code tables compare the four indices explicitly.
4. `is_ascii_spec_bytes` is a broadcast lemma, not a callable one; the ASCII premise for
   `str`→`bytes` needs `broadcast use`, plus a proved lemma that an eight-digit hex string is
   ASCII.
5. Array `==` in a *spec* does not discharge by itself; tables compare indices.

## Not done (M3/M4)

- No `MP`/`MR`/`MC`/`MA`/`MJ` reference model yet; T6's invariant is phrased directly over the
  parser, which is what M2 requires. M3 supplies the `parse(b)=Ok(E) ⇔ P(b,E)` bridge.
- No non-vacuity/mutation protocol yet (M4).

## M3c status — exact parser/model bridge

T7 and T8 now verify with the parser's live `parses_to` postcondition and concrete decoder call-site derivations. L4, L5 and L6, including their compact vector instantiations, verify; L1 and L2 were already proved. `runs/121.log` records `verification results:: 142 verified, 0 errors`; the M3c dispatch used ten `./verify.sh` runs (112–121). `cargo test --test conformance -- --nocapture` reports 111/111 vectors. See `PROOF-REPORT-M3c.md`. L3 and the remaining full non-vacuity protocol belong to later work.

## M3 notes — the MP reference model (historical, incomplete at that time)

The MP model is now defined and internally proved in `src/model.rs` (94 obligations verify,
`runs/077.log`): `mp`, `scan`, `walk`, `entry_at`, the R15 error order, `classify` and the
`Accepted`/`p_of` predicates, plus the walk/scan bridge, the `be32` round-trip, the `H(k,s)`
subrange lemmas and the all-empty dot-string entry lemmas (`big_e`, `entries_big_e`).

T7 and T8 are **not** proved: `parse`'s live postcondition is still M2's `parsed_inv`; the
`parses_to` statement and its per-branch early-return lemmas exist, but the entry loop was not
rewritten to carry the ghost offset sequence before the tool-call cap was reached. L1/L2/L4 have
their core lemmas proved but are not assembled into the manifest's stated form; L5 and L6 are not
started. See `PROOF-REPORT-M3.md`.

## M4c status — non-vacuity and mutation protocol

All of M4c is done. `./verify.sh` reports `verification results:: 315 verified, 0 errors` (run 206,
252 s; 212 before this milestone plus 102 vector obligations and `lemma_mj_ms_eq`).
`cargo test --test conformance` reports `conformance: 111/111 vectors passed`,
`test result: ok. 1 passed; 0 failed`.

- `src/model_vectors.rs` (generated, 3,260 lines): 102 proof obligations, one per vector at or
  below 4,096 bytes, each stating that the model application equals the fixture's `expected` and
  discharged by `by (compute)`. Generated by `tools/gen_model_vectors.py`, which reads expected
  values only from `spec-frozen/VECTORS.json` and checks each of the nine large vectors' expanded
  bytes against the manifest's symbolic construction.
- `src/model.rs` gained `mj_ms` (MJ over `MpEntry`) and `lemma_mj_ms_eq`.
- Mutations 1-10 run in `mutation/mN/` from the pristine `mutation/base/`; each fails the
  verifier on the obligation `THEOREMS.md` names. Logs: `mutation/mN/failure.log`; summaries:
  `mutation/mN/result.txt`. Tooling: `tools/mutations.py`, `tools/run_mutations.sh`,
  `tools/mutation_summary.py`.
- Mutation 5 and mutation 10 both failed directly, so neither manifest fallback was applied.
- Mutation 3 needed the check moved in *both* `scan` and `walk`; the single-site attempt failed at
  the scan/walk bridge instead of L6 (`runs/197.log`, superseded by `runs/205.log`).
- See `PROOF-REPORT-M4c.md` for the full tick sheet.

## M4b status — MJ; T12, T13

T12 and T13 are proved. `./verify.sh` reports `verification results:: 212 verified, 0 errors`
(run 175, full reverify after `cargo clean`). `cargo test --test conformance` reports
`conformance: 111/111 vectors passed`, `test result: ok. 1 passed; 0 failed`. The M4b additions
live in `src/model.rs`: `interleave_rgba`, `mj`, `combine_decoded_eq`, the `MJ` case lemmas
(`lemma_mj_missing_color`, `lemma_mj_missing_mask`, `lemma_mj_mc_err`, `lemma_mj_ma_err`,
`lemma_mj_mismatch`, `lemma_mj_ok`), `lemma_interleave_rgba_len`/`_exact`, the result-unpacking
lemmas `lemma_decoded_eq_ok`/`_err`, the field-transfer lemmas
`lemma_mc_fields_eq`/`lemma_ma_fields_eq`/`lemma_mj_fields_eq`, and the two exec wrappers
`t12_combine_exact_for_p` and `t13_combine_after_parse`. `combine` in `src/icns.rs` carries the
postcondition `combine_decoded_eq(r, bytes@, entries@, color_index, mask_index)`; its executable
statements are unchanged (the `?` operators became explicit `match`es, and the RGBA loop gained
four ghost `forall` invariants). A working M4a copy of icns.rs is kept as `src/icns.rs.m4a`
(not compiled). 10 `./verify.sh` runs (166–175).

## M4a status — MR, MC, MA; T9, T10, T11; L3

T9, T10, T11 and L3 are proved. `./verify.sh` reports `verification results:: 193 verified, 0
errors` (run 164, full reverify after `cargo clean`). `cargo test --test conformance` reports
111/111. The M4a model lives in `src/model.rs`: `rle_count`/`rle_needed`/`rle_literal`/`rle_bytes`,
`rle_chan_from`, `rle_rest`/`rle_run`, `mr`, `it32_prefix`/`strip_it32`, `interleave_rgb`, `mc`/`ma`
(over executable `Entry`), `mc_ms`/`ma_ms` (over `MpEntry`), `decoded_eq`/`rle_result_eq`, and the
L3 lemmas `l3_string`/`l3_entry`/`lemma_l3_large_mask`/`lemma_l3_mask_t8mk`. Executable
postconditions: `rle_decode` carries `rle_result_eq(r, mr(payload@, n))`; `decode_rgb`/`decode_mask`
carry `rgb_decoded_eq`/`mask_decoded_eq`; `rle_copy` carries the `write_seq` packet equality. A
working M3 copy of icns.rs is kept as `src/icns.rs.m3`. 44 `./verify.sh` runs (122–165).
T12, T13 and the non-vacuity protocol are M4b.

## usize width (coordinator note, 2026-11-09)
Checked on a standalone Verus file with the same toolchain (0.2026.11.02): `assert(usize::MAX == 0xffff_ffff_ffff_ffff)` FAILS while `assert(usize::MAX >= 0xffff_ffff)` verifies, i.e. Verus treats `usize` as a 32-or-64-bit type by default unless a crate declares `global size_of usize == 8` (none of the crates here do: grep for `global size_of`/`size_of usize` is empty). The proofs therefore hold for both widths in the sense of the manifest's 'parameterised by a word width'. This addresses the 'no 32-bit verifier run' review finding as a width-generic proof; it is not an actual i686 build or test run.

## no_std + alloc conversion (2026-11-12)

`icns_verus` is now `#![no_std]` + `extern crate alloc` with vstd `default-features = false, features = ["alloc"]`. Verifier: **315 verified, 0 errors** (unchanged; `runs/001.log`, the first log in a recreated `runs/`). Conformance test green. Edit set (imports only), certification evidence and limits: `NO_STD.md`. Backups `icns_verus/**/*.nostd-1`.

## alloc-delta migration (2026-11-12) — in progress

The `try_*` ICNS crate replaces the old `parse`/`decode_rgb`/`decode_mask`/
`combine` with the sealed-reservation-log API (A-ICNS-1..8). Work in progress on
`dib-policy`: WIP snapshots in `icns/alloc-wip-1/` and `icns/alloc-notes/`.
`icns_verus/src/*.nostd-1` is the pre-delta green state (315 verified, 0 errors).

### Changes needed in the glycin-icns loader (do NOT edit glycin from here)

`~/projects/glycin-plan/glycin/glycin-loaders/glycin-icns/src/lib.rs` and
`tests/vectors.rs` must follow the API rename and the `type_hex` type change:

1. `src/lib.rs:55` `fn legacy_kind(type_hex: &str) -> Option<(bool, u32)>` and its
   call at `:104` (`legacy_kind(&e.type_hex)`): `Entry::type_hex` is now
   `[u8; 8]` (decision D2), so the parameter becomes `&[u8; 8]`, the eight
   `"69733332" => ...` match arms become byte-string literals
   (`b"is32" => ...`, `b"il32"`, `b"ih32"`, `b"it32"`, `b"s8mk"`, `b"l8mk"`,
   `b"h8mk"`, `b"t8mk"`), and the call site passes `&e.type_hex`.
2. `tests/vectors.rs:48` `fn side_of(type_hex: &str)` and its calls at `:135`
   and `:154`: the same `&str` -> `&[u8; 8]` change; `entries.get(...).
   and_then(|e| side_of(&e.type_hex))` still reads `&e.type_hex` as `&[u8; 8]`.
   Matching by `type_hex` (rather than `nominal_side`) is what keeps every
   `combine` call well-formed: the colour and mask always have R8 types of the
   same side. The byte-string literals preserve that property exactly.
3. API renames: `icns::parse` -> `icns::try_parse`; `icns::decode_rgb` ->
   `icns::try_decode_rgb`; `icns::decode_mask` -> `icns::try_decode_mask`;
   `icns::combine` -> `icns::try_combine`. The three decoders now return
   `Result<Decoded, TryError>` where `TryError` is `Format(Error)` or
   `OutOfMemory(..)`. `core_error(e: icns::Error)` still takes the frozen
   `Error`; a `try_*` caller must fold `TryError` first: `Format(e) => core_error(e)`,
   `OutOfMemory(_) => ` the loader's own allocation-failure `ProcessError`
   (never a byte-format error; THEOREMS-ALLOC-DELTA.md §0). `icns::MAX_FILE_LEN`
   and `icns::Error::name()` are unchanged.
4. `Entry` fields are unchanged apart from `type_hex` (still `payload: [u32; 2]`,
   `opaque_candidate: bool`, `nominal_side: Option<u32>`); `payload(data, entry)`
   and the `candidates`/`decode` logic need no other change.

## proof-contract gaps 4/5/7/8 closed (2026-11-12)

Sol's review of the alloc-delta ICNS step named four proof-contract gaps
(`(4)` MEDIUM, `(5)` MEDIUM, `(7)` LOW, `(8)` LOW). All four are now closed.
`./verify.sh` reports **413 verified, 0 errors** (`runs/044.log`, exit 0), up
from the 406 of the migration's green state; **no pre-existing obligation was
removed** (the +7 are the `pre_alloc_{c,a,j}`, `pre_alloc_*_concl` and
`site_max`/`req_within_site_max` spec functions, `lemma_public_{rgb,mask,combine}_prealloc`,
and the tightened `lemma_req_*` obligations). Green backups:
`src/*.alloc-f44`, `alloc-wip-1/alloc_guard.green-f44/`, `alloc-wip-1/{icns,model}.f44.rs`.

- **(4) `OutOfMemory` names its failed request on the mask and combine wrappers.**
  `try_decode_mask` and `try_combine` now carry the clause
  `Err(OutOfMemory(o)) ==> oom_in_req(o.site_spec(), o.elements_spec() as nat, Req_*)`,
  the same public postcondition `try_decode_rgb` already had. It is read out of
  the logged twins: `g_*_clause2` gives the failed last entry's `(site, elements)`
  and the prefix relation, and the new clause turns the last element of that
  prefix into a member of `Req_*`.
- **(5) `Error::name` and `TryError::name` bridged to `name_spec`.**
  `Error::name` (icns.rs) now has `ensures s@ == self.name_spec()@`;
  `TryError::name` (alloc_guard/sites.rs) has the same, discharging the
  `Format(e)` arm through `Error::name`'s own bridge. **No
  `#[verifier::when_used_as_spec]`** anywhere, matching the sibling cores.
  Sensitivity checked on two disposable `VERIFY_CRATE` copies
  (`alloc-wip-1/mut-name-{1,2}`, deleted after): mutating one spelling in
  `Error::name` gives exactly **1** error (runs/047.log, `412 verified, 1 errors`,
  the `Error::name` postcondition at `icns.rs:82`); mutating the `OutOfMemory`
  literal in `TryError::name` gives exactly **1** error (runs/048.log).
- **(7) Named `PreAlloc` corollaries for the RGB, mask and combine logged entry
  points.** `lemma_g_{rgb,mask,combine}_prealloc` prove
  `Req == [] ==> not OutOfMemory ∧ Eq_f ∧ L == []` from `G`; the logged twins
  now carry `pre_alloc_{c,a,j}_concl` in their `ensures`, next to the `PreAlloc`
  corollary `try_parse_logged` already had. The public bridges
  `lemma_public_{rgb,mask,combine}_prealloc` derive the observable parts
  (non-`OutOfMemory` and `Eq_f`) from the public contract's `oom_in_req` clause
  plus `pre_alloc_*`.
- **(8) G clause 3 tightened to the per-site bound.** The four clause-3
  predicates (`g_r_clause3`, `g_c_clause3`, `g_a_clause3`, `g_j_clause3`) now
  state `req_within_site_max`, i.e. each request `<= site_max(its own site)`:
  16,384 for `Channel(k)` and `MaskData`, 49,152 for `RgbOut`, 65,536 for
  `CombineOut`, `MAX_ROOM` entries for `EntryList` — the frozen per-site figures
  of `THEOREMS-ALLOC-DELTA.md` §0/`SPEC.md` §4, not the global 67,108,864
  `MAX_BYTES`. The helper lemmas `lemma_req_{r,c,a,j}_*` derive the per-site
  numbers from the checked sides (`lemma_legacy_side_n_bound` etc.).
  `g_p_clause3` was already per-site (`MAX_ROOM`) and is unchanged.

### `alloc_guard/sites.rs` hash for the coordinator (tools/ is owned elsewhere)

`tools/audit_alloc.sh` pins `alloc_guard/sites.rs` by
`EXPECTED_SITES_SHA`, and this change edits `sites.rs` (the `TryError::name`
`ensures`). `tools/` is owned by another agent, so the pin was **not** edited
here. Applied by the coordinator:

- old (green at run 41):
  `c48050e7af642407912e1792cf03be064ea48d3a4269a857be484201609de77e`
- new (after this change):
  `adcc09bed170a230216248e1c3898dcc1b32af392b45032352ed717044c88c41`

`alloc_guard/mod.rs` is unchanged:
`018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6`.

> Superseded by ERRATA E6 (2026-11-13): `alloc_guard/mod.rs` now has SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7` (fill by `resize`; see the E6 section of this file).

## alloc-delta migration — result (2026-11-12)

`./verify.sh` reports **406 verified, 0 errors** (`runs/041.log`, exit 0). The
pre-delta `no_std` green state was 315 verified, 0 errors; the 91 added
obligations are the `at_site_log_{p,r,c,a,j}` shape functions and their exit
lemmas, the four `try_*` entry points and their logged twins, the guard call
sites and `lemma_req_{r,c,a,j}_*`. **No pre-existing obligation was removed**:
the M2–M4c items (`parse`, `rle_decode`, `decode_rgb`, `decode_mask`, `combine`,
the model, `model_vectors`) are still present as `*_tracked` functions and their
`try_*` wrappers carry the same postconditions. Green backups:
`src/icns.rs.alloc-1`, `src/model.rs.alloc-1`, `src/lib.rs.alloc-1` and
`alloc-wip-1/alloc_guard.green-1/` (the `alloc_guard/` copies live outside `src/`
so the source lint does not scan the backup).

`cargo test --offline`: `tests/conformance.rs` 111/111 frozen vectors pass;
`tests/fault_injection.rs` 8/8 pass, exercising a failure at each of the six
reservation indices across the 111 vectors (87 cases allocate: 87 fail at
reservation 1, 33 at 2, 33 at 3, 16 at 4, 3 at 5, 2 at 6; 24 are `PreAlloc`).
`tools/audit_alloc.sh` (with `tools/alloc_scan.py`) reports OK on the production
modules, pins `alloc_guard/mod.rs` and `alloc_guard/sites.rs` by hash, and its
`--self-test` negative controls pass.

### Proof structure decisions

- Each `at_site_log_*` predicate separates the `OutOfMemory` arm (a genuine
  prefix, last entry failed, earlier entries `Ok`) from the non-`OutOfMemory` arm
  (the log is exactly the request list, every entry `Ok`). The exact-sequence
  form of the non-`OutOfMemory` arm is what makes the `G` clause-1 `all_ok`
  discharge.
- `combine` shares one log between the colour and mask decodes. `decode_mask_tracked`
  takes the colour decode's log as a prefix: `at_site_log_a` is stated relative to
  that prefix (`log_extends`), and `lemma_req_prefix_compose` reads the mask
  decode's log as `Req_C ++ (a prefix of Req_A)`.
- The two `Seq::push` index facts (`lemma_all_ok_push_false`/`_true`) are isolated
  in their own lemmas with an `implies #[trigger]` body; the same form is used at
  every `assert forall` over a log's `Ok` flags.
- `lemma_req_{r,c,a}_bound` decompose the clause-3 element bounds, so
  `lemma_req_j_clause3` only has to combine them.
- `model.rs` gains `lemma_legacy_side_ge_16` (the R8 table's smallest side is 16,
  needed for `1 <= n` in the decoders) and `lemma_entry_payload_bytes_len` now
  takes the subrange recommendations as explicit hypotheses.

### Errata for alloc-delta (do not edit alloc-delta/*.md)

- No ICNS site-count discrepancy: the frozen `THEOREMS-ALLOC-DELTA.md` §3/§4
  already says ICNS has seven call sites, matching the implementation
  (`try_with_room` once, `try_zeroed` five times, `try_copy` once; five
  `AllocSite` variants with `Channel(k)` called three times). The task brief's
  "six sites" understates the call sites but not the frozen text.
- The `log_extends`/prefix form of `at_site_log_a` is an implementation choice not
  in the frozen text; the frozen A-ICNS-5 describes only the standalone
  `try_decode_mask`, whose `pre` is `[]`. The frozen statement still holds as the
  `pre == []` instance.

## test and lint gaps after Sol review (2026-11-12)

Edited only `icns_verus/tests/fault_injection.rs`, `tools/alloc_scan.py`,
`tools/audit_alloc.sh` and this file. No verifier run; `icns_verus/src` untouched.
Results: `cargo test --offline -- --test-threads=1` gives conformance 1 test (111
vectors) ok and fault injection 12 tests ok (8 existing, 4 new); `tools/audit_alloc.sh
--self-test` OK.

**Peak live bytes (finding 1).** The allocator now tracks a PEAK (maximum of the
live-byte total over the call, updated only after `System` returned non-null, so a
failed request counts for nothing) and the live level at the moment a request is
failed. New test `combine_peak_live_bytes_follow_the_stated_phases` builds ten
colour/mask side pairs. Matching sides (16, 32, 48, 128): failing the RGB output
shows 3 n_c, failing the mask copy shows peak 6 n_c with 3 n_c live, failing the
combine output shows 3 n_c + n_m live, the full call peaks at 8 n (131,072 bytes
at 128x128). Mismatched sides (six pairs): peak is max(6 n_c, 3 n_c + n_m) and
`TypeMismatch` leaks nothing; 16x16 colour with 128x128 mask peaks at 17,152 bytes
(768 + 16,384) after five allocations (no combine output). The asserts are `<=` the
stated number (the mandated check) plus `==` (request sizes are exact under
`try_reserve_exact`). The numbers are an ownership fact checked by this test, not a
verified clause. `frozen_vectors_respect_the_peak_bounds` bounds every frozen
vector by 131,072 bytes (largest parse peak 114,688 = 4,096 entries x 28 bytes,
largest decode peak 98,304), and each frozen vector's observed peak must equal an
ownership model of its logical request list.

**Physical trace and gates (finding 2).** Every call records the ordered successful
allocations (size, align, realloc flag). Per vector: trace length equals the number
of logical requests with k >= 1 (recovered from the `OutOfMemory` values at event
1, 2, ...); each event has size >= elements x element size, align == element align,
no realloc (checked in all runs, injected or not); the k-th failed event equals the
control trace's k-th event; the trace before a failure is the control's prefix; live
bytes are 0 after an `OutOfMemory`; site order is a prefix of the spec's order with
n, 3n, 4n relations. On this corpus all 174 events have exactly the requested size.
Returned `Vec` capacities are checked (`capacity() >= request`, and live bytes after
the call equal capacity x element size) for the entry list and the three `Decoded`
outputs. The ad hoc thresholds 1/256/768/769 in the suite are replaced by
T in {1, 4,096, 65,536, 67,108,864}, each windowed to every request size of the
call (fail iff size >= T; exactly one failed request; result and trace otherwise
unchanged). Suite assertions: reached cases = injected cases for each schedule.
Counts (cases where some request >= T / injected): 1 B: 87/87; 4,096 B: 13/13;
65,536 B: 6/6; 64 MiB: 0/0, stated UNREACHABLE (largest ICNS request is the
4,096-entry list, 114,688 bytes), so it is a no-failure control. The small
threshold test now runs the mandated schedule on the 768-byte RGB output plus the
boundary values 256, 768, 769.

**Zero-element request versus PreAlloc (finding 3).** The suite classifies each case
from an independent oracle of R2-R4 (`header_ok`) and the physical event count:
PreAlloc (error before the first reservation attempt), zero-element `EntryList`
request (valid header, `(|b|-8)/8 == 0`, logical event, no physical allocation, no
event under any schedule) and physical-allocation. Counts on the 111 frozen
vectors: 22 PreAlloc + 2 zero-element (`minimal-empty`, `truncated-entry-header`,
both asserted by name) + 87 physical = 111. The previous 24 "PreAlloc" was these 22
plus the 2 zero-element cases. The per-reservation failure counts keep their
meaning and are now asserted exactly: 87/33/33/16/3/2 (reservations 1..6). New
test `zero_element_request_is_a_logical_event_with_no_allocation`. The logical
zero-element event itself cannot be read at run time (the log is ghost): the test
checks that no allocator event occurs and that the oracle says the reservation call
is reached.

**Lint scoping (finding 6).** `alloc_scan.py --allow-reserved` blanks `res.push(` and
`res.into_vec(` only inside a function body that binds `res` from the guard
(`Ok(res)` / `Ok(mut res)`, or `let [mut] res = [match] ..try_with_room(..)`, which
is how `try_parse_logged` binds it) and calls `try_with_room` in that same body.
`audit_alloc.sh` now requires the `--allow-reserved` scan to be clean and the scan
without the flag to show exactly one `.push(` and one `.into_vec(` and nothing else;
it no longer counts findings "anywhere". Self-test negative controls: an unrelated
`fn bad(res: &mut Vec<u8>) { res.push(1); }`, an unrelated `res.into_vec()`, a local
`res` Vec beside a stray `try_with_room`, and a `v.push(x)` replacing the guarded
push are all flagged; the guarded forms and the real `icns.rs` stay unflagged; an
`invariant` clause containing `Seq::push` stays unflagged (clause stripping kept).
Still a falsification lint, not enforcement: inside a guard-binding function a
second variable named `res` would be suppressed.

**Pin to update.** `EXPECTED_SITES_SHA` in `tools/audit_alloc.sh` is the single
variable holding the `alloc_guard/sites.rs` hash (comment above it says so). It is
still the pre-change value `c48050e7...` and the script passed against the current
`sites.rs` at the time of writing; when the other agent changes `sites.rs`, update
that one line to the new `sha256sum`.
- 2026-11-12T16:27:22+00:00: coordinator re-pinned EXPECTED_SITES_SHA c48050e7... -> adcc09be... after proof-contract fixes (run 50, 413/0)

## ERRATA E6 update (2026-11-13): `try_zeroed` fills with `resize`

`alloc-delta/ERRATA.md` E6 changed the guard body in all four cores: `try_zeroed` now calls `try_reserve_exact(n)` and, on `Ok`, `v.resize(n, 0u8)`, instead of a `push` loop. `alloc_guard/mod.rs` is byte-identical in the four cores with SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`; the pin in `tools/audit_alloc.*` was updated. The trusted list gains A-STD-4: `Vec::<u8>::resize(n, 0)` on a `Vec` with `len == 0` and `capacity() >= n` does not allocate or reallocate. It is observed behaviour of std 1.99.1 (pinned source, counting-allocator and fault-injection tests: one allocation, zero reallocations), not a documented guarantee; `Vec`'s docs say bulk insertion methods "may reallocate, even when not necessary". A reallocation inside `resize` would be infallible and would abort, so it cannot be detected after the fact. The reason is cost: for a 64 MiB buffer the `push` loop takes 47.8 ms and `resize(n, 0)` 6.6 ms (`bench/fillbench/RESULTS.md`).

Latest verifier result: run 51, **412 verified, 0 errors** (before E6: 413 (run 50)). `alloc_guard/sites.rs` is unchanged (SHA-256 `adcc09bed170a230216248e1c3898dcc1b32af392b45032352ed717044c88c41`). A-STD-2 is still used, by `try_with_room`/`Reserved` in `sites.rs` for the entry list (the `push` of each entry below `room`); it is no longer used by `try_zeroed`, which serves the three channel buffers, the RGB output and the combined output. `tests/fault_injection.rs` labels its no-`realloc` assertions A-STD-2; for the `try_zeroed` buffers the property they check is the one A-STD-4 states.

Statements elsewhere in this file that describe the guard as a `push` loop, that use A-STD-2 for `try_zeroed`, that quote the old `mod.rs` hash `018ed17a...` or that give the pre-E6 verified count describe the cores before E6 and are superseded by this section. Test and lint results recorded in this file predate E6 unless dated 2026-11-13; a post-E6 rerun of them is not recorded here.

## ERRATA E10 update (2026-11-14): D3/D4/D8 visibility and `#[inline(never)]`

`alloc-delta/ERRATA.md` E10 applied the owner's decisions D3/D4/D8 to `alloc_guard/sites.rs`: `Reserved::push` and `Reserved::into_vec` gained `#[inline(never)]` (D8), and `Reserved<T>`, `Reserved::push`, `Reserved::into_vec` and `try_with_room<T>` became `pub(crate)` (D3/D4), so a downstream crate can no longer call them and violate `Reserved::push`'s `len < room` precondition. No `requires`/`ensures` and no proof changed. The new `sites.rs` SHA-256 is `5dff9ec076a778b04b782b3dc2e8a290b92c333cee0b281ebdb4619ac037892b` (was `adcc09be...`), `EXPECTED_SITES_SHA` in `tools/audit_alloc.sh` was re-pinned, and the three `generic-export` entries were removed from `alloc-delta/audit/allowlist/decisions/icns.toml`. Verifier: run 52, **412 verified, 0 errors** (unchanged). Tests: 111/111 conformance, 12/12 fault injection. The production audit reports only `allowlist-reviewer` (`runs/20261114T055635Z`); the `guard-inline-never` finding is gone.

The "Latest verifier result" and `sites.rs` hash in the paragraph above are superseded by this section.

## ERRATA E11 update (2026-11-14): public-API reduction

The intended public API is `try_parse`, `try_decode_rgb`, `try_decode_mask`,
`try_combine`, `Entry`, `Decoded`, `Error`, `TryError`, `MAX_FILE_LEN` and
`Entry::type_code`. `model` and `model_vectors` are now `pub(crate)`; in
`src/icns.rs` the `pub` items no `pub` contract names were demoted (80 -> 73) and
the 61 named by a `*_logged` or public `try_*` contract (the five `*_logged`
functions, the `G`/`Prog`/`Req`/`PreAlloc`/`AtSiteLog` families, `entry_count`,
`reach_p`, `site_max`, `log_extends`, `oom_in_req`, and the five
`lemma_*_prealloc` bridge lemmas) are `#[doc(hidden)]`. No contract or proof text
changed. Verifier: **412 verified, 0 errors** (unchanged). Tests: 111/111
conformance, 12/12 fault injection. `glycin-icns` checks clean.
See `alloc-delta/audit/API-SURVEY.md` and ERRATA E11.

## ERRATA E12 update (2026-11-14): `try_rle_logged` demoted to `pub(crate)`

Sol's review of E11 found that `try_rle_logged` (the logged RLE entry point,
`src/icns.rs` line 1822) was still `#[doc(hidden)] pub` while its size
restriction `1 <= n <= 128*128` was only a Verus `requires` clause. Ordinary Rust
from another crate could call it with a size of its choosing and make it reserve
three 64 MiB channel buffers. It has no external caller (its only caller is the
`t9_rle_exact` witness), is not one of ICNS's `required_roots`, and no `pub`
contract names it, so it is now `pub(crate)`; `#[doc(hidden)]` was removed and no
`requires`, `ensures` or proof text changed. Verus accepted the demotion, so no
executable size check was needed. Verifier: run 53, **412 verified, 0 errors**
(unchanged; a visibility change adds and removes no obligation). Tests: 111/111
conformance, 12/12 fault injection. The draft allowlist was regenerated (new
`source_hash` `cda2ebe74a1a747f9862bfb1e519294c9925515d88816cf01d237df2cea7b269`;
entry set unchanged at 50). The production audit is `alloc-delta/audit/runs/20261114T084716Z`
(only `allowlist-reviewer`); ICNS's walk is one instance and three edges smaller
because `try_rle_logged` is no longer a root. See `alloc-delta/ERRATA.md` E12 and
`alloc-delta/audit/API-SURVEY.md` section 6.
