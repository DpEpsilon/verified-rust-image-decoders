use vstd::prelude::*;
use dib_verus::{decode_with_steps, Decoded, Error, Mode};

verus! {

// Skeleton: proves the cross-crate contract of the DIB core is usable here.
pub fn skeleton(b: &[u8]) -> (r: Result<Decoded, Error>)
    ensures
        dib_verus::dib::decode_mp_eq(r, b@, Mode::IcoEntryDibSlack),
{
    let (r, _steps) = decode_with_steps(b, Mode::IcoEntryDibSlack);
    r
}

} // verus!
