# lzw_proof: the T-LZW core (milestone M2)

The standalone, stack-free Verus LZW core now proves exact refinement to G10's
model for Strict and RecommendedBeta, including error prefixes, final bit
positions and E4. The public decoder uses caller-owned prefix, suffix and length
views totaling 20,480 bytes and writes backwards directly into `out`; it allocates
nothing and performs no per-frame table initialisation.

This crate holds no cap of its own: its inputs are borrowed views and an `a`-pixel
raster, so a cap change does not reach it. It is verified with the G1 caps of the
GIF core at `MAX_INPUT` 256 MiB (2^28) and `MAX_OUTPUT` 64 MiB (2^26; `W·H ≤
16,777,216` pixels). `MAX_OUTPUT` stays at 64 MiB because it must not exceed the
shared alloc guard's `MAX_BYTES` (67,108,864); the owner expects to raise the
canvas cap and `MAX_BYTES` together later, as a strict widening through an
alloc-delta erratum with re-verification of every core.

| file | responsibility |
|---|---|
| `src/model.rs` | complete-observation `ML`, G10 transitions and L-FULL |
| `src/chain.rs` | prefix chains, literal/stored lengths, roots and length bounds |
| `src/bits.rs` | LSB-first reader and bounded powers; u128 bit positions |
| `src/dictionary.rs` | readable-range invariant, cached lengths, insertion and historical bounds |
| `src/refinement.rs` | table agreement, prefix/suffix framing, fuel independence, checked roots and L-DISCARD |
| `src/emission.rs` | stack-free, raster-clipped backward emission and KwKwK joining |
| `src/decode.rs` | total `lzw_decode`, exact result contract and concrete-call `theorem_t_lzw` |
| `src/lzw.rs` | public policy/error types and decoder re-export |
| `src/{witnesses,regressions,policy_samples}.rs` | 12 original streams, 3 policy regressions and 96 oracle-backed complete observations |
| `tests/conformance.rs` | actual decoder against 200 random streams, 47 vector calls, 12 witnesses and 10,848 policy records |
| `tests/{emission,stress}.rs` | stale/long/clipped chains and generated >=8 MiB payloads at width 12/full tables |

From the repository root:

```sh
timeout 25m bash gif/lzw_proof/verify.sh "what I am proving"
```

The script records every verifier run in `RUNLOG.tsv`, uses two verifier threads,
and enables `--no-cheating`. Do not run verifiers concurrently. From this crate:

```sh
PATH=/opt/cargo/bin:/opt/verus:$PATH timeout 25m cargo test
timeout 25m python3 -I tools/check_model_policy_gap.py
python3 -I tools/emit_witness_rs.py
```

The policy comparison regenerates runtime records and concrete Verus samples;
rerun verification after regeneration. The stress test generates and oracle-checks
its inputs at runtime, sends them over a pipe and stores no large vector files.

`STUCK.md` starts with the resolved status and preserves earlier diagnoses as
history. `LEDGER.md` records the final run, count, tools and remaining integration
scope. Raw-byte accessors, allocator/parser integration, compositing and the full
T12 work-accounting proof are outside this standalone milestone.
