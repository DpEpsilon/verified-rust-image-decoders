//! G10's reference model `ML` (SPEC draft 0.2), inside `verus!`: the index sequence
//! a conforming LZW decoder emits, the first error if any, the final bit position,
//! and the E4 flag. Errors retain the prefix emitted before the failing check.
//!
//! The model is a pure function of the concatenated image data `D` (G9's
//! de-sub-blocked payload), the minimum code size `m` and the frame area `A`; a
//! `Params` record carries every G9/G10 policy switch, so Strict and RecommendedBeta
//! instantiate one function rather than two (nothing here hard-codes RecommendedBeta).
//!
//! Table representation (the coordinator's D20 ruling and Sol's `code_len` note).
//! The model's table *is* the executable's: two parallel arrays `pf` (parent codes)
//! and `sf` (suffix indices), with the strings defined by `crate::chain`'s walk. A
//! code below `clear` is a literal by rule, so no `2^m` literal entry is ever
//! written and the arrays need no initialisation ramp; entries are written only in
//! `[clear + 2, next)`. `chain::str_of`/`rev_of`/`root_of` are the string, its stack
//! order and its first index, and `chain::code_len` is the length the model's
//! recurrence fixes and the executable's `length` region caches.
//!
//! Because the model and the executable share the representation, T-LZW's statement
//! is a *fold identity*: one executable iteration is one `Vd` step of `L`, and the
//! emitted bytes are the `o` the fold accumulates. The chain lemmas supply the
//! string facts; nothing needs a `Seq<Seq<u8>>`-to-arrays bridge.
//!
//! Bit positions (a choice the SPEC leaves open; `gif_oracle.py` returns bare error
//! names). The position is the number of code bits consumed when the failing check
//! runs: the pre-read exhaustion error (step 1) reports the position *before* the
//! failed read, and every error raised after a code is decoded (steps 4, 5, 6, 8 and
//! the checked-literal rule) reports the position *after* it.

use vstd::prelude::*;

#[cfg(verus_only)]
use crate::chain::{root_of, str_of};
#[cfg(verus_only)]
use crate::lzw::Params;

verus! {

// ---------------------------------------------------------------------------
// The closed error kinds of G10.2 (G16's names) and the model's result type.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrK {
    InvalidLzwMinCodeSize,
    InvalidLzwCode,
    TooManyPixels,
    TooFewPixels,
    MissingEndOfInformation,
    DataAfterEndOfInformation,
    ColorIndexOutOfRange,
}

/// The complete observable result, including the prefix emitted before an error.
/// `e4` records discarded indices of a raster-crossing string under StopAtRaster.
pub struct MRes {
    pub idx: Seq<u8>,
    pub error: Option<ErrK>,
    pub pos: int,
    pub e4: bool,
}

pub open spec fn observation(idx: Seq<u8>, error: Option<ErrK>, pos: int, e4: bool) -> MRes {
    MRes { idx, error, pos, e4 }
}

// ---------------------------------------------------------------------------
// Bits and codes (G10's preamble).
// ---------------------------------------------------------------------------

/// `2^m` for `m <= 16`, by recursion on `m` (`m <= 11` here, so `ipow2(m) <= 2048`).
pub open spec fn ipow2(m: int) -> int
    decreases m,
{
    if m <= 0 {
        1
    } else {
        2 * ipow2(m - 1)
    }
}

/// G10: bit `8n+j` of the payload is bit `j` of byte `n`.
pub open spec fn bit_at(d: Seq<u8>, j: int) -> int {
    ((d[j / 8] >> (j % 8) as u8) & 1u8) as int
}

/// The code in `width` bits at bit position `pos`, least significant bit first.
pub open spec fn code_at(d: Seq<u8>, pos: int, width: int, i: int) -> int
    decreases width - i,
{
    if i >= width {
        0
    } else {
        bit_at(d, pos + i) + 2 * code_at(d, pos, width, i + 1)
    }
}

pub open spec fn nbits(d: Seq<u8>) -> int {
    8 * (d.len() as int)
}

/// G10.1's `clear` and `eoi`.
pub open spec fn clear_of(m: int) -> int {
    ipow2(m)
}

pub open spec fn eoi_of(m: int) -> int {
    ipow2(m) + 1
}

/// The table capacity (G10.2 step 7's `4096`).
pub open spec fn NTAB() -> int {
    4096
}

/// G10.2 step 5's tail test: whole bytes after the byte holding the End of
/// Information code's last bit.
pub open spec fn eoi_tail(after: int, d: Seq<u8>) -> bool {
    (after + 7) / 8 < d.len() as int
}

/// G10.2's step 6's test: a string exists for `code`, searched after the `clear` and
/// `eoi` branches have taken their codes. A literal is always resolvable; a stored
/// entry is `[clear + 2, next)`; the KwKwK case needs a previous string and room for
/// the new entry. The last branch is also the only one that admits
/// `code = next = 4096`, so the deferred clear needs no case split.
pub open spec fn str_ok(m: int, next: int, prev: int, code: int) -> bool {
    if code < clear_of(m) {
        true
    } else if code < next {
        clear_of(m) + 2 <= code
    } else {
        code == next && prev >= 0 && next < NTAB()
    }
}

/// G10.2 step 7's widening test, as the width after the table reaches `n`.
pub open spec fn widen(width: int, n: int) -> int {
    if width < 12 && n == ipow2(width) {
        width + 1
    } else {
        width
    }
}

// ---------------------------------------------------------------------------
// G10.1/G10.2 state.
// ---------------------------------------------------------------------------

/// G10.1's mutable state. `prev` is a *code* (`-1` is G10.1's `none`) and the string
/// it denotes stays in the table, exactly as the executable keeps it. `o` accumulates
/// the emitted indices, so the loop is a tail-recursive fold and one step's equation
/// `L(s) = L(step(s))` is exact, emitted output included.
pub struct St {
    pub pos: int,
    pub width: int,
    pub next: int,
    pub prev: int,
    pub q: int,
    pub pf: Seq<u16>,
    pub sf: Seq<u8>,
    pub o: Seq<u8>,
}

/// G10.1's initial state: `next = clear+2`, `width = m+1`, `prev = none`, `q = 0`.
/// The canonical model starts with zero arrays. The executable may receive stale
/// bytes from an earlier frame; its readable-range agreement invariant makes every
/// initial byte irrelevant until the frame overwrites its dictionary entry (D20).
pub open spec fn init_st(m: int) -> St {
    St {
        pos: 0,
        width: m + 1,
        next: clear_of(m) + 2,
        prev: -1,
        q: 0,
        pf: Seq::new(NTAB() as nat, |_i: int| 0u16),
        sf: Seq::new(NTAB() as nat, |_i: int| 0u8),
        o: Seq::empty(),
    }
}

/// G10.2's Clear reset (step 3). The arrays are *not* re-zeroed: `next` returns to
/// `clear + 2`, which makes every entry added since the last Clear unreadable, and
/// `q`, `o` and the position survive.
pub open spec fn clear_st(m: int, st: St, after: int) -> St {
    St {
        pos: after,
        width: m + 1,
        next: clear_of(m) + 2,
        prev: -1,
        q: st.q,
        pf: st.pf,
        sf: st.sf,
        o: st.o,
    }
}

/// G10.2's step 6: the string `s` a code denotes. `code < clear` is a literal,
/// `code < next` a stored entry, else the KwKwK case `prev + first(prev)`.
pub open spec fn semit(m: int, st: St, code: int) -> Seq<u8> {
    if code < st.next {
        str_of(st.sf, st.pf, clear_of(m), code, fuel_of())

    } else {
        str_of(st.sf, st.pf, clear_of(m), st.prev, fuel_of()).push(
            str_of(st.sf, st.pf, clear_of(m), st.prev, fuel_of())[0],
        )
    }
}

/// The walk's budget: every index in the arrays is below 4,096, so `fuel_of()` is
/// above every code and `chain::fuel_above` holds at every entry.
pub open spec fn fuel_of() -> nat {
    4096
}

/// G10.2's step 7 suffix: the first index of the resolved string `s`.
pub open spec fn first_of(m: int, st: St, code: int) -> u8 {
    if code < st.next {
        root_of(st.pf, clear_of(m), code, fuel_of()) as u8
    } else {
        root_of(st.pf, clear_of(m), st.prev, fuel_of()) as u8
    }
}

/// The state after emitting the string of `code`: G10.2's steps 7 and 8. `grow` is
/// step 7's guard (`prev != none && next < 4096`), so `next`, the width and the
/// arrays change only when an entry is added, while `prev`, `q` and `o` always advance.
pub open spec fn emit_st(m: int, st: St, after: int, code: int, s: Seq<u8>, v: u8, grow: bool) -> St {
    let e = st.next;
    St {
        pos: after,
        width: if grow { widen(st.width, e + 1) } else { st.width },
        next: if grow { e + 1 } else { e },
        prev: code,
        q: st.q + s.len(),
        pf: if grow { st.pf.update(e, st.prev as u16) } else { st.pf },
        sf: if grow { st.sf.update(e, v) } else { st.sf },
        o: st.o + s,
    }
}

/// G10.2's loop control: halt with an error, halt accepted at a final state, or
/// advance to a successor state.
pub enum Vd {
    Halt(MRes),
    Step(St),
}

/// G9's minimum-code-size check, reported at bit position 0 (it precedes the codes).
pub open spec fn m_ok(ps: Params, m: int, a: int) -> bool {
    &&& ps.m_lo as int <= m <= ps.m_hi as int
    &&& (!(ps.size1_only_one_pixel && m <= 1) || a == 1)
}

/// One step of G10.2, in G10.2's written order of checks.
pub open spec fn verdict(m: int, d: Seq<u8>, a: int, ps: Params, st: St) -> Vd {
    if ps.stop_at_raster && st.q == a {
        // `lzw_tail = StopAtRaster` (RecommendedBeta): the raster is complete.
        Vd::Halt(observation(st.o, None, st.pos, false))
    } else if st.pos + st.width > nbits(d) {
        // Step 1: fewer than `width` bits remain.
        Vd::Halt(observation(st.o, Some(if st.q < a { ErrK::TooFewPixels } else { ErrK::MissingEndOfInformation }), st.pos, false))
    } else {
        let code = code_at(d, st.pos, st.width, 0);
        let after = st.pos + st.width;
        if code == clear_of(m) {
            // Step 3: Clear.
            Vd::Step(clear_st(m, st, after))
        } else if st.q == a && code != eoi_of(m) {
            // Step 4 precedes the checked-literal rule, including custom policies.
            Vd::Halt(observation(st.o, Some(ErrK::MissingEndOfInformation), after, false))
        } else if code == eoi_of(m) {
            // Step 5.
            if st.q < a {
                Vd::Halt(observation(st.o, Some(ErrK::TooFewPixels), after, false))
            } else if !ps.stop_at_raster && eoi_tail(after, d) {
                Vd::Halt(observation(st.o, Some(ErrK::DataAfterEndOfInformation), after, false))
            } else {
                Vd::Halt(observation(st.o, None, after, false))
            }
        } else if ps.checked_literals && code < clear_of(m) && code >= 256 {
            Vd::Halt(observation(st.o, Some(ErrK::ColorIndexOutOfRange), after, false))
        } else if str_ok(m, st.next, st.prev, code) {
            // Steps 6 to 8. A raster-crossing string emits its prefix through A.
            // Strict then reports TooManyPixels; StopAtRaster discards the excess
            // indices, records E4 and succeeds at this code's final bit position.
            let s = semit(m, st, code);
            if st.q + s.len() > a {
                Vd::Halt(observation(st.o + s.take(a - st.q),
                    if ps.stop_at_raster { None } else { Some(ErrK::TooManyPixels) },
                    after, ps.stop_at_raster))
            } else {
                let grow = st.prev >= 0 && st.next < NTAB();
                Vd::Step(emit_st(m, st, after, code, s, first_of(m, st, code), grow))
            }
        } else {
            // Step 6's failure. The deferred clear's `code = next = 4096` is
            // unreadable (`width` is then 12), so it is `InvalidLzwCode` too.
            Vd::Halt(observation(st.o, Some(ErrK::InvalidLzwCode), after, false))
        }
    }
}

/// G10.2's loop from `st`. The measure is PLAN M2's variant `8*|D| - pos`: every
/// advancing step consumes `width` bits of the finite `D`, so it strictly decreases.
pub open spec fn L(m: int, d: Seq<u8>, a: int, ps: Params, st: St) -> MRes
    decreases nbits(d) - st.pos,
{
    match verdict(m, d, a, ps, st) {
        Vd::Halt(r) => r,
        // The guard is the termination measure, not a G10.2 condition: an advancing
        // step always leaves `pos` larger (`lemma_L_step_pos`), so the `else` value is
        // unreachable and `L` is the plain fold. It is written as a guard because the
        // successor position is a field `verdict` does not expose as an expression,
        // and Verus must see the measure decrease at the recursive call.
        Vd::Step(st2) => if st2.pos > st.pos {
            L(m, d, a, ps, st2)
        } else {
            observation(st.o, Some(ErrK::InvalidLzwCode), st.pos, false)
        },
    }
}

/// Expose exactly one fold equation without recursively unfolding the successor.
pub proof fn lemma_L_unfold(m:int,d:Seq<u8>,a:int,ps:Params,st:St)
    ensures L(m,d,a,ps,st)==match verdict(m,d,a,ps,st) {
        Vd::Halt(r)=>r,
        Vd::Step(ns)=>if ns.pos>st.pos {L(m,d,a,ps,ns)}
            else {observation(st.o,Some(ErrK::InvalidLzwCode),st.pos,false)},
    },
{reveal(L);}

/// Every advancing step consumes the current code, so the guard above holds at every
/// step G10.2 can reach.
pub proof fn lemma_L_step_pos(m: int, d: Seq<u8>, a: int, ps: Params, st: St)
    ensures
        forall|st2: St|
            #[trigger] verdict(m, d, a, ps, st) == Vd::Step(st2) ==> st2.pos == st.pos
                + st.width,
{
    reveal(verdict);
    reveal(clear_st);
    reveal(emit_st);
    assert forall|st2: St| #[trigger] verdict(m, d, a, ps, st) == Vd::Step(st2) implies st2.pos
        == st.pos + st.width by {
        let after = st.pos + st.width;
        if !(ps.stop_at_raster && st.q == a) && st.pos + st.width <= nbits(d) {
            let code = code_at(d, st.pos, st.width, 0);
            if code == clear_of(m) {
                assert(st2 == clear_st(m, st, after));
            } else if str_ok(m, st.next, st.prev, code) && st.q + semit(m, st, code).len() <= a
                && !(ps.checked_literals && code < clear_of(m) && code >= 256)
                && !(st.q == a && code != eoi_of(m)) && code != eoi_of(m) {
                let s = semit(m, st, code);
                let grow = st.prev >= 0 && st.next < NTAB();
                assert(st2 == emit_st(m, st, after, code, s, first_of(m, st, code), grow));
            }
        }
    }
}

/// L-FULL: every advancing non-Clear step at a full table preserves the
/// dictionary and 12-bit width. The executable's width invariant establishes
/// width=12 whenever m>=2 and next=4096.
pub proof fn lemma_full_step(m:int,d:Seq<u8>,a:int,ps:Params,st:St,ns:St)
    requires st.next==4096, st.width==12,
        code_at(d,st.pos,st.width,0)!=clear_of(m),
        verdict(m,d,a,ps,st)==Vd::Step(ns),
    ensures ns.next==4096, ns.width==12, ns.pf==st.pf, ns.sf==st.sf,
{
    reveal(verdict); reveal(emit_st);
}

/// G10's `ML`. The caller supplies `A >= 1` (SPEC G9 rejects a zero-area frame before
/// G10); the size-1 rule is `m_ok`'s second conjunct, and the code-size range comes
/// from the policy record.
pub open spec fn ML(m: int, d: Seq<u8>, a: int, ps: Params) -> MRes {
    if !m_ok(ps, m, a) {
        observation(Seq::empty(), Some(ErrK::InvalidLzwMinCodeSize), 0, false)
    } else {
        L(m, d, a, ps, init_st(m))
    }
}

} // verus!
