//! Verified P5 duplicate/order/fixed-length preamble used by the real walk.
//! CRC policy selection precedes this stage; per-chunk values follow it.
use vstd::prelude::*;
use crate::kind::{Kind, seen_contains};
use crate::png::Error;
#[cfg(verus_only)]
use crate::{kind::seen_view, walk_model as mw};

verus! {

#[derive(Clone, Copy)]
pub(crate) struct Rules {
    pub first_duplicate_wins: bool,
    pub drop_misordered: bool,
    pub drop_invalid: bool,
    pub ignore_unexpected_plte: bool,
    pub lenient_iend: bool,
}
impl Rules {
    pub(crate) open spec fn view(self) -> mw::Rules {
        mw::Rules { first_duplicate_wins: self.first_duplicate_wins,
            drop_misordered: self.drop_misordered, drop_invalid: self.drop_invalid,
            ignore_unexpected_plte: self.ignore_unexpected_plte, lenient_iend: self.lenient_iend }
    }
}

pub(crate) enum OrderAction { Keep(usize), Drop, Fail(Error, bool) }
impl OrderAction {
    pub(crate) open spec fn view(self) -> mw::OrderAction {
        match self {
            Self::Keep(n) => mw::OrderAction::Keep(n as nat),
            Self::Drop => mw::OrderAction::Drop,
            Self::Fail(e, structural) => mw::OrderAction::Fail(e, structural),
        }
    }
}

pub(crate) open spec fn phase_view(idat: u8) -> mw::Phase {
    if idat == 0 { mw::Phase::Before }
    else if idat == 1 { mw::Phase::During }
    else { mw::Phase::After }
}

/// Update the phase even when a following CRC-policy branch discards the
/// chunk. In particular, a dropped ancillary chunk still ends an IDAT run.
pub(crate) fn advance_phase(idat: &mut u8, kind: Kind)
    ensures *final(idat) == if *old(idat) == 1 && kind != Kind::Idat { 2u8 } else { *old(idat) },
        *old(idat) <= 2 ==> *final(idat) <= 2,
        phase_view(*final(idat)) == mw::advance_phase(phase_view(*old(idat)), kind.view()),
{
    if *idat == 1 && !matches!(kind, Kind::Idat) { *idat = 2; }
}

pub(crate) fn preamble(seen: u32, idat: u8, plte_seen: bool, ct: u8,
    kind: Kind, ancillary: bool, length: usize, rules: Rules) -> (r: OrderAction)
    ensures r.view() == mw::order(seen_view(seen), phase_view(idat), plte_seen, ct,
        kind.view(), ancillary, length as nat, rules.view()),
        match r { OrderAction::Keep(n) => n <= length, _ => true },
{
    if seen_contains(seen, kind) {
        if ancillary && rules.first_duplicate_wins { return OrderAction::Drop; }
        if matches!(kind, Kind::Plte) && rules.ignore_unexpected_plte && (ct == 0 || ct == 4) {
            return OrderAction::Drop;
        }
        return OrderAction::Fail(Error::DuplicateChunk, kind.is_apng());
    }
    let misplaced = (kind.before_plte() && (plte_seen || idat != 0))
        || (kind.after_plte() && (idat != 0 || (!plte_seen && (ct == 3 || matches!(kind, Kind::Hist)))))
        || (kind.before_idat() && idat != 0);
    if misplaced {
        if matches!(kind, Kind::Actl) { return OrderAction::Fail(Error::ChunkOutOfOrder, true); }
        if rules.drop_misordered { return OrderAction::Drop; }
        return OrderAction::Fail(Error::ChunkOutOfOrder, false);
    }
    let mut n = length;
    if matches!(kind, Kind::Iend) && n != 0 && rules.lenient_iend { n = 0; }
    if let Some(expected) = kind.fixed_len() {
        if n != expected {
            if ancillary && !matches!(kind, Kind::Actl) && rules.drop_invalid { return OrderAction::Drop; }
            return OrderAction::Fail(Error::InvalidChunkLength, matches!(kind, Kind::Actl));
        }
    }
    OrderAction::Keep(n)
}

} // verus!
