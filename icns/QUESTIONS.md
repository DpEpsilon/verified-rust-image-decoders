# Questions and ambiguities

Points where SPEC.md needed a reading. I continued with the most literal reading
in each case; no vector distinguishes these, so nothing here changes behaviour.

1. **R9, `ic04`/`ic05`/`icsb`.** The general rule already says every type other
   than an R8 code is opaque exactly when a full PNG or JP2 signature matches, and
   that such a match carries `nominal_side = None`. The dedicated sentence for
   these three codes therefore adds no behaviour, yet its "their ARGB variant is
   outside scope" hints they were expected to be opaque by type instead. I read
   the general rule as controlling, so `ic04`/`ic05`/`icsb` are opaque only on a
   signature match. If the intent was opaque-by-type, R9's general rule would need
   to exclude them, and the `argb-not-opaque-ic04/ic05/icsb` vectors say otherwise.

2. **R11, the four-zero prefix when the payload is shorter than four bytes.** "if
   its payload starts with four zero bytes" — I read this as requiring at least
   four bytes; a 0..3-byte payload cannot start with four zero bytes, so it is
   consumed whole as RLE. `it32-four-zero-prefix-only` (exactly four zero bytes)
   confirms the removal leaves an empty RLE stream, which is `RleTruncated`.

3. **R9, a listed type whose payload is shorter than its signature.** The spec says
   a listed type "with a nonmatching or short payload remains opaque". I do not
   inspect the payload of a listed type at all, so this holds. Noted only because
   the same sentence also allows a listed type to carry a matching signature; the
   table side wins there, which the code does too.

4. **R6/R15, the 4,097th entry.** "after validating that entry's header and length
   but before adding it to the result" leaves the relative order of
   `TooManyEntries` against `EntryOutOfBounds`/`InvalidEntryLength` implicit; R15
   fixes it as TruncatedEntryHeader, InvalidEntryLength, EntryOutOfBounds, then
   TooManyEntries, which the `invalid-4097th-entry-before-limit` and
   `entry-count-followed-by-overflowing-4097th` vectors confirm. I follow R15.
