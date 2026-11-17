//! OPTIONS.md / policy_oracle.py semantics for every implemented Policy value.
//! Definitions use the independent header/sub-block, string-dictionary and
//! compositor models, never executable parsing, LZW or drawing functions.
//! The singleton field enums exclude unimplemented P2 values at the type level;
//! loop_scan=FirstSubBlockOnly is implemented metadata, not conservative.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::{model as core,frame_model as fm,string_model as strings};
#[cfg(verus_only)]
use crate::model::{HeaderM,WalkM,FrameHeaderM,ColorTableM,GceM,InfoM,MbOut};
#[cfg(verus_only)]
use crate::policy::*;
#[cfg(verus_only)]
use crate::gif::Error;

verus! {

pub enum Event { E1, E2, E3, E4 }

/// OPTIONS: version has only Known; G2-G4 are unchanged for every record.
pub open spec fn mh(b:Seq<u8>,_p:Policy)->Result<HeaderM,Error> {core::mh(b)}

pub open spec fn trailer(b:Seq<u8>,p:Policy,s:WalkM)->Result<MbOut,Error> {
    if s.gce is Some && p.gce_orphan==GceOrphan::Reject {Err(Error::GraphicControlWithoutGraphic)}
    else if s.p+1<b.len() && p.trailing==Trailing::Reject {Err(Error::TrailingBytes)}
    else {Ok(MbOut::Trailer)}
}

/// policy_oracle.parse_extension: label/scope, duplicate, exact GCE structure,
/// undefined disposal, extension size, then the complete sub-block scan.
pub open spec fn extension(b:Seq<u8>,p:Policy,s:WalkM)->Result<WalkM,Error> {
    if !core::have(b,s.p,2) {Err(Error::TruncatedExtension)} else {
        let label=b[s.p+1]; let q=s.p+2;
        if label<=0x7f && s.gce is Some && p.gce_consumers==GceConsumers::Reject {
            Err(Error::UnsupportedGraphicControlTarget)
        } else if label==0xf9 {
            if s.gce is Some && p.gce_duplicate==GceDuplicate::Reject {Err(Error::DuplicateGraphicControl)}
            else if !core::have(b,q,1) {Err(Error::TruncatedExtension)}
            else if b[q]!=4 {Err(Error::InvalidGraphicControl)}
            else if !core::have(b,q,6) {Err(Error::TruncatedExtension)}
            else if b[q+5]!=0 {Err(Error::InvalidGraphicControl)} else {
                let packed=b[q+1];
                let disp=(packed>>2u8)&7u8;
                if disp>3 && p.disposal_undefined==DisposalUndefined::Reject {Err(Error::UndefinedDisposal)}
                else {Ok(WalkM {p:q+6,gce:Some(GceM {disposal:disp,user_input:packed&2u8!=0,
                    delay_cs:core::le16(b,q+2) as u16,transparent:if packed&1u8!=0 {Some(b[q+4])} else {None}}),..s})}
            }
        } else if !core::have(b,q,1) {Err(Error::TruncatedExtension)}
        else if label==1 && b[q]!=12 && p.ext_sizes==ExtSizes::Exact {Err(Error::InvalidPlainText)}
        else if label==0xff && b[q]!=11 && p.ext_sizes==ExtSizes::Exact {Err(Error::InvalidApplicationExtension)}
        else {match core::sub_scan(b,q) {
            None=>Err(Error::TruncatedExtension),
            Some((blocks,end))=>{
                let count=if label==0xff && core::is_netscape(blocks) && s.frames_seen==0 {
                    match core::loop_value(blocks.subrange(1,blocks.len() as int),p.loop_scan==LoopScan::FirstSubBlockOnly) {
                        Some(n)=>Some(n),None=>s.loop_count,
                    }
                } else {s.loop_count};
                Ok(WalkM {p:end,loop_count:count,..s})
            },
        }}
    }
}

pub open spec fn data(b:Seq<u8>,p:Policy,s:WalkM,c:int,a:int)->Result<(int,Seq<u8>,WalkM),Error> {
    if !core::have(b,c,1) {Err(Error::TruncatedImageData)} else {
        let m=b[c] as int;
        let allowed=if p.lzw_min==LzwMin::Spec {2<=m<=8} else {1<=m<=11 && (m!=1 || a==1)};
        if !allowed {Err(Error::InvalidLzwMinCodeSize)} else {match core::sub_scan(b,c+1) {
            None=>Err(Error::TruncatedImageData),
            Some((blocks,end))=>Ok((m,core::concat(blocks),WalkM {p:end,gce:None,
                frames_seen:s.frames_seen+1,total_pixels:s.total_pixels+a,loop_count:s.loop_count})),
        }}
    }
}

pub open spec fn descriptor(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM)
    ->Result<(FrameHeaderM,Option<GceM>,int,Seq<u8>,WalkM),Error>
{
    if s.frames_seen>=core::max_frames() {Err(Error::TooManyFrames)}
    else if !core::have(b,s.p,10) {Err(Error::TruncatedImageDescriptor)} else {
        let x=core::le16(b,s.p+1); let y=core::le16(b,s.p+3);
        let w=core::le16(b,s.p+5); let ht=core::le16(b,s.p+7);
        let packed=b[s.p+9]; let a=w*ht; let q=s.p+10;
        let inside=x+w<=h.width && y+ht<=h.height;
        if w==0 || ht==0 {Err(Error::InvalidFrameDimensions)}
        else if !inside && p.frame_bounds==FrameBounds::Reject {Err(Error::FrameOutsideScreen)}
        else if s.total_pixels+a>core::max_total_pixels() {Err(Error::PixelBudgetExceeded)} else {
            let table=if core::flag_lct(packed) {Some(ColorTableM {off:q,entries:core::table_entries(packed)})} else {h.global};
            if core::flag_lct(packed) && !core::have(b,q,3*core::table_entries(packed)) {Err(Error::TruncatedColorTable)}
            else {match table {
                None=>Err(Error::NoColorTable),
                Some(table)=>{
                    let c=if core::flag_lct(packed) {q+3*table.entries} else {q};
                    match data(b,p,s,c,a) {
                        Err(e)=>Err(e),
                        Ok((m,d,walk))=>Ok((FrameHeaderM {left:x,top:y,w,h:ht,
                            interlaced:core::flag_interlace(packed),table,inside},s.gce,m,d,walk)),
                    }
                },
            }}
        }
    }
}

pub open spec fn mb_fuel(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM,fuel:nat)->Result<MbOut,Error>
    decreases fuel,
{
    if fuel==0 || s.p>=b.len() {
        if p.missing_trailer==MissingTrailer::Reject {Err(Error::MissingTrailer)} else {trailer(b,p,s)}
    } else if b[s.p]==0x3b {trailer(b,p,s)}
    else if b[s.p]==0x21 {match extension(b,p,s) {
        Err(e)=>Err(e),Ok(next)=>mb_fuel(b,h,p,next,(fuel-1) as nat),
    }} else if b[s.p]==0x2c {match descriptor(b,h,p,s) {
        Err(e)=>Err(e),Ok((fh,g,m,d,walk))=>Ok(MbOut::Image(fh,g,m,d,walk)),
    }} else {Err(Error::UnknownBlock)}
}
pub open spec fn mb(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM)->Result<MbOut,Error> {
    mb_fuel(b,h,p,s,(b.len()-s.p) as nat)
}

pub open spec fn mp_fuel(b:Seq<u8>,p:Policy,s:WalkM,fuel:nat)->Result<WalkM,Error>
    decreases fuel,
{
    if fuel==0 || s.p>=b.len() {
        if p.missing_trailer==MissingTrailer::Reject {Err(Error::MissingTrailer)}
        else {match trailer(b,p,s) {Err(e)=>Err(e),Ok(_)=>Ok(s)}}
    } else if b[s.p]==0x3b {match trailer(b,p,s) {Err(e)=>Err(e),Ok(_)=>Ok(s)}}
    else if b[s.p]==0x21 {match extension(b,p,s) {Err(e)=>Err(e),Ok(next)=>mp_fuel(b,p,next,(fuel-1) as nat)}}
    else if b[s.p]==0x2c {Ok(s)} else {Err(Error::UnknownBlock)}
}
pub open spec fn mp(b:Seq<u8>,p:Policy)->Result<(InfoM,WalkM),Error> {
    match mh(b,p) {
        Err(e)=>Err(e),Ok(h)=>{
            let seed=WalkM {p:h.cursor,gce:None,frames_seen:0,total_pixels:0,loop_count:None};
            match mp_fuel(b,p,seed,(b.len()-h.cursor) as nat) {
                Err(e)=>Err(e),Ok(walk)=>Ok((InfoM {width:h.width,height:h.height,version:h.version,
                    background_index:h.background_index,aspect:h.aspect,loop_count:walk.loop_count},walk)),
            }
        },
    }
}

/// Only the LzwParams data type is shared with the decoder; no executable
/// policy mapper or decoder transition occurs in this definition.
pub open spec fn params(p:Policy)->crate::lzw::LzwParams {
    let checked=p.lzw_min==LzwMin::Range1to11Checked;
    crate::lzw::LzwParams {m_lo:if checked {1} else {2},m_hi:if checked {11} else {8},
        size1_only_one_pixel:checked,checked_literals:checked,deferred_clear:true,
        stop_at_raster:p.lzw_tail==LzwTail::StopAtRaster}
}
pub open spec fn ml(m:int,d:Seq<u8>,a:int,p:Policy)->crate::lzw_model::MRes {
    strings::ST(m,d,a,params(p))
}

/// E2 and E3 have no producing branch in the implemented Policy type: its
/// index_range, short_frame, lzw_invalid and truncation enums contain Reject only.
pub open spec fn events(p:Policy,fh:FrameHeaderM,e4:bool)->Set<Event> {
    let clip=if p.frame_bounds==FrameBounds::Clip && !fh.inside {Set::empty().insert(Event::E1)} else {Set::empty()};
    if p.lzw_tail==LzwTail::StopAtRaster && e4 {clip.insert(Event::E4)} else {clip}
}

pub struct Call {pub result:fm::Call,pub events:Set<Event>}

/// G12 uses the independent compositor. Error frames expose no partial canvas.
/// Image-domain/inside conditions are established by the structural model.
pub open spec fn render(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fh:FrameHeaderM,g:Option<GceM>,
    m:int,d:Seq<u8>,walk:WalkM,still:bool)->Option<Call>
{
    if !fm::image_domain(b,h,fh) {None} else {
        let fr=fm::frame(fh,g);
        let canvas=fm::disposed(s.canvas,s.saved,s.prev,h.width as u32,h.height as u32);
        let saved=fm::saved_image(s,fr,canvas,h.width as u32,h.height as u32,still);
        let reserve=fm::raw_disposal(g)==3 && !still && s.saved is None;
        let l=ml(m,d,fh.w*fh.h,p);
        let ev=events(p,fh,l.e4);
        let bad=exists|q:int| 0<=q<l.idx.len() && fr.transparent!=Some(#[trigger] l.idx[q]) && l.idx[q] as int>=fr.c;
        let error=if bad {Some(Error::ColorIndexOutOfRange)} else {match l.error {Some(e)=>Some(fm::lzw_error(e)),None=>None}};
        match error {
            Some(e)=>Some(Call {events:ev,result:fm::Call {after:fm::State {walk,canvas:s.canvas,saved,prev:s.prev,status:fm::Status::Failed},
                frame:None,error:Some(e),reserve_saved:reserve}}),
            None=>match crate::mc::draw_fold(canvas,fr,b.subrange(fh.table.off,fh.table.off+3*fh.table.entries),h.width as u32,h.height as u32,l.idx,0) {
                Err(_)=>None,
                Ok(out)=>Some(Call {events:ev,result:fm::Call {
                    after:fm::State {walk,canvas:out,saved,prev:Some(fm::Previous {rect:fr.rect,disposal:fr.disposal}),status:fm::Status::Running},
                    frame:Some(fm::FrameRec {rgba:out,rect:(fh.left,fh.top,fh.w,fh.h),
                        delay_cs:match g {Some(g)=>g.delay_cs,None=>0},disposal:fm::raw_disposal(g),
                        user_input:match g {Some(g)=>g.user_input,None=>false},transparent:fr.transparent,
                        interlaced:fh.interlaced,recovered:ev!=Set::<Event>::empty()}),error:None,reserve_saved:reserve}}),
            },
        }
    }
}

pub open spec fn initial(b:Seq<u8>,p:Policy)->Option<fm::State> {
    match mp(b,p) {Err(_)=>None,Ok((i,s))=>Some(fm::State {walk:s,
        canvas:Seq::new((4*i.width*i.height) as nat,|_i:int|0u8),saved:None,prev:None,
        status:if 0<=s.p<b.len() && b[s.p]==0x2c {fm::Status::Running} else {fm::Status::Done}})}
}
pub open spec fn call(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,still:bool)->Option<Call> {
    match s.status {
        fm::Status::Done|fm::Status::Failed=>Some(Call {events:Set::empty(),result:fm::Call {after:s,frame:None,error:None,reserve_saved:false}}),
        fm::Status::Running=>match mb(b,h,p,s.walk) {
            Err(e)=>Some(Call {events:Set::empty(),result:fm::Call {after:fm::State {status:fm::Status::Failed,..s},frame:None,error:Some(e),reserve_saved:false}}),
            Ok(MbOut::Trailer)=>Some(Call {events:Set::empty(),result:fm::Call {after:fm::State {status:fm::Status::Done,..s},frame:None,error:None,reserve_saved:false}}),
            Ok(MbOut::Image(fh,g,m,d,walk))=>render(b,h,p,s,fh,g,m,d,walk,still),
        },
    }
}
pub open spec fn ms(b:Seq<u8>,p:Policy)->Option<Result<fm::StillImage,Error>> {
    match mp(b,p) {Err(e)=>Some(Err(e)),Ok((i,_))=>match mh(b,p) {
        Err(e)=>Some(Err(e)),Ok(h)=>match call(b,h,p,initial(b,p).unwrap(),true) {
            None=>None,Some(c)=>match c.result.error {
                Some(e)=>Some(Err(e)),None=>match c.result.frame {
                    None=>Some(Err(Error::NoImage)),Some(f)=>Some(Ok(fm::StillImage {width:i.width,height:i.height,rgba:f.rgba,recovered:f.recovered})),
                },
            },
        },
    }}
}
pub struct Stream {pub frames:Seq<fm::FrameRec>,pub events:Seq<Set<Event>>,pub end:fm::End}
pub open spec fn stream(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fuel:nat)->Stream
    decreases fuel,
{
    if fuel==0 {Stream {frames:Seq::empty(),events:Seq::empty(),end:fm::End::OutsideDomain}} else {
        match call(b,h,p,s,false) {
            None=>Stream {frames:Seq::empty(),events:Seq::empty(),end:fm::End::OutsideDomain},
            Some(c)=>match c.result.error {
                Some(e)=>Stream {frames:Seq::empty(),events:Seq::empty(),end:fm::End::Error(e)},
                None=>match c.result.frame {
                    None=>Stream {frames:Seq::empty(),events:Seq::empty(),end:fm::End::Trailer},
                    Some(f)=>{let tail=stream(b,h,p,c.result.after,(fuel-1) as nat);
                        Stream {frames:seq![f]+tail.frames,events:seq![c.events]+tail.events,end:tail.end}},
                },
            },
        }
    }
}
pub open spec fn mf(b:Seq<u8>,p:Policy)->Result<(InfoM,Stream),Error> {
    match mp(b,p) {Err(e)=>Err(e),Ok((i,_))=>match mh(b,p) {
        Err(e)=>Err(e),Ok(h)=>Ok((i,stream(b,h,p,initial(b,p).unwrap(),b.len()+1))),
    }}
}

// Agreement proofs refer to the old indexed models only below this boundary.
pub proof fn lemma_structure(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM,c:int,a:int)
    requires 0<=s.p,
    ensures extension(b,p,s)==core::ext_step(b,p,s),trailer(b,p,s)==core::trailer_step(b,p,s),
        data(b,p,s,c,a)==core::data_step(b,p,s,c,a),descriptor(b,h,p,s)==core::image_step(b,h,p,s),
        params(p)==fm::params(p),
{
    hide(core::sub_scan);hide(core::concat);hide(core::loop_value);hide(core::is_netscape);
    if !core::have(b,s.p+2,1) {
        assert(core::sub_scan(b,s.p+2) is None) by {reveal_with_fuel(core::sub_scan,1);}
    }
}

pub proof fn lemma_walks(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM,fuel:nat)
    requires 0<=s.p,
    ensures mb_fuel(b,h,p,s,fuel)==core::mb_fuel(b,h,p,s,fuel),mp_fuel(b,p,s,fuel)==core::mp_fuel(b,p,s,fuel),
    decreases fuel,
{
    hide(extension);hide(descriptor);hide(core::ext_step);hide(core::image_step);
    reveal_with_fuel(mb_fuel,1);reveal_with_fuel(mp_fuel,1);
    reveal_with_fuel(core::mb_fuel,1);reveal_with_fuel(core::mp_fuel,1);
    lemma_structure(b,h,p,s,0,0);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 && extension(b,p,s) is Ok {
        let next=extension(b,p,s).unwrap();
        crate::parse::lemma_ext_success_progress(b,p,s,next);
        lemma_walks(b,h,p,next,(fuel-1) as nat);
    }
}

pub proof fn theorem_parser_bridge(b:Seq<u8>,p:Policy)
    ensures mh(b,p)==core::mh(b),mp(b,p)==core::mp(b,p),initial(b,p)==fm::initial(b,p),
{
    hide(core::mh);hide(core::mp_fuel);hide(mp_fuel);
    if core::mh(b) is Ok {
        let h=core::mh(b).unwrap();
        core::lemma_mh_ok_bounds(b,h);
        let seed=WalkM {p:h.cursor,gce:None,frames_seen:0,total_pixels:0,loop_count:None};
        lemma_walks(b,h,p,seed,(b.len()-h.cursor) as nat);
    }
}

pub proof fn theorem_lzw_bridge(m:int,d:Seq<u8>,a:int,p:Policy)
    ensures ml(m,d,a,p)==crate::lzw_model::ML(m,d,a,fm::params(p)),
{
    hide(strings::fold);hide(crate::lzw_model::L);
    if 1<=m<=11 && (m<=8 || params(p).checked_literals) {strings::theorem_ml_st(m,d,a,params(p));}
}

pub proof fn lemma_events(p:Policy,fh:FrameHeaderM,e4:bool)
    ensures events(p,fh,e4)!=Set::<Event>::empty()
        <==> (p.frame_bounds==FrameBounds::Clip && !fh.inside || p.lzw_tail==LzwTail::StopAtRaster && e4),
        events(p,fh,e4).contains(Event::E1) <==> p.frame_bounds==FrameBounds::Clip && !fh.inside,
        events(p,fh,e4).contains(Event::E4) <==> p.lzw_tail==LzwTail::StopAtRaster && e4,
        !events(p,fh,e4).contains(Event::E2),!events(p,fh,e4).contains(Event::E3),
{
    let ev=events(p,fh,e4);
    if p.frame_bounds==FrameBounds::Clip && !fh.inside {
        assert(ev.contains(Event::E1));
        assert(!Set::<Event>::empty().contains(Event::E1));
    } else if p.lzw_tail==LzwTail::StopAtRaster && e4 {
        assert(ev.contains(Event::E4));
        assert(!Set::<Event>::empty().contains(Event::E4));
    }
}

pub open spec fn erase_call(c:Option<Call>)->Option<fm::Call> {
    match c {None=>None,Some(c)=>Some(c.result)}
}
pub proof fn lemma_render_bridge(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fh:FrameHeaderM,g:Option<GceM>,
    m:int,d:Seq<u8>,walk:WalkM,still:bool)
    requires fh.inside || p.frame_bounds==FrameBounds::Clip,
    ensures erase_call(render(b,h,p,s,fh,g,m,d,walk,still))==fm::image_mode(b,h,p,s,fh,g,m,d,walk,still),
        match render(b,h,p,s,fh,g,m,d,walk,still) {
            Some(c)=>c.result.frame is Some ==> c.result.frame.unwrap().recovered==(c.events!=Set::<Event>::empty()),
            None=>true,
        },
{
    hide(crate::mc::draw_fold);hide(crate::lzw_model::ML);hide(ml);
    theorem_lzw_bridge(m,d,fh.w*fh.h,p);
    if p.lzw_tail==LzwTail::Strict {crate::policy_proofs::lemma_no_e4(m,d,fh.w*fh.h,fm::params(p));}
    lemma_events(p,fh,ml(m,d,fh.w*fh.h,p).e4);
}

pub proof fn lemma_call_bridge(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,still:bool)
    requires 0<=s.walk.p,
    ensures erase_call(call(b,h,p,s,still))==fm::call_mode(b,h,p,s,still),
        match call(b,h,p,s,still) {
            Some(c)=>c.result.frame is Some ==> c.result.frame.unwrap().recovered==(c.events!=Set::<Event>::empty()),
            None=>true,
        },
{
    hide(render);hide(fm::image_mode);hide(mb);hide(core::mb);
    if s.status==fm::Status::Running {
        lemma_walks(b,h,p,s.walk,(b.len()-s.walk.p) as nat);
        assert(mb(b,h,p,s.walk)==core::mb(b,h,p,s.walk)) by {reveal(mb);reveal(core::mb);}
        match mb(b,h,p,s.walk) {
            Ok(MbOut::Image(fh,g,m,d,walk))=>{
                if p.frame_bounds==FrameBounds::Reject {
                    assert(core::mb_fuel(b,h,p,s.walk,(b.len()-s.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,d,walk))) by {reveal(core::mb);}
                    crate::policy_proofs::lemma_reject_inside(b,h,p,s.walk,(b.len()-s.walk.p) as nat,fh,g,m,d,walk);
                }
                lemma_render_bridge(b,h,p,s,fh,g,m,d,walk,still);
            },
            _=>{},
        }
    }
}

pub proof fn theorem_ms_bridge(b:Seq<u8>,p:Policy)
    ensures ms(b,p)==fm::ms(b,p),
{
    hide(core::mp);hide(core::mh);hide(mp);hide(call);hide(fm::call_mode);
    theorem_parser_bridge(b,p);
    if core::mp(b,p) is Ok {
        fm::lemma_initial_domain(b,p);
        lemma_call_bridge(b,core::mh(b).unwrap(),p,fm::initial(b,p).unwrap(),true);
    }
}

pub proof fn lemma_stream_bridge(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fuel:nat)
    requires core::mh_bounds(b,h),0<=s.walk.p<=b.len(),
    ensures stream(b,h,p,s,fuel).frames==fm::stream_fuel(b,h,p,s,fuel).frames,
        stream(b,h,p,s,fuel).end==fm::stream_fuel(b,h,p,s,fuel).end,
        stream(b,h,p,s,fuel).events.len()==stream(b,h,p,s,fuel).frames.len(),
        forall|i:int| 0<=i<stream(b,h,p,s,fuel).frames.len()
            ==> #[trigger] stream(b,h,p,s,fuel).frames[i].recovered==(stream(b,h,p,s,fuel).events[i]!=Set::<Event>::empty()),
    decreases fuel,
{
    hide(call);hide(fm::call);hide(fm::call_mode);
    reveal_with_fuel(stream,1);reveal_with_fuel(fm::stream_fuel,1);
    if fuel>0 {
        lemma_call_bridge(b,h,p,s,false);
        assert(fm::call(b,h,p,s)==erase_call(call(b,h,p,s,false))) by {reveal(fm::call);}
        fm::lemma_call_progress(b,h,p,s);
        match call(b,h,p,s,false) {
            Some(c)=>if c.result.error is None && c.result.frame is Some {
                lemma_stream_bridge(b,h,p,c.result.after,(fuel-1) as nat);
                let tail=stream(b,h,p,c.result.after,(fuel-1) as nat);
                let result=stream(b,h,p,s,fuel);
                assert forall|i:int| 0<=i<result.frames.len() implies
                    #[trigger] result.frames[i].recovered==(result.events[i]!=Set::<Event>::empty()) by {
                    if i==0 {assert(result.frames[i]==c.result.frame.unwrap());assert(result.events[i]==c.events);}
                    else {assert(result.frames[i]==tail.frames[i-1]);assert(result.events[i]==tail.events[i-1]);}
                }
            },
            _=>{},
        }
    }
}

pub open spec fn erase_mf(r:Result<(InfoM,Stream),Error>)->Result<(InfoM,fm::Stream),Error> {
    match r {Err(e)=>Err(e),Ok((i,s))=>Ok((i,fm::Stream {frames:s.frames,end:s.end}))}
}

/// All implemented records, including test-only combinations and metadata.
/// The Strict specialization is P-BRIDGE; this stronger bridge also supplies
/// the independent semantic model needed by P-T9 on the actual decoder paths.
pub proof fn theorem_mf_bridge(b:Seq<u8>,p:Policy)
    ensures erase_mf(mf(b,p))==fm::mf(b,p),
        match mf(b,p) {Ok((_,s))=>s.events.len()==s.frames.len()
            && forall|i:int| 0<=i<s.frames.len() ==> #[trigger] s.frames[i].recovered==(s.events[i]!=Set::<Event>::empty()),_=>true},
{
    hide(core::mh);hide(core::mp);hide(mp);hide(stream);hide(fm::stream_fuel);
    theorem_parser_bridge(b,p);
    if core::mp(b,p) is Ok {
        fm::lemma_initial_domain(b,p);
        lemma_stream_bridge(b,core::mh(b).unwrap(),p,fm::initial(b,p).unwrap(),b.len()+1);
    }
}

pub(crate) proof fn theorem_strict_bridge(b:Seq<u8>,m:int,d:Seq<u8>,a:int)
    ensures mh(b,STRICT)==core::mh(b),mp(b,STRICT)==core::mp(b,STRICT),
        ml(m,d,a,STRICT)==crate::lzw_model::ML(m,d,a,crate::lzw::strict_params()),
        ms(b,STRICT)==fm::ms(b,STRICT),erase_mf(mf(b,STRICT))==fm::mf(b,STRICT),
        match ms(b,STRICT) {Some(Ok(s))=>!s.recovered,_=>true},
        match mf(b,STRICT) {Ok((_,s))=>forall|i:int| 0<=i<s.frames.len()
            ==> !#[trigger] s.frames[i].recovered && s.events[i]==Set::<Event>::empty(),_=>true},
{
    hide(ms);hide(mf);hide(fm::ms);hide(fm::mf);
    theorem_parser_bridge(b,STRICT);theorem_lzw_bridge(m,d,a,STRICT);
    theorem_ms_bridge(b,STRICT);theorem_mf_bridge(b,STRICT);
    crate::policy_proofs::theorem_strict_no_recovery(b);
    if mf(b,STRICT) is Ok {
        let s=mf(b,STRICT).unwrap().1;
        assert forall|i:int| 0<=i<s.frames.len() implies
            !#[trigger] s.frames[i].recovered && s.events[i]==Set::<Event>::empty() by {
            assert(fm::mf(b,STRICT).unwrap().1.frames==s.frames);
            assert(!s.frames[i].recovered);
            assert(s.frames[i].recovered==(s.events[i]!=Set::<Event>::empty()));
        }
    }
}

/// E4's independent event site: the ST raster-completing code carries a
/// string beyond the descriptor area. This does not inspect recovered/events.
pub open spec fn raster_overrun_site(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State)->bool {
    match mb(b,h,p,s.walk) {
        Ok(MbOut::Image(fh,_,m,d,_))=>p.lzw_tail==LzwTail::StopAtRaster && ml(m,d,fh.w*fh.h,p).e4,
        _=>false,
    }
}
pub open spec fn clip_site(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State)->bool {
    match mb(b,h,p,s.walk) {Ok(MbOut::Image(fh,_,_,_,_))=>p.frame_bounds==FrameBounds::Clip && !fh.inside,_=>false}
}
pub open spec fn rectangle_inside(f:fm::FrameRec,w:int,h:int)->bool {
    f.rect.0+f.rect.2<=w && f.rect.1+f.rect.3<=h
}

/// A parallel model trace records the independent E4 site for each delivered
/// frame. Its booleans come from ST, not from an event set or recovery flag.
pub open spec fn overrun_trace(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fuel:nat)->Seq<bool>
    decreases fuel,
{
    if fuel==0 {Seq::empty()} else {match call(b,h,p,s,false) {
        Some(c)=>if c.result.error is None && c.result.frame is Some {
            seq![raster_overrun_site(b,h,p,s)]+overrun_trace(b,h,p,c.result.after,(fuel-1) as nat)
        } else {Seq::empty()},
        None=>Seq::empty(),
    }}
}
pub open spec fn overruns(b:Seq<u8>,p:Policy)->Seq<bool> {
    match mh(b,p) {Ok(h)=>match initial(b,p) {Some(s)=>overrun_trace(b,h,p,s,b.len()+1),None=>Seq::empty()},Err(_)=>Seq::empty()}
}
pub open spec fn still_events(b:Seq<u8>,p:Policy)->Set<Event> {
    match mh(b,p) {Ok(h)=>match initial(b,p) {
        Some(s)=>match call(b,h,p,s,true) {Some(c)=>c.events,None=>Set::empty()},None=>Set::empty(),
    },Err(_)=>Set::empty()}
}
pub open spec fn still_clip_site(b:Seq<u8>,p:Policy)->bool {
    match mh(b,p) {Ok(h)=>match initial(b,p) {Some(s)=>clip_site(b,h,p,s),None=>false},Err(_)=>false}
}
pub open spec fn still_overrun_site(b:Seq<u8>,p:Policy)->bool {
    match mh(b,p) {Ok(h)=>match initial(b,p) {Some(s)=>raster_overrun_site(b,h,p,s),None=>false},Err(_)=>false}
}

pub proof fn lemma_bounds_record(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM,fuel:nat,
    fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM)
    requires mb_fuel(b,h,p,s,fuel)==Ok(MbOut::Image(fh,g,m,d,walk)),
    ensures fh.inside==(fh.left+fh.w<=h.width && fh.top+fh.h<=h.height),
    decreases fuel,
{
    hide(extension);hide(data);
    reveal_with_fuel(mb_fuel,1);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 {
        lemma_bounds_record(b,h,p,extension(b,p,s).unwrap(),(fuel-1) as nat,fh,g,m,d,walk);
    }
}

pub proof fn lemma_call_event_sites(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,still:bool,c:Call)
    requires call(b,h,p,s,still)==Some(c),c.result.frame is Some,
    ensures c.result.frame.unwrap().recovered==(c.events!=Set::<Event>::empty()),
        c.events.contains(Event::E1)==clip_site(b,h,p,s),
        c.events.contains(Event::E1)==(p.frame_bounds==FrameBounds::Clip && !rectangle_inside(c.result.frame.unwrap(),h.width,h.height)),
        c.events.contains(Event::E4)==raster_overrun_site(b,h,p,s),
        !c.events.contains(Event::E2),!c.events.contains(Event::E3),
{
    hide(ml);hide(crate::mc::draw_fold);hide(mb);
    if s.status==fm::Status::Running {
        match mb(b,h,p,s.walk) {
            Ok(MbOut::Image(fh,g,m,d,walk))=>{
                lemma_events(p,fh,ml(m,d,fh.w*fh.h,p).e4);
                assert(mb_fuel(b,h,p,s.walk,(b.len()-s.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,d,walk))) by {reveal(mb);}
                lemma_bounds_record(b,h,p,s.walk,(b.len()-s.walk.p) as nat,fh,g,m,d,walk);
            },_=>{},
        }
    }
}

pub proof fn lemma_stream_event_sites(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fuel:nat)
    requires core::mh_bounds(b,h),0<=s.walk.p<=b.len(),
    ensures overrun_trace(b,h,p,s,fuel).len()==stream(b,h,p,s,fuel).frames.len(),
        stream(b,h,p,s,fuel).events.len()==stream(b,h,p,s,fuel).frames.len(),
        forall|i:int| #![trigger stream(b,h,p,s,fuel).frames[i]] 0<=i<stream(b,h,p,s,fuel).frames.len() ==> {
            let f=stream(b,h,p,s,fuel).frames[i]; let ev=stream(b,h,p,s,fuel).events[i];
            &&& f.recovered==(ev!=Set::<Event>::empty())
            &&& ev.contains(Event::E1)==(p.frame_bounds==FrameBounds::Clip && !rectangle_inside(f,h.width,h.height))
            &&& ev.contains(Event::E4)==overrun_trace(b,h,p,s,fuel)[i]
            &&& !ev.contains(Event::E2) && !ev.contains(Event::E3)
        },
    decreases fuel,
{
    hide(call);hide(fm::call_mode);hide(fm::call);
    reveal_with_fuel(stream,1);reveal_with_fuel(overrun_trace,1);
    if fuel>0 {
        lemma_call_bridge(b,h,p,s,false);fm::lemma_call_progress(b,h,p,s);
        assert(fm::call(b,h,p,s)==erase_call(call(b,h,p,s,false))) by {reveal(fm::call);}
        match call(b,h,p,s,false) {
            Some(c)=>if c.result.error is None && c.result.frame is Some {
                lemma_call_event_sites(b,h,p,s,false,c);
                lemma_stream_event_sites(b,h,p,c.result.after,(fuel-1) as nat);
                let result=stream(b,h,p,s,fuel);let tail=stream(b,h,p,c.result.after,(fuel-1) as nat);
                assert forall|i:int| #![trigger result.frames[i]] 0<=i<result.frames.len() implies {
                    let f=result.frames[i]; let ev=result.events[i];
                    &&& f.recovered==(ev!=Set::<Event>::empty())
                    &&& ev.contains(Event::E1)==(p.frame_bounds==FrameBounds::Clip && !rectangle_inside(f,h.width,h.height))
                    &&& ev.contains(Event::E4)==overrun_trace(b,h,p,s,fuel)[i]
                    &&& !ev.contains(Event::E2) && !ev.contains(Event::E3)
                } by {
                    if i==0 {assert(result.frames[i]==c.result.frame.unwrap());assert(result.events[i]==c.events);}
                    else {assert(result.frames[i]==tail.frames[i-1]);assert(result.events[i]==tail.events[i-1]);
                        assert(overrun_trace(b,h,p,s,fuel)[i]==overrun_trace(b,h,p,c.result.after,(fuel-1) as nat)[i-1]);}
                }
            },_=>{},
        }
    }
}

pub proof fn theorem_mf_event_sites(b:Seq<u8>,p:Policy)
    ensures match mf(b,p) {Err(_)=>true,Ok((info,s))=>{
        &&& overruns(b,p).len()==s.frames.len() && s.events.len()==s.frames.len()
        &&& forall|i:int| #![trigger s.frames[i]] 0<=i<s.frames.len() ==> {
            let f=s.frames[i];let ev=s.events[i];
            &&& f.recovered==(ev!=Set::<Event>::empty())
            &&& ev.contains(Event::E1)==(p.frame_bounds==FrameBounds::Clip && !rectangle_inside(f,info.width,info.height))
            &&& ev.contains(Event::E4)==overruns(b,p)[i]
            &&& !ev.contains(Event::E2) && !ev.contains(Event::E3)
        }
    }},
{
    hide(core::mh);hide(core::mp);hide(mp);hide(stream);hide(overrun_trace);
    theorem_parser_bridge(b,p);
    if core::mp(b,p) is Ok {
        fm::lemma_initial_domain(b,p);
        assert(core::mp(b,p).unwrap().0.width==core::mh(b).unwrap().width
            && core::mp(b,p).unwrap().0.height==core::mh(b).unwrap().height) by {reveal(core::mp);}
        lemma_stream_event_sites(b,core::mh(b).unwrap(),p,initial(b,p).unwrap(),b.len()+1);
    }
}

pub proof fn theorem_ms_event_sites(b:Seq<u8>,p:Policy)
    ensures match ms(b,p) {Some(Ok(s))=>{
        let ev=still_events(b,p);
        &&& s.recovered==(ev!=Set::<Event>::empty())
        &&& ev.contains(Event::E1)==still_clip_site(b,p)
        &&& ev.contains(Event::E4)==still_overrun_site(b,p)
        &&& !ev.contains(Event::E2) && !ev.contains(Event::E3)
    },_=>true},
{
    hide(core::mh);hide(core::mp);hide(mp);hide(call);
    if ms(b,p) is Some && ms(b,p).unwrap() is Ok {
        let h=mh(b,p).unwrap();let s=initial(b,p).unwrap();
        lemma_call_event_sites(b,h,p,s,true,call(b,h,p,s,true).unwrap());
    }
}

} // verus!
