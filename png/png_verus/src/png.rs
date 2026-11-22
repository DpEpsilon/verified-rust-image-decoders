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
        match r {Ok((doc,raw,out))=>doc.wf(b@) && doc.ihdr.wf() && b.len()<=crate::limits::MAX_INPUT && crate::transport_model::idat_result(b@,doc.view())
            && crate::layout_model::image_buffers(doc.ihdr.view(),wide,raw@,out@)
            && out.len()<=crate::alloc_guard::MAX_BYTES
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
    ensures final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),false,wide,r,final(trace).log_view()),
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
    }
    let ghost output_len=out.len();
    {
        let mut src=IdatReader::for_doc(b,&doc);
        let mut sink=Sink::image(&mut out,doc.ihdr.w,doc.ihdr.h,wide);
        let decoded=decode_image(&doc,b,&mut src,&mut raw,doc.ihdr.w,doc.ihdr.h,pol,ev,&mut sink);
        assert(raster::document_data(b@,doc.view(),*pol)==Some(decoded));
        sink.finish();
        match decoded {Err(e)=>return Err(TryError::Format(e)),Ok(())=>{}}

    }
    assert(out.len()==output_len);
    assert(crate::allocation_model::returned_info(b@,pol.walk_view(),false,doc.info.view()));
    Ok((doc,out))
}

fn still8(b: &[u8],pol: &Policy,ev: &mut Events,trace: &mut Trace) -> (r: Result<Still,TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),false,false,r,final(trace).log_view()),
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
    ensures final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),false,true,r,final(trace).log_view()),
        match r {Ok(s)=>s.info.ranges_wf(b.len() as nat) && crate::allocation_model::returned_info(b@,pol.walk_view(),false,s.info.view()),_=>true},
        raster::fmt_eq(b@,*pol,true,r),
        match r {Ok(s)=>raster::shape_matches(b@,*pol,true,s.width,s.height,s.rgba16.len() as nat),_=>true},
{
    let (doc,rgba)=match still_impl(b,pol,true,ev,trace) {Ok(v)=>v,Err(e)=>return Err(e)};
    Ok(assemble_still16(doc,rgba))
}

/// Strict RGBA8 public entry: checked walk-error dispatch and Info forwarding.
pub fn try_decode_still(b: &[u8]) -> (r: Result<Still,TryError>)
    ensures forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,crate::document_model::strict())==crate::document_model::Outcome::Error(e,ev)
            ==> r==Err(TryError::Format(e)),
        match r {Ok(s)=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,crate::document_model::strict(),false,s.info.model_value()),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,STRICT,false,r),
        raster::fmt_eq(b@,STRICT,false,r),
        match r {Ok(s)=>raster::shape_matches(b@,STRICT,false,s.width,s.height,s.rgba.len() as nat),_=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let result=still8(b,&STRICT,&mut ev,&mut trace);
    proof {if let Ok(s)=&result {s.info.public_observations(b.len() as nat);}}
    result
}
pub fn try_decode_still16(b: &[u8]) -> (r: Result<Still16,TryError>)
    ensures raster::fmt_eq(b@,STRICT,true,r),
        match r {Ok(s)=>raster::shape_matches(b@,STRICT,true,s.width,s.height,s.rgba16.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,crate::document_model::strict())==crate::document_model::Outcome::Error(e,ev)
            ==> r==Err(TryError::Format(e)),
        match r {Ok(s)=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,crate::document_model::strict(),false,s.info.model_value()),_=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let result=still16(b,&STRICT,&mut ev,&mut trace);
    proof {if let Ok(s)=&result {s.info.public_observations(b.len() as nat);}}
    result
}

pub fn try_decode_still_with_policy(b: &[u8],p: Profile) -> (r: Result<(Still,bool),TryError>)
    ensures raster::fmt_eq(b@,raster::profile(p),false,r),
        match r {Ok((s,_))=>raster::shape_matches(b@,raster::profile(p),false,s.width,s.height,s.rgba.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,p.walk_model())==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        match r {Ok((s,_))=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,p.walk_model(),false,s.info.model_value()),_=>true},
{
    let pol=p.checked_policy();let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let s=match still8(b,&pol,&mut ev,&mut trace) {Ok(s)=>s,Err(e)=>return Err(e)};
    proof {s.info.public_observations(b.len() as nat);}
    Ok((s,ev.any()))
}
pub fn try_decode_still16_with_policy(b: &[u8],p: Profile) -> (r: Result<(Still16,bool),TryError>)
    ensures raster::fmt_eq(b@,raster::profile(p),true,r),
        match r {Ok((s,_))=>raster::shape_matches(b@,raster::profile(p),true,s.width,s.height,s.rgba16.len() as nat),_=>true},
        forall|e: Error,ev: crate::document_model::Events|
        crate::document_model::MW_p(b@,p.walk_model())==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        match r {Ok((s,_))=>s.info.ranges_within(b.len() as nat) && crate::allocation_model::returned_info(b@,p.walk_model(),false,s.info.model_value()),_=>true},
{
    let pol=p.checked_policy();let mut ev=Events {r1:false,r3:false,r4:false};let mut trace=Trace::new();
    let s=match still16(b,&pol,&mut ev,&mut trace) {Ok(s)=>s,Err(e)=>return Err(e)};
    proof {s.info.public_observations(b.len() as nat);}
    Ok((s,ev.any()))
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

    /// Storage lengths and presence of the snapshot required by cached disposal.
    pub closed spec fn buffers_wf(&self) -> bool {
        self.doc.ihdr.wf()
        && crate::layout_model::image_buffers(self.doc.ihdr.view(),false,self.raw@,self.canvas@)
        && (match self.saved {None=>true,Some(v)=>v@.len()==self.canvas@.len()})
        && (crate::frame_state::previous_needs_saved(self.state.previous()) ==> self.saved.is_some())
        && self.restores_previous(self.state.previous())
        && (match self.state.index() {Some(k)=>self.saved.is_some()==crate::frame_transport_model::saved_before(self.doc.frame_ranges@,k),None=>true})
    }

    closed spec fn restores_previous(&self,previous: crate::frame_state::Previous) -> bool {
        match previous {
            Some(((x,y,w,h),2))=>composite::saved_before_frame(self.canvas@,self.saved_pixels(),self.snapshot@,
                self.doc.ihdr.w as nat,composite::rect(x,y,w,h)),
            _=>true,
        }
    }

    /// The current reached call qualifies; prior qualifiers are captured by buffers_wf.
    closed spec fn saved_call(&self) -> bool {
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
        ensures r.source_wf(),r.buffers_wf(),r.b==b,r.pol==pol,r.base==base,r.saved.is_none(),
            r.raw@==raw@,r.canvas@==canvas@,
            r.doc.info.view()==crate::document_model::constructor_info(doc.view()),r.doc.info.ranges_wf(b.len() as nat),
            r.state.single()==doc.actl.is_none(),!r.state.done(),r.state.previous()==None,
            r.state.index()==if doc.actl.is_some() {Some(0nat)}else{None},
    {
        crate::frame_state::constructor_info(b,&mut doc);
        let state=FrameState::new(b,&doc);
        proof {crate::frame_transport_model::saved_before_start(doc.frame_ranges@);}
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);reveal(crate::frame_state::source_document);}
        Self {b,pol,doc,raw,canvas,saved:None,snapshot:Ghost(None),base,state}
    }

    fn begin_source(&mut self) -> (r: Step<'a>)
        requires old(self).source_wf(),
        ensures final(self).source_wf(),
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
        requires old(self).source_wf(),resume.wf(old(self).b@,old(self).doc),old(self).restores_previous(resume.previous()),
            crate::frame_state::previous_needs_saved(resume.previous()) ==> old(self).saved.is_some(),
            match resume.index() {Some(k)=>old(self).saved.is_some()==crate::frame_transport_model::saved_before(old(self).doc.frame_ranges@,k),None=>true},
        ensures final(self).source_wf(),
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

    fn ensure_saved(&mut self, needed: bool, trace: &mut Trace) -> (r: Result<(),TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).state.done(),old(trace).wf(),old(trace).n==0,
        ensures final(self).source_wf(),final(self).buffers_wf(),final(trace).wf(),
            final(self).doc==old(self).doc,final(self).state==old(self).state,
            final(self).raw==old(self).raw,final(self).canvas==old(self).canvas,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).snapshot==old(self).snapshot,
            crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
            !needed || old(self).saved.is_some() ==> final(trace).log_view().len()==0,
            needed && old(self).saved.is_none() ==> final(trace).log_view()==seq![(AllocSite::Saved,old(self).canvas@.len(),r.is_ok())],
            r.is_ok() && needed ==> final(self).saved.is_some(),
            r.is_ok() ==> final(self).saved.is_some()==(old(self).saved.is_some() || needed),
            old(self).saved.is_some() || !needed ==> final(self).saved==old(self).saved,
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
        ensures match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf()
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
        ensures match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf()
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

    fn with_record(b: &'a [u8],pol: Policy) -> (r: Result<Frames<'a>,TryError>)
        ensures match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf()
            && receiver.doc.info.ranges_wf(b.len() as nat)
            && crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view()),_=>true},
            forall|e: Error,ev: crate::document_model::Events| crate::document_model::MWF_p(b@,pol.walk_view())
                ==crate::document_model::Outcome::Error(e,ev) ==> r==Err(TryError::Format(e)),
        crate::allocation_model::propagated_inflate_error(b@,pol,true,r),
    {
        let mut trace=Trace::new();
        Self::with_trace(b,pol,&mut trace)
    }

    fn with_trace(b: &'a [u8],pol: Policy,trace: &mut Trace) -> (r: Result<Frames<'a>,TryError>)
        requires old(trace).wf(),old(trace).n==0,
        ensures final(trace).wf(),crate::allocation_model::image_call(b@,pol.walk_view(),true,false,r,final(trace).log_view()),
            match r {Ok(receiver)=>receiver.source_wf() && receiver.buffers_wf()
                && receiver.doc.info.ranges_wf(b.len() as nat)
                && crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view()),_=>true},
        crate::allocation_model::propagated_inflate_error(b@,pol,true,r),
    {
        hide(Doc::wf);hide(crate::document_model::MWF_p);hide(inflate_verus::model::Inflate);
        let mut ev=Events {r1:false,r3:false,r4:false};
        let prepared=prepare_image(b,&pol,true,false,&mut ev,trace);
        proof {crate::allocation_model::prepared_exit::<Frames<'a>>(b@,pol.walk_view(),true,false,prepared_image_view(prepared),ev.view(),trace.log_view());}
        proof {crate::allocation_model::prepared_inflate_error::<Frames<'a>>(b@,pol,true,false,prepared_image_view(prepared),ev.view(),trace.log_view());}
        let (doc,mut raw,mut canvas)=match prepared {Ok(v)=>v,Err(e)=>return Err(e)};
        proof {crate::limits::caps_ok();crate::icc_model::short_log(trace.log_view());}
        let ghost before=doc.view();
        let ghost output_len=canvas.len();
        {
            let mut src=IdatReader::for_doc(b,&doc);
            let mut sink=Sink::image(&mut canvas,doc.ihdr.w,doc.ihdr.h,false);
            let decoded=decode_image(&doc,b,&mut src,&mut raw,doc.ihdr.w,doc.ihdr.h,&pol,&mut ev,&mut sink);
            sink.finish();
            match decoded {Err(e)=>return Err(TryError::Format(e)),Ok(())=>{}}
        }
        assert(canvas.len()==output_len);
        if doc.actl.is_some() && !doc.info.default_is_frame {zero_all(&mut canvas);}
        let receiver=Frames::from_decoded(b,pol,doc,raw,canvas,ev);
        assert(crate::allocation_model::returned_frame_info(b@,pol.walk_view(),receiver.info_value().view()));
        Ok(receiver)
    }
}

impl<'a> Frames<'a> {
    /// The next frame, `Ok(None)` after the last one, or the error that ends
    /// the sequence. Fused: after `Ok(None)` or any error every later call
    /// returns `Ok(None)` without allocating or decoding (D3).
    pub fn try_next(&mut self) -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),
        ensures final(self).source_wf(),final(self).buffers_wf(),final(self).info_value()==old(self).info_value(),
    {
        let mut trace=Trace::new();
        self.next_with_trace(&mut trace)
    }

    proof fn source_context(&self)
        requires self.source_wf(),
        ensures self.doc.wf(self.b@),self.b.len()<=crate::limits::MAX_INPUT,
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
    fn dispose_and_save(&mut self,prev: crate::frame_state::Previous,current: (u32,u32,u32,u32),op: u8)
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).state.done(),old(self).restores_previous(prev),
            crate::frame_state::previous_fits(prev,old(self).doc.ihdr.view()),
            crate::frame_state::previous_fits(Some((current,op)),old(self).doc.ihdr.view()),
            crate::frame_state::previous_needs_saved(prev) || op==2 ==> old(self).saved.is_some(),
        ensures final(self).source_wf(),final(self).buffers_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).state==old(self).state,final(self).raw==old(self).raw,
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

    /// Weak selected-image continuation: preserve layout and source metadata.
    fn draw_selected(&mut self,job: crate::frame_state::FrameJob<'a>) -> (r: Result<Events,TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).state.done(),job.wf(old(self).b@,old(self).doc),
            job.effective_dispose==2 ==> old(self).snapshot@==Some(old(self).canvas@)
                && match old(self).saved {Some(v)=>composite::snapshot_effect(v@,old(self).canvas@,old(self).doc.ihdr.w as nat,
                    composite::rect(job.control.x,job.control.y,job.control.w,job.control.h)),None=>false},
            job.effective_dispose==2 ==> old(self).saved.is_some(),
        ensures final(self).source_wf(),final(self).buffers_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).state==old(self).state,final(self).saved==old(self).saved,final(self).snapshot==old(self).snapshot,
            composite::drawing_frame(old(self).canvas@,final(self).canvas@,
                (2,old(self).canvas@.len(),old(self).doc.ihdr.w as nat,job.control.x as nat,job.control.y as nat,job.control.blend==1),
                job.control.w as nat,job.control.h as nat),
            final(self).restores_previous(Some(((job.control.x,job.control.y,job.control.w,job.control.h),job.effective_dispose))),
            r.is_err() ==> matches!(r,Err(TryError::Format(_))),
    {
        hide(raster::data);hide(raster::completed);hide(raster::inflated);
        hide(Doc::wf);hide(crate::document_model::frame_stream);
        hide(crate::frame_transport_model::control_record);hide(crate::frame_transport_model::cursor_for);
        proof {reveal(Frames::source_wf);reveal(Frames::buffers_wf);reveal(crate::frame_state::FrameJob::wf);crate::limits::caps_ok();self.source_context();}
        let ghost before=self.canvas@;
        let w=self.doc.ihdr.w;let h=self.doc.ihdr.h;let f=job.control;
        let ghost geometry=(2nat,before.len(),w as nat,f.x as nat,f.y as nat,f.blend==1);
                let mut fev=self.base;
                if job.reader.is_none() {
                    if f.blend==1 {zero_alpha_rgb(&mut self.canvas);}
                    proof {composite::full_drawing_frame(before,self.canvas@,w as nat,h as nat,f.blend==1);}
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
        requires old(self).source_wf(),old(self).buffers_wf(),old(self).state.done(),job.wf(old(self).b@,old(self).doc),
            old(self).restores_previous(job.previous),
            crate::frame_state::previous_needs_saved(job.previous) || job.effective_dispose==2 ==> old(self).saved.is_some(),
            old(self).saved.is_some()==crate::frame_transport_model::saved_before(old(self).doc.frame_ranges@,job.index as nat+1),
        ensures final(self).source_wf(),final(self).buffers_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(self).saved.is_some()==old(self).saved.is_some(),
            r.is_err() ==> final(self).state.done() && matches!(r,Err(TryError::Format(_))),
            r.is_ok() ==> final(self).state==job.resume,
    {
        hide(Doc::wf);hide(crate::frame_transport_model::control_record);
        proof {job.storage_context(self.b@,self.doc);}
        let f=job.control;let resume=job.resume;
        self.dispose_and_save(job.previous,(f.x,f.y,f.w,f.h),job.effective_dispose);
        let events=match self.draw_selected(job) {Ok(events)=>events,Err(e)=>return Err(e)};
        self.commit_source(resume);
        Ok(events)
    }

    fn next_with_trace(&mut self,trace: &mut Trace) -> (r: Result<Option<FrameView<'_>>,TryError>)
        requires old(self).source_wf(),old(self).buffers_wf(),old(trace).wf(),old(trace).n==0,
        ensures final(self).source_wf(),final(self).buffers_wf(),final(self).doc==old(self).doc,
            final(self).b==old(self).b,final(self).pol==old(self).pol,final(self).base==old(self).base,
            final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),final(trace).log_view().len()<=1,
            forall|i: int| 0<=i<final(trace).log_view().len() ==> (#[trigger] final(trace).log_view()[i]).0==AllocSite::Saved
                && final(trace).log_view()[i].1==old(self).canvas.len(),
            old(self).saved.is_some() ==> final(self).saved.is_some() && final(trace).log_view().len()==0,
            r.is_err() ==> final(self).state.done(),
            matches!(r,Ok(None)) ==> final(self).canvas==old(self).canvas && final(self).saved==old(self).saved,
            old(self).state.done() ==> matches!(r,Ok(None)) && final(trace).log_view().len()==0,
            final(trace).log_view()==if old(self).saved_call() && old(self).saved.is_none()
                {seq![(AllocSite::Saved,old(self).canvas@.len(),final(self).saved.is_some())]}
                else {Seq::<(AllocSite,nat,bool)>::empty()},
    {
        hide(Frames::buffers_wf);
        hide(composite::dispose_regions);hide(composite::snapshot_effect);
        hide(composite::copy_rect);hide(composite::clear_rect);
        hide(Doc::wf);hide(crate::document_model::frame_stream);
        hide(crate::frame_transport_model::control_record);hide(crate::frame_transport_model::cursor_for);
        let ghost prior_state=self.state;
        let ghost prior_saved=self.saved.is_some();
        let ghost wanted=self.saved_call();
        proof {self.saved_context();}
        let w=self.doc.ihdr.w;let h=self.doc.ihdr.h;
        match self.begin_source() {
            Step::Done=>Ok(None),
            Step::Single=>Ok(Some(FrameView {rgba:&self.canvas,rect:(0,0,w,h),delay:None,dispose:0,blend:0,recovered:self.base.any(),events:self.base})),
            Step::Frame(job)=>{
                proof {
                    job.storage_context(self.b@,self.doc);
                    crate::frame_transport_model::saved_before_step(self.doc.frame_ranges@,job.index as nat);
                    assert(prior_state.index()==Some(job.index as nat));
                    assert(prior_saved==crate::frame_transport_model::saved_before(self.doc.frame_ranges@,job.index as nat));
                }
                let k=job.index;let f=job.control;let resume=job.resume;
                proof {assert(wanted==(k>0 && f.dispose==2));}
                match self.ensure_saved(k>0 && f.dispose==2,trace) {Ok(())=>{},Err(e)=>return Err(e)}
                let fev=match self.finish_job(job) {Ok(ev)=>ev,Err(e)=>return Err(e)};
                Ok(Some(FrameView {rgba:&self.canvas,rect:(f.x,f.y,f.w,f.h),delay:Some((f.delay_num,f.delay_den)),
                    dispose:f.dispose,blend:f.blend,recovered:fev.any(),events:fev}))
            },
        }
    }
}

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
