//! Independent P5/P14 control-state vocabulary and P5 preamble. Singleton
//! membership is a mathematical set; no executable bit positions occur here.
//! The complete document/trace walk is defined in document_model.
use vstd::prelude::*;
use crate::png::Error;

verus! {

#[derive(PartialEq, Eq)]
pub enum Name {
    Ihdr, Plte, Idat, Iend, Trns, Chrm, Gama, Iccp, Sbit, Srgb, Cicp, Mdcv, Clli, Bkgd, Hist, Phys, Splt,
    Exif, Time, Text, Ztxt, Itxt, Actl, Fctl, Fdat, Unknown(bool),
}

pub open spec fn classify(t: Seq<u8>) -> Name {
    if t == seq![b'I',b'H',b'D',b'R'] { Name::Ihdr }
    else if t == seq![b'P',b'L',b'T',b'E'] { Name::Plte }
    else if t == seq![b'I',b'D',b'A',b'T'] { Name::Idat }
    else if t == seq![b'I',b'E',b'N',b'D'] { Name::Iend }
    else if t == seq![b't',b'R',b'N',b'S'] { Name::Trns }
    else if t == seq![b'c',b'H',b'R',b'M'] { Name::Chrm }
    else if t == seq![b'g',b'A',b'M',b'A'] { Name::Gama }
    else if t == seq![b'i',b'C',b'C',b'P'] { Name::Iccp }
    else if t == seq![b's',b'B',b'I',b'T'] { Name::Sbit }
    else if t == seq![b's',b'R',b'G',b'B'] { Name::Srgb }
    else if t == seq![b'c',b'I',b'C',b'P'] { Name::Cicp }
    else if t == seq![b'm',b'D',b'C',b'V'] { Name::Mdcv }
    else if t == seq![b'c',b'L',b'L',b'I'] { Name::Clli }
    else if t == seq![b'b',b'K',b'G',b'D'] { Name::Bkgd }
    else if t == seq![b'h',b'I',b'S',b'T'] { Name::Hist }
    else if t == seq![b'p',b'H',b'Y',b's'] { Name::Phys }
    else if t == seq![b's',b'P',b'L',b'T'] { Name::Splt }
    else if t == seq![b'e',b'X',b'I',b'f'] { Name::Exif }
    else if t == seq![b't',b'I',b'M',b'E'] { Name::Time }
    else if t == seq![b't',b'E',b'X',b't'] { Name::Text }
    else if t == seq![b'z',b'T',b'X',b't'] { Name::Ztxt }
    else if t == seq![b'i',b'T',b'X',b't'] { Name::Itxt }
    else if t == seq![b'a',b'c',b'T',b'L'] { Name::Actl }
    else if t == seq![b'f',b'c',b'T',b'L'] { Name::Fctl }
    else if t == seq![b'f',b'd',b'A',b'T'] { Name::Fdat }
    else { Name::Unknown(t.len() == 4 && t[0] & 0x20u8 != 0) }
}

pub open spec fn singleton(n: Name) -> bool {
    matches!(n, Name::Ihdr | Name::Plte | Name::Trns | Name::Chrm | Name::Gama | Name::Iccp
        | Name::Sbit | Name::Srgb | Name::Cicp | Name::Mdcv | Name::Clli | Name::Bkgd | Name::Hist
        | Name::Phys | Name::Exif | Name::Time | Name::Actl)
}

pub open spec fn before_plte(n: Name) -> bool {
    set![Name::Chrm, Name::Gama, Name::Iccp, Name::Sbit, Name::Srgb, Name::Cicp, Name::Mdcv, Name::Clli].contains(n)
}
pub open spec fn after_plte(n: Name) -> bool { set![Name::Bkgd, Name::Hist, Name::Trns].contains(n) }
pub open spec fn before_idat(n: Name) -> bool { set![Name::Exif, Name::Phys, Name::Splt, Name::Actl].contains(n) }
pub open spec fn apng(n: Name) -> bool { set![Name::Actl, Name::Fctl, Name::Fdat].contains(n) }
pub open spec fn fixed_length(n: Name) -> Option<nat> {
    match n {
        Name::Gama => Some(4), Name::Chrm => Some(32), Name::Srgb => Some(1), Name::Cicp => Some(4),
        Name::Mdcv => Some(24), Name::Clli => Some(8), Name::Phys => Some(9), Name::Time => Some(7),
        Name::Actl => Some(8), Name::Iend => Some(0), _ => None,
    }
}

pub enum Phase { Before, During, After }

pub open spec fn advance_phase(phase: Phase, name: Name) -> Phase {
    if matches!(phase, Phase::During) && name != Name::Idat { Phase::After } else { phase }
}
pub struct Range { pub offset: nat, pub len: nat }
pub enum Source { Idat, Fdat(Seq<Range>) }
pub struct Frame {
    /// fcTL header offset in the original input, for erased re-walk provenance.
    pub control_offset: nat,
    pub width: u32, pub height: u32, pub x: u32, pub y: u32,
    pub delay_num: u16, pub delay_den: u16, pub dispose: u8, pub blend: u8,
    pub source: Source,
}
/// Independently evolving document-control state. Streams and frames are
/// mathematical sequences, not the executable's first-offset representation.
pub struct State {
    pub seen: Set<Name>,
    pub plte: Option<Range>,
    pub phase: Phase,
    pub sequence: nat,
    pub actl: Option<(u32, u32)>,
    pub frames: Seq<Frame>,
    pub current: Option<nat>,
    pub current_has_fdat: bool,
    pub fctl_before_idat: bool,
    pub pixel_total: nat,
    pub idat: Seq<Range>,
    pub default_is_frame: bool,
}

pub open spec fn initial() -> State {
    State { seen: set![Name::Ihdr], plte: None, phase: Phase::Before, sequence: 0,
        actl: None, frames: Seq::empty(), current: None, current_has_fdat: false,
        fctl_before_idat: false, pixel_total: 0, idat: Seq::empty(), default_is_frame: false }
}

/// The five independent policy choices used in the P5 preamble. The complete
/// policy walk will additionally model CRC selection and APNG fallback sites.
pub struct Rules {
    pub first_duplicate_wins: bool,
    pub drop_misordered: bool,
    pub drop_invalid: bool,
    pub ignore_unexpected_plte: bool,
    pub lenient_iend: bool,
}

pub enum OrderAction { Keep(nat), Drop, Fail(Error, bool) }

/// Duplicate, ancillary ordering, then fixed lengths, in that exact order.
/// PLTE's own ordering/value branch is a separate following stage, as is P14.
pub open spec fn order(seen: Set<Name>, phase: Phase, plte_seen: bool, ct: u8,
    name: Name, ancillary: bool, length: nat, rules: Rules) -> OrderAction
{
    if singleton(name) && seen.contains(name) {
        if ancillary && rules.first_duplicate_wins { OrderAction::Drop }
        else if name == Name::Plte && rules.ignore_unexpected_plte && (ct == 0 || ct == 4) { OrderAction::Drop }
        else { OrderAction::Fail(Error::DuplicateChunk, apng(name)) }
    } else if (before_plte(name) && (plte_seen || !matches!(phase, Phase::Before)))
        || (after_plte(name) && (!matches!(phase, Phase::Before) || (!plte_seen && (ct == 3 || name == Name::Hist))))
        || (before_idat(name) && !matches!(phase, Phase::Before)) {
        if name == Name::Actl { OrderAction::Fail(Error::ChunkOutOfOrder, true) }
        else if rules.drop_misordered { OrderAction::Drop }
        else { OrderAction::Fail(Error::ChunkOutOfOrder, false) }
    } else {
        let adjusted = if name == Name::Iend && rules.lenient_iend { 0nat } else { length };
        match fixed_length(name) {
            Some(expected) => if adjusted != expected {
                if ancillary && name != Name::Actl && rules.drop_invalid { OrderAction::Drop }
                else { OrderAction::Fail(Error::InvalidChunkLength, name == Name::Actl) }
            } else { OrderAction::Keep(adjusted) },
            None => OrderAction::Keep(adjusted),
        }
    }
}

} // verus!
