//! Short CRC segments compose into the independent P3 bitwise recurrence.
use vstd::prelude::*;
use crate::model as m;
verus! {
pub(crate) open spec fn fold(register: u32, bytes: Seq<u8>) -> u32
    decreases bytes.len(),
{
    if bytes.len()==0 {register}
    else {m::crc_bits(fold(register,bytes.drop_last()) ^ bytes.last() as u32,8)}
}
pub(crate) proof fn suffix(prefix: Seq<u8>, bytes: Seq<u8>)
    ensures fold(m::crc_register(prefix),bytes)==m::crc_register(prefix+bytes),
    decreases bytes.len(),
{
    if bytes.len()==0 {assert(prefix+bytes =~= prefix);}
    else {
        suffix(prefix,bytes.drop_last());
        assert((prefix+bytes).drop_last() =~= prefix+bytes.drop_last());
        assert((prefix+bytes).last()==bytes.last());
    }
}
} // verus!
