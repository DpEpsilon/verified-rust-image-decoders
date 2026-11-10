use vstd::prelude::*;
use dib_verus::{decode_with_steps, Decoded, Error, Mode};

pub mod ico;

verus! {

// Skeleton: proves the cross-crate contract of the DIB core is usable here.
// M1 keeps the plain ICO implementation in `crate::ico` (outside `verus!`);
// later milestones re-state it inside and replace this probe.
pub fn skeleton(b: &[u8]) -> (r: Result<Decoded, Error>)
    ensures
        dib_verus::dib::decode_mp_eq(r, b@, Mode::IcoEntryDibSlack),
{
    let (r, _steps) = decode_with_steps(b, Mode::IcoEntryDibSlack);
    r
}

} // verus!
