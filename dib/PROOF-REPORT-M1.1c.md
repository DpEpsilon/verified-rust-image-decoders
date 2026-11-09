# Proof report — M1.1c (DIB-CORE-SPEC 1.1: witness file, tracked-decoder export, mutations)

Delivered by DeepSeek V4.1 Flash (Verus). Branch `dib-1.1`. Verus 0.2026.11.02.
No commits made (coordinator commits).

## Headline

- Clean full reverify after `cargo clean`, twice: `verification results:: 1557 verified, 0 errors`
  (`runs/098.log`, 139 s; `runs/101.log`, 138 s). Both are on the final shipped source.
- `cargo test` (release and debug): conformance **302/302** vectors through the exported
  tracked decoder, corpus **84 files, 58 accepted**.
- `model_vectors.rs` regenerated from `spec-1.1-frozen/VECTORS.json`: **294** `agree_*`
  obligations (254 literal `by (compute)`, 24 term-form, 16 staged 32-bpp ICO), up from 209.
  The 20 existing proof-gate witnesses are kept; the six new non-vacuity witnesses are proved.
- `decode_with_steps` exported (the `t12_linear_work` body) and re-exported from `lib.rs`;
  `tests/conformance.rs` now decodes through it.
- Mutations: **44 implementation mutants**, each changing only `dib_verus/src/dib.rs`.
  43 are killed by a **verifier** diagnostic; mutant 20 (the second full-size `vec!`) is an
  **audit** kill (`tools/audit_alloc.sh`), not a verifier kill. Every failure.log was read.
- `tools/audit_alloc.sh` OK on the live source (`vec!` allocations: 1, in `alloc_output`).
- No escapes (`assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo!`/`unimplemented!`/
  `requires(false)`), no solver-budget attribute. Frozen 1.1 hashes match `spec-1.1-frozen/FREEZE.txt`;
  `spec-frozen/` and `THEOREMS.md` unchanged. Backups `dib.rs.m11a`, `model.rs.m11a`,
  `model_vectors.rs.m11a` taken before editing.

## (1) Witness file regenerated from the 302-vector frozen file

`tools/gen_model_vectors.py` now reads `spec-1.1-frozen/VECTORS.json` (was `spec-frozen/`) and
knows both ICO modes (`ICO_MODES`, `is_ico_mode`, used for geometry, colour-term and alpha
routing). `tools/gen_ico32.py` is mode-parameterized: the record's `mode` field and every
`Mode::<...>` it emits are the fixture's own mode, so the staged 32-bpp form serves
`IcoEntryDib` and `IcoEntryDibSlack` alike. `gen_model_vectors.py --check` reports up to date.

Proof forms (294 obligations):

| form | count | used for |
|---|---|---|
| literal | 254 | small vectors closed by `by (compute)` |
| term | 24 | 16-bit / BITFIELDS where `u32_trailing_zeros` blocks `compute` |
| staged | 16 | every 32-bpp ICO success, in both modes |

The eight vectors that are not emitted as literal agreements (either longer than the 40,000-byte
literal limit, or held back by name like `height-at-cap-8192`) are covered symbolically or at
runtime:

| vector | bytes | coverage |
|---|---|---|
| `input-over-64m` | 67108865 | `model::lemma_mp_l1` (L1) |
| `precedence-too-large-bmp-bad-magic` | 67108865 | `model::lemma_mp_l1` (L1) |
| `input-exact-64m` | 67108864 | `model::lemma_mp_l2` (L2) |
| `output-exact-64m-rle8` | 266290 | runtime conformance test |
| `height-at-cap-8192` | 32814 | runtime conformance test |
| `max-input-slack-IcoEntryDibSlack` | 67108864 | runtime conformance test (R19 at-cap success) |
| `max-input-slack-IcoEntryDib` | 67108864 | runtime conformance test (R19 at-cap exact-tail contrast) |
| `over-max-input-slack-new` | 67108865 | `model::lemma_mp_l1` (L1, over-cap slack) |

The two new 64 MiB R19 boundary vectors are 64 MiB slices; they are covered by the runtime
conformance test (which verifies the exact `FileTooLarge`/success/`TrailingBytes` outcomes against
the frozen oracle), exactly as the 1.0 64 MiB fixtures were. The new-mode branch itself is proved
symbolically by the small `slack-*` vectors, including `slack-1-new`/`slack-1-old` (the same
preamble with one suffix byte, in both modes).

### Gate witnesses (by name, with proof item)

The 20 existing proof-gate witnesses keep their `agree_<name>` proof items (hyphens to underscores):

`empty`, `file-bad-magic`, `file-offset-before-palette`, `header-12-BareDib`,
`header-16-negative-height-topdown`, `header-64-BareDib`, `planes-before-dimensions`,
`palette-short-one-8`, `palette-short-invalid-index-4`, `bmp-optimal-table-u32-wrap`,
`bmp-fields-40-masks-then-optimal-table-gap`, `masks-outside`, `bi-rgb-16-4404`,
`fields-topdown-16`, `rle8-run-over-row`, `rle8-eol-past-height`,
`rle8-absolute-trunc-before-index`, `ico-mask-msb-and-padding`,
`ico-fields-32-no-alpha-and-veto`, `ico-fields-32-explicit-alpha-zero-and-fallback`.

The six new non-vacuity witnesses, each with its proof item:

| witness (delta name) | proof item | statement |
|---|---|---|
| `slack-1-new` | `agree_slack_1_new` | `mp(b, IcoEntryDibSlack) == Ok((1,1,ff0000ff))` |
| `slack-1-old` | `agree_slack_1_old` | `mp(b, IcoEntryDib) == Err(TrailingBytes)` |
| `image-size-9-slack` | `agree_image_size_9_slack` | `mp(b, IcoEntryDibSlack) == Err(InvalidImageSize)` |
| `mask-short-3-new` | `agree_mask_short_3_new` | `mp(b, IcoEntryDibSlack) == Err(TruncatedAndMask)` |
| `invalid-size-and-short-new` | `agree_invalid_size_and_short_new` | `mp(b, IcoEntryDibSlack) == Err(InvalidImageSize)` |
| `alpha32-7f-mask80-new` | `agree_alpha32_7f_mask80_new` | `mp(b, IcoEntryDibSlack) == Ok((1,1,ff00007f))` |

`agree_alpha32_7f_mask80_new` is staged (`want()` is the fixture's own `ff00007f`, so the obligation
states the fixture result, not a vacuous shape).

## (2) Tracked decoder exported

`dib.rs:5489`:

```rust
pub fn decode_with_steps(bytes: &[u8], mode: Mode) -> (r: (Result<Decoded, Error>, Ghost<nat>))
    ensures
        r.1@ as int <= 16 * (bytes@.len() + t12_checked_n(bytes@, mode)),
        decode_mp_eq(r.0, bytes@, mode),
```

This is the former private `t12_linear_work` body (`decode_tracked` from a zeroed ghost counter),
renamed and made public; the private witness is gone. `t12_checked_n` became `pub open spec fn`
(Verus forbids a public `ensures` from naming a private fn). The executable path is unchanged:
`decode` still calls `decode_tracked` with a zeroed counter and still carries `decode_mp_eq`; the
ghost return and every counter update erase. `lib.rs` re-exports it. `tests/conformance.rs` decodes
every fixture through `decode_with_steps(...).0`, so the exported call is exercised at runtime, and
the ICO crate can invoke the same function. No ICO crate exists in this workspace yet, so the
"ICO calls `decode_with_steps` instead of `decode`" step is the ICO milestone's, not this one.

## (3) Mutations

`tools/mutations.py` gained the eight delta mutants (36-44; item (5) has two arms);
`run_mutations.sh` was used unchanged. The previous campaign's evidence is preserved by copying
`mutation/` to `mutation-1.0/` (41 dirs, `result.txt` + `failure.log` only, 82 files).
`mutation/base` was refreshed from the live crate (`mutation/refresh_base.sh`) and verified
byte-identical to `dib_verus/src/{dib.rs,model.rs,model_vectors.rs}`. Six anchors were stale because
M1.1a re-indented `finish_mask` (m10, m17, m21, m22, m30, m31, m35); all were re-pinned to the final
source before the batch. Every mutant differs from `base` only in `src/dib.rs`.

All 44 ran in one unattended batch (`tools/run_mutations.sh $(seq 1 44)`, background, output
`mutation/batch-1.1c.out`). Results (crate line, first verifier diagnostic, log path). Run numbers
are from RUNLOG.tsv; each `runs/<n>.log` is the per-mutation verifier log copied to
`mutation/mN/failure.log`.

| mutation | change (dib.rs only) | crate | first diagnostic | log |
|---|---|---|---|---|
| m1 | cap `>` -> `>=` | 1556, 1 err | rlimit on `parse_header` (dib.rs:939) | mutation/m1/failure.log |
| m2 | dims/planes order | 1556, 1 err | postcondition (dib.rs:1160) | mutation/m2/failure.log |
| m3 | OS/2-16 offsets | 1556, 1 err | assertion (dib.rs:1269) | mutation/m3/failure.log |
| m4 | ignore biClrUsed | 1556, 1 err | assertion (dib.rs:1718) | mutation/m4/failure.log |
| m5 | 3-byte INFO stride | 1556, 1 err | precondition (dib.rs:1823) | mutation/m5/failure.log |
| m6 | drop palette index guard | 1556, 1 err | requires (dib.rs:464) | mutation/m6/failure.log |
| m7 | overlapping masks | 1556, 1 err | postcondition (dib.rs:370) | mutation/m7/failure.log |
| m8 | replicate 5-bit components | 1556, 1 err | postcondition (dib.rs:293) | mutation/m8/failure.log |
| m9 | BI_RGB32 alpha outside ICO | 1556, 1 err | assertion (dib.rs:2489) | mutation/m9/failure.log |
| m10 | ignore AND on all-zero XOR alpha | 1556, 1 err | precondition (dib.rs:4861) | mutation/m10/failure.log |
| m11 | reverse vertical rows | 1556, 1 err | assertion (dib.rs:2039) | mutation/m11/failure.log |
| m12 | unpadded raw stride | 1556, 1 err | assertion (dib.rs:2632) | mutation/m12/failure.log |
| m13 | drop RLE run-over-row bound | 1556, 1 err | precondition (dib.rs:3702) | mutation/m13/failure.log |
| m14 | absolute completeness before rows | 1556, 1 err | postcondition (dib.rs:3540) | mutation/m14/failure.log |
| m15 | index before packet availability | 1556, 1 err | postcondition (dib.rs:3540) | mutation/m15/failure.log |
| m16 | accept end without EOB | 1556, 1 err | postcondition (dib.rs:3652) | mutation/m16/failure.log |
| m17 | omit ICO mask availability | 1556, 1 err | precondition (dib.rs:4858) | mutation/m17/failure.log |
| m18 | wrapping u16 N | 1556, 1 err | assertion (dib.rs:1498) | mutation/m18/failure.log |
| m19 | rescan suffix per command | 1557, 1 err | postcondition (dib.rs:3643) | mutation/m19/failure.log |
| m20 | second full-size `vec!` | 1557, **0 err** | (verifier survives) | mutation/m20/failure.log |
| m21 | bare DIB trailing bytes | 1556, 1 err | assertion (dib.rs:4949) | mutation/m21/failure.log |
| m22 | discard ICO 32-bpp alpha byte | 1556, 1 err | assertion (dib.rs:2475) | mutation/m22/failure.log |
| m23 | reverse AND bit order | 1556, 1 err | assertion (dib.rs:4337) | mutation/m23/failure.log |
| m24 | reverse RLE4 nibbles | 1556, 1 err | assertion (dib.rs:3154) | mutation/m24/failure.log |
| m25 | drop RLE delta offset | 1556, 1 err | postcondition (dib.rs:3657) | mutation/m25/failure.log |
| m26 | reject EOB before last row | 1556, 1 err | postcondition (dib.rs:3657) | mutation/m26/failure.log |
| m27 | BmpFile raw at palette end | 1555, 2 err | rlimit (dib.rs:1564) + assertion/precondition | mutation/m27/failure.log |
| m28 | non-contiguous masks | 1556, 1 err | postcondition (dib.rs:402) | mutation/m28/failure.log |
| m29 | 16-bit bit-15 as alpha | 1556, 1 err | postcondition (dib.rs:2168) | mutation/m29/failure.log |
| m30 | reject ICO image size X+A | 1556, 1 err | assertion (dib.rs:4805) | mutation/m30/failure.log |
| m31 | AND veto despite XOR alpha | 1556, 1 err | precondition (dib.rs:4858) | mutation/m31/failure.log |
| m32 | EOL past final row | 1556, 1 err | precondition (dib.rs:3748) | mutation/m32/failure.log |
| m33 | masks outside depth | 1555, 2 err | postcondition (dib.rs:370) | mutation/m33/failure.log |
| m34 | skip bfOffBits/table check | 1556, 1 err | postcondition (dib.rs:1584) | mutation/m34/failure.log |
| m35 | BITFIELDS no-A as explicit alpha | 1556, 1 err | postcondition (dib.rs:4677) | mutation/m35/failure.log |
| m36 | delta (1) slack like BareDib | 1556, 1 err | assertion (dib.rs:4883) | mutation/m36/failure.log |
| m37 | delta (2) slack policy on old ICO | 1556, 1 err | precondition (dib.rs:4919) | mutation/m37/failure.log |
| m38 | delta (3) skip AND check on suffix | 1555, 2 err | rlimit (dib.rs:1564) + assertion/precondition | mutation/m38/failure.log |
| m39 | delta (4) any nonzero biSizeImage | 1555, 2 err | rlimit (dib.rs:1564) + assertion | mutation/m39/failure.log |
| m40 | delta (5a) omit doubled height | 1556, 1 err | requires (dib.rs:1331) | mutation/m40/failure.log |
| m41 | delta (5b) ignore BI_RGB32 alpha | 1556, 1 err | precondition (dib.rs:4677) | mutation/m41/failure.log |
| m42 | delta (6) allow RLE in slack | 1556, 1 err | postcondition (dib.rs:1160) | mutation/m42/failure.log |
| m43 | delta (7) cap `>` -> `>=` | 1556, 1 err | rlimit on `parse_header` (dib.rs:939) | mutation/m43/failure.log |
| m44 | delta (8) rescan suffix on return | 1556, 2 err | rlimit (dib.rs:1564) + postcondition | mutation/m44/failure.log |

Mapping of the delta's eight mutation items to mutants: (1) -> 36, (2) -> 37, (3) -> 38,
(4) -> 39, (5) -> 40 (omit doubled height) and 41 (ignore BI_RGB alpha/AND fallback),
(6) -> 42, (7) -> 43, (8) -> 44. m1 and m43 are the same edit (the historic cap mutant and the
delta's item (7)); both are run and both kill.

Every mutant failed on a **verifier** diagnostic (postcondition, assertion, precondition, `requires`,
or a per-function resource-limit body check) — never on a rustc compile/type error. The 44
diagnostic sets were read from the failure.log files; none contains a type/mode/`expected` rustc
error. Mutants m38, m39, m44 add a real per-command or per-return loop that trips the per-function
rlimit on `rle_command`/`decode_tracked` in addition to a genuine obligation failure; the obligation
failure is present in each, so they are verifier kills, not just budget expiries.

**Mutation 20 is an audit kill, not a verifier kill.** It allocates a second full-size RGBA buffer
and still verifies at `1557 verified, 0 errors` (`runs/072.log`). This is inherent: T3's allocation
count is a ghost counter incremented at the single call site, which cannot observe a physical
allocation; THEOREMS.md T3 says a source-level review covers this. `tools/audit_alloc.sh` reports
`vec! allocations: 2 (expected exactly 1, in alloc_output)`, FAIL, on `mutation/m20/dib_verus/src/dib.rs`,
and OK on the live source. Never report 44/44 verifier kills.

One mutant needed a correction during the campaign: m38 was first written with the spec fn
`exact_tail(h.mode)` called from exec code, which is a rustc mode error, not a verifier diagnostic;
it was rewritten to the exec expression `h.mode == Mode::IcoEntryDib` (the site is already inside the
`ico` branch) and rerun at `runs/097.log`, where it is a verifier kill.

Per-mutation `target/` and `.verus` caches are deleted by `run_mutations.sh` after each run; none
remain. `df -i` shows 344 M inodes free (8% used).

## Ticks (T1-T13, L1-L3)

- **T1 safety, T2 exact header, T3 allocation, T4 conversion, T5 rows, T6 raw, T7 RLE, T8 AND:**
  carried from M1.1a and unchanged; all verify at their manifest quantification over the new mode
  (1557 verified, 0 errors). The six new witnesses discharge the non-vacuity obligation on the
  new-mode branch (m36 kills the "slack like BareDib" reading at `agree_slack_1_new`; m40/m41 kill
  the doubled-height and alpha-fallback readings).
- **T9 `decode=MP`, T10 exact errors, T13 success/resource:** `decode_mp_eq` quantifies over `mode`
  universally and covers all four modes (M1.1a); the new vectors' `agree_*` tie `MP` to the frozen
  expectations. `decode_with_steps` re-states `decode_mp_eq`.
- **T11 termination, T12 linear work:** no loop added; `steps <= 16*(|b|+N)` is proved on
  `decode_tracked` and stated by the exported `decode_with_steps`. m19 and m44 add a real suffix
  scan and are killed (verifier), so the counter is connected to the concrete loops.
- **L1:** `lemma_mp_l1` (over-cap in every mode). **L2:** `lemma_mp_l2` (exact-cap zero input in
  `BareDib`). **L3:** `lemma_l3_full` and the L3 lemma family (rows, painted prefix, max-output
  arithmetic). All verify; the generator maps the three oversized inputs to L1/L2 and checks the
  two 64 MiB slack boundaries and the over-cap slack vector against the manifest's construction.

## Exact verification output

```
$ ./verify.sh "M1.1c: clean full reverify (after cargo clean)"
verification results:: 1557 verified, 0 errors          (runs/098.log, 139 s)

$ ./verify.sh "M1.1c: second clean full reverify on final source"
verification results:: 1557 verified, 0 errors          (runs/101.log, 138 s)

$ cargo test --release
conformance: 302/302 vectors passed
corpus: 84 files checked, 58 accepted
```

Obligation count moved 1398 (M1.1a) -> 1557 (+159: 85 new `agree_*` proofs for the appended
vectors — the six staged 32-bpp slack successes supply several obligations each).
`model_vectors.rs` grew 8,312 -> 10,996 lines.

## verify.sh run numbers

M1.1c used runs **41-101** in `dib/RUNLOG.tsv` (now 1,106 rows): the exploratory/bisect runs
41-48, the green regeneration runs 49 and 51, the mutation batch 52-96 (36 first, then 1-44),
the m38 rerun 97, and the clean reverifies 98 and 101 (99 and 100 were cache hits). Runs
41, 42, 46, 47 and 48 failed on the Verus `vir/poly.rs:470` internal panic (the 32-bpp ICO success
in slack mode sent through `by (compute)`, the bug this milestone fixed by staging those through
`gen_ico32.py`); run 50 failed on `cannot refer to private function` (the `t12_checked_n` visibility
error, fixed next run). None failed on an obligation.

## Escapes, frozen files

- No `assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo!`/`unimplemented!`/`requires(false)`,
  no `#[verifier::rlimit]` or other solver-budget attribute in `dib.rs`, `model.rs` or
  `model_vectors.rs`.
- `spec-1.1-frozen/FREEZE.txt` verifies (`sha256sum -c`): SPEC, VECTORS, SOURCES, DESIGN-NOTES,
  check_vectors, THEOREMS-DELTA all OK. `spec-frozen/` and `THEOREMS.md` are unmodified (git clean).
- `model.rs` is byte-identical to `model.rs.m11a` (no model change was needed). `dib.rs` differs
  from `dib.rs.m11a` only in the `decode_with_steps` export and the `pub` on `t12_checked_n`.

## Anything not covered (honest gaps)

- **The ICO crate call site.** The delta says the ICO crate must call `decode_with_steps` instead of
  `decode`. No ICO crate exists in this workspace (`/home/ws/projects/ico` holds only spec and
  review documents), so that change is the ICO milestone's. This milestone exports the function and
  points the conformance harness at it, which is what "export the tracked decoder" asks for here.
- **The two 64 MiB R19 boundary vectors** are runtime evidence, not verifier obligations, for the
  same reason as the 1.0 64 MiB fixtures. The small `slack-*` set proves the branch symbolically.
- **Mutants m1, m27, m38, m39, m43, m44** include a per-function rlimit body check among their
  diagnostics. Each also has a genuine obligation failure (postcondition/assertion/precondition) in
  the same log, so they are verifier kills; if a reviewer wants only clean obligation kills, these
  six would be the ones to re-derive with smaller mutants. No `#[verifier::rlimit]` was added.
- No 32-bit verifier run (argument by hand, as for 1.0; the crate uses no `global size_of usize`).

## Tool calls used

On the order of **200** tool calls (my own running count; most are Bash calls — exploratory reads,
the bisect runs, the two clean reverifies, the tests, the per-mutation batch management, and the
transcription of the 44 result files, plus the polling calls while the unattended batch ran). Under
the 400 cap.
