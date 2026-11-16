from pathlib import Path
import shutil
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text();b=p.parent.parent.parent/'runs/sol-20261116/t12-pre-packages-gif.rs';assert not b.exists();shutil.copy2(p,b)
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
pos=s.index('proof fn lemma_previous_area(')
s=s[:pos]+'''/// Operational T12 observations: work counts concrete bodies/fills and the
/// pixel charge increases only after preparation succeeds, at the G10 entry.
pub closed spec fn t12_measured_call(a:Frames,d:Frames,n:int)->bool {
    d.work@ as int<=a.work@ as int+C_CALL as int *
        (effective_cursor(d)-effective_cursor(a)+d.charged_pixels@ as int-a.charged_pixels@ as int
         +(if a.state==State::Running {prior_area(a) as int} else {0})
         +(if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,(n+1) as nat) {header_pixels(a.b@) as int} else {0}))
}
proof fn lemma_work_total(d:Frames)
    requires storage_wf(d),work_accounted(d),
    ensures total_work_bound(d),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    lemma_storage_bounds(d);
}
proof fn lemma_t12_measured_call(a:Frames,d:Frames,n:int,r:Option<Result<FrameView,TryError>>)
    requires storage_wf(a),storage_wf(d),wf_p(a,n),next_allocation(a,d,n,r),next_work(a,d),work_ready(a),
    ensures t12_measured_call(a,d,n),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    hide(crate::frame_model::saved_at);hide(iterator_log);hide(iterator_observation);
    reveal(storage_wf);reveal(next_allocation);
    if a.saved is None && d.saved is Some {
        assert(a.log@.view().len()>=2);
        assert(d.log@.view().len()==3);
        assert(crate::alloc_guard::all_ok(d.log@.view()));
        assert(crate::alloc_guard::all_ok(a.log@.view()));
        assert(crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,(n+1) as nat));
    }
}

/// Hoare-style finite histories of the real APIs. An unchanged snapshot is
/// an info call; every non-identity edge uses next_frame's proved next_work.
pub proof fn lemma_finite_work_history(ds:Seq<Frames>)
    requires ds.len()>=1,constructor_work(ds[0]),
        forall|i:int| 0<=i<ds.len() ==> storage_wf(#[trigger] ds[i]),
        forall|i:int| 0<=i<ds.len()-1 ==> ds[i+1]==ds[i] || next_work(#[trigger] ds[i],ds[i+1]),
    ensures work_accounted(ds.last()),total_work_bound(ds.last()),
    decreases ds.len(),
{
    if ds.len()==1 {lemma_work_total(ds[0]);}
    else {
        let prefix=ds.take(ds.len() as int-1);
        assert forall|i:int| 0<=i<prefix.len() implies storage_wf(#[trigger] prefix[i]) by {assert(prefix[i]==ds[i]);}
        assert forall|i:int| 0<=i<prefix.len()-1 implies prefix[i+1]==prefix[i] || next_work(#[trigger] prefix[i],prefix[i+1]) by {assert(prefix[i]==ds[i]);assert(prefix[i+1]==ds[i+1]);}
        lemma_finite_work_history(prefix);
        assert(prefix.last()==ds[ds.len() as int-2]);
    }
}

'''+s[pos:]
# State the SavedAt-indexed inequality on the actual public next_frame.
a='''            next_work(*old(self),*final(self)),''';assert s.count(a)==1;s=s.replace(a,a+'''\n            forall|n:int| wf_p(*old(self),n) && work_ready(*old(self)) ==> t12_measured_call(*old(self),*final(self),n),''')
# work_ready must be opaque public for this public postcondition.
rep('spec fn work_ready(d:Frames)->bool {','pub closed spec fn work_ready(d:Frames)->bool {')
# At both next-frame exits, introduce the indexed numerical clause after the
# existing work and allocation packages are known.
a='proof {lemma_next_work(original,*self);}';assert s.count(a)==2
s=s.replace(a,'''proof {
                    lemma_next_work(original,*self);
                    assert forall|n:int| wf_p(original,n) && work_ready(original) implies t12_measured_call(original,*self,n) by {
                        lemma_t12_measured_call(original,*self,n,None);
                    }
                }''',1)
# Locate the remaining occurrence, which follows construction of out.
s=s.replace(a,'''proof {
            lemma_next_work(original,*self);
            assert forall|n:int| wf_p(original,n) && work_ready(original) implies t12_measured_call(original,*self,n) by {
                lemma_t12_measured_call(original,*self,n,out);
            }
        }''',1)
# Ordinary still wrappers erase no work: they all invoke this same measured
# constructor/step path and add only straight-line projection of the result.
rep('            proof {lemma_still_empty(d);}','''            proof {
                lemma_still_empty(d);reveal(constructor_work);lemma_work_total(d);
                assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+d.charged_pixels@ as int)+C_FIXED as int);
            }''')
rep('    proof {lemma_still_step(initial,d,step);lemma_still_trace(initial,d,step);}','''    proof {
        lemma_still_step(initial,d,step);lemma_still_trace(initial,d,step);
        reveal(constructor_work);lemma_next_work(initial,d);
        assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+d.charged_pixels@ as int)+C_FIXED as int);
    }''')
p.write_text(s)
print('Added SavedAt-indexed numeric clause, finite histories and still-path bound assertions')
