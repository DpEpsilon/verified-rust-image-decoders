# Design notes — review before freeze

- Decode to top-first RGBA8 rather than retain the source origin: one output coordinate convention lets clients compare pixels without a second flip.
- Honor indexed colour-map `first` and reject an absent palette index: the baseline indexes a zero-based map and returns transparent black, which hides malformed references.
- Support 15-bit true-colour and 16-bit indexed samples beyond the loader's accepted depths; both decode with a fixed little-endian rule and no inferred channels.
- Treat descriptor attribute count as authoritative even in pre-2.0 files: a declared alpha channel survives, an undeclared byte or high bit does not, and no output is premultiplied. This rejects unconventional descriptors and may differ from legacy producer intent.
- Require map metadata to be zero on non-indexed images and reject unsupported interleaving: no ambiguous payload cursor and no silently ignored header fields.
- Accept a 2.0 footer signature only after complete pixels; ignore footer offsets and all preceding suffix bytes, but reject an unmarked suffix. This is a compatibility/strictness compromise, not validation of extension structure.
- Cap input and output separately at 64 MiB and either dimension at 8,192; the compressed 8,192×2,048 image reaches the output cap without a huge input. These are policy constants, not TGA limits.
- Error ordering follows byte consumption after fixed header checks; in RLE a packet's bounds win over its first invalid palette index. This makes precedence provable without speculative reads.
- No file is frozen and no digest has been assigned. An independent reviewer should decide whether supporting 15-bit true-colour and 16-bit indexed samples, and ignoring footer offsets, matches the intended product compatibility policy.
