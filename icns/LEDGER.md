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
| nostd-1 | no_std | claude (sonnet 5.5) | convert icns_verus to no_std + alloc; certification checks | - | - | - | 315 verified, 0 errors (unchanged from run 208; runs/001.log). Imports and Cargo feature only (alloc imports in icns.rs and model.rs; model.rs import is verifier-only). conformance green. Lib builds on x86_64-unknown-none, thumbv7m-none-eabi, i686-unknown-linux-gnu; rlib -Zls shows core/alloc/compiler_builtins only. See NO_STD.md. Backups *.nostd-1. |

## alloc-delta ICNS step (2026-11-12, deepseek-v4.1-flash)

The `try_*` ICNS crate: `./verify.sh` **406 verified, 0 errors** (`runs/041.log`),
up from the 315 of the pre-delta `no_std` state; no pre-existing obligation was
removed (the six "missing" top-level names are the renames `parse` ->
`parse_tracked`, `rle_decode` -> `rle_decode_tracked`, `decode_rgb`/`decode_mask`/
`combine` -> `*_tracked`, `hex4` -> `hex4_bytes`). `cargo test --offline`:
`tests/conformance.rs` 111/111 vectors, `tests/fault_injection.rs` 8/8. The
source lint `tools/audit_alloc.sh --self-test` exits 0. Verifier runs 14-42
(28 runs, 1 green at 41 after a first green at 41; runs 001-013 are the previous
agent's). Backups `src/*.alloc-1` and `alloc-wip-1/alloc_guard.green-1/`.

## proof-contract gaps 4/5/7/8 (2026-11-12, deepseek-v4.1-flash)

The four gaps from Sol's review of the ICNS alloc-delta step are closed: (4) the
`OutOfMemory`-names-`Req` postcondition on `try_decode_mask` and `try_combine`,
matching `try_decode_rgb`; (5) `ensures` tying `Error::name` and `TryError::name`
to their `name_spec` (no `when_used_as_spec`); (7) named `PreAlloc` corollaries
`lemma_g_{rgb,mask,combine}_prealloc` on the RGB, mask and combine logged entry
points plus public bridges; (8) G clause 3 tightened to the per-site bound
(16,384 / 49,152 / 65,536, `MAX_ROOM` for `EntryList`) instead of the global
67,108,864.

`./verify.sh` **413 verified, 0 errors** (`runs/044.log`), up from 406; no
pre-existing obligation removed. `cargo test --offline`: `tests/conformance.rs`
111/111 vectors, `tests/fault_injection.rs` 12/12. Mutation sensitivity: two
disposable `VERIFY_CRATE` copies (`alloc-wip-1/mut-name-{1,2}`, deleted) each fail
with exactly **1** error when one `name` spelling is mutated (`runs/047.log`,
`runs/048.log`; `412 verified, 1 errors` each). Verifier runs 43-48 (runs 046 is a
no-op re-run on the main crate). `tools/audit_alloc.sh` is clean except the
`sites.rs` hash pin, which `tools/` (owned by another agent) must update:
`c48050e7…` -> `adcc09be…`; `mod.rs` unchanged at `018ed17a…`. Backups
`src/*.alloc-f44`, `alloc-wip-1/alloc_guard.green-f44/`, `alloc-wip-1/*.f44.rs`.

> Superseded by ERRATA E6 (2026-11-13): `mod.rs` is now `26e4c832...`.

E6 (2026-11-13, ERRATA E6, `try_zeroed` fills with `resize(n, 0)`, trusted item A-STD-4): run 51, 412 verified, 0 errors (previous total 413 (run 50)). Agent, tokens and tool counts are not recorded here.
