//! Entry points (SPEC.md G1, G14, G15, G20): `try_decode_still`, the frame
//! decoder `Frames`, and `plays`.
//!
//! One stream walk serves both entry points. `Frames::try_new*` checks G2–G4,
//! reserves `Canvas` and `LzwTable`, then walks the blocks up to the first image
//! separator (the constructor's prefix walk, G15), which fixes `info()` and its
//! `loop_count`. Each `next_frame()` continues the walk through one image and
//! returns a view of the whole canvas. The still entry point runs the same
//! walk through the first image and moves the canvas out as `Still::rgba`.
//!
//! An error inside a frame leaves the canvas partly drawn, but the decoder is
//! then fused and the canvas is never exposed again (G12.4), so no per-frame
//! copy is made.

use alloc::vec::Vec;

use crate::alloc_guard::sites::TryError;
use crate::alloc_guard::{try_zeroed, AllocLog, AllocSite};
use crate::compose::{clear_rect, copy_rect, Drawer, Rect};
use crate::lzw::{self, BitReader, LzwOptions, Tail, LZW_TABLE_BYTES};
use crate::parse::{self, Header, WalkState};
use crate::policy::{GceOrphan, LzwMin, LzwTail, MissingTrailer, Policy, Profile, Trailing, UnknownBlock};

use vstd::prelude::*;

verus! {

/// `Req_new(b) = Req_still(b)`: the two reservations of a still call or of the
/// iterator constructor, `[(Canvas, 4·N), (LzwTable, LZW_TABLE_BYTES)]`, and
/// only when G2–G4 pass (`ReqLzw = [(LzwTable, 20,480)]`, G18, D4).
pub open spec fn req_new(b: Seq<u8>) -> Seq<(AllocSite, nat)> {
    match crate::model::mh(b) {
        Ok(m) => seq![
            (AllocSite::Canvas, (4 * m.width * m.height) as nat),
            (AllocSite::LzwTable, crate::lzw::LZW_TABLE_BYTES as nat),
        ],
        Err(_) => Seq::empty(),
    }
}

/// `PreAlloc` for the constructor (THEOREMS-ALLOC-DELTA.md §0): `Req_new` is
/// empty exactly when `MH` rejects the input.
pub open spec fn pre_alloc_new(b: Seq<u8>) -> bool {
    req_new(b).len() == 0
}

/// Clause 2 of `G` for the constructor (THEOREMS.md A-GIF-2/A-GIF-6): an
/// `OutOfMemory` result has a non-empty log whose requests are a prefix of
/// `Req_new`, every earlier entry `Ok`, and the last entry the failed request.
pub open spec fn g_new_clause2(b: Seq<u8>, r_is_oom: bool, l: Seq<(AllocSite, nat, bool)>) -> bool {
    if r_is_oom {
        l.len() >= 1 && crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b)) && (forall|j: int|
            0 <= j < l.len() - 1 ==> l[j].2) && !l[l.len() - 1].2
    } else {
        crate::alloc_guard::all_ok(l)
    }
}

/// Clause 3 of `G`: every request of `Req_new` is within its site's maximum,
/// proved from the checked header before each guard call.
pub open spec fn g_new_clause3(b: Seq<u8>) -> bool {
    forall|j: int| 0 <= j < req_new(b).len() ==> req_new(b)[j].1 <= crate::alloc_guard::MAX_BYTES
}

/// The closed G16 error set (33 names). `name()` returns the exact,
/// case-sensitive spelling. Allocation failure is not in this set (G18); it is
/// `TryError::OutOfMemory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FileTooLarge,
    TruncatedHeader,
    BadSignature,
    UnsupportedVersion,
    InvalidDimensions,
    OutputTooLarge,
    TruncatedColorTable,
    MissingTrailer,
    UnknownBlock,
    TruncatedExtension,
    DuplicateGraphicControl,
    InvalidGraphicControl,
    UndefinedDisposal,
    InvalidApplicationExtension,
    InvalidPlainText,
    GraphicControlWithoutGraphic,
    TooManyFrames,
    TruncatedImageDescriptor,
    InvalidFrameDimensions,
    FrameOutsideScreen,
    PixelBudgetExceeded,
    NoColorTable,
    TruncatedImageData,
    InvalidLzwMinCodeSize,
    InvalidLzwCode,
    TooManyPixels,
    TooFewPixels,
    MissingEndOfInformation,
    DataAfterEndOfInformation,
    ColorIndexOutOfRange,
    TrailingBytes,
    NoImage,
    UnsupportedGraphicControlTarget,
}

impl Error {
    /// The G16 spelling table, `open` and field-free so it is well-formed
    /// everywhere, including in `alloc_guard::sites`.
    #[doc(hidden)]
    pub open spec fn name_spec(&self) -> &'static str {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedHeader => "TruncatedHeader",
            Error::BadSignature => "BadSignature",
            Error::UnsupportedVersion => "UnsupportedVersion",
            Error::InvalidDimensions => "InvalidDimensions",
            Error::OutputTooLarge => "OutputTooLarge",
            Error::TruncatedColorTable => "TruncatedColorTable",
            Error::MissingTrailer => "MissingTrailer",
            Error::UnknownBlock => "UnknownBlock",
            Error::TruncatedExtension => "TruncatedExtension",
            Error::DuplicateGraphicControl => "DuplicateGraphicControl",
            Error::InvalidGraphicControl => "InvalidGraphicControl",
            Error::UndefinedDisposal => "UndefinedDisposal",
            Error::InvalidApplicationExtension => "InvalidApplicationExtension",
            Error::InvalidPlainText => "InvalidPlainText",
            Error::GraphicControlWithoutGraphic => "GraphicControlWithoutGraphic",
            Error::TooManyFrames => "TooManyFrames",
            Error::TruncatedImageDescriptor => "TruncatedImageDescriptor",
            Error::InvalidFrameDimensions => "InvalidFrameDimensions",
            Error::FrameOutsideScreen => "FrameOutsideScreen",
            Error::PixelBudgetExceeded => "PixelBudgetExceeded",
            Error::NoColorTable => "NoColorTable",
            Error::TruncatedImageData => "TruncatedImageData",
            Error::InvalidLzwMinCodeSize => "InvalidLzwMinCodeSize",
            Error::InvalidLzwCode => "InvalidLzwCode",
            Error::TooManyPixels => "TooManyPixels",
            Error::TooFewPixels => "TooFewPixels",
            Error::MissingEndOfInformation => "MissingEndOfInformation",
            Error::DataAfterEndOfInformation => "DataAfterEndOfInformation",
            Error::ColorIndexOutOfRange => "ColorIndexOutOfRange",
            Error::TrailingBytes => "TrailingBytes",
            Error::NoImage => "NoImage",
            Error::UnsupportedGraphicControlTarget => "UnsupportedGraphicControlTarget",
        }
    }

    /// `'static`, never allocates.
    pub fn name(&self) -> (s: &'static str)
        ensures s@ == self.name_spec()@,
    {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedHeader => "TruncatedHeader",
            Error::BadSignature => "BadSignature",
            Error::UnsupportedVersion => "UnsupportedVersion",
            Error::InvalidDimensions => "InvalidDimensions",
            Error::OutputTooLarge => "OutputTooLarge",
            Error::TruncatedColorTable => "TruncatedColorTable",
            Error::MissingTrailer => "MissingTrailer",
            Error::UnknownBlock => "UnknownBlock",
            Error::TruncatedExtension => "TruncatedExtension",
            Error::DuplicateGraphicControl => "DuplicateGraphicControl",
            Error::InvalidGraphicControl => "InvalidGraphicControl",
            Error::UndefinedDisposal => "UndefinedDisposal",
            Error::InvalidApplicationExtension => "InvalidApplicationExtension",
            Error::InvalidPlainText => "InvalidPlainText",
            Error::GraphicControlWithoutGraphic => "GraphicControlWithoutGraphic",
            Error::TooManyFrames => "TooManyFrames",
            Error::TruncatedImageDescriptor => "TruncatedImageDescriptor",
            Error::InvalidFrameDimensions => "InvalidFrameDimensions",
            Error::FrameOutsideScreen => "FrameOutsideScreen",
            Error::PixelBudgetExceeded => "PixelBudgetExceeded",
            Error::NoColorTable => "NoColorTable",
            Error::TruncatedImageData => "TruncatedImageData",
            Error::InvalidLzwMinCodeSize => "InvalidLzwMinCodeSize",
            Error::InvalidLzwCode => "InvalidLzwCode",
            Error::TooManyPixels => "TooManyPixels",
            Error::TooFewPixels => "TooFewPixels",
            Error::MissingEndOfInformation => "MissingEndOfInformation",
            Error::DataAfterEndOfInformation => "DataAfterEndOfInformation",
            Error::ColorIndexOutOfRange => "ColorIndexOutOfRange",
            Error::TrailingBytes => "TrailingBytes",
            Error::NoImage => "NoImage",
            Error::UnsupportedGraphicControlTarget => "UnsupportedGraphicControlTarget",
        }
    }
}

} // verus!

verus! {

/// The header's version string (G2). It gates no later rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V87a,
    V89a,
}

impl Version {
    pub open spec fn as_str_spec(&self) -> &'static str {
        match self {
            Version::V87a => "87a",
            Version::V89a => "89a",
        }
    }

    pub fn as_str(&self) -> (s: &'static str)
        ensures s@ == self.as_str_spec()@,
    {
        match self {
            Version::V87a => "87a",
            Version::V89a => "89a",
        }
    }
}

/// G18's reservation prefix of the still call and of the constructor: after
/// G2–G4, `try_zeroed(Canvas, 4·W·H)` then
/// `try_zeroed(LzwTable, LZW_TABLE_BYTES)`, in that order, and nothing else.
/// Its `ensures` states A-GIF-3's clause 3 and the log shape A-GIF-2 needs,
/// proved from `parse_header`'s checked values (`hdr_inv`); no axiom is used.
fn reserve(
    b: &[u8],
    hdr: &Header,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (r: Result<(Vec<u8>, Vec<u8>), TryError>)
    requires
        crate::parse::hdr_inv(b@, *hdr),
        crate::parse::hdr_ok(b@, *hdr),
        log.view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        r is Ok ==> crate::alloc_guard::req_of_log(final(log).view()) == req_new(b@) && crate::alloc_guard::all_ok(
            final(log).view(),
        ),
        r is Err ==> g_new_clause2(b@, true, final(log).view()),
        r is Err ==> pre_alloc_new(b@) == false,
{
    proof {
        reveal(req_new);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        reveal(g_new_clause2);
        reveal(pre_alloc_new);
        reveal(crate::parse::hdr_ok);
        reveal(crate::parse::hdr_inv);
    }
    // 4·W·H <= MAX_OUTPUT = alloc_guard::MAX_BYTES, and >= 4 (hdr_inv).
    let w = hdr.width as u32;
    let h = hdr.height as u32;
    proof {
        assert(1 <= hdr.width as int <= crate::model::max_dimension());
        assert(1 <= hdr.height as int <= crate::model::max_dimension());
        assert(crate::model::max_dimension() == 65535);
        assert(w == hdr.width);
        assert(h == hdr.height);
        assert(1 <= w <= 65535);
        assert(1 <= h <= 65535);
        // `hdr_ok` gives the model record, and `mh_bounds` its area cap.
        let m = choose|m: crate::model::HeaderM| crate::model::mh(b@) == Ok(m) && crate::parse::hdr_matches(b@, m, *hdr);
        crate::model::lemma_mh_ok_bounds(b@, m);
        reveal(crate::parse::hdr_matches);
        assert(4 * (w as int) * (h as int) <= crate::model::max_output());
        crate::model::lemma_4wh_usize(w, h);
        assert(crate::model::max_output() == crate::alloc_guard::MAX_BYTES);
    }
    let n = 4 * (w as usize) * (h as usize);
    proof {
        let m = choose|m: crate::model::HeaderM| crate::model::mh(b@) == Ok(m) && crate::parse::hdr_matches(b@, m, *hdr);
        crate::model::lemma_mh_ok_bounds(b@, m);
        reveal(crate::parse::hdr_matches);
        assert(4 * (w as int) * (h as int) <= crate::model::max_output());
        crate::model::lemma_4wh_usize(w, h);
        assert(n as int == 4 * (w as int) * (h as int));
        assert(n as nat == (4 * m.width * m.height) as nat);
        assert(n <= crate::alloc_guard::MAX_BYTES);
        assert(n >= 4);
    }
    let canvas = match try_zeroed(AllocSite::Canvas, n, Tracked(log)) {
        Ok(v) => v,
        Err(o) => return Err(TryError::OutOfMemory(o)),
    };
    proof {
        reveal(g_new_clause2);
        reveal(crate::alloc_guard::req_of_log);
        let m = choose|m: crate::model::HeaderM| crate::model::mh(b@) == Ok(m) && crate::parse::hdr_matches(b@, m, *hdr);
        reveal(crate::parse::hdr_matches);
        assert(n as nat == (4 * m.width * m.height) as nat);
        assert(req_new(b@)[0] == (AllocSite::Canvas, n as nat));
        let l = log.view();
        assert(l.len() == 1);
        assert(l[0] == (AllocSite::Canvas, n as nat, true));
        assert(crate::alloc_guard::req_of_log(l)[0] == (AllocSite::Canvas, n as nat));
        assert(crate::alloc_guard::req_of_log(l).len() == 1);
        assert(crate::alloc_guard::req_of_log(l)[0] == req_new(b@)[0]);
        assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b@))) by {
            assert(crate::alloc_guard::req_of_log(l) == seq![req_new(b@)[0]]);
            assert(seq![req_new(b@)[0]].is_prefix_of(req_new(b@))) by {
                assert(seq![req_new(b@)[0]] == req_new(b@).take(1));
            }
        }
    }
    let table = match try_zeroed(AllocSite::LzwTable, crate::lzw::LZW_TABLE_BYTES, Tracked(log)) {
        Ok(v) => v,
        Err(o) => return Err(TryError::OutOfMemory(o)),
    };
    proof {
        reveal(crate::alloc_guard::req_of_log);
        let l = log.view();
        assert(l.len() == 2);
        assert(l[0] == (AllocSite::Canvas, n as nat, true));
        assert(l[1] == (AllocSite::LzwTable, crate::lzw::LZW_TABLE_BYTES as nat, true));
        assert(crate::alloc_guard::req_of_log(l) =~= req_new(b@));
    }
    Ok((canvas, table))
}

} // verus!

/// Stream information (G1), fixed by the constructor's prefix walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Info {
    pub width: u16,
    pub height: u16,
    pub version: Version,
    /// Raw; not used for compositing (N2).
    pub background_index: u8,
    /// Raw pixel aspect ratio byte; never applied.
    pub aspect: u8,
    /// The raw NETSCAPE2.0 loop value found before the first image descriptor
    /// (G6.3, rule A); `plays` gives its meaning (G20).
    pub loop_count: Option<u16>,
}

/// The first frame composited on the logical screen (G14). `rgba` is
/// `4·width·height` bytes, the `Canvas` reservation itself.
#[derive(Debug, PartialEq, Eq)]
pub struct Still {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
}

/// One frame (G1 frame record) as a view of the decoder's single canvas.
#[derive(Debug)]
pub struct FrameView<'f> {
    /// The whole canvas after this frame was drawn, `4·W·H` bytes.
    pub rgba: &'f [u8],
    /// `(left, top, width, height)` from the image descriptor.
    pub rect: (u16, u16, u16, u16),
    /// Raw Graphic Control delay in hundredths of a second (Q16).
    pub delay_cs: u16,
    /// Raw disposal (0–3; 4–7 only under RecommendedBeta, drawn as 1).
    pub disposal: u8,
    pub user_input: bool,
    pub transparent: Option<u8>,
    pub interlaced: bool,
    /// A recovery event (E1 clip, E4 string overrun) occurred in this frame.
    /// Always `false` under Strict.
    pub recovered: bool,
}

/// G20: the documented meaning of `Info::loop_count`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plays {
    Once,
    Forever,
    Repeats(u16),
}

/// G20: `None ↦ Once`, `Some(0) ↦ Forever`, `Some(n) ↦ Repeats(n)`.
pub fn plays(loop_count: Option<u16>) -> Plays {
    match loop_count {
        None => Plays::Once,
        Some(0) => Plays::Forever,
        Some(n) => Plays::Repeats(n),
    }
}

impl Plays {
    /// Total plays: `Once` 1, `Repeats(n)` `n + 1` (at most 65,536, in `u32`),
    /// `Forever` none.
    pub fn total_plays(self) -> Option<u32> {
        match self {
            Plays::Once => Some(1),
            Plays::Forever => None,
            Plays::Repeats(n) => Some(n as u32 + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    /// The cursor is at a block boundary; more frames may follow.
    Running,
    /// The Trailer was reached.
    Done,
    /// An error was returned; every later call returns `None`.
    Fused,
}

/// The previous frame's rectangle and the disposal compositing applies to it
/// (4–7 act as 1).
#[derive(Clone, Copy)]
struct Prev {
    rect: Rect,
    disposal: u8,
}

/// Fields of one decoded frame, without the canvas.
#[derive(Clone, Copy)]
struct FrameMeta {
    rect: (u16, u16, u16, u16),
    delay_cs: u16,
    disposal: u8,
    user_input: bool,
    transparent: Option<u8>,
    interlaced: bool,
    recovered: bool,
}

enum Stop {
    AtImage,
    Trailer,
}

/// The frame decoder (G15). Borrows the input; owns one canvas (`Canvas`),
/// one LZW table (`LzwTable`) and, from the first disposal-3 frame on, the
/// restore buffer (`Saved`). Not an `Iterator`: each view borrows the canvas.
pub struct Frames<'a> {
    b: &'a [u8],
    pol: Policy,
    hdr: Header,
    info: Info,
    p: usize,
    st: WalkState,
    canvas: Vec<u8>,
    /// G18's `LzwTable`: the one 20,480-byte `try_zeroed` region, viewed by
    /// `lzw::decode` as prefix, suffix and length areas.
    table: Vec<u8>,
    saved: Option<Vec<u8>>,
    prev: Option<Prev>,
    state: State,
    /// `try_decode_still*`: the first frame's disposal is never applied, so a
    /// disposal-3 first frame reserves no `Saved` (G18 `Req(still)`).
    still: bool,
}

fn fmt<T>(r: Result<T, crate::gif::Error>) -> Result<T, TryError> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(TryError::Format(e)),
    }
}

impl<'a> Frames<'a> {
    /// `Frames::try_new_with_policy(b, Profile::Strict)`.
    pub fn try_new(b: &'a [u8]) -> Result<Frames<'a>, TryError> {
        Frames::open(b, Profile::Strict.policy(), false)
    }

    /// G2–G4, then `Canvas` and `LzwTable`, then the prefix walk to the first
    /// image descriptor. An error in that prefix is the constructor's error.
    pub fn try_new_with_policy(b: &'a [u8], p: Profile) -> Result<Frames<'a>, TryError> {
        Frames::open(b, p.policy(), false)
    }

    /// Test-only: a decoder for an arbitrary record (feature `test-records`).
    #[cfg(feature = "test-records")]
    #[doc(hidden)]
    pub fn try_new_with_record(b: &'a [u8], pol: Policy) -> Result<Frames<'a>, TryError> {
        Frames::open(b, pol, false)
    }

    fn open(b: &'a [u8], pol: Policy, still: bool) -> Result<Frames<'a>, TryError> {
        let hdr = fmt(parse::parse_header(b))?;
        // 4·W·H <= MAX_OUTPUT = alloc_guard::MAX_BYTES, and >= 4.
        let n = 4 * (hdr.width as usize) * (hdr.height as usize);
        let canvas = match try_zeroed(
            AllocSite::Canvas,
            n,
            Tracked::<&mut AllocLog>::assume_new(),
        ) {
            Ok(v) => v,
            Err(o) => return Err(TryError::OutOfMemory(o)),
        };
        let table = match try_zeroed(
            AllocSite::LzwTable,
            LZW_TABLE_BYTES,
            Tracked::<&mut AllocLog>::assume_new(),
        ) {
            Ok(v) => v,
            Err(o) => return Err(TryError::OutOfMemory(o)),
        };
        let info = Info {
            width: hdr.width,
            height: hdr.height,
            version: hdr.version,
            background_index: hdr.background_index,
            aspect: hdr.aspect,
            loop_count: None,
        };
        let mut d = Frames {
            b,
            pol,
            hdr,
            info,
            p: hdr.cursor,
            st: WalkState::new(),
            canvas,
            table,
            saved: None,
            prev: None,
            state: State::Running,
            still,
        };
        match d.walk() {
            Ok(Stop::AtImage) => {}
            Ok(Stop::Trailer) => d.state = State::Done,
            Err(e) => return Err(TryError::Format(e)),
        }
        d.info.loop_count = d.st.loop_count;
        Ok(d)
    }

    /// Width, height, version, background index, aspect and `loop_count`.
    pub fn info(&self) -> &Info {
        &self.info
    }

    /// `Some(Ok(view))` for each frame in stream order, then `Some(Err(e))`
    /// once if the stream ends in an error, then `None` on every later call.
    pub fn next_frame(&mut self) -> Option<Result<FrameView<'_>, TryError>> {
        if self.state != State::Running {
            return None;
        }
        match self.step() {
            Ok(Some(m)) => Some(Ok(FrameView {
                rgba: &self.canvas,
                rect: m.rect,
                delay_cs: m.delay_cs,
                disposal: m.disposal,
                user_input: m.user_input,
                transparent: m.transparent,
                interlaced: m.interlaced,
                recovered: m.recovered,
            })),
            Ok(None) => {
                self.state = State::Done;
                None
            }
            Err(e) => {
                self.state = State::Fused;
                Some(Err(e))
            }
        }
    }

    /// G5 block loop from the cursor until an image separator (cursor left on
    /// it) or the Trailer.
    fn walk(&mut self) -> Result<Stop, Error> {
        let b = self.b;
        loop {
            let c;
            if self.p >= b.len() {
                if self.pol.missing_trailer == MissingTrailer::Reject {
                    return Err(Error::MissingTrailer);
                }
                c = 0x3B; // missing_trailer = AtEof: EOF acts as the Trailer.
            } else {
                c = parse::get(b, self.p);
            }
            if c == 0x3B {
                if self.st.gce.is_some() && self.pol.gce_orphan == GceOrphan::Reject {
                    return Err(Error::GraphicControlWithoutGraphic);
                }
                if self.p < b.len() && b.len() - self.p > 1 && self.pol.trailing == Trailing::Reject {
                    return Err(Error::TrailingBytes);
                }
                return Ok(Stop::Trailer);
            }
            if c == 0x21 {
                self.p = parse::parse_extension(b, self.p, &mut self.st, &self.pol)?;
                continue;
            }
            if c == 0x2C {
                return Ok(Stop::AtImage);
            }
            match self.pol.unknown_block {
                UnknownBlock::Reject => return Err(Error::UnknownBlock),
            }
        }
    }

    /// Walk to the next image and decode it (G8–G12), or report the Trailer.
    fn step(&mut self) -> Result<Option<FrameMeta>, TryError> {
        match fmt(self.walk())? {
            Stop::Trailer => Ok(None),
            Stop::AtImage => self.image().map(Some),
        }
    }

    fn image(&mut self) -> Result<FrameMeta, TryError> {
        let b = self.b;
        let (fh, c) = fmt(parse::parse_image_descriptor(b, self.p, &self.hdr, &self.st, &self.pol))?;
        let area = fh.rect.width as u64 * fh.rect.height as u64;
        let data = fmt(parse::scan_image_data(b, c, &self.pol, area))?;
        let gce = self.st.gce;
        self.st.gce = None;
        self.st.frames_seen += 1;
        self.st.total_pixels += area;
        self.p = data.end;

        let sw = self.hdr.width as u32;
        let sh = self.hdr.height as u32;
        // G12.2: apply the previous frame's disposal first.
        if let Some(prev) = self.prev {
            if prev.disposal == 2 {
                clear_rect(&mut self.canvas, sw, sh, prev.rect);
            } else if prev.disposal == 3 {
                if let Some(saved) = &self.saved {
                    copy_rect(&mut self.canvas, saved, sw, sh, prev.rect);
                }
            }
        }
        let disposal = match gce {
            Some(g) => g.disposal,
            None => 0,
        };
        // G12.2: a disposal-3 frame saves its rectangle; `Saved` is reserved
        // once, at the first such frame (G18).
        if disposal == 3 && !self.still {
            if self.saved.is_none() {
                let n = self.canvas.len();
                match try_zeroed(
                    AllocSite::Saved,
                    n,
                    Tracked::<&mut AllocLog>::assume_new(),
                ) {
                    Ok(v) => self.saved = Some(v),
                    Err(o) => return Err(TryError::OutOfMemory(o)),
                }
            }
            if let Some(saved) = &mut self.saved {
                copy_rect(saved, &self.canvas, sw, sh, fh.rect);
            }
        }

        let toff = fh.table.offset;
        let tlen = 3 * fh.table.entries as usize;
        let colors: &[u8] = match b.get(toff..toff.saturating_add(tlen)) {
            Some(s) => s,
            None => &[],
        };
        let transparent = match gce {
            Some(g) => g.transparent,
            None => None,
        };
        let opts = LzwOptions {
            tail: if self.pol.lzw_tail == LzwTail::StopAtRaster { Tail::StopAtRaster } else { Tail::Strict },
            checked_literals: self.pol.lzw_min == LzwMin::Range1to11Checked,
        };
        let table: &mut [u8] = &mut self.table;
        let mut sink = Drawer::new(&mut self.canvas, colors, transparent, fh.rect, fh.interlaced, sw, sh);
        let mut reader = BitReader::new(b, data.first, data.data_len);
        // area <= MAX_TOTAL_PIXELS = 2^30 after G8, so it fits in u32.
        let area32 = if area > u32::MAX as u64 { u32::MAX } else { area as u32 };
        let overrun = fmt(lzw::decode(
            &mut reader,
            data.data_len,
            data.min_code_size,
            area32,
            opts,
            table,
            &mut sink,
        ))?;

        self.prev = Some(Prev { rect: fh.rect, disposal: if disposal > 3 { 1 } else { disposal } });
        let (delay_cs, user_input) = match gce {
            Some(g) => (g.delay_cs, g.user_input),
            None => (0, false),
        };
        Ok(FrameMeta {
            // Each field came from an LE16, so it is at most 65,535.
            rect: (fh.rect.left as u16, fh.rect.top as u16, fh.rect.width as u16, fh.rect.height as u16),
            delay_cs,
            disposal,
            user_input,
            transparent,
            interlaced: fh.interlaced,
            recovered: !fh.inside || overrun,
        })
    }
}

/// `try_decode_still_with_policy(b, Profile::Strict)` without the flag (G14).
pub fn try_decode_still(b: &[u8]) -> Result<Still, TryError> {
    match try_decode_still_with_policy(b, Profile::Strict) {
        Ok((s, _)) => Ok(s),
        Err(e) => Err(e),
    }
}

/// G14 under a profile: the first frame on the logical screen and its
/// `recovered` flag. Bytes after the first image's block terminator are not
/// examined. `Req = [(Canvas, 4·W·H), (LzwTable, 20_480)]` when G2–G4 pass.
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> Result<(Still, bool), TryError> {
    still_with(b, p.policy())
}

/// Test-only: the still entry point for an arbitrary record.
#[cfg(feature = "test-records")]
#[doc(hidden)]
pub fn try_decode_still_with_record(b: &[u8], pol: Policy) -> Result<(Still, bool), TryError> {
    still_with(b, pol)
}

fn still_with(b: &[u8], pol: Policy) -> Result<(Still, bool), TryError> {
    let mut d = Frames::open(b, pol, true)?;
    if d.state != State::Running {
        return Err(TryError::Format(Error::NoImage));
    }
    match d.step()? {
        Some(m) => Ok((Still { width: d.hdr.width, height: d.hdr.height, rgba: d.canvas }, m.recovered)),
        None => Err(TryError::Format(Error::NoImage)),
    }
}

