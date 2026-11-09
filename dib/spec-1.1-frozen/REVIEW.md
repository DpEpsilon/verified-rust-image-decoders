# Independent review of the DIB-CORE-SPEC 1.1 amendment (IcoEntryDibSlack)

Reviewer: independent of the author. Marking: RECALLED = not read from a primary source here. Read in full: SPEC.md (diffed against the frozen 1.0 SPEC.md), VECTORS.json (diffed against the frozen 1.0 file), check_vectors.py (diffed against the frozen oracle), gen_vectors.py, SOURCES.md, DESIGN-NOTES.md, THEOREMS-DELTA.md, and the shipped crate's `dib.rs`/`model.rs` mode sites.

Counts: **blocking 1, should-fix 5, nit 3.**

Verdict: **freeze after listed fixes.** The amendment is minimal and the old three modes are provably untouched at the vector level and, by differential fuzzing, at the behaviour level. The one blocking item is a vector gap in exactly the cases that motivated the mode (32-bpp slack-bearing icons).

## 1. Is the amendment minimal? (yes)

- SPEC.md diff against `spec-frozen/SPEC.md` (word-level): R1 enum, R3 ("either ICO mode"), R12 (one sentence on the new final check), R14 (trailing-bytes site list; "unreachable in BmpFile and IcoEntryDibSlack" in R15), R16 (vector count text), and new R19/R20. Nothing in R1-R11, R13, R17, R18 changed semantically. The rest of the spec says "ICO" generically; R19 makes the slack mode share every ICO rule, so no per-rule mode list is missing (R5, R8, R12 "in ICO" are covered by R19's umbrella; naming the modes in R5/R12 would still be tidier, nit N1).
- VECTORS.json: `n[:214]==frozen` as Python lists (names, modes, inputs, repeat fields and expected values identical). 58 appended vectors: modes used are only `IcoEntryDib` (old-mode contrasts) and `IcoEntryDibSlack`.
- check_vectors.py diff against the frozen oracle (sha256 `01029baeccb7...`): 7 hunks, all of the form `mode == 'IcoEntryDib'` becoming `ico` (a local `mode in (IcoEntryDib, IcoEntryDibSlack)`), plus the mode whitelist and the single trailing check `mode in ('BareDib','IcoEntryDib')`. That is the amendment and nothing else.
- Behavioural check of "old modes unchanged": 6,000 random inputs (single/double byte mutations of every small vector input and of the 16 corpus DIB entries, truncations, 1/2/5/70 random suffix bytes) times three old modes = 18,000 comparisons of the frozen oracle against the 1.1 oracle. **0 differences.** Same inputs in slack mode: author's 1.1 oracle versus my derived slack oracle (below): 0 differences; 796 of them were `TrailingBytes` in `IcoEntryDib` that slack mode turns into a decode.

## 2. Own oracle on all 272 vectors

I did not rewrite a 250-line DIB decoder. Slack mode is defined by R19 as "the old ICO mode, minus the final cursor check", so I derived it from the **frozen 1.0 oracle** (hash-checked) by semantics, independent of the author's amended `check_vectors.py`: `Slack(b)` = `FileTooLarge` if `|b|>64 MiB`, else `IcoEntryDib(b)` unless that returns `TrailingBytes`, in which case the consumed length `c` is found by binary search (`IcoEntryDib(b[:k])` is `Ok` exactly at `k=c`, `TrailingBytes` above, a truncation error below) and `IcoEntryDib(b[:c])` is returned. Result: **272/272 agree, 0 disagreements**, including both 64 MiB slack vectors and the over-cap vector (scratch: `/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/dib11.py`). Neither SPEC text nor any vector is wrong. Limitation: the derivation shares the frozen oracle's DIB logic, so this tests the amendment, not DIB 1.0.

## 3. Mutation sweep of the new branch

18 source mutants of the 1.1 oracle at the final check, mask availability, `biSizeImage` set, doubled height, odd height, RLE rejection, AND stride, alpha policy and BI_RGB alpha byte, run against all 272 vectors (the 64 MiB-input pairs skipped for speed; they only exercise the cap). 14 killed (by new vectors where the mutant is slack-specific), 1 equivalent, **3 real survivors**:

- `slack_alpha_noand`: in slack mode, the ICO whole-image alpha rule (any nonzero XOR alpha keeps XOR alpha; all-zero XOR alpha uses AND) is replaced by "never use AND". Survives.
- `slack_alpha_bytes_ignored`: BI_RGB 32-bpp alpha byte ignored (opaque) in slack mode only. Survives.
- `slack_rle_ok`: RLE accepted in slack mode instead of `UnsupportedOrientation`. Survives.
- Equivalent: `pos` not advanced past the AND mask in slack mode (`pos` is only used by the skipped check).
Killed correctly: slack treated like BareDib (21 vectors), old ICO given slack (20), BmpFile made strict, BareDib made slack, `pos>len` instead of `!=`, mask availability skipped (4) or off by one, `biSizeImage` accepted unchecked (2), as `X` only (`image-size-8-slack`), as `len-pixel_start` (`image-size-9-slack`), image size checked after mask (`precedence-image-size-before-short-and`, `invalid-size-and-short-new`), odd height accepted, height not halved (27), AND stride dropped (5).

## Blocking

### B1. Every new slack vector is a 1x1 24-bit image; the slack-mode alpha, RLE and indexed paths are untested (VECTORS.json 215-272; gen_vectors.py:6-12)
`body()` builds one 24-bpp 1x1 DIB. The motivating real inputs are 32-bpp (`pillow.ico` entries 1-3, slack 192/128/48 bytes) and 1-bpp palette (`hopper_mask.ico`). Add slack-mode vectors (each with a 1-byte slack and the old-mode contrast returning `TrailingBytes`, so that the slack branch itself is exercised), concrete layouts as 1x1 DIBs with stored height 2:
- 32-bpp BI_RGB, XOR `00 00 ff 80`, AND `80 00 00 00`: alpha stays `80`; expect `ff000080`. Kills `slack_alpha_noand`.
- 32-bpp BI_RGB, XOR `00 00 ff 00`, AND `80 00 00 00`: all XOR alpha zero, AND=1: expect `ff000000`; with AND `00 00 00 00`: expect `ff0000ff`.
- 32-bpp BI_RGB, XOR alpha `7f`: expect `ff00007f`. Kills `slack_alpha_bytes_ignored` (a decoder treating the byte as unused returns `ff0000ff`).
- 32-bpp BITFIELDS without an alpha mask (AND veto), and with an explicit alpha mask.
- RLE8 (compression 1, 8 bpp) and RLE4 header: expect `UnsupportedOrientation`. Kills `slack_rle_ok`.
- 1-bpp with two palette entries and `biClrUsed=0`, 16x16 or 8x1, plus 64 zero slack bytes (the shape of `hopper_mask.ico`); 8-bpp indexed with `PaletteIndexOutOfRange` before slack.
- 24-bpp with `biClrUsed=1`: `InvalidColorCount` (ICO high-depth rule survives in slack mode).
- Negative stored height `-2`: `InvalidDimensions`.
Also add one slack vector whose `biSizeImage` equals the whole input length (`X+A+slack`): expect `InvalidImageSize` (a plausible implementer shortcut: "image size counts the slack").

## Should-fix

### S1. DESIGN-NOTES.md mixes 1.0 and 1.1 statements; several are now false (DESIGN-NOTES.md:3, 13-14, 17, 21-28)
Line 3 still says the core accepts `BmpFile, BareDib, IcoEntryDib` (three modes); the AND-plane bullet says `hopper_mask.ico` and `pillow.ico` "fail `TrailingBytes` (D5)", true only for the old mode; the "BareDib and IcoEntryDib remain exact-slice decoders" sentence and "the shim ... for `mask_1.cur`" remain; the corpus-cost section is labelled historical but is mixed with 1.1 text. Either move the 1.1 delta to the top and mark each inherited bullet "1.0 text, still true for the old modes", or rewrite the three bullets.

### S2. SOURCES.md reports ICO fixture counts inside a DIB document (SOURCES.md:3, 16-17)
"The 200 revised fixtures ... all 186 draft-one vectors" are ICO/DIB-1.0 draft history, not 1.1 facts (the DIB 1.0 vector count is 214, the 1.1 count is 272). Delete or label them historical; state once that the 1.1 oracle passes 272 and that the independent review (this one) agrees on 272.

### S3. THEOREMS-DELTA is incomplete as a work list (THEOREMS-DELTA.md)
Missing: (a) `model_vectors.rs` (8,312 lines, 137 occurrences of `IcoEntryDib`, the `agree_*` witness proofs): the 20 proof-gate witnesses include six ICO ones; the delta says nothing about regenerating or re-verifying them after `Mode` and `mp_mask_phase` change, nor whether any of the 58 new vectors become witnesses (I suggest `slack-1-new`, `slack-1-old`, `image-size-9-slack`, `mask-short-3-new`, `invalid-size-and-short-new`, and the new B1 alpha vector). (b) The enum change is a public-API change: `Mode` gains a variant, so any exhaustive `match` downstream breaks and the crate needs a version bump; `tests/conformance.rs` reads `spec-frozen/VECTORS.json` and must be pointed at, and learn to parse, the 1.1 `mode` string. (c) Site count: `dib.rs` has 32 and `model.rs` 16 mentions of `Mode::IcoEntryDib` (e.g. `dib.rs:733, 759, 1101, 1285-1327, 1414, 2446-2467, 4189-4272, 4380-4393, 4476, 4583, 4623-4672, 4691-4779, 5303`). The mask helper `mask_done_eq` / `mp_mask_phase` need a three-way case (BmpFile, exact ICO and BareDib, slack), not two. Recommend introducing one `is_ico(mode)` helper and one `exact_tail(mode)` helper first, so that the 32 sites change mechanically. (d) the mutation list omits "slack mode rejects RLE / non-ICO alpha" (B1 vectors) and the cap boundary mutants (`>=` for the slack mode's 64 MiB vector).

### S4. The T12 export advice is more pessimistic than the code needs (THEOREMS-DELTA.md T11/T12 bullet)
The shipped crate already has `t12_linear_work` (`dib.rs:5388`): same body as `decode` (both call `decode_tracked` with a zero counter) and it ensures `steps<=16*(|b|+N)` and `decode_mp_eq`. Making a renamed copy `pub fn decode_with_steps` public, and having the ICO crate call that function instead of `decode`, does connect the bound to the ICO call: the ghost return is erased, so the executable is identical. The delta says merely making it public does not establish the bound; that is true only if the ICO crate keeps calling `decode`. State the cheap route explicitly (the ICO spec then names the tracked function as its DIB call), and demote "the least bounded part" accordingly. The ICO redraft's R8 currently says it calls `dib_verus::decode(...)`; it must say which of the two.

### S5. Effort estimate
The delta is right that the code change is one branch. It underestimates the regression pass: the 1,367-obligation proof carries its mode matches in specs and invariants (S3c), and the witness file is 8,312 lines whose `agree_*` proofs evaluate `mp` by unfolding and may need new fuel after the model changes. For a DeepSeek-class implementer I would budget: helper refactor and enum plus models (one milestone), concrete mask helper and integrated equality (one), witness proofs and export (one), with the full verify run between each. "Several focused proof iterations" is fair; "a small fraction of decoder logic" is true of the logic and false of the verification time. I have not run any of it.

## Nits

- N1. Name `IcoEntryDibSlack` explicitly in R5, R8 and R12 where the text says "ICO", or say once at R1 "ICO means either ICO mode".
- N2. R19 "for the complete already-decoded pixel result" is clumsy; write: "if `IcoEntryDib(b)` returns `TrailingBytes` after decoding `d`, slack returns `Ok(d)`".
- N3. Slack mode with a producer whose AND stride is wider than the standard (the `pillow.ico` shape: zero slack, harmless because the XOR alpha is nonzero) silently reads wrong mask rows when XOR alpha is all zero. Say in DESIGN-NOTES that slack mode trades `TrailingBytes` for this possibility. I measured the slack-mode output of `hopper_mask.ico` and `pillow.ico` entry 1 (48x48) against the installed gdk-pixbuf 2.42.12: pixel-identical.

## Ready to freeze?

**Freeze after listed fixes:** B1 (new vectors, then regenerate VECTORS.json and the ICO oracle's pinned SHA, which will change because the DIB oracle file changes), S1, S2, S3, S4 wording. S5 and the nits do not block.

# REVIEW addendum (final recheck of the 302-vector draft)

Verdict: **ready to freeze.** All findings fixed; no real mutation survivor remains.

- **Frozen vectors:** `VECTORS.json[:214] == spec-frozen/VECTORS.json` (Python list equality, names, modes, inputs, repeat fields and expected values). 302 total = 214 + 88. `check_vectors.py` is unchanged since the first review (sha256 `214dbf12...`, same 7-hunk diff against the frozen oracle); `python3 -I check_vectors.py`: 302 passed. `run_corpus.py` (new): 60 accepted, 22 rejected, 2 container-only on the 84 local files.
- **Own oracle on all 302** (frozen oracle plus R19 slack semantics, binary-searched consumed length; SPEC.md text changed only by the umbrella sentence "ICO means either ICO mode" and the R19 wording fix, so no oracle change): **0 disagreements.** Earlier 272 still agree.
- **Mutation sweep** (same 18 mutants, plus two stronger alpha mutants "slack never uses AND" and "slack always uses AND", run against the 302 vectors, 64 MiB-repeat vectors skipped): previous real survivors all **killed**: `slack_alpha_noand` (now `fields32-no-alpha-new`; the stronger variants by `alpha32-00-mask00-new`/`fields32-no-alpha-new` and `alpha32-80-mask80-new`/`alpha32-7f-mask80-new`/`alpha32-global-new`), `slack_alpha_bytes_ignored` (`alpha32-*-new`, 4 vectors), `slack_rle_ok` (`rle8-forbidden-new`, `rle4-forbidden-new`). New kills for `slack_sizeimage_any`/`_len` (`image-size-includes-slack-new`) and `slack_odd_height_ok` (`negative-height-new`). The one remaining survivor (`pos` not advanced in slack mode) is equivalent: `pos` is only used by the skipped check. Cap boundary (`>` versus `>=` for slack) is covered by `max-input-slack-*` and `over-max-input-slack-new`, which I verified by expansion in the first review.
- **Hand derivation (4 new fixtures):** `alpha32-80-mask80-new`: 1x1, 32 bpp, BI_RGB, XOR bytes `00 00 ff 80` (B,G,R,A) gives `ff000080` before the AND mask; AND `80 00 00 00` would force alpha 0, but XOR alpha is nonzero so the whole AND plane is ignored: `ff000080`; slack byte `5a` after the 4-byte mask; total 40+4+4+1=49 matches. `alpha32-00-mask80-new`: XOR alpha 0, so the AND fallback applies and bit 1 gives alpha 0: `ff000000`. `indexed1-wide8-new` (120 bytes): 8x1, 1 bpp, `biClrUsed 0` so 2 entries `00 00 00 00` (black) and `00 00 ff 00` (red); one row `ff 00 00 00` (stride 4, all eight bits set, index 1) gives eight red pixels `ff0000ff`; AND 4 bytes of zero; 64 slack bytes: 40+8+4+4+64=120 (the `hopper_mask.ico` shape). `fields32-no-alpha-new`: BITFIELDS 32 bpp with masks R `00ff0000`, G `0000ff00`, B `000000ff`, no alpha mask, pixel word `00ff0000` gives opaque red, then AND bit 1 vetoes: `ff000000`.
- **Earlier findings:** B1 fixed (vectors listed above; each has its `-old` contrast returning `TrailingBytes` or the shared error). S1 fixed (DESIGN-NOTES text now says the original mode rejects, slack accepts; the D5 bullet rewritten). S2 fixed (SOURCES labels the 1.0 counts historical and states 302 and the independent agreement). S3 fixed (THEOREMS-DELTA now has the inventory paragraph: 32 `dib.rs` and 16 `model.rs` sites, `is_ico`/`exact_tail` helpers, three-way `mask_done_eq`/`mp_mask_phase`, `model_vectors.rs` 8,312 lines/137 mentions and its regeneration, six non-vacuity witnesses, public `Mode` variant break and version bump, `tests/conformance.rs` retargeting). S4 fixed (the delta now says export `t12_linear_work` as `pub fn decode_with_steps` and have ICO call it instead of `decode`; erasure leaves the executable path identical). S5 fixed (three milestones with full verification between; "not one-line tasks"). N1 fixed (umbrella sentence in SPEC.md:3), N2 fixed (R19 wording), N3 fixed (DESIGN-NOTES now states the wider-AND-stride trade-off).
- **Not fixed / new:** none. Remaining work is not a spec defect: no 1.1 proof exists; the freeze must record the vector hash and the oracle hash `214dbf12...`, because the ICO pin depends on it.
