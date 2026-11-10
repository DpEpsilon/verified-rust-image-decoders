## Significant observations (seed 731, 7,000 inputs; witnesses linked above)

1. jxl-oxide alone accepts 308 inputs that glycin (jxl-rs) and libjxl reject; led by ftyp box mutations (TBox, brand, minor version): it does not require the declared ftyp brand.
2. jxl-oxide and libjxl accept 233 inputs that jxl-rs (glycin) rejects: truncations (69), appended trailing bytes (60) and xml box length changes. Cause not investigated.
3. libjxl alone accepts 62 inputs (brob LBox, jxlp index, trailing bytes): it tolerates container-level inconsistencies that both Rust decoders reject.
4. glycin and libjxl accept 32 inputs that jxl-oxide rejects (box insert 20, duplicate 9, zero-length box 2).
5. glycin and jxl-oxide accept 3 inputs that libjxl rejects, all codestream header byte mutations (bytes 6 and 7).
6. Lossy VarDCT agreement: all 17 lossy seeds decode within 1 level of each other across three independent implementations (mean difference about 0.2 for jxl-rs vs the others, 0.005 for jxl-oxide vs libjxl); no tolerance above 1 was needed on unmutated seeds.
7. Modular lossless seeds (gray/RGB/RGBA, 8/16-bit, palette, squeeze, linear/P3/PQ encodings) and animated seeds are bit-identical in all three, including frame counts; orientation 6 is applied by all three (output size swaps), as glycin's loader leaves rotation to the decoder.
8. Pixel disagreements are rare: 16 inputs, all from bit flips or byte replacement in the codestream, with maxima 57 to 255; the same corrupted stream decodes differently, e.g. 7 inputs where glycin differs from both others.
9. Container wrapping of a bare codestream (jxlc, jxlp with correct indices, 2 parts) is accepted by all three and gives identical pixels; incorrect jxlp last-flag, index gap, reversed order produce split outcomes recorded in the partition table.
10. 5,155 of 7,000 inputs are rejected by all three; 576 identical; 615 within tolerance (mostly trailing-bytes and wrap variants of lossy seeds).
