//! Verified first-code and symbol-offset construction; the actual Huffman
//! constructor calls this function. No executable tables enter the model.
use crate::limits::*;
#[cfg(verus_only)]
use crate::model as m;
use vstd::prelude::*;

verus! {

pub fn first_codes(Ghost(lengths): Ghost<Seq<u8>>, counts: &[u16; MAX_CODE_BITS + 1],
    first: &mut [u32; MAX_CODE_BITS + 1], offsets: &mut [u16; MAX_CODE_BITS + 1],
    next: &mut [u16; MAX_CODE_BITS + 1]) -> (work: Ghost<nat>)
    requires
        m::lengths_ok(lengths), m::Kraft(lengths) <= CODE_SPACE,
        counts@[0] == 0,
        forall|n: int| 1 <= n <= MAX_CODE_BITS ==> counts@[n] == m::bl_count(lengths, n as nat),
    ensures
        work@ == MAX_CODE_BITS,
        forall|n: int| 0 <= n <= MAX_CODE_BITS ==>
            final(first)@[n] == m::next_code(lengths, n as nat),
        forall|n: int| 0 <= n <= MAX_CODE_BITS ==>
            final(offsets)@[n] == m::shorter_prefix(lengths, n as nat, lengths.len()),
        forall|n: int| 0 <= n <= MAX_CODE_BITS ==> final(next)@[n] == final(offsets)@[n],
{
    proof { huffman_format_ok(); m::lemma_shorter_small(lengths, 0, lengths.len()); }
    first[0] = 0;
    offsets[0] = 0;
    next[0] = 0;
    let mut n = 1usize;
    let ghost mut steps: nat = 0;
    while n <= MAX_CODE_BITS
        invariant
            1 <= n <= MAX_CODE_BITS + 1, steps == n - 1,
            m::lengths_ok(lengths), m::Kraft(lengths) <= CODE_SPACE,
            counts@[0] == 0,
            forall|j: int| 1 <= j <= MAX_CODE_BITS ==> counts@[j] == m::bl_count(lengths, j as nat),
            forall|j: int| 0 <= j < n ==> first@[j] == m::next_code(lengths, j as nat),
            forall|j: int| 0 <= j < n ==> offsets@[j] == m::shorter_prefix(lengths, j as nat, lengths.len()),
            forall|j: int| 0 <= j < n ==> next@[j] == offsets@[j],
        decreases MAX_CODE_BITS + 1 - n,
    {
        proof {
            steps = steps + 1;
            huffman_format_ok();
            m::lemma_canonical_capacity(lengths, n as nat);
            m::lemma_pow2_monotone(n as nat, MAX_CODE_BITS as nat);
            m::lemma_shorter_recurrence(lengths, n as nat);
            m::lemma_shorter_bound(lengths, n as nat, lengths.len());
            assert(counts@[(n - 1) as int] == m::bl_count(lengths, (n - 1) as nat));
        }
        first[n] = (first[n - 1] + counts[n - 1] as u32) * 2;
        offsets[n] = offsets[n - 1] + counts[n - 1];
        next[n] = offsets[n];
        n += 1;
    }
    Ghost(steps)
}

/// Fill only assigned symbol slots. Work is recorded at the real loop in
/// erased ghost state; the ordinary caller may keep a separate test diagnostic.
pub fn fill_symbols(lengths: &[u8], Ghost(offsets): Ghost<Seq<u16>>,
    symbols: &mut [u16; FIXED_LIT_COUNT], next: &mut [u16; MAX_CODE_BITS + 1]) -> (work: Ghost<nat>)
    requires
        offsets.len() == MAX_CODE_BITS + 1,
        m::lengths_ok(lengths@),
        forall|n: int| 0 <= n <= MAX_CODE_BITS ==>
            offsets[n] == m::shorter_prefix(lengths@, n as nat, lengths@.len()),
        forall|n: int| 0 <= n <= MAX_CODE_BITS ==> old(next)@[n] == offsets[n],
    ensures
        work@ == lengths.len(),
        forall|n: int| 1 <= n <= MAX_CODE_BITS ==>
            final(next)@[n] == offsets[n] + m::bl_count(lengths@, n as nat),
        forall|s: int| 0 <= s < lengths.len() && lengths@[s] > 0 ==>
            final(symbols)@[m::symbol_slot(lengths@, s) as int] == s,
        forall|n: int, j: int| #![trigger final(next)@[n], final(symbols)@[j]]
            1 <= n <= MAX_CODE_BITS && offsets[n] <= j < final(next)@[n] ==>
            final(symbols)@[j] < lengths.len()
            && lengths@[final(symbols)@[j] as int] == n
            && m::symbol_slot(lengths@, final(symbols)@[j] as int) == j,
{
    proof {
        assert forall|n: int| #![trigger m::count_prefix(lengths@, n as nat, 0)]
            1 <= n <= MAX_CODE_BITS implies
            m::count_prefix(lengths@, n as nat, 0) == 0 by {}
    }
    let mut s = 0usize;
    let ghost mut steps: nat = 0;
    while s < lengths.len()
        invariant
            s <= lengths.len(), m::lengths_ok(lengths@), steps == s,
            forall|n: int| 0 <= n <= MAX_CODE_BITS ==>
                offsets[n] == m::shorter_prefix(lengths@, n as nat, lengths@.len()),
            forall|n: int| 1 <= n <= MAX_CODE_BITS ==>
                next@[n] == offsets[n] + m::count_prefix(lengths@, n as nat, s as nat),
            forall|t: int| 0 <= t < s && lengths@[t] > 0 ==>
                symbols@[m::symbol_slot(lengths@, t) as int] == t,
            forall|n: int, j: int| #![trigger next@[n], symbols@[j]]
                1 <= n <= MAX_CODE_BITS && offsets[n] <= j < next@[n] ==>
                symbols@[j] < s && lengths@[symbols@[j] as int] == n
                && m::symbol_slot(lengths@, symbols@[j] as int) == j,
        decreases lengths.len() - s,
    {
        proof { steps = steps + 1; }
        let len = lengths[s] as usize;
        let ghost old_symbols = symbols@;
        let ghost old_next = next@;
        if len != 0 {
            proof {
                huffman_format_ok();
                m::lemma_symbol_slot_bound(lengths@, s as int);
                m::lemma_bucket_bound(lengths@, len as nat);
                m::lemma_count_monotone(lengths@, len as nat, s as nat, lengths@.len());
                assert(next@[len as int] == m::symbol_slot(lengths@, s as int));
            }
            let slot = next[len] as usize;
            symbols[slot] = s as u16;
            next[len] += 1;
            proof {
                assert forall|t: int| 0 <= t < s + 1 && lengths@[t] > 0 implies
                    symbols@[m::symbol_slot(lengths@, t) as int] == t by {
                    if t != s { m::lemma_symbol_slot_distinct(lengths@, t, s as int); }
                }
                assert forall|n: int, j: int| #![trigger next@[n], symbols@[j]]
                    1 <= n <= MAX_CODE_BITS && offsets[n] <= j < next@[n] implies
                    symbols@[j] < s + 1 && lengths@[symbols@[j] as int] == n
                    && m::symbol_slot(lengths@, symbols@[j] as int) == j by {
                    if j == slot {
                        if n != len {
                            assert(offsets[n] <= j < old_next[n]);
                            let t = old_symbols[j] as int;
                            assert(0 <= t < s && lengths@[t] == n);
                            assert(m::symbol_slot(lengths@, t) == j);
                            m::lemma_symbol_slot_distinct(lengths@, t, s as int);
                            assert(false);
                        }
                    } else {
                        assert(offsets[n] <= j < old_next[n]);
                        assert(symbols@[j] == old_symbols[j]);
                    }
                }
            }
        }
        proof {
            assert forall|n: int| 1 <= n <= MAX_CODE_BITS implies
                next@[n] == offsets[n] + m::count_prefix(lengths@, n as nat, (s + 1) as nat) by {}
        }
        s += 1;
    }
    Ghost(steps)
}

pub open spec fn table_repr(lengths: Seq<u8>, counts: Seq<u16>, first: Seq<u32>,
    offsets: Seq<u16>, symbols: Seq<u16>) -> bool {
    m::lengths_ok(lengths) && m::Kraft(lengths) <= CODE_SPACE
    && counts.len() == MAX_CODE_BITS + 1 && first.len() == MAX_CODE_BITS + 1
    && offsets.len() == MAX_CODE_BITS + 1 && symbols.len() == FIXED_LIT_COUNT
    && counts[0] == 0
    && (forall|n: int| 1 <= n <= MAX_CODE_BITS ==> counts[n] == m::bl_count(lengths, n as nat))
    && (forall|n: int| 0 <= n <= MAX_CODE_BITS ==> first[n] == m::next_code(lengths, n as nat))
    && (forall|n: int| 0 <= n <= MAX_CODE_BITS ==> offsets[n] == m::shorter_prefix(lengths, n as nat, lengths.len()))
    && (forall|s: int| 0 <= s < lengths.len() && lengths[s] > 0 ==>
        symbols[m::symbol_slot(lengths, s) as int] == s)
    && (forall|n: int, j: int| #![trigger counts[n], symbols[j]]
        1 <= n <= MAX_CODE_BITS && offsets[n] <= j < offsets[n] + counts[n] ==>
        symbols[j] < lengths.len() && lengths[symbols[j] as int] == n
        && m::symbol_slot(lengths, symbols[j] as int) == j)
}

pub open spec fn table_lookup(counts: Seq<u16>, first: Seq<u32>, offsets: Seq<u16>,
    symbols: Seq<u16>, n: nat, code: nat) -> Option<u16> {
    if 1 <= n <= MAX_CODE_BITS && n < counts.len() && n < first.len() && n < offsets.len()
        && code >= first[n as int] && code - first[n as int] < counts[n as int]
        && offsets[n as int] + code - first[n as int] < symbols.len() {
        Some(symbols[(offsets[n as int] + code - first[n as int]) as int])
    } else { None }
}

pub open spec fn lookup_eq(lengths: Seq<u8>, r: Option<u16>, n: nat, code: nat) -> bool {
    match r {
        Some(s) => s < lengths.len() && lengths[s as int] == n && m::cw_value(lengths, s as int) == code,
        None => forall|s: int| 0 <= s < lengths.len() && lengths[s] == n ==>
            m::cw_value(lengths, s) != code,
    }
}

pub proof fn lemma_lookup_representation(lengths: Seq<u8>, counts: Seq<u16>, first: Seq<u32>,
    offsets: Seq<u16>, symbols: Seq<u16>, n: nat, code: nat)
    requires table_repr(lengths, counts, first, offsets, symbols), 1 <= n <= MAX_CODE_BITS,
    ensures lookup_eq(lengths, table_lookup(counts, first, offsets, symbols, n, code), n, code),
{
    m::lemma_bucket_bound(lengths, n);
    if code >= first[n as int] && code - first[n as int] < counts[n as int] {
        let j = (offsets[n as int] + code - first[n as int]) as int;
        assert(offsets[n as int] <= j < offsets[n as int] + counts[n as int]);
        assert(j < symbols.len());
        assert(symbols[j] < lengths.len() && lengths[symbols[j] as int] == n);
        assert(m::symbol_slot(lengths, symbols[j] as int) == j);
    } else {
        assert forall|s: int| 0 <= s < lengths.len() && lengths[s] == n implies
            m::cw_value(lengths, s) != code by {
            m::lemma_rank_bound(lengths, s);
        }
    }
}

/// L-CANON's forward direction: the independent word of every present symbol
/// selects exactly that symbol; no claim constrains unused table slots.
pub proof fn lemma_lookup_canonical(lengths: Seq<u8>, counts: Seq<u16>, first: Seq<u32>,
    offsets: Seq<u16>, symbols: Seq<u16>, s: int)
    requires table_repr(lengths, counts, first, offsets, symbols),
        0 <= s < lengths.len(), lengths[s] > 0,
    ensures table_lookup(counts, first, offsets, symbols, lengths[s] as nat,
        m::cw_value(lengths, s)) == Some(s as u16),
{
    m::lemma_rank_bound(lengths, s);
    m::lemma_symbol_slot_bound(lengths, s);
    assert(symbols[m::symbol_slot(lengths, s) as int] == s);
}

/// L-CANON's reverse direction, expressed over actual codeword bit strings.
pub proof fn lemma_lookup_word(lengths: Seq<u8>, counts: Seq<u16>, first: Seq<u32>,
    offsets: Seq<u16>, symbols: Seq<u16>, n: nat, code: nat, s: u16)
    requires table_repr(lengths, counts, first, offsets, symbols), 1 <= n <= MAX_CODE_BITS,
        table_lookup(counts, first, offsets, symbols, n, code) == Some(s),
    ensures m::cw(lengths, s as int) == Some(m::msb_bits(code, n)),
{
    lemma_lookup_representation(lengths, counts, first, offsets, symbols, n, code);
}

/// The actual concrete table lookup used by Huffman::decode. Its specification
/// covers every independent length vector represented by the supplied tables.
pub fn lookup(counts: &[u16; MAX_CODE_BITS + 1], first: &[u32; MAX_CODE_BITS + 1],
    offsets: &[u16; MAX_CODE_BITS + 1], symbols: &[u16; FIXED_LIT_COUNT],
    n: usize, code: u32) -> (r: Option<u16>)
    requires 1 <= n <= MAX_CODE_BITS,
        offsets@[n as int] + counts@[n as int] <= FIXED_LIT_COUNT,
    ensures
        r == table_lookup(counts@, first@, offsets@, symbols@, n as nat, code as nat),
        forall|lengths: Seq<u8>| #[trigger] table_repr(lengths, counts@, first@, offsets@, symbols@)
            ==> lookup_eq(lengths, r, n as nat, code as nat),
{
    let r = if code >= first[n] && code - first[n] < counts[n] as u32 {
        let index = offsets[n] as usize + (code - first[n]) as usize;
        Some(symbols[index])
    } else { None };
    proof {
        assert forall|lengths: Seq<u8>| #[trigger] table_repr(lengths, counts@, first@, offsets@, symbols@)
            implies lookup_eq(lengths, r, n as nat, code as nat) by {
            lemma_lookup_representation(lengths, counts@, first@, offsets@, symbols@, n as nat, code as nat);
        }
    }
    r
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildError {
    TooManySymbols,
    InvalidLength(usize),
    Oversubscribed,
}

/// Complete executable canonical-table constructor, including exact rejection
/// and an erased loop/fill work observation. Dynamic admissibility is separate.
pub fn build_tables(lengths: &[u8], counts: &mut [u16; MAX_CODE_BITS + 1],
    first: &mut [u32; MAX_CODE_BITS + 1], offsets: &mut [u16; MAX_CODE_BITS + 1],
    symbols: &mut [u16; FIXED_LIT_COUNT])
    -> (r: (Result<(u32, usize), BuildError>, Ghost<nat>))
    ensures
        r.0.is_ok() == (m::lengths_ok(lengths@) && m::Kraft(lengths@) <= CODE_SPACE),
        match r.0 {
            Ok((kraft, nonzero)) => {
                table_repr(lengths@, final(counts)@, final(first)@, final(offsets)@, final(symbols)@)
                && kraft == m::Kraft(lengths@)
                && nonzero == lengths.len() - m::count_prefix(lengths@, 0, lengths@.len())
                && r.1@ == CANON_FIXED_STEPS + 2 * lengths.len()
            },
            Err(BuildError::TooManySymbols) => lengths.len() > FIXED_LIT_COUNT && r.1@ == 0,
            Err(BuildError::InvalidLength(index)) => lengths.len() <= FIXED_LIT_COUNT
                && index < lengths.len() && lengths@[index as int] > MAX_CODE_BITS
                && r.1@ == MAX_CODE_BITS + 1 + index + 1,
            Err(BuildError::Oversubscribed) => m::lengths_ok(lengths@)
                && m::Kraft(lengths@) > CODE_SPACE && r.1@ == MAX_CODE_BITS + 1 + lengths.len(),
        },
{
    if lengths.len() > FIXED_LIT_COUNT {
        return (Err(BuildError::TooManySymbols), Ghost(0nat));
    }
    let (counted, count_work) = crate::classification::count_lengths(lengths, counts);
    let (kraft, nonzero) = match counted {
        Ok(value) => value,
        Err(index) => return (Err(BuildError::InvalidLength(index)), count_work),
    };
    if kraft > CODE_SPACE { return (Err(BuildError::Oversubscribed), count_work); }
    let mut next = [0u16; MAX_CODE_BITS + 1];
    let _first_work = first_codes(Ghost(lengths@), counts, first, offsets, &mut next);
    let _symbol_work = fill_symbols(lengths, Ghost(offsets@), symbols, &mut next);
    proof {
        huffman_format_ok();
        assert forall|n: int, j: int| #![trigger counts@[n], symbols@[j]]
            1 <= n <= MAX_CODE_BITS && offsets@[n] <= j < offsets@[n] + counts@[n] implies
            symbols@[j] < lengths.len() && lengths@[symbols@[j] as int] == n
            && m::symbol_slot(lengths@, symbols@[j] as int) == j by {
            assert(next@[n] == offsets@[n] + counts@[n]);
        }
        assert(table_repr(lengths@, counts@, first@, offsets@, symbols@));
    }
    (Ok((kraft, nonzero)), Ghost(count_work@ + (MAX_CODE_BITS + 1) as nat + _first_work@ + _symbol_work@))
}

/// Actual reverse loop used by Huffman construction. Its invariant is the
/// independent per-symbol remaining mass, not another table recurrence.
pub fn live_counts(Ghost(lengths): Ghost<Seq<u8>>, counts: &[u16; MAX_CODE_BITS + 1],
    live: &mut [u32; MAX_CODE_BITS + 1]) -> (work: Ghost<nat>)
    requires m::lengths_ok(lengths), m::Kraft(lengths) <= CODE_SPACE,
        forall|n: int| 1 <= n <= MAX_CODE_BITS ==> counts@[n] == m::bl_count(lengths, n as nat),
    ensures work@ == MAX_CODE_BITS + 1,
        forall|n: int| 0 <= n <= MAX_CODE_BITS ==> final(live)@[n] == m::live_count(lengths, n as nat),
{
    proof { huffman_format_ok(); m::lemma_remaining_bounds(lengths, MAX_CODE_BITS as nat, lengths.len()); }
    let mut higher = 0u32;
    let mut n = MAX_CODE_BITS;
    let ghost mut steps: nat = 0;
    loop
        invariant
            n <= MAX_CODE_BITS, steps == MAX_CODE_BITS - n,
            m::lengths_ok(lengths), m::Kraft(lengths) <= CODE_SPACE,
            higher == m::remaining_mass(lengths, n as nat),
            forall|j: int| 1 <= j <= MAX_CODE_BITS ==> counts@[j] == m::bl_count(lengths, j as nat),
            forall|j: int| n < j <= MAX_CODE_BITS ==> live@[j] == m::live_count(lengths, j as nat),
        decreases n,
    {
        proof {
            steps = steps + 1;
            huffman_format_ok();
            m::lemma_remaining_bounds(lengths, n as nat, lengths.len());
            m::lemma_pow2_monotone((MAX_CODE_BITS - n) as nat, MAX_CODE_BITS as nat);
            m::lemma_pow2_positive((MAX_CODE_BITS - n) as nat);
            m::lemma_pow2_vstd((MAX_CODE_BITS - n) as nat);
            vstd::bits::lemma_u32_shl_is_mul(1, (MAX_CODE_BITS - n) as u32);
        }
        let unit = 1u32 << (MAX_CODE_BITS - n);
        live[n] = (higher + unit - 1) / unit;
        if n == 0 { return Ghost(steps); }
        proof { m::lemma_remaining_step(lengths, n as nat, lengths.len());
            m::lemma_remaining_bounds(lengths, (n - 1) as nat, lengths.len()); }
        higher += counts[n] as u32 * unit;
        n -= 1;
    }
}

/// Representation bridge for the actual post-lookup dead-prefix expression.
/// This is not a proof of Huffman::decode's reader loop or reachable states.
pub proof fn lemma_dead_prefix_representation(lengths: Seq<u8>, counts: Seq<u16>, first: Seq<u32>,
    offsets: Seq<u16>, symbols: Seq<u16>, live: Seq<u32>, kind: m::CodeKind,
    single_accept: bool, n: nat, code: nat)
    requires table_repr(lengths, counts, first, offsets, symbols),
        m::admissible(lengths, kind, single_accept),
        live.len() == MAX_CODE_BITS + 1,
        forall|j: int| 0 <= j <= MAX_CODE_BITS ==> live[j] == m::live_count(lengths, j as nat),
        1 <= n <= MAX_CODE_BITS, code < m::pow2(n),
        table_lookup(counts, first, offsets, symbols, n, code).is_none(),
    ensures
        forall|s: int| !m::exact(lengths, m::msb_bits(code, n), s),
        code < first[n as int] || code >= first[n as int] + counts[n as int],
        !m::live(lengths, m::msb_bits(code, n)) == (code < first[n as int]
            || code - first[n as int] - counts[n as int] >= live[n as int]),
{
    m::lemma_bucket_bound(lengths, n);
    assert(code < first[n as int] || code >= first[n as int] + counts[n as int]);
    m::lemma_numeric_exact(lengths, n, code);
    m::lemma_admissible_dead_prefix(lengths, kind, single_accept, n, code);
}

pub proof fn lemma_lookup_bounds(lengths: Seq<u8>, counts: Seq<u16>, first: Seq<u32>,
    offsets: Seq<u16>, symbols: Seq<u16>, n: nat)
    requires table_repr(lengths, counts, first, offsets, symbols), 1 <= n <= MAX_CODE_BITS,
    ensures offsets[n as int] + counts[n as int] <= FIXED_LIT_COUNT,
{
    m::lemma_bucket_bound(lengths, n);
}

} // verus!
