# Release gates

Items to raise with the owner before any release declaration.

1. Raise the shared alloc guard `MAX_BYTES` (and GIF `MAX_OUTPUT`, currently limited to 64 MiB by it). The owner intends to; it needs an alloc-delta erratum, re-verification of all cores, a re-run audit and new guard hashes. Owner decision 2026-11-16.
