//! Stored/fixed stream composition reverified against public M2 models.
use inflate_verus::model as m;
use vstd::prelude::*;
verus! {
pub(crate) proof fn stored_payload(z: Seq<u8>, raw: Seq<u8>)
    requires raw.len() <= u16::MAX, 7+raw.len() <= z.len(),
        z.subrange(7,(7+raw.len()) as int) == raw,
    ensures m::MStoredPayload(z,7,Seq::empty(),raw.len(),raw.len()) ==
        (m::StoredEffect::End { out: raw, position: 8*(7+raw.len()) }),
{
    hide(m::MStoredPayload);
    m::lemma_stored_payload(z,7,Seq::empty(),raw.len(),raw.len());
    assert(Seq::<u8>::empty()+raw =~= raw);
}

pub(crate) proof fn stored_stream(z: Seq<u8>, raw: Seq<u8>, n: nat, cmf: nat, flg: nat, checksum: nat)
    requires raw.len() == n, n <= u16::MAX, z.len() == n+11,
        z.subrange(7,(7+n) as int) == raw,
        z[3] as nat + 256*z[4] as nat == n, z[5] as nat + 256*z[6] as nat == u16::MAX-n,
        m::field_value(z,16,1) == 1, m::field_value(z,17,2) == 0,
        m::be_value(z,0,1) == cmf, m::be_value(z,1,1) == flg,
        cmf == 120, flg < 256, (cmf*256+flg)%31 == 0, (flg/32)%2 == 0,
        m::be_value(z,(7+n) as nat,4) == checksum, m::Adler(raw) == checksum,
    ensures m::Inflate(z,n,true,false).status == m::InflateStatus::Done,
        m::Inflate(z,n,true,false).out == raw, m::Inflate(z,n,true,false).r2 == false,
{
    hide(m::sum_bytes); hide(m::sum_prefixes); hide(m::MStoredPayload);
    assert(inflate_verus::limits::WINDOW_MAX == 32768) by (compute);
    assert(m::pow2(15) == 32768) by (compute);
    stored_payload(z,raw);
    assert(m::MStoredBlock(z,19,Seq::empty(),n) == (m::StoredEffect::End {out:raw,position:8*(7+n)}));
    assert(m::MBlk(z,16,Seq::empty(),n,32768,true) == (m::StoredEffect::End {out:raw,position:8*(7+n)}));
    assert(m::MFinish(z,8*(7+n),raw,n,false) == (m::StoredEffect::End {out:raw,position:8*z.len()}));
}
pub(crate) proof fn fixed_stream(z: Seq<u8>, raw: Seq<u8>, ts: Seq<crate::m4_trace_witness_model::Step>, n: nat, flg: nat, checksum: nat)
    requires raw.len() == n, z.len() >= 2, ts.len() > 0,
        ts[0].pos == 19, ts[0].before == 0,
        crate::m4_trace_witness_model::trace(z,raw,ts,0,ts.len()),
        m::field_value(z,16,1) == 1, m::field_value(z,17,2) == 1,
        z[0] == 120, z[1] == flg,
        flg < 256, (120*256+flg)%31 == 0, (flg/32)%2 == 0,
        (ts.last().next+7)/8+4 == z.len(),
        m::be_value(z,((ts.last().next+7)/8) as nat,4) == checksum, m::Adler(raw) == checksum,
    ensures m::Inflate(z,n,true,false).status == m::InflateStatus::Done,
        m::Inflate(z,n,true,false).out == raw, m::Inflate(z,n,true,false).r2 == false,
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressed); hide(m::MCompressedStep);
    hide(m::sum_bytes); hide(m::sum_prefixes);
    assert(inflate_verus::limits::WINDOW_MAX == 32768) by (compute);
    assert(m::pow2(15) == 32768) by (compute);
    crate::m4_trace_witness_model::effects(z,raw,ts,0,ts.len());
    crate::m4_trace_witness_model::expand(z,raw,ts,0);
    assert(raw.take(0) =~= Seq::<u8>::empty());
    assert(m::MBlk(z,16,Seq::empty(),n,32768,true) == (m::StoredEffect::End {out:raw,position:ts.last().next}));
    assert(m::MFinish(z,ts.last().next,raw,n,false) == (m::StoredEffect::End {out:raw,position:8*z.len()}));
}
} // verus!
