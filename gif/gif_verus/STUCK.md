# STUCK notes, M3 (header and block parser)

Milestone M3 in `gif/gif_verus`. Each entry names the lemma that resisted, how
many runs it cost, and how it was split or resolved. Runs are the numbered
`runs/NNN.log` and the `RUNLOG.tsv` lines.

## Layout constraint found first (runs 5-11): where code may live

Verus refuses to use a type in a `spec fn` unless the type is declared *inside*
a `verus!` block. Wrapping whole modules (all of `gif.rs`, `parse.rs`, `lzw.rs`,
`compose.rs`) in one `verus!` block is wrong for this crate: it pulls every M1
exec body into the verifier, which then demands the full M3-M5 proof work at
once — 31 errors, most of them `possible arithmetic underflow/overflow` in code
whose proofs are later milestones.

The working rule (the one TGA and ICNS use): put *inside* `verus!` only the
items the model or a contract must name, and keep the plain exec bodies outside
it until their milestone proves them.

- `gif.rs`: the `Error` enum and its `name_spec`/`name`, the `Version` enum,
  `req_new`/`pre_alloc_new`/`g_new_clause2`/`g_new_clause3` and `reserve` are
  inside two `verus!` blocks; `Info`, `Frames`, `still_with` and the walk stay
  plain Rust.
- `parse.rs`: `ColorTable`, `Header`, `hdr_matches`/`hdr_inv`/`hdr_ok`,
  `need`/`get`/`le16`/`table_entries`/`parse_header` are inside; the block-walk
  functions below them are plain Rust until M3 proves them.
- `lzw.rs`: only `TABLE_SIZE`, `LZW_TABLE_BYTES`, `SUFFIX_OFF`, `LENGTH_OFF`
  are inside; the decode loop is plain until the M2 core lands.
- `limits.rs` and `policy.rs`: fully inside, because `model.rs` names the caps
  and the record.

## `reserve` and `req_new` (runs 34-46, 13 runs): the one that stalled

`reserve` is G18's reservation prefix (`Canvas` then `LzwTable`) with A-GIF-3's
clause 3 and A-GIF-2's log shape as its contract. It stalled for 13 runs on
four separate proof obligations, none of them deep — all four are arithmetic
about `usize` and `nat` that the SMT solver will not finish unaided.

Split, and each piece fixed by naming it rather than by asking more of the
solver:

1. **`4·W·H` overflow and its equality with `4·W·H` in `int`.** The direct
   assertion in `usize` needs `bit_vector`, which cannot see datatype fields
   (`hdr.width`) — the error message is "Unsupported expression in bit_vector
   proofs". Resolution: `model::lemma_4wh_usize(w, h)` takes `w` and `h` as
   plain `u32` and proves the equality, the `usize::MAX` bound (from
   `4·W·H ≤ MAX_OUTPUT = 67,108,864`, which fits 32 bits) and `4·W·H ≥ 4`.
   The call site binds `w`/`h` from `hdr` first.
2. **`hdr_inv` alone does not say `mh` succeeded.** `reserve` needs
   `4·W·H ≤ MAX_OUTPUT` with `4·W·H` the model's product, and `hdr_inv` is only
   the executable bounds. Resolution: `parse::hdr_ok(b, h)` (an existential
   over the model record that `hdr_matches` it) is a second precondition, and
   `reserve` uses `choose` plus `lemma_mh_ok_bounds` to get the area cap. This
   is the form `parse_header` establishes, so the caller discharges it.
3. **`req_of_log` after one `try_zeroed`.** The assertion
   `req_of_log(log.view()).len() == 1` did not follow by itself; the solver
   left the `Seq::new`/`map_values` relation of `req_of_log` folded.
   Resolution: `reveal(crate::alloc_guard::req_of_log)` plus a named local
   `let l = log.view()` and the two concrete `l[0]`/`l[1]` facts; then
   `is_prefix_of` is closed by `seq![req_new(b@)[0]] == req_new(b@).take(1)`.
4. **The incoming log must be stated empty.** With the log's length unstated,
   "exactly one entry appended" is not provable. Resolution: a third
   precondition, `log.view() == Seq::<(AllocSite, nat, bool)>::empty()`, which
   the constructor's `new_log()` satisfies. It cost 6 runs to find.

Result: run 46, 93 verified, 0 errors.

General lesson for the block walk below: state the incoming log, name every
intermediate (`let l = ...`, `let m = choose ...`) and call a small `int`-typed
lemma rather than asserting in `usize`. Each of these four cost runs because I
first tried the direct assertion.

## The structural sub-block scan (runs 47-86, 40 runs): the second stall

`read_sub_blocks` (G5/G9) stalled for 40 runs. It carried three obligations at
once and each masked the others: the structural scan, the payload total, and
G6.3 rule A's `loop_count` update through a `&mut Option<u16>`.

Three separate splits, in the order they were found:

1. **Rule A moved out of the scan.** `read_sub_blocks` took
   `Option<(&mut Option<u16>, bool)>` and wrote the value through the pointer,
   which no contract can state cleanly (the write is invisible to an `ensures`).
   Resolution: `read_sub_blocks` lost the `loop_target` parameter and returns
   `(end, total)` only; a second function `loop_count_of(b, q, seed, first_only)`
   rescans the same block and returns the value, with the model's `loop_fold` as
   its contract. Two passes over a sub-block sequence cost nothing (`G18`'s work
   bound charges input bytes, not passes), and each function now has one job.
2. **The model tuple dropped its running total.** `sub_scan` first returned
   `(payloads, end, total)`, a third component that had to agree with a separate
   `concat_len` in the proof. Resolution: `sub_scan` returns `(payloads, end)`
   and `concat_len(payloads)` is the total; the invariant then ties the `u64`
   accumulator to the payload list alone.
3. **Cursor accounting instead of an arithmetic bound.** `p + 1 <= usize::MAX`
   and `p + 1 + n <= usize::MAX` would not discharge from `p < |b|`:
   `bit_vector` refuses `usize::MAX` ("not a finite-width integer") and refuses
   `b@.len()` ("unsupported expression"). Resolution: the invariant
   `sub_consumed(q, p, acc) := p - q == concat_len(acc) + |acc|` (one size byte
   per payload plus its bytes) gives `|acc| + 1 <= p - q + 1 <= |b|` by linear
   arithmetic, which needs no `bit_vector` at all. The remaining `usize::MAX`
   facts are discharged the way DIB (`dib.rs:853`) and TGA (`tga.rs:1352`) do:
   `assert(b@.len() <= usize::MAX)` plus `by (nonlinear_arith)`.

Four smaller traps, each one run: `bit_vector` cannot see a datatype field
(`hdr.width`) or a `Seq` length, so the facts must be stated over plain
`u8`/`usize` locals and datatype fields bound to locals first;
`vstd::seq::lemma_seq_subrange_len` is the (public, under `vstd::seq`) lemma for
`|subrange| == k - j`; `loop_fold` had to be defined with `loop_hit(h, off, fo)`
so the executable's `(!first_only || index == 0)` condition matches the model's
hypothesis verbatim; and `hit` (`n == 3 && first == 1`) had to be tested once
into a `let hit` so the model's `if` and the executable's `if` are the same
boolean.

Result: run 86, 129 verified, 0 errors: `read_sub_blocks` equals `sub_scan`
(payloads, cursor, total) and `loop_count_of` equals `loop_fold`.

## `gce_ext` (runs 88-130, 43 runs): the third stall

`gce_ext` (G6.1) has eight exits and each had to cite its own `model::gce_step`
branch. It stalled for 43 runs, almost all on one recurring Verus friction, in
three parts:

1. **`Option::is_some` has no spec in vstd.** `if st.gce.is_some()` does not give
   the ghost fact `st.gce is Some` inside the `if`, so `lemma_gce_step_dup`'s
   precondition `st.gce is Some` was unprovable. Resolution (TGA's idiom,
   `tga.rs:6859`, `:7481`): bind the guard to a `bool` local with a **`match`**
   (`let has_gce = match st.gce { Some(_) => true, None => false };`), which
   Verus does model, then `assert(has_gce == (st.gce is Some))` in a `proof`
   block right after the `let`, and `if has_gce && dup_reject` on the locals.
   The same for `pol.gce_duplicate == GceDuplicate::Reject` (a policy field):
   `let dup_reject = match pol.gce_duplicate { Reject => true, _ => false };`.
   A plain `let b = expr;` of an exec `bool` is **opaque** to the solver — only
   the `match` form carries the ghost fact. This cost ~15 runs to find.
2. **Model/exec type unification.** `model::GceM` and `parse::Gce` were separate
   records, so every field needed a conversion lemma. Resolution: `model.rs`
   declares `pub use crate::parse::Gce as GceM;`, so the model and the executable
   compare the record with `==`. Fields became `u8`/`u16`, and the model's
   `gce_record(b, q)` builds the same record from the input bytes (with the
   model's `le16`, the value the exec `le16` returns).
3. **One lemma per model branch.** Rather than a single `match`-shaped
   postcondition, `gce_ext`'s contract is the predicate `gce_ext_post` and each
   exit reveals the branch lemma it needs (`lemma_gce_step_dup`,
   `_trunc`, `_size`, `_body_trunc`, `_term`, `_disposal`, `_ok_simple`), each
   stated over the exec `WalkState` and unfolding `walk_m_unused` inside.

Result: run 130, 144 verified, 0 errors.

## Coordinator's LZW-interface note (Sol's review), 2026-11-15

Sol confirmed T-LZW's executable (`gif/lzw_proof/src/decode.rs`) takes contiguous
de-sub-blocked `d: &[u8]`, a separate `out: &mut [u8]` index buffer, and typed
`&mut [u16]`/`&mut [u8]`/`&mut [u16]` table slices. Three constraints follow for
this crate's side of the interface, none of which blocks the M3 parser but which
the integration must respect:

- `SPEC.md` G18 (lines 161-163) forbids input-dependent extra allocation, so the
  integrated path cannot materialise `D` as a contiguous buffer nor allocate an
  index buffer. `read_sub_blocks`/`loop_count_of` here return the payload list as
  a ghost `Seq<Seq<u8>>` only (the exec `read_sub_blocks` returns `(end, total)`
  and copies nothing), and `parse::ImageData` keeps only `first`, `data_len` and
  `end` — no copy of `D`. The LZW core's contiguous-`d` interface therefore
  cannot be fed from this parser by copying; either the parser presents the
  sub-block sequence as a transparent reader the core consumes, or the core
  takes the reader (its own `bits.rs::read_code` reads `d[bitpos/8]`, which is
  exactly what a zero-copy multi-sub-block view needs). I have not treated
  T-LZW's interface as final, per the note.
- The index buffer: `parse` emits indices straight to the canvas through
  `compose::Drawer` (a `Sink`), never into an array; T-LZW's `out: &mut [u8]`
  index buffer is the one interface element that conflicts with G18's "no index
  buffer". A `Sink`-shaped core (the shape `lzw::decode` already has in
  `gif_verus/src/lzw.rs`) is the compatible form; T-LZW's `decode.rs` would need
  that adaptation at integration.
- L-TABLE-VIEW and G12.2 precedence: `model.rs` here does not yet state
  L-TABLE-VIEW (it belongs with the LZW core's table accessors, `THEOREMS.md`
  R11) and states no colour error; `parse::Drawer` in `compose.rs` already
  implements G12.2's transparent-before-bounds test and the first-violation
  reporting, but its contract is M4b work. Both remain integration obligations
  for me or M5.

Nothing in the parser's current interface blocks the LZW core: the parser hands
the core a sub-block cursor plus `|D|` and a `Sink`, which is the `Sink` shape
`lzw::decode` already has.

## Extension dispatch (runs 131-159, 29 runs)

`plain_text_ext`, `app_ext` and `parse_extension_verified` (G6/G7) are written
and verified against the model's `plain_text_step`, `app_step` and `ext_step`.
`skip_ext` (G6.4 Comment and unknown labels) is verified by construction.

Two obstacles, both about control flow rather than logic:

1. **`Policy` field comparisons in a proof block.** As with `gce_ext`, each
   exec guard over `pol.ext_sizes`, `pol.gce_consumers` and `pol.loop_scan` is
   bound to a `bool` local through a `match` (`let exact = match
   pol.ext_sizes { ExtSizes::Exact => true, _ => false };`) plus
   `assert(exact == (pol.ext_sizes == ExtSizes::Exact))`. A plain `let b = <cmp>`
   carries no ghost fact. The `plain_text_ext` and `app_ext` exits each reveal a
   named branch lemma (`lemma_pt_step_*`, `lemma_app_step_*`) instead of trying
   the whole `match`.
2. **`usize::MAX` cannot appear in a `bit_vector` proof.** Verus rejects
   `usize::MAX` ("not a finite-width integer") and `b@.len()` ("unsupported
   expression") in `bit_vector` blocks. The working form is DIB's and TGA's
   (`dib.rs:853`, `tga.rs:2636`): a plain `assert(x <= usize::MAX)` on a small
   literal, which the solver discharges unaided, with the arithmetic
   (`q + 12 <= MAX_INPUT + 12`, `MAX_INPUT + 12 <= usize::MAX`) in
   `by (nonlinear_arith)` blocks over `int`. Stating the bound in `u64`
   (`usize::MAX as u64`) does not help and costs runs.

## The one open obligation in the parser

**`app_ext`'s `loop_count` value.** The executable computes it by scanning the
sub-blocks after the `NETSCAPE2.0` identifier with `loop_count_of` (the verified
rule-A fold). The contract proves the structural half: the error order, the
cursor, `frames_seen`, `total_pixels` and `gce`. What is not yet proved is the
equality of the returned `loop_count` with `model::app_step`'s `loop_value` call
in that one branch, which needs `sub_scan(b, q)`'s unfolding (the identifier
payload followed by `sub_scan(b, q + 1 + size)`) to be visible to the solver.
The value is correct by construction (both compute `loop_fold` over the same
tail); only the model unfolding is missing. `app_ext_post` and `ext_step_post`
state exactly what is proved, and `STUCK.md`'s note here is the pointer.
`read_sub_blocks` and `loop_count_of` are both fully verified, so the missing
step is a lemma about `sub_scan`, not about the executable.

## Still open

- That `sub_scan` unfolding lemma (above).
- `parse_extension_verified` is not yet called by `parse::parse_extension`
  (the plain M1 function still runs; swapping it is the M3 wiring step).
- The descriptor (`image_step`, `data_step`), the walk `MB`/`MP` and
  `walk`/`step` wiring.
- L-BITS (the bit reader against the ghost `D`).
- Threading the logged constructor end to end (A-GIF-2's clause 1 needs `MP`
  equality, which needs the whole block walk).
