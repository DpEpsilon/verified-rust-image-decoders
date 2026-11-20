//! Entry points (SPEC P1, P15–P17): `try_decode_still`, `try_decode_still16`,
//! their policy variants, the frame iterator `Frames` and `try_icc_profile`;
//! the closed error list (P18); `Info` (P8.3).
//!
//! Every entry point runs the structural walk to IEND first (`walk.rs`); only
//! then does it reserve, through the sealed guard, in the order of SPEC P20.
//! M1b: ordinary Rust. The guard calls pass `Tracked::assume_new()` logs, as
//! GIF's M1 did; M3–M6 replace them with the sealed log and the proofs.

use alloc::vec::Vec;

use inflate_verus::limits::WINDOW_MAX;
use inflate_verus::{inflate_zlib_into, inflate_zlib_measure, InflateError, InflateOptions, SegmentSlices, Status};

use crate::alloc_guard::sites::{try_empty, TryError};
use crate::alloc_guard::{try_zeroed, AllocLog, AllocSite, OutOfMemory};
use crate::image::{decode_image, raw_size, Sink};
use crate::limits::MAX_ICC;
use crate::policy::{Policy, Profile, STRICT};
use crate::reader::{next_selected_fctl, FdatReader, IdatReader};
use crate::walk::{frame_at, Doc, Fctl, Ihdr};

use vstd::prelude::*;

verus! {

/// The closed, case-sensitive error set of P18 (62 names), in check order.
/// Allocation failure is not in this set (`TryError::OutOfMemory`, P20).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    FileTooLarge,
    BadSignature,
    TruncatedChunk,
    ChunkTooLong,
    InvalidChunkType,
    CrcMismatch,
    MissingIhdr,
    InvalidChunkLength,
    InvalidDimensions,
    OutputTooLarge,
    InvalidColorTypeDepth,
    UnsupportedCompressionMethod,
    UnsupportedFilterMethod,
    UnsupportedInterlaceMethod,
    UnknownCriticalChunk,
    DuplicateChunk,
    ChunkOutOfOrder,
    MissingPalette,
    UnexpectedPalette,
    InvalidPalette,
    NonConsecutiveIdat,
    MissingImageData,
    MissingIend,
    TrailingBytes,
    InvalidTransparency,
    InvalidGamma,
    InvalidSrgb,
    InvalidIccp,
    InvalidSbit,
    InvalidCicp,
    InvalidBackground,
    InvalidHistogram,
    InvalidPhys,
    InvalidSplt,
    InvalidExif,
    InvalidTime,
    InvalidText,
    MissingAnimationControl,
    InvalidAnimationControl,
    TooManyFrames,
    InvalidFrameControl,
    SequenceError,
    UnexpectedFrameData,
    MissingFrameData,
    FrameCountMismatch,
    PixelBudgetExceeded,
    InvalidZlibHeader,
    UnsupportedZlibMethod,
    ZlibPresetDictionary,
    InvalidBlockType,
    StoredLengthMismatch,
    InvalidCodeLengths,
    InvalidSymbol,
    DistanceTooFar,
    TruncatedZlib,
    AdlerMismatch,
    TrailingZlibData,
    TruncatedImageData,
    ExcessImageData,
    InvalidFilterType,
    PaletteIndexOutOfRange,
    IccProfileTooLarge,
}

impl Error {
    /// The P18 spelling. `'static`, never allocates.
    pub fn name(&self) -> &'static str {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::BadSignature => "BadSignature",
            Error::TruncatedChunk => "TruncatedChunk",
            Error::ChunkTooLong => "ChunkTooLong",
            Error::InvalidChunkType => "InvalidChunkType",
            Error::CrcMismatch => "CrcMismatch",
            Error::MissingIhdr => "MissingIhdr",
            Error::InvalidChunkLength => "InvalidChunkLength",
            Error::InvalidDimensions => "InvalidDimensions",
            Error::OutputTooLarge => "OutputTooLarge",
            Error::InvalidColorTypeDepth => "InvalidColorTypeDepth",
            Error::UnsupportedCompressionMethod => "UnsupportedCompressionMethod",
            Error::UnsupportedFilterMethod => "UnsupportedFilterMethod",
            Error::UnsupportedInterlaceMethod => "UnsupportedInterlaceMethod",
            Error::UnknownCriticalChunk => "UnknownCriticalChunk",
            Error::DuplicateChunk => "DuplicateChunk",
            Error::ChunkOutOfOrder => "ChunkOutOfOrder",
            Error::MissingPalette => "MissingPalette",
            Error::UnexpectedPalette => "UnexpectedPalette",
            Error::InvalidPalette => "InvalidPalette",
            Error::NonConsecutiveIdat => "NonConsecutiveIdat",
            Error::MissingImageData => "MissingImageData",
            Error::MissingIend => "MissingIend",
            Error::TrailingBytes => "TrailingBytes",
            Error::InvalidTransparency => "InvalidTransparency",
            Error::InvalidGamma => "InvalidGamma",
            Error::InvalidSrgb => "InvalidSrgb",
            Error::InvalidIccp => "InvalidIccp",
            Error::InvalidSbit => "InvalidSbit",
            Error::InvalidCicp => "InvalidCicp",
            Error::InvalidBackground => "InvalidBackground",
            Error::InvalidHistogram => "InvalidHistogram",
            Error::InvalidPhys => "InvalidPhys",
            Error::InvalidSplt => "InvalidSplt",
            Error::InvalidExif => "InvalidExif",
            Error::InvalidTime => "InvalidTime",
            Error::InvalidText => "InvalidText",
            Error::MissingAnimationControl => "MissingAnimationControl",
            Error::InvalidAnimationControl => "InvalidAnimationControl",
            Error::TooManyFrames => "TooManyFrames",
            Error::InvalidFrameControl => "InvalidFrameControl",
            Error::SequenceError => "SequenceError",
            Error::UnexpectedFrameData => "UnexpectedFrameData",
            Error::MissingFrameData => "MissingFrameData",
            Error::FrameCountMismatch => "FrameCountMismatch",
            Error::PixelBudgetExceeded => "PixelBudgetExceeded",
            Error::InvalidZlibHeader => "InvalidZlibHeader",
            Error::UnsupportedZlibMethod => "UnsupportedZlibMethod",
            Error::ZlibPresetDictionary => "ZlibPresetDictionary",
            Error::InvalidBlockType => "InvalidBlockType",
            Error::StoredLengthMismatch => "StoredLengthMismatch",
            Error::InvalidCodeLengths => "InvalidCodeLengths",
            Error::InvalidSymbol => "InvalidSymbol",
            Error::DistanceTooFar => "DistanceTooFar",
            Error::TruncatedZlib => "TruncatedZlib",
            Error::AdlerMismatch => "AdlerMismatch",
            Error::TrailingZlibData => "TrailingZlibData",
            Error::TruncatedImageData => "TruncatedImageData",
            Error::ExcessImageData => "ExcessImageData",
            Error::InvalidFilterType => "InvalidFilterType",
            Error::PaletteIndexOutOfRange => "PaletteIndexOutOfRange",
            Error::IccProfileTooLarge => "IccProfileTooLarge",
        }
    }

    /// The eleven inflate errors keep their names (P9, P10).
    pub(crate) fn from_inflate(e: InflateError) -> Error {
        match e {
            InflateError::TruncatedZlib => Error::TruncatedZlib,
            InflateError::InvalidZlibHeader => Error::InvalidZlibHeader,
            InflateError::UnsupportedZlibMethod => Error::UnsupportedZlibMethod,
            InflateError::ZlibPresetDictionary => Error::ZlibPresetDictionary,
            InflateError::InvalidBlockType => Error::InvalidBlockType,
            InflateError::StoredLengthMismatch => Error::StoredLengthMismatch,
            InflateError::InvalidCodeLengths => Error::InvalidCodeLengths,
            InflateError::InvalidSymbol => Error::InvalidSymbol,
            InflateError::DistanceTooFar => Error::DistanceTooFar,
            InflateError::AdlerMismatch => Error::AdlerMismatch,
            InflateError::TrailingZlibData => Error::TrailingZlibData,
        }
    }
}

} // verus!

verus! {

/// The `recovered` events reachable in RecommendedBeta (OPTIONS.md R1, R3,
/// R4; R2 and R5 are unreachable there). Strict never sets one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Events {
    pub r1: bool,
    pub r3: bool,
    pub r4: bool,
}

impl Events {
    pub(crate) open spec fn view(self) -> crate::document_model::Events {
        crate::document_model::Events {r1:self.r1,r3:self.r3,r4:self.r4}
    }
    pub(crate) fn discard_trns(&mut self, kind: crate::kind::Kind, ct: u8)
        ensures final(self).r1 == (old(self).r1 || (kind.view()==crate::walk_model::Name::Trns && (ct==0 || ct==2 || ct==3))),
            final(self).r3==old(self).r3,final(self).r4==old(self).r4,
    {
        if matches!(kind,crate::kind::Kind::Trns) && (ct==0 || ct==2 || ct==3) {self.r1=true;}
    }
    pub(crate) fn any(&self) -> bool {
        self.r1 || self.r3 || self.r4
    }
    /// The event names in sorted order (`None` for an event that did not occur).
    #[cfg(feature = "test-records")]
    fn names(&self) -> [Option<&'static str>; 3] {
        [
            if self.r1 { Some("R1") } else { None },
            if self.r3 { Some("R3") } else { None },
            if self.r4 { Some("R4") } else { None },
        ]
    }
}

/// Three-valued metadata access (PNG-Q4, P8.3): absent, discarded by the
/// profile, or present. Strict never reports `Discarded` (it fails instead).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meta<T> {
    Absent,
    Discarded,
    Present(T),
}

impl<T> Meta<T> {
    pub(crate) fn discard(&mut self)
        ensures *final(self) == match *old(self) {Self::Absent=>Self::Discarded,other=>other},
    {
        if let Meta::Absent = self {
            *self = Meta::Discarded;
        }
    }
}

/// tIME (P8.1), raw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// P8.3: the IHDR fields and the raw values of the colour and metadata chunks.
/// Byte ranges are `(offset, length)` into the caller's input; nothing is
/// copied. No ancillary value except tRNS changes a pixel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub interlace: u8,
    /// PLTE entries (0 without PLTE).
    pub palette_entries: u32,
    pub trns: Meta<()>,
    pub gama: Meta<u32>,
    pub chrm: Meta<[u32; 8]>,
    pub srgb: Meta<u8>,
    /// The iCCP profile name's byte range.
    pub iccp_name: Meta<(usize, usize)>,
    /// sBIT values and their count (1–4).
    pub sbit: Meta<([u8; 4], u8)>,
    pub cicp: Meta<[u8; 4]>,
    pub mdcv: Meta<[u8; 24]>,
    pub clli: Meta<[u32; 2]>,
    /// bKGD: the palette index, the grey value, or the RGB values.
    pub bkgd: Meta<[u16; 3]>,
    /// pHYs: x and y pixels per unit, unit.
    pub phys: Meta<(u32, u32, u8)>,
    /// eXIf's byte range.
    pub exif: Meta<(usize, usize)>,
    pub time: Meta<Time>,
    /// acTL `num_plays` (raw), for the frame iterator; `None` without acTL.
    pub num_plays: Option<u32>,
    /// fcTL frames in the stream (0 without acTL).
    pub frame_count: u32,
    /// Whether the default image is frame 0 (an fcTL precedes the first IDAT).
    pub default_is_frame: bool,
}

impl Info {
    pub(crate) fn new(h: &Ihdr) -> (r: Info)
        ensures r.view() == crate::document_model::initial_info(h.view()),r.ranges_wf(0),
    {
        let r=Info {
            width: h.w,
            height: h.h,
            bit_depth: h.depth,
            color_type: h.ct,
            interlace: h.interlace,
            palette_entries: 0,
            trns: Meta::Absent,
            gama: Meta::Absent,
            chrm: Meta::Absent,
            srgb: Meta::Absent,
            iccp_name: Meta::Absent,
            sbit: Meta::Absent,
            cicp: Meta::Absent,
            mdcv: Meta::Absent,
            clli: Meta::Absent,
            bkgd: Meta::Absent,
            phys: Meta::Absent,
            exif: Meta::Absent,
            time: Meta::Absent,
            num_plays: None,
            frame_count: 0,
            default_is_frame: false,
        };
        proof { assert(r.view().slots =~= crate::document_model::initial_info(h.view()).slots); }
        r
    }
}

} // verus!

/// The default image as RGBA8 (P15): `4·width·height` bytes, top row first,
/// non-premultiplied.
#[derive(Debug, Clone)]
pub struct Still {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub info: Info,
}

/// The default image as RGBA16 (P13.3): `8·width·height` bytes, four samples
/// per pixel, each big-endian.
#[derive(Debug, Clone)]
pub struct Still16 {
    pub width: u32,
    pub height: u32,
    pub rgba16: Vec<u8>,
    pub info: Info,
}

/// The reservation requests a call made, in order (P20 `Req`), whether or not
/// each succeeded. A fixed-size runtime record; the sealed `AllocLog` is its
/// ghost counterpart once the proofs land.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Trace {
    sites: [(AllocSite, usize); 3],
    n: usize,
}

impl Trace {
    fn new() -> Trace {
        Trace { sites: [(AllocSite::Raw, 0); 3], n: 0 }
    }
    fn record(&mut self, site: AllocSite, elements: usize) {
        if self.n < 3 {
            self.sites[self.n] = (site, elements);
            self.n += 1;
        }
    }
    /// The requests in order: the first `n` entries.
    #[cfg(feature = "test-records")]
    fn list(&self) -> ([(AllocSite, usize); 3], usize) {
        (self.sites, self.n)
    }
}

fn oom(o: OutOfMemory) -> TryError {
    TryError::OutOfMemory(o)
}

/// The checked still/ICC structural walk, before any reservation.
fn still_walk(b: &[u8], pol: &Policy, ev: &mut Events) -> Result<Doc, TryError> {
    crate::walk::still(b,pol,ev).map_err(TryError::Format)
}

/// `Raw` then `Output` (P20), after a successful walk. `out_bytes` is `4·N` or
/// `8·N`; both requests are at most `MAX_BYTES` by `caps_ok`.
fn reserve_image(doc: &Doc, out_bytes: usize, trace: &mut Trace) -> Result<(Vec<u8>, Vec<u8>), TryError> {
    let r = raw_size(&doc.ihdr, doc.ihdr.w, doc.ihdr.h) as usize;
    trace.record(AllocSite::Raw, r);
    let raw = try_zeroed(AllocSite::Raw, r, Tracked::<&mut AllocLog>::assume_new()).map_err(oom)?;
    trace.record(AllocSite::Output, out_bytes);
    let out = try_zeroed(AllocSite::Output, out_bytes, Tracked::<&mut AllocLog>::assume_new()).map_err(oom)?;
    Ok((raw, out))
}

/// P15 for RGBA8 (`wide = false`) or RGBA16: walk, reserve, decode.
fn still_impl(b: &[u8], pol: &Policy, wide: bool, ev: &mut Events, trace: &mut Trace) -> Result<(Doc, Vec<u8>), TryError> {
    let doc = still_walk(b, pol, ev)?;
    let n = doc.ihdr.w as usize * doc.ihdr.h as usize;
    let (mut raw, mut out) = reserve_image(&doc, if wide { 8 * n } else { 4 * n }, trace)?;
    let mut src = IdatReader::for_doc(b, &doc);
    let mut sink = if wide { Sink::Rgba16(&mut out) } else { Sink::Rgba8(&mut out) };
    decode_image(&doc, b, &mut src, &mut raw, doc.ihdr.w, doc.ihdr.h, pol, ev, &mut sink).map_err(TryError::Format)?;
    drop(raw);
    Ok((doc, out))
}

fn still8(b: &[u8], pol: &Policy, ev: &mut Events, trace: &mut Trace) -> Result<Still, TryError> {
    let (doc, rgba) = still_impl(b, pol, false, ev, trace)?;
    Ok(Still { width: doc.ihdr.w, height: doc.ihdr.h, rgba, info: doc.info })
}

fn still16(b: &[u8], pol: &Policy, ev: &mut Events, trace: &mut Trace) -> Result<Still16, TryError> {
    let (doc, rgba16) = still_impl(b, pol, true, ev, trace)?;
    Ok(Still16 { width: doc.ihdr.w, height: doc.ihdr.h, rgba16, info: doc.info })
}

/// `decode_still` (P15): the default image as RGBA8, Strict.
pub fn try_decode_still(b: &[u8]) -> Result<Still, TryError> {
    still8(b, &STRICT, &mut Events::default(), &mut Trace::new())
}

/// `decode_still16` (P15, P13.3): the default image as RGBA16, Strict.
pub fn try_decode_still16(b: &[u8]) -> Result<Still16, TryError> {
    still16(b, &STRICT, &mut Events::default(), &mut Trace::new())
}

/// The profiled RGBA8 still; the `bool` is `recovered` (OPTIONS.md R1–R5).
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> Result<(Still, bool), TryError> {
    let mut ev = Events::default();
    let s = still8(b, &p.policy(), &mut ev, &mut Trace::new())?;
    Ok((s, ev.any()))
}

/// The profiled RGBA16 still (PNG-Q13 (i)); the `bool` is `recovered`.
pub fn try_decode_still16_with_policy(b: &[u8], p: Profile) -> Result<(Still16, bool), TryError> {
    let mut ev = Events::default();
    let s = still16(b, &p.policy(), &mut ev, &mut Trace::new())?;
    Ok((s, ev.any()))
}

/// Test-only (feature `test-records`): the profiled RGBA8 still with its event
/// names (sorted) and the request trace.
#[cfg(feature = "test-records")]
#[doc(hidden)]
pub fn try_decode_still_traced(b: &[u8], p: Profile) -> (Result<(Still, [Option<&'static str>; 3]), TryError>, ([(AllocSite, usize); 3], usize)) {
    let mut ev = Events::default();
    let mut trace = Trace::new();
    let r = still8(b, &p.policy(), &mut ev, &mut trace).map(|s| (s, ev.names()));
    (r, trace.list())
}

/// Test-only: the profiled RGBA16 still with its event names.
#[cfg(feature = "test-records")]
#[doc(hidden)]
pub fn try_decode_still16_traced(b: &[u8], p: Profile) -> (Result<(Still16, [Option<&'static str>; 3]), TryError>, ([(AllocSite, usize); 3], usize)) {
    let mut ev = Events::default();
    let mut trace = Trace::new();
    let r = still16(b, &p.policy(), &mut ev, &mut trace).map(|s| (s, ev.names()));
    (r, trace.list())
}

// ------------------------------------------------------------------ P16 frames

/// One frame (P16): the whole canvas after the frame is composited, borrowed
/// from the decoder, plus the frame's fields. `rect` is `(x, y, w, h)`; `delay`
/// is the raw `(delay_num, delay_den)` (`None` without acTL).
#[derive(Debug)]
pub struct FrameView<'f> {
    pub rgba: &'f [u8],
    pub rect: (u32, u32, u32, u32),
    pub delay: Option<(u16, u16)>,
    pub dispose: u8,
    pub blend: u8,
    pub recovered: bool,
    #[cfg_attr(not(feature = "test-records"), allow(dead_code))]
    events: Events,
}

impl FrameView<'_> {
    /// Test-only: this frame's event names (sorted).
    #[cfg(feature = "test-records")]
    #[doc(hidden)]
    pub fn event_names(&self) -> [Option<&'static str>; 3] {
        self.events.names()
    }
}

#[derive(Clone, Copy)]
enum FState {
    /// No acTL, or the R4 fallback: the default image is the only frame.
    Single,
    /// The animation: `k` frames returned so far; the next fcTL is searched
    /// from `cursor`; `prev` is frame `k − 1`'s rectangle and effective
    /// disposal.
    Anim { k: u32, cursor: usize, prev: Option<((u32, u32, u32, u32), u8)> },
    /// After `Ok(None)` or any returned error (fused, D3).
    Done,
}

/// The frame decoder (P16). Borrows the input; owns `Raw` (filtered data of the
/// full image, reused by every frame), `Output` (the canvas) and, from step S
/// on, `Saved`. Not an `Iterator`: each view borrows the canvas.
pub struct Frames<'a> {
    b: &'a [u8],
    pol: Policy,
    doc: Doc,
    raw: Vec<u8>,
    canvas: Vec<u8>,
    saved: Option<Vec<u8>>,
    base: Events,
    state: FState,
}

impl<'a> Frames<'a> {
    /// The Strict frame decoder (D1: the unqualified entry is Strict). Defined
    /// as `try_new_with_policy(b, Profile::Strict)`, with the same errors and
    /// guard requests (council M1BQ-Q5b; THEOREMS.md writes it
    /// `Frames::try_new(b, Strict)`).
    pub fn try_new(b: &'a [u8]) -> Result<Frames<'a>, TryError> {
        Frames::try_new_with_policy(b, Profile::Strict)
    }

    /// The frame decoder under a named profile: the walk (with the R4 fallback
    /// of `apng_errors = Static`), `Raw` and `Output`, and the default image;
    /// its errors are the constructor's. THEOREMS.md writes it
    /// `Frames::try_new(b, p)` (council M1BQ-Q5b).
    pub fn try_new_with_policy(b: &'a [u8], p: Profile) -> Result<Frames<'a>, TryError> {
        Frames::with_record(b, p.policy())
    }

    /// Test-only: the constructor under an arbitrary record (D1: arbitrary
    /// records are never reachable from a consumer). Used for the council's
    /// mixed-record cases (M1BQ-Q1 record scope, M1BQ-Q2).
    #[cfg(feature = "test-records")]
    #[doc(hidden)]
    pub fn try_new_with_record(b: &'a [u8], pol: Policy) -> Result<Frames<'a>, TryError> {
        Frames::with_record(b, pol)
    }

    fn with_record(b: &'a [u8], pol: Policy) -> Result<Frames<'a>, TryError> {
        let mut ev = Events::default();
        let doc = crate::walk::frames(b,&pol,&mut ev).map_err(TryError::Format)?;
        let mut trace = Trace::new();
        let n = doc.ihdr.w as usize * doc.ihdr.h as usize;
        let (mut raw, mut canvas) = reserve_image(&doc, 4 * n, &mut trace)?;
        let mut src = IdatReader::for_doc(b, &doc);
        decode_image(&doc, b, &mut src, &mut raw, doc.ihdr.w, doc.ihdr.h, &pol, &mut ev, &mut Sink::Rgba8(&mut canvas))
            .map_err(TryError::Format)?;
        let mut doc = doc;
        let state = match doc.actl {
            None => FState::Single,
            Some(a) => {
                doc.info.num_plays = Some(a.num_plays);
                if !doc.info.default_is_frame {
                    // A default image outside the animation is validated, not shown (P16).
                    canvas.fill(0);
                }
                FState::Anim { k: 0, cursor: frame_at(b, 8).map(|c| c.next).unwrap_or(b.len()), prev: None }
            }
        };
        Ok(Frames { b, pol, doc, raw, canvas, saved: None, base: ev, state })
    }

    /// IHDR, the colour chunks and `num_plays` (P8.3).
    pub fn info(&self) -> &Info {
        &self.doc.info
    }

    /// The next frame, `Ok(None)` after the last one, or the error that ends
    /// the sequence. Fused: after `Ok(None)` or any error every later call
    /// returns `Ok(None)` without allocating or decoding (D3).
    pub fn try_next(&mut self) -> Result<Option<FrameView<'_>>, TryError> {
        let w = self.doc.ihdr.w;
        let h = self.doc.ihdr.h;
        match self.state {
            FState::Done => Ok(None),
            FState::Single => {
                self.state = FState::Done;
                Ok(Some(FrameView {
                    rgba: &self.canvas,
                    rect: (0, 0, w, h),
                    delay: None,
                    dispose: 0,
                    blend: 0,
                    recovered: self.base.any(),
                    events: self.base,
                }))
            }
            FState::Anim { k, cursor, prev } => {
                if k >= self.doc.info.frame_count {
                    self.state = FState::Done;
                    return Ok(None);
                }
                let fpos = match next_selected_fctl(self.b,&self.doc,k,cursor) {
                    Some(q) => q,
                    None => {
                        self.state = FState::Done;
                        return Ok(None);
                    }
                };
                // The walk checked this chunk's framing, length 26 and CRC.
                let c = match frame_at(self.b, fpos) {
                    Ok(c) if c.len == 26 => c,
                    _ => {
                        self.state = FState::Done;
                        return Ok(None);
                    }
                };
                let f = Fctl::parse(&self.b[c.data..c.data + 26]);
                self.state = FState::Done;
                // Step S (P20, D9): Saved, once, at the start of the first
                // reached call for a frame k > 0 with dispose_op = 2.
                if k > 0 && f.dispose == 2 && self.saved.is_none() {
                    let n = self.canvas.len();
                    match try_zeroed(AllocSite::Saved, n, Tracked::<&mut AllocLog>::assume_new()) {
                        Ok(v) => self.saved = Some(v),
                        Err(o) => return Err(oom(o)),
                    }
                }
                // P14.3: frame k − 1's disposal.
                if let Some(((px, py, pw, ph), op)) = prev {
                    if op == 1 {
                        clear_rect(&mut self.canvas, w, px, py, pw, ph);
                    } else if op == 2 {
                        if let Some(s) = &self.saved {
                            self.canvas.copy_from_slice(s);
                        }
                    }
                }
                let op = if k == 0 && f.dispose == 2 { 1 } else { f.dispose };
                if op == 2 {
                    if let Some(s) = self.saved.as_mut() {
                        s.copy_from_slice(&self.canvas);
                    }
                }
                let mut fev = self.base;
                if k == 0 && self.doc.info.default_is_frame {
                    // Frame 0 is the default image, already on the canvas. OVER onto
                    // the transparent canvas equals the source except that a pixel
                    // with alpha 0 becomes (0,0,0,0) (P14.4 with d = (0,0,0,0)).
                    if f.blend == 1 {
                        let mut i = 0;
                        while i + 3 < self.canvas.len() {
                            if self.canvas[i + 3] == 0 {
                                self.canvas[i] = 0;
                                self.canvas[i + 1] = 0;
                                self.canvas[i + 2] = 0;
                            }
                            i += 4;
                        }
                    }
                } else {
                    let mut src = FdatReader::for_frame(self.b,&self.doc,k,c.next);
                    let mut sink = Sink::Canvas { canvas: &mut self.canvas, cw: w, fx: f.x, fy: f.y, over: f.blend == 1 };
                    if let Err(e) = decode_image(&self.doc, self.b, &mut src, &mut self.raw, f.w, f.h, &self.pol, &mut fev, &mut sink) {
                        return Err(TryError::Format(e));
                    }
                }
                self.state = FState::Anim { k: k + 1, cursor: c.next, prev: Some(((f.x, f.y, f.w, f.h), op)) };
                Ok(Some(FrameView {
                    rgba: &self.canvas,
                    rect: (f.x, f.y, f.w, f.h),
                    delay: Some((f.delay_num, f.delay_den)),
                    dispose: f.dispose,
                    blend: f.blend,
                    recovered: fev.any(),
                    events: fev,
                }))
            }
        }
    }
}

/// P14.3 disposal 1: the rectangle becomes `(0,0,0,0)`.
fn clear_rect(canvas: &mut [u8], cw: u32, x: u32, y: u32, w: u32, h: u32) {
    let mut row = 0;
    while row < h {
        let o = 4 * ((y + row) as usize * cw as usize + x as usize);
        canvas[o..o + 4 * w as usize].fill(0);
        row += 1;
    }
}

// ------------------------------------------------------------------ P17 ICC

fn icc_impl(b: &[u8], trace: &mut Trace) -> Result<Option<Vec<u8>>, TryError> {
    let mut ev = Events::default();
    let doc = still_walk(b, &STRICT, &mut ev)?;
    let (o, n) = match doc.iccp {
        None => return Ok(None),
        Some(r) => r,
    };
    let parts: [&[u8]; 1] = [&b[o..o + n]];
    let opts = InflateOptions::STRICT;
    // First pass: measure into the WINDOW_MAX ring (D7).
    trace.record(AllocSite::IccWindow, WINDOW_MAX);
    let mut ring = try_zeroed(AllocSite::IccWindow, WINDOW_MAX, Tracked::<&mut AllocLog>::assume_new()).map_err(oom)?;
    let m = {
        let mut src = match SegmentSlices::new(&parts) {
            Some(s) => s,
            None => return Err(TryError::Format(Error::TruncatedZlib)),
        };
        let r = inflate_zlib_measure(&mut src, &mut ring, MAX_ICC, opts);
        match r.status {
            Status::Done => r.produced as usize,
            Status::Full => return Err(TryError::Format(Error::IccProfileTooLarge)),
            Status::Err(e) => return Err(TryError::Format(Error::from_inflate(e))),
        }
    };
    drop(ring);
    // Second pass into exactly m bytes; it ends Done (L-LIMIT-MONO).
    trace.record(AllocSite::Icc, m);
    let mut prof = if m == 0 {
        try_empty(AllocSite::Icc, Tracked::<&mut AllocLog>::assume_new()).map_err(oom)?
    } else {
        try_zeroed(AllocSite::Icc, m, Tracked::<&mut AllocLog>::assume_new()).map_err(oom)?
    };
    let mut src = match SegmentSlices::new(&parts) {
        Some(s) => s,
        None => return Err(TryError::Format(Error::TruncatedZlib)),
    };
    let r = inflate_zlib_into(&mut src, &mut prof, opts);
    match r.status {
        Status::Done => Ok(Some(prof)),
        Status::Full => Err(TryError::Format(Error::IccProfileTooLarge)),
        Status::Err(e) => Err(TryError::Format(Error::from_inflate(e))),
    }
}

/// `icc_profile` (P17, Strict): `Ok(None)` without iCCP, otherwise the profile
/// inflated with limit `MAX_ICC`. A damaged profile is an error of this call
/// only; no pixel entry point inflates it.
pub fn try_icc_profile(b: &[u8]) -> Result<Option<Vec<u8>>, TryError> {
    icc_impl(b, &mut Trace::new())
}

/// Test-only: `try_icc_profile` with its request trace.
#[cfg(feature = "test-records")]
#[doc(hidden)]
pub fn try_icc_profile_traced(b: &[u8]) -> (Result<Option<Vec<u8>>, TryError>, ([(AllocSite, usize); 3], usize)) {
    let mut trace = Trace::new();
    let r = icc_impl(b, &mut trace);
    (r, trace.list())
}
