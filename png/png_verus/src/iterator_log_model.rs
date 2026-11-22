//! Sealed decoder-log shape; data reachability is a separate iterator relation.
use vstd::prelude::*;
use crate::alloc_guard::AllocSite;
verus! {
pub open spec fn initial(raw: nat,out: nat) -> Seq<(AllocSite,nat,bool)> {
    seq![(AllocSite::Raw,raw,true),(AllocSite::Output,out,true)]
}
pub open spec fn wf(log: Seq<(AllocSite,nat,bool)>,raw: nat,out: nat,saved: bool,done: bool) -> bool {
    (log.len()==2 || log.len()==3) && log.take(2)==initial(raw,out)
    && if log.len()==2 {!saved} else {
        log[2].0==AllocSite::Saved && log[2].1==out && log[2].2==saved && (!log[2].2 ==> done)
    }
}
pub proof fn initial_wf(raw: nat,out: nat)
    ensures wf(initial(raw,out),raw,out,false,false),
{assert(initial(raw,out).take(2)=~=initial(raw,out));}
pub proof fn append_saved(log: Seq<(AllocSite,nat,bool)>,raw: nat,out: nat,ok: bool)
    requires log==initial(raw,out),
    ensures wf(log.push((AllocSite::Saved,out,ok)),raw,out,ok,true),
        log.push((AllocSite::Saved,out,ok))==log+seq![(AllocSite::Saved,out,ok)],
{
    assert(log.push((AllocSite::Saved,out,ok)).take(2)=~=initial(raw,out));
    assert(log.push((AllocSite::Saved,out,ok))=~=log+seq![(AllocSite::Saved,out,ok)]);
}
pub proof fn all_ok_cases(log: Seq<(AllocSite,nat,bool)>,raw: nat,out: nat,saved: bool,done: bool)
    requires wf(log,raw,out,saved,done),
    ensures crate::alloc_guard::all_ok(log)==(log.len()==2 || saved),
        !done && !saved ==> log.len()==2,
        log.len()==2 ==> log==initial(raw,out),
{}
}
