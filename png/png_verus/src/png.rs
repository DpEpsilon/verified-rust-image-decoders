//! Entry points (SPEC P1, P15–P17): `try_decode_still`, `try_decode_still16`,
//! their policy variants, the frame iterator `Frames` and `try_icc_profile`;
//! the closed error list (P18); `Info` (P8.3).
//!
//! Every entry point runs the structural walk to IEND first (`walk.rs`); only
//! then does it reserve, through the sealed guard, in the order of SPEC P20.
//! M3 guard helpers borrow a real sealed log owned by each call's Trace.
//! The Strict ICC entry composes its full result/log equation; complete public
//! still/frame log/decoder composition remains separate from these helpers.

use alloc::vec::Vec;

use inflate_verus::limits::WINDOW_MAX;
use inflate_verus::{inflate_zlib_into, inflate_zlib_measure, InflateError, InflateOptions, SegmentSlices, Status};

use crate::alloc_guard::sites::{try_empty, TryError};
use crate::alloc_guard::{try_zeroed, AllocLog, AllocSite, OutOfMemory};
use crate::image::{decode_image, raw_size, Sink};
use crate::limits::MAX_ICC;
use crate::policy::{Policy, Profile, STRICT};
use crate::reader::IdatReader;
use crate::frame_state::{FrameState, Step};
#[cfg(verus_only)]
use crate::raster_model as raster;
#[cfg(verus_only)]
use crate::composite_model as composite;
#[cfg(verus_only)]
use crate::paint_model as paint;
#[cfg(verus_only)]
use crate::iterator_log_model as iterlog;
#[cfg(verus_only)]
use crate::iterator_data_model as iterdata;
#[cfg(verus_only)]
use crate::iterator_canvas_model as canvas_model;
#[cfg(verus_only)]
use inflate_verus::Segments;
use crate::walk::{Doc, Ihdr};

use vstd::prelude::*;

// Native builds retain only the zero-sized carrier of the erased history.
#[cfg(verus_only)]
type CallHistory = crate::iterator_data_model::History;
#[cfg(not(verus_only))]
type CallHistory = ();
#[cfg(verus_only)]
type CallCanvas = crate::iterator_canvas_model::Trace;
#[cfg(not(verus_only))]
type CallCanvas = ();

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
    pub(crate) fn from_inflate(e: InflateError) -> (r: Error)
        ensures r==crate::icc_model::inflate_error(e),
    {
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
    pub(crate) fn any(&self) -> (r: bool)
        ensures r==(self.r1 || self.r3 || self.r4),
    {
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
        ensures r.view() == crate::document_model::initial_info(h.view()),r.ranges_wf(0),r.repr_wf(),
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

verus! {

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

/// No byte-range metadata changes while assembling the returned still.
fn assemble_still(doc: Doc,rgba: Vec<u8>) -> (r: Still)
    ensures r.width==doc.ihdr.w,r.height==doc.ihdr.h,r.info==doc.info,r.rgba==rgba,
{Still {width:doc.ihdr.w,height:doc.ihdr.h,rgba,info:doc.info}}
fn assemble_still16(doc: Doc,rgba16: Vec<u8>) -> (r: Still16)
    ensures r.width==doc.ihdr.w,r.height==doc.ihdr.h,r.info==doc.info,r.rgba16==rgba16,
{Still16 {width:doc.ihdr.w,height:doc.ihdr.h,rgba16,info:doc.info}}

/// One public call's runtime request record and owned, sealed logical log.
pub(crate) struct Trace {
    sites: [(AllocSite, usize); 3],
    n: usize,
    log: Tracked<AllocLog>,
}
impl Trace {
    closed spec fn log_view(&self) -> Seq<(AllocSite,nat,bool)> {self.log@.view()}
    closed spec fn prefix_recorded(&self, count: nat) -> bool {
        count<=3 && count<=self.log_view().len()
        && forall|i: int| 0<=i<count ==> self.sites@[i].0==self.log_view()[i].0
            && self.sites@[i].1==self.log_view()[i].1
    }
    closed spec fn wf(&self) -> bool {
        self.n==self.log_view().len() && self.prefix_recorded(self.n as nat)
    }
    fn new() -> (r: Self)
        ensures r.wf(),r.n==0,r.log_view()==Seq::<(AllocSite,nat,bool)>::empty(),
    {
        let tracked log=crate::alloc_guard::new_log();
        Self {sites:[(AllocSite::Raw,0);3],n:0,log:Tracked(log)}
    }
    /// Record after the guard logs its success/failure; public return traces
    /// include the request on both paths, without an unsealed log mutation.
    fn record(&mut self, site: AllocSite, elements: usize)
        requires old(self).n<3,old(self).log_view().len()==old(self).n+1,
            old(self).prefix_recorded(old(self).n as nat),
            old(self).log_view().last().0==site,old(self).log_view().last().1==elements,
        ensures final(self).wf(),final(self).n==old(self).n+1,
            final(self).log_view()==old(self).log_view(),
    {
        self.sites[self.n]=(site,elements);
        self.n+=1;
    }
    fn into_log(self) -> (r: Tracked<AllocLog>)
        ensures r@.view()==self.log_view(),
    {self.log}
    #[cfg(feature = "test-records")]
    fn list(&self) -> (r: ([(AllocSite, usize); 3], usize))
        ensures r==(self.sites,self.n),
    {(self.sites,self.n)}
}
fn oom(o: OutOfMemory) -> (r: TryError)
    ensures r==TryError::OutOfMemory(o),
{TryError::OutOfMemory(o)}

} // verus!

verus! {

/// Actual Raw/Output prefix, borrowing this call's sealed log.
fn reserve_image(doc: &Doc, out_bytes: usize, trace: &mut Trace) -> (r: Result<(Vec<u8>,Vec<u8>),TryError>)
    requires doc.ihdr.wf(),1<=out_bytes<=crate::alloc_guard::MAX_BYTES,
        old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        crate::alloc_guard::req_of_log(final(trace).log_view()).is_prefix_of(seq![
            (AllocSite::Raw,crate::layout_model::raw(doc.ihdr.interlace,crate::layout_model::bits(doc.ihdr.view()),doc.ihdr.w as nat,doc.ihdr.h as nat)),
            (AllocSite::Output,out_bytes as nat)]),
        match r {Ok((raw,out))=>
            raw@==Seq::new(crate::layout_model::raw(doc.ihdr.interlace,crate::layout_model::bits(doc.ihdr.view()),doc.ihdr.w as nat,doc.ihdr.h as nat),|_i: int|0u8)
            && out@==Seq::new(out_bytes as nat,|_i: int|0u8)
            && final(trace).log_view()==seq![(AllocSite::Raw,raw@.len(),true),(AllocSite::Output,out_bytes as nat,true)],
            Err(TryError::Format(_))=>false,
            Err(TryError::OutOfMemory(_))=>final(trace).log_view().len()==1 || final(trace).log_view().len()==2},
{
    let bytes=raw_size(&doc.ihdr,doc.ihdr.w,doc.ihdr.h);
    proof {crate::limits::caps_ok();}
    let n=bytes as usize;
    let result=try_zeroed(AllocSite::Raw,n,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::Raw,n);
    let raw=match result {Ok(v)=>v,Err(o)=>return Err(oom(o))};
    let result=try_zeroed(AllocSite::Output,out_bytes,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::Output,out_bytes);
    match result {Ok(out)=>Ok((raw,out)),Err(o)=>Err(oom(o))}
}

fn reserve_saved(n: usize,Tracked(log): Tracked<&mut AllocLog>) -> (r: Result<Vec<u8>,TryError>)
    requires 1<=n<=crate::alloc_guard::MAX_BYTES,old(log).view().len()==2,crate::alloc_guard::all_ok(old(log).view()),
    ensures final(log).view()==old(log).view().push((AllocSite::Saved,n as nat,r.is_ok())),
        crate::alloc_guard::sites::log_tied(r,final(log).view()),
        match r {Ok(v)=>v@==Seq::new(n as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let result=try_zeroed(AllocSite::Saved,n,Tracked(log));
    match result {Ok(v)=>Ok(v),Err(o)=>Err(oom(o))}
}

fn reserve_icc_window(trace: &mut Trace) -> (r: Result<Vec<u8>,TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        final(trace).log_view()==seq![(AllocSite::IccWindow,WINDOW_MAX as nat,r.is_ok())],
        match r {Ok(v)=>v@==Seq::new(WINDOW_MAX as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    proof {crate::limits::caps_ok();assert(1<=WINDOW_MAX) by (bit_vector);}
    let result=try_zeroed(AllocSite::IccWindow,WINDOW_MAX,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::IccWindow,WINDOW_MAX);
    match result {Ok(v)=>Ok(v),Err(o)=>Err(oom(o))}
}

fn reserve_icc(n: usize, trace: &mut Trace) -> (r: Result<Vec<u8>,TryError>)
    requires n<=MAX_ICC,old(trace).wf(),old(trace).n==1,
        crate::alloc_guard::all_ok(old(trace).log_view()),
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        final(trace).log_view()==old(trace).log_view().push((AllocSite::Icc,n as nat,r.is_ok())),
        match r {Ok(v)=>v@==Seq::new(n as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>n>0,_=>false},
{
    proof {crate::limits::caps_ok();}
    let result=if n==0 {try_empty(AllocSite::Icc,Tracked(trace.log.borrow_mut()))}
        else {try_zeroed(AllocSite::Icc,n,Tracked(trace.log.borrow_mut()))};
    trace.record(AllocSite::Icc,n);
    match result {Ok(v)=>Ok(v),Err(o)=>Err(oom(o))}
}

spec fn prepared_image_view(r: Result<(Doc,Vec<u8>,Vec<u8>),TryError>) -> Result<crate::document_model::Doc,TryError> {
    match r {Ok((doc,_,_))=>Ok(doc.view()),Err(e)=>Err(e)}
}

/// The actual still/constructor prefix: walk fully before either reservation.
fn prepare_image(b: &[u8], pol: &Policy, frames: bool, wide: bool,
    ev: &mut Events, trace: &mut Trace) -> (r: Result<(Doc,Vec<u8>,Vec<u8>),TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures !frames ==> crate::policy_still_equivalence::header_facts(b@,*pol,wide),
        !frames && *pol==STRICT ==> crate::still_equivalence::header_facts(b@,wide),
        final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        crate::allocation_model::image_prepared(b@,pol.walk_view(),frames,wide,prepared_image_view(r),final(ev).view(),final(trace).log_view()),
        match r {Ok((doc,raw,out))=>doc.wf(b@) && (doc.ihdr.ct==3 ==> doc.plte.is_some()) && doc.ihdr.wf() && b.len()<=crate::limits::MAX_INPUT && crate::transport_model::idat_result(b@,doc.view())
            && crate::layout_model::image_buffers(doc.ihdr.view(),wide,raw@,out@)
            && out.len()<=crate::alloc_guard::MAX_BYTES
            && raw@==Seq::new(raw@.len(),|_i: int|0u8) && out@==Seq::new(out@.len(),|_i: int|0u8)
            && (frames ==> crate::frame_state::source_document(b@,doc)),Err(_)=>true},
{
    hide(Doc::wf);hide(crate::transport_model::idat_result);hide(crate::layout_model::raw);
    hide(crate::document_model::MWF_p);hide(crate::document_model::MW_p);
    let result=if frames {crate::callers::frames(b,pol,ev)}else{crate::callers::still(b,pol,ev)};
    proof {
        if !frames && *pol==STRICT {
            crate::walk_policy::profiles_project_exactly();
            match &result {
                Ok(doc)=>{
                    crate::still_equivalence::header_request_bounds(doc.ihdr.view(),wide);
                    assert(crate::still_equivalence::header_facts(b@,wide));
                },Err(_)=>{assert(crate::still_equivalence::header_facts(b@,wide));},
            }
        }
    }
    proof {
        if !frames {
            crate::policy_still_model::walk_exact(*pol);
            match &result {
                Ok(doc)=>{crate::still_equivalence::header_request_bounds(doc.ihdr.view(),wide);},
                Err(_)=>{},
            }
            assert(crate::policy_still_equivalence::header_facts(b@,*pol,wide));
        }
    }
    let doc=match result {Err(e)=>return Err(e),Ok(doc)=>doc};
    proof {if frames {crate::frame_state::source_palette(b@,doc);}}
    let out_bytes=crate::image::output_size(&doc.ihdr,wide);
    match reserve_image(&doc,out_bytes,trace) {
        Err(e)=>Err(e),Ok((raw,out))=>Ok((doc,raw,out)),
    }
}

spec fn prepared_icc_view(r: Result<Option<(Doc,Vec<u8>)>,TryError>) -> Result<Option<crate::document_model::Doc>,TryError> {
    match r {Ok(Some((doc,_)))=>Ok(Some(doc.view())),Ok(None)=>Ok(None),Err(e)=>Err(e)}
}

/// The actual ICC prefix: no profile means no reservation; otherwise Window
/// precedes zlib validation, after the successful structural walk.
fn prepare_icc(b: &[u8], ev: &mut Events, trace: &mut Trace) -> (r: Result<Option<(Doc,Vec<u8>)>,TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        crate::allocation_model::icc_prepared(b@,prepared_icc_view(r),final(ev).view(),final(trace).log_view()),
        match r {Ok(Some((doc,ring)))=>doc.wf(b@) && b.len()<=crate::limits::MAX_INPUT && doc.iccp.is_some()
            && ring@==Seq::new(WINDOW_MAX as nat,|_i: int|0u8),_=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    let doc=match crate::callers::still(b,&STRICT,ev) {Err(e)=>return Err(e),Ok(doc)=>doc};
    if doc.iccp.is_none() {return Ok(None);}
    match reserve_icc_window(trace) {Err(e)=>Err(e),Ok(ring)=>Ok(Some((doc,ring)))}
}

/// Returned logs are the owned capabilities used by the shared production prefix.
fn image_preparation_logged(b: &[u8],pol: &Policy,frames: bool,wide: bool)
    -> (r: (Result<(Doc,Vec<u8>,Vec<u8>),TryError>,Events,Tracked<AllocLog>))
    ensures crate::allocation_model::image_prepared(b@,pol.walk_view(),frames,wide,prepared_image_view(r.0),r.1.view(),r.2@.view()),
        crate::alloc_guard::sites::log_tied(r.0,r.2@.view()),
        match r.0 {Ok((doc,raw,out))=>doc.wf(b@) && crate::transport_model::idat_result(b@,doc.view())
            && crate::layout_model::image_buffers(doc.ihdr.view(),wide,raw@,out@)
            && out.len()<=crate::alloc_guard::MAX_BYTES
            && raw@==Seq::new(raw@.len(),|_i: int|0u8) && out@==Seq::new(out@.len(),|_i: int|0u8)
            && (frames ==> crate::frame_state::source_document(b@,doc)),_=>true},
{
    let mut trace=Trace::new();
    let mut ev=Events {r1:false,r3:false,r4:false};
    let result=prepare_image(b,pol,frames,wide,&mut ev,&mut trace);
    let log=trace.into_log();
    (result,ev,log)
}

fn icc_preparation_logged(b: &[u8])
    -> (r: (Result<Option<(Doc,Vec<u8>)>,TryError>,Events,Tracked<AllocLog>))
    ensures crate::allocation_model::icc_prepared(b@,prepared_icc_view(r.0),r.1.view(),r.2@.view()),
        crate::alloc_guard::sites::log_tied(r.0,r.2@.view()),
        match r.0 {Ok(Some((doc,ring)))=>doc.wf(b@) && b.len()<=crate::limits::MAX_INPUT && doc.iccp.is_some()
            && ring@==Seq::new(WINDOW_MAX as nat,|_i: int|0u8),_=>true},
{
    let mut trace=Trace::new();
    let mut ev=Events {r1:false,r3:false,r4:false};
    let result=prepare_icc(b,&mut ev,&mut trace);
    let log=trace.into_log();
    (result,ev,log)
}

/// P15 for RGBA8 (`wide = false`) or RGBA16: walk, reserve, decode.
fn still_impl(b: &[u8],pol: &Policy,wide: bool,ev: &mut Events,trace: &mut Trace) -> (r: Result<(Doc,Vec<u8>),TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures crate::policy_still_equivalence::header_facts(b@,*pol,wide),
        match r {Ok(_)=>crate::policy_still_model::events_match(b@,*pol,final(ev).view()),_=>true},
        *pol==STRICT ==> crate::still_equivalence::header_facts(b@,wide),
        *pol==STRICT ==> match r {Ok(_)=>crate::still_model::walk_context(b@),_=>true},
        match r {Ok((_,out))=>paint::still_pixels(b@,*pol,wide,out@),_=>true},
        final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),false,wide,r,final(trace).log_view()),
        match r {Ok((doc,out))=>doc.wf(b@) && crate::allocation_model::returned_info(b@,pol.walk_view(),false,doc.info.view())
            && out.len()==crate::layout_model::output_count(doc.ihdr.view(),wide),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,*pol,false,r),
        raster::fmt_eq(b@,*pol,wide,r),
        match r {Ok((doc,out))=>raster::shape_matches(b@,*pol,wide,doc.ihdr.w,doc.ihdr.h,out.len() as nat),_=>true},
{
    hide(Doc::wf);hide(crate::document_model::MW_p);hide(inflate_verus::model::Inflate);
    let prepared=prepare_image(b,pol,false,wide,ev,trace);
    proof {crate::allocation_model::prepared_exit::<(Doc,Vec<u8>)>(b@,pol.walk_view(),false,wide,prepared_image_view(prepared),ev.view(),trace.log_view());}
    proof {crate::allocation_model::prepared_inflate_error::<(Doc,Vec<u8>)>(b@,*pol,false,wide,prepared_image_view(prepared),ev.view(),trace.log_view());}
    proof {raster::prepared_result::<(Doc,Vec<u8>)>(b@,*pol,wide,prepared_image_view(prepared),ev.view(),trace.log_view());}
    let (doc,mut raw,mut out)=match prepared {Ok(v)=>v,Err(e)=>return Err(e)};
    proof {
        crate::limits::caps_ok();crate::icc_model::short_log(trace.log_view());
        assert(crate::allocation_model::returned_info(b@,pol.walk_view(),false,doc.info.view()));
        crate::allocation_model::prepared_still_document(b@,pol.walk_view(),wide,doc.view(),ev.view(),trace.log_view());
    }
    let ghost walked_events=ev.view();
    let ghost output_len=out.len();
    let ghost default_z=crate::document_model::default_stream(b@,doc.view()).unwrap();
    {
        let mut src=IdatReader::for_doc(b,&doc);
        let mut sink=Sink::image(&mut out,doc.ihdr.w,doc.ihdr.h,wide);
        let ghost before_pixels=sink.pixels();
        let ghost after_pixels;
        let decoded=decode_image(&doc,b,&mut src,&mut raw,doc.ihdr.w,doc.ihdr.h,pol,ev,&mut sink);
        assert(raster::document_data(b@,doc.view(),*pol)==Some(decoded));
        proof {
            if decoded.is_ok() {
                crate::policy_still_model::walk_exact(*pol);
                assert(crate::policy_still_model::events_match(b@,*pol,ev.view()));
            }
        }
        proof {
            if *pol==STRICT {
                crate::still_equivalence::document_context(b@,doc);
                assert(crate::still_model::walk_context(b@));
            }
        }
        proof {after_pixels=sink.pixels();}
        sink.finish();
        match decoded {Err(e)=>return Err(TryError::Format(e)),Ok(())=>{}}
        proof {
            paint::image_from_progress(raster::inflated(default_z,paint::default_raster(doc.view(),*pol),*pol).out,
                paint::default_raster(doc.view(),*pol),crate::colour_model::from_doc(doc.view(),b@),before_pixels,after_pixels,wide,
                pol.palette_index==crate::policy::PaletteIndex::Black);
        }

    }
    assert(out.len()==output_len);
    assert(crate::allocation_model::returned_info(b@,pol.walk_view(),false,doc.info.view()));
    proof {paint::still_pixels_intro(b@,doc.view(),*pol,wide,out@,default_z,walked_events);}
    Ok((doc,out))
}

fn still8(b: &[u8],pol: &Policy,ev: &mut Events,trace: &mut Trace) -> (r: Result<Still,TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures crate::policy_still_equivalence::header_facts(b@,*pol,false),
        match r {Ok(_)=>crate::policy_still_model::events_match(b@,*pol,final(ev).view()),_=>true},
        *pol==STRICT ==> crate::still_equivalence::header_facts(b@,false),
        *pol==STRICT ==> match r {Ok(_)=>crate::still_model::walk_context(b@),_=>true},
        match r {Ok(s)=>paint::still_pixels(b@,*pol,false,s.rgba@),_=>true},
        final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),false,false,r,final(trace).log_view()),
        match r {Ok(s)=>s.info.ranges_wf(b.len() as nat) && crate::allocation_model::returned_info(b@,pol.walk_view(),false,s.info.view()),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,*pol,false,r),
        raster::fmt_eq(b@,*pol,false,r),
        match r {Ok(s)=>raster::shape_matches(b@,*pol,false,s.width,s.height,s.rgba.len() as nat),_=>true},
{
    let (doc,rgba)=match still_impl(b,pol,false,ev,trace) {Ok(v)=>v,Err(e)=>return Err(e)};
    Ok(assemble_still(doc,rgba))
}
fn still16(b: &[u8],pol: &Policy,ev: &mut Events,trace: &mut Trace) -> (r: Result<Still16,TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures crate::policy_still_equivalence::header_facts(b@,*pol,true),
        match r {Ok(_)=>crate::policy_still_model::events_match(b@,*pol,final(ev).view()),_=>true},
        *pol==STRICT ==> crate::still_equivalence::header_facts(b@,true),
        *pol==STRICT ==> match r {Ok(_)=>crate::still_model::walk_context(b@),_=>true},
        match r {Ok(s)=>paint::still_pixels(b@,*pol,true,s.rgba16@),_=>true},
        final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),false,true,r,final(trace).log_view()),
        match r {Ok(s)=>s.info.ranges_wf(b.len() as nat) && crate::allocation_model::returned_info(b@,pol.walk_view(),false,s.info.view()),_=>true},
        raster::fmt_eq(b@,*pol,true,r),
        match r {Ok(s)=>raster::shape_matches(b@,*pol,true,s.width,s.height,s.rgba16.len() as nat),_=>true},
{
    let (doc,rgba)=match still_impl(b,pol,true,ev,trace) {Ok(v)=>v,Err(e)=>return Err(e)};
    Ok(assemble_still16(doc,rgba))
}

/// Strict RGBA8 public entry: checked walk-error dispatch and Info forwarding.
pub fn try_decode_still(b: &[u8]) -> (r: Result<Still,TryError>)
    ensures crate::still_equivalence::oom_member(crate::still_equivalence::requests(b@,false),r),
        crate::still_equivalence::eq(b@,false,crate::still_equivalence::native8(r)),
        match crate::still_equivalence::native8(r) {Ok(value)=>crate::still_equivalence::success_package(b@,false,value),_=>true},
        match r {Ok(s)=>paint::still_pixels(b@,STRICT,false,s.rgba@),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,crate::document_model::strict())==crate::document_model::Outcome::Error(e,ev)
            ==> r==Err(TryError::Format(e)),
        match r {Ok(s)=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,crate::document_model::strict(),false,s.info.model_value()),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,STRICT,false,r),
        raster::fmt_eq(b@,STRICT,false,r),
        match r {Ok(s)=>raster::shape_matches(b@,STRICT,false,s.width,s.height,s.rgba.len() as nat),_=>true},
{
    let (result,log)=try_decode_still_logged(b);
    proof {crate::still_equivalence::failure_member(crate::still_equivalence::requests(b@,false),result,log@.view());}
    result
}

pub fn try_decode_still16(b: &[u8]) -> (r: Result<Still16,TryError>)
    ensures crate::still_equivalence::oom_member(crate::still_equivalence::requests(b@,true),r),
        crate::still_equivalence::eq(b@,true,crate::still_equivalence::native16(r)),
        match crate::still_equivalence::native16(r) {Ok(value)=>crate::still_equivalence::success_package(b@,true,value),_=>true},
        match r {Ok(s)=>paint::still_pixels(b@,STRICT,true,s.rgba16@),_=>true},
        raster::fmt_eq(b@,STRICT,true,r),
        match r {Ok(s)=>raster::shape_matches(b@,STRICT,true,s.width,s.height,s.rgba16.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,crate::document_model::strict())==crate::document_model::Outcome::Error(e,ev)
            ==> r==Err(TryError::Format(e)),
        match r {Ok(s)=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,crate::document_model::strict(),false,s.info.model_value()),_=>true},
{
    let (result,log)=try_decode_still16_logged(b);
    proof {crate::still_equivalence::failure_member(crate::still_equivalence::requests(b@,true),result,log@.view());}
    result
}

/// Strict result and the owned sealed log used by this same call.
pub fn try_decode_still_logged(b: &[u8]) -> (r: (Result<Still,TryError>,Tracked<AllocLog>))
    ensures crate::still_equivalence::G(b@,false,crate::still_equivalence::native8(r.0),r.1@.view()),
        crate::still_equivalence::Prog(b@,false,crate::still_equivalence::native8(r.0),r.1@.view()),
        crate::still_equivalence::only_still_sites(r.1@.view()),
        match crate::still_equivalence::native8(r.0) {Ok(value)=>crate::still_equivalence::success_package(b@,false,value),_=>true},
        match r.0 {Ok(s)=>paint::still_pixels(b@,STRICT,false,s.rgba@),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,crate::document_model::strict())==crate::document_model::Outcome::Error(e,ev)
            ==> r.0==Err(TryError::Format(e)),
        match r.0 {Ok(s)=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,crate::document_model::strict(),false,s.info.model_value()),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,STRICT,false,r.0),
        raster::fmt_eq(b@,STRICT,false,r.0),
        match r.0 {Ok(s)=>raster::shape_matches(b@,STRICT,false,s.width,s.height,s.rgba.len() as nat),_=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let result=still8(b,&STRICT,&mut ev,&mut trace);
    proof {if let Ok(s)=&result {s.info.public_observations(b.len() as nat);}}
    proof {
        let logical=crate::still_equivalence::native8(result);
        assert(raster::fmt_eq(b@,STRICT,false,logical));
        crate::still_equivalence::complete(b@,false,logical);
        if let Ok(value)=logical {crate::still_equivalence::success(b@,false,value);}
        assert(crate::allocation_model::image_call(b@,crate::document_model::strict(),false,false,logical,trace.log_view()));
        crate::still_equivalence::allocation_complete(b@,false,logical,trace.log_view());
    }
    let log=trace.into_log();
    (result,log)
}

/// Strict result and the owned sealed log used by this same call.
pub fn try_decode_still16_logged(b: &[u8]) -> (r: (Result<Still16,TryError>,Tracked<AllocLog>))
    ensures crate::still_equivalence::G(b@,true,crate::still_equivalence::native16(r.0),r.1@.view()),
        crate::still_equivalence::Prog(b@,true,crate::still_equivalence::native16(r.0),r.1@.view()),
        crate::still_equivalence::only_still_sites(r.1@.view()),
        match crate::still_equivalence::native16(r.0) {Ok(value)=>crate::still_equivalence::success_package(b@,true,value),_=>true},
        match r.0 {Ok(s)=>paint::still_pixels(b@,STRICT,true,s.rgba16@),_=>true},
        raster::fmt_eq(b@,STRICT,true,r.0),
        match r.0 {Ok(s)=>raster::shape_matches(b@,STRICT,true,s.width,s.height,s.rgba16.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,crate::document_model::strict())==crate::document_model::Outcome::Error(e,ev)
            ==> r.0==Err(TryError::Format(e)),
        match r.0 {Ok(s)=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,crate::document_model::strict(),false,s.info.model_value()),_=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let result=still16(b,&STRICT,&mut ev,&mut trace);
    proof {if let Ok(s)=&result {s.info.public_observations(b.len() as nat);}}
    proof {
        let logical=crate::still_equivalence::native16(result);
        assert(raster::fmt_eq(b@,STRICT,true,logical));
        crate::still_equivalence::complete(b@,true,logical);
        if let Ok(value)=logical {crate::still_equivalence::success(b@,true,value);}
        assert(crate::allocation_model::image_call(b@,crate::document_model::strict(),false,true,logical,trace.log_view()));
        crate::still_equivalence::allocation_complete(b@,true,logical,trace.log_view());
    }
    let log=trace.into_log();
    (result,log)
}

pub fn try_decode_still_with_policy(b: &[u8],p: Profile) -> (r: Result<(Still,bool),TryError>)
    ensures crate::still_equivalence::oom_member(crate::policy_still_equivalence::requests(b@,raster::profile(p),false),r),
        crate::policy_still_equivalence::eq(b@,raster::profile(p),false,crate::policy_still_equivalence::native8(r)),
        match r {Ok((s,_))=>paint::still_pixels(b@,raster::profile(p),false,s.rgba@),_=>true},
        raster::fmt_eq(b@,raster::profile(p),false,r),
        match r {Ok((s,_))=>raster::shape_matches(b@,raster::profile(p),false,s.width,s.height,s.rgba.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,p.walk_model())==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        match r {Ok((s,_))=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,p.walk_model(),false,s.info.model_value()),_=>true},
{
    let (result,log)=try_decode_still_with_policy_logged(b,p);
    proof {crate::still_equivalence::failure_member(crate::policy_still_equivalence::requests(b@,raster::profile(p),false),result,log@.view());}
    result
}

/// Named-policy complete result and the sealed log from this same decode.
pub fn try_decode_still_with_policy_logged(b: &[u8],p: Profile) -> (r: (Result<(Still,bool),TryError>,Tracked<AllocLog>))
    ensures crate::policy_still_equivalence::G(b@,raster::profile(p),false,crate::policy_still_equivalence::native8(r.0),r.1@.view()),
        crate::policy_still_equivalence::Prog(b@,raster::profile(p),false,crate::policy_still_equivalence::native8(r.0),r.1@.view()),
        crate::still_equivalence::only_still_sites(r.1@.view()),
        match r.0 {Ok((s,_))=>paint::still_pixels(b@,raster::profile(p),false,s.rgba@),_=>true},
        raster::fmt_eq(b@,raster::profile(p),false,r.0),
        match r.0 {Ok((s,_))=>raster::shape_matches(b@,raster::profile(p),false,s.width,s.height,s.rgba.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,p.walk_model())==crate::document_model::Outcome::Error(e,ev) ==> r.0==Err(TryError::Format(e)),
        match r.0 {Ok((s,_))=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,p.walk_model(),false,s.info.model_value()),_=>true},
{
    let pol=p.checked_policy();let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let decoded=still8(b,&pol,&mut ev,&mut trace);
    let result=match decoded {
        Ok(s)=>{
            proof {s.info.public_observations(b.len() as nat);}
            Ok((s,ev.any()))
        },Err(e)=>Err(e),
    };
    proof {
        crate::policy_still_model::walk_exact(pol);
        let logical=crate::policy_still_equivalence::native8(result);
        assert(raster::fmt_eq(b@,pol,false,logical));
        if logical.is_ok() {crate::policy_still_equivalence::bind_recovery(b@,pol,ev.view());}
        crate::policy_still_equivalence::complete(b@,pol,false,logical);
        assert(crate::allocation_model::image_call(b@,pol.walk_view(),false,false,logical,trace.log_view()));
        crate::policy_still_equivalence::allocation_complete(b@,pol,false,logical,trace.log_view());
    }
    let log=trace.into_log();
    (result,log)
}

pub fn try_decode_still16_with_policy(b: &[u8],p: Profile) -> (r: Result<(Still16,bool),TryError>)
    ensures crate::still_equivalence::oom_member(crate::policy_still_equivalence::requests(b@,raster::profile(p),true),r),
        crate::policy_still_equivalence::eq(b@,raster::profile(p),true,crate::policy_still_equivalence::native16(r)),
        match r {Ok((s,_))=>paint::still_pixels(b@,raster::profile(p),true,s.rgba16@),_=>true},
        raster::fmt_eq(b@,raster::profile(p),true,r),
        match r {Ok((s,_))=>raster::shape_matches(b@,raster::profile(p),true,s.width,s.height,s.rgba16.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,p.walk_model())==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        match r {Ok((s,_))=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,p.walk_model(),false,s.info.model_value()),_=>true},
{
    let (result,log)=try_decode_still16_with_policy_logged(b,p);
    proof {crate::still_equivalence::failure_member(crate::policy_still_equivalence::requests(b@,raster::profile(p),true),result,log@.view());}
    result
}

/// Named-policy complete result and the sealed log from this same decode.
pub fn try_decode_still16_with_policy_logged(b: &[u8],p: Profile) -> (r: (Result<(Still16,bool),TryError>,Tracked<AllocLog>))
    ensures crate::policy_still_equivalence::G(b@,raster::profile(p),true,crate::policy_still_equivalence::native16(r.0),r.1@.view()),
        crate::policy_still_equivalence::Prog(b@,raster::profile(p),true,crate::policy_still_equivalence::native16(r.0),r.1@.view()),
        crate::still_equivalence::only_still_sites(r.1@.view()),
        match r.0 {Ok((s,_))=>paint::still_pixels(b@,raster::profile(p),true,s.rgba16@),_=>true},
        raster::fmt_eq(b@,raster::profile(p),true,r.0),
        match r.0 {Ok((s,_))=>raster::shape_matches(b@,raster::profile(p),true,s.width,s.height,s.rgba16.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,p.walk_model())==crate::document_model::Outcome::Error(e,ev) ==> r.0==Err(TryError::Format(e)),
        match r.0 {Ok((s,_))=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,p.walk_model(),false,s.info.model_value()),_=>true},
{
    let pol=p.checked_policy();let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let decoded=still16(b,&pol,&mut ev,&mut trace);
    let result=match decoded {
        Ok(s)=>{
            proof {s.info.public_observations(b.len() as nat);}
            Ok((s,ev.any()))
        },Err(e)=>Err(e),
    };
    proof {
        crate::policy_still_model::walk_exact(pol);
        let logical=crate::policy_still_equivalence::native16(result);
        assert(raster::fmt_eq(b@,pol,true,logical));
        if logical.is_ok() {crate::policy_still_equivalence::bind_recovery(b@,pol,ev.view());}
        crate::policy_still_equivalence::complete(b@,pol,true,logical);
        assert(crate::allocation_model::image_call(b@,pol.walk_view(),false,true,logical,trace.log_view()));
        crate::policy_still_equivalence::allocation_complete(b@,pol,true,logical,trace.log_view());
    }
    let log=trace.into_log();
    (result,log)
}

} // verus!

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

verus! {

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

impl<'f> FrameView<'f> {
    pub closed spec fn byte_len(&self) -> nat {self.rgba@.len()}
    pub closed spec fn pixels(&self) -> Seq<u8> {self.rgba@}
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
    /// Erased reference captured from the actual canvas before frame painting.
    snapshot: Ghost<Option<Seq<u8>>>,
    history: Ghost<CallHistory>,
    pixel_history: Ghost<CallCanvas>,
    log: Tracked<AllocLog>,
    base: Events,
    state: FrameState,
}

pub closed spec fn constructor_log(result: &(Result<Frames<'_>,TryError>,Tracked<Option<AllocLog>>)) -> Seq<(AllocSite,nat,bool)> {
    match result.0 {Ok(receiver)=>receiver.log_view(),Err(_)=>match result.1@ {Some(log)=>log.view(),None=>Seq::empty()}}
}

impl<'a> Frames<'a> {
    pub closed spec fn saved_present(&self) -> bool {self.saved.is_some()}
    pub closed spec fn canvas_len(&self) -> nat {self.canvas@.len()}
    pub closed spec fn is_done(&self) -> bool {self.state.done()}
    pub closed spec fn log_view(&self) -> Seq<(AllocSite,nat,bool)> {self.log@.view()}
    pub closed spec fn history_view(&self) -> iterdata::History {self.history@}
    pub closed spec fn call_count(&self) -> nat {self.history@.calls}
    pub closed spec fn model_input(&self) -> Seq<u8> {self.b@}
    pub closed spec fn model_document(&self) -> crate::document_model::Doc {self.doc.view()}
    pub closed spec fn model_policy(&self) -> Policy {self.pol}
    pub closed spec fn raw_len(&self) -> nat {self.raw@.len()}
    pub closed spec fn canvas_pixels(&self) -> Seq<u8> {self.canvas@}
    pub closed spec fn restore_reference(&self) -> Option<Seq<u8>> {self.snapshot@}
    pub closed spec fn canvas_trace(&self) -> canvas_model::Trace {self.pixel_history@}
    pub closed spec fn previous_control(&self) -> Option<((u32,u32,u32,u32),u8)> {self.state.previous()}
    closed spec fn pixel_conditions(&self) -> bool {
        canvas_model::wf(self.b@,self.doc.view(),self.pol,self.pixel_history@)
        && self.pixel_history@.after.len()==self.history@.returned
        && ((matches!(self.history_view().phase,iterdata::Phase::Active) || matches!(self.history_view().phase,iterdata::Phase::Iend))
            ==> self.canvas@==canvas_model::canvas_before(self.pixel_history@,self.history@.returned))
        && (matches!(self.history_view().phase,iterdata::Phase::Active) ==> match canvas_model::previous(self.doc.view(),self.history@.returned) {
            Some((_,2))=>self.snapshot@==canvas_model::reference(self.pixel_history@,self.history@.returned),_=>true})
    }
    /// Public-boundary pixel history: the reference is the derived before-draw
    /// canvas of the preceding frame, not an arbitrary saved-buffer witness.
    #[verifier::opaque]
    pub closed spec fn pixel_wf(&self) -> bool {self.pixel_conditions()}
    proof fn close_pixels(&self)
        requires self.pixel_conditions(),
        ensures self.pixel_wf(),
    {reveal(Frames::pixel_wf);}
    pub proof fn pixel_context(&self)
        requires self.pixel_wf(),
        ensures canvas_model::wf(self.model_input(),self.model_document(),self.model_policy(),self.canvas_trace()),
            self.canvas_trace().after.len()==self.history_view().returned,
            (matches!(self.history_view().phase,iterdata::Phase::Active) || matches!(self.history_view().phase,iterdata::Phase::Iend))
                ==> self.canvas_pixels()==canvas_model::canvas_before(self.canvas_trace(),self.history_view().returned),
            (matches!(self.history_view().phase,iterdata::Phase::Active) || matches!(self.history_view().phase,iterdata::Phase::Iend))
                ==> self.canvas_pixels().len()==4*(self.model_document().ihdr.width as nat)*(self.model_document().ihdr.height as nat),
            matches!(self.history_view().phase,iterdata::Phase::Active) ==> match canvas_model::previous(self.model_document(),self.history_view().returned) {
                Some((_,2))=>self.restore_reference()==canvas_model::reference(self.canvas_trace(),self.history_view().returned),_=>true},
    {reveal(Frames::pixel_wf);canvas_model::current_len(self.b@,self.doc.view(),self.pol,self.pixel_history@);}

    proof fn pixels_preserve_iterator(before: Frames<'a>,after: Frames<'a>)
        requires before.iterator_wf(),after.b==before.b,after.doc==before.doc,after.pol==before.pol,
            after.raw==before.raw,after.canvas==before.canvas,after.state==before.state,
            after.history==before.history,after.log==before.log,
        ensures after.iterator_wf(),
    {reveal(Frames::iterator_wf);}

    pub proof fn canvas_geometry(&self)
        requires self.buffers_wf(),
        ensures paint::domain(paint::default_raster(self.model_document(),self.model_policy())),
            self.canvas_pixels().len()==4*(self.model_document().ihdr.width as nat)*(self.model_document().ihdr.height as nat),1<=self.model_document().ihdr.width,
    {crate::limits::caps_ok();}

    proof fn pixels_idle(before: Frames<'a>,after: Frames<'a>)
        requires before.pixel_wf(),after.b==before.b,after.doc==before.doc,after.pol==before.pol,
            after.pixel_history==before.pixel_history,after.history@.returned==before.history@.returned,
            (matches!(after.history_view().phase,iterdata::Phase::Active) || matches!(after.history_view().phase,iterdata::Phase::Iend))
                ==> (matches!(before.history_view().phase,iterdata::Phase::Active) || matches!(before.history_view().phase,iterdata::Phase::Iend)) && after.canvas@==before.canvas@,
            matches!(after.history_view().phase,iterdata::Phase::Active) ==> matches!(before.history_view().phase,iterdata::Phase::Active) && after.snapshot==before.snapshot,
        ensures after.pixel_wf(),
    {before.pixel_context();after.close_pixels();}

    proof fn pixels_returned(before: Frames<'a>,after: Frames<'a>)
        requires before.pixel_wf(),before.buffers_wf(),after.b==before.b,after.doc==before.doc,after.pol==before.pol,
            after.canvas@.len()==before.canvas@.len(),
            before.history@.returned<iterdata::count(before.doc.view()),
            canvas_model::step(before.b@,before.doc.view(),before.pol,before.history@.returned,before.canvas@,
                canvas_model::reference(before.pixel_history@,before.history@.returned),after.canvas@),
            after.pixel_history@==canvas_model::append(before.doc.view(),before.pixel_history@,after.canvas@),
            after.history@.returned==before.history@.returned+1,
            matches!(before.history_view().phase,iterdata::Phase::Active),
            matches!(after.history_view().phase,iterdata::Phase::Active) || matches!(after.history_view().phase,iterdata::Phase::Iend),
            match canvas_model::previous(after.doc.view(),after.history@.returned) {
                Some((_,2))=>after.snapshot@==Some(canvas_model::before_draw(before.doc.view(),before.pixel_history@,before.history@.returned)),_=>true},
        ensures after.pixel_wf(),
    {
        before.pixel_context();canvas_model::context(before.b@,before.doc.view(),before.pol,before.pixel_history@);
        canvas_model::append_wf(before.b@,before.doc.view(),before.pol,before.pixel_history@,after.canvas@);
        after.close_pixels();
    }

    closed spec fn native_history(&self,h: iterdata::History) -> bool {
        match h.phase {
            iterdata::Phase::Active=>if self.doc.actl.is_some() {self.state.index()==Some(h.returned)}else{self.state.single() && h.returned==0},
            _=>self.state.done(),
        }
    }
    #[verifier::opaque]
    pub closed spec fn iterator_wf(&self) -> bool {
        iterdata::from_walk(self.b@,self.doc.view(),self.pol)
        && iterdata::history_wf(self.b@,self.doc.view(),self.pol,self.history@)
        && self.native_history(self.history@)
        && self.log_view()==iterdata::log_for(self.b@,self.doc.view(),self.pol,self.history@,self.raw@.len(),self.canvas@.len())
    }
    proof fn history_preserves_state(before: Frames<'a>,after: Frames<'a>)
        requires before.source_wf(),after.b==before.b,after.doc==before.doc,after.state==before.state,
            after.pol==before.pol,after.base==before.base,after.raw==before.raw,after.canvas==before.canvas,
            after.saved==before.saved,after.snapshot==before.snapshot,after.log==before.log,
        ensures after.source_wf(),before.buffers_wf() ==> after.buffers_wf(),
            before.allocation_wf() ==> after.allocation_wf(),before.initial_wf() ==> after.initial_wf(),
            after.log_view()==before.log_view(),after.raw@==before.raw@,after.canvas@==before.canvas@,
    {reveal(Frames::source_wf);}
    proof fn iterator_context(&self)
        requires self.iterator_wf(),
        ensures iterdata::from_walk(self.b@,self.doc.view(),self.pol),
            iterdata::history_wf(self.b@,self.doc.view(),self.pol,self.history@),self.native_history(self.history@),
            self.log_view()==iterdata::log_for(self.b@,self.doc.view(),self.pol,self.history@,self.raw@.len(),self.canvas@.len()),
    {reveal(Frames::iterator_wf);}
    proof fn execution_context(&self)
        requires self.iterator_wf(),
        ensures iterdata::history_wf(self.b@,self.doc.view(),self.pol,self.history@),self.native_history(self.history@),
    {self.iterator_context();}

    #[verifier::spinoff_prover]
    proof fn compose_call(before: Frames<'a>,after: Frames<'a>,call: iterdata::Call)
        requires before.iterator_wf(),before.buffers_wf(),after.buffers_wf(),after.b==before.b,after.doc==before.doc,after.pol==before.pol,
            after.raw@.len()==before.raw@.len(),after.canvas@.len()==before.canvas@.len(),
            after.history@.calls==before.history@.calls+1,
            iterdata::history_wf(after.b@,after.doc.view(),after.pol,after.history@),after.native_history(after.history@),
            iterdata::is_saved_at(before.b@,before.doc.view(),before.pol,before.history@.calls) ==> !iterdata::is_oom(before.history@),
            !iterdata::is_saved_at(before.b@,before.doc.view(),before.pol,before.history@.calls) ==> iterdata::is_oom(after.history@)==iterdata::is_oom(before.history@),
            iterdata::is_oom(after.history@) && !iterdata::is_oom(before.history@) ==> after.history@.returned==before.history@.calls,
            after.log_view()==before.log_view()+if iterdata::is_saved_at(before.b@,before.doc.view(),before.pol,before.history@.calls)
                {seq![(AllocSite::Saved,before.canvas@.len(),!iterdata::is_oom(after.history@))]}else{Seq::<(AllocSite,nat,bool)>::empty()},
            match call {
                iterdata::Call::Frame=>matches!(before.history_view().phase,iterdata::Phase::Active) && after.history@.returned==before.history@.returned+1,
                iterdata::Call::Format(e)=>matches!(before.history_view().phase,iterdata::Phase::Active) && after.history@.phase==iterdata::Phase::Format(e) && after.history@.returned==before.history@.returned,
                iterdata::Call::End=>matches!(before.history_view().phase,iterdata::Phase::Active) ==> matches!(after.history_view().phase,iterdata::Phase::Iend) && after.history@.returned==before.history@.returned,
                iterdata::Call::Oom=>true,
            },
        ensures after.iterator_wf(),iterdata::call_eq(before.b@,before.doc.view(),before.pol,before.history@,call),after.model_input()==before.model_input(),after.model_document()==before.model_document(),after.model_policy()==before.model_policy(),crate::alloc_guard::req_of_log(after.log_view())==crate::alloc_guard::req_of_log(before.log_view())+iterdata::request_p(before.b@,before.pol,before.history@.calls),
            iterdata::is_oom(after.history@) && !iterdata::is_oom(before.history@) ==> iterdata::SavedAt_p(before.b@,before.pol)==Some(before.history@.calls),
    {
        hide(iterdata::log_for);hide(iterdata::is_saved_at);hide(raster::document_data);
        before.iterator_context();
        iterdata::log_step(before.b@,before.doc.view(),before.pol,before.history@,after.history@,before.raw@.len(),before.canvas@.len());
        after.close_iterator();
        iterdata::call_transition(before.b@,before.doc.view(),before.pol,before.history@,after.history@,call);
        before.allocation_clauses();after.allocation_clauses();iterdata::requests_step(before.b@,before.pol,before.history@.calls);
        if iterdata::is_oom(after.history@) && !iterdata::is_oom(before.history@) {
            iterdata::history_context(after.b@,after.doc.view(),after.pol,after.history@);
            iterdata::default_intro(after.b@,after.doc.view(),after.pol);
            iterdata::canonical_saved_at(after.b@,after.doc.view(),after.pol);
            iterdata::saved_at_unique(after.b@,after.doc.view(),after.pol,after.history@.returned,after.history@.returned);
        }
    }

    proof fn close_iterator(&self)
        requires iterdata::from_walk(self.b@,self.doc.view(),self.pol),
            iterdata::history_wf(self.b@,self.doc.view(),self.pol,self.history@),self.native_history(self.history@),
            self.log_view()==iterdata::log_for(self.b@,self.doc.view(),self.pol,self.history@,self.raw@.len(),self.canvas@.len()),
        ensures self.iterator_wf(),
    {reveal(Frames::iterator_wf);}
    /// Expose mathematical per-call requests without exposing receiver fields.
    pub proof fn allocation_history(&self)
        requires self.iterator_wf(),
        ensures iterdata::from_walk(self.model_input(),self.model_document(),self.model_policy()),
            iterdata::history_wf(self.model_input(),self.model_document(),self.model_policy(),self.history_view()),
            self.log_view()==iterdata::log_for(self.model_input(),self.model_document(),self.model_policy(),self.history_view(),self.raw_len(),self.canvas_len()),
            iterdata::is_oom(self.history_view()) ==> iterdata::SavedAt(self.model_input(),self.model_document(),self.model_policy())==Some(self.history_view().returned),
    {
        self.iterator_context();iterdata::history_context(self.b@,self.doc.view(),self.pol,self.history@);
        if iterdata::is_oom(self.history@) {iterdata::saved_at_unique(self.b@,self.doc.view(),self.pol,self.history@.returned,self.history@.returned);}
    }
    #[verifier::spinoff_prover]
    proof fn allocation_sizes(&self)
        requires self.buffers_wf(),
        ensures self.raw_len()==crate::layout_model::raw(self.model_document().ihdr.interlace,
                crate::layout_model::bits(self.model_document().ihdr),self.model_document().ihdr.width as nat,self.model_document().ihdr.height as nat),
            self.canvas_len()==crate::layout_model::output_count(self.model_document().ihdr,false),
    {reveal(Frames::buffers_wf);}

    /// A-PNG-4 clauses 1-2: completed calls determine the exact cumulative
    /// requests; the OOM phase retains its unique failed Saved entry.
    pub proof fn allocation_clauses(&self)
        requires self.iterator_wf(),self.buffers_wf(),
        ensures crate::alloc_guard::req_of_log(self.log_view())==iterdata::requests_p(self.model_input(),self.model_policy(),self.call_count()),
            crate::alloc_guard::all_ok(self.log_view())==!iterdata::is_oom(self.history_view()),
            iterdata::is_oom(self.history_view()) ==> iterdata::SavedAt_p(self.model_input(),self.model_policy())==Some(self.history_view().returned)
                && self.log_view()==seq![(AllocSite::Raw,self.raw_len(),true),(AllocSite::Output,self.canvas_len(),true),(AllocSite::Saved,self.canvas_len(),false)],
    {
        self.iterator_context();self.allocation_sizes();
        iterdata::history_context(self.b@,self.doc.view(),self.pol,self.history@);
        iterdata::default_intro(self.b@,self.doc.view(),self.pol);
        iterdata::canonical_saved_at(self.b@,self.doc.view(),self.pol);
        iterdata::requests_canonical(self.b@,self.doc.view(),self.pol,self.history@.calls);
        iterdata::log_properties(self.b@,self.doc.view(),self.pol,self.history@,self.raw@.len(),self.canvas@.len());
        if iterdata::is_oom(self.history@) {iterdata::oom_log(self.b@,self.doc.view(),self.pol,self.history@,self.raw@.len(),self.canvas@.len());}
    }

    proof fn request_context(&self)
        requires self.iterator_wf(),self.source_wf(),self.buffers_wf(),
        ensures (self.saved_call() && self.saved.is_none())==iterdata::is_saved_at(self.b@,self.doc.view(),self.pol,self.history@.calls),
    {
        hide(Frames::buffers_wf);
        self.iterator_context();self.saved_context();
        iterdata::history_context(self.b@,self.doc.view(),self.pol,self.history@);
        match self.history@.phase {
            iterdata::Phase::Active=>{
                if self.doc.actl.is_some() && self.history@.returned<iterdata::count(self.doc.view()) {
                    iterdata::cache_request(self.b@,self.doc.view(),self.pol,self.history@.returned);
                }
            },
            _=>{iterdata::history_fused(self.b@,self.doc.view(),self.pol,self.history@);},
        }
    }
    pub closed spec fn default_data_ok(&self) -> bool {iterdata::default_success(self.b@,self.doc.view(),self.pol)}
    pub closed spec fn allocation_wf(&self) -> bool {
        iterlog::wf(self.log_view(),self.raw@.len(),self.canvas@.len(),self.saved.is_some(),self.state.done())
    }
    proof fn allocation_context(&self)
        requires self.allocation_wf(),
        ensures self.log_view().len()==2 || self.log_view().len()==3,
            self.log_view().take(2)==iterlog::initial(self.raw@.len(),self.canvas@.len()),
            crate::alloc_guard::all_ok(self.log_view())==(self.log_view().len()==2 || self.saved.is_some()),
            !self.state.done() && self.saved.is_none() ==> self.log_view().len()==2,
    {iterlog::all_ok_cases(self.log_view(),self.raw@.len(),self.canvas@.len(),self.saved.is_some(),self.state.done());}

    /// Source/metadata invariant only; buffer layout and pixel semantics are
    /// separate obligations of the image and allocation stages.
    #[verifier::opaque]
    pub closed spec fn source_wf(&self) -> bool {
        self.state.wf(self.b@,self.doc)
        && self.doc.info.num_plays==match self.doc.actl {Some(a)=>Some(a.num_plays),None=>None}
    }

    /// Storage lengths and presence of the snapshot required by cached disposal.
    pub closed spec fn buffers_wf(&self) -> bool {
        self.doc.ihdr.wf()
        && crate::layout_model::image_buffers(self.doc.ihdr.view(),false,self.raw@,self.canvas@)
        && (match self.saved {None=>true,Some(v)=>v@.len()==self.canvas@.len()})
        && (crate::frame_state::previous_needs_saved(self.state.previous()) ==> self.saved.is_some())
        && self.restores_previous(self.state.previous())
        && (match self.state.index() {Some(k)=>self.saved.is_some()==crate::frame_transport_model::saved_before(self.doc.frame_ranges@,k),None=>true})
    }

    /// Current pixels have the constructor's independent default/initial values.
    pub closed spec fn initial_pixels(&self) -> bool {
        paint::initial_canvas(self.b@,self.doc.view(),self.pol,self.canvas@)
    }

    pub closed spec fn initial_wf(&self) -> bool {
        (self.state.single() || self.state.index()==Some(0nat) ==> self.initial_pixels())
        && (self.state.index()==Some(0nat) ==> self.state.previous()==None)
    }
    proof fn initial_context(&self)
        requires self.initial_wf(),
        ensures self.state.single() || self.state.index()==Some(0nat) ==> self.initial_pixels(),
            self.state.index()==Some(0nat) ==> self.state.previous()==None,
    {}

    closed spec fn restores_previous(&self,previous: crate::frame_state::Previous) -> bool {
        match previous {
            Some(((x,y,w,h),2))=>composite::saved_before_frame(self.canvas@,self.saved_pixels(),self.snapshot@,
                self.doc.ihdr.w as nat,composite::rect(x,y,w,h)),
            _=>true,
        }
    }

    /// The current reached call qualifies; prior qualifiers are captured by buffers_wf.
    pub closed spec fn saved_call(&self) -> bool {
        match self.state.index() {
            Some(k)=>0<k<self.doc.frame_ranges@.len() && self.doc.frame_ranges@[k as int].dispose==2,
            None=>false,
        }
    }

    closed spec fn saved_pixels(&self) -> Option<Seq<u8>> {
        match self.saved {None=>None,Some(v)=>Some(v@)}
    }

    fn from_decoded(b: &'a [u8], pol: Policy, mut doc: Doc,
        raw: Vec<u8>, canvas: Vec<u8>, base: Events) -> (r: Self)
        requires crate::frame_state::source_document(b@,doc),
            crate::layout_model::image_buffers(doc.ihdr.view(),false,raw@,canvas@),
        ensures r.source_wf(),r.buffers_wf(),r.history@==iterdata::initial(),r.native_history(iterdata::initial()),r.log_view()==Seq::<(AllocSite,nat,bool)>::empty(),r.b==b,r.pol==pol,r.base==base,r.saved.is_none(),
            r.raw@==raw@,r.canvas@==canvas@,r.pixel_history@==canvas_model::empty(canvas@),
            paint::initial_canvas(b@,doc.view(),pol,canvas@) ==> r.initial_pixels() && r.initial_wf() && r.pixel_wf(),
            r.doc.view()==iterdata::constructor_doc(doc.view()),
            r.doc.info.view()==crate::document_model::constructor_info(doc.view()),r.doc.info.ranges_wf(b.len() as nat),
            r.state.single()==doc.actl.is_none(),!r.state.done(),r.state.previous()==None,
            r.state.index()==if doc.actl.is_some() {Some(0nat)}else{None},
    {
        let ghost original_doc=doc.view();
        let ghost original_pixels=canvas@;
        crate::frame_state::constructor_info(b,&mut doc);
        proof {paint::initial_metadata_preserved(b@,original_doc,doc.view(),pol,canvas@);}
        let state=FrameState::new(b,&doc);
        proof {crate::frame_transport_model::saved_before_start(doc.frame_ranges@);}
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);reveal(crate::frame_state::source_document);}
        {
            let tracked log=crate::alloc_guard::new_log();
            let r=Self {b,pol,doc,raw,canvas,saved:None,snapshot:Ghost(None),history:Ghost(iterdata::initial()),
                pixel_history:Ghost(canvas_model::empty(original_pixels)),log:Tracked(log),base,state};
            proof {
                if paint::initial_canvas(b@,original_doc,pol,original_pixels) {
                    canvas_model::initial(b@,r.doc.view(),pol,original_pixels);
                    r.close_pixels();
                }
            }
            r
        }
    }

    fn begin_source(&mut self) -> (r: Step<'a>)
        requires old(self).source_wf(),
        ensures final(self).source_wf(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,final(self).log_view()==old(self).log_view(),
            old(self).allocation_wf() ==> final(self).allocation_wf(),
            old(self).buffers_wf() ==> final(self).buffers_wf(),final(self).state.done(),
            final(self).doc==old(self).doc,final(self).b==old(self).b,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            final(self).saved==old(self).saved,final(self).base==old(self).base,
            final(self).pol==old(self).pol,
            final(self).snapshot==old(self).snapshot,
            old(self).state.done() ==> matches!(r,Step::Done),
            match r {Step::Done=>old(self).state.done() || old(self).state.index()==Some(old(self).doc.info.frame_count as nat),
                Step::Single=>old(self).state.single(),Step::Frame(_)=>true},
            match r {Step::Frame(job)=>job.wf(final(self).b@,final(self).doc)
                && (old(self).buffers_wf() ==> final(self).restores_previous(job.previous))
                && job.previous==old(self).state.previous() && old(self).state.index()==Some(job.index as nat)
                && (old(self).buffers_wf() && crate::frame_state::previous_needs_saved(job.previous) ==> final(self).saved.is_some()),_=>true},
    {
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);}
        self.state.begin_next(self.b,&self.doc)
    }

    fn commit_source(&mut self, resume: FrameState)
        requires old(self).source_wf(),
            old(self).allocation_wf() ==> crate::alloc_guard::all_ok(old(self).log_view()),resume.wf(old(self).b@,old(self).doc),old(self).restores_previous(resume.previous()),
            crate::frame_state::previous_needs_saved(resume.previous()) ==> old(self).saved.is_some(),
            match resume.index() {Some(k)=>old(self).saved.is_some()==crate::frame_transport_model::saved_before(old(self).doc.frame_ranges@,k),None=>true},
        ensures final(self).source_wf(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,final(self).log_view()==old(self).log_view(),
            old(self).allocation_wf() ==> final(self).allocation_wf(),
            old(self).buffers_wf() ==> final(self).buffers_wf(),final(self).state==resume,
            final(self).doc==old(self).doc,final(self).b==old(self).b,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            final(self).saved==old(self).saved,final(self).base==old(self).base,
            final(self).pol==old(self).pol,
            final(self).snapshot==old(self).snapshot,
    {
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);}
        self.state=resume;
    }

    fn ensure_saved(&mut self,needed: bool) -> (r: Result<(),TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).allocation_wf(),old(self).state.done(),
            needed && old(self).saved.is_none() ==> old(self).log_view().len()==2,
        ensures final(self).source_wf(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,final(self).buffers_wf(),final(self).allocation_wf(),
            final(self).doc==old(self).doc,final(self).state==old(self).state,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).snapshot==old(self).snapshot,
            !needed || old(self).saved.is_some() ==> final(self).log_view()==old(self).log_view(),
            needed && old(self).saved.is_none() ==> final(self).log_view()==old(self).log_view()+seq![(AllocSite::Saved,old(self).canvas@.len(),r.is_ok())],
            r.is_ok() && needed ==> final(self).saved.is_some(),
            r.is_ok() ==> final(self).saved.is_some()==(old(self).saved.is_some() || needed),
            old(self).saved.is_some() || !needed ==> final(self).saved==old(self).saved,
            r.is_err() ==> needed && old(self).saved.is_none(),
            r.is_err() ==> matches!(r,Err(TryError::OutOfMemory(_))) && final(self).saved.is_none(),
            r.is_err() ==> crate::alloc_guard::sites::log_tied(r,final(self).log_view()),
    {
        proof {
            reveal(Frames::source_wf);reveal(Frames::buffers_wf);
            self.allocation_context();reveal(Frames::allocation_wf);crate::limits::caps_ok();
        }
        if !needed || self.saved.is_some() {return Ok(());}
        let n=self.canvas.len();
        proof {
            assert(4*(self.doc.ihdr.w as int)*(self.doc.ihdr.h as int)>=4) by (nonlinear_arith)
                requires self.doc.ihdr.w>=1,self.doc.ihdr.h>=1;
            assert(1<=n<=crate::limits::MAX_OUTPUT<=crate::alloc_guard::MAX_BYTES);
        }
        proof {iterlog::all_ok_cases(self.log_view(),self.raw@.len(),self.canvas@.len(),self.saved.is_some(),self.state.done());}
        let ghost prior=self.log_view();
        let result=reserve_saved(n,Tracked(self.log.borrow_mut()));
        proof {iterlog::append_saved(prior,self.raw@.len(),n as nat,result.is_ok());}
        match result {Err(e)=>Err(e),Ok(v)=>{self.saved=Some(v);Ok(())}}

    }

    pub closed spec fn info_value(&self) -> Info {self.doc.info}

    /// IHDR, the colour chunks and raw num_plays (P8.3).
    pub fn info(&self) -> (r: &Info)
        ensures *r==self.info_value(),
    {&self.doc.info}

}

/// Source/metadata composition only: supplied buffers are never decoded here.
#[cfg(verus_only)]
pub(crate) fn receiver_source_witness(b: &[u8], pol: &Policy, raw: Vec<u8>, canvas: Vec<u8>) {
    hide(crate::frame_transport_model::result);
    let mut events=Events {r1:true,r3:true,r4:true};
    let result=crate::callers::frames(b,pol,&mut events);
    match result {Err(_)=>{},Ok(doc)=> {
        let ghost before=doc.view();
        proof {reveal(crate::frame_state::source_document);}
        let raw_len=raw_size(&doc.ihdr,doc.ihdr.w,doc.ihdr.h);
        let out_len=crate::image::output_size(&doc.ihdr,false);
        if raw.len() as u64!=raw_len || canvas.len()!=out_len {return;}
        let mut receiver=Frames::from_decoded(b,*pol,doc,raw,canvas,events);
        let info=receiver.info();
        proof {assert(info.view()==crate::document_model::constructor_info(before));}
        let step=receiver.begin_source();
        match step {Step::Frame(job)=> {
            proof {reveal(crate::frame_state::FrameJob::wf);assert(job.index==0);assert(job.effective_dispose!=2);crate::frame_transport_model::saved_before_start(receiver.doc.frame_ranges@);}
            receiver.commit_source(job.resume);
            proof {assert(receiver.source_wf());}
        },_=>{assert(receiver.source_wf());}}
    }}
}

fn zero_all(out: &mut [u8])
    ensures final(out).len()==old(out).len(),
        forall|j: int| 0<=j<final(out).len() ==> #[trigger] final(out)[j]==0,
{
    let n=out.len();let mut i=0usize;
    while i<n
        invariant i<=n,out.len()==n,
            forall|j: int| 0<=j<i ==> #[trigger] out[j]==0,
        decreases n-i,
    {out[i]=0;i+=1;}
}

impl<'a> Frames<'a> {
    /// The Strict frame decoder (D1: the unqualified entry is Strict). Defined
    /// as `try_new_with_policy(b, Profile::Strict)`, with the same errors and
    /// guard requests (council M1BQ-Q5b; THEOREMS.md writes it
    /// `Frames::try_new(b, Strict)`).
    pub fn try_new(b: &'a [u8]) -> (r: Result<Frames<'a>,TryError>)
        ensures iterdata::constructor_eq(b@,STRICT,r),match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_pixels() && receiver.initial_wf() && receiver.pixel_wf() && receiver.allocation_wf() && receiver.default_data_ok() && receiver.iterator_wf() && receiver.history_view()==iterdata::initial() && receiver.model_input()==b@ && receiver.model_policy()==STRICT
            && receiver.info_value().ranges_within(b.len() as nat)
            && crate::allocation_model::returned_frame_info(b@,crate::document_model::strict(),receiver.info_value().model_value()),_=>true},
            forall|e: Error,ev: crate::document_model::Events| crate::document_model::MWF_p(b@,crate::document_model::strict())
                ==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        crate::allocation_model::propagated_inflate_error(b@,STRICT,true,r),
    {
        proof {crate::walk_policy::profiles_project_exactly();}
        let result=Self::with_record(b,STRICT);
        proof {if let Ok(receiver)=&result {receiver.info_value().public_observations(b.len() as nat);}}
        result
    }

    /// The frame decoder under a named profile: the walk (with the R4 fallback
    /// of `apng_errors = Static`), `Raw` and `Output`, and the default image;
    /// its errors are the constructor's. THEOREMS.md writes it
    /// `Frames::try_new(b, p)` (council M1BQ-Q5b).
    pub fn try_new_with_policy(b: &'a [u8],p: Profile) -> (r: Result<Frames<'a>,TryError>)
        ensures iterdata::constructor_eq(b@,raster::profile(p),r),match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_pixels() && receiver.initial_wf() && receiver.pixel_wf() && receiver.allocation_wf() && receiver.default_data_ok() && receiver.iterator_wf() && receiver.history_view()==iterdata::initial() && receiver.model_input()==b@ && receiver.model_policy()==raster::profile(p)
            && receiver.info_value().ranges_within(b.len() as nat)
            && crate::allocation_model::returned_frame_info(b@,p.walk_model(),receiver.info_value().model_value()),_=>true},
            forall|e: Error,ev: crate::document_model::Events| crate::document_model::MWF_p(b@,p.walk_model())
                ==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
    {
        let pol=p.checked_policy();let result=Self::with_record(b,pol);
        proof {if let Ok(receiver)=&result {receiver.info_value().public_observations(b.len() as nat);}}
        result
    }

    /// Test-only: the constructor under an arbitrary record (D1: arbitrary
    /// records are never reachable from a consumer). Used for the council's
    /// mixed-record cases (M1BQ-Q1 record scope, M1BQ-Q2).
    #[cfg(feature = "test-records")]
    #[doc(hidden)]
    pub fn try_new_with_record(b: &'a [u8], pol: Policy) -> Result<Frames<'a>, TryError> {
        Frames::with_record(b, pol)
    }

    pub fn try_new_logged(b: &'a [u8]) -> (r: (Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures iterdata::constructor_eq(b@,STRICT,r.0),crate::allocation_model::image_call(b@,crate::document_model::strict(),true,false,r.0,constructor_log(&r)),
            match r.0 {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_wf() && receiver.pixel_wf() && receiver.allocation_wf() && receiver.default_data_ok() && receiver.iterator_wf() && receiver.history_view()==iterdata::initial() && receiver.model_input()==b@ && receiver.model_policy()==STRICT && r.1@.is_none(),Err(_)=>r.1@.is_some()},
    {
        proof {crate::walk_policy::profiles_project_exactly();}
        Self::with_logged(b,STRICT)
    }

    /// The named-profile constructor moves its actual sealed log into the
    /// decoder on success and returns that log alongside an error on failure.
    pub fn try_new_with_policy_logged(b: &'a [u8],p: Profile)
        -> (r: (Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures iterdata::constructor_eq(b@,raster::profile(p),r.0),crate::allocation_model::image_call(b@,p.walk_model(),true,false,r.0,constructor_log(&r)),
            match r.0 {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_pixels()
                && receiver.initial_wf() && receiver.pixel_wf() && receiver.allocation_wf() && receiver.default_data_ok() && receiver.iterator_wf() && receiver.history_view()==iterdata::initial() && receiver.model_input()==b@ && receiver.model_policy()==raster::profile(p) && r.1@.is_none(),Err(_)=>r.1@.is_some()},
    {Self::with_logged(b,p.checked_policy())}

    #[verifier::spinoff_prover]
    fn with_logged(b: &'a [u8],pol: Policy) -> (r: (Result<Frames<'a>,TryError>,Tracked<Option<AllocLog>>))
        ensures iterdata::constructor_eq(b@,pol,r.0),crate::allocation_model::image_call(b@,pol.walk_view(),true,false,r.0,constructor_log(&r)),
            match r.0 {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_pixels() && receiver.initial_wf() && receiver.pixel_wf() && receiver.allocation_wf() && receiver.default_data_ok() && receiver.iterator_wf() && receiver.history_view()==iterdata::initial() && receiver.model_input()==b@ && receiver.model_policy()==pol
                && receiver.doc.info.ranges_wf(b.len() as nat)
                && crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view()) && r.1@.is_none(),Err(_)=>r.1@.is_some()},
            forall|e: Error,ev: crate::document_model::Events| crate::document_model::MWF_p(b@,pol.walk_view())
                ==crate::document_model::Outcome::Error(e,ev) ==> r.0==Err(TryError::Format(e)),
            crate::allocation_model::propagated_inflate_error(b@,pol,true,r.0),
    {
        proof {reveal(Frames::source_wf);}
        let mut trace=Trace::new();
        let result=Self::with_trace(b,pol,&mut trace);
        let owned=trace.into_log();
        match result {
            Ok(mut receiver)=>{
                proof {receiver.pixel_context();}
                receiver.log=owned;
                proof {
                    iterlog::initial_wf(receiver.raw@.len(),receiver.canvas@.len());
                    iterdata::default_context(b@,receiver.doc.view(),pol);
                    iterdata::history_initial(b@,receiver.doc.view(),pol);
                    receiver.close_iterator();
                    receiver.close_pixels();
                }
                (Ok(receiver),Tracked(None))
            },
            Err(e)=>{let tracked log=owned.get();(Err(e),Tracked(Some(log)))},
        }
    }

    fn with_record(b: &'a [u8],pol: Policy) -> (r: Result<Frames<'a>,TryError>)
        ensures iterdata::constructor_eq(b@,pol,r),match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_pixels() && receiver.initial_wf() && receiver.pixel_wf() && receiver.allocation_wf() && receiver.default_data_ok() && receiver.iterator_wf() && receiver.history_view()==iterdata::initial() && receiver.model_input()==b@ && receiver.model_policy()==pol
            && receiver.doc.info.ranges_wf(b.len() as nat)
            && crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view()),_=>true},
            forall|e: Error,ev: crate::document_model::Events| crate::document_model::MWF_p(b@,pol.walk_view())
                ==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        crate::allocation_model::propagated_inflate_error(b@,pol,true,r),
    {
        Self::with_logged(b,pol).0
    }

    #[verifier::spinoff_prover]
    fn with_trace(b: &'a [u8],pol: Policy,trace: &mut Trace) -> (r: Result<Frames<'a>,TryError>)
        requires old(trace).wf(),old(trace).n==0,
        ensures iterdata::constructor_eq(b@,pol,r),final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),true,false,r,final(trace).log_view()),
            match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf() && receiver.initial_pixels() && receiver.initial_wf() && receiver.pixel_wf()
                && receiver.doc.info.ranges_wf(b.len() as nat)
                && crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view())
                && receiver.saved.is_none() && !receiver.state.done() && receiver.history@==iterdata::initial() && receiver.native_history(iterdata::initial()) && receiver.b==b && receiver.pol==pol
                && iterdata::default_success(b@,receiver.doc.view(),pol) && iterdata::from_walk(b@,receiver.doc.view(),pol)
                && final(trace).log_view()==iterlog::initial(receiver.raw@.len(),receiver.canvas@.len()),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,pol,true,r),
    {
        hide(Doc::wf);hide(crate::document_model::MWF_p);hide(inflate_verus::model::Inflate);
        let mut ev=Events {r1:false,r3:false,r4:false};
        let prepared=prepare_image(b,&pol,true,false,&mut ev,trace);
        proof {crate::allocation_model::prepared_exit::<Frames<'a>>(b@,pol.walk_view(),true,false,prepared_image_view(prepared),ev.view(),trace.log_view());}
        proof {crate::allocation_model::prepared_inflate_error::<Frames<'a>>(b@,pol,true,false,prepared_image_view(prepared),ev.view(),trace.log_view());}
        proof {iterdata::prepared_constructor::<Frames<'a>>(b@,pol,prepared_image_view(prepared),ev.view(),trace.log_view());}
        let (doc,mut raw,mut canvas)=match prepared {Ok(v)=>v,Err(e)=>return Err(e)};
        proof {iterdata::from_walk_rule(b@,iterdata::constructor_doc(doc.view()),pol);}
        proof {crate::limits::caps_ok();crate::icc_model::short_log(trace.log_view());}
        let ghost before=doc.view();
        let ghost output_len=canvas.len();
        let ghost default_z;
        {
            let mut src=IdatReader::for_doc(b,&doc);
            proof {default_z=src.concat();}
            let mut sink=Sink::image(&mut canvas,doc.ihdr.w,doc.ihdr.h,false);
            let ghost before_pixels=sink.pixels();
            let decoded=decode_image(&doc,b,&mut src,&mut raw,doc.ihdr.w,doc.ihdr.h,&pol,&mut ev,&mut sink);
            proof {assert(raster::document_data(b@,doc.view(),pol)==Some(decoded));}
            let ghost after_pixels=sink.pixels();
            sink.finish();
            proof {assert(canvas@==after_pixels);}
            match decoded {Err(e)=>{
                proof {iterdata::constructor_data_error::<Frames<'a>>(b@,doc.view(),pol,e);}
                return Err(TryError::Format(e));
            },Ok(())=>{}}
            proof {
                paint::image_from_progress(raster::inflated(default_z,paint::default_raster(doc.view(),pol),pol).out,
                    paint::default_raster(doc.view(),pol),crate::colour_model::from_doc(doc.view(),b@),before_pixels,after_pixels,false,
                    pol.palette_index==crate::policy::PaletteIndex::Black);
            }
        }
        assert(canvas.len()==output_len);
        if doc.actl.is_some() && !doc.info.default_is_frame {
            zero_all(&mut canvas);
            proof {paint::initial_transparent(b@,doc.view(),pol,canvas@);}
        }else{proof {paint::initial_decoded(b@,doc.view(),pol,canvas@,default_z);}}
        proof {iterdata::constructor_default(b@,doc.view(),pol);iterdata::from_walk_rule(b@,iterdata::constructor_doc(doc.view()),pol);}
        let receiver=Frames::from_decoded(b,pol,doc,raw,canvas,ev);
        assert(crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view()));
        proof {iterdata::default_intro(b@,receiver.doc.view(),pol);}
        assert(iterdata::default_success(b@,receiver.doc.view(),pol));
        assert(iterdata::from_walk(b@,receiver.doc.view(),pol));
        let result=Ok(receiver);
        proof {iterdata::constructor_ready(b@,result.get_Ok_0().doc.view(),pol,result);}
        result
    }
}

impl<'a> Frames<'a> {
    /// The next frame, `Ok(None)` after the last one, or the error that ends
    /// the sequence. Fused: after `Ok(None)` or any error every later call
    /// returns `Ok(None)` without allocating or decoding (D3).
    pub fn try_next(&mut self) -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires old(self).pixel_wf(),old(self).source_wf(),old(self).iterator_wf(),old(self).buffers_wf(),old(self).initial_wf(),old(self).allocation_wf(),old(self).default_data_ok(),
        ensures final(self).pixel_wf(),
            match r {
                Ok(Some(view))=>view.pixels()==final(self).canvas_pixels()
                    && canvas_model::step(old(self).model_input(),old(self).model_document(),old(self).model_policy(),old(self).history_view().returned,
                        old(self).canvas_pixels(),canvas_model::reference(old(self).canvas_trace(),old(self).history_view().returned),view.pixels())
                    && final(self).canvas_trace()==canvas_model::append(old(self).model_document(),old(self).canvas_trace(),view.pixels()),
                _=>final(self).canvas_trace()==old(self).canvas_trace(),
            },
            final(self).source_wf(),
            iterdata::call_eq(old(self).model_input(),old(self).model_document(),old(self).model_policy(),old(self).history_view(),iterdata::kind(r)),
            final(self).model_input()==old(self).model_input(),final(self).model_document()==old(self).model_document(),final(self).model_policy()==old(self).model_policy(),
            matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==old(self).history_view().returned+1,
            matches!(r,Ok(Some(_))) ==> final(self).history_view().phase==if old(self).model_document().actl.is_some() {iterdata::Phase::Active}else{iterdata::Phase::Iend},
            !matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==old(self).history_view().returned,
            matches!(r,Ok(None)) ==> final(self).is_done(),
            crate::alloc_guard::req_of_log(final(self).log_view())==crate::alloc_guard::req_of_log(old(self).log_view())+iterdata::request_p(old(self).model_input(),old(self).model_policy(),old(self).call_count()),final(self).iterator_wf(),final(self).call_count()==old(self).call_count()+1,
            matches!(r,Err(TryError::OutOfMemory(_))) ==> iterdata::is_oom(final(self).history_view())
                && iterdata::SavedAt_p(old(self).model_input(),old(self).model_policy())==Some(old(self).call_count()),
            !iterdata::is_oom(old(self).history_view()) && !matches!(r,Err(TryError::OutOfMemory(_))) ==> !iterdata::is_oom(final(self).history_view()),
            iterdata::is_oom(old(self).history_view()) ==> iterdata::is_oom(final(self).history_view()) && final(self).history_view().returned==old(self).history_view().returned && matches!(r,Ok(None)) && final(self).log_view()==old(self).log_view(),
            final(self).buffers_wf(),final(self).initial_wf(),final(self).allocation_wf(),final(self).default_data_ok(),final(self).info_value()==old(self).info_value(),
            final(self).log_view().len()<=3,
            final(self).log_view()==old(self).log_view()+if old(self).saved_call() && !old(self).saved_present()
                {seq![(AllocSite::Saved,old(self).canvas_len(),final(self).saved_present())]}else{Seq::<(AllocSite,nat,bool)>::empty()},
            r.is_err() ==> final(self).is_done(),
            old(self).is_done() ==> matches!(r,Ok(None)) && final(self).log_view()==old(self).log_view(),
            old(self).saved_present() ==> final(self).saved_present() && final(self).log_view()==old(self).log_view(),
            matches!(r,Err(TryError::OutOfMemory(_))) ==> crate::alloc_guard::sites::log_tied(r,final(self).log_view()),
    {
        self.next_logged()
    }

    proof fn source_context(&self)
        requires self.source_wf(),
        ensures self.doc.wf(self.b@),self.doc.ihdr.ct==3 ==> self.doc.plte.is_some(),self.b.len()<=crate::limits::MAX_INPUT,
            self.doc.info.frame_count==self.doc.frame_ranges@.len(),
    {
        reveal(Frames::source_wf);reveal(crate::frame_state::source_document);
    }

    proof fn saved_context(&self)
        requires self.source_wf(),self.buffers_wf(),
        ensures self.doc.info.frame_count==self.doc.frame_ranges@.len(),
            match self.state.index() {Some(k)=>self.saved.is_some()==crate::frame_transport_model::saved_before(self.doc.frame_ranges@,k),None=>true},
    {
        reveal(Frames::buffers_wf);
        self.source_context();
    }

    /// Exact pre-paint rectangle disposal and snapshot restore effect. The
    /// integrated MK relation to preceding source-frame pixels remains separate.
    #[verifier::spinoff_prover]
    fn dispose_and_save(&mut self,prev: crate::frame_state::Previous,current: (u32,u32,u32,u32),op: u8)
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).state.done(),old(self).restores_previous(prev),
            crate::frame_state::previous_fits(prev,old(self).doc.ihdr.view()),
            crate::frame_state::previous_fits(Some((current,op)),old(self).doc.ihdr.view()),
            crate::frame_state::previous_needs_saved(prev) || op==2 ==> old(self).saved.is_some(),
        ensures final(self).source_wf(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,final(self).log_view()==old(self).log_view(),
            old(self).allocation_wf() ==> final(self).allocation_wf(),final(self).buffers_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).state==old(self).state,final(self).raw==old(self).raw,
            prev.is_none() ==> final(self).canvas@==old(self).canvas@,
            final(self).saved.is_some()==old(self).saved.is_some(),
            composite::dispose_regions(old(self).canvas@,old(self).saved_pixels(),old(self).doc.ihdr.w as nat,previous_regions(prev))==Some(final(self).canvas@),
            op!=2 ==> final(self).saved==old(self).saved && final(self).snapshot==old(self).snapshot,
            op==2 ==> final(self).snapshot@==Some(final(self).canvas@),
            crate::frame_state::previous_needs_saved(prev) ==> old(self).snapshot@.is_some()
                && final(self).canvas@==old(self).snapshot@.unwrap(),
            op==2 ==> match final(self).saved {Some(s)=>composite::snapshot_effect(s@,final(self).canvas@,
                old(self).doc.ihdr.w as nat,composite::rect(current.0,current.1,current.2,current.3)),None=>false},
    {
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);crate::limits::caps_ok();}
        let w=self.doc.ihdr.w;let h=self.doc.ihdr.h;
        let ghost disposed=composite::dispose_regions(self.canvas@,self.saved_pixels(),w as nat,previous_regions(prev));
        if let Some(((px,py,pw,ph),dispose))=prev {
            proof {
                assert(4*(py as int+ph as int)*(w as int)<=self.canvas.len()) by (nonlinear_arith)
                    requires py as int+ph as int<=h,0<=w,self.canvas.len()==4*(w as int)*(h as int);
            }
            if dispose==1 {clear_rect(&mut self.canvas,w,px,py,pw,ph);}
            else if dispose==2 {
                proof {composite::saved_before_frame_restore(self.canvas@,self.saved_pixels(),self.snapshot@,w as nat,composite::rect(px,py,pw,ph));}
                if let Some(s)=&self.saved {copy_rect(&mut self.canvas,s,w,px,py,pw,ph);}
            }
        }
        proof {assert(disposed==Some(self.canvas@));}
        if op==2 {
            proof {self.snapshot=Ghost(Some(self.canvas@));}
            let (x,y,rw,rh)=current;
            proof {
                assert(4*(y as int+rh as int)*(w as int)<=self.canvas.len()) by (nonlinear_arith)
                    requires y as int+rh as int<=h,0<=w,self.canvas.len()==4*(w as int)*(h as int);
            }
            if let Some(s)=self.saved.as_mut() {
                let ghost prior=s@;
                copy_rect(s,&self.canvas,w,x,y,rw,rh);
                proof {
                    composite::matching_restore(self.canvas@,self.canvas@,prior,w as nat,composite::rect(x,y,rw,rh));
                    assert(composite::copy_rect(self.canvas@,self.canvas@,w as nat,composite::rect(x,y,rw,rh))=~=self.canvas@);
                }
            }
        }
    }

    /// Keep the independent IDAT source/normalization equation out of the
    /// query that also handles fdAT inflation and the Saved invariant.
    proof fn reused_values(&self,job: crate::frame_state::FrameJob<'a>,before: Seq<u8>)
        requires self.source_wf(),self.buffers_wf(),job.wf(self.b@,self.doc),job.reader.is_none(),
            paint::initial_canvas(self.b@,self.doc.view(),self.pol,before),
            self.canvas@==if job.control.blend==1 {composite::zero_alpha(before)}else{before},
        ensures composite::drawing_frame(before,self.canvas@,
                (2,before.len(),self.doc.ihdr.w as nat,job.control.x as nat,job.control.y as nat,job.control.blend==1),
                job.control.w as nat,job.control.h as nat),
            match crate::document_model::frame_stream(self.b@,self.doc.view(),job.index as nat) {
                Some(z)=>paint::progress(raster::inflated(z,crate::image::raster_context(self.doc,job.control.w,job.control.h,self.pol),self.pol).out,
                    crate::image::raster_context(self.doc,job.control.w,job.control.h,self.pol),crate::colour_model::from_doc(self.doc.view(),self.b@),
                    (2,before.len(),self.doc.ihdr.w as nat,job.control.x as nat,job.control.y as nat,job.control.blend==1),
                    Seq::new(before.len(),|j: int| 0u8),self.canvas@,
                    raster::count(crate::image::raster_context(self.doc,job.control.w,job.control.h,self.pol)),0,
                    self.pol.palette_index==crate::policy::PaletteIndex::Black),
                None=>false,
            },
    {
        hide(raster::inflated);hide(Doc::wf);hide(crate::document_model::frame_stream);
        reveal(Frames::source_wf);reveal(Frames::buffers_wf);
        job.storage_context(self.b@,self.doc);job.reused_source(self.b@,self.doc);
        self.doc.info.animation_view();
        paint::initial_default_source(self.b@,self.doc.view(),self.pol,before);
        let z=crate::document_model::default_stream(self.b@,self.doc.view()).unwrap();
        paint::reuse_progress(raster::inflated(z,paint::default_raster(self.doc.view(),self.pol),self.pol).out,
            paint::default_raster(self.doc.view(),self.pol),crate::colour_model::from_doc(self.doc.view(),self.b@),before,
            job.control.blend==1,self.pol.palette_index==crate::policy::PaletteIndex::Black);
        composite::full_drawing_frame(before,self.canvas@,self.doc.ihdr.w as nat,self.doc.ihdr.h as nat,job.control.blend==1);
    }

    /// Weak selected-image continuation: preserve layout and source metadata.
    #[verifier::spinoff_prover]
    fn draw_selected(&mut self,job: crate::frame_state::FrameJob<'a>) -> (r: Result<Events,TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).state.done(),job.wf(old(self).b@,old(self).doc),
            job.effective_dispose==2 ==> old(self).snapshot@==Some(old(self).canvas@)
                && match old(self).saved {Some(v)=>composite::snapshot_effect(v@,old(self).canvas@,old(self).doc.ihdr.w as nat,
                    composite::rect(job.control.x,job.control.y,job.control.w,job.control.h)),None=>false},
            job.effective_dispose==2 ==> old(self).saved.is_some(),
            job.reader.is_none() ==> iterdata::frame_ok(old(self).b@,old(self).doc.view(),old(self).pol,job.index as nat),
            job.reader.is_none() ==> old(self).initial_pixels(),
        ensures final(self).source_wf(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,iterdata::result_eq(old(self).b@,old(self).doc.view(),old(self).pol,job.index as nat,r),final(self).log_view()==old(self).log_view(),
            old(self).allocation_wf() ==> final(self).allocation_wf(),final(self).buffers_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).state==old(self).state,final(self).saved==old(self).saved,final(self).snapshot==old(self).snapshot,
            composite::drawing_frame(old(self).canvas@,final(self).canvas@,
                (2,old(self).canvas@.len(),old(self).doc.ihdr.w as nat,job.control.x as nat,job.control.y as nat,job.control.blend==1),
                job.control.w as nat,job.control.h as nat),
            final(self).restores_previous(Some(((job.control.x,job.control.y,job.control.w,job.control.h),job.effective_dispose))),
            r.is_ok() ==> match crate::document_model::frame_stream(old(self).b@,old(self).doc.view(),job.index as nat) {
                Some(z)=>paint::progress(raster::inflated(z,crate::image::raster_context(old(self).doc,job.control.w,job.control.h,old(self).pol),old(self).pol).out,
                    crate::image::raster_context(old(self).doc,job.control.w,job.control.h,old(self).pol),crate::colour_model::from_doc(old(self).doc.view(),old(self).b@),
                    (2,old(self).canvas@.len(),old(self).doc.ihdr.w as nat,job.control.x as nat,job.control.y as nat,job.control.blend==1),
                    if job.reader.is_none() {Seq::new(old(self).canvas@.len(),|j: int| 0u8)}else{old(self).canvas@},
                    final(self).canvas@,raster::count(crate::image::raster_context(old(self).doc,job.control.w,job.control.h,old(self).pol)),0,
                    old(self).pol.palette_index==crate::policy::PaletteIndex::Black),
                None=>false,
            },
            r.is_err() ==> matches!(r,Err(TryError::Format(_))),
    {
        hide(raster::data);hide(raster::completed);hide(raster::inflated);
        hide(Doc::wf);hide(crate::document_model::frame_stream);
        hide(crate::frame_transport_model::control_record);hide(crate::frame_transport_model::cursor_for);
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);reveal(crate::frame_state::FrameJob::wf);crate::limits::caps_ok();self.source_context();}
        proof {job.data_context(self.b@,self.doc,self.pol);}
        let ghost before=self.canvas@;
        let w=self.doc.ihdr.w;let h=self.doc.ihdr.h;let f=job.control;
        let ghost geometry=(2nat,before.len(),w as nat,f.x as nat,f.y as nat,f.blend==1);
                let mut fev=self.base;
                if job.reader.is_none() {
                    normalize_reused(&mut self.canvas,f.blend==1);
                    proof {self.reused_values(job,before);}
                }else {
                    let mut src=job.reader.unwrap();
                    proof {
                        assert(4*(f.y as int+f.h as int)*(w as int)<=self.canvas.len()) by (nonlinear_arith)
                            requires f.y as int+f.h as int<=h,0<=w,self.canvas.len()==4*(w as int)*(h as int);
                    }
                    let mut sink=Sink::canvas(&mut self.canvas,w,f.x,f.y,f.w,f.h,f.blend==1);
                    proof {crate::layout_model::raw_dimensions(self.doc.ihdr.interlace,crate::layout_model::bits(self.doc.ihdr.view()),
                        f.w as nat,f.h as nat,w as nat,h as nat);}
                    let decoded=decode_image(&self.doc,self.b,&mut src,&mut self.raw,f.w,f.h,&self.pol,&mut fev,&mut sink);
                    proof {assert(iterdata::frame_data(self.b@,self.doc.view(),self.pol,job.index as nat)==Some(decoded));}
                    let ghost after=sink.pixels();
                    sink.finish();
                    proof {assert(self.canvas@==after);}
                    match decoded {Err(e)=>{
                        proof {
                            if job.effective_dispose==2 {composite::drawing_restores_before(before,self.canvas@,self.saved_pixels().unwrap(),w as nat,composite::rect(f.x,f.y,f.w,f.h),f.blend==1);}
                        }
                        return Err(TryError::Format(e));
                    },Ok(())=>{}}
                }
        proof {
            if job.effective_dispose==2 {composite::drawing_restores_before(before,self.canvas@,self.saved_pixels().unwrap(),w as nat,composite::rect(f.x,f.y,f.w,f.h),f.blend==1);}
        }
        Ok(fev)
    }

    /// Paint/dispose state composition is separate from the allocation-log query.
    fn finish_job(&mut self,job: crate::frame_state::FrameJob<'a>) -> (r: Result<Events,TryError>)
        requires old(self).source_wf(),old(self).allocation_wf(),crate::alloc_guard::all_ok(old(self).log_view()),old(self).buffers_wf(),old(self).state.done(),job.wf(old(self).b@,old(self).doc),
            old(self).restores_previous(job.previous),
            job.reader.is_none() ==> iterdata::frame_ok(old(self).b@,old(self).doc.view(),old(self).pol,job.index as nat),
            job.reader.is_none() ==> old(self).initial_pixels() && job.previous.is_none(),
            crate::frame_state::previous_needs_saved(job.previous) || job.effective_dispose==2 ==> old(self).saved.is_some(),
            old(self).saved.is_some()==crate::frame_transport_model::saved_before(old(self).doc.frame_ranges@,job.index as nat+1),
        ensures final(self).source_wf(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,iterdata::result_eq(old(self).b@,old(self).doc.view(),old(self).pol,job.index as nat,r),final(self).log_view()==old(self).log_view(),
            old(self).allocation_wf() ==> final(self).allocation_wf(),final(self).buffers_wf(),final(self).initial_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).saved.is_some()==old(self).saved.is_some(),
            r.is_err() ==> final(self).state.done() && matches!(r,Err(TryError::Format(_))),
            r.is_ok() ==> final(self).state==job.resume,
            job.effective_dispose==2 ==> final(self).snapshot@==Some(paint::disposed_reference(old(self).canvas@,old(self).snapshot@,
                old(self).doc.ihdr.w as nat,previous_regions(job.previous))),

            r.is_ok() ==> match crate::document_model::frame_stream(old(self).b@,old(self).doc.view(),job.index as nat) {
                Some(z)=>paint::MK(z,crate::image::raster_context(old(self).doc,job.control.w,job.control.h,old(self).pol),
                    crate::colour_model::from_doc(old(self).doc.view(),old(self).b@),old(self).pol,old(self).doc.ihdr.w as nat,
                    composite::rect(job.control.x,job.control.y,job.control.w,job.control.h),job.control.blend==1,
                    previous_regions(job.previous),old(self).snapshot@,old(self).canvas@,final(self).canvas@,job.reader.is_none()),
                None=>false,
            },
    {
        hide(Doc::wf);hide(crate::frame_transport_model::control_record);
        proof {job.storage_context(self.b@,self.doc);}
        let f=job.control;let resume=job.resume;
        let ghost reuse=job.reader.is_none();
        let ghost original=self.canvas@;
        let ghost reference=self.snapshot@;
        proof {paint::reference_disposal(original,self.saved_pixels(),reference,self.doc.ihdr.w as nat,previous_regions(job.previous));}
        self.dispose_and_save(job.previous,(f.x,f.y,f.w,f.h),job.effective_dispose);
        let events=match self.draw_selected(job) {Ok(events)=>events,Err(e)=>return Err(e)};
        proof {
            if reuse {
                let zero=Seq::new(original.len(),|j: int| 0u8);
                composite::full_drawing_frame(zero,self.canvas@,self.doc.ihdr.w as nat,self.doc.ihdr.h as nat,f.blend==1);
            }
        }
        proof {
            let z=crate::document_model::frame_stream(self.b@,self.doc.view(),job.index as nat).unwrap();
            paint::mk_intro(z,crate::image::raster_context(self.doc,f.w,f.h,self.pol),crate::colour_model::from_doc(self.doc.view(),self.b@),
                self.pol,self.doc.ihdr.w as nat,composite::rect(f.x,f.y,f.w,f.h),f.blend==1,previous_regions(job.previous),reference,
                original,self.canvas@,reuse);
        }
        self.commit_source(resume);
        Ok(events)
    }

    /// Keep per-position paint-model terms out of the cumulative-log query.
    fn finish_iteration(&mut self,job: crate::frame_state::FrameJob<'a>) -> (r: Result<Events,TryError>)
        requires old(self).source_wf(),old(self).allocation_wf(),crate::alloc_guard::all_ok(old(self).log_view()),old(self).buffers_wf(),old(self).state.done(),job.wf(old(self).b@,old(self).doc),
            old(self).restores_previous(job.previous),
            job.reader.is_none() ==> iterdata::frame_ok(old(self).b@,old(self).doc.view(),old(self).pol,job.index as nat),
            job.reader.is_none() ==> old(self).initial_pixels() && job.previous.is_none(),
            crate::frame_state::previous_needs_saved(job.previous) || job.effective_dispose==2 ==> old(self).saved.is_some(),
            old(self).saved.is_some()==crate::frame_transport_model::saved_before(old(self).doc.frame_ranges@,job.index as nat+1),
        ensures final(self).source_wf(),final(self).raw@.len()==old(self).raw@.len(),final(self).canvas@.len()==old(self).canvas@.len(),final(self).history==old(self).history,final(self).pixel_history==old(self).pixel_history,iterdata::result_eq(old(self).b@,old(self).doc.view(),old(self).pol,job.index as nat,r),final(self).log_view()==old(self).log_view(),
            old(self).allocation_wf() ==> final(self).allocation_wf(),final(self).buffers_wf(),final(self).initial_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).saved.is_some()==old(self).saved.is_some(),
            r.is_err() ==> final(self).state.done() && matches!(r,Err(TryError::Format(_))),
            r.is_ok() ==> final(self).state==job.resume,
            job.effective_dispose==2 ==> final(self).snapshot@==Some(paint::disposed_reference(old(self).canvas@,old(self).snapshot@,
                old(self).doc.ihdr.w as nat,previous_regions(job.previous))),
            r.is_ok() ==> match crate::document_model::frame_stream(old(self).b@,old(self).doc.view(),job.index as nat) {
                Some(z)=>paint::MK(z,crate::image::raster_context(old(self).doc,job.control.w,job.control.h,old(self).pol),
                    crate::colour_model::from_doc(old(self).doc.view(),old(self).b@),old(self).pol,old(self).doc.ihdr.w as nat,
                    composite::rect(job.control.x,job.control.y,job.control.w,job.control.h),job.control.blend==1,
                    previous_regions(job.previous),old(self).snapshot@,old(self).canvas@,final(self).canvas@,job.reader.is_none()),
                None=>false,
            },
    {
        self.finish_job(job)
    }

    #[verifier::spinoff_prover]
    fn selected_logged(&mut self,job: crate::frame_state::FrameJob<'a>,Ghost(prior_receiver): Ghost<Frames<'a>>)
        -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires prior_receiver.pixel_wf(),old(self).source_wf(),old(self).buffers_wf(),old(self).allocation_wf(),old(self).state.done(),
            prior_receiver.source_wf(),prior_receiver.buffers_wf(),prior_receiver.initial_wf(),prior_receiver.allocation_wf(),prior_receiver.default_data_ok(),prior_receiver.iterator_wf(),
            old(self).doc==prior_receiver.doc,old(self).b==prior_receiver.b,old(self).pol==prior_receiver.pol,old(self).base==prior_receiver.base,
            old(self).raw==prior_receiver.raw,old(self).canvas==prior_receiver.canvas,old(self).saved==prior_receiver.saved,old(self).snapshot==prior_receiver.snapshot,
            old(self).history==prior_receiver.history,old(self).pixel_history==prior_receiver.pixel_history,old(self).log_view()==prior_receiver.log_view(),
            job.wf(old(self).b@,old(self).doc),prior_receiver.state.index()==Some(job.index as nat),job.previous==prior_receiver.state.previous(),
            old(self).restores_previous(job.previous),crate::frame_state::previous_needs_saved(job.previous) ==> old(self).saved.is_some(),
        ensures final(self).pixel_wf(),
            match r {
                Ok(Some(view))=>view.pixels()==final(self).canvas_pixels()
                    && canvas_model::step(prior_receiver.model_input(),prior_receiver.model_document(),prior_receiver.model_policy(),prior_receiver.history_view().returned,
                        prior_receiver.canvas_pixels(),canvas_model::reference(prior_receiver.canvas_trace(),prior_receiver.history_view().returned),view.pixels())
                    && final(self).canvas_trace()==canvas_model::append(prior_receiver.model_document(),prior_receiver.canvas_trace(),view.pixels()),
                _=>final(self).canvas_trace()==prior_receiver.canvas_trace(),
            },
            final(self).source_wf(),
            iterdata::call_eq(prior_receiver.model_input(),prior_receiver.model_document(),prior_receiver.model_policy(),prior_receiver.history_view(),iterdata::kind(r)),
            final(self).model_input()==prior_receiver.model_input(),final(self).model_document()==prior_receiver.model_document(),final(self).model_policy()==prior_receiver.model_policy(),
            matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==prior_receiver.history_view().returned+1,
            matches!(r,Ok(Some(_))) ==> final(self).history_view().phase==if prior_receiver.model_document().actl.is_some() {iterdata::Phase::Active}else{iterdata::Phase::Iend},
            !matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==prior_receiver.history_view().returned,
            matches!(r,Ok(None)) ==> final(self).is_done(),
            r.is_ok() ==> matches!(r,Ok(Some(_))),
            crate::alloc_guard::req_of_log(final(self).log_view())==crate::alloc_guard::req_of_log(prior_receiver.log_view())+iterdata::request_p(prior_receiver.model_input(),prior_receiver.model_policy(),prior_receiver.call_count()),final(self).iterator_wf(),final(self).call_count()==prior_receiver.call_count()+1,
            matches!(r,Err(TryError::OutOfMemory(_))) ==> iterdata::is_oom(final(self).history_view())
                && iterdata::SavedAt_p(prior_receiver.model_input(),prior_receiver.model_policy())==Some(prior_receiver.call_count()),
            !iterdata::is_oom(prior_receiver.history_view()) && !matches!(r,Err(TryError::OutOfMemory(_))) ==> !iterdata::is_oom(final(self).history_view()),
            iterdata::is_oom(prior_receiver.history_view()) ==> iterdata::is_oom(final(self).history_view()) && final(self).history_view().returned==prior_receiver.history_view().returned && matches!(r,Ok(None)) && final(self).log_view()==prior_receiver.log_view(),
            final(self).buffers_wf(),final(self).initial_wf(),final(self).allocation_wf(),
            final(self).doc==prior_receiver.doc,final(self).b==prior_receiver.b,final(self).pol==prior_receiver.pol,final(self).base==prior_receiver.base,
            final(self).log_view().len()<=3,
            prior_receiver.saved.is_some() ==> final(self).saved.is_some() && final(self).log_view()==prior_receiver.log_view(),
            r.is_err() ==> final(self).state.done(),
            matches!(r,Ok(None)) ==> final(self).canvas==prior_receiver.canvas && final(self).saved==prior_receiver.saved,
            prior_receiver.state.done() ==> matches!(r,Ok(None)) && final(self).log_view()==prior_receiver.log_view(),
            r.is_err() && matches!(r,Err(TryError::OutOfMemory(_))) ==> crate::alloc_guard::sites::log_tied(r,final(self).log_view()),
            final(self).log_view()==prior_receiver.log_view()+if prior_receiver.saved_call() && prior_receiver.saved.is_none()
                {seq![(AllocSite::Saved,prior_receiver.canvas@.len(),final(self).saved.is_some())]}
                else {Seq::<(AllocSite,nat,bool)>::empty()},
    {
        hide(Frames::buffers_wf);hide(Frames::allocation_wf);
        hide(iterdata::is_saved_at);hide(iterdata::prefix);hide(iterdata::frame_ok);hide(raster::document_data);
        let ghost prior_history=prior_receiver.history@;
        let ghost prior_state=prior_receiver.state;
        let ghost prior_saved=prior_receiver.saved.is_some();
        let ghost wanted=prior_receiver.saved_call();
        let ghost before_log=prior_receiver.log_view();
        proof {
            prior_receiver.execution_context();prior_receiver.request_context();prior_receiver.saved_context();prior_receiver.initial_context();prior_receiver.allocation_context();
            iterdata::history_context(self.b@,self.doc.view(),self.pol,prior_history);
        }

                proof {
                    job.storage_context(self.b@,self.doc);job.reuse_data_context(self.b@,self.doc,self.pol);
                    crate::frame_transport_model::saved_before_step(self.doc.frame_ranges@,job.index as nat);
                    assert(prior_state.index()==Some(job.index as nat));
                    assert(prior_saved==crate::frame_transport_model::saved_before(self.doc.frame_ranges@,job.index as nat));
                }
                let k=job.index;let f=job.control;let resume=job.resume;
                proof {
                    assert(!prior_state.done());
                    if !prior_saved {assert(before_log.len()==2);}
                }
                proof {
                    assert(wanted==(k>0 && f.dispose==2));
                    if job.reader.is_none() {assert(k==0);assert(job.previous.is_none());assert(self.initial_pixels());}
                }
                match self.ensure_saved(k>0 && f.dispose==2) {Ok(())=>{},Err(e)=>{
                    let ghost next_history=iterdata::History {calls:prior_history.calls+1,returned:prior_history.returned,phase:iterdata::Phase::Oom};
                proof {
                    iterdata::history_oom(self.b@,self.doc.view(),self.pol,prior_history);
                    let ghost before_history_update=*self;
                    self.history=Ghost(next_history);Frames::history_preserves_state(before_history_update,*self);Frames::compose_call(prior_receiver,*self,iterdata::Call::Oom);Frames::pixels_idle(prior_receiver,*self);
                }
                return Err(e);
                }}
                proof {self.allocation_context();assert(crate::alloc_guard::all_ok(self.log_view()));}
                self.completed_selected(job,Ghost(prior_receiver))
    }

    #[verifier::spinoff_prover]
    fn completed_selected(&mut self,job: crate::frame_state::FrameJob<'a>,Ghost(prior_receiver): Ghost<Frames<'a>>)
        -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires prior_receiver.pixel_wf(),old(self).source_wf(),old(self).buffers_wf(),old(self).allocation_wf(),old(self).state.done(),
            crate::alloc_guard::all_ok(old(self).log_view()),
            prior_receiver.source_wf(),prior_receiver.buffers_wf(),prior_receiver.initial_wf(),prior_receiver.allocation_wf(),prior_receiver.default_data_ok(),prior_receiver.iterator_wf(),
            old(self).doc==prior_receiver.doc,old(self).b==prior_receiver.b,old(self).pol==prior_receiver.pol,old(self).base==prior_receiver.base,
            old(self).raw==prior_receiver.raw,old(self).canvas==prior_receiver.canvas,old(self).snapshot==prior_receiver.snapshot,
            old(self).history==prior_receiver.history,old(self).pixel_history==prior_receiver.pixel_history,
            old(self).log_view()==prior_receiver.log_view()+if prior_receiver.saved_call() && prior_receiver.saved.is_none()
                {seq![(AllocSite::Saved,prior_receiver.canvas@.len(),true)]}else{Seq::<(AllocSite,nat,bool)>::empty()},
            old(self).saved.is_some()==(prior_receiver.saved.is_some() || prior_receiver.saved_call()),
            job.wf(old(self).b@,old(self).doc),prior_receiver.state.index()==Some(job.index as nat),job.previous==prior_receiver.state.previous(),
            old(self).restores_previous(job.previous),crate::frame_state::previous_needs_saved(job.previous) || job.effective_dispose==2 ==> old(self).saved.is_some(),
            old(self).saved.is_some()==crate::frame_transport_model::saved_before(old(self).doc.frame_ranges@,job.index as nat+1),
            job.reader.is_none() ==> old(self).initial_pixels(),
        ensures final(self).pixel_wf(),
            match r {
                Ok(Some(view))=>view.pixels()==final(self).canvas_pixels()
                    && canvas_model::step(prior_receiver.model_input(),prior_receiver.model_document(),prior_receiver.model_policy(),prior_receiver.history_view().returned,
                        prior_receiver.canvas_pixels(),canvas_model::reference(prior_receiver.canvas_trace(),prior_receiver.history_view().returned),view.pixels())
                    && final(self).canvas_trace()==canvas_model::append(prior_receiver.model_document(),prior_receiver.canvas_trace(),view.pixels()),
                _=>final(self).canvas_trace()==prior_receiver.canvas_trace(),
            },
            final(self).source_wf(),
            iterdata::call_eq(prior_receiver.model_input(),prior_receiver.model_document(),prior_receiver.model_policy(),prior_receiver.history_view(),iterdata::kind(r)),
            final(self).model_input()==prior_receiver.model_input(),final(self).model_document()==prior_receiver.model_document(),final(self).model_policy()==prior_receiver.model_policy(),
            matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==prior_receiver.history_view().returned+1,
            matches!(r,Ok(Some(_))) ==> final(self).history_view().phase==if prior_receiver.model_document().actl.is_some() {iterdata::Phase::Active}else{iterdata::Phase::Iend},
            !matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==prior_receiver.history_view().returned,
            matches!(r,Ok(None)) ==> final(self).is_done(),
            r.is_ok() ==> matches!(r,Ok(Some(_))),
            crate::alloc_guard::req_of_log(final(self).log_view())==crate::alloc_guard::req_of_log(prior_receiver.log_view())+iterdata::request_p(prior_receiver.model_input(),prior_receiver.model_policy(),prior_receiver.call_count()),final(self).iterator_wf(),final(self).call_count()==prior_receiver.call_count()+1,
            matches!(r,Err(TryError::OutOfMemory(_))) ==> iterdata::is_oom(final(self).history_view())
                && iterdata::SavedAt_p(prior_receiver.model_input(),prior_receiver.model_policy())==Some(prior_receiver.call_count()),
            !iterdata::is_oom(prior_receiver.history_view()) && !matches!(r,Err(TryError::OutOfMemory(_))) ==> !iterdata::is_oom(final(self).history_view()),
            iterdata::is_oom(prior_receiver.history_view()) ==> iterdata::is_oom(final(self).history_view()) && final(self).history_view().returned==prior_receiver.history_view().returned && matches!(r,Ok(None)) && final(self).log_view()==prior_receiver.log_view(),
            final(self).buffers_wf(),final(self).initial_wf(),final(self).allocation_wf(),
            final(self).doc==prior_receiver.doc,final(self).b==prior_receiver.b,final(self).pol==prior_receiver.pol,final(self).base==prior_receiver.base,
            final(self).log_view().len()<=3,
            prior_receiver.saved.is_some() ==> final(self).saved.is_some() && final(self).log_view()==prior_receiver.log_view(),
            r.is_err() ==> final(self).state.done(),
            matches!(r,Ok(None)) ==> final(self).canvas==prior_receiver.canvas && final(self).saved==prior_receiver.saved,
            prior_receiver.state.done() ==> matches!(r,Ok(None)) && final(self).log_view()==prior_receiver.log_view(),
            r.is_err() && matches!(r,Err(TryError::OutOfMemory(_))) ==> crate::alloc_guard::sites::log_tied(r,final(self).log_view()),
            final(self).log_view()==prior_receiver.log_view()+if prior_receiver.saved_call() && prior_receiver.saved.is_none()
                {seq![(AllocSite::Saved,prior_receiver.canvas@.len(),final(self).saved.is_some())]}
                else {Seq::<(AllocSite,nat,bool)>::empty()},
    {
        hide(Frames::buffers_wf);hide(Frames::allocation_wf);hide(iterdata::result_eq);
        hide(iterdata::is_saved_at);hide(iterdata::prefix);hide(iterdata::frame_ok);hide(raster::document_data);
        let ghost prior_history=prior_receiver.history@;
        let f=job.control;
        let ghost prior_pixels=prior_receiver.canvas@;
        let ghost prior_reference=prior_receiver.snapshot@;
        proof {
            prior_receiver.pixel_context();job.canvas_context(self.b@,self.doc,self.pol);
            prior_receiver.execution_context();prior_receiver.request_context();prior_receiver.saved_context();prior_receiver.initial_context();prior_receiver.allocation_context();
            iterdata::history_context(self.b@,self.doc.view(),self.pol,prior_history);
            job.storage_context(self.b@,self.doc);job.reuse_data_context(self.b@,self.doc,self.pol);
        }
                let painted=self.finish_iteration(job);
                proof {iterdata::result_context(self.b@,self.doc.view(),self.pol,job.index as nat,painted);}
                let fev=match painted {Ok(ev)=>ev,Err(e)=>{
                    let ghost fault=e.arrow_Format_0();
                    let ghost next_history=iterdata::History {calls:prior_history.calls+1,returned:prior_history.returned,phase:iterdata::Phase::Format(fault)};
                proof {
                    iterdata::history_format(self.b@,self.doc.view(),self.pol,prior_history,fault);
                    let ghost before_history_update=*self;
                    self.history=Ghost(next_history);Frames::history_preserves_state(before_history_update,*self);Frames::compose_call(prior_receiver,*self,iterdata::Call::Format(fault));Frames::pixels_idle(prior_receiver,*self);
                }
                return Err(e);
                }};
                proof {
                    let z=crate::document_model::frame_stream(self.b@,self.doc.view(),job.index as nat).unwrap();
                    canvas_model::step_intro(self.b@,self.doc.view(),self.pol,job.index as nat,z,prior_pixels,prior_reference,self.canvas@);
                    canvas_model::reference_irrelevant(self.b@,self.doc.view(),self.pol,job.index as nat,prior_pixels,prior_reference,
                        canvas_model::reference(prior_receiver.pixel_history@,job.index as nat),self.canvas@);
                }
                let ghost next_history=iterdata::History {calls:prior_history.calls+1,returned:prior_history.returned+1,phase:iterdata::Phase::Active};
                proof {
                    iterdata::history_success(self.b@,self.doc.view(),self.pol,prior_history,false);
                    let ghost before_history_update=*self;
                    self.history=Ghost(next_history);Frames::history_preserves_state(before_history_update,*self);Frames::compose_call(prior_receiver,*self,iterdata::Call::Frame);
                    let ghost before_pixels_update=*self;
                    self.pixel_history=Ghost(canvas_model::append(self.doc.view(),prior_receiver.pixel_history@,self.canvas@));
                    Frames::history_preserves_state(before_pixels_update,*self);
                    Frames::pixels_preserve_iterator(before_pixels_update,*self);
                    Frames::pixels_returned(prior_receiver,*self);
                }
                Ok(Some(FrameView {rgba:&self.canvas,rect:(f.x,f.y,f.w,f.h),delay:Some((f.delay_num,f.delay_den)),
                    dispose:f.dispose,blend:f.blend,recovered:fev.any(),events:fev}))
    }

    #[verifier::spinoff_prover]
    fn terminal_logged(&mut self,single: bool,Ghost(prior_receiver): Ghost<Frames<'a>>)
        -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires prior_receiver.pixel_wf(),old(self).source_wf(),old(self).buffers_wf(),old(self).allocation_wf(),old(self).state.done(),
            prior_receiver.source_wf(),prior_receiver.buffers_wf(),prior_receiver.initial_wf(),prior_receiver.allocation_wf(),prior_receiver.default_data_ok(),prior_receiver.iterator_wf(),
            old(self).doc==prior_receiver.doc,old(self).b==prior_receiver.b,old(self).pol==prior_receiver.pol,old(self).base==prior_receiver.base,
            old(self).raw==prior_receiver.raw,old(self).canvas==prior_receiver.canvas,old(self).saved==prior_receiver.saved,old(self).snapshot==prior_receiver.snapshot,
            old(self).history==prior_receiver.history,old(self).pixel_history==prior_receiver.pixel_history,old(self).log_view()==prior_receiver.log_view(),
            single ==> prior_receiver.state.single(),
            !single ==> prior_receiver.state.done() || prior_receiver.state.index()==Some(prior_receiver.doc.info.frame_count as nat),
        ensures final(self).pixel_wf(),
            match r {
                Ok(Some(view))=>view.pixels()==final(self).canvas_pixels()
                    && canvas_model::step(prior_receiver.model_input(),prior_receiver.model_document(),prior_receiver.model_policy(),prior_receiver.history_view().returned,
                        prior_receiver.canvas_pixels(),canvas_model::reference(prior_receiver.canvas_trace(),prior_receiver.history_view().returned),view.pixels())
                    && final(self).canvas_trace()==canvas_model::append(prior_receiver.model_document(),prior_receiver.canvas_trace(),view.pixels()),
                _=>final(self).canvas_trace()==prior_receiver.canvas_trace(),
            },
            final(self).source_wf(),
            iterdata::call_eq(prior_receiver.model_input(),prior_receiver.model_document(),prior_receiver.model_policy(),prior_receiver.history_view(),iterdata::kind(r)),
            final(self).model_input()==prior_receiver.model_input(),final(self).model_document()==prior_receiver.model_document(),final(self).model_policy()==prior_receiver.model_policy(),
            matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==prior_receiver.history_view().returned+1,
            matches!(r,Ok(Some(_))) ==> final(self).history_view().phase==if prior_receiver.model_document().actl.is_some() {iterdata::Phase::Active}else{iterdata::Phase::Iend},
            !matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==prior_receiver.history_view().returned,
            matches!(r,Ok(None)) ==> final(self).is_done(),
            crate::alloc_guard::req_of_log(final(self).log_view())==crate::alloc_guard::req_of_log(prior_receiver.log_view())+iterdata::request_p(prior_receiver.model_input(),prior_receiver.model_policy(),prior_receiver.call_count()),final(self).iterator_wf(),final(self).call_count()==prior_receiver.call_count()+1,
            matches!(r,Err(TryError::OutOfMemory(_))) ==> iterdata::is_oom(final(self).history_view())
                && iterdata::SavedAt_p(prior_receiver.model_input(),prior_receiver.model_policy())==Some(prior_receiver.call_count()),
            !iterdata::is_oom(prior_receiver.history_view()) && !matches!(r,Err(TryError::OutOfMemory(_))) ==> !iterdata::is_oom(final(self).history_view()),
            iterdata::is_oom(prior_receiver.history_view()) ==> iterdata::is_oom(final(self).history_view()) && final(self).history_view().returned==prior_receiver.history_view().returned && matches!(r,Ok(None)) && final(self).log_view()==prior_receiver.log_view(),
            final(self).buffers_wf(),final(self).initial_wf(),final(self).allocation_wf(),
            final(self).doc==prior_receiver.doc,final(self).b==prior_receiver.b,final(self).pol==prior_receiver.pol,final(self).base==prior_receiver.base,
            final(self).log_view().len()<=3,
            prior_receiver.saved.is_some() ==> final(self).saved.is_some() && final(self).log_view()==prior_receiver.log_view(),
            r.is_err() ==> final(self).state.done(),
            matches!(r,Ok(None)) ==> final(self).canvas==prior_receiver.canvas && final(self).saved==prior_receiver.saved,
            prior_receiver.state.done() ==> matches!(r,Ok(None)) && final(self).log_view()==prior_receiver.log_view(),
            r.is_err() && matches!(r,Err(TryError::OutOfMemory(_))) ==> crate::alloc_guard::sites::log_tied(r,final(self).log_view()),
            final(self).log_view()==prior_receiver.log_view()+if prior_receiver.saved_call() && prior_receiver.saved.is_none()
                {seq![(AllocSite::Saved,prior_receiver.canvas@.len(),final(self).saved.is_some())]}
                else {Seq::<(AllocSite,nat,bool)>::empty()},
    {
        hide(Frames::buffers_wf);hide(Frames::allocation_wf);
        hide(iterdata::is_saved_at);hide(iterdata::prefix);hide(iterdata::frame_ok);hide(raster::document_data);
        let ghost prior_history=prior_receiver.history@;
        let ghost prior_state=prior_receiver.state;
        let ghost prior_saved=prior_receiver.saved.is_some();
        let ghost wanted=prior_receiver.saved_call();
        let ghost before_log=prior_receiver.log_view();
        proof {
            prior_receiver.pixel_context();
            prior_receiver.execution_context();prior_receiver.request_context();prior_receiver.saved_context();prior_receiver.initial_context();prior_receiver.allocation_context();
            iterdata::history_context(self.b@,self.doc.view(),self.pol,prior_history);
        }
        let w=self.doc.ihdr.w;let h=self.doc.ihdr.h;
        if single {
                proof {
                    iterdata::static_ready(self.b@,self.doc.view(),self.pol);
                    self.canvas_geometry();
                    canvas_model::static_step(self.b@,self.doc.view(),self.pol,self.canvas@);
                }
                let ghost next_history=iterdata::History {calls:prior_history.calls+1,returned:prior_history.returned+1,phase:iterdata::Phase::Iend};
                proof {
                    iterdata::history_success(self.b@,self.doc.view(),self.pol,prior_history,true);
                    let ghost before_history_update=*self;
                    self.history=Ghost(next_history);Frames::history_preserves_state(before_history_update,*self);Frames::compose_call(prior_receiver,*self,iterdata::Call::Frame);
                    let ghost before_pixels_update=*self;
                    self.pixel_history=Ghost(canvas_model::append(self.doc.view(),prior_receiver.pixel_history@,self.canvas@));
                    Frames::history_preserves_state(before_pixels_update,*self);
                    Frames::pixels_preserve_iterator(before_pixels_update,*self);
                    Frames::pixels_returned(prior_receiver,*self);
                }
                Ok(Some(FrameView {rgba:&self.canvas,rect:(0,0,w,h),delay:None,dispose:0,blend:0,recovered:self.base.any(),events:self.base}))
        }else{
                let ghost next_history=if matches!(prior_history.phase,iterdata::Phase::Active) {iterdata::History {calls:prior_history.calls+1,returned:prior_history.returned,phase:iterdata::Phase::Iend}}else{iterdata::History {calls:prior_history.calls+1,..prior_history}};
                proof {
                    if matches!(prior_history.phase,iterdata::Phase::Active) {iterdata::history_end(self.b@,self.doc.view(),self.pol,prior_history);}else{iterdata::history_fused(self.b@,self.doc.view(),self.pol,prior_history);}
                    let ghost before_history_update=*self;
                    self.history=Ghost(next_history);Frames::history_preserves_state(before_history_update,*self);Frames::compose_call(prior_receiver,*self,iterdata::Call::End);Frames::pixels_idle(prior_receiver,*self);
                }
                Ok(None)
        }
    }

    #[verifier::spinoff_prover]
    fn next_logged(&mut self) -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires old(self).pixel_wf(),old(self).source_wf(),old(self).iterator_wf(),iterdata::default_success(old(self).b@,old(self).doc.view(),old(self).pol),old(self).buffers_wf(),old(self).initial_wf(),old(self).allocation_wf(),
        ensures final(self).pixel_wf(),
            match r {
                Ok(Some(view))=>view.pixels()==final(self).canvas_pixels()
                    && canvas_model::step(old(self).model_input(),old(self).model_document(),old(self).model_policy(),old(self).history_view().returned,
                        old(self).canvas_pixels(),canvas_model::reference(old(self).canvas_trace(),old(self).history_view().returned),view.pixels())
                    && final(self).canvas_trace()==canvas_model::append(old(self).model_document(),old(self).canvas_trace(),view.pixels()),
                _=>final(self).canvas_trace()==old(self).canvas_trace(),
            },
            final(self).source_wf(),
            iterdata::call_eq(old(self).model_input(),old(self).model_document(),old(self).model_policy(),old(self).history_view(),iterdata::kind(r)),
            final(self).model_input()==old(self).model_input(),final(self).model_document()==old(self).model_document(),final(self).model_policy()==old(self).model_policy(),
            matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==old(self).history_view().returned+1,
            matches!(r,Ok(Some(_))) ==> final(self).history_view().phase==if old(self).model_document().actl.is_some() {iterdata::Phase::Active}else{iterdata::Phase::Iend},
            !matches!(r,Ok(Some(_))) ==> final(self).history_view().returned==old(self).history_view().returned,
            matches!(r,Ok(None)) ==> final(self).is_done(),
            crate::alloc_guard::req_of_log(final(self).log_view())==crate::alloc_guard::req_of_log(old(self).log_view())+iterdata::request_p(old(self).model_input(),old(self).model_policy(),old(self).call_count()),final(self).iterator_wf(),final(self).call_count()==old(self).call_count()+1,
            matches!(r,Err(TryError::OutOfMemory(_))) ==> iterdata::is_oom(final(self).history_view())
                && iterdata::SavedAt_p(old(self).model_input(),old(self).model_policy())==Some(old(self).call_count()),
            !iterdata::is_oom(old(self).history_view()) && !matches!(r,Err(TryError::OutOfMemory(_))) ==> !iterdata::is_oom(final(self).history_view()),
            iterdata::is_oom(old(self).history_view()) ==> iterdata::is_oom(final(self).history_view()) && final(self).history_view().returned==old(self).history_view().returned && matches!(r,Ok(None)) && final(self).log_view()==old(self).log_view(),
            final(self).buffers_wf(),final(self).initial_wf(),final(self).allocation_wf(),
            final(self).doc==old(self).doc,final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).log_view().len()<=3,
            old(self).saved.is_some() ==> final(self).saved.is_some() && final(self).log_view()==old(self).log_view(),
            r.is_err() ==> final(self).state.done(),
            matches!(r,Ok(None)) ==> final(self).canvas==old(self).canvas && final(self).saved==old(self).saved,
            old(self).state.done() ==> matches!(r,Ok(None)) && final(self).log_view()==old(self).log_view(),
            r.is_err() && matches!(r,Err(TryError::OutOfMemory(_))) ==> crate::alloc_guard::sites::log_tied(r,final(self).log_view()),
            final(self).log_view()==old(self).log_view()+if old(self).saved_call() && old(self).saved.is_none()
                {seq![(AllocSite::Saved,old(self).canvas@.len(),final(self).saved.is_some())]}
                else {Seq::<(AllocSite,nat,bool)>::empty()},
    {
        let ghost prior_receiver=*self;
        match self.begin_source() {
            Step::Done=>self.terminal_logged(false,Ghost(prior_receiver)),
            Step::Single=>self.terminal_logged(true,Ghost(prior_receiver)),
            Step::Frame(job)=>self.selected_logged(job,Ghost(prior_receiver)),
        }
    }

}

/// Expose only the complete normalization equation to the combined frame query.
fn normalize_reused(canvas: &mut [u8],blend: bool)
    requires old(canvas).len()<=crate::alloc_guard::MAX_BYTES,
    ensures final(canvas).len()==old(canvas).len(),
        final(canvas)@==if blend {composite::zero_alpha(old(canvas)@)}else{old(canvas)@},
{if blend {zero_alpha_rgb(canvas);}}

fn zero_alpha_rgb(canvas: &mut [u8])
    requires old(canvas).len()<=crate::alloc_guard::MAX_BYTES,
    ensures final(canvas)@==composite::zero_alpha(old(canvas)@),
        forall|p: nat| 4*p+4<=old(canvas).len() ==> #[trigger] final(canvas)@.subrange((4*p) as int,(4*p+4) as int)
            ==composite::over(old(canvas)@.subrange((4*p) as int,(4*p+4) as int),seq![0u8,0u8,0u8,0u8]),
{
    proof {crate::limits::caps_ok();}
    let ghost before=canvas@;
    let n=canvas.len();let mut i=0usize;
    while i+3<n
        invariant i<=n,i%4==0,canvas.len()==n,n<=crate::alloc_guard::MAX_BYTES,before.len()==n,
            forall|j: int| 0<=j<n ==> #[trigger] canvas[j]
                ==if j<i && j%4<3 && before[4*(j/4)+3]==0 {0u8}else{before[j]},
        decreases n-i,
    {
        let ghost prior=canvas@;
        if canvas[i+3]==0 {canvas[i]=0;canvas[i+1]=0;canvas[i+2]=0;}
        proof {
            assert forall|j: int| 0<=j<n implies #[trigger] canvas[j]
                ==if j<i+4 && j%4<3 && before[4*(j/4)+3]==0 {0u8}else{before[j]} by {
                if i<=j<i+4 {composite::pixel_block(i as nat,j);}
            }
        }
        i+=4;
    }
    proof {
        assert forall|j: int| 0<=j<n implies canvas@[j]==composite::zero_alpha(before)[j] by {
            if j>=i {composite::pixel_block(i as nat,j);}
        }
        assert(canvas@=~=composite::zero_alpha(before));
        assert forall|p: nat| 4*p+4<=before.len() implies #[trigger] canvas@.subrange((4*p) as int,(4*p+4) as int)
            ==composite::over(before.subrange((4*p) as int,(4*p+4) as int),seq![0u8,0u8,0u8,0u8]) by {
            composite::zero_alpha_pixel(before,p);
        }
    }
}

/// Convert native cached geometry without importing iterator state into the model.
spec fn previous_regions(previous: crate::frame_state::Previous) -> Option<(composite::Rect,u8)> {
    match previous {Some(((x,y,w,h),op))=>Some((composite::rect(x,y,w,h),op)),None=>None}
}

proof fn rectangle_row(cw: nat,x: nat,y: nat,w: nat,h: nat,row: nat,n: nat)
    requires cw>=1,x+w<=cw,row<h,4*(y+h)*cw<=n<=crate::alloc_guard::MAX_BYTES,
    ensures (y+row)*cw+x<=usize::MAX,4*((y+row)*cw+x)+4*w<=n,
{
    crate::limits::caps_ok();
    assert((y+row)*cw+x+w<=(y+row+1)*cw) by (nonlinear_arith) requires x+w<=cw;
    assert((y+row+1)*cw<=(y+h)*cw) by (nonlinear_arith) requires row<h,cw>=1;
    assert(4*((y+row)*cw+x)+4*w<=4*(y+h)*cw) by (nonlinear_arith)
        requires (y+row)*cw+x+w<=(y+row+1)*cw,(y+row+1)*cw<=(y+h)*cw;
}

/// Clear precisely the preceding frame's rectangle, including all RGBA channels.
#[verifier::spinoff_prover]
fn clear_rect(canvas: &mut [u8],cw: u32,x: u32,y: u32,w: u32,h: u32)
    requires cw>=1,x as int+w as int<=cw,y as int+h as int<=u32::MAX,
        4*(y as int+h as int)*(cw as int)<=old(canvas).len()<=crate::alloc_guard::MAX_BYTES,
    ensures final(canvas)@==composite::clear_rect(old(canvas)@,cw as nat,composite::rect(x,y,w,h)),
{
    proof {crate::limits::caps_ok();}
    let ghost before=canvas@;
    let ghost r=composite::rect(x,y,w,h);
    let n=canvas.len();let mut row=0u32;
    proof {assert(canvas@=~=composite::clear_rows(before,cw as nat,r,0));}
    while row<h
        invariant row<=h,canvas.len()==n,cw>=1,x as int+w as int<=cw,y as int+h as int<=u32::MAX,
            4*(y as int+h as int)*(cw as int)<=n<=crate::alloc_guard::MAX_BYTES,
            before.len()==n,r==composite::rect(x,y,w,h),
            canvas@==composite::clear_rows(before,cw as nat,r,row as nat),
        decreases h-row,
    {
        proof {rectangle_row(cw as nat,x as nat,y as nat,w as nat,h as nat,row as nat,n as nat);}
        let o=4*((y+row) as usize*cw as usize+x as usize);
        let ghost prior=canvas@;
        zero_all(&mut canvas[o..o+4*w as usize]);
        proof {composite::clear_row_finished(before,prior,canvas@,cw as nat,r,row as nat);}
        row+=1;
    }
}

/// Copy only rectangle bytes. The access lemma is checked at the actual
/// source load: restore never reads Saved outside the matching rectangle.
fn copy_rect(canvas: &mut [u8],source: &[u8],cw: u32,x: u32,y: u32,w: u32,h: u32)
    requires cw>=1,x as int+w as int<=cw,y as int+h as int<=u32::MAX,
        source.len()==old(canvas).len(),
        4*(y as int+h as int)*(cw as int)<=old(canvas).len()<=crate::alloc_guard::MAX_BYTES,
    ensures final(canvas)@==composite::copy_rect(old(canvas)@,source@,cw as nat,composite::rect(x,y,w,h)),
{
    proof {crate::limits::caps_ok();}
    let ghost before=canvas@;
    let ghost r=composite::rect(x,y,w,h);
    let n=canvas.len();let mut row=0u32;
    proof {assert(canvas@=~=composite::copy_rows(before,source@,cw as nat,r,0));}
    while row<h
        invariant row<=h,canvas.len()==n,source.len()==n,
            cw>=1,x as int+w as int<=cw,y as int+h as int<=u32::MAX,
            4*(y as int+h as int)*(cw as int)<=n<=crate::alloc_guard::MAX_BYTES,
            before.len()==n,r==composite::rect(x,y,w,h),
            canvas@==composite::copy_rows(before,source@,cw as nat,r,row as nat),
        decreases h-row,
    {
        proof {rectangle_row(cw as nat,x as nat,y as nat,w as nat,h as nat,row as nat,n as nat);}
        let o=4*((y+row) as usize*cw as usize+x as usize);
        let bytes=4*w as usize;
        let mut col=0usize;
        while col<bytes
            invariant col<=bytes,bytes==4*w,o==composite::row_base(cw as nat,r,row as nat),
                row<h,row<=h,cw>=1,x as int+w as int<=cw,r==composite::rect(x,y,w,h),
                canvas.len()==n,source.len()==n,before.len()==n,o+bytes<=n,
                forall|j: int| 0<=j<n ==> #[trigger] canvas[j]
                    == if composite::inside_rows(cw as nat,r,row as nat,j) || o<=j<o+col
                        {source@[j]}else{before[j]},
            decreases bytes-col,
        {
            proof {
                composite::rectangle_access(cw as nat,r,row as nat,col as nat);
                assert(composite::inside(cw as nat,r,(o+col) as int));
            }
            canvas[o+col]=source[o+col];
            col+=1;
        }
        proof {composite::copy_row_finished(before,source@,canvas@,cw as nat,r,row as nat);}
        row+=1;
    }
}

} // verus!

// ------------------------------------------------------------------ P17 ICC

verus! {

/// Bind the measuring pass to exactly the supplied iCCP stream.
fn measure_profile(bytes: &[u8],ring: &mut [u8]) -> (r: Result<usize,Error>)
    requires bytes.len()<=inflate_verus::limits::SEGMENT_BYTES_MAX,old(ring).len()==WINDOW_MAX,
    ensures final(ring).len()==WINDOW_MAX,
        match r {
            Ok(n)=>n<=MAX_ICC && crate::icc_model::measured(bytes@).status==inflate_verus::model::InflateStatus::Done
                && n==crate::icc_model::measured(bytes@).out.len(),
            Err(e)=>crate::icc_model::data(bytes@)==crate::icc_model::Outcome::Error(e),
        },
{
    hide(inflate_verus::model::Inflate);
    let parts: [&[u8];1]=[bytes];
    proof {
        assert(inflate_verus::segments::parts_prefix(parts@,0)==Seq::<u8>::empty());
        assert(inflate_verus::segments::parts_prefix(parts@,1)==bytes@);
    }
    let mut src=match SegmentSlices::new(&parts) {
        Some(s)=>s,None=>{assert(false);return Err(Error::TruncatedZlib);},
    };
    let result=inflate_zlib_measure(&mut src,ring,MAX_ICC,InflateOptions::STRICT);
    match result.status {
        Status::Done=>{
            proof {crate::limits::caps_ok();}
            Ok(result.produced as usize)
        },
        Status::Full=>Err(Error::IccProfileTooLarge),
        Status::Err(e)=>{
            proof {
                if let inflate_verus::model::InflateStatus::Error(f)=crate::icc_model::measured(bytes@).status {
                    crate::icc_model::fault_agrees(e,f);
                }
            }
            Err(Error::from_inflate(e))
        },
    }
}

/// A successful measurement makes the exact-sized second pass infallible.
fn write_profile(bytes: &[u8],out: &mut [u8]) -> (r: Result<(),Error>)
    requires bytes.len()<=inflate_verus::limits::SEGMENT_BYTES_MAX,
        crate::icc_model::measured(bytes@).status==inflate_verus::model::InflateStatus::Done,
        old(out).len()==crate::icc_model::measured(bytes@).out.len(),old(out).len()<=MAX_ICC,
    ensures r==Ok(()),final(out)@==crate::icc_model::measured(bytes@).out,
{
    hide(inflate_verus::model::Inflate);
    proof {inflate_verus::limit_mono::lemma_limit_mono(bytes@,MAX_ICC as nat,out.len() as nat,true,false);}
    let parts: [&[u8];1]=[bytes];
    proof {
        assert(inflate_verus::segments::parts_prefix(parts@,0)==Seq::<u8>::empty());
        assert(inflate_verus::segments::parts_prefix(parts@,1)==bytes@);
    }
    let mut src=match SegmentSlices::new(&parts) {
        Some(s)=>s,None=>{assert(false);return Err(Error::TruncatedZlib);},
    };
    let result=inflate_zlib_into(&mut src,out,InflateOptions::STRICT);
    match result.status {
        Status::Done=>Ok(()),
        Status::Full=>Err(Error::IccProfileTooLarge),
        Status::Err(e)=>Err(Error::from_inflate(e)),
    }
}

fn icc_impl(b: &[u8], trace: &mut Trace) -> (r: Result<Option<Vec<u8>>, TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::icc_model::complete(b@,crate::icc_model::result_view(r),final(trace).log_view()),
{
    hide(inflate_verus::model::Inflate);
    hide(crate::document_model::MW_p);
    proof {crate::limits::caps_ok();}
    let mut ev = Events {r1:false,r3:false,r4:false};
    let (bytes,m)={
        let prepared=prepare_icc(b,&mut ev,trace);
        proof {crate::icc_model::prefix_exit(b@,prepared_icc_view(prepared),ev.view(),trace.log_view());}
        let (doc,mut ring)=match prepared {Err(e)=>return Err(e),Ok(None)=>return Ok(None),Ok(Some(ready))=>ready};
        proof {crate::limits::caps_ok();crate::icc_model::short_log(trace.log_view());}
        let (o,n)=match doc.iccp {Some(r)=>r,None=>return Ok(None)};
        let bytes=&b[o..o+n];
        let m=match measure_profile(bytes,&mut ring) {Ok(n)=>n,Err(e)=>return Err(TryError::Format(e))};
        proof {
            assert(crate::icc_model::MI(b@)==crate::icc_model::Outcome::Profile(crate::icc_model::measured(bytes@).out));
            assert(crate::icc_model::requests(b@)==seq![(AllocSite::IccWindow,WINDOW_MAX as nat),(AllocSite::Icc,m as nat)]);
        }
        (bytes,m)
    }; // The measuring Window leaves scope before the profile reservation.
    let reserved=reserve_icc(m,trace);
    proof {crate::icc_model::short_log(trace.log_view());}
    let mut prof=match reserved {Ok(v)=>v,Err(e)=>return Err(e)};
    let written=write_profile(bytes,&mut prof);
    proof {
        assert(crate::alloc_guard::all_ok(trace.log_view()));
        assert(crate::alloc_guard::req_of_log(trace.log_view())==crate::icc_model::requests(b@));
    }
    match written {Ok(())=>Ok(Some(prof)),Err(e)=>Err(TryError::Format(e))}
}

/// `icc_profile` (P17, Strict): `Ok(None)` without iCCP, otherwise the profile
/// inflated with limit `MAX_ICC`. A damaged profile is an error of this call
/// only; no pixel entry point inflates it.
pub fn try_icc_profile(b: &[u8]) -> (r: Result<Option<Vec<u8>>, TryError>)
    ensures !matches!(r,Err(TryError::OutOfMemory(_))) ==> crate::icc_model::eq(crate::icc_model::result_view(r),crate::icc_model::MI(b@)),
{
    let mut trace=Trace::new();
    icc_impl(b,&mut trace)
}

/// The public ICC entry and the owned sealed log it actually used.
pub fn try_icc_profile_logged(b: &[u8]) -> (r: (Result<Option<Vec<u8>>,TryError>,Tracked<AllocLog>))
    ensures crate::icc_model::complete(b@,crate::icc_model::result_view(r.0),r.1@.view()),
{
    let mut trace=Trace::new();
    let result=icc_impl(b,&mut trace);
    let log=trace.into_log();
    (result,log)
}

} // verus!

/// Test-only: `try_icc_profile` with its request trace.
#[cfg(feature = "test-records")]
#[doc(hidden)]
pub fn try_icc_profile_traced(b: &[u8]) -> (Result<Option<Vec<u8>>, TryError>, ([(AllocSite, usize); 3], usize)) {
    let mut trace = Trace::new();
    let r = icc_impl(b, &mut trace);
    (r, trace.list())
}
