from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text()
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
a='''    old(steps)@<=final(steps)@,
        match r {''';assert s.count(a)==1;s=s.replace(a,'''        old(steps)@<=final(steps)@,
        r is Err && r->Err_0 is OutOfMemory ==> final(steps)@ as int<=old(steps)@ as int+4*header_pixels(b@) as int+crate::lzw::LZW_TABLE_BYTES as int,
        match r {''')
a='''                proof {assert(steps@<=C_CALL*(b.len() as nat+header_pixels(b@))+C_FIXED);}''';assert s.count(a)==1;s=s.replace(a,'''                proof {
                    assert(steps@<=C_CALL*(b.len() as nat+header_pixels(b@))+C_FIXED);
                    crate::work_model::lemma_initial_cur_bounds(b@,pol);
                    if !(e is OutOfMemory) {reveal(prepare_result);assert(crate::model::mp(b@,pol) is Err);}
                    assert(steps@ as int<=C_CALL as int*(crate::work_model::cur_n(b@,pol,None,-1)+header_pixels(b@) as int)+C_FIXED as int);
                }''')
pos=s.index('proof fn lemma_work_header(')
s=s[:pos]+'''proof fn lemma_initial_model(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,d.still),
    ensures crate::frame_model::initial(d.b@,d.pol)==Some(model_state(d)),
{
    hide(crate::model::mp);reveal(constructor_ok);
}
proof fn lemma_still_work_area(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)
    requires constructor_ok(a,a.b@,a.pol,true),a.charged_pixels@==0,a.state==State::Running,
        step_model_work(a,d,r),!(r is Err && r->Err_0 is OutOfMemory),
    ensures d.charged_pixels@==crate::work_model::P(a.b@,a.pol,None,1),
{
    hide(crate::model::mp);hide(crate::model::mb);
    lemma_work_mh(a);lemma_initial_model(a);
    reveal_with_fuel(crate::frame_model::after_calls,1);
}
proof fn lemma_still_empty_work_area(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,true),d.charged_pixels@==0,d.state!=State::Running,
    ensures d.charged_pixels@==crate::work_model::P(d.b@,d.pol,None,1),
{
    hide(crate::model::mp);hide(crate::model::mb);
    lemma_work_mh(d);lemma_initial_model(d);
    reveal_with_fuel(crate::frame_model::after_calls,1);
}

pub closed spec fn steps_total(ds:Seq<Frames>)->int
    decreases ds.len(),
{
    if ds.len()==0 {0} else if ds.len()==1 {ds[0].work@ as int}
    else {steps_total(ds.take(ds.len() as int-1))+ds.last().work@ as int-ds[ds.len() as int-2].work@ as int}
}
proof fn lemma_steps_total(ds:Seq<Frames>)
    requires ds.len()>0,
    ensures steps_total(ds)==ds.last().work@ as int,
    decreases ds.len(),
{
    reveal_with_fuel(steps_total,1);
    if ds.len()>1 {
        let pr=ds.take(ds.len() as int-1);lemma_steps_total(pr);
        assert(pr.last()==ds[ds.len() as int-2]);
    }
}
/// Constructor plus any finite next_frame history; info calls are immutable
/// and can be inserted without changing any snapshot or work summand.
pub proof fn theorem_T12_finite(ds:Seq<Frames>,rs:Seq<Option<Result<FrameView,TryError>>>,cuts:Seq<Option<nat>>)
    requires ds.len()==rs.len()+1,cuts.len()==ds.len(),cuts[0] is None,
        constructor_work(ds[0]),!ds[0].still,
        forall|i:int| 0<=i<ds.len() ==> storage_wf(#[trigger] ds[i]) && ds[i].b==ds[0].b && ds[i].pol==ds[0].pol,
        forall|i:int| 0<=i<rs.len() ==> T12_next(#[trigger] ds[i],ds[i+1],i as nat,rs[i])
            && cuts[i+1]==next_cut(cuts[i],i as nat,rs[i]),
    ensures work_wf(ds.last(),rs.len() as int-1,cuts.last()),
        steps_total(ds)<=C_CALL as int*(ds[0].b.len() as int+2*header_pixels(ds[0].b@) as int
            +2*crate::work_model::P(ds[0].b@,ds[0].pol,cuts.last(),rs.len()) as int)+C_FIXED as int,
        steps_total(ds)<=C_WORK as int*(ds[0].b.len() as int+header_pixels(ds[0].b@) as int
            +crate::work_model::P(ds[0].b@,ds[0].pol,cuts.last(),rs.len()) as int)+C_FIXED as int,
    decreases rs.len(),
{
    hide(crate::work_model::P);hide(crate::work_model::A_n);hide(crate::work_model::cur_n);
    if rs.len()==0 {reveal(constructor_work);lemma_work_total(ds[0]);}
    else {
        let dp=ds.take(ds.len() as int-1);
        let rp=rs.take(rs.len() as int-1);
        let cp=cuts.take(cuts.len() as int-1);
        assert forall|i:int| 0<=i<dp.len() implies storage_wf(#[trigger] dp[i]) && dp[i].b==dp[0].b && dp[i].pol==dp[0].pol by {assert(dp[i]==ds[i]);}
        assert forall|i:int| 0<=i<rp.len() implies T12_next(#[trigger] dp[i],dp[i+1],i as nat,rp[i])
            && cp[i+1]==next_cut(cp[i],i as nat,rp[i]) by {assert(dp[i]==ds[i]);assert(dp[i+1]==ds[i+1]);}
        theorem_T12_finite(dp,rp,cp);
        let i=rs.len() as int-1;
        assert(dp.last()==ds[i]);assert(cp.last()==cuts[i]);
        reveal(T12_next);
    }
    lemma_steps_total(ds);
}

'''+s[pos:]
# Assert the frozen P, not only the measured pixel counter, on every actual still path.
a='''                lemma_still_empty(d);reveal(constructor_work);lemma_work_total(d);''';assert s.count(a)==1;s=s.replace(a,a+'\n                lemma_still_empty_work_area(d);')
a='''                assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+d.charged_pixels@ as int)+C_FIXED as int);''';assert s.count(a)==1;s=s.replace(a,a+'''\n                assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+crate::work_model::P(b@,pol,None,1) as int)+C_FIXED as int);''')
a='''        reveal(constructor_work);lemma_next_work(initial,d);''';assert s.count(a)==1;s=s.replace(a,a+'\n        lemma_still_work_area(initial,d,step);')
a='''        assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+d.charged_pixels@ as int)+C_FIXED as int);''';# contains one in empty too substring; replace wholeline unindented anchored
lines=s.splitlines(True);found=0
for i,line in enumerate(lines):
 if line.startswith(a):
  lines[i]=line+'        assert(d.work@ as int<=C_WORK as int*(b.len() as int+header_pixels(b@) as int+crate::work_model::P(b@,pol,None,1) as int)+C_FIXED as int);\n';found+=1
assert found==1,found;s=''.join(lines)
p.write_text(s)
print('Added exact constructor error bound, still P identification and finite frozen T12 sum')
