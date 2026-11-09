# Sol review of the ICNS Verus core (saved by the coordinator from Sol's reply, 2026-11-08)

Verdict: no blocking defect in the implementation of frozen ICNS-CORE-SPEC 1.1 or in the stated on-domain Verus proofs. Sol reverified a copy: 315 verified, 0 errors. Independent Python oracle (written from the spec text) agreed with all 111 vectors. Release build and tests pass.

Findings
1. Blocking only if advertised as a complete gdk-pixbuf replacement: files with no R8 legacy pair (modern-only ic08 PNG, ic04/ic05 ARGB) yield no pixels from the core alone (decode_rgb gives UnsupportedType). Needs an outer PNG/JP2/ARGB fallback and selection policy. (The glycin loader already delegates PNG entries; JP2 and ARGB are not handled.)
2. Should-fix compatibility policy: one padding byte after complete RLE makes the core return RleTrailingBytes; gdk-pixbuf accepts it, and a macOS compatibility report mentions a NULL-byte workaround. Strict R12 decision, not a mismatch.
3. Should-fix evidence gap: all 111 fixtures are synthetic; strict R4/R5/R10/R11 (user-approved) reject inputs gdk-pixbuf accepts (suffix after declared length, 1-7 byte entry-header suffix, uncompressed 4N ARGB payload in is32). No measured real-producer corpus.
4. Nit: MC/MA/MJ do not return the manifest's OutsideDomain marker when not P(b,E); T10-T13 are unaffected.

Not checked: iconutil/gdk-pixbuf execution on the constructed cases, a real ICNS corpus, 32-bit build, Verus/vstd internals, rerun of the ten mutations.
