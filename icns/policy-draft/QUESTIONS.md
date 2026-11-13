# ICNS policy: owner decisions and places the draft guessed

Each item names the decision, the options and a recommendation, and can be read on its own. Q1–Q3 block a freeze; Q4–Q8 can wait without changing Recommended. Nothing here is decided. Field names refer to `OPTIONS.md` in this directory; "probe `x`" is a case in `/home/ws/projects/leniency/policy-probe/probe_cases.py`, results in `results-icns.json` there.

## Decisions that block a freeze

**Q1. Is an ICNS policy layer worth proving now?** On the 15,701-file corpus, Strict's parse rejects 7 files and none of the 6 decode-level fields would recover any of them (truncated or non-ICNS files that both libraries also reject). All 765 legacy colour entries with masks decode under Strict's RLE rules. The gap between Strict (267 decodable files) and the libraries (`icns` crate 11,736, gdk-pixbuf 454) comes from PNG/JPEG 2000 elements: 15,404 files hold PNG elements and no legacy colour+mask pair. Options:
- (a) Freeze this catalogue, implement only a `Strict` profile entry point (API parity with DIB/TGA), defer every relaxation.
- (b) Implement all 6 fields and the two compatibility profiles now.
- (c) Write no ICNS policy layer; spend the effort on Q2.

*Recommendation: (a).* Each of the 6 fields is small, but none buys a file; a frozen catalogue keeps the measured facts without a proof campaign.

**Q2. PNG and JPEG 2000 elements.** The frozen core classifies them as opaque candidates and never decodes them (R9). gdk-pixbuf decodes only `ic08`/`ic09` and does so through its generic sniffing loader, which accepts **any** installed format with any dimensions (probe `bmp-in-ic08` decodes a BMP nested in `ic08`; `png-ic08-16x16` a 16×16 PNG); it ignores `ic07`, `ic10`–`ic14` and `icp4`–`icp6`. The `icns` crate decodes PNG for every listed type, requires the type's pixel size, and rejects JPEG 2000. Options:
- (a) Keep delegation in the (unverified) glycin glue, as today; the verified core supplies ranges only.
- (b) Add to the verified layer a *classification* of each opaque candidate as `Png`/`Jp2`/`Unknown` by signature (already computed by R9) and require the caller's decoder to check the nominal side; still no pixel decoding.
- (c) Decode PNG inside the verified layer (a new verified PNG core).

*Recommendation: (a) for this policy layer, (b) if glycin wants a stated contract for the dimension check.* Never adopt gdk's nested sniffing: it turns an ICNS file into a carrier for every other format.

**Q3. A verified selector.** The core leaves selection to its caller (R7, R14). gdk-pixbuf tries a fixed size order (256 via `ic08`/`ic09`, then 128, 48, 32, 16), takes the last duplicate, and falls back to the next size on any failure (probes `overrun-il32-good-is32`, `jp2sig-ic08-plus-is32`, `duplicate-is32`). The `icns` crate (with the harness's rule) takes the largest available type, PNG or legacy, the first duplicate, and does not fall back (`png-ic07-plus-is32`, `png-ic10-1024-plus-is32`). *Recommendation: no selector in the policy layer; specify one separately, as a pure function over the parsed entry list, if glycin needs a verified choice. Its fallback would have to call `combine` more than once, which needs its own allocation and work bound.*

## Decisions that can wait

**Q4. Bytes after the declared length (`file_suffix`, R4).** Both libraries accept them (probes `suffix-1-byte`, `suffix-8-bytes`, `suffix-entry`, `declared-short-at-entry-boundary`), and R4's own text records that. The rejection is a settled, user-approved decision; the corpus has 1 such file, which is malformed inside its declared length as well. *Recommendation: keep `Reject` in Recommended; offer `Ignore` in GdkCompat and `CrateStream` in IcnsCrateCompat.*

**Q5. Uncompressed `4N` colour payloads (`raw_argb`, R10).** gdk-pixbuf reads a colour payload of exactly `4N` bytes as ARGB (probe `argb-raw-4N`), before trying RLE. A `4N`-byte payload can also be valid RLE, so gdk's `RawFirst` changes some Strict-accepted images; `RawOnRleError` (raw only after RLE fails) is the error-site-only variant. One corpus file has `4N` payloads but no masks, so even gdk rejects it. *Recommendation: `Reject` in Recommended; `RawFirst` only in GdkCompat.*

**Q6. Name of the Rust-incumbent profile.** DIB and TGA use `ImageCrateCompat-0.25.10`. `image` 0.25.10 has no ICNS decoder (no `icns` string in its sources), so the Rust incumbent is the `icns` crate 0.3.1. *Recommendation: `IcnsCrateCompat-0.3.1`.* Alternative: omit the profile, since the crate's decode-level leniency (`CrateStream`, `AllTypes`) touches no corpus file.

**Q7. The `recovered` flag.** None of the 6 fields substitutes or discards decoded pixel data: `Ignore` and `CrateStream` skip bytes outside every decoded payload, and `RawFirst`/`AllTypes` are alternative interpretations. Following TGA's closed-list rule (interpretations do not set it), `recovered` is always false for ICNS. *Recommendation: keep the flag in the API for uniformity, define it as constantly false, and prove that (obligation E is then one lemma).* Alternative: count `RawOnRleError` firing as an event, because it replaces a failed decode.

**Q8. Unlimited entry counts (`entry_cap`).** Both libraries accept more than 4,096 entries (probe `entries-4097-junk`); no corpus file has that many. `Unlimited` raises the entry-list reservation bound from 4,096 records to about 8.4 million for a 64 MiB input. *Recommendation: keep 4,096 in every profile and list it as a known residual of GdkCompat/IcnsCrateCompat.*

## Places the draft had to guess

- **gdk-pixbuf's size 24.** `sizes[]` lists 24 but the `switch` in `load_resources` has no `case 24`, so that pass returns FALSE at the first block (`io-icns.c:171–172`); read from source, consistent with every probe.
- **Truncated RLE under gdk-pixbuf** reads past the payload (into the next entry or the heap). The draft treats it as excluded, not as a reproducible rule; probes `rle-truncated-then-mask` and `broken-il32-good-is32` show it returning images built from neighbouring bytes.
- **Library acceptance counts** (11,736 and 454) come from the bench's equality gate on the same corpus (`bench/README.md:250`), not from a rerun in this phase; a full rerun was skipped because reading the 668 MB corpus through the volume took about 15 minutes for headers alone.
- **The format template.** The brief named `dib/policy/QUESTIONS.md`; that file does not exist. This file follows `tga/policy/QUESTIONS.md`.
