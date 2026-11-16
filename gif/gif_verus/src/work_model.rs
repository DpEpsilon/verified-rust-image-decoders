//! Independent G18 work observations over the structural and frame models.
//! Terminal cursor observations do not use the retained terminal State.walk.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::model::{HeaderM,WalkM,MbOut};
#[cfg(verus_only)]
use crate::policy::Policy;

verus! {

/// MB's structural endpoint: MP finds the Trailer after the same extensions.
/// Errors consume the remaining accounting interval by the frozen convention.
pub open spec fn mb_cur(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM)->int {
    match crate::model::mb(b,h,p,s) {
        Err(_)=>b.len() as int,
        Ok(MbOut::Image(_,_,_,_,walk))=>walk.p,
        Ok(MbOut::Trailer)=>match crate::model::mp_fuel(b,p,s,(b.len() as int-s.p) as nat) {
            Ok(walk)=>walk.p,Err(_)=>b.len() as int,
        },
    }
}

/// Descriptor area after G8-G9, before the possible Saved failure.
pub open spec fn mb_area(b:Seq<u8>,h:HeaderM,p:Policy,s:WalkM)->nat {
    match crate::model::mb(b,h,p,s) {
        Ok(MbOut::Image(fh,_,_,_,_))=>(fh.w*fh.h) as nat,
        _=>0,
    }
}

pub open spec fn initial_cur(b:Seq<u8>,p:Policy)->int {
    match crate::model::mp(b,p) {Ok((_,s))=>s.p,Err(_)=>b.len() as int}
}

/// Cursor after k calls in the allocation-free model, with terminal stability.
pub open spec fn cur_calls(b:Seq<u8>,p:Policy,k:nat)->int
    decreases k,
{
    if k==0 {initial_cur(b,p)} else {
        match crate::model::mh(b) {
            Err(_)=>b.len() as int,
            Ok(h)=>match crate::frame_model::after_calls(b,h,p,(k-1) as nat) {
                None=>b.len() as int,
                Some(s)=>if s.status==crate::frame_model::Status::Running {
                    match crate::frame_model::call(b,h,p,s) {
                        Some(c)=>if c.error is Some {b.len() as int} else {mb_cur(b,h,p,s.walk)},
                        None=>b.len() as int,
                    }
                } else {cur_calls(b,p,(k-1) as nat)},
            },
        }
    }
}

/// Frozen cur_n, including cur_{-1}; cut is the actual Saved-OOM index.
pub open spec fn cur_n(b:Seq<u8>,p:Policy,cut:Option<nat>,n:int)->int {
    if cut is Some && cut.unwrap() as int<=n {b.len() as int}
    else {cur_calls(b,p,if n<0 {0nat} else {(n+1) as nat})}
}

/// Frozen A_n: a failed Saved reservation does not reach G10.
pub open spec fn A_n(b:Seq<u8>,p:Policy,cut:Option<nat>,n:int)->nat {
    if n<0 || cut is Some && cut.unwrap() as int<=n {0}
    else {match crate::model::mh(b) {
        Err(_)=>0,
        Ok(h)=>match crate::frame_model::after_calls(b,h,p,n as nat) {
            Some(s)=>if s.status==crate::frame_model::Status::Running {mb_area(b,h,p,s.walk)} else {0},
            None=>0,
        },
    }}
}

/// Frozen P over a finite call prefix; later terminal calls add zero.
pub open spec fn P(b:Seq<u8>,p:Policy,cut:Option<nat>,calls:nat)->nat
    decreases calls,
{
    if calls==0 {0} else {P(b,p,cut,(calls-1) as nat)+A_n(b,p,cut,calls as int-1)}
}

pub proof fn lemma_P_cut_prefix(b:Seq<u8>,p:Policy,i:nat,calls:nat)
    requires calls<=i,
    ensures P(b,p,Some(i),calls)==P(b,p,None,calls),
    decreases calls,
{
    reveal_with_fuel(P,1);
    if calls>0 {lemma_P_cut_prefix(b,p,i,(calls-1) as nat);}
}

pub proof fn lemma_P_next(b:Seq<u8>,p:Policy,oldcut:Option<nat>,newcut:Option<nat>,calls:nat)
    requires newcut==oldcut || oldcut is None && newcut==Some(calls),
    ensures P(b,p,newcut,calls+1)==P(b,p,oldcut,calls)+A_n(b,p,newcut,calls as int),
{
    reveal_with_fuel(P,1);
    if newcut!=oldcut {lemma_P_cut_prefix(b,p,calls,calls);}
}

pub proof fn lemma_initial_cur_bounds(b:Seq<u8>,p:Policy)
    ensures 0<=initial_cur(b,p)<=b.len(),
{
    hide(crate::model::mp);
    if crate::model::mp(b,p) is Ok {crate::frame_model::lemma_initial_domain(b,p);}
}

} // verus!
