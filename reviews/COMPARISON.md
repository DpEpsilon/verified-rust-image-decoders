# Verus vs Creusot: ICNS core, effort comparison (2026-11-08)

Both projects prove the same manifest (T1-T13, L1-L6, 102 vector agreements, 40 witnesses, 10 mutations).
REVIEW-EQUAL-STRENGTH.md found all 19 statements EQUAL in strength and no deficiency in either, so the effort numbers compare like for like.
Sources: LEDGER.md of each project (harness token and tool counts), RUNLOG.tsv. Dollar spend is only visible on the OpenRouter dashboards.

| | Verus | Creusot | Creusot / Verus |
|---|---|---|---|
| DeepSeek Flash tokens (M1-M4) | 2,545,737 | 3,825,996 | 1.50x |
| Stronger-model tokens | Sol 112,284 | Sol 79,336 + Fable 174,157 + Sol (M4c, cut off by rate limit, not captured) | |
| DeepSeek tool calls (harness) | 1,488 | 1,990 | 1.34x |
| Stronger-model tool calls | 56 | 36 + 45 + 316 | |
| DeepSeek wall time | 4h49m | 9h31m | 1.97x |
| Handoffs | 1 (Sol, M3) | 3 (Sol and Fable on M3; Sol on M4c) | |
| Verifier runs logged | 208 (63 green) | 227 rows (49 green; some run numbers collide) | |
| Final proof | 315 verified, 0 errors; clean run 5m10s | Proved (7077 files); clean run about 11 min | |
| Mutations 1-10 | all fail | all fail | |

Caveats
- M3 run counts in both projects are unreliable (scratch crates outside verify.sh).
- Agents under-reported tool calls in both projects; the table uses harness counts. Both exceeded the 400-call cap once (Verus M4c 477, Creusot M3 518 and M4c 452).
- The first-pass user-reported OpenRouter spend was Creusot about 2.5x Verus after M1; the token ratio now is 1.5x. Dollar figures should come from the dashboards.
- Earlier running totals in the ledgers ("about 3.0M" for Verus) were rounded up; the sums above are the exact harness figures.

Qualitative differences
- Verus evaluated the concrete vectors with `by (compute)` and needed no per-index lemma chains. Creusot's recursive models are opaque to Why3 outside their modules, so every vector became a chain of single-step lemmas, which cost one handoff to Fable (M3) and one to Sol (M4c).
- Creusot proofs are faster to re-check on the small crate (clean 68s-1m35s through M4b) but the 102 inlined vectors grew the crate to 2.6 MB and 7,077 Why3 files (inode pressure on this volume).
- Creusot mutations 6-9 mutate the executable, Verus mutates the model; Verus gives the stronger sensitivity demonstration for RLE, it32 and MJ. Creusot m8 fails the executable helper contract, not the vector-named goal.
- Creusot's PROOF-REPORT-M4d.md overstates mutation 5 (claims the T13 site fails; it does not). Creusot's vc_map_push is flaky in 6 of 10 mutation copies.

Verdict: Verus is the winner on effort (1.5x fewer tokens, 1.3x fewer calls, 2x less time, one handoff instead of three) at equal proof strength. Per the standing instruction, the glycin loader is built on the Verus core.
