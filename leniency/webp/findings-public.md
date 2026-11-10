## Significant observations (seed 731, 7,000 inputs; witnesses linked above)

1. Animated frames: on the unmutated anim-ll-3f seed, image-webp and glycin agree with libwebp-anim on frame count and on the first frame, but frame 2 differs by one level (0x1d vs 0x1e on the opaque background); the pair is `within-tol`, not equal.
2. glycin and a direct image-webp call (the same code) disagree on later-frame pixels for 592 mutated animation inputs (mainly ANMF flags, duration, offsets, fourcc, size). In witness 66 glycin returns a transparent pixel at (0,0) of frames 2 and 3 where the image-crate path returns an opaque pixel. Seeds are identical in both, so the divergence is in the glycin animation loop on perturbed ANMF headers.
3. glycin/image-webp accept and libwebp-anim rejects on 723 inputs, led by ANMF x/y offset mutations, VP8L signature and size fields: libwebp's demuxer validates frame geometry and chunk consistency that image-webp does not.
4. A wrong RIFF size (delta -9..+1000, or boundary values) is accepted by image-webp/glycin and libwebp-anim rejects it; libwebp-simple also rejects (606 inputs where both libwebp APIs reject and both Rust paths accept).
5. libwebp-anim alone accepts 200 inputs (ANMF width/height and VP8L fourcc mutations) that both Rust paths reject.
6. libwebp-simple alone accepts 64 inputs (VP8L/VP8/EXIF chunk size mutations) that every other column rejects; libwebp-simple and libwebp-anim, one library, disagree on acceptance for 194 inputs (the demux validation of AnimDecoder versus the incremental parser of WebPDecode).
7. Inserted, duplicated or padded extra chunks: the Rust paths and libwebp-simple accept (130 inputs) while libwebp-anim rejects (demux strictness).
8. VP8X flag bits and VP8X wrapping of simple files change what libwebp-simple/libwebp-anim accept against the Rust paths (101 inputs where only libwebp accepts, e.g. ALPH/EXIF fourcc mutations); a falsely set animation or alpha flag yields different pixels in 31 inputs.
9. Lossless and lossy still images (30 seeds, including lossy+ALPH, near-lossless, exact) decode bit-identically in all four columns; lossy needs no tolerance on unmutated seeds.
10. All-reject is 2,875 of 7,000 inputs and 1,249 are identical-pixel accepts; truncation, bit flips and magic changes are rejected by every column.
