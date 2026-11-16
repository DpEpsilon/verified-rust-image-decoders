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
use crate::lzw::{LzwOptions, Tail, LZW_TABLE_BYTES};
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
        crate::alloc_guard::sites::log_tied(r,final(log).view()),
        g_new_clause3(b@),
        r is Err ==> r.unwrap_err() is OutOfMemory,
        r is Ok ==> r->Ok_0.0@ == Seq::new((4*(hdr.width as int)*(hdr.height as int)) as nat,|_i:int|0u8),
        r is Ok ==> r->Ok_0.1@ == Seq::new(crate::lzw::LZW_TABLE_BYTES as nat,|_i:int|0u8),
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

verus! {
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

} // verus!

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

verus! {
/// G20: the documented meaning of `Info::loop_count`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plays {
    Once,
    Forever,
    Repeats(u16),
}

/// G20: `None ↦ Once`, `Some(0) ↦ Forever`, `Some(n) ↦ Repeats(n)`.
pub fn plays(loop_count: Option<u16>) -> (p: Plays)
    ensures p==plays_model(loop_count),
{
    match loop_count {
        None => Plays::Once,
        Some(0) => Plays::Forever,
        Some(n) => Plays::Repeats(n),
    }
}

impl Plays {
    /// Total plays: `Once` 1, `Repeats(n)` `n + 1` (at most 65,536, in `u32`),
    /// `Forever` none.
    pub fn total_plays(self) -> (p: Option<u32>)
        ensures p==total_plays_model(self),
    {
        match self {
            Plays::Once => Some(1),
            Plays::Forever => None,
            Plays::Repeats(n) => Some(n as u32 + 1),
        }
    }
}


pub open spec fn plays_model(x:Option<u16>)->Plays {
    match x {None=>Plays::Once,Some(0)=>Plays::Forever,Some(n)=>Plays::Repeats(n)}
}
pub open spec fn total_plays_model(p:Plays)->Option<u32> {
    match p {Plays::Once=>Some(1),Plays::Forever=>None,Plays::Repeats(n)=>Some((n as int+1) as u32)}
}
/// T-PLAYS: the total-play observation identifies every optional wire value.
pub proof fn lemma_total_plays_injective(x:Option<u16>,y:Option<u16>)
    ensures total_plays_model(plays_model(x))==total_plays_model(plays_model(y)) ==> x==y,
{}
} // verus!

verus! {
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
    log: Tracked<AllocLog>,
}
} // verus!

fn fmt<T>(r: Result<T, crate::gif::Error>) -> Result<T, TryError> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(TryError::Format(e)),
    }
}

impl<'a> Frames<'a> {
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

    /// Walk to the next image and decode it (G8–G12), or report the Trailer.
    fn step(&mut self) -> Result<Option<FrameMeta>, TryError> {
        match fmt(parse::parse_mb(self.b, &mut self.p, &self.hdr, &self.pol, &mut self.st))? {
            None => Ok(None),
            Some(image) => self.image(image).map(Some),
        }
    }

    fn image(&mut self, image: parse::ImageStep) -> Result<FrameMeta, TryError> {
        let b = self.b;
        let fh = image.fh;
        let data = image.data;
        let gce = image.gce;
        let area = fh.rect.width as u64 * fh.rect.height as u64;
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
            self.ensure_saved()?;
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
        let sink_fr = crate::compose_new::Frame {
            rect: fh.rect,
            interlaced: fh.interlaced,
            // The parser validated the table's entry count before LZW.
            c: fh.table.entries,
            transparent,
            // Drawing ignores disposal; normalize Keep for the sink invariant.
            disposal: if disposal > 3 { 1 } else { disposal as u32 },
        };
        let mut sink = Drawer::new(&mut self.canvas, colors, sink_fr, sw, sh);
        // area <= MAX_TOTAL_PIXELS = 2^30 after G8, so it fits in u32.
        let area32 = if area > u32::MAX as u64 { u32::MAX } else { area as u32 };
        let overrun = fmt(crate::compose_new::decode_drawer(
            b,
            data,
            area32,
            opts,
            self.pol.lzw_min,
            table,
            &mut sink,
        ).public_result())?;

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


verus! {
#[derive(Debug)]
struct Prepared {
    hdr:Header,
    p:usize,
    st:WalkState,
    stop:crate::parse::PrefixStop,
    canvas:Vec<u8>,
    table:Vec<u8>,
}

spec fn prepared_ok(b:Seq<u8>,pol:Policy,d:Prepared)->bool {
    &&& crate::parse::hdr_inv(b,d.hdr) && crate::parse::hdr_ok(b,d.hdr)
    &&& b.len()<=crate::limits::MAX_INPUT && d.p<=b.len()
    &&& d.st.frames_seen==0 && d.st.total_pixels==0
    &&& d.canvas@==Seq::new((4*(d.hdr.width as int)*(d.hdr.height as int)) as nat,|_i:int|0u8)
    &&& d.table@==Seq::new(crate::lzw::LZW_TABLE_BYTES as nat,|_i:int|0u8)
    &&& match d.stop {
        crate::parse::PrefixStop::AtImage=>d.p<b.len() && b[d.p as int]==0x2c,
        crate::parse::PrefixStop::Trailer=>d.p==b.len() || d.p<b.len() && b[d.p as int]==0x3b,
    }
    &&& match crate::model::mp(b,pol) {
        Err(_)=>false,
        Ok((mi,ms))=> {
            &&& mi.width==d.hdr.width as int && mi.height==d.hdr.height as int
            &&& mi.version==crate::model::version_code(d.hdr.version)
            &&& mi.background_index==d.hdr.background_index as int && mi.aspect==d.hdr.aspect as int
            &&& mi.loop_count==d.st.loop_count && crate::parse::st_matches(d.p,d.st,ms)
        },
    }
}
spec fn prepare_result(b:Seq<u8>,pol:Policy,r:Result<Prepared,TryError>)->bool {
    match r {
        Ok(d)=>prepared_ok(b,pol,d),
        Err(TryError::Format(e))=>crate::model::mp(b,pol)==Err(e),
        Err(TryError::OutOfMemory(_))=>crate::model::mh(b) is Ok,
    }
}

/// The exact header/reservation/prefix constructor, without duplicating or
/// initializing any input-dependent storage beyond the sealed reservations.
fn prepare(b:&[u8],pol:Policy,Tracked(log):Tracked<&mut AllocLog>)->(r:Result<Prepared,TryError>)
    requires log.view()==Seq::<(AllocSite,nat,bool)>::empty(),
    ensures prepare_result(b@,pol,r),
        crate::alloc_guard::sites::log_tied(r,final(log).view()),
        g_new_clause2(b@,r is Err && r->Err_0 is OutOfMemory,final(log).view()),g_new_clause3(b@),
        r is Ok ==> crate::alloc_guard::req_of_log(final(log).view())==req_new(b@),
        crate::alloc_guard::req_of_log(final(log).view()).is_prefix_of(req_new(b@)),
        !(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(final(log).view())==req_new(b@),
{
    hide(crate::model::mp_fuel);
    let hdr=match parse::parse_header(b) {
        Ok(h)=>h,
        Err(e)=>return Err(TryError::Format(e)),
    };
    proof {
        let mh=crate::model::mh(b@).unwrap();
        crate::model::lemma_mh_ok_bounds(b@,mh);
        crate::parse::lemma_header_model_matches(b@,hdr,mh);
        assert(crate::parse::hdr_ok(b@,hdr));
    }
    let (canvas,table)=match reserve(b,&hdr,Tracked(log)) {
        Ok(v)=>v,Err(e)=>return Err(e),
    };
    let mut p=hdr.cursor;
    let mut st=WalkState::new();
    let stop=match crate::parse::parse_mp(b,&mut p,&pol,&mut st) {
        Ok(stop)=>stop,Err(e)=>return Err(TryError::Format(e)),
    };
    Ok(Prepared {hdr,p,st,stop,canvas,table})
}
} // verus!

verus! {
pub open spec fn info_model(i:Info)->crate::model::InfoM {
    crate::model::InfoM {width:i.width as int,height:i.height as int,version:crate::model::version_code(i.version),
        background_index:i.background_index as int,aspect:i.aspect as int,loop_count:i.loop_count}
}

/// Storage and parser bounds required between frame calls. The semantic
/// canvas/disposal relation to the full frame model is a separate invariant.
pub closed spec fn storage_wf(d:Frames)->bool {
    &&& d.b.len()<=crate::limits::MAX_INPUT && crate::parse::hdr_inv(d.b@,d.hdr) && crate::parse::hdr_ok(d.b@,d.hdr)
    &&& d.p<=d.b.len() && d.st.frames_seen<=crate::limits::MAX_FRAMES && d.st.total_pixels<=crate::limits::MAX_TOTAL_PIXELS
    &&& d.canvas@.len()==4*(d.hdr.width as int)*(d.hdr.height as int)
    &&& d.table@.len()==crate::lzw::LZW_TABLE_BYTES
    &&& d.st.loop_count==d.info.loop_count
    &&& match crate::model::mp(d.b@,d.pol) {Err(_)=>false,Ok((mi,_))=>info_model(d.info)==mi}
    &&& match d.saved {
        None=>d.log@.view()==seq![(AllocSite::Canvas,d.canvas@.len(),true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true)]
            || d.state==State::Fused && d.log@.view()==seq![(AllocSite::Canvas,d.canvas@.len(),true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true),(AllocSite::Saved,d.canvas@.len(),false)],
        Some(v)=>v@.len()==d.canvas@.len() && d.log@.view()==seq![(AllocSite::Canvas,d.canvas@.len(),true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true),(AllocSite::Saved,d.canvas@.len(),true)],
    }
}

pub closed spec fn constructor_ok(d:Frames,b:Seq<u8>,pol:Policy,still:bool)->bool {
    &&& storage_wf(d) && d.b@==b && d.pol==pol && d.still==still
    &&& d.st.frames_seen==0 && d.st.total_pixels==0 && d.saved is None && d.prev is None
    &&& d.canvas@==Seq::new(d.canvas@.len(),|_i:int|0u8)
    &&& d.table@==Seq::new(crate::lzw::LZW_TABLE_BYTES as nat,|_i:int|0u8)
    &&& crate::alloc_guard::req_of_log(d.log@.view())==req_new(b) && crate::alloc_guard::all_ok(d.log@.view())
    &&& match crate::model::mp(b,pol) {Err(_)=>false,Ok((_,ms))=>crate::parse::st_matches(d.p,d.st,ms)}
    &&& match d.state {State::Running=>d.p<b.len() && b[d.p as int]==0x2c,State::Done=>d.p==b.len() || d.p<b.len() && b[d.p as int]==0x3b,State::Fused=>false}
}

pub closed spec fn constructor_result(b:Seq<u8>,pol:Policy,still:bool,r:Result<Frames,TryError>)->bool {
    match r {
        Ok(d)=>constructor_ok(d,b,pol,still),
        Err(TryError::Format(e))=>crate::model::mp(b,pol)==Err(e),
        Err(TryError::OutOfMemory(_))=>crate::model::mh(b) is Ok,
    }
}

/// Success owns the log in Frames; only an error returns it separately.
pub closed spec fn construction_log(r:&(Result<Frames,TryError>,Tracked<Option<AllocLog>>))->Seq<(AllocSite,nat,bool)> {
    match r.0 {Ok(d)=>d.log@.view(),Err(_)=>match r.1@ {Some(l)=>l.view(),None=>Seq::empty()}}
}

impl<'a> Frames<'a> {
    pub closed spec fn log_view(&self)->Seq<(AllocSite,nat,bool)> {self.log@.view()}
    pub closed spec fn info_view(&self)->Info {self.info}

    /// Width, height, version, background index, aspect and `loop_count`.
    pub fn info(&self)->(i:&Info)
        ensures *i==self.info_view(),
    {&self.info}

    fn open_logged(b:&'a [u8],pol:Policy,still:bool)->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,pol,still,r.0),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,
            r.0 is Err ==> r.1@ is Some,
    {
        hide(crate::model::mp);
        reveal(constructor_result);reveal(constructor_ok);reveal(storage_wf);reveal(construction_log);
        let tracked mut log=crate::alloc_guard::new_log();
        let prepared=prepare(b,pol,Tracked(&mut log));
        match prepared {
            Err(e)=>(Err(e),Tracked(Some(log))),
            Ok(d)=> {
                let info=Info {width:d.hdr.width,height:d.hdr.height,version:d.hdr.version,
                    background_index:d.hdr.background_index,aspect:d.hdr.aspect,loop_count:d.st.loop_count};
                let state=match d.stop {crate::parse::PrefixStop::AtImage=>State::Running,crate::parse::PrefixStop::Trailer=>State::Done};
                let dec=Frames {b,pol,hdr:d.hdr,info,p:d.p,st:d.st,canvas:d.canvas,table:d.table,
                    saved:None,prev:None,state,still,log:Tracked(log)};
                proof {
                    assert(match crate::model::mp(dec.b@,dec.pol) {Err(_)=>false,Ok((mi,_))=>info_model(dec.info)==mi});
                    assert(dec.log@.view()==seq![(AllocSite::Canvas,dec.canvas@.len(),true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true)]);
                    assert(storage_wf(dec));assert(constructor_ok(dec,b@,pol,still));
                }
                (Ok(dec),Tracked(None))
            },
        }
    }

    fn open(b:&'a [u8],pol:Policy,still:bool)->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,pol,still,r),
    {Self::open_logged(b,pol,still).0}

    pub fn try_new(b:&'a [u8])->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,crate::policy::profile_record(Profile::Strict),false,r),
    {Self::open(b,Profile::Strict.policy(),false)}

    pub fn try_new_with_policy(b:&'a [u8],p:Profile)->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,crate::policy::profile_record(p),false,r),
    {Self::open(b,p.policy(),false)}

    pub fn try_new_logged(b:&'a [u8])->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,crate::policy::profile_record(Profile::Strict),false,r.0),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,r.0 is Err ==> r.1@ is Some,
    {Self::open_logged(b,Profile::Strict.policy(),false)}

    pub fn try_new_with_policy_logged(b:&'a [u8],p:Profile)->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,crate::policy::profile_record(p),false,r.0),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,r.0 is Err ==> r.1@ is Some,
    {Self::open_logged(b,p.policy(),false)}

    #[cfg(feature="test-records")]
    #[doc(hidden)]
    pub fn try_new_with_record(b:&'a [u8],pol:Policy)->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,pol,false,r),
    {Self::open(b,pol,false)}

    /// The sole Saved reservation: a real borrow of the decoder's sealed log.
    #[verifier::spinoff_prover]
    fn ensure_saved(&mut self)->(r:Result<(),TryError>)
        requires storage_wf(*old(self)),old(self).state==State::Running,!old(self).still,
        ensures storage_wf(*final(self)),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).canvas@==old(self).canvas@,final(self).table@==old(self).table@,
            final(self).still==old(self).still,final(self).prev==old(self).prev,
            crate::alloc_guard::sites::log_tied(r,final(self).log@.view()),
            final(self).log@.view()==if old(self).saved is None {
                old(self).log@.view().push((AllocSite::Saved,old(self).canvas@.len(),r is Ok))
            } else {old(self).log@.view()},
            r is Err ==> match r->Err_0 {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as int==old(self).canvas@.len(),_=>false},
            match old(self).saved {
                Some(v)=>match final(self).saved {Some(w)=>w@==v@,None=>false},
                None=>r is Ok ==> match final(self).saved {Some(w)=>w@==Seq::new(old(self).canvas@.len(),|_i:int|0u8),None=>false},
            },
            r is Ok ==> final(self).state==old(self).state && final(self).saved is Some,
            r is Err ==> final(self).state==State::Fused && r->Err_0 is OutOfMemory,
            final(self).log@.view().len()<=3,
    {
        hide(crate::model::mp);
        hide(crate::parse::hdr_ok);
        reveal(storage_wf);
        if let Some(_) = self.saved {return Ok(());}
        proof {
            assert(4*(self.hdr.width as int)*(self.hdr.height as int)>=4) by(nonlinear_arith)
                requires self.hdr.width>=1,self.hdr.height>=1;
        }
        let n=self.canvas.len();
        let ghost l0=self.log@.view();
        proof {assert(l0==seq![(AllocSite::Canvas,n as nat,true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true)]);}
        match try_zeroed(AllocSite::Saved,n,Tracked(self.log.borrow_mut())) {
            Ok(v)=>{
                proof {assert(self.log@.view()==l0.push((AllocSite::Saved,n as nat,true)));}
                self.saved=Some(v);
                proof {
                    assert(self.log@.view() =~= seq![(AllocSite::Canvas,n as nat,true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true),(AllocSite::Saved,n as nat,true)]);
                    assert(match crate::model::mp(self.b@,self.pol) {Err(_)=>false,Ok((mi,_))=>info_model(self.info)==mi});
                    assert(storage_wf(*self));
                }
                Ok(())
            },
            Err(o)=>{
                proof {assert(self.log@.view()==l0.push((AllocSite::Saved,n as nat,false)));}
                self.state=State::Fused;
                proof {
                    assert(self.log@.view() =~= seq![(AllocSite::Canvas,n as nat,true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true),(AllocSite::Saved,n as nat,false)]);
                    assert(match crate::model::mp(self.b@,self.pol) {Err(_)=>false,Ok((mi,_))=>info_model(self.info)==mi});
                    assert(storage_wf(*self));
                }
                Err(TryError::OutOfMemory(o))
            },
        }
    }
}
spec fn walk_state_same(a:Frames,b:Frames)->bool {crate::parse::walk_eq(a.st,b.st)}
} // verus!
