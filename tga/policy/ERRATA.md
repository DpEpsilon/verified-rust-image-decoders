# Errata against frozen POLICY-SPEC

This file records defects found in the frozen policy specification
(`POLICY-SPEC.md`, `THEOREMS-POLICY-DELTA.md`, `policy_oracle.py`,
`gen_policy_vectors.py`, `POLICY-VECTORS.json`, `QUESTIONS.md`; see `FREEZE.txt`).
It is a new file, not covered by `FREEZE.txt`, and it never edits a frozen file.
Each entry names the frozen text, the verified implementation behaviour, the
controlling text, and what a later revision should say.

## E1 — 8-bit greyscale with descriptor attribute count 8

### Frozen text

`POLICY-SPEC.md` line 24 (`attribute_bits`), the `ImageTable` sentence:

> allowed counts as R5 except 8-bit grey allows {0, 8} (8 is treated like 0) …

`THEOREMS-POLICY-DELTA.md` lines 9–12 add, to the policy header record, `eff`:

> the **effective attribute count** `eff` (equal to the descriptor count whenever
> R5 allows it; otherwise the `Lenient` value or `InvalidAttributeBits`) …

### Implemented behaviour

Three independent layers retain effective count 8 for an 8-bit greyscale image
whose descriptor attribute count is 8, rather than treating 8 like 0:

- the oracle (`policy_oracle.py` lines 59–61 and 153–162): `IMAGE_ATTR[('grey',
  8)] = (0, 8)` makes the count pass the `ImageTable` acceptance test, so `eff =
  count = 8`;
- the model (`policy_model.rs` lines 285–335 `image_attr_allowed`/`eff_attr`, and
  lines 1076–1090 `gov_depth`/`eff_count_img`): the accepted count is kept;
- the executable (`tga.rs` lines 5148–5165 `eff_attr_exec`): the accepted count
  is kept.

`MHModel` has no `eff` field: the effective count is derived from the record's
own fields (`image_type`, `pixel_depth`, `map_entry_depth`, `descriptor`) by
`eff_count_img`, so the delta's "adds to the 1.0 header record … `eff`" is not
how the model carries it. The header *record* is therefore the same 15 fields as
1.0; only the derived conversion projection `PktH` carries `attr = eff_count_img`.

### Effect on public output

None for the pixels. An 8-bit greyscale image is non-indexed with `kind = 2` and
`depth = 8`, so `sample_bytes = 1`; `conv_sample_policy` writes the byte to R, G
and B and alpha 255 unconditionally, reading no attribute byte. `stored_alpha`
is consulted only for 32-bit true-colour, 32-bit palette entries and 16-bit grey
(`policy_model.rs` `stored_alpha_policy`/`stored_alpha_img`), none of which is an
8-bit greyscale image. The effective count therefore reaches no alpha or pixel
decision on this element, and all four profiles decode the file to the same
RGBA8 bytes whichever of {0, 8} the count is given. The corpus counts
(Strict/Recommended/GdkCompat/ImageCrateCompat) are likewise unchanged, because
acceptance depends only on membership in the allowed set, which both readings
give.

### Which text controls

The listed layers are the proof obligation: `POLICY-SPEC.md` line 24's parenthetical
"(8 is treated like 0)" is superseded by the acceptance sentence that precedes it
("8-bit grey allows {0, 8}") and by the oracle, model and executable, which are
consistent with each other and are what the frozen `POLICY-VECTORS.json` was
generated from. The delta's `eff` description is a summary of intent, not a field
the record literally gained.

### What a revision should say

- `POLICY-SPEC.md`, `attribute_bits`/`ImageTable`: strike "(8 is treated like 0)".
  Write instead: "8-bit grey allows {0, 8}; an accepted count is kept as the
  effective count, as for every other element." If the intended behaviour really
  is to treat 8 like 0, the three implementation layers must change together and
  the frozen vectors regenerated — it is not a documentation-only fix.
- `THEOREMS-POLICY-DELTA.md` lines 9–12: say the effective count is a *derived*
  field of the conversion projection, not a new field of the header record:
  "`MH(b,P)` is the 1.0 header record unchanged; the conversion projection
  (`PktH`) carries `attr = eff`, the count the record's `attribute_bits` table
  accepts, which equals the descriptor count whenever that count is allowed."

No frozen file was modified to record this errata. No proof-visible behaviour
changes.
