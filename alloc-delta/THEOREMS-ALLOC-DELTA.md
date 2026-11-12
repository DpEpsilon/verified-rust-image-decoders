# Fallible allocation: theorem delta (draft, not a completed proof; third revision, owner decisions of 2026-11-12, see `REVIEW-SOL.md`)

This document extends DIB `THEOREMS.md` (with frozen 1.1 `THEOREMS-DELTA.md` and `policy/THEOREMS-POLICY-DELTA.md`), TGA `THEOREMS.md`, ICNS `THEOREMS.md` and ICO `THEOREMS.md` (`origin/main`). It edits none of them.

The frozen-named public functions are **removed** from the main crates (`SPEC.md` §1.2), so each verified API theorem stated on them is **retired**. Each is replaced by a verified obligation on the corresponding `try_*` function (below), with no wrapper and no inheritance argument in the main crate. Theorems on internal helpers remain verified. The tables in §1–§4 mark every frozen row as *helper, kept*, *retired; restated on `try_*`*, or *replaced*.

The exclusions are unchanged: no `assume`, `admit`, `external_body`, trusted FFI, `unsafe` or false precondition, and no new `assume_specification`. The trust items every proof report must name are A-STD-1/2 (`SPEC.md` §3), the guard body's binding of `Err` to `try_reserve_exact`'s `Err` arm (hash-pinned text), and the call-graph audit's allowlist (manually reviewed, per target, with its hash and reviewer; `SPEC.md` §6).

## 0. Shared definitions

- `fmt_part(r)`, for a `try_*` result `r` that is not `OutOfMemory`, is the frozen-typed result: `Ok(v) ↦ Ok(v)`, `Err(Format(e)) ↦ Err(e)`. For the DIB policy pair, it applies to `r.0`.
- `Eq_f(x, r)` is entry point `f`'s result relation: the frozen executable-to-model predicate applied to `fmt_part(r)`, as tabulated in `SPEC.md` §2 ("Result relations"). Per core: DIB `decode_mp_eq`; DIB policy `decode_policy_mp_eq ∧ decode_policy_alpha_eq ∧ r.1 == recovered_policy`; TGA `decode_eq`; ICNS `parses_to`, `rle_result_eq`, `rgb_decoded_eq`, `mask_decoded_eq`, `combine_decoded_eq`; ICO `parse_eq`, `md_eq`. No clause in this document equates an executable result with a model value directly; executables return `Vec`/`Decoded`, models return `Seq`/tuples, and ICNS's parse model has its own error enum `MpError`.
- `L` is the final `view()` of the sealed `alloc_guard::AllocLog` that the `*_logged` entry point creates with `new_log()` and returns (`SPEC.md` §2). Each element is `(site, k, ok)`. `req(L)` is `L` with `ok` dropped. Only guard functions append to the log, one element per call, with `ok == (r is Ok)` for that call's result; ICO's `import_dib` (`SPEC.md` §3.1) is the one guard function that appends a whole imported DIB log. Code outside `alloc_guard` can neither construct an `OutOfMemory` nor edit the log.
- `G(f, Eq_f, Req)` holds for input `x`, result `r` and log `L` when all of the following hold:
  1. `r is Ok ∨ r is Err(Format(_))` ⇒ `Eq_f(x, r) ∧ req(L) == Req(x) ∧ ∀j: L[j].ok`.
  2. `r == Err(OutOfMemory(o))` ⇒ `L.len() ≥ 1 ∧ req(L).is_prefix_of(Req(x)) ∧ (∀j < L.len()−1: L[j].ok) ∧ ¬L.last().ok ∧ (L.last().site, L.last().k) == (o.site(), o.elements())`.
  3. Every `(s, k)` in `Req(x)` has `k` within the per-site maximum (`SPEC.md` §4), proved from checked values **before** the guard call.
- `log_tied(r, L)` is the log-only part of clauses 1–2 (drop `Eq_f` and `Req`). DIB exports it as `dib::dib::log_tied` for ICO's `import_dib`.
- `Prog(f, Eq_f, Req)`, the conditional progress theorem: `(∀j: L[j].ok) ⇒ r is not OutOfMemory ∧ Eq_f(x, r) ∧ req(L) == Req(x)`. It follows from `G` clauses 1–2; each core proves it by name so that a mutation weakening either clause leaves a failing target.
- `PreAlloc(x) ⇔ Req(x) == []`, with `Reach` its negation defined by code reachability (`SPEC.md` §2). Named corollary: `PreAlloc(x) ⇒ r is not OutOfMemory ∧ Eq_f(x, r) ∧ L == []`. Header errors are never `OutOfMemory`.
- The public `try_*` function is the `*_logged` function with the log discarded. Its `ensures` carries `r is not OutOfMemory ⇒ Eq_f(x, r)`, and `r == Err(OutOfMemory(o)) ⇒ (o.site(), o.elements()) ∈ Req(x)`.
- **No wrapper inheritance.** A main crate has no frozen-named function, so each retired theorem is replaced by the `try_*` theorem alone: `Eq_f` on `fmt_part(r)` is exactly the frozen predicate the retired theorem stated (`decode_mp_eq` for DIB `decode`, `decode_eq` for TGA, `parses_to` for ICNS `parse`, `md_eq` for ICO `decode_entry`, and so on). The optional `<core>_abort_shim` crates carry an informal argument of the same shape (`SPEC.md` §1.3); it is documentation of an unverified crate and appears in no main crate's proof report.
- **Mutation kills depend on `Eq_f`.** Every verifier kill in §6 that cites clause 1, `Prog` or the `PreAlloc` corollary assumes `Eq_f` is the full frozen predicate. If a conjunct is weakened, those kill claims are void until the mutations are re-run.

## 1. DIB

The quantification ranges over every `b` representable as a slice on 32- and 64-bit targets, every `mode ∈ {BmpFile, BareDib, IcoEntryDib, IcoEntryDibSlack}`, every `bm ∈ {BmpFile, BareDib}`, and **every value of the `Policy` record type**, including those reachable only through `cfg(test)` `try_decode_custom_policy`, which replaces `decode_custom_policy`. Functional clauses compare against the selected model.

| ID | Statement |
|---|---|
| A-DIB-1 graceful | `∀b,mode: G ∧ Prog (try_decode_with_steps_logged(b,mode), decode_mp_eq(fmt_part(·), b, mode), Req_DIB(·,mode))`, where `Req_DIB(b,mode) = if mh(b,mode) is Ok(m) then [(Output, 4·m.n)] else []`. `try_decode`, `try_decode_with_steps` discard the log (and, for `try_decode`, the counter). |
| A-DIB-2 graceful, policy | `∀b,bm,p: G ∧ Prog (try_decode_policy_tracked(b,bm,p), Eq_p, Req_DIBp(·,bm,p))`, with `Eq_p(r) = decode_policy_mp_eq(fmt_part(r.0),b,bm,p) ∧ decode_policy_alpha_eq(fmt_part(r.0),b,bm,p) ∧ r.1 == recovered_policy(b,bm,p)`; `try_decode_with_policy(b,bm,profile)` instantiates `p = profile_policy(profile)`, where `Req_DIBp = [(Output, 4·policy_checked_n)]` iff `Reach(b,bm,p)` (`SPEC.md` §2: the complete path passes `mh_policy` and `¬gdk_rgb_reject`, or the Blank second pass passes its four revalidation returns at `dib.rs:8243`, `:8249`, `:8255`, `:8258`). Required lemma: the four second-pass conditions hold together iff `mh_partial(b,bm,p) is Some`. On the second path the reservation is followed only by `fill_blank` and `Ok`, so `OutOfMemory` is the sole alternative to success. `r.1 == false` when `r.0` is `OutOfMemory`. |
| A-DIB-3 one request | `L.len() ≤ 1`. If `req(L) == [(Output,k)]` then `k == 4N`, `1 ≤ N ≤ 16,777,216` and `4N ≤ 67,108,864`, proved before `try_zeroed` from `header_inv_light` (as `alloc_output` does today, `dib.rs:3573–3603`), or from the checked field record on the Blank path (`:8261–8266`). The log replaces the ghost `alloc_count` in `decode_tracked`. |
| A-DIB-4 no allocation after reservation | After `try_zeroed` returns `Ok(out)`, the only operations on `out` are `set`, `len` and indexing (vstd `Vec::set` writes in place). Proof part: `out@.len() == 4N` is invariant through every pixel, RLE, AND and Blank-fill loop (already proved for T5–T8). The absence of other operations is an audit fact (`SPEC.md` §6 call-graph audit). |
| A-DIB-5 termination | T11's argument, on `try_*`. The `OutOfMemory` branch has no loop and returns directly after the failed call. |
| A-DIB-6 work | `∀b,mode: steps ≤ 16·(|b| + t12_checked_n(b,mode))` and `∀b,bm,p: steps ≤ 16·(|b| + policy_checked_n(b,bm,p))`, for **every** outcome. The `4N` initialisation is charged by the caller only after `try_zeroed` returns `Ok`. The bound above is the required one. Optional strengthening (decided to stay optional, O5): `r is OutOfMemory ⇒ steps ≤ 16·|b|`, which needs the pre-allocation work, including the Blank path's failed first pass and revalidation, charged within `16·|b|`. |
| A-DIB-7 strict bridge | Two independent calls, `r1, L1 = try_decode_logged(b,mode)` and `r2, L2 = try_decode_policy_logged(b, bmp_mode(mode), Strict)`, for `mode ∈ {BmpFile, BareDib}`. Their reservations can have different outcomes (a fault injector may fail one and not the other), so log equality is not claimed unconditionally. (a) Model lemma, no runs: `Req_DIB(b,mode) == Req_DIBp(b, bmp_mode(mode), Strict)`, extending the `MH_1_1 := MH(Strict)` identity. Hence `req(L1) == req(L2)` for every outcome, because each `Req` has at most one element and clauses 1–2 force `req(L) == Req` whenever `|Req| ≤ 1`. (b) Antecedent: neither `r1` nor `r2.0` is `OutOfMemory`. Conclusion: `L1 == L2` (both all-`Ok`, by clause 1), `decode_mp_eq(fmt_part(r1), b, mode)`, `decode_policy_mp_eq(fmt_part(r2.0), b, bmp_mode(mode), Strict)`; by the frozen spec-level strict bridge (`policy/THEOREMS-POLICY-DELTA.md` line 28: equal first errors, RGBA bytes and dimensions), the two results agree on the error, or on width, height and RGBA bytes. No allocator coupling is assumed; a statement that also covers the `OutOfMemory` cases would need "both reservations return the same outcome" as an explicit antecedent, and is not part of this delta. |
| A-DIB-8 export for ICO | `try_decode_with_steps_logged` is public, and its `ensures` states A-DIB-1 and A-DIB-6 over the returned log and counter, plus `log_tied(r, L)` in DIB's exported form `dib::dib::log_tied`, so that ICO's `import_dib` can cite them across the crate boundary. The log is returned as `Tracked<dib::alloc_guard::AllocLog>`. |

Frozen rows:

| Frozen | Status |
|---|---|
| T1 safety | Retired on `decode`*. Restated on `try_*` as `Safe'`: no panic, OOB, overflow or unchecked narrowing on any `b`, mode or policy, without the allocation-success antecedent. |
| T2 exact header | Helper, kept. `mh` is computed before any reservation (A-DIB-1 corollary). |
| T3 allocation-call bound | Replaced by A-DIB-3/A-DIB-4. "Exactly one `4N` request on every successful path" holds as one guard call. |
| T4–T8 | Helpers, kept. They run only on `out` from an `Ok` reservation. |
| T9 integrated / T10 precedence | Retired on `decode`*. Restated as A-DIB-1/2 clause 1. Precedence is exact on every non-`OutOfMemory` result. `OutOfMemory` can displace only post-reservation errors (`TruncatedPixels`, `PaletteIndexOutOfRange`, `Rle*`, `InvalidImageSize`, `TruncatedAndMask`, `TrailingBytes`, and policy-specific post-header errors). |
| T11 termination | Retired on `decode`*. Restated as A-DIB-5. |
| T12 linear work | Retired on `decode_with_steps`*. Restated as A-DIB-6; ICO cites A-DIB-8 instead of `decode_with_steps`'s postcondition. |
| T13 success/resource | Retired on `decode`*. Restated under A-DIB-1 clause 1: success implies the one reservation succeeded. |
| Policy T1/T3/T9–T13 | Retired on `decode_with_policy`*. Restated as A-DIB-2/3/5/6, quantified over every record, never conditional on a profile or on success. |

\* "Retired" here and below: the frozen name is removed from the main crate, and the `try_*` restatement replaces its theorem, with no inheritance argument (§0). The private witnesses at `dib.rs:8815–8962` are rewritten over `try_*`.

## 2. TGA

The quantification ranges over every `b` representable on 32- and 64-bit targets.

| ID | Statement |
|---|---|
| A-TGA-1 graceful | `∀b: G ∧ Prog (try_decode_logged(b), decode_eq(fmt_part(·), b), Req_TGA)`, where `Req_TGA(b) = if MH(b) is Ok(m) then [(Output, 4·m.n)] else []`. The reservation is at today's `tga.rs:2561`, after `parse_header` and before the ID/map checks, as frozen R6 permits. |
| A-TGA-2 one request | `L.len() ≤ 1`, `k == 4N ≤ 67,108,864`, proved before the call (today's `tga.rs:2546–2557` arithmetic). |
| A-TGA-3 no allocation after reservation | As A-DIB-4. The colour map stays a borrowed slice (`dec_inv`, `tga.rs:261`), and no lookup table is built. |
| A-TGA-4 termination | T10's argument, on `try_decode`. The `OutOfMemory` branch has no loop. |

Frozen rows: T1 retired on `decode`, restated as `Safe'`. T2–T6 are helpers, kept (the header precedes the reservation; T4/T6 run on the `Ok` buffer). T7/T8/T9 retired on `decode`, restated as A-TGA-1 clause 1; `OutOfMemory` can displace `TruncatedImageId`, `TruncatedColorMap`, `TruncatedPixels`, `Rle*`, `ColorIndexOutOfRange` and `TrailingBytes`, never an R1–R6 header error. T10 retired, restated as A-TGA-4. T11 retired, restated on the concrete `try_decode` call path, which threads the log. The private witnesses at `tga.rs:4571`, `:4609` and `:4939` are rewritten over `try_decode`.

## 3. ICNS

This section assumes decision D2: `Entry::type_hex: [u8; 8]`, compared to the frozen model through `hex_str(a) = Seq::new(8, |i| a[i] as char)` (`SPEC.md` §4). Every executable-`Entry` occurrence of `e.type_hex@` in `icns_verus/src/model.rs` (15 lines: 167, 183, 184, 1037, 1152, 1358, 1659, 2251, 2278, 2287, 3107, 3606, 3631, 3928, 3948; five of them compare two executable entries and change on both sides) is retargeted to `hex_str`, with the lemmas and witnesses that use them; `MpEntry` occurrences are untouched. The inventory is in `SPEC.md` §4 and is part of this step's scope. The relations below are those of `SPEC.md` §2: `parses_to` (through `entry_eq_fields`, retargeted), `rle_result_eq`, `rgb_decoded_eq`, `mask_decoded_eq`, `combine_decoded_eq`, each on `fmt_part(r)`.

| ID | Statement |
|---|---|
| A-ICNS-1 parse | `∀b: G ∧ Prog (try_parse_logged(b), parses_to(b, fmt_part(·)), Req_P)`, where `Req_P(b) = [(EntryList, min(4096, (|b|−8)/8))]` iff R2–R4 pass, else `[]`. A zero-entry request (`8 ≤ |b| ≤ 15`) is in `L` with `ok == true` and reaches no allocator. `parses_to` maps the model's `MpError` through `exec_err` and compares entries through `entry_eq_fields`, so `type_hex` is compared through `hex_str`. `OutOfMemory` can displace every in-loop parse error (`TruncatedEntryHeader`, `InvalidEntryLength`, `TooManyEntries` …); decision D5 accepts this, including when enough memory would remain to report the format error. |
| A-ICNS-2 entry room | Loop invariant: `entries.view().len() == os.len() ≤ (offset−8)/8` (every accepted entry advances `offset` by `length ≥ 8`) and `entries.room() == min(4096, (|b|−8)/8)`. At each `Reserved::push`, `len < room` follows from the `TooManyEntries` check (`len < 4096`) and from `offset + 8 ≤ |b|` (the `TruncatedEntryHeader` check). No regrowth. |
| A-ICNS-3 rle | `∀s, n ∈ {256,1024,2304,16384}, |s| ≤ 67,108,864: G (try_rle(s,n), rle_result_eq(fmt_part(·), mr(s,n)), Req_R)`, where `Req_R = [(Channel(0),n),(Channel(1),n),(Channel(2),n)]`. If `Channel(1)` fails, `Channel(0)`'s vector is dropped before return. T2's "at most three `n`-byte channel buffers" becomes this trace. |
| A-ICNS-4 rgb | `∀b,E,i, P(b,E): G ∧ Prog (try_decode_rgb_logged(b,E,i), rgb_decoded_eq(fmt_part(·),b,E,i), Req_C)`, where `Req_C = []` on index/type errors, `Req_R` on an RLE error, and `Req_R ++ [(RgbOut,3n)]` otherwise. |
| A-ICNS-5 mask | `∀b,E,i, P(b,E): G ∧ Prog (try_decode_mask_logged(b,E,i), mask_decoded_eq(fmt_part(·),b,E,i), Req_A)`, where `Req_A = [(MaskData,n)]` iff the length check passes. `try_copy`'s `ensures out@ == payload@` replaces the push loop at `icns.rs:1133–1144`. |
| A-ICNS-6 combine | `∀b,E,ci,mi, P(b,E): G ∧ Prog (try_combine_logged(b,E,ci,mi), combine_decoded_eq(fmt_part(·),b,E,ci,mi), Req_J)`, where `Req_J = Req_C(ci) ++ (if MC Ok then Req_A(mi)) ++ (if both Ok ∧ sides equal then [(CombineOut,4n)])`. Peak live bytes (an ownership fact, not a verified clause; `SPEC.md` §4): `6·n_c` during the colour decode, `3·n_c + n_m` until the side check whether or not the sides match, `8n` after a matching check; at most 131,072 bytes in every case. |
| A-ICNS-7 termination | Parse: `offset` strictly increases by ≥ 8 (unchanged). Every `OutOfMemory` branch has no loop. |
| A-ICNS-8 type code | `Entry::type_code` keeps its frozen contract with `hex_str(self.type_hex)` for `self.type_hex@`, and allocates nothing. `hex4_bytes(code)` ensures `hex_str(result) == model::hex4(code)`. |

Frozen rows: T1 retired on `parse`, restated as `Safe'` with A-ICNS-1/2 in place of "bounded output-entry storage (at most 4,096 records, each with an eight-digit type string)"; no string is requested. T2 (helper, `rle`) becomes A-ICNS-3. T3/T4/T5 retired on the decoders, restated as `Safe'` with the request bounds above. T6 retired on `parse`, restated on `try_parse`, with "every `type_hex` is eight lowercase hex digits" read through `hex_str`. T7/T8 retired on `parse`, restated as A-ICNS-1 clause 1. T9 (helper) restated over `try_rle`. T10–T13 retired on `decode_rgb`/`decode_mask`/`combine`, restated as A-ICNS-4/5/6 clause 1; T13's concrete-call provenance proof carries the log. The private T-wrappers (`parse_with_t8`, `rle_after_call`, `t9_rle_exact`, `decode_rgb_with_t10` …, `icns.rs:1297–1483`) are rewritten over `try_*` results.

## 4. ICO

ICO's source is tracked on `origin/main` (`ico/ico_verus/src/{ico.rs,lib.rs,model.rs,model_vectors.rs,witnesses.rs}`, 303 verified obligations per `ico/README.md`). This section depends on A-DIB-1, A-DIB-6 and A-DIB-8, and so on the DIB delta landing in the DIB crate that ICO's `Cargo.toml` path dependency (`../../dib/dib_verus`) resolves to. Decided (O6): ICO is implemented only after DIB's verified logged export is on that branch, or on a merged branch, with the DIB source hash recorded in ICO's audit report. ICO's site enum is `EntryList | Dib(dib::AllocSite)`. The relations are `parse_eq(b, fmt_part(r))` and `md_eq(b, E, i, fmt_part(r))` (`origin/main:ico/ico_verus/src/model.rs:218`, `:949`).

| ID | Statement |
|---|---|
| A-ICO-1 parse | `∀b: G ∧ Prog (try_parse_logged(b), parse_eq(b, fmt_part(·)), Req_P)`, where `Req_P(b) = [(EntryList, count)]` iff `H(b)` is `Ok`. The guard call replaces `Vec::with_capacity(header.count)` at `ico.rs:773`, and `Reserved::push` replaces `:817`; `len < room` holds from the loop invariant `i < header.count`. The ghost `alloc` counter (`parse_alloc_calls`) becomes `L.len()`. |
| A-ICO-2 decode | `∀b,E,i:u32: G ∧ Prog (try_decode_entry_logged(b,E,i), md_eq(b,E,i,fmt_part(·)), Req_D)`, where `Req_D = []` if `MD` returns before the DIB call, and otherwise `map_dib`-renamed `Req_DIB(b[a..z], IcoEntryDibSlack)`, i.e. `Output ↦ Dib(Output)`. ICO creates its own log with `new_log()`. On the DIB path, the call at `ico.rs:1192` becomes `dib_verus::try_decode_with_steps_logged`, and its result `r0` and log `L0` go to `import_dib` (`SPEC.md` §3.1), which requires ICO's log empty and `log_tied(r0, L0)` (from A-DIB-8) and ensures ICO's log becomes `map_dib(L0)`. Proof of clause 2: if `r0 = Err(OutOfMemory(o))`, `log_tied` makes `L0`'s last entry `(o.site(), o.elements(), false)` with all earlier entries `Ok`; `import_dib` returns `o'` with `(o'.site(), o'.elements()) == (Dib(o.site()), o.elements())` and makes ICO's last entry `(Dib(o.site()), o.elements(), false)`; prefix-of-`Req_D` follows from A-DIB-1 clause 2 under `map_dib`. Proof of clause 1: if `r0` is not `OutOfMemory`, `L0` is all-`Ok` and equals `Req_DIB` as requests, and `lemma_md_eq_dib` (ICO `model.rs:967`) applies unchanged to `fmt_part(r0)` because A-DIB-1 gives `decode_mp_eq(fmt_part(r0), slice, IcoEntryDibSlack)`. A DIB `OutOfMemory` is never mapped to `Error::Dib(_)`. |
| A-ICO-3 work | T9's bounds hold for every outcome: `steps(parse) ≤ 16·(|b|+1)`, `steps(wrapper) ≤ 16·(|b|+1)`, and the combined sum (`decode_entry_with_steps`'s `r.1 + r.2` postcondition) uses A-DIB-6 via A-DIB-8 on the `OutOfMemory` path as well. |
| A-ICO-4 parse_with_steps | `try_parse_with_steps` carries `parse_with_steps`'s three postconditions (`ico.rs:844–852`) with `r.2@ == L.len() ≤ 1` in place of the ghost allocation count. |

Frozen rows (all on public calls or their witnesses, so all retired on the frozen names): T1 restated as `Safe'`. T2/T3/T4/T6 restated as A-ICO-1/2 clause 1; record statuses are unaffected. T5 restated on `try_decode_entry`: A-DIB-8 gives `decode_mp_eq(fmt_part(r0), slice, IcoEntryDibSlack)` when `r0` is not `OutOfMemory`, which is `lemma_md_eq_dib`'s unchanged `requires`. T7: "wrapper never allocates a pixel vector" still holds, since its only reservation is DIB's. T8 termination restated on `try_*`. T9 becomes A-ICO-3/4. T10 becomes A-ICO-1's one-element log, with A-ICO-2's log equal to DIB's. T11's public-call witnesses (`witnesses.rs:86`, `:142`, `:201`) and the public theorem functions `t5_decode_exact`, `t6_*`, `t7_dib_output_bound`, `t9_*` and `lib.rs::skeleton` are rewritten over `try_*` and stay `pub`, with contracts retargeted to `TryError` (decided, O1).

## 5. Runtime checks (falsification gates for A-STD and the guard binding)

These tests are regression checks of A-STD-1/2/3 and of the guard body's `Err` binding on the pinned toolchain and allocator (`SPEC.md` §3). They do not prove anything. They run per core over every frozen vector and the corpus, in a test binary whose `#[global_allocator]` wraps `System` and records events within one `try_*` call.

- **Logical versus physical trace.** Let `L⁺` be `L` restricted to entries with `k ≥ 1`. The recorded `alloc`/`alloc_zeroed` events must correspond one-to-one and in order with `L⁺`. For each pair, the event's `Layout` has `align == align_of::<T_site>()` and `size ≥ k·size_of::<T_site>()`. Equality is expected on the pinned std and recorded per run; a larger size is allowed rounding; a smaller size fails. After a successful reservation, `capacity() ≥ k` (A-STD-1). There must be **no `realloc`** events (A-STD-2). Entries with `k == 0` must produce no event. Inputs with `PreAlloc` must produce no event.
- **Fault injection by nonzero reservation.** For each `j` in `1..=|L⁺|`, the allocator returns null on the `j`-th `alloc` event of the call. The result must be `Err(OutOfMemory)` with `(site, elements)` equal to `L⁺[j]`, the process must survive, and live bytes must return to the pre-call value after the result is dropped (no leak). A second mode fails every request of at least T bytes, for T ∈ {1, 4096, 65,536, 64 MiB}. An `Err` from the guard with no corresponding null-returning event fails the test, since under the size precondition it would mean capacity overflow (A-STD-3) or a guard that fabricates `Err`.
- **ICO through DIB.** Fault injection on the DIB reservation inside `try_decode_entry` must return ICO `OutOfMemory` with site `Dib(Output)` and DIB's element count, and the returned ICO log (in a test build that exposes `try_decode_entry_logged`) must end with that failed entry.
- **ICNS peak live bytes.** The counting allocator records the maximum live bytes per `try_combine` call and checks it against `SPEC.md` §4's three phases, including a mismatched-sides input (16×16 colour, 128×128 mask: 17,152 bytes before `TypeMismatch`).
- **Expected sizes per target.** The `size_of::<Entry>()` values used in these checks come from the per-target layout declaration (gate F2), not from constants in the test.
- **Abort shim crates (only if built).** These tests belong to the shim crate, not to the main crate's gates. In a child process, a shim function under fault injection must terminate. On the pinned std and target with the default alloc-error hook and without `-Zoom=panic`, the check is stricter: exit by SIGABRT, and stderr contains `memory allocation of <k·size_of::<T_site>()> bytes failed`. Elsewhere only termination without a normal return is checked.

## 6. Mutation checks

Run each mutation in a disposable copy with all theorems enabled, as in the existing mutation protocols. A kill counts only at the gate named. "Build" means the mutation fails to type-check. "Audit" means the call-graph audit unless stated.

| # | Mutation | Must fail at |
|---|---|---|
| A1 | Ignore the reservation result: `let _ = v.try_reserve_exact(n); v.resize(n,0)` outside `alloc_guard` | audit (`try_reserve_exact`, `resize` reached outside the guard); fault test (abort) |
| A2 | Inside `alloc_guard`, return `Ok` regardless of `try_reserve_exact`'s result | fault-injection test (`push` growth aborts instead of `OutOfMemory`) and the guard hash check; **not** the verifier (vstd does not constrain the result) |
| A2′ | Inside `alloc_guard`, return `Err` after `try_reserve_exact` returned `Ok` | fault-injection test (`Err` with no null-returning event), counting test, and hash check; not the verifier |
| A3 | Reserve one element short (`try_zeroed(Output, 4N−1)`, ICNS room `−1`) | verifier: A-DIB-3/A-TGA-2 `k == 4N`; A-ICNS-2 `len < room` |
| A4 | Reintroduce `vec![0u8; 4*n]` at the DIB/TGA site | audit (`from_elem` reachable); source lint; fault test (abort) |
| A5 | Return `OutOfMemory` on a header error | build (`OutOfMemory` cannot be constructed outside `alloc_guard`); with a stashed `o` from a guard call, verifier: `PreAlloc` corollary |
| A6 | Move the reservation before the `OutputTooLarge`/R6 check | verifier: clause 3 bound unprovable before the check; `PreAlloc ⇒ L == []` |
| A7 | Report the wrong size (`try_zeroed(Output, N)` with `4N` written later) | verifier: clause 1 `req(L) == Req(x)` and A-DIB-3 |
| A8 | Retry on failure (`while` loop around the guard call) | verifier: clause 2 (last entry must be the only failed one) and termination without `decreases` |
| A9 | Leak on the `OutOfMemory` path (`mem::forget(channels)` in ICNS `try_rle`) | audit (`mem::forget` reached); fault-injection live-byte check |
| A10 | ICNS: restore `String` `type_hex` / `hex4` | build (`Entry` is `[u8;8]`); audit (`String::push` reached) |
| A11 | ICNS: build the entry list with `Vec::new` + `push` (regrowth) | build (`Reserved::push` needs a `Reserved`); with a raw `Vec`, audit (`Vec::push` outside the guard); counting test sees `realloc` |
| A12 | ICNS: swap the colour and mask decode order in `try_combine` | verifier: `Req_J` order (A-ICNS-6) and T12 precedence |
| A13 | ICO: map DIB `OutOfMemory` to `Error::Dib(..)`, or drop it | verifier: A-ICO-2 clause 1 (`MD` never yields such an error, and the log has a failed entry) |
| A14 | DIB policy: second buffer for Blank fill | verifier: A-DIB-3 `L.len() ≤ 1` if through the guard; audit otherwise |
| A15 | Return `recovered = true` with `OutOfMemory` | verifier: A-DIB-2 |
| A16 | Glue: map core `OutOfMemory` to `CORRUPT_IMAGE` (gdk) or `ProcessError::expected` (glycin) | glue fault-injection test expecting `INSUFFICIENT_MEMORY` / `is_out_of_memory()` |
| A17 | Return `OutOfMemory` (a stashed `o`) after the guard returned `Ok` | verifier: clause 2 (`¬L.last().ok` fails) |
| A18 | Call `new_log()` again mid-function to hide a reservation | ghost-reading check (`new_log` count); counting test (extra event) |
| A19 | Swallow a guard `Err` and continue to a format error | verifier: clause 1 (`∀j: L[j].ok` fails) |
| A20 | DIB Blank path: reserve before the `masks_missing` size check (`dib.rs:8258`) | verifier: `PreAlloc` corollary on a `TruncatedMasks` input whose header size is neither 40 nor 64; if the model admits no such input, report the mutation as equivalent |
| A21 | ICO: pass a fresh empty DIB log (`dib::alloc_guard::new_log()`) to `import_dib` in place of the returned one | verifier: `import_dib`'s `requires log_tied` fails when `r0` is `OutOfMemory`; otherwise A-ICO-2 clause 1 `req(L) == Req_D` fails on any input that reaches DIB's reservation |
| A22 | ICO: drop `import_dib` and return `Error::Dib`/`Ok` from `r0` directly, leaving ICO's log empty | verifier: A-ICO-2 clause 1 (`req(L) == Req_D` with `Req_D` non-empty); with a DIB `OutOfMemory`, build (no other constructor of ICO's `OutOfMemory`) |
| A23 | Reintroduce an aborting entry point in a main crate (a frozen-named wrapper calling `handle_alloc_error` or `process::abort` on `OutOfMemory`) | audit (`handle_alloc_error`/`process::abort` reached outside `push`'s allowlisted growth path); source lint |

The frozen mutation lists stay as they are. DIB mutation 20 ("second full-size scratch buffer") and ICO M21 ("second scratch vector without ghost charge") remain audit kills, now by the call-graph audit, and the counting test also catches them.

## 7. Cost and order

DIB and TGA each have **one** guard call site. TGA is cheaper than DIB: one public function, no policy record, no step counter, no near-rlimit body. DIB has three call paths (`decode_tracked:7767`, `decode_policy_tracked:8600`, `decode_partial_metadata:8268`), three public functions plus the logged export, and the private witnesses. `decode_policy_tracked` runs close to its rlimit (comment at `dib.rs:8504`), so widening its return type and threading the log may need a split. ICO has one own site, `import_dib` and an implementation on `origin/main`; it waits on DIB's `try_decode_with_steps_logged` being on the branch its path dependency resolves to (O6). ICNS is the most work: seven call sites, a public `Entry` type change with the 15-line `model.rs` retargeting and a glycin-icns follow-up, the parse loop invariant, the `[Vec<u8>;3]` channel return, the combine composition, and roughly ten T-wrapper functions. Every step also migrates its core's consumers (`SPEC.md` §1.4) and may add its `<core>_abort_shim` crate, neither of which is verified work.

Sequencing: step 0 (guard and tooling) comes first. The TGA policy implementation, separate ongoing work that edits `tga.rs`, must finish before step 1 starts. The pre-freeze gates F1–F3 (`SPEC.md` §8) must pass before the delta is frozen; F1 needs step 0's audit prototype.

Effort in units of "one DIB metadata-only policy gate" (`THEOREMS-POLICY-DELTA.md`'s smallest unit):

| Step | Work | Effort |
|---|---|---|
| 0 | `alloc_guard` (sealed log, verified once, text-identical per core), `tools/alloc_audit` prototype on rustc 1.99.1 and the reviewed x86_64 allowlists, counting/fault-injection harness | 2–2.5 |
| 1 | **TGA** pilot, after the TGA policy implementation finishes: one site (plus the policy decoder's one reservation, `try_`-only), return-type change to `TryError`, A-TGA-1–4, mutations A1–A9, A17–A19 | 0.5–1 |
| 2 | **DIB**: one site, three call paths, Blank reachability lemma, policy universality, A-DIB-1–8, A14/A15/A20 | 1–2 |
| 3 | Glue and consumers: gdk-pixbuf BMP mapping, glycin dib/tga mapping, migration of the DIB/TGA consumers in `SPEC.md` §1.4, optional `dib_abort_shim`/`tga_abort_shim` (unverified; tests only) | 0.5 |
| 4 | **ICO** on `origin/main` or a merged branch (O6), after DIB step 2: `import_dib`, A-ICO-1–4, A21/A22 | 0.5–1 |
| 5 | **ICNS**: seven sites, `Entry` change, witness restatements, glycin-icns `legacy_kind` and `try_*` migration | 2–3 |

Main risks:

- rlimit pressure in `decode_policy_tracked`.
- A-STD drifting with the toolchain. The counting test catches that drift on the pinned toolchain; no proof does.
- The call-graph audit's allowlist for precompiled std leaves is reviewed by hand; an item misclassified as non-allocating is a hole only the falsification gates can expose. The named reviewer, the recorded hash and the re-review rule on any toolchain change bound this risk without removing it.
- `rustc_driver` internals change between compiler releases, so the audit tool is tied to rustc 1.99.1 and must be ported with any Verus toolchain bump.
- i686 stays unaudited until its std component is installed (`SPEC.md` §6).
- Verus support for a `tracked` struct with a private ghost field across modules and crates (gate F3), and for `global layout` on the entry structs (gate F2). If the first fails, the theorem shape must be revised before freezing; if the second fails, the fallback is a monomorphic `try_with_room_entry`.
