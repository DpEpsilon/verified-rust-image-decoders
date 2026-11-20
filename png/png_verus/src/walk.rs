//! The structural pass, SPEC P2–P8 and P14.1, under a policy record
//! (`../../policy-draft/policy_oracle.py` `_walk`, restricted to the values
//! Strict and RecommendedBeta use). No pixel is decoded and no zlib stream is
//! inflated here. The result is a fixed-size record: no chunk list and no frame
//! list (SPEC P20); data chunks are re-walked by offset (`reader.rs`).
//!
//! Check order per chunk: P3 (framing, letters), the IDAT phase, the CRC rule,
//! P5.1 duplicates, P5.2 ordering, P5.3 fixed lengths, then the chunk's own
//! section, exactly as the oracle's `_walk`.

use crate::apng::{self, Control};
use crate::critical::{self, CrcAction};
use vstd::prelude::*;
use crate::png::{Error, Events, Info, Meta, Time};
use crate::policy::{
    AncillaryDuplicate, AncillaryOrder, AncillaryValues, ApngErrors, ApngInStill, CrcAncillary, Iend,
    MissingIend, PaletteSize, PlteUnexpected, Policy, Trailing, TrnsInvalid,
};

pub(crate) use crate::framing::{be16, be32, frame_at};

pub(crate) use crate::kind::{kind_of, Kind};
use crate::kind::{initial_seen, mark_seen, after_plte_seen};
use crate::ordering::{advance_phase, preamble, OrderAction, Rules as OrderRules};

pub(crate) use crate::header::Ihdr;

/// acTL (P14.1).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Actl {
    pub num_frames: u32,
    pub num_plays: u32,
}

/// The walk's result: the document record used by P11–P17.
pub(crate) struct Doc {
    pub ihdr: Ihdr,
    /// PLTE data `(offset, length)` in the input.
    pub plte: Option<(usize, usize)>,
    /// tRNS data `(offset, length)`.
    pub trns: Option<(usize, usize)>,
    /// The compressed profile field of iCCP `(offset, length)`.
    pub iccp: Option<(usize, usize)>,
    /// Offset of the first IDAT chunk (the re-walk starts there, L-REWALK).
    pub idat_first: usize,
    pub actl: Option<Actl>,
    pub info: Info,
}

impl Doc {
    pub fn palette_entries(&self) -> usize {
        match self.plte {
            Some((_, n)) => n / 3,
            None => 0,
        }
    }
}

/// Which entry point runs the walk (`policy_oracle.py` `entry`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Entry {
    Still,
    Frames,
}

verus! {

/// Why the walk stopped without a document.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WalkFail {
    Format(Error),
    /// `apng_errors = Static` on the frames entry: an APNG structural error
    /// (`sfail`, `apng_fail`); the caller re-walks with APNG off (R4).
    Fallback,
}

impl WalkFail {
    pub(crate) open spec fn view(self) -> crate::critical_model::Failure {
        match self {
            Self::Format(e) => crate::critical_model::Failure::Format(e),
            Self::Fallback => crate::critical_model::Failure::Fallback,
        }
    }
}

} // verus!

impl From<Error> for WalkFail {
    fn from(e: Error) -> WalkFail {
        WalkFail::Format(e)
    }
}

/// The per-walk state (P5): a fixed-size record.
struct State {
    seen: u32,
    /// 0 before the first IDAT, 1 in the IDAT run, 2 after it.
    idat: u8,
    apng: Control,
}

/// P2–P8, P14.1 under `pol`. `apng_off` is the re-walk of the R4 fallback, with
/// acTL, fcTL and fdAT treated as unknown ancillary chunks (`apng = Off`).
pub(crate) fn walk(b: &[u8], pol: &Policy, entry: Entry, apng_off: bool, ev: &mut Events) -> Result<Doc, WalkFail> {
    let (ihdr, after_ihdr) = crate::header::prefix(b)?;
    let ct = ihdr.ct;
    let depth = ihdr.depth;
    let mut doc = Doc {
        ihdr,
        plte: None,
        trns: None,
        iccp: None,
        idat_first: 0,
        actl: None,
        info: Info::new(&ihdr),
    };
    let mut st = State { seen: initial_seen(), idat: 0, apng: Control::new() };
    let apng_on = !apng_off && !(entry == Entry::Still && pol.apng_in_still == ApngInStill::Ignore);
    let static_frames = pol.apng_errors == ApngErrors::Static && entry == Entry::Frames;
    let mut p = after_ihdr;
    loop {
        let candidate = critical::read(b,p,st.idat,pol.missing_iend == MissingIend::AtEof)?;
        let c = candidate.chunk;
        let (t,kind0,dpos,mut n,ok,p_end) = (c.ty,candidate.kind,c.data,c.len,c.crc_ok,c.next);
        advance_phase(&mut st.idat, kind0);
        let chunk_offset = p;
        p = p_end;
        let (kind,anc) = critical::effective(kind0,kind0.ancillary(&t),apng_on);
        let d = &b[dpos..dpos + n];
        // M1b-Q2: only the enabled frames-walk CRC-drop site makes a bad
        // fcTL structural; fdAT CRC remains mandatory even in Static fallback.
        match critical::crc_rule(kind0,kind,anc,ok,
            pol.crc_ancillary == CrcAncillary::SkipExceptFdat,
            pol.iend == Iend::Lenient,entry == Entry::Frames) {
            CrcAction::Keep => {},
            CrcAction::Drop => { discard(&mut doc,kind,ev); continue; },
            CrcAction::Fail(e,structural) => return Err(critical::route(e,structural,static_frames)),
        }
        // P5.1/P5.2/P5.3: the verified preamble preserves the exact first
        // error and the structural/non-structural recovery distinction.
        let rules = OrderRules {
            first_duplicate_wins: pol.ancillary_duplicate == AncillaryDuplicate::FirstWins,
            drop_misordered: pol.ancillary_order == AncillaryOrder::Drop,
            drop_invalid: pol.ancillary_values == AncillaryValues::Drop,
            ignore_unexpected_plte: pol.plte_unexpected == PlteUnexpected::Ignore,
            lenient_iend: pol.iend == Iend::Lenient,
        };
        match preamble(st.seen, st.idat, doc.plte.is_some(), ct, kind, anc, n, rules) {
            OrderAction::Keep(length) => n = length,
            OrderAction::Drop => {
                // Ignored PLTE in grey/grey-alpha has no metadata/event to
                // discard; this is also what the original branch did.
                discard(&mut doc, kind, ev);
                continue;
            }
            OrderAction::Fail(e, structural) => {
                return Err(critical::route(e,structural,static_frames));
            }
        }
        let d = &d[..n];
        mark_seen(&mut st.seen, kind);
        // ---- the chunk's own section
        match kind {
            Kind::Ihdr => return Err(Error::DuplicateChunk.into()),
            Kind::Plte => {
                let entries = match crate::metadata::palette(n,ct,depth,st.idat != 0,
                    after_plte_seen(st.seen), pol.plte_unexpected == PlteUnexpected::Ignore,
                    pol.palette_size == PaletteSize::Allow)? {
                    Some(entries) => entries,
                    None => continue,
                };
                doc.plte = Some((dpos,n));
                doc.info.palette_entries = entries;
            }
            Kind::Idat => {
                let first = critical::idat(&mut st.idat,ct,doc.plte.is_some())?;
                if first {
                    if st.apng.begin_idat() { doc.info.default_is_frame = true; }
                    doc.idat_first = chunk_offset;
                }
            }
            Kind::Iend => {
                let declared = doc.actl.map(|a| a.num_frames);
                if let Err((e,structural)) = critical::iend(st.idat,st.apng,declared,p,b.len(),pol.trailing == Trailing::Ignore) {
                    return Err(critical::route(e,structural,static_frames));
                }
                doc.info.frame_count = st.apng.frames as u32;
                return Ok(doc);
            }
            Kind::Trns => {
                if !crate::metadata::transparency(n,ct,doc.palette_entries(),pol.trns_invalid == TrnsInvalid::Drop)? {
                    discard(&mut doc,kind,ev);
                    continue;
                }
                doc.trns = Some((dpos,n));
                doc.info.trns = Meta::Present(());
            }
            Kind::Actl => {
                let (num_frames, num_plays) = match apng::actl(d) {
                    Ok(a) => a,
                    Err((e, structural)) => return Err(critical::route(e,structural,static_frames)),
                };
                doc.actl = Some(Actl { num_frames, num_plays });
            }
            Kind::Fctl => {
                let (_, next) = match apng::fctl(d, doc.actl.is_some(), st.idat == 0,
                    doc.ihdr.w, doc.ihdr.h, st.apng) {
                    Ok(r) => r,
                    Err((e, structural)) => return Err(critical::route(e,structural,static_frames)),
                };
                st.apng = next;
            }
            Kind::Fdat => {
                st.apng = match apng::fdat(d, doc.actl.is_some(), st.idat == 0, st.apng) {
                    Ok(next) => next,
                    Err((e, structural)) => return Err(critical::route(e,structural,static_frames)),
                };
            }
            Kind::Unknown(a) => { critical::unknown(a)?; }
            _ => match ancillary_values(kind, b, d, dpos, &mut doc, ct, depth) {
                Ok(()) => {}
                Err(e) => {
                    if pol.ancillary_values == AncillaryValues::Drop {
                        discard(&mut doc, kind, ev);
                        continue;
                    }
                    return Err(e.into());
                }
            },
        }
    }
}

pub(crate) use crate::apng::Fctl;

/// A chunk discarded by the profile (`policy_oracle.py` `drop`): R1 for tRNS in
/// colour types 0, 2 and 3; the metadata item becomes `Discarded` unless an
/// earlier chunk of the type was accepted.
fn discard(doc: &mut Doc, kind: Kind, ev: &mut Events) {
    let ct = doc.ihdr.ct;
    if kind == Kind::Trns && (ct == 0 || ct == 2 || ct == 3) {
        ev.r1 = true;
    }
    let i = &mut doc.info;
    match kind {
        Kind::Trns => i.trns.discard(),
        Kind::Gama => i.gama.discard(),
        Kind::Chrm => i.chrm.discard(),
        Kind::Srgb => i.srgb.discard(),
        Kind::Iccp => i.iccp_name.discard(),
        Kind::Sbit => i.sbit.discard(),
        Kind::Cicp => i.cicp.discard(),
        Kind::Mdcv => i.mdcv.discard(),
        Kind::Clli => i.clli.discard(),
        Kind::Bkgd => i.bkgd.discard(),
        Kind::Phys => i.phys.discard(),
        Kind::Exif => i.exif.discard(),
        Kind::Time => i.time.discard(),
        _ => {}
    }
}

/// P8.1 value rules, and the raw values for `Info` (P8.3).
fn ancillary_values(kind: Kind, b: &[u8], d: &[u8], dpos: usize, doc: &mut Doc, ct: u8, depth: u8) -> Result<(), Error> {
    use crate::metadata::Value;
    let value = crate::metadata::parse(kind,d,ct,depth,doc.palette_entries())?;
    match value {
        Value::Ignored => {},
        Value::Gama(g) => doc.info.gama = Meta::Present(g),
        Value::Chrm(v) => doc.info.chrm = Meta::Present(v),
        Value::Srgb(v) => doc.info.srgb = Meta::Present(v),
        Value::Iccp {name_len,profile_start,profile_len} => {
            doc.iccp = Some(crate::metadata::rebase(b,dpos,d.len(),profile_start,profile_len));
            doc.info.iccp_name = Meta::Present(crate::metadata::rebase(b,dpos,d.len(),0,name_len));
        },
        Value::Sbit(v,n) => doc.info.sbit = Meta::Present((v,n)),
        Value::Cicp(v) => doc.info.cicp = Meta::Present(v),
        Value::Mdcv(v) => doc.info.mdcv = Meta::Present(v),
        Value::Clli(a,b) => doc.info.clli = Meta::Present([a,b]),
        Value::Bkgd(v,_) => doc.info.bkgd = Meta::Present(v),
        Value::Phys(x,y,u) => doc.info.phys = Meta::Present((x,y,u)),
        Value::Exif(n) => doc.info.exif = Meta::Present(crate::metadata::rebase(b,dpos,d.len(),0,n)),
        Value::Time(year,month,day,hour,minute,second) => {
            doc.info.time = Meta::Present(Time {year,month,day,hour,minute,second});
        },
    }
    Ok(())
}
