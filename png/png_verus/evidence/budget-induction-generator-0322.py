#!/usr/bin/env python3
"""Generate untrusted small byte certificates and symbolic-index budget proofs."""
from pathlib import Path
import json, hashlib
ROOT=Path(__file__).resolve().parents[1]
source=ROOT.parent/'limits-frozen/VECTORS-CAPS.json'
fixtures=json.loads(source.read_text())
names=['apng-bad/pixel-budget-exactly-2^30-then-zlib','apng-bad/pixel-budget']
data={c:bytes.fromhex(next(x for x in fixtures if x['name']==n)['input_hex']) for c,n in zip([64,65],names)}
assert len(data[64])==79+56*64 and len(data[65])==79+56*65
pairs=[data[65][67+56*k:123+56*k] for k in range(65)]
assert data[64][67:-12]==b''.join(pairs[:64])
assert data[64][-12:]==data[65][-12:]
def seq(b):return 'seq!['+','.join(str(x)+'u8' for x in b)+']'
def arr(b):return '['+','.join(str(x)+'u8' for x in b)+']'
lines=['//! Symbolic-index APNG budget attempt; certificates come from authored frozen bytes.','use vstd::prelude::*;','use crate::{document_model as m,apng_model as a,walk_model as w,critical_model as c,png::Error,alloc_guard::sites::TryError};','verus! {']
lines += ['spec fn header() -> crate::model::Ihdr {crate::model::Ihdr {width:8192,height:2048,depth:8,color_type:6,interlace:0}}',
          'spec fn prefix_bytes(count: u32) -> Seq<u8> {if count==64 {'+seq(data[64][:67])+'} else {'+seq(data[65][:67])+'}}',
          'spec fn pair(k: nat) -> Seq<u8> {']
lines += [f'{"if" if k==0 else "else if"} k=={k} {{{seq(p)}}}' for k,p in enumerate(pairs[:-1])]
lines += ['else {'+seq(pairs[-1])+'}}','spec fn ending() -> Seq<u8> {'+seq(data[64][-12:])+'}']
lines += [r'''
spec fn bytes(count: u32) -> Seq<u8> {
    Seq::new(79+56*(count as nat), |i: int|
        if i<67 {prefix_bytes(count)[i]}
        else if i<67+56*(count as int) {pair(((i-67)/56) as nat)[(i-67)%56]}
        else {ending()[i-67-56*(count as int)]})
}
spec fn control(k: nat) -> a::Control {
    a::Control {sequence:2*k,current:if k==0 {a::Current::None}else{a::Current::Frame(true)},
        frame_count:k,fctl_before_idat:false,pixel_total:k*(8192nat*2048nat)}
}
spec fn middle_control(k: nat) -> a::Control {
    a::Control {sequence:2*k+1,current:a::Current::Frame(false),frame_count:k+1,
        fctl_before_idat:false,pixel_total:(k+1)*(8192nat*2048nat)}
}
spec fn frame() -> a::FrameControl {
    a::FrameControl {width:8192,height:2048,x:0,y:0,delay_num:1,delay_den:10,dispose:0,blend:0}
}
#[verifier::opaque]
spec fn wire_good(b: Seq<u8>,k: nat) -> bool {
    let p=67+56*k;
    (forall|phase: w::Phase| #[trigger] c::read(b,p,phase,false)==Ok(c::Candidate {
        ty:seq![102u8,99u8,84u8,76u8],data:p+8,len:26,next:p+38,crc_ok:true,synthetic:false}))
    && (forall|phase: w::Phase| #[trigger] c::read(b,p+38,phase,false)==Ok(c::Candidate {
        ty:seq![102u8,100u8,65u8,84u8],data:p+46,len:6,next:p+56,crc_ok:true,synthetic:false}))
    && a::fctl(b.subrange((p+8) as int,(p+34) as int),true,false,8192,2048,control(k))
        == (if k<64 {Ok((frame(),middle_control(k)))}else{Err((Error::PixelBudgetExceeded,false))})
    && (k<64 ==> a::fdat(b.subrange((p+46) as int,(p+52) as int),true,false,middle_control(k))==Ok(control(k+1)))
}
spec fn prefix_state(s: m::State,k: nat,count: u32) -> bool {
    s.pos==67+56*k && s.doc.ihdr==header() && s.doc.plte==None
    && s.doc.actl==Some((count,0u32)) && s.doc.frames.len()==k
    && s.seen==set![w::Name::Ihdr,w::Name::Actl]
    && s.phase==(if k==0 {w::Phase::During}else{w::Phase::After})
    && s.control==control(k) && s.events==m::no_events()
}
spec fn middle(s: m::State,k: nat) -> m::State {
    m::State {pos:s.pos+38,phase:w::Phase::After,control:middle_control(k),
        doc:m::Doc {frames:s.doc.frames.push(w::Frame {control_offset:s.pos,width:8192,height:2048,
            x:0,y:0,delay_num:1,delay_den:10,dispose:0,blend:0,source:w::Source::Fdat(Seq::empty())}),..s.doc},..s}
}
spec fn after_pair(s: m::State,k: nat) -> m::State {
    let t=middle(s,k);
    m::State {pos:s.pos+56,control:control(k+1),doc:m::Doc {frames:t.doc.frames.update(k as int,
        w::Frame {source:w::Source::Fdat(seq![w::Range {offset:s.pos+50,len:2}]),..t.doc.frames[k as int]}),..t.doc},..t}
}
#[verifier::spinoff_prover]
proof fn tags()
    ensures w::classify(seq![102u8,99u8,84u8,76u8])==w::Name::Fctl,
        w::classify(seq![102u8,100u8,65u8,84u8])==w::Name::Fdat,
        w::classify(seq![73u8,69u8,78u8,68u8])==w::Name::Iend,
{
    assert(w::classify(seq![102u8,99u8,84u8,76u8])==w::Name::Fctl) by (compute);
    assert(w::classify(seq![102u8,100u8,65u8,84u8])==w::Name::Fdat) by (compute);
    assert(w::classify(seq![73u8,69u8,78u8,68u8])==w::Name::Iend) by (compute);
}
#[verifier::spinoff_prover]
proof fn fctl_step(b: Seq<u8>,s: m::State,k: nat,count: u32)
    requires k<64, count==64 || count==65, b.len()==79+56*(count as nat),
        prefix_state(s,k,count),wire_good(b,k),
    ensures m::step(b,m::strict(),true,false,s)==m::Step::Next(middle(s,k)),
{
    hide(c::read);hide(a::fctl);hide(a::fdat);hide(w::classify);
    tags();reveal(wire_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn fdat_step(b: Seq<u8>,s: m::State,k: nat,count: u32)
    requires k<64, count==64 || count==65,b.len()==79+56*(count as nat),
        prefix_state(s,k,count),wire_good(b,k),
    ensures m::step(b,m::strict(),true,false,middle(s,k))==m::Step::Next(after_pair(s,k)),
        prefix_state(after_pair(s,k),k+1,count),
{
    hide(c::read);hide(a::fctl);hide(a::fdat);hide(w::classify);
    tags();reveal(wire_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
    let t=middle(s,k);
    assert(t.doc.frames.len()==k+1);
    assert(t.doc.frames[k as int].source==w::Source::Fdat(Seq::empty()));
    assert(t.doc.frames[k as int].source==w::Source::Fdat(Seq::empty()));
}
spec fn budget_outcome(out: m::Outcome,count: u32) -> bool {
    if count==65 {out==m::Outcome::Error(Error::PixelBudgetExceeded,m::no_events())}
    else {match out {m::Outcome::Done(doc,ev)=>doc.ihdr==header() && ev==m::no_events(),_=>false}}
}
#[verifier::opaque]
spec fn end_good(b: Seq<u8>,count: u32) -> bool {
    if count==65 {wire_good(b,64)}else{
        c::read(b,3651,w::Phase::After,false)==Ok(c::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:3659,len:0,next:3663,crc_ok:true,synthetic:false})}
}
#[verifier::spinoff_prover]
proof fn terminal(b: Seq<u8>,s: m::State,count: u32)
    requires count==64 || count==65,b.len()==79+56*(count as nat),prefix_state(s,64,count),end_good(b,count),
    ensures budget_outcome(m::drive(b,m::strict(),true,false,s),count),
{
    hide(m::drive);hide(c::read);hide(a::fctl);hide(w::classify);
    tags();reveal(end_good);reveal(wire_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
    if count==65 {
        assert(m::step(b,m::strict(),true,false,s)==m::Step::Error(Error::PixelBudgetExceeded,m::no_events()));
    } else {
        assert(a::finish(control(64),Some(64u32))==Ok(())) by (compute);
        assert(match m::step(b,m::strict(),true,false,s) {m::Step::Done(doc,ev)=>doc.ihdr==header() && ev==m::no_events(),_=>false});
    }
    m::drive_step(b,m::strict(),true,false,s);
}
#[verifier::spinoff_prover]
proof fn finish(b: Seq<u8>,s: m::State,k: nat,count: u32)
    requires count==64 || count==65,k<=64,b.len()==79+56*(count as nat),
        prefix_state(s,k,count),end_good(b,count),
        forall|j: nat| j<64 ==> #[trigger] wire_good(b,j),
    ensures budget_outcome(m::drive(b,m::strict(),true,false,s),count),
    decreases 64-k,
{
    hide(m::drive);
    if k==64 {terminal(b,s,count);}else{
        fctl_step(b,s,k,count);fdat_step(b,s,k,count);
        finish(b,after_pair(s,k),k+1,count);
        m::drive_step(b,m::strict(),true,false,middle(s,k));
        m::drive_step(b,m::strict(),true,false,s);
    }
}
#[verifier::opaque]
spec fn start_good(b: Seq<u8>,count: u32) -> bool {
    crate::model::mh(b)==Ok((header(),33nat))
    && c::read(b,33,w::Phase::Before,false)==Ok(c::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false})
    && a::actl(b.subrange(41,49))==Ok((count,0u32))
    && c::read(b,53,w::Phase::Before,false)==Ok(c::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61,len:2,next:67,crc_ok:true,synthetic:false})
}
spec fn start_one(count: u32) -> m::State {
    let s=m::initial(header(),33,m::no_events());
    m::State {pos:53,seen:s.seen.insert(w::Name::Actl),doc:m::Doc {actl:Some((count,0u32)),..s.doc},..s}
}
spec fn start_two(count: u32) -> m::State {
    let s=start_one(count);
    m::State {pos:67,phase:w::Phase::During,doc:m::Doc {idat:seq![w::Range {offset:61,len:2}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn start(b: Seq<u8>,count: u32)
    requires count==64 || count==65,b.len()==79+56*(count as nat),start_good(b,count),
        end_good(b,count),forall|j: nat| j<64 ==> #[trigger] wire_good(b,j),
    ensures budget_outcome(m::MWF_p(b,m::strict()),count),
{
    hide(m::drive);hide(c::read);hide(a::actl);hide(crate::model::mh);hide(m::initial_info);
    reveal(start_good);reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(m::step(b,m::strict(),true,false,m::initial(header(),33,m::no_events()))==m::Step::Next(start_one(count)));
    assert(m::step(b,m::strict(),true,false,start_one(count))==m::Step::Next(start_two(count)));
    assert(prefix_state(start_two(count),0,count));
    finish(b,start_two(count),0,count);
    m::drive_step(b,m::strict(),true,false,start_one(count));
    m::drive_step(b,m::strict(),true,false,m::initial(header(),33,m::no_events()));
}
''']
# Each certificate computes only lazy source slices and short authored chunks;
# no concrete frame-state sequence is emitted.
for k in range(65):
    p=67+56*k
    lines += ['#[verifier::spinoff_prover]',f'proof fn wire_{k}(count: u32)',f'    requires (count==64 || count==65), {k}nat<count as nat,',f'    ensures wire_good(bytes(count),{k}nat),','{','    reveal(wire_good);']
    for count in ([64,65] if k<64 else [65]):
        lines += [f'    if count=={count}u32 {{']
        for off,tag,length,nxt in [(p,[102,99,84,76],26,p+38),(p+38,[102,100,65,84],6,p+56)]:
            expr=f'c::read(bytes({count}u32),{off}nat,phase,false)==Ok(c::Candidate {{ty:{seq(tag)},data:{off+8}nat,len:{length}nat,next:{nxt}nat,crc_ok:true,synthetic:false}})'
            lines += [f'        assert forall|phase: w::Phase| {expr} by {{',f'            assert({expr}) by (compute);','        }']
        value=f'Ok((frame(),middle_control({k}nat)))' if k<64 else 'Err((Error::PixelBudgetExceeded,false))'
        expr=f'a::fctl(bytes({count}u32).subrange({p+8},{p+34}),true,false,8192,2048,control({k}nat))=={value}'
        lines += [f'        assert({expr}) by (compute);']
        if k<64:
            expr=f'a::fdat(bytes({count}u32).subrange({p+46},{p+52}),true,false,middle_control({k}nat))==Ok(control({k+1}nat))'
            lines += [f'        assert({expr}) by (compute);']
        lines += ['    }']
    lines += ['}']
lines += ['#[verifier::spinoff_prover]','proof fn all_wire(count: u32)','    requires count==64 || count==65,','    ensures forall|k: nat| k<count as nat ==> #[trigger] wire_good(bytes(count),k),','{','    // wire_good is already opaque; no explicit hide fuel is needed.','    assert forall|k: nat| k<count as nat implies #[trigger] wire_good(bytes(count),k) by {']
for k in range(65):lines += [f'        {"if" if k==0 else "else if"} k=={k} {{wire_{k}(count);}}']
lines += ['    }','}']
lines += ['#[verifier::spinoff_prover]','proof fn bookends(count: u32)','    requires count==64 || count==65,','    ensures start_good(bytes(count),count),end_good(bytes(count),count),','{','    reveal(start_good);reveal(end_good);']
for count in [64,65]:
    lines += [f'    if count=={count} {{',f'        assert(start_good(bytes({count}u32),{count}u32)) by (compute);']
    if count==64:lines += ['        assert(end_good(bytes(64u32),64u32)) by (compute);']
    else:lines += ['        wire_64(65u32);']
    lines += ['    }']
lines += ['}']
lines += [r'''
#[verifier::spinoff_prover]
proof fn full_model(count: u32)
    requires count==64 || count==65,
    ensures budget_outcome(m::MWF_p(bytes(count),m::strict()),count),
{
    hide(bytes);hide(m::MWF_p);
    byte_length(count);all_wire(count);bookends(count);start(bytes(count),count);
}
#[verifier::spinoff_prover]
proof fn byte_length(count: u32)
    ensures bytes(count).len()==79+56*(count as nat),
{}
#[verifier::spinoff_prover]
proof fn short_lengths(count: u32)
    requires count==64 || count==65,
    ensures prefix_bytes(count).len()==67,ending().len()==12,
{}
#[verifier::spinoff_prover]
proof fn pair_length(k: nat)
    ensures pair(k).len()==56,
{}
#[verifier::spinoff_prover]
proof fn byte_slices(count: u32,k: nat)
    requires count==64 || count==65,k<count as nat,
    ensures bytes(count).len()==79+56*(count as nat),
        bytes(count).subrange(0,67)==prefix_bytes(count),
        bytes(count).subrange((67+56*k) as int,(123+56*k) as int)==pair(k),
        bytes(count).subrange((67+56*(count as nat)) as int,bytes(count).len() as int)==ending(),
{
    hide(pair);hide(prefix_bytes);hide(ending);
    pair_length(k);short_lengths(count);
    assert(prefix_bytes(count).len()==67);
    assert(ending().len()==12);
    assert(bytes(count).subrange(0,67) =~= prefix_bytes(count));
    assert(bytes(count).subrange((67+56*k) as int,(123+56*k) as int) =~= pair(k)) by {
        assert forall|j: int| 0<=j<56 implies bytes(count)[67+56*(k as int)+j]==pair(k)[j] by {
            assert((56*(k as int)+j)/56==k as int);
            assert((56*(k as int)+j)%56==j);
        }
    }
    assert(bytes(count).subrange((67+56*(count as nat)) as int,bytes(count).len() as int) =~= ending());
}
#[verifier::spinoff_prover]
proof fn join(s: Seq<u8>,lo: int,hi: int)
    requires 0<=lo<=hi<=s.len(),
    ensures s.subrange(0,lo)+s.subrange(lo,hi)==s.subrange(0,hi),
{
    assert(s.subrange(0,lo)+s.subrange(lo,hi) =~= s.subrange(0,hi));
}
fn append(out: &mut [u8],pos: usize,chunk: &[u8])
    requires pos<=old(out).len(),chunk.len()<=old(out).len()-pos,
    ensures final(out).len()==old(out).len(),
        final(out)@.subrange(0,(pos+chunk.len()) as int)==old(out)@.subrange(0,pos as int)+chunk@,
{
    let ghost before=out@;let n=out.len();let mut j=0usize;
    while j<chunk.len()
        invariant out.len()==n,n==before.len(),before==old(out)@,pos<=n,chunk.len()<=n-pos,j<=chunk.len(),
            forall|i: int| 0<=i<n ==> out@[i]==(if pos as int<=i<pos as int+j as int {chunk@[(i-pos as int) as int]}else{before[i]}),
        decreases chunk.len()-j,
    {
        out[pos+j]=chunk[j];j+=1;
    }
    assert(out@.subrange(0,(pos+chunk.len()) as int) =~= before.subrange(0,pos as int)+chunk@);
}
''']
lines += ['fn literal_pair(k: usize) -> (out: [u8;56])','    requires k<65,','    ensures out@==pair(k as nat),','{','    match k {']
lines += [f'        {k}=>{arr(p)},' for k,p in enumerate(pairs[:-1])]
lines += ['        _=>'+arr(pairs[-1])+',','    }','}']
lines += ['fn literal_prefix(count: u32) -> (out: [u8;67])','    requires count==64 || count==65,','    ensures out@==prefix_bytes(count),','{',f'    if count==64 {{{arr(data[64][:67])}}} else {{{arr(data[65][:67])}}}','}']
lines += [r'''
fn build(out: &mut [u8],count: u32)
    requires count==64 || count==65,old(out).len()==79+56*(count as nat),
    ensures final(out).len()==old(out).len(),final(out)@==bytes(count),
{
    hide(bytes);hide(pair);hide(prefix_bytes);hide(ending);
    let n=out.len();let p=literal_prefix(count);
    proof {byte_slices(count,0);}
    append(out,0,&p);
    let mut k=0usize;let mut pos=67usize;
    while k<(count as usize)
        invariant count==64 || count==65,out.len()==n,n==79+56*(count as nat),
            k<=count as usize,pos==67+56*k,pos<=n,
            bytes(count).len()==n,
            out@.subrange(0,pos as int)==bytes(count).subrange(0,pos as int),
        decreases count as usize-k,
    {
        proof {byte_slices(count,k as nat);}
        let p=literal_pair(k);
        append(out,pos,&p);
        proof {join(bytes(count),pos as int,(pos+56) as int);}
        pos+=56;k+=1;
    }
''']
lines += ['    let end='+arr(data[64][-12:])+';','    proof {byte_slices(count,0);}','    append(out,pos,&end);','    proof {join(bytes(count),pos as int,n as int);assert(out@ =~= bytes(count));}','}']
lines += [r'''
fn budget_overflow_witness() {
    hide(bytes);hide(m::MWF_p);
    let mut b=[0u8;3719];build(&mut b,65u32);
    proof {full_model(65u32);}
    let result=crate::png::Frames::try_new(&b);
    assert(matches!(result,Err(TryError::Format(Error::PixelBudgetExceeded))));
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile.
fn budget_exact_projection_witness() {
    hide(bytes);hide(m::MWF_p);
    let mut b=[0u8;3663];build(&mut b,64u32);
    proof {full_model(64u32);}
    let mut ev=crate::png::Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::frames(&b,&crate::policy::STRICT,&mut ev);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==header());assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}},
        Err(_)=>{assert(false);},
    }
    let _public_result=crate::png::Frames::try_new(&b);
}
} // verus!
''']
(ROOT/'src/witness_m3_budget_inductive.rs').write_text('\n'.join(line.rstrip() for line in lines)+'\n')
records=[dict(name=n,source=str(source),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),input_sha256=hashlib.sha256(data[c]).hexdigest(),bytes=len(data[c]),frames=c) for c,n in zip([64,65],names)]
(ROOT/'runs/m3-budget-inductive-inputs.json').write_text(json.dumps(dict(records=records,generic_steps=['fctl_step','fdat_step'],induction='finish',certificates='wire_0 through wire_64; source slices are checked against independent framing/CRC/control models',runtime_literal_builder='build, append, literal_pair and literal_prefix; short chunks from frozen authored bytes',status='UNVERIFIED until a complete accepted full-crate run'),indent=2)+'\n')
print('Generated one symbolic fcTL step, one fdAT step, one induction and small per-frame certificates; no 64-state unrolling.')
