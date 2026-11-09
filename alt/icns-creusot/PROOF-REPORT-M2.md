# PROOF-REPORT-M2 — safety theorems T1–T6

## Result

`./verify.sh` run 13 (log `runs/013.log`), exit 0, final line:

```
Proved (34 files) ✔
```

All 34 generated Coma files prove; there are no unproved goals and no escapes
(`grep` for `unsafe`, `assume`, `admit`, `trusted`, `external_body`, `todo!`, `unimplemented`
over `src/lib.rs` returns nothing). `cargo test` reports `conformance: 111/111 vectors passed`.
The public API is unchanged: `parse`, `decode_rgb`, `decode_mask`, `combine`, the `Entry` record
and the `Error` enum, with the same signatures, derives visible to callers
(`#[derive(Clone, Debug)]` on `Entry`, `#[derive(Clone, Copy, Debug)]` on `Error`) and the same
`Display`/`std::error::Error` impls.

## Theorems

| ID | Statement | Proved |
| --- | --- | --- |
| T1 | `∀b: Safe(parse, b)`; on `Ok(E)`, `|E| ≤ 4096` and each range is within `b` and in the eight-digit-string form. | Yes |
| T2 | `∀s,n (|s|≤67,108,864, n∈{256,1024,2304,16384}): Safe(rle, s, n)`; at most three `n`-byte channel buffers, never indexes past `s`, strictly decreasing loop variant. | Yes |
| T3 | `∀b,E,i, parse(b)=Ok(E): Safe(rgb, b, E, i)`; success gives `3d² ≤ 49,152` bytes and each channel `d² ≤ 16,384`. | Yes |
| T4 | `∀b,E,i, parse(b)=Ok(E): Safe(mask, b, E, i)`; success gives `d² ≤ 16,384` bytes. | Yes |
| T5 | `∀b,E,ci,mi, parse(b)=Ok(E) ∧ ci,mi∈Option<u32>: Safe(combine, b, E, ci, mi)`; success gives `4d² ≤ 65,536` bytes; internal calls use the same precondition. | Yes |
| T6 | `∀b,E: parse(b)=Ok(E) ⇒ 0 ≤ |E| ≤ 4096 ∧ ∀j< |E|: 8 ≤ start ≤ end ≤ |b| ≤ 67,108,864`, `type_hex` eight lowercase hex digits, classification matches the parsed bytes; proved from the parser and used at each decoder call site. | Yes |

## How each is obtained

**Reference model.** A Pearlite model independent of the executable code defines every helper
functionally: `magic_model`, `eq4`, `nominal_model`, `legacy_colour_model`, `legacy_mask_model`,
`is_legacy_m`, `starts_model`/`sig_model`, `opaque_cand_model`, `hex_model`, and the entry
predicate `entry_ok` (R1 range + `type_hex` provenance + R9 classification + R14 nominal side).
Each runtime helper carries an `#[ensures]` of exact (not range) equality with its model:
`nominal_side_of`, `legacy_colour_side`, `legacy_mask_side`, `is_legacy_code`, `starts_with_sig`,
`is_signature`, `type_hex_of`, `is_icns_magic`, `read4`, `read_be32`. Because this Creusot does not
reduce `match` on `Int` or `Seq == seq![..]`, the models use `if`-chains over byte-wise `eq4`.

**Success invariant (T1, T6).** `parser_ok(b, E)` = `|E| ≤ 4096 ∧ |b| ≤ 67,108,864 ∧ ∀j< |E|.
entry_ok(b, E[j])`, where `entry_ok` gives `8 ≤ start ≤ end ≤ |b|`, proves `type_hex` is the hex
model of the eight type bytes at `start−8..start−4`, and ties `opaque_candidate` and
`nominal_side` to the model of the type and payload. `parse`'s postcondition is
`match result { Ok(es) => parser_ok(bytes@, es@), Err(_) => true }`; it is established by a loop
invariant carrying `parser_ok` unchanged across iterations, so T6 is proved *from the parser*.
`parser_ok` is non-vacuous: a fresh goal `#[ensures(parser_ok(bytes@, es@))] fn f(bytes, es) {}`
fails to verify (checked during development), while `parse` discharges it.

**Termination (T1–T5).** Every loop in `parse`, `starts_with_sig`, `rle_channel`, `decode_rgb`
and `combine` carries a `#[variant]` (parser offset grows to `declared`; RLE `n − produced`;
per-pixel copies `n − i`); Creusot emits and proves each loop-variant goal even without
`#[check(terminates)]`, which I verified on a scratch crate with deliberately wrong variants
(they fail) and correct ones (they pass). No function recurses. `#[check(terminates)]` itself is
*not* used: it forbids calling `Vec::from_elem`/`String::from_iter`, which the core must allocate.

**Decoder bounds (T3–T5).** `decode_rgb` postcondition: on `Ok((w,h,out))`, `w == h`, `w ≤ 128`,
`out@.len() == 3*w*h`, `w*h ≤ 16,384`, `3*w*h ≤ 49,152`. `decode_mask`: `w*h == a@.len()`,
`w*h ≤ 16,384`. `combine`: `out@.len() == 4*w*h`, `4*w*h ≤ 65,536`. The temporary channels are
bounded by `rle_decode`'s postcondition `(chs@[0])@.len() == n@ ∧ … ∧ (chs@[2])@.len() == n@` with
`n = d² ≤ 16,384`. Buffer sizes are exact allocations (`vec![0u8; 3*n]`, `4*n`, `n`), so the
"requested buffer" bounds are the return-length equalities.

**Call-site preconditions (T3, T4, T5).** The decoders take `#[requires(parser_ok(bytes@, entries@))]`.
They are *not* proved for forged entries — that is outside the API contract. The relation is
discharged from a concrete successful parse by three wrappers that bind `E` to the actual result:

- `rgb_at_parse(bytes, i)` = `match parse(bytes) { Ok(es) => decode_rgb(bytes, &es, i), Err(e) => Err(e) }`
  with `#[ensures]` restating the `3d² ≤ 49,152` bound;
- `mask_at_parse(bytes, i)` likewise for `d² ≤ 16,384`;
- `combine_at_parse(bytes, ci, mi)` likewise for `4d² ≤ 65,536`.

Creusot verifies these three, and each verifies by *unfolding the real `parse` call*, not by an
axiom. As a proof-sensitivity check I weakened `parse`'s postcondition to `Ok(_) => true` in a
disposable copy: **only** `rgb_at_parse`, `mask_at_parse` and `combine_at_parse` then fail
(6/7, 6/7, 6/7), while `parse` and the decoders still verify. That is exactly the modular
call-site discipline T3–T5 require, and it shows the wrappers are not vacuous.

## Number of `./verify.sh` runs

**7 runs** were used this dispatch (runs 7–13; runs 1–6 are M1).
Run 7 failed to compile (ambiguous bare `Clone` derive); run 8 was an unproved run polluted by a
`verif/` cache carried over from the pre-port file; runs 9–13 all report `Proved (34 files) ✔`.
Per BRIEF rule 5 the five-failure stop rule did not trigger: no obligation failed five times — the
run-8 failures were a stale-cache artifact, cleared by removing `verif/`, after which the same
source verified.

## Verifier version (trusted computing base)

- `cargo-creusot 0.14.0`, Rust toolchain `nightly-2026-08-03`, `creusot-rustc` from the bundled
  toolchain.
- Why3 platform `1.8.2+git`; `why3find v1.3.0+dev`; provers Alt-Ergo `2.6.4`, Z3 `5.1.0`,
  CVC4 `1.8`, CVC5 `1.4.1`. `creusot-std 0.14.0` for the standard-library contracts.

## Surprises

- **Loop-variant goals are checked without `#[check(terminates)]`.** That attribute demands every
  callee be `terminates`, which excludes the allocating `Vec`/`String` constructors the core needs,
  so it is unusable; the `#[variant]` goals alone carry the termination obligation.
- **`str::as_bytes`, `String::chars`/`Clone`, and `format!` are unspecified in creusot-std 0.14.0.**
  `type_hex` provenance therefore had to be proved through `String::from_iter<char>` over a
  16-character hex table indexed by `n/16`, `n%16` (division axioms exist; the `shr`/`bw_and`
  builtins are uninterpreted, so `>>`/`&` cannot be used to prove nibble arithmetic).
- **`Seq == seq![..]` and `match` on `Int` do not reduce** in the prover, so the model is written
  with explicit byte-wise `if`-chains.
- **`Option<T>` has no `View`**, so `nominal_side` cannot be talked about as `x@`; the model
  compares `Option<u32>` values directly.
- **Visibility affects `#[logic]` transparency**: a `#[logic]` function cannot be inlined into a
  more-visible caller, which forced the model functions to stay `pub` (they erase away anyway).

## Unfinished

- Only T1–T6 were requested. T7–T13, the 40 named witnesses, the nine large-input lemmas L1–L6,
  and the ten mutations remain (M3/M4).
- The empty successful file is covered by `parse`'s universally quantified postcondition and by
  the executable vector `minimal-empty`; it is not separately witnessed inside the prover, because
  unfolding `parse` at a concrete call site to exhibit `Ok([])` needs the *exact-result*
  postcondition that is T7/T8 (M3) work.
