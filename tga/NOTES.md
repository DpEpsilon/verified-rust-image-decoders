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
