## Resolved by Astra — final clean run 194 (2026-11-12)

No outstanding policy blocker remains from this handoff. Active source
verifies **927 obligations, 0 errors**. The complete RLE executable, total
policy decoder, universal work/allocation/event witnesses, named-profile exact
semantics and executable Recommended conservativity are delivered. Each
profile passes 302/302 frozen policy vectors and all 169 original inputs against
its expected semantics; the differential harness reports 52,348 comparisons
and zero mismatches. `NOTES.md` records the proof boundaries, corpus counts,
manifest hash, allocation audit and final backups (`src/*.pol-a7`).

The failure records below are preserved as history, not current diagnostics.

## Sol handoff: policy RLE executable stalled after runs 173–177 (2026-11-12)

Five consecutive `./verify.sh` runs on the RLE packet executable failed. The
shipped source has been restored from the 865-obligation green backup. The
unverified work remains under `tga_verus/src/{tga.rs,policy_model.rs}.pol-s1`;
`src/tga.rs.pol-s0` is the pre-attempt green copy. Do not treat `.pol-s1` as a
proved implementation. The recovered draft consists of `pkt_out_policy`,
`rle_packet_policy`, `rle_repeat_packet_policy`, and
`rle_literal_packet_policy`; **it does not yet contain `decode_rle_policy`**.

Run 173 restored the packet draft from `.pol-rc1` and failed compilation on a
missing `lemma_lit_out_policy_peel`. Runs 174–177 added a recursive append lemma,
prefix-index lemmas, carried `map_bytes_flags`, `paint <= need`, and
`lit_ok_policy` invariants, changed the overrun guard to a `match`, and
constrained literal conversion to painted positions. The run-177 log is the
precise failure record (`runs/177.log`):

- `lemma_lit_ok_policy_extend` fails its `lit_ok_policy(..., i+1)` postcondition;
  the recursive prefix proof needs a separate scalar bridge from the first
  index check and the suffix check after shifting `start` by the sample stride.
- `lemma_lit_good_policy_first_bad` fails both the first-bad-index and
  `!lit_ok_policy` postconditions; factor the recursion into small scalar
  statements and prove the first-sample case explicitly.
- `lemma_lit_out_policy_peel` fails its append postcondition and the zero-prefix
  case; a direct recursive definition unfolds too broadly (run 176 hit rlimit).
  Prefer the existing `lit_seq_policy` peel and length lemmas with a separate
  equality bridge to `lit_out_policy`, proved independently of the loop.
- The literal loop's `lemma_seq_subrange_composition` call cannot prove
  `cur0 + 1 + need*sb <= bytes.len()` after loop isolation. Carry that exact
  bound and the packet slice relation as invariants, then equate the sample
  slice with the input subrange before invoking index/append lemmas.

The packet draft has no clip/substitution flag postcondition or charged-step
bound yet. After the packet helpers verify, add `decode_rle_policy` with the
1.0 `decode_rle` loop as a template; only then integrate the public decoder,
work bound, witnesses and tests. Run 177 reported 883 verified, 4 verification
errors, and seven compiler errors total; the extra compiler errors include
proof failures and a sequence subrange precondition, not Rust execution errors.
No budget attribute was added. Do not continue this obligation in this dispatch
under the five-failure escalation rule.

## TGA policy: Stage 2/3 status (2026-11-12, run 172)

Delivered tree is green: run 172, **865 verified, 0 errors**; `cargo test --test
conformance` 169/169; `tools/audit_alloc.sh` OK (one `vec!`); `policy/FREEZE.txt`
hashes OK. No `assume`/`admit`/`external_body`/`trusted`/`rlimit` anywhere,
including the new code. This note records what is proved, what is not, and the
exact next step.

### 1. Obligations R and E — DONE (unchanged from run 062)

`lemma_mp_policy_recommended_on_strict_ok` and the closed-event flag lemmas
stand as recorded in run 062 (see NOTES.md).

### 2. Piece B1 (policy header parser) — DONE (run 068/082)

`parse_header_policy`, `header_verdict`, the eleven exec guard wrappers,
`header_inv_policy`, `mh_of_header` and `pkt_of_policy`. T5:
`mh_policy(bytes@,p) == Ok(mh_of_header(h,p))` (or the exact error) through
`policy_model::lemma_mh_policy_eq_of_header`. `header_inv_policy` carries the
`map_length <= 65535` bound (proved from `lemma_le16_lt` in
`parse_header_policy`). The 1.0 `parse_header` is untouched; the `Header` struct
and its spec accessors became `pub(crate)` so the model-facing spec functions
could be `pub(crate) open`.

### 3. Piece B2 (policy conversion layer) — DONE (run 105)

`convert_sample_policy`, `convert_index_policy`, `convert_one_policy`, and the
channel/alpha/index exec helpers (`rgb5_chan_exec`, `map5_chan_exec`,
`conv555_policy`, `map555_exec`, `map_entry_alpha_exec`, `stored_alpha_exec`,
`entry_j_exec`, `index_in_range_policy`, `black_substitute_exec`). Each proves
its result equals `policy_model::conv2_policy`. `pkt_of_policy` builds the
attribute count through `policy_model::eff_count_img` (the effective count on the
governing depth: the map entry depth for an indexed image, the image depth
otherwise) — this was a real bug in an earlier draft.

### 4. Piece B3, the raw loop — DONE (run 148)

`decode_raw_policy(bytes,h,map,indexed,out,cursor,n,c0g,p)`: the policy analog of
`decode_raw`. On success the final cursor is `c0 + N*sb`, every sample is written
at its oriented cell (`policy_model::wrote_samples_policy`, over
`conv2_policy`), and `raw_result_eq_policy` ties the result to
`mp_raw_cur_policy` (`Ok` gives the final cursor, `Err` the model's exact first
error). Loop invariants: the cursor equation, `wrote_samples_policy`,
`raw_prefix_ok_policy` and `mp_raw_cur_policy(c0,0) == mp_raw_cur_policy(c0,k)`.

### 5. The policy RLE stream — MODEL SIDE DONE, EXEC SIDE NOT

All of the model side is proved and in `policy_model.rs`:

- `mr_cur_policy` and `lemma_mr_cur_policy_proj` (its equality with
  `mr_at_policy`'s cursor/produced projection, by induction).
- `mr_gcur_policy`.
- The eight packet-step lemmas: `lemma_mr_cur_policy_ctrl_absent`, `_overrun`,
  `_rep_missing`, `_rep_index`, `_lit_missing`, `_lit_index`, `_rep_ok`, `_lit_ok`.
- `pkt_post_policy` (the packet helper's postcondition under `pol`) and its six
  exit lemmas (`lemma_pkt_policy_ctrl_absent` … `lemma_pkt_policy_lit_index`)
  plus the two `Ok` arms `lemma_pkt_policy_rep_ok` and `lemma_pkt_policy_lit_ok`,
  each taking the model's own pixel split as a hypothesis.
- `lemma_mr_pix_policy_rep`, `lemma_mr_pix_policy_lit`, `lemma_mr_pix_policy_len`,
  `lemma_lit_good_policy_le`.
- `lit_seq_policy` (a `Seq::new` form of `lit_out_policy`) with
  `lemma_lit_seq_policy_peel` and `lemma_lit_seq_policy_len`, so a loop can peel
  one sample without a recursive proof.
- `lemma_write_preserves_policy_quads` (one `write_pixel` at sample `k`'s
  oriented offset preserves every earlier quad).

The exec side — `decode_rle_policy`, `rle_packet_policy`,
`rle_repeat_packet_policy`, `rle_literal_packet_policy`, with the local
`pkt_out_policy` — was drafted (a two-helper shape mirroring 1.0) and **removed
to keep the tree green** after the last five failed runs landed on mechanical
errors, not design errors. The four that were left:

1. The literal helper's `let need = match p.rle_overrun { ClipSamples => paint, _
   => count }` does not reduce to `paint <= need` on its own: write the
   `ClipSamples`/other case split explicitly (`if p.rle_overrun == ClipSamples {
   assert(need == paint) } else { assert(need == count) }`).
2. `map@.len() == map_bytes_flags(...)` and `map_length <= 65535` must be
   re-asserted after `reveal(header_inv_policy)` inside each loop body, or the
   `convert_one_policy` precondition is not discharged.
3. The literal loop must carry `lit_ok_policy(bytes@,pkt,cur0+1,paint,p)` as an
   invariant (the 1.0 loop carries `lit_pref`), or
   `lemma_mr_pix_policy_lit`/`lemma_pkt_policy_lit_ok`'s `lit_ok_policy`
   hypothesis is missing. `lit_good_policy` at the failing sample is `i` and must
   be asserted before the `_lit_index` exit lemma.
4. The packet helper's `reject_overrun` must be computed with the same `match`
   shape the model uses (or bridged by an `==`-of-`bool` assert), not by a bare
   `&&`/`==` expression.

### 6. Still to do after the RLE stream

- `decode_policy_tracked` with the charged-step bound `steps <= c*(|b|+N)`, and
  `decode_with_policy(bytes, profile) -> (Result<Decoded, Error>, recovered)`
  with reset state; `Clip*` must iterate only over painted positions (the
  drafted loops do: the paint loop runs `min(count, N-produced)` times).
- The universal T1–T4/T8-bounds/T10/A/E witness functions on the policy path.
- Piece C: the Rust harness over `policy/POLICY-VECTORS.json` (302 vectors per
  profile), the 169 frozen vectors against the four profiles, the per-profile
  corpus acceptance counts, and the `cfg(test)` differential harness against
  `policy_oracle.py`.
  - The corpus bytes are absent. `policy/validate.py` reads
    `/tmp/claude-0/corpora/tga`; `/home/ws/projects/leniency/corpora/NOTES.md`
    records the sources but the repo carries no fetch script, and the corpus is
    not in this sandbox. Re-fetching needs the GitHub code-search acquisition
    tooling described in that NOTES.md (not present here).

## Strict bridge (Sol finding 1, MEDIUM): RESOLVED — run 205

The continuation proved `lemma_mp_policy_strict_is_mp(b)` for **every** byte sequence, including exact first errors and all successful RGBA bytes. `t7_strict_is_1_0` calls both public decoders and verifies identical results and `recovered == false`. Run 205: **947 verified, 0 errors**; green backups `src/{model.rs,policy_model.rs,tga.rs}.pol-c1`. The section below records the earlier run-199 handoff and is historical, not an outstanding obligation.

The dispatch asked for `mp_policy(b, strict_policy()) == lift(model::mp(b))` at
full quantification, then the executable corollary `decode_with_policy(bytes,
Strict)` = `decode(bytes)`. This dispatch did **not** close the full bridge. What
landed and verifies, and the exact continuation point, are below. The tree is
green; no obligation was left failing.

**Green state: run 199 / backup run 200 — 936 verified, 0 errors.** Backups
`src/{model.rs}.pol-b2` (= run-199 green) and `src/*.pol-b1` (= run-197 green,
931); `src/policy_model.rs`, `src/tga.rs`, `src/policy_tests.rs` are byte-equal
to the run-194 `*.pol-a7` state (the bridge additions were reverted rather than
left failing).

### Landed and verified (all in `src/model.rs`, 1.0-side, 9 new obligations 927→936)

- `lemma_mh_fields(b, m)`: a successful `mh` record's raw header fields read back
  at the R1 offsets, plus `attr_ok(b)`. This is what makes
  `pkt_of_mh_policy(m, Strict) == pkt_of_mh(m)` provable field by field; `mh`
  alone does not expose `b[0]`, `b[7]`, `le16(b,12)` etc. without re-unfolding
  the whole header body (a likely `rlimit` site), and `attr_ok(b)` is needed
  because `mh_selfcons` is stated over `type/depth`, not `attr_ok`.
- `lemma_mp_raw_cur_mono_ok` and `lemma_raw_prefix_ok_of_cur_ok`: a successful
  1.0 raw cursor `Ok` at 0 gives `Ok` at every `k <= N` and `raw_prefix_ok` for
  every prefix. These discharge the `raw_prefix_ok` hypothesis of the existing
  `policy_model::lemma_mp_raw_acc_policy_strict` from `MP`'s own `Ok` (that
  lemma currently has no caller, because raw-prefix was never lifted from `Ok`).
- `lemma_mr_ok_shape`, `lemma_mr_pix_ok_rep`, `lemma_mr_pix_ok_lit`,
  `lemma_mr_pix_ok_lit_prefix`: a successful 1.0 `MR`/`mr_at` at `(cur, prod)`
  fixes the produced count and pixel sequence, and each packet branch fixes its
  own pixel contribution. These are the 1.0 lemmas the packet-level bridge needs;
  `mr_cur_pix_proj` alone does not split repeat from literal.

### Not landed — the exact continuation

The policy-side bridge itself was drafted in two pieces and **reverted unverified**
(they were left mid-edit when the dispatch window closed; they were never
compiled past the first error):

1. `lemma_pkt_of_mh_policy_strict(m: MHModel)` — `pkt_of_mh_policy(m, Strict) ==
   pkt_of_mh(m)`. Uses `lemma_mh_fields` (above) plus `reveal(eff_attr)` and the
   `map_attr_ok`/`core_attr_allowed` split: Strict's descriptor count is 1.0's
   `(descriptor & 0x0f)` and its `kind` is `kind_of_type(m.image_type)`.
2. The phase lemmas `lemma_rle_policy_strict`, `lemma_suffix_policy_strict`,
   `lemma_mp_raw_policy_strict`, and the packet-by-packet induction
   `mr_at_policy(.., Strict) == mr_at(..)` at a fitting packet.

Watch items found while developing, which the next dispatch should not re-learn:

- **`mp_raw_policy` at Strict needs `raw_prefix_ok` as a hypothesis, not the raw
  `mp_raw_cur` equality alone.** The policy raw recursion carries an extra
  `index_range == Reject` guard that 1.0's `mp_raw_acc` does not; the two agree
  only where every sample is in range. `lemma_mp_raw_cur_mono_ok` +
  `lemma_raw_prefix_ok_of_cur_ok` (landed) supply it from `MP`'s `Ok`.
- **Bridge the phases with `mp_after_policy`'s R14 order, not `mp_policy`'s outer
  match** (as the existing `lemma_mp_policy_recommended_on_strict_ok` does). The
  raw error arm resolves only once inside `mp_after_policy`.
- **A literal-packet error under Strict cannot occur** (`Reject` makes every
  literal sample in range, else the RLE stream would have reported
  `ColorIndexOutOfRange`). `lemma_mr_pix_ok_lit_prefix` records the prefix
  shape for completeness; the bridge uses `lemma_mr_pix_ok_lit`.
- **The literal packet under `ClipSamples` consumes only the painted samples; the
  Strict bridge stays in the `Reject`/`ClipPacket` shapes**, so `need = count *
  sample_bytes` on both sides.
- `lemma_strict_mr_ok_no_overrun` (existing, `policy_model.rs:5391`) fixes
  `count <= N-prod` from a Strict `Ok`; call it once per packet before the
  branch split, or the fit hypothesis for `lemma_mr_ok_shape` is missing.

### Dispatch items 2 and 3: DONE

- **Corpus SHA-256 (finding 2, LOW)**: `examples/policy_corpus.rs` now verifies
  each file's digest against manifest column 2 (std-only SHA-256 with a FIPS-validated
  self-test), panics loudly on mismatch. Re-run on `/home/ws/corpora/tga`:
  `rows=294 available=294 missing=0 sha256=verified`; counts unchanged
  252/266/264/259 of 294 (Strict/Recommended/GdkCompat/ImageCrateCompat),
  recovered 0/2/4/2. Recorded in `notes/policy-corpus-sha256.txt`. No corpus
  bytes copied into the repo. The example is outside the verifier's scope (not
  in `src/`), so it consumes no verifier run.
- **Errata (finding 3, LOW)**: `policy/ERRATA.md` (new file, not in `FREEZE.txt`)
  plus a "Errata against frozen POLICY-SPEC" section in `NOTES.md`, recording
  the 8-bit-grey/count-8 discrepancy between `POLICY-SPEC.md:24` and the
  oracle/model/executable, with no public-output difference and the exact
  revision text. No frozen file edited.
