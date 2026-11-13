# ICNS policy layer: proof-phase plan (draft)

Conclusion first: if the owner takes QUESTIONS Q1(a), the proof phase is one small milestone, **P1** (typed profiles, policy-indexed models with Strict as the definitional instance, the Strict bridge, `try_*_with_policy` entry points, and a new charged-work obligation W), estimated at 60–120 new Verus obligations over the shipped 412 and one dispatch. Implementing all 6 decode-level fields (**P2**) adds roughly 150–250 obligations and 2–3 dispatches, for no corpus gain. Selection and PNG delegation (Q2, Q3) are separate specifications, not part of this plan.

This plan follows `dib/BRIEF-POLICY.md` (P1 then option groups) and the TGA instance (`tga/policy/THEOREMS-POLICY-DELTA.md`, `tga/tga_verus/src/{policy.rs,policy_model.rs}`, `try_decode_with_policy` at `tga/tga_verus/src/tga.rs:9249`).

## 1. What changes in the core

- **New files only for policy code**: `icns_verus/src/policy.rs` (field enums, `Policy` record, `#[non_exhaustive]` `Profile` with `Default = Strict`, exhaustive `profile_policy`/`profile_policy_exec`) and `icns_verus/src/policy_model.rs` (policy-indexed models). As in DIB/TGA, `Policy`'s fields are `pub` for spec well-formedness, its only constructors are crate-private, and a custom-record decoder exists only under `#[cfg(test)]`.
- **`icns.rs`**: the parser and RLE helper gain a `Policy` argument in new internal functions; the shipped `try_parse`, `try_decode_rgb`, `try_decode_mask`, `try_combine` and their `_logged` forms keep their signatures, contracts and bodies (they become calls of the policy functions at `Strict`, or stay byte-identical; the brief for P1 should require byte-identical bodies, as DIB P1 did).
- **New public API**: `try_parse_with_policy(bytes, profile) -> Result<Vec<Entry>, TryError>` and `try_combine_with_policy(bytes, entries, ci, mi, profile) -> (Result<Decoded, TryError>, bool)` (plus `try_decode_rgb_with_policy` only if glycin needs it). The `bool` is `recovered`, constantly false under Q7's recommendation.
- **Decoder precondition.** The shipped decoders require `parsed_inv(bytes, entries)`, a provenance relation (forged entries are outside the contract, R1). Parse-level fields (`file_suffix`, `entry_tail`, `entry_cap`) change *which* entry lists are produced, but every produced range still satisfies `8 ≤ start ≤ end ≤ L ≤ |b|`. Define `parsed_inv_policy(b, P, E)` and prove `parsed_inv_policy(b, P, E) ⇒ ranges_ok(b, E)`, the weaker fact the decoders actually use; then state the policy decoders over `∃P'. parsed_inv_policy(b, P', E)` so that a list parsed under one profile can be decoded under another. Decode-level fields (`raw_argb`, `zero_prefix`, `rle_trailing`) touch only `MR`/`MC`.
- **No new allocation site.** The shipped `try_combine` reserves, in order, `Channel(0..2)` (3 × `N`), `RgbOut` (3N), `MaskData` (N), `CombineOut` (4N), and `try_parse` reserves one `EntryList`. Every policy path must keep this exact request sequence (`req_*` in `alloc_guard/sites.rs`), including `raw_argb=RawFirst`, which fills the three channel buffers from the raw payload instead of from RLE. Then the A-ICNS obligations (G/Prog/PreAlloc over the log) carry over with the policy record as an extra argument, as in TGA's `g_policy`/`prog_policy`/`pre_alloc_policy_concl`. `entry_cap=Unlimited` would change `EntryList`'s bound from 4,096 to `(L−8)/8` records and is therefore excluded from P1/P2 unless the owner accepts it (Q8).

## 2. Policy-indexed models and the Strict bridge

Index the frozen models by `P`: `MP(b,P)` (parse: `file_suffix`, `entry_tail`, `entry_cap`), `MR(s,n,P)` (RLE: `rle_trailing`), `MC(b,E,i,P)` (colour: `raw_argb`, `zero_prefix`), `MA` unchanged, `MJ(b,E,ci,mi,P)`. Define the shipped models as the Strict instances: `MP_1_1 := MP(·,Strict)`, etc., so that T7–T13 transfer by unfolding (TGA's requirement; TGA needed a dispatch, pol-7, to close `lemma_mp_policy_strict_is_mp` after an earlier attempt drifted, so state it from the start). Prove the executable corollary `t13_strict_is_1_1(bytes, …)`: the Strict policy entry points return exactly what the shipped ones return (first error or every byte), with `recovered=false`; OutOfMemory is the only case where the two independent runs may differ, and then each names its own failed reservation (TGA's `t7_strict_is_1_0` shape).

## 3. Theorems (P1/P2 structure, mirroring DIB and TGA)

| Claim | Policy change |
|---|---|
| T1–T6 safety, bounds | Quantified over every retained `Policy` record, not only named profiles. New reads: none for `Ignore` fields (they read fewer bytes); `RawFirst`/`RawOnRleError` read exactly `4N` payload bytes. |
| T7/T8 exact parse | `parse_with_policy(b,p) = MP(b,P(p))` as an exact `Result` for each named profile; `parsed_inv_policy` characterised both ways. |
| T9 RLE | `rle(s,n,P) = MR(s,n,P)` per named profile, including the `rle_trailing=Ignore` end condition (cursor ≤ |s| instead of =). |
| T10–T13 decode/combine | Exact for named profiles against `MC`/`MA`/`MJ` at `P`, under the relaxed precondition of §1. |
| **W (new) work bound** | ICNS 1.1 has no charged-step theorem (no `steps` in `icns.rs`). Add one for every record, as TGA did: `steps(parse) ≤ c·(|b|+1)` and `steps(combine) ≤ c·(|payload_c|+|payload_m|+N)` with a fixed constant `c` proved for the implementation's own charged counter, independent of any oracle. |
| **A allocation** | §1: the shipped request sequences, for every record and every input; no input-dependent scratch beyond them. |
| **R (Recommended conservativity)** | `∀b: MP(b,Strict)=Ok(E) ⇒ MP(b,Recommended)=Ok(E)`, and likewise for `MJ` with `recovered=false`. With Recommended = Strict (OPTIONS), R is definitional; it becomes real only when a relaxation enters Recommended under a new name. |
| **E (flag exactness)** | `recovered = true` iff an event of the closed list occurs. With Q7's empty list: `recovered = false` for every record and input, one lemma. |

Resource and flag obligations (T1–T6 bounds, W, A, E) are universally quantified and never conditional on success; exact-semantics claims are for named profiles only.

## 4. Fallible allocation

The policy entry points are `try_*` from the start: no infallible allocation is introduced, `OutOfMemory` names the failed `(site, elements)` reservation as the first element of the policy request sequence (`req_*_policy`), and the public contract has TGA's shape: non-OOM results equal the model; OOM implies `recovered=false`; a `_logged` variant returns the `Tracked<AllocLog>` so a caller (glycin glue, a future selector) can compose logs the way ICO imports DIB's. Since §1 keeps the request sequence identical for every record, the shipped `req_*` lemmas need only an extra `P` parameter that they ignore.

## 5. Vectors and oracle

- `policy/policy_oracle.py`: an independent Python reference with the frozen ICNS checker's semantics as `Strict` and an `EXPERIMENTAL_OPTIONS` switch for custom records (DIB/TGA pattern). It must reproduce all 111 frozen vectors under Strict on every run.
- `policy/gen_policy_vectors.py`: per-profile vectors from (i) every probe in `/home/ws/projects/leniency/policy-probe/probe_cases.py` (`icns_cases`), (ii) one positive and one negative witness per field value, (iii) the coupled cases `file_suffix` × `entry_tail`, `raw_argb` × `zero_prefix` (an `it32` payload of `4N+4` bytes starting with zeros), `rle_trailing` × `raw_argb`. Records `{name, operation, input_hex, expected}` with `profile` and, on success, `recovered`.
- `policy/validate.py`: compares GdkCompat with gdk-pixbuf 2.42.12 (forced type, whole buffer) and IcnsCrateCompat with `icns` 0.3.1 on the vectors and the corpus, using the existing runners in `/home/ws/projects/leniency/policy-probe/`. Because the libraries select and the core does not, the comparison must feed both sides the **same chosen pair** (the library's choice reconstructed from its type and the duplicate rule), or compare only files with exactly one legacy pair. Any seed used while writing the spec is a regression set; only a seed generated after the last spec edit counts as held out.
- `FREEZE.txt` hashes POLICY-SPEC, THEOREMS-POLICY-DELTA, oracle, generator and vectors, as for DIB/TGA.

## 6. Staging and estimate

| Milestone | Content | Estimate |
|---|---|---|
| P1 | `policy.rs`, `policy_model.rs`, Strict/Recommended profiles (Recommended = Strict), Strict bridge and `t13_strict_is_1_1`, `try_parse_with_policy`/`try_combine_with_policy`, W for the shipped code paths, A by unchanged request sequences, E constant false; policy conformance (vectors for both profiles) beside the 111 frozen vectors. | 60–120 obligations; 1 dispatch. Reference: DIB P1 added 37 obligations (1,557→1,594) with no work-bound change; ICNS needs W from scratch. |
| P2 | The 6 fields and the two compatibility profiles, one field at a time: parse-level first (`file_suffix`, `entry_tail`), then `rle_trailing`, `zero_prefix`, `raw_argb`. Each needs model branch, executable branch, exact-semantics lemma and witnesses. | 150–250 obligations; 2–3 dispatches. Reference: TGA's 13 fields took 642→947 obligations over 7 dispatches, with RLE executable helpers the stall point (pol-3, pol-4). |

Hazards: an `rle_trailing=Ignore` implementation that reads the suffix; `RawFirst` reserving a different buffer sequence (breaks A); a parse-level relaxation that lets a range end past `L` but within `|b|` (fine for safety, but the model must say which); multiplying the 111 frozen vectors per profile in `model_vectors.rs` (keep the frozen witness file single and add small policy witnesses, as DIB/TGA did).
