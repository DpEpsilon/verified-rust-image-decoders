from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/png/png_verus/src/png.rs');s=p.read_text()
a=s.index('/// The reservation requests a call made');z=s.index('/// The checked still/ICC structural walk',a)
s=s[:a]+'''verus! {

/// One public call's runtime request record and owned, sealed logical log.
pub(crate) struct Trace {
    sites: [(AllocSite, usize); 3],
    n: usize,
    log: Tracked<AllocLog>,
}
impl Trace {
    closed spec fn log_view(&self) -> Seq<(AllocSite,nat,bool)> {self.log@.view()}
    closed spec fn prefix_recorded(&self, count: nat) -> bool {
        count<=3 && count<=self.log_view().len()
        && forall|i: int| 0<=i<count ==> self.sites@[i].0==self.log_view()[i].0
            && self.sites@[i].1==self.log_view()[i].1
    }
    closed spec fn wf(&self) -> bool {
        self.n==self.log_view().len() && self.prefix_recorded(self.n as nat)
    }
    fn new() -> (r: Self)
        ensures r.wf(),r.n==0,r.log_view()==Seq::<(AllocSite,nat,bool)>::empty(),
    {
        let tracked log=crate::alloc_guard::new_log();
        Self {sites:[(AllocSite::Raw,0);3],n:0,log:Tracked(log)}
    }
    /// Record after the guard logs its success/failure; public return traces
    /// include the request on both paths, without an unsealed log mutation.
    fn record(&mut self, site: AllocSite, elements: usize)
        requires old(self).n<3,old(self).log_view().len()==old(self).n+1,
            old(self).prefix_recorded(old(self).n as nat),
            old(self).log_view().last().0==site,old(self).log_view().last().1==elements,
        ensures final(self).wf(),final(self).n==old(self).n+1,
            final(self).log_view()==old(self).log_view(),
    {
        self.sites[self.n]=(site,elements);
        self.n+=1;
    }
    #[cfg(feature = "test-records")]
    fn list(&self) -> (r: ([(AllocSite, usize); 3], usize))
        ensures r==(self.sites,self.n),
    {(self.sites,self.n)}
}
fn oom(o: OutOfMemory) -> (r: TryError)
    ensures r==TryError::OutOfMemory(o),
{TryError::OutOfMemory(o)}

} // verus!

'''+s[z:]
a=s.index('/// `Raw` then `Output`');z=s.index('/// P15 for RGBA8',a)
s=s[:a]+'''verus! {

/// Actual Raw/Output prefix, borrowing this call's sealed log.
fn reserve_image(doc: &Doc, out_bytes: usize, trace: &mut Trace) -> (r: Result<(Vec<u8>,Vec<u8>),TryError>)
    requires doc.ihdr.wf(),1<=out_bytes<=crate::alloc_guard::MAX_BYTES,
        old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        match r {Ok((raw,out))=>
            raw@==Seq::new(crate::layout_model::raw(doc.ihdr.interlace,crate::layout_model::bits(doc.ihdr.view()),doc.ihdr.w as nat,doc.ihdr.h as nat),|_i: int|0u8)
            && out@==Seq::new(out_bytes as nat,|_i: int|0u8)
            && final(trace).log_view()==seq![(AllocSite::Raw,raw@.len(),true),(AllocSite::Output,out_bytes as nat,true)],
            Err(TryError::Format(_))=>false,
            Err(TryError::OutOfMemory(_))=>final(trace).log_view().len()==1 || final(trace).log_view().len()==2},
{
    let bytes=raw_size(&doc.ihdr,doc.ihdr.w,doc.ihdr.h);
    proof {crate::limits::caps_ok();}
    let n=bytes as usize;
    let result=try_zeroed(AllocSite::Raw,n,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::Raw,n);
    let raw=match result {Ok(v)=>v,Err(o)=>return Err(oom(o))};
    let result=try_zeroed(AllocSite::Output,out_bytes,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::Output,out_bytes);
    match result {Ok(out)=>Ok((raw,out)),Err(o)=>Err(oom(o))}
}

fn reserve_saved(n: usize, trace: &mut Trace) -> (r: Result<Vec<u8>,TryError>)
    requires 1<=n<=crate::alloc_guard::MAX_BYTES,old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        final(trace).log_view()==seq![(AllocSite::Saved,n as nat,r.is_ok())],
        match r {Ok(v)=>v@==Seq::new(n as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let result=try_zeroed(AllocSite::Saved,n,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::Saved,n);
    match result {Ok(v)=>Ok(v),Err(o)=>Err(oom(o))}
}

fn reserve_icc_window(trace: &mut Trace) -> (r: Result<Vec<u8>,TryError>)
    requires old(trace).wf(),old(trace).n==0,
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        final(trace).log_view()==seq![(AllocSite::IccWindow,WINDOW_MAX as nat,r.is_ok())],
        match r {Ok(v)=>v@==Seq::new(WINDOW_MAX as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    proof {crate::limits::caps_ok();}
    let result=try_zeroed(AllocSite::IccWindow,WINDOW_MAX,Tracked(trace.log.borrow_mut()));
    trace.record(AllocSite::IccWindow,WINDOW_MAX);
    match result {Ok(v)=>Ok(v),Err(o)=>Err(oom(o))}
}

fn reserve_icc(n: usize, trace: &mut Trace) -> (r: Result<Vec<u8>,TryError>)
    requires n<=MAX_ICC,old(trace).wf(),old(trace).n==1,
        crate::alloc_guard::all_ok(old(trace).log_view()),
    ensures final(trace).wf(),crate::alloc_guard::sites::log_tied(r,final(trace).log_view()),
        final(trace).log_view()==old(trace).log_view().push((AllocSite::Icc,n as nat,r.is_ok())),
        match r {Ok(v)=>v@==Seq::new(n as nat,|_i: int|0u8),Err(TryError::OutOfMemory(_))=>n>0,_=>false},
{
    proof {crate::limits::caps_ok();}
    let result=if n==0 {try_empty(AllocSite::Icc,Tracked(trace.log.borrow_mut()))}
        else {try_zeroed(AllocSite::Icc,n,Tracked(trace.log.borrow_mut()))};
    trace.record(AllocSite::Icc,n);
    match result {Ok(v)=>Ok(v),Err(o)=>Err(oom(o))}
}

} // verus!

'''+s[z:]
s=s.replace('        let doc = crate::callers::frames(b,&pol,&mut ev)?;\n        let mut trace = Trace::new();','        let mut trace = Trace::new();\n        let doc = crate::callers::frames(b,&pol,&mut ev)?;',1)
s=s.replace("    pub fn try_next(&mut self) -> Result<Option<FrameView<'_>>, TryError> {\n        let w", "    pub fn try_next(&mut self) -> Result<Option<FrameView<'_>>, TryError> {\n        let mut trace=Trace::new();\n        let w",1)
s=s.replace('match try_zeroed(AllocSite::Saved, n, Tracked::<&mut AllocLog>::assume_new()) {\n                        Ok(v) => self.saved = Some(v),\n                        Err(o) => return Err(oom(o)),','match reserve_saved(n,&mut trace) {\n                        Ok(v) => self.saved = Some(v),\n                        Err(e) => return Err(e),',1)
s=s.replace('    trace.record(AllocSite::IccWindow, WINDOW_MAX);\n    let mut ring = try_zeroed(AllocSite::IccWindow, WINDOW_MAX, Tracked::<&mut AllocLog>::assume_new()).map_err(oom)?;','    let mut ring=reserve_icc_window(trace)?;',1)
a=s.index('    trace.record(AllocSite::Icc, m);');z=s.index('    let mut src = match SegmentSlices::new(&parts)',a);s=s[:a]+'    let mut prof=reserve_icc(m,trace)?;\n'+s[z:]
s=s.replace("//! M1b: ordinary Rust. The guard calls pass `Tracked::assume_new()` logs, as\n//! GIF's M1 did; M3–M6 replace them with the sealed log and the proofs.","//! M3 guard helpers borrow a real sealed log owned by each call's Trace.\n//! Complete public log/decoder composition remains separate from these helpers.",1)
assert 'assume_new' not in s;p.write_text(s)
