//! The structural pass, SPEC P2–P8 and P14.1, under a policy record
//! (`../../policy-draft/policy_oracle.py` `_walk`, restricted to the values
//! Strict and RecommendedBeta use). No pixel is decoded and no zlib stream is
//! inflated here. The result is a fixed-size record: no chunk list and no frame
//! list (SPEC P20); data chunks are re-walked by offset (`reader.rs`).
//!
//! Check order per chunk: P3 (framing, letters), the IDAT phase, the CRC rule,
//! P5.1 duplicates, P5.2 ordering, P5.3 fixed lengths, then the chunk's own
//! section, exactly as the oracle's `_walk`.

use crate::crc::crc32;
use crate::limits::{MAX_DIMENSION, MAX_FRAMES, MAX_INPUT, MAX_OUTPUT, MAX_TOTAL_PIXELS};
use crate::png::{Error, Events, Info, Meta, Time};
use crate::policy::{
    AncillaryDuplicate, AncillaryOrder, AncillaryValues, ApngErrors, ApngInStill, CrcAncillary, Iend,
    MissingIend, PaletteSize, PlteUnexpected, Policy, Trailing, TrnsInvalid,
};

pub(crate) const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

#[inline]
pub(crate) fn be32(b: &[u8], p: usize) -> u32 {
    ((b[p] as u32) << 24) | ((b[p + 1] as u32) << 16) | ((b[p + 2] as u32) << 8) | (b[p + 3] as u32)
}

#[inline]
pub(crate) fn be16(b: &[u8], p: usize) -> u16 {
    ((b[p] as u16) << 8) | (b[p + 1] as u16)
}

/// One framed chunk (P3): its type, the offset and length of its data, the
/// offset of the next chunk and whether its CRC matched.
#[derive(Clone, Copy)]
pub(crate) struct Chunk {
    pub ty: [u8; 4],
    pub data: usize,
    pub len: usize,
    pub next: usize,
    pub crc_ok: bool,
}

fn letter(c: u8) -> bool {
    (0x41..=0x5A).contains(&c) || (0x61..=0x7A).contains(&c)
}

/// P3 steps 1–3: the framing of the chunk at `p`, without its CRC (`crc_ok` is
/// `false`). The re-walking readers use it on chunks the walk already checked.
pub(crate) fn frame_at(b: &[u8], p: usize) -> Result<Chunk, Error> {
    let len = b.len();
    if p > len || len - p < 8 {
        return Err(Error::TruncatedChunk);
    }
    let n = be32(b, p);
    let ty = [b[p + 4], b[p + 5], b[p + 6], b[p + 7]];
    // The PNG wire limit on chunk lengths (P3 step 1), `i32::MAX`.
    if n > i32::MAX as u32 {
        return Err(Error::ChunkTooLong);
    }
    if !(letter(ty[0]) && letter(ty[1]) && letter(ty[2]) && letter(ty[3])) {
        return Err(Error::InvalidChunkType);
    }
    let n = n as usize;
    // `p + 12 + n <= |b|`, written without overflow: `len - p - 8 >= n + 4`.
    if ((len - p - 8) as u64) < n as u64 + 4 {
        return Err(Error::TruncatedChunk);
    }
    Ok(Chunk { ty, data: p + 8, len: n, next: p + 12 + n, crc_ok: false })
}

/// P3 step 4: the CRC-32 over the type and data equals the stored value.
pub(crate) fn crc_matches(b: &[u8], c: &Chunk) -> bool {
    crc32(&b[c.data - 4..c.data + c.len]) == be32(b, c.data + c.len)
}

/// P3 steps 1–4: framing as `frame_at`, plus the CRC comparison (reported, not
/// failed: the caller applies `crc_critical`/`crc_ancillary`).
pub(crate) fn chunk_at(b: &[u8], p: usize) -> Result<Chunk, Error> {
    let mut c = frame_at(b, p)?;
    c.crc_ok = crc_matches(b, &c);
    Ok(c)
}

/// The chunk types the walk distinguishes. `Unknown(ancillary)` keeps bit 5 of
/// byte 0 (P3, P5.5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    Ihdr, Plte, Idat, Iend, Trns, Chrm, Gama, Iccp, Sbit, Srgb, Cicp, Mdcv, Clli, Bkgd, Hist, Phys, Splt,
    Exif, Time, Text, Ztxt, Itxt, Actl, Fctl, Fdat, Unknown(bool),
}

pub(crate) fn kind_of(t: &[u8; 4]) -> Kind {
    match t {
        b"IHDR" => Kind::Ihdr,
        b"PLTE" => Kind::Plte,
        b"IDAT" => Kind::Idat,
        b"IEND" => Kind::Iend,
        b"tRNS" => Kind::Trns,
        b"cHRM" => Kind::Chrm,
        b"gAMA" => Kind::Gama,
        b"iCCP" => Kind::Iccp,
        b"sBIT" => Kind::Sbit,
        b"sRGB" => Kind::Srgb,
        b"cICP" => Kind::Cicp,
        b"mDCV" => Kind::Mdcv,
        b"cLLI" => Kind::Clli,
        b"bKGD" => Kind::Bkgd,
        b"hIST" => Kind::Hist,
        b"pHYs" => Kind::Phys,
        b"sPLT" => Kind::Splt,
        b"eXIf" => Kind::Exif,
        b"tIME" => Kind::Time,
        b"tEXt" => Kind::Text,
        b"zTXt" => Kind::Ztxt,
        b"iTXt" => Kind::Itxt,
        b"acTL" => Kind::Actl,
        b"fcTL" => Kind::Fctl,
        b"fdAT" => Kind::Fdat,
        _ => Kind::Unknown(t[0] & 0x20 != 0),
    }
}

impl Kind {
    fn ancillary(self, t: &[u8; 4]) -> bool {
        match self {
            Kind::Unknown(a) => a,
            _ => t[0] & 0x20 != 0,
        }
    }
    /// The bit of the singleton set (P5.1); `None` for types that may repeat.
    fn single_bit(self) -> Option<u32> {
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
    fn before_plte(self) -> bool {
        matches!(self, Kind::Chrm | Kind::Gama | Kind::Iccp | Kind::Sbit | Kind::Srgb | Kind::Cicp | Kind::Mdcv | Kind::Clli)
    }
    fn after_plte(self) -> bool {
        matches!(self, Kind::Bkgd | Kind::Hist | Kind::Trns)
    }
    fn before_idat(self) -> bool {
        matches!(self, Kind::Exif | Kind::Phys | Kind::Splt | Kind::Actl)
    }
    fn fixed_len(self) -> Option<usize> {
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
    fn is_apng(self) -> bool {
        matches!(self, Kind::Actl | Kind::Fctl | Kind::Fdat)
    }
}

const AFTER_PLTE_BITS: u32 = (1 << 2) | (1 << 11) | (1 << 12);

/// The IHDR fields the pixel stages need (P4).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Ihdr {
    pub w: u32,
    pub h: u32,
    pub depth: u8,
    pub ct: u8,
    pub interlace: u8,
}

impl Ihdr {
    /// `C(ct)` (P4).
    pub fn channels(&self) -> u32 {
        match self.ct {
            0 => 1,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => 4,
        }
    }
    /// `B = C·d` bits per pixel.
    pub fn bits_pp(&self) -> u32 {
        self.channels() * self.depth as u32
    }
    /// Filter byte distance `bpp = max(1, B/8)`.
    pub fn bpp(&self) -> usize {
        let b = (self.bits_pp() / 8) as usize;
        if b == 0 { 1 } else { b }
    }
}

fn valid_depth(ct: u8, d: u8) -> bool {
    match ct {
        0 => matches!(d, 1 | 2 | 4 | 8 | 16),
        2 | 4 | 6 => matches!(d, 8 | 16),
        3 => matches!(d, 1 | 2 | 4 | 8),
        _ => false,
    }
}

/// P4, in order.
fn parse_ihdr(d: &[u8]) -> Result<Ihdr, Error> {
    let w = be32(d, 0);
    let h = be32(d, 4);
    let (depth, ct, cm, fm, im) = (d[8], d[9], d[10], d[11], d[12]);
    if !(1 <= w && w <= MAX_DIMENSION && 1 <= h && h <= MAX_DIMENSION) {
        return Err(Error::InvalidDimensions);
    }
    // `4·W·H ≤ MAX_OUTPUT` exactly in u64: W, H ≤ i32::MAX, so 4·W·H < u64::MAX.
    if 4 * (w as u64) * (h as u64) > MAX_OUTPUT {
        return Err(Error::OutputTooLarge);
    }
    if !valid_depth(ct, depth) {
        return Err(Error::InvalidColorTypeDepth);
    }
    if cm != 0 {
        return Err(Error::UnsupportedCompressionMethod);
    }
    if fm != 0 {
        return Err(Error::UnsupportedFilterMethod);
    }
    if im > 1 {
        return Err(Error::UnsupportedInterlaceMethod);
    }
    Ok(Ihdr { w, h, depth, ct, interlace: im })
}

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

/// P8.2 keywords: 1–79 bytes of `20–7E` or `A1–FF`, no leading, trailing or
/// consecutive spaces.
fn valid_keyword(k: &[u8]) -> bool {
    if k.is_empty() || k.len() > 79 {
        return false;
    }
    let mut prev_space = false;
    for &c in k {
        if !((0x20..=0x7E).contains(&c) || c >= 0xA1) {
            return false;
        }
        if c == 0x20 && prev_space {
            return false;
        }
        prev_space = c == 0x20;
    }
    k[0] != 0x20 && k[k.len() - 1] != 0x20
}

fn find_nul(d: &[u8], from: usize) -> Option<usize> {
    let mut i = from;
    while i < d.len() {
        if d[i] == 0 {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// `split_keyword`: the offset of the keyword's terminating NUL.
fn split_keyword(d: &[u8], err: Error) -> Result<usize, Error> {
    match find_nul(d, 0) {
        Some(z) if valid_keyword(&d[..z]) => Ok(z),
        _ => Err(err),
    }
}

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
    if b.len() as u64 > MAX_INPUT {
        return Err(Error::FileTooLarge.into());
    }
    if b.len() < 8 || b[..8] != SIGNATURE {
        return Err(Error::BadSignature.into());
    }
    let c = chunk_at(b, 8)?;
    if !c.crc_ok {
        return Err(Error::CrcMismatch.into());
    }
    if &c.ty != b"IHDR" {
        return Err(Error::MissingIhdr.into());
    }
    if c.len != 13 {
        return Err(Error::InvalidChunkLength.into());
    }
    let ihdr = parse_ihdr(&b[c.data..c.data + 13])?;
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
    let mut st = State { seen: 1, idat: 0, seq: 0, cur: None, frames: 0, fctl_before_idat: false, pixels: 0 };
    let apng_on = !apng_off && !(entry == Entry::Still && pol.apng_in_still == ApngInStill::Ignore);
    let static_frames = pol.apng_errors == ApngErrors::Static && entry == Entry::Frames;
    // sfail and apng_fail coincide for the values Strict and RecommendedBeta use.
    let sfail = |e: Error| -> WalkFail { if static_frames { WalkFail::Fallback } else { WalkFail::Format(e) } };
    let mut p = c.next;
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
        if st.idat == 1 && kind0 != Kind::Idat {
            st.idat = 2;
        }
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
        // ---- P5.1 duplicates
        if let Some(bit) = kind.single_bit() {
            if st.seen & bit != 0 {
                if anc && pol.ancillary_duplicate == AncillaryDuplicate::FirstWins {
                    discard(&mut doc, kind, ev);
                    continue;
                }
                if kind == Kind::Plte && pol.plte_unexpected == PlteUnexpected::Ignore && (ct == 0 || ct == 4) {
                    continue;
                }
                if kind.is_apng() {
                    return Err(sfail(Error::DuplicateChunk));
                }
                return Err(Error::DuplicateChunk.into());
            }
        }
        // ---- P5.2 ordering
        let plte_seen = doc.plte.is_some();
        let misplaced = (kind.before_plte() && (plte_seen || st.idat != 0))
            || (kind.after_plte() && (st.idat != 0 || (!plte_seen && (ct == 3 || kind == Kind::Hist))))
            || (kind.before_idat() && st.idat != 0);
        if misplaced {
            if kind == Kind::Actl {
                return Err(sfail(Error::ChunkOutOfOrder));
            }
            match pol.ancillary_order {
                AncillaryOrder::Drop => {
                    discard(&mut doc, kind, ev);
                    continue;
                }
                AncillaryOrder::Reject => return Err(Error::ChunkOutOfOrder.into()),
            }
        }
        // ---- P5.3 fixed lengths
        if kind == Kind::Iend && n != 0 && pol.iend == Iend::Lenient {
            n = 0;
        }
        if let Some(fl) = kind.fixed_len() {
            if n != fl {
                if anc && kind != Kind::Actl && pol.ancillary_values == AncillaryValues::Drop {
                    discard(&mut doc, kind, ev);
                    continue;
                }
                if kind == Kind::Actl {
                    return Err(sfail(Error::InvalidChunkLength));
                }
                return Err(Error::InvalidChunkLength.into());
            }
        }
        let d = &d[..n];
        if let Some(bit) = kind.single_bit() {
            st.seen |= bit;
        }
        // ---- the chunk's own section
        match kind {
            Kind::Ihdr => return Err(Error::DuplicateChunk.into()),
            Kind::Plte => {
                // P6, P5.2 (PLTE after IDAT or after bKGD/hIST/tRNS)
                if st.idat != 0 || st.seen & AFTER_PLTE_BITS != 0 {
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
