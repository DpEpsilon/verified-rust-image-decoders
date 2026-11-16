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
- Threading the logged constructor end to end (A-GIF-2's clause 1 needs `MP`
  equality, which needs the whole block walk).

# M5 progress (integration), runs 166-181

Baseline at run 166 was 168 verified, 0 errors (the M3 parser state). Runs 167-214
add the following, all green at run 214: **200 verified, 0 errors** (run 214 log
`runs/214.log`; run 215 was a cache hit). The tree at run 214 is the state this
note describes. Only `src/lzw.rs` and `src/model.rs` differ from the M3 tree;
the standalone `lzw_proof/` and `compose_proof/` crates are untouched.

## M5a proved so far

**L-TABLE-VIEW** (`src/lzw.rs`, runs 167-171). The three raw accessors
`raw_prefix`/`raw_suffix`/`raw_length` are now the *contracts* of the exec
getters `prefix_get`/`suffix_get`/`stored_len` and the *postconditions* of the
setters `prefix_set`/`suffix_set`/`stored_len_set` (each stated as a `Seq::update`
of exactly its own bytes), and three lemmas `lemma_prefix_set_region`,
`lemma_suffix_set_region`, `lemma_length_set_region` prove the region separation
L-TABLE-VIEW asks for: a setter changes only its own region and only its own
code's bytes. `table_len_ok(t) := t.len() == LZW_TABLE_BYTES` is the precondition
that makes every offset in range; it replaces the old `.get(..)` fallbacks, which
the lemmas now prove unreachable. The disjointness facts were split into
`lemma_regions_disjoint` (prefix vs suffix vs length, no hypothesis linking the
two codes — the suffix/length regions start above the largest prefix offset) plus
`lemma_prefix_pairs_disjoint`/`lemma_length_pairs_disjoint` (the same region,
two distinct codes). The split was what made it close: one monolithic lemma with
all 16 inequalities stalled.

**L-BITS foundation** (`src/model.rs`, runs 172-181). `payload_byte(b, q, i)` is
the `i`-th payload byte of the sub-block sequence at size byte `q`, defined by the
same recursion `sub_scan` runs. `lemma_payload_byte_concat` proves the index
identity T6's L-BITS needs: on `sub_scan(b, q) = Some((ps, e))` the walk's byte
`i` equals `D[i] = concat(ps)[i]` for every `i < |D|`. `blk_at(b, q, i)` is the
reader's state — `(size_off, left, byte_off)` — after `i` payload bytes;
`lemma_blk_at_byte` shows `payload_byte(b, q, i) == b[blk_at(b, q, i).2]` and
`lemma_blk_at_step` gives the one-step transition both within a payload and
across a sub-block boundary (where the reader must skip the size byte).

## M5a: L-BITS's model half complete (runs 216-244)

The model side of T6's L-BITS is now proved; the crate is green at run 244,
**214 verified, 0 errors**. What is in `src/model.rs`:

- `payload_bit`/`payload_code` (the bit and code view of the sub-block payload,
  G10) and `concat_bit`/`concat_code` (the same over a contiguous `D`, matching
  `lzw_proof`'s `model::bit_at`/`code_at`), with `lemma_payload_bit_concat` and
  `lemma_payload_code_concat`: **reading `width` bits of the payload equals
  reading them from the concatenation `D`**. This is the reader-refinement lemma
  the brief named.
- `rd_state`/`rd_next_off`: the executable `BitReader`'s `(pos, left_in_block)`
  after `k` payload bytes, mirroring its size-byte deferral (it reads a
  sub-block's size byte only when it needs that block's first payload byte).
  `rd_rel` + `lemma_rd_rel` prove the deferral walk and `blk_at` name the same
  payload byte at every index (`rd_next_off(b,q,k) == blk_at(b,q,k).2`), with
  `lemma_rd_nonneg` and `lemma_blk_at_step` as its pieces.
- `mask32`/`bit32i`/`low_bits_val` and `lemma_mask_eq_low_bits`: a `u32` value's
  low-`width` mask equals the low-bits value recursion. This is the bridge from
  the executable's `acc & ((1<<width)-1)` to a bit-indexed code.
- `acc_view(b, q, fetched, nacc, acc)`: the accumulator's low `nacc` bits are the
  payload bits at `[8·fetched − nacc, 8·fetched)`.

## M5a: the L-BITS model half is complete (runs 245-261)

Green at run 261, **225 verified, 0 errors**. Steps 1 and 2 of the executable
half's *model* prerequisites are proved, in `src/model.rs`:

- `lemma_low_bits_val_is_payload_code`: `low_bits_val(x, width, i)` equals
  `payload_code` when the two agree bit by bit on `[i, width)`. This is the
  bridge from a `u32` value's bits to G10's code recursion.
- `lemma_acc_push` (step 1): `acc | (byte << nacc)` extends `acc_view` by eight
  bits, given `byte == payload_byte(b, q, fetched)`. `acc_view` gained a
  conjunct `acc >> nacc == 0` (the accumulator's high bits are clear), which is
  what makes the `bit_vector` step at the shifted index go through.
- `lemma_acc_take` (step 2): `(acc & mask32(width)) == payload_code(b, q,
  8·fetched − nacc, width, 0)` — a read at the reader's bit position returns the
  G10.2 step-2 code — and the shifted accumulator keeps `acc_view` at the next
  position.

## M5a still open (the executable half of L-BITS: step 3) — SUPERSEDED

*This section and the next are the pre-run-262 plan; step 3 is now CLOSED (see
"M5a step 3" above, runs 262-323, initially 248 verified). The draft they
describe was not committed and no longer matches the proved contracts.*


`BitReader`'s struct and methods (`new`/`fetch_byte`/`read`/`bit_pos`) are still
plain Rust with no contracts; `acc_view`, `acc_push` and `acc_take` have no user
yet, and `lzw::decode`'s contract is unchanged. The remaining work is exactly
step 3: move `BitReader` (and its `impl`) inside a `verus!` block, add ghost
fields `seq_start`/`fetched`, and prove:

- `new`'s postcondition: `pos == first`, `left_in_block == 0`, `nacc == 0`,
  `acc == 0`, `fetched == 0`, `bits_left == total_bits == data_len * 8`;
- `fetch_byte`: returns `payload_byte(b, q, fetched)` and advances `(pos,
  left_in_block)` from `rd_state(b, q, fetched)` to `rd_state(b, q, fetched + 1)`,
  under `rd_step_ok` (the current byte is present, and a size byte is positive
  with its payload present);
- `read(width)`: returns `payload_code(b, q, bit_pos_before, width, 0)`, with
  `bit_pos()` advancing by `width` and `acc_view` maintained by `lemma_acc_push`
  in the fill loop and `lemma_acc_take` at the end;
- the invariant `bit_pos() == 8·fetched − nacc`, which `acc_view`'s definition
  and `bits_left`'s decrements pin.

A draft of the struct's `verus!` form (ghost `seq_start`/`fetched`, `new`'s and
`fetch_byte`'s contracts, `read`'s frame/position postcondition, `bit_pos` and a
`bit_pos_spec`) was written to the scratchpad but is **not committed**: it was
not compiled, and its `read` postcondition omits the code-value clause that
`lemma_acc_take` supplies, so committing it would have been a false claim.

## M5a still open (the executable half of L-BITS) — SUPERSEDED




`BitReader`'s `fetch_byte`/`read`/`bit_pos` are still plain Rust with no
contracts; no contract of `lzw::decode` was touched, and `acc_view` has no user
yet. The remaining steps, in order:

1. `lemma_acc_push`: `acc | (byte << nacc)` extends `acc_view` by eight bits
   (the arithmetic is drafted in the run log but not committed).
2. `lemma_acc_take`: `(acc & mask32(width)) == payload_code(…)` at the reader's
   position and the shifted accumulator keeps the view — this needs the two bit
   recursions bridged: `low_bits_val(x, width, i)` equals `payload_code` when
   `bit32i(x, j) == payload_bit(b, q, pos + j)` on `[i, width)`, which is a
   one-line induction, plus a `lemma_payload_code_concat` citation for the
   contiguous reading.
3. The exec loop: give `BitReader` a ghost field or an invariant tying
   `pos`/`left_in_block`/`acc`/`nacc`/`bits_left` to `rd_state`/`acc_view`, and
   prove `read(width)` returns `payload_code` at `pos(s)` and `bit_pos` equals
   it. Then `lzw::decode`'s contract can cite L-BITS.

Lessons recorded (all cost runs): `bit_vector` rejects `int`-typed hypotheses
("expected finite-width integer, got `Int(Int)`"), so every shift/width quantity
in a `bit_vector` block must be bound to a `u32` local first; in proof mode
`u32 - u32` yields `int`, so masks need `(n - 1) as u32`; `1u32 << (n as u32)`
inside a spec `u32` return needs an outer `as u32` on the subtraction; and a
`by (bit_vector)` on a shift with an `int` RHS is refused ("possibly negative
shift") without the cast. Also: `X + c as int` does not parse (write
`(X as int) + (c as int)`), and a Python `str.replace` in this environment
silently no-ops when the anchor carries a different quote character, so every
insertion should assert on its return value (one such miss corrupted `model.rs`
and had to be repaired).

## M5a step 3: the BitReader executable half is CLOSED (runs 262-323)

Green at run 323, **248 verified, 0 errors**; `cargo test` passes (the 373-vector
conformance suite, the requirement tests and the 37,239-frame stress test). The
L-BITS executable half of T6 is now a set of contracts, not prose:

- `model::rd_step_ok(b, q, k)` (`src/model.rs`): the reader's step condition at
  `k` payload bytes fetched — the byte `fetch_byte` dereferences is present, it is
  `payload_byte(b, q, k)` there, and at a sub-block boundary the size byte is
  positive with its payload present.
- `model::lemma_rd_step_ok_all(b, q, ps, e)`: from G9's structural scan
  (`sub_scan(b, q) = Some((ps, e))`) the step condition holds at **every** payload
  index below `|D|`. This is the structural induction over the sub-block sequence
  that both `fetch_byte` and `read` take as their standing hypothesis. Its pieces
  are `lemma_rd_state_within_first` (the reader inside a block: `q + 1 + j`,
  `n - j`) and `lemma_rd_state_shift` (crossing a boundary: the deferral walks on
  into `q + 1 + n`).
- `lzw::BitReader` and its `impl` are inside a `verus!` block, with ghost fields
  `seq_start`, `fetched` and `data_len`. `new` ensures `pos == first`,
  `left_in_block == 0`, `acc == nacc == 0`, `total_bits == bits_left == data_len*8`.
  `fetch_byte` returns `payload_byte(b, q, fetched)` and moves `(pos,
  left_in_block)` from `rd_state(b, q, fetched)` to `rd_state(b, q, fetched + 1)`;
  it does **not** advance `fetched` (see below). `read(width)` returns
  `payload_code(b, q, bit_pos_before, width, 0)`, advances the integer position
  `(total_bits − bits_left)` by `width`, and its accumulator contracts are
  `acc_view` maintained by `lemma_acc_push` in the fill loop and `lemma_acc_take`
  at the end; `bit_pos()` returns that same integer position.
- `BitReader`'s fields are `pub` and `new` is `pub` because a `pub fn`'s contract
  may not name an opaque datatype's fields and `gif.rs` calls `new`.

Three deviations from the previous agent's step-3 sketch, each for a Verus
reason, and each recorded because they change the shape of the contracts:

1. **`fetch_byte` does not advance `fetched`.** `acc_view(b, q, fetched, nacc,
   acc)` says the accumulator's low `nacc` bits are the payload bits at
   `[8·fetched − nacc, 8·fetched)`, and `lemma_acc_push` relates the state
   *before* a push to the state after it with `fetched` advanced. If
   `fetch_byte` advanced `fetched`, `read` would have to reconstruct the old
   `acc_view` from the new one. Instead `read` advances the ghost `fetched` in a
   `proof` block right after `self.acc |= byte << nacc`, so the two lemmas apply
   to exactly the states they are stated over.
2. **The contracts are integer-value predicates on the fields, not calls to
   spec functions in exec position.** `bit_pos_spec()` may not be called in an
   exec context (Verus: "cannot call function … with mode spec"), and
   `rd_state`/`acc_view` field accesses do not survive into `bit_vector` blocks.
   Every contract that mentions a field of `self` therefore writes
   `(self.total_bits as int) - (self.bits_left as int)` and the like, and every
   `bit_vector` step is fed locals (`ebl`, `ebits`, `ga`, `gp`).
3. **`fetch_byte`'s body branches once on `self.left_in_block == 0` and assigns
   each field once** (the size-byte branch and the inside-block branch are the
   two cases of `rd_state`'s step), because two assignments to `self` in
   separate branches do not let the `final(self)` postconditions unify. The
   `st = …` temporary of the sketch is gone; each branch writes `pos` and
   `left_in_block` once, only the ghost `fetched` is untouched.

Verus friction met and resolved (all in the run log):
`u32 − u32` yields `int` in proof mode, so `bit_pos_spec` returns
`(total_bits − bits_left) as u32` and every other subtraction is either a
`bit_vector` step or an `int`-typed `nonlinear_arith`; `0x1_0000_0000u32` is an
out-of-range literal, so no exec contract writes it (the `u32_max` spec helper
that replaced it was removed at run 323 after the last user went); a `u32`
`while` variant `width − nacc` is refused outright, so the fill loop's variant is
`32u32 − nacc`; `bytes += 1` needs `bytes ≤ 2` (the fill loop runs at most twice,
since `nacc ≥ width − 8 + 1` on entry and `width ≤ 12`) to discharge underflow;
and `X as int < self.bits_left as int` does not parse — write
`(X as int) < (self.bits_left as int)`.

## M5a: what is proved and what is not

Proved (runs 167-323): L-TABLE-VIEW on the raw accessors; the whole L-BITS
**model** half (`payload_bit`/`payload_code`, `concat_bit`/`concat_code`,
`lemma_payload_bit_concat`/`lemma_payload_code_concat`, `blk_at`,
`rd_state`/`rd_rel`, `mask32`/`low_bits_val`/`lemma_mask_eq_low_bits`,
`acc_view`/`lemma_acc_push`/`lemma_acc_take`); and now the L-BITS **executable**
half — `BitReader::new`, `fetch_byte`, `read`, `bit_pos` and `rd_step_ok`'s
structural availability. `lzw::decode`'s own contract is **unchanged**: it is
still plain M1 code with no contract, so T-LZW's executable integration and the
`decode`-level L-BITS statement (position at each error, per R12) remain open.
That is the next step, not part of step 3.

## M5a: step 3 recipe, exact (runs 328-359; step 3 not started)

`lzw::decode`'s rewrite is the one remaining task, and the shape is now fixed by
study of `gif/lzw_proof/src/decode.rs` (verified at M2). Concretely:

1. **Move `decode` into `verus!`** and mirror `lzw_proof`'s loop invariant: shadow
   the model's `St` in a ghost variable, so the postcondition is
   `ML(m, D, A) == L(m, D, A, ps, st)` at entry and the fold equation
   `ML == L(verdict-step)` is re-established at each iteration with
   `lzw_model::lemma_L_unfold`. Every exit then cites the branch of `verdict` it
   is: `Vd::Halt(r)` with `r` the observation for this state, giving error, `pos`
   (R12) and `e4` in one step. `lemma_L_step_pos` discharges the fold's
   `st2.pos > st.pos` guard.
2. **The code read** cites `BitReader::read`'s postcondition (code =
   `payload_code` at the position before the read, position advanced by `width`)
   and `model::lemma_lbits_read` to turn `payload_code` into
   `lzw_model::code_at(D, pos, width, 0)` — the code `verdict` computes.
3. **The emit loop** keeps two ghost accumulators:
   * `emo: Seq<u8>`, the indices in *string order*, invariant `emo == st.o`. The
     contract clause is then `emo == ML(m, D, A).idx`, since `ML`'s fold
     accumulates exactly this (`st.o`). This is what `decode`'s caller
     (the compositor, M4b) consumes.
   * `offered: Seq<(u32, u8)>`, the pairs in *call order*, invariant
     `offered == offered0 + sink_pairs(s, q, area, 0)` per code, where `s` is the
     code's string and `sink_pairs` is already proved. This is the `Sink`
     observation.
4. **The log-order subtlety step 3 must respect** (found while studying this, and
   the reason the observation is not simply `idx` zipped with positions): the
   emit loop offers positions from `q+len-1` **down to** `q`, so within one code
   the pairs are in *reverse* string order. The reverse order is load-bearing for
   the compositor (`compose::Drawer` writes, and the comment in `decode` says
   "later (smaller) positions overwrite: the earliest wins" — G12.2's
   first-violation reporting), so it cannot be flipped. The observation therefore
   cannot be stated as `offered == pairs_at(idx, q0)` directly; it is: `|offered|
   == |idx|`, `emo == idx`, and every pair of `offered` is some
   `((q0 + k) as u32, idx[k])` with the positions distinct (which with the count
   and the range pins `offered` to `pairs_at(idx, q0)` as a set of pairs). Proving
   that needs one small lemma not yet in the tree:
   `|sink_pairs(s, q, a, 0)| == |s.take(a - q)|` (an induction on `s`, mirroring
   `lemma_sink_pairs_complete`), plus `values`-level agreement from
   `lemma_sink_pairs_ok`. Both are mechanical; neither exists yet.
5. **G12.2 precedence** enters through `bad`: when any `put` above returns
   `false`, the code returns `ColorIndexOutOfRange` at the position *after* the
   code (R12), which is what makes `lzw-index-error-before-code-error`'s
   colour-before-LZW order hold — the colour test sits after the code read and
   before the string's later checks, exactly as the executable already orders it.

**Update (runs 361-363): item 4's lemma is now in the tree.**
`lemma_sink_pairs_peel` (stepping the walk past `s`'s last index is the walk over
`s.take(|s|-1)` at the previous step) and `lemma_sink_pairs_len`
(`|sink_pairs(s, q, a, k)| == min(|s| - k, a - q)`) are proved; green at run 363,
**311 verified, 0 errors**. With `lemma_sink_pairs_ok` (every offered pair is a
string pair) and `lemma_sink_pairs_complete` (every below-raster index is
offered) the `Sink` observation's three halves — count, values, positions — are
all available, so step 3's item 4 no longer needs any new lemma.

**Why step 3 is still not in this session's tree**: items 1-3 are a rewrite of
the loop into `verus!` with a ghost `St` and two accumulators (`emo == st.o`,
`offered == offered0 + sink_pairs(s, q, area, 0)`), the model-level analogue of
`lzw_proof/src/decode.rs`'s 240-line body. The M2 crate spent runs 201-226
getting that loop through; the work here is the same order of magnitude, and the
loop must also keep the two accumulators in step. It is a multi-hour task that
should start with fresh context, with everything it needs already proved.

## M5a: `lzw::decode` contract, steps 1-2 done (runs 328-358)

Green at run 358, **309 verified, 0 errors**; `cargo test` passes.

**Step 2 is complete.** What landed in `gif_verus/src/lzw.rs`, all proved:

- `sink_pairs(s, q, a, k)`: the sequence of `(position, index)` pairs G10.2 step 8
  offers the sink for a string `s` from position `q` with raster `a` — `s`'s
  indices at positions `q+len-1` down to `q`, each offered only when the position
  is below `a` (G12.2's clip; `decode`'s `pos < area` test). This is the `Sink`
  observation's exact object.
- `lemma_sink_pairs_ok`: every offered pair is one of the string's `(q+j, s[j])`
  pairs.
- `lemma_sink_pairs_complete`: every index of the string at a position below the
  raster is offered once. The domain is `k <= |s| - 1 - j` (step `k` still has `j`
  ahead); the earlier `j < k`-free form was unprovable and is superseded.
- `lemma_raw_walk_eq_seq(t, clear, c, fuel)`: the raw walk `str_of_raw`/
  `root_of_from_raw` over the three-region reservation equals the array walk
  `chain::str_of`/`chain::root_of` over `raw_pf_seq(t)`/`raw_sf_seq(t)`. It takes
  `raw_link_ok(t, clear, c)` — the raw mirror of `chain::link_ok`, parents in
  `[clear+2, c]` strictly below their entry — plus the domain clause
  `c < clear || clear + 2 <= c`.
- `lemma_walk_agrees(t, pf, sf, clear, c, fuel)`: given `chain_ok(pf, clear, c)`
  and `table_agrees(t, pf, sf, clear, c)` (the two agree on the raw prefix and
  suffix of every entry in `[clear+2, c]`), the array walk and the raw walk name
  the same string and root. It passes `refinement::table_agreement` the two raw
  sequences as the "other side".

**What remains is step 3**, and it is now a single focused task: rewrite the emit
loop's body to walk `code`'s chain over `t` (as the executable already does),
carry the ghost log (empty; `decode` needs no log — see below) and the invariant
`log == sink_pairs(ML_cur_idx, q, area, j)`, and close each exit with the R12
position. `ML` is available (`crate::lzw_model::ML`), the walk bridge is proved,
and `ML`'s `ML_idx0`/`ML_error`/`ML_pos`/`ML_e4` accessors exist
(`lzw_model.rs`). The remaining unknowns are the ordinary loop-invariant work of
a ~120-line function; there is no longer any missing lemma. `decode` does not
need a ghost log field at all: its sink *is* the log, so the contract can state
the property of the pairs it offers directly, and the loop invariant is
`log == sink_pairs(s_cur, q, area, j)`.

## M5a: `lzw::decode` contract, step 1 of 3 done (runs 328-337)

**Step 1 (adapt model/chain/dictionary) is complete.** Green at run 337, **303
verified, 0 errors**; `cargo test` passes. What landed, all as new files under
`gif_verus/src/`:

| file | content | runs |
|---|---|---|
| `chain.rs` | `gif/lzw_proof/src/chain.rs` verbatim (998 lines), with one change: `stored_len(t, c)` now reads `crate::lzw::raw_length(t, c)` (the length region's LE16 at `12288 + 2c`) instead of `lzw_proof`'s `t[2c] + 256*t[2c+1]`, so the walk is stated over this crate's three-region reservation. Plus the raw-form walk `raw_pf`/`str_of_raw`/`root_of_from_raw` that `lzw::code_first`/`chain` name. | 328, 330 |
| `refinement.rs` | `lzw_proof`'s refinement lemmas verbatim (98 lines). | 329 |
| `dictionary.rs` | `lzw_proof`'s dictionary invariants verbatim (133 lines). | 329 |
| `lzw_model.rs` | `gif/lzw_proof/src/model.rs`'s `ML` fold: `ErrK`/`MRes`/`St`/`Vd`, `lzw_ipow2`/`lzw_bit_at`/`code_at`/`nbits`, `lzw_clear_of`/`lzw_eoi_of`/`eoi_tail`/`str_ok`/`widen`, `init_st`/`clear_st`/`semit`/`first_of`/`emit_st`, `verdict`/`L`/`ML`, plus the fold lemmas (`lemma_L_unfold`, `lemma_L_step_pos`, `lemma_full_step`). Identifier prefixes `lzw_` avoid clashes with this crate's `model`/`chain`. | 336 |
| `lzw.rs` | the `LzwParams` record and the two spec records (`strict_params`, `recommended_beta_params`), `params_ok`, `lemma_m_ok_iff`, and the code-level table view `code_suffix`/`code_first`/`chain` (`THEOREMS.md` T-LZW sub-obligations, R11) stated over `raw_prefix`/`raw_suffix`/`raw_length`. | 331-335 |

Two things the adaptation required and are worth knowing:
- `#[cfg(verus_only)]` guards on the `use crate::chain::…` imports: plain
  `cargo test` compiles under `--no-cheating` where the spec functions do not
  exist, so any import of a spec-only item must carry the guard (the crate's
  `Cargo.toml` already documents that cfg).
- `ML`'s successors are still `lzw_proof`'s `St { pos, width, next, prev, q, pf,
  sf, o }` with `pf: Seq<u16>`/`sf: Seq<u8>` and `semit`/`first_of` calling
  `chain::str_of`/`root_of` on those arrays. That is fine for *step 1* (the fold
  is the same object M2 verified), but the executable's `t` is a `Seq<u8>`, so
  step 2/3 must either (a) state a relation between `(pf, sf)` and `t` on the
  readable range (`clear+2 <= c < next`) and use `refinement::table_agreement` to
  carry `str_of`/`root_of` across, or (b) restate the fold over `str_of_raw`.
  Option (a) reuses the M2 lemmas unchanged and is the recommended path.

**Step 2 (restate the observation over `Sink` with a ghost log) and step 3
(`decode`'s contract) are not started.** The plan for them is unchanged from the
section below, which remains the accurate description of the remaining work.

## M5a: the `lzw::decode` contract — scope (steps 2-3)

Runs 262-326 left `lzw::decode` **uncontracted**, and that is a deliberate stop,
not an oversight. The state is green (249 verified) and the reason matters for
whoever continues, because the previous entries under-described the work.

`lzw::decode` cannot be given T-LZW's contract by wrapping it and citing
`BitReader::read` alone. Three separate pieces are missing, in dependency order:

1. **`ML` is not in this crate.** `gif_verus/src/model.rs` has no `ML`: there is
   no `verdict`/`L`/`str_ok` fold, no `St`/`Vd`/`MRes`/`ErrK`, no `Params`, and
   no `semit`/`first_of`, and the code-level table functions the fold needs
   (`code_length`, `code_suffix`, `code_first`, `chain` of `THEOREMS.md` T-LZW
   sub-obligations, R11) are not in `lzw.rs` either — only `code_len` (the length
   alone) is. So the first work item is adapting `lzw_proof/src/model.rs` (~470
   lines, plus the `chain.rs` fuel walk it is built on, ~950 lines) into
   `gif_verus`, with the name splits changed from `lzw_proof`'s `pf`/`sf` arrays
   to this crate's three-region `u8` reservation. This is a large, mostly
   mechanical copy, and it is the reason a bare contract cannot be written yet.
2. **The `Sink` shape changes the model's statement.** `lzw_proof`'s `lzw_decode`
   writes into an index buffer `out` and states `out.take(written) == mr.idx`.
   This crate writes each index through `Sink::put(q, v)` and keeps no buffer
   (G18 forbids one). The equivalent statement is that the sequence of
   position/index pairs offered to the sink equals `ML`'s `idx` at those
   positions — a ghost log of sink calls, e.g. a ghost `Seq<(u32, u8)>` field
   accumulated in `decode` and compared against `ML(...).idx` zipped with
   `0..q`. The colour-error precedence (G12.2: a colour error precedes the LZW
   error; `lzw-index-error-before-code-error`) enters here, through what the
   sink's `false` return does to the emitted prefix, so it must be stated, not
   assumed.
3. **The chain-walk and termination lemmas.** `decode`'s inner `while j > 0`
   loop needs `code_len(t, clear, code) <= 4096` and `prefix(t, code) < code`
   (the invariant `THEOREMS.md` calls the table invariant), neither of which
   exists here yet; T11's variant `8·|D| − pos` needs `width >= m + 1`.

The pieces this chunk did add, which the continuation consumes:
`model::lemma_lbits_read(b, q, ps, e, pos, width)` (runs 325-326): the
`width`-bit code at reader bit position `pos` over the sub-block payload equals
the code G10's model reads from the concatenation `D` at the same position. With
`BitReader::read`'s postcondition (returns `payload_code` at the position before
the read, advances the position by `width`) every code the executable reads is
the code any `ML` transcribes. What is left is exactly items 1-3 above.

## M5b: the `app_ext` `loop_count` gap (runs 182-200)

The missing model lemma is now proved (`src/model.rs`,
`lemma_app_step_of_scan`, green at run 200, **198 verified, 0 errors**). It
exposes `app_step`'s success case in the shape `app_ext` computes it: from the
`sub_scan` unfolding (`subs[0]` the identifier payload, `subs.subrange(1,|subs|)`
the tail, `e` the end cursor) and the rule-A value the executable's
`loop_count_of` returns, `app_step` returns exactly the `WalkM` with cursor `e`
and that `loop_count`. Two supporting lemmas carry it:
`lemma_loop_fold_seed` (folding `s` from a seed `acc` equals folding from `None`
except that a `None` result is `acc` — the identity that lets `loop_count_of`'s
fold-from-`st.loop_count` match `loop_value`'s fold-from-`None`) and
`lemma_fold_some_seed` (a fold from a `Some` seed stays `Some`), both in
`src/model.rs`.

What this closes and what it does not: the *model* side of rule A is now
available as one callable lemma. `app_ext` still has `app_ext_post` (the
structural contract) and its body still does not cite
`lemma_app_step_of_scan`, and `ext_step_post` therefore still omits the
`loop_count` component. Wiring `app_ext` to the lemma — binding `rest`, `e`,
`netscape_match`, `first_only` and `lc` from its own locals and calling the
lemma — is the next step, and it needs the `sub_scan(b, q + 1 + size)` fact that
`read_sub_blocks` establishes (it returns `(end, total)` only, not the payload
list, so `app_ext` must re-derive the payload list or the contract of
`read_sub_blocks` must expose it). That re-derivation is why this is a separate
step, not part of the lemma above.

An attempt to prove the wired form directly (a `parse.rs` lemma
`lemma_app_step_ok` that took the executable's locals and concluded `app_step`'s
`Ok` record) was abandoned after 9 runs: the two facts its body needed were
unavailable at the call site. The `sub_scan(b, q + 1 + size)` payload list is not
in scope in `app_ext` (only `read_sub_blocks`'s `(end, total)`), and the
`b[q] != 11 != Exact` guard fact does not survive into the proof block in the
form the model's `if` needs once `b[q]` is a `Seq` read. The one-line
`0 <= q` precondition that attempt left on `lemma_sub_scan_at` was reverted with
the rest; the model lemma above states the same conclusion without either
problem, because it takes the scan's result as a hypothesis.

## M5b: the gap is CLOSED (runs 202-210)

`app_ext`'s `loop_count` obligation is proved. Runs 202-210 wired the executable
to the model lemma and strengthened three contracts, green at run 214,
**200 verified, 0 errors**:

- `app_ext`'s comparison loop (`src/parse.rs`) now carries the invariant
  `is_netscape == id_matches(b, q, i)` over the model's `id_matches` (the "first
  `i` identifier bytes match" predicate), stepping with
  `lemma_id_matches_step` and closing at `lemma_id_matches_eq`
  (`id_matches(b,q,11) == (the 11-byte payload == NETSCAPE2.0)`), also in
  `src/model.rs`. The loop's exec reads (`get`, `NETSCAPE_ID[i]`) are bound to
  model facts before the `if`, which is what the exec/spec mix needed.
- `app_ext`'s final proof block recovers the scan's payload list with a ghost
  `choose` over `crate::model::sub_scan(b@, q+1+size)`, builds
  `subs = seq![payload] + rest.0`, and calls `lemma_app_step_of_scan`; that
  gives `app_step`'s exact `Ok` record for the returned `(nsub, lc)`. No
  contract of `read_sub_blocks` changed, and `lemma_sub_scan_at` is cited for
  the unfolding.
- `app_ext_post` and `ext_step_post` now both conclude the **full** `st_matches`
  (cursor, `gce`, `frames_seen`, `total_pixels`, `loop_count`), where before they
  omitted `loop_count`. `parse_extension_verified`'s Application branch passes
  `app_ext` through unchanged.

What this closes: **T6's `loop_count` clause for the Application extension** is
now an executable contract, not an open obligation. `ext_step_post` is the exact
model equality for the whole extension dispatch. What remains for T6: the
descriptor (`image_step`) and image-data (`data_step`) steps, and the walk
(`MB`/`MP`, `walk`/`step`), which are still M1 code with no contracts.

## M5b still open

- The descriptor (`image_step`) and image-data (`data_step`) steps and their
  equality with the model; the walk `MB`/`MP`, `walk`/`step` wiring; the
  constructor's A-GIF-2 clause 1; the iterator T-ITER/T-PLAYS/T9; T12/T13.
- The executable half of L-BITS.
- `parse_extension_verified` is still not called by `parse::parse_extension`
  (the plain M1 function runs); swapping it is the wiring step.

## What resisted, and the resolutions

- **Nested `verus!`** (run 167, compile error): the L-TABLE-VIEW block opened a
  second `verus! {` inside the module's own block. Removed; every item lives in
  the module's one block.
- **`X as int` parse** (run 168): `SUFFIX_OFF + c as int` does not parse; the
  form is `(SUFFIX_OFF as int) + (c as int)`.
- **`bit_vector` on `int`** (run 173): `SUFFIX_OFF as int == 8192` inside a
  `bit_vector` block is rejected ("expected finite-width integer, got
  Boxed(Int(Int))"). `SUFFIX_OFF + c2 != SUFFIX_OFF + c` follows by plain
  arithmetic from `c2 != c`; the `bit_vector` block was not needed.
- **`concat_len` unfolding in a proof fn** (runs 174-176): `concat_len(ps)` on a
  `ps` known to be `seq![p] + t` does not unfold by itself, and asserting it needs
  `u.subrange(1, u.len()) =~= t`. Both are now inside `lemma_concat_append_one`.
- **`concat(rest).len() == concat_len(rest)`** (run 177): not automatic; the
  bridge is `lemma_concat_len(rest, Seq::empty())`.
- **`lemma_seq_add_index2`'s precondition** (runs 175-176): it wants the index
  `s1.len() <= i`; the shifted index `i - n` is not in that form, so the call
  takes `i` and the conclusion is bridged with `payload.len() == n`.
- **`blk_at(b,q,i).1 >= 2` is false in general** (run 179): it holds only when
  `i + 1` is still inside the same payload, which the boundary branch of
  `lemma_blk_at_step` does not have. The lemma now concludes the weaker
  `>= 1`, which is what the caller needs.

Lesson (the same as M3's): name the intermediate, keep each lemma to one fact,
and state arithmetic over plain `int` locals rather than inside `bit_vector`.


## M5a step 3: the log observation, and the resume point (runs 360-366)

Step 3 is not done. Runs 365-366 landed two prerequisites and pinned the design;
the loop rewrite itself needs the discovery schedule below. Green at run 366,
**313 verified, 0 errors**; `cargo test` passes (checked at run 364's tree, whose
`src/lzw.rs` differs only in the two spec items below).

### What landed

- `model::lemma_concat_len_add(s, t)`: `concat_len(s + t) == concat_len(s) +
  concat_len(t)`. The loop's log invariant splits the finished codes' payload list
  at every code, so this is needed once per iteration.
- `lzw::u32_of(i)`: total `int`->`u32` (out-of-range maps to 0; every use is on a
  position below `MAX_TOTAL_PIXELS = 2^30`, so the branch is never taken on a
  reachable call).
- `lzw::code_log(s, q, a, k, q0, j)`: the per-code piece of the sink log `decode`
  will accumulate. It is `sink_pairs`'s walk (string order `s[len-1]` down to
  `s[0]`, only positions below `a`) with one extra pair at **every offered
  position**: the value offered is `s[i]`, the position reported is
  `q0[j.len()-1] + i` — the *latest earlier* code's base plus the same index,
  which is the position that earlier code's sink call actually wrote. `q0` and
  `j` grow by one entry per sink call of the current code (step `k` appends
  `q + i` and `k`), so `j.len()` is the step count and `q0[j.len()-1]` the base
  to report. `k >= s.len()` is the base case, but the clamp argument below never
  reaches it.

### Why the log is a pair sequence, not `idx` zipped with positions

`idx` alone is not enough: the sink is handed `(q + |s| - 1 - k, s[len-1-k])` for
the index already written at that position, and `s[len-1-k]` equals the *current*
string at index `i = len-1-k` only *sometimes* — a collision with an earlier code's
string has both indices agreeing when their strings share the suffix. So the log
a call site can hope to equal is not `idx[t] / positions[t]` but a pair sequence
in which every offered pair carries the value actually offered. The contradiction
with `st.o` under a collision (below) is what makes the emit branch splice rather
than append.

### The collision in the emit order (found at run 365; the design's crux)

A string can be offered at a position a **later** code's earlier-offered string
also occupies, and the values differ. Witness on a 2-colour frame
(`m = 1`, `D = [140, 11]`, `A = 2`; bit 9 in `lzw_proof`'s regression, which counts
`L(2, D, 2)` as `[1, 1]`, bit 9):

- step 4 (Clear) and step 5 (literal 1) resolve to `[1]`, emitted at `q = 2`
  (a no-op, `q == A`).
- step 6 is code 6 = `next`, the KwKwK string `prev + first(prev) = [1, 1]`, emitted
  at `q = 3` with positions `4` and `3`.

String order for `[1, 1]` is index `1` at `3` then index `0` at `4`, and the walk
offers `4` before `3`. On the 2-pixel raster `A = 2` both are `>= A`, so
`code_log`'s guard drops them and offers nothing; but with the raster set to cover
positions `3` and `4` (the same frame drawn at `left = 3`, or `A = 5` with three
pixels already emitted) the walk offers `(4, 1)` then `(3, 1)`, while
`idx`-zipped positions want `(3, 1)` then `(4, 0)` — the position `4`'s value is
`0` from the model's string and `1` from the offer at position `4`. No total
function of `(idx, positions)` equals the call log here, which is exactly the
`THEOREMS.md` "log-order" finding (step 3 item 4) and why the observation must be
count + values + positions, not the pair sequence itself.

### The invariant, and why it cannot be a plain "log == code_log(s, q, area, j, hints)"

The constants the executable's per-code call log matches are not a prefix of the
explanatory constant sequence. Verified by hand on `A = 2`, `s = [1, 1]`, `q = 1`:

| step | model string | call log |
|---|---|---|
| step 4 (Clear) | none | none (`q = A`) |
| step 5 (`[1]` at `q = 2`) | `(2,1)` | `(2,1)` |
| step 6 (`[1,1]` at `q = 3`) | none | none (`4, 3 >= A`) |
| step 7 (Clear) | none | `q = A`, none |
| step 8 (`eoi`) | none | none |

but an earlier 2-pixel emission at `q = 1` with a 4-bit width puts `(2, 1)` before
step 4's step, so the executable's constant sequence is the model's with an insert
at the call of the code whose string's index `0` lands exactly on `A`. So
"`log == code_log(...)` at every iteration" is false and must not be attempted.
What `decode` can maintain per iteration is the pair sequence *it* produced
(`offered == off + the current code's calls`), with `off` the loop's own ghost
accumulator, and the **model** equality proved per exit from the `Vd` branch:

- at exit, `off`'s calls for the halting code are the prefix the model emits
  (`Q + s.take(limit)` with `limit = min(s.len(), A - q)` for step 8's overrun;
  nothing for a step-1/4/5/6 error, since the model emits `st.o` only), and
- each pair's value is the model's `idx[t]` and the positions are distinct and
  `(q0k + k)`, i.e. the count/value/position triple of step 3 item 4.

### Resume point, concretely

1. Move `decode` into the second `verus!` block (line 553) with the contract
   sketched above; the only plain-build change needed is `Sink` (line 528) and the
   `compose::Drawer` impl (line 141) moving inside a `verus!` block so the trait is
   usable there — Verus will then require a contract on `Sink::put` (give it
   `ensures true` first, then the G12.2 clause when M4b lands). `BitReader`,
   `LzwOptions` and `code_len` (with its `table_len_ok` precondition, which the
   loop invariant carries as `t@.len() == LZW_TABLE_BYTES`) are already inside.
2. State the loop invariant as: the position/width/next/prev/q/side tables quoted
   in `lzw_proof/src/decode.rs`'s lines 84-105 *translated to this crate's raw
   accessors* (`table_ok`'s counterpart `raw_link_ok(t@, clear, next)`, plus
   `code_len(t@, m, x) == raw_length(t@, x)` on `[clear+2, next)`), and
   `ML(m, D, A) == L(m, D, A, st)`, with the log side as `offered` and `emo`.
3. The tree transition (`t` -> `st.pf`/`st.sf`) is `lzw::lemma_walk_agrees`
   (`t`, `raw_pf_seq(t@)`, `raw_sf_seq(t@)`, `clear`, `x`), whose `table_agrees`
   hypothesis is exactly the raw-readable-range invariant; call it after every
   mutating step with `c := next - 1` (the freshly written entry, proved by
   `lemma_prefix_set_region`/`lemma_suffix_set_region`: the setters change only
   their own code) and once at entry for the whole range.
4. The code read: `BitReader::read`'s postcondition gives
   `code == payload_code(b, seq_start, pos_before, width, 0)`; `model::lemma_lbits_read`
   (with `ps`/`e` from `sub_scan` at `seq_start`, available as the standing
   hypothesis `lemma_rd_step_ok_all` already is) turns it into
   `lzw_model::code_at(D, pos_before, width, 0)`.
5. Every exit closes with `verdict`'s branch for that state (`Vd::Halt(r)`), so
   `ML = L(st) = r` by `lemma_L_unfold`; `lemma_L_step_pos` discharges the fold's
   `st2.pos > st.pos` guard at each advancing step, and the position clause of
   R12 is the `r.pos` the branch carries (step 1 before the read, every later
   error after the code). The colour case is the `bad` exit: a `put` returning
   `false` gives `Err(ColorIndexOutOfRange)` at `reader.bit_pos()` after the code,
   which is `verdict`'s `checked_literals`/step-8 colour branch — do **not** model
   G12.2's colour check inside `lzw_model` (its `verdict` has only the
   checked-literal colour branch, since the sink's semantic is M4b's).
6. Only then replace the plain `decode`: `cargo test` (373 vectors, the stress
   run) must pass with the same body; the plain-build form of the ghost
   parameters is the `Ghost<Seq<...>>` form `dib` uses
   (`alloc-delta/proto/alloc_audit/pinned-dib/src/dib.rs:4569`), which `verus!`
   erases — the ghost accumulators can be declared inside the body as
   `let ghost mut offered = Seq::empty();` so the signature does not change.

# M5 emission, memo design (DESIGN-M5-EMISSION.md): steps 1-3 landed

Baseline at run 366 was **313 verified, 0 errors**. Runs 367-411 follow the
design memo's 7-step order. The tree is green at run 411, **453 verified, 0
errors**; runs 388-389 are Verus cache hits on the byte-identical tree green at
run 387 (435), and the run-411 tree re-verifies in full.

## What the memo asked, and what landed

**Step 1 (retire `code_log` and the collision story): done, run 367, 312
verified.** `lzw::code_log` and `lzw::u32_of` are deleted (their only users were
each other), and `model.rs`'s `lemma_concat_len_add` doc no longer names
`code_log`. `sink_pairs` and its four lemmas (`lemma_sink_pairs_ok`,
`_complete`, `_peel`, `_len`) are kept unchanged. The count drops by exactly the
one item removed: `code_log` was one `spec fn`.

The memo's counterexample was checked against the frozen oracle before acting on
it (`python3 -I`, `timeout`). For `D = [140, 11]`, the oracle's `lzw_decode`
calls `emit` with exactly:

| `m`, `A` | calls | error |
|---|---|---|
| 1, 2 | `[(0,0)]` | `TooFewPixels` |
| 1, 5 | `[(0,0)]` | `TooFewPixels` |
| 2, 2 | `[(0,1), (1,1)]` | `TooManyPixels` |
| 2, 5 | `[(0,1), (1,1), (2,1)]` | `TooFewPixels` |

This confirms the memo's corrected trace (m = 2, A = 5 emits position 2 and not
a model value 0 at position 4) and its contradiction of `code_log` (which for a
singleton string fabricates two pairs where the executable calls `put` once).
The collision story in the previous STUCK sections is superseded: reverse writes
are permitted by G18, and `code_log` was a false witness.

**Step 3 (repair `BitReader::read`'s precondition): done, run 371, 312
verified.** The precondition `old(self).fetched < old(self).data_len` is now
`<= old(self).fetched <= old(self).data_len`. Two proof edits were needed, both
because the old strict bound was doing double duty:

1. The entry assertion was `nacc < bits_left` (geometric: `fetched < data_len`
   and `bits_left >= width`). At `fetched == data_len` that bound is `0 < 0`,
   false. The entry bound at `nacc <= bits_left` is recovered from
   `nacc = 8·fetched − gp` and `bits_left = 8·data_len − gp` alone, and the
   fill-loop body derives `fetched < data_len` itself from the loop test
   `nacc < width` and `bits_left >= width` (that derivation already existed at
   `read`'s inner `assert((self.fetched as int) + 1 <= gd)`).
2. The separate `assert(self.nacc <= 32)` after the shift has no precondition of
   its own and forced `fetched < data_len` at *every* entry, not only on entry
   to the fill loop. It is discharged by `nacc < 32` alone (the arithmetic
   `nacc < bits_left <= total_bits = 8·data_len <= 8·2^28`), so the entry bound
   no longer leaks in.

**Step 2 (Sink contract and the Drawer adapter): the Sink contract and the
adapter are done and proved; the memo's "Move `Sink` into `verus!`" and the
per-call `draw_step` relation are the parts that landed.**

- `lzw::Sink` moved inside the second `verus!` block and now carries
  `sink_area`, `sink_inv`, `sink_ok` and `sink_log`, with `put`'s contract: it
  requires `q < sink_area` and the invariant, and ensures the Boolean is
  `sink_ok(v)` (a function of the offered index alone, independent of earlier
  calls and of call order — the memo's point that a sink must not latch a `false`
  after one rejection), the log is `log.push((q, v))`, and the invariant is
  preserved. `Tail` and `LzwOptions` moved in beside it (the second `verus!`
  block gained them; the first lost only `code_log`/`u32_of`).
- The M4 compositor `compose_proof/src/{model.rs,interlace.rs,compose.rs}` was
  ported into `gif_verus/src/{mc.rs,interlace.rs,compose_new.rs}` (module refs
  rewritten `crate::model::`→`crate::mc::`, `crate::compose::`→
  `crate::compose_new::`; `compose.rs` had no `#[cfg(test)]` block to strip).
  This is why the verified count jumps from 312 to 435: M4's entire proof set
  (T4, T5, T7, T8) now compiles and verifies inside `gif_verus`.
- `compose_new::DrawerSink` implements `lzw::Sink` for the compositor: it builds
  the temporary `DrawState { q }` the memo prescribes (a fresh state per call, so
  no persistent counter is reused and the interval-cache proof the memo warns
  about is not needed), names `sink_ok(v) = transparent == Some(v) || v < c`, and
  keeps the ghost log. Its `put` delegates to the already-proved `draw_index` and
  adds the canvas clause `final(canvas) == draw_step(...)`'s `Ok` canvas, or the
  incoming canvas on `Err`, via `lemma_draw_step_ok_shape`
  (`draw_step(..) is Ok <==> drawer_ok(fr, v)`).
- `gif_verus/src/compose.rs` is now a re-export shim. The M1 bodies that lived
  there — an unverified `interlace_row`, `clip`, `clear_rect`, `copy_rect`, the
  interval-cached `Drawer` and its `impl Sink` — are gone; they were plain M1
  Rust that no verifier had touched, and their `impl Sink` cannot satisfy the new
  trait (no `sink_ok`/`sink_log`). The proved M4 module replaces them. `Drawer`
  is now a re-export of `DrawerSink`, so `gif.rs`'s call site was updated to
  build a `compose_new::Frame` (`c: fh.table.entries`, `disposal as u32`) — the
  row cache of the old `Drawer` is not represented in `DrawerSink`, whose
  per-call cost is `draw_index`'s two divisions per written pixel (the memo's
  "the temporary-state adapter avoids needing a monotone cache proof").

## Honest gaps (as of run 411)

- **The memo's Drawer clause is stronger than what is proved.** The memo asks
  `put`'s ensures to be `final(canvas) == draw_step`'s canvas. `lzw::Sink` is a
  generic trait and its `put` contract cannot name `compose_new::draw_step`
  without an upward reference from `lzw` to the compositor, so `DrawerSink`'s
  `impl` adds that clause to its *own* `put` (a trait method may add `ensures` to
  the inherited `requires`), which discharges at every concrete `DrawerSink::put`
  call but not at a generic `S::put` call inside `decode`. `decode`'s own T-LZW
  observations (steps 4-5) therefore do not depend on it; the compositor's
  whole-canvas equality (step 6) consumes it at the concrete call.
- **Steps 4-7 are not started at the decode level.** `lzw::decode` is still the
  plain M1 body with no contract: the report type and its contract, the four
  T-LZW observations, the timestamped minimum-fault statement, L-INTERLEAVE's
  projection, the permutation/pixel lemmas and the m admissibility wiring are all
  open. What has landed is its prerequisite stack (items 1-4 of the resume point
  below). The memo's own §4 says no erratum follows from the design, and none is
  needed so far.
- **Two design constants are not yet pinned.** The memo's `ColourFault` carries
  "the position after its code". `Sink::put` receives only `(q, v)`; the
  after-code bit position must come from `BitReader::bit_pos()` at the call site
  inside `decode` (which knows it), so the report's `colour: Option<ColourFault>`
  is assembled there and not from the sink. Its ghost `B` sequence (the codes'
  base `q`, resolved `s` and after-code `e`) is a `decode`-local ghost, as the
  memo says.
- **`gif.rs` still calls the old `lzw::decode` signature** (`Result<bool, Error>`)
  with `DrawerSink`; it compiles and the 373-vector conformance suite and the
  37,239-frame stress test pass in both debug and release at run 411. Swapping
  the call site to the report signature is M5b wiring.

## Resume point for steps 4-5 (exact)

Everything they need is now in the tree (runs 366-411):

1. `Sink`'s contract (above), `BitReader::read`/`bit_pos` with L-BITS
   (`lemma_rd_step_ok_all`, `lemma_lbits_read`), `ML`/`L`/`verdict` and
   `lemma_L_unfold`/`lemma_L_step_pos` in `lzw_model.rs`, `lemma_walk_agrees` /
   `raw_link_ok` / `table_agrees` in `lzw.rs`, `sink_pairs` and its four lemmas,
   `dictionary.rs`'s `table_ok`/`valid`/`chain_valid`/`history*`,
   `model::lemma_concat_len_add`.
2. **The log spec layer is landed and proved as of run 411, 453 verified**
   (`log_has_at`, `log_block_at`, `log_pos_below`, `log_vals`, `log_below_ok`,
   `log_below_covers`, `log_complete`, `log_block`, and the two append lemmas
   `lemma_log_below_ok_append` / `lemma_log_covers_append` with
   `lemma_log_complete_append` on top), plus two bridges it needs:
   `lemma_log_block_sink_pairs` (`sink_pairs(s, q, a, 0)` is `log_block` for the
   string's `j` range, given `o[q + j] == s[j]` on the string), its supporting
   `lemma_sink_pairs_pos_below` (every offered position is below the raster), and
   `lemma_u32_roundtrip` (`(x as u32) as int == x` for `0 <= x <= 2^32 - 1`,
   proved `by (nonlinear_arith)` over `int` with the shift bound supplied —
   `bit_vector` refuses an `int`-typed operand, so it cannot prove this cast).
   The log invariant the loop runs is now mechanically steppable once per code:
   `lemma_log_complete_append` grows `log_complete` given `log_block`, and
   `lemma_log_block_sink_pairs` supplies `log_block` from the code just emitted.

   **One dead end worth recording.** A first attempt used a push-recursion
   `log_raster(log, n)` with the aim `log_raster(log, q) == o`. That target is
   false: the recursion is "last pair naming a position wins", the log is a
   permutation, and the last pair sits at the last string's smallest position,
   so `log_raster(log, q)` pushes `o[h.0]` onto `o.take(q-1)` and equals `o` only
   by luck. The first observation does not need it: the loop carries `O == st.o`
   and `lemma_L_unfold` gives `ML(idx) == st.o` at any halt. `log_complete` is
   still needed, for `|H| == q` (the report's `written`) and the fault, but it
   does not imply a `log_raster` equation on its own, and it must not be applied
   to a truncated log (`log_complete(log, o, q-1)` is false when `log.len() == q`).

3. **Attack order that fits the machinery**: prove the no-fault success case
   first (T-LZW's success branch: `O == ML(...).idx` follows from the loop ghost
   `O == st.o` and `lemma_L_unfold`, and `log_complete`'s `|H| == q` gives the
   report's `written`), then the pure-error branches (`lemma_L_unfold` per exit),
   then the colour fault. Each `put`'s `log.push` (trait ensures) and `sink_ok`
   are the only facts the loop needs from the sink; `sink_area` discharges
   `put`'s `q < A` at every call site because `decode` guards `pos < area`.
4. **Two concrete footguns met while proving the append lemmas** (each cost a
   run or two; both are reusable knowledge):
   - `vstd::seq::lemma_seq_add_index1/2` demand the *split-index* form
     (`i < s1.len()` / `s1.len() <= i < s1.len() + s2.len()`); for a fixed index
     into the second part use `lemma_seq_add_index1_alt` / `_alt2`, which take
     `0 <= i < s.len()` and conclude `(s1 + s2)[i + s1.len()] == s2[i]`.
   - `choose|x: int| ... (log[x].0 as int) == p` fails outright with "value may
     be out of range of the target type" when the cast is inside the binder;
     write the position in the extended (`int`) form `(log[i].0 as int) == p`
     everywhere, so no quantified `u32`-to-`int` cast appears. Likewise
     `forall|i| 0 <= i < log.len() ==> (log[i].0 as int) < n && log[i].1 ==
     o[log[i].0 as int]` cannot infer triggers; split it into `log_pos_below`
     and `log_vals`.

## M5 emission, decode rewrite attempt (runs 413-421): where it blocked

Runs 413-414 added the report layer to `src/lzw.rs` and it verified (re-added
and re-verified at run 423 after the body attempt was parked, still 453)
(`LzwReport { written, error, pos, e4, h: Ghost<Seq<(u32,u8)>> }`,
`lzw_ps(m, opts)` — the `LzwParams` a `decode` call's `(m, opts)` denotes,
`lzw_ps_known`, `err_of`/`errk_to_error`, `report_matches`, `read_d`). Green at
run 414, 453 verified.

Runs 415-421 lifted `decode` into `verus!` with the memo's report contract and
did not get it through. The body was then parked: the tree carries the run-411 helpers plus the
report layer, and re-verifies at run 423 (453 verified, 0 errors; tests pass). What the attempt established,
and the exact blockers, so the next session does not repeat them:

**The contract that compiled** (up to the body's obligations):

```
pub fn decode<S: Sink>(reader, data_len, m, area, opts, t, sink) -> (rep: LzwReport)
  requires
    data_len <= MAX_INPUT, area <= MAX_TOTAL_PIXELS,
    sink.sink_area() == area as int, sink.sink_inv(),
    old(reader).data_len == data_len, old(reader).seq_start as int >= 0,
    old(reader).b@.len() == old(reader).b.len(),
    old(reader).b.len() <= MAX_INPUT,
    sub_scan(old(reader).b@, old(reader).seq_start as int).is_Some(),
    old(reader).bit_pos_spec() == 0,
    table_len_ok(old(t)@),
  ensures
    rep.h@.len() == rep.written as int,
    final(sink).sink_log() == old(sink).sink_log() + rep.h@,
    rep.h@.len() <= area as int,
    rep.pos as int <= 8 * (data_len as int),
    lzw_ps_known(m, opts) ==> report_matches(rep,
        ML(m as int, read_d(old(reader).b@, old(reader).seq_start as int),
           area as int, lzw_ps(m, opts))),
```

and inside, `let ghost h0 = sink.sink_log(); let ghost mut h = Seq::empty();`
with the loop invariant `h.len() == q as int`, `sink.sink_log() == h0 + h`,
`table_len_ok(t@)`, `1 <= width <= 12`, `next <= TABLE_SIZE`, `q <= area`,
`m >= 1 && m <= 11`, `decreases area - q`. Every exit returns
`LzwReport { written: q, error: ..., pos: reader.bit_pos(), e4: false, h: h }`.

**Blockers (run 421, four error classes; 11 diagnostics):**

1. **`add_entry` has no contract and is outside `verus!`.** Verus refuses to call
   it from a `verus!` body ("ignored because it is declared outside the verus!
   macro"). Wrapping it in `verus!` removes the error but then it needs its own
   contract whose `table_len_ok(t@)` precondition the loop must carry — which
   then appears at `prefix_set`/`suffix_set`/`code_len` inside `add_entry`
   (run 421's errors at lines 2082-2084). Fix: give `add_entry` a contract
   `requires table_len_ok(old(t)@), next < TABLE_SIZE, width <= 12, ...` and the
   postcondition that only entry `next`'s three regions change (the
   `lemma_*_set_region` facts already exist), plus the width/next update.
2. **`clear + 1` overflow**: `let eoi: u32 = clear + 1` needs `clear <= 2^32 - 1`,
   from `m <= 11` (`clear = 1 << m <= 2048`). One `assert(clear <= 2048) by
   (nonlinear_arith)` / `bit_vector` before it; the `1u32 << m` itself needs
   `m < 32`.
3. **The loop's ghost `h` must replace `Seq::empty()` at every exit**, and the
   `|h| == q` postcondition then follows from the invariant. (The draft already
   wrote `h: h`; the compile errors at 1986/1990 were the *old* `Seq::empty()`
   exits — the ones still carrying `Ghost(Seq::empty())` because they were not
   updated.)
4. **`q + j`, `suffix_get`/`prefix_get` bounds** in the walk: `q + j` needs
   `len <= TABLE_SIZE` (so `q + j` stays below `area + 4096`, inside `u32`), and
   the accessors need `table_len_ok(t@)` visible, i.e. item 1's contract.

**The step-5 observations are a further, larger body:** beyond these four the
loop needs (a) the ghost model state `st` with `ML == L(st)`, advanced by
`lemma_L_unfold` + `lemma_L_step_pos` per exit; (b) the readable-range table
agreement `raw_link_ok(t@, clear, next-1)` after every `add_entry`, via
`lemma_walk_agrees`/`lemma_prefix_set_region`; (c) `lemma_log_complete_append` +
`lemma_log_block_sink_pairs` per emitted code to keep `log_complete(h, O, q)`,
which needs the emitted `O` (`st.o`) and the per-position `o[q+j] == s[j]` fact
from the chain walk; and (d) the `q + len > area` / KwKwK exits each citing
`verdict`'s branch. That is the M2-sized body (`lzw_proof/src/decode.rs`'s
240 lines) plus the log and report; it fits the machinery now in the tree but is
a multi-hour proof, not a one-sitting edit.

**Resume point:** start from the run-411 `src/lzw.rs`, add `add_entry`'s contract
plus the three small bound asserts (items 1-4 above) to get the report contract
*stated and compiling* with the body's exits passing `h: h` — that alone is the
first green point (contract present, model equality still `==>`-gated) — then
discharge the model/log invariant per exit in the memo's case order (1) no-fault
success, (2) pure errors, (3) colour fault.

## Integrated LZW core proved (Sol, runs 425-457)

The tree is green at run 457: **502 verified, 0 errors**, from the run-423
baseline of 453. The core first went green at run 447 (480); colour projection
went green at run 449 (484). All runs used the existing `gif/verify.sh`, one at
a time, `timeout 25m`, one cargo job and default solver limits. No assume,
admit, external body, unsafe, axiom or raised resource limit was added.

`src/lzw.rs::decode` is now the verified executable, not a parked body or a
conditional model claim. Its postconditions establish the pure count, exact
error, R12 bit position and E4 against `ML`; `log_complete` and `raster` establish
`raster(rep.h) == ML.idx` on every outcome. The preconditions are a scanned,
fresh reader, the G1 caps, G9-admissible minimum size, the sink invariant and
area, and only the reservation's length. Incoming table bytes remain arbitrary.
The loop maintains `ML == L(st)`, typed table/cache/history facts, raw link
validity, readable-range model/raw agreement and the complete sink log. Its
variant is the remaining payload bits; each advancing iteration consumes the
current width. The inner walk maintains performed calls plus `sink_pairs` of
the unvisited portion, and decreases the remaining chain length.

The report carries a separate `Option<ColourFault>` and erased code timestamps.
A rejected put does not stop emission or dictionary evolution. `fault_matches`
keeps the minimum rejected raster position; `log_time_agrees` connects physical
call order to logical timestamp order. `code_stamps` follows the pure model
fold, so the saved position is the position after the offending index's code,
not the reader's later position. `colour_observation` is a decode postcondition.
`lemma_colour_projection` and `lemma_public_projection` prove L-INTERLEAVE and
first-invalid-index precedence. `public_result` projects a saved colour fault
before the pure error; `public_pos` projects the saved colour timestamp.

Two prerequisite defects were repaired: `Sink::put` now preserves `sink_area`
and every value of `sink_ok`, and `lzw_ps` selects minimum-size policy from
`checked_literals`, not from `m`. The latter covers RecommendedBeta's m=1 and
m=2..8 calls as well as m=9..11, with G9's one-pixel condition for m=1.
Both tail modes work for each minimum-size policy.

The monolithic decoder query exceeded the default resource limit. Splitting
`read_step` (control and reader), `run_code` (emission and dictionary), and
model/raw, timestamp, log and colour lemmas resolved it. Keep raw view,
reader, chain and observation predicates hidden in the main fold query;
re-expanding their quantified definitions there reintroduces the bottleneck.
`lemma_code_models` is the missing explicit bridge from `model::concat_code`
to `lzw_model::code_at`; the earlier reader lemma alone did not state it.
The ordinary insertion now happens before the crossing result is projected;
this matches G10 step 7 and changes no exposed error canvas or pure report.

After the core proof, memo steps 6-7 advanced:

- `src/emission_compose.rs` proves retained-cell injectivity under Clip,
  generalized per-put pixel preservation including transparent indices beyond
  C, commuting successful puts, and whole-canvas equality for arbitrary
  successful call-log permutations with the same pair multiset.
- `src/parse.rs::scan_image_data` is now verified with an exact G9 postcondition:
  minimum-byte truncation, policy minimum-size error before scanning, structural
  truncation, or the exact minimum size, first byte, payload length and end.
- `BitReader::new_scanned`, `lemma_min_admissibility`, `lemma_image_data_ready`
  and `decode_image_data` connect those facts to the core. The actual frame path
  now calls `decode_image_data(...).public_result()`.

Release tests pass: all 373 conformance vectors, allocation fault tests and the
37,239-frame stress test, plus five new direct report tests in `src/lzw_tests.rs`.
The new tests check pure continuation after colour failure, saved positions,
KwKwK/crossing/E4, pre-read versus post-code errors, checked literal 300,
one-byte sub-blocks, m=1 and arbitrary nonzero/stale table bytes. The final test
log is `gif/runs/sol-20261116/tests-final.log`. All hashes in
`spec-frozen/FREEZE.txt` still match; no frozen file was edited and no erratum
was needed.

### Remaining integration boundary and precise resume point

Do not claim T7/T9/T12/T13 or full iterator integration from this result.
The permutation theorem is proved over `draw_calls`, but the generic Sink
contract still exposes only its log/validity predicate, not its canvas action.
The concrete Drawer method's extra canvas postcondition is not available inside
a generic `S::put` call. Connect that action through an abstract Sink effect or
a preserved history relation, instantiated by Drawer, then apply the proved
permutation theorem. Also prove the pair-multiset bridge from `log_complete`
to the canonical forward `(q, idx[q])` log and the forward `draw_calls`/
`mc::draw_fold` bridge. No equality of private error canvases is required.
The generalized pixel lemma is available but not yet an integrated canvas
invariant. The outer frame/iterator still needs fusion/non-exposure and full
parser-state wiring. Step accounting remains open: no concrete ghost step
counter or complete T12 sum was added in this session.

## Continuation through M5b (Sol, runs 458-476)

The coordinator requested continuation through the canvas boundary and M5b.
Final run 476 is green: **529 verified, 0 errors**. The preceding section's
remaining log/permutation bridges and generic Sink-effect gap are now resolved
at the concrete Drawer call; the outer Frames invariant is still not proved.

`Sink` now exposes `sink_view` and an immutable first-class `sink_effect`.
Every actual generic put establishes the effect on the view and preserves its
length. `emit_raw`, `run_code`, `decode` and `decode_image_data` propagate the
exact `effect_fold(initial_view, actual_call_log, initial_effect)` relation.
Drawer instantiates that action with `draw_step` on success and identity on
rejection. The main decoder again needed one smaller proof boundary:
`verdict` stays hidden and `lemma_control_emit` exposes its emission branch;
no solver limit was raised.

`lemma_log_canonical_multiset` proves that `log_complete` gives exactly the
canonical forward position/index multiset. `lemma_forward_draw_fold` identifies
forward `draw_calls` with `mc::draw_fold`. `lemma_complete_log_canvas` combines
those bridges with successful permutation and the effect fold.
`compose_new::decode_drawer` calls the actual generic decoder and proves whole
canvas equality on its concrete Drawer state whenever no sink colour fault
occurs. It also states the exact public colour-before-pure-error projection and
preserves the pixel-origin predicate for every table-history set containing the
active table, even on rejected private frames. The actual frame path now calls
this concrete entry, not the generic LZW entry directly.

The concrete canvas theorem observes `final(sink).canvas` against
`old(sink).canvas`. An attempted additional wrapper taking an independent
`canvas: &mut [u8]`, moving/reborrowing it into Drawer and promising a postcondition
on that original formal parameter did not prove: the current abstraction did
not rejoin that outer borrow. That failed wrapper was replaced, not admitted or
left with a weaker claim. When lifting Frames, preserve the owned canvas state
or explicitly prove the borrow rejoin; do not silently assume a postcondition
on the enclosing Frames canvas from a different mutable-reference binding.

M5b parser contracts now cover the actual functions:

- `parse_image_descriptor` proves G8's exact model continuation or error, all
  descriptor fields, active-table byte bounds and frame/pixel budget checks.
- `parse_data_step` proves `model::data_step`, including GCE consumption, exact
  minimum size and D, cursor and bounded counter increments after a full scan.
- `parse_image_step` composes them into exact `model::image_step` equality.
- `parse_mb` proves the complete `model::mb` outcome, including extensions,
  Trailer/EOF policy, exact first errors and the complete successful image/state.

`hdr_inv` now includes the global table's offset 13; `parse_header` proves the
strengthened invariant. `header_model` and `frame_header_model` name the exact
exec-to-model record conversions. The MB loop carries the model's decreasing
extension fuel separately from its byte-cursor variant. The new
`lemma_sub_scan_bounds` and `lemma_ext_success_progress` establish progress,
state counter preservation and the post-first-frame loop-count property.

The old plain `parse_extension` body has already been replaced by a verified
wrapper around `parse_extension_verified`, since MB needs that contract.
`Frames::step` now calls `parse_mb`, and `Frames::image` receives its already
parsed `ImageStep`; it no longer repeats G8/G9 or duplicates their state updates.
The drawing-only sink frame normalizes undefined disposal to Keep (1), while
public FrameMeta retains the raw disposal value. This discharges the sink's
existing disposal-domain invariant for RecommendedBeta's accepted 4..7 values
without changing drawing, save/dispose decisions or returned metadata.

Final runtime log: `gif/runs/sol-20261116/tests-m5b-final.log`. Release tests pass
for all 373 vectors, the five report tests, the four allocation-fault tests and
the 37,239-frame stress test. All 14 frozen manifest hashes match. No frozen
file, new axiom, admission, unsafe body or resource-limit override was added.
There are no background jobs or commits.

### Resume at M5c

The requested stopping point is now M5b, not full M5. `Frames::walk` still
implements the constructor's prefix walk as plain code, and `Frames::open`,
`try_new`, `try_new_with_policy`, `next_frame` and the disposal/save/frame adapter
remain without the global Frames refinement invariant. Next prove the prefix
walk against `model::mp_fuel`/`mp`, stopping *before* the first descriptor; do
not use `parse_mb` there, because MB consumes G8/G9 and the constructor must not.
Reuse MB's extension-progress and fuel proof, and the now-swapped verified
extension entry. Then connect `header_model` to `hdr_matches`/`MH`, use the
proved `reserve` instead of the plain open body's duplicated reservations, and
thread a real `Tracked<AllocLog>` through Frames and Saved allocation. The
legacy plain open/Saved paths still contain their old `Tracked::assume_new`
placeholders; this session did not turn them into proved allocation claims.
A-GIF-2 clause 1, T-ITER/T-PLAYS/T9, complete disposal/save integration,
non-exposure/fusion invariants and T12/T13 remain the later work. No concrete
step counter or full work sum has yet been introduced.

## Constructor/log continuation (Sol, runs 477-490)

Final run 490 is green: **551 verified, 0 errors**. This supersedes the
preceding M5b handoff at 529. The actual public constructors, reservation path,
prefix walk, Info accessor and Saved allocation helper are now verified.
The full semantic iterator invariant is not yet proved; keep that distinction
when describing this checkpoint.

`parse::parse_mp` proves `model::mp_fuel` for the constructor walk, including
extension effects, exact Trailer/EOF checks and errors. It stops on the first
image introducer without reading the descriptor or image data. Its model fuel
counts extensions and its executable variant is the remaining input bytes.
`WalkState::new` now exposes its initial fields, and
`lemma_header_model_matches` connects the exec record conversion to MH.

`gif::prepare` composes actual `parse_header`, the existing proved `reserve`
and the new prefix walk. It has no valid-input precondition: header errors,
prefix errors, success and allocation failure all have contracts. `reserve`
now additionally states the exact zeroed buffers, the sealed OutOfMemory/log
binding and the site bounds. `prepare` and the logged constructor prove full
`Req_new` equality on every non-OutOfMemory result, including format errors
after reservation, and the failed-request prefix on OutOfMemory.

`Frames::open`, `try_new`, `try_new_with_policy`, the test-record constructor
and the new logged variants now call that proved preparation path. The old
plain duplicated reservations and constructor walk were removed. `Frames`,
Info and the state types are inside `verus!`; Frames owns a real
`Tracked<AllocLog>`. The logged variants return
`(Result<Frames, TryError>, Tracked<Option<AllocLog>>)`: on success the second
component is None and the decoder owns the log; on error it is Some(log).
`construction_log` gives one observation for both alternatives. The ordinary
public APIs retain their old Rust signatures.

`constructor_result` proves the exact MP error on a format result, and on
success `constructor_ok` establishes every Info field (including loop_count),
the exact model walk state, zero canvas/table, empty Saved/Prev, the correct
Running/Done state, storage bounds and the successful two-entry request trace.
An OutOfMemory result requires MH success and its sealed trace identifies the
failed site and size. `Profile::policy` now exposes its exact profile-record
relation. The actual Info accessor also has a proved value contract.

`storage_wf` is the current global **storage/parser/log** predicate: input and
header bounds, vector lengths, parser counter caps, fixed MP Info/loop_count,
and the two-entry or single-Saved-entry trace. It is established by every
successful constructor. It is deliberately not called the full frozen
`wf_p(dec, n)`: it does not yet relate the current canvas/disposal/saved contents
or frame history to MF.

`Frames::ensure_saved` is the actual Saved allocation path. It borrows the
owned tracked log, allocates only when Saved is absent, proves the request's
site and checked size, appends exactly one event, and preserves `storage_wf`.
On failure it sets Fused and returns the matching sealed OutOfMemory. An
already-present buffer performs no allocation and preserves its bytes/log.
All `Tracked::assume_new` placeholders have been removed from gif.rs. The
plain image caller still needs its precondition/preservation proof; therefore
do not claim A-GIF-3's universal finite-call-sequence theorem from the helper
contract alone.

The storage proof needed exact literal log-shape assertions and an opaque MP
observation. A later edit tipped Saved verification over the default resource
limit; `spinoff_prover` plus hiding the unchanged hdr_ok relation resolved it,
without increasing a limit or weakening a contract.

The small T-PLAYS item is also complete: the actual public `plays` and
`Plays::total_plays` functions equal their models, the u32 addition is checked,
and `lemma_total_plays_injective` proves injectivity over all Option<u16>
inputs (65,537 inputs by quantification, not enumeration).

Release tests pass in `gif/runs/sol-20261116/tests-m5c-final.log`: all 373
vectors, five report tests, four allocation-fault tests, the exhaustive runtime
plays check and the 37,239-frame stress test. All 14 frozen hashes match.
No frozen statement, guard body, axiom, admission, unsafe body or solver-limit
override was changed; no commits or background jobs remain.

### Remaining invariant/iterator boundary

The constructor's exact MP and allocation clauses are established on the
actual entry points. A full `Eq_new_p`/frozen `wf_p` claim still needs the bridge
from `constructor_ok` to the semantic MF invariant and its preservation.
`Frames::step`, `Frames::image`, `next_frame` and the still wrappers remain
plain functions. Next define the complete frame-history/canvas/disposal/Saved
relation, strengthen the bounds predicate as needed for Prev rectangles, and
prove the frame adapter and iterator preserve it. `parse_mb`, `decode_drawer`,
the disposal/copy helpers and the real `ensure_saved` contract are available.
Use the existing post-first-frame loop-count lemma to retain the constructor's
Info while MB traverses later extensions. Revisit the outer canvas-borrow
rejoin noted in the preceding section when lifting the owned Frames canvas.
Only then claim T-ITER, complete A-GIF-2/A-GIF-3/A-GIF-4 over finite call
sequences, T9 and integrated T13. T12 still needs actual step counters and the
full per-call/summed inequalities. T-PLAYS does not depend on that remaining
work and is already proved.
