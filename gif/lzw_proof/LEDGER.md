# M2 (T-LZW) ledger

Crate `gif/lzw_proof`, standalone (to be moved into `gif_verus` at integration).
**Final run 227: 184 verified, 0 errors**, with default solver limits, two verifier
threads and `--no-cheating`. The concrete stack-free decoder, its exact
complete-observation refinement and `theorem_t_lzw` are proved. `cargo test` passes
8 tests with none ignored, including two generated >=8 MiB compressed streams.

| runs | what | verified |
|---|---|---|
| 1 | `/tmp` scratch smoke; `verify.sh` path bug | - |
| 2 | `no_std` skeleton | 2 |
| 3-5 | `model.rs` compile fixes (int shifts, nat indices, `Vd` needing a `Done` arm) | - |
| 6 | **first hard obstacle**: the chain walk as a spec recursion — `decreases c` refused for `pf[c] as int`; 9 errors | 8 / 9 |
| 7-13 | model rewritten with a `Seq<Seq<u8>>` table; `ML` complete | 8 |
| 14 | probe: `decreases c` with the link fact as a hypothesis is refused outright | 1 / 2 |
| 15-29 | `chain.rs` (fuel walk). Runs 15-28 were the fuel arithmetic: budget `>= c - clear + 2`, then `fuel > c`, then `chain_ok`'s domain clause. Green at run 29 | **21** |
| 30-50 | exec surface: `Params` record, `strict`/`recommended_beta`, the `err_of` map, `params_ok`/`m_ok` bridge, `result_eq` | 24 |
| 51-60 | model v3: the **same** prefix-chain representation as the executable, so T-LZW is a fold identity rather than a table bridge | 24 |
| 61-96 | the witness gate: `tools/gen_witnesses.py` builds 12 streams and runs `gif_oracle.py` on them, plus a Python mirror of the model for the bit positions; `tools/emit_witness_rs.py` emits one `by (compute)` obligation each | **37** |
| 97 | model + chain + witnesses; decode-loop stub `#[ignore]`d | 39 |
| 98-150 | the chain theory the push loop needs: `peel`, `peek`, `pushbytes`, `peel_f`, `peel_stored`, `lemma_pushbytes_cons`, `lemma_pushbytes_peek`, `lemma_peek_splice`, `lemma_rev_splice`, `lemma_rev_is_reverse`, `lemma_str_shape`, `lemma_len_bound`, `lemma_peel_bound`, `lemma_peel_zero/one/step_pub/add`, `lemma_link_not_reserved`, `seg_ok`/`stack_ok`, and `emit_string`'s contract | 54 |
| 151-190 | previous stage: push-loop experiments, final checked-in emission stub | **56** |
| 191 | previous stage's cached build: no verifier summary, not a new proof count | - |
| 192 | Sol: prove `lemma_peel_not_reserved`, replace the reflexive length-bound claim with real bounds, prove the model/policy counterexample | **59** |
| 193 | concrete emission attempt: missing slice-setter trait | compile error |
| 194 | emission proof: one remaining arithmetic-overflow obligation | 60 / 1 |
| 195 | full push/pop proof, root byte bound, output arithmetic bound | **61** |
| 196 | direct slice assignment; complete helper and independent runtime tests | **61** |
| 197 | initial handoff before coordinator-approved model correction | **61** |
| 198 | correct StopAtRaster and checked-literal precedence; full `(idx,error,pos,e4)` observation; stronger witnesses | **63** |
| 199-200 | stack-free emission, explicit take-index facts and clipped output | **65** |
| 201-204 | exact LSB-first reader with u128 positions; table agreement, prefix/suffix framing, fuel independence and cache shape | **74** |
| 205-206 | dictionary insertion and length-cache preservation | **79** |
| 207-212 | complete decode loop; guards, emitted prefixes and readable-range agreement; split concrete-slice triggers unlock final proof | **82** |
| 213 | public decoder replaces stub; 96 oracle-backed observations; all runtime conformance runs executable | **174** |
| 214-223 | width, size-1, checked roots and L-DISCARD; modular history lemmas; remaining decoder query hits default resource limit | 180 / 1 at run 223 |
| 224 | bounded 20-unit diagnostic proves strengthened decoder | **181** |
| 225 | move resolved-code emission to its own verified function; default solver limit restored | **182** |
| 226 | concrete-call T-LZW theorem, connected L-FULL/size-1 facts; full stress suite passes | **184** |
| 227 | final source audit, default solver limits, two threads, `--no-cheating`; executable conformance rechecked | **184** |

The resumed stage's peak summed RSS was 1,183 MiB; the final verifier run took
11 seconds and peaked at 908 MiB. The final source contains no rlimit attribute,
assume/admit/external_body/unsafe or new axioms. Run 224's temporary 20-unit
resource limit was diagnostic and was removed before run 225.

## Proved result and scope

`src/decode.rs::lzw_decode` is total over caller arguments. When policy bounds
satisfy `1 <= m_lo <= m_hi <= 11`, the three table views have 4,096 entries each,
and `out.len() >= a`, its result equals `ML` exactly. `theorem_t_lzw` calls the
actual decoder and states exact equality with no BufferTooSmall disjunct. The
proof covers Strict, RecommendedBeta and every Boolean Params switch combination
on that domain, including invalid minimum code sizes as exact model errors.

`MRes` now retains the emitted indices, optional first error, final bit position
and E4. The executable returns `Decoded { written, error, pos, e4 }`; `result_eq`
compares `out[..written]`, the exact error, position and flag on every outcome.
The decoder uses u128 positions so `8*|D|` fits for both 32-bit and 64-bit Rust
slice domains. No per-frame table initialisation or scratch stack remains.

The corrected model accepts `m=2,D=[140,11],A=2` as `[1,1]`, bit 9, E4 under
RecommendedBeta; Strict retains the same emitted prefix and reports TooManyPixels
at bit 9. A custom Strict-tail/checked-literal regression confirms step-4 error
precedence. The formerly missing success positions and error-prefix indices are
now part of all witness observations rather than omitted from the theorem.

The main loop proves readable-range array agreement, cached-length recurrence,
width scheduling, the size-1 one-pixel branch, checked roots, full-table behaviour
and termination by `8*|D|-pos`. The stack-free chain walk writes each index at its
forward raster position, clips writes at A, and decreases its remaining length.
Historical dictionary bounds imply a crossing string discards at most A-1
indices (L-DISCARD).

Raw-byte table accessors, allocator/parser integration, compositing and the full
T12 step-accounting proof remain outside this standalone milestone; their proofs
are not claimed here. No standalone decode equality obligation remains open.

## Tools and runtime validation

- Verus 0.2026.11.02.8bf68ee, release, linux_x86_64.
- vstd 0.0.0-2026-11-01-0251.
- rustc 1.99.1 (58cf8f9c3 2026-10-13).
- `cargo test`: conformance 4 passed, executable emission 3 passed, stress 1 passed; none ignored.
- Conformance runs the executable on 200 random streams, 47 vector calls, 12 witnesses and 10,848 policy records.
- `check_model_policy_gap.py` passes 57,120 Python transcription/oracle comparisons across policy switches.
- The Verus gate includes 96 generated oracle-backed observations in addition to 12 original streams and 3 regressions.
- Stress payloads contain 11,487,200 and 11,911,619 compressed bytes and decode 8,388,608 indices each.

The stress encoder and independent Strict oracle validate both generated streams
before the Rust decoder sees them. Both reach width 12 and a full table, with and
without a Clear at the full table. Stress data travels over a pipe and is not
stored in vector files. The policy comparison preserves oracle error prefixes,
positions and E4 in its runtime records.

All verifier runs used `timeout 25m` and `verify.sh`, one at a time, with no
background work and no commits. Only files under `gif/lzw_proof/` were edited.
