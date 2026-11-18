//! Original-segment replay without unfolding a concrete thousand-part prefix.
use crate::{model as m, segments::parts_prefix, witness_policy::check_policy_parts, Status};
use vstd::prelude::*;
verus! {
pub(crate) open spec fn cuts_ok(cuts: Seq<usize>, a: nat, b: nat, size: nat) -> bool
    decreases b-a,
{
    if a >= b {true} else if b == a+1 {cuts[a as int] <= cuts[(a+1) as int] <= size}
    else {let mid=(a+(b-a)/2) as nat; cuts_ok(cuts,a,mid,size) && cuts_ok(cuts,mid,b,size)}
}
pub(crate) proof fn cuts_quant(cuts: Seq<usize>, a: nat, b: nat, size: nat)
    requires a <= b < cuts.len(), cuts_ok(cuts,a,b,size),
    ensures forall|j:int| a <= j < b ==> #[trigger] cuts[j] <= cuts[j+1] <= size,
    decreases b-a,
{
    if a < b && b != a+1 {
        let mid=(a+(b-a)/2) as nat;
        cuts_quant(cuts,a,mid,size); cuts_quant(cuts,mid,b,size);
    }
}
proof fn prefix_equal(a: Seq<&[u8]>, b: Seq<&[u8]>, k: nat)
    requires k <= a.len(), k <= b.len(),
        forall|j:int| 0 <= j < k ==> a[j]@ == b[j]@,
    ensures parts_prefix(a,k) == parts_prefix(b,k),
    decreases k,
{
    if k > 0 {prefix_equal(a,b,(k-1) as nat);}
}
pub(crate) fn replay_layout<'a>(wire: &'a [u8], cuts: &[usize], parts: &mut [&'a [u8]], output: &mut [u8], raw: Ghost<Seq<u8>>)
    requires cuts.len() == old(parts).len()+1, cuts[0] == 0, cuts@.last() == wire.len(),
        forall|j:int| 0 <= j < old(parts).len() ==> #[trigger] cuts@[j] <= cuts@[j+1] <= wire.len(),
        wire.len() <= crate::limits::SEGMENT_BYTES_MAX,
        m::Inflate(wire@,old(output).len() as nat,true,false).status == m::InflateStatus::Done,
        m::Inflate(wire@,old(output).len() as nat,true,false).out == raw@,
        raw@.len() <= old(output).len(),
{
    hide(m::Inflate);
    let mut i=0usize;
    while i<parts.len()
        invariant i <= parts.len(), cuts.len() == parts.len()+1,
            cuts[0] == 0, cuts@.last() == wire.len(),
            forall|j:int| 0 <= j < parts.len() ==> #[trigger] cuts@[j] <= cuts@[j+1] <= wire.len(),
            parts_prefix(parts@,i as nat) == wire@.take(cuts[i as int] as int),
        decreases parts.len()-i,
    {
        let start=cuts[i]; let end=cuts[i+1];
        let ghost before=parts@;
        parts[i]=&wire[start..end];
        proof {
            assert forall|j:int| 0 <= j < i ==> before[j]@ == parts@[j]@ by {}
            prefix_equal(before,parts@,i as nat);
            assert(wire@.take(start as int)+wire@.subrange(start as int,end as int) =~= wire@.take(end as int));
        }
        i+=1;
    }
    proof {assert(parts_prefix(parts@,parts@.len()) =~= wire@);}
    check_policy_parts(parts,output,true,false,Status::Done,raw);
}
} // verus!
