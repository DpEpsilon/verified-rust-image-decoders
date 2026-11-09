# M2b (Sol handoff): public decode is the verified function (saved by the coordinator from Sol's reply)

Result (coordinator reproduced from a clean copy): `verification results:: 151 verified, 0 errors`; `conformance: 214/214 vectors passed`; `corpus: 84 files checked, 58 accepted`; frozen hashes unchanged; no assume/admit/external_body/trusted/unsafe/todo/unimplemented/requires(false)/rlimit in src/.
Sol's runs: ./verify.sh 229-235 (7 runs; 234 and 235 green, 235 after cargo clean). Backups: src/dib.rs.m2a, src/model.rs.m2a (model.rs not changed). Harness: 134,364 tokens, 86 tool calls.

What changed: the sole exported `decode(bytes, mode)` is now inside `verus!`; it calls the MH-exact `parse_header` on its actual input, makes one bounded output allocation after header success, and runs verified raw/BITFIELDS, RLE, palette, alpha-scan and ICO AND-mask loops. The plain M1 decoder is removed from the public path.

Covered: T1 no-panic, checked narrowing, bounded allocation, no OOB memory access and arithmetic safety on all public paths and all three modes, no input precondition. T2: header prefix = parse_header = MH (exact record or exact first error). T3: `alloc_output` requests exactly 4N <= 67,108,864 bytes; the call-site ghost counter is 0 at start, stays 0 on header errors, 1 on the unique call. Loops have explicit decreasing variants (RLE: b.len() - pos).

NOT covered (explicit limits):
- `read_at` (src/dib.rs ~2033) is a guarded total reader: it returns 0 when an address is outside the slice. M2 does NOT prove that every computed raw/AND sample address lies within its checked row/mask, nor exact pixel semantics or errors versus MP. Exactness (T5-T10, T13) must show the guard is never taken on accepted inputs; do not treat the guard as a proof of correct addressing.
- Not claimed: the separate T11 termination theorem, a model-level termination proof, an actual 32-bit run (bounds fit 32-bit usize by argument only).
