# Release gates

Items to raise with the owner before any release declaration.

1. Raise the shared alloc guard `MAX_BYTES` (and GIF `MAX_OUTPUT`, currently limited to 64 MiB by it). The owner intends to; it needs an alloc-delta erratum, re-verification of all cores, a re-run audit and new guard hashes. Owner decision 2026-11-16.
   The PNG corpus pass decides the target value and the PNG output cap sizing (owner decision 2026-11-16): the 64 MiB output cap and the 16,384 per-side cap are provisional, held out of the PNG freeze, and tested against very large multi-monitor screenshots and long thin images; `MAX_BYTES` is not raised until that pass says what to raise it to. At the proposed PNG caps, `Raw` for 16-bit RGBA and the RGBA16 output already exceed one 64 MiB reservation (`png/spec-draft/SPEC.md` P20).
