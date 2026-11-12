# Fallible allocation: specification delta (draft, not frozen; third revision, owner decisions of 2026-11-12, see `REVIEW-SOL.md`)

This delta makes every heap allocation that the four verified cores (DIB, TGA, ICNS, ICO) make on their production call paths fallible. Each main crate exports only verified `try_*` entry points returning `TryError { Format(Error), OutOfMemory(..) }`; a failed reservation yields `Err(OutOfMemory)`, which sits **outside** every frozen error list. The frozen function names (`decode`, `parse`, `combine` and the rest) are **removed** from the main crates (decision D1), so no main crate contains a function that aborts on allocation failure. Their verified API theorems are retired and replaced by the `try_*` theorems, with no wrapper in between (§1.2). Aborting entry points, if anyone wants them, live in separate optional and unverified `<core>_abort_shim` crates outside the main crates' proof and audit boundary (§1.3). The public API is explicitly unstable, so these breaking changes need no compatibility gate (decision D9). No frozen spec, vector or hash file changes, and no `THEOREMS.md`/`THEOREMS-DRAFT.md` manifest is edited. The new obligations live in `THEOREMS-ALLOC-DELTA.md`. User-facing behaviour per integration is in `TABLE.md`.

Every frozen `Safe` definition conditions on allocation success: DIB `THEOREMS.md` line 3 and T1, frozen DIB 1.1 R17, TGA `THEOREMS.md` line 15 and R17, ICNS `THEOREMS.md` line 17, and ICO `THEOREMS.md` line 3 on `origin/main` ("Safety conditions exclude allocator exhaustion"). This delta removes that antecedent for the `try_*` entry points only. The remaining antecedents are an ordinary stack, an allocator that honours the `GlobalAlloc` contract (it returns null or a valid block for the requested `Layout`), and the trusted std contract A-STD (§3).

Source locations: DIB, TGA and ICNS refer to the `dib-policy` working tree. ICO refers to `origin/main:ico/ico_verus/src/{ico.rs,lib.rs,witnesses.rs}`; the `dib-policy` worktree carries only the `*.m4b` milestone snapshots of ICO, but the implementation (303 verified obligations, `origin/main:ico/README.md`) is tracked on `main`.

## 1. The outcome and the API

Each core gains, in its `alloc_guard` module (§3):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfMemory { site: AllocSite, elements: usize }   // fields private to alloc_guard
impl OutOfMemory { pub fn site(&self) -> AllocSite; pub fn elements(&self) -> usize; }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryError { Format(Error), OutOfMemory(OutOfMemory) }
```

- `Error` is the frozen enum, unchanged. `TryError::name()` returns the frozen spelling for `Format(e)` and `"OutOfMemory"` otherwise. It is `'static` and never allocates.
- `AllocSite` is a closed per-core enum naming each reservation site (§4). `elements` is the element count of the failed request; its byte size is `elements * size_of::<T_site>()` for the site's element type, so at every `u8` site the count is the byte count.
- Code outside `alloc_guard` cannot construct an `OutOfMemory`: its fields are private, and the only constructors are the guard's reservation functions, which return one only from the `Err` arm of `try_reserve_exact` (§2, §3). ICO has one more constructor, `import_dib` in ICO's `alloc_guard/sites.rs`, which consumes DIB's returned log, appends its events to ICO's log, and wraps a DIB `OutOfMemory` (§3.1). There is no `from_dib`.
- `site()` and `elements()` carry `#[verifier::when_used_as_spec]` spec counterparts, so `ensures` clauses in this and other crates can name them.
- `OutOfMemory` is not a byte-format error. No vector file gains it, and the Python oracles do not model it.

### 1.1 Inventory of public functions

| Core | Verified `try_*` API (the crate's whole public API) | Frozen name, removed from the main crate | Public theorem witnesses that call a frozen name today |
|---|---|---|---|
| DIB | `try_decode`, `try_decode_with_steps`, `try_decode_with_policy`; `try_decode_with_steps_logged` (exports the reservation log for ICO) | `decode` (`dib.rs:7944`), `decode_with_steps` (`:7975`), `decode_with_policy` (`:8179`) | private `t*` witnesses at `dib.rs:8815–8962`; `cfg(test)` `decode_custom_policy` (`:8967`) |
| TGA | `try_decode` | `decode` (`tga.rs:2466`) | private witnesses at `tga.rs:4571`, `:4609`, `:4939` |
| ICNS | `try_parse`, `try_decode_rgb`, `try_decode_mask`, `try_combine` | `parse` (`icns.rs:371`), `decode_rgb` (`:935`), `decode_mask` (`:1075`), `combine` (`:1160`) | private T-wrappers at `icns.rs:1297–1483`; `combine`'s own calls at `:1200`, `:1220` |
| ICO | `try_parse`, `try_parse_with_steps`, `try_decode_entry`, `try_decode_entry_with_steps` | `parse` (`ico.rs:719`), `parse_with_steps` (`:844`), `decode_entry` (`:969`), `decode_entry_with_steps` (`:1263`) | **public** `t5_decode_exact` (`:1437`), `t6_parse_directory_priority` (`:1514`), `t6_decode_ordering` (`:1538`), `t7_dib_output_bound` (`:1591`), `t9_parse_linear_work` (`:1632`), `t9_wrapper_linear_work` (`:1647`), `witnesses::t11_*` (`witnesses.rs:86`, `:142`, `:201`), `lib.rs::skeleton` (`lib.rs:19`, calls `dib_verus::decode_with_steps`) |

The frozen names no longer exist, so every witness in the last column, and ICO's call to `dib_verus::decode_with_steps` at `ico.rs:1192`, is retargeted to the `try_*` function and restated over `TryError`. DIB's `cfg(test)` `decode_custom_policy` becomes `try_decode_custom_policy`. The ICO witnesses stay `pub` with retargeted `TryError` contracts (decided, O1). ICO's T5 currently relies on DIB's postcondition `decode_mp_eq` on `decode_with_steps` through `lemma_md_eq_dib` (`origin/main:ico/ico_verus/src/model.rs:967`); after the delta it relies on the same predicate, now on `fmt_part` of `try_decode_with_steps_logged`'s result when that result is not `OutOfMemory` (§2, `THEOREMS-ALLOC-DELTA.md` A-ICO-2).

The frozen TGA policy decoder (`tga/policy/POLICY-SPEC.md`, frozen 2026-11-12) has no implementation yet. Like the rest of the main crate it is `try_`-only, and its single output reservation goes through the guard. An aborting variant, if wanted, goes in `tga_abort_shim` (§1.3; O4). That implementation is separate ongoing work that edits `tga.rs`; it must finish before the TGA allocation pilot starts (§7).

### 1.2 Frozen names removed; theorems retired

The frozen names are deleted from the main crates, not wrapped. Consequences:

- **The verified API theorems on the frozen names are retired.** This covers DIB T1, T3, T9–T13 and the policy delta's T1, T3, T9–T13 on `decode`/`decode_with_steps`/`decode_with_policy`; TGA T1, T7–T11 on `decode`; ICNS T1, T3–T8 and T10–T13 on `parse`/`decode_rgb`/`decode_mask`/`combine`; ICO T1–T11 on `parse`/`parse_with_steps`/`decode_entry`/`decode_entry_with_steps` and their witnesses. Theorems stated on internal helpers (DIB T2, T4–T8; TGA T2–T6; ICNS T2 and T9 on `rle_decode`) remain verified, retargeted where a helper's return type gains `TryError`.
- **The verified `try_*` theorems replace them directly** (`THEOREMS-ALLOC-DELTA.md` §1–§4). A main crate has no wrapper, so there is no inheritance argument and no unverified source in its proof report; its trust items are A-STD, the guard binding and the audit allowlist.
- **Under allocation success** nothing changes for a migrated caller: `fmt_part` of the `try_*` result satisfies the frozen predicate `Eq_f` (§2, clause 1), so mapping `Format(e)` to `e` reproduces the frozen result exactly.
- **Under allocation failure** the `try_*` function returns `OutOfMemory`. Nothing in a main crate terminates the process on allocation failure; the audit makes every aborting allocation path a finding (§6).

Inside the verified code, a failed request is never retried and never converted into a format error.

### 1.3 Optional abort shim crates

Aborting entry points, if wanted, live in a separate crate per core: `dib_abort_shim`, `tga_abort_shim`, `icns_abort_shim`, `ico_abort_shim`. Each depends on its main crate; no main crate depends on a shim. A shim crate is unverified, outside the main crate's proof and call-graph audit, and its documentation says: "aborts the process on allocation failure; for callers who don't care about recovery and don't need API stability." Building any of them is optional. Each function is at most five lines:

```rust
// dib_abort_shim/src/lib.rs (unverified, not audited)
pub fn decode(b: &[u8], mode: Mode) -> Result<Decoded, Error> {
    match dib_verus::try_decode(b, mode) {
        Ok(d) => Ok(d),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(o)) => abort_oom(o),
    }
}
```

`abort_oom(o)`, defined in each shim crate, computes `Layout::array::<T_site>(o.elements())` for the site's element type: `u8` at every site except the `EntryList` sites, whose element type is the core's public `Entry`. On `Ok(layout)` it calls `std::alloc::handle_alloc_error(layout)`; on `Err` (excluded by the site caps, but nothing here is verified) it calls `std::process::abort()`. `o.elements() >= 1` always, because zero-element reservations never fail (§3), so the layout is never zero-sized. The `_with_steps` shims pass the `Ghost<nat>` through unchanged.

What a shim does, argued by review of its text and not by Verus: whenever `g(x)` returns, its result is `fmt_part(try_g(x))`, which satisfies `Eq_f(x, ·)`, the predicate the retired theorem stated on `g` (for `decode_with_policy` the shim passes `r.1` through, giving all three frozen postconditions). If a reservation fails, the process terminates. DIB, TGA and ICO reserve at the same program point and size as today's `vec!`/`Vec::with_capacity`, so a shim's abort point and requested size match today's frozen functions. ICNS differs: `try_parse` reserves the entry list once before its loop, where today `Vec::new()` allocates nothing until the first `push` and each `type_hex` `String` allocates after its entry's checks. So under allocation failure `icns_abort_shim::parse` can abort on a malformed file where today `parse` returns `InvalidEntryLength`, for example a 16-byte file whose first entry declares length 7, and the failing request sizes differ because today's list, string and mask growth make several small requests where the delta makes one exact request.

The exact message and signal are promised only under these conditions: the pinned std and target (§3), std's default alloc-error hook, and no `-Zoom=panic`. Then stderr shows `memory allocation of <N> bytes failed` and the process gets SIGABRT. Under any other configuration a shim promises only that it does not return normally. The shim crate's own test checks this (`THEOREMS-ALLOC-DELTA.md` §5). These shims are unrelated to the C-API shims of decision D3 (`TABLE.md`), which write a fixed non-allocating message and call `process::abort()` directly.

### 1.4 Consumers to migrate

Every existing caller of a frozen name stops compiling when its core's step lands, so the step migrates it to `try_*` or to the shim crate. Production glue goes to `try_*`, because it must map `OutOfMemory` to its host's error (`TABLE.md`). Tests inside a main crate go to `try_*` (a dev-dependency on the shim crate would build a second copy of the main crate) and treat `OutOfMemory` as a test failure. Leniency runners and tools may use either. Inventory on the `dib-policy` tree, the glycin fork `DpEpsilon/glycin` `verified-loaders` at `7344b95` (`~/projects/glycin-plan/glycin`), and `origin/main` for ICO:

| Consumer | Frozen-name calls | Target |
|---|---|---|
| gdk-pixbuf BMP module | `gdkpixbuf-loaders/src/bmp.rs:36` `decode_with_policy`; doc comments `bmp.rs:3`, `lib.rs:5` | `try_decode_with_policy` (`TABLE.md`) |
| glycin-dib loader | `glycin-loaders/glycin-dib/src/lib.rs:54` `dib::decode` | `try_decode` → `ProcessError::out_of_memory()` |
| glycin-dib tests | `glycin-dib/tests/robustness.rs:46`, `:55`; `tests/corpus.rs:50` | `try_decode` or `dib_abort_shim` |
| glycin-tga loader | `glycin-tga/src/lib.rs:44` `tga::decode` | `try_decode` → `ProcessError::out_of_memory()` |
| glycin-tga tests | `glycin-tga/tests/robustness.rs:36` | `try_decode` or `tga_abort_shim` |
| glycin-icns loader | `glycin-icns/src/lib.rs:152` `icns::combine`, `:229` `icns::parse`; doc comments `:29`, `:97` | `try_combine`, `try_parse` → `ProcessError::out_of_memory()` |
| glycin-icns tests | `glycin-icns/tests/robustness.rs:56`, `:59`; `tests/loader.rs:365`; `tests/vectors.rs:95`, `:172` | `try_*` or `icns_abort_shim` |
| Core conformance and corpus tests | `dib/dib_verus/tests/{conformance.rs, corpus.rs, policy_conformance.rs}`; `tga/tga_verus/tests/conformance.rs`; `icns/icns_verus/tests/conformance.rs` (7 calls); `origin/main:ico/ico_verus/tests/conformance.rs:214`, `:227`, `:256`, `:318`, `:323` | `try_*` |
| Leniency runners | `leniency/bmp/src/main.rs:32` `dib::decode`; `leniency/shared/src/main.rs:61` `tga::decode`, `:63` `icns_verus::parse`, `:67` `icns_verus::combine` | `try_*` or the shim crates |
| Differential harness | `tools/diff-harness/src/main.rs:99`, `:105`, `:108`, `:111` (ICNS `parse`, `decode_rgb`, `decode_mask`, `combine`) | `try_*` or `icns_abort_shim` |
| Benchmarks | none call a core directly; the fork's `benches/` drives the loaders through glycin | no source change; rerun after the loaders move |

Not migrated: the frozen ICO mutation copies `ico/mutation/{base,m1–m34}` call `dib_verus::decode_with_steps`; they are evidence for the frozen ICO proof and are not rebuilt against the new DIB. The `*.p1`/`*.p2*` test snapshots in `dib/dib_verus/tests/` are not built. The ICNS `type_hex` follow-up in glycin-icns (§4) is separate from this list.

## 2. Semantics

Fix a core and a `try_*` entry point `f`, where `x` stands for the input (`b`, plus `mode`, `bm`, `policy`, `E` and indices as applicable). DIB policies are quantified over every value of the `Policy` record type, not only the named profiles; this includes the records reachable only through `cfg(test)` `try_decode_custom_policy`.

**Result relations.** An equation `r == lift(M(x))` does not type-check in Verus: the executables return `Vec`/`Decoded` values, the models return `Seq`s and tuples, and ICNS's parse model has its own error enum (`MpError`, `icns/icns_verus/src/model.rs:321`, mapped by `exec_err`). The frozen proofs already bridge each gap with an executable-to-model predicate, so the delta states every functional clause through those predicates and never through an equation. For a `try_*` result `r` that is not `OutOfMemory`, `fmt_part(r)` is the frozen-typed result: `Ok(v) ↦ Ok(v)`, `Err(Format(e)) ↦ Err(e)`. The relation `Eq_f(x, r)` is:

| Entry point | `Eq_f(x, r)` (frozen predicate, unchanged) |
|---|---|
| DIB `try_decode`, `try_decode_with_steps`, `try_decode_with_steps_logged` | `decode_mp_eq(fmt_part(r), b, mode)` (`dib.rs:6780`, against `model::mp`, `model.rs:5501`) |
| DIB `try_decode_policy_tracked` (record `p`) and `try_decode_with_policy` (`p = profile_policy(profile)`) | `decode_policy_mp_eq(fmt_part(r.0), b, bm, p) ∧ decode_policy_alpha_eq(fmt_part(r.0), b, bm, p) ∧ r.1 == recovered_policy(b, bm, p)` (`dib.rs:7998`, `:8015`; the three postconditions of `decode_with_policy`, `:8179–8184`) |
| TGA `try_decode` | `decode_eq(fmt_part(r), b)` (`tga.rs:281`) |
| ICNS `try_parse` | `parses_to(b, fmt_part(r))` (`model.rs:1166`: `MpError` through `exec_err`, entries through `entries_eq_models`/`entry_eq_fields`, retargeted to `hex_str` in §4) |
| ICNS `try_rle` (helper) | `rle_result_eq(fmt_part(r), mr(s, n))` (`model.rs:3461`) |
| ICNS `try_decode_rgb`, `try_decode_mask`, `try_combine` | `rgb_decoded_eq`, `mask_decoded_eq`, `combine_decoded_eq` on `fmt_part(r)` (`model.rs:3642`, `:3581`, `:3711`) |
| ICO `try_parse`, `try_parse_with_steps` | `parse_eq(b, fmt_part(r))` (on `r.0` for the steps variant) (`origin/main:ico/ico_verus/src/model.rs:218`) |
| ICO `try_decode_entry`, `try_decode_entry_with_steps` | `md_eq(b, E, i, fmt_part(r))` (ICO `model.rs:949`) |

`fmt_part` is a spec function in each core's `try` module, outside the guard. Every kill claim in `THEOREMS-ALLOC-DELTA.md` §6 that names "clause 1" or `Prog` as its gate assumes `Eq_f` is exactly the frozen predicate listed here. If a conjunct is dropped (for example `decode_policy_alpha_eq`), those kill claims are void and the mutations must be re-run.

**Two traces.** The *logical reservation trace* `L` is the sequence of guard calls a run actually makes, each with its result: `Seq<(AllocSite, nat, bool)>`, the `bool` being whether that reservation returned `Ok`. The *physical allocator-event trace* is the sequence of `GlobalAlloc` calls the run makes. The theorems speak only about `L`. The relation between the two is a tested property of the pinned std, not a proved one (§3, `THEOREMS-ALLOC-DELTA.md` §5). `req(L)` drops the `bool`.

**The log is sealed.** `L` lives in a `tracked` value of type `alloc_guard::AllocLog`, whose single ghost field is private to `alloc_guard`. Rust privacy, which Verus enforces in ghost and proof code as well, means code outside the module can neither construct a log with entries nor edit one. The only operations are `new_log()` (empty) and the guard's reservation functions, each of which appends exactly one entry whose `bool` equals `r is Ok` for that call's result. Each `try_*` function creates its log once, at entry, and returns it from a `*_logged` variant; the public `try_*` is that variant with the log discarded. `Tracked::assume_new`, `assume` and `admit` are excluded by the existing no-assumption rule, and the audit reads ghost code for them (§6). Because the source audit strips ghost code for its allocation checks, nothing about `L` rests on the audit.

**Request model.** `Req(x)` is a spec function written in model terms (frozen `MH`/`MP`/`walk`/`MR`/`H`/`MD`), not by reading executable code: the ordered `(site, elements)` requests that `f` issues on `x` when every reservation succeeds. §4 gives it per core.

**Graceful-failure theorem shape.** For all `x`, with `r` the result and `L` the final log:

1. `r = Ok(_)` or `r = Err(Format(_))` implies `Eq_f(x, r)`, `req(L) = Req(x)`, and every entry of `L` is `Ok`.
2. `r = Err(OutOfMemory(o))` implies `L` is non-empty, `req(L)` is a prefix of `Req(x)`, every entry but the last is `Ok`, the last entry is **not** `Ok`, and its `(site, elements)` equals `(o.site(), o.elements())`.
3. Every `(s, k)` in `Req(x)` has `k` within the site's maximum (§4), proved from checked values before the guard call.

Clauses 1–2 tie **every** `OutOfMemory` return to a reservation that actually failed and that was the last one issued. An implementation that returns `OutOfMemory` after `try_reserve_exact` returned `Ok` fails clause 2, since the last entry would be `Ok`; one that swallows a failure and returns a format error or `Ok` fails clause 1. Neither check depends on the audit.

**Progress.** If every entry of `L` is `Ok`, then `r` is not `OutOfMemory`, `Eq_f(x, r)` and `req(L) = Req(x)`. This is the contrapositive of clause 2 combined with clause 1. Each core states it as a named theorem, so that a mutation that weakens either clause still leaves a failing target.

**Where the `bool` comes from.** The guard sets an entry's `bool` to `r is Ok`, and the guard body returns `Ok` exactly from the `Ok` arm of `try_reserve_exact` (or for a zero-element request, §3). That body is verified against vstd, but vstd's `try_reserve_exact` spec does not constrain its result, so the binding "`Err` from the guard iff `Err` from `try_reserve_exact`" holds by the guard's hash-pinned text (§3), not by an `ensures`. The fault-injection test checks it at runtime (mutation A2).

**Pre-allocation predicate.** `PreAlloc(x)` holds iff `Req(x) = []`, and `Reach(x)` is its negation. Each core proves the named corollary `PreAlloc(x) ⇒ r is not OutOfMemory ∧ Eq_f(x, r) ∧ L = []`, so header errors are never `OutOfMemory`. `Reach` is defined by the code's actual reachability of the first guard call, restated in model terms, not by a looser model condition:

- **DIB, `decode` family.** `Reach(b, mode)` iff `mh(b, mode)` is `Ok`.
- **DIB, policy.** `Reach(b, bm, p)` iff either the complete path reaches `alloc_output` (`mh_policy(b,bm,p) = Ok(m)` and `¬gdk_rgb_reject(b,m,p)`, `dib.rs:8590–8600`), or the Blank second pass reaches `alloc_output_n` (`dib.rs:8268`). The second pass runs only after the first header pass has failed with `TruncatedMasks` or `TruncatedPalette` under `incomplete_metadata = Blank` (`:8424–8427`); it then revalidates and returns the original error at four points: `|b| > MAX_INPUT` (`:8243`), `bmp_prefix_policy` error (`:8249`), `dib_fields` error (`:8255`), and missing masks with a header size other than 40 or 64 (`:8258`). The required lemma is that the conjunction of passing all four equals `mh_partial(b,bm,p) is Some` (`model.rs:8829`). In that case the reservation is followed only by `fill_blank` and `Ok`, so `OutOfMemory` is the sole alternative to success.
- **TGA.** `Reach(b)` iff `MH(b)` is `Ok`.
- **ICNS.** `try_parse`: R2–R4 pass. Decoders: index and type checks pass and `rle_decode` is entered (colour) or the mask length check passes (mask).
- **ICO.** `try_parse`: `H(b)` is `Ok`. `try_decode_entry`: `MD` reaches the DIB call, and then DIB's own `Reach` on the selected slice in `IcoEntryDibSlack` mode.

**Displaced errors.** An input that the model rejects *after* the first reservation may yield `OutOfMemory` instead of its format error. DIB `TruncatedPixels`, TGA `TruncatedImageId` (TGA reserves before the ID check, which frozen R6 permits) and every ICNS in-loop parse error are examples. The ICNS case is new with this delta: the entry list is reserved once, after R2–R4 and before the loop, so a 16-byte file whose first entry declares length 7 can return `OutOfMemory` before `InvalidEntryLength`. The owner accepts this (decision D5), including when enough memory would remain to report the format error: under memory pressure, reporting `OutOfMemory` in place of a malformed-entry error is acceptable.

**Partial work, no partial output.** `OutOfMemory` may be returned after earlier reservations succeeded and after work was done (ICNS `try_combine` decodes the colour entry before it reserves the mask; ICNS `try_rle` reserves three channels in turn). On every path:

- An `Err(OutOfMemory)` carries no pixels, no entry list and no `recovered` flag. `try_decode_with_policy` returns `recovered = false` with `OutOfMemory`.
- No API has a `&mut` output parameter, so there is no partially written caller buffer.
- Every vector created before the failure is dropped on return by ordinary ownership. The production call graph contains no `mem::forget`, `Box::leak`, `ManuallyDrop`, `Rc` or `Arc` (audit, §6).

**Work bound.** DIB T12 (`steps <= 16*(|b|+N)`, `N` the validated `w*h`, or 0 on a failed header) and ICO T9 hold **for every outcome**, including `OutOfMemory`, with the same counter and `N`; this is the required bound. The guard takes no step counter: the caller charges the `4N` initialisation after the guard returns `Ok`, so the `OutOfMemory` branch carries no initialisation charge. `steps_OOM <= 16*|b|` stays an optional strengthening (decided, O5); it would need the pre-allocation work, including the Blank path's failed first pass and revalidation, charged within `16*|b|`. TGA and ICNS have no step counter and keep only termination.

**Termination.** No core retries a reservation. Each guard call sits at a fixed program point outside any loop. The ICNS entry list is reserved once before its loop, and the three ICNS channels are three straight-line calls.

## 3. Reservation primitive, vstd gap and A-STD

vstd `0.0.0-2026-11-01-0251` (`std_specs/vec.rs:133–145`) specifies `try_reserve` and `try_reserve_exact` with only `final(vec)@ == old(vec)@`. It relates `Ok`/`Err` to nothing and models no capacity. `push` (`:149`), `resize` (`:329`), `extend_from_slice` (`:171`) and `from_elem`/`vec!` (`:414`) specify only the resulting view. So Verus treats every reservation as possibly failing, which is the property this delta wants, and Verus **cannot** prove that a later `push` does not allocate. vstd has no `String::try_reserve*` or `String::with_capacity`, so a `String` cannot be built fallibly in verified code without a new `assume_specification`, which ICNS's boundary forbids.

**The guard module.** `alloc_guard/mod.rs` is identical text in every core, and its SHA-256 is recorded and checked (decision D6). The hash authenticates the text only: it shows the module is the reviewed text, and says nothing about its correctness beyond what verification and review establish. A per-core child module `alloc_guard/sites.rs` (separately hashed and reviewed) declares `AllocSite`, the element types, the per-target layout facts (below) and, in ICO only, `import_dib` (§3.1). Rust privacy lets the child module see the parent's private fields, and nothing else can. The guard is the only place where production code may construct, grow or fill a `Vec`:

```rust
// alloc_guard/mod.rs. Fields private. #[inline(never)] on every exec fn.
pub tracked struct AllocLog { ghost ev: Seq<(AllocSite, nat, bool)> }
pub closed spec fn AllocLog::view(&self) -> Seq<(AllocSite, nat, bool)>;
pub proof fn new_log() -> (tracked l: AllocLog) ensures l.view() == Seq::empty();

pub struct Reserved<T> { v: Vec<T>, room: Ghost<nat> }      // invariant: v@.len() <= room <= v.capacity() (A-STD-1)

pub fn try_zeroed(site: AllocSite, n: usize, log: Tracked<&mut AllocLog>) -> (r: Result<Vec<u8>, OutOfMemory>)
    requires 1 <= n <= MAX_BYTES,                           // MAX_BYTES = 67,108,864
    ensures final(log).view() == old(log).view().push((site, n as nat, r is Ok)),
            r is Ok  ==> r->Ok_0@ == Seq::new(n as nat, |_i| 0u8),
            r is Err ==> r->Err_0.site() == site && r->Err_0.elements() == n;
pub fn try_copy(site: AllocSite, s: &[u8], log: Tracked<&mut AllocLog>) -> (r: Result<Vec<u8>, OutOfMemory>)
    requires 1 <= s.len() <= MAX_BYTES,
    ensures /* log as above with s.len() */, r is Ok ==> r->Ok_0@ == s@, ...;
pub fn try_with_room<T>(site: AllocSite, n: usize, log: Tracked<&mut AllocLog>) -> (r: Result<Reserved<T>, OutOfMemory>)
    requires n <= MAX_ROOM,                                 // MAX_ROOM = 4,096
             1 <= size_of::<T>(), n * size_of::<T>() <= isize::MAX,
    ensures /* log as above */, n == 0 ==> r is Ok,
            r is Ok ==> r->Ok_0.view() == Seq::<T>::empty() && r->Ok_0.room() == n;
impl<T> Reserved<T> {
    pub fn push(&mut self, x: T) requires old(self).view().len() < old(self).room(), ...;
    pub fn into_vec(self) -> (v: Vec<T>) ensures v@ == self.view();
}
```

Each body is `Vec::new()`; then, if `n == 0`, `Ok` without calling `try_reserve_exact`; otherwise `try_reserve_exact(n)`, returning `Err(OutOfMemory { site, elements: n })` from its `Err` arm, and from its `Ok` arm either a `push(0)` loop to length `n` (`try_zeroed`), a `push` loop over `s` (`try_copy`) or nothing (`try_with_room`). The fill uses `push` only, so A-STD rests on documented guarantees (decision D4). `resize`/`extend_from_slice` may be adopted later only if a measurement shows the push loop matters, and then A-STD gains a clause for them.

The `size_of::<T>()` precondition is discharged at each call site from a Verus `global layout` declaration for the concrete element type (`icns::Entry`, `ico::Entry`), which rustc checks as a static assertion. It excludes zero-sized types and keeps the byte size far below `isize::MAX` (4,096 entries of at most a few dozen bytes each). If vstd's layout support does not cover these structs, the fallback is a monomorphic `try_with_room_entry` per core in `sites.rs`, with the size fixed by the same static assertion. `size_of::<Entry>()` differs between 32- and 64-bit targets (`usize` and pointer fields), so the declaration, or the fallback's assertion, states a size per target. No numeric bound that depends on it (the byte maxima in §4, the counting test's expected sizes) is frozen until the declaration is instantiated with concrete sizes for each `Entry` type and checked by rustc on each production target (pre-freeze gate F2, §8).

**A-STD**, the one trusted std contract, is stated for the concrete types the cores use: `Vec<u8>`, `Vec<icns::Entry>` and `Vec<ico::Entry>` with the `Global` allocator, both element types non-zero-sized. It is documented behaviour, tested at runtime and not proved:

- **A-STD-1** (`Vec::try_reserve_exact`, "Errors" and capacity docs). On an empty `Vec<T>` with `n >= 1` and `n * size_of::<T>() <= isize::MAX`, `Ok` implies `capacity() >= n`.
- **A-STD-2** (`Vec` "Guarantees": `push` "will never (re)allocate if the reported capacity is sufficient"). `push` does not allocate while `len < capacity`.
- **A-STD-3** (`try_reserve_exact` "Errors": capacity overflow or allocator failure). Under the size precondition above, capacity overflow cannot occur, so an `Err` means the allocator reported failure (returned null). This clause is used only to interpret `OutOfMemory` in documentation and tests; no theorem depends on it, and `OutOfMemory` means "the reservation returned `Err`".

Proof reports name the pinned toolchain (rustc 1.99.1, commit `58cf8f9c3`, 2026-10-13, the version Verus `0.0.0-2026-11-01-0251` pins, with its std) and the allocator under test (std `System`, glibc `malloc` on `x86_64-unknown-linux-gnu`, and on `i686-unknown-linux-gnu` once that target's std is installed, §6). The counting-allocator test (`THEOREMS-ALLOC-DELTA.md` §5) is a regression check of A-STD on that toolchain, not a proof of it.

### 3.1 ICO: importing DIB's log

DIB's guard keeps its own sealed log, of type `dib_verus::alloc_guard::AllocLog` with elements `(dib::AllocSite, nat, bool)`. ICO's log has elements `(ico::AllocSite, nat, bool)` with `ico::AllocSite = EntryList | Dib(dib::AllocSite)`. ICO's decode path reserves nothing of its own, so without an import its log would stay empty when DIB returns `OutOfMemory`, and A-ICO-2 clause 2 could not be established. ICO's `alloc_guard/sites.rs` therefore defines one verified exec function, which is part of the hashed and reviewed guard text:

```rust
// ico/ico_verus/src/alloc_guard/sites.rs (verified; hashed with sites.rs)
pub open spec fn map_dib(s: Seq<(dib::AllocSite, nat, bool)>) -> Seq<(AllocSite, nat, bool)> {
    Seq::new(s.len(), |j| (AllocSite::Dib(s[j].0), s[j].1, s[j].2))
}

pub fn import_dib(
    r: Result<dib::Decoded, dib::TryError>,
    dlog: Tracked<dib::alloc_guard::AllocLog>,     // consumed: a tracked value is linear
    log: Tracked<&mut AllocLog>,
) -> (out: Result<Result<dib::Decoded, dib::Error>, OutOfMemory>)
    requires
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
        dib::dib::log_tied(r, dlog@.view()),
    ensures
        final(log).view() == map_dib(dlog@.view()),
        match r {
            Ok(d)                         => out == Ok(Ok::<_, dib::Error>(d)),
            Err(dib::TryError::Format(e)) => out == Ok(Err::<dib::Decoded, _>(e)),
            Err(dib::TryError::OutOfMemory(o)) => out is Err
                && out->Err_0.site() == AllocSite::Dib(o.site())
                && out->Err_0.elements() == o.elements()
                && final(log).view().last() == (AllocSite::Dib(o.site()), o.elements() as nat, false),
        },
```

`dib::dib::log_tied(r, L)` is the log part of DIB's `G` clauses 1–2, exported by DIB with A-DIB-8: `r` is `OutOfMemory(o)` ⇒ `L` is non-empty, all entries but the last are `Ok`, and the last is `(o.site(), o.elements(), false)`; otherwise every entry is `Ok`. Given that `requires`, the `OutOfMemory` arm's last conjunct follows from `map_dib` and needs no further proof at the call site. The body's ghost part sets `log.ev = map_dib(dlog.view())`, which it may do because `sites.rs` is a child of ICO's `alloc_guard` and the field is private to that module; reading `dlog.view()` needs only DIB's `pub closed spec fn view` and DIB's `ensures`, not the field. The exec part matches on `r` and, in the `OutOfMemory` arm, builds `OutOfMemory { site: Dib(o.site()), elements: o.elements() }`.

Usage is fixed: `try_decode_entry_impl` calls `dib_verus::try_decode_with_steps_logged` once and passes its result and log straight to `import_dib`, which is the only route from a DIB result to an ICO result. The `requires old(log).view() == []` excludes a second import into a non-empty log, and the linear `Tracked` argument excludes importing one DIB log twice. The ghost-reading audit checks one call each (§6).

**Memory and timing.** Today `vec![0u8; n]` uses `alloc_zeroed`, which on Linux gets lazily zeroed pages. `try_zeroed` uses `alloc` and then pushes `n` zero bytes, so the 64 MiB maximum now costs one byte-wise fill and commits its pages at reservation time. DIB T12 already charges `4N` for this initialisation. Stable Rust has no fallible zeroed `Vec` constructor without `unsafe`.

**When `OutOfMemory` is observed.** With the size caps, a reservation fails only when the allocator returns null (A-STD-3): under `RLIMIT_AS` (glycin's sandbox sets one, see `TABLE.md`), `vm.overcommit_memory=2`, 32-bit address-space exhaustion or a failing custom allocator. Under Linux's default heuristic overcommit, a request of at most 64 MiB almost always succeeds, and real exhaustion surfaces later as the OOM killer, which no user-space code can turn into an error. A zero-element reservation never fails and never reaches the allocator.

## 4. Allocation sites and request models per core

Sizes are maxima implied by the frozen caps. `N = w*h`; ICNS `n = d*d` with `d ∈ {16, 32, 48, 128}`.

| Core | Site (`AllocSite`) | Current code | Guard call | Max | Reached iff (model terms) |
|---|---|---|---|---|---|
| DIB | `Output` | `dib.rs:8210` `vec![0u8; 4*n]` in `alloc_output_n`, called from `alloc_output` (`:3573`; reached from `decode_tracked:7767` and `decode_policy_tracked:8600`) and from `decode_partial_metadata:8268` | `try_zeroed(Output, 4N)` | 67,108,864 B | `Reach` in §2 |
| TGA | `Output` | `tga.rs:2561` `vec![0u8; 4 * n]` | `try_zeroed(Output, 4N)` | 67,108,864 B | `MH(b)` is `Ok` (before the ID/map/pixel checks) |
| ICNS | `EntryList` | `icns.rs:449` `Vec::new()` + `:579` `push` (geometric regrowth) | one `try_with_room(EntryList, min(4096, (|b|-8)/8))` after R2–R4, before the loop. Every entry occupies at least 8 bytes, which bounds the count. Zero for `8 <= |b| <= 15`, so no allocator call | 4,096 entries | R2–R4 pass |
| ICNS | none after the change | `icns.rs:342–364` `hex4`: `String::new` + 8 `push`, once per entry (up to 4,096 small requests) | **removed**: `type_hex: [u8; 8]` (decision D2, below) | 0 | n/a |
| ICNS | `Channel(k)`, k = 0, 1, 2 | `icns.rs:692` `[vec![0u8;n]; 3]` in `rle_decode` | `try_zeroed(Channel(k), n)`, three straight-line calls in order | 16,384 B each | colour entry selected, type and `it32` checks passed (`rle_decode` entered) |
| ICNS | `RgbOut` | `icns.rs:1023` `vec![0u8; 3*n]` | `try_zeroed(RgbOut, 3n)` | 49,152 B | `MR(payload, n)` is `Ok` |
| ICNS | `MaskData` | `icns.rs:1132` `Vec::new()` + `:1142` `push` loop (regrowth) | `try_copy(MaskData, payload)` | 16,384 B | `MA` passes the `InvalidMaskLength` check (then always `Ok`) |
| ICNS | `CombineOut` | `icns.rs:1250` `vec![0u8; 4*n]` | `try_zeroed(CombineOut, 4n)` | 65,536 B | both sub-decodes `Ok` and sides equal |
| ICO | `EntryList` | `ico.rs:773` `Vec::with_capacity(header.count)` + `:817` `push` (`origin/main`) | `try_with_room(EntryList, count)` | 1,024 entries | `H(b)` is `Ok` |
| ICO | `Dib(dib::AllocSite::Output)` | through `dib_verus::decode_with_steps` at `ico.rs:1192` | DIB's `Output` request on the selected slice, via `dib_verus::try_decode_with_steps_logged`; its result and log go to `import_dib` (§3.1) | 64 MiB | `MD` reaches the DIB call; then DIB's `Reach` on the slice in `IcoEntryDibSlack` mode |

Guard call sites per core: DIB 1, TGA 1, ICNS 7 (one list, three channels, `RgbOut`, `MaskData`, `CombineOut`), ICO 1 plus the one DIB call.

`Req` for each entry point:

- **DIB** `try_decode`/`try_decode_with_steps`: `[(Output, 4N)]` iff `Reach(b, mode)`, else `[]`. `try_decode_with_policy`: the same with the policy `Reach`. At most one request, which preserves frozen T3/R17's "exactly one allocating call requesting `4N`" on success.
- **TGA** `try_decode`: `[(Output, 4N)]` iff `MH(b)` is `Ok`.
- **ICNS** `try_parse`: `[(EntryList, min(4096, (|b|-8)/8))]` iff R2–R4 pass. `try_decode_rgb`: `[]` before `rle_decode`; `[(Channel 0, n), (Channel 1, n), (Channel 2, n)]` if `MR` errs; that list plus `(RgbOut, 3n)` otherwise. `try_decode_mask`: `[(MaskData, n)]` iff the length check passes. `try_combine`: `Req_rgb(ci)`, then `Req_mask(mi)` if the colour decode is `Ok`, then `(CombineOut, 4n)` if both are `Ok` and the sides match. This follows `MJ`'s absent-index precedence: `MissingColor`/`MissingMask` give `[]`.
- **ICO** `try_parse`/`try_parse_with_steps`: `[(EntryList, count)]` iff `H(b)` is `Ok`. `try_decode_entry`/`try_decode_entry_with_steps`: `[]` until `MD` reaches the DIB call, then DIB's `Req` on `b[a..z]` in `IcoEntryDibSlack`, with `Output` mapped to `Dib(Output)`. ICO's `L` equals `map_dib` of DIB's returned log, installed by `import_dib` (§3.1). A DIB `OutOfMemory` becomes ICO `OutOfMemory`, never `Error::Dib(_)`.

Peak live heap per call is bounded by the sum of live requests. This is a statement about ownership and drop points read from the code, not a verified clause: Verus has no live-heap model. The counting test checks it. DIB and TGA hold one buffer of at most 64 MiB. ICO holds the entry list (owned by the caller) plus one DIB output. ICNS `try_combine`, with `n_c` and `n_m` the colour and mask pixel counts:

- During the colour decode: the three channels plus `RgbOut`, `6·n_c` (`decode_rgb` reserves its output at `icns.rs:1023` while the channels returned at `:1017` are live). The channels are dropped before `try_decode_rgb` returns.
- From the mask reservation until the side check: `3·n_c + n_m`. Both buffers are live when the sides are compared (`icns.rs:1240`), whether or not they match; a 16×16 colour entry with a 128×128 mask holds 768 + 16,384 = 17,152 bytes before returning `TypeMismatch`.
- After a matching side check: `8n` (`rgb 3n + mask n + out 4n`, with `n = n_c = n_m`).

The largest of these is `8n` at `d = 128`, 131,072 bytes (128 KiB); the mismatched case peaks at `3·16,384 + 16,384` = 65,536 bytes and the colour decode at 98,304. So the absolute bound of 128 KiB per `try_combine` call is unchanged.

**ICNS `Entry::type_hex` becomes `[u8; 8]`** (decision D2). The field holds the eight lowercase ASCII hex digits.

- Construction: `hex4_bytes(code: [u8; 4]) -> (a: [u8; 8]) ensures hex_str(a) == model::hex4(code)` writes eight bytes into a stack array, with no allocation. `hex_str(a) = Seq::new(8, |i| a[i] as char)` is the model projection; lowercase hex digits are ASCII, so `(c as u8) as char == c` for each digit and the projection is injective on the values the parser produces.
- The frozen model is unchanged: `MpEntry.type_hex` stays `Seq<char>` (`model.rs:335`). `icns_verus/src/model.rs` is not only the model, though: many of its spec functions and lemmas take the **executable** `Entry` (`mc`, `ma` and `mj` themselves take `E: Seq<Entry>`). Every occurrence of the executable field's `String` view `e.type_hex@` becomes `hex_str(e.type_hex)`. The complete inventory on the `dib-policy` tree:

  | `model.rs` line | Item | Change |
  |---|---|---|
  | 167 | `entry_code(e: Entry)` | `code_of(hex_str(e.type_hex))` |
  | 183, 184 | `entry_inv(b, e: Entry)` | `is_hex8(hex_str(..))`, `hex_str(..) == hex4(entry_code(e))` |
  | 1037 | `entry_matches(b, e: Entry, m: MpEntry)` | `hex_str(e.type_hex) == m.type_hex` |
  | 1152 | `entry_eq_fields(e: Entry, m: MpEntry)` (used by `parses_to`) | same |
  | 1358 | `lemma_entry_matches_at(b, off, e: Entry)` | `ensures` clause through `hex_str` |
  | 1659 | `lemma_entry_inv_from_at` | `assert` through `hex_str` |
  | 2251 | `entries_same_fields(a, b: Seq<Entry>)` | `hex_str(a[i].type_hex) == hex_str(b[i].type_hex)` |
  | 2278, 2287 | `lemma_exact_entry_equivalence` | both sides through `hex_str` |
  | 3107 | `entry_type_code(e: Entry)` | `code_of(hex_str(e.type_hex))` |
  | 3606, 3631 | `lemma_mc_ms_eq`, `lemma_ma_ms_eq` | `hex_str(e.type_hex) == m.type_hex`; the following `entry_type_code` assertion (`:3607`, `:3632`) is unchanged text |
  | 3928, 3948 | `lemma_mc_fields_eq`, `lemma_ma_fields_eq` | both sides through `hex_str` |

  In `icns.rs`: the field at `:74`; `type_code`'s contract and body at `:84–125` (next bullet); `parse`'s construction at `:537–539` (`hex4_bytes`) and its assertion at `:557`; `hex4` at `:342–364`, deleted. Occurrences that are **not** changed, because they are on `MpEntry` or on model sequences: `model.rs:335`, `:364`, `:702`, `:792`, `:949`, `:2117`, `:2219`, `:2224`, `:3405`, `:3426`, `:3607`, `:3632`, and all 39 in `model_vectors.rs` (all inside `-> Seq<MpEntry>` witness functions). The injectivity of `hex_str` on lowercase hex digits (above) is the one new lemma these retargeted proofs need; `lemma_hex8_is_ascii`'s use in `type_code` becomes unnecessary. This retargeting is part of the ICNS step's scope (`THEOREMS-ALLOC-DELTA.md` §7) and is re-verified with it. Any new occurrence found when the step starts is added to this table before editing.
- `Entry::type_code` (`icns.rs:80–125`) keeps its contract with `hex_str(self.type_hex)` in place of `self.type_hex@`, keeps `requires is_hex8(...)` (the field is public, so a caller can build an `Entry` with arbitrary bytes), and indexes the array directly in place of `as_str().as_bytes()`.
- `Entry` can now derive `Copy`, and its `Clone` no longer allocates.
- Changing a public field's type breaks the API, which is acceptable (decision D9). Version note, not a gate (O3): `icns_verus` 0.1.0 → 0.2.0, DIB 0.3.0 → 0.4.0, ICO 0.1.0 → 0.2.0. The conformance test (`tests/conformance.rs:83`) compares through `std::str::from_utf8(&a.type_hex)`. glycin-icns changes `legacy_kind(&str)` (`glycin-icns/src/lib.rs:55`) to take `&[u8; 8]` and match byte-string literals (`b"69733332"` …), and its call at `:104` and the test helper `side_of` (`tests/vectors.rs:48`, `:135`, `:154`) change the same way.

## 5. Deliberately infallible (not covered by `OutOfMemory`)

- **Ghost and spec values.** `Seq`, `Ghost<_>`, `Tracked<_>` and the `AllocLog` are erased at compile time and allocate nothing.
- **Stack.** Locals are fixed-size, and no core recursion depends on the input (frozen T1/T10 review). Stack exhaustion stays outside `Safe` and cannot be reported.
- **Panic-path allocation.** The cores contain no reachable panic (T1). In glue code the panic machinery boxes its payload and formats the message, so that allocation is infallible. A caught panic in the gdk-pixbuf module becomes `GDK_PIXBUF_ERROR_FAILED`. An out-of-memory abort is not a panic, and `catch_unwind` does not catch it.
- **Host-library allocation.** GLib's `g_malloc`/`g_slice` in gdk-pixbuf glue (`GError`, the `GdkPixbuf` GObject instance) abort by GLib's design. So do glycin's D-Bus, zbus and memfd code and the `png`/`image` crates on glycin-icns's unverified PNG path. `TABLE.md` lists each site.
- **Error-path strings in glue.** glycin's `ProcessError` messages (`format!`, `to_string`) are small and allocate only on a path that is already failing.
- **Abort shim crates.** They terminate on `OutOfMemory` by design and sit outside this delta's guarantee (§1.3).
- **Physical memory.** A successful reservation does not guarantee resident pages; see §3.
- **Deallocation.** It cannot fail.

## 6. Audit (per core, mandatory, run before every proof report)

Verus sees only what the guard signatures promise; it cannot see an allocation that bypasses the guard. The audit's job is to establish that no such allocation is reachable, and only a typed call-graph analysis can do that. Text regexes cannot decide whether `.clone()` is on a `Copy` type, cannot follow user helpers or transitive callees, and cannot see allocations inside trait impls; an rlib's LLVM IR shows only external declarations for calls into precompiled std. So the gate is the call-graph audit below. The source lint, binary scan and runtime tests are falsification gates: a hit fails the gate, and a pass proves nothing.

**1. Call-graph audit (the gate).** The gate is `tools/alloc_audit`, a `rustc_driver` program built against the pinned rustc 1.99.1's `rustc-dev` component (installed for `x86_64-unknown-linux-gnu`; `#![feature(rustc_private)]`, built with `RUSTC_BOOTSTRAP=1` because 1.99.1 is a stable release). Cargo invokes it as `RUSTC_WORKSPACE_WRAPPER` on the core crate only. A prototype must exist and pass on DIB before this delta is frozen (pre-freeze gate F1, §8; this closes O2's tool choice).

*Mechanism.* The tool implements `rustc_driver::Callbacks::after_analysis`. With the `TyCtxt`, it:

1. Takes as roots `Instance::mono(tcx, def_id)` for every `pub` non-generic function the crate exports (the `try_*` functions and the `*_logged` exports; a main crate exports nothing else). No core exports a generic function; the tool fails if it finds one.
2. Runs a worklist over `Instance`s. For each instance with MIR available (`tcx.is_mir_available` for items, `tcx.instance_mir` for shims), it instantiates the body with the instance's generic arguments (`instantiate_mir_and_normalize_erasing_regions` under `TypingEnv::fully_monomorphized()`), which is the post-monomorphisation MIR, and walks every basic block:
   - `TerminatorKind::Call`/`TailCall` whose `func` operand has type `FnDef(def_id, args)`: resolve with `Instance::try_resolve`; the result is an edge to a concrete `Instance` (an impl method, a closure body, a shim). `Ok(None)` or `Err` fails closed.
   - `TerminatorKind::Drop { place }`: an edge to `Instance::resolve_drop_in_place(tcx, ty)` for the place's monomorphic type, whose drop-glue shim MIR is walked in turn, so `Drop` impls and field drops are reached.
   - `TerminatorKind::Assert`, and `Call`s to the `panic_*` lang items: an edge to the panic entry point (`core::panicking::panic_bounds_check`, `panic_const_*`, …). These paths allocate in std's panic handler.
   - Function pointers created in the body (`CastKind::PointerCoercion(ReifyFnPointer | ClosureFnPointer)`, `fn` items as constants): an edge to the pointed-to instance, so its body is audited even though the call through it cannot be resolved.
   - `Rvalue`s and constants that mention a `dyn` vtable (unsizing to `dyn Trait`): edges to every vtable method instance.
   - `InlineAsm`, `GlobalAsm`, `MonoItem::Static` with a function-pointer initialiser: fail closed.
3. Reports the reachable set and every edge's source span.

*Unresolved edges fail closed.* An edge the tool cannot follow to a body is a **leaf**, and a leaf passes only if it is an allowlist entry. That rule covers every case below:

- Precompiled non-generic std bodies without exported MIR (non-`#[inline]` functions such as `alloc::raw_vec::finish_grow`): leaf; allowed only by an allowlist entry whose contract says what the item allocates.
- Calls through a function pointer (`FnPtr` callee) or a trait object (`InstanceKind::Virtual`): fail. No core uses either; a justified exception would need the full set of possible targets listed in the allowlist, and none is planned.
- Intrinsics (`InstanceKind::Intrinsic`): leaf; allowed per intrinsic by name (`size_of`, `copy_nonoverlapping`, …). An unlisted intrinsic fails.
- Compiler-generated panic paths: leaf at the panic entry point; allowed only with the justification "unreachable by Verus T1/`Safe'` on this function", citing the verified theorem. The audit records the edge count per function so a new panic edge shows up as a diff.
- Compiler-generated allocation paths (`exchange_malloc` for `Box::new`, `alloc::alloc::*`): leaf; allowed only from the callers in the allocation-capable list below.
- FFI (`extern` items, `ForeignItem`): fail. The cores have none.
- `-Zshare-generics`: if enabled, generic std instantiations would come from upstream crates instead of being instantiated locally. The release profile disables it; the tool checks `tcx.sess.opts.share_generics()` and fails if it is on.

*The allowlist* is a manually reviewed, target-specific, hash-recorded transitive contract: `tools/alloc_audit/allowlist/<target>/<core>.toml`. Each entry names a resolved item by def-path and generic arguments, states what the item and everything it calls can allocate (`none`, or the exact allocation events), and cites the evidence (std documentation clause, std source at the pinned commit, or the Verus theorem for panic edges). The file records the rustc commit hash (`58cf8f9c3`), the vstd version, the target triple and the **named reviewer** who signed it; its SHA-256 goes into the proof report. Any change of rustc/std commit, vstd version or target invalidates the file, and it must be re-reviewed before the audit can pass again; the tool refuses an allowlist whose recorded commit differs from the running compiler's. The entries are in three groups:

- Non-allocating leaves and walked items the core calls: slice indexing, `Vec::len`, `Vec::set`, `Vec::new`, checked integer ops, the intrinsics in use.
- Allocation-capable items, enumerated per caller:
  - `Vec::<T>::try_reserve_exact`, `Vec::<T>::push`: only from `alloc_guard` functions, for `T ∈ {u8, Entry}`. The `push` entry's contract names its growth path, which ends in `handle_alloc_error`, and cites A-STD-2 and `Reserved`'s `len < room ≤ capacity` invariant (or, in `try_zeroed`/`try_copy`, the preceding `try_reserve_exact(n)` and A-STD-1) for why that path is not taken.
  - `drop_in_place::<Vec<T>>` (deallocation): anywhere.
  - No entry for `std::alloc::handle_alloc_error`, `std::process::abort` or `core::alloc::Layout::array` outside the `push` contract above. A main crate has no aborting entry point, so any other edge to them fails the audit (mutation A23).
  - ICO only: `dib_verus::try_decode_with_steps_logged`, exactly one call site, from `try_decode_entry_impl`, followed by `import_dib`.
- Panic entry points, each with its T1 citation.

Anything reachable and not on the allowlist fails the audit, so an unlisted std item is a finding until the reviewer adds it.

*Targets.* The audit runs on every production configuration: release profile, default features, `cfg(test)` off, `cfg(verus_only)` off, per target. On `x86_64-unknown-linux-gnu` it runs as described. The pinned toolchain has no `i686-unknown-linux-gnu` std installed (no `rust-std` component, and no `rust-src` for `-Zbuild-std`), so the tool cannot compile the core for that target. Until the owner adds `rust-std-i686-unknown-linux-gnu` for 1.99.1 to the image, a proof report states the audit-backed claim ("no allocation outside the guard is reachable") for x86_64 only and marks i686 **unaudited**; the Verus theorems themselves already quantify over 32- and 64-bit `usize`. When the component is installed, i686 gets its own allowlist file, reviewed separately, because std's resolved instances and `Entry`'s layout differ between the two targets.

- **Dependency contracts.** ICO's allowlist admits the DIB entry point only together with DIB's own passing audit report for the same DIB source hash and target. vstd and std are pinned by version, and the allowlist names items, not crates.
- Enumerated sites that must be **absent** after the change (each was reachable before it): ICNS `hex4` `String::new` + 8 `String::push` (`icns.rs:342–364`); ICNS parser list `Vec::new` + `Vec::push` (`:449`, `:579`); ICNS channels `from_elem` (`:692`); ICNS RGB `from_elem` (`:1023`); ICNS mask `Vec::new` + `Vec::push` growth (`:1132–1142`); ICNS combine `from_elem` (`:1250`); DIB `from_elem` (`dib.rs:8210`); TGA `from_elem` (`tga.rs:2561`); ICO `Vec::with_capacity` + `Vec::push` (`ico.rs:773`, `:817`). ICO's DIB output is reached only through DIB.
- `mem::forget`, `Box::leak`, `ManuallyDrop`, `Rc`, `Arc` are unreachable.

**2. Ghost-reading checks.** These read ghost and proof code, which the allocation lint strips: no `assume`, `admit`, `Tracked::assume_new`, `external_body`, `#[verifier::external]`, `#[verifier::truncate]`, or new `assume_specification`; `alloc_guard::new_log` called exactly once per `*_logged` entry point; in ICO, `import_dib` called exactly once, in `try_decode_entry_impl`, on the result and log of the one `try_decode_with_steps_logged` call; `alloc_guard/mod.rs` hash equals the recorded value, and `sites.rs` hash equals its own recorded value (for ICO this hash covers `import_dib` and `map_dib`).

**3. Source lint (falsification).** The existing `audit_alloc.sh` scripts (DIB, ICO) generalise into `tools/audit_alloc.py`, which strips comments and ghost code and then flags `vec!`, `Vec::new`, `Vec::with_capacity`, `.push(`, `.resize(`, `.extend`, `.insert(`, `.append(`, `.reserve`, `.try_reserve`, `.collect`, `.to_vec(`, `.to_owned(`, `.clone(`, `String`, `format!`, `to_string`, `Box::`, `Rc::`, `Arc::`, `iter::repeat`, `repeat_n`, `.into_boxed`, `mem::forget`, `Box::leak`, `ManuallyDrop`, `handle_alloc_error` and `process::abort` outside `alloc_guard`. A hit is a finding; `Reserved::push` and `Vec::set` are whitelisted by full path. The frozen DIB expectation "exactly 1 `vec!`" becomes "0 `vec!`, 1 `try_zeroed` call site", and ICO's "exactly one `Vec::with_capacity(header.count)`" becomes "0, and 1 `try_with_room` call site".

**4. Binary scan (falsification).** Build the shipped artefact (`cdylib` or loader binary) in release mode, and list symbols that reach `__rust_alloc`, `__rust_alloc_zeroed`, `__rust_realloc`, `alloc::raw_vec::finish_grow` and `handle_alloc_error` from the core's symbols. Every hit must be in an `alloc_guard` function (`#[inline(never)]`) or glue listed in `TABLE.md`; the shipped glue links no abort shim crate. This catches drift between the analysed MIR and the shipped code; it cannot see through precompiled std and is not proof of absence. It stays a falsification gate even after the call-graph prototype exists.

**5. Runtime tests (falsification).** Counting allocator and fault injection, `THEOREMS-ALLOC-DELTA.md` §5.

What each core's audit additionally checks:

- **DIB.** One `try_zeroed` call site, in `alloc_output_n`, reached on the three existing paths. No palette, scratch or RLE allocation, and no `String`.
- **TGA.** One `try_zeroed` call site, in `try_decode` after `parse_header`. The colour map is read in place; there is no lookup table (frozen R4).
- **ICNS.** One `try_with_room` site in `try_parse`, before the loop; one `Reserved::push` per iteration; three channel sites, `RgbOut`, `MaskData`, `CombineOut`. No `String`, no `hex4`. `try_combine` does not clone `rgb.data` or `mask.data`.
- **ICO.** One `try_with_room` site in `try_parse_tracked`. `try_decode_entry_impl` allocates nothing of its own, makes exactly one call to `dib_verus::try_decode_with_steps_logged`, and passes its result and log to `import_dib`.

## 7. Decided defaults and sequencing

The owner settled the former open items:

- O1. ICO's theorem witnesses (`t5_decode_exact`, `t6_*`, `t7_dib_output_bound`, `t9_*`, `witnesses::t11_*`, `lib.rs::skeleton`) stay `pub`, with contracts retargeted to `TryError`.
- O3. Version note, not a gate (D9: the API is unstable and breaking changes are acceptable): DIB 0.3.0 → 0.4.0; ICO 0.1.0 → 0.2.0; `icns_verus` 0.1.0 → 0.2.0.
- O4. The unimplemented TGA policy API is `try_`-only, like every main-crate API. An aborting variant, if wanted, goes in `tga_abort_shim` (§1.3).
- O5. `steps_OOM <= 16·|b|` stays optional; the required bound `16·(|b|+N)` holds for every result.
- O6. ICO is implemented only after DIB's verified `try_decode_with_steps_logged` has landed on the branch that ICO's path dependency (`../../dib/dib_verus`) resolves to, or on a merged branch, with the DIB source hash recorded in ICO's audit report.
- O2 (audit tool) is decided as §6's `rustc_driver` mechanism; its prototype is pre-freeze gate F1.

The owner's decisions D1–D9 are listed in `TABLE.md`.

Order: the guard module and the tooling (audit prototype, counting and fault-injection harness) come first. The TGA policy implementation, separate ongoing work that edits `tga.rs`, must finish before the TGA allocation pilot, so the pilot does not conflict with it and covers the policy decoder's reservation from the start. Then DIB, then ICO (per O6), then ICNS. Each core's step migrates that core's consumers (§1.4) and may add its abort shim crate.

## 8. Pre-freeze gates

The delta is not frozen until each gate passes and its evidence is recorded:

- **F1. Audit prototype.** `tools/alloc_audit` (§6) runs on DIB on x86_64, resolves every edge or fails closed as specified, and produces a reviewed, hash-recorded allowlist with a named reviewer. The i686 handling (§6, "Targets") is recorded in the same report.
- **F2. Per-target `Entry` layout.** The `global layout` declaration for `icns::Entry` and `ico::Entry`, or the monomorphic `try_with_room_entry` fallback with its static assertion, is written with concrete sizes and alignments and checked by rustc on each production target. Until then, no byte bound derived from `size_of::<Entry>()` (§4 maxima, §5 test sizes in `THEOREMS-ALLOC-DELTA.md`) is frozen.
- **F3. Sealed log across modules and crates.** Verus accepts a `tracked` struct with a private ghost field read through a `pub closed spec fn view`, and accepts ICO's `import_dib` (§3.1) consuming DIB's `Tracked<AllocLog>` across the crate boundary. If either fails, the theorem shape in §2 must be revised before freezing, not patched with an `assume`.
