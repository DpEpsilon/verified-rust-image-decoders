## alloc-delta: guard + `try_*` API landed — green run 230 (2026-11-12)

The fallible-allocation delta for TGA is implemented and green after Sol's
soundness review: **run 230, 975 verified, 0 errors** (run 223's 974 plus the
verified test-only entry added for Sol finding 2). Tests: **169/169**
conformance, **302/302** per profile for each of Strict, Recommended, GdkCompat
and ImageCrateCompat, **28/28** custom frozen vectors, **52,348 differential
comparisons, 0 mismatches**, and the fault-injection suite (**8/8** cases:
per-reservation failure, threshold mode, no-`realloc`, physical-event layout,
drop-without-leak, `PreAlloc`, and every one of the 169 vectors plus 12 corpus
inputs under an injected reservation failure). Corpus counts unchanged (Strict
252/294, Recommended 266/294, GdkCompat 264/294, ImageCrateCompat 259/294;
recovered 0/2/4/2). `tools/audit_alloc.sh --self-test`: OK, 0 `vec!`, exactly 1
`try_zeroed` call site, both guard files hash-pinned, and the scanner's own
positive/negative controls pass. All three no_std targets build with `std`
absent from the linked closure.

### Claim, trust boundary and what is *not* claimed

**975 Verus obligations verified; implementation and source-lint checks run; no
production allocation audit and no proof report.** SPEC.md §8 forbids any
core-level "proved" or "audit shows" claim before the production audit exists,
and none is made here. The trust boundary is: **A-STD** (the guard body's binding
of `Err` to `try_reserve_exact`'s `Err` arm, hash-pinned text), the
**hash-reviewed guard text** (`alloc_guard/mod.rs` at
`018ed17a…22e6`, `alloc_guard/sites.rs` at `58e1fb28…c885`), and the **future
signed allowlist** (not built; `tools/alloc_audit`, the production call-graph
audit, is a step-0 item). `tools/audit_alloc.sh` is a **falsification gate, not
enforcement**: it lists allocating constructs in production exec code and fails
on the ones it recognises; it cannot prove the list is exhaustive.

### What landed

- **`src/alloc_guard/`** (`mod.rs`, `sites.rs`). `mod.rs` is DIB's shipped guard
  text (`dib/alloc-wip-1/alloc_guard/mod.rs`) with **one added line**,
  `use alloc::vec::Vec;`, which the no_std conversion requires (DIB's std prelude
  supplied `Vec`; a no_std crate's does not — confirmed empirically, run 208).
  New SHA-256 `018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6`.
  `sites.rs` carries TGA's `AllocSite { Output }`, `TryError { Format(Error),
  OutOfMemory }`, `name_spec`/`name` (**no** `when_used_as_spec`, matching DIB's
  shipped hazard fix), `fmt_part` and `log_tied`. The cross-core erratum and the
  full four-file record are in `alloc-delta/ERRATA.md` (new file, not covered by
  `FREEZE.txt`): the byte-identical cross-core text now includes that import, and
  a std core adopts it on conversion.
- **One allocation site.** `tga.rs`'s `vec![0u8; 4*n]` is gone. The sole site is
  `alloc_output_guarded`, which calls `alloc_guard::try_zeroed(Output, 4N)` and
  charges the `4N` initialisation to `steps` **only on the `Ok` arm**
  (Sol finding 1). A-TGA-3 holds: the colour map is still a borrowed slice and no
  lookup table is built.
- **Public API: `try_*` only.** `try_decode`, `try_decode_with_steps_logged`,
  `try_decode_with_policy`, test-only `try_decode_custom_policy` (and
  `try_decode_policy_tracked`). The frozen names `decode`, `decode_with_policy`
  and `decode_custom_policy` are gone; `decode_tracked`, `decode_complete_policy`
  and `decode_policy_tracked` are the internal threaded forms. The real TGA
  `try_*` set is corrected against the stale SPEC.md §1.1 row in
  `alloc-delta/ERRATA.md` E2.
- **Spec surface.** `req_tga`/`req_tga_policy` with `g`/`g_clause1..3`,
  `prog`/`prog_concl`, `pre_alloc`, `at_site_log`, and the policy twins
  (`g_policy`, `prog_policy`, `at_site_log_policy`, `pre_alloc_policy`), plus the
  helper lemmas ported from DIB's `alloc-wip-1/dib.rs:3618–3866`. **Sol finding
  3** closed: `g_policy_clause1` and `prog_policy_concl` now carry
  `policy_result(fmt_part(r)) == mp_policy(b, p)` (the recovered component lives
  in the flag clauses), and the named `PreAlloc` corollary
  `pre_alloc_policy_concl` over the returned log is an `ensures` of the policy
  entry points.
- **A-TGA-5's charge bound.** TGA has no policy step counter, so this added
  obligation (not frozen A-TGA-4, which is termination; see
  `alloc-delta/ERRATA.md` E3) is a charge bound. `decode_tracked` proves
  `steps == 4*model_checked_n(b)` exactly on success and **zero charge on the
  `OutOfMemory` arm** (Sol finding 1), and `try_decode_with_steps_logged` lifts it
  to `steps <= 16*(|b| + N)` with `N = model_checked_n(b)`. The policy path keeps
  `decode_policy_tracked`'s `steps <= 16*(|b| + policy_checked_n(b,p))`,
  unchanged.
- **Sol finding 2 closed.** The test-only entry's contract no longer asserts
  `r.1 == recovered_policy(bytes, p)` on error results: that equality holds only
  on `Ok`, both error variants report `false`, and `false` is genuinely different
  from `recovered_policy` after a completed clip or substitution (the counterexample
  — 1x1 24-bit RLE, `ClipPacket`, a repeat count of 2 over `N = 1`, one trailing
  byte, which clips then returns `TrailingBytes` — is a regression test in
  `src/policy_tests.rs`). The contract is stated on the non-`cfg(test)` helper
  `decode_custom_policy_core`, which the verifier checks in every build (hence
  the run-227 count of 975); the `#[cfg(test)]` wrapper `try_decode_custom_policy`
  only forwards it.
- **Strict bridge** `t7_strict_is_1_0` retargeted: runs `try_decode` and
  `try_decode_with_policy(Strict)` and proves agreement on every outcome except
  `OutOfMemory` (where each result names its own `(Output, 4N)`).
- **Tests migrated** to `try_*` with a local `fmt_part` that panics on
  `OutOfMemory` (`tests/conformance.rs`, `src/policy_tests.rs`,
  `examples/policy_corpus.rs`), and `tests/fault_injection.rs` strengthened (Sol
  finding 5): a test-only failing `#[global_allocator]`; **every test takes a
  `Mutex` guard** so the process-global injector and counters are serial despite
  Rust's concurrent tests; `LIVE` is credited only after `System.alloc` succeeds;
  a successful output is dropped with no leak; the threshold mode
  (`T ∈ {1, 4096, 65536, 65537}` vs `4N`); zero `realloc`; the physical
  `Layout` checked against the ghost request (size `>= 4N`, align 1 for `u8`);
  and every one of the 169 vectors plus 12 corpus inputs (read in place, never
  copied into the repo) run under an injected reservation failure, each giving
  exactly `OutOfMemory` or a `PreAlloc` format error that reaches no allocator.
- **`tools/audit_alloc.sh`** rewritten (Sol finding 6) around the new
  `tools/alloc_scan.py`. It globs all `src/**/*.rs` production modules except
  `alloc_guard/` and the `#[cfg(test)]` `policy_tests.rs`, matches `.push(` with
  syntax-aware stripping of proof/ghost/spec regions (so `Seq::push` in a spec
  fn, an `ensures` clause or an `assert` is not flagged while an exec `.push(`
  outside the guard is), fixes `//` handling inside strings, hashes **both**
  `alloc_guard/mod.rs` and `alloc_guard/sites.rs`, requires exactly one
  `try_zeroed` call site and zero `vec!`, and has its own self-test mode
  (`--self-test`) with a negative control (a temporary unguarded exec
  `v.push(x)` must fail).

### Verified obligations

A-TGA-1 (`G` and `Prog`: graceful failure), A-TGA-2 (one request, `k == 4N <=
67,108,864`), A-TGA-3 (no allocation after the reservation), frozen **A-TGA-4**
(termination, `t10_termination`), and the added **A-TGA-5** (the charge bound)
are each carried by verified Verus obligations, on both the 1.0 path and the
policy path, quantified over every `Policy` value via
`decode_custom_policy_core` (and its test-only wrapper). "Verified" here means
the Verus obligations on those functions discharged at run 229; it is **not** a
claim that the delta's assurance prerequisites hold (see "Claim, trust boundary
and what is *not* claimed" above and `alloc-delta/ERRATA.md`). No
`assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo`/`requires(false)`/
`rlimit` in active source. Trust boundary as stated above: A-STD, the
hash-reviewed guard text, and the future signed allowlist.

### Backups / run history

Green backups `src/*.alloc-2` are the run-223 state; `src/*.alloc-f223` are that
same state kept for this review dispatch, and `src/*.alloc-f227` (and the
matching `tests/`/`examples/` copies) are the post-fix sources. Runs 208–230, all
through `./verify.sh`, one at a time. Runs 224 and 225 failed mechanically while
threading the new `(result, flag)` pair through the policy `G` lemmas; runs
226–230 are green, all at 975 verified (226 the first, 227 after the A-TGA-5
comment renames, 228 after the `g_policy_clause1` doc tidy, 229 from a clean
target, 230 after the counterexample wording tidy). The run-207 pre-delta state
is `src/*.alloc-1`.

Test outputs are saved: `notes/alloc-delta-tests-f230.txt` (full `cargo test
--offline -- --nocapture`: 3 unit tests including the finding-2 regression,
169/169 conformance, 8/8 fault injection) and `notes/alloc-delta-corpus-f230.txt`
(corpus 252/266/264/259 of 294).


## alloc-delta: reconnaissance only, no verifier run (2026-11-12)

The dispatch (fallible-allocation delta for TGA) was cut short by a sandbox
suspend request before any edit or verifier run. **The live `tga_verus` tree is
byte-identical to the run-207 green state** (`src/{lib.rs,tga.rs,policy.rs,policy_tests.rs}`
diff-identical to the `*.nostd-1` backups; `model.rs`, `policy_model.rs`,
`model_vectors.rs` are the run-205 `*.pol-c1` sources, unchanged by the no_std
step). No new run number, no counts change: the current green state is still
**run 207: 947 verified, 0 errors; 169/169 conformance; four profiles 302/302;
52,348 oracle differential zero mismatches; corpus 252/266/264/259; audit_alloc OK
(1 `vec!`)**. Nothing under `tga/spec-frozen/`, `tga/THEOREMS.md` or
`tga/policy/{POLICY-SPEC.md,THEOREMS-POLICY-DELTA.md,...}` was touched. No
`alloc-wip-*` directory was created (there is nothing to restore). No target
directory and no VERIFY_CRATE copy exist.

What the reconnaissance established for the next dispatch, so it is not redone:

- **Guard text is available and pinned.** DIB's production guard
  `dib/alloc-wip-1/alloc_guard/{mod.rs,sites.rs}` is the verified reference
  (`dib_verus/src/alloc_guard/mod.rs` is the same file: SHA-256
  `5d145f45adcf8d1c5ab398f2ac2ec0d50eb3bcbd54a542b277d7b86a41ae33b1`). TGA copies
  `mod.rs` byte-identically and adapts `sites.rs`: one `AllocSite` variant
  `Output`, `TryError { Format(Error), OutOfMemory(OutOfMemory) }` over TGA's
  `Error`, and `fmt_part` / `log_tied` with `super::super::tga::{Decoded, Error}`.
  TGA's `Error::name` (tga.rs:67) needs a `name_spec` sibling and
  `#[verifier::when_used_as_spec(name_spec)]` added, exactly as DIB did
  (`alloc-wip-1/dib.rs:88,120`); the prototype's hazard note (avoid
  `when_used_as_spec` on an unrelated fn) is about a different function, not
  about `Error::name`.
- **The one site is `alloc_output` (tga.rs:8208, `vec![0u8; 4*n]`), the sole
  `vec!`.** It is called once from 1.0 `decode` (tga.rs:2571) and once from
  `decode_complete_policy` (tga.rs:8362). Both must thread `Tracked(log)` down
  from a `new_log()` at the entry point and charge the `4N` initialisation to
  `steps` only on the `Ok` arm, mirroring DIB's `alloc_output`
  (`alloc-wip-1/dib.rs:3878`).
- **DIB's alloc-delta proof skeleton to port**: `req_dib` / `g_clause1..3` / `g`
  / `prog` / `prog_concl` / `pre_alloc` / `at_site_log` plus the lemmas
  `lemma_at_site_log_{ok,oom,prealloc,parts}` (`alloc-wip-1/dib.rs:3618–3866`).
  TGA's analogues are `req_tga(b) = if mh(b) is Ok(m) { [(Output, 4*m.n)] } else []`,
  `g_clause3` from `model::lemma_mh_ok_bounds` (`model.rs:315`), and the policy
  side from `policy_model::lemma_mh_policy_ok_bounds` (`policy_model.rs:3864`).
  TGA has no step counter, so A-TGA-4's terminal obligation is a charge bound,
  not `16*(|b|+N)`.
- **Frozen names removed, witnesses retargeted**: `decode` (1.0, tga.rs:2476),
  `decode_with_policy` (:8408), `decode_custom_policy` (:8637, `cfg(test)`) → only
  `try_*` exported. Witnesses using the old names are `t9_precedence` (:4574),
  `t7_decode_exact` (:4593), `t10_termination`'s `decode_rle`/`decode_raw` calls
  (kept — internal helpers), `t5_header_exact`/`t2_header_bounds`
  (:4891/:4912, header helpers, kept), `t1_decode_safe` (:4937), and the policy
  witnesses `t7_strict_is_1_0` (:8426), `t2_t5_policy_header` (:8452),
  `t1_t8_t10_w_a_e_policy` (:8579), `t7_t9_policy_exact` (:8597),
  `r_recommended_conservative` (:8606), `e_policy_no_events` (:8623). `t1_decode_safe`
  and `t7_decode_exact` call `decode`; `t7_strict_is_1_0` calls both `decode` and
  `decode_with_policy`.
- **Tests to migrate**: `tests/conformance.rs:13,85` (`use ...{decode, Error}`,
  `decode(&bytes)`), `src/policy_tests.rs:5,105,113,131,144,146`
  (`decode`, `decode_custom_policy`, `decode_with_policy`),
  `examples/policy_corpus.rs:8,154` (`decode_with_policy`). DIB's pattern is a
  local `fmt_part` that panics on `OutOfMemory` (`alloc-wip-1/conformance.rs:20`,
  `dib_verus/src/policy_tests.rs:18`). `policy_tests.rs:146`'s
  `assert_eq!(decode(&bytes), result.0)` becomes
  `assert_eq!(fmt_part(try_decode(&bytes)), fmt_part(strict_result.0))`.
- **Audit extension**: `tools/audit_alloc.sh` currently expects exactly 1 `vec!`
  and flags no `handle_alloc_error`/`process::abort`. The delta needs: 0 `vec!`,
  exactly 1 `try_zeroed` call site, `handle_alloc_error`, `process::abort`,
  `try_reserve` and the `Vec::*`/`.push(`/`.resize(` patterns as findings, and an
  `alloc_guard` exemption. The existing script's `Vec::[a-z_]+` regex would flag
  the guard's `Vec::new`/`try_reserve_exact` (`alloc_guard/mod.rs:130,132,140`),
  so the guard text must be excluded from the scan.
- **no_std**: `tga_verus` is `#![no_std]` + `extern crate alloc;` with
  `vstd = { default-features = false, features = ["alloc"] }`. `tga.rs:22-23`
  already has `use alloc::vec; use alloc::vec::Vec;`. The guard's `Vec` name
  resolves through `vstd::prelude::*` only under `verus_keep_ghost`/`alloc`
  (vstd prelude re-exports `Vec` via `super::std_specs::vec`); confirm on the
  first build, and add `use alloc::vec::Vec;` to the guard module if not.

**Precise next step for the resuming dispatch**: copy `dib/alloc-wip-1/alloc_guard/mod.rs`
byte-identically to `tga_verus/src/alloc_guard/mod.rs`; write TGA's
`sites.rs`; add `name_spec` + `when_used_as_spec` to `Error::name`; add
`alloc_guard` to `lib.rs`; port DIB's `req_tga`/`g`/`prog`/`pre_alloc`/`at_site_log`
and its lemmas; convert `alloc_output` to `try_zeroed` and thread the log
through `decode`/`decode_policy_tracked`; export `try_decode`,
`try_decode_with_policy`, `try_decode_with_steps_logged`, and test-only
`try_decode_custom_policy`; retarget the witnesses and tests; run
`./verify.sh "alloc-delta: guard + try_decode"` once. Expect the first run to
fail on mechanical items (log threading in `decode_complete_policy`, the
`PreAlloc` corollary on the header-error exit, `t7_strict_is_1_0`'s two results
now carrying `TryError`); DIB's `decode_tracked` exit-by-exit proof
(`alloc-wip-1/dib.rs:8106–8204`) is the worked template for each.

## Strict-to-1.0 full bridge — run 205 (2026-11-12)

The model theorem `policy_model::lemma_mp_policy_strict_is_mp(b)` proves, for every finite byte sequence, `mp_policy(b, strict_policy()) == model::mp(b)` as the exact `Result`: the same first error, or the same dimensions and every oriented RGBA byte. This closes Sol finding 1 and transfers the frozen 1.0 T7 to Strict. The verified public executable witness `tga::t7_strict_is_1_0(bytes)` calls both `decode_with_policy(bytes, Strict)` and `decode(bytes)` and proves that their results agree and that the Strict `recovered` flag is false.

The proof follows R14's header, ID, map, pixel, suffix and assembly order. The packet-field bridge uses the successful 1.0 header's actual fields; the raw induction compares the first error before conversion (so it needs no all-samples-in-range precondition); the RLE induction compares API error or the full successful tuple (ghost partial pixels on error are irrelevant). On success, the existing cursor/pixel projections and a raw-sample extensionality proof equate the separate assembly pixel sequences. The phase argument needs no `rlimit` attribute or proof escape. Runs 201–205 were sequential through `./verify.sh`; final run 205: **947 verified, 0 errors**, up from run 199's 936. Backups `src/{model.rs,policy_model.rs,tga.rs}.pol-c1` capture the verified sources; frozen files remain unchanged.

## Errata against frozen POLICY-SPEC (2026-11-12)

Recorded, not edited: no frozen file (`spec-frozen/*`, `THEOREMS.md`,
`policy/{POLICY-SPEC.md,THEOREMS-POLICY-DELTA.md,policy_oracle.py,
gen_policy_vectors.py,POLICY-VECTORS.json,QUESTIONS.md}`) was touched. The full
entry with the revision text is the new file `policy/ERRATA.md` (not covered by
`FREEZE.txt`). Summary:

**E1 — 8-bit greyscale, descriptor attribute count 8.** `POLICY-SPEC.md:24`
says `ImageTable` "8-bit grey allows {0, 8} (8 is treated like 0)". The oracle
(`policy_oracle.py:59-61,153-162`), the model (`policy_model.rs:285-335,1085-1087`)
and the executable (`tga.rs:5148-5165`) all retain effective count 8 rather than
0. There is no public-output difference: 8-bit grey is `kind = 2`, `depth = 8`,
`sample_bytes = 1`, so `conv_sample_policy` writes R=G=B=sample and alpha 255
without reading an attribute byte, and the stored-alpha rules consult `eff` only
for 32-bit true-colour, 32-bit palette entries and 16-bit grey. The controlling
text is line 24's own acceptance sentence ("allows {0, 8}") plus the three
consistent layers; the parenthetical "(8 is treated like 0)" is the defect. A
revision should strike that parenthetical and state that an accepted count is
kept as the effective count. Separately, `THEOREMS-POLICY-DELTA.md:9-12`
describes `eff` as a new header-record field; `MHModel` has no such field and the
effective count is derived from the record's own fields (`eff_count_img`), carried
only in the conversion projection `PktH.attr`; a revision should say so.

## Astra policy completion — final clean run 194 (2026-11-12)

The complete policy executable and its witnesses verify: **927 verified, 0
errors**, from a clean target directory in `runs/194.log`. Runs 179–194 contain
15 verifier invocations (8 successful, 7 failed); there is no run 192 because
`verify.sh` counts directory entries and the retained test log
`runs/policy-tests-astra-191.txt` occupied that count. Every invocation used
`verify.sh` with `CARGO_NET_OFFLINE=true`, one verifier at a time.

- `rle_packet_policy`, its repeat/literal helpers and `decode_rle_policy` are
  proved, including MR's full success/error tuple and partial oriented pixels.
- One-layer literal unfold lemmas and scalar stride identities close the failed
  prefix inductions; the literal error branch now returns its current ghost
  accumulator rather than its entry accumulator.
- `decode_policy_tracked` has no preconditions and proves the exact policy MP
  result plus `steps <= 16*(|bytes|+N)` for every Policy value and every outcome;
  N is the validated header's pixel count, or zero on a header error.
- The charged work counts capped header inspection, 4N output initialization,
  sample attempts, four output-byte writes per painted pixel, packet dispatch
  and capped suffix inspection; neither clipping mode loops over discarded
  positions.
- `decode_with_policy(bytes, profile)` returns `(Result<Decoded, Error>, bool)`;
  every successful flag equals the closed clip/substitution model, and every
  error returns false with fresh state on the next call.
- Universal witnesses cover header/map addressing, conversion, packet safety,
  bounds, termination, work, allocation-call counts and event exactness; named
  profiles have exact T5–T7/T9 witnesses and executable Recommended
  conservativity (`r_recommended_conservative`).
- Ten small frozen-vector header witnesses extend the existing
  `model_vectors.rs`; regenerate them with `tools/gen_policy_header_witnesses.py`
  after regenerating the original core vector module.

`alloc_output` contains the sole `vec![0u8; 4*n]` site. The only executable edit
to the 1.0 `decode` body replaces that same expression with the shared helper
call; its signature, decoding logic and exact-model contract are unchanged.
`signature_tail` gained an explicit representable-length invariant. The ghost
allocation counter proves one call after a successful header and zero after a
header error; `tools/audit_alloc.sh` independently checks that no other
input-dependent allocation construct appears. This is not a physical allocator
trace or an allocation-failure theorem; the separate allocation delta remains
future work.

The final test command was `cargo --offline test --release -- --nocapture
--test-threads=1` (full output `notes/policy-tests-final.txt`): 302/302 policy
vectors for each of Strict, Recommended, GdkCompat and ImageCrateCompat; 28/28
custom frozen vectors; 169/169 original core vectors; and **52,348 oracle
comparisons, zero mismatches**. The differential test includes all 169 original
inputs under every profile, all 7,578 proper prefixes per profile, all 32 option
values on 398 inputs each, coupled palette/header/RLE groups and 512 generated
all-field records. Arbitrary executable records and their decoder entry exist
only under `cfg(test)`.

Corpus measurement (`notes/policy-corpus-astra.txt`) used
`/home/ws/corpora/tga/MANIFEST.tsv`, SHA-256
`0f8a8a41ea00caf25d480cb4f9ca9bc1c30572007f6e880e9ca2876e2520584c`.
The manifest was unchanged across the run; all 294 listed files were present
and independently SHA-256-matched. Acceptance: Strict **252/294**, Recommended
**266/294**, GdkCompat **264/294**, ImageCrateCompat **259/294**. Successful
recovery flags: 0, 2, 4 and 2 respectively. The runner reads corpus bytes in
place; no corpus bytes were copied into the project.

All six policy freeze hashes and all six referenced 1.0 freeze hashes match
(the historical `check_vectors.py` is in `spec-history/spec`, not
`spec-frozen`). The source audit reports one allocation site. The inherited
M3 `#[verifier::rlimit(80)]` in `model.rs` was still present despite the earlier
handoff's claim; it was removed, and runs 193/194 prove that lemma at the
default budget. No proof escapes or budget attributes remain in active source.
Green backups: `src/*.pol-a7` is the final source; earlier `.pol-a0` through
`.pol-a6` retain the verified checkpoints. No Git state-changing command was
run. The target directory is removed after the final checks.

The earlier status entries below are historical; their outstanding policy
items are closed by this section.

## Sol policy RLE attempt — stalled (runs 173–178)

The 1.0 and policy-model source is green at run 178: **865 verified, 0 errors**.
The executable policy RLE packet draft from `.pol-rc1` failed five consecutive
verifier runs (173–177); its updated, still-unproved version is saved as
`src/{tga.rs,policy_model.rs}.pol-s1`, and the real source was restored from
`src/tga.rs.pol-s0` and `src/policy_model.rs.pol-w5`. `STUCK.md` gives the
remaining literal-prefix and slice-composition failures and the run-177
messages. No `decode_rle_policy` was present in the recovered draft, and
`decode_policy_tracked`, the public policy decoder, W/executable witnesses and
policy conformance tests remain unimplemented. Do not count the 883 verified
obligations in failed run 177 as delivered.

The final full verification is run 178 (`865 verified, 0 errors`). The existing
169-vector 1.0 conformance test passes (`cargo --offline test --test
conformance`). `tools/audit_alloc.sh` reports one `vec!`; all six policy frozen
hashes match `policy/FREEZE.txt`. `/home/ws/corpora/tga/files` exists with 296
files, but per-profile corpus acceptance was not measured because there is no
public policy decoder; no policy fixture count is claimed. The five-failure
escalation rule stops work on this obligation here.

# Notes

T6 (exact packet semantics), T7 (total functional correctness), T8
(success/resource bounds per pixel), T9 (precedence) and T11 (integrated call
path) of `THEOREMS.md` are proved, together with the complete total reference
model `MP` and the packet-level `MR` it needs. See `PROOF-REPORT-M4a.md` (and
`PROOF-REPORT-M4c.md` for the M4c strengthening) for the theorem-by-theorem
account, the verifier line, the run count and the honesty caveats.

## Done (M1, unchanged)

- `spec-frozen/VECTORS.json` and the other frozen files: hashes unchanged.
- `cargo test --test conformance` passes 169/169.

## Done (M2, unchanged)

- T1, T2, T5 for the header path (M2's report).

## Done (M3, unchanged)

- T3, T4, T10 and the orientation bijection, colour-conversion and packet
  repetition lemmas (M3's report). The executable is byte-identical.

## Done (M4a)

`tga_verus/src/model.rs` (5,870 lines): the M3 model plus `MP` and its lemmas,
all written from the spec text independently of `crate::tga`.

- `MP(b)` (THEOREMS.md "Total reference models"): `mh` first, then the ID and
  map slices (`mp_id_end`, `mp_map_end`), R10's raw phase (`mp_raw`, threads a
  logical sample index `k`), R11's packet stream (`mr`, extended to carry the
  ghost error cursor/produced/pixels), R12's suffix test (`footer`,
  `mp_suffix_ok`), and R9/R8's output assembly (`mp_pixels`, `orient_bytes`,
  `px_byte`). `mp_assemble` places decoded pixel `j` at the oriented cell
  `fcoord(j)`; `mp_byte` reads cell `i/4` as the conversion of the source sample
  `gcoord(i/4)`, the orientation bijection's inverse.
- The §"Wait" that mattered: the output cannot be assembled from the *input*
  bytes for RLE (packets repeat and reorder samples), so assembly is defined
  from the decoded pixel sequence (`mr_pix` / `acc_conv`), not from `b[c0 + 4k]`.
- `mr_cur` and `mr_pix`: the cursor/produced/error projection of `MR` and its
  pixel sequence, with `lemma_mr_cur_pix_proj` proving the two agree with `mr_at`
  for every starting state and accumulator. Error-branch lemmas
  (`lemma_mr_cur_ctrl_absent`, `_overrun`, `_rep_missing`, `_rep_index`,
  `_lit_missing`, `_lit_index`) fix the control/overrun/whole-packet/index order.
- The raw phase: `mp_raw_acc`, `mp_raw_cur`, `acc_conv`, and first-error lemmas
  (`lemma_mp_raw_first_trunc`, `lemma_mp_raw_first_index`) that carry the exact
  ghost cursor/produced at the failing sample.
- The output assembly: `wrote_px`, `wrote_samples`, `rle_quad_ok`,
  `lemma_wrote_px_write_step`, `lemma_quad_unchanged`,
  `lemma_quad_ok_to_orient`, `lemma_orient_bytes_index`, `lemma_div4_lt`.
- `MP`'s result lemmas: `lemma_mp_rle_ok`, `lemma_mp_raw_ok_result`,
  `lemma_mp_rle_err`, `lemma_mp_raw_err_result`, `lemma_mp_trunc_id`,
  `lemma_mp_trunc_map`, `lemma_mp_trailing_rle`, `lemma_mp_trailing_raw`, plus
  the suffix lemmas `lemma_suffix_tail`, `lemma_mp_suffix_{zero,footer,short,bad}`.

`tga_verus/src/tga.rs` (4,173 lines): the executable is byte-identical as
behaviour aside from one added ghost parameter.

- `decode_raw` and `decode_rle` gained an executable `c0g: usize` entry-cursor
  parameter (always the call-site cursor; it exists so the success postcondition
  can name the function-entry cursor, which `old()` cannot inside a loop body).
- `decode_raw` carries `raw_result_eq`, `wrote_samples` and, on success,
  `raw_prefix_ok`: exact `MP` raw phase.
- `decode_rle` carries `rle_full_eq` (`rle_result_eq` + `rle_pixels_eq`) and, as
  loop invariants, the model's own cursor recursion `mr_cur(c0g,0) = mr_cur(cur,
  produced)`, the pixel split `mr_pix(c0g,0) = acc + mr_pix(cur, produced)`, and
  `wrote_px(out, acc, ...)`.
- `decode`'s postcondition is `decode_eq(r, bytes@)`, i.e. `decode(b) = MP(b)` as
  the exact `Result`. It proves each exit against the matching `MP` lemma.
- Witnesses `t6_rle_exact`, `t7_decode_exact`, `t8_output_cells`,
  `t9_precedence` carry the manifest's exact T6/T7/T8/T9 claims.

## Decisions

- `MP` is stated over the model record `MHModel`, with `pkt_of_mh` built from the
  record's own fields; the executable links through the proved `pkt_of` bridge
  (`lemma_pkt_of_hdr_matches`), never an axiom.
- `MR` returns `Err((e, cursor', produced', pixels))`, the ghost error state the
  manifest names as "proof observations, not additional public API result
  fields". `rle_result_eq` compares only the error variant (and, on success, the
  final cursor and produced count); the pixel sequence is compared by
  `rle_pixels_eq`.
- The raw model threads a logical sample index `k` rather than an absolute
  cursor, so `decreases N - k` is syntactic and the absolute cursor is a derived
  quantity.

## Process

Every verifier run went through `./verify.sh`; no probe crate, no direct `cargo
verus verify`. Runs this milestone: 112-255 (144 runs, 24 exited 0). The final
full reverify after `cargo clean` is run 254 (`373 verified, 0 errors`, 79 s);
`runs/254.log` also contains the 2,159-obligation count of the
`--profile`-style pass Verus emits first.

## Not done

Nothing. M4b was the last milestone; every obligation of `THEOREMS.md`'s
"Non-vacuity and compact vector protocol" is discharged (see
`PROOF-REPORT-M4b.md`).

## Done (M4b)

- `tga_verus/src/model.rs` (6,800 lines): L1-L4 exactly as the manifest states
  them, each with its named instantiation. `lemma_l1_oversized` (`forall b,
  |b|>67,108,864`), `lemma_l2_at_limit` (`|b|=67,108,864 and b[2]=0`),
  `lemma_l3_output_at_64m_8192x2048` and
  `lemma_l4_output_at_64m_4096_square`. L3/L4 need no 64 MiB evaluation:
  `lemma_packet_step_periodic` lifts one `compute`-decided packet to all 131,072
  repetitions, `lemma_packet_repetition_mr` (M4a) sums them, and
  `lemma_orient_const` turns the constant decoded pixel into the single R16 run.
- `tga_verus/src/model_vectors.rs` (generated, 2,954 lines): 165 model-vs-vector
  obligations, one per small vector, each `MP(b_v()) == <fixture expected>`
  discharged `by (compute)`. Registered in `lib.rs` as `pub mod model_vectors;`.
- `tools/gen_model_vectors.py` emits that file from `spec-frozen/VECTORS.json`.
  It reads expected values only from the fixture's `expected` field, and
  `check_large` refuses to run unless each large vector's expanded bytes equal
  the manifest's symbolic construction *and* its expected result equals the
  manifest's stated result.
- `tools/mutations.py`, `tools/run_mutations.sh`, `tools/mutation_summary.py`,
  `tools/mutation_table.py` and `tools/check_anchors.py`: the twenty mutations,
  each applied to a disposable copy under `mutation/mN/` rebuilt from a pristine
  `mutation/base/`, verified through `VERIFY_CRATE=mutation/mN/tga_verus
  ./verify.sh`, with the failing log kept as `mutation/mN/failure.log` and the
  outcome in `mutation/mN/result.txt`. All twenty fail verification; none is a
  vacuity finding. Regenerable `target/` caches are deleted from each copy after
  its run.

## Process (M4b)

Every verifier run went through `./verify.sh`; no probe crate, no direct `cargo
verus verify`. Runs this milestone: 256-296 (40 logged runs, 6 exited 0). The
final full reverify after `cargo clean` is run 296 (`578 verified, 0 errors`,
82 s); `cargo test` reports `conformance: 169/169 vectors passed`. Run 278 has a
log but no `RUNLOG.tsv` row: it was killed mid-verify and never reached its
append; mutation 9's definitive run is 280.

- `QUESTIONS.md` remains empty: the spec read literally and every vector matched.

## M4e (T6 failure clause closed)

- `decode_rle` now takes `produced: &mut usize` (entry value `p0g`, the
  manifest's `produced0`); its postcondition `rle_exact_from` is T6 stated on
  `MR`'s own result tuple: on `MR = Ok(c',p',px)` the helper returned `Ok`,
  `*final(cursor) = c'`, `*final(produced) = p' = N`, and `px` sits at the
  oriented cells `f(p0g + j)` of `final(out)`; on `MR = Err(e,c',p',px)` the
  helper returned exactly `e`, `*final(cursor) = c'`, `*final(produced) = p'`,
  and the partial `px` sits at its oriented cells of `final(out)`. The older
  projections (`rle_result_eq_from`, `rle_pixels_eq_from`, `rle_full_eq_from`,
  the `mr_gcur` clause) are kept and `decode`'s proof is unchanged.
- The helper's domain is every finite stream with `*cursor <= |bytes|` and
  `p0g <= N`: the `bytes.len() <= MAX_INPUT` and `>= 18` premises are gone
  (nothing in the RLE proof needed them; every packet is at most 513 bytes).
  `t6_rle_exact(bytes, h, map, indexed, out, cursor, produced, n)` quantifies
  over `*old(cursor)` and `*old(produced)` with no cap.
- Structure: `decode_rle`'s loop body is one call to `rle_packet` (control
  presence, overrun, dispatch), which calls `rle_repeat_packet` or
  `rle_literal_packet`. Each returns `(Result, Ghost<Seq<Seq<u8>>>)` (the ghost
  pixel accumulator) and ensures `pkt_post` (the model step: `mr_cur`/`mr_gcur`
  carried on `Ok`; on `Err(e)` the model fails with `e`, ghost cursor = final
  cursor, accumulated pixels = the run's `mr_pix`) and `pkt_out` (the pixels
  written at their cells). Every exit is one lemma call (`lemma_pkt_*`), and
  every `decode_rle` exit is `lemma_rle_post_from_state`. `model::lemma_mr_gcur_proj`
  (new) ties `mr_gcur` to `mr_at`'s error-tuple cursor.
- Lesson: Verus's isolated loop bodies see neither the function's `requires`
  nor the definitions of unmodified locals; the sixth exit (literal packet,
  invalid later index) needed `width == h.width` in the inner-loop invariant
  and the packet start `(cur0, pg0)` as ghost parameters rather than ghost
  locals. No budget attribute; the single M3 `rlimit(80)` is unchanged.
- The M4c exit-lemma predicates `rle_fail_eq_from`, `rle_ghost_eq_from` and
  `lemma_rle_fail_from_split` are removed (superseded by `rle_exact_from`).
- Runs 472-476; clean full reverify after `cargo clean` is run 476
  (`642 verified, 0 errors`, 91 s); `cargo test`: `conformance: 169/169 vectors
  passed`. `mutation/base2` refreshed to this source and the 15 executable
  mutations rerun (anchors of 6, 7, 15 follow the new layout; the mutations
  are unchanged); previous outcome files kept as `mutation/exec-mN.m4d/`.
  `src/*.m4d` are the M4d-state backups.

## M4c

- **Finding 6 (x/y origin).** `MHModel` and the executable `Header` now carry
  R1's `x_origin = LE16(8)` and `y_origin = LE16(10)`, and `hdr_matches` relates
  them. They are metadata only (R1: "Coordinates are metadata, not an extra crop
  or offset"); R9's flips come from descriptor bits 4/5, so no pixel changes.
- **Finding 3 (T8 projection).** `model::lemma_mp_accepted_shape` proves
  `MP(b) = Ok(w,h,out) ==> mp_ok_shape(b,w,h,out)`, so `mp_ok_shape` is a
  projection of `MP`'s own success, not a premise. `model::t8_cell` states the
  per-cell claim over `MP(b) = Ok(w,h,out)`; `model::lemma_mp_t8_cell` proves it
  for every accepted `b` and every `q < N`; `tga::t8_output_cells` takes the
  `MP(b) = Ok(...)` hypothesis, and `t7_decode_exact` (the public result proof)
  asserts `t8_cell` for every cell, so the witness is not orphaned.
- **Finding 4 (OutsideDomain).** `MR`'s model result is a distinct type
  `model::MrErr { Api(Error), Outside }`: a reachable call yields exactly one of
  R15's 18 errors (`Api`), and a call on other records/offsets yields `Outside`,
  the internal proof marker (THEOREMS.md:13), never an executable `Error`. The
  executable errors are unchanged; `MP` never observes `Outside` (its calls are
  in-domain) and the `mp_after` `Outside` arm is documented as unreachable.
- **Finding 1 (T6 at nonzero produced0).** `decode_rle` gained an entry produced
  count `p0g`; its postconditions are `rle_result_eq_from` / `rle_pixels_eq_from`
  / `rle_full_eq_from` (the `MR(s,H,map,cursor0,produced0)` projections), and
  `t6_rle_exact` takes `p0: usize` with `p0 <= n`. The output relation is
  `model::rle_quad_ok_off` / `wrote_px_off`, the `produced0`-offset forms of the
  oriented write invariant. On failure the exact first error is `MR`'s
  (`rle_result_eq_from`) and the concrete final cursor equals `MR`'s ghost cursor
  (`model::mr_gcur`, maintained as a loop invariant with the per-branch step
  lemmas `lemma_mr_gcur_step_rep`/`_lit`/`_lit_err`). The partial ghost pixels on
  failure were carried by `tga::rle_fail_eq_from`, an exit lemma at five of the
  six error exits, not a clause of `decode_rle`'s postcondition (see
  PROOF-REPORT-M4c/M4d for the residual). Closed in M4e: see above.
- **Findings 2 and 7 (mutations).** The 13 model-only mutations are kept as
  model-sensitivity evidence under `mutation/model-side/`; each has a new
  executable-side mutation under `mutation/exec-mN/` (rebuilt from
  `mutation/base2/`, editing only `src/tga.rs`), 15 in all, each failing on a
  verifier diagnostic. `tools/mutations_exec.py` and
  `tools/run_exec_mutations.sh` drive them.
- **Finding 5 (no 32-bit run).** Only `x86_64-unknown-linux-gnu` is installed, so
  no 32-bit verifier run exists. The proof is width-independent because every
  bound it uses is far below `2^31`: R2 caps each dimension at 8,192 so
  `N <= 16,777,216` and `4*N <= 67,108,864`; R4 caps the map at `262,140` bytes;
  R1 caps the input at 67,108,864 bytes; and every cursor is at most `|b|`
  (`<= 67,108,864`). All are below `isize::MAX` and `usize::MAX` on the 32-bit
  target, and Verus checks the same `usize`/`isize` arithmetic, so a 32-bit run
  would discharge the same obligations.

## usize width (coordinator note, 2026-11-09)
Checked on a standalone Verus file with the same toolchain (0.2026.11.02): `assert(usize::MAX == 0xffff_ffff_ffff_ffff)` FAILS while `assert(usize::MAX >= 0xffff_ffff)` verifies, i.e. Verus treats `usize` as a 32-or-64-bit type by default unless a crate declares `global size_of usize == 8` (none of the crates here do: grep for `global size_of`/`size_of usize` is empty). The proofs therefore hold for both widths in the sense of the manifest's 'parameterised by a word width'. This addresses the 'no 32-bit verifier run' review finding as a width-generic proof; it is not an actual i686 build or test run.

## TGA policy (POLICY-SPEC.md / THEOREMS-POLICY-DELTA.md)

The frozen policy layer. New crate modules `src/policy.rs` (the 13 option enums,
the `Policy` record, the four named profile records, the `Profile` selector, the
`pub(crate)` constructors) and `src/policy_model.rs` (the policy-indexed models
and their lemmas). `src/tga.rs`, `src/model.rs` and `src/model_vectors.rs` are
byte-identical to the 642-obligation 1.0 state (the `decode` arity and result
type are unchanged); `src/lib.rs` gains the two modules.

### Stage 1 — policy-indexed models with Strict as the definitional instance (DONE)

- `policy.rs`: 13 `#[non_exhaustive]` option enums with Strict's value first;
  `Policy` with `pub` fields (Verus needs them public for the model's spec fns to
  be well-formed) but `#[non_exhaustive]` and `pub(crate)` constructors
  `strict_policy_exec`/`profile_policy_exec`, so no external literal or mutation;
  `Profile {Strict, Recommended}` (`#[non_exhaustive]`, `Default = Strict`); the
  spec records `strict_policy`, `recommended_policy`, `gdk_compat_policy`,
  `image_crate_compat_policy` and `profile_policy`. The two compat records are
  present (frozen field values) but the compat `Profile` variants are added when
  their nine pixel-rule fields are implemented.
- `policy_model.rs`: `mh_policy` (R1–R7 with the option branches), `conv2_policy`
  (`conv_sample_policy`/`conv_map_policy` with `rgb5`, `map5`, `alpha`,
  `map16_alpha`, `map_index`, `index_range`), and the packet/raw/full model
  `mr_at_policy`, `mr_pix_policy`, `mp_raw_acc_policy`, `mp_pixels_policy`,
  `mp_assemble_policy`, `mp_after_policy`, `mp_policy`, `recovered_policy`. Also
  `map_bytes_policy`, `attr_ok_policy`, `eff_attr`, `pkt_of_mh_policy`,
  `image_index_depth_bad`.
- Strict is the definitional instance: `lemma_mh_policy_strict`,
  `lemma_conv2_policy_strict`, `lemma_lit_{ok,good,out}_policy_strict`,
  `lemma_mp_raw_acc_policy_strict`, plus the per-check bridges
  (`lemma_{type_ok,map,depth_ok,map_coherence,attr_ok,map_bytes}_policy_strict`,
  `lemma_in_range_policy_strict`) — each a definitional reduction at
  `strict_policy()`, no existing theorem weakened.
- `policy_strict` bridge for the packet stream is stated on the 1.0 domain
  (every sample's index in range): `lemma_mp_raw_acc_policy_strict` takes that
  as a hypothesis, split into a small step lemma to stay inside the default
  solver budget (no `#[verifier::rlimit]`).
- Real-crate run 025: **698 verified, 0 errors** (was 642); `cargo test --test
  conformance` 169/169; `spec-frozen/*` and `policy/{POLICY-SPEC.md,
  THEOREMS-POLICY-DELTA.md, policy_oracle.py, gen_policy_vectors.py,
  POLICY-VECTORS.json, QUESTIONS.md}` hashes match `policy/FREEZE.txt`.

### Stage 2/3 — model-level obligations E and R (E DONE; R partial)

- Final delivered state: real-crate run 040, **722 verified, 0 errors**; `cargo
  test --test conformance` 169/169; `tools/audit_alloc.sh` OK (one `vec!`); the
  frozen policy and spec-frozen hashes all match.

- **E (flag exactness)** DONE. `recovered_policy` is *defined* as the closed
  two-event disjunction (clip bit OR substitution bit), so E's "iff" is by
  construction. The named corollaries are proved: `lemma_pkt_clip_reject`,
  `lemma_pkt_subst_reject`, `lemma_mr_flags_no_events`,
  `lemma_lit_flags_no_events`, `lemma_raw_subst_no_events`,
  `lemma_recovered_policy_reject`, `lemma_recovered_policy_strict` (every record
  with `rle_overrun=Reject` and `index_range=Reject`, Strict included, has
  `recovered=false`). Probe run 026.
- **R (Recommended conservativity)** partial. Proved: the four site lemmas
  (`lemma_suffix_ignore_on_footer`, `lemma_eff_attr_lenient_accepted`,
  `lemma_map_recommended_on_strict`, `lemma_map_coherence_recommended`), the
  header record equality `lemma_mh_policy_recommended_on_strict_ok`
  (`mh_policy(b,Recommended) = Ok(m)` on a Strict-accepted input), the conversion
  equality `lemma_conv2_policy_recommended_on_strict` and the record-indexed
  equalities `lemma_conv2_policy_rec_eq_strict`, `lemma_in_range_rec_eq_strict`,
  `lemma_raw_subst_rec_eq_strict`, `lemma_lit_{ok,good,out}_rec_eq_strict`,
  `lemma_mp_raw_acc_rec_eq_strict` (the raw phase is identical under the two
  records for *every* input), and the packet-phase induction
  `lemma_mr_at_rec_eq_strict_on_ok` (Strict-ok implies the Recommended stream
  returns the same `Ok`). NOT done: the end-to-end `mp_policy` conservativity
  (`mp_policy(b,Recommended) = mp_policy(b,Strict)` with `recovered=false`), which
  needs the pixel-sequence/assembly equality `mr_pix_policy(Recommended) =
  mr_pix_policy(Strict)` glued into `mp_assemble_policy` and an induction for the
  Recommended flag scan; the intermediate stubs for these failed (a
  pixel-sequence equality and an rlimit on the assembled lemma) and were removed
  rather than left unproved.
- Obligation **A** (allocation): `tools/audit_alloc.sh` (the DIB script adapted
  to `src/tga.rs`) reports exactly one `vec!` and no other allocating construct.

### Stage 2/3 — model-level obligation R (DONE, run 062)

- Final delivered state: real-crate run 062, **739 verified, 0 errors**; `cargo
  test --test conformance` 169/169; `tools/audit_alloc.sh` OK (one `vec!`); the
  frozen policy and spec-frozen hashes all match `policy/FREEZE.txt`.
- **R (Recommended conservativity) DONE** at the model level:
  `lemma_mp_policy_recommended_on_strict_ok` proves `forall b, r:
  mp_policy(b, Strict) = Ok(r) ==> mp_policy(b, Recommended) = Ok(r)` and
  `recovered_policy(b, Recommended) = false`. Chain (all in `policy_model.rs`):
  the four site lemmas and header equality (above); `pkt_of_mh_policy` record
  equality; the packet-phase pixel equality
  `lemma_mr_pix_rec_eq_strict_on_ok` and the flag-scan zero
  `lemma_mr_flags_rec_false_on_strict_ok`, each a per-packet induction fed by
  `lemma_strict_mr_ok_no_overrun` (a Strict-`Ok` packet's count fits the
  remaining pixels, so Recommended's `ClipPacket` never fires); the raw-phase
  equality `lemma_mp_raw_acc_rec_eq_strict` and
  `lemma_raw_subst_no_events`; `orient` equality from the pixel-sequence
  equality; and the RLE/raw `mp_after` case lemmas behind one dispatch.
- The rlimit hazard was avoided by factoring, not by any budget attribute: the
  single `mp_after` body and its RLE case both exceeded the default solver
  budget, so each was split into small non-recursive shape lemmas
  (`lemma_after_rle_shape`, `_recommended`, `lemma_after_rle_ok_suffix`,
  `lemma_recovered_rle_false`) called in sequence. No `#[verifier::rlimit]`.
- `policy.rs` now carries the four-variant `Profile` (`Strict`, `Recommended`,
  `GdkCompat`, `ImageCrateCompat`) and the compatibility records in
  `profile_policy_exec`; the `Profile` extension is compiling and verifying but
  has **no executable effect** yet (nothing calls it).
- `policy_model.rs` additionally carries the header-indexed spec helpers
  (`mh_policy_of`, `*_flags` map/attr/depth predicates,
  `lemma_mh_policy_eq_of_header`) and `conv_domain`, all verified. These are the
  spec side of the policy header parser (piece B1) and are currently unused.

### Stage 2/3 — NOT done (see STUCK.md)

- **B (executable)**: `decode_with_policy(bytes, profile) -> result + recovered`
  is not implemented. The policy-parameterised `parse_header`, `decode_one`,
  `convert_sample`/`convert_index`, `decode_raw`, `decode_rle` and the
  `decode_policy_tracked` charged-step bound `steps <= c*(|b|+N)` are not
  present. `tga.rs` is byte-identical to the 722-obligation state
  (`tga.rs.pol-4` == `tga.rs`); a full draft of `parse_header_policy`,
  `header_verdict`, `header_inv_policy`, `pkt_of_policy` and the eight guard
  wrappers existed and compiled but was **reverted** to the green tree on the
  sandbox-restart request; its design is recorded in STUCK.md for continuation.
- **C (tests)**: the Rust conformance harness over `POLICY-VECTORS.json`, the
  corpus acceptance counts and the `cfg(test)` differential harness against
  `policy_oracle.py` are not written. The corpus bytes are at
  `/tmp/claude-0/corpora/tga/` per `policy/validate.py` (absent in this sandbox;
  regenerate or re-fetch before the corpus counts).
- The compat-profile exact semantics and the universal T1-T4/T8/T10/W
  obligations on the policy-indexed executable are not stated (they need B).

### Green backups

`src/*.pol-<n>`: `tga.rs`/`model.rs`/`lib.rs` `.pol-4` are the 722-obligation
run-041 state; `policy.rs`/`policy_model.rs` `.pol-7` (and the identical
`policy.rs`/`policy_model.rs`/`tga.rs` `.pol-7`) are the 739-obligation run-062
state (`tga.rs.pol-7` == `tga.rs.pol-4`).

### Stage 2/3 — piece B1/B2 (policy header parser + conversion layer) DONE

- Final state run 105, **801 verified, 0 errors**. `tga.rs` gains the policy
  header parser and conversion layer; the 1.0 `decode`/`parse_header` are
  unchanged (behaviour and proofs), so the 739-obligation results stand.
- B1 (`parse_header_policy`, `header_verdict`, `header_inv_policy`,
  `mh_of_header`, `pkt_of_policy`): the 18 header fields are read once, the
  eleven exec guard wrappers (`type_ok_hdr_exec`, `kind_hdr_exec`,
  `flag_ok_hdr_exec`, `flag_present_hdr_exec`, `depth_ok_hdr_exec`,
  `core_attr_ok_exec`, `image_attr_ok_exec`, `attr_ok_hdr_exec`,
  `eff_attr_exec`, `coherence_bad_hdr_exec`, `range_bad_hdr_exec`,
  `index_depth_bad_hdr_exec`) each prove equality with the matching
  `policy_model` predicate, and `header_verdict` returns the model's exact
  first error. T5: `mh_policy(bytes@, p) == Ok(mh_of_header(h, p))` (or the
  exact error) via `policy_model::lemma_mh_policy_eq_of_header`. The header
  `Header` struct and its spec accessors became `pub(crate)` so the model-facing
  spec functions could be `pub(crate) open`; the 1.0 readers are unchanged.
- B2 (`convert_sample_policy`, `convert_index_policy`, `convert_one_policy`):
  the policy `conv2`, guard by guard: `rgb5`/`map5` channel rules
  (`rgb5_chan_exec`/`map5_chan_exec`, `GdkSwapShift` swapping the outer fields),
  `map16_alpha` (`map_entry_alpha_exec`), `alpha=Stored`
  (`stored_alpha_exec`), `map_index` (`entry_j_exec`/`index_in_range_policy`)
  and `index_range=Black` (`black_substitute_exec`). Each proves its result
  equals `policy_model::conv2_policy` for the accepted header.
- Design fixes along the way: `pkt_of_policy` now builds the attribute count
  through `policy_model::eff_count_img` (the effective count on the governing
  depth) so the record's `attr` is what the conversion reads; a comparison
  whose operands have mixed integer types must be bridged by an explicit
  `==`-of-`bool` assert or a `match`, because Verus does not reduce
  `attr == 1u8` into `(attr as int) == 1` on its own.


### Stage 2/3 — piece B3 raw loop DONE (run 148, 837 verified, 0 errors)

- `decode_raw_policy(bytes, h, map, indexed, out, cursor, n, c0g, p)`: the policy
  raw loop, parameterised by `Policy` exactly as `decode_raw` is by nothing.
  Loop invariants: `*cursor == c0 + k*sb`, `wrote_samples_policy` (the policy
  analog of `wrote_samples`, over `conv2_policy`), `raw_prefix_ok_policy` (every
  sample `j<k` present and its index in range or substituted), and
  `mp_raw_cur_policy(c0,0) == mp_raw_cur_policy(c0,k)`. Postconditions: on success
  the final cursor is `c0 + N*sb`, every sample written at its oriented cell, and
  `raw_result_eq_policy` (the result equals the policy model's raw phase as the
  exact `Result`). On `Err` the model's exact first error.
- New model lemmas in `policy_model.rs`: `raw_sample_ok_policy`,
  `raw_prefix_ok_policy` and their step/at/intro/zero lemmas;
  `lemma_raw_sample_ok_policy_of_loop`; `mp_raw_cur_policy`'s three branch
  lemmas (`_trunc`, `_index`, `_ok_guard`) and `_mono`;
  `lemma_write_preserves_policy_quads` (a single `write_pixel` preserves every
  earlier quad, since `fcoord` is injective).
- `header_inv_policy` gained `h.map_length as int <= 65535`, proved by
  `parse_header_policy` from the `u16` read (`lemma_le16_lt`); the map-entry
  stride `1..=4` is derived inside `convert_index_policy` from
  `map_entry_depth_ok` rather than carried as a precondition.
- The governing depth of the indexed path is the **map entry depth**
  (`h.map_entry_depth`): an earlier draft used the image depth, which was wrong
  for indexed images (this was also a real bug in `convert_sample_policy`'s
  precondition; the depth table now takes `h.pixel_depth` for the
  `kind = 1|2` records and the entry depth for `kind = 0`).

### Stage 2/3 — handback state (run 172, 865 verified, 0 errors)

Verified and in the tree: B1 (policy header parser), B2 (policy conversion
layer) and B3-raw (`decode_raw_policy`). `cargo test --test conformance`
169/169; `tools/audit_alloc.sh` OK (one `vec!`); frozen hashes OK; no
assume/admit/external_body/trusted/unsafe/todo/rlimit anywhere in the new code.

Model side, additionally verified: the policy packet-stream cursor projection
`mr_cur_policy` and its `lemma_mr_cur_policy_proj`; the eight packet-step
lemmas (`_ctrl_absent`, `_overrun`, `_rep_missing`, `_rep_index`, `_lit_missing`,
`_lit_index`, `_rep_ok`, `_lit_ok`); `mr_gcur_policy`; `pkt_post_policy` and its
six exit lemmas plus the two `Ok` arms (`lemma_pkt_policy_rep_ok`,
`lemma_pkt_policy_lit_ok`, both taking the model's own pixel split as a
hypothesis); `lemma_mr_pix_policy_rep`/`_lit`; `lemma_mr_pix_policy_len`;
`lit_seq_policy` with its peel and length lemmas; `lemma_write_preserves_policy_
quads`.

STILL OPEN (the exact continuation point):
1. `decode_rle_policy`, `rle_packet_policy`, `rle_repeat_packet_policy`,
   `rle_literal_packet_policy` in `tga.rs` — a full draft existed but did not
   verify; it was removed to keep the tree green. The two helper postconditions
   already exist as `policy_model::pkt_post_policy` and the local
   `pkt_out_policy`, and every packet exit they need is proved, so the port is
   now a matter of the loop bodies. Watch items found in the draft: the literal
   helper's `paint <= need` needs the `ClipSamples`/other case split spelled out
   (`match`-derived `need` does not reduce on its own); the loop's
   `map@.len() == map_bytes_flags(...)` must be re-asserted after `reveal` in the
   loop body or the `convert_one_policy` precondition is not discharged; the
   literal loop must carry `lit_ok_policy` (as the 1.0 loop carries `lit_pref`)
   to close `lemma_mr_pix_policy_lit`/`lemma_pkt_policy_lit_ok`; and the packet
   helper's `reject_overrun` must be computed by the same `match` shape the
   model uses, not by `&&`/`==`, so their equality asserts go through.
2. `decode_policy_tracked` (the charged-step bound `steps <= c*(|b|+N)`),
   `decode_with_policy(bytes, profile) -> (Result<Decoded, Error>, recovered)`,
   the universal T1-T4/T8/T10/A/E statements and the witness functions.
3. C: the Rust harness over `POLICY-VECTORS.json` (302 per profile), the
   per-profile corpus counts (the corpus bytes are absent; `policy/validate.py`
   reads `/tmp/claude-0/corpora/tga`, and the repo-side fetch helpers are not
   present in this sandbox — the fetch needs tooling I do not have here) and the
   `cfg(test)` differential harness against `policy_oracle.py`.

Green backups: `src/*.pol-w5` is this state (865); `*.pol-w4` (863),
`*.pol-w3` (828/837), `*.pol-w2` (801), `*.pol-w1` (775), `*.pol-w0` (739,
the pre-B baseline).

## no_std + alloc conversion (2026-11-12)

`tga_verus` is now `#![no_std]` + `extern crate alloc` with vstd `default-features = false, features = ["alloc"]`. Runs 206, 207: **947 verified, 0 errors** (unchanged); conformance and unit tests green; `tools/audit_alloc.sh` OK. Edit set, certification evidence and limits: `NO_STD.md`. Backups `tga_verus/**/*.nostd-1`. The unit-test module needs `#[macro_use] extern crate std;` at the crate root (lib.rs) for `format!`/`println!`.

## tooling hardening port (2026-11-12)

Ported from DIB ('tooling hardening after Sol review'). Test and lint tooling only: no change under `tga_verus/src`, no frozen file, no git state change, no verifier run. Edited: `tools/alloc_scan.py`, `tga_verus/tests/fault_injection.rs`, this file. `tools/audit_alloc.sh` is unchanged (it already runs the scanner and its self-test).

**1. Lint patterns (`tools/alloc_scan.py`).** The alloc pattern now also flags `.split_off(`, `.clone_from(`, `insert*` (`insert_str`, ...), `shrink*`, `.splice(`, `.into_vec(`, `.into_string(`, `.concat(`, `.join(`, `.repeat(`, `resize*` (was `resize(` only), bare `with_capacity(`, and `HashMap/HashSet/BTreeMap/BTreeSet/VecDeque/BinaryHeap::`. The push pattern is now `push*` (`push_str`, `push_back`, ...), with whitespace and turbofish variants; ghost-region stripping is unchanged, so `Seq::push`/`Seq::insert` in `spec fn`, `ensures`, `proof {}` and `assert` stay unflagged. The self-test now has 44 negative controls (each must be flagged), 8 non-allocating look-alikes that must stay unflagged (`split_at`, `copy_from_slice`, `clone_from_slice`, `truncate`, `swap`, `len`/`get`, `iter().sum`, `first`), and a proof-region `Seq::push/insert` control, on top of the original controls. It remains a falsification lint, not the call-graph audit: it matches method names, so an allocating method not listed, or allocation behind a callee, is invisible. `tools/audit_alloc.sh --self-test`: OK; production sources clean, one `try_zeroed` call site, both guard hashes OK.

**2. Alignment (`tests/fault_injection.rs`).** The allocator now records the alignment of the failed event (`FAILED_ALIGN`) and of the last successful block of exactly the watched size (`WATCH_ALIGN`, immune to harness allocations of other sizes). For every allocating input, the failed reservation has align 1 and a size that is a multiple of 4, and the successful reservation has align == `align_of::<u8>()` == 1 and size >= 4*width*height. TGA has a single reservation (the 4N output), so there is no per-reservation list.

**3. Suite-level coverage.** New test `every_schedule_is_exercised_across_the_suite` runs every schedule on all 169 frozen vectors plus up to 12 corpus inputs (read in place from `~/corpora/tga`, never copied), learning each request size S from the allocation-1 failure and windowing the threshold schedules to S so harness allocations are never faulted. It prints per-schedule counts and asserts: allocation 1, thresholds 1, 4096, 65536 and 64 MiB each injected at least once (64 MiB would instead print "UNREACHABLE" if the largest request were below it); allocation 2 reached by every allocating case and never fired. Observed: 181 inputs, 120 allocate, largest reservation 67,108,864 bytes; nth=1 120/120; nth=2 reached 120, injected 0; thr=1 120/120; thr=4096 6/6; thr=65536 5/5; thr=64 MiB 2/2 (reachable: two frozen vectors reserve at least 64 MiB).

**Results (`cargo test --offline -- --nocapture --test-threads=1`, all green).** Conformance 169/169; four profiles 302/302 each; custom 28/28; differential 52,348 comparisons, 0 mismatches; corpus acceptance Strict 252, Recommended 266, GdkCompat 264, ImageCrateCompat 259 of 294; fault injection 9/9 (the original 8 plus the suite test). `target/` deleted afterwards.
