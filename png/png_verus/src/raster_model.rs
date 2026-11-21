//! P11 whole-raster MF trace and data-stage result, without converted pixels.
//! Palette index validity is the only conversion observation needed for P11's
//! per-row error order; colour/alpha values remain M5 obligations.
use vstd::prelude::*;
use crate::{filter_model as f,layout_model as l,png::Error};

verus! {

pub struct Raster {
    pub ihdr: crate::model::Ihdr,
    pub width: nat,
    pub height: nat,
    pub entries: nat,
    pub reject_index: bool,
}
pub open spec fn shape(h: crate::model::Ihdr,w: nat,height: nat,entries: nat,pol: crate::policy::Policy) -> Raster {
    Raster {ihdr:h,width:w,height,entries,reject_index:pol.palette_index==crate::policy::PaletteIndex::Reject}
}
pub open spec fn bits(a: Raster) -> nat {l::bits(a.ihdr)}
pub open spec fn count(a: Raster) -> nat {l::count(a.ihdr.interlace)}
pub open spec fn pw(a: Raster,k: nat) -> nat {f::pass_width(a.ihdr.interlace,a.width,k)}
pub open spec fn ph(a: Raster,k: nat) -> nat {f::pass_height(a.ihdr.interlace,a.height,k)}
pub open spec fn rb(a: Raster,k: nat) -> nat {l::scan_bytes(bits(a),pw(a,k))}
pub open spec fn base(a: Raster,k: nat) -> nat {l::prefix(a.ihdr.interlace,bits(a),a.width,a.height,k)}
pub open spec fn raw(a: Raster) -> nat {l::raw(a.ihdr.interlace,bits(a),a.width,a.height)}
pub open spec fn line(input: Seq<u8>,a: Raster,k: nat,row: nat) -> Seq<u8> {
    f::pass_row(input,base(a,k),rb(a,k),f::bpp(bits(a)),row)
}

/// P13's most-significant-bit-first palette sample; palette depths are 1,2,4,8.
pub open spec fn palette_sample(line: Seq<u8>,depth: u8,x: nat) -> u8 {
    if depth==1 {(line[(x/8) as int] >> (7-(x%8)) as u8)&1u8}
    else if depth==2 {(line[(x/4) as int] >> (6-2*(x%4)) as u8)&3u8}
    else if depth==4 {(line[(x/2) as int] >> (4-4*(x%2)) as u8)&15u8}
    else {line[x as int]}
}
pub open spec fn palette_valid(line: Seq<u8>,depth: u8,width: nat,entries: nat) -> bool {
    forall|x: nat| x<width ==> (#[trigger] palette_sample(line,depth,x) as nat)<entries
}
pub open spec fn row_result(input: Seq<u8>,a: Raster,k: nat,row: nat) -> Result<(),Error> {
    if input[f::position(base(a,k),rb(a,k),row) as int]>4 {Err(Error::InvalidFilterType)}
    else if a.ihdr.color_type==3 && a.reject_index && !palette_valid(line(input,a,k,row),a.ihdr.depth,pw(a,k),a.entries) {
        Err(Error::PaletteIndexOutOfRange)
    } else {Ok(())}
}
pub open spec fn chain(first: Result<(),Error>,rest: Result<(),Error>) -> Result<(),Error> {
    match first {Err(e)=>Err(e),Ok(())=>rest}
}
pub closed spec fn pass_result(input: Seq<u8>,a: Raster,k: nat,row: nat) -> Result<(),Error>
    decreases ph(a,k)-row,
{
    if pw(a,k)==0 || row>=ph(a,k) {Ok(())}
    else {chain(row_result(input,a,k,row),pass_result(input,a,k,row+1))}
}
pub closed spec fn scan(input: Seq<u8>,a: Raster,k: nat) -> Result<(),Error>
    decreases count(a)-k,
{
    if k>=count(a) {Ok(())}
    else {chain(pass_result(input,a,k,0),scan(input,a,k+1))}
}
pub proof fn pass_step(input: Seq<u8>,a: Raster,k: nat,row: nat)
    ensures pass_result(input,a,k,row)==if pw(a,k)==0 || row>=ph(a,k) {Ok(())}
        else {chain(row_result(input,a,k,row),pass_result(input,a,k,row+1))},
{reveal_with_fuel(pass_result,2);}
pub proof fn scan_step(input: Seq<u8>,a: Raster,k: nat)
    ensures scan(input,a,k)==if k>=count(a) {Ok(())}
        else {chain(pass_result(input,a,k,0),scan(input,a,k+1))},
{reveal_with_fuel(scan,2);}

pub open spec fn completed(input: Seq<u8>,state: Seq<u8>,a: Raster,n: nat) -> bool {
    state.len()==input.len() && forall|k: nat,r: nat| k<n && pw(a,k)>0 && r<ph(a,k) ==>
        #[trigger] f::row_data(state,base(a,k),rb(a,k),r)==line(input,a,k,r)
}
pub proof fn row_end(a: Raster,k: nat,row: nat)
    requires a.ihdr.interlace<=1,k<count(a),pw(a,k)>0,row<ph(a,k),
    ensures f::position(base(a,k),rb(a,k),row)+1+rb(a,k)<=base(a,k+1),
{
    reveal_with_fuel(l::prefix,2);
    assert(l::contribution(a.ihdr.interlace,bits(a),a.width,a.height,k)==ph(a,k)*(1+rb(a,k)));
    assert((row+1)*(1+rb(a,k))<=ph(a,k)*(1+rb(a,k))) by (nonlinear_arith) requires row<ph(a,k);
    assert(f::position(base(a,k),rb(a,k),row)+1+rb(a,k)==base(a,k)+(row+1)*(1+rb(a,k))) by (nonlinear_arith);
}
pub proof fn preserve_completed(input: Seq<u8>,before: Seq<u8>,after: Seq<u8>,a: Raster,n: nat)
    requires a.ihdr.interlace<=1,n<=count(a),completed(input,before,a,n),after.len()==before.len(),
        base(a,n)<=before.len(),forall|j: int| 0<=j<base(a,n) ==> #[trigger] after[j]==before[j],
    ensures completed(input,after,a,n),
{
    assert forall|k: nat,r: nat| k<n && pw(a,k)>0 && r<ph(a,k) implies
        #[trigger] f::row_data(after,base(a,k),rb(a,k),r)==line(input,a,k,r) by {
        row_end(a,k,r);
        l::monotone(a.ihdr.interlace,bits(a),a.width,a.height,k+1,n);
        assert(f::row_data(before,base(a,k),rb(a,k),r)==line(input,a,k,r));
        assert(f::row_data(after,base(a,k),rb(a,k),r) =~= f::row_data(before,base(a,k),rb(a,k),r)) by {
            assert forall|j: int| 0<=j<rb(a,k) implies
                #[trigger] f::row_data(after,base(a,k),rb(a,k),r)[j]==f::row_data(before,base(a,k),rb(a,k),r)[j] by {}
        }
    }
}
pub proof fn complete_pass(input: Seq<u8>,state: Seq<u8>,a: Raster,k: nat)
    requires completed(input,state,a,k),
        f::pass_ready(input,state,base(a,k),rb(a,k),f::bpp(bits(a)),ph(a,k)),
    ensures completed(input,state,a,k+1),
{
    assert forall|p: nat,r: nat| p<k+1 && pw(a,p)>0 && r<ph(a,p) implies
        #[trigger] f::row_data(state,base(a,p),rb(a,p),r)==line(input,a,p,r) by {
        if p<k {}else{assert(p==k);}
    }
}
pub proof fn complete_empty(input: Seq<u8>,state: Seq<u8>,a: Raster,k: nat)
    requires completed(input,state,a,k),pw(a,k)==0 || ph(a,k)==0,
    ensures completed(input,state,a,k+1),
{
    assert forall|p: nat,r: nat| p<k+1 && pw(a,p)>0 && r<ph(a,p) implies
        #[trigger] f::row_data(state,base(a,p),rb(a,p),r)==line(input,a,p,r) by {assert(p<k);}
}

pub open spec fn inflated(z: Seq<u8>,a: Raster,pol: crate::policy::Policy) -> inflate_verus::model::Inflated {
    inflate_verus::model::Inflate(z,raw(a),pol.window==crate::policy::Window::Enforce,
        pol.zlib_tail==crate::policy::ZlibTail::IgnoreTail)
}
/// None is M2's explicit OutsideDomain, not a successful image result.
pub open spec fn data(z: Seq<u8>,a: Raster,pol: crate::policy::Policy) -> Option<Result<(),Error>> {
    let i=inflated(z,a,pol);
    match i.status {
        inflate_verus::model::InflateStatus::Done=>Some(if i.out.len()<raw(a) {Err(Error::TruncatedImageData)}else{scan(i.out,a,0)}),
        inflate_verus::model::InflateStatus::Full=>Some(Err(Error::ExcessImageData)),
        inflate_verus::model::InflateStatus::Error(e)=>Some(Err(crate::icc_model::fault(e))),
        inflate_verus::model::InflateStatus::OutsideDomain=>None,
    }
}

pub open spec fn document_shape(doc: crate::document_model::Doc,pol: crate::policy::Policy) -> Raster {
    shape(doc.ihdr,doc.ihdr.width as nat,doc.ihdr.height as nat,crate::document_model::entries(doc),pol)
}
pub open spec fn document_data(b: Seq<u8>,doc: crate::document_model::Doc,pol: crate::policy::Policy) -> Option<Result<(),Error>> {
    match crate::document_model::default_stream(b,doc) {
        Some(z)=>data(z,document_shape(doc,pol),pol),None=>None,
    }
}

/// M4 deletes channel values, not success/error, dimensions or physical byte length.
pub enum Projection {Error(Error),Ready {width:u32,height:u32,bytes:nat},OutsideDomain}
pub open spec fn finish(doc: crate::document_model::Doc,wide: bool,result: Option<Result<(),Error>>) -> Projection {
    match result {
        Some(Err(e))=>Projection::Error(e),
        Some(Ok(()))=>Projection::Ready {width:doc.ihdr.width,height:doc.ihdr.height,bytes:l::output_count(doc.ihdr,wide)},
        None=>Projection::OutsideDomain,
    }
}
pub closed spec fn still(b: Seq<u8>,pol: crate::policy::Policy,wide: bool) -> Projection {
    match crate::document_model::MW_p(b,pol.walk_view()) {
        crate::document_model::Outcome::Error(e,_)=>Projection::Error(e),
        crate::document_model::Outcome::Done(doc,_)=>finish(doc,wide,document_data(b,doc,pol)),
        _=>Projection::OutsideDomain,
    }
}
pub(crate) proof fn still_walk(b: Seq<u8>,pol: crate::policy::Policy,wide: bool)
    ensures still(b,pol,wide)==match crate::document_model::MW_p(b,pol.walk_view()) {
        crate::document_model::Outcome::Error(e,_)=>Projection::Error(e),
        crate::document_model::Outcome::Done(doc,_)=>finish(doc,wide,document_data(b,doc,pol)),
        _=>Projection::OutsideDomain,
    },
{
    hide(crate::document_model::MW_p);hide(document_data);
    reveal(still);
}
pub open spec fn fmt_eq<T>(b: Seq<u8>,pol: crate::policy::Policy,wide: bool,result: Result<T,crate::alloc_guard::sites::TryError>) -> bool {
    match result {
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,
        _=>match still(b,pol,wide) {
            Projection::Error(e)=>result==Err(crate::alloc_guard::sites::TryError::Format(e)),
            Projection::Ready {..}=>result.is_ok(),
            Projection::OutsideDomain=>false,
        },
    }
}
pub open spec fn shape_matches(b: Seq<u8>,pol: crate::policy::Policy,wide: bool,width: u32,height: u32,bytes: nat) -> bool {
    still(b,pol,wide)==Projection::Ready {width,height,bytes}
}
pub(crate) proof fn prepared_result<T>(b: Seq<u8>,pol: crate::policy::Policy,wide: bool,
    prefix: Result<crate::document_model::Doc,crate::alloc_guard::sites::TryError>,
    events: crate::document_model::Events,log: Seq<(crate::alloc_guard::AllocSite,nat,bool)>)
    requires crate::allocation_model::image_prepared(b,pol.walk_view(),false,wide,prefix,events,log),
    ensures match prefix {
        Err(e)=>fmt_eq::<T>(b,pol,wide,Err(e)),
        Ok(doc)=>still(b,pol,wide)==finish(doc,wide,document_data(b,doc,pol)),
    },
{
    hide(crate::document_model::MW_p);hide(document_data);
    still_walk(b,pol,wide);
    match crate::document_model::MW_p(b,pol.walk_view()) {
        crate::document_model::Outcome::Done(doc,_)=>{},
        crate::document_model::Outcome::Error(_,_)=>{},
        _=>{},
    }
}
pub open spec fn profile(p: crate::policy::Profile) -> crate::policy::Policy {
    match p {crate::policy::Profile::Strict=>crate::policy::STRICT,
        crate::policy::Profile::RecommendedBeta=>crate::policy::RECOMMENDED_BETA}
}

pub(crate) proof fn first_row_error(input: Seq<u8>,a: Raster,e: Error)
    requires count(a)>0,pw(a,0)>0,ph(a,0)>0,row_result(input,a,0,0)==Err(e),
    ensures scan(input,a,0)==Err(e),
{scan_step(input,a,0);pass_step(input,a,0,0);}

pub(crate) open spec fn default_case(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,entries: nat) -> bool {
    match crate::document_model::MW_p(b,crate::document_model::strict()) {
        crate::document_model::Outcome::Done(doc,_)=>doc.ihdr==h
            && crate::document_model::entries(doc)==entries
            && crate::document_model::default_stream(b,doc)==Some(z),
        _=>false,
    }
}
pub(crate) proof fn default_error(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,entries: nat,e: Error)
    requires default_case(b,h,z,entries),
        data(z,shape(h,h.width as nat,h.height as nat,entries,crate::policy::STRICT),crate::policy::STRICT)==Some(Err(e)),
    ensures still(b,crate::policy::STRICT,false)==Projection::Error(e),
{
    hide(crate::document_model::MW_p);hide(data);
    crate::walk_policy::profiles_project_exactly();
    still_walk(b,crate::policy::STRICT,false);
}

} // verus!
