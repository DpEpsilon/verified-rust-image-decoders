# PROOF-REPORT-M4b — the executable-only mutation campaign (THEOREMS.md, "Planned executable-only mutations")

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), toolchain
1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`, dependency
`dib_verus` (path `../dib/dib_verus`, version 0.3.0, unmodified).
**Pristine base:** `mutation/base/ico_verus`, a fresh copy of the live
`ico_verus/{src,tests,Cargo.toml,Cargo.lock}` (no `target/`), byte-identical to
the live source (`refresh_base.sh` regenerates it). The base copy verifies
**303 verified, 0 errors** (run 127), equal to the live crate.
**Clean full reverify of the live crate** (`runs/162.log`, after `rm -rf
ico_verus/target`):
`verification results:: 303 verified, 0 errors`.
**Conformance:** `cargo test --release -- --nocapture` →
`conformance: 217/217 vectors passed`, `test result: ok. 1 passed; 0 failed`.
**`./verify.sh` runs this milestone:** 128–162 (35 runs). Base green 127; 33
manifest mutations 128–160; the M17 rework 161; the live clean reverify 162.
All but one exited 101 (a verifier diagnostic); m21 exited 0 (audit kill).
**Live source:** unchanged by the campaign (`mutation/base/ico_verus/src/*` still
byte-identical to `ico_verus/src/*`; no `target/` left in any `mutation/mN`
copy; 409,881 free inodes).
**Tool calls:** about 65 (my own count; the harness count is authoritative),
well under the 250 cap.

## Method

`tools/mutations.py` edits **only** `ico_verus/src/ico.rs` in a disposable copy
`mutation/mN/ico_verus` rebuilt from `mutation/base/ico_verus`; the frozen model
`model.rs`, `model_vectors.rs`, `witnesses.rs`, `lib.rs` and `spec-frozen/` are
untouched, so a failure is a verifier diagnostic on the executable obligation the
manifest names, not a corrupted model. `apply_edits` refuses to mutate unless
every anchor matches exactly once; a per-mutation `diff -rq` against base confirms
the only differing file is `src/ico.rs`. `tools/run_mutations.sh` (copied from
`../dib/tools/run_mutations.sh`) runs each copy through `./verify.sh` with
`VERIFY_CRATE=mutation/mN/ico_verus`, keeps `failure.log` and `result.txt`, runs
`tools/audit_alloc.sh` on the mutated source, and deletes the copy's `target/`
and caches. All 34 runs were launched in **one unattended background batch** and
waited on once (a done-file loop); no polling.

**Every failure is a Verus diagnostic, not a rustc error.** A grep over all
`failure.log`s for Rust compile diagnostics (`error[E…`, `mismatched types`,
`cannot borrow`, `no method named`, `not found in this scope`) finds **none**;
each `error: could not compile` line is Verus's own exit-message following a
verification error, with no `[E…]` code. The first diagnostic in each log is a
postcondition / precondition / assertion / loop-invariant failure.

## Result table

Result: **32 verifier kills** (31 of the 33 manifest mutants + the M17 rework),
**1 weak kill** (m17, rlimit-only), **1 audit kill** (m21). No mutation survived
except m21, whose verifier success is the manifest's own honest label.

| # | What changed (src/ico.rs) | Expected theorem / fixture | Result | First verifier diagnostic | Log |
| --- | --- | --- | --- | --- | --- |
| M1 | `directory_guard`: cap `len > MAX_INPUT` → `>=` | T2/T5 · L1 `file-size-67108864` | verifier kill | postcondition not satisfied, `directory_guard` ensures (ico.rs:274) | `mutation/m1/failure.log` |
| M2 | read `reserved` before the `len < 6` guard | T1/T2 · `short-dir-0` | verifier kill | precondition not satisfied, `rd16` call (ico.rs:293) | `mutation/m2/failure.log` |
| M3 | `rd16` → `hi \| (lo << 8)` (big-endian) | T2 · `dib-basic-parse` | verifier kill | postcondition not satisfied, `rd16` ensures (ico.rs:189) | `mutation/m3/failure.log` |
| M4 | `count_word == 0` → `false && …` (accept count 0) | T2 · `empty-directory-1` | verifier kill | assertion failed (ico.rs:367) | `mutation/m4/failure.log` |
| M5 | table guard moved before the count cap | T2/T6 · `count-cap-1025` | verifier kill | postcondition not satisfied, `directory_guard` ensures (ico.rs:274) | `mutation/m5/failure.log` |
| M6 | record stride 15 (`p = 6 + 15*i`) | T2 · `two-dib-png` | verifier kill | assertion failed (ico.rs:613) | `mutation/m6/failure.log` |
| M7 | drop the R5 zero-means-256 rule | T2/T3 · `dimensions-0-0-type1` | verifier kill | assertion failed (ico.rs:640) | `mutation/m7/failure.log` |
| M8 | swap ICO/CUR word roles | T2 · `words-65535-0-type2` | verifier kill | assertion failed (ico.rs:692) | `mutation/m8/failure.log` |
| M9 | abort `parse` on the first invalid record | T2/T5 · `valid-then-bad-reserved`, `bad-then-valid-oob` | verifier kill | postcondition not satisfied, `parse_tracked` ensures (ico.rs:746) | `mutation/m9/failure.log` |
| M10 | zero-size test made conditional on non-overlap | T2/T6 · `zero-size-before-overlap` | verifier kill | assertion failed (ico.rs:538) | `mutation/m10/failure.log` |
| M11 | offset compared with 6 instead of `dir_end` | T2/T3 · `directory-overlap-21` | verifier kill | assertion failed (ico.rs:519) | `mutation/m11/failure.log` |
| M12 | wrapping u32 `(offset + size)` instead of u64 subtraction | T1/T2/T7 · `wide-sum-wrap`, `size-4294967295` | verifier kill | assertion failed (ico.rs:526) | `mutation/m12/failure.log` |
| M13 | length guard `end - start < 8` → `< 7` | T4 · `signature-prefix-7` | verifier kill | invariant not satisfied before loop (ico.rs:423) | `mutation/m13/failure.log` |
| M14 | length guard `< 8` → `< 12` (require bytes after signature) | T4/T5 · `signature-prefix-8`, `png-8` | verifier kill | postcondition not satisfied, `has_png_signature` ensures (ico.rs:405) | `mutation/m14/failure.log` |
| M15 | reject overlapping/nested/straddling valid ranges | T2 · `two-identical-range`, `straddling-ranges` | verifier kill | postcondition not satisfied, `parse_tracked` ensures (ico.rs:746) | `mutation/m15/failure.log` |
| M16 | index/count guard on `index as u8 >= count as u8` | T1/T5/T7 · `index-narrow-256` | verifier kill | assertion failed (ico.rs:1036) | `mutation/m16/failure.log` |
| M17 | DIB slice `bytes[start..]` (offset-to-EOF) | T5 · `entry-size-truncates-dib` | **weak kill (rlimit-only)** | function body check: resource limit exceeded (ico.rs:987) | `mutation/m17/failure.log` |
| M18 | reject DIB output whose dimensions differ from the record | T5 · `dib-mismatch-decode` | verifier kill | postcondition not satisfied, `decode_entry_impl` ensures `md_eq` (ico.rs:996) | `mutation/m18/failure.log` |
| M19 | `has_png_signature` always `false` (PNG judged Dib) | T5/T6 · `two-dib-png-decode1` | verifier kill | postcondition not satisfied, `has_png_signature` ensures (ico.rs:405) | `mutation/m19/failure.log` |
| M20 | charged rescan of all previous records per record | T9 · `count-exact-1024` | verifier kill | invariant not satisfied at end of loop body (ico.rs:810) | `mutation/m20/failure.log` |
| M21 | extra uncharged `vec![0u8;4]` in `decode_entry_impl` | T10 · AUDIT (`dib-basic-parse`) | **audit kill** | verifier 303/0; `audit_alloc.sh` FAIL (`vec! allocations: 1`) | `mutation/m21/failure.log` |
| M22 | drop the short-payload short-circuit (`if false`) | T4 · `png-prefix-followed-by-rest-7` | verifier kill | invariant not satisfied before loop (ico.rs:423) | `mutation/m22/failure.log` |
| M23 | overlap boundary = current record end `(p+16)` | T2/T3 · `table-boundary-record0-off22` | verifier kill | precondition not satisfied, `record_status` call (ico.rs:661) | `mutation/m23/failure.log` |
| M24 | read size/offset as u16 | T2/T7 · `wide-size-10001`, `wide-offset-10016` | verifier kill | assertion failed (ico.rs:646) | `mutation/m24/failure.log` |
| M25 | index/count guard on `index as u16 as u32` | T5/T7 · `index-narrow-65536` | verifier kill | assertion failed (ico.rs:1046) | `mutation/m25/failure.log` |
| M26 | record cursor `6 + 16*(i & 255)` | T2/T5 · `count-exact-1024` | verifier kill | assertion failed (ico.rs:613) | `mutation/m26/failure.log` |
| M27 | sort records by descending `raw_size` | T2 · `directory-order-large-first`, `-small-first` | verifier kill | invariant not satisfied at end of loop body (ico.rs:804) | `mutation/m27/failure.log` |
| M28 | `record_matches = true` (trust the supplied record) | T1/T5 · `forged-wrong-kind`, `forged-wrong-end`, `forged-empty` | verifier kill | assertion failed (ico.rs:1129) | `mutation/m28/failure.log` |
| M29 | return the wire record's defect before the mismatch check | T5/T6 · `wire-invalid-forged-valid`, `-defect` | verifier kill | postcondition not satisfied, `decode_entry_impl` ensures `md_eq` (ico.rs:996) | `mutation/m29/failure.log` |
| M30 | DIB mode `IcoEntryDib` instead of `IcoEntryDibSlack` | T5 · `dib-slack-64-decode` | verifier kill | precondition not satisfied, `lemma_md_eq_dib` call (ico.rs:1198) | `mutation/m30/failure.log` |
| M31 | `record_eq` compares only width and status | T5/T6 · `forged-wrong-height/…-color-count/…-planes/…-bit-count/…-hotspot-*/…-raw-size` | verifier kill | postcondition not satisfied, `record_eq` ensures (ico.rs:946) | `mutation/m31/failure.log` |
| M32 | validate every supplied record, not only the selected | T5/T6 · `forged-unselected-ignored` | verifier kill | postcondition not satisfied, `decode_entry_impl` ensures `md_eq` (ico.rs:996) | `mutation/m32/failure.log` |
| M33 | type guard compares only the low byte (`kind as u8`) | T2/T5/T6 · `type-257`, `type-513` | verifier kill | assertion failed (ico.rs:315) | `mutation/m33/failure.log` |
| M17r | (rework of M17) DIB slice end = EOF, `end = bytes.len()` | T5 · `entry-size-truncates-dib` | verifier kill | precondition not satisfied, `lemma_md_eq_dib` call (ico.rs:1198) | `mutation/m34/failure.log` |

## The weak kill and its rework

**m17** (DIB slice `bytes[start..]`, offset-to-EOF) fails **only** on a
function-body resource-limit diagnostic — `error: function body check: resource
limit (rlimit) exceeded … --> src/ico.rs:987 fn decode_entry_impl` — with no
postcondition, assertion or invariant error in the log. The literal `bytes[start..]`
makes the slice length symbolic (`|b| - start`), and the composition proof then
cannot close within the per-function budget. Per the campaign rule this is a
**weak kill** (not a real obligation failure), so M17 was **reworked** as
**m34**: instead of a symbolic slice length, the mutation fixes the end to a
concrete but wrong value (`let end = bytes.len();`), keeping the slice `[start, |b|)`.
m34 fails on a **real obligation**: `precondition not satisfied` at the
`crate::model::lemma_md_eq_dib(…)` call (ico.rs:1198) — the composition lemma's
`slice == b.subrange(start, end)` premise no longer holds, so the postcondition
`md_eq` cannot be discharged. m34 is a clean verifier kill. Both runs are logged;
m17 stays as recorded (weak kill).

## The audit kill

**m21** adds `let _scratch: Vec<u8> = vec![0u8; 4];` at the top of
`decode_entry_impl`. The verifier still reports **303 verified, 0 errors**
(run 148): the T10 ghost allocation counter cannot observe a physical allocation,
exactly as the manifest's T10 row says ("a Verus-verified ghost count cannot
detect a new uncharged `vec!`, clone, `collect` or formatting call. Source audit
is mandatory"). `tools/audit_alloc.sh` on the mutated source **FAILS**:
`vec! allocations: 1 (expected 0)`. This is reported as an **audit kill**, never
as a verifier kill. On the live source the same script prints `OK`.

## Faithfulness notes

- Each mutation changes the code path the manifest names, on the fixture/theorem
  it names. The `expect` column of `tools/mutations.py` records that mapping for
  every entry; the table above repeats it.
- M6 and M26 both perturb the record cursor but with different mutants: M6 a
  15-byte stride (`two-dib-png`), M26 a stride that resets past record 256
  (`i & 255`, `count-exact-1024`).
- M16 and M25 are the two index-narrowing mutants the manifest names separately
  ("index E before index/count guard or narrow u32 index to u8/u16" and "read
  index as u16/u8 or compare after narrowing"): M16 narrows the comparison, M25
  re-narrows before comparing.
- M31 is one faithful instance of the manifest's "compare only a subset" (it
  compares only width and status); the manifest lists several field-subset
  witnesses and M31 discriminates all of them at once.
- M4 uses `false && count_word == 0` (the sibling campaigns' idiom) rather than
  deleting the arm, so the whole guard chain is untouched and the failure is the
  precise obligation, not an unrelated dead-code change.
- M12 computes the wrap by `((offset+size) as u64) as u32`, not
  `wrapping_add`, so no new API name enters the proof; the failure is the
  arithmetic obligation.

## Sanity checks (the sibling campaign's equivalent)

- Pristine base copy verifies green: `mutation/base/ico_verus` → **303 verified,
  0 errors** (run 127), equal to the live crate.
- `mutation/base/ico_verus/src/{ico,model,model_vectors,witnesses,lib}.rs`,
  `Cargo.toml`, `Cargo.lock`, `tests/conformance.rs` are byte-identical to the
  live source (`cmp -s`, all "same"), before and after the campaign.
- The live crate, reverified clean after the campaign: **303 verified, 0 errors**
  (run 162); `cargo test --release` → **217/217** vectors, 1 passed / 0 failed.
- No `target/` directory remains under any `mutation/mN`; the live `target/` and
  `.verus` are removed; 409,881 free inodes.
- `../dib` unmodified; `spec-frozen/` untouched; no `assume/admit/external_body/
  trusted/unsafe/todo/requires(false)`, no `#[verifier::rlimit]` introduced.

## Vacuity honesty

No mutation survived the verifier except m21, and m21 is the manifest's
designated audit-only case, killed by `tools/audit_alloc.sh`. After the M17
rework there is no rlimit-only kill left: every mutation of the manifest's
planned list is either a real verifier kill (postcondition / precondition /
assertion / loop invariant) or the audit kill. The manifest's exact-mutant list
is covered (33 rows M1–M33); M17 additionally has the reworked variant m34.

## Executable changes

None to the live crate. All mutation edits live only under `mutation/mN/`
(disposable copies). New campaign tooling: `tools/mutations.py`,
`tools/run_mutations.sh`, `mutation/base/ico_verus/`, `mutation/refresh_base.sh`,
`mutation/dib` (a symlink to `../../dib`, so each copy resolves its
`path = "../../dib/dib_verus"` dependency). Per-mutation `failure.log` and
`result.txt` under `mutation/mN/`.

## Deferred

A 32-bit verifier run is not installed (unchanged from M2/M4a). The DIB crate's
own mutation campaign is separate (`../dib/PROOF-REPORT-M4b2.md` etc.); the ICO
campaign mutates only `ico.rs` and never the DIB dependency.
