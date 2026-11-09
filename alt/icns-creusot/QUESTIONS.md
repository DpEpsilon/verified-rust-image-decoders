# QUESTIONS — spec readings I had to fix by choice (ICNS-CORE-SPEC 1.1)

None of these blocked M1; each has a chosen literal reading recorded below. The spec was not
edited. Listed for a second reader to confirm.

1. **R11 and an `it32` payload shorter than four bytes.**
   R11 removes the four-byte prefix "if its payload starts with four zero bytes". A payload of
   fewer than four bytes cannot start with four bytes at all, so I remove nothing there. The
   vectors cover the exact-four-zero case (`it32-four-zero-prefix-only`, `RleTruncated`) and the
   eight-zero case (`it32-eight-zero-prefix-then-literals`, only the first four removed), but not a
   one-, two- or three-byte payload. A one-byte payload is then a valid control byte, e.g. it32
   with payload `00` decodes as one literal run for channel R.

2. **R9 `nominal_side` versus `opaque_candidate`.**
   R1 says `nominal_side` is set "exactly when the type belongs to its fourteen-code table", and
   R9 confirms a listed type keeps its table side even when a signature matches. I read these as two
   independent fields: `nominal_side` depends only on the four type bytes; `opaque_candidate`
   depends on the type table and, for other types, on the signature. So an `ic04`/`ic05`/`icsb`
   entry with a matching signature is `opaque_candidate: true` with `nominal_side: null`, which the
   vectors (`png-signature-ic04` etc.) confirm.

3. **R14 absent-index order versus range.**
   R14 lists the checks as "MissingColor when its colour index is absent, then MissingMask when its
   mask index is absent. For both present, call decode_rgb, then decode_mask". I read "absent" as
   `None` only, so `combine(bytes, entries, Some(0), None)` returns `MissingMask` even when entry 0
   does not exist (the colour index is never range-checked before the mask absence check). The
   vectors agree for the cases they cover (`colour-without-mask` = `MissingMask`,
   `combine-colour-error-before-mask-error` = `IndexOutOfRange` with both present). The specific
   combination "present-but-out-of-range colour, absent mask" is not in the fixtures.

4. **R12 run count 2 is unrepresentable.**
   A literal control yields `control + 1` in 1..=128 and a repeat control yields `control - 125`
   in 3..=130, so no single run encodes exactly two samples. This is a property of the encoding,
   not a question, but it means adjacent runs of the same value are kept separate (the harness
   compares maximal runs, and none of the fixtures needs count 2).
