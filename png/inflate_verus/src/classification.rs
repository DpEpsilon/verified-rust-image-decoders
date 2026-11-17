//! Executable length classification shared by the real Huffman constructor.
//! Canonical words remain defined independently in model.rs.
use crate::limits::*;
#[cfg(verus_only)]
use crate::model as m;
use vstd::prelude::*;

verus! {

#[derive(Clone, Copy)]
pub enum Kind { CodeLen, LitLen, Dist }

pub open spec fn model_kind(kind: Kind) -> m::CodeKind {
    match kind {
        Kind::CodeLen => m::CodeKind::CodeLen,
        Kind::LitLen => m::CodeKind::LitLen,
        Kind::Dist => m::CodeKind::Dist,
    }
}

pub open spec fn admissible_numbers(kind: Kind, kraft: nat, nonzero: nat, ones: nat, single_accept: bool) -> bool {
    kraft <= CODE_SPACE && (kraft == CODE_SPACE || match kind {
        Kind::CodeLen => false,
        Kind::LitLen => nonzero == 1 && ones == 1 && single_accept,
        Kind::Dist => nonzero == 0 || (nonzero == 1 && ones == 1),
    })
}

pub fn is_admissible(kind: Kind, kraft: u32, nonzero: usize, ones: u16, single_accept: bool) -> (r: bool)
    ensures r == admissible_numbers(kind, kraft as nat, nonzero as nat, ones as nat, single_accept),
{
    kraft <= CODE_SPACE && (kraft == CODE_SPACE || match kind {
        Kind::CodeLen => false,
        Kind::LitLen => nonzero == 1 && ones == 1 && single_accept,
        Kind::Dist => nonzero == 0 || (nonzero == 1 && ones == 1),
    })
}

pub proof fn lemma_admissible_from_counts(lengths: Seq<u8>, kind: Kind, single_accept: bool)
    requires m::lengths_ok(lengths),
    ensures m::admissible(lengths, model_kind(kind), single_accept)
        == admissible_numbers(kind, m::Kraft(lengths),
            (lengths.len() - m::count_prefix(lengths, 0, lengths.len())) as nat,
            m::bl_count(lengths, 1), single_accept),
{
    m::lemma_count_bound(lengths, 0, lengths.len());
    match kind { Kind::CodeLen => {}, Kind::LitLen => {}, Kind::Dist => {} }
}

pub proof fn lemma_weight_step(k: int, s: int, weight: int, term: int, cap: int)
    requires 0 <= s < cap, 0 <= weight, k <= s * weight, term <= weight,
    ensures k + term <= (s + 1) * weight, (s + 1) * weight <= cap * weight,
{
    assert(k + term <= (s + 1) * weight) by (nonlinear_arith)
        requires k <= s * weight, term <= weight;
    assert((s + 1) * weight <= cap * weight) by (nonlinear_arith)
        requires s < cap, 0 <= weight;
}

/// Exact counts/Kraft even on oversubscribed vectors. Invalid lengths fail;
/// admissibility is a separate operation, and no table lookup occurs here.
/// Both concrete loops update an erased Ghost<nat> work counter. An invalid
/// length returns its index, so an external test wrapper can retain diagnostics.
pub fn count_lengths(lengths: &[u8], counts: &mut [u16; MAX_CODE_BITS + 1])
    -> (r: (Result<(u32, usize), usize>, Ghost<nat>))
    requires lengths.len() <= FIXED_LIT_COUNT,
    ensures
        r.1@ <= MAX_CODE_BITS + 1 + lengths.len(),
        match r.0 {
            Ok((kraft, nonzero)) => {
                m::lengths_ok(lengths@)
                && kraft == m::Kraft(lengths@)
                && nonzero == lengths.len() - m::count_prefix(lengths@, 0, lengths@.len())
                && final(counts)@[0] == 0
                && (forall|n: int| 1 <= n <= MAX_CODE_BITS ==>
                    final(counts)@[n] == m::count_prefix(lengths@, n as nat, lengths@.len()))
                && r.1@ == MAX_CODE_BITS + 1 + lengths.len()
            },
            Err(index) => !m::lengths_ok(lengths@) && index < lengths.len()
                && lengths@[index as int] > MAX_CODE_BITS
                && r.1@ == MAX_CODE_BITS + 1 + index + 1,
        },
{
    proof { huffman_format_ok(); }
    let ghost mut steps: nat = 0;
    let mut n = 0usize;
    while n <= MAX_CODE_BITS
        invariant
            n <= MAX_CODE_BITS + 1,
            steps == n,
            forall|j: int| 0 <= j < n ==> counts@[j] == 0,
        decreases MAX_CODE_BITS + 1 - n,
    {
        proof { steps = steps + 1; }
        counts[n] = 0;
        n += 1;
    }
    let mut kraft = 0u32;
    let mut nonzero = 0usize;
    let mut s = 0usize;
    while s < lengths.len()
        invariant
            s <= lengths.len() <= FIXED_LIT_COUNT,
            steps == MAX_CODE_BITS + 1 + s,
            nonzero == s - m::count_prefix(lengths@, 0, s as nat),
            nonzero <= s,
            kraft == m::kraft_prefix(lengths@, s as nat),
            kraft <= s * KRAFT_TERM_MAX,
            counts@[0] == 0,
            forall|j: int| 0 <= j < s ==> lengths@[j] <= MAX_CODE_BITS,
            forall|j: int| 1 <= j <= MAX_CODE_BITS ==>
                counts@[j] == m::count_prefix(lengths@, j as nat, s as nat),
        decreases lengths.len() - s,
    {
        proof { steps = steps + 1; }
        let len = lengths[s] as usize;
        if len > MAX_CODE_BITS {
            assert(!m::lengths_ok(lengths@)) by {
                assert(lengths@[s as int] > MAX_CODE_BITS);
            }
            return (Err(s), Ghost(steps));
        }
        if len != 0 {
            proof {
                huffman_format_ok();
                m::lemma_count_bound(lengths@, len as nat, s as nat);
                m::lemma_pow2_monotone((MAX_CODE_BITS - len) as nat, (MAX_CODE_BITS - 1) as nat);
                m::lemma_pow2_vstd((MAX_CODE_BITS - len) as nat);
                vstd::bits::lemma_u32_shl_is_mul(1, (MAX_CODE_BITS - len) as u32);
                lemma_weight_step(kraft as int, s as int, KRAFT_TERM_MAX as int,
                    m::pow2((MAX_CODE_BITS - len) as nat) as int, FIXED_LIT_COUNT as int);
            }
            let old_count = counts[len];
            counts[len] = old_count + 1;
            kraft += 1u32 << (MAX_CODE_BITS - len);
            nonzero += 1;
        } else {
            proof {
                lemma_weight_step(kraft as int, s as int, KRAFT_TERM_MAX as int, 0, FIXED_LIT_COUNT as int);
            }
        }
        proof {
            assert forall|j: int| 1 <= j <= MAX_CODE_BITS implies
                counts@[j] == m::count_prefix(lengths@, j as nat, (s + 1) as nat) by {
                reveal(m::count_prefix);
            }
            reveal(m::kraft_prefix);
            reveal(m::count_prefix);
        }
        s += 1;
    }
    (Ok((kraft, nonzero)), Ghost(steps))
}

} // verus!
