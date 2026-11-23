#!/usr/bin/env python3
"""Check erased iterator observations against actual work-entry sites."""
import argparse
import re
from pathlib import Path
ROOT=Path('/home/ws/projects/verified-decoders/png/png_verus')

def function(text,name):
    import re
    hit=re.search(r'\bfn\s+'+re.escape(name)+r'\b',text)
    assert hit,name
    # All checked entry bodies use this delimiter after their contracts.
    opening=re.search(r'\n\s*\{\n',text[hit.start():])
    assert opening,name
    start=hit.start()+opening.end()-2
    assert text[start]=='{',(name,text[start:start+20])
    depth=1;i=start+1
    while depth:
        assert i<len(text),name
        depth+=(text[i]=='{')-(text[i]=='}');i+=1
    return text[hit.start():start],text[start+1:i-1]

def allowed_terminal_operations(png,state):
    """Pin the fixed-size executable dispatch and every helper on its fused path.

    Ghost bookkeeping is erased using the existing allocation scanner. This
    allowlist rejects any new executable statement or helper call, even when
    the new helper hides its loop outside terminal_logged.
    """
    import importlib.util
    path=ROOT.parents[1]/'gif/tools/alloc_scan.py'
    spec=importlib.util.spec_from_file_location('terminal_erasure_scanner',path)
    scanner=importlib.util.module_from_spec(spec);spec.loader.exec_module(scanner)
    def executable(body):
        value=scanner.strip_verus_nonexec(scanner.strip_literals_and_comments(body))
        value=re.sub(r'\b(?:hide|reveal|reveal_with_fuel)\s*\([^;]*\)\s*;','',value)
        return ''.join(value.split())
    allowed={
      'try_next': 'self.next_logged()',
      'next_logged': '''match self.begin_source_observed(Tracked(&mut activity)) {
        Step::Done=>self.terminal_logged(false,Tracked(&mut activity),Ghost(prior_receiver)),
        Step::Single=>self.terminal_logged(true,Tracked(&mut activity),Ghost(prior_receiver)),
        Step::Frame(job)=>self.selected_logged(job,Tracked(&mut activity),Ghost(prior_receiver)),
      }''',
      'begin_source_observed': 'self.state.begin_next_observed(self.b,&self.doc,Tracked(activity))',
      'terminal_logged': '''let w=self.doc.ihdr.w;let h=self.doc.ihdr.h;
        if single {Ok(Some(FrameView {rgba:&self.canvas,rect:(0,0,w,h),delay:None,
          dispose:0,blend:0,recovered:self.base.any(),events:self.base}))}else{Ok(None)}''',
      'decode_frame_image': 'let result=decode_image(doc,b,src,raw,w,h,pol,ev,sink);result',
      'any': 'self.r1 || self.r3 || self.r4',
      'begin_next_observed': '''let prior=self.kind;self.kind=Kind::Done;
        match prior {
          Kind::Done=>Step::Done,Kind::Single=>Step::Single,
          Kind::Anim {index,cursor,previous}=>{
            if index>=doc.info.frame_count {return Step::Done;}
            Step::Frame(FrameJob::select(b,doc,index,cursor,previous))
          },
        }''',
    }
    for name,expected in allowed.items():
        _,body=function(state if name=='begin_next_observed' else png,name)
        assert executable(body)==executable(expected),(name,'unexpected executable operation or helper call on terminal dispatch')
    return len(allowed)

def negative_terminal_controls(png,state):
    """In-memory control checks; no decoder or corpus mutation is run."""
    _,body=function(png,'terminal_logged')
    _,forward_body=function(png,'begin_source_observed')
    cases=[
      png.replace(body,body.replace('        if single {','        let _extra=crate::crc::crc32(self.b);\n        if single {',1),1),
      png.replace(forward_body,'\n        let _extra=crate::crc::crc32(self.b);\n'+forward_body,1),
    ]
    assert all(source!=png for source in cases)
    for source in cases:
        try:allowed_terminal_operations(source,state)
        except AssertionError:continue
        raise AssertionError('Unexpected terminal helper call passed the allowlist')
    return len(cases)

def check(png,state,trace):
    bindings=[
      (state,'begin_next_observed','Select','Step::Frame(FrameJob::select(b,doc,index,cursor,previous))'),
      (png,'ensure_saved','ReserveSaved','let result=reserve_saved(n,Tracked(self.log.borrow_mut()));'),
      (png,'draw_selected','NormalizeReused','normalize_reused(&mut self.canvas,f.blend==1);'),
      (png,'decode_frame_image','Decode','let result=decode_image(doc,b,src,raw,w,h,pol,ev,sink);'),
      (png,'finish_job','DisposeSave','self.dispose_and_save(job.previous,(f.x,f.y,f.w,f.h),job.effective_dispose);'),
      (png,'finish_job','Resume','self.commit_source(resume);'),
    ]
    for source,name,site,call in bindings:
        header,body=function(source,name)
        marker='proof {activity.record(crate::frame_activity::Site::'+site+');}'
        assert body.count(marker)==1,(name,site,'marker count')
        assert body.count(call)==1,(name,site,'call count')
        assert marker+' '+call in ' '.join(body.split()),(name,site,'marker is not immediately before actual call')
        assert 'Tracked<&mut crate::png::IteratorActivity>' in header,name
        assert 'Trace::new' not in body,(name,'must not reset observation')
        assert not re.search(r'\bactivity\s*=',body),(name,'must not replace the threaded trace')
    for name in ['begin_source_observed','selected_logged','completed_selected','terminal_logged','finish_iteration']:
        header,body=function(png,name)
        assert 'Tracked<&mut crate::png::IteratorActivity>' in header,name
        assert 'Trace::new' not in body,(name,'must not reset observation')
    header,body=function(png,'next_logged')
    assert body.count('Trace::new()')==1
    assert 'self.begin_source_observed(Tracked(&mut activity))' in body
    assert body.count('Tracked(&mut activity)')==4
    for name in ['try_next','next_logged']:
        header,body=function(png,name)
        assert 'old(self).is_done() ==> crate::frame_activity::idle(final(self).last_work())' in header,name
    assert 'pub(crate) type IteratorActivity = crate::frame_activity::Trace;' in png
    assert '#[cfg(not(verus_only))]\npub(crate) type IteratorActivity = ();' in png
    forwarding={
      'begin_source_observed':['self.state.begin_next_observed(self.b,&self.doc,Tracked(activity))'],
      'selected_logged':['self.ensure_saved(k>0 && f.dispose==2,Tracked(activity))','self.completed_selected(job,Tracked(activity),Ghost(prior_receiver))'],
      'completed_selected':['self.finish_iteration(job,Tracked(activity))'],
      'finish_iteration':['self.finish_job(job,Tracked(activity))'],
      'finish_job':['self.draw_selected(job,Tracked(activity))'],
      'draw_selected':['decode_frame_image(&self.doc,self.b,&mut src,&mut self.raw,f.w,f.h,&self.pol,&mut fev,&mut sink,Tracked(activity))'],
    }
    for name,calls in forwarding.items():
        _,body=function(png,name)
        assert 'Trace::new' not in body and not re.search(r'\bactivity\s*=',body),name
        for call in calls:assert body.count(call)==1,(name,call)
    assert png.count('self.last_work=Ghost(activity.view());')==5
    assert len(re.findall(r'\blast_work\s*=',png))==5
    assert png.count('Frames::work_preserves(before_work,*self);')==5
    assert 'self.last_work=Ghost(crate::frame_activity::zero())' not in png
    header,body=function(png,'terminal_logged')
    assert 'final(self).last_work()==old(activity).view()' in header
    assert not any(word in body.split() for word in ['loop','while','for'])
    assert 'ghost work: Work' in trace
    assert 'self.work=recorded(self.work,site)' in trace
    assert trace.count('self.work=')==1
    assert state.count('FrameJob::select(')==1
    assert png.count('reserve_saved(n,Tracked(self.log.borrow_mut()))')==1
    assert allowed_terminal_operations(png,state)==7
    assert negative_terminal_controls(png,state)==2
    return len(bindings)

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,default=ROOT)
    args=parser.parse_args()
    src=args.root/'src'
    n=check((src/'png.rs').read_text(),(src/'frame_state.rs').read_text(),(src/'frame_activity.rs').read_text())
    print(f'Actual-site iterator observations: {n} bindings; one threaded trace; five snapshots; seven executable-body allowlists; two hidden-helper rejection controls passed.')
