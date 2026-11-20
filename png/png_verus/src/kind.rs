//! Executable chunk tags and the singleton-mask representation of P5 sets.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::walk_model as mw;

verus! {

/// The chunk types the walk distinguishes. `Unknown(ancillary)` keeps bit 5 of
/// byte 0 (P3, P5.5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    Ihdr, Plte, Idat, Iend, Trns, Chrm, Gama, Iccp, Sbit, Srgb, Cicp, Mdcv, Clli, Bkgd, Hist, Phys, Splt,
    Exif, Time, Text, Ztxt, Itxt, Actl, Fctl, Fdat, Unknown(bool),
}

fn type_is(t: &[u8; 4], a: u8, b: u8, c: u8, d: u8) -> (r: bool)
    ensures r == (t@ == seq![a, b, c, d]),
{
    let r = t[0] == a && t[1] == b && t[2] == c && t[3] == d;
    proof { if r { assert(t@ =~= seq![a, b, c, d]); } }
    r
}

pub(crate) fn kind_of(t: &[u8; 4]) -> (r: Kind)
    ensures r.view() == mw::classify(t@),
{
    if type_is(t, b'I', b'H', b'D', b'R') { Kind::Ihdr }
    else if type_is(t, b'P', b'L', b'T', b'E') { Kind::Plte }
    else if type_is(t, b'I', b'D', b'A', b'T') { Kind::Idat }
    else if type_is(t, b'I', b'E', b'N', b'D') { Kind::Iend }
    else if type_is(t, b't', b'R', b'N', b'S') { Kind::Trns }
    else if type_is(t, b'c', b'H', b'R', b'M') { Kind::Chrm }
    else if type_is(t, b'g', b'A', b'M', b'A') { Kind::Gama }
    else if type_is(t, b'i', b'C', b'C', b'P') { Kind::Iccp }
    else if type_is(t, b's', b'B', b'I', b'T') { Kind::Sbit }
    else if type_is(t, b's', b'R', b'G', b'B') { Kind::Srgb }
    else if type_is(t, b'c', b'I', b'C', b'P') { Kind::Cicp }
    else if type_is(t, b'm', b'D', b'C', b'V') { Kind::Mdcv }
    else if type_is(t, b'c', b'L', b'L', b'I') { Kind::Clli }
    else if type_is(t, b'b', b'K', b'G', b'D') { Kind::Bkgd }
    else if type_is(t, b'h', b'I', b'S', b'T') { Kind::Hist }
    else if type_is(t, b'p', b'H', b'Y', b's') { Kind::Phys }
    else if type_is(t, b's', b'P', b'L', b'T') { Kind::Splt }
    else if type_is(t, b'e', b'X', b'I', b'f') { Kind::Exif }
    else if type_is(t, b't', b'I', b'M', b'E') { Kind::Time }
    else if type_is(t, b't', b'E', b'X', b't') { Kind::Text }
    else if type_is(t, b'z', b'T', b'X', b't') { Kind::Ztxt }
    else if type_is(t, b'i', b'T', b'X', b't') { Kind::Itxt }
    else if type_is(t, b'a', b'c', b'T', b'L') { Kind::Actl }
    else if type_is(t, b'f', b'c', b'T', b'L') { Kind::Fctl }
    else if type_is(t, b'f', b'd', b'A', b'T') { Kind::Fdat }
    else { Kind::Unknown(t[0] & 0x20 != 0) }
}

impl Kind {
    pub(crate) open spec fn view(self) -> mw::Name {
        match self {
            Kind::Ihdr => mw::Name::Ihdr,
            Kind::Plte => mw::Name::Plte,
            Kind::Idat => mw::Name::Idat,
            Kind::Iend => mw::Name::Iend,
            Kind::Trns => mw::Name::Trns,
            Kind::Chrm => mw::Name::Chrm,
            Kind::Gama => mw::Name::Gama,
            Kind::Iccp => mw::Name::Iccp,
            Kind::Sbit => mw::Name::Sbit,
            Kind::Srgb => mw::Name::Srgb,
            Kind::Cicp => mw::Name::Cicp,
            Kind::Mdcv => mw::Name::Mdcv,
            Kind::Clli => mw::Name::Clli,
            Kind::Bkgd => mw::Name::Bkgd,
            Kind::Hist => mw::Name::Hist,
            Kind::Phys => mw::Name::Phys,
            Kind::Splt => mw::Name::Splt,
            Kind::Exif => mw::Name::Exif,
            Kind::Time => mw::Name::Time,
            Kind::Text => mw::Name::Text,
            Kind::Ztxt => mw::Name::Ztxt,
            Kind::Itxt => mw::Name::Itxt,
            Kind::Actl => mw::Name::Actl,
            Kind::Fctl => mw::Name::Fctl,
            Kind::Fdat => mw::Name::Fdat,
            Kind::Unknown(a) => mw::Name::Unknown(a),
        }
    }

    pub(crate) fn ancillary(self, t: &[u8; 4]) -> (r: bool)
        ensures r == match self.view() { mw::Name::Unknown(a) => a, _ => t@[0] & 0x20u8 != 0 },
    {
        match self {
            Kind::Unknown(a) => a,
            _ => t[0] & 0x20 != 0,
        }
    }
    /// The bit of the singleton set (P5.1); `None` for types that may repeat.
    pub(crate) fn single_bit(self) -> (r: Option<u32>)
        ensures match singleton_index(self.view()) {
            Some(i) => r == Some(1u32 << i), None => r.is_none(),
        },
    {
        let i = match self {
            Kind::Ihdr => 0,
            Kind::Plte => 1,
            Kind::Trns => 2,
            Kind::Chrm => 3,
            Kind::Gama => 4,
            Kind::Iccp => 5,
            Kind::Sbit => 6,
            Kind::Srgb => 7,
            Kind::Cicp => 8,
            Kind::Mdcv => 9,
            Kind::Clli => 10,
            Kind::Bkgd => 11,
            Kind::Hist => 12,
            Kind::Phys => 13,
            Kind::Exif => 14,
            Kind::Time => 15,
            Kind::Actl => 16,
            _ => return None,
        };
        Some(1u32 << i)
    }
    pub(crate) fn before_plte(self) -> (r: bool)
        ensures r == mw::before_plte(self.view()),
    {
        matches!(self, Kind::Chrm | Kind::Gama | Kind::Iccp | Kind::Sbit | Kind::Srgb | Kind::Cicp | Kind::Mdcv | Kind::Clli)
    }
    pub(crate) fn after_plte(self) -> (r: bool)
        ensures r == mw::after_plte(self.view()),
    {
        matches!(self, Kind::Bkgd | Kind::Hist | Kind::Trns)
    }
    pub(crate) fn before_idat(self) -> (r: bool)
        ensures r == mw::before_idat(self.view()),
    {
        matches!(self, Kind::Exif | Kind::Phys | Kind::Splt | Kind::Actl)
    }
    pub(crate) fn fixed_len(self) -> (r: Option<usize>)
        ensures match r { Some(n) => mw::fixed_length(self.view()) == Some(n as nat), None => mw::fixed_length(self.view()).is_none() },
    {
        match self {
            Kind::Gama => Some(4),
            Kind::Chrm => Some(32),
            Kind::Srgb => Some(1),
            Kind::Cicp => Some(4),
            Kind::Mdcv => Some(24),
            Kind::Clli => Some(8),
            Kind::Phys => Some(9),
            Kind::Time => Some(7),
            Kind::Actl => Some(8),
            Kind::Iend => Some(0),
            _ => None,
        }
    }
    pub(crate) fn is_apng(self) -> (r: bool)
        ensures r == mw::apng(self.view()),
    {
        matches!(self, Kind::Actl | Kind::Fctl | Kind::Fdat)
    }
}

pub(crate) const AFTER_PLTE_BITS: u32 = (1 << 2) | (1 << 11) | (1 << 12);


pub(crate) open spec fn singleton_index(n: mw::Name) -> Option<u32> {
    match n {
        mw::Name::Ihdr => Some(0), mw::Name::Plte => Some(1), mw::Name::Trns => Some(2),
        mw::Name::Chrm => Some(3), mw::Name::Gama => Some(4), mw::Name::Iccp => Some(5),
        mw::Name::Sbit => Some(6), mw::Name::Srgb => Some(7), mw::Name::Cicp => Some(8),
        mw::Name::Mdcv => Some(9), mw::Name::Clli => Some(10), mw::Name::Bkgd => Some(11),
        mw::Name::Hist => Some(12), mw::Name::Phys => Some(13), mw::Name::Exif => Some(14),
        mw::Name::Time => Some(15), mw::Name::Actl => Some(16), _ => None,
    }
}

pub(crate) open spec fn seen_view(mask: u32) -> Set<mw::Name> {
    set![mw::Name::Ihdr, mw::Name::Plte, mw::Name::Trns, mw::Name::Chrm, mw::Name::Gama, mw::Name::Iccp,
        mw::Name::Sbit, mw::Name::Srgb, mw::Name::Cicp, mw::Name::Mdcv, mw::Name::Clli, mw::Name::Bkgd,
        mw::Name::Hist, mw::Name::Phys, mw::Name::Exif, mw::Name::Time, mw::Name::Actl]
        .filter(|n: mw::Name| match singleton_index(n) {
        Some(i) => mask & (1u32 << i) != 0,
        None => false,
    })
}

proof fn index_facts(n: mw::Name)
    ensures singleton_index(n).is_some() == mw::singleton(n),
        singleton_index(n).is_some() ==> singleton_index(n).unwrap() < 17,
{}

proof fn index_unique(a: mw::Name, b: mw::Name)
    requires singleton_index(a).is_some(), singleton_index(a) == singleton_index(b),
    ensures a == b,
{}

proof fn lemma_insert(mask: u32, name: mw::Name, i: u32)
    requires singleton_index(name) == Some(i),
    ensures seen_view(mask | (1u32 << i)) == seen_view(mask).insert(name),
{
    index_facts(name);
    assert forall|other: mw::Name| seen_view(mask | (1u32 << i)).contains(other)
        == seen_view(mask).insert(name).contains(other) by {
        index_facts(other);
        match singleton_index(other) {
            None => { assert(other != name); },
            Some(j) => {
                if i == j { index_unique(name, other); }
                assert(((mask | (1u32 << i)) & (1u32 << j) != 0)
                    == ((mask & (1u32 << j) != 0) || i == j)) by (bit_vector)
                    requires i < 17, j < 17;
            },
        }
    }
    assert(seen_view(mask | (1u32 << i)) =~= seen_view(mask).insert(name));
}

/// Actual singleton-mask initialization used by the executable walk.
pub(crate) fn initial_seen() -> (r: u32)
    ensures seen_view(r) == set![mw::Name::Ihdr],
{
    proof {
        assert forall|n: mw::Name| seen_view(1).contains(n) == (n == mw::Name::Ihdr) by {
            index_facts(n);
            match singleton_index(n) {
                None => {},
                Some(i) => {
                    if i == 0 { index_unique(n, mw::Name::Ihdr); }
                    assert((1u32 & (1u32 << i) != 0) == (i == 0)) by (bit_vector) requires i < 17;
                },
            }
        }
        assert(seen_view(1) =~= set![mw::Name::Ihdr]);
    }
    1
}

pub(crate) fn mark_seen(mask: &mut u32, kind: Kind)
    ensures seen_view(*final(mask)) == if mw::singleton(kind.view()) {
        seen_view(*old(mask)).insert(kind.view())
    } else { seen_view(*old(mask)) },
{
    proof { index_facts(kind.view()); }
    if let Some(bit) = kind.single_bit() {
        proof { lemma_insert(*mask, kind.view(), singleton_index(kind.view()).unwrap()); }
        *mask |= bit;
    }
}

pub(crate) fn seen_contains(mask: u32, kind: Kind) -> (r: bool)
    ensures r == seen_view(mask).contains(kind.view()),
        r ==> mw::singleton(kind.view()),
{
    proof { index_facts(kind.view()); }
    match kind.single_bit() { Some(bit) => mask & bit != 0, None => false }
}

/// PLTE's own ordering test reads the same mathematical singleton set, not
/// an unrelated magic mask whose bit positions could drift.
pub(crate) fn after_plte_seen(mask: u32) -> (r: bool)
    ensures r == (seen_view(mask).contains(mw::Name::Trns)
        || seen_view(mask).contains(mw::Name::Bkgd) || seen_view(mask).contains(mw::Name::Hist)),
{
    assert((mask & AFTER_PLTE_BITS != 0) == ((mask & (1u32 << 2u32) != 0)
        || (mask & (1u32 << 11u32) != 0) || (mask & (1u32 << 12u32) != 0))) by (bit_vector);
    mask & AFTER_PLTE_BITS != 0
}

} // verus!
