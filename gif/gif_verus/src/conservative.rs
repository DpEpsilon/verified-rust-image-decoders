//! L-TAIL and conservative-record simulation. No executable transition is
//! used: structural steps, independent string dictionaries and MC supply the
//! three synchronization boundaries of OPTIONS.md.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::{model as core,frame_model as fm,lzw_model as ml,string_model as st,policy_model as policy};
#[cfg(verus_only)]
use crate::model::{HeaderM,WalkM,FrameHeaderM,GceM,MbOut};
#[cfg(verus_only)]
use crate::policy::*;

verus! {

/// Every non-Strict variant in these ten fields is declared error-site-only.
/// The only additional implemented variant, FirstSubBlockOnly, is excluded:
/// it can change Info on Strict-accepted inputs (OPTIONS.md).
pub open spec fn conservative(p:Policy)->bool {p.loop_scan==LoopScan::EverySubBlock}

/// An exact LSB-first trace, including successful EOI padding. The last code
/// is EOI; every earlier code is Clear at the width in effect for that read.
pub open spec fn tail_trace(m:int,d:Seq<u8>,s:ml::St,codes:Seq<int>,end:int)->bool
    decreases codes.len(),
{
    if codes.len()==0 {false} else {
        let c=ml::code_at(d,s.pos,s.width,0); let after=s.pos+s.width;
        &&& 0<=s.pos && 1<=s.width && after<=ml::nbits(d)
        &&& codes[0]==c
        &&& if codes.len()==1 {c==ml::lzw_eoi_of(m) && end==after && !ml::eoi_tail(after,d)}
            else {c==ml::lzw_clear_of(m) && tail_trace(m,d,ml::clear_st(m,s,after),codes.drop_first(),end)}
    }
}

pub proof fn lemma_tail_trace_shape(m:int,d:Seq<u8>,s:ml::St,codes:Seq<int>,end:int)
    requires tail_trace(m,d,s,codes,end),
    ensures codes.len()>=1,codes.last()==ml::lzw_eoi_of(m),
        forall|i:int| 0<=i<codes.len()-1 ==> #[trigger] codes[i]==ml::lzw_clear_of(m),
    decreases codes.len(),
{
    reveal_with_fuel(tail_trace,1);
    if codes.len()>1 {
        lemma_tail_trace_shape(m,d,ml::clear_st(m,s,s.pos+s.width),codes.drop_first(),end);
        assert forall|i:int| 0<=i<codes.len()-1 implies #[trigger] codes[i]==ml::lzw_clear_of(m) by {
            if i>0 {assert(codes[i]==codes.drop_first()[i-1]);}
        }
    }
}

/// Physical dictionary storage and every emitted index survive a Clear. The
/// discarded abstract dictionary entries are unreadable after next resets.
pub proof fn lemma_clear_tail_neutral(m:int,s:ml::St,after:int)
    ensures ml::clear_st(m,s,after).q==s.q,ml::clear_st(m,s,after).o==s.o,
        ml::clear_st(m,s,after).pf==s.pf,ml::clear_st(m,s,after).sf==s.sf,
{}

pub proof fn theorem_l_tail(m:int,d:Seq<u8>,a:int,ps:crate::lzw::LzwParams,s:ml::St)
    requires 1<=m<=11,!ps.stop_at_raster,s.q==a,0<=s.pos,1<=s.width,
        ml::L(m,d,a,ps,s).error is None,
    ensures ml::L(m,d,a,ps,s).idx==s.o,!ml::L(m,d,a,ps,s).e4,
        exists|codes:Seq<int>| tail_trace(m,d,s,codes,ml::L(m,d,a,ps,s).pos),
    decreases ml::nbits(d)-s.pos,
{
    hide(ml::L);hide(ml::semit);hide(ml::first_of);
    ml::lemma_L_unfold(m,d,a,ps,s);
    match ml::verdict(m,d,a,ps,s) {
        ml::Vd::Step(next)=>{
            assert(next==ml::clear_st(m,s,s.pos+s.width));
            lemma_clear_tail_neutral(m,s,s.pos+s.width);
            theorem_l_tail(m,d,a,ps,next);
            let codes=choose|codes:Seq<int>| tail_trace(m,d,next,codes,ml::L(m,d,a,ps,next).pos);
            let all=seq![ml::lzw_clear_of(m)]+codes;
            assert(all.drop_first()==codes) by {assert(all.drop_first()=~=codes);}
            assert(tail_trace(m,d,s,all,ml::L(m,d,a,ps,s).pos));
        },
        ml::Vd::Halt(r)=>{
            assert(tail_trace(m,d,s,seq![ml::lzw_eoi_of(m)],r.pos));
        },
    }
}

pub proof fn lemma_small_clear(m:int)
    requires 2<=m<=8,
    ensures ml::lzw_clear_of(m)<=256,
{
    reveal_with_fuel(ml::lzw_ipow2,9);
}

/// Independent string-model tail: successful post-raster iterations cannot
/// resolve a dictionary string, emit an index or raise E4.
pub(crate) proof fn lemma_string_tail(m:int,d:Seq<u8>,a:int,s:st::State)
    requires 2<=m<=8,s.q==a,0<=s.pos,1<=s.width,
        st::fold(m,d,a,policy::params(STRICT),s).error is None,
    ensures st::fold(m,d,a,policy::params(STRICT),s).idx==s.out,
        !st::fold(m,d,a,policy::params(STRICT),s).e4,
    decreases ml::nbits(d)-s.pos,
{
    hide(st::fold);hide(st::resolve);
    reveal_with_fuel(st::fold,1);
    match st::step(m,d,a,policy::params(STRICT),s) {
        st::Step::Continue(next)=>{
            assert(next==st::clear(m,s,s.pos+s.width));
            lemma_string_tail(m,d,a,next);
        },
        _=>{},
    }
}

/// S2: before raster completion every successful Strict iteration is the
/// identical string-state transition under any conservative record. A
/// raster-crossing string is excluded by the Strict-success premise.
pub(crate) proof fn lemma_s2_step(m:int,d:Seq<u8>,a:int,p:Policy,s:st::State,next:st::State)
    requires 2<=m<=8,s.q<a,st::step(m,d,a,policy::params(STRICT),s)==st::Step::Continue(next),
    ensures st::step(m,d,a,policy::params(p),s)==st::Step::Continue(next),
{
    hide(st::resolve);hide(st::emit);hide(st::clear);
    lemma_small_clear(m);
}

/// LZW observations at S3; final bit position/dictionary are deliberately not
/// compared. Equality of the working canvas follows from equal emitted indices.
pub(crate) proof fn lemma_lzw_simulation(m:int,d:Seq<u8>,a:int,p:Policy,s:st::State)
    requires 2<=m<=8,0<=s.q<=a,0<=s.pos,1<=s.width,
        st::fold(m,d,a,policy::params(STRICT),s).error is None,
    ensures st::fold(m,d,a,policy::params(p),s).error is None,
        st::fold(m,d,a,policy::params(p),s).idx==st::fold(m,d,a,policy::params(STRICT),s).idx,
        !st::fold(m,d,a,policy::params(p),s).e4,
    decreases ml::nbits(d)-s.pos,
{
    hide(st::fold);hide(st::resolve);
    reveal_with_fuel(st::fold,1);
    lemma_small_clear(m);
    if s.q==a && p.lzw_tail==LzwTail::StopAtRaster {
        lemma_string_tail(m,d,a,s);
    } else {
        match st::step(m,d,a,policy::params(STRICT),s) {
            st::Step::Continue(next)=>{
                assert(st::step(m,d,a,policy::params(p),s)==st::Step::Continue(next));
                lemma_lzw_simulation(m,d,a,p,next);
            },
            st::Step::Halt(r)=>{
                assert(st::step(m,d,a,policy::params(p),s)==st::Step::Halt(r));
            },
        }
    }
}

pub(crate) proof fn theorem_lzw_conservative(m:int,d:Seq<u8>,a:int,p:Policy)
    requires 2<=m<=8,a>=1,policy::ml(m,d,a,STRICT).error is None,
    ensures policy::ml(m,d,a,p).error is None,
        policy::ml(m,d,a,p).idx==policy::ml(m,d,a,STRICT).idx,!policy::ml(m,d,a,p).e4,
{
    hide(st::fold);
    lemma_lzw_simulation(m,d,a,p,st::initial(m));
}

/// Nine pure values: their relaxed branches cannot change any successful
/// Strict structural step. The only nontrivial metadata choice is fixed by
/// conservative(p); all remaining non-Strict values are the declared ten.
pub(crate) proof fn lemma_pure_parser_steps(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM,c:int,a:int)
    requires conservative(p),0<=s.p,
    ensures core::ext_step(b,STRICT,s) is Ok ==> core::ext_step(b,p,s)==core::ext_step(b,STRICT,s),
        core::trailer_step(b,STRICT,s) is Ok ==> core::trailer_step(b,p,s)==core::trailer_step(b,STRICT,s),
        core::data_step(b,STRICT,s,c,a) is Ok ==> core::data_step(b,p,s,c,a)==core::data_step(b,STRICT,s,c,a),
        core::image_step(b,h,STRICT,s) is Ok ==> core::image_step(b,h,p,s)==core::image_step(b,h,STRICT,s),
{
    hide(core::sub_scan);hide(core::concat);hide(core::loop_value);hide(core::is_netscape);
}

/// S1: every successful Strict block/constructor segment reaches the same
/// cursor and complete walk state under any declared-value combination.
pub(crate) proof fn lemma_s1_parser(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM,fuel:nat)
    requires conservative(p),0<=s.p,
    ensures core::mb_fuel(b,h,STRICT,s,fuel) is Ok ==> core::mb_fuel(b,h,p,s,fuel)==core::mb_fuel(b,h,STRICT,s,fuel),
        core::mp_fuel(b,STRICT,s,fuel) is Ok ==> core::mp_fuel(b,p,s,fuel)==core::mp_fuel(b,STRICT,s,fuel),
    decreases fuel,
{
    hide(core::ext_step);hide(core::image_step);
    reveal_with_fuel(core::mb_fuel,1);reveal_with_fuel(core::mp_fuel,1);
    lemma_pure_parser_steps(b,h,p,s,0,0);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 && core::ext_step(b,STRICT,s) is Ok {
        let next=core::ext_step(b,STRICT,s).unwrap();
        crate::parse::lemma_ext_success_progress(b,STRICT,s,next);
        lemma_s1_parser(b,h,p,next,(fuel-1) as nat);
    }
}

pub(crate) proof fn lemma_strict_image_domain(b:Seq<u8>,h:HeaderM,s:WalkM,fuel:nat,
    fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM)
    requires 0<=s.p,core::mb_fuel(b,h,STRICT,s,fuel)==Ok(MbOut::Image(fh,g,m,d,walk)),
    ensures fh.inside,2<=m<=8,1<=fh.w*fh.h,
    decreases fuel,
{
    hide(core::ext_step);hide(core::sub_scan);hide(core::concat);
    reveal_with_fuel(core::mb_fuel,1);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 {
        let next=core::ext_step(b,STRICT,s).unwrap();
        crate::parse::lemma_ext_success_progress(b,STRICT,s,next);
        lemma_strict_image_domain(b,h,next,(fuel-1) as nat,fh,g,m,d,walk);
    } else {
        core::lemma_le16_lt(b,s.p+5);core::lemma_le16_lt(b,s.p+7);
        assert(fh.w>=1 && fh.h>=1);
        assert(fh.w*fh.h>=1) by(nonlinear_arith) requires fh.w>=1,fh.h>=1;
    }
}

pub(crate) proof fn lemma_initial_simulation(b:Seq<u8>,p:Policy)
    requires conservative(p),core::mp(b,STRICT) is Ok,
    ensures core::mp(b,p)==core::mp(b,STRICT),fm::initial(b,p)==fm::initial(b,STRICT),
        policy::mp(b,p)==core::mp(b,STRICT),policy::initial(b,p)==fm::initial(b,STRICT),
{
    hide(core::mh);hide(core::mp_fuel);hide(policy::mp);hide(policy::initial);
    let h=core::mh(b).unwrap();
    core::lemma_mh_ok_bounds(b,h);
    let seed=WalkM {p:h.cursor,gce:None,frames_seen:0,total_pixels:0,loop_count:None};
    lemma_s1_parser(b,h,p,seed,(b.len()-h.cursor) as nat);
    policy::theorem_parser_bridge(b,p);
}

/// S3 compares the complete compositor/walk state, public frame and event set,
/// not the LZW dictionary or position discarded on return from the frame.
pub(crate) proof fn lemma_s3_render(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fh:FrameHeaderM,g:Option<GceM>,
    m:int,d:Seq<u8>,walk:WalkM,still:bool,c:policy::Call)
    requires fh.inside,2<=m<=8,1<=fh.w*fh.h,
        policy::render(b,h,STRICT,s,fh,g,m,d,walk,still)==Some(c),c.result.frame is Some,c.result.error is None,
    ensures policy::render(b,h,p,s,fh,g,m,d,walk,still)==Some(c),c.events==Set::<policy::Event>::empty(),
{
    hide(policy::ml);hide(crate::mc::draw_fold);
    assert(policy::ml(m,d,fh.w*fh.h,STRICT).error is None);
    theorem_lzw_conservative(m,d,fh.w*fh.h,p);
    policy::lemma_events(p,fh,policy::ml(m,d,fh.w*fh.h,p).e4);
}

/// A successful Strict call segment is a simulation from S1 through S2 to S3,
/// or from S1 to the Trailer. Equality includes Saved and previous disposal.
pub(crate) proof fn lemma_call_simulation(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,still:bool,c:fm::Call)
    requires conservative(p),0<=s.walk.p,fm::call_mode(b,h,STRICT,s,still)==Some(c),c.error is None,
    ensures fm::call_mode(b,h,p,s,still)==Some(c),
        policy::call(b,h,p,s,still) is Some,policy::call(b,h,p,s,still).unwrap().result==c,
        policy::call(b,h,p,s,still).unwrap().events==Set::<policy::Event>::empty(),
{
    hide(core::mb);hide(fm::image_mode);hide(policy::call);hide(policy::render);
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);
    policy::lemma_call_bridge(b,h,p,s,still);
    policy::lemma_call_bridge(b,h,STRICT,s,still);
    if s.status==fm::Status::Running {
        lemma_s1_parser(b,h,p,s.walk,(b.len()-s.walk.p) as nat);
        assert(core::mb(b,h,STRICT,s.walk) is Ok);
        assert(core::mb(b,h,p,s.walk)==core::mb(b,h,STRICT,s.walk)) by {reveal(core::mb);}
        assert(policy::mb(b,h,p,s.walk)==core::mb(b,h,p,s.walk)) by {
            policy::lemma_walks(b,h,p,s.walk,(b.len()-s.walk.p) as nat);
            reveal(policy::mb);reveal(core::mb);
        }
        match core::mb(b,h,STRICT,s.walk) {
            Ok(MbOut::Image(fh,g,m,d,walk))=>{
                assert(core::mb_fuel(b,h,STRICT,s.walk,(b.len()-s.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,d,walk))) by {reveal(core::mb);}
                lemma_strict_image_domain(b,h,s.walk,(b.len()-s.walk.p) as nat,fh,g,m,d,walk);
                assert(policy::mb(b,h,STRICT,s.walk)==core::mb(b,h,STRICT,s.walk)) by {
                    policy::lemma_walks(b,h,STRICT,s.walk,(b.len()-s.walk.p) as nat);
                    reveal(policy::mb);reveal(core::mb);
                }
                assert(policy::render(b,h,STRICT,s,fh,g,m,d,walk,still) is Some) by {reveal(policy::call);}
                let trace=policy::render(b,h,STRICT,s,fh,g,m,d,walk,still).unwrap();
                assert(trace.result==c) by {reveal(policy::call);}
                assert(c.frame is Some) by {reveal(fm::image_mode);}
                lemma_s3_render(b,h,p,s,fh,g,m,d,walk,still,trace);
                reveal(policy::call);
                assert(policy::mb(b,h,p,s.walk)==core::mb(b,h,p,s.walk)) by {
                    policy::lemma_walks(b,h,p,s.walk,(b.len()-s.walk.p) as nat);
                    reveal(policy::mb);reveal(core::mb);
                }
            },
            _=>{reveal(policy::call);},
        }
    } else {reveal(policy::call);}
}

pub(crate) proof fn theorem_r_still(b:Seq<u8>,p:Policy,out:fm::StillImage)
    requires conservative(p),fm::ms(b,STRICT)==Some(Ok(out)),
    ensures fm::ms(b,p)==Some(Ok(out)),policy::ms(b,p)==Some(Ok(out)),!out.recovered,
        policy::call(b,core::mh(b).unwrap(),p,policy::initial(b,p).unwrap(),true) is Some,
        policy::call(b,core::mh(b).unwrap(),p,policy::initial(b,p).unwrap(),true).unwrap().events==Set::<policy::Event>::empty(),
{
    hide(core::mp);hide(core::mh);hide(fm::call_mode);hide(policy::ms);
    fm::lemma_initial_domain(b,STRICT);
    lemma_initial_simulation(b,p);
    let h=core::mh(b).unwrap();let s=fm::initial(b,STRICT).unwrap();
    let c=fm::call_mode(b,h,STRICT,s,true).unwrap();
    lemma_call_simulation(b,h,p,s,true,c);
    crate::policy_proofs::theorem_strict_no_recovery(b);
    policy::theorem_ms_bridge(b,p);
}

pub open spec fn preserves_stream(s:fm::Stream,r:policy::Stream)->bool {
    &&& s.frames.len()<=r.frames.len()
    &&& r.events.len()==r.frames.len()
    &&& forall|i:int| 0<=i<s.frames.len() ==> #[trigger] r.frames[i]==s.frames[i]
        && !r.frames[i].recovered && r.events[i]==Set::<policy::Event>::empty()
    &&& (s.end==fm::End::Trailer ==> r.frames==s.frames && r.end==fm::End::Trailer)
}

pub(crate) proof fn lemma_stream_simulation(b:Seq<u8>,h:HeaderM,p:Policy,s:fm::State,fuel:nat)
    requires conservative(p),core::mh_bounds(b,h),0<=s.walk.p<=b.len(),
    ensures preserves_stream(fm::stream_fuel(b,h,STRICT,s,fuel),policy::stream(b,h,p,s,fuel)),
    decreases fuel,
{
    hide(fm::call);hide(fm::call_mode);hide(policy::call);
    reveal_with_fuel(fm::stream_fuel,1);reveal_with_fuel(policy::stream,1);
    policy::lemma_stream_bridge(b,h,p,s,fuel);
    if fuel>0 {
        fm::lemma_call_progress(b,h,STRICT,s);
        let c=fm::call(b,h,STRICT,s).unwrap();
        assert(fm::call_mode(b,h,STRICT,s,false)==Some(c)) by {reveal(fm::call);}
        if c.error is None {
            lemma_call_simulation(b,h,p,s,false,c);
            if c.frame is Some {
                lemma_stream_simulation(b,h,p,c.after,(fuel-1) as nat);
                let strict_tail=fm::stream_fuel(b,h,STRICT,c.after,(fuel-1) as nat);
                let relaxed_tail=policy::stream(b,h,p,c.after,(fuel-1) as nat);
                let strict_result=fm::stream_fuel(b,h,STRICT,s,fuel);
                let relaxed_result=policy::stream(b,h,p,s,fuel);
                policy::lemma_call_bridge(b,h,p,s,false);
                assert(!c.frame.unwrap().recovered);
                assert forall|i:int| 0<=i<strict_result.frames.len() implies
                    #[trigger] relaxed_result.frames[i]==strict_result.frames[i]
                        && !relaxed_result.frames[i].recovered && relaxed_result.events[i]==Set::<policy::Event>::empty() by {
                    if i==0 {assert(relaxed_result.frames[i]==c.frame.unwrap());}
                    else {
                        assert(strict_result.frames[i]==strict_tail.frames[i-1]);
                        assert(relaxed_result.frames[i]==relaxed_tail.frames[i-1]);
                        assert(relaxed_result.events[i]==relaxed_tail.events[i-1]);
                    }
                }
            }
        }
    }
}

pub(crate) proof fn theorem_r_prefix(b:Seq<u8>,p:Policy,info:core::InfoM,s:fm::Stream)
    requires conservative(p),fm::mf(b,STRICT)==Ok((info,s)),
    ensures policy::mf(b,p) is Ok,policy::mf(b,p).unwrap().0==info,
        policy::mf(b,p).unwrap().1.end!=fm::End::OutsideDomain,
        preserves_stream(s,policy::mf(b,p).unwrap().1),
        fm::mf(b,p) is Ok,fm::mf(b,p).unwrap().0==info,
        fm::mf(b,p).unwrap().1.frames==policy::mf(b,p).unwrap().1.frames,
        fm::mf(b,p).unwrap().1.end==policy::mf(b,p).unwrap().1.end,
{
    hide(core::mp);hide(core::mh);hide(fm::stream_fuel);hide(policy::mf);hide(policy::stream);
    fm::lemma_initial_domain(b,STRICT);
    lemma_initial_simulation(b,p);
    let h=core::mh(b).unwrap();let state=fm::initial(b,STRICT).unwrap();
    lemma_stream_simulation(b,h,p,state,b.len()+1);
    policy::theorem_mf_bridge(b,p);
    fm::lemma_mf_defined(b,p);
    assert(policy::mf(b,p)==Ok((info,policy::stream(b,h,p,state,b.len()+1)))) by {reveal(policy::mf);}
}

pub(crate) proof fn theorem_r_stream(b:Seq<u8>,p:Policy,info:core::InfoM,s:fm::Stream)
    requires conservative(p),fm::mf(b,STRICT)==Ok((info,s)),s.end==fm::End::Trailer,
    ensures fm::mf(b,p)==Ok((info,s)),policy::mf(b,p) is Ok,
        policy::mf(b,p).unwrap().0==info,policy::mf(b,p).unwrap().1.frames==s.frames,
        policy::mf(b,p).unwrap().1.end==fm::End::Trailer,
        forall|i:int| 0<=i<s.frames.len() ==> !#[trigger] s.frames[i].recovered
            && policy::mf(b,p).unwrap().1.events[i]==Set::<policy::Event>::empty(),
{
    hide(fm::mf);hide(policy::mf);
    theorem_r_prefix(b,p,info,s);
}

pub(crate) proof fn theorem_recommended_beta_conservative()
    ensures conservative(RECOMMENDED_BETA),conservative(STRICT),
{}

pub(crate) proof fn lemma_clip_inert(b:Seq<u8>,h:HeaderM,s:WalkM)
    requires core::image_step(b,h,STRICT,s) is Ok,
    ensures core::image_step(b,h,Policy {frame_bounds:FrameBounds::Clip,..STRICT},s)==core::image_step(b,h,STRICT,s),
{hide(core::sub_scan);hide(core::concat);}
pub(crate) proof fn lemma_disposal_keep_inert(b:Seq<u8>,s:WalkM,q:int)
    requires core::gce_step(b,STRICT,s,q) is Ok,
    ensures core::gce_step(b,Policy {disposal_undefined:DisposalUndefined::Keep,..STRICT},s,q)==core::gce_step(b,STRICT,s,q),
{}
pub(crate) proof fn lemma_last_wins_inert(b:Seq<u8>,s:WalkM,q:int)
    requires core::gce_step(b,STRICT,s,q) is Ok,
    ensures core::gce_step(b,Policy {gce_duplicate:GceDuplicate::LastWins,..STRICT},s,q)==core::gce_step(b,STRICT,s,q),
{}
pub(crate) proof fn lemma_image_only_inert(b:Seq<u8>,s:WalkM)
    requires core::ext_step(b,STRICT,s) is Ok,
    ensures core::ext_step(b,Policy {gce_consumers:GceConsumers::ImageOnly,..STRICT},s)==core::ext_step(b,STRICT,s),
{hide(core::sub_scan);hide(core::concat);hide(core::loop_value);hide(core::is_netscape);}
pub(crate) proof fn lemma_orphan_ignore_inert(b:Seq<u8>,s:WalkM)
    requires core::trailer_step(b,STRICT,s) is Ok,
    ensures core::trailer_step(b,Policy {gce_orphan:GceOrphan::Ignore,..STRICT},s)==core::trailer_step(b,STRICT,s),
{}
pub(crate) proof fn lemma_extension_sizes_ignore_inert(b:Seq<u8>,s:WalkM,q:int)
    ensures core::plain_text_step(b,STRICT,s,q) is Ok ==> core::plain_text_step(b,Policy {ext_sizes:ExtSizes::Ignore,..STRICT},s,q)==core::plain_text_step(b,STRICT,s,q),
        core::app_step(b,STRICT,s,q) is Ok ==> core::app_step(b,Policy {ext_sizes:ExtSizes::Ignore,..STRICT},s,q)==core::app_step(b,STRICT,s,q),
{hide(core::sub_scan);hide(core::loop_value);hide(core::is_netscape);}
pub(crate) proof fn lemma_checked_minimum_inert(b:Seq<u8>,s:WalkM,q:int,a:int)
    requires core::data_step(b,STRICT,s,q,a) is Ok,
    ensures core::data_step(b,Policy {lzw_min:LzwMin::Range1to11Checked,..STRICT},s,q,a)==core::data_step(b,STRICT,s,q,a),
{hide(core::sub_scan);hide(core::concat);}
pub(crate) proof fn lemma_missing_trailer_at_eof_inert(b:Seq<u8>,h:HeaderM,s:WalkM,fuel:nat)
    requires 0<=s.p,
    ensures core::mb_fuel(b,h,STRICT,s,fuel) is Ok ==>
        core::mb_fuel(b,h,Policy {missing_trailer:MissingTrailer::AtEof,..STRICT},s,fuel)==core::mb_fuel(b,h,STRICT,s,fuel),
{
    lemma_s1_parser(b,h,Policy {missing_trailer:MissingTrailer::AtEof,..STRICT},s,fuel);
}
pub(crate) proof fn lemma_trailing_ignore_inert(b:Seq<u8>,s:WalkM)
    requires core::trailer_step(b,STRICT,s) is Ok,
    ensures core::trailer_step(b,Policy {trailing:Trailing::Ignore,..STRICT},s)==core::trailer_step(b,STRICT,s),
{}

/// S2 traces stop at their first raster-complete state. Every earlier Strict
/// Continue step is the same full independent string-state transition under p.
pub open spec fn s2_trace(m:int,d:Seq<u8>,a:int,p:Policy,states:Seq<st::State>)->bool {
    &&& states.len()>=1
    &&& forall|i:int| 0<=i<states.len() ==> 0<=#[trigger] states[i].q<=a
    &&& forall|i:int| 0<=i<states.len()-1 ==> states[i].q<a
        && st::step(m,d,a,policy::params(p),states[i])==st::Step::Continue(states[i+1])
}
pub(crate) proof fn theorem_s2_trace(m:int,d:Seq<u8>,a:int,p:Policy,states:Seq<st::State>)
    requires 2<=m<=8,s2_trace(m,d,a,STRICT,states),
    ensures s2_trace(m,d,a,p,states),
{
    assert forall|i:int| 0<=i<states.len()-1 implies states[i].q<a
        && st::step(m,d,a,policy::params(p),states[i])==st::Step::Continue(states[i+1]) by {
        lemma_s2_step(m,d,a,p,states[i],states[i+1]);
    }
}

pub(crate) proof fn witness_recommended_beta_two_colour()
    ensures policy::ms(crate::model_vectors::bytes_two_colour_m2(),RECOMMENDED_BETA)==Some(Ok(fm::StillImage {
        width:3,height:1,rgba:seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8],recovered:false})),
{
    crate::model_vectors::model_two_colour_m2();
    lemma_profile_records();
    let b=crate::model_vectors::bytes_two_colour_m2();
    let s=fm::ms(b,STRICT).unwrap().unwrap();
    theorem_recommended_beta_conservative();
    theorem_r_still(b,RECOMMENDED_BETA,s);
}

} // verus!
