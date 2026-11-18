//! Symbolic all-zero stored prefixes followed by independently authored matches.
use crate::{model as m, InflateOptions, SegmentSlices, Status, Window};
use vstd::prelude::*;
verus! {
pub(crate) open spec fn zeros(n:nat) -> Seq<u8> {Seq::new(n,|i:int|0u8)}
proof fn zero_copy(n:nat,d:nat,c:nat)
    requires 1 <= d <= n,
    ensures m::copy_distance(zeros(n),d,c) == zeros(n+c),
    decreases c,
{
    if c > 0 {zero_copy(n,d,(c-1) as nat); assert(zeros((n+c-1) as nat).push(0) =~= zeros(n+c));}
}
pub(crate) proof fn zero_sums(n:nat)
    ensures m::sum_bytes(zeros(n)) == 0, m::sum_prefixes(zeros(n)) == n,
    decreases n,
{
    if n > 0 {
        zero_sums((n-1) as nat); assert(zeros((n-1) as nat).push(0) =~= zeros(n));
        m::lemma_adler_sums_push(zeros((n-1) as nat),0);
    } else {assert(zeros(0) =~= Seq::<u8>::empty());}
}
pub(crate) proof fn zero_blocks(z:Seq<u8>,n:nat,window:nat,ds:nat,de:nat,width:nat)
    requires 1 <= n <= u16::MAX, 1 <= window <= crate::limits::WINDOW_MAX,
        7+n <= z.len(), 8*(7+n)+22+width <= 8*z.len(),
        z[3] as nat+256*z[4] as nat == n, z[5] as nat+256*z[6] as nat+n == u16::MAX,
        z.subrange(7,(7+n) as int) == zeros(n),
        m::field_value(z,16,1) == 0, m::field_value(z,17,2) == 0,
        m::field_value(z,8*(7+n),1) == 1, m::field_value(z,8*(7+n)+1,2) == 1,
        ds < crate::limits::NDIST_MAX, m::distance_extra(ds) == width, m::distance_base(ds)+de == n,
        m::field_value(z,8*(7+n)+15,width) == de,
        m::MDec(m::fixed_lit_lengths(),z,8*(7+n)+3) == (m::Decoded::Symbol{symbol:257,position:8*(7+n)+10}),
        m::MDec(m::fixed_dist_lengths(),z,8*(7+n)+10) == (m::Decoded::Symbol{symbol:ds,position:8*(7+n)+15}),
        m::MDec(m::fixed_lit_lengths(),z,8*(7+n)+15+width) == (m::Decoded::Symbol{symbol:256,position:8*(7+n)+22+width}),
    ensures m::MBlk(z,16,Seq::empty(),n+3,window,true) == if n <= window {
        m::StoredEffect::End{out:zeros(n+3),position:8*(7+n)+22+width}
    } else {m::StoredEffect::Error{error:m::InflateFault::DistanceTooFar,out:zeros(n),position:8*(7+n)+15+width}},
{
    hide(m::MDec); hide(m::MCompressedStep); hide(m::MCompressed);
    assert(crate::limits::LEN_MAX == 258) by (compute);
    m::lemma_stored_payload(z,7,Seq::empty(),n+3,n);
    assert(Seq::<u8>::empty()+z.subrange(7,(7+n) as int) =~= zeros(n));
    assert(m::MStoredPayload(z,7,Seq::empty(),n+3,n) == (m::StoredEffect::End{out:zeros(n),position:8*(7+n)}));
    assert(m::MStoredBlock(z,19,Seq::empty(),n+3) == (m::StoredEffect::End{out:zeros(n),position:8*(7+n)}));
    assert(m::MOneBlock(z,16,Seq::empty(),n+3,window,true) == (false,m::StoredEffect::End{out:zeros(n),position:8*(7+n)}));
    if n <= window {
        zero_copy(n,n,3);
        assert(m::MCopy(zeros(n),n+3,window,n,3) == (m::OutputEffect::Next{out:zeros(n+3)}));
        assert(m::MCompressedStep(z,8*(7+n)+3,zeros(n),n+3,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next{out:zeros(n+3),position:8*(7+n)+15+width})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressedStep(z,8*(7+n)+15+width,zeros(n+3),n+3,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::End{out:zeros(n+3),position:8*(7+n)+22+width})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressed(z,8*(7+n)+3,zeros(n),n+3,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End{out:zeros(n+3),position:8*(7+n)+22+width})) by {reveal_with_fuel(m::MCompressed,3);}
    } else {
        assert(m::MCompressedStep(z,8*(7+n)+3,zeros(n),n+3,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Error{error:m::InflateFault::DistanceTooFar,out:zeros(n),position:8*(7+n)+15+width})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressed(z,8*(7+n)+3,zeros(n),n+3,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::Error{error:m::InflateFault::DistanceTooFar,out:zeros(n),position:8*(7+n)+15+width})) by {reveal(m::MCompressed);}
    }
    let effect = if n <= window {m::StoredEffect::End{out:zeros(n+3),position:8*(7+n)+22+width}}
        else {m::StoredEffect::Error{error:m::InflateFault::DistanceTooFar,out:zeros(n),position:8*(7+n)+15+width}};
    assert(m::MOneBlock(z,8*(7+n),zeros(n),n+3,window,true) == (true,effect));
    assert(m::MBlk(z,8*(7+n),zeros(n),n+3,window,true) == effect);
    assert(m::MBlk(z,16,Seq::empty(),n+3,window,true) == effect);
}

pub(crate) fn check_options(wire:&[u8],output:&mut[u8],enforce:bool,expected:Status,raw:Ghost<Seq<u8>>)
    requires wire.len() <= crate::limits::SEGMENT_BYTES_MAX, raw@.len() <= old(output).len(),
        crate::status_eq(expected,m::Inflate(wire@,old(output).len() as nat,enforce,false).status),
        raw@ == m::Inflate(wire@,old(output).len() as nat,enforce,false).out,
{
    hide(m::Inflate);
    let options=InflateOptions{window:if enforce {Window::Enforce} else {Window::Ignore},..InflateOptions::STRICT};
    let limit=output.len() as u64;
    let parts:[&[u8];1]=[wire];
    proof {reveal_with_fuel(crate::segments::parts_prefix,2);assert(crate::segments::parts_prefix(parts@,1) =~= wire@);}
    let mut source=match SegmentSlices::new(&parts) {Some(s)=>s,None=>{proof {assert(false);}return;}};
    let result=crate::inflate_zlib_into(&mut source,output,options);
    proof {assert(result.status == expected);assert(result.produced == raw@.len());assert(output@.take(result.produced as int) == raw@);}
    let mut source=match SegmentSlices::new(&parts) {Some(s)=>s,None=>{proof {assert(false);}return;}};
    let mut ring=[0xccu8;crate::limits::WINDOW_MAX];
    proof {crate::limits::caps_ok();assert(crate::limits::WINDOW_MAX == 32768) by (compute);assert(ring@.len() == crate::limits::WINDOW_MAX);}
    let measured=crate::inflate_zlib_measure(&mut source,&mut ring,limit,options);
    proof {assert(measured.status == expected);assert(measured.produced == raw@.len());assert(crate::ring_repr(ring@,raw@));}
}
} // verus!
