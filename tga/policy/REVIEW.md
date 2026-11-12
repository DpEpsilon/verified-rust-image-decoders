# TGA policy review (Sol, condensed by coordinator)

Verdict: not ready to freeze for implementation. None of the findings requires changing the frozen 1.0 decoder.

- High: universal claims lack universal proof obligations. POLICY-SPEC.md:5,9-14 requires safety and the closed `recovered` meaning for every retained policy value. THEOREMS-POLICY-DELTA.md:21-24,34-36 states the map-address bound (T2) and conversion bounds (T4) but leaves them out of the universally quantified list, and restricts flag exactness (E) to four named profiles. Quantify T2, T4, E over all retained records or narrow the spec claim. The implementation must prove the constant in steps <= c*(|b|+N) independently of the oracle.
- Medium: the pixel-difference count is 85 exact pairs across 41 files (20 gdk-file, 20 gdk-forced-whole, 23 image, 22 tinytga), not 83 (POLICY-SPEC.md:64, QUESTIONS.md:15). investigate_pixdiff.py shows each listed field is necessary within the chosen record; it does not prove sufficiency or uniqueness, and the tinytga post-transforms are not independently validated.
- Medium: agreement is not independent validation except the seed-4242 run (1465 distinct inputs, 1465/1465 agreement for Strict/verified, Gdk/forced-GDK, Image/image-rs; same mutation generator). Corpus and tuned-seed figures are regressions. validate.py:85 compares rejection only, not error names. glycin/image-rs and the two GDK entry points are not independent votes.
- Low: spec-frozen/FREEZE.txt lists THEOREMS.md and check_vectors.py by paths that live elsewhere (tga/, tga/spec-history/spec/); hashes match, content unchanged.
- State the DIB-style allocation invariant for TGA explicitly (no input-dependent scratch allocation beyond the one output allocation) and add an obligation, or drop the claim.

Owner decisions: accept Q1 (four Recommended error-site relaxations) and Q4 (whole-packet ClipPacket); accept Q2 (effective alpha count 8 for invalid 32-bit attributes) as a documented guess, not proven intent; keep Q3's two-event `recovered`; keep Q5 compat profiles opt-in and deferrable; Q6 is an integration decision, Recommended is a reasonable default (14 extra corpus acceptances, not a population-wide benefit).

Q9 (bit-15 palette alpha): do not change shipped Strict (1.0 R8 and frozen vectors promise bit-15 alpha for 16-bit palette entries with attribute count one). Do not silently make Recommended opaque either: obligation R promises identical RGBA for every Strict-accepted image. Use a separately versioned revision or an explicit opt-in profile; if a Recommended-only change is mandatory, revoke and re-prove conservativity and rename the profile.

Freeze gate: explicit Q1-Q6 decisions; repair T2/T4/E quantification and allocation intent; correct 83 to 85 and qualify the independence and causality language.
