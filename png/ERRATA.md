# Errata against the frozen PNG document set

Append-only. This file is **not** covered by the PNG freeze: it records
discrepancies between the frozen texts (`THEOREMS.md`, `spec-frozen/`, `PLAN.md`)
and the implementation. Nothing here edits a frozen file. New entries are appended
at the end with their date and the artifact they concern. An entry marked PROPOSED
has not been applied: it takes effect only after the owner approves it, and the
entry is then amended in place of a new one by appending an "Owner decision" line.

## Index and status (as of 2026-11-17)

| Erratum | Status |
|---|---|
| E1 | PROPOSED, awaiting council/owner decision |

## E1 — 2026-11-17: `inflate_verus` "no `alloc` dependency" (A-PNG-6) — PROPOSED, awaiting council/owner decision (not applied)

**Frozen text.** `THEOREMS.md:161` (A-PNG-6): "`inflate_verus` has no `alloc`
dependency (build check, alloc-delta §9)". `PLAN.md:9` says the same of the
`inflate_verus` module list: "No allocation; no guard; no `alloc` dependency."

**Implementation.** The proof crate `png/inflate_verus` uses the same `vstd`
dependency declaration and feature set as `gif/gif_verus`, with the `alloc`
feature enabled, so that the pre-seeded verification artifact can be reused.
`vstd` is ghost and spec only: its model code is `cfg(verus_only)` and is erased,
so the shipped library contains no allocator references. Checked on the host and
`thumbv7m` release libraries, and on probes that instantiate both public entry
points (`inflate_verus/runs/0078.log`, `inflate_verus/runs/0060.log`).

**Proposed erratum.** Permit a ghost-only `vstd` dependency with the `alloc`
feature in the verification configuration, and require that production and
freestanding builds contain no `alloc::` paths and no `__rust_alloc`,
`__rust_dealloc` or `__rust_realloc` symbols, checked by instantiated-entry
probes.

**Effect on users.** None: the shipped code is unchanged and allocator-free.

**If rejected.** A `vstd` configuration without the `alloc` feature must be
built. Verus refuses that under `--no-cheating` in this sandbox (run 19:
`--is-vstd is not allowed with --no-cheating`; `inflate_verus/runs/0019.log`).

### E1 Status (2026-11-17)

PROPOSED. Awaiting council/owner decision. Not applied; the frozen text stands.
