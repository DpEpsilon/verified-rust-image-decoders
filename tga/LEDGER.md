# Effort ledger (TGA, Verus)

One row per finished agent. Source of numbers: the harness completion notice. Escalation: the
implementer is deepseek-v4.1-flash-verus. After 5 failed verifier runs on the same obligation, hand
it to gpt-6-sol; after 5 more, to Fable. Log every handoff. Tool-call counts are harness counts.

| # | phase | agent (model) | task | tokens | tool uses | duration | outcome |
|---|-------|---------------|------|--------|-----------|----------|---------|
| s1 | spec | gpt-6-sol | write TGA spec draft 1 (118 vectors) + manifest draft | 68771 | 31 | 8m05s | oracle 118/118 |
| s2 | spec | claude-sonnet-5-5 (reviewer) | review draft 1 | 135906 | 27 | 10m18s | 2 blocking, 10 should-fix, 6 nit |
| s3 | spec | gpt-6-sol | draft 2 (167 vectors) | 53776 | 59 | 8m27s | all findings addressed |
| s4 | spec | claude-sonnet-5-5 (reviewer) | recheck | 177560 + 183346 | 14 + 5 | ~7m | 0 blocking after N-1; ready |
| s5 | spec | gpt-6-sol | N-1..N-5 (169 vectors) | 73155 | 29 | 2m46s | oracle 169/169 |
| 1 | M1 | deepseek-v4.1-flash-verus (medium) | plain implementation + conformance harness | 77418 | 34 | 2m16s | 169/169 (verified by me); frozen hashes unchanged; no unsafe/unwrap/expect/panic; skeleton verifies; agent self-report matched (32 vs 34 calls) |
| 2 | M2 | deepseek-v4.1-flash-verus (medium) | T1, T2, T5 (header) | 470916 | 217 (harness; agent said ~110) | 36m48s | 93 verified, 0 errors (reproduced from clean copy); 169/169; no escapes; frozen hashes unchanged; 108 verifier runs (27 pass; 21 on a probe crate under VERIFY_CRATE); 4 sensitivity mutations fail on T5; agent under-reported calls |
| 3 | M3 | deepseek-v4.1-flash-verus (medium) | T3, T4, T10 + orientation/conversion/repetition lemmas | 375017 | 205 (harness; agent said ~60) | 43m35s | 189 verified, 0 errors (reproduced from clean copy); 169/169; no escapes except one #[verifier::rlimit(80)] on lemma_packet_repetition (model.rs:1558); frozen hashes unchanged. PROCESS VIOLATION: ~45 proof-development runs in an in-project probe/ crate with cargo verus verify called directly, not logged in RUNLOG (only 2 logged runs), so M3 run count is not comparable. Probe crate deleted. |
| 4a | M4a | deepseek-v4.1-flash-verus (medium) | MP/MR models, T6, T7, T8, T9, T11 | 616423 | 364 (harness; agent said ~160) | 76m24s | 373 verified, 0 errors (reproduced from clean copy); 169/169; no escapes; no new rlimit; frozen hashes unchanged; 144 logged verifier runs (24 pass), all via verify.sh; one executable-surface change (extra usize param c0g on decode_raw/decode_rle, ghost cursor start); t8 takes mp_ok_shape hypothesis |
| 4b | M4b | deepseek-v4.1-flash-verus (medium) | L1-L4, 165 vector agreements, 54 witnesses, 20 mutations | 907511 | 520 (harness; agent said ~250; cap 400 exceeded) | 82m41s | 578 verified, 0 errors (reproduced from clean copy); 169/169; no escapes, no new rlimit; frozen hashes unchanged; 20/20 mutations fail with kept logs; 40 logged runs (256-296), one unlogged (278, killed). |
TGA totals (DeepSeek): 77418+470916+375017+616423+907511 = 2,447,285 tokens; harness tool calls 34+217+205+364+520 = 1340; Sol 0 handoffs; Fable 0.
| 4c | M4c | deepseek-v4.1-flash-verus (medium) | close Sol review findings 1-8 | 861567 | 699 (harness; agent said ~550-600; cap 400 exceeded) | 138m53s | 636 verified, 0 errors (reproduced from clean copy); 169/169; frozen hashes unchanged; no escapes, no new rlimit; 15 exec-side mutations (only src/tga.rs differs from base2) all fail with verifier diagnostics; findings 3,4,6,7 closed, 2 closed, 5 documented, 8 not attempted, 1 substantially closed with residual (failure-path partial pixels are an exit lemma, not a clause of decode_rle's postcondition); 164 logged runs (297-460) |
TGA cumulative DeepSeek: 3,308,852 tokens, harness calls 2039.
| 4d | M4d | gpt-6-sol (high) | close T6 failure-path clause; repair mutants 5, 20, 7 (handoff #1 for TGA) | 114724 | 90 | 22m26s | T6 NOT closed (4 failed runs 462-465, source restored, no budget attribute); mutants 5/20/7 repaired and fail on verifier diagnostics; correction note PROOF-REPORT-M4d.md; clean run 471: 636 verified, 0 errors, 169/169 |
| 4e | M4e | fable (claude agent) | T6 failure-path clause, refresh + rerun 15 exec mutations (handoff #2 for TGA) | 282614 | 86 | 50m | 642 verified, 0 errors (reproduced from clean copy); 169/169; T6 closed at manifest quantification (no input cap, arbitrary entry state); 15/15 exec mutations fail on verifier diagnostics; no new budget attribute; runs 472-491 |
TGA totals: DeepSeek 3,308,852 tokens; Sol 114,724 (M4d) + review work; Fable 282,614; handoffs 2 (Sol, Fable).
| rev | review | gpt-6-sol (high) | final independent review of TGA after Fable restructuring | 113807 | 67 | 6m35s | T6/T7 closed; no decoder discrepancy; fuzz 7,583 cases zero mismatches; should-fix: MR total-domain, m15 wording; usize width note; report ~/projects/REVIEW-SOL-TGA-2.md |
