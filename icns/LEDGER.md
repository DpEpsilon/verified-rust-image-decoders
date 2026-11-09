# Effort ledger

One row per finished agent. Source of numbers: the harness' completion notice (subagent_tokens,
tool_uses, duration). Token totals are not comparable across model families; DeepSeek dollar
spend comes from each project's own OpenRouter key dashboard (Verus key, Creusot key).

Escalation rule (identical in both projects): the implementer is deepseek-v4.1-flash-<tool>.
After 5 failed verifier runs on the same obligation, hand it to gpt-6-sol. After 5 more
failed runs, hand it to Fable. Log every handoff.

| # | phase | agent (model) | task | tokens | tool uses | duration | outcome |
|---|-------|---------------|------|--------|-----------|----------|---------|
| 1 | M1 | deepseek-v4.1-flash-verus (medium) | plain implementation + conformance harness | 64455 | 26 | 2m44s | 111/111 (verified by me); 0 verifier runs; no escapes; mutation test caught by 4 vectors |

User-reported OpenRouter spend after M1 (project key dashboards): Creusot about 2.5x Verus (exact dollars not recorded here). Harness token totals gave 1.7x (109,001 vs 64,455); tool calls were 79 vs 26.
| 2 | M2 | deepseek-v4.1-flash-verus (medium) | prove T1-T6 | 399304 | 246 | 35m14s | 47 verified, 0 errors (reproduced by me after cargo clean); 111/111 tests; 76 verifier runs (20 pass, 56 fail); no escapes; no STUCK; no handoffs |
| 3 | M3 | deepseek-v4.1-flash-verus (medium) | T7, T8, L1-L6 (attempt 1) | 578605 | 278 | 61m14s | INCOMPLETE: model.rs (1371 lines) + bridge lemmas proved (94 verified, 0 errors; M2 intact), T7/T8/L5/L6 not proved. Agent claimed it hit the 400 cap but the harness counts 278 calls. Only 2 verifier runs logged; most work ran in an unlogged scratch copy that no longer exists, so the run count is NOT comparable for this row |
| 3b | M3 cont. | deepseek-v4.1-flash-verus (medium) | T7 parse postcondition, T8 plumbing, L1-L6 (attempt 2) | 282690 | 115 | 37m42s | L1, L2 and T8 model side proved; T7 parse rewrite NOT done (runs 106-110 = 5 failures in a row). Harness-ended, not cap/STUCK; agent said ~60 calls, harness counts 115; agent said no obligation reached 5 failures, its own log shows 5. 32 logged runs. Handoff #1 to gpt-6-sol |
| 3c | M3 cont. | gpt-6-sol (high) | T7 live postcondition, T8 call sites, L4-L6 (handoff #1) | 112284 | 56 | 8m39s | 142 verified, 0 errors (reproduced after cargo clean); 111/111; no escapes; diff harness rerun 17.67M cases, 0 divergences; 10 verifier runs (112-121); finished |

Verus M3 totals: DeepSeek attempts 1+2: 860,000 tokens (578,605 + 282,690), 393 tool calls, 98m56s, no T7; Sol: 112,284 tokens, 56 calls, 8m39s, finished T7/T8/L4-L6.
| 4a | M4a | deepseek-v4.1-flash-verus (medium) | MR/MC/MA models, T9, T10, T11, L3 | 520141 | 245 | 42m52s | 193 verified, 0 errors (reproduced after cargo clean); 111/111; no escapes; 44 verifier runs (16 pass, 28 fail); finished; agent self-reported ~120 calls vs harness 245 (cap not exceeded) |
| 4b | M4b | deepseek-v4.1-flash-verus (medium) | MJ model, T12, T13 | 238887 | 101 | 15m59s | 212 verified, 0 errors (reproduced after cargo clean); 111/111; no escapes; T13 binds E to parse result, P derived from T7/T8; 10 verifier runs (4 pass, 6 fail); finished; self-reported 96 calls vs harness 101 |
| 4c | M4c | deepseek-v4.1-flash-verus (medium) | 102 vector agreements, 40 witnesses, mutations 1-10 | 461655 | 477 (harness; agent reported ~250; cap 400 exceeded) | 93m40s | 315 verified, 0 errors (reproduced after cargo clean, 5m10s); 111/111; no escapes; all 10 mutations fail the verifier (m1 6 err, m3 2, m5 2, m9 3, m10 1; m2/m4/m6/m7/m8 compile-abort), logs kept; 33 verifier runs (14 probes); agent self-report otherwise matched |
Verus M4 totals (4a+4b+4c): 1,220,683 tokens, 823 tool calls (harness). Verus cumulative DeepSeek tokens: about 3.0M after M4c.
