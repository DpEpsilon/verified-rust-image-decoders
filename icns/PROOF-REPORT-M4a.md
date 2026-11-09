# PROOF-REPORT-M4a — MR, MC, MA; T9, T10, T11; L3

**Stopping reason:** Finished the requested obligations. Verus 0.2026.11.02.8bf68ee, release
profile, toolchain 1.99.1-x86_64-unknown-linux-gnu.
**Final result line:** `verification results:: 193 verified, 0 errors`
(`/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/runs/164.log`, a full reverify
after `cargo clean`).
**Conformance:** `cargo test` → `conformance: 111/111 vectors passed`,
`test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 44, numbered 122–165 (16 exited 0, 28 were intermediate
failing builds; run 165 was a cached confirmation that made no source change). No single obligation hit the five-failure stop rule; no `STUCK.md` was needed.

## Theorems proved

- **T9 (proved).** `rle_decode`'s postcondition is
  `rle_result_eq(r, mr(payload@, n as int))`: the executable result is the exact three `n`-byte
  channels or the exact first error, for every payload with `|payload| ≤ usize::MAX` and every
  `n ≤ 16384` (the required `n ∈ {256,1024,2304,16384}` all satisfy that). `mr` is the R12 model
  written in `model.rs` independently of the executable: the control count is `control+1` below
  128 and `control-125` otherwise, the overrun test (`count > n - produced`) precedes the
  payload-availability test, and the trailing test is `cursor == |s|`. The wrapper `t9_rle_exact`
  states it as a theorem with no precondition on the payload.
- **T10 (proved).** `decode_rgb`'s postcondition is
  `rgb_decoded_eq(r, bytes@, entries@, color_index)`, i.e.
  `decoded_eq(r, MC(bytes, entries, color_index))`: width, height and every RGB byte, plus the
  exact error. `MC` does the index test, then the four legacy colour types, then R11's four-zero
  `it32` prefix removal, then `MR`, then the R3-byte interleave. The wrapper `t10_rgb_exact_for_p`
  states it over `P(b,E)` (a stronger premise than needed: the postcondition holds for every `E`
  with `parsed_inv`, which `P` implies through `lemma_p_exec_inv`).
- **T11 (proved).** `decode_mask`'s postcondition is
  `mask_decoded_eq(r, bytes@, entries@, mask_index)`, i.e. `decoded_eq(r, MA(...))`: the exact
  index → type → length error precedence and every alpha byte. `t11_mask_exact_for_p` states it
  over `P(b,E)`.
- **L3 (proved).** `lemma_l3_large_mask(a)` for every `|a| = 16,384`: with
  `b = icns · BE32(16,400) · "t8mk" · BE32(16,392) · a`, `mp(b) = Ok([l3_entry()])` where
  `l3_entry = ("74386d6b", [16, 16,400], false, None)`, and `MA(b, [l3_entry], 0) = Ok(128,128,a)`.
  The instantiation `lemma_l3_mask_t8mk` fixes `a` to 16,384 `7f` bytes.

T1–T8 and L1, L2, L4, L5, L6 remain proved (they are a subset of the 193 obligations). The M2
and M3 statements were not weakened; the only change to those executables is at the RLE/decoder
layer described below.

## The reference models (independently written)

`model.rs` now defines, in Verus's logic and without calling any executable code:

- `rle_count`, `rle_needed`, `rle_literal`, `rle_bytes` — R12's packet arithmetic and bytes;
- `rle_chan_from` — one channel loop, `produced`/`cursor` threaded, overrun-before-availability;
- `rle_rest(s,n,ci,prefix,cur)` — the remaining channel work with a per-channel prefix, and
  `rle_run = rle_rest(s,n,0,[],0)`;
- `mr(s,n)` — `rle_run` plus the trailing-cursor test, returning the three channels or the error;
- `it32_prefix`, `strip_it32` — R11; `interleave_rgb` — R10's R3-byte interleave;
- `mc`, `ma` — the decoders on executable `Entry` records, and `mc_ms`, `ma_ms` on model entries;
- `decoded_eq`, `rle_result_eq`, `rgb_decoded_eq`, `mask_decoded_eq` — the exact result relations.

The executables carry these as postconditions; the model is a separate recursive definition, not
a copy of the executable's loops. The `mr` model's error precedence is fixed by its structure
(control-absent, then overrun, then data-absent per packet; `RleTrailingBytes` only after all
three channels).

## What the executables now carry

- `rle_copy` — postcondition `final(channel)@ == write_seq(old@, produced, rle_bytes(...))`, so
  the copied span is exactly the model's packet bytes.
- `rle_decode` — postcondition `rle_result_eq(r, mr(payload@, n))`. Its outer loop carries
  `acc == chans_prefix_view(channels, ci)` and the ghost predicate `rle_pref`; the inner loop
  carries `rle_pref` at the current `produced`. Executable statements are unchanged: the same
  `payload[cursor]` reads, the same three error returns in the same order, the same `rle_copy`
  calls. Only ghost state (`acc`, `cur0`, `ch_before`) and proof blocks were added.
- `decode_rgb`, `decode_mask` — the model result relations added alongside their M2 bounds.
  Executable statements unchanged apart from proof blocks; the `it32` prefix test, the `legacy_side`
  match and the payload-length check are byte-for-byte the M2 code.

## Surprises and tool obstacles

1. The first draft of the model carried a `starts : Seq<int>` of control offsets and a
   "freshness" invariant to prove trailing-data detection. It was unnecessary: `mr`'s own
   `cursor == |s|` test covers it, and the freshness bookkeeping only added instantiation
   burden. Removing it took the RLE model from ~14 errors to 0.
2. `Seq::new` takes a `nat`, and `Seq::subrange`/`a.push(x)` take `int`/`int`; mixing `int` and
   `nat` model indices produced a steady stream of `E0308`s. The fix was to carry model offsets as
   `int` throughout and cast at the `Seq::new` boundary (`ci as nat` in `chans_prefix_view`).
3. `assert(a == b)` between a `Seq<u8>` and an `[u8; 4]` does not type: the model's `code4` is a
   four-element array while `b.subrange(...)` is a sequence. The L3 proof compares the two
   through `entry_at` instead.
4. Proving `scan(l3_string(a), 16400, 8, [])` needs the subrange equalities for the magic, the
   declared length, the entry type, the entry length and the payload, then the `8 + 16392 = 16400`
   arithmetic to let `scan`'s base case fire; the arithmetic has to be written explicitly.
5. `lemma_interleave_rgb_exact` needed `j / 3 < R.len()` from `j < 3 * R.len()`; Z3 needs the
   `lemma_fundamental_div_mod` hint, and the `0 <= k` half of the bound follows from `j / 3 <= j`
   in the model's arithmetic but was easiest to leave to the `nonlinear_arith` block on the upper
   bound alone.

## Assurance boundary

`grep -nE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external" src/icns.rs src/model.rs` finds nothing. The trusted base is
unchanged: the verifier, `vstd` contracts and the compiler. No theorem statement was weakened.
T12, T13, the 102 model-vector comparisons, the 40 witness reports, the nine large-vector lemma
instantiations beyond L3 and the ten mutations are M4b, not this dispatch; this report does not
claim them.
