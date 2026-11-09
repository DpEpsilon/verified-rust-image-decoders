# NOTES — M1 (plain implementation)

## Status
Done. `cargo test` passes all 111 vectors in `spec-frozen/VECTORS.json`.

## What was built
- `icns_creusot/src/lib.rs` — the whole spec as ordinary safe Rust: `parse`, `decode_rgb`,
  `decode_mask`, `combine`, the `Entry` record (R1) and the closed `Error` enum (R17, each variant
  carrying its exact case-sensitive name via `Error::name`). No contracts, no `creusot_std`
  dependency in the code path (the crate still declares `creusot-std`, unused in M1).
- `icns_creusot/tests/conformance.rs` — loads all vectors, runs `parse` first for every vector
  (R18), then the operation, and compares against `entries` or the expanded `*_runs`.

## Key decisions
- Offsets, sums and pixel products are computed in `u64` (R1) and only narrowed to `u32`/`usize`
  after the `offset <= len(bytes)` check, so a 32-bit target never narrows early.
- `parse` follows R15's exact error order: `FileTooLarge`, `TruncatedHeader`, `BadMagic`,
  `InvalidFileLength`, declared-length mismatch (`TruncatedFile`/`TrailingFileBytes`), then per
  entry `TruncatedEntryHeader`, `InvalidEntryLength`, `EntryOutOfBounds`, `TooManyEntries`.
- The 4,097th entry's header and length are validated before `TooManyEntries` is returned (R6/R15),
  matching the four `entry-count-*` vectors.
- `opaque_candidate` (R9): an R8 legacy code is never opaque; a type in the fourteen-code table is
  always opaque regardless of payload; any other type is opaque only on a full PNG or JP2
  signature match, and then `nominal_side` stays `None`.
- `decode_rgb` (R10/R11) treats the payload as three planar RLE channels only. The `it32`
  four-zero-byte prefix is removed only when at least four bytes are present; no other type removes
  a prefix even when its payload starts with `00 00 00 00`.
- RLE (R12) shares one input cursor across the three channels, keeps a fresh `produced = 0` per
  channel, checks `count > N - produced` before reading run data, and requires the cursor to be at
  the payload end afterwards (`RleTrailingBytes`).

## Harness notes
- The crate reads the fixture from `../spec-frozen/VECTORS.json` relative to `CARGO_MANIFEST_DIR`;
  the file is not copied or edited.
- `serde_json` is a dev-dependency only (allowed by BRIEF rule 1).
- `run_vector` returns a `Result` so the single test reports every failing vector, not just the
  first; the pass count is printed as `conformance: 111/111 vectors passed`.

## Not done (later milestones)
- No proofs or contracts yet (M2-M4). The code translates under `cargo creusot`, but with no
  contracts Creusot still emits panic-freedom goals for every function and they are unproved:
  `./verify.sh` run 6 exited 1 with 11 unproved files (`vc_parse`, `vc_decode_rgb`, `vc_rle_decode`,
  `vc_combine`, and the small helper functions). Those are exactly the M2 obligations, which M1 does
  not ask for; no contract was added to silence them.

## Creusot tooling constraints found while translating (recorded for M2)
- `#[derive(PartialEq, Eq)]` on a local type makes the Creusot translator fail: its derived
  `PartialEq` reaches for `DeepModel` (and, with the prelude glob-imported, `Clone`/`PartialEq`
  become ambiguous against std). `Entry` and `Error` therefore derive only `Clone`/`Debug`/`Copy`;
  the conformance harness compares their fields directly, so no equality trait is needed.
- Byte-string literals (`b"is32"`) panic the translator: they lower to `Literal::Bytes`, which is
  `todo!()` in `creusot/src/backend/term.rs`. All type codes and the `icns` magic are written as
  arrays of byte-char literals (`[b'i', b's', b'3', b'2']`).
- `for (i, chunk) in s.chunks_exact(2)` fails to compile under Creusot: every `for` iterator must
  implement `IteratorSpec`, and `ChunksExact` does not. `type_bytes_of` uses an indexed `while` loop.
- The crate keeps `extern crate creusot_std;` (not a glob `use`) so `cargo creusot` loads the crate
  without the name clashes above. It has no effect on ordinary builds.

## Exact result
```
running 1 test
test all_vectors ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.97s
```

# NOTES — M2 (proofs T1–T6)

## Status
Done. `./verify.sh` run 13: `Proved (34 files) ✔`, exit 0. `cargo test`: `conformance: 111/111 vectors passed`.
Executable behaviour unchanged: the only source edits are contracts, the RLE helper split
(`rle_channel` now returns the cursor instead of taking `&mut`), and a refactor of the four
lookup helpers from table loops to `if`-chains; the 111 vectors still pass byte-for-byte.

## Shape of the proofs
- A Pearlite **reference model** (`magic_model`, `eq4`, `nominal_model`, `legacy_colour_model`,
  `legacy_mask_model`, `is_legacy_m`, `sig_model`, `opaque_cand_model`, `hex_model`, plus the
  entry predicates `entry_ok`/`parser_ok`) is written independently of the executable code and
  relates each helper to its runtime twin by a *functional* `#[ensures]` (exact value equality),
  not just a range invariant. Literal `match` on `Int` and `Seq == seq![..]` do not reduce in this
  Creusot, so the models use `if`-chains over byte-wise `eq4(t, ..)`.
- `parser_ok(b, E)` is the success invariant: `E.len() ≤ 4096`, `b.len() ≤ 67,108,864`, and every
  entry satisfies `entry_ok` — the R1 range `8 ≤ start ≤ end ≤ |b|`, `type_hex` equal to the hex
  model of the eight bytes preceding the payload, `opaque_candidate` equal to the R9 classification
  of the type and payload, and `nominal_side` equal to the R14 table value. `parse` establishes it
  by a loop invariant; the four decoders take it as `#[requires]`; the wrappers `rgb_at_parse`,
  `mask_at_parse`, `combine_at_parse` **discharge it from a concrete `parse(b)`** and re-export the
  decoder bounds.
- `type_hex` provenance is proved, not assumed: `hex_hi`/`hex_lo` index a 16-char table by
  `n/16` and `n%16`, and `type_hex_of` builds the string with `String::from_iter<char>`; the
  postcondition `char_views(result@) == hex_model((*ty)@)` proves the eight hex digits are exactly
  those of the four type bytes.
- Termination: every loop carries a `#[variant]` (parser offset, RLE `n - produced`, per-pixel
  `n - i`). Creusot emits and proves the loop-variant goals without `#[check(terminates)]` (that
  attribute is unusable here: it forbids calling `Vec::from_elem`/`String::from_iter`, which the
  core must allocate). No recursion anywhere.

## Tooling constraints found (for M3/M4)
- `str::as_bytes` and `String::as_bytes`/`chars`/`Clone` have no spec in creusot-std 0.14.0, and
  `format!` is unusable. An `extern_spec!` for `str::as_bytes` translating to `self@.to_bytes()`
  compiles, but it is unnecessary here.
- Bare `derive(Clone/Debug)` under the `prelude::*` glob is ambiguous; derives and the manual
  `Display`/`Debug` impls are written fully qualified (`::core::clone::Clone`).
- `Option<T>` has no `View`; `x@` on an `Option` field fails. Models use `Option` values directly
  and compare them with `==`.
- `#[logic]` helper transparency requires the callee to be at least as visible as the caller; the
  model functions are `pub`, the executable helpers private.

## Not done (later milestones)
- T7/T8 (M3) will need `parse`'s exact `Result` (first error and all four fields per entry) to
  equal the model, and the `P(b,E) ⇔ parse(b)=Ok(E)` bridge; M2 only proves the invariant direction
  needed to discharge the decoder preconditions.
- The 40 named witnesses, the nine large-input lemmas L1–L6, and the ten mutations are M4.

# NOTES — M3 (T7, T8, L1, L2 proved; L4–L6 pending)

## Status
`./verify.sh` run 17: `Proved (51 files) ✔`, exit 0, at the **unchanged** M2 prover budget
(`fast 0.2 / time 1.0 / depth 6`). `cargo test`: `conformance: 111/111 vectors passed`.
Executable behaviour is unchanged: M3 edits added only `#[logic]` model functions, `#[ensures]`
postconditions, loop invariants, `proof_assert!` hints, and one refactor of `parse`'s
`opaque_candidate` computation into `classify_opaque`; the RLE/decoder bodies and the four public
signatures are untouched, and all 111 vectors still pass byte-for-byte.

## What M3 adds
- **`MP` model, independent of the code.** `model::mp` is the ordered R15 algorithm: `FileTooLarge`,
  `TruncatedHeader`, `BadMagic`, `InvalidFileLength`, `TruncatedFile`, `TrailingFileBytes`, then the
  entry loop `model::mp_loop` with `TruncatedEntryHeader`/`InvalidEntryLength`/`EntryOutOfBounds`/
  `TooManyEntries` in that order. Entries are the model record `MEntry` (type-hex code points, both
  offsets, `opaque_candidate`, `nominal_side`), built by `mk_entry` from the same helper models M2
  already proved functionally equal to the runtime helpers.
- **T7 (`parse(b) = MP(b)` as an exact Result).** `parse`'s first postcondition is
  `res_model(result) == model::mp(bytes@)`, where `res_model` maps an `Ok` vector through
  `entry_view` and passes an `Err` through unchanged. This is exact: same error variant, same number
  of entries, and each entry's four fields. The loop carries the equation
  `model::good(bytes, offset, declared, entries)`, i.e. the model from `8` equals the model
  continued from `offset` with the entries so far; `good_init`/`good_step`/`good_stop` and the four
  error-branch lemmas (`good_hdr`, `good_len`, `good_bounds`, `good_many`) are proved *inside*
  the `model` module, where `mp_loop` is visible.
- **T8 (`parse(b)=Ok(E) ⇔ P(b,E)` at the call sites).** Proved from T7, at the actual parse result:
  `parse_iff_p` states `∀E. (res_model(result)=Ok(E)) ⇔ p_model(bytes, E)` for the concrete `parse(b)`,
  and `parse_p_forward` states the concrete `Ok(es) ⇒ P(b, entries_view(es))` direction. `P` is
  `model::mp(b) == Ok(E)` with exact list equality and the same immutable `b`.
- **L1, L2.** `l1`: `|b| > 67,108,864 ⇒ MP(b)=Err(FileTooLarge)`. `l2`:
  `|b| = 67,108,864 ∧ ¬magic(b) ⇒ MP(b)=Err(BadMagic)` (not FileTooLarge), both proved from the
  model's top-level case order.

## Why `mp_loop` is hidden in a submodule
If `mp_loop` is `#[logic(open)]` at crate root, every call site unfolds the full recursion and the
parser-loop goals exceed the skeleton's prover budget (1 s, depth 6). Putting `mp_loop` in
`pub mod model` with `#[logic(open(self))]` makes it opaque to the root: the parser reasons about it
only through the `good_*` lemmas, each of which is a small, budget-sized proof. The model statements
are unchanged; only their *visibility* is restricted.

## Not done (this milestone)
- **L4, L5, L6 are not proved in the project.** The symbolic `H(k)`, `H(4097, s)` and `B(q)` lemmas
  are stated and each individual obligation checks out at a raised prover budget, but the crate does
  not pass them at the default budget; see `STUCK.md` for the exact failing goals and what was tried.
  Consequently the nine large vectors are not yet discharged by lemma instantiation.
- The 40 named witnesses and the ten mutations remain M4.

# NOTES — M4b (T12, T13: combine = MJ)

## Status
Done. `./verify.sh` run 127: `Proved (162 files) ✔`, exit 0, at the unchanged default prover budget
(`fast 0.2 / time 1.0 / depth 6`; `why3find.json` untouched). Clean-cache reproduction run 128,
`Proved (162 files) ✔` in 97 s. `cargo test`: `conformance: 111/111 vectors passed`.
`lib.rs` was backed up to `src/lib.rs.m4a` first. No `unsafe`/`assume`/`admit`/`trusted`/
`external_body`/`todo!`/`unimplemented!`/`requires(false)`.

## What M4b adds
- **`MJ`, independent of the code.** `rgba4(rgb, alpha, n)` is R13's row-major RGBA layout as a
  byte-at-index `Seq::create` (no concat/repeat). `mj_pair(b, E, ci, mi)` is the total present-pair
  model in R13's order: colour index, colour decode, mask index, mask decode, side test, `rgba4`.
  `mj_model(b, E, ci, mi)` prefixes R14's absent-index order (`MissingColor`, then `MissingMask`).
  `mj_pair_c`/`mj_model_c` are the same two functions on a concrete `Entry` list (selected decodes
  `mc_entry_c`/`ma_entry_c`); `mj_pair_view`/`mj_model_view` prove the two phrasings equal at every
  index pair, including out-of-range (both sides reduce before selecting an entry).
- **T12 (`combine = MJ` under `P(b,E)`).** `combine` is split: `combine_pair` (both indices present,
  contract exactly `mj_pair_c` + the T5 bound) and `combine` (absent-index `match`es + call to
  `combine_pair`, contract `mj_model`). `t12_combine` takes `p_model`, discharges `parser_ok` through
  `p_parser_ok`, and calls `combine`.
- **T13 (at a real parse).** `t13_combine_at_parse` calls the real `parse`; in the `Ok` branch it
  reads `P(b, entries_view(es@))` off `parse`'s T7 postcondition and calls `combine`. Its `#[ensures]`
  quantifies over every `e` with `p_model(bytes@, e)`, so it states the theorem for the unique such
  `E` and is vacuous only when `parse(b)` is `Err` (which is when `P(b,·)` is unsatisfiable).
- **Non-vacuity.** `witness_t12_p_realisable`/`witness_t13_parse_succeeds` build the minimal
  `H(0, ε)` header, prove `MP = Ok(empty)` via `l4`, and return `p_model` (the latter also calls the
  real `parse` and proves its `Ok` branch is taken). Five mutations in `vac-check` (runs 118–122):
  weakening `combine`'s own model `Ok` clause fails exactly T12 and T13; the others fail
  `vc_combine`/`vc_combine_pair` in the real call chain.

## Changes to existing code
Only `combine`'s body: `ok_or(..)?` → explicit `match`es pinned to `mj_model_c`, and the RGBA loop
moved into `zip_rgba` (which now requires `n ≤ 16384` to bound the `4*n` allocation). Public
signature, T5 bound and executable behaviour unchanged. All M2–M4a goals still among the 162.

## Not done (M4c)
The 102 small-vector model-vs-vector agreement, the ten manifest mutations, the remaining L1–L6
instantiations (L4–L6 were proved in M3).
