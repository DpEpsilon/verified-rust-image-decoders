## TGA policy: Stage 2/3 status (2026-11-12, run 062)

Delivered tree is green: run 062, **739 verified, 0 errors**; `cargo test --test
conformance` 169/169; `tools/audit_alloc.sh` OK (one `vec!`); `policy/FREEZE.txt`
hashes OK. No `assume`/`admit`/`external_body`/`trusted`/`rlimit` anywhere. This
note records what is proved, what is not, and the exact next step.

### 1. Obligation R — DONE (was the open item from run 041)

`forall b, r: mp_policy(b, Strict) = Ok(r) ==> mp_policy(b, Recommended) =
Ok(r)` and `recovered_policy(b, Recommended) = false` are proved by
`policy_model::lemma_mp_policy_recommended_on_strict_ok` (713-line chain, all in
`policy_model.rs`). The factoring STUCK.md previously suggested was the fix: a
per-packet step lemma (`lemma_strict_mr_ok_no_overrun`), the pixel-sequence
induction (`lemma_mr_pix_rec_eq_strict_on_ok`), the flag-scan induction
(`lemma_mr_flags_rec_false_on_strict_ok`), and small non-recursive shape
lemmas for the RLE/raw `mp_after` cases. The two rlimit failures recorded under
run 036/039 are gone without any budget attribute: the single dispatcher bodies
were each split until no body exceeded the default solver budget.

### 2. Piece B (`decode_with_policy`) — NOT done

Not implemented. `tga.rs` is byte-identical to the 722-obligation state
(`tga.rs.pol-4`). A complete draft existed and **compiled to 754 verified with 4
residual errors** (runs 046-061) before being reverted for the sandbox restart.

**What already exists and verifies (keep, do not rewrite):**

- `policy.rs`: the four-variant `Profile` (`Strict`, `Recommended`, `GdkCompat`,
  `ImageCrateCompat`), `profile_policy`, and `profile_policy_exec` covering all
  four records. Compiling and verified; no executable caller yet.
- `policy_model.rs`: the header-indexed spec layer for the parser —
  `map_flag_ok_policy_flags`, `map_present_policy_flags`,
  `map_coherence_bad_flags`, `map_range_bad_flags`,
  `image_index_depth_bad_flags`, `map_bytes_flags`, `mh_policy_of`,
  `lemma_mh_policy_eq_of_header` (proved: `mh_policy(b,p) == mh_policy_of(...)`
  when the header's fields are `b`'s bytes), plus `conv_domain` and
  `lemma_pkt_of_domain_conv`. All verified.

**The draft parser design that compiled (reverted, re-derive from this):**

- `mh_of_header(h, p)` — the model record a parsed header reconstructs, i.e. the
  header's own field values plus `map_bytes_flags(...)`. T5 for the parser is
  `mh_policy(bytes@, p) == Ok(mh_of_header(h, p))`.
- `parse_header_policy(bytes, p)` — reads the 18 header fields with the existing
  `le16`, builds `Header`, folds `Ok(h)`/`Err(e)` through
  `header_verdict_spec(&h, p)` = `Result<(), Error>` of `mh_policy_of` on `h`'s
  fields, then calls `crate::policy_model::lemma_mh_policy_eq_of_header` to tie
  `mh_policy(bytes@, p)` to `mh_policy_of`. Postcondition: T5 plus
  `header_inv_policy(h, p)`.
- `header_inv_policy(h, p)` — the policy analogue of `header_inv`: `type_ok`,
  flag rule, dimension bounds, `descriptor&0xc0 == 0 || interleave = Ignore`,
  depth table, `1 <= sample_bytes <= 4`, output cap,
  `!map_coherence_bad_flags`, `!map_range_bad_flags`,
  `!image_index_depth_bad_flags`, `attr_ok_policy`, the map-slice bound
  `map_length*stride <= 262140`, and `conv_domain(pkt_of_policy(&h, p))`.
- `pkt_of_policy(h, p)` — the executable `PktH` (kind via `kind_policy`, `attr`
  via `eff_attr`, `map_index`/`index_range` handled in the conversion). Bridge
  `lemma_pkt_of_policy_eq` proves it equals
  `pkt_of_mh_policy(mh_of_header(h, p), p)`.
- Eight `*_exec` guard wrappers, each `if cond { .. }` with an exec condition
  and an `assert` bridging it to the spec predicate: `kind_hdr_exec`,
  `type_ok_hdr_exec`, `flag_ok_hdr_exec`, `flag_present_hdr_exec`,
  `depth_ok_hdr_exec`, `core_attr_ok_exec`, `image_attr_ok_exec`,
  `attr_ok_hdr_exec`, `coherence_bad_hdr_exec`, `range_bad_hdr_exec`,
  `index_depth_bad_hdr_exec`. `header_verdict(&h, p)` calls them in the model's
  order.

**The 4 residual errors (all mechanical, not design):**

1. `type_ok_hdr_exec`'s postcondition (`r == type_ok_policy(...)`) needed
   `reveal(crate::model::type_ok)` in addition to `reveal(type_ok_policy)`.
2. Three `header_verdict` exits (interleave, `depth_ok`, `attr_ok`) did not yet
   have the bridge `assert` from the exec guard to
   `header_verdict_spec(*h, p)`; the value-carrying form
   (`let r = <guard>; assert(r == <spec>); if !r { return Err }
   else { assert(header_verdict_spec == Ok(())); }`) is what the last several
   runs were converging on (runs 059-060 moved 753 -> 754 verified and the
   errors down from 8 to 4).

**Next step for a continuation agent:** restore the draft from the design above
(it is not on disk; only `tga.rs.pol-4` is), apply the two fixes in items 1-2,
then continue with `decode_policy_tracked` (raw/RLE parameterised by `Policy`),
the conversion layer (`convert_sample`/`convert_index` with the nine pixel-rule
options and the `conv` case split), `decode_with_policy(bytes, profile)`, the
charged-step bound W (`steps <= c*(|b|+N)`, `Clip*` iterating only painted
positions), and the `cfg(test)` `decode_custom_policy`. Do **not** edit 1.0
`decode`/`parse_header`; keep `tga.rs.pol-4`'s behaviour.

### 3. Compatibility profiles, policy conformance, corpus (NOT started)

Section 2 of the previous STUCK.md still stands: the nine pixel-rule fields, the
`GdkCompat`/`ImageCrateCompat` exact-semantics proofs, the `POLICY-VECTORS.json`
Rust harness (302 per profile), the corpus acceptance counts and the
`policy_oracle.py` differential harness are not written. Corpus bytes are read
from `/tmp/claude-0/corpora/tga/` (`policy/validate.py:31`); that directory is
absent in this sandbox, so re-fetch or regenerate before the counts.
