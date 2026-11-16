# Current status: T-LZW proved (2026-11-15)

**Run 227: 184 verified, 0 errors, default solver limits, `--no-cheating`.**
The executable decode loop and the concrete-call theorem are proved. The model,
emission and main-loop obstructions below are resolved; the remaining sections
preserve the earlier investigation, not current blockers.

`src/decode.rs::lzw_decode` is total over representable caller arguments. For
valid policy bounds `1 <= m_lo <= m_hi <= 11`, three 4,096-entry table views and
sufficient output space, its result equals `ML` exactly. `theorem_t_lzw` calls the
actual decoder and states the exact theorem without a BufferTooSmall alternative.
Strict and RecommendedBeta instantiate the same theorem; every Boolean switch
combination is covered on that policy-bound domain.

The model now returns `(idx, error, pos, e4)`. Errors retain the emitted prefix;
success retains the final bit position. A raster-crossing string emits through A,
then Strict returns TooManyPixels while StopAtRaster succeeds with E4. The former
counterexample `m=2, D=[140,11], A=2` is now a positive regression: `[1,1]`, bit 9,
E4=true under RecommendedBeta. The custom Strict-tail/checked-literal combination
also respects the oracle's step-4 precedence.

The decoder uses no scratch stack. `emit_backwards` walks the chain and writes
indices directly at their forward output positions, skipping positions at or past
A without computing a potentially overflowing address. `emit_code` handles the
KwKwK append and the clipped-prefix joining proof. The table remains the three
caller-owned views totaling 20,480 bytes, with no literal-table initialisation.
Stale bytes from earlier frames are allowed; the readable-range invariant makes
them irrelevant. Bit positions use u128 to cover eight times any representable
32-bit or 64-bit slice length without narrowing or overflow.

The proved main-loop invariants include cached-length recurrence, prefix/suffix
agreement with the canonical model on `[clear+2,next)`, width scheduling for m>=2,
the size-1 one-pixel case and byte-valued roots under checked literals. L-FULL
preserves a full table and width 12 on non-Clear advancing steps. L-DISCARD proves
that a raster-crossing code discards at most A-1 indices. The termination variant
is exactly `8*|D| - pos`; each chain walk decreases its remaining string length.

The difficult solver points were concrete-slice triggers for array agreement and
the combined emission/fold query. Splitting prefix and suffix quantifiers and
moving resolved-code emission into its own verified helper solved them. Run 224
used a temporary bounded 20-unit diagnostic limit; the final source has no rlimit
attribute and runs 225–227 pass at the default limit.

Validation now runs the actual decoder, not the former Rust model mirror:

- 200 random streams, 47 vector calls and all 12 original witnesses pass.
- 10,848 oracle-backed records compare indices, exact errors, positions and E4.
- The Python transcription/oracle comparison passes 57,120 cases across policy switches.
- The original 12 and 96 additional oracle samples verify complete ML observations.
- Three executable emission tests cover stale, branching, maximal and clipped chains.
- Two generated stress payloads contain 11,487,200 and 11,911,619 compressed bytes.

Both stress payloads decode 8,388,608 indices, reach width 12 and a full table, and
match the independent Strict oracle, with and without Clear at a full table.
`cargo test` passes eight tests with none ignored. The generators send stress data
over a pipe and retain no large vector files.

Raw-byte accessors, allocator/parser integration, the compositor and the full T12
work-accounting proof remain outside this standalone milestone. No requested
standalone decode-loop equality obligation remains open. No source outside this
crate was edited, no commits were made, and no background work was left running.

---

# Historical: initial Sol stop before coordinator-approved model correction

The independent emission helper was proved, but T-LZW and the main loop were not
yet proved. The following diagnosis led to the coordinator's instruction to fix
the model and replace scratch-stack emission with direct backward writes.

## Blocking model/policy counterexample

Use `m = 2`, `D = [140, 11]`, `A = 2`, RecommendedBeta. The packed codes are
Clear(4), literal(1), KwKwK(6), EOI(5), each three bits wide:

- Clear ends at bit 3 and resets the dictionary.
- Literal 1 ends at bit 6, leaving `q = 1`, `prev = 1`, `next = 6`.
- Code 6 ends at bit 9 and resolves to `[1, 1]`, although only one pixel remains.

`policy-draft/OPTIONS.md`'s `lzw_tail=StopAtRaster` discards the last index,
accepts `[1, 1]`, and records E4. `policy_oracle.py` does exactly that. The current
`src/model.rs::verdict` unconditionally returns `TooManyPixels` when
`st.q + s.len() > a`, even when `ps.stop_at_raster` is true, so its `ML` returns
`Err((TooManyPixels, 9))`. `src/regressions.rs` proves that result by computation;
`python3 -I tools/check_model_policy_gap.py` checks the independent oracle side.

This is not a reason to weaken T-LZW or to prove an executable that implements the
incorrect model. The next stage needs to correct the model transcription while
preserving the policy specification and add an E4-overrun conformance case before
resuming the main-loop proof. I have not changed the model's semantics.

The current result schema also cannot express the whole manifest conclusion:
`MRes = Result<Seq<u8>, (ErrK, int)>` drops the successful bit position and drops the
emitted index prefix on errors, whereas `THEOREMS.md`'s ML row specifies
`(idx, out, pos)` for every outcome. Those observations must remain available in
an exact T-LZW theorem; `result_eq` currently does not state them.

## What is now proved

- `lemma_peel_not_reserved`, including the literal-parent contradiction with
  `lemma_peel_one` called inside that branch.
- `lemma_str_len_bound`, with a real nonempty-string bound of at most 4,096 and
  the stronger bound `len <= c - clear` for stored codes.
- `lemma_len_bound`, replacing its old reflexive postcondition with that bound.
- `emit_string`'s complete concrete push and pop loops, with table/stack/output
  bounds, termination, exact forward emission and preservation outside the segment.
- `emit_string` returns `BufferTooSmall` exactly when the remaining output is too
  short and leaves the output unchanged on that error.

The push loop carries `chain_ok` for its current cursor directly and uses
`lemma_chain_step` for each parent read. It does not need to rediscover the
reserved-code exclusion through `peel` at every iteration. Its flat stack
invariant uses `pushbytes`; `lemma_rev_splice` and `lemma_rev_is_reverse` connect
that invariant to the forward output. A root-preservation invariant proves the
literal-to-byte cast fits; the caller must establish `root_of(...) < 256`.

The helper needs a 4,096-byte caller-owned stack. G18's 20,480-byte table contains
no stack, and the current decode shell asks only for `stack.len() >= a`.
Connecting this helper to G18 therefore remains open; no reservation or table
layout was changed.

## Verification and tests

Runs 192–196 are in `RUNLOG.tsv`; run 196 verified **61 obligations, 0 errors**.
The final documentation-only update is reverified in the final run recorded in
`LEDGER.md`. No `assume`, `admit`, `external_body`, `unsafe`, new axioms or `rlimit`
were added. `cargo test` passes five tests and ignores the existing executable
random-stream comparison. The two new emission tests run the actual helper on
branching chains, maximal-length chains, `m=11`-sized literal domains, stale table
bytes, insufficient output and preservation of untouched output/stack positions.

The requested executable random/vector gate remains pending because `lzw_decode`
is still a stub. Its dormant random test also selects RecommendedBeta despite
using Strict expectations and accepts any error name; those checks need correction
when the real executable is connected. Neither array agreement nor the length
cache nor the main fold invariant has been proved.

---

# Historical: the prefix-chain walk in `chain.rs` — RESOLVED

## The problem

The executable stores a code's string as a prefix chain (`prefix[c]`, `suffix[c]`)
with literals resolved by rule (D20). The model needs the walk as a spec function,
and Verus refuses `decreases c` for a recursion over `pf[c] as int`: the link fact
`prefix[c] < c` is a hypothesis, so no measure Verus can see decreases structurally
(probe at run 14: `could not prove termination` on the walk's own definition).

## What failed, in order

1. **`decreases c` with the link fact as `requires`** — rejected for the spec
   definitions themselves (the measure has to be structural *at the definition*).
2. **Fuel budget `fuel >= c - clear + 2`** — works for the walk, but the hand-off
   arithmetic is case-split by the parent's kind: a link points at a literal
   (`< clear`) or at an entry in `[clear + 2, c)`, and the gap at `clear`, `eoi`
   makes "one less fuel still covers the child" two different arguments
   (`nonlinear_arith` on each call site, 2-3 errors left over at runs 14-17).
3. **`decreases c` with the fuel as well** — Verus then cannot see `fuel - 1`
   decrease either, and every recursive call fails its precondition at runs 19-26.

## The fix

Two changes, both structural:

* **Budget `fuel > c`**, not `fuel >= c - clear + 2`. The budget is uniform in the
  entry index, so the hand-off is one line of arithmetic
  (`fuel > c` and `p < c` imply `fuel - 1 > p`) with no case split — `lemma_fuel_step`.
* **`chain_ok` carries the walk's domain**: `c < clear || clear + 2 <= c < 4096`,
  with `0 <= c`. That turns "the parent is a literal or a stored entry" into a
  disjunction the caller can quote directly instead of deriving from the link
  `forall`, which is what the leftover `postcondition not satisfied` on
  `0 <= root_of(...) < clear` needed.

The recursive lemmas then keep `decreases c` (each parent is smaller) and the fuel
is a pure *permission* argument: every call site has `fuel_above` for the child from
`lemma_fuel_step`, and the budget never runs out. Result: run 29, **21 verified,
0 errors**.

## Note for the model

`chain.rs` keeps `stored_len` (the raw LE16 pair, zero under a literal) and
`code_len` (`1` for a literal, else `stored_len`) apart, per Sol's review note. The
string/length lemmas use `code_len`; only `lemma_frame` (which entries a walk reads)
uses the byte region directly.


---

# STUCK: the executable decode loop (`lzw_decode`'s body)

## Where I am

The executable's *shell* verifies: the four guard clauses, the G9 check through
`lemma_m_ok_iff`, the total signature and the `result_eq` postcondition shape. The
loop body itself is **not written**: after the guards the function returns
`BufferTooSmall`, so `lzw_decode` is panic-free and total but does **not** yet equal
`ML`. That is the one thing T-LZW still needs, and it is the largest single
obligation of the milestone.

## What is already in place for it

* `model::lzw_decode`'s invariant has a working shape (verified at run 96's
  predecessor): the concrete `pos`/`width`/`next`/`prev`/`q` and the emit loop, with
  the model's `L` fold as the ghost state. I verified that shape at 24 obligations
  with a stub body, so the plumbing is not what is missing.
* Every chain fact the body needs is proved in `chain.rs`: `lemma_chain_step`,
  `lemma_rev_is_reverse`, `lemma_str_shape`, `lemma_len_bound`, `lemma_root_literal`,
  `lemma_frame`, `lemma_fresh_entry`.
* `model::verdict` is the step function, and `lemma_L_step_pos` gives the variant
  decrease, so one iteration of the concrete loop is one `verdict` equation.

## What is left, concretely

1. **The stack push/pop pair.** The loop must walk the code's chain, pushing
   `suffix[c]` while `c >= clear`, then pop into `out`. `chain::lemma_rev_is_reverse`
   is exactly the identity that makes the pushed order the emitted order, but the
   loop invariant relating `stack[0..top]` to `chain::rev_of` at each chain step is
   the piece I have not written.
2. **The array-agreement invariant.** The model reads `pf`/`sf` directly; the
   concrete loop writes `prefix`/`suffix` through `&mut` slices. The invariant "the
   two slices are the model's arrays on `[clear + 2, next)`" has to be stated as a
   `forall` over the *current* ghost `next`, and the Clear step's non-re-zeroing
   (which is D20's whole point) has to be *used*, not re-proved: after a Clear the
   unreadable range grows, and `lemma_frame` is what keeps the old bytes irrelevant.
3. **`length` as a cache.** The model needs no length array (its `code_len` is
   computed by `chain::code_len`), but the executable's per-frame work bound requires
   the cached value, so the invariant must also say `length[c] == code_len(...)` and
   the write must be shown to preserve it via `lemma_fresh_entry`.

## Estimate

The three pieces are each a loop invariant plus a small step lemma: I would budget
another 15-25 verifier runs and 60-120 obligations, in the same shape as the
`chain.rs` campaign (which took 17 runs from first attempt to green, most of them
spent on the fuel arithmetic rather than on the mathematics).

---

# Continuing the decode loop (coordinator's follow-up)

## Progress since the report

Chain theory for piece (a) is **proved** (runs 99-110, green at 46 obligations):

* `peel(pf, c, k)` — the push loop's cursor after `k` parent links.
* `pushbytes(sf, pf, c, k)` — the bytes the push loop writes, *flat*: slot `i` is
  `sf[peel(c, i)]`. This is the form the push loop's invariant can read directly,
  unlike the recursive `peek`.
* `lemma_pushbytes_cons` — the cons recurrence `pushbytes(k) = [sf[c]] + pushbytes(k-1)`.
* `lemma_pushbytes_peek` — while the first `k` codes are non-literal, the flat form
  equals `peek`.
* `lemma_peek_splice` / `lemma_rev_splice` — the splice identity
  `rev_of(c, fuel) = pushbytes(c, k) + rev_of(peel(c, k), fuel - k)`, which is what
  lets the push loop stop after `k` steps and hand the rest to the model's walk.

The lesson from this run of work: `peek` (recursive, on `c`) is right for *stating*
the splice, and `pushbytes` (flat, on `k`) is right for *maintaining* the loop
invariant; the two need the bridge `lemma_pushbytes_peek`, and trying to use either
one alone costs several runs (runs 100-107 were exactly that).

## What is still missing

1. The concrete push/pop loops inside `lzw_decode` (piece (a)'s executable half): the
   push loop maintains `stack[k - 1 - i] == pushbytes(c, k)[i]` and the pop loop
   writes `out[q0 + j] == str_of(...)[j]`. `lemma_pushbytes_peek` +
   `lemma_rev_splice` + `lemma_rev_is_reverse` discharge the two exits.
2. The `[clear + 2, next)` array-agreement invariant and `length[c] == code_len`
   (pieces (b) and (c)); `lemma_frame` and `lemma_fresh_entry` are the lemmas.
3. The main loop's fold invariant linking concrete `pos`/`width`/`next`/`prev`/`q`
   to `model::verdict`, which I had verified in *shape* at 24 obligations.

## Honest estimate

20-35 more verifier runs. The three sub-pieces are independent enough that a stall in
one need not block the others, but the array-agreement invariant (2) is the one I
expect to be hardest: it has to be stated over `next` as a *variable* bound while
`lemma_frame`'s phrasing fixes the written slot, so a push into the array must first
move `next` in the ghost, then the frame lemma applies.

---

# The push loop's cursor range: split, per the coordinator's 10-run rule

## What stalled

The executable push loop (`emit_string`, runs 111-150, ~40 runs) is in bounds and its
`stack@[i] == pushbytes[i]` invariant holds; two conjuncts fail to be *preserved*:

1. `cur >= clear + 2` — the peeled cursor is a stored entry, not a literal.
2. `(pf[cur] as int) < cur` — the parent is strictly smaller.

Both are one-line consequences of `chain::link_ok` (`pf[cur] < clear || (clear + 2 <=
pf[cur] && pf[cur] < cur)`) and `peel_stored`. They do not go through because the
*Ghost* view and the *exec* read are not being connected: `let pfx: u16 = pf[cur]`
introduces an exec value, and `pf@[cur as int] == pfx` is what the arithmetic needs,
which Verus does not hand over automatically for a `&[u16]` parameter. Every attempt
(runs 140-150) failed on that bridge, not on the mathematics.

## The split, as separate lemmas to attack

**P1. `lemma_slice_view`** (the bridge; this is the real blocker).

```rust
proof fn lemma_slice_view(pf: &[u16], cur: usize, pfx: u16)
    requires cur < pf.len(), pf[cur] == pfx
    ensures  pf@[cur as int] == pfx
```

Either proved once, or avoided by never binding `pfx` and instead writing
`assert((pf@[cur as int] as int) < cur)` and then `cur = pf[cur] as usize` with the
ghost fact quoted after the write. I did not try the second form.

**P2. `lemma_cur_stored`** — `cur >= clear + 2` from `peel_stored(pf, clear, c, top)`.
Independent of P1; needs only `peel`'s recursion at `top > 0` and `cur == c` at
`top == 0`.

**P3. `lemma_cur_decreases`** — `(pf@[cur] as int) < cur` from `link_ok`, i.e.
`chain::lemma_chain_step` at `c := cur` with `fuel := fuel_of()`. Needs `cur < 4096`
(from `lemma_peel_bound`, proved) and `chain_ok(pf, clear, cur)` (from the loop's
`chain_ok` at `c` plus monotonicity — **not yet proved**: `lemma_chain_ok_widen` was
attempted at runs 130-138 and failed, so this is a second candidate for splitting).

## What is proved and can be relied on

`chain::lemma_peel_bound`, `lemma_peel_zero`, `lemma_peel_one`,
`lemma_peel_step_pub`, `lemma_pushbytes_cons`, `lemma_pushbytes_peek`,
`lemma_peek_splice`, `lemma_rev_splice`, `lemma_rev_is_reverse`, `lemma_str_shape`,
`lemma_fresh_entry`, `lemma_frame`. The emission's two-leg structure needs no new
mathematics: once P1-P3 discharge, `lemma_rev_splice` + `lemma_rev_is_reverse` close
the pop loop and `str_of`'s definition closes the model link.

## Runs spent

Runs 111-150 on this sub-piece, of which ~40 were mine and the rest the earlier
`chain.rs` campaign's tail. Per the coordinator's rule I am stopping here and
reporting rather than continuing to grind.

## Sharpened diagnosis (runs 151-172)

P1 (the slice-view bridge) is **not** a blocker after all: `src/probe.rs` (run 172)
proved that Verus hands over `pf@[i as int] == v` for an exec read `let v = pf[i]`
with `i < pf.len()`. So the read must simply come *before* the assert in the body,
which the last attempts did.

The real stall is **P2**, and it is an off-by-one in `peel_stored`'s instantiation:

```rust
pub open spec fn peel_stored(pf, clear, c, k) -> bool {
    forall|j: int| 0 <= j < k ==> peel(pf, c, j) >= clear     // j < k, not <= k
}
```

The loop needs `peel(c, top) >= clear` (the cursor `cur` is a stored entry, so
`pf[cur]` is a *table* read, not a literal). But `peel_stored(.., top)` only bounds
`j < top`. The fix is one of:

* state the invariant as `peel_stored(.., top + 1)`, so `j = top` is included, and
  prove the loop body re-establishes it at `top + 1` (needs `peel(c, top+1) = pfx`
  and `pfx >= clear`, i.e. exactly P3); or
* add `lemma_peel_bound`-style `<=`-form: `forall j, 0 <= j <= k ==> peel >= clear`,
  which is `peel_stored` at `k + 1` and needs no new mathematics.

**P3** (`(pf[cur] as int) < cur` from `link_ok`) is a one-liner once `cur >= clear + 2`
is in scope, because `link_ok(pf, clear, cur)` is then directly available from the
loop's `chain_ok`; the last attempts failed only because P2's failure hid it.

So the whole remaining obstruction is P2's bound form, and the second fix above is
the cheapest: changing `peel_stored`'s `j < k` to `j <= k` (or adding a `+ 1` at the
invariant) should unlock the push loop, after which the pop loop is
`lemma_rev_splice` + `lemma_rev_is_reverse` and the model link is `str_of`'s
definition.

## Final state of the push loop (runs 173-210), and the one obstruction left

The stall is now pinned to a **single** arithmetic fact, not to the mathematics. What
is in place and proved:

* `peel_stored` widened to `0 <= j <= k`, so the loop's `cur >= clear` is immediate.
* `lemma_peel_add` (associativity: `peel(c, a + b) == peel(peel(c, a), b)`), which is
  what makes `peel(c, top + 1) == pf[cur]` go through.
* `lemma_link_not_reserved` (`link_ok(pf, clear, x)` at `x >= clear + 2` gives
  `pf[x] != clear` and `pf[x] != clear + 1`).
* The slice-view bridge is not a problem: `src/probe.rs` (run 172) proved
  `pf@[i as int] == v` follows from `let v = pf[i]` with `i < pf.len()`.

The **one** remaining fact is `cur >= clear + 2` inside the loop, i.e. that the
cursor is never `clear` or `eoi`. It needs `lemma_peel_not_reserved`
(`peel(pf, c, k) != clear && != clear + 1`), which I attempted at runs 195-210 and did
not finish: its literal-parent branch needs the case `k >= 2 ==> peel(c, 1) ==
pf[c] >= clear`, i.e. `peel_stored` at `j = 1` contradicting `pf[c] < clear`, and
`peel(pf, c, 1) == pf[c]` needs `lemma_peel_one` to be quoted *after* the case split
rather than before. My last attempt (run 209) failed on exactly that
`assert(peel(pf, c, 1) == p)`.

That is a two-line fix in one lemma, then the push loop's invariant should close, and
the pop loop is `lemma_rev_splice` + `lemma_rev_is_reverse` with no new mathematics.

## Runs

Runs 111-210 on the executable emission pair. Per the coordinator's rule this is
reported rather than ground further. No `assume`/`admit`/`external_body`/`unsafe`/
`rlimit` anywhere in the crate; the one I briefly added at run 168 was removed before
the next run and never verified.
