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
use crate::walk::{Doc, Ihdr};

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

fn reserve_saved(n: usize, trace: &mut Trace) -> (r: Result<Vec<u8>,TryError>)
    requires 1<=n<=crate::alloc_guard::MAX_BYTES,old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        final(trace).log_view()==seq![(AllocSite::Saved,n as nat,r.is_ok())],
        match r {Ok(v)=>v@==Seq::new(n as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let result=try_zeroed(AllocSite::Saved,n,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::Saved,n);
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
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        crate::allocation_model::image_prepared(b@,pol.walk_view(),frames,wide,prepared_image_view(r),final(ev).view(),final(trace).log_view()),
        match r {Ok((doc,raw,out))=>doc.wf(b@) && crate::transport_model::idat_result(b@,doc.view())
            && crate::layout_model::image_buffers(doc.ihdr.view(),wide,raw@,out@)
            && raw@==Seq::new(raw@.len(),|_i: int|0u8) && out@==Seq::new(out@.len(),|_i: int|0u8)
            && (frames ==> crate::frame_state::source_document(b@,doc)),Err(_)=>true},
{
    hide(Doc::wf);hide(crate::transport_model::idat_result);hide(crate::layout_model::raw);
    hide(crate::document_model::MWF_p);hide(crate::document_model::MW_p);
    let result=if frames {crate::callers::frames(b,pol,ev)}else{crate::callers::still(b,pol,ev)};
    let doc=match result {Err(e)=>return Err(e),Ok(doc)=>doc};
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

} // verus!

/// P15 for RGBA8 (`wide = false`) or RGBA16: walk, reserve, decode.
fn still_impl(b: &[u8], pol: &Policy, wide: bool, ev: &mut Events, trace: &mut Trace) -> Result<(Doc, Vec<u8>), TryError> {
    let (doc,mut raw,mut out)=prepare_image(b,pol,false,wide,ev,trace)?;
    let mut src = IdatReader::for_doc(b, &doc);
    let mut sink = if wide { Sink::Rgba16(&mut out) } else { Sink::Rgba8(&mut out) };
    decode_image(&doc, b, &mut src, &mut raw, doc.ihdr.w, doc.ihdr.h, pol, ev, &mut sink).map_err(TryError::Format)?;
    drop(raw);
    Ok((doc, out))
}

fn still8(b: &[u8], pol: &Policy, ev: &mut Events, trace: &mut Trace) -> Result<Still, TryError> {
    let (doc, rgba) = still_impl(b, pol, false, ev, trace)?;
    Ok(assemble_still(doc,rgba))
}

fn still16(b: &[u8], pol: &Policy, ev: &mut Events, trace: &mut Trace) -> Result<Still16, TryError> {
    let (doc, rgba16) = still_impl(b, pol, true, ev, trace)?;
    Ok(assemble_still16(doc,rgba16))
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

/// The frame decoder (P16). Borrows the input; owns `Raw` (filtered data of the
/// full image, reused by every frame), `Output` (the canvas) and, from step S
/// on, `Saved`. Not an `Iterator`: each view borrows the canvas.
verus! {

pub struct Frames<'a> {
    b: &'a [u8],
    pol: Policy,
    doc: Doc,
    raw: Vec<u8>,
    canvas: Vec<u8>,
    saved: Option<Vec<u8>>,
    base: Events,
    state: FrameState,
}

impl<'a> Frames<'a> {
    /// Source/metadata invariant only; buffer layout and pixel semantics are
    /// separate obligations of the image and allocation stages.
    #[verifier::opaque]
    pub closed spec fn source_wf(&self) -> bool {
        self.state.wf(self.b@,self.doc)
        && self.doc.info.num_plays==match self.doc.actl {Some(a)=>Some(a.num_plays),None=>None}
    }

    /// Storage lengths only; sample values and subframe-fit semantics are separate.
    pub closed spec fn buffers_wf(&self) -> bool {
        self.doc.ihdr.wf()
        && crate::layout_model::image_buffers(self.doc.ihdr.view(),false,self.raw@,self.canvas@)
        && match self.saved {None=>true,Some(v)=>v@.len()==self.canvas@.len()}
    }

    fn from_decoded(b: &'a [u8], pol: Policy, mut doc: Doc,
        raw: Vec<u8>, canvas: Vec<u8>, base: Events) -> (r: Self)
        requires crate::frame_state::source_document(b@,doc),
            crate::layout_model::image_buffers(doc.ihdr.view(),false,raw@,canvas@),
        ensures r.source_wf(),r.buffers_wf(),r.b==b,r.pol==pol,r.base==base,r.saved.is_none(),
            r.raw@==raw@,r.canvas@==canvas@,
            r.doc.info.view()==crate::document_model::constructor_info(doc.view()),
            r.state.single()==doc.actl.is_none(),!r.state.done(),
    {
        crate::frame_state::constructor_info(b,&mut doc);
        let state=FrameState::new(b,&doc);
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);reveal(crate::frame_state::source_document);}
        Self {b,pol,doc,raw,canvas,saved:None,base,state}
    }

    fn begin_source(&mut self) -> (r: Step<'a>)
        requires old(self).source_wf(),
        ensures final(self).source_wf(),
            old(self).buffers_wf() ==> final(self).buffers_wf(),final(self).state.done(),
            final(self).doc==old(self).doc,final(self).b==old(self).b,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            final(self).saved==old(self).saved,final(self).base==old(self).base,
            match r {Step::Frame(job)=>job.wf(final(self).b@,final(self).doc),_=>true},
    {
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);}
        self.state.begin_next(self.b,&self.doc)
    }

    fn commit_source(&mut self, resume: FrameState)
        requires old(self).source_wf(),resume.wf(old(self).b@,old(self).doc),
        ensures final(self).source_wf(),
            old(self).buffers_wf() ==> final(self).buffers_wf(),final(self).state==resume,
            final(self).doc==old(self).doc,final(self).b==old(self).b,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            final(self).saved==old(self).saved,final(self).base==old(self).base,
    {
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);}
        self.state=resume;
    }

    fn ensure_saved(&mut self, needed: bool, trace: &mut Trace) -> (r: Result<(),TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),old(trace).wf(),old(trace).n==0,
        ensures final(self).source_wf(),final(self).buffers_wf(),final(trace).wf(),
            final(self).doc==old(self).doc,final(self).state==old(self).state,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
            !needed || old(self).saved.is_some() ==> final(trace).log_view().len()==0,
            needed && old(self).saved.is_none() ==> final(trace).log_view()==seq![(AllocSite::Saved,old(self).canvas@.len(),r.is_ok())],
            r.is_ok() && needed ==> final(self).saved.is_some(),
            r.is_err() ==> matches!(r,Err(TryError::OutOfMemory(_))) && final(self).saved.is_none(),
    {
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);crate::limits::caps_ok();}
        if !needed || self.saved.is_some() {return Ok(());}
        let n=self.canvas.len();
        proof {
            assert(4*(self.doc.ihdr.w as int)*(self.doc.ihdr.h as int)>=4) by (nonlinear_arith)
                requires self.doc.ihdr.w>=1,self.doc.ihdr.h>=1;
            assert(1<=n<=crate::alloc_guard::MAX_BYTES);
        }
        match reserve_saved(n,trace) {
            Err(e)=>Err(e),Ok(v)=>{self.saved=Some(v);Ok(())},
        }
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
            proof {reveal(crate::frame_state::FrameJob::wf);}
            receiver.commit_source(job.resume);
            proof {assert(receiver.source_wf());}
        },_=>{assert(receiver.source_wf());}}
    }}
}

} // verus!

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
        let mut trace = Trace::new();
        let (doc,mut raw,mut canvas)=prepare_image(b,&pol,true,false,&mut ev,&mut trace)?;
        let mut src = IdatReader::for_doc(b, &doc);
        decode_image(&doc, b, &mut src, &mut raw, doc.ihdr.w, doc.ihdr.h, &pol, &mut ev, &mut Sink::Rgba8(&mut canvas))
            .map_err(TryError::Format)?;
        if doc.actl.is_some() {
            if !doc.info.default_is_frame {
                // A default image outside the animation is validated, not shown (P16).
                canvas.fill(0);
            }
        }
        Ok(Frames::from_decoded(b,pol,doc,raw,canvas,ev))
    }

    /// The next frame, `Ok(None)` after the last one, or the error that ends
    /// the sequence. Fused: after `Ok(None)` or any error every later call
    /// returns `Ok(None)` without allocating or decoding (D3).
    pub fn try_next(&mut self) -> Result<Option<FrameView<'_>>, TryError> {
        let mut trace=Trace::new();
        let w = self.doc.ihdr.w;
        let h = self.doc.ihdr.h;
        match self.begin_source() {
            Step::Done => Ok(None),
            Step::Single => {
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
            Step::Frame(job) => {
                let k = job.index;
                let f = job.control;
                let prev = job.previous;
                // Step S (P20, D9): Saved, once, at the start of the first
                // reached call for a frame k > 0 with dispose_op = 2.
                self.ensure_saved(k>0 && f.dispose==2,&mut trace)?;
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
                let op = job.effective_dispose;
                if op == 2 {
                    if let Some(s) = self.saved.as_mut() {
                        s.copy_from_slice(&self.canvas);
                    }
                }
                let mut fev = self.base;
                if job.reader.is_none() {
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
                    let mut src = job.reader.unwrap();
                    let mut sink = Sink::Canvas { canvas: &mut self.canvas, cw: w, fx: f.x, fy: f.y, over: f.blend == 1 };
                    if let Err(e) = decode_image(&self.doc, self.b, &mut src, &mut self.raw, f.w, f.h, &self.pol, &mut fev, &mut sink) {
                        return Err(TryError::Format(e));
                    }
                }
                self.commit_source(job.resume);
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
