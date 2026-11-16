from pathlib import Path
import shutil
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/gif.rs');s=p.read_text()
b=p.parent.parent.parent/'runs/sol-20261116/t12-pre-integrated-gif.rs';assert not b.exists();shutil.copy2(p,b)
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
def amend(name,contract='',body=None):
 global s
 start=s.index('fn '+name+'(');end=s.index('\n    }' if s[start-4:start].strip()=='' and False else '\n}',start)+2 if False else None
 # Method closing braces are indented; use signature indentation.
 line=s.rfind('\n',0,start)+1;indent=len(s[line:start])-len(s[line:start].lstrip())
 # Actual fn may follow pub; indentation is whitespace at line start.
 indent=len(s[line:])-len(s[line:].lstrip(' '))
 close='\n'+' '*indent+'}'
 end=s.index(close,s.index('\n'+' '*indent+'{',start))+len(close)
 f=s[start:end]
 brace='\n'+' '*indent+'{';i=f.index(brace)
 if contract:f=f[:i]+'\n'+(' '* (indent+4))+contract+f[i:]
 if body:f=body(f)
 assert f!=s[start:end],name;s=s[:start]+f+s[end:]
# Cost units are erased, and the fixed allowance is never charged per frame.
anchor='verus! {\n\n/// Entries' # constants instead placed in first verus block via known req function
pos=s.index('pub open spec fn req_new(')
s=s[:pos]+'''pub const C_CALL: nat = 32;
pub const C_WORK: nat = 64;
pub const C_FIXED: nat = 41_984;

pub open spec fn header_pixels(b:Seq<u8>)->nat {
    match crate::model::mh(b) {Ok(h)=>(h.width*h.height) as nat,Err(_)=>0}
}

'''+s[pos:]
rep('    work: Ghost<nat>,','    work: Ghost<nat>,\n    charged_pixels: Ghost<nat>,')
rep('work:Ghost(0nat)}','work:Ghost(d.work@),charged_pixels:Ghost(0nat)}')
# Make successful reservation fills visible to the caller; the guard stays pinned.
rep('    hdr: &Header,\n    Tracked(log)', '    hdr: &Header,\n    steps: &mut Ghost<nat>,\n    Tracked(log)')
amend('reserve','''old(steps)@ <= final(steps)@,
        final(steps)@ as int <= old(steps)@ as int + 4*(hdr.width as int)*(hdr.height as int) + crate::lzw::LZW_TABLE_BYTES as int,
        r is Ok ==> final(steps)@ as int == old(steps)@ as int + 4*(hdr.width as int)*(hdr.height as int) + crate::lzw::LZW_TABLE_BYTES as int,''',lambda f:f.replace('        Ok(v) => v,','        Ok(v) => {proof {steps@=steps@+v@.len();} v},'))
rep('    table:Vec<u8>,\n}', '    table:Vec<u8>,\n    work:Ghost<nat>,\n}')
rep('fn prepare(b:&[u8],pol:Policy,Tracked(log)', 'fn prepare(b:&[u8],pol:Policy,steps:&mut Ghost<nat>,Tracked(log)')
amend('prepare','''old(steps)@<=final(steps)@,
        match r {
            Ok(d)=>d.work@==final(steps)@ && final(steps)@ <= old(steps)@ + C_CALL * (d.p as nat+header_pixels(b@)) + C_FIXED,
            Err(_)=>final(steps)@ <= old(steps)@ + C_CALL * (b.len() as nat+header_pixels(b@)) + C_FIXED,
        },''',lambda f:f.replace('reserve(b,&hdr,Tracked(log))','reserve(b,&hdr,steps,Tracked(log))').replace('    let mut work=Ghost(0nat);\n','').replace('&mut st,&mut work)','&mut st,steps)').replace('Ok(Prepared {hdr,p,st,stop,canvas,table})','Ok(Prepared {hdr,p,st,stop,canvas,table,work:Ghost(steps@)})').replace('        assert(crate::parse::hdr_ok(b@,hdr));','''        assert(crate::parse::hdr_ok(b@,hdr));
        reveal(header_pixels);
        assert(header_pixels(b@) as int==(hdr.width as int)*(hdr.height as int));'''))
rep('        let prepared=prepare(b,pol,Tracked(&mut log));','        let mut steps=Ghost(0nat);\n        let prepared=prepare(b,pol,&mut steps,Tracked(&mut log));')
amend('open_logged','r.0 is Ok ==> constructor_work(r.0->Ok_0),',lambda f:f.replace('            Err(e)=>(Err(e),Tracked(Some(log))),','''            Err(e)=>{
                proof {assert(steps@<=C_CALL*(b.len() as nat+header_pixels(b@))+C_FIXED);}
                (Err(e),Tracked(Some(log)))
            },'''))
# Cost observations do not expand the semantic/storage invariants.
pos=s.index('pub closed spec fn storage_wf(')
s=s[:pos]+'''pub closed spec fn constructor_work(d:Frames)->bool {
    d.work@ <= C_CALL * (d.p as nat+header_pixels(d.b@)) + C_FIXED && d.charged_pixels@==0
}
pub closed spec fn work_view(d:Frames)->nat {d.work@}
pub closed spec fn charged_pixels(d:Frames)->nat {d.charged_pixels@}
spec fn rect_area(r:Rect)->int {(r.width as int)*(r.height as int)}
spec fn prior_area(d:Frames)->int {match d.prev {None=>0,Some(p)=>rect_area(p.rect)}}
spec fn saved_fill(a:Frames,d:Frames)->int {
    if a.saved is None && d.saved is Some {a.canvas@.len() as int} else {0}
}
spec fn effective_cursor(d:Frames)->int {if d.state==State::Fused {d.b.len() as int} else {d.p as int}}
spec fn image_cost(a:Frames,im:parse::ImageStep,d:Frames,r:Result<FrameMeta,TryError>)->bool {
    let area=if r is Err && r->Err_0 is OutOfMemory {0int} else {rect_area(im.fh.rect)};
    &&& d.charged_pixels@ as int==a.charged_pixels@ as int+area
    &&& a.work@<=d.work@
    &&& d.work@ as int <= a.work@ as int + 5*im.data.data_len as int + 4*area + 2*prior_area(a) + saved_fill(a,d)
}
spec fn step_cost(a:Frames,d:Frames)->bool {
    &&& a.work@<=d.work@ && a.charged_pixels@<=d.charged_pixels@
    &&& a.p<=d.p
    &&& d.work@ as int <= a.work@ as int + 9*(effective_cursor(d)-a.p as int)
        +4*(d.charged_pixels@ as int-a.charged_pixels@ as int)+2*prior_area(a)+saved_fill(a,d)+1
}

'''+s[pos:]
# Propagate every measured helper cost through the actual split adapters.
amend('scan_next','''final(self).charged_pixels@==old(self).charged_pixels@,
            old(self).p<=final(self).p,old(self).work@<=final(self).work@,
            final(self).work@ as int <= old(self).work@ as int + 4 * (effective_cursor(*final(self))-old(self).p as int)+1,
            r is Ok && r->Ok_0 is Some ==> r->Ok_0->Some_0.data.data_len as int<=final(self).p as int-old(self).p as int,''')
amend('apply_previous','''final(self).charged_pixels@==old(self).charged_pixels@,
            old(self).work@<=final(self).work@,
            final(self).work@ as int<=old(self).work@ as int+2*prior_area(*old(self)),''',lambda f:f.replace('clear_rect(&mut self.canvas,sw,sh,prev.rect);','let cost=clear_rect(&mut self.canvas,sw,sh,prev.rect);proof {self.work@=self.work@+cost@;}').replace('copy_rect(&mut self.canvas,saved,sw,sh,prev.rect);','let cost=copy_rect(&mut self.canvas,saved,sw,sh,prev.rect);proof {self.work@=self.work@+cost@;}'))
amend('ensure_saved','''final(self).charged_pixels@==old(self).charged_pixels@,
            final(self).work@ as int == old(self).work@ as int+saved_fill(*old(self),*final(self)),''',lambda f:f.replace('                self.saved=Some(v);','                proof {self.work@=self.work@+v@.len();}\n                self.saved=Some(v);'))
amend('prepare_canvas','''final(self).charged_pixels@==old(self).charged_pixels@,
            old(self).work@<=final(self).work@,
            final(self).work@ as int<=old(self).work@ as int+2*prior_area(*old(self))+saved_fill(*old(self),*final(self))
                + if r is Ok && disposal==3 && !old(self).still {2*rect_area(rect)} else {0},''',lambda f:f.replace('copy_rect(saved,&self.canvas,sw,sh,rect);','let cost=copy_rect(saved,&self.canvas,sw,sh,rect);proof {self.work@=self.work@+cost@;}'))
amend('image_prepared','''final(self).charged_pixels@ as int==old(self).charged_pixels@ as int+rect_area(image.fh.rect),
            old(self).work@<=final(self).work@,
            final(self).work@ as int<=old(self).work@ as int+5*image.data.data_len as int+2*rect_area(image.fh.rect),''',lambda f:f.replace('        let ghost observed=report;','''        proof {
            self.work@=self.work@+report.work@;
            self.charged_pixels@=(self.charged_pixels@ as int+rect_area(image.fh.rect)) as nat;
        }
        let ghost observed=report;'''))
amend('image','image_cost(*old(self),image,*final(self),r),')
amend('step','step_cost(*old(self),*final(self)),')
amend('next_frame','''work_view(*old(self))<=work_view(*final(self)),charged_pixels(*old(self))<=charged_pixels(*final(self)),
            old(self).state!=State::Running ==> work_view(*old(self))==work_view(*final(self)) && charged_pixels(*old(self))==charged_pixels(*final(self)),
            old(self).state==State::Running ==> step_cost(*old(self),*final(self)),''')
# Successful constructor wrappers retain the observed cumulative measurement.
for name in ['open','try_new','try_new_with_policy','try_new_with_record']:
 amend(name,'r is Ok ==> constructor_work(r->Ok_0),')
for name in ['try_new_logged','try_new_with_policy_logged']:
 amend(name,'r.0 is Ok ==> constructor_work(r.0->Ok_0),')
p.write_text(s)
print('Connected cumulative work, successful fills, reached-G10 pixels and split adapters')
