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
use crate::compose::{clear_rect, copy_rect, Rect};
use crate::lzw::{LzwOptions, Tail, LZW_TABLE_BYTES};
use crate::parse::{self, Header, WalkState};
use crate::policy::{GceOrphan, LzwMin, LzwTail, MissingTrailer, Policy, Profile, Trailing, UnknownBlock};

use vstd::prelude::*;

verus! {

/// `Req_new(b) = Req_still(b)`: the two reservations of a still call or of the
/// iterator constructor, `[(Canvas, 4·N), (LzwTable, LZW_TABLE_BYTES)]`, and
/// only when G2–G4 pass (`ReqLzw = [(LzwTable, 20,480)]`, G18, D4).
pub const C_CALL: usize = 32;
pub const C_WORK: usize = 64;
pub const C_FIXED: usize = 41_984;

pub open spec fn header_pixels(b:Seq<u8>)->nat {
    match crate::model::mh(b) {Ok(h)=>(h.width*h.height) as nat,Err(_)=>0}
}

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
    steps: &mut Ghost<nat>,
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
    old(steps)@ <= final(steps)@,
        final(steps)@ as int <= old(steps)@ as int + 4*(hdr.width as int)*(hdr.height as int) + crate::lzw::LZW_TABLE_BYTES as int,
        r is Ok ==> final(steps)@ as int == old(steps)@ as int + 4*(hdr.width as int)*(hdr.height as int) + crate::lzw::LZW_TABLE_BYTES as int,
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
    // 4·W·H <= MAX_OUTPUT <= alloc_guard::MAX_BYTES, and >= 4 (hdr_inv).
    let w = hdr.width as u32;
    let h = hdr.height as u32;
    proof {
        assert(1 <= hdr.width as int <= crate::model::max_dimension());
        assert(1 <= hdr.height as int <= crate::model::max_dimension());
        crate::limits::caps_ok();
        assert(w == hdr.width);
        assert(h == hdr.height);
        assert(1 <= w <= u16::MAX as u32);
        assert(1 <= h <= u16::MAX as u32);
        // `hdr_ok` gives the model record, and `mh_bounds` its area cap.
        let m = choose|m: crate::model::HeaderM| crate::model::mh(b@) == Ok(m) && crate::parse::hdr_matches(b@, m, *hdr);
        crate::model::lemma_mh_ok_bounds(b@, m);
        reveal(crate::parse::hdr_matches);
        assert(4 * (w as int) * (h as int) <= crate::model::max_output());
        crate::model::lemma_4wh_usize(w, h);
        assert(crate::model::max_output() <= crate::alloc_guard::MAX_BYTES);
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
        Ok(v) => {proof {steps@=steps@+v@.len();} v},
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
        Ok(v) => {proof {steps@=steps@+v@.len();} v},
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

verus! {
/// The first frame composited on the logical screen (G14). `rgba` is
/// `4·width·height` bytes, the `Canvas` reservation itself.
#[derive(Debug, PartialEq, Eq)]
pub struct Still {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
}
} // verus!

verus! {
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
} // verus!

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
    work: Ghost<nat>,
    charged_pixels: Ghost<nat>,
}
} // verus!

verus! {
fn fmt<T>(r: Result<T, crate::gif::Error>) -> (out: Result<T, TryError>)
    ensures out==match r {Ok(v)=>Ok(v),Err(e)=>Err(TryError::Format(e))},
{
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(TryError::Format(e)),
    }
}
} // verus!

verus! {
impl<'a> Frames<'a> {
    /// `Some(Ok(view))` for each frame in stream order, then `Some(Err(e))`
    /// once if the stream ends in an error, then `None` on every later call.
    #[verifier::spinoff_prover]
    pub fn next_frame(&mut self) -> (out: Option<Result<FrameView<'_>, TryError>>)
        requires storage_wf(*old(self)),
        ensures storage_wf(*final(self)),next_result(*old(self),*final(self),out),fixed_stream(*old(self),*final(self)),
            forall|n:int| wf_p(*old(self),n) ==> wf_p(*final(self),n+1),
            forall|n:int| wf_p(*old(self),n) ==> iterator_observation(*old(self),n,out),
            forall|n:int| wf_p(*old(self),n) ==> next_allocation(*old(self),*final(self),n,out),
            next_work(*old(self),*final(self)),next_model_work(*old(self),*final(self),out),
            forall|n:int| wf_p(*old(self),n) && work_ready(*old(self)) ==> t12_measured_call(*old(self),*final(self),n),
            forall|k:nat| T12_next(*old(self),*final(self),k,out),
            forall|n:int| wf_p(*old(self),n) ==> T13_view(*final(self),n+1,out),
            P_UNIV_next(*old(self),*final(self),out),
    {
        hide(storage_wf);hide(step_result);hide(wf_p);hide(iterator_observation);hide(next_allocation);
        let ghost original=*self;
        match self.state {
            State::Running=>{},
            _=>{
                proof {
                    assert forall|n:int| wf_p(*old(self),n) implies wf_p(*self,n+1) by {lemma_terminal_wf(*self,n);}
                    assert forall|n:int| wf_p(original,n) implies iterator_observation(original,n,None) && next_allocation(original,*self,n,None) by {
                        lemma_next_observation(original,*self,None,n);
                        lemma_next_allocation(original,*self,n,None);
                    }

                }
                proof {
                    lemma_next_work(original,*self);
                    assert forall|n:int| wf_p(original,n) && work_ready(original) implies t12_measured_call(original,*self,n) by {
                        lemma_t12_measured_call(original,*self,n,None);
                    }
                }
                proof {
                    assert(next_model_work(original,*self,None));
                    assert forall|k:nat| T12_next(original,*self,k,None) by {lemma_T12_next(original,*self,k,None);}
                    assert forall|n:int| wf_p(original,n) implies T13_view(*self,n+1,None) by {lemma_T13_next(original,*self,n,None);}
                    lemma_p_univ_next(original,*self,None);
                }
                return None;
            },
        }
        let out=match self.step() {
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
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        };
        proof {
            assert forall|n:int| wf_p(original,n) implies iterator_observation(original,n,out) && next_allocation(original,*self,n,out) by {
                lemma_next_observation(original,*self,out,n);
                lemma_next_allocation(original,*self,n,out);
            }
        }
        proof {
            lemma_next_work(original,*self);
            assert forall|n:int| wf_p(original,n) && work_ready(original) implies t12_measured_call(original,*self,n) by {
                lemma_t12_measured_call(original,*self,n,out);
            }
        }
        proof {
            assert(next_model_work(original,*self,out));
            assert forall|k:nat| T12_next(original,*self,k,out) by {lemma_T12_next(original,*self,k,out);}
            assert forall|n:int| wf_p(original,n) implies T13_view(*self,n+1,out) by {lemma_T13_next(original,*self,n,out);}
            lemma_p_univ_next(original,*self,out);
        }
        out
    }

}
} // verus!

verus! {
impl<'a> Frames<'a> {
    /// Walk through one complete image, or make the terminal transition once.
    #[verifier::spinoff_prover]
    fn step(&mut self)->(r:Result<Option<FrameMeta>,TryError>)
        requires storage_wf(*old(self)),old(self).state==State::Running,
        ensures storage_wf(*final(self)),step_result(*old(self),*final(self),r),
            forall|n:int| wf_p(*old(self),n) ==> wf_p(*final(self),n+1),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).still==old(self).still,
            match r {Err(_)=>final(self).state==State::Fused,Ok(None)=>final(self).state==State::Done,Ok(Some(_))=>final(self).state==State::Running},
        step_cost(*old(self),*final(self)),step_model_work(*old(self),*final(self),r),
    {
        hide(storage_wf);hide(step_result);hide(image_result);hide(crate::parse::mb_exec_post);hide(wf_p);
        hide(crate::model::mb);hide(crate::model::mp);hide(crate::parse::hdr_ok);
        hide(crate::frame_model::image_mode);hide(crate::lzw::read_d);hide(crate::parse::image_data_post);
        proof {lemma_storage_bounds(*self);}
        let ghost original=*self;
        let parsed=self.scan_next();
        let out=match parsed {
            Err(e)=>{
                proof {lemma_step_terminal(original,*self,parsed);}
                Err(TryError::Format(e))
            },
            Ok(None)=>{
                proof {lemma_step_terminal(original,*self,parsed);}
                Ok(None)
            },
            Ok(Some(im))=>{
                let ghost scanned=*self;
                let r=self.image(im);
                proof {lemma_step_image(original,scanned,*self,im,r);}
                match r {Ok(m)=>Ok(Some(m)),Err(e)=>Err(e)}
            },
        };
        proof {
            assert forall|n:int| wf_p(original,n) implies wf_p(*self,n+1) by {
                lemma_step_wf(original,*self,out,n);
            }
        }
        out
    }

    #[verifier::spinoff_prover]
    fn scan_next(&mut self)->(r:Result<Option<parse::ImageStep>,Error>)
        requires storage_wf(*old(self)),old(self).state==State::Running,
        ensures storage_wf(*final(self)),
            crate::parse::mb_exec_post(old(self).b@,old(self).p,old(self).hdr,old(self).pol,old(self).st,final(self).p,final(self).st,r),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).still==old(self).still,
            final(self).canvas@==old(self).canvas@,final(self).table@==old(self).table@,saved_view(*final(self))==saved_view(*old(self)),
            final(self).prev==old(self).prev,final(self).log@.view()==old(self).log@.view(),
            match r {
                Err(_)=>final(self).state==State::Fused,
                Ok(None)=>final(self).state==State::Done,
                Ok(Some(im))=>final(self).state==State::Running && image_ready(*final(self),im),
            },
        final(self).charged_pixels@==old(self).charged_pixels@,
            old(self).p<=final(self).p<=old(self).b.len(),old(self).work@<=final(self).work@,
            final(self).work@ as int <= old(self).work@ as int + 4 * (effective_cursor(*final(self))-old(self).p as int)+1,
            r is Ok && r->Ok_0 is Some ==> old(self).p<final(self).p && r->Ok_0->Some_0.data.data_len as int<=final(self).p as int-old(self).p as int,
            old(self).p<old(self).b.len() && old(self).b@[old(self).p as int]==0x2c ==> !(r is Ok && r->Ok_0 is None),
            effective_cursor(*final(self))==crate::work_model::mb_cur(old(self).b@,crate::parse::header_model(old(self).hdr),old(self).pol,crate::parse::walk_m_at(old(self).st,old(self).p)),
            (match r {Ok(Some(im))=>rect_area(im.fh.rect),_=>0nat})
                ==crate::work_model::mb_area(old(self).b@,crate::parse::header_model(old(self).hdr),old(self).pol,crate::parse::walk_m_at(old(self).st,old(self).p)),
    {
        hide(storage_wf);hide(crate::parse::mb_exec_post);
        hide(crate::model::mb);hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::parse::image_data_post);
        proof {lemma_storage_bounds(*self);}
        let ghost original=*self;
        let parsed=parse::parse_mb(self.b,&mut self.p,&self.hdr,&self.pol,&mut self.st,&mut self.work);
        match parsed {
            Err(_)=>{self.state=State::Fused;proof {lemma_storage_walk(original,*self);}},
            Ok(None)=>{self.state=State::Done;proof {lemma_storage_walk(original,*self);}},
            Ok(Some(im))=>{proof {lemma_storage_walk(original,*self);lemma_mb_image_ready(original,*self,im);}},
        }
        parsed
    }
}
} // verus!

verus! {
#[verifier::spinoff_prover]
fn decode_image_pixels(b:&[u8],hdr:Header,pol:Policy,im:parse::ImageStep,t:&mut [u8],cv:&mut [u8])->(rep:crate::lzw::LzwReport)
    requires b.len()<=crate::limits::MAX_INPUT,crate::parse::hdr_inv(b@,hdr),image_inputs(b@,hdr,pol,im),
        crate::compose_new::canvas_ok(old(cv),hdr.width as u32,hdr.height as u32),crate::lzw::table_len_ok(old(t)@),
    ensures final(cv)@.len()==old(cv)@.len(),crate::lzw::table_len_ok(final(t)@),
        pixels_observation(b@,hdr,pol,im,old(cv)@,final(cv)@,rep),
        rep.work@ as int <= 5 * im.data.data_len as int + 2 * ((im.fh.rect.width as int) * (im.fh.rect.height as int)),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);
    hide(crate::parse::image_data_post);hide(crate::model::sub_scan);hide(crate::lzw::read_d);
    let fh=im.fh;
    let data=im.data;
    let disposal=match im.gce {Some(g)=>g.disposal,None=>0};
    let transparent=match im.gce {Some(g)=>g.transparent,None=>None};
    let area=(fh.rect.width as u64)*(fh.rect.height as u64);
    proof {
        crate::limits::caps_ok();
        assert((fh.rect.width as int)*(fh.rect.height as int)<=u32::MAX) by(nonlinear_arith)
            requires 0<=fh.rect.width<=u16::MAX,
                0<=fh.rect.height<=u16::MAX;
    }
    let area32=area as u32;
    let opts=LzwOptions {
        tail:match pol.lzw_tail {LzwTail::StopAtRaster=>Tail::StopAtRaster,LzwTail::Strict=>Tail::Strict},
        checked_literals:match pol.lzw_min {LzwMin::Range1to11Checked=>true,LzwMin::Spec=>false},
    };
    let fr=crate::compose_new::Frame {rect:fh.rect,interlaced:fh.interlaced,c:fh.table.entries,transparent,
        disposal:if disposal>3 {1} else {disposal as u32}};
    let colors=&b[fh.table.offset..fh.table.offset+3*fh.table.entries as usize];
    proof {
        assert(area32 as int==(fh.rect.width as int)*(fh.rect.height as int));
        assert(area32 >= 1) by(nonlinear_arith)
            requires area32 as int==(fh.rect.width as int)*(fh.rect.height as int),fh.rect.width>=1,fh.rect.height>=1;
        assert(crate::lzw::lzw_ps(data.min_code_size,opts)==crate::frame_model::params(pol));
        assert(fr.spec_frame()==crate::frame_model::frame(crate::parse::frame_header_model(fh),im.gce));
        crate::lzw::lemma_image_data_ready(b@,data.first as int-1,pol,area32,data);
    }
    let rep=crate::compose_new::decode_canvas(b,data,area32,opts,pol.lzw_min,t,cv,colors,fr,hdr.width as u32,hdr.height as u32);
    proof {
        assert(2 * area32 as int == 2 * (fh.rect.width as int) * (fh.rect.height as int)) by(nonlinear_arith)
            requires area32 as int == (fh.rect.width as int) * (fh.rect.height as int);
    }
    rep
}

impl<'a> Frames<'a> {
    /// Exact frame adapter after MB's descriptor/data scan. Separate reservation
    /// and prepared-drawing queries establish the semantic image result.
    #[verifier::spinoff_prover]
    fn image(&mut self, image: parse::ImageStep) -> (r: Result<FrameMeta, TryError>)
        requires storage_wf(*old(self)),old(self).state==State::Running,image_ready(*old(self),image),
        ensures storage_wf(*final(self)),image_result(*old(self),image,*final(self),r),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).still==old(self).still,
            r is Ok ==> final(self).state==State::Running,
            r is Err ==> final(self).state==State::Fused,
            final(self).log@.view()==if crate::frame_model::raw_disposal(image.gce)==3 && !old(self).still && old(self).saved is None {
                old(self).log@.view().push((AllocSite::Saved,old(self).canvas@.len(),!(r is Err && r->Err_0 is OutOfMemory)))
            } else {old(self).log@.view()},
        image_cost(*old(self),image,*final(self),r),
    {
        hide(crate::model::mp);hide(crate::parse::hdr_ok);
        hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);hide(crate::mc::draw_fold);
        hide(crate::lzw_model::ML);
        hide(crate::parse::image_data_post);hide(crate::model::sub_scan);hide(crate::lzw::read_d);
        hide(storage_wf);hide(image_result);hide(prepared_matches);
        proof {lemma_storage_bounds(*self);}
        let ghost original=*self;
        let b = self.b;
        let fh = image.fh;
        let data = image.data;
        let gce = image.gce;
        self.p = data.end;
        let disposal = match gce {
            Some(g) => g.disposal,
            None => 0,
        };
        match self.prepare_canvas(fh.rect,disposal) {
            Ok(())=>{},
            Err(e)=>{
                proof {lemma_image_oom(original,image,*self,e);}
                return Err(e);
            },
        }
        self.image_prepared(image,Ghost(original))
    }

    #[verifier::spinoff_prover]
    fn render_pixels(&mut self,image:parse::ImageStep)->(report:crate::lzw::LzwReport)
        requires storage_wf(*old(self)),image_ready(*old(self),image),
        ensures storage_wf(*final(self)),measured_pixels(*old(self),*final(self),image),
            final(self).canvas@.len()==old(self).canvas@.len(),final(self).table@.len()==old(self).table@.len(),
            pixels_observation(old(self).b@,old(self).hdr,old(self).pol,image,old(self).canvas@,final(self).canvas@,report),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).still==old(self).still,final(self).state==old(self).state,final(self).prev==old(self).prev,
            saved_view(*final(self))==saved_view(*old(self)),(final(self).saved is Some)==(old(self).saved is Some),
            final(self).log@.view()==old(self).log@.view(),
    {
        hide(storage_wf);hide(pixels_observation);hide(measured_pixels);
        hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::parse::image_data_post);hide(crate::lzw::read_d);
        proof {lemma_storage_bounds(*self);}
        let ghost before=*self;
        let report=decode_image_pixels(self.b,self.hdr,self.pol,image,&mut self.table,&mut self.canvas);
        proof {
            self.work@=self.work@+report.work@;
            self.charged_pixels@=(self.charged_pixels@ as int+rect_area(image.fh.rect)) as nat;
            lemma_measured_pixels(before,*self,image,report.work@);
            reveal(prev_bounds);lemma_storage_reframe(before,*self);
        }
        report
    }

    #[verifier::spinoff_prover]
    fn image_prepared(&mut self,image:parse::ImageStep,Ghost(original):Ghost<Frames<'a>>)->(r:Result<FrameMeta,TryError>)
        requires storage_wf(*old(self)),old(self).state==State::Running,image_ready(*old(self),image),image_ready(original,image),
            old(self).b==original.b,old(self).hdr==original.hdr,old(self).pol==original.pol,old(self).p==original.p,
            old(self).still==original.still,walk_state_same(original,*old(self)),old(self).prev==original.prev,
            crate::frame_model::raw_disposal(image.gce)==3 && !old(self).still ==> old(self).saved is Some,
            prepared_matches(original,*old(self),image.fh.rect,crate::frame_model::raw_disposal(image.gce)),
        ensures storage_wf(*final(self)),image_result(original,image,*final(self),r),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).still==old(self).still,final(self).log@.view()==old(self).log@.view(),
            r is Ok ==> final(self).state==State::Running,
            r is Err ==> final(self).state==State::Fused && r->Err_0 is Format,
        r is Ok ==> final(self).prev is Some && final(self).prev.unwrap().rect==image.fh.rect,
            saved_view(*final(self))==saved_view(*old(self)),
            (final(self).saved is Some)==(old(self).saved is Some),
            measured_pixels(*old(self),*final(self),image),
    {
        hide(crate::model::mp);hide(crate::parse::hdr_ok);
        hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);hide(crate::mc::draw_fold);
        hide(crate::lzw_model::ML);hide(crate::lzw::read_d);hide(crate::model::sub_scan);hide(crate::parse::image_data_post);
        hide(storage_wf);hide(image_result);hide(prepared_matches);hide(pixels_observation);hide(measured_pixels);
        proof {lemma_storage_bounds(*self);}
        let fh=image.fh;
        let disposal=match image.gce {Some(g)=>g.disposal,None=>0};
        let ghost prepared=*self;
        let report=self.render_pixels(image);
        let ghost costed=*self;
        let ghost observed=report;
        let overrun=match report.public_result() {
            Ok(v)=>v,
            Err(e)=>{
                self.state=State::Fused;
                proof {
                    reveal(prev_bounds);lemma_storage_reframe(prepared,*self);
                    lemma_pixels_cost_frame(prepared,costed,*self,image);
                    lemma_image_finish(original,image,prepared,*self,observed,Err(e));
                }
                return Err(TryError::Format(e));
            },
        };
        self.prev = Some(Prev { rect: fh.rect, disposal: if disposal > 3 { 1 } else { disposal } });
        proof {reveal(prev_bounds);lemma_storage_reframe(prepared,*self);}
        let meta=frame_metadata(image,overrun);
        proof {lemma_pixels_cost_frame(prepared,costed,*self,image);lemma_image_finish(original,image,prepared,*self,observed,Ok(meta));}
        Ok(meta)
    }
}
} // verus!

verus! {
/// `try_decode_still_with_policy(b, Profile::Strict)` without the flag (G14).
pub fn try_decode_still(b: &[u8]) -> (r: Result<Still, TryError>)
    ensures still_core_result(b@,r),T13_core(b@,r),
{
    let r=match try_decode_still_with_policy(b, Profile::Strict) {
        Ok((s, _)) => Ok(s),
        Err(e) => Err(e),
    };
    proof {lemma_T13_core(b@,r);}
    r
}

/// G14 under a profile: the first frame on the logical screen and its
/// `recovered` flag. Bytes after the first image's block terminator are not
/// examined. `Req = [(Canvas, 4·W·H), (LzwTable, 20_480)]` when G2–G4 pass.
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> (r: Result<(Still, bool), TryError>)
    ensures still_result(b@,crate::policy::profile_record(p),r),T13_still(b@,crate::policy::profile_record(p),r),
        P_T9_still(b@,crate::policy::profile_record(p),r),
        p==Profile::Strict && r is Ok ==> !r.unwrap().1,
        crate::policy_proofs::obligation_r_still_result(b@,crate::policy::profile_record(p),r),
{
    let r=still_with(b, p.policy());
    proof {
        crate::policy_proofs::lemma_obligation_r_still_result(b@,crate::policy::profile_record(p),r);
        if p==Profile::Strict {
            crate::policy::lemma_profile_records();
            crate::policy_proofs::lemma_still_no_recovery(b@,crate::policy::profile_record(p));
        }
    }
    r
}

/// Test-only: the still entry point for an arbitrary record.
#[cfg(feature = "test-records")]
#[doc(hidden)]
pub fn try_decode_still_with_record(b: &[u8], pol: Policy) -> (r: Result<(Still, bool), TryError>)
    ensures still_result(b@,pol,r),T13_still(b@,pol,r),P_T9_still(b@,pol,r),
{
    still_with(b, pol)
}

fn still_with(b: &[u8], pol: Policy) -> (out: Result<(Still, bool), TryError>)
    ensures still_result(b@,pol,out),T13_still(b@,pol,out),P_T9_still(b@,pol,out),
{
    let r=still_with_logged(b,pol);
    proof {reveal(still_g);lemma_T13_still(b@,pol,r.0);}
    r.0
}

#[verifier::spinoff_prover]
fn still_with_logged(b:&[u8],pol:Policy)->(out:(Result<(Still,bool),TryError>,Tracked<AllocLog>))
    ensures still_g(b@,pol,out.0,out.1@.view()),P_T9_still(b@,pol,out.0),
{
    let mut measured=Ghost(0nat);
    still_with_logged_measured(b,pol,&mut measured)
}

#[verifier::spinoff_prover]
fn still_with_logged_measured(b:&[u8],pol:Policy,measured:&mut Ghost<nat>)->(out:(Result<(Still,bool),TryError>,Tracked<AllocLog>))
    ensures still_g(b@,pol,out.0,out.1@.view()),P_UNIV_still(b@,pol,out.0,out.1@.view(),final(measured)@),
{
    hide(crate::model::mp);hide(crate::frame_model::ms);hide(still_result);hide(still_g);
    hide(step_result);hide(storage_wf);
    let opened=Frames::open_logged_measured(b,pol,true,measured);
    proof {reveal(construction_log);reveal(P_UNIV_constructor);crate::work_model::lemma_initial_cur_bounds(b@,pol);}
    let (result,Tracked(error_log))=opened;
    let mut d=match result {
        Ok(d)=>d,
        Err(e)=>{
            let tracked log=error_log.tracked_unwrap();
            let r=Err(e);
            proof {lemma_still_constructor_error(b@,pol,e);lemma_still_package(b@,pol,r,log.view());
                lemma_p_univ_still(b@,pol,r,log.view(),measured@);}
            return (r,Tracked(log));
        },
    };
    proof {reveal(constructor_result);reveal(constructor_ok);}
    let ghost initial=d;
    match d.state {
        State::Running=>{},
        _=>{
            proof {
                lemma_still_empty(d);reveal(constructor_work);lemma_work_total(d);
                lemma_still_empty_work_area(d);
                assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+d.charged_pixels@ as int)+C_FIXED as int);
                assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+crate::work_model::P(b@,pol,None,1) as int)+C_FIXED as int);
            }
            let r=Err(TryError::Format(Error::NoImage));
            *measured=Ghost(d.work@);
            let tracked log=d.log.get();
            proof {lemma_still_package(b@,pol,r,log.view());lemma_p_univ_still(b@,pol,r,log.view(),measured@);}
            return (r,Tracked(log));
        },
    }
    let step=d.step();
    proof {
        lemma_still_step(initial,d,step);lemma_still_trace(initial,d,step);
        reveal(constructor_work);lemma_next_work(initial,d);
        lemma_still_work_area(initial,d,step);
        assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+d.charged_pixels@ as int)+C_FIXED as int);
        assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+crate::work_model::P(b@,pol,None,1) as int)+C_FIXED as int);
    }
    *measured=Ghost(d.work@);
    let r=match step {
        Err(e)=>Err(e),
        Ok(Some(m))=>Ok((Still {width:d.hdr.width,height:d.hdr.height,rgba:d.canvas},m.recovered)),
        Ok(None)=>Err(TryError::Format(Error::NoImage)),
    };
    let tracked log=d.log.get();
    proof {lemma_still_package(b@,pol,r,log.view());lemma_p_univ_still(b@,pol,r,log.view(),measured@);}
    (r,Tracked(log))
}

pub fn try_decode_still_with_policy_logged(b:&[u8],p:Profile)->(r:(Result<(Still,bool),TryError>,Tracked<AllocLog>))
    ensures still_g(b@,crate::policy::profile_record(p),r.0,r.1@.view()),T13_still(b@,crate::policy::profile_record(p),r.0),
        P_T9_still(b@,crate::policy::profile_record(p),r.0),
{
    let r=still_with_logged(b,p.policy());
    proof {reveal(still_g);lemma_T13_still(b@,crate::policy::profile_record(p),r.0);}
    r
}

pub fn try_decode_still_logged(b:&[u8])->(out:(Result<Still,TryError>,Tracked<AllocLog>))
    ensures still_core_g(b@,out.0,out.1@.view()),T13_core(b@,out.0),
{
    let (r,log)=still_with_logged(b,Profile::Strict.policy());
    let ghost observed=r;
    let result=match r {Ok((s,_))=>Ok(s),Err(e)=>Err(e)};
    proof {lemma_still_strip(b@,observed,log@.view());reveal(still_core_g);lemma_T13_core(b@,result);}
    (result,log)
}
} // verus!


verus! {
struct Prepared {
    hdr:Header,
    p:usize,
    st:WalkState,
    stop:crate::parse::PrefixStop,
    canvas:Vec<u8>,
    table:Vec<u8>,
    work:Ghost<nat>,
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
fn prepare(b:&[u8],pol:Policy,steps:&mut Ghost<nat>,Tracked(log):Tracked<&mut AllocLog>)->(r:Result<Prepared,TryError>)
    requires log.view()==Seq::<(AllocSite,nat,bool)>::empty(),
    ensures prepare_result(b@,pol,r),
        crate::alloc_guard::sites::log_tied(r,final(log).view()),
        g_new_clause2(b@,r is Err && r->Err_0 is OutOfMemory,final(log).view()),g_new_clause3(b@),
        r is Ok ==> crate::alloc_guard::req_of_log(final(log).view())==req_new(b@),
        crate::alloc_guard::req_of_log(final(log).view()).is_prefix_of(req_new(b@)),
        !(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(final(log).view())==req_new(b@),
        old(steps)@<=final(steps)@,
        r is Err && r->Err_0 is OutOfMemory ==> final(steps)@ as int<=old(steps)@ as int+4*header_pixels(b@) as int+crate::lzw::LZW_TABLE_BYTES as int,
        match r {
            Ok(d)=>d.work@==final(steps)@ && final(steps)@ <= old(steps)@ + C_CALL * (d.p as nat+header_pixels(b@)) + C_FIXED
                && final(steps)@ as int<=old(steps)@ as int+4*header_pixels(b@) as int+crate::lzw::LZW_TABLE_BYTES as int+4*d.p as int+1,
            Err(_)=>final(steps)@ <= old(steps)@ + C_CALL * (b.len() as nat+header_pixels(b@)) + C_FIXED,
        },
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
        reveal(header_pixels);
        assert(header_pixels(b@) as int==(hdr.width as int)*(hdr.height as int));
        assert(4*(hdr.width as int)*(hdr.height as int)==4*header_pixels(b@) as int) by(nonlinear_arith)
            requires header_pixels(b@) as int==(hdr.width as int)*(hdr.height as int);

    }
    let (canvas,table)=match reserve(b,&hdr,steps,Tracked(log)) {
        Ok(v)=>v,Err(e)=>return Err(e),
    };
    let mut p=hdr.cursor;
    let mut st=WalkState::new();
    let stop=match crate::parse::parse_mp(b,&mut p,&pol,&mut st,steps) {
        Ok(stop)=>stop,Err(e)=>return Err(TryError::Format(e)),
    };
    Ok(Prepared {hdr,p,st,stop,canvas,table,work:Ghost(steps@)})
}
} // verus!

verus! {
pub open spec fn info_model(i:Info)->crate::model::InfoM {
    crate::model::InfoM {width:i.width as int,height:i.height as int,version:crate::model::version_code(i.version),
        background_index:i.background_index as int,aspect:i.aspect as int,loop_count:i.loop_count}
}

/// Storage and parser bounds required between frame calls. The semantic
/// canvas/disposal relation to the full frame model is a separate invariant.
/// Frozen work observations after n+1 calls, including the actual OOM cut.
pub closed spec fn work_wf(d:Frames,n:int,cut:Option<nat>)->bool {
    &&& wf_p(d,n) && work_accounted(d)
    &&& match cut {
        None=>crate::alloc_guard::all_ok(d.log@.view()),
        Some(i)=>i as int<=n && d.state==State::Fused
            && crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i),
    }
    &&& effective_cursor(d)==crate::work_model::cur_n(d.b@,d.pol,cut,n)
    &&& d.charged_pixels@==crate::work_model::P(d.b@,d.pol,cut,(n+1) as nat)
    &&& (d.state==State::Running ==> prior_area(d)==crate::work_model::A_n(d.b@,d.pol,cut,n))
}
spec fn call_oom(r:Option<Result<FrameView,TryError>>)->bool {
    r is Some && r->Some_0 is Err && r->Some_0->Err_0 is OutOfMemory
}
spec fn next_cut(cut:Option<nat>,k:nat,r:Option<Result<FrameView,TryError>>)->Option<nat> {
    if call_oom(r) {Some(k)} else {cut}
}
/// T12 in the frozen names, call k beginning in the state indexed k-1.
pub closed spec fn T12_next(a:Frames,d:Frames,k:nat,r:Option<Result<FrameView,TryError>>)->bool {
    forall|cut:Option<nat>| #[trigger] work_wf(a,k as int-1,cut) ==> {
        let nc=next_cut(cut,k,r);
        &&& work_wf(d,k as int,nc)
        &&& d.work@ as int-a.work@ as int<=C_CALL as int *
            (crate::work_model::cur_n(a.b@,a.pol,nc,k as int)-crate::work_model::cur_n(a.b@,a.pol,cut,k as int-1)
             +crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int
             +crate::work_model::A_n(a.b@,a.pol,cut,k as int-1) as int
             +if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k) {header_pixels(a.b@) as int} else {0})
        &&& d.work@ as int<=C_CALL as int*(a.b.len() as int+2*header_pixels(a.b@) as int+2*crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
        &&& d.work@ as int<=C_WORK as int*(a.b.len() as int+header_pixels(a.b@) as int+crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
    }
}

pub closed spec fn T13_resources(d:Frames)->bool {
    &&& 1<=d.hdr.width<=crate::limits::MAX_DIMENSION && 1<=d.hdr.height<=crate::limits::MAX_DIMENSION
    &&& d.canvas@.len()==4*(d.hdr.width as int)*(d.hdr.height as int)<=crate::limits::MAX_OUTPUT
    &&& d.table@.len()==crate::lzw::LZW_TABLE_BYTES
    &&& match d.saved {None=>true,Some(v)=>v@.len()==d.canvas@.len()}
    &&& d.canvas@.len() as int+d.table@.len() as int+(match d.saved {None=>0int,Some(v)=>v@.len() as int})
        <=2*d.canvas@.len() as int+crate::lzw::LZW_TABLE_BYTES as int
}
pub closed spec fn T13_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)->bool {
    &&& still_result(b,p,r)
    &&& match r {
        Ok((s,_))=>1<=s.width<=crate::limits::MAX_DIMENSION && 1<=s.height<=crate::limits::MAX_DIMENSION
            && s.rgba@.len()==4*(s.width as int)*(s.height as int)<=crate::limits::MAX_OUTPUT
            && crate::mc::pixels_well_formed(s.rgba@,crate::pixel_model::active_tables(b,crate::model::mh(b).unwrap(),p,1)),
        _=>true,
    }
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> {
        &&& (crate::frame_model::ms(b,p) is Some && crate::frame_model::ms(b,p).unwrap() is Ok ==> r is Ok)
        &&& forall|e:Error| (r is Err && r->Err_0==TryError::Format(e)) <==> crate::frame_model::ms(b,p)==Some(Err(e))
    })
}
pub closed spec fn T13_core(b:Seq<u8>,r:Result<Still,TryError>)->bool {
    let p=crate::policy::profile_record(Profile::Strict);
    &&& still_core_result(b,r)
    &&& match r {
        Ok(s)=>1<=s.width<=crate::limits::MAX_DIMENSION && 1<=s.height<=crate::limits::MAX_DIMENSION
            && s.rgba@.len()==4*(s.width as int)*(s.height as int)<=crate::limits::MAX_OUTPUT
            && crate::mc::pixels_well_formed(s.rgba@,crate::pixel_model::active_tables(b,crate::model::mh(b).unwrap(),p,1)),
        _=>true,
    }
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> {
        &&& (crate::frame_model::ms(b,p) is Some && crate::frame_model::ms(b,p).unwrap() is Ok ==> r is Ok)
        &&& forall|e:Error| (r is Err && r->Err_0==TryError::Format(e)) <==> crate::frame_model::ms(b,p)==Some(Err(e))
    })
}
pub closed spec fn T13_view(d:Frames,n:int,r:Option<Result<FrameView,TryError>>)->bool {
    &&& T13_resources(d)
    &&& match r {
        Some(Ok(v))=>v.rgba@==d.canvas@ && crate::mc::pixels_well_formed(v.rgba@,
            crate::pixel_model::active_tables(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat)),
        _=>true,
    }
}
proof fn lemma_T13_resources(d:Frames)
    requires storage_wf(d),ensures T13_resources(d),
{lemma_storage_bounds(d);}
proof fn lemma_T13_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)
    requires still_result(b,p,r),ensures T13_still(b,p,r),
{
    hide(crate::frame_model::ms);
    crate::pixel_model::lemma_ms_pixels(b,p);
}
/// Expose the profiled call's exact model observation without opening its
/// closed allocation-aware result predicate in another proof module.
pub proof fn lemma_still_policy_observation(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)
    requires still_result(b,p,r),
    ensures match r {
        Ok((s,recovered))=>crate::frame_model::ms(b,p)==Some(Ok(crate::frame_model::StillImage {
            width:s.width as int,height:s.height as int,rgba:s.rgba@,recovered})),
        Err(TryError::Format(e))=>crate::frame_model::ms(b,p)==Some(Err(e)),
        Err(TryError::OutOfMemory(_))=>crate::model::mh(b) is Ok,
    },
{}

pub proof fn lemma_still_core_observation(b:Seq<u8>,r:Result<Still,TryError>)
    requires still_core_result(b,r),
    ensures match r {
        Ok(s)=>match crate::frame_model::ms(b,crate::policy::profile_record(Profile::Strict)) {
            Some(Ok(m))=>m.width==s.width as int && m.height==s.height as int && m.rgba==s.rgba@,
            _=>false,
        },
        Err(TryError::Format(e))=>crate::frame_model::ms(b,crate::policy::profile_record(Profile::Strict))==Some(Err(e)),
        Err(TryError::OutOfMemory(_))=>crate::model::mh(b) is Ok,
    },
{}

proof fn lemma_T13_core(b:Seq<u8>,r:Result<Still,TryError>)
    requires still_core_result(b,r),ensures T13_core(b,r),
{
    hide(crate::frame_model::ms);
    crate::pixel_model::lemma_ms_pixels(b,crate::policy::profile_record(Profile::Strict));
}
#[verifier::spinoff_prover]
proof fn lemma_T13_next(a:Frames,d:Frames,n:int,r:Option<Result<FrameView,TryError>>)
    requires wf_p(d,n+1),next_result(a,d,r),fixed_stream(a,d),
    ensures T13_view(d,n+1,r),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::frame_model::after_calls);
    lemma_T13_resources(d);
    if r is Some && r->Some_0 is Ok {
        reveal(next_result);
        assert(d.state==State::Running);
        lemma_wf_running(d,n+1);lemma_work_mh(d);
        crate::pixel_model::lemma_after_pixels(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+2) as nat);
    }
}

pub closed spec fn constructor_work(d:Frames)->bool {
    &&& d.work@ <= C_CALL * (d.p as nat+header_pixels(d.b@)) + C_FIXED
    &&& d.charged_pixels@==0
    &&& work_accounted(d)
    &&& (!d.still ==> work_wf(d,-1,None))
    &&& T13_resources(d)
}
pub closed spec fn work_ready(d:Frames)->bool {
    d.state==State::Running ==> d.prev is Some || d.p<d.b.len() && d.b@[d.p as int]==0x2c
}
spec fn work_potential(d:Frames)->int {
    d.work@ as int + if d.state==State::Running {2*prior_area(d)+1} else {0}
        + if d.saved is None {d.canvas@.len() as int} else {0}
}
pub closed spec fn work_accounted(d:Frames)->bool {
    &&& work_ready(d)
    &&& work_potential(d)<=C_CALL as int * (effective_cursor(d)+header_pixels(d.b@) as int+2*d.charged_pixels@ as int)+C_FIXED as int
}
pub closed spec fn total_work_bound(d:Frames)->bool {
    d.work@ as int<=C_CALL as int*(d.b.len() as int+2*header_pixels(d.b@) as int+2*d.charged_pixels@ as int)+C_FIXED as int
        && d.work@ as int<=C_WORK as int*(d.b.len() as int+header_pixels(d.b@) as int+d.charged_pixels@ as int)+C_FIXED as int
}
pub closed spec fn work_view(d:Frames)->nat {d.work@}
pub closed spec fn charged_pixels(d:Frames)->nat {d.charged_pixels@}
spec fn rect_area(r:Rect)->nat {(r.width as nat)*(r.height as nat)}
spec fn prior_area(d:Frames)->nat {match d.prev {None=>0,Some(p)=>rect_area(p.rect)}}
spec fn saved_fill(a:Frames,d:Frames)->int {
    if a.saved is None && d.saved is Some {a.canvas@.len() as int} else {0}
}
spec fn effective_cursor(d:Frames)->int {if d.state==State::Fused {d.b.len() as int} else {d.p as int}}
spec fn step_model_work(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)->bool {
    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+
        if r is Err && r->Err_0 is OutOfMemory {0int} else {crate::work_model::mb_area(a.b@,crate::parse::header_model(a.hdr),a.pol,crate::parse::walk_m_at(a.st,a.p)) as int}
    &&& effective_cursor(d)==if r is Err {a.b.len() as int} else {crate::work_model::mb_cur(a.b@,crate::parse::header_model(a.hdr),a.pol,crate::parse::walk_m_at(a.st,a.p))}
}
pub closed spec fn next_model_work(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>)->bool {
    if a.state!=State::Running {d==a} else {
        step_model_work(a,d,match r {None=>Ok(None),Some(Err(e))=>Err(e),Some(Ok(v))=>Ok(Some(view_metadata(v)))})
    }
}

spec fn image_cost(a:Frames,im:parse::ImageStep,d:Frames,r:Result<FrameMeta,TryError>)->bool {
    let area=if r is Err && r->Err_0 is OutOfMemory {0nat} else {rect_area(im.fh.rect)};
    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+area
    &&& (a.saved is Some ==> d.saved is Some)
    &&& (r is Ok ==> d.prev is Some && prior_area(d)==area)
    &&& a.work@<=d.work@
    &&& d.work@ as int <= a.work@ as int + 5*im.data.data_len as int + 4*area + 2*prior_area(a) + saved_fill(a,d)
}
spec fn step_cost(a:Frames,d:Frames)->bool {
    &&& a.work@<=d.work@ && a.charged_pixels@<=d.charged_pixels@
    &&& a.p<=d.p
    &&& (a.saved is Some ==> d.saved is Some)
    &&& (d.state==State::Running ==> d.prev is Some && a.p<d.p && prior_area(d) as int==d.charged_pixels@ as int-a.charged_pixels@ as int)
    &&& (d.state==State::Done && a.p<a.b.len() && a.b@[a.p as int]==0x2c ==> false)
    &&& d.work@ as int <= a.work@ as int + 9*(effective_cursor(d)-a.p as int)
        +4*(d.charged_pixels@ as int-a.charged_pixels@ as int)+2*prior_area(a)+saved_fill(a,d)+1
}

pub closed spec fn next_work(a:Frames,d:Frames)->bool {
    &&& a.work@<=d.work@ && a.charged_pixels@<=d.charged_pixels@
    &&& (a.state!=State::Running ==> a.work@==d.work@ && a.charged_pixels@==d.charged_pixels@)
    &&& (a.state==State::Running ==> step_cost(a,d))
    &&& (work_accounted(a) ==> work_accounted(d) && total_work_bound(d))
    &&& (work_ready(a) ==> d.work@ as int <= a.work@ as int + C_CALL as int *
        (effective_cursor(d)-effective_cursor(a)+d.charged_pixels@ as int-a.charged_pixels@ as int
         + if a.state==State::Running {prior_area(a) as int} else {0}
         + if a.saved is None && d.saved is Some {header_pixels(a.b@) as int} else {0}))
}
spec fn measured_pixels(a:Frames,d:Frames,im:parse::ImageStep)->bool {
    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+rect_area(im.fh.rect)
    &&& a.work@<=d.work@
    &&& d.work@ as int<=a.work@ as int+5*im.data.data_len as int+2*rect_area(im.fh.rect)
}
proof fn lemma_measured_pixels(a:Frames,d:Frames,im:parse::ImageStep,cost:nat)
    requires d.work@==a.work@+cost,d.charged_pixels@==a.charged_pixels@+rect_area(im.fh.rect),
        cost as int<=5*im.data.data_len as int+2*rect_area(im.fh.rect),
    ensures measured_pixels(a,d,im),
{}
proof fn lemma_pixels_cost_frame(a:Frames,p:Frames,d:Frames,im:parse::ImageStep)
    requires measured_pixels(a,p,im),d.work@==p.work@,d.charged_pixels@==p.charged_pixels@,
    ensures measured_pixels(a,d,im),
{}
/// Operational T12 observations: work counts concrete bodies/fills and the
/// pixel charge increases only after preparation succeeds, at the G10 entry.
pub closed spec fn t12_measured_call(a:Frames,d:Frames,n:int)->bool {
    d.work@ as int<=a.work@ as int+C_CALL as int *
        (effective_cursor(d)-effective_cursor(a)+d.charged_pixels@ as int-a.charged_pixels@ as int
         +(if a.state==State::Running {prior_area(a) as int} else {0})
         +(if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,(n+1) as nat) {header_pixels(a.b@) as int} else {0}))
}
proof fn lemma_work_total(d:Frames)
    requires storage_wf(d),work_accounted(d),
    ensures total_work_bound(d),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    lemma_storage_bounds(d);
}
proof fn lemma_t12_measured_call(a:Frames,d:Frames,n:int,r:Option<Result<FrameView,TryError>>)
    requires storage_wf(a),storage_wf(d),wf_p(a,n),next_allocation(a,d,n,r),next_work(a,d),work_ready(a),
    ensures t12_measured_call(a,d,n),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    hide(crate::frame_model::saved_at);hide(iterator_log);hide(iterator_observation);
    reveal(storage_wf);reveal(next_allocation);
    if a.saved is None && d.saved is Some {
        assert(a.log@.view().len()>=2);
        assert(d.log@.view().len()==3);
        assert(crate::alloc_guard::all_ok(d.log@.view()));
        assert(crate::alloc_guard::all_ok(a.log@.view()));
        assert(crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,(n+1) as nat));
    }
}

/// Hoare-style finite histories of the real APIs. An unchanged snapshot is
/// an info call; every non-identity edge uses next_frame's proved next_work.
pub proof fn lemma_finite_work_history(ds:Seq<Frames>)
    requires ds.len()>=1,constructor_work(ds[0]),
        forall|i:int| 0<=i<ds.len() ==> storage_wf(#[trigger] ds[i]),
        forall|i:int| 0<=i<ds.len()-1 ==> ds[i+1]==ds[i] || next_work(#[trigger] ds[i],ds[i+1]),
    ensures work_accounted(ds.last()),total_work_bound(ds.last()),
    decreases ds.len(),
{
    if ds.len()==1 {lemma_work_total(ds[0]);}
    else {
        let prefix=ds.take(ds.len() as int-1);
        assert forall|i:int| 0<=i<prefix.len() implies storage_wf(#[trigger] prefix[i]) by {assert(prefix[i]==ds[i]);}
        assert forall|i:int| 0<=i<prefix.len()-1 implies prefix[i+1]==prefix[i] || next_work(#[trigger] prefix[i],prefix[i+1]) by {assert(prefix[i]==ds[i]);assert(prefix[i+1]==ds[i+1]);}
        lemma_finite_work_history(prefix);
        assert(prefix.last()==ds[ds.len() as int-2]);
    }
}

proof fn lemma_work_mh(d:Frames)
    requires storage_wf(d),
    ensures crate::model::mh(d.b@)==Ok(crate::parse::header_model(d.hdr)),
{
    hide(crate::model::mp);reveal(storage_wf);reveal(crate::parse::hdr_ok);
    let h=choose|h:crate::model::HeaderM| crate::model::mh(d.b@)==Ok(h) && crate::parse::hdr_matches(d.b@,h,d.hdr);
    crate::parse::lemma_header_model_matches(d.b@,d.hdr,h);
}

#[verifier::spinoff_prover]
proof fn lemma_work_wf_step(a:Frames,d:Frames,k:nat,cut:Option<nat>,r:Option<Result<FrameView,TryError>>)
    requires work_wf(a,k as int-1,cut),wf_p(d,k as int),fixed_stream(a,d),
        next_work(a,d),next_model_work(a,d,r),next_result(a,d,r),
        next_allocation(a,d,k as int-1,r),
    ensures work_wf(d,k as int,next_cut(cut,k,r)),
{
    hide(crate::model::mp);hide(crate::model::mb);hide(crate::parse::hdr_ok);
    hide(crate::frame_model::call);hide(crate::frame_model::call_mode);hide(crate::frame_model::after_calls);
    hide(crate::frame_model::saved_at);hide(iterator_log);hide(iterator_observation);
    reveal(work_wf);reveal(next_allocation);
    lemma_work_mh(a);lemma_work_mh(d);
    let nc=next_cut(cut,k,r);
    if a.state==State::Running {
        lemma_wf_running(a,k as int-1);
        assert(cut is None);
        let step=match r {None=>Ok(None),Some(Err(e))=>Err(e),Some(Ok(v))=>Ok(Some(view_metadata(v)))};
        reveal(next_result);reveal(step_result);
        assert(step_result(a,d,step));
        lemma_step_call_defined(a,d,step);
        reveal(crate::frame_model::call);
        let c=crate::frame_model::call(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a)).unwrap();
        if call_oom(r) {
            assert(nc==Some(k));
        } else {
            assert(c.error is Some <==> r is Some && r->Some_0 is Err);
        }
    } else {
        assert(d==a);assert(!call_oom(r));
        if cut is None {
            reveal(wf_p);
            let st=crate::frame_model::after_calls(a.b@,crate::parse::header_model(a.hdr),a.pol,k).unwrap();
            assert(st.status!=crate::frame_model::Status::Running);
        }
    }
    if call_oom(r) {
        assert(cut is None);
        assert(crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k));
    } else if cut is None {
        assert forall|i:int| 0<=i<d.log@.view().len() implies #[trigger] d.log@.view()[i].2 by {
            if i<a.log@.view().len() {assert(d.log@.view()[i]==a.log@.view()[i]);}
        }
        assert(crate::alloc_guard::all_ok(d.log@.view()));
    }
    crate::work_model::lemma_P_next(a.b@,a.pol,cut,nc,k);
    assert(d.charged_pixels@ as int-a.charged_pixels@ as int==crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int);
    reveal_with_fuel(crate::work_model::cur_calls,1);
    assert(effective_cursor(d)==crate::work_model::cur_n(a.b@,a.pol,nc,k as int));
}

#[verifier::spinoff_prover]
proof fn lemma_T12_next(a:Frames,d:Frames,k:nat,r:Option<Result<FrameView,TryError>>)
    requires storage_wf(a),storage_wf(d),fixed_stream(a,d),next_work(a,d),next_model_work(a,d,r),next_result(a,d,r),
        wf_p(a,k as int-1) ==> wf_p(d,k as int),
        wf_p(a,k as int-1) ==> next_allocation(a,d,k as int-1,r),
        wf_p(a,k as int-1) && work_ready(a) ==> t12_measured_call(a,d,k as int-1),
    ensures T12_next(a,d,k,r),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    hide(crate::work_model::P);hide(crate::work_model::cur_n);hide(crate::work_model::A_n);
    assert forall|cut:Option<nat>| #[trigger] work_wf(a,k as int-1,cut) implies {
        let nc=next_cut(cut,k,r);
        &&& work_wf(d,k as int,nc)
        &&& d.work@ as int-a.work@ as int<=C_CALL as int *
            (crate::work_model::cur_n(a.b@,a.pol,nc,k as int)-crate::work_model::cur_n(a.b@,a.pol,cut,k as int-1)
             +crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int+crate::work_model::A_n(a.b@,a.pol,cut,k as int-1) as int
             +if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k) {header_pixels(a.b@) as int} else {0})
        &&& d.work@ as int<=C_CALL as int*(a.b.len() as int+2*header_pixels(a.b@) as int+2*crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
        &&& d.work@ as int<=C_WORK as int*(a.b.len() as int+header_pixels(a.b@) as int+crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
    } by {
        reveal(work_wf);
        lemma_work_wf_step(a,d,k,cut,r);
        lemma_work_total(d);
        let nc=next_cut(cut,k,r);
        if a.state==State::Running {
            assert(d.charged_pixels@ as int-a.charged_pixels@ as int==crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int) by {
                reveal(crate::work_model::P);
                if nc!=cut {crate::work_model::lemma_P_cut_prefix(a.b@,a.pol,k,k);}
            }
        } else {
            assert(d==a);
        }
    }
}

proof fn lemma_initial_model(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,d.still),
    ensures crate::frame_model::initial(d.b@,d.pol)==Some(model_state(d)),
{
    hide(crate::model::mp_fuel);reveal(constructor_ok);reveal(storage_wf);
    lemma_work_mh(d);
    let initial=crate::frame_model::initial(d.b@,d.pol).unwrap();
    assert(model_state(d)==initial);
}
proof fn lemma_still_work_area(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)
    requires constructor_ok(a,a.b@,a.pol,true),a.charged_pixels@==0,a.state==State::Running,
        step_model_work(a,d,r),!(r is Err && r->Err_0 is OutOfMemory),
    ensures d.charged_pixels@==crate::work_model::P(a.b@,a.pol,None,1),
{
    hide(crate::model::mp);hide(crate::model::mb);
    lemma_work_mh(a);lemma_initial_model(a);
    reveal_with_fuel(crate::work_model::P,2);reveal(crate::work_model::A_n);reveal(step_model_work);
    reveal_with_fuel(crate::frame_model::after_calls,1);
}
proof fn lemma_still_empty_work_area(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,true),d.charged_pixels@==0,d.state!=State::Running,
    ensures d.charged_pixels@==crate::work_model::P(d.b@,d.pol,None,1),
{
    hide(crate::model::mp);hide(crate::model::mb);
    lemma_work_mh(d);lemma_initial_model(d);
    reveal_with_fuel(crate::work_model::P,2);reveal(crate::work_model::A_n);
    reveal_with_fuel(crate::frame_model::after_calls,1);
}

pub closed spec fn steps_total(ds:Seq<Frames>)->int
    decreases ds.len(),
{
    if ds.len()==0 {0} else if ds.len()==1 {ds[0].work@ as int}
    else {steps_total(ds.take(ds.len() as int-1))+ds.last().work@ as int-ds[ds.len() as int-2].work@ as int}
}
proof fn lemma_steps_total(ds:Seq<Frames>)
    requires ds.len()>0,
    ensures steps_total(ds)==ds.last().work@ as int,
    decreases ds.len(),
{
    reveal_with_fuel(steps_total,1);
    if ds.len()>1 {
        let pr=ds.take(ds.len() as int-1);lemma_steps_total(pr);
        assert(pr.last()==ds[ds.len() as int-2]);
    }
}
/// Constructor plus any finite next_frame history; info calls are immutable
/// and can be inserted without changing any snapshot or work summand.
proof fn theorem_T12_finite(ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>)
    requires ds.len()==rs.len()+1,cuts.len()==ds.len(),cuts[0] is None,
        constructor_work(ds[0]),!ds[0].still,
        forall|i:int| 0<=i<ds.len() ==> storage_wf(#[trigger] ds[i]) && ds[i].b==ds[0].b && ds[i].pol==ds[0].pol,
        forall|i:int| 0<=i<rs.len() ==> T12_next(#[trigger] ds[i],ds[i+1],i as nat,rs[i])
            && cuts[i+1]==next_cut(cuts[i],i as nat,rs[i]),
    ensures work_wf(ds.last(),rs.len() as int-1,cuts.last()),
        steps_total(ds)<=C_CALL as int*(ds[0].b.len() as int+2*header_pixels(ds[0].b@) as int
            +2*crate::work_model::P(ds[0].b@,ds[0].pol,cuts.last(),rs.len()) as int)+C_FIXED as int,
        steps_total(ds)<=C_WORK as int*(ds[0].b.len() as int+header_pixels(ds[0].b@) as int
            +crate::work_model::P(ds[0].b@,ds[0].pol,cuts.last(),rs.len()) as int)+C_FIXED as int,
    decreases rs.len(),
{
    hide(crate::work_model::P);hide(crate::work_model::A_n);hide(crate::work_model::cur_n);
    if rs.len()==0 {reveal(constructor_work);lemma_work_total(ds[0]);}
    else {
        let dp=ds.take(ds.len() as int-1);
        let rp=rs.take(rs.len() as int-1);
        let cp=cuts.take(cuts.len() as int-1);
        assert forall|i:int| 0<=i<dp.len() implies storage_wf(#[trigger] dp[i]) && dp[i].b==dp[0].b && dp[i].pol==dp[0].pol by {assert(dp[i]==ds[i]);}
        assert forall|i:int| 0<=i<rp.len() implies T12_next(#[trigger] dp[i],dp[i+1],i as nat,rp[i])
            && cp[i+1]==next_cut(cp[i],i as nat,rp[i]) by {assert(dp[i]==ds[i]);assert(dp[i+1]==ds[i+1]);}
        theorem_T12_finite(dp,rp,cp);
        let i=rs.len() as int-1;
        assert(dp.last()==ds[i]);assert(cp.last()==cuts[i]);
        reveal(T12_next);
    }
    lemma_steps_total(ds);
}

proof fn lemma_work_header(d:Frames)
    requires storage_wf(d),
    ensures d.canvas@.len() as int==4*header_pixels(d.b@) as int,
{
    hide(crate::model::mp);
    reveal(storage_wf);reveal(crate::parse::hdr_ok);
    let h=choose|h:crate::model::HeaderM| crate::model::mh(d.b@)==Ok(h) && crate::parse::hdr_matches(d.b@,h,d.hdr);
    assert(header_pixels(d.b@) as int==(d.hdr.width as int)*(d.hdr.height as int));
    assert(4*(d.hdr.width as int)*(d.hdr.height as int)==4*header_pixels(d.b@) as int) by(nonlinear_arith)
        requires header_pixels(d.b@) as int==(d.hdr.width as int)*(d.hdr.height as int);
}
proof fn lemma_constructor_work(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,d.still),d.charged_pixels@==0,!d.still ==> wf_p(d,-1),
        d.work@ as int<=4*header_pixels(d.b@) as int+crate::lzw::LZW_TABLE_BYTES as int+4*d.p as int+1,
    ensures constructor_work(d),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    reveal(constructor_ok);lemma_work_header(d);lemma_work_mh(d);lemma_T13_resources(d);
}
proof fn lemma_work_transition(a:Frames,d:Frames)
    requires storage_wf(a),storage_wf(d),fixed_stream(a,d),
        a.state==State::Running,step_cost(a,d),work_ready(a),
    ensures work_ready(d),
        work_potential(d)<=work_potential(a)+C_CALL as int *
            (effective_cursor(d)-effective_cursor(a)+2*(d.charged_pixels@ as int-a.charged_pixels@ as int)),
        d.work@ as int<=a.work@ as int+C_CALL as int *
            (effective_cursor(d)-effective_cursor(a)+d.charged_pixels@ as int-a.charged_pixels@ as int+prior_area(a) as int
                +if a.saved is None && d.saved is Some {header_pixels(a.b@) as int} else {0}),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    lemma_storage_bounds(a);lemma_storage_bounds(d);
    lemma_work_header(a);
    if a.prev is Some {
        let r=a.prev.unwrap().rect;
        assert(prior_area(a)>=1) by(nonlinear_arith)
            requires r.width>=1,r.height>=1,prior_area(a)==(r.width as nat)*(r.height as nat);
    }
}
proof fn lemma_next_work(a:Frames,d:Frames)
    requires storage_wf(a),storage_wf(d),fixed_stream(a,d),
        if a.state==State::Running {step_cost(a,d)} else {d==a},
    ensures next_work(a,d),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    if a.state==State::Running && work_ready(a) {lemma_work_transition(a,d);}
    if work_accounted(a) {
        lemma_storage_bounds(d);
        assert(work_potential(d)>=d.work@ as int);
    }
}

pub closed spec fn storage_wf(d:Frames)->bool {
    &&& prev_bounds(d)
    &&& (d.state==State::Running && d.st.frames_seen==0 ==> d.p<d.b.len() && d.b@[d.p as int]==0x2c)
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
        Ok(d)=>constructor_ok(d,b,pol,still) && (!still ==> wf_p(d,-1)),
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
        ensures *i==self.info_view(),storage_wf(*self) ==> info_result(*self,*i) && P_T9_info(*self,*i),
    {
        proof {if storage_wf(*self) {lemma_info_result(*self);theorem_p_t9_info(*self,self.info);}}
        &self.info
    }

    fn open_logged(b:&'a [u8],pol:Policy,still:bool)->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,pol,still,r.0),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,r.0 is Err ==> r.1@ is Some,
            r.0 is Ok ==> constructor_work(r.0->Ok_0),P_T9_constructor(b@,pol,still,r.0),
    {
        let mut measured=Ghost(0nat);
        Self::open_logged_measured(b,pol,still,&mut measured)
    }

    fn open_logged_measured(b:&'a [u8],pol:Policy,still:bool,measured:&mut Ghost<nat>)->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,pol,still,r.0),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,
            r.0 is Err ==> r.1@ is Some,
        r.0 is Ok ==> constructor_work(r.0->Ok_0),
        P_UNIV_constructor(b@,pol,still,r.0,construction_log(&r),final(measured)@),
    {
        hide(crate::model::mp);
        reveal(constructor_result);reveal(constructor_ok);reveal(storage_wf);reveal(construction_log);
        let tracked mut log=crate::alloc_guard::new_log();
        let mut steps=Ghost(0nat);
        let prepared=prepare(b,pol,&mut steps,Tracked(&mut log));
        match prepared {
            Err(e)=>{
                proof {
                    assert(steps@<=C_CALL*(b.len() as nat+header_pixels(b@))+C_FIXED);
                    crate::work_model::lemma_initial_cur_bounds(b@,pol);
                    if !(e is OutOfMemory) {reveal(prepare_result);assert(crate::model::mp(b@,pol) is Err);}
                    assert(steps@ as int<=C_CALL as int*(crate::work_model::cur_n(b@,pol,None,-1)+header_pixels(b@) as int)+C_FIXED as int);
                }
                *measured=Ghost(steps@);
                let r=(Err(e),Tracked(Some(log)));
                proof {lemma_p_univ_constructor(b@,pol,still,r.0,construction_log(&r),measured@);}
                r
            },
            Ok(d)=> {
                let info=Info {width:d.hdr.width,height:d.hdr.height,version:d.hdr.version,
                    background_index:d.hdr.background_index,aspect:d.hdr.aspect,loop_count:d.st.loop_count};
                let state=match d.stop {crate::parse::PrefixStop::AtImage=>State::Running,crate::parse::PrefixStop::Trailer=>State::Done};
                let dec=Frames {b,pol,hdr:d.hdr,info,p:d.p,st:d.st,canvas:d.canvas,table:d.table,
                    saved:None,prev:None,state,still,log:Tracked(log),work:Ghost(d.work@),charged_pixels:Ghost(0nat)};
                proof {
                    assert(match crate::model::mp(dec.b@,dec.pol) {Err(_)=>false,Ok((mi,_))=>info_model(dec.info)==mi});
                    assert(dec.log@.view()==seq![(AllocSite::Canvas,dec.canvas@.len(),true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true)]);
                    assert(storage_wf(dec));assert(constructor_ok(dec,b@,pol,still));
                    if !still {lemma_constructor_wf(dec);}
                    lemma_constructor_work(dec);
                    assert(crate::work_model::cur_n(b@,pol,None,-1)==dec.p as int);
                }
                *measured=Ghost(dec.work@);
                let r=(Ok(dec),Tracked(None));
                proof {lemma_p_univ_constructor(b@,pol,still,r.0,construction_log(&r),measured@);}
                r
            },
        }
    }

    fn open(b:&'a [u8],pol:Policy,still:bool)->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,pol,still,r),P_T9_constructor(b@,pol,still,r),
        r is Ok ==> constructor_work(r->Ok_0),
    {Self::open_logged(b,pol,still).0}

    pub fn try_new(b:&'a [u8])->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,crate::policy::profile_record(Profile::Strict),false,r),
        r is Ok ==> constructor_work(r->Ok_0),
    {Self::open(b,Profile::Strict.policy(),false)}

    pub fn try_new_with_policy(b:&'a [u8],p:Profile)->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,crate::policy::profile_record(p),false,r),
            P_T9_constructor(b@,crate::policy::profile_record(p),false,r),
        r is Ok ==> constructor_work(r->Ok_0),
    {Self::open(b,p.policy(),false)}

    pub fn try_new_logged(b:&'a [u8])->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,crate::policy::profile_record(Profile::Strict),false,r.0),
            constructor_g(b@,crate::policy::profile_record(Profile::Strict),r.0,construction_log(&r)),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,r.0 is Err ==> r.1@ is Some,
        r.0 is Ok ==> constructor_work(r.0->Ok_0),
    {
        let r=Self::open_logged(b,Profile::Strict.policy(),false);
        proof {lemma_constructor_package(b@,crate::policy::profile_record(Profile::Strict),r.0,construction_log(&r));}
        r
    }

    pub fn try_new_with_policy_logged(b:&'a [u8],p:Profile)->(r:(Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures constructor_result(b@,crate::policy::profile_record(p),false,r.0),
            P_T9_constructor(b@,crate::policy::profile_record(p),false,r.0),
            constructor_g(b@,crate::policy::profile_record(p),r.0,construction_log(&r)),
            crate::alloc_guard::sites::log_tied(r.0,construction_log(&r)),
            g_new_clause2(b@,r.0 is Err && r.0->Err_0 is OutOfMemory,construction_log(&r)),g_new_clause3(b@),
            crate::alloc_guard::req_of_log(construction_log(&r)).is_prefix_of(req_new(b@)),
            !(r.0 is Err && r.0->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(construction_log(&r))==req_new(b@),
            r.0 is Ok ==> r.1@ is None,r.0 is Err ==> r.1@ is Some,
        r.0 is Ok ==> constructor_work(r.0->Ok_0),
    {
        let r=Self::open_logged(b,p.policy(),false);
        proof {lemma_constructor_package(b@,crate::policy::profile_record(p),r.0,construction_log(&r));}
        r
    }

    #[cfg(feature="test-records")]
    #[doc(hidden)]
    pub fn try_new_with_record(b:&'a [u8],pol:Policy)->(r:Result<Frames<'a>,TryError>)
        ensures constructor_result(b@,pol,false,r),P_T9_constructor(b@,pol,false,r),
        r is Ok ==> constructor_work(r->Ok_0),
    {Self::open(b,pol,false)}

    #[verifier::spinoff_prover]
    fn apply_previous(&mut self)
        requires storage_wf(*old(self)),
        ensures storage_wf(*final(self)),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).table@==old(self).table@,final(self).prev==old(self).prev,final(self).still==old(self).still,
            final(self).state==old(self).state,saved_view(*final(self))==saved_view(*old(self)),
            final(self).log@.view()==old(self).log@.view(),
            final(self).canvas@==crate::frame_model::disposed(old(self).canvas@,saved_view(*old(self)),previous_view(*old(self)),old(self).hdr.width as u32,old(self).hdr.height as u32),
        final(self).charged_pixels@==old(self).charged_pixels@,
            old(self).work@<=final(self).work@,
            final(self).work@ as int<=old(self).work@ as int+2*prior_area(*old(self)),
    {
        hide(crate::model::mp);hide(crate::parse::hdr_ok);
        hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
        hide(storage_wf);reveal(prev_bounds);
        proof {lemma_storage_bounds(*self);}
        let ghost before=*self;
        let sw=self.hdr.width as u32;
        let sh=self.hdr.height as u32;
        if let Some(prev)=self.prev {
            if prev.disposal==2 {let cost=clear_rect(&mut self.canvas,sw,sh,prev.rect);proof {self.work@=self.work@+cost@;}}
            else if prev.disposal==3 {
                if let Some(saved)=&self.saved {let cost=copy_rect(&mut self.canvas,saved,sw,sh,prev.rect);proof {self.work@=self.work@+cost@;}}
            }
        }
        proof {lemma_storage_reframe(before,*self);}
    }

    /// Apply the previous disposal, then save this frame's background before
    /// its first LZW code. Failure exposes no frame and records only Saved OOM.
    #[verifier::spinoff_prover]
    fn prepare_canvas(&mut self,rect:Rect,disposal:u8)->(r:Result<(),TryError>)
        requires storage_wf(*old(self)),old(self).state==State::Running,
            1<=rect.width<=crate::limits::MAX_DIMENSION,1<=rect.height<=crate::limits::MAX_DIMENSION,
            rect.left<=crate::limits::MAX_DIMENSION,rect.top<=crate::limits::MAX_DIMENSION,
        ensures storage_wf(*final(self)),
            r is Ok ==> prepared_matches(*old(self),*final(self),rect,disposal),
            final(self).b==old(self).b,final(self).hdr==old(self).hdr,final(self).pol==old(self).pol,
            final(self).info==old(self).info,final(self).p==old(self).p,walk_state_same(*old(self),*final(self)),
            final(self).table@==old(self).table@,final(self).prev==old(self).prev,final(self).still==old(self).still,
            final(self).canvas@==crate::frame_model::disposed(old(self).canvas@,saved_view(*old(self)),previous_view(*old(self)),old(self).hdr.width as u32,old(self).hdr.height as u32),
            final(self).log@.view()==if disposal==3 && !old(self).still && old(self).saved is None {
                old(self).log@.view().push((AllocSite::Saved,old(self).canvas@.len(),r is Ok))
            } else {old(self).log@.view()},
            r is Ok ==> final(self).state==State::Running
                && saved_view(*final(self))==if disposal==3 && !old(self).still {
                    Some(crate::mc::copy_rect(match saved_view(*old(self)) {Some(v)=>v,None=>Seq::new(old(self).canvas@.len(),|_i:int|0u8)},
                        final(self).canvas@,old(self).hdr.width as u32,old(self).hdr.height as u32,rect.spec_rect()))
                } else {saved_view(*old(self))},
            r is Err ==> disposal==3 && !old(self).still && old(self).saved is None
                && final(self).saved is None && final(self).state==State::Fused
                && match r->Err_0 {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as int==old(self).canvas@.len(),_=>false},
        final(self).charged_pixels@==old(self).charged_pixels@,
            old(self).work@<=final(self).work@,
            final(self).work@ as int<=old(self).work@ as int+2*prior_area(*old(self))+saved_fill(*old(self),*final(self))
                + if r is Ok && disposal==3 && !old(self).still {2*rect_area(rect)} else {0},
    {
        hide(crate::model::mp);hide(crate::parse::hdr_ok);
        hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
        reveal(storage_wf);reveal(prev_bounds);
        let sw=self.hdr.width as u32;
        let sh=self.hdr.height as u32;
        self.apply_previous();
        proof {assert(storage_wf(*self));}
        if disposal==3 && !self.still {
            match self.ensure_saved() {Ok(())=>{},Err(e)=>return Err(e)}
            if let Some(saved)=&mut self.saved {let cost=copy_rect(saved,&self.canvas,sw,sh,rect);proof {self.work@=self.work@+cost@;}}
        }
        proof {assert(storage_wf(*self));}
        Ok(())
    }

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
        final(self).charged_pixels@==old(self).charged_pixels@,
            final(self).work@ as int == old(self).work@ as int+saved_fill(*old(self),*final(self)),
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
                proof {self.work@=self.work@+v@.len();}
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

/// Rectangles remain unnarrowed, including when Clip retains no pixels.
pub closed spec fn prev_bounds(d:Frames)->bool {
    match d.prev {
        None=>true,
        Some(p)=>{
            &&& p.rect.left<=crate::limits::MAX_DIMENSION && p.rect.top<=crate::limits::MAX_DIMENSION
            &&& 1<=p.rect.width<=crate::limits::MAX_DIMENSION && 1<=p.rect.height<=crate::limits::MAX_DIMENSION
            &&& p.disposal<=3
            &&& (p.disposal==3 ==> d.saved is Some || d.still)
        },
    }
}

proof fn lemma_storage_bounds(d:Frames)
    requires storage_wf(d),
    ensures prev_bounds(d),d.b.len()<=crate::limits::MAX_INPUT,crate::parse::hdr_inv(d.b@,d.hdr),
        d.p<=d.b.len(),d.st.frames_seen<=crate::limits::MAX_FRAMES,d.st.total_pixels<=crate::limits::MAX_TOTAL_PIXELS,
        d.st.loop_count==d.info.loop_count,
        d.state==State::Running && d.st.frames_seen==0 ==> d.p<d.b.len() && d.b@[d.p as int]==0x2c,
        match d.saved {None=>true,Some(v)=>v@.len()==d.canvas@.len()},

        d.canvas@.len()==4*(d.hdr.width as int)*(d.hdr.height as int),d.table@.len()==crate::lzw::LZW_TABLE_BYTES,
        1<=d.hdr.width<=crate::limits::MAX_DIMENSION,1<=d.hdr.height<=crate::limits::MAX_DIMENSION,
        4*(d.hdr.width as int)*(d.hdr.height as int)<=crate::limits::MAX_OUTPUT,
{reveal(storage_wf);}

proof fn lemma_storage_reframe(a:Frames,d:Frames)
    requires storage_wf(a),prev_bounds(d),
        d.b==a.b,d.hdr==a.hdr,d.pol==a.pol,d.info==a.info,d.p==a.p,walk_state_same(a,d),d.still==a.still,
        d.canvas@.len()==a.canvas@.len(),d.table@.len()==a.table@.len(),saved_view(d)==saved_view(a),d.log@.view()==a.log@.view(),
        a.state==State::Fused ==> d.state==State::Fused,
        d.state==State::Running ==> a.state==State::Running,
    ensures storage_wf(d),
{reveal(storage_wf);}

spec fn model_state(d:Frames)->crate::frame_model::State {
    crate::frame_model::State {walk:crate::parse::walk_m_at(d.st,d.p),canvas:d.canvas@,
        saved:saved_view(d),prev:previous_view(d),status:match d.state {
            State::Running=>crate::frame_model::Status::Running,State::Done=>crate::frame_model::Status::Done,State::Fused=>crate::frame_model::Status::Failed}}
}

spec fn image_inputs(b:Seq<u8>,hdr:Header,pol:Policy,im:parse::ImageStep)->bool {
    &&& crate::frame_model::image_domain(b,crate::parse::header_model(hdr),crate::parse::frame_header_model(im.fh))
    &&& (im.fh.rect.width as int)*(im.fh.rect.height as int)<=crate::limits::MAX_TOTAL_PIXELS
    &&& crate::parse::image_data_post(b,im.data.first as int-1,pol,(im.fh.rect.width as int)*(im.fh.rect.height as int),Ok(im.data))
}
spec fn image_ready(d:Frames,im:parse::ImageStep)->bool {
    d.p==im.data.end && image_inputs(d.b@,d.hdr,d.pol,im)
}

spec fn meta_matches(m:FrameMeta,f:crate::frame_model::FrameRec)->bool {
    &&& m.rect.0 as int==f.rect.0 && m.rect.1 as int==f.rect.1 && m.rect.2 as int==f.rect.2 && m.rect.3 as int==f.rect.3
    &&& m.delay_cs==f.delay_cs && m.disposal==f.disposal && m.user_input==f.user_input
    &&& m.transparent==f.transparent && m.interlaced==f.interlaced && m.recovered==f.recovered
}

spec fn pixels_observation(b:Seq<u8>,hdr:Header,pol:Policy,im:parse::ImageStep,before:Seq<u8>,after:Seq<u8>,rep:crate::lzw::LzwReport)->bool {
    let mr=crate::lzw_model::ML(im.data.min_code_size as int,crate::lzw::read_d(b,im.data.first as int),
        (im.fh.rect.width as int)*(im.fh.rect.height as int),crate::frame_model::params(pol));
    let fr=crate::frame_model::frame(crate::parse::frame_header_model(im.fh),im.gce);
    &&& crate::lzw::report_matches(rep,mr)
    &&& crate::lzw::colour_observation(rep,mr.idx,|v:u8|fr.transparent==Some(v) || (v as int)<fr.c)
    &&& (rep.colour is None ==> crate::mc::draw_fold(before,fr,
        b.subrange(im.fh.table.offset as int,im.fh.table.offset as int+3*(im.fh.table.entries as int)),hdr.width as u32,hdr.height as u32,mr.idx,0)==Ok(after))
}

fn frame_metadata(im:parse::ImageStep,e4:bool)->(m:FrameMeta)
    requires im.fh.rect.left<=u16::MAX,im.fh.rect.top<=u16::MAX,im.fh.rect.width<=u16::MAX,im.fh.rect.height<=u16::MAX,
    ensures meta_matches(m,expected_metadata(im,Seq::empty(),e4)),
{
    let (disposal,delay_cs,user_input,transparent)=match im.gce {
        Some(g)=>(g.disposal,g.delay_cs,g.user_input,g.transparent),None=>(0,0,false,None),
    };
    FrameMeta {rect:(im.fh.rect.left as u16,im.fh.rect.top as u16,im.fh.rect.width as u16,im.fh.rect.height as u16),
        disposal,delay_cs,user_input,transparent,interlaced:im.fh.interlaced,recovered:!im.fh.inside || e4}
}

spec fn prepared_matches(a:Frames,p:Frames,rect:Rect,disposal:u8)->bool {
    &&& p.canvas@==crate::frame_model::disposed(a.canvas@,saved_view(a),previous_view(a),a.hdr.width as u32,a.hdr.height as u32)
    &&& saved_view(p)==if disposal==3 && !a.still {
        Some(crate::mc::copy_rect(match saved_view(a) {Some(v)=>v,None=>Seq::new(a.canvas@.len(),|_i:int|0u8)},p.canvas@,
            a.hdr.width as u32,a.hdr.height as u32,rect.spec_rect()))
    } else {saved_view(a)}
    &&& p.log@.view()==if disposal==3 && !a.still && a.saved is None {
        a.log@.view().push((AllocSite::Saved,a.canvas@.len(),true))
    } else {a.log@.view()}
}

spec fn expected_metadata(im:parse::ImageStep,cv:Seq<u8>,e4:bool)->crate::frame_model::FrameRec {
    crate::frame_model::FrameRec {rgba:cv,rect:(im.fh.rect.left as int,im.fh.rect.top as int,im.fh.rect.width as int,im.fh.rect.height as int),
        delay_cs:match im.gce {Some(g)=>g.delay_cs,None=>0},disposal:crate::frame_model::raw_disposal(im.gce),
        user_input:match im.gce {Some(g)=>g.user_input,None=>false},
        transparent:match im.gce {Some(g)=>g.transparent,None=>None},interlaced:im.fh.interlaced,recovered:!im.fh.inside || e4}
}

#[verifier::spinoff_prover]
proof fn lemma_image_finish(a:Frames,im:parse::ImageStep,p:Frames,d:Frames,rep:crate::lzw::LzwReport,r:Result<FrameMeta,Error>)
    requires image_ready(a,im),prepared_matches(a,p,im.fh.rect,crate::frame_model::raw_disposal(im.gce)),
        d.p==a.p,walk_state_same(a,d),saved_view(d)==saved_view(p),d.log@.view()==p.log@.view(),
        pixels_observation(a.b@,a.hdr,a.pol,im,p.canvas@,d.canvas@,rep),
        match r {
            Err(e)=>crate::lzw::projected_error(rep)==Some(e) && d.state==State::Fused && d.prev==a.prev,
            Ok(m)=>crate::lzw::projected_error(rep) is None && d.state==State::Running
                && meta_matches(m,expected_metadata(im,d.canvas@,rep.e4))
                && previous_view(d)==Some(crate::frame_model::Previous {rect:im.fh.rect.spec_rect(),
                    disposal:crate::frame_model::frame(crate::parse::frame_header_model(im.fh),im.gce).disposal}),
        },
    ensures image_result(a,im,d,match r {Ok(m)=>Ok(m),Err(e)=>Err(TryError::Format(e))}),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
    hide(crate::lzw::read_d);hide(crate::frame_model::image_mode);hide(crate::parse::image_data_post);
    crate::frame_model::lemma_image_observation(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),crate::parse::frame_header_model(im.fh),
        im.gce,im.data.min_code_size as int,crate::lzw::read_d(a.b@,im.data.first as int),crate::parse::walk_m_at(a.st,im.data.end),a.still,rep,d.canvas@);
}

proof fn lemma_image_oom(a:Frames,im:parse::ImageStep,d:Frames,e:TryError)
    requires crate::frame_model::raw_disposal(im.gce)==3,!a.still,a.saved is None,d.saved is None,d.state==State::Fused,d.prev==a.prev,
        d.log@.view()==a.log@.view().push((AllocSite::Saved,a.canvas@.len(),false)),
        match e {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as int==a.canvas@.len(),_=>false},
    ensures image_result(a,im,d,Err(e)),
{}

pub closed spec fn fixed_stream(a:Frames,d:Frames)->bool {
    d.b==a.b && d.hdr==a.hdr && d.pol==a.pol && d.info==a.info && d.still==a.still
}

spec fn view_metadata(v:FrameView)->FrameMeta {
    FrameMeta {rect:v.rect,delay_cs:v.delay_cs,disposal:v.disposal,user_input:v.user_input,transparent:v.transparent,
        interlaced:v.interlaced,recovered:v.recovered}
}

/// One-call observation against the independent model, including a view of the
/// decoder's own canvas. Successive-call WF/MF induction is a separate theorem.
pub closed spec fn next_result(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>)->bool {
    if a.state!=State::Running {r is None && d==a} else {
        match r {
            None=>d.state==State::Done && step_result(a,d,Ok(None)),
            Some(Err(e))=>d.state==State::Fused && step_result(a,d,Err(e)),
            Some(Ok(v))=>d.state==State::Running && v.rgba@==d.canvas@ && step_result(a,d,Ok(Some(view_metadata(v)))),
        }
    }
}

pub closed spec fn still_result(b:Seq<u8>,pol:Policy,r:Result<(Still,bool),TryError>)->bool {
    match r {
        Err(TryError::OutOfMemory(_))=>crate::model::mh(b) is Ok,
        Err(TryError::Format(e))=>crate::frame_model::ms(b,pol)==Some(Err(e)),
        Ok((s,recovered))=>crate::frame_model::ms(b,pol)==Some(Ok(crate::frame_model::StillImage {
            width:s.width as int,height:s.height as int,rgba:s.rgba@,recovered})),
    }
}
pub closed spec fn still_core_result(b:Seq<u8>,r:Result<Still,TryError>)->bool {
    let pol=crate::policy::profile_record(Profile::Strict);
    match r {
        Err(TryError::OutOfMemory(_))=>crate::model::mh(b) is Ok,
        Err(TryError::Format(e))=>crate::frame_model::ms(b,pol)==Some(Err(e)),
        Ok(s)=>match crate::frame_model::ms(b,pol) {
            Some(Ok(m))=>m.width==s.width as int && m.height==s.height as int && m.rgba==s.rgba@,
            _=>false,
        },
    }
}

pub closed spec fn constructor_g(b:Seq<u8>,pol:Policy,r:Result<Frames,TryError>,l:Seq<(AllocSite,nat,bool)>)->bool {
    &&& constructor_result(b,pol,false,r)
    &&& crate::alloc_guard::sites::log_tied(r,l)
    &&& requests_bounded(l) && l.len()<=2
    &&& g_new_clause2(b,r is Err && r->Err_0 is OutOfMemory,l) && g_new_clause3(b)
    &&& crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b))
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b))
    &&& (crate::alloc_guard::all_ok(l) ==> !(r is Err && r->Err_0 is OutOfMemory))
    &&& match r {Ok(d)=>info_result(d,d.info) && iterator_log(d,-1),Err(_)=>true}
}
#[verifier::spinoff_prover]
proof fn lemma_constructor_package(b:Seq<u8>,pol:Policy,r:Result<Frames,TryError>,l:Seq<(AllocSite,nat,bool)>)
    requires constructor_result(b,pol,false,r),crate::alloc_guard::sites::log_tied(r,l),
        crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b)),
        !(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b),
    ensures constructor_g(b,pol,r,l),
{
    hide(crate::model::mp);reveal(constructor_g);
    lemma_new_prefix_sites(b,l);
    if r is Ok {
        reveal(constructor_result);reveal(constructor_ok);
        lemma_info_result(r.unwrap());lemma_iterator_log(r.unwrap(),-1);
    }
}

pub closed spec fn next_allocation(a:Frames,d:Frames,n:int,r:Option<Result<FrameView,TryError>>)->bool {
    let oom=r is Some && r.unwrap() is Err && r.unwrap()->Err_0 is OutOfMemory;
    let size=(4*(a.hdr.width as int)*(a.hdr.height as int)) as nat;
    let at=crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,(n+1) as nat);
    &&& iterator_log(d,n+1)
    &&& if crate::alloc_guard::all_ok(a.log@.view()) {
        if oom {
            &&& at && d.state==State::Fused
            &&& d.log@.view()==a.log@.view().push((AllocSite::Saved,size,false))
            &&& match r.unwrap()->Err_0 {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as nat==size,_=>false}
        } else {
            d.log@.view()==a.log@.view()+if at {seq![(AllocSite::Saved,size,true)]} else {Seq::empty()}
        }
    } else {r is None && d.log@.view()==a.log@.view()}
    &&& (!oom ==> crate::alloc_guard::req_of_log(d.log@.view())==crate::alloc_guard::req_of_log(a.log@.view())+req_next(a,(n+1) as nat))
    &&& (crate::alloc_guard::all_ok(d.log@.view()) ==> crate::alloc_guard::all_ok(a.log@.view()) && !oom && iterator_observation(a,n,r))
}

#[verifier::spinoff_prover]
proof fn lemma_next_allocation(a:Frames,d:Frames,n:int,r:Option<Result<FrameView,TryError>>)
    requires wf_p(a,n),wf_p(d,n+1),fixed_stream(a,d),next_result(a,d,r),
    ensures next_allocation(a,d,n,r),
{
    hide(crate::frame_model::after_calls);hide(crate::frame_model::call);hide(crate::frame_model::call_mode);
    hide(crate::model::mp);hide(crate::model::mh);hide(crate::model::mb);hide(crate::frame_model::image_mode);
    hide(storage_wf);
    lemma_iterator_log(d,n+1);lemma_storage_bounds(a);lemma_next_observation(a,d,r,n);
    reveal(wf_p);reveal(next_result);reveal(fixed_stream);reveal(next_allocation);reveal(req_next);
    let k=(n+1) as nat;
    if crate::alloc_guard::all_ok(a.log@.view()) {
        let s=crate::frame_model::after_calls(a.b@,crate::parse::header_model(a.hdr),a.pol,k).unwrap();
        if a.state==State::Running {
            lemma_wf_running(a,n);
            let step=match r {None=>Ok(None),Some(Err(e))=>Err(e),Some(Ok(v))=>Ok(Some(view_metadata(v)))};
            lemma_step_call_defined(a,d,step);
            let c=crate::frame_model::call(a.b@,crate::parse::header_model(a.hdr),a.pol,s).unwrap();
            crate::frame_model::lemma_saved_current(a.b@,crate::parse::header_model(a.hdr),a.pol,k,s,c);
            assert(crate::frame_model::call_mode(a.b@,crate::parse::header_model(a.hdr),a.pol,s,a.still)==Some(c)) by {reveal(crate::frame_model::call);}
        } else {
            crate::frame_model::lemma_terminal_call(a.b@,crate::parse::header_model(a.hdr),a.pol,s);
            assert(!crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k));
        }
    } else {
        let i=choose|i:nat| i<=n && crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,i)
            && match (crate::frame_model::after_calls(a.b@,crate::parse::header_model(a.hdr),a.pol,i),crate::frame_model::call_at(a.b@,crate::parse::header_model(a.hdr),a.pol,i)) {
                (Some(s),Some(c))=>prev_matches(a,s.prev) && crate::parse::st_matches(a.p,a.st,c.after.walk),_=>false,
            };
        if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k) {
            crate::frame_model::lemma_saved_unique(a.b@,crate::parse::header_model(a.hdr),a.pol,i,k);
        }
    }
    let oom=r is Some && r.unwrap() is Err && r.unwrap()->Err_0 is OutOfMemory;
    if oom {
        assert(!crate::alloc_guard::all_ok(d.log@.view())) by {
            let j=a.log@.view().len() as int;
            assert(d.log@.view()[j].2==false);
        }
    } else {
        let at=crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k);
        assert(d.log@.view() =~= a.log@.view()+if at {seq![(AllocSite::Saved,a.canvas@.len(),true)]} else {Seq::empty()});
        assert(crate::alloc_guard::req_of_log(d.log@.view()) =~= crate::alloc_guard::req_of_log(a.log@.view())+req_next(a,k));
    }
    if crate::alloc_guard::all_ok(d.log@.view()) {
        assert forall|j:int| 0<=j<a.log@.view().len() implies a.log@.view()[j].2 by {
            assert(d.log@.view()[j]==a.log@.view()[j]);
        }
    }
}

pub closed spec fn saved_before(d:Frames,n:int)->bool {
    exists|i:nat| i<=n && crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i)
}
pub closed spec fn req_next(d:Frames,k:nat)->Seq<(AllocSite,nat)> {
    if crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,k) {
        seq![(AllocSite::Saved,(4*(d.hdr.width as int)*(d.hdr.height as int)) as nat)]
    } else {Seq::empty()}
}
pub closed spec fn iterator_log(d:Frames,n:int)->bool {
    let l=d.log@.view();
    let size=(4*(d.hdr.width as int)*(d.hdr.height as int)) as nat;
    &&& requests_bounded(l) && l.len()<=3
    &&& if crate::alloc_guard::all_ok(l) {
        crate::alloc_guard::req_of_log(l)==req_new(d.b@)+if saved_before(d,n) {seq![(AllocSite::Saved,size)]} else {Seq::empty()}
    } else {
        &&& saved_before(d,n) && d.state==State::Fused && d.saved is None
        &&& l==seq![(AllocSite::Canvas,size,true),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat,true),(AllocSite::Saved,size,false)]
    }
}

#[verifier::spinoff_prover]
proof fn lemma_storage_sites(d:Frames)
    requires storage_wf(d),
    ensures requests_bounded(d.log@.view()),d.log@.view().len()<=3,
        req_new(d.b@)==seq![(AllocSite::Canvas,d.canvas@.len()),(AllocSite::LzwTable,crate::lzw::LZW_TABLE_BYTES as nat)],
{
    hide(crate::model::mp);hide(crate::model::mh);hide(crate::parse::hdr_ok);hide(crate::parse::hdr_inv);
    lemma_model_header(d);lemma_new_sites(d.b@);reveal(storage_wf);
    assert(site_bounded(AllocSite::Canvas,d.canvas@.len()));
    assert(site_bounded(AllocSite::Saved,d.canvas@.len()));
}

#[verifier::spinoff_prover]
proof fn lemma_iterator_log(d:Frames,n:int)
    requires wf_p(d,n),
    ensures iterator_log(d,n),
{
    hide(crate::frame_model::after_calls);hide(crate::frame_model::call);hide(crate::model::mp);hide(crate::model::mh);
    hide(crate::parse::hdr_ok);hide(crate::parse::hdr_inv);
    reveal(wf_p);reveal(storage_wf);reveal(iterator_log);reveal(saved_before);
    lemma_storage_sites(d);
    if crate::alloc_guard::all_ok(d.log@.view()) {
        let s=crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat).unwrap();
        assert((s.saved is Some)==(d.saved is Some));
        if s.saved is Some {
            crate::frame_model::lemma_saved_exists(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat,s);
            assert(saved_before(d,n));
            assert(crate::alloc_guard::req_of_log(d.log@.view()) =~= req_new(d.b@)+seq![(AllocSite::Saved,d.canvas@.len())]);
        } else {
            crate::frame_model::lemma_no_saved_before(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat,s);
            assert(!saved_before(d,n)) by {
                if saved_before(d,n) {
                    let i=choose|i:nat| i<=n && crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i);
                    assert(!crate::frame_model::call_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i).unwrap().reserve_saved);
                }
            }
            assert(crate::alloc_guard::req_of_log(d.log@.view()) =~= req_new(d.b@)+Seq::empty());
        }
    } else {assert(saved_before(d,n));}
}

pub open spec fn site_bounded(site:AllocSite,n:nat)->bool {
    1<=n && n<=crate::alloc_guard::MAX_BYTES && match site {
        AllocSite::Canvas|AllocSite::Saved=>n<=crate::limits::MAX_OUTPUT,
        AllocSite::LzwTable=>n==crate::lzw::LZW_TABLE_BYTES,
    }
}
pub open spec fn requests_bounded(l:Seq<(AllocSite,nat,bool)>)->bool {
    forall|j:int| 0<=j<l.len() ==> site_bounded(l[j].0,l[j].1)
}
proof fn lemma_new_sites(b:Seq<u8>)
    ensures req_new(b).len()<=2,g_new_clause3(b),
        forall|j:int| 0<=j<req_new(b).len() ==> site_bounded(req_new(b)[j].0,req_new(b)[j].1),
{
    hide(crate::model::mh);
    if crate::model::mh(b) is Ok {
        let h=crate::model::mh(b).unwrap();
        crate::model::lemma_mh_ok_bounds(b,h);
        assert(4*h.width*h.height>=4) by(nonlinear_arith) requires h.width>=1,h.height>=1;
        crate::limits::caps_ok();
        assert(1<=crate::lzw::LZW_TABLE_BYTES<=crate::alloc_guard::MAX_BYTES) by(compute);
    }
}
proof fn lemma_new_prefix_sites(b:Seq<u8>,l:Seq<(AllocSite,nat,bool)>)
    requires crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b)),
    ensures requests_bounded(l),l.len()<=2,g_new_clause3(b),
{
    lemma_new_sites(b);
    assert forall|j:int| 0<=j<l.len() implies site_bounded(l[j].0,l[j].1) by {
        assert(crate::alloc_guard::req_of_log(l)[j]==req_new(b)[j]);
    }
}
proof fn lemma_still_package(b:Seq<u8>,pol:Policy,r:Result<(Still,bool),TryError>,l:Seq<(AllocSite,nat,bool)>)
    requires still_result(b,pol,r),crate::alloc_guard::sites::log_tied(r,l),
        crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b)),
        !(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b),
    ensures still_g(b,pol,r,l),
{
    reveal(still_g);lemma_new_prefix_sites(b,l);
}

/// A-GIF-1's complete still observation: semantic result, sealed failure,
/// exact request sequence/prefix, site bounds and all-success progress.
pub closed spec fn still_g(b:Seq<u8>,pol:Policy,r:Result<(Still,bool),TryError>,l:Seq<(AllocSite,nat,bool)>)->bool {
    &&& still_result(b,pol,r)
    &&& requests_bounded(l) && l.len()<=2
    &&& crate::alloc_guard::sites::log_tied(r,l)
    &&& g_new_clause2(b,r is Err && r->Err_0 is OutOfMemory,l) && g_new_clause3(b)
    &&& crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b))
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b))
    &&& (crate::alloc_guard::all_ok(l) ==> !(r is Err && r->Err_0 is OutOfMemory))
}
pub closed spec fn still_core_g(b:Seq<u8>,r:Result<Still,TryError>,l:Seq<(AllocSite,nat,bool)>)->bool {
    &&& still_core_result(b,r)
    &&& requests_bounded(l) && l.len()<=2
    &&& crate::alloc_guard::sites::log_tied(r,l)
    &&& g_new_clause2(b,r is Err && r->Err_0 is OutOfMemory,l) && g_new_clause3(b)
    &&& crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b))
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b))
    &&& (crate::alloc_guard::all_ok(l) ==> !(r is Err && r->Err_0 is OutOfMemory))
}

#[verifier::spinoff_prover]
proof fn lemma_still_trace(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)
    requires constructor_ok(a,a.b@,a.pol,true),a.state==State::Running,step_result(a,d,r),
    ensures !(r is Err && r->Err_0 is OutOfMemory),d.log@.view()==a.log@.view(),
        crate::alloc_guard::all_ok(d.log@.view()),crate::alloc_guard::req_of_log(d.log@.view())==req_new(a.b@),
{
    hide(crate::model::mp);hide(crate::frame_model::call_mode);
    reveal(constructor_ok);
    crate::frame_model::lemma_still_no_reserve(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a));
}

proof fn lemma_still_strip(b:Seq<u8>,r:Result<(Still,bool),TryError>,l:Seq<(AllocSite,nat,bool)>)
    requires still_g(b,crate::policy::profile_record(Profile::Strict),r,l),
    ensures still_core_g(b,match r {Ok((s,_))=>Ok(s),Err(e)=>Err(e)},l),
{
    reveal(still_g);reveal(still_core_g);reveal(still_result);reveal(still_core_result);
}

proof fn lemma_still_constructor_error(b:Seq<u8>,pol:Policy,e:TryError)
    requires constructor_result(b,pol,true,Err(e)),
    ensures still_result(b,pol,Err(e)),
{
    hide(crate::model::mp);reveal(constructor_result);reveal(still_result);
}

#[verifier::spinoff_prover]
proof fn lemma_still_initial(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,true),
    ensures crate::frame_model::initial(d.b@,d.pol)==Some(model_state(d)),
        crate::model::mh(d.b@)==Ok(crate::parse::header_model(d.hdr)),
        crate::model::mp(d.b@,d.pol) is Ok,
        crate::model::mp(d.b@,d.pol).unwrap().0.width==d.hdr.width as int,
        crate::model::mp(d.b@,d.pol).unwrap().0.height==d.hdr.height as int,
{
    hide(crate::model::mp_fuel);
    reveal(constructor_ok);reveal(storage_wf);reveal(crate::parse::hdr_ok);
    let h=choose|h:crate::model::HeaderM| crate::model::mh(d.b@)==Ok(h) && crate::parse::hdr_matches(d.b@,h,d.hdr);
    crate::parse::lemma_header_model_matches(d.b@,d.hdr,h);
}

#[verifier::spinoff_prover]
proof fn lemma_still_empty(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,true),d.state!=State::Running,
    ensures still_result(d.b@,d.pol,Err(TryError::Format(Error::NoImage))),
{
    hide(crate::model::mp);
    lemma_still_initial(d);
    reveal(constructor_ok);reveal(still_result);
}

#[verifier::spinoff_prover]
proof fn lemma_still_step(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)
    requires constructor_ok(a,a.b@,a.pol,true),a.state==State::Running,step_result(a,d,r),d.hdr==a.hdr,
    ensures still_result(a.b@,a.pol,match r {
        Err(e)=>Err(e),Ok(None)=>Err(TryError::Format(Error::NoImage)),
        Ok(Some(m))=>Ok((Still {width:d.hdr.width,height:d.hdr.height,rgba:d.canvas},m.recovered)),
    }),
{
    hide(crate::model::mp);hide(crate::frame_model::call_mode);
    lemma_still_initial(a);
    reveal(constructor_ok);reveal(storage_wf);reveal(still_result);
}

spec fn view_matches(v:FrameView,f:crate::frame_model::FrameRec)->bool {
    v.rgba@==f.rgba && meta_matches(view_metadata(v),f)
}

/// T-ITER's observable result at a zero-based call index, with only Saved OOM
/// allowed to displace the pure stream result.
pub closed spec fn iter_result(a:Frames,k:nat,r:Option<Result<FrameView,TryError>>)->bool {
    match crate::frame_model::mf(a.b@,a.pol) {
        Err(_)=>false,
        Ok((_,st))=>if r is Some && r.unwrap() is Err && r.unwrap()->Err_0 is OutOfMemory {
            &&& crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k)
            &&& match r.unwrap()->Err_0 {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as int==4*(a.hdr.width as int)*(a.hdr.height as int),_=>false}
        } else if k<st.frames.len() {
            match r {Some(Ok(v))=>view_matches(v,st.frames[k as int]),_=>false}
        } else if k==st.frames.len() {
            match st.end {crate::frame_model::End::Trailer=>r is None,crate::frame_model::End::Error(e)=>r==Some(Err(TryError::Format(e))),_=>false}
        } else {r is None},
    }
}

#[verifier::spinoff_prover]
proof fn lemma_model_header(d:Frames)
    requires storage_wf(d),
    ensures crate::model::mp(d.b@,d.pol) is Ok,crate::model::mh(d.b@)==Ok(crate::parse::header_model(d.hdr)),
        crate::model::mh_bounds(d.b@,crate::parse::header_model(d.hdr)),
{
    hide(crate::model::mp);reveal(storage_wf);reveal(crate::parse::hdr_ok);
    let h=choose|h:crate::model::HeaderM| crate::model::mh(d.b@)==Ok(h) && crate::parse::hdr_matches(d.b@,h,d.hdr);
    crate::parse::lemma_header_model_matches(d.b@,d.hdr,h);
    crate::model::lemma_mh_ok_bounds(d.b@,h);
}

#[verifier::spinoff_prover]
proof fn lemma_next_iter(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>,n:int)
    requires wf_p(a,n),crate::alloc_guard::all_ok(a.log@.view()),next_result(a,d,r),
    ensures iter_result(a,(n+1) as nat,r),
{
    hide(crate::frame_model::mf);hide(crate::frame_model::call);hide(crate::frame_model::call_mode);hide(crate::frame_model::after_calls);
    hide(crate::model::mp);hide(crate::model::mh);hide(storage_wf);hide(crate::model::mb);hide(crate::frame_model::image_mode);
    reveal(wf_p);reveal(next_result);reveal(iter_result);
    lemma_model_header(a);
    let k=(n+1) as nat;
    crate::frame_model::lemma_mf_position(a.b@,a.pol,k);
    let s=crate::frame_model::after_calls(a.b@,crate::parse::header_model(a.hdr),a.pol,k).unwrap();
    let c=crate::frame_model::call_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k).unwrap();
    if a.state==State::Running {
        lemma_wf_running(a,n);
        assert(s==model_state(a));
        if r is Some && r.unwrap() is Err && r.unwrap()->Err_0 is OutOfMemory {
            let e=r.unwrap()->Err_0;
            lemma_step_call_defined(a,d,Err(e));
            crate::frame_model::lemma_first_saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k,s,c);
            lemma_storage_bounds(a);
        } else {
            assert(crate::frame_model::call_mode(a.b@,crate::parse::header_model(a.hdr),a.pol,s,a.still)==Some(c)) by {reveal(crate::frame_model::call);}
        }
    } else {
        crate::frame_model::lemma_terminal_call(a.b@,crate::parse::header_model(a.hdr),a.pol,s);
    }
}

pub closed spec fn iterator_observation(a:Frames,n:int,r:Option<Result<FrameView,TryError>>)->bool {
    if crate::alloc_guard::all_ok(a.log@.view()) {iter_result(a,(n+1) as nat,r)} else {r is None}
}

#[verifier::spinoff_prover]
proof fn lemma_next_observation(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>,n:int)
    requires wf_p(a,n),next_result(a,d,r),
    ensures iterator_observation(a,n,r),
{
    if crate::alloc_guard::all_ok(a.log@.view()) {lemma_next_iter(a,d,r,n);}
    else {reveal(wf_p);reveal(next_result);}
}

pub closed spec fn strict_constructor_agrees(b:Seq<u8>,a:Result<Frames,TryError>,c:Result<Frames,TryError>)->bool {
    match (a,c) {
        (Ok(d),Ok(e))=>d.b@==b && e.b@==b && d.pol==crate::policy::profile_record(Profile::Strict)
            && e.pol==d.pol && d.info==e.info && wf_p(d,-1) && wf_p(e,-1),
        (Err(TryError::Format(x)),Err(TryError::Format(y)))=>x==y,
        _=>false,
    }
}
pub closed spec fn same_strict_input(a:Frames,c:Frames)->bool {
    a.b@==c.b@ && a.pol==crate::policy::profile_record(Profile::Strict) && c.pol==a.pol
}

/// P-BRIDGE's constructor observation for two independently allocated calls.
/// Neither decoder's private canvas or reservation identity is compared.
pub proof fn theorem_strict_constructor_calls(b:Seq<u8>,a:Result<Frames,TryError>,c:Result<Frames,TryError>)
    requires constructor_result(b,crate::policy::profile_record(Profile::Strict),false,a),
        constructor_result(b,crate::policy::profile_record(Profile::Strict),false,c),
        !(a is Err && a->Err_0 is OutOfMemory),!(c is Err && c->Err_0 is OutOfMemory),
    ensures strict_constructor_agrees(b,a,c),
{
    hide(crate::model::mp);hide(crate::model::mh);
    reveal(constructor_result);reveal(constructor_ok);
}

pub closed spec fn conservative_pair_input(a:Frames,c:Frames)->bool {
    a.b@==c.b@ && a.pol==crate::policy::profile_record(Profile::Strict) && crate::conservative::conservative(c.pol)
}
pub closed spec fn conservative_construction_agrees(a:Result<Frames,TryError>,c:Result<Frames,TryError>)->bool {
    match (a,c) {(Ok(d),Ok(e))=>d.info==e.info && conservative_pair_input(d,e) && wf_p(d,-1) && wf_p(e,-1),_=>false}
}
pub proof fn theorem_conservative_constructor_calls(b:Seq<u8>,p:Policy,a:Result<Frames,TryError>,c:Result<Frames,TryError>)
    requires crate::conservative::conservative(p),
        constructor_result(b,crate::policy::profile_record(Profile::Strict),false,a),a is Ok,
        constructor_result(b,p,false,c),!(c is Err && c->Err_0 is OutOfMemory),
    ensures conservative_construction_agrees(a,c),
{
    hide(crate::model::mp);hide(crate::model::mh);
    reveal(constructor_result);reveal(constructor_ok);
    crate::policy::lemma_profile_records();
    crate::conservative::lemma_initial_simulation(b,p);
}

pub open spec fn same_frame_observation(a:Option<Result<FrameView,TryError>>,c:Option<Result<FrameView,TryError>>)->bool {
    match (a,c) {
        (None,None)=>true,
        (Some(Err(TryError::Format(e))),Some(Err(TryError::Format(f))))=>e==f,
        (Some(Ok(v)),Some(Ok(w)))=>v.rgba@==w.rgba@ && v.rect==w.rect && v.delay_cs==w.delay_cs
            && v.disposal==w.disposal && v.user_input==w.user_input && v.transparent==w.transparent
            && v.interlaced==w.interlaced && !v.recovered && !w.recovered,
        _=>false,
    }
}

/// Two Strict iterators at the same model call index expose identical frames,
/// including every metadata field. The real next_frame contract supplies
/// iter_result while all previous requests succeeded (iterator_observation).
pub proof fn theorem_strict_iterator_calls(a:Frames,c:Frames,k:nat,
    r:Option<Result<FrameView,TryError>>,s:Option<Result<FrameView,TryError>>)
    requires same_strict_input(a,c),
        iter_result(a,k,r),iter_result(c,k,s),
        !(r is Some && r.unwrap() is Err && r.unwrap()->Err_0 is OutOfMemory),
        !(s is Some && s.unwrap() is Err && s.unwrap()->Err_0 is OutOfMemory),
    ensures same_frame_observation(r,s),
{
    hide(crate::frame_model::mf);
    reveal(iter_result);
    crate::policy::lemma_profile_records();
    crate::policy_proofs::theorem_strict_no_recovery(a.b@);
}

/// R-prefix on two actual iterator-result contracts. Only Strict's delivered
/// frames are compared; the conservative iterator may continue after Strict's
/// eventual format error. Prior allocations succeeded when iter_result holds.
pub proof fn theorem_conservative_iterator_frame(a:Frames,c:Frames,k:nat,
    r:Option<Result<FrameView,TryError>>,s:Option<Result<FrameView,TryError>>)
    requires conservative_pair_input(a,c),iter_result(a,k,r),iter_result(c,k,s),r is Some,r.unwrap() is Ok,
        !(s is Some && s.unwrap() is Err && s.unwrap()->Err_0 is OutOfMemory),
    ensures s is Some,s.unwrap() is Ok,same_frame_observation(r,s),
{
    hide(crate::frame_model::mf);
    reveal(iter_result);
    crate::policy::lemma_profile_records();
    let strict=crate::frame_model::mf(a.b@,a.pol).unwrap();
    crate::conservative::theorem_r_prefix(a.b@,c.pol,strict.0,strict.1);
}

pub closed spec fn strict_stream_accepted(a:Frames)->bool {
    match crate::frame_model::mf(a.b@,a.pol) {Ok((_,s))=>s.end==crate::frame_model::End::Trailer,_=>false}
}

/// R-stream also fixes the terminating call when Strict reaches a Trailer.
pub proof fn theorem_conservative_iterator_stream(a:Frames,c:Frames,k:nat,
    r:Option<Result<FrameView,TryError>>,s:Option<Result<FrameView,TryError>>)
    requires conservative_pair_input(a,c),iter_result(a,k,r),iter_result(c,k,s),
        strict_stream_accepted(a),
        !(r is Some && r.unwrap() is Err && r.unwrap()->Err_0 is OutOfMemory),
        !(s is Some && s.unwrap() is Err && s.unwrap()->Err_0 is OutOfMemory),
    ensures same_frame_observation(r,s),
{
    hide(crate::frame_model::mf);
    reveal(iter_result);
    crate::policy::lemma_profile_records();
    let strict=crate::frame_model::mf(a.b@,a.pol).unwrap();
    crate::conservative::theorem_r_stream(a.b@,c.pol,strict.0,strict.1);
}

/// P-E for an actual still result. Event sites come from the independent
/// parser/string trace, not from the executable recovery bit.
pub closed spec fn P_E_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)->bool {
    match r {Ok((_,recovered))=>{
        let ev=crate::policy_model::still_events(b,p);
        &&& recovered==(ev!=Set::<crate::policy_model::Event>::empty())
        &&& ev.contains(crate::policy_model::Event::E1)==crate::policy_model::still_clip_site(b,p)
        &&& ev.contains(crate::policy_model::Event::E4)==crate::policy_model::still_overrun_site(b,p)
        &&& !ev.contains(crate::policy_model::Event::E2) && !ev.contains(crate::policy_model::Event::E3)
    },_=>true}
}
pub proof fn theorem_p_e_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)
    requires still_result(b,p,r),ensures P_E_still(b,p,r),
{
    hide(crate::frame_model::ms);hide(crate::policy_model::ms);
    lemma_still_policy_observation(b,p,r);
    crate::policy_model::theorem_ms_bridge(b,p);
    crate::policy_model::theorem_ms_event_sites(b,p);
}

/// P-E for a view returned at model call k. E1 uses the actual view's rectangle
/// and screen Info; E4 uses the independent raster-completing string trace.
pub closed spec fn P_E_view(a:Frames,k:nat,r:Option<Result<FrameView,TryError>>)->bool {
    match r {Some(Ok(v))=>match crate::policy_model::mf(a.b@,a.pol) {
        Err(_)=>false,Ok((info,s))=>k<s.frames.len() && k<s.events.len() && k<crate::policy_model::overruns(a.b@,a.pol).len() && {
            let ev=s.events[k as int];
            &&& v.recovered==(ev!=Set::<crate::policy_model::Event>::empty())
            &&& ev.contains(crate::policy_model::Event::E1)==(a.pol.frame_bounds==crate::policy::FrameBounds::Clip
                && !((v.rect.0 as int)+(v.rect.2 as int)<=info.width && (v.rect.1 as int)+(v.rect.3 as int)<=info.height))
            &&& ev.contains(crate::policy_model::Event::E4)==crate::policy_model::overruns(a.b@,a.pol)[k as int]
            &&& !ev.contains(crate::policy_model::Event::E2) && !ev.contains(crate::policy_model::Event::E3)
        },
    },_=>true}
}
pub proof fn theorem_p_e_view(a:Frames,k:nat,r:Option<Result<FrameView,TryError>>)
    requires iter_result(a,k,r),ensures P_E_view(a,k,r),
{
    hide(crate::frame_model::mf);hide(crate::policy_model::mf);
    reveal(iter_result);
    crate::policy_model::theorem_mf_bridge(a.b@,a.pol);
    crate::policy_model::theorem_mf_event_sites(a.b@,a.pol);
}

pub closed spec fn P_T9_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)->bool {
    &&& T13_still(b,p,r) && P_E_still(b,p,r)
    &&& match r {
        Ok((s,recovered))=>crate::policy_model::ms(b,p)==Some(Ok(crate::frame_model::StillImage {
            width:s.width as int,height:s.height as int,rgba:s.rgba@,recovered})),
        Err(TryError::Format(e))=>crate::policy_model::ms(b,p)==Some(Err(e)),
        Err(TryError::OutOfMemory(_))=>crate::policy_model::mh(b,p) is Ok,
    }
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> {
        &&& (crate::policy_model::ms(b,p) is Some && crate::policy_model::ms(b,p).unwrap() is Ok ==> r is Ok)
        &&& forall|e:Error| (r is Err && r->Err_0==TryError::Format(e)) <==> crate::policy_model::ms(b,p)==Some(Err(e))
    })
}
pub proof fn theorem_p_t9_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)
    requires still_result(b,p,r),ensures P_T9_still(b,p,r),
{
    hide(crate::frame_model::ms);hide(crate::policy_model::ms);
    lemma_T13_still(b,p,r);theorem_p_e_still(b,p,r);
    crate::policy_model::theorem_ms_bridge(b,p);
    lemma_still_policy_observation(b,p,r);
}
pub closed spec fn P_T9_iterator(a:Frames,k:nat,r:Option<Result<FrameView,TryError>>)->bool {
    &&& iter_result(a,k,r) && P_E_view(a,k,r)
    &&& crate::policy_model::erase_mf(crate::policy_model::mf(a.b@,a.pol))==crate::frame_model::mf(a.b@,a.pol)
}
pub proof fn theorem_p_t9_iterator(a:Frames,k:nat,r:Option<Result<FrameView,TryError>>)
    requires iter_result(a,k,r),ensures P_T9_iterator(a,k,r),
{
    hide(crate::frame_model::mf);hide(crate::policy_model::mf);
    theorem_p_e_view(a,k,r);crate::policy_model::theorem_mf_bridge(a.b@,a.pol);
}

pub closed spec fn P_T9_constructor(b:Seq<u8>,p:Policy,still:bool,r:Result<Frames,TryError>)->bool {
    &&& constructor_result(b,p,still,r)
    &&& match r {
        Ok(d)=>match crate::policy_model::mp(b,p) {Ok((i,_))=>info_model(d.info)==i,Err(_)=>false},
        Err(TryError::Format(e))=>crate::policy_model::mp(b,p)==Err(e),
        Err(TryError::OutOfMemory(_))=>crate::policy_model::mh(b,p) is Ok,
    }
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> forall|e:Error|
        (r is Err && r->Err_0==TryError::Format(e)) <==> crate::policy_model::mp(b,p)==Err(e))
}
pub proof fn theorem_p_t9_constructor(b:Seq<u8>,p:Policy,still:bool,r:Result<Frames,TryError>)
    requires constructor_result(b,p,still,r),ensures P_T9_constructor(b,p,still,r),
{
    hide(crate::model::mp);hide(crate::policy_model::mp);hide(crate::model::mh);
    reveal(constructor_result);reveal(constructor_ok);reveal(storage_wf);
    crate::policy_model::theorem_parser_bridge(b,p);
}

/// P-UNIV's allocation/observation/work package. Safety and termination are
/// checked on the generic measured executable that establishes this predicate,
/// not represented by a freestanding boolean or allocation-success premise.
pub closed spec fn P_UNIV_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>,l:Seq<(AllocSite,nat,bool)>,steps:nat)->bool {
    &&& crate::policy::implemented_record(p)
    &&& still_g(b,p,r,l) && P_T9_still(b,p,r)
    &&& (crate::model::mh(b) is Err ==> l.len()==0 && !(r is Err && r->Err_0 is OutOfMemory))
    &&& steps as int<=C_WORK as int*(b.len() as int+header_pixels(b) as int+crate::work_model::P(b,p,None,1) as int)+C_FIXED as int
}
pub closed spec fn P_UNIV_constructor(b:Seq<u8>,p:Policy,still:bool,r:Result<Frames,TryError>,l:Seq<(AllocSite,nat,bool)>,steps:nat)->bool {
    &&& crate::policy::implemented_record(p)
    &&& P_T9_constructor(b,p,still,r)
    &&& (crate::model::mh(b) is Err ==> l.len()==0 && !(r is Err && r->Err_0 is OutOfMemory))
    &&& (!still ==> constructor_g(b,p,r,l))
    &&& crate::alloc_guard::sites::log_tied(r,l) && requests_bounded(l) && l.len()<=2
    &&& g_new_clause2(b,r is Err && r->Err_0 is OutOfMemory,l) && g_new_clause3(b)
    &&& crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b))
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b))
    &&& (crate::alloc_guard::all_ok(l) ==> !(r is Err && r->Err_0 is OutOfMemory))
    &&& match r {Ok(d)=>constructor_work(d) && steps==d.work@,_=>true}
    &&& steps as int<=C_CALL as int*(crate::work_model::cur_n(b,p,None,-1)+header_pixels(b) as int)+C_FIXED as int
}
proof fn lemma_p_univ_prealloc(b:Seq<u8>,l:Seq<(AllocSite,nat,bool)>)
    requires crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b)),
    ensures crate::model::mh(b) is Err ==> l.len()==0,
{
    hide(crate::model::mh);
    assert(crate::alloc_guard::req_of_log(l).len()==l.len());
}
proof fn lemma_p_univ_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>,l:Seq<(AllocSite,nat,bool)>,steps:nat)
    requires still_g(b,p,r,l),
        steps as int<=C_WORK as int*(b.len() as int+header_pixels(b) as int+crate::work_model::P(b,p,None,1) as int)+C_FIXED as int,
    ensures P_UNIV_still(b,p,r,l,steps),
{
    reveal(still_g);crate::policy::theorem_full_record_domain(p);theorem_p_t9_still(b,p,r);
    lemma_p_univ_prealloc(b,l);lemma_still_policy_observation(b,p,r);
}

proof fn lemma_p_univ_constructor(b:Seq<u8>,p:Policy,still:bool,r:Result<Frames,TryError>,l:Seq<(AllocSite,nat,bool)>,steps:nat)
    requires constructor_result(b,p,still,r),crate::alloc_guard::sites::log_tied(r,l),
        g_new_clause2(b,r is Err && r->Err_0 is OutOfMemory,l),
        crate::alloc_guard::req_of_log(l).is_prefix_of(req_new(b)),
        !(r is Err && r->Err_0 is OutOfMemory) ==> crate::alloc_guard::req_of_log(l)==req_new(b),
        match r {Ok(d)=>constructor_work(d) && steps==d.work@,_=>true},
        steps as int<=C_CALL as int*(crate::work_model::cur_n(b,p,None,-1)+header_pixels(b) as int)+C_FIXED as int,
    ensures P_UNIV_constructor(b,p,still,r,l,steps),
{
    crate::policy::theorem_full_record_domain(p);theorem_p_t9_constructor(b,p,still,r);
    lemma_new_prefix_sites(b,l);lemma_p_univ_prealloc(b,l);
    if !still {lemma_constructor_package(b,p,r,l);}
}

pub closed spec fn P_UNIV_next(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>)->bool {
    &&& crate::policy::implemented_record(a.pol)
    &&& storage_wf(d) && next_result(a,d,r) && fixed_stream(a,d)
    &&& next_work(a,d) && next_model_work(a,d,r)
    &&& forall|k:nat| T12_next(a,d,k,r)
    &&& forall|n:int| #[trigger] wf_p(a,n) ==> {
        &&& wf_p(d,n+1) && iterator_observation(a,n,r) && next_allocation(a,d,n,r) && T13_view(d,n+1,r)
        &&& P_E_view(a,(n+1) as nat,r)
        &&& (crate::alloc_guard::all_ok(a.log@.view()) ==> P_T9_iterator(a,(n+1) as nat,r))
    }
}
proof fn lemma_p_univ_next(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>)
    requires storage_wf(d),next_result(a,d,r),fixed_stream(a,d),next_work(a,d),next_model_work(a,d,r),
        forall|k:nat| T12_next(a,d,k,r),
        forall|n:int| #[trigger] wf_p(a,n) ==> wf_p(d,n+1) && iterator_observation(a,n,r) && next_allocation(a,d,n,r) && T13_view(d,n+1,r),
    ensures P_UNIV_next(a,d,r),
{
    hide(wf_p);hide(P_T9_iterator);hide(P_E_view);
    hide(storage_wf);hide(next_result);hide(fixed_stream);hide(next_work);hide(next_model_work);
    hide(T12_next);hide(next_allocation);hide(T13_view);hide(iterator_observation);hide(iter_result);
    hide(crate::frame_model::mf);hide(crate::policy_model::mf);
    crate::policy::theorem_full_record_domain(a.pol);
    assert forall|n:int| #[trigger] wf_p(a,n) implies P_E_view(a,(n+1) as nat,r)
        && (crate::alloc_guard::all_ok(a.log@.view()) ==> P_T9_iterator(a,(n+1) as nat,r)) by {
        reveal(iterator_observation);
        if crate::alloc_guard::all_ok(a.log@.view()) {
            theorem_p_t9_iterator(a,(n+1) as nat,r);theorem_p_e_view(a,(n+1) as nat,r);
        }
        else {assert(r is None);reveal(P_E_view);}
    }
}

pub closed spec fn P_T9_info(d:Frames,i:Info)->bool {
    match crate::policy_model::mf(d.b@,d.pol) {Err(_)=>false,Ok((mi,_))=>info_model(i)==mi}
}
pub proof fn theorem_p_t9_info(d:Frames,i:Info)
    requires info_result(d,i),ensures P_T9_info(d,i),
{
    hide(crate::frame_model::mf);hide(crate::policy_model::mf);
    crate::policy_model::theorem_mf_bridge(d.b@,d.pol);
}

/// A finite history rooted in an actual successful generic constructor.
/// Input bytes and all Policy fields are unrestricted; each transition is the
/// package now guaranteed by the real next_frame method. Immutable info calls
/// can be inserted without changing snapshots, logs or measured work.
pub closed spec fn P_UNIV_history_input(b:Seq<u8>,p:Policy,ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>)->bool {
    &&& ds.len()==rs.len()+1 && cuts.len()==ds.len() && cuts[0] is None
    &&& constructor_result(b,p,false,Ok(ds[0])) && constructor_work(ds[0])
    &&& forall|i:int| 0<=i<ds.len() ==> storage_wf(#[trigger] ds[i]) && ds[i].b@==b && ds[i].b==ds[0].b && ds[i].pol==p
    &&& forall|i:int| 0<=i<rs.len() ==> #[trigger] P_UNIV_next(ds[i],ds[i+1],rs[i])
        && cuts[i+1]==next_cut(cuts[i],i as nat,rs[i])
}
pub closed spec fn P_UNIV_history_report(b:Seq<u8>,p:Policy,ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>)->bool {
    &&& crate::policy::implemented_record(p)
    &&& work_wf(ds.last(),rs.len() as int-1,cuts.last())
    &&& steps_total(ds)<=C_CALL as int*(b.len() as int+2*header_pixels(b) as int+2*crate::work_model::P(b,p,cuts.last(),rs.len()) as int)+C_FIXED as int
    &&& steps_total(ds)<=C_WORK as int*(b.len() as int+header_pixels(b) as int+crate::work_model::P(b,p,cuts.last(),rs.len()) as int)+C_FIXED as int
    &&& forall|i:int| 0<=i<ds.len() ==> wf_p(#[trigger] ds[i],i-1) && iterator_log(ds[i],i-1)
        && requests_bounded(ds[i].log@.view()) && ds[i].log@.view().len()<=3
    &&& forall|i:int| 0<=i<rs.len() ==> #[trigger] P_E_view(ds[i],i as nat,rs[i])
        && next_allocation(ds[i],ds[i+1],i-1,rs[i])
        && (crate::alloc_guard::all_ok(ds[i].log@.view()) ==> P_T9_iterator(ds[i],i as nat,rs[i]))
}
proof fn lemma_p_univ_next_at(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>,n:int)
    requires P_UNIV_next(a,d,r),wf_p(a,n),
    ensures wf_p(d,n+1),iterator_observation(a,n,r),next_allocation(a,d,n,r),T13_view(d,n+1,r),
        P_E_view(a,(n+1) as nat,r),crate::alloc_guard::all_ok(a.log@.view()) ==> P_T9_iterator(a,(n+1) as nat,r),
        forall|k:nat| T12_next(a,d,k,r),
{
    hide(wf_p);hide(iterator_observation);hide(next_allocation);hide(T13_view);hide(P_E_view);hide(P_T9_iterator);hide(T12_next);
    hide(storage_wf);hide(next_result);hide(fixed_stream);hide(next_work);hide(next_model_work);
}

proof fn lemma_p_univ_history_wf(b:Seq<u8>,p:Policy,ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>,k:nat)
    requires P_UNIV_history_input(b,p,ds,rs,cuts),k<ds.len(),
    ensures wf_p(ds[k as int],k as int-1),
    decreases k,
{
    hide(wf_p);hide(constructor_result);hide(constructor_work);hide(storage_wf);hide(P_UNIV_next);
    if k==0 {reveal(constructor_result);}
    else {
        lemma_p_univ_history_wf(b,p,ds,rs,cuts,(k-1) as nat);
        let j=k as int-1;
        assert(P_UNIV_next(ds[j],ds[j+1],rs[j]));
        lemma_p_univ_next_at(ds[j],ds[j+1],rs[j],j-1);
    }
}
proof fn lemma_p_univ_history_report(b:Seq<u8>,p:Policy,ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>)
    requires crate::policy::implemented_record(p),work_wf(ds.last(),rs.len() as int-1,cuts.last()),
        steps_total(ds)<=C_CALL as int*(b.len() as int+2*header_pixels(b) as int+2*crate::work_model::P(b,p,cuts.last(),rs.len()) as int)+C_FIXED as int,
        steps_total(ds)<=C_WORK as int*(b.len() as int+header_pixels(b) as int+crate::work_model::P(b,p,cuts.last(),rs.len()) as int)+C_FIXED as int,
        forall|i:int| 0<=i<ds.len() ==> wf_p(#[trigger] ds[i],i-1) && iterator_log(ds[i],i-1)
            && requests_bounded(ds[i].log@.view()) && ds[i].log@.view().len()<=3,
        forall|i:int| 0<=i<rs.len() ==> #[trigger] P_E_view(ds[i],i as nat,rs[i]) && next_allocation(ds[i],ds[i+1],i-1,rs[i])
            && (crate::alloc_guard::all_ok(ds[i].log@.view()) ==> P_T9_iterator(ds[i],i as nat,rs[i])),
    ensures P_UNIV_history_report(b,p,ds,rs,cuts),
{
    hide(work_wf);hide(steps_total);hide(header_pixels);hide(crate::work_model::P);hide(wf_p);hide(iterator_log);
    hide(requests_bounded);hide(P_E_view);hide(next_allocation);hide(P_T9_iterator);
}

#[verifier::spinoff_prover]
pub proof fn theorem_p_univ_history(b:Seq<u8>,p:Policy,ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>)
    requires P_UNIV_history_input(b,p,ds,rs,cuts),
    ensures P_UNIV_history_report(b,p,ds,rs,cuts),
{
    hide(wf_p);hide(iterator_log);hide(P_E_view);hide(P_T9_iterator);hide(T12_next);hide(next_allocation);
    hide(work_wf);hide(steps_total);hide(crate::work_model::P);hide(storage_wf);hide(constructor_work);
    hide(P_UNIV_next);hide(constructor_result);hide(constructor_ok);hide(requests_bounded);hide(header_pixels);
    hide(crate::frame_model::mf);hide(crate::policy_model::mf);hide(crate::model::mh);hide(crate::model::mp);
    crate::policy::theorem_full_record_domain(p);
    assert forall|i:int| 0<=i<ds.len() implies wf_p(#[trigger] ds[i],i-1) && iterator_log(ds[i],i-1)
        && requests_bounded(ds[i].log@.view()) && ds[i].log@.view().len()<=3 by {
        lemma_p_univ_history_wf(b,p,ds,rs,cuts,i as nat);
        lemma_iterator_log(ds[i],i-1);lemma_storage_sites(ds[i]);
    }
    assert forall|i:int| 0<=i<rs.len() implies T12_next(#[trigger] ds[i],ds[i+1],i as nat,rs[i])
        && cuts[i+1]==next_cut(cuts[i],i as nat,rs[i]) by {
        assert(P_UNIV_next(ds[i],ds[i+1],rs[i]));
        assert(cuts[i+1]==next_cut(cuts[i],i as nat,rs[i]));
        lemma_p_univ_history_wf(b,p,ds,rs,cuts,i as nat);
        lemma_p_univ_next_at(ds[i],ds[i+1],rs[i],i-1);
        assert(T12_next(ds[i],ds[i+1],i as nat,rs[i]));
    }
    assert forall|i:int| 0<=i<ds.len() implies storage_wf(#[trigger] ds[i]) && ds[i].b==ds[0].b && ds[i].pol==ds[0].pol by {}
    assert(!ds[0].still) by {reveal(constructor_result);reveal(constructor_ok);}
    theorem_T12_finite(ds,rs,cuts);
    assert(ds[0].b@==b && ds[0].pol==p);
    assert(ds[0].b.len() as int==b.len());
    assert(work_wf(ds.last(),rs.len() as int-1,cuts.last()));
    assert(steps_total(ds)<=C_CALL as int*(b.len() as int+2*header_pixels(b) as int+2*crate::work_model::P(b,p,cuts.last(),rs.len()) as int)+C_FIXED as int);
    assert(steps_total(ds)<=C_WORK as int*(b.len() as int+header_pixels(b) as int+crate::work_model::P(b,p,cuts.last(),rs.len()) as int)+C_FIXED as int);
    assert forall|i:int| 0<=i<rs.len() implies #[trigger] P_E_view(ds[i],i as nat,rs[i])
        && next_allocation(ds[i],ds[i+1],i-1,rs[i])
        && (crate::alloc_guard::all_ok(ds[i].log@.view()) ==> P_T9_iterator(ds[i],i as nat,rs[i])) by {
        lemma_p_univ_history_wf(b,p,ds,rs,cuts,i as nat);
        lemma_p_univ_next_at(ds[i],ds[i+1],rs[i],i-1);
    }
    lemma_p_univ_history_report(b,p,ds,rs,cuts);
}

pub closed spec fn info_result(d:Frames,i:Info)->bool {
    match crate::frame_model::mf(d.b@,d.pol) {Err(_)=>false,Ok((mi,_))=>info_model(i)==mi}
}
proof fn lemma_info_result(d:Frames)
    requires storage_wf(d),
    ensures info_result(d,d.info),
{
    hide(crate::model::mp);hide(crate::model::mh);hide(crate::frame_model::stream_fuel);
    lemma_model_header(d);reveal(storage_wf);reveal(info_result);
}

spec fn step_result(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)->bool {
    if r is Err && r->Err_0 is OutOfMemory {
        match crate::model::mb(a.b@,crate::parse::header_model(a.hdr),a.pol,crate::parse::walk_m_at(a.st,a.p)) {
            Ok(crate::model::MbOut::Image(fh,g,m,data,walk))=>{
                &&& crate::frame_model::raw_disposal(g)==3 && !a.still && a.saved is None
                &&& crate::frame_model::image_domain(a.b@,crate::parse::header_model(a.hdr),fh)
                &&& d.state==State::Fused && d.saved is None && d.prev==a.prev
                &&& crate::parse::st_matches(d.p,d.st,walk)
                &&& d.log@.view()==a.log@.view().push((AllocSite::Saved,a.canvas@.len(),false))
                &&& match r->Err_0 {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as int==a.canvas@.len(),_=>false}
            },
            _=>false,
        }
    } else {
        match crate::frame_model::call_mode(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),a.still) {
            None=>false,Some(c)=>{
                &&& semantic_state_matches(d,c.after)
                &&& d.log@.view()==if c.reserve_saved {a.log@.view().push((AllocSite::Saved,a.canvas@.len(),true))} else {a.log@.view()}
                &&& match r {
                    Ok(None)=>c.frame is None && c.error is None,
                    Ok(Some(m))=>match c.frame {Some(f)=>meta_matches(m,f) && d.canvas@==f.rgba && c.error is None,None=>false},
                    Err(TryError::Format(e))=>c.error==Some(e) && c.frame is None,
                    Err(_)=>false,
                }
            },
        }
    }
}

proof fn lemma_storage_walk(a:Frames,d:Frames)
    requires storage_wf(a),a.state==State::Running,d.b==a.b,d.hdr==a.hdr,d.pol==a.pol,d.info==a.info,d.still==a.still,
        d.canvas@==a.canvas@,d.table@==a.table@,saved_view(d)==saved_view(a),d.prev==a.prev,d.log@.view()==a.log@.view(),
        d.p<=d.b.len(),d.st.frames_seen<=crate::limits::MAX_FRAMES,d.st.total_pixels<=crate::limits::MAX_TOTAL_PIXELS,
        d.st.loop_count==a.st.loop_count,
        d.state==State::Running && d.st.frames_seen==0 ==> d.p<d.b.len() && d.b@[d.p as int]==0x2c,
    ensures storage_wf(d),
{reveal(storage_wf);reveal(prev_bounds);}

proof fn lemma_mb_image_ready(a:Frames,p:Frames,im:parse::ImageStep)
    requires crate::parse::hdr_inv(a.b@,a.hdr),p.b==a.b,p.hdr==a.hdr,p.pol==a.pol,
        crate::parse::mb_exec_post(a.b@,a.p,a.hdr,a.pol,a.st,p.p,p.st,Ok(Some(im))),
    ensures image_ready(p,im),
{
    hide(crate::model::mb);hide(crate::parse::image_data_post);
}

#[verifier::spinoff_prover]
proof fn lemma_step_terminal(a:Frames,d:Frames,parsed:Result<Option<parse::ImageStep>,Error>)
    requires a.state==State::Running,!(parsed is Ok && parsed.unwrap() is Some),
        crate::parse::mb_exec_post(a.b@,a.p,a.hdr,a.pol,a.st,d.p,d.st,parsed),
        d.canvas@==a.canvas@,saved_view(d)==saved_view(a),d.prev==a.prev,d.log@.view()==a.log@.view(),
        match parsed {Err(_)=>d.state==State::Fused,Ok(_)=>d.state==State::Done},
    ensures step_result(a,d,match parsed {Err(e)=>Err(TryError::Format(e)),Ok(_)=>Ok(None)}),
{
    hide(crate::model::mb);hide(crate::frame_model::image_mode);
}

#[verifier::spinoff_prover]
proof fn lemma_step_image(a:Frames,p:Frames,d:Frames,im:parse::ImageStep,r:Result<FrameMeta,TryError>)
    requires a.state==State::Running,image_ready(p,im),crate::parse::mb_exec_post(a.b@,a.p,a.hdr,a.pol,a.st,p.p,p.st,Ok(Some(im))),
        p.b==a.b,p.hdr==a.hdr,p.pol==a.pol,p.still==a.still,
        p.canvas@==a.canvas@,saved_view(p)==saved_view(a),p.prev==a.prev,p.log@.view()==a.log@.view(),
        image_result(p,im,d,r),d.p==p.p,walk_state_same(p,d),
    ensures step_result(a,d,match r {Ok(m)=>Ok(Some(m)),Err(e)=>Err(e)}),
{
    hide(crate::model::mb);hide(crate::frame_model::image_mode);hide(crate::lzw::read_d);
    let fh=crate::parse::frame_header_model(im.fh);
    crate::frame_model::lemma_image_state_extensional(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),model_state(p),fh,
        im.gce,im.data.min_code_size as int,crate::lzw::read_d(a.b@,im.data.first as int),crate::parse::walk_m_at(p.st,p.p),a.still);
    if !(r is Err && r->Err_0 is OutOfMemory) {
        crate::frame_model::lemma_image_reservation(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),fh,
            im.gce,im.data.min_code_size as int,crate::lzw::read_d(a.b@,im.data.first as int),crate::parse::walk_m_at(p.st,p.p),a.still);
    }
}

spec fn image_result(a:Frames,im:parse::ImageStep,d:Frames,r:Result<FrameMeta,TryError>)->bool {
    let oom=r is Err && r->Err_0 is OutOfMemory;
    &&& d.log@.view()==if crate::frame_model::raw_disposal(im.gce)==3 && !a.still && a.saved is None {
        a.log@.view().push((AllocSite::Saved,a.canvas@.len(),!oom))
    } else {a.log@.view()}
    &&& if oom {
        &&& crate::frame_model::raw_disposal(im.gce)==3 && !a.still && a.saved is None
        &&& d.state==State::Fused && d.saved is None && d.prev==a.prev
        &&& match r->Err_0 {TryError::OutOfMemory(o)=>o.site_spec()==AllocSite::Saved && o.elements_spec() as int==a.canvas@.len(),_=>false}
    } else {
        match crate::frame_model::image_mode(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),crate::parse::frame_header_model(im.fh),
            im.gce,im.data.min_code_size as int,crate::lzw::read_d(a.b@,im.data.first as int),crate::parse::walk_m_at(a.st,im.data.end),a.still) {
            None=>false,
            Some(c)=>semantic_state_matches(d,c.after) && match r {
                Ok(m)=>match c.frame {Some(f)=>c.error is None && meta_matches(m,f) && d.canvas@==f.rgba,None=>false},
                Err(TryError::Format(e))=>c.error==Some(e) && c.frame is None,
                Err(_)=>false,
            },
        }
    }
}

spec fn previous_view(d:Frames)->Option<crate::frame_model::Previous> {
    match d.prev {None=>None,Some(p)=>Some(crate::frame_model::Previous {rect:p.rect.spec_rect(),disposal:p.disposal as int})}
}
spec fn saved_view(d:Frames)->Option<Seq<u8>> {
    match d.saved {None=>None,Some(v)=>Some(v@)}
}

spec fn prev_matches(d:Frames,p:Option<crate::frame_model::Previous>)->bool {
    match (d.prev,p) {
        (None,None)=>true,
        (Some(a),Some(b))=>a.rect.spec_rect()==b.rect && a.disposal as int==b.disposal,
        _=>false,
    }
}

spec fn saved_matches(d:Frames,s:Option<Seq<u8>>)->bool {
    match (d.saved,s) {(None,None)=>true,(Some(a),Some(b))=>a@==b,_=>false}
}

/// Terminal states abstract away the private parser cursor. Error states also
/// abstract away canvas and Saved bytes: no later call exposes either buffer.
spec fn semantic_state_matches(d:Frames,s:crate::frame_model::State)->bool {
    &&& prev_matches(d,s.prev)
    &&& match s.status {
        crate::frame_model::Status::Running=>d.state==State::Running
            && crate::parse::st_matches(d.p,d.st,s.walk) && d.canvas@==s.canvas && saved_matches(d,s.saved),
        crate::frame_model::Status::Done=>d.state==State::Done && d.canvas@==s.canvas && saved_matches(d,s.saved),
        crate::frame_model::Status::Failed=>d.state==State::Fused && (d.saved is Some)==(s.saved is Some),
    }
}

/// Frozen iterator relation after n+1 calls. The normal branch relates all
/// live canvas/disposal/Saved bytes to the independent frame-call model. The
/// OOM branch records a cut at the first Saved request and permits no exposure
/// or further mutation except the call count represented by n.
pub closed spec fn wf_p(d:Frames,n:int)->bool {
    &&& -1<=n && !d.still && storage_wf(d)
    &&& if crate::alloc_guard::all_ok(d.log@.view()) {
        match crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat) {
            None=>false,Some(s)=>semantic_state_matches(d,s),
        }
    } else {
        &&& d.state==State::Fused && d.saved is None
        &&& exists|i:nat| i<=n && crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i)
            && match (crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,i),
                crate::frame_model::call_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i)) {
                (Some(s),Some(c))=>prev_matches(d,s.prev) && crate::parse::st_matches(d.p,d.st,c.after.walk),
                _=>false,
            }
    }
}

#[verifier::spinoff_prover]
proof fn lemma_wf_running(d:Frames,n:int)
    requires wf_p(d,n),d.state==State::Running,
    ensures crate::alloc_guard::all_ok(d.log@.view()),
        crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat)==Some(model_state(d)),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::parse::hdr_inv);
    hide(crate::frame_model::after_calls);
    reveal(wf_p);reveal(storage_wf);
    let s=crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat).unwrap();
    assert(previous_view(d)==s.prev);
    assert(saved_view(d)==s.saved);
    assert(model_state(d)==s);
}

#[verifier::spinoff_prover]
proof fn lemma_step_call_defined(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)
    requires a.state==State::Running,!a.still,step_result(a,d,r),
    ensures crate::frame_model::call(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a)) is Some,
        r is Err && r->Err_0 is OutOfMemory ==> ({
            let c=crate::frame_model::call(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a)).unwrap();
            c.reserve_saved && a.saved is None && d.prev==a.prev && d.saved is None && crate::parse::st_matches(d.p,d.st,c.after.walk)
        }),
{
    hide(crate::model::mb);hide(crate::frame_model::image_mode);
    if r is Err && r->Err_0 is OutOfMemory {
        match crate::model::mb(a.b@,crate::parse::header_model(a.hdr),a.pol,crate::parse::walk_m_at(a.st,a.p)) {
            Ok(crate::model::MbOut::Image(fh,g,m,data,walk))=>{
                crate::frame_model::lemma_image_defined(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),fh,g,m,data,walk,false);
                crate::frame_model::lemma_image_reservation(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a),fh,g,m,data,walk,false);
            },
            _=>{},
        }
    }
}

#[verifier::spinoff_prover]
proof fn lemma_step_wf(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>,n:int)
    requires wf_p(a,n),a.state==State::Running,storage_wf(d),fixed_stream(a,d),step_result(a,d,r),
    ensures wf_p(d,n+1),
{
    hide(crate::frame_model::call);hide(crate::frame_model::call_mode);hide(crate::frame_model::after_calls);
    hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::parse::hdr_inv);
    lemma_wf_running(a,n);
    reveal(wf_p);reveal(fixed_stream);
    lemma_step_call_defined(a,d,r);
    let k=(n+1) as nat;
    let s=model_state(a);
    let c=crate::frame_model::call(a.b@,crate::parse::header_model(a.hdr),a.pol,s).unwrap();
    if r is Err && r->Err_0 is OutOfMemory {
        crate::frame_model::lemma_first_saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k,s,c);
        assert(!crate::alloc_guard::all_ok(d.log@.view())) by {
            let j=a.log@.view().len() as int;
            assert(d.log@.view()[j].2==false);
        }
        assert(exists|i:nat| i<=n+1 && crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i)
            && match (crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,i),crate::frame_model::call_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i)) {
                (Some(s),Some(c))=>prev_matches(d,s.prev) && crate::parse::st_matches(d.p,d.st,c.after.walk),_=>false,
            }) by {
            assert(prev_matches(d,s.prev));
            assert(crate::frame_model::call_at(d.b@,crate::parse::header_model(d.hdr),d.pol,k)==Some(c));
        }
    } else {
        assert(crate::frame_model::call_mode(a.b@,crate::parse::header_model(a.hdr),a.pol,s,a.still)==Some(c)) by {reveal(crate::frame_model::call);}
        crate::frame_model::lemma_after_calls_step(a.b@,crate::parse::header_model(a.hdr),a.pol,k,s,c);
        assert(crate::alloc_guard::all_ok(d.log@.view())) by {
            assert forall|j:int| 0<=j<d.log@.view().len() implies d.log@.view()[j].2 by {
                if j<a.log@.view().len() {assert(a.log@.view()[j].2);}
            }
        }
    }
}

#[verifier::spinoff_prover]
proof fn lemma_terminal_wf(d:Frames,n:int)
    requires wf_p(d,n),d.state!=State::Running,
    ensures wf_p(d,n+1),
{
    hide(crate::frame_model::after_calls);hide(crate::frame_model::call);
    reveal(wf_p);
    if crate::alloc_guard::all_ok(d.log@.view()) {
        let s=crate::frame_model::after_calls(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat).unwrap();
        crate::frame_model::lemma_terminal_call(d.b@,crate::parse::header_model(d.hdr),d.pol,s);
        crate::frame_model::lemma_after_calls_step(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat,s,
            crate::frame_model::Call {after:s,frame:None,error:None,reserve_saved:false});
    }
}

proof fn lemma_constructor_wf(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,false),
    ensures wf_p(d,-1),
{
    hide(crate::frame_model::call);
    reveal(constructor_ok);reveal(storage_wf);reveal(wf_p);
    reveal_with_fuel(crate::frame_model::after_calls,1);
    let initial=crate::frame_model::initial(d.b@,d.pol).unwrap();
    assert(prev_matches(d,initial.prev));
    assert(saved_matches(d,initial.saved));
    assert(semantic_state_matches(d,initial));
}
} // verus!
