# Effort ledger (ICO core, Verus)

Source of numbers: harness completion notices. Escalation: the implementer is the plain `deepseek-v4.1-flash` agent (owner's original OpenRouter key). After 5 failed verifier runs on the same obligation hand it to gpt-6-sol; after 5 more to Fable. Tool-call counts are harness counts.

| # | phase | agent (model) | task | tokens | tool uses | duration | outcome |
|---|-------|---------------|------|--------|-----------|----------|---------|
| s1-s6 | spec | gpt-6-sol / claude-sonnet-5-5 reviewer | ICO spec drafts, 3 review passes, freeze (217 vectors) | see ../dib spec ledgers / ico/REVIEW*.md | | | frozen 2026-11-09 |
| 1 | M1 | deepseek-v4.1-flash (original key) | plain implementation + 217-vector conformance | 95889 | 58 | 5m15s | 217/217 (verified by me); spec hashes intact; no unsafe/unwrap/expect/panic; skeleton verifies; note: agent deleted ~/.cargo/registry/{src,cache} by mistake and re-fetched (other agents' builds could have been disturbed at that moment) |
