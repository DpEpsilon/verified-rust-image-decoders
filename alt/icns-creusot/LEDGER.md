# Effort ledger

One row per finished agent. Source of numbers: the harness' completion notice (subagent_tokens,
tool_uses, duration). Token totals are not comparable across model families; DeepSeek dollar
spend comes from each project's own OpenRouter key dashboard (Verus key, Creusot key).

Escalation rule (identical in both projects): the implementer is deepseek-v4.1-flash-<tool>.
After 5 failed verifier runs on the same obligation, hand it to gpt-6-sol. After 5 more
failed runs, hand it to Fable. Log every handoff.

| # | phase | agent (model) | task | tokens | tool uses | duration | outcome |
|---|-------|---------------|------|--------|-----------|----------|---------|
| 1 | M1 | deepseek-v4.1-flash-creusot (medium) | plain implementation + conformance harness | 109001 | 79 | 9m23s | 111/111 (verified by me); 6 prover runs (5 translator failures, then translates; 11 unproved files = M2 work); no escapes |

User-reported OpenRouter spend after M1 (project key dashboards): Creusot about 2.5x Verus (exact dollars not recorded here). Harness token totals gave 1.7x (109,001 vs 64,455); tool calls were 79 vs 26.
| 2 | M2 | deepseek-v4.1-flash-creusot (medium) | prove T1-T6 | 747697 | 262 | 75m47s | Proved (34 files), reproduced by me from a clean copy with proof cache deleted (74s); 111/111 tests; 7 prover runs this milestone (13 total); no escapes; no STUCK; no handoffs |
| 3 | M3 | deepseek-v4.1-flash-creusot (medium) | T7, T8, L1-L6 | 789521 | 518 | 2h45m22s | T7, T8, L1, L2 proved (Proved 51 files, reproduced by me from clean copy, 65s); L4-L6 STUCK.md (true individually but exceed prover budget); 5 logged runs but scratch probe crates exist, so run count not comparable; exceeded the 400-call cap (harness counts 518); no escapes; 111/111. Handoff #1 to gpt-6-sol for L4-L6 |
| 3c | M3 cont. | gpt-6-sol (high) | L4-L6 (handoff #1) | 79336 | 36 | 7m07s | STUCK after 5 failed runs (20-24) on the entry_from_header classification goal; baseline restored (run 25, 51 files); no settings changed. Handoff #2 to Fable |
| 3d | M3 cont. | fable (claude agent) | L4-L6 (handoff #2) | 174157 | 45 | 19m23s | Proved (96 files), reproduced by me from clean copy (68s); 111/111; additive diff; default budget; no escapes; 4 runs (26-29); finished |

Creusot M3 totals: DeepSeek 789,521 tokens / 518 calls / 2h45m (T7, T8, L1, L2); Sol 79,336 / 36 / 7m (STUCK); Fable 174,157 / 45 / 19m (L4-L6).
| 4a | M4a | deepseek-v4.1-flash-creusot (medium) | MR/MC/MA models, T9, T10, T11, L3 | 811115 | 391 (harness; agent reported ~95) | 106m38s | Proved (152 files) reproduced from clean copy (1m35s); 111/111; no escapes; default prover budget unchanged; T10/T11 exact equality to MC/MA under P(b,E); 85 verifier runs; agent under-reported calls |
Creusot cumulative DeepSeek tokens: about 3.0M after M4a (plus Sol/Fable handoffs).
| 4b | M4b | deepseek-v4.1-flash-creusot (medium) | MJ model, T12, T13 | 263559 | 138 (harness; agent reported 62) | 26m55s | Proved (162 files) reproduced from clean copy; 111/111; no escapes; default budget; no contract lines removed from diff vs lib.rs.m4a; T13 quantifies over all E with p_model (P derived from parse's T7 postcondition; equivalent to binding E by functionality of MP, a different form than Verus; flag for equal-strength review); 14 verifier runs |
Creusot M4 so far (4a+4b): 1,074,674 tokens, 529 calls (harness). Cumulative DeepSeek about 3.3M.
| 4c | M4c | deepseek-v4.1-flash-creusot (medium) | 102 vector agreements, 40 witnesses, mutations | 863336 | 452 (harness; cap 400 exceeded) | 132m42s | INCOMPLETE: 54/102 vector obligations (all parse) proved; 11 decoding/combine goals failing in the full-102 run; mutations not started. Live crate green: Proved (472 files) from clean copy, 111/111, no escapes. RUNLOG run numbers collide (agent deleted a log). Handoff to Sol pending. |
Creusot cumulative DeepSeek tokens: about 4.2M.
| 4d | M4c cont. | gpt-6-sol (partial, rate-limited at 316 harness calls, tokens not captured) then deepseek-v4.1-flash-creusot (241767 tokens, 150 calls, 54m) | 102 vector agreements (Sol), 40 witnesses, mutations 1-10 (Sol driver m1-m9, DeepSeek m10 + report) | n/a + 241767 | 316 + 150 | ~8h wall incl. rate-limit | Proved (7077 files) reproduced from clean copy; 111/111; no escapes; base == live lib.rs; runs 216-225 all exit=1 with kept failure.log/result.txt; m8 fails the executable it32-strip contract (vc_strip4_prog), not the vector-named goal; verifier runs 214-225 |
Creusot M4 totals: DeepSeek 863336+263559+811115+241767 = 2,179,777 tokens in M4; cumulative DeepSeek about 4.4M plus Sol (handoffs #1 M3, #3 M4c) and Fable (#2 M3).
