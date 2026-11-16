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

## Semantic-state foundation and borrow rejoin (Sol, runs 491-516)

Final run 516 verifies **565 functions, 0 errors**. The final release suite
passes in `gif/runs/sol-20261116/tests-semantic-foundation-final.log`: five
report tests, seven conformance functions (373 vectors plus the other checks),
four allocation-fault tests and the 37,239-frame stress test. All 14 frozen
hashes match. No commits, background jobs, new assumptions or solver-limit
increases were made in this continuation.

### Independent model and constructor initialization

New `src/frame_model.rs` composes the existing independent MB, ML and drawing
models. It defines complete frame records, previous disposal/rectangle,
canvas/Saved state, status, per-call outcomes and the first Saved request,
including requests for frames that later fail LZW or colour validation.
`image_mode` handles the still path's omission of Saved; the iterator model
uses the normal mode. Colour precedence stays outside pure ML. `after_calls`
retains the terminal call and later fused calls; `stream_fuel`/`mf` give the
frame sequence and terminal result. OutsideDomain is a separate internal
marker, not another GIF error.

The model is independent of executable frame control flow. Its report bridge,
`lemma_image_observation`, is proved from exact ML/colour observations and the
successful forward drawing equation. The model still needs the reachable
non-OutsideDomain/fuel adequacy proof and the bridge from successive calls to
MF's complete frame sequence before claiming the frozen T-ITER statement.

`gif::wf_p` is now defined, with live parser/canvas/disposal/Saved equality to
`after_calls`, an OOM cut at the first Saved request, and terminal abstraction
of private bytes and cursor that no later call exposes. `prev_bounds` adds
positive rectangle dimensions, coordinate/sum bounds, normalized disposal and
the Saved-presence requirement for disposal 3. It is part of `storage_wf`.
Actual non-still constructors now establish `wf_p(dec,-1)` through their
`constructor_result` contracts. This is a proved base case, **not yet a
preserved semantic invariant**.

### The canvas-borrow boundary is resolved

The earlier failed outer-canvas wrapper was missing two borrow equations.
`DrawerSink::new` now proves
`final(canvas)@ == final(s.canvas)@`. That alone was insufficient: generic
Sink calls could replace the nested mutable reference while preserving the
current state observations. The trait now has a prophetic `sink_future`
observation, with an empty default and Drawer's implementation equal to
`final(self.canvas)@`. Every put and the emit/run/decode helpers preserve it.
This is a proved future-reference contract, not a new axiom or an admission.

`compose_new::decode_canvas` consequently proves length preservation,
ML/colour observations, successful whole-canvas equality and pixel-origin
preservation on the caller's original mutable slice. The actual frame path
uses this wrapper. Do not return to the old wrapper without the borrow-endpoint
condition: run 496 still failed despite the constructor equation alone;
run 498 passed after generic endpoint preservation.

### Actual frame code lifted so far

`Frames::apply_previous` is verified against the exact disposal effect.
`prepare_canvas` composes it with the actual Saved helper and row copy,
proving exact disposed canvas, saved pixels, request trace, storage preservation
and Saved OOM/fusion. Both are used by the actual image path, not ghost-only
stand-ins. The combined query initially exceeded the resource limit; splitting
previous disposal from current saving resolved it at default limits.

`decode_image_pixels` is verified against independent model parameters and
frame/table views; it carries exact ML/colour observations and successful
whole-canvas equality through the original slice borrow. The actual `image`
body is now inside `verus!` and proves safety, storage preservation, fixed
input/header/policy/Info/parser state, Running on success, Fused on error, and
the exact optional Saved-log append. A format error now fuses inside `image`
itself; `next_frame`'s existing fuse assignment is idempotent. Runtime results
are unchanged. The palette slice uses the bounds MB validated rather than an
unreachable empty-table fallback.

`parse_extension`, `parse_data_step`, `parse_image_step` and `parse_mb` now
expose loop-count stability. MB preserves it after the first frame, or when
its initial cursor is not an extension introducer. The loop invariant retains
the unchanged initial cursor in the first-frame/non-extension case.

### Current blocker and exact remaining scope

The stronger `image_result` predicate expresses the semantic result relation,
but **it is not an ensured postcondition of the final image function**.
Runs 504-515 attempted that lift, with the later attempts exceeding the
default resource limit even after isolating pixels and the model-report lemma.
The default-limit storage proof became green after keeping `storage_wf`
opaque and using `lemma_storage_bounds`/`lemma_storage_reframe` instead of
expanding the complete invariant in the executable query. The attempted
semantic version is preserved at
`gif/runs/sol-20261116/gif-image-semantic-attempt.rs`; it predates the final
storage-framing isolation, so use it as reference, not as a replacement file.

Next split the semantic image introduction into a separate proof lemma using
the prepared-state snapshot, `lemma_image_observation` and the concrete return
metadata; prove `image_result` on the actual body. Then lift `step`,
`next_frame` and still wrappers. Those three remain plain functions here.
Complete WF preservation and the MF fuel/history bridge before claiming
T-ITER, T9 or A-GIF-2/3/4 over finite call sequences. T12 accounting and
integrated T13 remain open. No completion of these rows is claimed by the
new model definitions or the constructor base case alone.

One verifier issue occurred in run 511: hiding `frame_model::image_mode` after
removing its last reference from the function caused an undeclared-fuel-variable
AIR panic. Removing the unused hide avoided it. Runs 514 and 516 are green.

## Actual semantic image contract (Sol, runs 517-523)

Run 523 verifies **569 functions, 0 errors**. The preceding section's image
semantic-postcondition blocker is resolved: the actual `Frames::image` now
ensures `image_result`, including exact non-OOM result/metadata/canvas relation
and Saved OOM observation. This is not yet step/next_frame or T-ITER.

The proof uses `lemma_image_finish` and `lemma_image_oom` as separate
introduction queries. `pixels_observation` packages the existing exact report,
colour and canvas observations; `prepared_matches` packages the pre-draw
canvas, Saved and trace. Keeping both observations and storage_wf opaque in
the executable query avoids expanding their quantified bodies there.
`frame_metadata` constructs the actual returned metadata with its own proof.

The allocating `image` wrapper now calls the verified `image_prepared`
helper after successful preparation. The latter receives a ghost snapshot of
the pre-disposal state, performs the actual pixel decode, updates Prev, and
introduces the semantic result on success or format failure. This separation
was needed in addition to the proof-lemma split: runs 518-520 still exhausted
the monolithic image query. With separate executable queries, run 522 exposed
two missing preconditions instead of exhausting the solver: the prepared state
retains the original Prev record, and disposal 3 on the non-still path has a
Saved buffer. The real wrapper proves both facts from preparation.

Adding functions changed solver behavior in `lzw::lemma_resolved_agrees`:
runs 520/522 failed to instantiate the raw/model agreement quantifier. A second
explicit quantified assertion, oriented and triggered on raw_pf_seq, supplies
the exact premise of refinement::table_agreement. No algorithm, contract,
assumption or solver limit changed. The preceding LZW source is backed up at
`gif/runs/sol-20261116/lzw-before-agreement-trigger.rs`.

Run 521 again hit the unused-hide AIR bug, this time because pixels_observation
was no longer referenced by the allocating wrapper after splitting it.
Removing that wrapper's unused hide resolved the verifier panic.

Final release tests pass in
`gif/runs/sol-20261116/tests-image-result-final.log`: five report tests, seven
conformance functions covering the 373 vectors and other checks, four
allocation-fault tests and the 37,239-frame stress test. All 14 frozen hashes
match. No commits or background jobs were started.

The next obligation is the actual `Frames::step` contract. It must establish
image_ready/storage_wf after parse_mb and lift the now-proved image_result from
the post-MB physical state to a frame-model call from the pre-MB state. Preserve
the first-frame cursor-at-introducer case and the post-first-frame L-LOOP
facts when proving fixed Info. Then lift next_frame and the still wrappers,
prove model OutsideDomain/fuel adequacy and the successive-call/MF sequence
bridge, and complete WF preservation/T-ITER. All of those remain open here;
the old source snapshot marked image-semantic-attempt is now superseded by the
proved current image body. T9, finite-call-sequence allocation theorems, T12
accounting and integrated T13 still depend on the remaining iterator work.

## Verified scan, step and next-frame adapter (Sol, runs 524-531)

Run 531 verifies **578 functions, 0 errors**. Actual `scan_next`, `step` and
public `next_frame` are now inside `verus!` and verified. The next_frame
contract is a **one-call semantic/storage observation**, not yet the frozen
WF/MF induction theorem T-ITER. The distinction below is intentional.

`storage_wf` now includes the first-frame cursor fact: when Running with zero
frames_seen, the cursor points at the first image introducer. The constructors
establish it. The data/image/MB parser contracts now expose the successful
image's exact frames_seen increment; MB's existing loop-count stability
therefore preserves fixed Info both at the first descriptor and after later
extensions. The stronger invariant is preserved by all verified frame helpers.

`scan_next` calls actual parse_mb and proves its complete postcondition,
post-MB storage bounds, fixed input/header/policy/Info, unchanged canvas/table/
Saved/Prev/log, and image_ready on an image result. It sets Done on Trailer
and Fused on a parse error. `lemma_storage_walk` isolates the bounds/Info
preservation proof; `lemma_mb_image_ready` isolates the descriptor/data bridge.

`step` calls the real scan_next and image paths. `step_result` relates the
result to an independent model call from the **pre-MB** state: exact error,
metadata and whole canvas on success; no partial output; exact optional Saved
trace append; matching parser position and site/size on Saved OOM. The proof
uses separate terminal and image introduction lemmas. A new pure image-state
extensionality lemma handles the changed parser state between the pre-MB and
post-MB snapshots. `lemma_image_reservation` exports the model's reserve_saved
flag without expanding its entire image fold in the adapter query.

The frame model now has call_mode for the still/stream distinction; its existing
call wrapper selects stream mode, so MF's behavior is unchanged. The still
wrappers are not yet verified, but the proved step relation supports their
no-Saved mode.

`FrameView` is inside verus. `next_result` proves that a successful view borrows
the decoder's actual single canvas and carries the model's complete metadata
and canvas; active calls transition to Running/Done/Fused as appropriate. A
call on Done or Fused returns None and preserves the entire decoder state,
including the real allocation log. `fixed_stream` exposes fixed Info and
input/header/policy without mentioning private fields in public contracts.
The old redundant Done/Fused assignments in next_frame were removed because
scan/image already perform them. Runtime outputs remain unchanged.

Proof engineering: the stronger cursor invariant made apply_previous exceed
the default limit in run 524; keeping storage_wf opaque and using the existing
framing lemma fixed it. Runs 526-527 exhausted the combined parse/semantic-step
query; separating scan_next from step made both pass at default limits. The
intermediate storage-walk lemma needed Running as a precondition, since a
failed Saved log must not be moved from Fused back to another state.

Final release tests pass in `gif/runs/sol-20261116/tests-next-frame-final.log`:
five report tests, seven conformance functions covering all 373 vectors and
other checks, four allocation-fault tests, and the 37,239-frame stress test.
All 14 frozen hashes match; no new assumption, admission, axiom, unsafe body,
solver-limit increase, commit or background job was introduced.

### Exact remaining theorem boundary

Actual next_frame requires storage_wf and ensures storage_wf, fixed_stream and
next_result. Its precondition is weaker than semantic wf_p, so constructors
and the method's own postcondition compose for storage/safety across repeated
calls. Its postcondition does **not** yet ensure wf_p(d,n+1), compare the nth
result with the complete MF frame sequence, or identify SavedAt across the
whole trace. Do not call this the full frozen T-ITER theorem.

Next lift the still wrappers and define their MS observation using stream-free
call_mode; then prove model image/call OutsideDomain unreachability, fuel
adequacy and the successive-call/MF sequence bridge. Prove semantic WF
preservation, including the first Saved request and OOM cut, and use the actual
next_frame contract for T-ITER. Global A-GIF-2/3/4 timing/equality claims, T9,
T12 work accounting and integrated T13 still need those remaining proofs.
The storage trace already limits reachable decoders to the constructor trace
plus at most one Saved event, but the complete frozen A-GIF-3/A-GIF-4 statements
also require MF's SavedAt timing and are not claimed complete here.

## MS, total MF and full T-ITER (Sol, runs 532-546)

Run 546 verifies **611 functions, 0 errors**. This continuation supersedes the
preceding section's remaining one-call/WF/MF limitations: the actual public
next_frame now carries the MF-indexed iterator observation and semantic WF
preservation. The still wrappers are verified against MS as well.

### Actual still entry points

`frame_model::ms` independently composes MP with the first call in still mode,
returning the complete Still bytes/dimensions/recovered observation or the
exact first format error. None is its internal OutsideDomain marker.
`lemma_ms_defined` proves that marker unreachable for every input and record.
`Still` is now inside verus. Actual still_with, try_decode_still,
try_decode_still_with_policy and the source test-record wrapper have MS
contracts: success matches every byte, width/height and the returned recovered
flag; a non-OOM error is exactly MS's error; OOM implies MH success. The core
wrapper discards recovered as its API requires. Constructor/error/empty-stream
and first-call bridges are separate proofs. No logged still variants or full
A-GIF-1 G/Prog package have been added yet.

### Total models and full frame sequence

The pure model now proves descriptor/table domain bounds and strict cursor
progress through a complete MB image, by induction over extension fuel and the
sub-block scan. A successful image's model drawing fold is defined whenever
all offered indices pass transparency/table validation; the complementary
case already returns the exact colour error. Thus image_mode cannot return
OutsideDomain on a validated frame, including the frame at a Saved OOM site.

`lemma_initial_domain` proves MP's cursor bounds; `lemma_call_progress` proves
model calls defined and successful frames strictly advance the cursor.
`lemma_stream_fuel_adequate` shows fuel greater than the remaining input bytes
cannot run out, so MF's `|b|+1` fuel suffices. `lemma_mf_defined` excludes
OutsideDomain after every successful MP. `lemma_stream_calls` and
`lemma_mf_position` connect every zero-based call index to MF's full frame
sequence: its frame, its single terminal error/Trailer, and error-free None
on every later terminal call.

### Semantic WF and Saved timing

`lemma_call_saved_presence`, `lemma_no_saved_before` and
`lemma_first_saved_at` prove that a reservation from a state without Saved is
the first reservation in the model call history. A reservation makes model
Saved present, and later calls cannot lose it. `lemma_step_call_defined` links
the actual OOM branch's validated MB image to a defined model call.

`lemma_step_wf` proves wf_p(d,n+1), including the first Saved reservation and
OOM cut. `lemma_terminal_wf` preserves it on terminal calls. Actual step and
next_frame now ensure, for every n, `wf_p(old,n) ==> wf_p(final,n+1)`.
The constructor already established wf_p(dec,-1), so these are connected
invariant proofs on actual calls, not merely model lemmas.

### Public T-ITER contract

`iter_result` states the complete MF-indexed observation. For each call index:
frame bytes and all metadata match MF; the terminal result matches MF.end;
later calls return None; Saved OOM may displace the result only at the model's
first Saved reservation, with exact Saved site and `4*W*H` size.
`iterator_observation` additionally returns None after the real trace records
an earlier OOM. `lemma_next_iter` and `lemma_next_observation` introduce these
facts from the concrete next_result and WF relation. Actual public next_frame
ensures iterator_observation for every n satisfying its incoming WF, as well
as preserving WF. It still requires only the weaker storage_wf for safety.

Actual info now also proves its complete record equals MF.info in every
storage-well-formed state, via info_result. All info reads are immutable;
next_result already proves terminal calls preserve the entire decoder and
real log. These contracts compose through arbitrary finite info/next_frame
interleavings. No equality of an error's private canvas was introduced.

### Validation and remaining work

All default limits remain unchanged. Run 532 repeated the known unused-hide
AIR panic for a constructor-only lemma that no longer referenced call_mode;
removing that hide fixed it. Run 540 needed an explicit MB-to-mb_fuel wrapper
identity in the pure progress proof. Run 543 needed a proof-block syntax fix.
All later queries are green; no new axiom, admission, unsafe body or assumption
was added.

Final release tests pass in `gif/runs/sol-20261116/tests-titer-final.log`: five
report tests, seven conformance functions covering all 373 vectors and other
checks, four allocation-fault tests and the 37,239-frame stress test. All 14
frozen hashes match. No commits or background jobs were started.

T-ITER, T-PLAYS and the actual MS observation are now proved. Still remaining:
logged still entry points and A-GIF-1 G/Prog; packaging the full T9 and
A-GIF-2/3/4 finite-sequence allocation/log equalities and Prog observations;
T12's actual ghost counters and complete work inequalities; integrated T13
pixel/resource theorems; the manifest's independent vector/mutation gates.
The current real trace/storage invariant and first-Saved model lemmas provide
the allocation proof inputs, but no new full A-GIF-1/3/4 theorem package is
claimed by this checkpoint.

## Allocation theorem packages (Sol, runs 547-554)

Run 554 verifies **628 functions, 0 errors**. The preceding section's
A-GIF-1/2/3/4 packaging gap is closed on the actual public APIs. T12 has not
been instrumented; this is a context checkpoint before the loop-accounting
pass, not a work-bound completion claim.

### A-GIF-1 and the actual still log

New public `try_decode_still_logged` and
`try_decode_still_with_policy_logged` return the actual owned
`Tracked<AllocLog>` on every outcome. They are re-exported at the crate root.
The ordinary still path calls the same private still_with_logged body and
merely discards the ghost trace observation. On constructor failure that body
extracts the real error log from the constructor's tracked Option; after a
successful constructor it moves the real log out of Frames. No fabricated log
or placeholder permission is introduced.

`still_g`/`still_core_g` combine exact MS observation, sealed log_tied failure,
request-prefix/full-equality clauses, per-site bounds and at most two requests,
plus all-log-entries-Ok implies no OutOfMemory. `site_bounded` proves positive
requests, Canvas/Saved at most MAX_OUTPUT and LzwTable exactly its frozen size,
all within the guard bound. `lemma_still_trace` proves the actual still step
adds no request and cannot return Saved OOM; still-mode model calls never
reserve Saved. Both public logged still entry points ensure their full G/Prog
package. Runtime still calls continue to share this proved path.

### A-GIF-2, finite-sequence A-GIF-3/4 and T9

Both public logged constructors now ensure `constructor_g`: exact MP/initial
WF observation, complete MF Info, initial iterator-log observation, sealed
failure, exact request sequence or failed prefix, site bounds/count and Prog.
The previous owner alternatives remain explicit (Frames owns the log on
success, the separate tracked Option owns it on error).

The converse Saved-history lemma is now proved: Saved present after k model
calls implies an earlier first Saved reservation. Saved timing is unique, and
`lemma_saved_current` identifies the current model call's reserve flag exactly
with SavedAt at that call index.

`iterator_log(d,n)` proves the real cumulative request trace equals Req_new
plus the unique Saved request at an index at most n, or the exact failed Saved
trace with two earlier successful reservations and Fused state. It proves site
bounds and at most three requests. `next_allocation` proves exact per-call
Req_next append, SavedAt-only OOM, exact false-event/object site and byte count,
log stability after a failure, and backward all-success progress. In
particular, all entries Ok after a call implies all entries were Ok before it,
that call was not OOM, and its MF observation is exact.

Actual public next_frame now ensures next_allocation for every incoming WF
index, alongside its existing WF/T-ITER postconditions. Constructor_g supplies
the initial trace; immutable info leaves it unchanged. These actual
initialization/transition contracts prove the finite-call-sequence packages
by induction, without a disconnected synthetic execution log. This supplies
T9's functional/allocation integration together with the logged still G/Prog
and the previously completed T-ITER contracts. No separate runtime simulator
or callback collection was added.

Run 550 needed explicit Saved-history absence and extensional Req equality;
run 552 needed an explicit failed-last-entry contradiction and elementwise
Req projection/progress proof. All were resolved at default limits.

Final release tests pass in
`gif/runs/sol-20261116/tests-allocation-packages-final.log`: five report tests,
seven conformance functions covering all 373 vectors and other checks, four
allocation-fault tests and the 37,239-frame stress test. All 14 frozen hashes
match. No commits or background jobs were started; no new assumptions,
admissions, unsafe bodies, axioms or solver-limit changes were introduced.

### T12 resume point

No concrete ghost step counter has yet been added, and no T12 bound is
claimed. Instrument actual loop bodies and the successful zero-fill calls;
do not replace their cost with a spec-only input formula. The hash-pinned guard
must remain unchanged: its callers can charge n filled bytes on an Ok result.

Concrete production loop sites to start from are the reader-fill loop in
lzw.rs, emit_raw's chain loop and decode's outer loop; parse.rs's sub-block,
NETSCAPE loop scan/identifier and MP/MB loops; and compose_new.rs's clear/copy
row/column loops. Its forward draw loop is a separate helper, not the actual
reverse-emission path. interlace.rs has no executable loop in the current
source. Propagate measured helper costs through the existing split adapters.

The design memo's dominant `5*|D|+2*A` sum is the target, not existing
instrumentation. Distinguish successful code reads (at most `4*|D|`) from
outer decoder bodies: an empty payload or a StopAtRaster halt can execute one
body without reading a code. The spare unit in the `2*A-1` chain bound can
pay that final body. Count the reader and chain loops as they actually run.

Do not use WalkState.total_pixels blindly as P: G9 updates it before Saved
reservation, whereas a Saved OOM frame does not reach G10 and has A_n=0 in
the frozen work bound. Disposal before that failure belongs to A_(n-1).
Fused calls run no loop and must leave the work counter unchanged. Remaining
after T12 are integrated T13 and the manifest's independent model/vector,
mutation and policy-proof gates not already completed elsewhere.

## Measured operational T12 packages (Sol, runs 555-575)

Run 575 verifies **650 functions, 0 errors**. All requested production loop
sites now have measured counters, and the actual public constructor/iterator
paths carry quantitative work observations. **Do not mark frozen T12 complete
without the model-cursor/area identification described below.** T13 and the
independent witness/mutation campaigns were not attempted.

`lzw.rs` counts reader-fill bodies, raw-chain bodies and outer decoder bodies.
`LzwReport.work` carries their measured sum, bounded by `5*data_len + 2*area`
through decode_image_data, decode_drawer, decode_canvas and decode_image_pixels.
The outer invariant charges completed code bodies to two consumed bits and
completed chain bodies to q; the final body's spare unit fits the positive
raster's `2*A` charge, including empty data and StopAtRaster. The helper domain
now explicitly requires area >= 1, exactly the frozen T-LZW domain; the real
image adapter proves it from the checked descriptor.

`parse.rs` counts sub-block scans, the NETSCAPE identifier comparison and all
NETSCAPE scan passes, plus MP/MB bodies. Its contracts propagate measured costs
through extension/data/image helpers. MP and MB cost at most four times their
cursor advance plus one final body, with |b| as the error endpoint. `clear_rect`
and `copy_rect` return erased measured costs; their row and column counters
prove at most `2*w*h`, including clipping and empty clipped rectangles.

`Frames.work` accumulates constructor fills, prefix parsing, MB, disposal,
Saved fills/copies and LZW. Callers charge successful guard fills by the actual
returned Vec length; failed fills add nothing and the pinned guard is unchanged.
`Frames.charged_pixels` increases only on the post-preparation render path,
including format failure, and not on Saved OOM. Previous disposal is charged to
prior_area, never to the Saved-OOM frame's area. All split adapters export their
measured costs. The new concrete render_pixels split keeps the semantic image
query below the default resource limit; no solver limit was raised.

`constructor_work` gives the 32*(constructor cursor + N)+41984 bound and
initializes work_accounted. The latter uses a potential of measured work,
two units per pending previous-frame pixel plus one terminal-body credit, and
a prepaid canvas-length credit while Saved is absent. Actual next_frame proves
next_work: monotonic work/pixel counters, exact stability on Done/Fused calls,
work_accounted preservation, and both 32*(|b|+2*N+2*charged_pixels)+41984 and
64*(|b|+N+charged_pixels)+41984 cumulative bounds. `t12_measured_call` supplies
the 32-factor per-call bound with the independently proved SavedAt indicator.
`lemma_finite_work_history` lifts these actual transition contracts over finite
snapshot histories, including immutable info steps. Actual still_with_logged
asserts the cumulative 64-factor bound before discarding the decoder; every
public still wrapper uses that same measured path. Constructor-error paths
assert their measured bound in open_logged before returning the error log.

### Exact remaining frozen boundary

The numeric packages use effective_cursor (the real parser cursor, or |b| on
Fused), the measured G10-area counter and the actual pending Prev area. They do
not yet prove aliases between those quantities and the frozen model-indexed
cur_n/A_n/A_(n-1)/P names. In particular, frame_model::call_mode intentionally
retains its incoming State.walk on Trailer, even after MB scanned extensions;
semantic_state_matches ignores a terminal decoder's private parser cursor.
Do not substitute after_calls(...).walk.p for the work cursor. A separate
independent structural-work cursor observer, or terminal-cursor enrichment of
the production model with preservation proofs, is needed. Add the explicit
MB-image-to-charged-area transition and finite area-sum bridge, keeping the
Saved-OOM current area zero. work_ready is an additional reachable invariant
established by constructor_work and preserved by next_work; it has not been
folded into semantic wf_p. These are the resume points for a fully literal
frozen T12 package. No frozen-statement counterexample or erratum is claimed.

The final release run is `gif/runs/sol-20261116/tests-t12-final.log`: five LZW
report tests, seven conformance functions covering all 373 vectors, four
allocation-fault tests and the 37,239-frame stress test all pass. All 14 frozen
hashes match; both guard hashes match their pinned values. Final run 575 took
14 seconds and peaked at 2,466 MiB summed RSS. Verus is
0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251, rustc 1.99.1
(58cf8f9c3 2026-10-13). No assumption, admission, axiom, unsafe body, solver-limit
increase, commit or background job was introduced. Pre-edit and intermediate
source backups are under `gif/runs/sol-20261116/t12-*.rs`.

### Independent witness and mutation handoff (list only)

No integrated src/model_vectors.rs or implementation-side mutation runner
exists. The manifest still requires independent expected-value proofs for all
362 inputs at most 4096 bytes, the M2/M3/M4/M5/P1/P2 named gates and symbolic
instantiations L1-L5 for the other 11 records. Register a new Rust proof module
in gif_verus/src/lib.rs, using spec-frozen/VECTORS.json's authored expected
values, not expectations computed from the implementation. Existing standalone
lzw_proof/src/{witnesses,regressions,policy_samples}.rs and its tools are inputs
to that work, not completed integrated public-entry witnesses. Standalone M2
already has its two >=8 MiB stress streams; their integrated public-path gate
still needs an adapter. The existing gif_verus/tools/gen_mutants.py mutates
runtime input records; it does not run the manifest's source-proof mutations.

All 26 implementation-side mutations of THEOREMS.md remain without integrated
verifier diagnostic artifacts. The alloc-delta mutations A1, A2, A2-prime, A4,
A5, A8, A9 and A17-A19 also need their specified real-core gates at Canvas,
LzwTable and Saved. Use disposable source copies, preserve the seeded target,
and run one verification at a time through gif/verify.sh with VERIFY_CRATE set
to the copy. Mutation 24's inserted loops must include their defined work
charge; an uncharged new loop bypasses the instrumentation rather than testing
the numeric theorem. The source lint script tools/audit_alloc.sh still expects
five reservation expressions (2,2,1); production has the intended three (1,1,1).
Its stale count gate needs repair before treating it as an allocation-mutation
runner, and the production call-graph audit has no GIF configuration yet.

## Frozen T12, integrated T13 and initial independent witnesses (Sol, runs 576-606)

Run 606 verifies **700 functions, 0 errors**. The preceding section's frozen
T12 identification gap is closed. T13's model and actual public pixel/resource
packages are also proved. The integrated independent-vector gate has six
model/real-public-call pairs, not the complete manifest gate. No mutation
campaign was run.

### Frozen work quantities on the actual APIs

New src/work_model.rs defines independent mb_cur/mb_area and the frozen-named
cur_n, A_n and P. mb_cur uses MP's structural prefix fold to retain the endpoint
of a Trailer reached after extensions; it does not reuse the terminal frame
model's retained State.walk. Actual parse_mb proves this cursor observation
and its MB descriptor area. scan_next and step propagate both observations to
public next_frame. P is a finite sum of A_n. A Saved-OOM cut makes its current
A_n and all later areas zero, while preserving every earlier area.

work_wf relates the measured counters to cur_n and P and identifies pending
Prev with the preceding A_n on Running states. constructor_work establishes
work_wf(-1,None); actual next_frame preserves it with the actual Saved-OOM cut.
Its T12_next postcondition states the frozen 32-factor per-call inequality
using cur_n, A_n, A_(n-1) and SavedAt, and both cumulative bounds using P.
theorem_T12_finite proves the finite-history result with an explicit telescoped
constructor-plus-call sum. Immutable info calls add no summand or state change.
Constructor-error branches now assert 32*(cur_(-1)+N)+41984, not merely the
weaker |b|-based constructor-error bound. The actual still path identifies its
measured area with the first-call P and asserts 64*(|b|+N+P)+41984. Thus the
numeric work statements include format errors and OOM, with the fixed allowance
charged only through construction. Fused calls preserve the measured counter.
Run 585 was the first green checkpoint for these complete work packages.

### T13 model and public packages

New src/pixel_model.rs proves LZW output-length bounds, clipped clear/copy and
generalized draw-fold pixel preservation. Its active_tables is a finite prefix
history of tables actually selected by model MB image steps, not arbitrary
input subranges. Induction over after_calls preserves both canvas and Saved
pixel invariants; the model MS theorem covers every model success, dimensions,
length/cap and opaque-table-entry versus transparent-black pixels. This also
covers disposal 3 restoring previously valid pixels.

Actual public ordinary/logged still entry points now ensure T13_core or
T13_still: the exact existing MS/whole-canvas observation, unconditional success
soundness, reverse success and error equivalences under the non-OOM antecedent,
and dimensions/length/cap/pixel clauses. Actual next_frame ensures T13_view for
every incoming semantic WF index: the returned bytes are the actual single
canvas and satisfy the model-history pixel invariant. T13_resources additionally
bounds the owned Canvas/LzwTable/optional-Saved lengths by 2*(4*N)+20480 bytes.
Allocator overhead, physical ownership/borrow enforcement and error-canvas
non-exposure retain the manifest's trust/API boundaries; no rollback or
error-canvas equality was added. Run 590 was the first green integrated T13
checkpoint. These packages concern the implemented Reject-index policies;
unsupported P2 substitution semantics are not newly claimed.

### Independent witness checkpoint and resume point

src/model_vectors.rs is registered in src/lib.rs. It proves the authored frozen
MS expectation and makes a real try_decode_still call for each of six records:
empty, header-12, bad-signature, version-88a, header-only-gct-trailer/still and
two-colour-m2. Executable success/error observations retain the necessary
non-OOM antecedent. The positive gate proves every RGBA byte, using staged MH,
MP, MB, individual LZW codes and drawing computations. Its LZW observation is
idx [0,1,0], no error, pos 16, E4 false; width becomes four before the EOI read.

Input and expected literals originate only in spec-frozen/VECTORS.json's
input_hex/expected fields. No oracle or executable supplied expected outputs.
Whole-positive-MS by-compute attempts exceeded the default compute recursion
depth. The checked-in gate instead uses closed concrete compute terms and
per-code fold equations; it raises no limit. An isolated nonlinear arithmetic
query establishes the 12-byte initial canvas. The two added observation lemmas
in gif.rs and policy.rs expose existing closed result/profile facts without
changing either statement or runtime behavior.

The next vector work is the other 356 small records, the 11 large-record
symbolic instantiations L1-L5, all remaining named M2/M3/M4/M5/P1/P2 gates,
MF/public iterator witnesses and the integrated >=8 MiB stress adapter. The six
still witnesses do not replace direct M2 complete-report witnesses or any
mutation kill. basic-4x3-gct and basic-87a were generated in an exploratory
backup but are NOT registered proved witnesses. The original emitter writes
only a new file and must not replace the staged checked-in proof. Useful
exploratory backups are runs/sol-20261116/model-vectors-*.rs; the current
src/model_vectors.rs is the authoritative green resume source.

All 26 implementation-side mutations and the previously listed alloc-delta
mutations still need their specified diagnostic artifacts. The prior section's
mutation-file mapping, seeded-target precautions and stale allocation-lint
count warning remain applicable.

Final release validation is runs/sol-20261116/tests-t12-t13-final.log: five
report tests, seven conformance functions covering 373 vectors, four allocation-
fault tests and the 37,239-frame stress test all pass. All 14 frozen hashes and
both pinned guard hashes match. Run 606 took 22 seconds and peaked at 2,684 MiB
summed RSS. Versions remain Verus 0.2026.11.02.8bf68ee, vstd
0.0.0-2026-11-01-0251 and rustc 1.99.1 (58cf8f9c3 2026-10-13). No assumptions,
admissions, external bodies, unsafe bodies, axioms, solver/compute limit raises,
protected-file edits, commits or background jobs were introduced.

## Symbolic-cap repair checkpoint and widening blocker (Sol, runs 607-612)

The working crate is green at run 612: **697 verified, 0 errors**, 21 seconds,
2,734 MiB peak summed RSS. The count fell from 700 because the repair removed
separate arithmetic tactic queries; it did not remove a decoder theorem.
Task 1 is PARTIAL: the requested halved-input control passes, but the full
frozen permitted-cap-change clause still has dependencies beyond copied
numerals. Task 2 was not started; this handoff stops at the Task 1 blocker.

### Proved and checked

src/limits.rs now contains a no-precondition caps_ok lemma that checks G1's
LE16 dimension bound, input-plus-one-LE16-domain-block bound on both supported
usize widths, output bound against the pinned guard and u32/usize, and the
unchecked dimension/product/budget bounds. The position lemmas cite caps_ok
instead of equating MAX_INPUT to its current value. The area comparison uses
Euclidean division and no longer requires MAX_OUTPUT to be a multiple of four.
The compositor's resource bounds use max_output()/max_output_quarter(); the
constructor uses MAX_OUTPUT <= MAX_BYTES rather than equality. Dimension
product helpers use wire-domain inequalities rather than a fixed cap product.
Wire/type facts use u16::MAX; no frozen file or allocation-guard file changed.

The new tools/lint_caps.sh scans Rust sources, including models and proof
bodies, for frozen cap numerals, common decimal/hex/shift spellings, the old
input-lookahead sum, output/4 and sufficiently large configured numeric values.
It excludes only limits.rs and the frozen alloc_guard/mod.rs (the latter owns
its separate, hash-pinned MAX_BYTES). It is a lexical falsification gate, not
a parser for all equivalent Rust expressions; small configured values collide
with non-cap literals. Scratch-copy controls reject eleven injected forms and
accept named wire/type bounds. audit_alloc.sh --self-test invokes this lint.

Run 609 changes ONLY limits.rs's MAX_INPUT declaration to 134217728 in a
disposable scratch copy: 697 verified, 0 errors. All other Rust sources were
byte-identical to the working sources at the time of the run; the source hash
record is runs/sol-20261116/caps-input-half-control.txt. Run 610 changes only
that declaration to u32::MAX as usize: caps_ok rejects the input-lookahead
postconditions, as required (the bit-counter multiplication also fails).
Run 607 was an initial caps_ok shift-arithmetic failure, repaired with a
compute proof of the configured budget bound; run 608 first passed.

### Blocker: a G1-permitted input cap breaks the existing bit-reader contract

Run 611 changes only MAX_INPUT to 536870912 (2^29). caps_ok PASSES: the input
plus the LE16-domain block is 536936448, below 2^32, and all other G1 bounds
are unchanged. Verification FAILS at src/lzw.rs:663, data_len * 8, because
BitReader stores total_bits/bits_left in u32. The smallest helper-domain
counterexample is BitReader::new(&[], 0, 536870912): its sole precondition,
data_len <= MAX_INPUT, holds, but the multiplication is 4294967296, one above
u32::MAX. This is a helper-contract counterexample, not a claimed reachable
512-MiB-file public decoding failure (sub-block framing consumes input bytes).

Do not conceal this failure by adding 8*MAX_INPUT <= u32::MAX to caps_ok:
frozen G1 does not require that bound. To preserve the frozen statement,
widen the reader's bit counts and associated positions/timestamps/reports to
u64, preserving their integer ML observations. If the owner instead chooses
to retain u32 counters, the proposed erratum is an additional G1 requirement
MAX_INPUT <= floor(u32::MAX/8); this narrows permitted cap changes and needs
explicit approval. No such erratum or new restriction was applied here.

A separate source-level resume point is frame_model.rs:255's remaining
MAX_DIMENSION == u16::MAX equality. image_domain bounds descriptor coordinates
and sizes by MAX_DIMENSION, although Clip permits the full LE16 descriptor
domain even when the screen cap is smaller. For example, with MAX_DIMENSION
lowered to 65534, a 1x1 screen and a Clip descriptor at left=65535, top=0,
w=h=1 passes MB's descriptor checks but fails image_domain. No dimension-cap
mutation was run. Replace descriptor-only cap bounds with wire bounds and
propagate that distinction rather than assuming the cap equals the wire limit.

### Validation and Task 2 resume point

runs/sol-20261116/caps-release-tests.log passes all five library/report tests,
seven conformance functions (373 vectors), four allocation-fault tests and the
37,239-frame stress test. runs/sol-20261116/caps-audit-self-test.log exits 0,
including both pinned guard hashes and the cap-lint controls. All six verifier
runs used verify.sh serially, CARGO_BUILD_JOBS=1 and timeout 25m, and appear in
RUNLOG.tsv. Pre-edit backups are runs/sol-20261116/caps-before/. No assumptions,
admissions, external bodies, unsafe code, axioms, solver-limit changes, commits
or surviving background jobs were introduced. Temporary control sources are
not the durable evidence; the run logs and source-hash record are.

The semantic statement chain is STILL public API -> existing ML -> shared
prefix/suffix-chain model. There is no independent ST string-table layer,
no ML/ST simulation and no executable-chain-to-ST representation lemma yet;
the [1,2] at code 6 obligation remains open. Resume Task 2 only after deciding
how to close the permitted-cap-change blocker above, or explicitly separating
that remaining Task 1 work from the independent-dictionary proof.

## T-LZW literal-domain counterexample (Sol, runs 613-614)

After the coordinator accepted Task 1 as partial and requested Task 2, source
review found an unsatisfiable part of the frozen representation clause itself.
THEOREMS.md:71 quantifies over every literal c < clear, including the extended
RB literal domain when m >= 9, and requires model_table[c][0] = code_first.
The requested independent dictionary has type Seq<Seq<u8>>. For m=9, clear=512
and c=256, code_first(t,9,256,4096)=256 and code_suffix(t,9,256)=256 by their
literal rules. No u8 element of any string can equal 256. This is the smallest
supported m and literal c exposing the contradiction. It already occurs in
the initial dictionary, without consuming any input or reading any table byte.
The actual checked-literal guard rejects such a code before a chain walk, so
this is not a public-output counterexample at the current caps.

New src/representation_domain.rs, registered in src/lib.rs, proves the
counterexample for every correctly sized executable reservation and every
4096-entry byte-string dictionary whose entry 256 has length one (the length
the frozen literal rule requires). Its first byte cannot equal code_first.
The lemma also proves that the executable literal chain has length one and
its element is below 256, whereas code_first is 256. Its preconditions are
satisfiable, for example with a zeroed reservation and singleton-zero strings.
This is a refutation of the universal first-byte obligation, NOT an independent
ST model or a proof of the requested representation theorem.

Proposed erratum, not applied: for checked-literal policies, restrict the
representation lemma's literal arm to 0 <= c < min(clear,256), retaining the
whole stored-entry arm clear+2 <= c < next and the checked-root invariant.
Require the existing checked-literal guard to reject the excluded literal
codes before any resolution/emission or table insertion. This preserves all
observable G10 behavior. An integer-string model would be another possible
design, but it would not satisfy the requested Seq<Seq<u8>> layer without a
separate checked conversion theorem. No frozen statement was changed and no
invalid literal was silently excluded from the claimed theorem.

Run 614 verifies 698, 0 errors. Run 613 was a failed exploratory proof that
additionally asserted 256int as u8 == 0; Verus treats that out-of-range ghost
cast as an unspecified byte, so the final proof does not assume a truncation
value and uses only the byte's type bound. Both runs are in RUNLOG.tsv.
runs/sol-20261116/representation-domain-release-tests.log passes all existing
release tests; representation-domain-audit-self-test.log exits 0, including
the cap lint and pinned guard hashes. The pre-registration lib.rs backup is
runs/sol-20261116/caps-before/lib-before-string-domain.rs. No public contract,
frozen file, guard file, assumption, admission, external body, unsafe code,
axiom, solver limit, commit or background job was added or changed.

Task 2 remains blocked on the literal-domain decision. No ST step/fold,
ML/ST simulation, executable-chain/ST representation proof or independent
m=2 literals 1,2 -> table[6]=[1,2] witness is claimed. The current public
statement chain remains API -> ML -> shared prefix/suffix-chain model.

## Independent ST representation layer completed on E4's domain (Sol, runs 615-625)

Run 625 verifies **718, 0 errors**, 23 seconds, 2,716 MiB peak summed RSS.
This section supersedes the preceding Task 2 blocker as an implementation
resume point: after the coordinator recorded proposed E4 and directed work on
its restricted literal domain, the independent model and its bridges were
completed. E4 is still proposed, not owner-approved. The proof does NOT claim
the impossible unrestricted literal-domain statement of frozen THEOREMS.md:71.
Task 1 remains partial for the previously recorded E3 and dimension-cap issues.

### Independent dictionary and observation simulation

New src/string_model.rs defines State.table: Seq<Seq<u8>> and
State.prev: Option<Seq<u8>>, initial, clear, resolve, emit, step, fold and ST.
The definitions never read a prefix/suffix/length representation. They reuse
only mathematical code extraction, powers, the minimum-size predicate and the
common observation/error types, not ML's dictionary operations. Initialization
assigns singleton strings to byte-readable literals and empty strings to
unreadable slots. Clear independently resets this dictionary and prev while
preserving emitted output. The sole insertion is prev ++ [first(cur)]; KwKwK
resolves prev ++ [first(prev)], and full-table non-Clear steps add nothing.
The excluded checked literals are rejected before resolution or insertion.

The readable domain is 0 <= c < min(clear,256) or clear+2 <= c < next. The
simulation relation relates only this domain; it additionally proves decreasing
prefix links, byte roots and the previous-string relation. lemma_initial,
lemma_clear and lemma_emit preserve it. lemma_step proves lockstep control and
state simulation, including exact error-prefix indices, error kind, position
and E4. lemma_fold lifts the relation over the finite bitstream.
theorem_ml_st proves ML(m,D,A,ps) == ST(m,D,A,ps) for 1 <= m <= 11 and
(m <= 8 or ps.checked_literals). It has no valid-input, successful-result or
minimum-size-admissibility precondition: invalid admissibility gives the same
initial error in both models. These conditions cover the actual core policies.
Separate independent lemmas state KwKwK and full-table deferred-clear facts.

### Executable table representation and actual-call connection

theorem_representation connects the independent string at every readable c to
the actual reservation: chain(t,m,c)=table[c], its length equals code_len, its
first byte equals code_first, its last byte equals code_suffix, and its length
is between one and LZW_CODES. The proof crosses raw_prefix/raw_suffix/raw_length
and the executable readable-range and cached-length invariants. Literal facts
come from the code, never from the raw bytes of literal slots. The quantified
package theorem_all_represented exports this result as represented(t,m,s).

Actual lzw::decode now carries an independent ghost sst alongside its existing
ML state. Its loop invariant includes relates and represented; construction,
Clear and each actual run_code insertion establish them. The insertion proof
also runs before a raster-crossing return, so an inserted entry is represented
even when emission subsequently ends at TooManyPixels or StopAtRaster.
lemma_accepted_code ties the checked-literal guard to the byte-readable domain.
The independent model is not defined by this executable ghost state: sst
advances by the separately defined string-table transitions proved to simulate
ML. This makes the representation lemma an actual decoder invariant rather
than an uncalled implication about arbitrarily chosen representations.

Both decode and the real frame adapter decode_image_data retain their existing
contracts and additionally ensure report_matches against ST, raster equality
against ST.idx and colour_observation against ST.idx. No existing public or
helper contract was weakened for this layer. src/lib.rs registers the model
under cfg(verus_only); its string-table states are proof-only and add no runtime
allocation or work loop to the decoder.

The final statement chain is: public still/iterator contracts -> existing
MS/MF/frame models -> the actual decode_image_data/decode ML observations ->
the proved ML == ST observation equality -> independent byte-string G10
semantics. At each concrete table state, the live representation invariant
connects the raw reservation's chain, length, first and suffix to ST.table.
This chain proves Task 2 on E4's corrected readable domain, and explicitly
depends on E4 for matching the frozen representation statement. It does not
apply E3 or resolve the separately proposed reachable-domain erratum E2.

### The reviewer's code-6 obligation and validation

lemma_literal_pair_steps uses the actual encoded first two codes in payload
[17]: m=2, literals 1 and 2, at positions 3 and 6. Before consulting any chain,
the independent dictionary acquires table[6]=[1,2], next=7. The complete
independent witness for payload [145,11] (codes 1,2,6,EOI, widths 3,3,3,4)
proves indices [1,2,1,2], success, bit position 13 and E4=false.

lzw::witness_literal_pair_entry calls the REAL add_entry storage operation
for the second literal, with an arbitrary correctly sized reservation. It
proves raw prefix 6=1, suffix 6=2, stored length 6=2, code_first 6=1 and
chain 6=[1,2]=the independently evolved table[6]. The reservation may contain
stale literal lengths: no premise says raw stored_length(1)=1. The new runtime
regression exercises both this real-call witness and the complete decoder on
zero-filled and 0xa5-filled storage, including exact report and emission log.

runs/sol-20261116/string-model-release-tests.log passes six library/report tests,
seven conformance functions covering 373 vectors, four allocation-fault tests
and the 37,239-frame stress test. string-model-audit-self-test.log exits 0 with
both guard hashes and the cap-lint controls. All 14 FREEZE.txt hashes match.
The final combined halved-cap control, run 624, also verifies 718: only the
MAX_INPUT declaration differs (134217728); every other Rust source is
byte-identical. Its durable source-hash record is
runs/sol-20261116/caps-input-half-with-st-control.txt. Earlier run 610 still
supplies the caps_ok negative control; no unapproved restriction was added.

Runs 615-625 are recorded in RUNLOG.tsv. Run 616 exposed missing raw-array
agreement facts in the representation proof; 617 added them. Run 619 hit the
default compute recursion limit on symbolic local states; 620 used closed
literal terms instead, without a limit increase. Run 621 needed the existing
raw-agreement quantifier to use table_agrees' exact <= next-1 interval; 622
proved the equivalent formulation. An initial release build exposed the model
enum's proof-only import, fixed by gating the whole model module. No assumptions,
admissions, external bodies, unsafe code, axioms, solver/compute limit changes,
frozen/guard edits, commits or surviving background jobs were introduced.
Pre-integration and intermediate source backups are under
runs/sol-20261116/string-before/. No further Task 2 proof work is pending on the
chosen E4 domain; remaining decisions are owner approval of errata and the
Task 1 issues explicitly retained above.
