# Effort ledger

One row per finished agent. Source of numbers: the harness' completion notice (subagent_tokens,
tool_uses, duration). Token totals are not comparable across model families; DeepSeek dollar
spend comes from each project's own OpenRouter key dashboard (Verus key, Creusot key).

Escalation rule (identical in both projects): the implementer is deepseek-v4.1-flash-<tool>.
After 5 failed verifier runs on the same obligation, hand it to gpt-6-sol. After 5 more
failed runs, hand it to Fable. Log every handoff.

| # | phase | agent (model) | task | tokens | tool uses | duration | outcome |
|---|-------|---------------|------|--------|-----------|----------|---------|
| 1 | spec | gpt-6-sol (high) | write SPEC.md, VECTORS.json (68), SOURCES.md | 45258 | 27 | 5m43s | 16 requirements; 68 vectors pass independent Python check |
| 2 | spec | claude-sonnet-5-5 (reviewer) | independent review of SPEC.md/VECTORS.json vs io-icns.c | 88610 | 14 | 4m31s | 3 blocking, 10 should-fix, 7 nit; 68/68 vectors correct |
| 3 | spec | gpt-6-sol (high) | revise spec per REVIEW.md; freeze 1.0 (107 vectors) | 74704* | 43* | 6m29s | all 20 findings adopted; hashes verified by me |

* Resumed agents report totals that appear cumulative (tool uses 27 -> 43); this run alone is probably ~29k tokens / 16 tool uses. Treat as unconfirmed.
| 4 | spec | claude-sonnet-5-5 (reviewer) | recheck revision, REVIEW2.md, oracle over 107 vectors | 119182* | 5 | 1m47s | 0 blocking, 1 should-fix, 4 nit; spec implementable as is |
| 5 | spec | gpt-6-sol (high) | apply 5 review items, refreeze 1.1 (111 vectors) | 92063* | 33* | 3m55s | hashes verified by me; checker 111/111 |
| 6 | manifest | gpt-6-sol (high) | write THEOREMS.md, then amend it (L1-L6, witness fixes) | ~35.7k* | 30* | ~7m | cumulative total 127,764 minus 92,063 at row 5; verified by me: 102/9 vector split, lemma coverage, 40 witnesses |
