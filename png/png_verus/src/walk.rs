//! The structural pass, SPEC P2–P8 and P14.1, under a policy record
//! (`../../policy-draft/policy_oracle.py` `_walk`, restricted to the values
//! Strict and RecommendedBeta use). No pixel is decoded and no zlib stream is
//! inflated here. The result is a fixed-size record: no chunk list and no frame
//! list (SPEC P20); data chunks are re-walked by offset (`reader.rs`).
//!
//! Check order per chunk: P3 (framing, letters), the IDAT phase, the CRC rule,
//! P5.1 duplicates, P5.2 ordering, P5.3 fixed lengths, then the chunk's own
//! section, exactly as the oracle's `_walk`.

use crate::limits::{MAX_FRAMES, MAX_TOTAL_PIXELS};
use crate::png::{Error, Events, Info, Meta, Time};
use crate::policy::{
    AncillaryDuplicate, AncillaryOrder, AncillaryValues, ApngErrors, ApngInStill, CrcAncillary, Iend,
    MissingIend, PaletteSize, PlteUnexpected, Policy, Trailing, TrnsInvalid,
};

pub(crate) use crate::framing::{be16, be32, frame_at};
use crate::framing::chunk_at;

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

/// Why the walk stopped without a document.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WalkFail {
    Format(Error),
    /// `apng_errors = Static` on the frames entry: an APNG structural error
    /// (`sfail`, `apng_fail`); the caller re-walks with APNG off (R4).
    Fallback,
}

impl From<Error> for WalkFail {
    fn from(e: Error) -> WalkFail {
        WalkFail::Format(e)
    }
}

use crate::keywords::{find_nul, split_keyword};

/// The per-walk state (P5): a fixed-size record.
struct State {
    seen: u32,
    /// 0 before the first IDAT, 1 in the IDAT run, 2 after it.
    idat: u8,
    seq: u32,
    /// The current frame: `(idat_based, fdat_seen)`.
    cur: Option<(bool, bool)>,
    frames: u64,
    fctl_before_idat: bool,
    pixels: u64,
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
    let mut st = State { seen: initial_seen(), idat: 0, seq: 0, cur: None, frames: 0, fctl_before_idat: false, pixels: 0 };
    let apng_on = !apng_off && !(entry == Entry::Still && pol.apng_in_still == ApngInStill::Ignore);
    let static_frames = pol.apng_errors == ApngErrors::Static && entry == Entry::Frames;
    // sfail and apng_fail coincide for the values Strict and RecommendedBeta use.
    let sfail = |e: Error| -> WalkFail { if static_frames { WalkFail::Fallback } else { WalkFail::Format(e) } };
    let mut p = after_ihdr;
    loop {
        let (t, kind0, dpos, mut n, ok, p_end) = if p == b.len() {
            if st.idat > 0 && pol.missing_iend == MissingIend::AtEof {
                (*b"IEND", Kind::Iend, p, 0usize, true, p)
            } else {
                return Err(Error::MissingIend.into());
            }
        } else {
            let c = chunk_at(b, p)?;
            (c.ty, kind_of(&c.ty), c.data, c.len, c.crc_ok, c.next)
        };
        advance_phase(&mut st.idat, kind0);
        let chunk_offset = p;
        p = p_end;
        let mut anc = kind0.ancillary(&t);
        let mut kind = kind0;
        if !apng_on && kind.is_apng() {
            kind = Kind::Unknown(true);
            anc = true;
        }
        let d = &b[dpos..dpos + n];
        if !ok {
            if kind == Kind::Iend {
                if pol.iend == Iend::Strict {
                    return Err(Error::CrcMismatch.into());
                }
            } else if anc && !(kind0 == Kind::Fdat && pol.crc_ancillary == CrcAncillary::SkipExceptFdat) {
                match pol.crc_ancillary {
                    CrcAncillary::Reject => return Err(Error::CrcMismatch.into()),
                    // Council M1BQ-Q2 (c): in the frames constructor's walk with
                    // APNG on, a fcTL that this rule would drop for its CRC is
                    // not dropped but is the APNG structural error `CrcMismatch`,
                    // raised here, after framing and bounds and before its
                    // sequence number is read. `sfail` makes it R4 under
                    // `apng_errors = Static` and the error under `Reject`. The
                    // walk site, not the name, selects recovery: the mandatory
                    // `CrcMismatch` returns above and below never reach `sfail`.
                    // `decode_still`, the ICC call and the R4 fallback's re-walk
                    // (`apng_off`, where `kind` is no longer `Fctl`) drop it.
                    CrcAncillary::SkipExceptFdat if kind == Kind::Fctl && entry == Entry::Frames => {
                        return Err(sfail(Error::CrcMismatch));
                    }
                    CrcAncillary::SkipExceptFdat => {
                        discard(&mut doc, kind, ev);
                        continue;
                    }
                }
            } else {
                // IHDR, PLTE, IDAT, unknown critical; fdAT under SkipExceptFdat.
                return Err(Error::CrcMismatch.into());
            }
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
                return Err(if structural { sfail(e) } else { WalkFail::Format(e) });
            }
        }
        let d = &d[..n];
        mark_seen(&mut st.seen, kind);
        // ---- the chunk's own section
        match kind {
            Kind::Ihdr => return Err(Error::DuplicateChunk.into()),
            Kind::Plte => {
                // P6, P5.2 (PLTE after IDAT or after bKGD/hIST/tRNS)
                if st.idat != 0 || after_plte_seen(st.seen) {
                    if pol.plte_unexpected == PlteUnexpected::Ignore && ct != 3 {
                        continue;
                    }
                    return Err(Error::ChunkOutOfOrder.into());
                }
                if ct == 0 || ct == 4 {
                    if pol.plte_unexpected == PlteUnexpected::Ignore {
                        continue;
                    }
                    return Err(Error::UnexpectedPalette.into());
                }
                if n == 0 || n % 3 != 0 || n > 768 {
                    return Err(Error::InvalidPalette.into());
                }
                if ct == 3 && n / 3 > (1usize << depth) && pol.palette_size == PaletteSize::Reject {
                    return Err(Error::InvalidPalette.into());
                }
                doc.plte = Some((dpos, n));
                doc.info.palette_entries = (n / 3) as u32;
            }
            Kind::Idat => {
                if st.idat == 2 {
                    return Err(Error::NonConsecutiveIdat.into());
                }
                if ct == 3 && doc.plte.is_none() {
                    return Err(Error::MissingPalette.into());
                }
                if st.idat == 0 {
                    if let Some(cur) = st.cur.as_mut() {
                        cur.0 = true;
                        doc.info.default_is_frame = true;
                    }
                    doc.idat_first = chunk_offset;
                }
                st.idat = 1;
            }
            Kind::Iend => {
                if st.idat == 0 {
                    return Err(Error::MissingImageData.into());
                }
                if let Some((idat_based, fdat_seen)) = st.cur {
                    if !idat_based && !fdat_seen {
                        return Err(sfail(Error::MissingFrameData));
                    }
                }
                if let Some(a) = doc.actl {
                    if st.frames != a.num_frames as u64 {
                        return Err(sfail(Error::FrameCountMismatch));
                    }
                }
                if p != b.len() && pol.trailing == Trailing::Reject {
                    return Err(Error::TrailingBytes.into());
                }
                doc.info.frame_count = st.frames as u32;
                return Ok(doc);
            }
            Kind::Trns => {
                let e = doc.palette_entries();
                let bad = ct == 4
                    || ct == 6
                    || (ct == 0 && n != 2)
                    || (ct == 2 && n != 6)
                    || (ct == 3 && !(1 <= n && n <= e));
                if bad {
                    match pol.trns_invalid {
                        TrnsInvalid::Drop => {
                            discard(&mut doc, kind, ev);
                            continue;
                        }
                        TrnsInvalid::Reject => return Err(Error::InvalidTransparency.into()),
                    }
                }
                doc.trns = Some((dpos, n));
                doc.info.trns = Meta::Present(());
            }
            Kind::Actl => {
                let nf = be32(d, 0);
                let plays = be32(d, 4);
                if nf == 0 {
                    return Err(sfail(Error::InvalidAnimationControl));
                }
                if nf > MAX_FRAMES {
                    return Err(Error::TooManyFrames.into());
                }
                doc.actl = Some(Actl { num_frames: nf, num_plays: plays });
            }
            Kind::Fctl => {
                if doc.actl.is_none() {
                    return Err(sfail(Error::MissingAnimationControl));
                }
                if st.idat == 0 && st.fctl_before_idat {
                    return Err(sfail(Error::ChunkOutOfOrder));
                }
                if n != 26 {
                    return Err(sfail(Error::InvalidChunkLength));
                }
                if be32(d, 0) != st.seq {
                    return Err(sfail(Error::SequenceError));
                }
                st.seq = st.seq.wrapping_add(1);
                if let Some((idat_based, fdat_seen)) = st.cur {
                    if !idat_based && !fdat_seen {
                        return Err(sfail(Error::MissingFrameData));
                    }
                }
                let f = Fctl::parse(d);
                let (wd, hd) = (doc.ihdr.w as u64, doc.ihdr.h as u64);
                if f.w == 0
                    || f.h == 0
                    || f.x as u64 + f.w as u64 > wd
                    || f.y as u64 + f.h as u64 > hd
                    || f.dispose > 2
                    || f.blend > 1
                {
                    return Err(sfail(Error::InvalidFrameControl));
                }
                if st.idat == 0 && (f.x != 0 || f.y != 0 || f.w as u64 != wd || f.h as u64 != hd) {
                    return Err(sfail(Error::InvalidFrameControl));
                }
                // w ≤ W, h ≤ H ≤ i32::MAX here, so w·h < u64::MAX / 4 and the running sum, at
                // most MAX_TOTAL_PIXELS before the addition, stays in u64.
                st.pixels += f.w as u64 * f.h as u64;
                if st.pixels > MAX_TOTAL_PIXELS {
                    return Err(Error::PixelBudgetExceeded.into());
                }
                if st.idat == 0 {
                    st.fctl_before_idat = true;
                }
                st.cur = Some((false, false));
                st.frames += 1;
            }
            Kind::Fdat => {
                if doc.actl.is_none() {
                    return Err(sfail(Error::MissingAnimationControl));
                }
                if st.idat == 0 {
                    return Err(sfail(Error::ChunkOutOfOrder));
                }
                if n < 4 {
                    return Err(sfail(Error::InvalidChunkLength));
                }
                if be32(d, 0) != st.seq {
                    return Err(sfail(Error::SequenceError));
                }
                st.seq = st.seq.wrapping_add(1);
                match st.cur.as_mut() {
                    Some(cur) if !cur.0 => cur.1 = true,
                    _ => return Err(sfail(Error::UnexpectedFrameData)),
                }
            }
            Kind::Unknown(a) => {
                if !a {
                    return Err(Error::UnknownCriticalChunk.into());
                }
            }
            _ => match ancillary_values(kind, d, dpos, &mut doc, ct, depth) {
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

/// The parsed fields of a 26-byte fcTL (P14.1).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Fctl {
    pub w: u32,
    pub h: u32,
    pub x: u32,
    pub y: u32,
    pub delay_num: u16,
    pub delay_den: u16,
    pub dispose: u8,
    pub blend: u8,
}

impl Fctl {
    pub fn parse(d: &[u8]) -> Fctl {
        Fctl {
            w: be32(d, 4),
            h: be32(d, 8),
            x: be32(d, 12),
            y: be32(d, 16),
            delay_num: be16(d, 20),
            delay_den: be16(d, 22),
            dispose: d[24],
            blend: d[25],
        }
    }
}

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
fn ancillary_values(kind: Kind, d: &[u8], dpos: usize, doc: &mut Doc, ct: u8, depth: u8) -> Result<(), Error> {
    let n = d.len();
    match kind {
        Kind::Gama => {
            let g = be32(d, 0);
            if g == 0 {
                return Err(Error::InvalidGamma);
            }
            doc.info.gama = Meta::Present(g);
        }
        Kind::Chrm => {
            let mut v = [0u32; 8];
            for (i, x) in v.iter_mut().enumerate() {
                *x = be32(d, 4 * i);
            }
            doc.info.chrm = Meta::Present(v);
        }
        Kind::Srgb => {
            if d[0] > 3 {
                return Err(Error::InvalidSrgb);
            }
            doc.info.srgb = Meta::Present(d[0]);
        }
        Kind::Iccp => {
            let z = split_keyword(d, Error::InvalidIccp)?;
            if z + 1 >= n || d[z + 1] != 0 {
                return Err(Error::InvalidIccp);
            }
            doc.iccp = Some((dpos + z + 2, n - z - 2));
            doc.info.iccp_name = Meta::Present((dpos, z));
        }
        Kind::Sbit => {
            let want = match ct {
                0 => 1,
                2 | 3 => 3,
                4 => 2,
                _ => 4,
            };
            if n != want {
                return Err(Error::InvalidChunkLength);
            }
            let top = if ct == 3 { 8 } else { depth };
            let mut v = [0u8; 4];
            for (i, &x) in d.iter().enumerate() {
                if !(1 <= x && x <= top) {
                    return Err(Error::InvalidSbit);
                }
                v[i] = x;
            }
            doc.info.sbit = Meta::Present((v, n as u8));
        }
        Kind::Cicp => {
            if d[2] != 0 || d[3] > 1 {
                return Err(Error::InvalidCicp);
            }
            doc.info.cicp = Meta::Present([d[0], d[1], d[2], d[3]]);
        }
        Kind::Mdcv => {
            let mut v = [0u8; 24];
            v.copy_from_slice(d);
            doc.info.mdcv = Meta::Present(v);
        }
        Kind::Clli => {
            doc.info.clli = Meta::Present([be32(d, 0), be32(d, 4)]);
        }
        Kind::Bkgd => {
            let want = match ct {
                0 | 4 => 2,
                2 | 6 => 6,
                _ => 1,
            };
            if n != want {
                return Err(Error::InvalidChunkLength);
            }
            let mut v = [0u16; 3];
            if ct == 3 {
                if d[0] as usize >= doc.palette_entries() {
                    return Err(Error::InvalidBackground);
                }
                v[0] = d[0] as u16;
            } else {
                let mut i = 0;
                while i < n {
                    let x = be16(d, i);
                    if (x as u32) >> depth != 0 {
                        return Err(Error::InvalidBackground);
                    }
                    v[i / 2] = x;
                    i += 2;
                }
            }
            doc.info.bkgd = Meta::Present(v);
        }
        Kind::Hist => {
            if n != 2 * doc.palette_entries() {
                return Err(Error::InvalidHistogram);
            }
        }
        Kind::Phys => {
            if d[8] > 1 {
                return Err(Error::InvalidPhys);
            }
            doc.info.phys = Meta::Present((be32(d, 0), be32(d, 4), d[8]));
        }
        Kind::Splt => {
            let z = split_keyword(d, Error::InvalidSplt)?;
            if z + 1 >= n || !(d[z + 1] == 8 || d[z + 1] == 16) {
                return Err(Error::InvalidSplt);
            }
            let entry = if d[z + 1] == 8 { 6 } else { 10 };
            if (n - z - 2) % entry != 0 {
                return Err(Error::InvalidSplt);
            }
        }
        Kind::Exif => {
            if n < 4 || !(d[..4] == *b"II*\0" || d[..4] == *b"MM\0*") {
                return Err(Error::InvalidExif);
            }
            doc.info.exif = Meta::Present((dpos, n));
        }
        Kind::Time => {
            let (y, mo, dd, hh, mi, ss) = (be16(d, 0), d[2], d[3], d[4], d[5], d[6]);
            if !((1..=12).contains(&mo) && (1..=31).contains(&dd) && hh <= 23 && mi <= 59 && ss <= 60) {
                return Err(Error::InvalidTime);
            }
            doc.info.time = Meta::Present(Time { year: y, month: mo, day: dd, hour: hh, minute: mi, second: ss });
        }
        Kind::Text => {
            split_keyword(d, Error::InvalidText)?;
        }
        Kind::Ztxt => {
            let z = split_keyword(d, Error::InvalidText)?;
            if z + 1 >= n || d[z + 1] != 0 {
                return Err(Error::InvalidText);
            }
        }
        Kind::Itxt => {
            let z = split_keyword(d, Error::InvalidText)?;
            if z + 2 >= n || d[z + 1] > 1 || (d[z + 1] == 1 && d[z + 2] != 0) {
                return Err(Error::InvalidText);
            }
            match find_nul(d, z + 3) {
                Some(z2) => {
                    if find_nul(d, z2 + 1).is_none() {
                        return Err(Error::InvalidText);
                    }
                }
                None => return Err(Error::InvalidText),
            }
        }
        _ => {}
    }
    Ok(())
}
