# PROOF-REPORT-M2 — theorems T1, T2, T5 (header path)

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), profile `release`,
toolchain 1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`.
**Final result line:** `verification results:: 93 verified, 0 errors`
(`runs/107.log`, a full reverify after `cargo clean`).
**Conformance:** `cargo test --test conformance -- --nocapture` →
`conformance: 169/169 vectors passed`, `test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 108 (runs 2–109 in `RUNLOG.tsv`). 27 exited 0; 81 were
intermediate failing builds during proof development. Of the 27 successes, 6 were on `tga_verus`
and 21 on the disposable `probe` crate; two of the six (runs 108, 109) were cached no-ops with no
`verification results` line, so the authoritative full verifications are runs 98, 99, 102 and 107
(107 is the post-`cargo clean` full reverify). No obligation hit the five-failure stop rule; no
`STUCK.md` was needed.
**Tool calls:** approximately 110 of the 400 allowed (my own count, from the transcript; the
harness count is authoritative). Roughly 40 were `./verify.sh` invocations, 30 read/edit passes on
the two source files, and the remainder probes, tests and inspection.

## Theorems proved

### T1 · safety — proved (header path as far as T2's invariants reach)

`decode` carries **no `requires`**, so Verus discharged the body for every `&[u8]` a 32- or 64-bit
target can represent (`|b| <= isize::MAX` is the language guarantee on the slice type). No panic,
out-of-bounds read or write, unchecked `usize` overflow or narrowing is reachable:

- R1 runs first: `if b.len() > MAX_INPUT { return Err(FileTooLarge) }` reads no byte, and the
  successful RGBA request `vec![0u8; 4*n]` occurs only after `parse_header` returns `Ok` — i.e.
  after the cap check. `n <= 16,777,216` (proved in `parse_header`) gives `4*n <= 67,108,864`.
- Every slice read is guarded by a length test performed *before* the read: the header reads are
  static (18 bytes, after `b.len() >= 18`), the ID and map reads test `id_end`/`map_end` against
  `bytes.len()`, and the pixel loops test `bytes.len() - *cursor < needed` before slicing.
- Every potentially overflowing `usize` sum is either proved in range first (`18 + id_length`,
  `map_start + map_bytes`, `*cursor + sb`, `*cursor + count*sb`, `by*w + bx`, `count * sb` all carry
  explicit `nonlinear_arith` bounds) or computed after subtracting, so no addition can wrap.
- The final statement in the postcondition is the resource bound: on success
  `d.rgba@.len() == 4 * (d.width * d.height) <= 67,108,864`.

No input-dependent recursion exists (all loops are `while` with `decreases`).

### T2 · header safety — proved

`parse_header` also has **no `requires`**; on `Ok` it establishes `header_inv(h)`, which contains
`1 <= width,height <= 8,192`, `N = width*height`, `4*N <= 67,108,864`, `1 <= sample_bytes <= 4`,
`1 <= map_stride <= 4`, the map-coherence facts and `map_first+map_length <= 65,536`. The bivariate
bound is proved by `model::lemma_mh_ok_bounds`, which derives `mh_bounds(m)` **from `mh` alone**
(no executable code): `1 <= w,h <= 8,192`, `1 <= N <= 16,777,216`, `4*N <= 67,108,864`,
`0 <= map_bytes <= 262,140`, `1 <= sample_bytes <= 4`. `mh_bounds` also carries
`18 + id_length + map_bytes <= 262,413`, which is the fact that the computed positions
(`id_end`, `map_end`) are checked against the input before use, in `decode`. The witness
`t2_header_bounds` states `hdr_matches(m,h) && model::mh_bounds(m)` for the model record `m`
that `MH` returns.

### T5 · exact header semantics — proved

`parse_header`'s postcondition is exactly the manifest claim, at the manifest's quantification:
`match crate::model::mh(b@) { Ok(m) => (returns Ok(h) && hdr_matches(m,h)) | (must return Err);
Err(e2) => (returns Err(e) && e == e2) | (must return Ok) }`. So `parse_header(b)` equals `MH(b)`
as the exact success record or the exact first error, with the R14 check order — the model and the
executable test the checks in the same sequence (`FileTooLarge`, `TruncatedHeader`,
`UnsupportedImageType`, `InvalidColorMapFlag`, `InvalidDimensions`, `UnsupportedInterleave`,
`UnsupportedPixelDepth`, `InvalidColorMap`, `ColorMapRange`, `InvalidAttributeBits`,
`OutputTooLarge`). `hdr_matches` reconstructs every R1 field plus the model's derived `N`,
`sample_bytes` and `map_bytes`, so it is record equality, not an abstraction.

## Theorems not proved

None of T1, T2, T5. T3, T4, T6–T11 and L1–L4 are out of M2's scope and untouched: no `MP`/`MR`
model exists yet, and `decode`'s postcondition covers only safety and the resource bound, not the
pixel result. No theorem is stated under a weaker-than-manifest form: the three witnesses
`t1_decode_safe`, `t2_header_bounds` and `t5_header_exact` carry the manifest's exact claims.

## The model (`src/model.rs`)

Written from the spec text, independently of `crate::tga`: `MH` (`mh`), the success predicate
`mh_bounds`, `le16`, and the R1/R3/R4/R5 classification predicates (`type_ok`, `is_indexed`,
`depth_ok`, `sample_bytes_of`, `map_entry_depth_ok`, `map_attr_ok`, `attr_ok`, `cm_bad`,
`cm_range_bad`, `tga_signature`). Nothing in `model.rs` calls executable code. The executable
links to the model by *proved postconditions* (`hdr_matches`), and the two bridge predicates
`header_inv`/`dec_inv` are proved from the concrete body, never assumed.

## Assurance boundary (no additional trust)

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external|requires\(false\)" tga_verus/src/`
finds nothing. The only trusted base is the verifier, its `vstd` contracts and the compiler's
execution of verified code, as the manifest allows.

Honest caveats:

- **`as` casts.** Verus does not check that a narrowing `as` cast is value-preserving. Every
  narrowing here is `u32 -> usize` or `u32 -> u64`, from values proved `<= 16,777,216`, and each
  carries a proof that the target type represents it (`n64 as usize`, `width as usize`). The
  `u32`-to-`usize` casts of header fields are from values in `0..65,536`. `parse_header`'s
  `(index - h.map_first)` subtraction is guarded by the range check `first <= index` before it
  runs, so it cannot underflow.
- **Allocation success** and process/stack exhaustion are excluded by `Safe` itself and are not
  proved.
- **The no-panic and no-overflow clauses of T1** are discharged by Verus verifying the body (there
  is no postcondition that can *state* "does not panic"); this is the same assurance shape the ICNS
  M2 report used. What is stated as a theorem is the resource bound and the success bounds.

## Surprises

1. `expr as int < y` does not parse — the `<` after a cast is read as the start of a generic
   argument list, and the error message ("expected `,`") points at the position *after* the
   offending expression. Every cast used in a comparison in this code is parenthesised
   (`(x as int) < y`). This cost the largest single block of debugging time.
2. `#[derive(Clone)]` on a non-`Copy` record (here `Decoded`) is only warned about, not rejected,
   matching ICNS.
3. `b[m..n] == sig_slice` and a `[u8; 18]` byte-string literal's view do not both work: the
   `*b"..."` literal has no usable view for the spec `Seq`, so the signature lives in `model.rs`
   and the executable compares against an explicit `[u8; 18]` array whose view is proved equal to
   it. `signature_tail` takes the *last 18* bytes (`base = b.len() - 18`, a single subtraction) so
   the bound `base + i <= b.len()` is provable, which the `start`-parameterised form was not
   without an extra arithmetic hint.
4. `mut`-reference postconditions use `final(c)`/`old(c)`; `*final(c) as int` in a comparison is
   parsed as a borrow and must be written with explicit parentheses.
5. `let`-bound `int`/`nat` values cannot be introduced in *exec* code; spec-level integers stay in
   `proof { }` blocks or in `spec fn`s (`spec_ox`/`spec_oy` for R9).
6. `Vec::index_mut`'s `set` is the supported in-place writer; `copy_from_slice` is assumed but
   needs `old(dst).len() == src.len()` and gave an awkward `final`-view postcondition, so R9's
   four-byte write is four `set` calls (`write_pixel`).
7. The `probe` crate (a `VERIFY_CRATE`-managed directory inside the project, deleted before the
   final run) was essential: ~21 of the 25 successful verifier runs were isolated probes of parse
   forms and arithmetic hints, which kept the full-crate runs (each 8–70 s) for real attempts.

## Proof-sensitivity check

Four of the manifest's header mutations were run in a disposable copy `sensitivity/`
(`VERIFY_CRATE=sensitivity`), each restored afterwards; logs are `runs/103.log`–`runs/106.log`.
All four fail the verifier on T5's postcondition (`postcondition not satisfied`,
`src/tga.rs:331` in the mutant = the `match crate::model::mh(b@)` postcondition), not merely a
runtime test:

- #2 swap the image-type and map-flag checks → 1 error;
- #3 remove the dimension cap → 1 error (plus two propagated `requires` failures);
- #1 `|b| > MAX_INPUT` → `|b| >= MAX_INPUT` → 1 error;
- drop the attribute-count check (`if false && !attr_ok`) → 1 error.

The five-run stop rule was never approached on a single obligation; the largest cluster of
same-obligation failures was the `signature` comparison loop (runs before 92), resolved by the
`base = b.len() - 18` rewrite, in fewer than five attempts.

## Deliverables

- `tga_verus/src/tga.rs` — the implementation inside `verus!`, with contracts (1,666 lines).
- `tga_verus/src/model.rs` — `MH` and its lemmas, written independently (525 lines).
- `tga_verus/src/tga.rs.m1` — the preserved M1 plain implementation.
- `tga_verus/src/lib.rs` — adds `pub mod model;`.
