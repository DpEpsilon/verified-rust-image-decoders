# TGA policy theorem delta (candidate, not frozen)

This extends `/home/ws/projects/verified-decoders/tga/THEOREMS.md` (TGA-CORE-SPEC 1.0, T1–T11) for `POLICY-SPEC.md` in this directory. It is a list of obligations, **not a completed proof**. Nothing here weakens a 1.0 obligation or reinterprets the shipped `decode(bytes)`.

## Model and executable boundary

Define the 13 fields of `POLICY-SPEC.md` as internal typed enums. Expose a `#[non_exhaustive]` profile selector with `Strict`, `Recommended`, `GdkCompat_2_42_12`, `ImageCrateCompat_0_25_10` and `Default = Strict`. Construction of an arbitrary field record exists only under `#[cfg(test)]` and is absent from release builds. New API: `decode_with_policy(bytes: &[u8], profile: Profile) -> Result<ProfiledDecoded, Error>`, where `ProfiledDecoded` holds width, height, RGBA8 bytes and `recovered: bool`. `decode(bytes)` keeps its signature and result.

Index the three 1.0 models by a policy record `P`:

- `MH(b,P)` adds to the 1.0 header record: `present` (map present, which depends on `map_header`), the **effective attribute count** `eff` (equal to the descriptor count whenever R5 allows it; otherwise the `Lenient` value or `InvalidAttributeBits`), and the selected depth table. `map_bytes` is `length*ceil(entry_depth/8)` when `present`, else 0, including `Gdk` zero-length maps.
- `conv(H,P,sample,map)` takes `eff`, `alpha`, `map16_alpha`, `rgb5`, `map5`, `map_index` and `index_range`, and returns either an RGBA pixel plus a substitution bit or `ColorIndexOutOfRange`.
- `MR(s,H,P,map,cursor0,produced0)` takes `rle_overrun` and returns, besides 1.0's state, a clip bit. `MP(b,P)` takes `trailing` and returns `recovered = clip_bit OR any substitution bit`.

`MH(b,Strict)`, `MR(...,Strict)` and `MP(b,Strict)` must be **definitionally** the 1.0 models (define `MP_1_0 := MP(·,Strict)`), so that the existing T5–T9 proofs carry over by unfolding rather than by a second equivalence proof between independently written decoders.

## Obligations

| Claim | Change for policies |
|---|---|
| T1 safety | Quantify over every retained field record, not only named profiles. New reads: none beyond 1.0 (a `Gdk`/`Image` map with `length=0` reads no map byte; `ZeroBased` reads entry `j<length` inside the checked map slice). |
| T2 header safety (map-address bound) | Quantify over every retained field record. `map_bytes<=262,140` still holds for every `map_header` value; the `Gdk` and `Image` values drop `ColorMapRange`, so prove that `first` is never used in an address computation under `ZeroBased` and only as `index-first` (natural-number subtraction guarded by `index>=first`) under `Absolute`. |
| T3 packet safety/termination | `ClipPacket` checks availability of `1+count*sample_bytes` (literal) or `1+sample_bytes` (repeat) bytes, then writes `min(count, N-produced)` pixels. `ClipSamples` checks `1+min(count,N-produced)*sample_bytes` for a literal packet. Both strictly increase `produced` by at least 1 per packet and end the loop at `produced=N`. |
| T4 conversion/orientation bounds | Quantify over every retained field record. Unchanged orientation bijection. `conv` reads at most one validated map entry, at an address inside the checked map slice, for every combination of `map_index`, `map_header`, `map5`, `map16_alpha` and `alpha`; the `Black` substitute reads none. |
| T5 exact header | `MH(b,P)` equals the policy-indexed R1–R7 model as an exact record or exact first error, for each named profile. Witness each new acceptance site (`Lenient`, `IgnoreAbsent`, `Gdk`, `Image`, depth tables) by a vector in `POLICY-VECTORS.json`. |
| T6 exact packet semantics | For each named profile, the concrete RLE helper equals `MR(·,P)` including the final cursor (which differs between `ClipPacket` and `ClipSamples` on a clipped literal packet) and the clip bit. |
| T7 total functional correctness | `decode_with_policy(b,p) = MP(b,P(p))` on every RGBA byte, the `recovered` bit and the exact first error, for each named profile. |
| T8 success/resources | `|p|=4*w*h<=67,108,864`, `1<=w,h<=8192` for every profile; every output pixel equals `conv` at its R9 position. |
| T9 precedence | R14 order with only the named check sites changed; the overrun site under `Clip*` yields `RleTruncated` (incomplete packet) or no error. |
| T10 termination | Universal over every retained record. |
| T11 integrated call path | Unchanged structure; the profile value flows from the public call into the header record and `conv`. |
| **W (new) work bound** | 1.0 has no charged-step theorem. State one for all records: header and map work is constant; raw decoding performs one conversion per sample (`N`); RLE performs constant work per packet plus one write per painted pixel, and a packet consumes at least one input byte, so `steps <= c*(|b|+N)` for a fixed constant `c`. The implementation must prove this bound for its own charged steps, with a constant `c` that is fixed in the proof and independent of the oracle; the oracle's running time is not evidence for it. `Clip*` iterates only over painted positions, never over the clipped remainder (at most 127 positions per packet, but they must cost zero iterations). |
| **A (new) allocation** | For every retained field record and every input: exactly one input-dependent allocation, the output buffer of `4N` bytes, on success; none on a header error; no other input-dependent scratch allocation (no palette cache, conversion table sized by the input, or intermediate row buffer). Map entries are read in place from the input slice. The Python oracle's temporaries and conversion cache are not evidence either way. |
| **R (new) Recommended conservativity** | `forall b: MP(b,Strict)=Ok(w,h,p) ==> MP(b,Recommended)=Ok(w,h,p) with recovered=false`. Prove it at the model level from four site lemmas: under Strict-success, (1) every descriptor count is R5-allowed, so `Lenient` sets `eff=count`; (2) flag 0 implies zero map fields, so `IgnoreAbsent` is inert; (3) every RLE packet has `count<=N-produced`, so `ClipPacket` never fires; (4) the suffix is empty or a footer, so `Ignore` is inert. Then T7 for both profiles transfers it to the executable. |
| **E (new) flag exactness** | For every retained field record and every successful profiled decode: `recovered=true` iff at least one event of the closed two-event list in `POLICY-SPEC.md` occurred: an RLE packet with `count>N-produced` under `rle_overrun ∈ {ClipPacket, ClipSamples}`, or a `Black` substitution of an out-of-range index at a pixel write under `index_range=Black`. Prove both directions. Corollary: every record with `rle_overrun=Reject` and `index_range=Reject`, Strict included, yields `recovered=false` (so `decode` is unaffected). The claim is exact for all records because the definition is closed and no other field has a flag-setting site; the oracle confirms this by construction (`policy_oracle.py` sets the flag only in the `Black` branch of `convert` and at `recovered \|= over` in the RLE loop). Other data-affecting interpretations, such as `image_type=GreyFallback` decoding an unknown type as raw grey or `map_index=ZeroBased`, deliberately do not set the flag; E holds for them because they are outside the definition, not because they leave pixels unchanged. |

Safety, resource and flag obligations (T1, T2, T3, T4, T8 bounds, T10, W, A, E) are universally quantified over every retained field record and never conditional on a named profile; the resource obligations are also never conditional on success. Exact-semantics claims (T5–T7, T9) are required only for the four named profiles, and R concerns Recommended alone; an experimental record gets exact-semantics proofs only when promoted into a profile.

## Staging and risk

1. Introduce the policy-indexed models with Strict as the definitional instance; re-verify the 1.0 crate unchanged. Do not change `decode`'s arity or result type.
2. Recommended first: its four fields only relax error sites, and obligation R is a short model-level argument. This stage delivers the corpus gain (14 of 300 real files, see `REPORT.md`) without any pixel-rule change.
3. Pixel-rule and image-rs-defect fields (`interleave`, `image_type`, `alpha`, `map16_alpha`, `rgb5`, `map5`, `map_index`, `index_range`, `depths`) extend `conv`; each is a small lemma, but the `conv` case split grows from 10 to about 30 cases. Keep `conv` a separate lemma as T7 already requires.
4. `Gdk`/`Image` `map_header` values remove `ColorMapRange` and allow zero-length maps; check that no 1.0 invariant (for example `length>=1` under `present`) is silently assumed by later lemmas.
5. Keep `model_vectors.rs` single: add only small per-profile witnesses (the policy vectors are all at most 8 KiB) instead of duplicating the 169 frozen fixtures four times.

Expected effort: the Recommended stage is a fraction of the 1.0 campaign (four guarded branch changes and one conservativity lemma). The compatibility profiles touch `conv`, the header model and RLE cursor accounting and should be budgeted as a further substantial verification campaign. Hazards: a `Clip*` loop that iterates over clipped positions (breaks W), a `Lenient` implementation that recomputes alpha from the raw count instead of `eff`, and `ZeroBased` lookups that reuse the `Absolute` subtraction.
