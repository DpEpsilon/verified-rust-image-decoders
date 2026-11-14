# Public-API survey of the four decoder cores (owner task: hide clutter)

Scope: every `pub` item reachable from each crate root, the external users of each,
and the decision applied. Branch `dib-policy`, 2026-11-14.

## 0. How the decisions were implemented

Two levers were used, in this order, because Verus's visibility rule is strict:

1. **Module visibility.** A `pub` item inside a non-`pub` module is not reachable
   from another crate. Making `model`, `model_vectors`, `policy_model` and
   `witnesses` `pub(crate)` hides every item in them at once, with no per-item
   edit and no effect on the proofs (Verus only requires that an item a `pub`
   contract *names* be `pub`; module nesting is irrelevant to that check, and a
   crate-internal reference between two of its own items is always allowed).
   `alloc_guard` stays `pub` (ICO reaches `dib_verus::alloc_guard::AllocLog`).
2. **Item demotion + `#[doc(hidden)]`.** Inside the implementation module
   (`dib.rs`, `tga.rs`, `icns.rs`, `ico.rs`) and `lib.rs`, items were demoted to
   `pub(crate)` unless a `pub` item's contract refers to them (Verus: "in
   `ensures` clause of public function, cannot refer to private function"), and
   the residual internal-but-public items were marked `#[doc(hidden)]`.

The rule Verus enforces, established by experiment against this toolchain
(verus-0.0.0-2026-11-01-0251, V-1.99.1):

- A `pub` fn/lemma whose `requires`/`ensures`/body names a spec or proof fn
  requires that fn to be `pub`. Demoting the named fn is a hard error.
- A `pub open spec fn` (in a **private** module) may be `pub(crate)` without
  error; but once the module is `pub`, `pub(crate) open spec fn` is an error
  ("function is marked `open` but not marked `pub`"). A `pub(crate) closed spec
  fn`, `pub(crate) proof fn`, `pub(crate) fn`, `pub(crate) const` and
  `pub(crate) struct/enum` are all accepted.
- A `pub(crate)` item cannot be re-exported through `pub use` (rustc E0364/E0365).
- Spec/proof items do not exist in a plain `cargo build`, so they cannot be
  re-exported at all (`pub use` of a spec fn fails to compile under `cargo test`).

Consequence for the intended result: the items a `pub` entry point's contract
names remain `pub` and are marked `#[doc(hidden)]`; the reference model, whose
items only the proofs and the sibling core's spec code name, is hidden by module
visibility (DIB's `model` must stay a `pub` module, hidden, because ICO's spec
code names `dib_verus::model::{mp, max_dimension, max_output, lemma_mp_ok_shape}`).

## 1. Per-core counts

"all pub" counts every `pub` item in every `src/*.rs` and `src/alloc_guard/*.rs`
before the change. "after" counts the implementation module only. The reference
model, `model_vectors`, `policy_model` and `witnesses` were demoted to
`pub(crate)`, so they are not reachable from another crate and contribute no
exported root. **The one exception is DIB's `model`**: it stays `#[doc(hidden)]
pub` because `ico_verus`'s spec code names `dib_verus::model::{mp,
max_dimension, max_output, lemma_mp_ok_shape}` (section 2), so those four items
(and anything else `pub` in that module) are reachable from ICO and enter ICO's
root set. "after" does not count them, because DIB's own exported surface is
unchanged by the exception.

| Core | all `pub` before | impl-module `pub` after | of which `#[doc(hidden)]` | documented API after | verifier |
|---|---|---|---|---|---|
| DIB  | 1182 | 62 | 28 | 34 | 1764 verified, 0 errors |
| TGA  | 956  | 22 | 17 | 5  | 974 verified, 0 errors |
| ICNS | 556  | 73 | 61 | 12 | 412 verified, 0 errors |
| ICO  | 536  | 21 | 11 | 10 | 346 verified, 0 errors |

## 2. Module decisions

| Core | module | before | after | external users | why |
|---|---|---|---|---|---|
| DIB | `model` | `pub` | `#[doc(hidden)] pub` | ico_verus (spec code) | ICO's allocation proof names `dib_verus::model::{mp,max_dimension,max_output,lemma_mp_ok_shape}` |
| DIB | `model_vectors` | `pub` | `pub(crate)` | none | generated vector obligations; proof-only |
| DIB | `alloc_guard` | `pub` | `pub` | ico_verus, own tests | `AllocLog`/`AllocSite`/`OutOfMemory` cross-crate; `mod.rs`/`sites.rs` hash-pinned |
| DIB | `dib` | `pub` | `pub` | glycin, gdkpixbuf, ico, tests | holds the entry points and result types |
| TGA | `model`, `model_vectors` | `pub` | `pub(crate)` | none | proof-only |
| TGA | `policy_model` | `pub` | `pub(crate)` | none | proof-only |
| TGA | `policy` | `pub` | `pub` | bench, tests, example | `Profile`, `Policy` and the `*_policy` constructors are public API |
| TGA | `tga`, `alloc_guard` | `pub` | `pub` | glycin, bench, tests | entry points and `AllocSite` |
| ICNS | `model`, `model_vectors` | `pub` | `pub(crate)` | none | proof-only |
| ICNS | `icns`, `alloc_guard` | `pub` | `pub` | glycin, leniency, bench, tests | entry points and `AllocSite` |
| ICO | `model`, `model_vectors` | `pub` | `pub(crate)` | none | proof-only (DIB's `model` is the one ICO names) |
| ICO | `witnesses` | `pub` | `pub(crate)` | none | T11 witness bridge; proof-only |
| ICO | `ico`, `alloc_guard` | `pub` | `pub` | leniency, bench, tests | entry points and `AllocSite` |

## 3. Entry points and documented API (kept plain `pub`)

| Core | item | kind | external users | decision |
|---|---|---|---|---|
| DIB | `try_decode` | fn | bench | keep `pub` |
| DIB | `try_decode_with_policy` | fn | bench, gdkpixbuf | keep `pub` |
| DIB | `try_decode_with_steps` | fn | ico_verus (`lib.rs::skeleton`) | keep `pub` |
| DIB | `try_decode_with_steps_logged` | fn | ico_verus (the allocation bridge) | keep `pub` |
| DIB | `Decoded` | struct | ico, glycin, tests | keep `pub` |
| DIB | `Error`, `TryError`, `AllocSite`, `OutOfMemory` | enum/struct | ico, glycin, gdkpixbuf, bench, glyph/leniency | keep `pub` |
| DIB | `Mode`, `BmpMode`, `BmpProfile`, `Policy` | enum/struct | ico, glycin, gdkpixbuf, bench | keep `pub` |
| DIB | `*Options` (22 enums) | enum | none direct | keep `pub` (fields of `Policy`) |
| DIB | `Error::name` | fn | glycin, gdkpixbuf (R15 spelling) | keep `pub` |
| DIB | `MAX_INPUT`, `MAX_OUTPUT` | const | gdkpixbuf (`MAX_INPUT`) | keep `pub` |
| DIB | `recommended_policy` … `profile_policy` (5 spec fns) | spec fn | ico (via policy) | keep `pub`, `#[doc(hidden)]` (contract-facing constructors; the exec entry point is `try_decode_with_policy`) |
| TGA | `try_decode`, `try_decode_with_policy` | fn | bench, glycin | keep `pub` |
| TGA | `try_decode_with_steps_logged` | fn | none | keep `pub`, `#[doc(hidden)]` (root in `required_roots`) |
| TGA | `Decoded`, `Error`, `Header` | struct/enum | bench, glycin, tests | keep `pub` |
| TGA | `Profile`, `Policy`, `*Options` | enum/struct | bench, tests, example | keep `pub` (in `policy.rs`) |
| TGA | `policy::{strict,recommended,gdk_compat,image_crate_compat,profile}_policy` | spec fn | none | keep `pub` (public constructors in `policy.rs`) |
| TGA | `Error::name`, `MAX_INPUT` | fn/const | glycin, tests | keep `pub` |
| ICNS | `try_parse`, `try_decode_rgb`, `try_decode_mask`, `try_combine` | fn | glycin, leniency, bench, tests | keep `pub` |
| ICNS | `*_logged` (5) | fn | none | keep `pub`, `#[doc(hidden)]` (allocation roots) |
| ICNS | `Entry`, `Decoded`, `Error`, `TryError` | struct/enum | glycin, leniency, bench, tests | keep `pub` |
| ICNS | `MAX_FILE_LEN` | const | glycin | keep `pub` |
| ICNS | `MAX_ENTRIES`, `MAX_LEGACY_SIDE` | const | none | keep `pub`, `#[doc(hidden)]` |
| ICNS | `Entry::type_code` | fn | glycin (`type_hex` field), bench | keep `pub` |
| ICO | `try_parse`, `try_decode_entry` | fn | leniency, bench, tests | keep `pub` |
| ICO | `try_parse_with_steps`, `try_decode_entry_with_steps` | fn | none | keep `pub`, `#[doc(hidden)]` (roots) |
| ICO | `Entry`, `EntryKind`, `EntryDefect`, `EntryStatus`, `Error`, `TryError`, `Header` | struct/enum | tests, leniency, bench | keep `pub` |
| ICO | `MAX_INPUT` | const | tests | keep `pub` |
| ICO | `Entry`/`EntryStatus`, `Entry::name`, `Error::name` | fn | tests | keep `pub` |

## 4. Every item left `pub` + `#[doc(hidden)]`, and why

These were left `pub` + `#[doc(hidden)]` at the E11 change. Most cannot be
demoted: Verus forbids demoting an item a `pub` contract names ("in `ensures`
clause of public function, cannot refer to private function"), and a
`required_roots` entry must stay an exported function. E12 then demoted one item
that had no such tie (`try_rle_logged`), and corrected the E11 list, which had
wrongly included DIB's `at_site_log` (E11 itself demoted that one to
`pub(crate)`); see E12 and section 6.

| Core | items | why hidden rather than demoted |
|---|---|---|
| DIB (27, was 28) | `name_spec`; `bmp_mode_to_mode`; `recommended_policy`,`strict_policy`,`gdk_compat_policy`,`image_crate_compat_policy`,`profile_policy`; `fmt_part`,`log_tied`,`req_dib`,`g_clause1..3`,`g`,`prog`,`prog_concl`,`pre_alloc`,`g_logged`,`prog_logged`,`lemma_req_dib_clause3`,`lemma_req_dib_len`,`decode_mp_eq`,`t12_checked_n`,`decode_policy_mp_eq`,`decode_policy_alpha_eq`,`policy_checked_n`,`policy_reach`,`req_dibp` | named by `try_*` contracts, or ICO's bridge, or the policy roots. **E12 correction: the E11 list wrongly included `at_site_log`; E11 demoted it to `pub(crate)`, so it is not hidden-public** |
| TGA (17) | `try_decode_with_steps_logged`; `name_spec`; `dec_vals`,`decode_eq`; `req_tga`,`req_tga_policy`; `g_clause1..3`,`g`,`prog`,`prog_concl`,`pre_alloc`; `policy_result`,`model_checked_n`,`policy_checked_n`,`decoded_bounds` | named by `try_*` contracts; `try_decode_with_steps_logged` is a `required_root` |
| ICNS (60, was 61) | the four `*_logged` fns (`try_parse_logged`, `try_decode_rgb_logged`, `try_decode_mask_logged`, `try_combine_logged`); `name_spec`; `MAX_ENTRIES`,`MAX_LEGACY_SIDE`; `entry_count`,`reach_p`,`req_p`,`pre_alloc_p`,`site_max`,`req_within_site_max`; the `g_*_clause*`,`g_*`,`prog_*`,`req_*`,`at_site_log_*`,`pre_alloc_*`,`oom_in_req`,`log_extends` groups; six `lemma_*` | named by the `try_*` contracts. **E12: `try_rle_logged` was demoted to `pub(crate)`** (no `pub` contract names it, no `required_root`, no external caller), so the five `*_logged` fns became four |
| ICO (11) | `name_spec`; `MAX_ENTRIES`,`DIRECTORY_HEADER_LEN`,`ENTRY_RECORD_LEN`,`MAX_DIRECTORY_LEN`,`PNG_SIGNATURE`,`TYPE_ICO`,`TYPE_CUR`; `try_parse_with_steps`,`try_decode_entry_with_steps` | named by `try_*` contracts, or `required_roots` |

Not every item `#[doc(hidden)] pub` in the implementation module is required by
a `pub` contract or an audit root. The E11 survey claimed they all were; that is
false. `try_rle_logged` (ICNS) was hidden-public with no `pub` contract naming
it, no `required_roots` entry and no external caller, and E12 demoted it. The
same reasoning applies to the four `_logged` decode functions, which E12 leaves
`pub` only because section 6 lists them as a documented caller-controlled
surface pending an owner decision, not because a contract requires it.

## 5. Audit-allowlist impact (follow-through, section 4 of the task)

A new **root set** is computed from the exported functions; hiding modules and
demoting items shrinks it. Consequences to regenerate:

- **`required_roots`** in `alloc-delta/audit/allowlist/decisions/<core>.toml`: the
  listed roots are the `try_*` / `*_logged` entry points. Every one remains an
  exported root (each is still `pub` and reachable from the crate root — DIB and
  TGA and ICNS export them from the `pub` implementation module; ICO's
  `try_parse_with_steps`/`try_decode_entry_with_steps` are `#[doc(hidden)] pub`,
  which is still an exported root). **No `required_roots` change is needed**, but
  the lists must be re-derived against the tool's root computation by running
  `tools/draft_all.sh` and `./run_audit.sh <core>`.
- **The `pub` surface of `model`/`model_vectors`/`policy_model`/`witnesses` is no
  longer in the exported root set**, so the tool's walk covers fewer roots. This
  is the intended reduction; it may surface `mono-item-unwalked` for a witness
  fn that is reachable only from a demoted module (a finding to report, not to
  paper over).
- **`source_hash`** changes for all four cores (the source files changed), so
  the four draft allowlists must be regenerated.

See `alloc-delta/ERRATA.md` E11 for the recorded values,
`alloc-delta/ERRATA.md` E12 for the `try_rle_logged` demotion and the correction
to the E11 `at_site_log` entry and
the current hashes, and `alloc-delta/audit/SIGNOFF.md` for the updated counts.

## 6. Known caller-controlled surfaces that remain public

These stay public; each is a place where the file does not, by itself, keep the
caller inside the size or shape its Verus contract assumes. They are recorded
here as open design items for the owner. None is changed by E12.

**Guard functions whose size cap is a Verus-only precondition.** `alloc_guard::try_zeroed`
exists in all four cores (`alloc_guard/mod.rs`), and `alloc_guard::sites::try_copy`
in ICNS. Each carries `requires 1 <= n <= MAX_BYTES` (`MAX_BYTES`; for `try_copy`
`1 <= s.len() <= MAX_BYTES`), but the parameter is chosen by the caller, and the
precondition is erased in the compiled artifact. Ordinary Rust from another
crate can call `try_zeroed` with a size of its choosing and make the core
allocate that much. (Before E12 the ICNS `try_rle_logged` was a second route to
the same three 64 MiB reservations; demoting it closed that route but not the
guard itself.) `alloc_guard/mod.rs` is shared and hash-pinned, so it is not a
place to add an executable check without an owner decision; `sites.rs` is
per-core and equally pinned. Both are stated surfaces, not defects the audit can
see: the audit's `guard-calls` entries count the call sites, and every one is a
crate-internal call whose meaning is a Verus proof, not an executable cap.

**ICNS decoders that require `parsed_inv` only in Verus.** `try_decode_rgb`,
`try_decode_mask`, `try_combine` and their `_logged` variants each carry
`requires crate::model::parsed_inv(bytes@, entries@)`, and the `_logged` ones are
`#[doc(hidden)] pub`. `Entry`'s fields are public, so ordinary Rust code can
construct a `Vec<Entry>` that does not satisfy `parsed_inv` and pass it in. The
contract is erased, and the decoder then indexes and slices on the forged
`entries` (and the byte ranges those entries name) with no executable check:
`decode_rgb_tracked` does `&entries[color_index as usize]` and
`payload_range(bytes, entry)`, which can panic or read out of bounds where
`parsed_inv` would have prevented it. The `_logged` variants make the same
observation reachable from another crate; the non-logged ones do too, and are
the documented API.

Candidate fixes for the owner, not applied here:

- an opaque `Parsed` handle returned by `try_parse` (a newtype over
  `Vec<Entry>` with private contents) that the decoders take instead of
  `&[Entry]`, so a forged entry list cannot be constructed outside the crate;
- or executable validation at each decoder entry (re-check the entry ranges
  against `bytes` before use), which costs a pass over `entries` per call;
- or, for the guards, an executable `if n == 0 || n > MAX_BYTES { return Err(...) }`
  before the reservation, which the `alloc_guard` proofs would need to be
  re-derived against.
