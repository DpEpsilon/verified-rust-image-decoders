# PROOF-REPORT-M2 — theorems T1, T2, T3, T4, T7 (ICO core)

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), toolchain
1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`, dependency
`dib_verus` (path `../dib/dib_verus`, unmodified).
**Final result line:** `verification results:: 79 verified, 0 errors`
(`runs/078.log`, a full reverify after `cargo clean`).
**Conformance:** `cargo test --release` →
`conformance: 217/217 vectors passed`, `test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 77 (runs 2–78 of `RUNLOG.tsv`; run 1 was the
M1 skeleton). 13 exited 0; the others were intermediate failing builds during
proof development, plus one disposable-copy mutant probe each for the cap and the
signature mutations. No obligation hit the five-failure stop rule; no `STUCK.md`.
**Tool calls:** about 150 of the 400 allowed (my own count; the harness count is
authoritative). Roughly half were probe-crate verifier runs isolating one Verus
form at a time.

## What was built

`ico_verus/src/ico.rs` (1,198 lines) moves the whole M1 implementation inside
`verus!` with contracts. `src/ico.rs.m1` preserves the M1 plain file byte for byte
(the same bytes as `ico.rs` before this milestone).

`ico_verus/src/model.rs` (862 lines) is the independent reference model over
`Seq<u8>` and mathematical integers, transcribed from `spec-frozen/SPEC.md`:
`b8`/`le16`/`le32`, the limits, `png_signature`, the directory guard `h`, the
record model `me` (with `defect_of`, `kind_of`, `nom_dim`), the parse model `mc`,
and the predicates `parse_eq`, `entries_well_formed`, `entry_range_ok`,
`entry_class_ok`, `classification_ok`, plus the equality model
(`opt_u16_eq`/`status_eq`/`entry_eq`), the decode-side decision `md_ico`, the
ICO-side postcondition `decode_pre`, `error_of_defect`, and the arithmetic
lemmas. Nothing in `model.rs` calls or mirrors `crate::ico`; the executable links
to the model only through proved postconditions.

## Theorems proved

### T2 · exact parse — proved at the manifest's quantification

`parse`'s postcondition is `parse_eq(bytes, r)`:
`match mc(b) { Ok(es) => r = Ok(E) with E@ = es | Err(e2) => r = Err(e2) }`.
So a bad first record beside a good last record yields both, in wire order, with
their invalid statuses, and the sole directory-wide error is `H`'s, in R2/R3
order (cap, six bytes, reserved, type, nonempty count, count cap, whole table).
`directory_guard`'s postcondition is `match h(b) { Ok(hr) => Ok(h) with
hdr_matches(hr,h) | Err(e) => Err(e) }`, so the executable's checks fire in the
model's order.

### T1 · safety — proved

`parse` and `decode_entry` carry **no `requires`**, so Verus discharged both
bodies for every `&[u8]` a 32- or 64-bit target can represent. No panic,
out-of-bounds read, unchecked `usize` overflow or lossy narrowing is reachable:
the cap is tested before any read; `read_u16`/`read_u32` were replaced by
`rd8`/`rd16`/`rd32` whose preconditions prove the bytes exist, and every call site
proves that precondition from the directory guard; `offset + size` is formed only
after `offset <= |b| && size <= |b| - offset` is proved in `u64`; the `u64`-to-`u32`
narrowing of `dir_end` and of the range end is proved exact. For `decode_entry`
this is T1 for the ICO-side code: the directory guards, the index check (kept
`u32`-wide, never narrowed), the list-length check, the selected-record equality
via `record_eq`, the selected-defect mapping, PNG delegation, and the guarded
declared-slice construction. The `decode` call on that slice is the DIB core's own
public contract, whose safety is the DIB crate's T1, not re-proved here. The
no-panic and no-overflow clauses are discharged by Verus verifying the body; there
is no postcondition that can *state* "does not panic", the same assurance shape the
TGA and ICNS reports used.

### T3 · record validity — proved

`parse`'s success branch additionally ensures
`entries_well_formed(bytes, E@)`: `1 <= count <= 1024`, `|E| = count`, every
`E[j] = me(b, hr, j)`, and every valid `E[j]` has
`dir_end <= raw_offset < raw_offset + raw_size <= |b| <= 67,108,864`. Invalid
entries carry raw offset/size and no range. Derived from `mc` alone by
`lemma_parse_ok_well_formed`, with the witness `t3_record_validity` (hypothesis
`parse_eq(b, Ok(v))`, conclusion `entries_well_formed(b, v@)`).

### T4 · signature classification — proved

`parse`'s success branch also ensures `classification_ok(bytes, E@)`: for every
valid `E[j]`, `kind = Png` iff `raw_size >= 8` and the eight bytes at
`raw_offset` equal `SIG`, with the payload `[start, start+size)`. Only valid
entries are constrained. `has_png_signature`'s postcondition is exactly the model
predicate, and its loop reads only bytes below `end`. Witness
`t4_signature_classification`.

### T7 · arithmetic and output bounds — proved

`dir_end <= 16,390` and `count <= 1,024` come from `lemma_dir_end_bound`
(`6 + 16*count` for `0 <= count <= 1024`). The width-generic argument, stated in
`lemma_u32_width` and in the witness `t7_arithmetic_bounds`: the cap
`|b| <= 67,108,864 = 2^26` bounds every `usize` quantity here by `2^26`, far below
`u32::MAX = 2^32 - 1`, and `u32::MAX` is at most `usize::MAX` on the 32-bit
(`2^32 - 1`) and the 64-bit (`2^64 - 1`) target. So each `u64` comparison
(`off > len`, `len_word > len - off`, `dir_end > |b|`) cannot wrap, and each
`u64 -> u32` narrowing keeps its value. The range is inside the file
(`raw_offset + raw_size <= |b|`), and `decode_entry` slices exactly
`[payload.start, payload.end)`; it never allocates a pixel vector and calls the
DIB core at most once. `parse`'s one reservation
(`Vec::with_capacity(header.count)`) is bounded by 1,024 after the whole-table
guard.

## What is deferred to M3 (T5, T6, T8)

`decode_entry`'s postcondition this milestone is `decode_pre`, which fixes the
whole ICO-controlled decision `md_ico` exactly and, on the selected-DIB branch,
requires the result to be a DIB success or a DIB error and nothing else. The exact
DIB bytes and errors, and the composition with `dib_verus::model::mp` on the
declared slice, are T5; the full precedence table (including the selected DIB R14
priority) is T6; termination is T8. No theorem is stated under a weaker-than-manifest
form: I did not name `decode_pre` "T5". The model `MD` is not written, since
writing it without its call would be a weaker-than-manifest `MD`.

One clause of T7 is **deliberately deferred**: the manifest's "DIB output obeys
the dependency's `4*w*h <= 67,108,864` postcondition". That follow-on claim
belongs to the M3 composition; the ICO-side bounds (input cap, table end, range)
are proved here, and the DIB call's own postcondition `decode_mp_eq` is on the
public `dib_verus::decode` signature the call uses.

## Executable changes

Three changes to `ico.rs`, all proof-driven and all behaviour-preserving
(217/217 still pass):

1. `read_u16`/`read_u32`/`read_u8` (`Option`-returning) became `rd16`/`rd32`/`rd8`
   with existence preconditions. Every call site already proved existence, so no
   runtime path changed; the M1 `None => Err(TruncatedDirectory)` arms were
   provably dead for a directory that passed the guard.
2. The M1 defensive `if dir_end > u32::MAX { return Err(TruncatedEntries) }` in
   `directory_guard` was dropped: `dir_end <= 16,390` makes it unreachable, and
   keeping it would have meant keeping a dead second `TruncatedEntries` site.
3. `reconstruct` returns `Entry` (not `Option<Entry>`) and `parse` has no
   unreachable arm: every read is proved in bounds from the directory guard.

## Sensitivity check (disposable copies, `VERIFY_CRATE=`, deleted afterwards)

Two of the manifest's mutations were run unchanged-model, implementation-only, and
killed by a verifier diagnostic, not a test:

- **M1** (`>` to `>=` in the cap test): `postcondition not satisfied`,
  `src/ico.rs:272` (`directory_guard`'s `h`-equality) — 1 error (run 76).
- **M13** (classify a seven-byte prefix as PNG): `assertion failed`,
  `src/ico.rs:425` and `:442` (`has_png_signature`'s postcondition) — 2 errors
  (run 77).

Both are labelled audit-only in the sense that they are a proof-sensitivity
demonstration, not the M4b campaign; M4b reruns the full set.

## Assurance boundary

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|
unimplemented!|verifier::external|verifier::rlimit|requires\(false\)|unwrap\(\)|expect\(|panic!"`
over `ico_verus/src/ico.rs`, `src/model.rs`, `src/lib.rs` finds nothing. No
`#[verifier::rlimit]` or other solver-budget attribute; no `STUCK.md`. The only
trusted base is the verifier, `vstd`'s contracts and the compiler, as the manifest
allows. `spec-frozen/` and `../dib` are unmodified (`SPEC.md` and `VECTORS.json`
hashes unchanged).

Honest caveats:

- **`as` casts.** Verus does not check that a narrowing `as` cast preserves the
  value. Every one here is proved value-preserving first (the `u64 -> u32`
  narrowings of `dir_end` and the range end, from values `<= 2^26`), the
  `u32 -> usize`/`u64` widening casts, and the `u8 -> u16`/`u16 -> u32` in the
  `me` fields.
- **32-bit target.** No 32-bit verifier is installed. The width argument is given
  as `lemma_u32_width` plus the vstd axiom `b@.len() <= usize::MAX`; each specific
  bound (`dir_end <= 16,390`, `|b| <= 2^26`, `count_word <= 65,535`) fits the
  32-bit `usize`, and every narrowing target is checked. This is an argument and a
  proof obligation on the constants, not a second compiler run.
- **Allocation and stack.** Allocator exhaustion and OS stack failure are excluded
  by `Safe` itself; `Vec::with_capacity` is at most 1,024 records and
  `decode_entry` allocates nothing of its own.
- **T9/T10/T11** (ICO-side linear work, allocation-call bound, mutation campaign)
  are M4a/M4b and untouched.

## Surprises

1. **Enum and array `==` do not bridge to the spec `==`.** `x == y` on a derived
   `PartialEq` enum, or on a `[u32; 2]` payload, does not give the spec-level
   equality of the two values, so a postcondition `r == (x == y)` fails, and a
   `!=` branch gives no spec-level inequality. Comparing elementwise — `p[0] ==
   q[0] && p[1] == q[1]`, and a `match` on each enum — does. That is why the model
   equality is `status_eq`/`entry_eq` over payload elements and defect/kind
   *codes*, not a single `!=`. This cost the largest single block of debugging
   time (probes 26–40).
2. **`reveal` does not apply to a function called with arguments that are not
   `int`-free.** `assert(r == crate::model::status_eq(a, b))` fails inside the
   branch that constructed `r` from the same match; the match had to be nested
   (`match a { ... match b { ... } }`) so the model's shape and the code's shape
   are the same tree. A tuple match `match (a, b)` did not work (probes 30–33).
3. **`as int` in a comparison needs parentheses** (`(x as int) < y`); the parser
   reads `<` after a cast as a generic-argument list. The error message points
   one token past the offending expression.
4. **`rustc` catches exec-mode `int` leakage before Verus does**: `if (w0 as int)
   < 4` is a hard compile error, and a `r as int` in a `usize::MAX` comparison
   fails type checking. Spec-only integers must stay in spec/proof position.
5. **`use vstd::seq::{lemma_seq_*}` must be `#[cfg(verus_only)]`** or the plain
   `cargo build` (for `tests/conformance.rs`) fails to resolve the ghost lemmas.
   The type imports (`Entry`, `Error`) need `#[allow(unused_imports)]` instead,
   because they appear in `verus!` signatures that survive erasure.
6. A ghost `choose|hr| h(b) == Ok(hr) && hdr_matches(hr, header)` works as in TGA,
   and `crate::model::lemma_seq_new_snoc` (a `Seq::new`-snoc lemma) handles the
   loop's ghost-list growth; `lemma_seq_new_len`/`lemma_seq_new_index`/
   `lemma_seq_push_*` are the pieces it needs.

## Deliverables

- `ico_verus/src/ico.rs` — the implementation inside `verus!`, with contracts (1,198 lines).
- `ico_verus/src/model.rs` — `H`/`ME`/`MC` and the equality/decision model, independent (862 lines).
- `ico_verus/src/ico.rs.m1` — the preserved M1 plain implementation.
- `ico_verus/src/lib.rs` — adds `pub mod model;` and keeps the cross-crate skeleton.
