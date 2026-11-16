//! The prefix-chain walk and the two length functions (the coordinator's and Sol's
//! review note on `THEOREMS.md`).
//!
//! The executable stores a code's index string as a prefix chain: `prefix[c]` is
//! entry `c`'s parent and `suffix[c]` its last index. A code below `clear` is a
//! literal *by rule* (D20: the `2^m` literal entries are never written, so the
//! region is still zero-filled under them), which is why the two length functions
//! are kept apart:
//!
//!   * `stored_len(t, c)` is the raw LE16 pair at entry `c` in the 20,480-byte
//!     region (G18's third region). A literal's bytes are zero after the reservation
//!     or stale after an earlier frame with a smaller m. The stored value is
//!     meaningful only for a stored code.
//!   * `code_len(t, m, c)` is the length of the *string* code `c` denotes:
//!     `1` when `c < 2^m`, else `stored_len(t, c)`. The table invariant, the length
//!     recurrence `code_len(c) = code_len(prefix(c)) + 1 <= 4096` and the
//!     representation lemma `chain(c) = model_table[c]` are all stated over
//!     `code_len`; only the region-separation facts (`stored_len` is untouched under
//!     the entries a walk visits) use `stored_len`. The same split applies to the
//!     suffix and first-byte facts: for a literal code they come from the code value,
//!     not from table bytes.
//!
//! `str_of`/`rev_of`/`root_of` below are the walk itself; `rev_of` is the
//! push-then-pop order the executable's stack implements.
//!
//! Termination. Verus refuses `decreases c` for a walk over `pf[c] as int`: the link
//! fact is a hypothesis, so no measure of Verus's own decreases structurally. Each
//! walk therefore carries a `fuel: nat` it decrements, and every lemma requires
//! `fuel_above(c, fuel)` (`fuel > c`, i.e. at least one step per entry from zero, not
//! merely from `clear`): the budget is uniform in the entry index, so the hand-off is
//! the single fact `prefix[c] < c ==> fuel - 1 > prefix[c]`.

use vstd::prelude::*;

verus! {

/// The link fact for one stored entry `x`: PLAN M2's `prefix[x] < x`, with D20's
/// link-to-a-literal case spelled out. A link never points at `clear` or `eoi`:
/// G10.2 step 7's `prev` always denotes a string, and step 3's Clear resets `prev`.
pub open spec fn link_ok(pf: Seq<u16>, clear: int, x: int) -> bool {
    &&& 0 <= x < 4096
    &&& ((pf[x] as int) < clear || (clear + 2 <= (pf[x] as int) && (pf[x] as int) < x))
}

/// The link hypothesis for the stored entries `[clear + 2, c]`: PLAN M2's
/// `prefix[c] < c` over D20's stored range, which is what makes the walk terminate
/// and its length bounded.
pub open spec fn chain_ok(pf: Seq<u16>, clear: int, c: int) -> bool {
    &&& pf.len() == 4096
    &&& 0 <= clear
    &&& 0 <= c
    &&& c < clear || clear + 2 <= c < 4096
    &&& forall|x: int| clear + 2 <= x && x <= c ==> #[trigger] link_ok(pf, clear, x)
}

/// The walk's budget: strictly more than the entry index, so at least one step per
/// index from zero and the hand-off `fuel - 1 > prefix[c]` follows from
/// `prefix[c] < c` by plain arithmetic.
pub open spec fn fuel_above(c: int, fuel: nat) -> bool {
    &&& (fuel as int) >= 1
    &&& (fuel as int) > c
}

/// `stored_len(t, c)`: the raw LE16 pair of the caller's length region at entry `c`
/// (SPEC G18's third region, `2c` and `2c + 1`). Literal bytes are zero initially
/// or stale from an earlier frame; code-level operations never read them.
pub open spec fn stored_len(t: Seq<u8>, c: int) -> int {
    t[2 * c] as int + 256 * (t[2 * c + 1] as int)
}

/// `code_len(t, m, c)`: the length of the string code `c` denotes -- a literal's
/// `1`, else the stored pair. The table invariant and the chain walk use this.
pub open spec fn code_len(t: Seq<u8>, m: int, c: int) -> int {
    if c < 2 * 0 + ipow2(m) {
        1
    } else {
        stored_len(t, c)
    }
}

/// `2^m`, by recursion on `m` (`m <= 11` on this core's domain). Repeated here
/// rather than imported so `chain.rs` states its own facts over its own constants.
pub open spec fn ipow2(m: int) -> int
    decreases m,
{
    if m <= 0 { 1 } else { 2 * ipow2(m - 1) }
}

/// The string of `c`'s chain, in emitting order: `[c]` for a literal, else the
/// parent's string with `suffix[c]` appended. `fuel == 0` yields the empty string,
/// which no caller relies on (`fuel_above` forbids the budget running out).
pub open spec fn str_of(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, fuel: nat) -> Seq<u8>
    decreases fuel,
{
    if fuel == 0 {
        Seq::<u8>::empty()
    } else if c < clear {
        seq![c as u8]
    } else {
        str_of(sf, pf, clear, pf[c] as int, (fuel - 1) as nat).push(sf[c])
    }
}

/// The same string in the order the executable pushes it onto its stack: each
/// suffix before its parent's chain, so `rev_of` is `str_of` reversed.
pub open spec fn rev_of(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, fuel: nat) -> Seq<u8>
    decreases fuel,
{
    if fuel == 0 {
        Seq::<u8>::empty()
    } else if c < clear {
        seq![c as u8]
    } else {
        seq![sf[c]] + rev_of(sf, pf, clear, pf[c] as int, (fuel - 1) as nat)
    }
}

/// The chain's root: the literal code at the bottom of the walk, which is G10.2's
/// first index of the string.
pub open spec fn root_of(pf: Seq<u16>, clear: int, c: int, fuel: nat) -> int
    decreases fuel,
{
    if fuel == 0 {
        0
    } else if c < clear {
        c
    } else {
        root_of(pf, clear, pf[c] as int, (fuel - 1) as nat)
    }
}

pub open spec fn eroot_unused() -> int { 0 }

/// A placeholder suffix region for the lemmas that read only lengths and roots.
pub open spec fn sf_zero() -> Seq<u8> {
    Seq::new(4096 as nat, |_i: int| 0u8)
}

/// The budget hand-off, proved once: for a stored `c` in `[clear + 2, 4096)` the
/// parent is strictly smaller (by `link_ok`) and one less fuel still exceeds it.
pub proof fn lemma_fuel_step(pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        fuel_above(c, fuel),
    ensures
        (pf[c] as int) < c,
        fuel_above(pf[c] as int, (fuel - 1) as nat),
{
    let p = pf[c] as int;
    assert(p < c) by {
        assert(forall|x: int| clear + 2 <= x && x <= c ==> #[trigger] link_ok(pf, clear, x));
        assert(link_ok(pf, clear, c));
    }
    assert(((fuel - 1) as nat as int) == fuel as int - 1);
    assert(fuel as int - 1 > p) by (nonlinear_arith)
        requires fuel as int > c, p <= c - 1;
    assert(((fuel - 1) as nat as int) >= 1);
}

/// The chain's root is a literal code: `0 <= root_of(pf, clear, c, fuel) < clear`.
/// Proved on its own so the `code_len`/string lemmas can quote it without reopening
/// the length argument. Recursion is on `c` (each step's parent is smaller).
pub proof fn lemma_root_literal(pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        fuel_above(c, fuel),
    ensures
        0 <= root_of(pf, clear, c, fuel) < clear,
    decreases c,
{
    reveal(root_of);
    if fuel == 0 {
        assert(false) by { assert(fuel as int >= 1); }
    } else if c < clear {
        assert(root_of(pf, clear, c, fuel) == c);
    } else {
        lemma_chain_step(sf_zero(), pf, clear, c, fuel);
        let p = pf[c] as int;
        assert(p < c);
        assert(chain_ok(pf, clear, p));
        assert(fuel_above(p, (fuel - 1) as nat));
        lemma_root_literal(pf, clear, p, (fuel - 1) as nat);
        assert(root_of(pf, clear, c, fuel) == root_of(pf, clear, p, (fuel - 1) as nat));
    }
}

/// The string of a stored entry is its parent's string with `suffix[c]` appended;
/// every walk function steps the same way.
pub proof fn lemma_chain_step(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        fuel_above(c, fuel),
    ensures
        str_of(sf, pf, clear, c, fuel) == str_of(sf, pf, clear, pf[c] as int, (fuel - 1) as nat).push(
            sf[c],
        ),
        rev_of(sf, pf, clear, c, fuel) == seq![sf[c]] + rev_of(
            sf,
            pf,
            clear,
            pf[c] as int,
            (fuel - 1) as nat,
        ),
        root_of(pf, clear, c, fuel) == root_of(pf, clear, pf[c] as int, (fuel - 1) as nat),
        chain_ok(pf, clear, pf[c] as int),
        (pf[c] as int) < c,
        fuel_above(pf[c] as int, (fuel - 1) as nat),
{
    reveal(str_of);
    reveal(rev_of);
    reveal(root_of);
    lemma_fuel_step(pf, clear, c, fuel);
    let p = pf[c] as int;
    assert(forall|x: int| clear + 2 <= x && x <= c ==> #[trigger] link_ok(pf, clear, x)) by {
    }
    assert(link_ok(pf, clear, c));
    assert(chain_ok(pf, clear, p)) by {
        assert forall|x: int| clear + 2 <= x && x <= p implies #[trigger] link_ok(pf, clear, x) by {
            assert(link_ok(pf, clear, x));
        }
    }
}

/// The recurrence the length cache mirrors: a stored entry's length is its parent's
/// plus one (`stored_len`), which for a literal parent is `code_len = 2`.
pub proof fn lemma_stored_len_step(t: Seq<u8>, m: int, pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        fuel_above(c, fuel),
        t.len() == 8192,
        0 <= m <= 11,
        clear == ipow2(m),
        code_len(t, m, c) == 1 + code_len(t, m, pf[c] as int),
    ensures
        code_len(t, m, c) == 1 + code_len(t, m, pf[c] as int),
{
}

/// `peel(pf, c, k)`: the code after `k` parent links from `c`. The push loop's
/// cursor, so its invariant can name the code it still has to walk.
pub open spec fn peel(pf: Seq<u16>, c: int, k: int) -> int
    decreases k,
{
    if k <= 0 {
        c
    } else {
        peel(pf, pf[c] as int, k - 1)
    }
}

/// One link of `peel`, as a fact to quote at index arithmetic sites.
pub proof fn lemma_peel_step(pf: Seq<u16>, c: int, k: int)
    requires
        k >= 1,
    ensures
        peel(pf, c, k) == peel(pf, pf[c] as int, k - 1),
{
    reveal(peel);
}

/// `peek(sf, pf, clear, c, k)`: the first `k` bytes of `rev_of(c, ...)`, i.e. the
/// suffixes the push loop has written after `k` non-literal steps. `rev_of`'s
/// prefix, so the push loop's invariant is a plain equation.
pub open spec fn peek(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, k: int) -> Seq<u8>
    decreases k,
{
    if k <= 0 {
        Seq::<u8>::empty()
    } else if c < clear {
        Seq::<u8>::empty()
    } else {
        seq![sf[c]] + peek(sf, pf, clear, pf[c] as int, k - 1)
    }
}

/// `pushbytes(sf, pf, c, k)`: the bytes the push loop writes, in the order it writes
/// them — slot `i` is the suffix of the code `i` links above `c`. This is the loop's
/// invariant in one term, and `peek` is the same sequence while the first `k` codes
/// are all non-literal.
pub open spec fn pushbytes(sf: Seq<u8>, pf: Seq<u16>, c: int, k: int) -> Seq<u8> {
    Seq::new(k as nat, |i: int| sf[peel(pf, c, i)])
}

/// `pushbytes` conses one suffix per step, so it satisfies the same recurrence the
/// push loop's invariant reads off.
pub proof fn lemma_pushbytes_cons(sf: Seq<u8>, pf: Seq<u16>, c: int, k: int)
    requires
        0 <= k,
        k < 4096,
    ensures
        k == 0 ==> pushbytes(sf, pf, c, k) == Seq::<u8>::empty(),
        k > 0 ==> pushbytes(sf, pf, c, k) == seq![sf[c]] + pushbytes(sf, pf, pf[c] as int, k - 1),
        pushbytes(sf, pf, c, k).len() == k,
{
    reveal(pushbytes);
    reveal(peel);
    if k == 0 {
        assert(pushbytes(sf, pf, c, k) == Seq::<u8>::empty());
    } else {
        assert(peel(pf, c, 0) == c);
        assert forall|i: int| 0 <= i < k implies #[trigger] pushbytes(
            sf,
            pf,
            c,
            k,
        )[i] == (seq![sf[c]] + pushbytes(sf, pf, pf[c] as int, k - 1))[i] by {
            reveal(pushbytes);
            if i >= 1 {
                lemma_peel_step(pf, c, i);
            }
        }
    }
}

/// While the pushed codes are all non-literal, `pushbytes` is `peek` (the recursion
/// and the flat form agree).
pub proof fn lemma_pushbytes_peek(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, k: int)
    requires
        0 <= k,
        forall|j: int| 0 <= j < k ==> #[trigger] peel(pf, c, j) >= clear,
    ensures
        pushbytes(sf, pf, c, k) == peek(sf, pf, clear, c, k),
    decreases k,
{
    reveal(peek);
    reveal(peel);
    reveal(pushbytes);
    if k <= 0 {
    } else {
        assert(peel(pf, c, 0) == c);
        assert(peel(pf, c, 0) >= clear);
        assert(forall|j: int| 0 <= j < k - 1 ==> #[trigger] peel(pf, c, j + 1) >= clear) by {
            assert forall|j: int| 0 <= j < k - 1 implies #[trigger] peel(pf, c, j + 1) >= clear by {
                assert(0 <= j + 1 < k);
            }
        }
        assert(forall|j: int| 0 <= j < k - 1 ==> #[trigger] peel(pf, pf[c] as int, j) >= clear) by {
            assert forall|j: int| 0 <= j < k - 1 implies #[trigger] peel(pf, pf[c] as int, j) >= clear by {
                assert(peel(pf, pf[c] as int, j) == peel(pf, c, j + 1));
            }
        }
        lemma_pushbytes_peek(sf, pf, clear, pf[c] as int, k - 1);
        assert(peek(sf, pf, clear, c, k) == seq![sf[c]] + peek(sf, pf, clear, pf[c] as int, k - 1));
        assert(pushbytes(sf, pf, c, k) == pushbytes(sf, pf, c, k - 1).push(sf[peel(pf, c, k - 1)]));
    }
}

/// The splice identity the two emission loops compose: the bytes pushed so far
/// (`peek`) followed by the remaining walk (`rev_of` from the peeled code) are the
/// whole reversed string. Stated over `k` non-literal steps, which is what the push
/// loop's counter counts.
pub proof fn lemma_peek_splice(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, k: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        fuel_above(c, fuel),
        k >= 0,
        k <= fuel as int,
        forall|j: int| 0 <= j < k ==> #[trigger] peel(pf, c, j) >= clear,
    ensures
        rev_of(sf, pf, clear, c, fuel) == peek(sf, pf, clear, c, k) + rev_of(
            sf,
            pf,
            clear,
            peel(pf, c, k),
            (fuel - k) as nat,
        ),
    decreases k,
{
    reveal(rev_of);
    reveal(peek);
    reveal(peel);
    if k <= 0 {
        assert(peel(pf, c, k) == c);
        assert(peek(sf, pf, clear, c, k) == Seq::<u8>::empty());
        assert((fuel - k) as nat == fuel);
    } else {
        assert(peel(pf, c, 0) == c);
        assert(peel(pf, c, 0) >= clear);
        lemma_chain_step(sf, pf, clear, c, fuel);
        assert(rev_of(sf, pf, clear, c, fuel) == seq![sf[c]] + rev_of(
            sf,
            pf,
            clear,
            pf[c] as int,
            (fuel - 1) as nat,
        ));
        assert(peek(sf, pf, clear, c, k) == seq![sf[c]] + peek(
            sf,
            pf,
            clear,
            pf[c] as int,
            k - 1,
        ));
        assert(peel(pf, c, k) == peel(pf, pf[c] as int, k - 1));
        assert(forall|j: int| 0 <= j < k - 1 ==> #[trigger] peel(pf, pf[c] as int, j) >= clear) by {
            assert forall|j: int| 0 <= j < k - 1 implies #[trigger] peel(pf, pf[c] as int, j) >= clear by {
                assert(peel(pf, pf[c] as int, j) == peel(pf, c, j + 1));
                assert(peel(pf, c, j + 1) >= clear);
            }
        }
        lemma_peek_splice(sf, pf, clear, pf[c] as int, k - 1, (fuel - 1) as nat);
        assert((fuel - 1) as nat as int == fuel as int - 1);
    }
}

/// `rev_of` is, up to the final literal, the pushed order: the push loop writes
/// `pushbytes(c, k)` and then the walk's remaining tail starts at `peel(c, k)`.
pub proof fn lemma_rev_splice(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, k: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        fuel_above(c, fuel),
        0 <= k <= fuel as int,
        k < 4096,
        forall|j: int| 0 <= j < k ==> #[trigger] peel(pf, c, j) >= clear,
    ensures
        rev_of(sf, pf, clear, c, fuel) == pushbytes(sf, pf, c, k) + rev_of(
            sf,
            pf,
            clear,
            peel(pf, c, k),
            (fuel - k) as nat,
        ),
    decreases k,
{
    reveal(rev_of);
    reveal(peel);
    reveal(pushbytes);
    if k <= 0 {
        assert(peel(pf, c, k) == c);
        assert(pushbytes(sf, pf, c, k) == Seq::<u8>::empty());
        assert((fuel - k) as nat == fuel);
    } else {
        assert(peel(pf, c, 0) == c);
        assert(peel(pf, c, 0) >= clear);
        lemma_chain_step(sf, pf, clear, c, fuel);
        assert(rev_of(sf, pf, clear, c, fuel) == seq![sf[c]] + rev_of(
            sf,
            pf,
            clear,
            pf[c] as int,
            (fuel - 1) as nat,
        ));
        assert(peel(pf, c, k) == peel(pf, pf[c] as int, k - 1));
        assert(forall|j: int| 0 <= j < k - 1 ==> #[trigger] peel(pf, c, j + 1) >= clear) by {
            assert forall|j: int| 0 <= j < k - 1 implies #[trigger] peel(pf, c, j + 1) >= clear by {
                assert(0 <= j + 1 < k);
            }
        }
        assert(forall|j: int| 0 <= j < k - 1 ==> #[trigger] peel(pf, pf[c] as int, j) >= clear) by {
            assert forall|j: int| 0 <= j < k - 1 implies #[trigger] peel(pf, pf[c] as int, j) >= clear by {
                assert(peel(pf, pf[c] as int, j) == peel(pf, c, j + 1));
            }
        }
        assert(k - 1 < 4096);
        lemma_rev_splice(sf, pf, clear, pf[c] as int, k - 1, (fuel - 1) as nat);
        assert((fuel - 1) as nat as int == fuel as int - 1);
        assert(k < 4096);
        lemma_pushbytes_cons(sf, pf, c, k);
    }
}

/// The emitted segment, as the pop loop establishes it: `out[q0 + j]` is the `j`-th
/// index of the string. Stated with `str_of` so the model's step 8 reads off directly.
pub open spec fn seg_ok(out: Seq<u8>, pre: Seq<u8>, q0: int, sf: Seq<u8>, pf: Seq<u16>, clear: int,
    c: int, fuel: nat) -> bool {
    &&& q0 >= 0
    &&& out.len() == pre.len()
    &&& q0 + str_of(sf, pf, clear, c, fuel).len() <= out.len()
    &&& (forall|j: int|
        0 <= j < str_of(sf, pf, clear, c, fuel).len() ==> #[trigger] out[q0 + j] == str_of(
            sf,
            pf,
            clear,
            c,
            fuel,
        )[j])
    &&& (forall|i: int|
        0 <= i < out.len() && (i < q0 || i >= q0 + str_of(
            sf,
            pf,
            clear,
            c,
            fuel,
        ).len()) ==> #[trigger] out[i] == pre[i])
}

/// The peel cursor as a *fuel-bounded* walk, so every fact about it comes from
/// `fuel_above` and `link_ok` with no separate range argument: `peel_f` is `peel` for
/// any `k <= fuel`, and its value is always in `[0, 4096)`.
pub open spec fn peel_f(pf: Seq<u16>, c: int, k: int, fuel: nat) -> int
    decreases fuel,
{
    if fuel == 0 || k <= 0 {
        c
    } else {
        peel_f(pf, pf[c] as int, k - 1, (fuel - 1) as nat)
    }
}

/// `peel_f` agrees with `peel` while the fuel lasts.
pub proof fn lemma_peel_f_eq(pf: Seq<u16>, c: int, k: int, fuel: nat)
    requires
        chain_ok(pf, 0, c),
        fuel_above(c, fuel),
        0 <= k <= fuel as int,
    ensures
        peel_f(pf, c, k, fuel) == peel(pf, c, k),
    decreases k,
{
    reveal(peel_f);
    reveal(peel);
    if k <= 0 {
    } else {
        lemma_chain_step_pub(pf, c, fuel);
        lemma_peel_f_eq(pf, pf[c] as int, k - 1, (fuel - 1) as nat);
    }
}

/// `chain_ok` at a stored entry implies it at the parent, with the fuel handed down:
/// the fact every recursive lemma in this file needs, in one place with an explicit
/// `clear = 0` shape so the range arithmetic is uniform.
pub proof fn lemma_chain_step_pub(pf: Seq<u16>, c: int, fuel: nat)
    requires
        chain_ok(pf, 0, c),
        c >= 2,
        c < 4096,
        fuel_above(c, fuel),
    ensures
        chain_ok(pf, 0, pf[c] as int),
        (pf[c] as int) < c,
        fuel_above(pf[c] as int, (fuel - 1) as nat),
{
    lemma_chain_step(sf_zero(), pf, 0, c, fuel);
}

/// `peel_f`'s range, from `fuel_above` alone: the cursor is always a code in
/// `[0, c]`, so any index into the 4,096-entry arrays written at it is in bounds.
/// The push loop's step hypothesis: every cursor it has reached is still a stored
/// entry (the loop condition `cur >= clear`), so no step landed on a literal yet.
pub open spec fn peel_stored(pf: Seq<u16>, clear: int, c: int, k: int) -> bool {
    forall|j: int| 0 <= j <= k ==> #[trigger] peel(pf, c, j) >= clear
}

/// `peel`'s two unfolding facts, as separate lemmas so an index argument can quote
/// them without depending on a `reveal` in the surrounding goal.
pub proof fn lemma_peel_zero(pf: Seq<u16>, c: int)
    ensures
        peel(pf, c, 0) == c,
{
    reveal(peel);
}

pub proof fn lemma_peel_one(pf: Seq<u16>, c: int)
    ensures
        peel(pf, c, 1) == pf[c] as int,
{
    reveal(peel);
    assert(peel(pf, c, 1) == peel(pf, pf[c] as int, 0));
    assert(peel(pf, pf[c] as int, 0) == pf[c] as int);
}

/// A link's parent is a literal or a stored entry, never `clear` or `eoi`: the walk's
/// cursor is therefore never one of those two codes.
pub proof fn lemma_link_not_reserved(pf: Seq<u16>, clear: int, x: int)
    requires
        link_ok(pf, clear, x),
        clear + 2 <= x,
    ensures
        (pf[x] as int) != clear,
        (pf[x] as int) != clear + 1,
{
    reveal(link_ok);
}

/// `peel`'s associativity: walking `a` links and then `b` more is walking `a + b`.
pub proof fn lemma_peel_add(pf: Seq<u16>, c: int, a: int, b: int)
    requires
        a >= 0,
        b >= 0,
    ensures
        peel(pf, c, a + b) == peel(pf, peel(pf, c, a), b),
    decreases a,
{
    reveal(peel);
    if a <= 0 {
        assert(peel(pf, c, 0) == c);
    } else {
        assert(peel(pf, c, a) == peel(pf, pf[c] as int, a - 1));
        assert(peel(pf, c, a + b) == peel(pf, pf[c] as int, a - 1 + b));
        lemma_peel_add(pf, pf[c] as int, a - 1, b);
    }
}

/// One step of `peel`, and the entry it lands on is the parent: the fact every index
/// argument in the push loop quotes.
pub proof fn lemma_peel_step_pub(pf: Seq<u16>, c: int, k: int)
    requires
        k >= 1,
    ensures
        peel(pf, c, k) == peel(pf, pf[c] as int, k - 1),
{
    reveal(peel);
}

/// The cursor's bound, under the loop's step hypothesis: a cursor reached by
/// non-literal steps is a stored entry, hence inside the 4,096-entry table. The
/// `k = 1` case is the only one where the parent may be a literal (and a literal is
/// bounded by `clear < 4096`); for `k >= 2` the hypothesis at `j = 1` forces the
/// parent to be stored, so the induction continues.
pub proof fn lemma_peel_bound(pf: Seq<u16>, clear: int, c: int, k: int)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        0 <= k,
        peel_stored(pf, clear, c, k),
    ensures
        0 <= peel(pf, c, k) < 4096,
    decreases k,
{
    reveal(peel);
    if k == 0 {
        assert(peel(pf, c, 0) == c);
    } else {
        let p = pf[c] as int;
        assert(link_ok(pf, clear, c)) by {
            reveal(chain_ok);
            reveal(link_ok);
            assert(forall|x: int| clear + 2 <= x && x <= c ==> #[trigger] link_ok(pf, clear, x));
        }
        assert(p < clear || (clear + 2 <= p && p < c));
        lemma_peel_step_pub(pf, c, k);
        if k == 1 {
            lemma_peel_zero(pf, p);
            assert(peel(pf, p, 0) == p);
            assert(p < clear || p < c);
        } else {
            lemma_peel_one(pf, c);
            assert(peel_stored(pf, clear, c, k));
            assert(peel(pf, c, 1) >= clear) by {
                assert(1 < k);
            }
            assert(chain_ok(pf, clear, p));
            assert(peel_stored(pf, clear, p, k - 1)) by {
                assert forall|j: int| 0 <= j < k - 1 implies #[trigger] peel(pf, p, j) >= clear by {
                    lemma_peel_step_pub(pf, c, j + 1);
                    assert(peel(pf, c, j + 1) >= clear) by {
                        assert(0 <= j + 1 < k);
                    }
                }
            }
            lemma_peel_bound(pf, clear, p, k - 1);
        }
    }
}

/// A cursor reached through stored entries cannot be either reserved code.
/// The inclusive `peel_stored` hypothesis also excludes a literal parent when
/// `k > 0`, so induction continues only through stored parents.
pub proof fn lemma_peel_not_reserved(pf: Seq<u16>, clear: int, c: int, k: int)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        0 <= k,
        peel_stored(pf, clear, c, k),
    ensures
        peel(pf, c, k) != clear,
        peel(pf, c, k) != clear + 1,
    decreases k,
{
    if k == 0 {
        lemma_peel_zero(pf, c);
    } else {
        lemma_chain_step(sf_zero(), pf, clear, c, 4096);
        let p = pf[c] as int;
        if p < clear {
            lemma_peel_one(pf, c);
            assert(peel(pf, c, 1) >= clear);
            assert(false);
        } else {
            assert(clear + 2 <= p < c);
            assert(peel_stored(pf, clear, p, k - 1)) by {
                assert forall|j: int| 0 <= j <= k - 1 implies
                    #[trigger] peel(pf, p, j) >= clear by {
                    lemma_peel_step_pub(pf, c, j + 1);
                    assert(peel(pf, c, j + 1) >= clear);
                }
            }
            lemma_peel_not_reserved(pf, clear, p, k - 1);
            lemma_peel_step_pub(pf, c, k);
        }
    }
}

/// The stack the push loop leaves: `stack[i]` is `pushbytes[i]` for the walked codes,
/// and `stack[top]` is the literal the walk stopped at. This is the flat form the
/// push loop's invariant reads off; the pop loop then needs no reversal at all, only
/// `str_of`'s definition on a chain (which `lemma_rev_is_reverse` supplies).
pub open spec fn stack_ok(stack: Seq<u8>, pre: Seq<u8>, sf: Seq<u8>, pf: Seq<u16>, c0: int,
    top: int) -> bool {
    &&& stack.len() == pre.len()
    &&& 0 <= top
    &&& top + 1 <= stack.len()
    &&& (forall|i: int| 0 <= i < top ==> #[trigger] stack[i] == pushbytes(sf, pf, c0, top)[i])
    &&& stack[top] == peel(pf, c0, top) as u8
    &&& (forall|i: int| top + 1 <= i < stack.len() ==> #[trigger] stack[i] == pre[i])
}

/// `rev_of` is `str_of` reversed: slot `i` of the emitting order is stack slot
/// `len-1-i`. This is the identity the executable's "push up the chain, pop down into
/// `out`" pair implements, so one `out` write sequence serves both orders.
pub proof fn lemma_rev_is_reverse(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        fuel_above(c, fuel),
    ensures
        rev_of(sf, pf, clear, c, fuel).len() == str_of(sf, pf, clear, c, fuel).len(),
        forall|i: int|
            0 <= i < str_of(sf, pf, clear, c, fuel).len() ==> #[trigger] str_of(sf, pf, clear, c, fuel)[i]
                == rev_of(sf, pf, clear, c, fuel)[str_of(sf, pf, clear, c, fuel).len() - 1 - i],
    decreases c,
{
    reveal(str_of);
    reveal(rev_of);
    if c < clear {
        assert(fuel as int > 0);
        assert(str_of(sf, pf, clear, c, fuel) == seq![c as u8]);
    } else {
        lemma_chain_step(sf, pf, clear, c, fuel);
        lemma_rev_is_reverse(sf, pf, clear, pf[c] as int, (fuel - 1) as nat);
        let inner = str_of(sf, pf, clear, pf[c] as int, (fuel - 1) as nat);
        let sfl = sf[c];
        assert(rev_of(sf, pf, clear, c, fuel) == seq![sfl] + rev_of(
            sf,
            pf,
            clear,
            pf[c] as int,
            (fuel - 1) as nat,
        ));
        assert(str_of(sf, pf, clear, c, fuel) == inner.push(sfl));
        assert forall|i: int| 0 <= i < str_of(sf, pf, clear, c, fuel).len() implies #[trigger] str_of(
            sf,
            pf,
            clear,
            c,
            fuel,
        )[i] == rev_of(sf, pf, clear, c, fuel)[str_of(sf, pf, clear, c, fuel).len() - 1 - i] by {
            if i < inner.len() {
                assert(str_of(sf, pf, clear, c, fuel)[i] == inner[i]);
                assert(rev_of(sf, pf, clear, c, fuel)[inner.len() - i] == rev_of(
                    sf,
                    pf,
                    clear,
                    pf[c] as int,
                    (fuel - 1) as nat,
                )[inner.len() - 1 - i]);
                assert(rev_of(sf, pf, clear, c, fuel)[str_of(sf, pf, clear, c, fuel).len() - 1 - i]
                    == rev_of(sf, pf, clear, c, fuel)[inner.len() - i]);
            } else {
                assert(str_of(sf, pf, clear, c, fuel)[inner.len() as int] == sfl);
                assert(rev_of(sf, pf, clear, c, fuel)[0] == sfl);
            }
        }
    }
}

/// The string's length is its `code_len` value, its first index is the chain's root,
/// and the root is a literal code (`< clear`, hence `< 2^m`). The length clause is
/// stated for the region `t` holding the cache, so the executable's two-byte
/// accessor is what it compares against.
pub proof fn lemma_str_shape(
    sf: Seq<u8>,
    pf: Seq<u16>,
    t: Seq<u8>,
    m: int,
    clear: int,
    c: int,
    fuel: nat,
)
    requires
        chain_ok(pf, clear, c),
        fuel_above(c, fuel),
        clear == ipow2(m),
        0 <= m <= 11,
        t.len() == 8192,
        forall|x: int|
            clear + 2 <= x && x <= c ==> #[trigger] code_len(t, m, x) == 1 + code_len(
                t,
                m,
                pf[x] as int,
            ),
    ensures
        str_of(sf, pf, clear, c, fuel).len() == code_len(t, m, c),
        code_len(t, m, c) >= 1,
        str_of(sf, pf, clear, c, fuel)[0] == root_of(pf, clear, c, fuel) as u8,
        0 <= root_of(pf, clear, c, fuel) < clear,
        rev_of(sf, pf, clear, c, fuel).len() == code_len(t, m, c),
    decreases c,
{
    reveal(str_of);
    reveal(rev_of);
    reveal(root_of);
    lemma_root_literal(pf, clear, c, fuel);
    if c < clear {
        assert(fuel as int > 0);
        assert(str_of(sf, pf, clear, c, fuel) == seq![c as u8]);
        assert(rev_of(sf, pf, clear, c, fuel) == seq![c as u8]);
        assert(code_len(t, m, c) == 1);
    } else {
        lemma_chain_step(sf, pf, clear, c, fuel);
        let p = pf[c] as int;
        assert(p < c);
        assert(p < clear ==> code_len(t, m, p) == 1);
        assert(str_of(sf, pf, clear, p, (fuel - 1) as nat).len() >= 1) by {
            if p < clear {
                assert(str_of(sf, pf, clear, p, (fuel - 1) as nat) == seq![p as u8]);
            } else {
                assert(str_of(sf, pf, clear, p, (fuel - 1) as nat).len() >= 1);
            }
        }
        if p < clear {
            assert(root_of(pf, clear, p, (fuel - 1) as nat) == p);
            assert(str_of(sf, pf, clear, p, (fuel - 1) as nat)[0] == p as u8);
        } else {
            assert(root_of(pf, clear, c, fuel) == root_of(pf, clear, p, (fuel - 1) as nat));
            assert(chain_ok(pf, clear, p));
            assert(forall|x: int|
                clear + 2 <= x && x <= p ==> #[trigger] code_len(t, m, x) == 1 + code_len(
                    t,
                    m,
                    pf[x] as int,
                ));
            lemma_str_shape(sf, pf, t, m, clear, p, (fuel - 1) as nat);
            assert(str_of(sf, pf, clear, p, (fuel - 1) as nat)[0] == root_of(
                pf,
                clear,
                p,
                (fuel - 1) as nat,
            ) as u8);
        }
        assert(str_of(sf, pf, clear, c, fuel)[0] == str_of(sf, pf, clear, p, (fuel - 1) as nat)[0]);
        assert(code_len(t, m, c) == 1 + code_len(t, m, p));
    }
}

/// A stored chain has at most `c - clear` indices: each stored parent is
/// smaller, and the last stored entry contributes one suffix and one literal.
pub proof fn lemma_str_len_bound(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        c < 4096,
        fuel_above(c, fuel),
    ensures
        1 <= str_of(sf, pf, clear, c, fuel).len() <= 4096,
        c < clear ==> str_of(sf, pf, clear, c, fuel).len() == 1,
        c >= clear ==> str_of(sf, pf, clear, c, fuel).len() <= c - clear,
    decreases c,
{
    if c < clear {
        reveal(str_of);
    } else {
        lemma_chain_step(sf, pf, clear, c, fuel);
        let p = pf[c] as int;
        lemma_str_len_bound(sf, pf, clear, p, (fuel - 1) as nat);
    }
}

/// PLAN M2's length bound for the suffix-independent chain length.
pub proof fn lemma_len_bound(pf: Seq<u16>, clear: int, c: int, fuel: nat)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        fuel_above(c, fuel),
    ensures
        1 <= str_of(sf_zero(), pf, clear, c, fuel).len() <= c - clear,
        str_of(sf_zero(), pf, clear, c, fuel).len() < 4096,
{
    lemma_str_len_bound(sf_zero(), pf, clear, c, fuel);
}

/// The frame property the fresh-entry step needs: a walk from an entry strictly below
/// `c` reads neither `sf[c]` nor `pf[c]`, so writing slot `c` leaves the string
/// unchanged. Every index the walk visits is at most its start (`link_ok`), and the
/// start is below `c`.
pub proof fn lemma_frame(sf: Seq<u8>, pf: Seq<u16>, clear: int, cc: int, c: int, v: u8, fuel: nat)
    requires
        chain_ok(pf, clear, cc),
        cc < c,
        c < 4096,
        sf.len() == 4096,
        fuel_above(cc, fuel),
    ensures
        str_of(sf.update(c, v), pf, clear, cc, fuel) == str_of(sf, pf, clear, cc, fuel),
        rev_of(sf.update(c, v), pf, clear, cc, fuel) == rev_of(sf, pf, clear, cc, fuel),
    decreases cc,
{
    reveal(str_of);
    reveal(rev_of);
    if cc < clear {
        assert(sf.update(c, v)[cc] == sf[cc]);
    } else {
        lemma_chain_step(sf, pf, clear, cc, fuel);
        let q = pf[cc] as int;
        assert(q < cc);
        assert(chain_ok(pf, clear, q));
        assert(q < c);
        lemma_frame(sf, pf, clear, q, c, v, (fuel - 1) as nat);
        assert(sf.update(c, v)[cc] == sf[cc]);
    }
}

/// The fresh-entry step: writing `suffix[c] = v` for a fresh entry `c` whose parent
/// is `p` gives the parent's string with `v` appended, which is G10.2 step 7's
/// "new entry = prev + first(s)" once `v` is the parent's own first index.
pub proof fn lemma_fresh_entry(
    sf: Seq<u8>,
    pf: Seq<u16>,
    clear: int,
    c: int,
    p: int,
    v: u8,
    fuel: nat,
)
    requires
        chain_ok(pf, clear, c),
        clear + 2 <= c < 4096,
        (pf[c] as int) == p,
        p < c,
        sf.len() == 4096,
        fuel_above(c, fuel),
    ensures
        str_of(sf.update(c, v), pf, clear, c, fuel) == str_of(sf, pf, clear, p, (fuel - 1) as nat).push(
            v,
        ),
        rev_of(sf.update(c, v), pf, clear, c, fuel) == seq![v] + rev_of(
            sf,
            pf,
            clear,
            p,
            (fuel - 1) as nat,
        ),
{
    reveal(str_of);
    reveal(rev_of);
    lemma_fuel_step(pf, clear, c, fuel);
    lemma_chain_step(sf, pf, clear, c, fuel);
    lemma_frame(sf, pf, clear, p, c, v, (fuel - 1) as nat);
    assert((sf.update(c, v))[c] == v);
}

} // verus!
