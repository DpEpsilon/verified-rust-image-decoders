//! L-PREFIX: successful still decoding depends only on the bytes through the
//! first image-data terminator, apart from the input-length guard.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::model::*;
#[cfg(verus_only)]
use crate::frame_model as fm;
#[cfg(verus_only)]
use crate::policy::Policy;

verus! {

pub open spec fn agrees(b:Seq<u8>,c:Seq<u8>,t:int)->bool {
    0<=t<=b.len() && t<=c.len()
        && forall|i:int| 0<=i<t ==> #[trigger] b[i]==c[i]
}

pub proof fn lemma_range(b:Seq<u8>,c:Seq<u8>,t:int,p:int,n:int)
    requires agrees(b,c,t),0<=p,0<=n,p+n<=t,
    ensures have(b,p,n),have(c,p,n),b.subrange(p,p+n)==c.subrange(p,p+n),
{
    assert(b.subrange(p,p+n)=~=c.subrange(p,p+n));
}

pub proof fn lemma_header(b:Seq<u8>,c:Seq<u8>,t:int,h:HeaderM)
    requires mh(b)==Ok(h),h.cursor<=t,agrees(b,c,t),c.len()<=max_input(),
    ensures mh(c)==Ok(h),
{
    lemma_mh_ok_bounds(b,h);
    lemma_range(b,c,t,0,13);
    assert forall|i:int| 0<=i<13 implies b[i]==c[i] by {};
}

pub proof fn lemma_scan(b:Seq<u8>,c:Seq<u8>,t:int,q:int,subs:Seq<Seq<u8>>,end:int)
    requires 0<=q,sub_scan(b,q)==Some((subs,end)),end<=t,agrees(b,c,t),
    ensures sub_scan(c,q)==Some((subs,end)),q<end,
    decreases (b.len()-q) as nat,
{
    reveal_with_fuel(sub_scan,1);
    crate::parse::lemma_sub_scan_bounds(b,q,subs,end);
    if b[q]!=0 {
        let n=b[q] as int;
        let tail=sub_scan(b,q+1+n).unwrap();
        lemma_scan(b,c,t,q+1+n,tail.0,tail.1);
        lemma_range(b,c,t,q+1,n);
    }
}

pub proof fn lemma_extension(b:Seq<u8>,c:Seq<u8>,t:int,pol:Policy,s:WalkM,ns:WalkM)
    requires 0<=s.p,ext_step(b,pol,s)==Ok(ns),ns.p<=t,agrees(b,c,t),
    ensures ext_step(c,pol,s)==Ok(ns),s.p<ns.p,
{
    hide(sub_scan);hide(loop_value);hide(is_netscape);
    crate::parse::lemma_ext_success_progress(b,pol,s,ns);
    let q=s.p+2;
    if b[s.p+1]==0xf9 {
        lemma_range(b,c,t,q,6);
        assert forall|i:int| q<=i<q+6 implies b[i]==c[i] by {};
    } else {
        let scan=sub_scan(b,q).unwrap();
        lemma_scan(b,c,t,q,scan.0,scan.1);
    }
}

pub proof fn lemma_data(b:Seq<u8>,c:Seq<u8>,t:int,pol:Policy,s:WalkM,p:int,a:int,
    m:int,d:Seq<u8>,ns:WalkM)
    requires 0<=p,data_step(b,pol,s,p,a)==Ok((m,d,ns)),ns.p<=t,agrees(b,c,t),
    ensures data_step(c,pol,s,p,a)==Ok((m,d,ns)),p<ns.p,
{
    hide(sub_scan);hide(concat);
    let scan=sub_scan(b,p+1).unwrap();
    lemma_scan(b,c,t,p+1,scan.0,scan.1);
}

pub proof fn lemma_image(b:Seq<u8>,c:Seq<u8>,t:int,h:HeaderM,pol:Policy,s:WalkM,
    fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,ns:WalkM)
    requires 0<=s.p,h.cursor<=s.p,mh_bounds(b,h),image_step(b,h,pol,s)==Ok((fh,g,m,d,ns)),ns.p<=t,agrees(b,c,t),
    ensures image_step(c,h,pol,s)==Ok((fh,g,m,d,ns)),s.p<ns.p,
        fh.table.off+3*fh.table.entries<=ns.p,
{
    hide(data_step);
    let packed=b[s.p+9];
    lemma_table_entries_range(packed);
    let off=if flag_lct(packed) {s.p+10+3*table_entries(packed)} else {s.p+10};
    lemma_data(b,c,t,pol,s,off,fh.w*fh.h,m,d,ns);
    lemma_range(b,c,t,s.p,10);
    assert forall|i:int| s.p<=i<s.p+10 implies b[i]==c[i] by {};
}

/// The two walks can have different fuel because the inputs can have different
/// lengths. The replacement needs only the bytes through the successful image.
pub proof fn lemma_mb(b:Seq<u8>,c:Seq<u8>,t:int,h:HeaderM,pol:Policy,s:WalkM,
    fuel:nat,other:nat,fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,ns:WalkM)
    requires 0<=s.p,h.cursor<=s.p,mh_bounds(b,h),agrees(b,c,t),ns.p<=t,other>=t-s.p,
        mb_fuel(b,h,pol,s,fuel)==Ok(MbOut::Image(fh,g,m,d,ns)),
    ensures mb_fuel(c,h,pol,s,other)==Ok(MbOut::Image(fh,g,m,d,ns)),s.p<ns.p,
        fh.table.off+3*fh.table.entries<=ns.p,
    decreases fuel,
{
    hide(ext_step);hide(image_step);
    reveal_with_fuel(mb_fuel,1);
    fm::lemma_mb_image_progress(b,h,pol,s,fuel,fh,g,m,d,ns);
    if b[s.p]==0x21 {
        let next=ext_step(b,pol,s).unwrap();
        crate::parse::lemma_ext_success_progress(b,pol,s,next);
        lemma_mb(b,c,t,h,pol,next,(fuel-1) as nat,(other-1) as nat,fh,g,m,d,ns);
        lemma_extension(b,c,t,pol,s,next);
    } else if b[s.p]==0x2c {
        lemma_image(b,c,t,h,pol,s,fh,g,m,d,ns);
    }
}

pub proof fn lemma_mp_walk(b:Seq<u8>,c:Seq<u8>,t:int,pol:Policy,s:WalkM,
    fuel:nat,other:nat,out:WalkM)
    requires 0<=s.p,agrees(b,c,t),other>=t-s.p,0<=out.p<t,b[out.p]==0x2c,
        mp_fuel(b,pol,s,fuel)==Ok(out),
    ensures mp_fuel(c,pol,s,other)==Ok(out),s.p<=out.p,
    decreases fuel,
{
    hide(ext_step);
    reveal_with_fuel(mp_fuel,1);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 {
        let next=ext_step(b,pol,s).unwrap();
        crate::parse::lemma_ext_success_progress(b,pol,s,next);
        lemma_mp_walk(b,c,t,pol,next,(fuel-1) as nat,(other-1) as nat,out);
        lemma_extension(b,c,t,pol,s,next);
    }
}

pub proof fn lemma_mp(b:Seq<u8>,c:Seq<u8>,t:int,pol:Policy,h:HeaderM,i:InfoM,s:WalkM)
    requires mh(b)==Ok(h),mp(b,pol)==Ok((i,s)),agrees(b,c,t),c.len()<=max_input(),
        0<=s.p<t,b[s.p]==0x2c,
    ensures mh(c)==Ok(h),mp(c,pol)==Ok((i,s)),h.cursor<=s.p,
{
    hide(mh);hide(mp_fuel);
    lemma_mh_ok_bounds(b,h);
    let seed=WalkM {p:h.cursor,gce:None,frames_seen:0,total_pixels:0,loop_count:None};
    lemma_mp_walk(b,c,t,pol,seed,(b.len()-h.cursor) as nat,(c.len()-h.cursor) as nat,s);
    lemma_header(b,c,t,h);
}

pub proof fn lemma_frame(b:Seq<u8>,c:Seq<u8>,t:int,h:HeaderM,pol:Policy,s:fm::State,
    fh:FrameHeaderM,g:Option<GceM>,m:int,d:Seq<u8>,walk:WalkM,still:bool)
    requires agrees(b,c,t),fm::image_domain(b,h,fh),fh.table.off+3*fh.table.entries<=t,
    ensures fm::image_mode(b,h,pol,s,fh,g,m,d,walk,still)==fm::image_mode(c,h,pol,s,fh,g,m,d,walk,still),
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);
    lemma_range(b,c,t,fh.table.off,3*fh.table.entries);
}

/// Cursor after the first image-data terminator on a successful still path.
/// This is MB's cursor, fixed by G9 before any LZW code is read.
pub open spec fn first_terminator(b:Seq<u8>,pol:Policy)->int {
    let h=mh(b).unwrap();
    let s=fm::initial(b,pol).unwrap();
    match mb(b,h,pol,s.walk) {
        Ok(MbOut::Image(_,_,_,_,walk))=>walk.p,
        _=>-1,
    }
}

pub proof fn theorem_successful_still_prefix(b:Seq<u8>,c:Seq<u8>,pol:Policy,out:fm::StillImage)
    requires fm::ms(b,pol)==Some(Ok(out)),agrees(b,c,first_terminator(b,pol)),c.len()<=max_input(),
    ensures fm::ms(c,pol)==Some(Ok(out)),
{
    hide(mh);hide(mp);hide(mb);hide(fm::image_mode);
    let h=mh(b).unwrap();
    let info=mp(b,pol).unwrap().0;
    let state=fm::initial(b,pol).unwrap();
    fm::lemma_initial_domain(b,pol);
    let image=mb(b,h,pol,state.walk);
    match image {
        Ok(MbOut::Image(fh,g,m,d,walk))=>{
            let t=walk.p;
            assert(mb_fuel(b,h,pol,state.walk,(b.len()-state.walk.p) as nat)==Ok(MbOut::Image(fh,g,m,d,walk))) by {reveal(mb);}
            fm::lemma_mb_image_progress(b,h,pol,state.walk,(b.len()-state.walk.p) as nat,fh,g,m,d,walk);
            lemma_mp(b,c,t,pol,h,info,state.walk);
            lemma_mb(b,c,t,h,pol,state.walk,(b.len()-state.walk.p) as nat,(c.len()-state.walk.p) as nat,fh,g,m,d,walk);
            assert(mb(c,h,pol,state.walk)==Ok(MbOut::Image(fh,g,m,d,walk))) by {reveal(mb);}
            assert(fm::initial(c,pol)==Some(state));
            // The active global table precedes MP; a local table precedes the
            // first terminator by the descriptor's checked availability.
            lemma_frame(b,c,t,h,pol,state,fh,g,m,d,walk,true);
        },
        _=>{},
    }
}

pub(crate) proof fn theorem_l_prefix(b:Seq<u8>,c:Seq<u8>,out:fm::StillImage)
    requires fm::ms(b,crate::policy::STRICT)==Some(Ok(out)),
        agrees(b,c,first_terminator(b,crate::policy::STRICT)),c.len()<=max_input(),
    ensures fm::ms(c,crate::policy::STRICT)==Some(Ok(out)),
{
    theorem_successful_still_prefix(b,c,crate::policy::STRICT,out);
}

/// Non-vacuity: the frozen two-colour-m2 input succeeds and its first image
/// terminator is at cursor 34. Every bounded replacement tail preserves its
/// authored dimensions and RGBA, including removing the Trailer entirely.
pub proof fn witness_two_colour_prefix(c:Seq<u8>)
    requires agrees(crate::model_vectors::bytes_two_colour_m2(),c,34),c.len()<=max_input(),
    ensures fm::ms(c,crate::policy::profile_record(crate::policy::Profile::Strict))==Some(Ok(fm::StillImage {
        width:3,height:1,rgba:seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8],recovered:false})),
{
    crate::model_vectors::model_two_colour_m2();
    assert(first_terminator(crate::model_vectors::bytes_two_colour_m2(),crate::policy::profile_record(crate::policy::Profile::Strict))==34) by(compute);
    let b=crate::model_vectors::bytes_two_colour_m2();
    let p=crate::policy::profile_record(crate::policy::Profile::Strict);
    let out=fm::ms(b,p).unwrap().unwrap();
    theorem_successful_still_prefix(b,c,p,out);
}

} // verus!
