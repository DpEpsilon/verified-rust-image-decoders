# PROOF-REPORT-M4a — the total model MP/MR; T6, T7, T8, T9, T11

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), release
profile, toolchain 1.99.1-x86_64-unknown-linux-gnu, vstd
`0.0.0-2026-11-01-0251`.
**Final result line:** `verification results:: 373 verified, 0 errors`
(`runs/254.log`, a full reverify after `cargo clean`, 79 s).
**Conformance:** `cargo test` → `conformance: 169/169 vectors passed`,
`test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 144, numbered 112–255 (24 exited 0, 120
were intermediate failing builds). No obligation hit the five-failure stop rule;
no `STUCK.md` was needed. Every run went through `./verify.sh`; no scratch crate
and no direct `cargo verus verify` was used.
**Frozen files:** `spec-frozen/SPEC.md`, `VECTORS.json`, `SOURCES.md`,
`DESIGN-NOTES.md` and `THEOREMS.md` have their `FREEZE.txt` hashes unchanged
(checked with `sha256sum`).
**Tool calls:** approximately 150 by my count from the transcript (the harness
count is authoritative), under the 400 cap.

## Theorems proved

### T6 · exact packet semantics — proved

The witness `t6_rle_exact` states the manifest's claim on the actual RLE helper
`decode_rle`: for every successful `MH` header/map prefix (`dec_inv`, proved by
`parse_header`, never assumed), every finite stream, and every valid starting
`(cursor0, produced0)` it establishes `rle_full_eq`:

- `rle_result_eq(bytes@, pkt_of(h), map@, c0g, *final(cursor), r)` — on success
  the final cursor is `MR`'s and the produced count is `N`; on failure the
  returned error is `MR`'s exact first error, i.e. control presence, then
  overrun, then whole-packet availability, then each sample's index in output
  order; and
- `rle_pixels_eq(...)` — on success every produced pixel is `MR`'s packet pixel
  written at R9's oriented cell (`rle_quad_ok`).

`MR` returns `Err((e, cursor', produced', pixels))`: the manifest's ghost error
state, documented as proof observations rather than public result fields. The
executable carries only the projection into the API `Error`, which is what
`rle_result_eq` compares. The model's produced pixels are checked by
`rle_pixels_eq`, so no clause of T6 is dropped.

The witness is quantified over the helper's whole domain (any `dec_inv` prefix,
any finite continuation, any valid start), not over vector-shaped inputs.

### T7 · total functional correctness — proved

`decode`'s postcondition is `decode_eq(r, bytes@)`, i.e.
`match r { Ok(d) => MP(bytes@) = Ok((d.width, d.height, d.rgba@)), Err(e) =>
MP(bytes@) = Err(e) }` for every `&[u8]` a target can represent. `decode` has no
`requires`, so this is the manifest's `forall b`. The proof dispatches on
`parse_header` (via `lemma_mp_mh_err`), then on each slice/suffix exit
(`lemma_mp_trunc_id`, `lemma_mp_trunc_map`, `lemma_mp_trailing_rle`/`_raw`,
`lemma_mp_rle_ok`, `lemma_mp_raw_ok_result`), and closes the success case with
`lemma_quad_ok_to_orient`, which turns the loop's `rle_quad_ok`/`wrote_samples`
into the model's exact `orient_bytes` byte sequence. The witness
`t7_decode_exact` restates `decode_eq` plus the dimension and length bounds.

### T8 · success and resource bounds — proved

`t7_decode_exact` carries `decode_eq` and, on `Ok`, `1 <= w,h <= 8192`,
`|p| = 4*w*h <= 67,108,864`; together with the model's `MP(b)=Ok(...)` shape this
is the manifest's "iff" in both directions (the model's `mh` bounds are proved by
M2's `lemma_mh_ok_bounds`, and `MP`'s acceptance is `decode_eq`'s right side).
The per-pixel clause is the witness `t8_output_cells`: for an accepted
`MP(b)=Ok(w,h,out)`, some `MH` record `m` built the result (`mp_ok_shape`), its
pixel sequence has exactly `N` entries, and for every output cell `q < N` the
four bytes `out[4q..4q+4]` are R8's non-premultiplied conversion of the source
sample `g(q)` at R9's position — with `f(g(q)) = q` from the orientation
bijection making that source and position unique. This is exactly
`mp_assemble`/`orient_bytes` unpacked by `lemma_orient_bytes_index`, so it is R8's
byte rule at R9's unique oriented position, not a one-way bound.

### T9 · precedence — proved

`t9_precedence` states the manifest's claim over `decode` itself:
`match decode(b) { Ok(d) => MP(b) = Ok((d.width,d.height,d.rgba@)), Err(e) =>
MP(b) = Err(e) }`. `decode` returns at most one error at its first failing check,
so "neither pixels nor a second error" holds by construction. The header order is
M2's `MH` order (``FileTooLarge``, `TruncatedHeader`, `UnsupportedImageType`,
`InvalidColorMapFlag`, `InvalidDimensions`, `UnsupportedInterleave`,
`UnsupportedPixelDepth`, `InvalidColorMap`, `ColorMapRange`,
`InvalidAttributeBits`, `OutputTooLarge`), proved equal to `MP`'s by
`lemma_mp_mh_err`; the RLE order (control, overrun, whole-packet bytes, then
indices) is fixed by `mr`/`mr_at`'s structure and transported by
`rle_result_eq`. The overrun-vs-truncation distinction is a distinct model branch
(`lemma_mr_cur_overrun` vs `lemma_mr_cur_rep_missing`/`_lit_missing`).

### T11 · integrated call path — proved

T11 is not a separate postcondition: it is the structure of the T7 proof.
`decode(b)`'s body is one concrete call path — `parse_header`, the ID/map slices,
the raw or RLE helper on the *same* `bytes` slice and the *same* `map` slice, the
suffix test, then the result. Every invariant used by the RLE helper, the palette,
the orientation and the footer is derived inside that call path:

- the header record `m` is obtained by `choose` from the actual `parse_header`
  result (`mh(bytes@) = Ok(m) && hdr_matches(m, h)`), and
  `lemma_pkt_of_hdr_matches` gives `pkt_of_mh(m) = pkt_of(&h)`, so the model's
  conversion record is the executable's with no forged header;
- `map@` is asserted equal to `bytes@.subrange(mp_id_end(m), mp_map_end(m))`, so
  the model and the helper use the same bytes;
- the RLE helper's `rle_full_eq` postcondition is unpacked by
  `lemma_rle_result_ok`/`lemma_rle_result_err` into the model's `mr_cur`/`mr_pix`,
  which the `MP` lemmas then consume.

No theorem is proved only for an impossible precondition, and no theorem is
stated under a weaker T-name.

## What is NOT proved (deferred to M4b, with the reason)

- **L1–L4** (the four large-vector lemmas) and their instantiations. They need
  `MP`'s orientation consequence and the packet-repetition algebra at the `MP`
  level; `MP` now exists, so they are M4b.
- The 54 named small witnesses, the model-vs-vector agreement obligations, and
  the mutations in `mutation/`. Not requested.
- T1–T5 and T10 remain proved (a subset of the 373 obligations). The M2/M3
  statements were not weakened; `src/*.m3` are the M3 files.

## The models (independently written)

`model.rs` grew from 2,085 to 5,870 lines and defines, without calling any
executable code:

- `MP`/`mp_after`/`mp` (R1–R13 order), `MP`'s ID/map ends `mp_id_end`/
  `mp_map_end`, R10's `mp_raw_acc`/`mp_raw_cur`/`acc_conv`, R11's `mr`/`mr_at`/
  `mr_cur`/`mr_pix`/`lit_pix`/`lit_good`/`lit_pref`, R12's `footer`/
  `mp_suffix_ok`, and R9/R8's `mp_pixels`/`orient_bytes`/`px_byte`/`wrote_px`/
  `wrote_samples`/`rle_quad_ok`;
- the conversion record `pkt_of_mh` built from `MHModel`'s own fields;
- the result relations `mp_ok_shape` and the executable-side `decode_eq`,
  `raw_result_eq`, `rle_result_eq`, `rle_pixels_eq`, `rle_full_eq`.

The executable carries these only as proved postconditions; `pkt_of` and
`hdr_matches` are the two bridge predicates, each derived from `parse_header`'s
result, never assumed.

## What changed in the executable

The only behavioural-looking change to `tga.rs` is that `decode_raw` and
`decode_rle` gained an executable `c0g: usize` parameter (the function-entry
cursor, passed as `cursor` at the sole call site in `decode` and as `*cursor` in
the witnesses) plus the `requires *cursor == c0g`. It exists because inside a
loop body `old()` denotes the iteration start, not the function entry, so the
success postcondition could not otherwise name the model's starting cursor. Every
other edit is a `proof { }` block, a postcondition or an invariant. The decode
result on all 169 vectors is unchanged (`cargo test` passes).

## Assurance boundary (no additional trust)

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external|requires\(false\)" tga_verus/src/` finds nothing.
The only solver-budget attribute is the single `#[verifier::rlimit(80)]` that M3
put on `lemma_packet_repetition` (now at `model.rs:4204`); no new budget
attribute was added. The only borrowed contract is `vstd`'s `Vec::set`, as in M2.

Honest caveats:

- M2's caveats about `as` casts and excluded allocation failure still apply.
- The T8 witness is stated over the model record `m` with `mp_ok_shape` as
  premise; the manifest's `MP(b)` is what `mp_ok_shape` is a projection of (any
  `Ok` of `MP` has such an `m`), and `t7_decode_exact` carries the `decode = MP`
  side at `MP`'s own quantification. I state this explicitly rather than claim
  the witness takes `MP(b) = Ok` syntactically.
- The model's output assembly is defined from the *decoded* pixel sequence
  (`mp_pixels`), not from the input bytes. For raw images the two are the same
  (`acc_conv`); for RLE the input bytes do not replay to the output without the
  packet structure, so defining assembly from `b` alone would be a model bug.

## Surprises and tool obstacles

1. **RLE output cannot be read off the input.** My first `mp_assemble` computed
   output byte `i` from `b[c0 + k*sb]` with `k = g(i/4)`, which is right for raw
   but wrong for RLE (a `0xff` packet repeats one sample and a literal packet is
   packed). I redefined assembly over `mp_pixels` (`mr_pix` for RLE).
2. **`old()` inside a loop body is iteration-relative.** The success
   postcondition needed the function-entry cursor; the fix was the `c0g`
   parameter. Verus rejected `#[verifier::proof]`-annotated and `ghost`-marked
   *executable* parameters ("cannot access spec-mode place in executable
   context"), so `c0g` is an ordinary `usize`.
3. **`lemma_wrote_px_write_step` needed a `vstd` frame-condition lemma of its
   own.** `Vec::set`'s postcondition ("everything outside the four bytes is
   unchanged") is a `forall` that Z3 would not instantiate at the four byte
   offsets; `lemma_outside_write` unpacks it at one index and `lemma_quad_unchanged`
   applies it four times.
4. **`mr_cur`/`mr_pix` paid for themselves.** Carrying the model's exact cursor
   recurrence (`mr_cur(c0g,0) = mr_cur(cur, produced)`) and pixel split
   (`mr_pix(c0g,0) = acc + mr_pix(cur, produced)`) as invariants, rather than the
   whole `mr` result, kept each loop iteration's obligations small; the outer
   loop's resource limit was only hit once, and the fix was the split, not a
   budget raise.
5. **`map_end_of` as a definitional bridge.** `mp_map_end(m) = 18 + id_length +
   map_bytes` and the executable's `map_end` reduce to the same `map_end_of(m,h)`
   once `hdr_matches` unfolds, so the equality is definitional rather than an
   arithmetic fact Z3 has to find.
6. **Verus emits two result lines.** `runs/254.log` prints a
   `2159 verified, 0 errors` line before the crate's `373 verified, 0 errors`;
   the report quotes the crate line.

## Deliverables

- `tga_verus/src/model.rs` — 5,870 lines: `MP`/`MR`, the projections, the output
  assembly and their lemmas.
- `tga_verus/src/tga.rs` — 4,173 lines: the M3 file with the `c0g` parameter,
  the `decode_eq`/`raw_result_eq`/`rle_full_eq` postconditions, the RLE loop
  invariants, and the `t6_rle_exact`/`t7_decode_exact`/`t8_output_cells`/
  `t9_precedence` witnesses.
- `tga_verus/src/tga.rs.m3`, `tga_verus/src/model.rs.m3` — the M3 files,
  preserved (as required).
- `PROOF-REPORT-M4a.md`, updated `NOTES.md`.
