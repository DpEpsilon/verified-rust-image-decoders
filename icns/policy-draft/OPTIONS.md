# ICNS policy options (draft, not frozen)

Conclusion first: the incumbents deviate from frozen ICNS-CORE-SPEC 1.1 (`../spec-frozen/SPEC.md`, R1–R18) at **6 decode-level sites** (container and legacy pixels) and at **4 selection/delegation sites** that lie outside the core's four functions, **10 fields** in all (DIB 22, TGA 13). On the 15,701-file corpus the 6 decode-level fields recover **no** file: every Strict container failure is a truncated or non-ICNS file that both libraries also reject, and no legacy colour entry exercises a lenient RLE rule. What separates Strict (267 decodable files) from the libraries (`icns` crate 11,736, gdk-pixbuf 454) is entry selection and PNG/JPEG 2000 delegation: 15,404 corpus files carry PNG elements and no legacy colour+mask pair. A Recommended profile can therefore equal Strict at decode level; the owner decisions that matter are in QUESTIONS Q1–Q3.

Values in **bold** are Strict (shipped 1.1). Every value is a deterministic function of the bytes and safe with guarded reads and checked arithmetic, unless the row says *excluded*.

## Scope of the core, from the frozen spec

The core parses the container (R2–R7), classifies opaque candidates (R9: fourteen listed types by code, other non-legacy types by a full PNG or JP2 signature), and decodes **only** the four legacy RLE24 colour types `is32`/`il32`/`ih32`/`it32` and the four 8-bit masks `s8mk`/`l8mk`/`h8mk`/`t8mk` (R8, R10–R14). It never decodes PNG or JPEG 2000 (R9: "never decodes PNG/JPEG 2000"), never decodes the ARGB `ic04`/`ic05`/`icsb` variant (R9: "outside scope"), does not decode the old 1/4/8-bit resource types (`ICN#`, `icl8`, …, R8), and never selects an entry (R7, R14). The bench and the leniency harness select "the largest legacy colour+mask pair" in their own glue (`bench/src/main.rs:67–79`), not in the core.

## Incumbents and evidence

- **gdk-pixbuf 2.42.12** `io-icns.c` at tag `2.42.12` (copy `/home/ws/projects/spec/gdk-pixbuf-2.42.12/io-icns.c`; the master checkout in `/home/ws/projects/spec/gdk-pixbuf` has the same `io-icns.c`). Installed `libpixbufloader-icns.so`, `GdkPixbufLoader` with the type forced, whole-buffer and one-byte writes (identical on every probe).
- **`icns` 0.3.1** (`~/.cargo/registry/src/*/icns-0.3.1/src/{family.rs,element.rs,icontype.rs}`), default `pngio` feature. Runner: read the family, take the largest of `available_icons()`, decode it (the leniency harness's rule).
- **`image` 0.25.10 has no ICNS decoder**: no `icns` string in its `src/` or `Cargo.toml`, and `ImageFormat` (`src/io/format.rs`) has no ICNS variant. Confirmed by search; not a column.
- **Strict**: shipped `icns_verus` (`try_parse`, `try_combine`) behind the largest-legacy-pair selector.

Hand-made probes: `/home/ws/projects/leniency/policy-probe/probe_cases.py` (`icns_cases`), runners in the same directory, results `results-icns.json`; "probe `x`" names a case. Corpus: `~/corpora/icns/files` (15,701 files, read in place), header-level survey `survey_icns.py` (regenerates `/tmp/claude-0/survey/icns-survey.json`); library acceptance counts from the bench equality gate (`bench/results/icns-corpus.json`, summarised in `bench/README.md:250`: `icns` crate ok 11,736, gdk-pixbuf ok 454, verified ok 267; of files both decode, identical 43 and 101 respectively).

Corpus survey facts used below: Strict container verdict ok on 15,694 files; the 7 failures are `TruncatedFile` 4, `BadMagic` 2, `TrailingFileBytes` 1, and none of the 7 has a legacy pair or a parseable PNG entry inside its declared length. 267 files have a complete legacy pair with an exact-length mask; their 765 colour entries all decode under Strict's RLE rules. 15,404 files have PNG elements and no legacy pair; 65 carry JP2 elements; 115 carry ARGB `ic04`/`ic05`/`icsb` elements; 98 carry old 1/8-bit types. No file has more than 4,096 entries.

## A. Decode-level fields (container and legacy pixels)

| Field | Values and ordered delta from 1.1 | Incumbent behaviour, witness; safety | Proposed Recommended |
|---|---|---|---|
| `file_suffix` | **`Reject`**: R4 `TrailingFileBytes` when declared length `L < |b|`. `Ignore`: parse `[0,L)` and never read bytes at or after `L`. `CrateStream`: as Ignore, but an entry whose header starts before `L` is read in full even when it extends past `L` (bounded by `|b|`); entries are read while `offset < L`. | gdk scans to `icnslen` only (`io-icns.c:91–96`), the crate reads elements while `file_position < file_length` (`family.rs:140`). Probes `suffix-1-byte`, `suffix-8-bytes`, `suffix-entry`, `declared-short-at-entry-boundary`: both decode, Strict rejects. Probe `declared-short-cuts-mask` (L cuts the mask entry): crate decodes (`CrateStream`), gdk rejects (`io-icns.c:102–104`). R4 is a user-approved settled decision. Corpus: 1 file, unrecoverable (its entries inside `L` are malformed). | `Reject` (settled R4, no corpus gain); QUESTIONS Q4. |
| `entry_tail` | **`Reject`**: R5 `TruncatedEntryHeader` for 1–7 bytes left before `L`. `Ignore`: stop scanning there. | gdk stops when fewer than 8 bytes remain (`io-icns.c:96`): probe `tail-3-bytes-inside-declared` decodes. The crate tries to read a header and fails (`failed to fill whole buffer`). Corpus: 2 files, both also `TruncatedFile`, unrecoverable. | `Reject`. |
| `entry_cap` | **`Max4096`**: R6 `TooManyEntries` on the 4,097th entry. `Unlimited`: no cap; the entry list is then bounded only by `(L−8)/8` records. | Neither library caps (probe `entries-4097-junk`: both decode the pair after 4,097 junk entries). `Unlimited` raises the parse allocation bound from 4,096 records to about 8.4 million for a 64 MiB input, which obligation A must price. Corpus: no file exceeds 4,096 entries. | `Max4096`. |
| `raw_argb` | **`Reject`**: R10, a colour payload is always RLE. `RawFirst`: if the payload length is exactly `4N`, read it as uncompressed ARGB (A ignored; R,G,B from bytes 1–3 of each pixel) instead of RLE. `RawOnRleError`: decode RLE; only if R12 returns an error and the payload length is `4N`, read it as ARGB. | gdk `io-icns.c:291–297` (`RawFirst`): probe `argb-raw-4N` decodes in gdk, Strict `RleOverrun`, crate rejects. A `4N`-byte payload **can** be valid RLE (2-byte literal packets cost 2 bytes per pixel, up to `6N`), so `RawFirst` changes some Strict-accepted images and breaks obligation R; `RawOnRleError` is the error-site-only variant. Corpus: 1 file has `4N` colour payloads for all four sides, but no masks, so gdk also rejects it. Safe: `4N ≤ 65,536`. | `Reject`. |
| `zero_prefix` | **`It32Only`**: R11, strip four leading zero bytes only for `it32`. `AllTypes`: strip them for every RLE24 colour type. `AllTypesOnError`: strip for a non-`it32` type only if R12 fails without stripping. | The crate strips for all types (`element.rs:303–307`): probe `is32-zero-prefix` decodes in the crate, Strict `RleOverrun`, gdk rejects (its prefix test is `it32`-only, `io-icns.c:119–127`). `AllTypes` is not error-site-only: four zero bytes are also two valid one-pixel literals. Corpus: no colour entry other than `it32` starts with four zeros. | `It32Only`. |
| `rle_trailing` | **`Reject`**: R12 `RleTrailingBytes` after the blue channel. `Ignore`: accept and never read the suffix. | gdk does not check (`io-icns.c:300–312`): probe `rle-trailing-1` decodes in gdk; the crate rejects (`element.rs:337`). Error-site-only. Corpus: no colour entry has trailing RLE bytes. | `Reject`. |

Checked and **identical** in every incumbent, hence no field: entry length below 8 (probes `entry-length-0-last`, `-4-last`), an entry past `L` (`entry-oob-first`, `-last`), declared length above the slice (`declared-long`), RLE overrun (`rle-overrun-last-run`), empty RLE payload, mask length ≠ `N` (`mask-long`, `mask-short`; gdk `io-icns.c:283`, crate `element.rs:189`), colour without its mask (`colour-without-mask`: all reject; nobody substitutes an opaque mask), mismatched pair sizes. A declared length below 8 is accepted by the crate as an empty family (`declared-below-8`) but yields no image, so it is no acceptance difference.

**Excluded:** gdk reads RLE control and data bytes without an input bound (`io-icns.c:206–238`); a truncated colour payload then reads the following entry's bytes (probe `rle-truncated-then-mask`: gdk returns a 16×16 image built from mask bytes; probe `broken-il32-good-is32`: a 32×32 image from garbage) or beyond the buffer. Its `it32` prefix test reads four bytes even from a shorter payload (`io-icns.c:123`). No safe byte semantics reproduces either; truncation stays an error in every profile.

## B. Selection and delegation (outside `parse`/`decode_rgb`/`decode_mask`/`combine`)

These describe how a library turns a container into one picture. The core deliberately has no such function (R7, R14). They become fields only if the owner adds a verified selector or delegation API (QUESTIONS Q1–Q3).

| Field | Values | Incumbent behaviour, witness | Proposed |
|---|---|---|---|
| `opaque` | **`Caller`**: R9, opaque candidates are ranges for the caller. `GdkNested`: only `ic08`/`ic09` count (as one 256-px slot, last occurrence wins); their payload goes to gdk-pixbuf's sniffing loader, so **any** installed format decodes and the dimensions are not checked. `CratePng`: every listed PNG/JP2 type (`icp4`–`ic14`) is a candidate; PNG must decode to the type's pixel size; JP2 data is an error. | gdk `io-icns.c:106–117,259–279`: probes `png-ic08-256` and `png-ic09-512` decode; `png-ic08-16x16` decodes 16×16 (no dimension check); **`bmp-in-ic08` decodes a BMP** nested in `ic08`; `png-ic07-plus-is32`, `png-icp4-16`, `png-ic10-1024-plus-is32` ignore the PNG and use the legacy pair or fail. Crate `element.rs:117–133`: probes `png-ic07…`, `png-icp4-16`, `png-ic10…` decode the PNG, `png-ic08-16x16` fails "wrong dimensions", `jp2sig-ic08-plus-is32` fails (JP2 unsupported). Corpus: 15,404 files need a PNG decoder to yield anything. Safety of nested decoding is the PNG decoder's, not this core's. | `Caller`: PNG/JP2 stay outside the verified core (Q1). |
| `size_order` | **`Caller`**. `GdkFixed`: try 256 (ic08/ic09), 128, 48, 32, 16 in that order (24 is listed but its `switch` has no case, so it always fails, `io-icns.c:171–172,331–338`). `CrateLargest`: the largest available type by pixel width, PNG and legacy alike. | Probes `png-ic07-plus-is32` (crate 128×128 PNG, gdk 16×16 legacy), `png-ic10-1024-plus-is32` (crate 1,024×1,024, gdk 16×16). | `Caller` (Q2). |
| `duplicates` | **`Caller`**. `Last`: the last entry of a type wins. `First`: the first wins. | gdk overwrites while scanning (`io-icns.c:113–169`), crate `find` returns the first (`family.rs:121`): probe `duplicate-is32`, gdk shows the second `is32`, crate and the Strict harness the first. | `Caller`. |
| `fallback` | **`Caller`**. `NextSize`: when the chosen size fails (bad RLE, missing or bad mask, nested loader failure), try the next size. `None`: the first choice's error is the result. | gdk loops over sizes until one decodes (`io-icns.c:372–373`): probes `overrun-il32-good-is32` and `jp2sig-ic08-plus-is32` decode the smaller `is32`. Crate (with the harness's largest rule) has no fallback: both fail. | `Caller`. |

## Named profiles (proposal)

| Field | Strict | Recommended | GdkCompat-2.42.12 | IcnsCrateCompat-0.3.1 |
|---|---|---|---|---|
| file_suffix | Reject | Reject | Ignore | CrateStream |
| entry_tail | Reject | Reject | Ignore | Reject |
| entry_cap | Max4096 | Max4096 | Unlimited | Unlimited |
| raw_argb | Reject | Reject | RawFirst | Reject |
| zero_prefix | It32Only | It32Only | It32Only | AllTypes |
| rle_trailing | Reject | Reject | Ignore | Reject |

`ImageCrateCompat` has no ICNS counterpart; the third-party Rust incumbent is the `icns` crate, named here `IcnsCrateCompat-0.3.1` (owner to confirm the name, Q6). Recommended equals Strict; it exists so the API shape matches DIB and TGA and so later relaxations get a stable name. Both compatibility profiles reproduce **only** the decode-level fields; neither reproduces the library's selection, so on the corpus their effect on acceptance is nil until a selector exists. Under `GdkCompat`, `RawFirst` and `Unlimited` need explicit allocation/obligation-A treatment (PLAN.md).

## Not reproducible, explicitly excluded

- gdk-pixbuf's unbounded RLE and prefix reads (above) and its nested sniffing loader, which hands `ic08`/`ic09` bytes to every installed decoder, including BMP and, recursively, ICNS.
- The crate's allocation of `element_length` bytes before reading them (`element.rs:219–221`): the leniency harness recorded multi-gigabyte allocation failures (`leniency/icns/LENIENCY.md`, "entry_length").
- ARGB `ic04`/`ic05`/`icsb`, old 1/4/8-bit types and JPEG 2000: neither incumbent decodes them through the measured paths (gdk has no JP2 loader installed; the crate rejects JP2 and lacks ARGB).
