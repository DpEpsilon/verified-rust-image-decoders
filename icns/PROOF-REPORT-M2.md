# PROOF-REPORT-M2 — theorems T1 to T6

**Verifier:** Verus 0.2026.11.02.8bf68ee, profile release, toolchain 1.99.1-x86_64-unknown-linux-gnu.
**Final result line:** `verification results:: 47 verified, 0 errors`
(`/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/runs/076.log`).
**Conformance:** `cargo test` → `conformance: 111/111 vectors passed`,
`test result: ok. 1 passed; 0 failed`. Executable behaviour is unchanged from M1.
**`./verify.sh` runs this milestone:** 76 (every run in `RUNLOG.tsv` is M2; 20 exited 0,
56 were intermediate failing builds during proof development, none hit the five-run stop rule on
a single obligation).

## Theorems proved

All six of T1–T6 are fully proved, at the exact quantification.

- **T1 (proved).** `parse` carries no `requires` and postcondition `parsed_inv(bytes@, E@)` on
  `Ok`. Every length (`0..67,108,864` and the larger representable ones, which fail the first
  branch without reading bytes) is safe: Verus discharged every slice index, `u64` sum and
  subtraction, so no panic, out-of-bounds access or overflow is reachable. Termination:
  `decreases declared - offset`, and `offset` advances by `length >= 8`. Allocation: at most
  `MAX_ENTRIES = 4096` records, each with an eight-digit `type_hex`; `parse` allocates no pixel
  buffer.
- **T2 (proved).** `rle_decode` requests exactly three `n`-byte channel buffers. Every payload
  read is guarded (`cursor == payload.len()`, `payload.len() - cursor < needed`); the run copy is
  a separately verified helper `rle_copy` with `produced + count <= channel.len()`; the outer
  loop has `decreases 3 - ci` and the inner `decreases n - produced` with `produced` increasing
  by `count >= 1`. The `n` values required by the theorem (256, 1024, 2304, 16384) all satisfy
  the stated `n <= 16384`; `|s|` is unconstrained, which is stronger than the theorem needs.
- **T3 (proved).** `decode_rgb` requires `parsed_inv(bytes@, entries@)` and, on success,
  `width == height <= 128` and `data.len() == width*width*3 <= 49,152`. The `side <= 128` fact is
  a proved lemma over the R8 table, which also bounds `n = d² <= 16,384`.
- **T4 (proved).** `decode_mask` gives `width == height <= 128` and
  `data.len() == width*width <= 16,384`. The mask-length check is the exact `payload.len() == n`.
- **T5 (proved).** `combine` gives `width == height <= 128` and
  `data.len() == width*width*4 <= 65,536`. Both internal calls discharge
  `parsed_inv(bytes@, entries@)` from the same precondition under which `combine` is verified.
- **T6 (proved).** `parsed_inv` is the success invariant of T6: `|b| <= 67,108,864`,
  `|E| <= 4096`, and `∀j < |E|: 8 <= E[j].payload[0] <= E[j].payload[1] <= |b|`, `type_hex` is
  eight lowercase hex digits equal to `hex4(type_code)`, and `opaque_candidate`/`nominal_side`
  equal `classify` of the actual parsed bytes. `parse` proves it directly. The decoder
  preconditions are then *discharged at concrete call sites* by `parse_success_invariant`,
  `decode_rgb_after_parse`, `decode_mask_after_parse` and `combine_after_parse`, each of which
  runs a real `parse(bytes)` and passes that result to the decoder (`icns.rs`, end of file). The
  successful empty file (`E = []`) satisfies the invariant.

## Theorems not proved

None of T1–T6. T7–T13 are out of scope for M2 and are untouched (no `MP`/`MR`/`MC`/`MA`/`MJ`
model exists yet); they are the M3/M4 dispatches.

## Assurance boundary (no additional trust)

`grep -E "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external" src/*.rs` finds nothing in either `icns.rs` or `model.rs`. The only
trusted base is the verifier, its `vstd` contracts (the skeleton's support crate, version
`0.0.0-2026-11-01-0251`) and the compiler's execution of verified code, as the manifest allows.

Honest caveats:

- **`as` casts.** Verus does not check narrowing `as` casts. Every `u32` narrowing of a payload
  offset in `parse` is now backed by an explicit proof that the value is `< 2^32` and unchanged
  by the cast (`assert(pstart as u32 as int == pstart)` and likewise for `end`). The remaining
  `usize` casts come from values bounded by 67,108,864, which every target `usize` represents.
- **Allocation success** is excluded by `Safe`, exactly as the manifest states; this is not
  proved and cannot be.

## Surprises

1. Exec functions cannot be called from spec/proof code, so the model could not be a spec copy
   that is invoked on `parse`. The executable instead carries a postcondition in model terms.
2. `format!` has no Verus postcondition, so `type_hex` is built by `push`, and the invariant is
   phrased over `hex4`; `Entry::type_code` is proved to invert it (`code_of(hex4(c)) == c`).
3. Three std calls are unsupported inside `verus!` and were replaced keeping behaviour identical
   (`from_be_bytes`, `String::as_bytes` via `as_str`, `slice::to_vec`). Array `for` loops,
   byte-string `match` patterns on `[u8; 4]`, and array `==` in specs are also unsupported and
   were rewritten (index loops, explicit index comparisons).
4. `is_ascii_spec_bytes` is a broadcast lemma, not callable; the `str`→`bytes` bridge needed
   `broadcast use` plus a proved lemma that an eight-digit hex string is ASCII.
5. The `verus-src/` clone (the Verus source for reference) was used only for reading `vstd`
   specs and the guide; it was deleted again after the work and is not part of the deliverable.
