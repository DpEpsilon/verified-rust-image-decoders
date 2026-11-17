//! Independent RFC 1951 model. This module never reads executable tables,
//! counters, buffers or Python code. Canonical words evolve from length counts.
use crate::limits::{CODE_SPACE, FIXED_LIT_COUNT, MAX_CODE_BITS};
use vstd::prelude::*;

verus! {

pub enum CodeKind { CodeLen, LitLen, Dist }
pub enum Decoded {
    Symbol { symbol: nat, position: nat },
    InvalidSymbol { position: nat },
    TruncatedZlib { position: nat },
    OutsideDomain,
}

pub open spec fn pow2(n: nat) -> nat
    decreases n,
{
    if n == 0 { 1 } else { 2 * pow2((n - 1) as nat) }
}

pub open spec fn lengths_ok(lengths: Seq<u8>) -> bool {
    lengths.len() <= FIXED_LIT_COUNT
    && forall|s: int| 0 <= s < lengths.len() ==> lengths[s] <= MAX_CODE_BITS
}

pub open spec fn count_prefix(lengths: Seq<u8>, n: nat, k: nat) -> nat
    decreases k,
{
    if k == 0 || k > lengths.len() { 0 }
    else { count_prefix(lengths, n, (k - 1) as nat)
         + if lengths[(k - 1) as int] == n { 1nat } else { 0nat } }
}

/// RFC bl_count[0] is zero: absent symbols do not receive canonical words.
pub open spec fn bl_count(lengths: Seq<u8>, n: nat) -> nat {
    if n == 0 { 0 } else { count_prefix(lengths, n, lengths.len()) }
}

pub open spec fn kraft_prefix(lengths: Seq<u8>, k: nat) -> nat
    decreases k,
{
    if k == 0 || k > lengths.len() { 0 }
    else {
        let len = lengths[(k - 1) as int];
        kraft_prefix(lengths, (k - 1) as nat)
          + if 1 <= len <= MAX_CODE_BITS { pow2((MAX_CODE_BITS - len) as nat) } else { 0 }
    }
}

pub open spec fn Kraft(lengths: Seq<u8>) -> nat {
    kraft_prefix(lengths, lengths.len())
}

pub open spec fn admissible(lengths: Seq<u8>, kind: CodeKind, single_accept: bool) -> bool {
    let total = lengths.len() - count_prefix(lengths, 0, lengths.len());
    let single = total == 1 && bl_count(lengths, 1) == 1;
    lengths_ok(lengths) && Kraft(lengths) <= CODE_SPACE && (
        Kraft(lengths) == CODE_SPACE || match kind {
            CodeKind::CodeLen => false,
            CodeKind::LitLen => single && single_accept,
            CodeKind::Dist => single || total == 0,
        })
}

/// RFC next_code recurrence, independent of executable first-code arrays.
pub open spec fn next_code(lengths: Seq<u8>, n: nat) -> nat
    decreases n,
{
    if n == 0 { 0 }
    else { 2 * (next_code(lengths, (n - 1) as nat) + bl_count(lengths, (n - 1) as nat)) }
}

pub open spec fn cw_value(lengths: Seq<u8>, s: int) -> nat {
    if 0 <= s < lengths.len() {
        next_code(lengths, lengths[s] as nat)
        + count_prefix(lengths, lengths[s] as nat, s as nat)
    } else { 0 }
}

pub open spec fn msb_bits(value: nat, n: nat) -> Seq<bool>
    decreases n,
{
    if n == 0 { Seq::empty() }
    else { msb_bits(value / 2, (n - 1) as nat).push(value % 2 == 1) }
}

pub open spec fn cw(lengths: Seq<u8>, s: int) -> Option<Seq<bool>> {
    if 0 <= s < lengths.len() && 1 <= lengths[s] <= MAX_CODE_BITS {
        Some(msb_bits(cw_value(lengths, s), lengths[s] as nat))
    } else { None }
}

pub open spec fn prefix(a: Seq<bool>, b: Seq<bool>) -> bool {
    a.len() <= b.len() && a =~= b.take(a.len() as int)
}

pub open spec fn bit(z: Seq<u8>, i: nat) -> bool {
    i < 8 * z.len() && (z[(i / 8) as int] as nat / pow2(i % 8)) % 2 == 1
}

pub open spec fn live(lengths: Seq<u8>, bits: Seq<bool>) -> bool {
    exists|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
        && prefix(bits, cw(lengths, s).unwrap())
}

pub open spec fn exact(lengths: Seq<u8>, bits: Seq<bool>, s: int) -> bool {
    0 <= s < lengths.len() && cw(lengths, s) == Some(bits)
}

pub open spec fn decode_prefix(lengths: Seq<u8>, z: Seq<u8>, start: nat, bits: Seq<bool>) -> Decoded
    decreases if bits.len() <= MAX_CODE_BITS { MAX_CODE_BITS - bits.len() + 1 } else { 0 },
{
    if bits.len() > MAX_CODE_BITS { Decoded::OutsideDomain }
    else if exists|s: int| exact(lengths, bits, s) {
        let s = choose|s: int| exact(lengths, bits, s);
        Decoded::Symbol { symbol: s as nat, position: start + bits.len() }
    } else if !live(lengths, bits) {
        Decoded::InvalidSymbol { position: start + bits.len() }
    } else if start + bits.len() >= 8 * z.len() {
        Decoded::TruncatedZlib { position: start + bits.len() }
    } else {
        decode_prefix(lengths, z, start, bits.push(bit(z, start + bits.len())))
    }
}

pub open spec fn MDec(lengths: Seq<u8>, z: Seq<u8>, start: nat) -> Decoded {
    if lengths_ok(lengths) && Kraft(lengths) <= CODE_SPACE && start <= 8 * z.len() {
        decode_prefix(lengths, z, start, Seq::empty())
    } else { Decoded::OutsideDomain }
}

pub proof fn lemma_pow2_vstd(n: nat)
    ensures pow2(n) == vstd::arithmetic::power2::pow2(n),
    decreases n,
{
    if n > 0 {
        lemma_pow2_vstd((n - 1) as nat);
        vstd::arithmetic::power2::lemma_pow2_unfold(n);
    } else {
        reveal(vstd::arithmetic::power::pow);
        assert(vstd::arithmetic::power2::pow2(0) == 1);
    }
}

pub proof fn lemma_pow2_monotone(a: nat, b: nat)
    requires a <= b,
    ensures pow2(a) <= pow2(b),
{
    lemma_pow2_vstd(a);
    lemma_pow2_vstd(b);
    if a < b { vstd::arithmetic::power2::lemma_pow2_strictly_increases(a, b); }
}

pub proof fn lemma_pow2_positive(n: nat)
    ensures pow2(n) > 0,
    decreases n,
{
    if n > 0 { lemma_pow2_positive((n - 1) as nat); }
}

pub proof fn lemma_count_bound(lengths: Seq<u8>, n: nat, k: nat)
    requires k <= lengths.len(),
    ensures count_prefix(lengths, n, k) <= k,
    decreases k,
{
    if k > 0 { lemma_count_bound(lengths, n, (k - 1) as nat); }
}

/// Sum of the n-bit leaf slots occupied by symbols of length at most n.
/// This per-symbol sum supplies the bridge from Kraft to the RFC recurrence.
pub open spec fn occupied_prefix(lengths: Seq<u8>, n: nat, k: nat) -> nat
    decreases k,
{
    if k == 0 || k > lengths.len() { 0 }
    else {
        let len = lengths[(k - 1) as int];
        occupied_prefix(lengths, n, (k - 1) as nat)
          + if 1 <= len <= n { pow2((n - len) as nat) } else { 0 }
    }
}

pub proof fn lemma_count_monotone(lengths: Seq<u8>, n: nat, a: nat, b: nat)
    requires a <= b <= lengths.len(),
    ensures count_prefix(lengths, n, a) <= count_prefix(lengths, n, b),
    decreases b - a,
{
    if a < b { lemma_count_monotone(lengths, n, a, (b - 1) as nat); }
}

pub proof fn lemma_rank_bound(lengths: Seq<u8>, s: int)
    requires 0 <= s < lengths.len(), lengths[s] > 0,
    ensures count_prefix(lengths, lengths[s] as nat, s as nat)
        < bl_count(lengths, lengths[s] as nat),
{
    lemma_count_monotone(lengths, lengths[s] as nat, (s + 1) as nat, lengths.len());
    assert(count_prefix(lengths, lengths[s] as nat, (s + 1) as nat)
        == count_prefix(lengths, lengths[s] as nat, s as nat) + 1);
}

pub proof fn lemma_rank_strict(lengths: Seq<u8>, s: int, t: int)
    requires 0 <= s < t < lengths.len(), lengths[s] == lengths[t],
    ensures count_prefix(lengths, lengths[s] as nat, s as nat)
        < count_prefix(lengths, lengths[t] as nat, t as nat),
{
    lemma_count_monotone(lengths, lengths[s] as nat, (s + 1) as nat, t as nat);
    assert(count_prefix(lengths, lengths[s] as nat, (s + 1) as nat)
        == count_prefix(lengths, lengths[s] as nat, s as nat) + 1);
}

pub proof fn lemma_occupied_zero(lengths: Seq<u8>, k: nat)
    requires k <= lengths.len(),
    ensures occupied_prefix(lengths, 0, k) == 0,
    decreases k,
{
    if k > 0 { lemma_occupied_zero(lengths, (k - 1) as nat); }
}

pub proof fn lemma_occupied_step(lengths: Seq<u8>, n: nat, k: nat)
    requires n > 0, k <= lengths.len(),
    ensures occupied_prefix(lengths, n, k)
        == 2 * occupied_prefix(lengths, (n - 1) as nat, k) + count_prefix(lengths, n, k),
    decreases k,
{
    if k > 0 {
        lemma_occupied_step(lengths, n, (k - 1) as nat);
        let len = lengths[(k - 1) as int];
        if 1 <= len < n {
            assert(pow2((n - len) as nat) == 2 * pow2((n - 1 - len) as nat));
        } else if len == n {
            assert(pow2(0) == 1);
        }
    }
}

pub proof fn lemma_occupied_canonical(lengths: Seq<u8>, n: nat)
    ensures occupied_prefix(lengths, n, lengths.len()) == next_code(lengths, n) + bl_count(lengths, n),
    decreases n,
{
    if n == 0 { lemma_occupied_zero(lengths, lengths.len()); }
    else {
        lemma_occupied_canonical(lengths, (n - 1) as nat);
        lemma_occupied_step(lengths, n, lengths.len());
    }
}

pub proof fn lemma_kraft_occupied(lengths: Seq<u8>, k: nat)
    requires k <= lengths.len(),
    ensures kraft_prefix(lengths, k) == occupied_prefix(lengths, MAX_CODE_BITS as nat, k),
    decreases k,
{
    if k > 0 { lemma_kraft_occupied(lengths, (k - 1) as nat); }
}

pub proof fn lemma_kraft_canonical(lengths: Seq<u8>)
    ensures Kraft(lengths) == next_code(lengths, MAX_CODE_BITS as nat)
        + bl_count(lengths, MAX_CODE_BITS as nat),
{
    lemma_kraft_occupied(lengths, lengths.len());
    lemma_occupied_canonical(lengths, MAX_CODE_BITS as nat);
}

pub proof fn lemma_capacity_from_top(lengths: Seq<u8>, n: nat, top: nat)
    requires n <= top, next_code(lengths, top) + bl_count(lengths, top) <= pow2(top),
    ensures next_code(lengths, n) + bl_count(lengths, n) <= pow2(n),
    decreases top - n,
{
    if n < top {
        lemma_capacity_from_top(lengths, n + 1, top);
        assert(next_code(lengths, n + 1)
            == 2 * (next_code(lengths, n) + bl_count(lengths, n)));
        assert(pow2(n + 1) == 2 * pow2(n));
    }
}

pub proof fn lemma_canonical_capacity(lengths: Seq<u8>, n: nat)
    requires Kraft(lengths) <= CODE_SPACE, n <= MAX_CODE_BITS,
    ensures next_code(lengths, n) + bl_count(lengths, n) <= pow2(n),
{
    crate::limits::huffman_format_ok();
    lemma_kraft_canonical(lengths);
    lemma_capacity_from_top(lengths, n, MAX_CODE_BITS as nat);
}

pub proof fn lemma_cw_value_bound(lengths: Seq<u8>, s: int)
    requires 0 <= s < lengths.len(), 1 <= lengths[s] <= MAX_CODE_BITS,
        Kraft(lengths) <= CODE_SPACE,
    ensures cw_value(lengths, s) < pow2(lengths[s] as nat),
{
    lemma_rank_bound(lengths, s);
    lemma_canonical_capacity(lengths, lengths[s] as nat);
}

pub proof fn lemma_double_product(a: int, p: int, value: int, extra: int)
    requires a * p <= value, 0 <= extra,
    ensures a * (2 * p) <= 2 * (value + extra),
{
    assert(a * (2 * p) <= 2 * (value + extra)) by (nonlinear_arith)
        requires a * p <= value, 0 <= extra;
}

pub proof fn lemma_multiply_order(a: int, b: int, p: int)
    requires a <= b, 0 <= p,
    ensures a * p <= b * p,
{
    assert(a * p <= b * p) by (nonlinear_arith) requires a <= b, 0 <= p;
}

pub proof fn lemma_code_growth(lengths: Seq<u8>, n: nat, m: nat)
    requires n < m,
    ensures (next_code(lengths, n) + bl_count(lengths, n)) * pow2((m - n) as nat)
        <= next_code(lengths, m),
    decreases m - n,
{
    if m == n + 1 {
        assert(pow2(1) == 2);
    } else {
        lemma_code_growth(lengths, n, (m - 1) as nat);
        lemma_double_product(
            (next_code(lengths, n) + bl_count(lengths, n)) as int,
            pow2((m - 1 - n) as nat) as int,
            next_code(lengths, (m - 1) as nat) as int,
            bl_count(lengths, (m - 1) as nat) as int);
        assert(pow2((m - n) as nat) == 2 * pow2((m - 1 - n) as nat));
    }
}

pub proof fn lemma_msb_bits_len(value: nat, n: nat)
    ensures msb_bits(value, n).len() == n,
    decreases n,
{
    if n > 0 { lemma_msb_bits_len(value / 2, (n - 1) as nat); }
}

pub proof fn lemma_half_bound(value: nat, p: nat)
    requires value < 2 * p,
    ensures value / 2 < p,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(value as int, 2);
}

pub proof fn lemma_msb_injective(a: nat, b: nat, n: nat)
    requires a < pow2(n), b < pow2(n), msb_bits(a, n) == msb_bits(b, n),
    ensures a == b,
    decreases n,
{
    if n > 0 {
        lemma_half_bound(a, pow2((n - 1) as nat));
        lemma_half_bound(b, pow2((n - 1) as nat));
        lemma_msb_bits_len(a / 2, (n - 1) as nat);
        lemma_msb_bits_len(b / 2, (n - 1) as nat);
        assert(msb_bits(a / 2, (n - 1) as nat) =~= msb_bits(a, n).take((n - 1) as int));
        assert(msb_bits(b / 2, (n - 1) as nat) =~= msb_bits(b, n).take((n - 1) as int));
        assert(msb_bits(a / 2, (n - 1) as nat) =~= msb_bits(b / 2, (n - 1) as nat));
        assert(msb_bits(a, n)[(n - 1) as int] == (a % 2 == 1));
        assert(msb_bits(b, n)[(n - 1) as int] == (b % 2 == 1));
        assert((a % 2 == 1) == (b % 2 == 1));
        lemma_msb_injective(a / 2, b / 2, (n - 1) as nat);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(a as int, 2);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(b as int, 2);
    }
}

pub proof fn lemma_double_interval(a: int, p: int, half: int, rem: int, value: int)
    requires a * p <= half < (a + 1) * p, 0 <= rem < 2, value == 2 * half + rem,
    ensures a * (2 * p) <= value < (a + 1) * (2 * p),
{
    assert(a * (2 * p) <= value < (a + 1) * (2 * p)) by (nonlinear_arith)
        requires a * p <= half < (a + 1) * p, 0 <= rem < 2, value == 2 * half + rem;
}

pub proof fn lemma_msb_prefix_interval(a: nat, n: nat, b: nat, m: nat)
    requires n <= m, a < pow2(n), b < pow2(m), prefix(msb_bits(a, n), msb_bits(b, m)),
    ensures a * pow2((m - n) as nat) <= b < (a + 1) * pow2((m - n) as nat),
    decreases m - n,
{
    lemma_msb_bits_len(a, n);
    lemma_msb_bits_len(b, m);
    if n == m {
        assert(pow2((m - n) as nat) == 1);
        assert(msb_bits(a, n) =~= msb_bits(b, m));
        lemma_msb_injective(a, b, n);
    } else {
        lemma_msb_bits_len(b / 2, (m - 1) as nat);
        lemma_half_bound(b, pow2((m - 1) as nat));
        assert(prefix(msb_bits(a, n), msb_bits(b / 2, (m - 1) as nat)));
        lemma_msb_prefix_interval(a, n, b / 2, (m - 1) as nat);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(b as int, 2);
        lemma_double_interval(a as int, pow2((m - 1 - n) as nat) as int,
            (b / 2) as int, (b % 2) as int, b as int);
        assert(pow2((m - n) as nat) == 2 * pow2((m - 1 - n) as nat));
    }
}

/// L-PREFIX-FREE: includes both proper-prefix exclusion and distinct words for
/// distinct symbols, for every non-oversubscribed length vector in the domain.
pub proof fn lemma_prefix_free(lengths: Seq<u8>, s: int, t: int)
    requires lengths_ok(lengths), Kraft(lengths) <= CODE_SPACE,
        0 <= s < lengths.len(), 0 <= t < lengths.len(), lengths[s] > 0, lengths[t] > 0,
    ensures prefix(cw(lengths, s).unwrap(), cw(lengths, t).unwrap()) ==> s == t,
{
    let n = lengths[s] as nat;
    let m = lengths[t] as nat;
    let a = cw_value(lengths, s);
    let b = cw_value(lengths, t);
    lemma_msb_bits_len(a, n);
    lemma_msb_bits_len(b, m);
    lemma_cw_value_bound(lengths, s);
    lemma_cw_value_bound(lengths, t);
    if prefix(cw(lengths, s).unwrap(), cw(lengths, t).unwrap()) {
        assert(n <= m);
        lemma_msb_prefix_interval(a, n, b, m);
        if n == m {
            assert(msb_bits(a, n) =~= msb_bits(b, n));
            lemma_msb_injective(a, b, n);
            assert(a == b);
            if s < t { lemma_rank_strict(lengths, s, t); }
            else if t < s { lemma_rank_strict(lengths, t, s); }
        } else {
            lemma_rank_bound(lengths, s);
            lemma_code_growth(lengths, n, m);
            lemma_multiply_order((a + 1) as int,
                (next_code(lengths, n) + bl_count(lengths, n)) as int,
                pow2((m - n) as nat) as int);
            assert(false);
        }
    }
}

pub open spec fn shorter_prefix(lengths: Seq<u8>, n: nat, k: nat) -> nat
    decreases k,
{
    if k == 0 || k > lengths.len() { 0 }
    else { shorter_prefix(lengths, n, (k - 1) as nat)
        + if 0 < lengths[(k - 1) as int] < n { 1nat } else { 0nat } }
}

pub proof fn lemma_shorter_bound(lengths: Seq<u8>, n: nat, k: nat)
    requires k <= lengths.len(),
    ensures shorter_prefix(lengths, n, k) <= k,
    decreases k,
{
    if k > 0 { lemma_shorter_bound(lengths, n, (k - 1) as nat); }
}

pub proof fn lemma_shorter_small(lengths: Seq<u8>, n: nat, k: nat)
    requires n <= 1, k <= lengths.len(),
    ensures shorter_prefix(lengths, n, k) == 0,
    decreases k,
{
    if k > 0 { lemma_shorter_small(lengths, n, (k - 1) as nat); }
}

pub proof fn lemma_shorter_step(lengths: Seq<u8>, n: nat, k: nat)
    requires n > 0, k <= lengths.len(),
    ensures shorter_prefix(lengths, n + 1, k)
        == shorter_prefix(lengths, n, k) + count_prefix(lengths, n, k),
    decreases k,
{
    if k > 0 { lemma_shorter_step(lengths, n, (k - 1) as nat); }
}

pub proof fn lemma_shorter_recurrence(lengths: Seq<u8>, n: nat)
    requires n > 0,
    ensures shorter_prefix(lengths, n, lengths.len())
        == shorter_prefix(lengths, (n - 1) as nat, lengths.len()) + bl_count(lengths, (n - 1) as nat),
{
    if n == 1 {
        lemma_shorter_small(lengths, 0, lengths.len());
        lemma_shorter_small(lengths, 1, lengths.len());
    } else { lemma_shorter_step(lengths, (n - 1) as nat, lengths.len()); }
}

pub proof fn lemma_exact_unique(lengths: Seq<u8>, bits: Seq<bool>, s: int, t: int)
    requires lengths_ok(lengths), Kraft(lengths) <= CODE_SPACE,
        exact(lengths, bits, s), exact(lengths, bits, t),
    ensures s == t,
{
    assert(lengths[s] > 0 && lengths[t] > 0);
    assert(prefix(bits, bits));
    lemma_prefix_free(lengths, s, t);
}

pub open spec fn symbol_slot(lengths: Seq<u8>, s: int) -> nat {
    if 0 <= s < lengths.len() {
        shorter_prefix(lengths, lengths[s] as nat, lengths.len())
            + count_prefix(lengths, lengths[s] as nat, s as nat)
    } else { 0 }
}

pub proof fn lemma_shorter_monotone(lengths: Seq<u8>, a: nat, b: nat, k: nat)
    requires a <= b, k <= lengths.len(),
    ensures shorter_prefix(lengths, a, k) <= shorter_prefix(lengths, b, k),
    decreases k,
{
    if k > 0 { lemma_shorter_monotone(lengths, a, b, (k - 1) as nat); }
}

pub proof fn lemma_shorter_nonzero_bound(lengths: Seq<u8>, n: nat, k: nat)
    requires k <= lengths.len(),
    ensures shorter_prefix(lengths, n, k) + count_prefix(lengths, 0, k) <= k,
    decreases k,
{
    if k > 0 { lemma_shorter_nonzero_bound(lengths, n, (k - 1) as nat); }
}

pub proof fn lemma_bucket_bound(lengths: Seq<u8>, n: nat)
    requires n > 0,
    ensures shorter_prefix(lengths, n, lengths.len()) + bl_count(lengths, n)
        <= lengths.len() - count_prefix(lengths, 0, lengths.len()),
{
    lemma_shorter_step(lengths, n, lengths.len());
    lemma_shorter_nonzero_bound(lengths, n + 1, lengths.len());
}

pub proof fn lemma_symbol_slot_bound(lengths: Seq<u8>, s: int)
    requires 0 <= s < lengths.len(), lengths[s] > 0,
    ensures symbol_slot(lengths, s) < lengths.len() - count_prefix(lengths, 0, lengths.len()),
{
    lemma_rank_bound(lengths, s);
    lemma_bucket_bound(lengths, lengths[s] as nat);
}

pub proof fn lemma_symbol_slot_distinct(lengths: Seq<u8>, s: int, t: int)
    requires 0 <= s < lengths.len(), 0 <= t < lengths.len(), s != t,
        lengths[s] > 0, lengths[t] > 0,
    ensures symbol_slot(lengths, s) != symbol_slot(lengths, t),
{
    if lengths[s] == lengths[t] {
        if s < t { lemma_rank_strict(lengths, s, t); }
        else { lemma_rank_strict(lengths, t, s); }
    } else if lengths[s] < lengths[t] {
        lemma_rank_bound(lengths, s);
        lemma_shorter_step(lengths, lengths[s] as nat, lengths.len());
        lemma_shorter_monotone(lengths, lengths[s] as nat + 1, lengths[t] as nat, lengths.len());
    } else {
        lemma_rank_bound(lengths, t);
        lemma_shorter_step(lengths, lengths[t] as nat, lengths.len());
        lemma_shorter_monotone(lengths, lengths[t] as nat + 1, lengths[s] as nat, lengths.len());
    }
}

} // verus!
