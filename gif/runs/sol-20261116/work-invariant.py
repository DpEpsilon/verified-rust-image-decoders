from pathlib import Path
import shutil
root=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src');p=root/'gif.rs';s=p.read_text();b=root.parent.parent/'runs/sol-20261116/t12-pre-work-invariant-gif.rs';assert not b.exists();shutil.copy2(p,b)
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
# Export the exact post-image area and successful Saved presence monotonically.
rep('    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+area','''    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+area
    &&& (a.saved is Some ==> d.saved is Some)
    &&& (r is Ok ==> d.prev is Some && prior_area(d)==area)''')
rep('    &&& a.p<=d.p\n    &&& d.work@ as int', '''    &&& a.p<=d.p
    &&& (a.saved is Some ==> d.saved is Some)
    &&& (d.state==State::Running ==> d.prev is Some && a.p<d.p && prior_area(d) as int==d.charged_pixels@ as int-a.charged_pixels@ as int)
    &&& (d.state==State::Done && a.p<a.b.len() && a.b@[a.p as int]==0x2c ==> false)
    &&& d.work@ as int''')
# Work ready is a reachable-state fact, separate from transient storage_wf.
rep('pub closed spec fn constructor_work(d:Frames)->bool {\n    d.work@ <= C_CALL * (d.p as nat+header_pixels(d.b@)) + C_FIXED && d.charged_pixels@==0\n}', '''pub closed spec fn constructor_work(d:Frames)->bool {
    &&& d.work@ <= C_CALL * (d.p as nat+header_pixels(d.b@)) + C_FIXED
    &&& d.charged_pixels@==0
    &&& work_accounted(d)
}
spec fn work_ready(d:Frames)->bool {
    d.state==State::Running ==> d.prev is Some || d.p<d.b.len() && d.b@[d.p as int]==0x2c
}
spec fn work_potential(d:Frames)->int {
    d.work@ as int + if d.state==State::Running {2*prior_area(d)+1} else {0}
        + if d.saved is None {d.canvas@.len() as int} else {0}
}
pub closed spec fn work_accounted(d:Frames)->bool {
    &&& work_ready(d)
    &&& work_potential(d)<=C_CALL as int * (effective_cursor(d)+header_pixels(d.b@) as int+2*d.charged_pixels@ as int)+C_FIXED as int
}
pub closed spec fn total_work_bound(d:Frames)->bool {
    d.work@ as int<=C_CALL as int*(d.b.len() as int+2*header_pixels(d.b@) as int+2*d.charged_pixels@ as int)+C_FIXED as int
        && d.work@ as int<=C_WORK as int*(d.b.len() as int+header_pixels(d.b@) as int+d.charged_pixels@ as int)+C_FIXED as int
}''')
# Per-call quantitative observation. The N term is needed only on successful Saved growth.
rep('    &&& (a.state==State::Running ==> step_cost(a,d))','''    &&& (a.state==State::Running ==> step_cost(a,d))
    &&& (work_accounted(a) ==> work_accounted(d) && total_work_bound(d))
    &&& (work_ready(a) ==> d.work@ as int <= a.work@ as int + C_CALL as int *
        (effective_cursor(d)-effective_cursor(a)+d.charged_pixels@ as int-a.charged_pixels@ as int
         + if a.state==State::Running {prior_area(a) as int} else {0}
         + if a.saved is None && d.saved is Some {header_pixels(a.b@) as int} else {0}))''')
# Tight constructor observation leaves credit for the once-only Saved fill.
rep('Ok(d)=>d.work@==final(steps)@ && final(steps)@ <= old(steps)@ + C_CALL * (d.p as nat+header_pixels(b@)) + C_FIXED,','''Ok(d)=>d.work@==final(steps)@ && final(steps)@ <= old(steps)@ + C_CALL * (d.p as nat+header_pixels(b@)) + C_FIXED
                && final(steps)@ as int<=old(steps)@ as int+4*header_pixels(b@) as int+crate::lzw::LZW_TABLE_BYTES as int+4*d.p as int+1,''')
rep('                    if !still {lemma_constructor_wf(dec);}','                    if !still {lemma_constructor_wf(dec);}\n                    lemma_constructor_work(dec);')
# Pure accounting lemmas keep arithmetic out of executable semantic queries.
pos=s.index('pub closed spec fn storage_wf(')
s=s[:pos]+'''proof fn lemma_work_header(d:Frames)
    requires storage_wf(d),
    ensures d.canvas@.len() as int==4*header_pixels(d.b@) as int,
{
    hide(crate::model::mp);
    reveal(storage_wf);reveal(crate::parse::hdr_ok);
    let h=choose|h:crate::model::HeaderM| crate::model::mh(d.b@)==Ok(h) && crate::parse::hdr_matches(d.b@,h,d.hdr);
    assert(header_pixels(d.b@) as int==(d.hdr.width as int)*(d.hdr.height as int));
    assert(4*(d.hdr.width as int)*(d.hdr.height as int)==4*header_pixels(d.b@) as int) by(nonlinear_arith)
        requires header_pixels(d.b@) as int==(d.hdr.width as int)*(d.hdr.height as int);
}
proof fn lemma_constructor_work(d:Frames)
    requires constructor_ok(d,d.b@,d.pol,d.still),d.charged_pixels@==0,
        d.work@ as int<=4*header_pixels(d.b@) as int+crate::lzw::LZW_TABLE_BYTES as int+4*d.p as int+1,
    ensures constructor_work(d),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    reveal(constructor_ok);lemma_work_header(d);
}
proof fn lemma_work_transition(a:Frames,d:Frames)
    requires storage_wf(a),storage_wf(d),fixed_stream(a,d),
        a.state==State::Running,step_cost(a,d),work_ready(a),
    ensures work_ready(d),
        work_potential(d)<=work_potential(a)+C_CALL as int *
            (effective_cursor(d)-effective_cursor(a)+2*(d.charged_pixels@ as int-a.charged_pixels@ as int)),
        d.work@ as int<=a.work@ as int+C_CALL as int *
            (effective_cursor(d)-effective_cursor(a)+d.charged_pixels@ as int-a.charged_pixels@ as int+prior_area(a) as int
                +if a.saved is None && d.saved is Some {header_pixels(a.b@) as int} else {0}),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    lemma_storage_bounds(a);lemma_storage_bounds(d);
    lemma_work_header(a);
    if a.prev is Some {
        let r=a.prev.unwrap().rect;
        assert(prior_area(a)>=1) by(nonlinear_arith)
            requires r.width>=1,r.height>=1,prior_area(a)==(r.width as nat)*(r.height as nat);
    }
}
proof fn lemma_next_work(a:Frames,d:Frames)
    requires storage_wf(a),storage_wf(d),fixed_stream(a,d),
        if a.state==State::Running {step_cost(a,d)} else {d==a},
    ensures next_work(a,d),
{
    hide(crate::model::mp);hide(crate::parse::hdr_ok);
    if a.state==State::Running && work_ready(a) {lemma_work_transition(a,d);}
    if work_accounted(a) {
        lemma_storage_bounds(d);
        assert(work_potential(d)>=d.work@ as int);
    }
}

'''+s[pos:]
# Call arithmetic transition only after the concrete frame view has been built.
rep('        let out=match self.step() {','        let out=match self.step() {')
rep('                return None;','                proof {lemma_next_work(original,*self);}\n                return None;')
rep('        out\n    }\n\n}\n} // verus!','        proof {lemma_next_work(original,*self);}\n        out\n    }\n\n}\n} // verus!')
# Explicit physical framing used by step_cost, independent of semantic image result.
rep('        image_cost(*old(self),image,*final(self),r),','        image_cost(*old(self),image,*final(self),r),')
rep('        saved_view(*final(self))==saved_view(*old(self)),','        r is Ok ==> final(self).prev is Some && prior_area(*final(self))==rect_area(image.fh.rect),\n            saved_view(*final(self))==saved_view(*old(self)),')
rep('            r is Ok && r->Ok_0 is Some ==> r->Ok_0->Some_0.data.data_len as int<=final(self).p as int-old(self).p as int,','''            r is Ok && r->Ok_0 is Some ==> old(self).p<final(self).p && r->Ok_0->Some_0.data.data_len as int<=final(self).p as int-old(self).p as int,
            old(self).p<old(self).b.len() && old(self).b@[old(self).p as int]==0x2c ==> !(r is Ok && r->Ok_0 is None),''')
p.write_text(s)
p=root/'parse.rs';s=p.read_text();start=s.index('pub fn parse_mb(');end=s.index('\n}',start)+2;f=s[start:end];i=f.index('\n{');f=f[:i]+'''\n        r is Ok && r->Ok_0 is Some ==> *old(p)<*final(p),
        *old(p)<b.len() && b@[*old(p) as int]==0x2c ==> !(r is Ok && r->Ok_0 is None),'''+f[i:];s=s[:start]+f+s[end:];p.write_text(s)
print('Added reachable work readiness, exact per-call arithmetic and a cumulative potential')
