from pathlib import Path
import shutil
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text();backup=p.parent.parent.parent/'runs/sol-20261116/t13-before-public-packages-gif.rs';assert not backup.exists();shutil.copy2(p,backup)
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
pos=s.index('pub closed spec fn constructor_work(')
s=s[:pos]+'''pub closed spec fn T13_resources(d:Frames)->bool {
    &&& 1<=d.hdr.width<=crate::limits::MAX_DIMENSION && 1<=d.hdr.height<=crate::limits::MAX_DIMENSION
    &&& d.canvas@.len()==4*(d.hdr.width as int)*(d.hdr.height as int)<=crate::limits::MAX_OUTPUT
    &&& d.table@.len()==crate::lzw::LZW_TABLE_BYTES
    &&& match d.saved {None=>true,Some(v)=>v@.len()==d.canvas@.len()}
    &&& d.canvas@.len() as int+d.table@.len() as int+(match d.saved {None=>0int,Some(v)=>v@.len() as int})
        <=2*d.canvas@.len() as int+crate::lzw::LZW_TABLE_BYTES as int
}
pub closed spec fn T13_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)->bool {
    &&& still_result(b,p,r)
    &&& match r {
        Ok((s,_))=>1<=s.width<=crate::limits::MAX_DIMENSION && 1<=s.height<=crate::limits::MAX_DIMENSION
            && s.rgba@.len()==4*(s.width as int)*(s.height as int)<=crate::limits::MAX_OUTPUT
            && crate::mc::pixels_well_formed(s.rgba@,crate::pixel_model::active_tables(b,crate::model::mh(b).unwrap(),p,1)),
        _=>true,
    }
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> {
        &&& (crate::frame_model::ms(b,p) is Some && crate::frame_model::ms(b,p).unwrap() is Ok ==> r is Ok)
        &&& forall|e:Error| (r is Err && r->Err_0==TryError::Format(e)) <==> crate::frame_model::ms(b,p)==Some(Err(e))
    })
}
pub closed spec fn T13_core(b:Seq<u8>,r:Result<Still,TryError>)->bool {
    let p=crate::policy::profile_record(Profile::Strict);
    &&& still_core_result(b,r)
    &&& match r {
        Ok(s)=>1<=s.width<=crate::limits::MAX_DIMENSION && 1<=s.height<=crate::limits::MAX_DIMENSION
            && s.rgba@.len()==4*(s.width as int)*(s.height as int)<=crate::limits::MAX_OUTPUT
            && crate::mc::pixels_well_formed(s.rgba@,crate::pixel_model::active_tables(b,crate::model::mh(b).unwrap(),p,1)),
        _=>true,
    }
    &&& (!(r is Err && r->Err_0 is OutOfMemory) ==> {
        &&& (crate::frame_model::ms(b,p) is Some && crate::frame_model::ms(b,p).unwrap() is Ok ==> r is Ok)
        &&& forall|e:Error| (r is Err && r->Err_0==TryError::Format(e)) <==> crate::frame_model::ms(b,p)==Some(Err(e))
    })
}
pub closed spec fn T13_view(d:Frames,n:int,r:Option<Result<FrameView,TryError>>)->bool {
    &&& T13_resources(d)
    &&& match r {
        Some(Ok(v))=>v.rgba@==d.canvas@ && crate::mc::pixels_well_formed(v.rgba@,
            crate::pixel_model::active_tables(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+1) as nat)),
        _=>true,
    }
}
proof fn lemma_T13_resources(d:Frames)
    requires storage_wf(d),ensures T13_resources(d),
{lemma_storage_bounds(d);}
proof fn lemma_T13_still(b:Seq<u8>,p:Policy,r:Result<(Still,bool),TryError>)
    requires still_result(b,p,r),ensures T13_still(b,p,r),
{
    hide(crate::frame_model::ms);
    crate::pixel_model::lemma_ms_pixels(b,p);
}
proof fn lemma_T13_core(b:Seq<u8>,r:Result<Still,TryError>)
    requires still_core_result(b,r),ensures T13_core(b,r),
{
    hide(crate::frame_model::ms);
    crate::pixel_model::lemma_ms_pixels(b,crate::policy::profile_record(Profile::Strict));
}
#[verifier::spinoff_prover]
proof fn lemma_T13_next(a:Frames,d:Frames,n:int,r:Option<Result<FrameView,TryError>>)
    requires wf_p(d,n+1),next_result(a,d,r),fixed_stream(a,d),
    ensures T13_view(d,n+1,r),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);hide(crate::frame_model::after_calls);
    lemma_T13_resources(d);
    if r is Some && r->Some_0 is Ok {
        reveal(next_result);
        assert(d.state==State::Running);
        lemma_wf_running(d,n+1);lemma_work_mh(d);
        crate::pixel_model::lemma_after_pixels(d.b@,crate::parse::header_model(d.hdr),d.pol,(n+2) as nat);
    }
}

'''+s[pos:]
# Resource ownership follows the real successful constructor's storage facts.
rep('    &&& (!d.still ==> work_wf(d,-1,None))','    &&& (!d.still ==> work_wf(d,-1,None))\n    &&& T13_resources(d)')
rep('    reveal(constructor_ok);lemma_work_header(d);lemma_work_mh(d);','    reveal(constructor_ok);lemma_work_header(d);lemma_work_mh(d);lemma_T13_resources(d);')
# One shared ordinary still wrapper supplies the policy result package.
rep('fn still_with(b: &[u8], pol: Policy) -> (out: Result<(Still, bool), TryError>)\n    ensures still_result(b@,pol,out),','fn still_with(b: &[u8], pol: Policy) -> (out: Result<(Still, bool), TryError>)\n    ensures still_result(b@,pol,out),T13_still(b@,pol,out),')
rep('    proof {reveal(still_g);}\n    r.0','    proof {reveal(still_g);lemma_T13_still(b@,pol,r.0);}\n    r.0')
rep('    ensures still_result(b@,crate::policy::profile_record(p),r),','    ensures still_result(b@,crate::policy::profile_record(p),r),T13_still(b@,crate::policy::profile_record(p),r),')
rep('    ensures still_result(b@,pol,r),','    ensures still_result(b@,pol,r),T13_still(b@,pol,r),')
rep('    ensures still_core_result(b@,r),','    ensures still_core_result(b@,r),T13_core(b@,r),')
a='''    match try_decode_still_with_policy(b, Profile::Strict) {
        Ok((s, _)) => Ok(s),
        Err(e) => Err(e),
    }''';assert s.count(a)==1;s=s.replace(a,'''    let r=match try_decode_still_with_policy(b, Profile::Strict) {
        Ok((s, _)) => Ok(s),
        Err(e) => Err(e),
    };
    proof {lemma_T13_core(b@,r);}
    r''')
a='''    ensures still_g(b@,crate::policy::profile_record(p),r.0,r.1@.view()),
{still_with_logged(b,p.policy())}''';assert s.count(a)==1;s=s.replace(a,'''    ensures still_g(b@,crate::policy::profile_record(p),r.0,r.1@.view()),T13_still(b@,crate::policy::profile_record(p),r.0),
{
    let r=still_with_logged(b,p.policy());
    proof {reveal(still_g);lemma_T13_still(b@,crate::policy::profile_record(p),r.0);}
    r
}''')
rep('    ensures still_core_g(b@,out.0,out.1@.view()),','    ensures still_core_g(b@,out.0,out.1@.view()),T13_core(b@,out.0),')
rep('    proof {lemma_still_strip(b@,observed,log@.view());}','    proof {lemma_still_strip(b@,observed,log@.view());reveal(still_core_g);lemma_T13_core(b@,result);}')
# Public next-frame result gets the exact model-history pixel invariant.
rep('            forall|k:nat| T12_next(*old(self),*final(self),k,out),','''            forall|k:nat| T12_next(*old(self),*final(self),k,out),
            forall|n:int| wf_p(*old(self),n) ==> T13_view(*final(self),n+1,out),''')
rep('''                    assert forall|k:nat| T12_next(original,*self,k,None) by {lemma_T12_next(original,*self,k,None);}''','''                    assert forall|k:nat| T12_next(original,*self,k,None) by {lemma_T12_next(original,*self,k,None);}
                    assert forall|n:int| wf_p(original,n) implies T13_view(*self,n+1,None) by {lemma_T13_next(original,*self,n,None);}''')
rep('''            assert forall|k:nat| T12_next(original,*self,k,out) by {lemma_T12_next(original,*self,k,out);}''','''            assert forall|k:nat| T12_next(original,*self,k,out) by {lemma_T12_next(original,*self,k,out);}
            assert forall|n:int| wf_p(original,n) implies T13_view(*self,n+1,out) by {lemma_T13_next(original,*self,n,out);}''')
p.write_text(s)
print('Packaged model pixels, exact success/error observations and resources on public APIs')
