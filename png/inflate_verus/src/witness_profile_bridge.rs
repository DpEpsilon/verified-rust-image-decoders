//! Authored code-length fields with the declared HCLEN, including omitted zeros.
use crate::model as m;
use vstd::prelude::*;
verus! {
pub(crate) open spec fn fields(z: Seq<u8>, p: nat, code: Seq<u8>, i: nat, ncode: nat) -> Seq<u8>
    decreases ncode-i,
{
    if i >= ncode {code}
    else {fields(z,p+3,code.update(m::code_length_order()[i as int],m::field_value(z,p,3) as u8),i+1,ncode)}
}
pub(crate) proof fn bridge(z: Seq<u8>, p: nat, code: Seq<u8>, i: nat, ncode: nat, nlen: nat, ndist: nat)
    requires i <= ncode, 4 <= ncode <= 19, code.len() == 19,
        forall|s:int| 0 <= s < code.len() ==> code[s] <= 7,
        p+3*(ncode-i) <= 8*z.len(), m::dynamic_sizes_ok(nlen,ndist),
        m::admissible(fields(z,p,code,i,ncode),m::CodeKind::CodeLen,true),
    ensures m::MDynamicCL(z,p,code,i,ncode,nlen,ndist,true) ==
        m::MDynamicLengths(z,(p+3*(ncode-i)) as nat,fields(z,p,code,i,ncode),Seq::empty(),nlen,ndist,true),
    decreases ncode-i,
{
    assert(crate::limits::NCL == 19) by (compute);
    if i < ncode {
        m::lemma_field_bound(z,p,3);
        assert(m::pow2(3) == 8) by (compute);
        bridge(z,p+3,code.update(m::code_length_order()[i as int],m::field_value(z,p,3) as u8),i+1,ncode,nlen,ndist);
    }
}
} // verus!
