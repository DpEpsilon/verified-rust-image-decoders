//! G12/G15 integration model, independent of the frame decoder's control flow.
//! A terminal state retains the last public canvas; an implementation may change
//! its private canvas while finding an error, but cannot expose those changes.

use vstd::prelude::*;
use crate::gif::Error;
use crate::model::WalkM;
#[cfg(verus_only)]
use crate::model::{HeaderM, FrameHeaderM, GceM, MbOut};
#[cfg(verus_only)]
use crate::policy::{Policy, LzwMin, LzwTail};

verus! {

pub enum Status { Running, Done, Failed }

pub struct FrameRec {
    pub rgba: Seq<u8>,
    pub rect: (int,int,int,int),
    pub delay_cs: u16,
    pub disposal: u8,
    pub user_input: bool,
    pub transparent: Option<u8>,
    pub interlaced: bool,
    pub recovered: bool,
}

pub struct Previous {
    pub rect: crate::mc::Rect,
    pub disposal: int,
}

pub struct State {
    pub walk: WalkM,
    pub canvas: Seq<u8>,
    pub saved: Option<Seq<u8>>,
    pub prev: Option<Previous>,
    pub status: Status,
}

pub struct Call {
    pub after: State,
    pub frame: Option<FrameRec>,
    pub error: Option<Error>,
    /// The request occurs after MB succeeds and before ML starts, even when
    /// this frame subsequently fails and never contributes a FrameRec.
    pub reserve_saved: bool,
}

pub open spec fn initial(b:Seq<u8>,pol:Policy)->Option<State> {
    match crate::model::mp(b,pol) {
        Err(_)=>None,
        Ok((info,s))=>Some(State {
            walk:s,
            canvas:Seq::new((4*info.width*info.height) as nat,|_i:int|0u8),
            saved:None,prev:None,
            status:if 0<=s.p<b.len() && b[s.p]==0x2c {Status::Running} else {Status::Done},
        }),
    }
}

pub open spec fn params(pol:Policy)->crate::lzw::LzwParams {
    let checked=pol.lzw_min==LzwMin::Range1to11Checked;
    crate::lzw::LzwParams {
        m_lo:if checked {1} else {2},m_hi:if checked {11} else {8},
        size1_only_one_pixel:checked,checked_literals:checked,deferred_clear:true,
        stop_at_raster:pol.lzw_tail==LzwTail::StopAtRaster,
    }
}

pub open spec fn lzw_error(e:crate::lzw_model::ErrK)->Error {
    match e {
        crate::lzw_model::ErrK::InvalidLzwMinCodeSize=>Error::InvalidLzwMinCodeSize,
        crate::lzw_model::ErrK::InvalidLzwCode=>Error::InvalidLzwCode,
        crate::lzw_model::ErrK::TooManyPixels=>Error::TooManyPixels,
        crate::lzw_model::ErrK::TooFewPixels=>Error::TooFewPixels,
        crate::lzw_model::ErrK::MissingEndOfInformation=>Error::MissingEndOfInformation,
        crate::lzw_model::ErrK::DataAfterEndOfInformation=>Error::DataAfterEndOfInformation,
        crate::lzw_model::ErrK::ColorIndexOutOfRange=>Error::ColorIndexOutOfRange,
    }
}

pub open spec fn raw_disposal(g:Option<GceM>)->u8 {
    match g {None=>0,Some(g)=>g.disposal}
}

pub open spec fn frame(fh:FrameHeaderM,g:Option<GceM>)->crate::mc::FrameSpec {
    crate::mc::FrameSpec {
        rect:crate::mc::Rect {left:fh.left as u32,top:fh.top as u32,width:fh.w as u32,height:fh.h as u32},
        interlaced:fh.interlaced,c:fh.table.entries,
        transparent:match g {None=>None,Some(g)=>g.transparent},
        disposal:if raw_disposal(g)>3 {1} else {raw_disposal(g) as int},
    }
}

pub open spec fn disposed(cv:Seq<u8>,saved:Option<Seq<u8>>,prev:Option<Previous>,sw:u32,sh:u32)->Seq<u8> {
    match prev {
        None=>cv,
        Some(p)=>if p.disposal==2 {crate::mc::clear_rect(cv,sw,sh,p.rect)}
            else if p.disposal==3 {match saved {Some(s)=>crate::mc::copy_rect(cv,s,sw,sh,p.rect),None=>cv}}
            else {cv},
    }
}

/// Restrict conversions to the wire domain. None is OutsideDomain, not an
/// additional GIF error; MB establishes these facts on reachable images.
pub open spec fn image_domain(b:Seq<u8>,h:HeaderM,fh:FrameHeaderM)->bool {
    &&& 1<=h.width<=crate::limits::MAX_DIMENSION && 1<=h.height<=crate::limits::MAX_DIMENSION
    &&& 0<=fh.left<=crate::limits::MAX_DIMENSION && 0<=fh.top<=crate::limits::MAX_DIMENSION
    &&& 1<=fh.w<=crate::limits::MAX_DIMENSION && 1<=fh.h<=crate::limits::MAX_DIMENSION
    &&& 0<=fh.table.off && 2<=fh.table.entries<=256
    &&& fh.table.off+3*fh.table.entries<=b.len()
}

pub open spec fn saved_image(s:State,fr:crate::mc::FrameSpec,cv:Seq<u8>,sw:u32,sh:u32,still:bool)->Option<Seq<u8>> {
    if fr.disposal==3 && !still {
        let base=match s.saved {Some(v)=>v,None=>Seq::new(s.canvas.len(),|_i:int|0u8)};
        Some(crate::mc::copy_rect(base,cv,sw,sh,fr.rect))
    } else {s.saved}
}

pub open spec fn image_mode(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM,still:bool)->Option<Call> {
    if !image_domain(b,h,fh) {None} else {
        let fr=frame(fh,g);
        let cv=disposed(s.canvas,s.saved,s.prev,h.width as u32,h.height as u32);
        let reserve=raw_disposal(g)==3 && !still && s.saved is None;
        let saved=saved_image(s,fr,cv,h.width as u32,h.height as u32,still);
        let ml=crate::lzw_model::ML(m,d,fh.w*fh.h,params(pol));
        let bad=exists|q:int| 0<=q<ml.idx.len() && fr.transparent!=Some(#[trigger] ml.idx[q]) && ml.idx[q] as int>=fr.c;
        let err=if bad {Some(Error::ColorIndexOutOfRange)} else {match ml.error {Some(e)=>Some(lzw_error(e)),None=>None}};
        match err {
            Some(e)=>Some(Call {
                after:State {walk,canvas:s.canvas,saved,prev:s.prev,status:Status::Failed},
                frame:None,error:Some(e),reserve_saved:reserve,
            }),
            None=>match crate::mc::draw_fold(cv,fr,b.subrange(fh.table.off,fh.table.off+3*fh.table.entries),h.width as u32,h.height as u32,ml.idx,0) {
                Err(_)=>None,
                Ok(out)=>Some(Call {
                    after:State {walk,canvas:out,saved,prev:Some(Previous {rect:fr.rect,disposal:fr.disposal}),status:Status::Running},
                    frame:Some(FrameRec {rgba:out,rect:(fh.left,fh.top,fh.w,fh.h),
                        delay_cs:match g {Some(g)=>g.delay_cs,None=>0},disposal:raw_disposal(g),
                        user_input:match g {Some(g)=>g.user_input,None=>false},transparent:fr.transparent,
                        interlaced:fh.interlaced,recovered:!fh.inside || ml.e4}),
                    error:None,reserve_saved:reserve,
                }),
            },
        }
    }
}

pub open spec fn image(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM)->Option<Call> {
    image_mode(b,h,pol,s,fh,g,m,d,walk,false)
}

pub proof fn lemma_draw_fold_defined(cv:Seq<u8>,fr:crate::mc::FrameSpec,table:Seq<u8>,sw:u32,sh:u32,idx:Seq<u8>,q:int)
    requires 0<=q<=idx.len(),forall|j:int| q<=j<idx.len() ==> fr.transparent==Some(#[trigger] idx[j]) || (idx[j] as int)<fr.c,
    ensures crate::mc::draw_fold(cv,fr,table,sw,sh,idx,q) is Ok,
    decreases idx.len()-q,
{
    hide(crate::mc::write_pixel);hide(crate::mc::target_cell);
    reveal_with_fuel(crate::mc::draw_fold,1);
    if q<idx.len() {
        let next=crate::mc::draw_step(cv,fr,table,sw,sh,q,idx[q]).unwrap();
        lemma_draw_fold_defined(next,fr,table,sw,sh,idx,q+1);
    }
}

pub proof fn lemma_image_defined(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM,still:bool)
    requires image_domain(b,h,fh),
    ensures image_mode(b,h,pol,s,fh,g,m,d,walk,still) is Some,
        image_mode(b,h,pol,s,fh,g,m,d,walk,still).unwrap().after.walk==walk,
        ({let c=image_mode(b,h,pol,s,fh,g,m,d,walk,still).unwrap();match c.frame {
            None=>c.after.status==Status::Failed && c.error is Some,
            Some(_)=>c.after.status==Status::Running && c.error is None,
        }}),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
    let fr=frame(fh,g);
    let cv=disposed(s.canvas,s.saved,s.prev,h.width as u32,h.height as u32);
    let ml=crate::lzw_model::ML(m,d,fh.w*fh.h,params(pol));
    if ml.error is None && !(exists|q:int| 0<=q<ml.idx.len() && fr.transparent!=Some(#[trigger] ml.idx[q]) && (ml.idx[q] as int)>=fr.c) {
        lemma_draw_fold_defined(cv,fr,b.subrange(fh.table.off,fh.table.off+3*fh.table.entries),h.width as u32,h.height as u32,ml.idx,0);
    }
}

pub proof fn lemma_image_reservation(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM,still:bool)
    requires image_mode(b,h,pol,s,fh,g,m,d,walk,still) is Some,
    ensures image_mode(b,h,pol,s,fh,g,m,d,walk,still).unwrap().reserve_saved==(raw_disposal(g)==3 && !still && s.saved is None),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
}

pub proof fn lemma_image_state_extensional(b:Seq<u8>,h:HeaderM,pol:Policy,a:State,p:State,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM,still:bool)
    requires a.canvas==p.canvas,a.saved==p.saved,a.prev==p.prev,
    ensures image_mode(b,h,pol,a,fh,g,m,d,walk,still)==image_mode(b,h,pol,p,fh,g,m,d,walk,still),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
}

pub open spec fn call_mode(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,still:bool)->Option<Call> {
    match s.status {
        Status::Done|Status::Failed=>Some(Call {after:s,frame:None,error:None,reserve_saved:false}),
        Status::Running=>match crate::model::mb(b,h,pol,s.walk) {
            Err(e)=>Some(Call {after:State {status:Status::Failed,..s},frame:None,error:Some(e),reserve_saved:false}),
            Ok(MbOut::Trailer)=>Some(Call {after:State {status:Status::Done,..s},frame:None,error:None,reserve_saved:false}),
            Ok(MbOut::Image(fh,g,m,d,walk))=>image_mode(b,h,pol,s,fh,g,m,d,walk,still),
        },
    }
}

pub open spec fn call(b:Seq<u8>,h:HeaderM,pol:Policy,s:State)->Option<Call> {
    call_mode(b,h,pol,s,false)
}

/// State before call k; unlike a sequence of FrameRec values, this retains
/// the terminal call and every later fused call.
pub open spec fn after_calls(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat)->Option<State>
    decreases k,
{
    if k==0 {initial(b,pol)} else {
        match after_calls(b,h,pol,(k-1) as nat) {
            None=>None,Some(s)=>match call(b,h,pol,s) {None=>None,Some(c)=>Some(c.after)},
        }
    }
}

pub open spec fn call_at(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat)->Option<Call> {
    match after_calls(b,h,pol,k) {None=>None,Some(s)=>call(b,h,pol,s)}
}

/// First Saved reservation, including a frame that subsequently fails ML or
/// the colour check. Earlier terminal calls make later reservations impossible.
pub open spec fn saved_at(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat)->bool {
    &&& call_at(b,h,pol,k) is Some
    &&& call_at(b,h,pol,k).unwrap().reserve_saved
    &&& forall|j:nat| j<k ==> call_at(b,h,pol,j) is Some && !call_at(b,h,pol,j).unwrap().reserve_saved
}

pub proof fn lemma_finish_progress(b:Seq<u8>,pol:Policy,s:WalkM,fh:FrameHeaderM,area:int,c:int,out:FrameHeaderM,g:Option<GceM>,m:int,data:Seq<u8>,walk:WalkM)
    requires 0<=c,crate::model::finish_image(b,pol,s,fh,area,c)==Ok((out,g,m,data,walk)),
    ensures out==fh,c<walk.p<=b.len(),
{
    hide(crate::model::sub_scan);
    let scan=crate::model::sub_scan(b,c+1).unwrap();
    crate::parse::lemma_sub_scan_bounds(b,c+1,scan.0,scan.1);
}

pub proof fn lemma_descriptor_domain(b:Seq<u8>,h:HeaderM,pol:Policy,s:WalkM,fh:FrameHeaderM,g:Option<GceM>,m:int,data:Seq<u8>,walk:WalkM)
    requires crate::model::mh_bounds(b,h),0<=s.p,crate::model::image_step(b,h,pol,s)==Ok((fh,g,m,data,walk)),
    ensures image_domain(b,h,fh),s.p<walk.p<=b.len(),
{
    hide(crate::model::finish_image);
    crate::model::lemma_le16_lt(b,s.p+1);crate::model::lemma_le16_lt(b,s.p+3);
    crate::model::lemma_le16_lt(b,s.p+5);crate::model::lemma_le16_lt(b,s.p+7);
    assert(crate::limits::MAX_DIMENSION==u16::MAX) by(compute);
    let left=crate::model::le16(b,s.p+1);
    let top=crate::model::le16(b,s.p+3);
    let w=crate::model::le16(b,s.p+5);
    let ht=crate::model::le16(b,s.p+7);
    let packed=b[s.p+9];
    let tab=if crate::model::flag_lct(packed) {
        crate::model::ColorTableM {off:s.p+10,entries:crate::model::table_entries(packed)}
    } else {h.global.unwrap()};
    crate::model::lemma_table_entries_range(packed);
    let desc=FrameHeaderM {left,top,w,h:ht,interlaced:crate::model::flag_interlace(packed),table:tab,inside:left+w<=h.width && top+ht<=h.height};
    let c=if crate::model::flag_lct(packed) {s.p+10+3*tab.entries} else {s.p+10};
    lemma_finish_progress(b,pol,s,desc,w*ht,c,fh,g,m,data,walk);
}

pub proof fn lemma_mb_image_progress(b:Seq<u8>,h:HeaderM,pol:Policy,s:WalkM,fuel:nat,fh:FrameHeaderM,g:Option<GceM>,m:int,data:Seq<u8>,walk:WalkM)
    requires crate::model::mh_bounds(b,h),0<=s.p,
        crate::model::mb_fuel(b,h,pol,s,fuel)==Ok(MbOut::Image(fh,g,m,data,walk)),
    ensures image_domain(b,h,fh),s.p<walk.p<=b.len(),
    decreases fuel,
{
    reveal_with_fuel(crate::model::mb_fuel,1);
    if fuel>0 && s.p<b.len() {
        if b[s.p]==0x21 {
            let ns=crate::model::ext_step(b,pol,s).unwrap();
            crate::parse::lemma_ext_success_progress(b,pol,s,ns);
            lemma_mb_image_progress(b,h,pol,ns,(fuel-1) as nat,fh,g,m,data,walk);
        } else if b[s.p]==0x2c {
            lemma_descriptor_domain(b,h,pol,s,fh,g,m,data,walk);
        }
    }
}

pub proof fn lemma_call_progress(b:Seq<u8>,h:HeaderM,pol:Policy,s:State)
    requires crate::model::mh_bounds(b,h),0<=s.walk.p<=b.len(),
    ensures call(b,h,pol,s) is Some,
        ({let c=call(b,h,pol,s).unwrap();
            &&& 0<=c.after.walk.p<=b.len()
            &&& match c.frame {
                Some(_)=>s.walk.p<c.after.walk.p && c.after.status==Status::Running && c.error is None,
                None=>c.after.status!=Status::Running,
            }
        }),
{
    hide(crate::model::mb);hide(image_mode);
    if s.status==Status::Running {
        match crate::model::mb(b,h,pol,s.walk) {
            Ok(MbOut::Image(fh,g,m,data,walk))=>{
                assert(crate::model::mb_fuel(b,h,pol,s.walk,(b.len()-s.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,data,walk))) by {reveal(crate::model::mb);}
                lemma_mb_image_progress(b,h,pol,s.walk,(b.len()-s.walk.p) as nat,fh,g,m,data,walk);
                lemma_image_defined(b,h,pol,s,fh,g,m,data,walk,false);
            },
            _=>{},
        }
    }
}

pub proof fn lemma_mp_walk_bounds(b:Seq<u8>,pol:Policy,s:WalkM,fuel:nat,t:WalkM)
    requires 0<=s.p<=b.len(),crate::model::mp_fuel(b,pol,s,fuel)==Ok(t),
    ensures 0<=t.p<=b.len(),
    decreases fuel,
{
    reveal_with_fuel(crate::model::mp_fuel,1);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 {
        let ns=crate::model::ext_step(b,pol,s).unwrap();
        crate::parse::lemma_ext_success_progress(b,pol,s,ns);
        lemma_mp_walk_bounds(b,pol,ns,(fuel-1) as nat,t);
    }
}

pub proof fn lemma_initial_domain(b:Seq<u8>,pol:Policy)
    requires crate::model::mp(b,pol) is Ok,
    ensures crate::model::mh(b) is Ok,initial(b,pol) is Some,
        crate::model::mh_bounds(b,crate::model::mh(b).unwrap()),0<=initial(b,pol).unwrap().walk.p<=b.len(),
{
    hide(crate::model::mp_fuel);
    let h=crate::model::mh(b).unwrap();
    let s=crate::model::mp(b,pol).unwrap().1;
    crate::model::lemma_mh_ok_bounds(b,h);
    let seed=WalkM {p:h.cursor,gce:None,frames_seen:0,total_pixels:0,loop_count:None};
    lemma_mp_walk_bounds(b,pol,seed,(b.len()-h.cursor) as nat,s);
}

pub proof fn lemma_stream_fuel_adequate(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fuel:nat)
    requires crate::model::mh_bounds(b,h),0<=s.walk.p<=b.len(),fuel>b.len()-s.walk.p,
    ensures stream_fuel(b,h,pol,s,fuel).end!=End::OutsideDomain,
    decreases fuel,
{
    hide(call);
    reveal_with_fuel(stream_fuel,1);
    lemma_call_progress(b,h,pol,s);
    let c=call(b,h,pol,s).unwrap();
    if c.frame is Some {lemma_stream_fuel_adequate(b,h,pol,c.after,(fuel-1) as nat);}
}

pub proof fn lemma_mf_defined(b:Seq<u8>,pol:Policy)
    requires crate::model::mp(b,pol) is Ok,
    ensures mf(b,pol) is Ok,mf(b,pol).unwrap().1.end!=End::OutsideDomain,
{
    hide(crate::model::mp);hide(crate::model::mh);hide(stream_fuel);
    lemma_initial_domain(b,pol);
    lemma_stream_fuel_adequate(b,crate::model::mh(b).unwrap(),pol,initial(b,pol).unwrap(),b.len()+1);
}

pub open spec fn stream_position(c:Call,st:Stream,j:nat)->bool {
    &&& st.end!=End::OutsideDomain
    &&& if j<st.frames.len() {c.frame==Some(st.frames[j as int]) && c.error is None}
        else if j==st.frames.len() {c.frame is None && c.error==match st.end {End::Error(e)=>Some(e),_=>None}}
        else {c.frame is None && c.error is None}
}

pub proof fn lemma_terminal_at(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State,j:nat)
    requires after_calls(b,h,pol,k)==Some(s),s.status!=Status::Running,
    ensures after_calls(b,h,pol,k+j)==Some(s),
        call_at(b,h,pol,k+j)==Some(Call {after:s,frame:None,error:None,reserve_saved:false}),
    decreases j,
{
    hide(after_calls);hide(call);
    lemma_terminal_call(b,h,pol,s);
    if j>0 {
        lemma_after_calls_step(b,h,pol,k,s,Call {after:s,frame:None,error:None,reserve_saved:false});
        lemma_terminal_at(b,h,pol,k+1,s,(j-1) as nat);
    }
}

pub proof fn lemma_stream_calls(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State,fuel:nat,j:nat)
    requires crate::model::mh_bounds(b,h),0<=s.walk.p<=b.len(),fuel>b.len()-s.walk.p,
        after_calls(b,h,pol,k)==Some(s),
    ensures call_at(b,h,pol,k+j) is Some,
        stream_position(call_at(b,h,pol,k+j).unwrap(),stream_fuel(b,h,pol,s,fuel),j),
    decreases fuel,
{
    hide(call);hide(after_calls);
    reveal_with_fuel(stream_fuel,1);
    lemma_call_progress(b,h,pol,s);
    let c=call(b,h,pol,s).unwrap();
    lemma_after_calls_step(b,h,pol,k,s,c);
    match c.frame {
        None=>{
            if j>0 {lemma_terminal_at(b,h,pol,k+1,c.after,(j-1) as nat);}
        },
        Some(f)=>{
            lemma_stream_fuel_adequate(b,h,pol,c.after,(fuel-1) as nat);
            if j>0 {lemma_stream_calls(b,h,pol,k+1,c.after,(fuel-1) as nat,(j-1) as nat);}
        },
    }
}

pub proof fn lemma_mf_position(b:Seq<u8>,pol:Policy,j:nat)
    requires crate::model::mp(b,pol) is Ok,
    ensures mf(b,pol) is Ok,
        call_at(b,crate::model::mh(b).unwrap(),pol,j) is Some,
        stream_position(call_at(b,crate::model::mh(b).unwrap(),pol,j).unwrap(),mf(b,pol).unwrap().1,j),
{
    hide(crate::model::mp);hide(crate::model::mh);hide(after_calls);hide(stream_fuel);
    lemma_initial_domain(b,pol);
    let h=crate::model::mh(b).unwrap();
    let s=initial(b,pol).unwrap();
    assert(after_calls(b,h,pol,0)==Some(s)) by {reveal_with_fuel(after_calls,1);}
    lemma_stream_calls(b,h,pol,0,s,b.len()+1,j);
}

pub proof fn lemma_call_mode_defined(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,still:bool)
    requires crate::model::mh_bounds(b,h),0<=s.walk.p<=b.len(),
    ensures call_mode(b,h,pol,s,still) is Some,
{
    hide(crate::model::mb);hide(image_mode);
    if s.status==Status::Running {
        match crate::model::mb(b,h,pol,s.walk) {
            Ok(MbOut::Image(fh,g,m,data,walk))=>{
                assert(crate::model::mb_fuel(b,h,pol,s.walk,(b.len()-s.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,data,walk))) by {reveal(crate::model::mb);}
                lemma_mb_image_progress(b,h,pol,s.walk,(b.len()-s.walk.p) as nat,fh,g,m,data,walk);
                lemma_image_defined(b,h,pol,s,fh,g,m,data,walk,still);
            },_=>{},
        }
    }
}

pub proof fn lemma_ms_defined(b:Seq<u8>,pol:Policy)
    ensures ms(b,pol) is Some,
{
    hide(crate::model::mp);hide(crate::model::mh);hide(call_mode);
    if crate::model::mp(b,pol) is Ok {
        lemma_initial_domain(b,pol);
        lemma_call_mode_defined(b,crate::model::mh(b).unwrap(),pol,initial(b,pol).unwrap(),true);
    }
}

pub proof fn lemma_still_no_reserve(b:Seq<u8>,h:HeaderM,pol:Policy,s:State)
    ensures call_mode(b,h,pol,s,true) is Some ==> !call_mode(b,h,pol,s,true).unwrap().reserve_saved,
{
    hide(crate::model::mb);hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
}

pub proof fn lemma_call_saved_presence(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,c:Call)
    requires call(b,h,pol,s)==Some(c),
    ensures s.saved is Some ==> c.after.saved is Some,
        c.reserve_saved ==> c.after.saved is Some && s.saved is None,
        s.saved is None && c.after.saved is Some ==> c.reserve_saved,
{
    hide(crate::model::mb);hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
}

pub proof fn lemma_no_saved_before(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State)
    requires after_calls(b,h,pol,k)==Some(s),s.saved is None,
    ensures forall|j:nat| j<k ==> call_at(b,h,pol,j) is Some && !call_at(b,h,pol,j).unwrap().reserve_saved,
    decreases k,
{
    hide(call);
    reveal_with_fuel(after_calls,1);
    if k>0 {
        let p=after_calls(b,h,pol,(k-1) as nat).unwrap();
        let c=call(b,h,pol,p).unwrap();
        lemma_call_saved_presence(b,h,pol,p,c);
        lemma_no_saved_before(b,h,pol,(k-1) as nat,p);
        assert forall|j:nat| j<k implies call_at(b,h,pol,j) is Some && !call_at(b,h,pol,j).unwrap().reserve_saved by {
            if j==k-1 {assert(call_at(b,h,pol,j)==Some(c));}
        }
    }
}

pub proof fn lemma_first_saved_at(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State,c:Call)
    requires after_calls(b,h,pol,k)==Some(s),s.saved is None,call(b,h,pol,s)==Some(c),c.reserve_saved,
    ensures saved_at(b,h,pol,k),
{
    lemma_no_saved_before(b,h,pol,k,s);
}

pub proof fn lemma_saved_exists(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State)
    requires after_calls(b,h,pol,k)==Some(s),s.saved is Some,
    ensures exists|i:nat| i<k && saved_at(b,h,pol,i),
    decreases k,
{
    hide(call);reveal_with_fuel(after_calls,1);
    if k>0 {
        let p=after_calls(b,h,pol,(k-1) as nat).unwrap();
        let c=call(b,h,pol,p).unwrap();
        lemma_call_saved_presence(b,h,pol,p,c);
        if p.saved is Some {lemma_saved_exists(b,h,pol,(k-1) as nat,p);}
        else {lemma_first_saved_at(b,h,pol,(k-1) as nat,p,c);}
    }
}

pub proof fn lemma_saved_unique(b:Seq<u8>,h:HeaderM,pol:Policy,i:nat,j:nat)
    requires saved_at(b,h,pol,i),saved_at(b,h,pol,j),
    ensures i==j,
{
    hide(call_at);
    if i<j {assert(!call_at(b,h,pol,i).unwrap().reserve_saved);}
    if j<i {assert(!call_at(b,h,pol,j).unwrap().reserve_saved);}
}

pub proof fn lemma_saved_current(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State,c:Call)
    requires after_calls(b,h,pol,k)==Some(s),call(b,h,pol,s)==Some(c),
    ensures saved_at(b,h,pol,k)==c.reserve_saved,
{
    hide(call);
    lemma_call_saved_presence(b,h,pol,s,c);
    if c.reserve_saved {lemma_first_saved_at(b,h,pol,k,s,c);}
}

pub struct StillImage {pub width:int,pub height:int,pub rgba:Seq<u8>,pub recovered:bool}

/// MS stops after the first image and never reserves Saved. None denotes the
/// internal OutsideDomain marker, not a GIF error.
pub open spec fn ms(b:Seq<u8>,pol:Policy)->Option<Result<StillImage,Error>> {
    match crate::model::mp(b,pol) {
        Err(e)=>Some(Err(e)),
        Ok((info,_))=>match crate::model::mh(b) {
            Err(e)=>Some(Err(e)),
            Ok(h)=>{
                let s=initial(b,pol).unwrap();
                match call_mode(b,h,pol,s,true) {
                    None=>None,
                    Some(c)=>match c.error {
                        Some(e)=>Some(Err(e)),
                        None=>match c.frame {
                            None=>Some(Err(Error::NoImage)),
                            Some(f)=>Some(Ok(StillImage {width:info.width,height:info.height,rgba:f.rgba,recovered:f.recovered})),
                        },
                    },
                }
            },
        },
    }
}

pub enum End { Trailer, Error(Error), OutsideDomain }
pub struct Stream {pub frames:Seq<FrameRec>,pub end:End}

pub open spec fn stream_fuel(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fuel:nat)->Stream
    decreases fuel,
{
    if fuel==0 {Stream {frames:Seq::empty(),end:End::OutsideDomain}} else {
        match call(b,h,pol,s) {
            None=>Stream {frames:Seq::empty(),end:End::OutsideDomain},
            Some(c)=>match c.error {
                Some(e)=>Stream {frames:Seq::empty(),end:End::Error(e)},
                None=>match c.frame {
                    None=>Stream {frames:Seq::empty(),end:End::Trailer},
                    Some(f)=>{
                        let tail=stream_fuel(b,h,pol,c.after,(fuel-1) as nat);
                        Stream {frames:seq![f]+tail.frames,end:tail.end}
                    },
                },
            },
        }
    }
}

/// MF's constructor is MP, not MH. The extra unit of fuel pays for the
/// terminal call; every successful image advances the input cursor.
pub open spec fn mf(b:Seq<u8>,pol:Policy)->Result<(crate::model::InfoM,Stream),Error> {
    match crate::model::mp(b,pol) {
        Err(e)=>Err(e),
        Ok((info,_))=>match crate::model::mh(b) {
            Err(e)=>Err(e),
            Ok(h)=>Ok((info,stream_fuel(b,h,pol,initial(b,pol).unwrap(),b.len()+1))),
        },
    }
}

/// Compose a report with the independent model without exposing ML's fold or
/// the rectangle byte functions to the enclosing executable frame query.
pub proof fn lemma_image_observation(b:Seq<u8>,h:HeaderM,pol:Policy,s:State,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM,still:bool,
    rep:crate::lzw::LzwReport,out:Seq<u8>)
    requires image_domain(b,h,fh),
        crate::lzw::report_matches(rep,crate::lzw_model::ML(m,d,fh.w*fh.h,params(pol))),
        crate::lzw::colour_observation(rep,crate::lzw_model::ML(m,d,fh.w*fh.h,params(pol)).idx,
            |v:u8|frame(fh,g).transparent==Some(v) || (v as int)<frame(fh,g).c),
        rep.colour is None ==> crate::mc::draw_fold(disposed(s.canvas,s.saved,s.prev,h.width as u32,h.height as u32),frame(fh,g),
            b.subrange(fh.table.off,fh.table.off+3*fh.table.entries),h.width as u32,h.height as u32,crate::lzw_model::ML(m,d,fh.w*fh.h,params(pol)).idx,0)==Ok(out),
    ensures image_mode(b,h,pol,s,fh,g,m,d,walk,still) is Some,
        ({let c=image_mode(b,h,pol,s,fh,g,m,d,walk,still).unwrap();
            &&& c.error==crate::lzw::projected_error(rep)
            &&& c.after.walk==walk
            &&& c.reserve_saved==(raw_disposal(g)==3 && !still && s.saved is None)
            &&& c.after.saved==saved_image(s,frame(fh,g),disposed(s.canvas,s.saved,s.prev,h.width as u32,h.height as u32),h.width as u32,h.height as u32,still)
            &&& match c.error {
                Some(_)=>c.frame is None && c.after.prev==s.prev && c.after.status==Status::Failed,
                None=>c.after.status==Status::Running && c.after.canvas==out
                    && c.after.prev==Some(Previous {rect:frame(fh,g).rect,disposal:frame(fh,g).disposal})
                    && c.frame==Some(FrameRec {rgba:out,rect:(fh.left,fh.top,fh.w,fh.h),delay_cs:match g {Some(g)=>g.delay_cs,None=>0},
                        disposal:raw_disposal(g),user_input:match g {Some(g)=>g.user_input,None=>false},transparent:frame(fh,g).transparent,
                        interlaced:fh.interlaced,recovered:!fh.inside || rep.e4}),
            }
        }),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);hide(crate::mc::copy_rect);hide(crate::mc::clear_rect);
    let mr=crate::lzw_model::ML(m,d,fh.w*fh.h,params(pol));
    crate::lzw::lemma_public_projection(rep,mr,|v:u8|frame(fh,g).transparent==Some(v) || (v as int)<frame(fh,g).c);
    assert forall|e:crate::lzw_model::ErrK| crate::lzw::err_of(e)==lzw_error(e) by {};
}

pub proof fn lemma_terminal_call(b:Seq<u8>,h:HeaderM,pol:Policy,s:State)
    requires s.status!=Status::Running,
    ensures call(b,h,pol,s)==Some(Call {after:s,frame:None,error:None,reserve_saved:false}),
{}

pub proof fn lemma_after_calls_step(b:Seq<u8>,h:HeaderM,pol:Policy,k:nat,s:State,c:Call)
    requires after_calls(b,h,pol,k)==Some(s),call(b,h,pol,s)==Some(c),
    ensures after_calls(b,h,pol,k+1)==Some(c.after),
{reveal_with_fuel(after_calls,1);}

} // verus!
