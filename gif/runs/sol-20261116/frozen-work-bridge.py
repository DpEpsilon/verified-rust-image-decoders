from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text()
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
pos=s.index('pub closed spec fn constructor_work(')
s=s[:pos]+'''/// Frozen work observations after n+1 calls, including the actual OOM cut.
pub closed spec fn work_wf(d:Frames,n:int,cut:Option<nat>)->bool {
    &&& wf_p(d,n) && work_accounted(d)
    &&& match cut {
        None=>crate::alloc_guard::all_ok(d.log@.view()),
        Some(i)=>i as int<=n && d.state==State::Fused
            && crate::frame_model::saved_at(d.b@,crate::parse::header_model(d.hdr),d.pol,i),
    }
    &&& effective_cursor(d)==crate::work_model::cur_n(d.b@,d.pol,cut,n)
    &&& d.charged_pixels@==crate::work_model::P(d.b@,d.pol,cut,(n+1) as nat)
    &&& (d.state==State::Running ==> prior_area(d)==crate::work_model::A_n(d.b@,d.pol,cut,n))
}
spec fn call_oom(r:Option<Result<FrameView,TryError>>)->bool {
    r is Some && r->Some_0 is Err && r->Some_0->Err_0 is OutOfMemory
}
spec fn next_cut(cut:Option<nat>,k:nat,r:Option<Result<FrameView,TryError>>)->Option<nat> {
    if call_oom(r) {Some(k)} else {cut}
}
/// T12 in the frozen names, call k beginning in the state indexed k-1.
pub closed spec fn T12_next(a:Frames,d:Frames,k:nat,r:Option<Result<FrameView,TryError>>)->bool {
    forall|cut:Option<nat>| work_wf(a,k as int-1,cut) ==> {
        let nc=next_cut(cut,k,r);
        &&& work_wf(d,k as int,nc)
        &&& d.work@ as int-a.work@ as int<=C_CALL as int *
            (crate::work_model::cur_n(a.b@,a.pol,nc,k as int)-crate::work_model::cur_n(a.b@,a.pol,cut,k as int-1)
             +crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int
             +crate::work_model::A_n(a.b@,a.pol,cut,k as int-1) as int
             +if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k) {header_pixels(a.b@) as int} else {0})
        &&& d.work@ as int<=C_CALL as int*(a.b.len() as int+2*header_pixels(a.b@) as int+2*crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
        &&& d.work@ as int<=C_WORK as int*(a.b.len() as int+header_pixels(a.b@) as int+crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
    }
}

'''+s[pos:]
rep('    &&& work_accounted(d)\n}', '    &&& work_accounted(d)\n    &&& (!d.still ==> work_wf(d,-1,None))\n}')
# Constructor proof gets the semantic base case already proved at the actual call.
rep('    requires constructor_ok(d,d.b@,d.pol,d.still),d.charged_pixels@==0,','    requires constructor_ok(d,d.b@,d.pol,d.still),d.charged_pixels@==0,!d.still ==> wf_p(d,-1),')
rep('    reveal(constructor_ok);lemma_work_header(d);','    reveal(constructor_ok);lemma_work_header(d);lemma_work_mh(d);')
pos=s.index('proof fn lemma_work_header(')
s=s[:pos]+'''proof fn lemma_work_mh(d:Frames)
    requires storage_wf(d),
    ensures crate::model::mh(d.b@)==Ok(crate::parse::header_model(d.hdr)),
{
    hide(crate::model::mp);reveal(storage_wf);reveal(crate::parse::hdr_ok);
    let h=choose|h:crate::model::HeaderM| crate::model::mh(d.b@)==Ok(h) && crate::parse::hdr_matches(d.b@,h,d.hdr);
    crate::parse::lemma_header_model_matches(d.b@,d.hdr,h);
}

#[verifier::spinoff_prover]
proof fn lemma_work_wf_step(a:Frames,d:Frames,k:nat,cut:Option<nat>,r:Option<Result<FrameView,TryError>>)
    requires work_wf(a,k as int-1,cut),wf_p(d,k as int),fixed_stream(a,d),
        next_work(a,d),next_model_work(a,d,r),next_result(a,d,r),
        next_allocation(a,d,k as int-1,r),
    ensures work_wf(d,k as int,next_cut(cut,k,r)),
{
    hide(crate::model::mp);hide(crate::model::mb);hide(crate::parse::hdr_ok);
    hide(crate::frame_model::call);hide(crate::frame_model::call_mode);hide(crate::frame_model::after_calls);
    hide(crate::frame_model::saved_at);hide(iterator_log);hide(iterator_observation);
    reveal(work_wf);reveal(next_allocation);
    lemma_work_mh(a);lemma_work_mh(d);
    let nc=next_cut(cut,k,r);
    if a.state==State::Running {
        lemma_wf_running(a,k as int-1);
        assert(cut is None);
        let c=crate::frame_model::call(a.b@,crate::parse::header_model(a.hdr),a.pol,model_state(a)).unwrap();
        if call_oom(r) {
            assert(nc==Some(k));
        } else {
            assert(c.error is Some <==> r is Some && r->Some_0 is Err);
        }
    } else {
        assert(d==a);assert(!call_oom(r));
        if cut is None {
            reveal(wf_p);
            let st=crate::frame_model::after_calls(a.b@,crate::parse::header_model(a.hdr),a.pol,k).unwrap();
            assert(st.status!=crate::frame_model::Status::Running);
        }
    }
    if call_oom(r) {
        assert(cut is None);
        assert(crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k));
    } else if cut is None {
        assert forall|i:int| 0<=i<d.log@.view().len() implies #[trigger] d.log@.view()[i].2 by {
            if i<a.log@.view().len() {assert(d.log@.view()[i]==a.log@.view()[i]);}
        }
        assert(crate::alloc_guard::all_ok(d.log@.view()));
    }
    crate::work_model::lemma_P_next(a.b@,a.pol,cut,nc,k);
    assert(d.charged_pixels@ as int-a.charged_pixels@ as int==crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int);
    assert(effective_cursor(d)==crate::work_model::cur_n(a.b@,a.pol,nc,k as int));
}

#[verifier::spinoff_prover]
proof fn lemma_T12_next(a:Frames,d:Frames,k:nat,r:Option<Result<FrameView,TryError>>)
    requires storage_wf(a),storage_wf(d),fixed_stream(a,d),next_work(a,d),next_model_work(a,d,r),next_result(a,d,r),
        wf_p(a,k as int-1) ==> wf_p(d,k as int),
        wf_p(a,k as int-1) ==> next_allocation(a,d,k as int-1,r),
        wf_p(a,k as int-1) && work_ready(a) ==> t12_measured_call(a,d,k as int-1),
    ensures T12_next(a,d,k,r),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    hide(crate::work_model::P);hide(crate::work_model::cur_n);hide(crate::work_model::A_n);
    assert forall|cut:Option<nat>| work_wf(a,k as int-1,cut) implies {
        let nc=next_cut(cut,k,r);
        &&& work_wf(d,k as int,nc)
        &&& d.work@ as int-a.work@ as int<=C_CALL as int *
            (crate::work_model::cur_n(a.b@,a.pol,nc,k as int)-crate::work_model::cur_n(a.b@,a.pol,cut,k as int-1)
             +crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int+crate::work_model::A_n(a.b@,a.pol,cut,k as int-1) as int
             +if crate::frame_model::saved_at(a.b@,crate::parse::header_model(a.hdr),a.pol,k) {header_pixels(a.b@) as int} else {0})
        &&& d.work@ as int<=C_CALL as int*(a.b.len() as int+2*header_pixels(a.b@) as int+2*crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
        &&& d.work@ as int<=C_WORK as int*(a.b.len() as int+header_pixels(a.b@) as int+crate::work_model::P(a.b@,a.pol,nc,k+1) as int)+C_FIXED as int
    } by {
        reveal(work_wf);
        lemma_work_wf_step(a,d,k,cut,r);
        lemma_work_total(d);
        let nc=next_cut(cut,k,r);
        if a.state==State::Running {
            assert(d.charged_pixels@ as int-a.charged_pixels@ as int==crate::work_model::A_n(a.b@,a.pol,nc,k as int) as int) by {
                reveal(crate::work_model::P);
                if nc!=cut {crate::work_model::lemma_P_cut_prefix(a.b@,a.pol,k,k);}
            }
        } else {
            assert(d==a);
        }
    }
}

'''+s[pos:]
# Actual method exports the frozen-named transition for each call index.
rep('            forall|n:int| wf_p(*old(self),n) && work_ready(*old(self)) ==> t12_measured_call(*old(self),*final(self),n),','''            forall|n:int| wf_p(*old(self),n) && work_ready(*old(self)) ==> t12_measured_call(*old(self),*final(self),n),
            forall|k:nat| T12_next(*old(self),*final(self),k,out),''')
rep('''                return None;
            },
        }
        let out=match self.step()''','''                proof {
                    assert(next_model_work(original,*self,None));
                    assert forall|k:nat| T12_next(original,*self,k,None) by {lemma_T12_next(original,*self,k,None);}
                }
                return None;
            },
        }
        let out=match self.step()''')
rep('''        out
    }

}
} // verus!''','''        proof {
            assert(next_model_work(original,*self,out));
            assert forall|k:nat| T12_next(original,*self,k,out) by {lemma_T12_next(original,*self,k,out);}
        }
        out
    }

}
} // verus!''')
p.write_text(s)
print('Added frozen-named T12 invariant, OOM cut, and actual public transition package')
