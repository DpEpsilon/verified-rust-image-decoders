//! A counterexample to the frozen representation lemma's unrestricted literal
//! domain under checked-literal policies. This does not supply the missing ST
//! model or establish the T-LZW representation obligation.
use vstd::prelude::*;

verus! {

/// m=9 is the smallest supported code size with a literal outside the byte
/// domain; c=256 is the smallest such literal. Literal operations inspect no
/// reservation byte, so the result holds for every correctly sized table.
/// No Seq<u8> dictionary can satisfy the frozen first-byte equality at c=256.
pub proof fn lemma_literal_domain_counterexample(t: Seq<u8>, model_table: Seq<Seq<u8>>)
    requires
        crate::lzw::table_len_ok(t),
        model_table.len() == 4096,
        model_table[256].len() == 1,
    ensures
        256 < crate::chain::ipow2(9),
        crate::lzw::code_suffix(t, 9, 256) == 256,
        crate::lzw::code_first(t, 9, 256, 4096) == 256,
        crate::lzw::chain(t, 9, 256, 4096).len() == 1,
        (crate::lzw::chain(t, 9, 256, 4096)[0] as int) < 256,
        (model_table[256][0] as int) != crate::lzw::code_first(t, 9, 256, 4096),
{
    assert(crate::chain::ipow2(9) == 512) by(compute);
}

} // verus!
