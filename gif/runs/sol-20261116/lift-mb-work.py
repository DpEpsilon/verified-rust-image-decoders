from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text()
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
# Structural observations flow from actual MB, through scan, into step.
a='''            old(self).p<old(self).b.len() && old(self).b@[old(self).p as int]==0x2c ==> !(r is Ok && r->Ok_0 is None),'''
assert s.count(a)==1;s=s.replace(a,a+'''
            effective_cursor(*final(self))==crate::work_model::mb_cur(old(self).b@,crate::parse::header_model(old(self).hdr),old(self).pol,crate::parse::walk_m_at(old(self).st,old(self).p)),
            (match r {Ok(Some(im))=>rect_area(im.fh.rect),_=>0nat})
                ==crate::work_model::mb_area(old(self).b@,crate::parse::header_model(old(self).hdr),old(self).pol,crate::parse::walk_m_at(old(self).st,old(self).p)),''')
rep('        step_cost(*old(self),*final(self)),','        step_cost(*old(self),*final(self)),step_model_work(*old(self),*final(self),r),')
rep('            next_work(*old(self),*final(self)),','            next_work(*old(self),*final(self)),next_model_work(*old(self),*final(self),out),')
pos=s.index('spec fn image_cost(')
s=s[:pos]+'''spec fn step_model_work(a:Frames,d:Frames,r:Result<Option<FrameMeta>,TryError>)->bool {
    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+
        if r is Err && r->Err_0 is OutOfMemory {0int} else {crate::work_model::mb_area(a.b@,crate::parse::header_model(a.hdr),a.pol,crate::parse::walk_m_at(a.st,a.p)) as int}
    &&& effective_cursor(d)==if r is Err {a.b.len() as int} else {crate::work_model::mb_cur(a.b@,crate::parse::header_model(a.hdr),a.pol,crate::parse::walk_m_at(a.st,a.p))}
}
pub closed spec fn next_model_work(a:Frames,d:Frames,r:Option<Result<FrameView,TryError>>)->bool {
    if a.state!=State::Running {d==a} else {
        step_model_work(a,d,match r {None=>Ok(None),Some(Err(e))=>Err(e),Some(Ok(v))=>Ok(Some(view_metadata(v)))})
    }
}

'''+s[pos:]
p.write_text(s)
print('Lifted independent MB cursor/area observations through the actual public call')
