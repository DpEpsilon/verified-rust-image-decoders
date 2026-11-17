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
        assert(pow2((m - n) as nat) == 2);
        assert(next_code(lengths, m) == 2 * (next_code(lengths, n) + bl_count(lengths, n)));
        assert((next_code(lengths, n) + bl_count(lengths, n)) * 2
            == next_code(lengths, m)) by (nonlinear_arith)
            requires next_code(lengths, m) == 2 * (next_code(lengths, n) + bl_count(lengths, n));
    } else {
        lemma_code_growth(lengths, n, (m - 1) as nat);
        lemma_double_product(
            (next_code(lengths, n) + bl_count(lengths, n)) as int,
            pow2((m - 1 - n) as nat) as int,
            next_code(lengths, (m - 1) as nat) as int,
            bl_count(lengths, (m - 1) as nat) as int);
        assert(pow2((m - n) as nat) == 2 * pow2((m - 1 - n) as nat));
        assert(next_code(lengths, m) == 2 * (next_code(lengths, (m - 1) as nat)
            + bl_count(lengths, (m - 1) as nat)));
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

/// Unassigned leaf mass after depth n, summed independently per symbol.
pub open spec fn remaining_mass_prefix(lengths: Seq<u8>, n: nat, k: nat) -> nat
    decreases k,
{
    if k == 0 || k > lengths.len() { 0 }
    else {
        let len = lengths[(k - 1) as int];
        remaining_mass_prefix(lengths, n, (k - 1) as nat)
            + if n < len <= MAX_CODE_BITS { pow2((MAX_CODE_BITS - len) as nat) } else { 0 }
    }
}

pub open spec fn remaining_mass(lengths: Seq<u8>, n: nat) -> nat {
    remaining_mass_prefix(lengths, n, lengths.len())
}

pub open spec fn live_count(lengths: Seq<u8>, n: nat) -> nat {
    let unit = pow2((MAX_CODE_BITS - n) as nat);
    ((remaining_mass(lengths, n) + unit - 1) / (unit as int)) as nat
}

pub proof fn lemma_remaining_bounds(lengths: Seq<u8>, n: nat, k: nat)
    requires k <= lengths.len(),
    ensures remaining_mass_prefix(lengths, n, k) <= kraft_prefix(lengths, k),
        n == 0 ==> remaining_mass_prefix(lengths, n, k) == kraft_prefix(lengths, k),
        n >= MAX_CODE_BITS ==> remaining_mass_prefix(lengths, n, k) == 0,
    decreases k,
{
    if k > 0 { lemma_remaining_bounds(lengths, n, (k - 1) as nat); }
}

pub proof fn lemma_remaining_step(lengths: Seq<u8>, n: nat, k: nat)
    requires 1 <= n <= MAX_CODE_BITS, k <= lengths.len(),
    ensures remaining_mass_prefix(lengths, (n - 1) as nat, k)
        == remaining_mass_prefix(lengths, n, k)
            + count_prefix(lengths, n, k) * pow2((MAX_CODE_BITS - n) as nat),
    decreases k,
{
    reveal(remaining_mass_prefix);
    reveal(count_prefix);
    if k > 0 {
        lemma_remaining_step(lengths, n, (k - 1) as nat);
        let len = lengths[(k - 1) as int];
        assert((n - 1 < len && len <= MAX_CODE_BITS)
            == ((n < len && len <= MAX_CODE_BITS) || len == n));
        let unit = pow2((MAX_CODE_BITS - n) as nat);
        let a = remaining_mass_prefix(lengths, (n - 1) as nat, (k - 1) as nat);
        let b = remaining_mass_prefix(lengths, n, (k - 1) as nat);
        let c = count_prefix(lengths, n, (k - 1) as nat);
        let extra = if n < len <= MAX_CODE_BITS { pow2((MAX_CODE_BITS - len) as nat) } else { 0nat };
        assert(a == b + c * unit);
        assert(remaining_mass_prefix(lengths, n, k) == b + extra);
        if len == n {
            assert(remaining_mass_prefix(lengths, (n - 1) as nat, k) == a + unit);
            assert(count_prefix(lengths, n, k) == c + 1);
            assert(a + unit == b + extra + (c + 1) * unit) by (nonlinear_arith)
                requires a == b + c * unit, extra == 0;
        } else {
            assert(remaining_mass_prefix(lengths, (n - 1) as nat, k) == a + extra);
            assert(count_prefix(lengths, n, k) == c);
        }
        assert(remaining_mass_prefix(lengths, (n - 1) as nat, k)
            == remaining_mass_prefix(lengths, n, k) + count_prefix(lengths, n, k) * unit);
    } else {
        assert(remaining_mass_prefix(lengths, (n - 1) as nat, k) == 0);
        assert(remaining_mass_prefix(lengths, n, k) == 0);
        assert(count_prefix(lengths, n, k) == 0);
        assert(count_prefix(lengths, n, k) * pow2((MAX_CODE_BITS - n) as nat) == 0) by (nonlinear_arith)
            requires count_prefix(lengths, n, k) == 0;
    }
}

pub proof fn lemma_pow2_add(a: nat, b: nat)
    ensures pow2(a + b) == pow2(a) * pow2(b),
{
    lemma_pow2_vstd(a); lemma_pow2_vstd(b); lemma_pow2_vstd(a + b);
    vstd::arithmetic::power2::lemma_pow2_adds(a, b);
}

pub proof fn lemma_remaining_partition(lengths: Seq<u8>, n: nat, k: nat)
    requires n <= MAX_CODE_BITS, k <= lengths.len(),
    ensures remaining_mass_prefix(lengths, n, k)
        + occupied_prefix(lengths, n, k) * pow2((MAX_CODE_BITS - n) as nat)
        == kraft_prefix(lengths, k),
    decreases k,
{
    if k > 0 {
        lemma_remaining_partition(lengths, n, (k - 1) as nat);
        let len = lengths[(k - 1) as int];
        let unit = pow2((MAX_CODE_BITS - n) as nat);
        let prev = occupied_prefix(lengths, n, (k - 1) as nat);
        if 1 <= len <= n {
            lemma_pow2_add((n - len) as nat, (MAX_CODE_BITS - n) as nat);
            assert(occupied_prefix(lengths, n, k) * unit
                == prev * unit + pow2((MAX_CODE_BITS - len) as nat)) by (nonlinear_arith)
                requires occupied_prefix(lengths, n, k) == prev + pow2((n - len) as nat),
                    pow2((MAX_CODE_BITS - len) as nat) == pow2((n - len) as nat) * unit;
        }
        assert(remaining_mass_prefix(lengths, n, k) + occupied_prefix(lengths, n, k) * unit
            == kraft_prefix(lengths, k));
    } else {
        assert(occupied_prefix(lengths, n, k) == 0);
        assert(occupied_prefix(lengths, n, k) * pow2((MAX_CODE_BITS - n) as nat) == 0) by (nonlinear_arith)
            requires occupied_prefix(lengths, n, k) == 0;
    }
}

/// The rounded mass is exact for complete codes: every still-unassigned
/// n-bit word is a proper prefix, with no unfilled tail interval.
pub proof fn lemma_complete_live_count(lengths: Seq<u8>, n: nat)
    requires Kraft(lengths) == CODE_SPACE, n <= MAX_CODE_BITS,
    ensures live_count(lengths, n) + next_code(lengths, n) + bl_count(lengths, n) == pow2(n),
{
    crate::limits::huffman_format_ok();
    lemma_remaining_partition(lengths, n, lengths.len());
    lemma_occupied_canonical(lengths, n);
    lemma_canonical_capacity(lengths, n);
    lemma_pow2_add(n, (MAX_CODE_BITS - n) as nat);
    lemma_pow2_positive((MAX_CODE_BITS - n) as nat);
    let unit = pow2((MAX_CODE_BITS - n) as nat);
    let used = next_code(lengths, n) + bl_count(lengths, n);
    let free = (pow2(n) - used) as nat;
    assert(remaining_mass(lengths, n) == free * unit) by (nonlinear_arith)
        requires remaining_mass(lengths, n) + used * unit == CODE_SPACE,
            CODE_SPACE == pow2(n) * unit, free == pow2(n) - used, unit > 0;
    assert(remaining_mass(lengths, n) == unit * free) by (nonlinear_arith)
        requires remaining_mass(lengths, n) == free * unit;
    vstd::arithmetic::div_mod::lemma_div_multiples_vanish_fancy(free as int, (unit - 1) as int, unit as int);
    vstd::arithmetic::div_mod::lemma_basic_div((unit - 1) as int, unit as int);
}

/// Every rank below the independent count names an actual symbol.
pub proof fn lemma_rank_exists(lengths: Seq<u8>, n: nat, k: nat, rank: nat)
    requires k <= lengths.len(), rank < count_prefix(lengths, n, k),
    ensures exists|s: int| 0 <= s < k && lengths[s] == n
        && count_prefix(lengths, n, s as nat) == rank,
    decreases k,
{
    assert(k > 0);
    if rank < count_prefix(lengths, n, (k - 1) as nat) {
        lemma_rank_exists(lengths, n, (k - 1) as nat, rank);
        let s = choose|s: int| 0 <= s < k - 1 && lengths[s] == n
            && count_prefix(lengths, n, s as nat) == rank;
        assert(0 <= s < k);
    } else {
        assert(lengths[(k - 1) as int] == n);
        assert(count_prefix(lengths, n, (k - 1) as nat) == rank);
    }
}

pub proof fn lemma_prefix_transitive(a: Seq<bool>, b: Seq<bool>, c: Seq<bool>)
    requires prefix(a, b), prefix(b, c),
    ensures prefix(a, c),
{
    assert(a =~= c.take(a.len() as int));
}

pub proof fn lemma_msb_extend(code: nat, n: nat)
    ensures prefix(msb_bits(code, n), msb_bits(2 * code, n + 1)),
{
    lemma_msb_bits_len(code, n);
    vstd::arithmetic::div_mod::lemma_div_multiples_vanish(code as int, 2);
    assert(msb_bits(2 * code, n + 1) == msb_bits(code, n).push(false));
}

/// Descend an independent complete code until a count interval contains the
/// numeric word. This produces a canonical-word witness without table slots.
pub proof fn lemma_complete_extension(lengths: Seq<u8>, n: nat, code: nat)
    requires lengths_ok(lengths), Kraft(lengths) == CODE_SPACE,
        n <= MAX_CODE_BITS, next_code(lengths, n) <= code < pow2(n),
    ensures exists|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
        && lengths[s] >= n && prefix(msb_bits(code, n), cw(lengths, s).unwrap()),
    decreases MAX_CODE_BITS - n,
{
    if code < next_code(lengths, n) + bl_count(lengths, n) {
        assert(n > 0);
        lemma_rank_exists(lengths, n, lengths.len(), (code - next_code(lengths, n)) as nat);
        let s = choose|s: int| 0 <= s < lengths.len() && lengths[s] == n
            && count_prefix(lengths, n, s as nat) == code - next_code(lengths, n);
        lemma_msb_bits_len(code, n);
        assert(cw(lengths, s) == Some(msb_bits(code, n)));
        assert(prefix(msb_bits(code, n), cw(lengths, s).unwrap()));
    } else {
        if n == MAX_CODE_BITS {
            crate::limits::huffman_format_ok(); lemma_kraft_canonical(lengths);
            assert(false);
        }
        assert(2 * code < pow2(n + 1));
        lemma_complete_extension(lengths, n + 1, 2 * code);
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && lengths[s] >= n + 1 && prefix(msb_bits(2 * code, n + 1), cw(lengths, s).unwrap());
        lemma_msb_extend(code, n);
        lemma_prefix_transitive(msb_bits(code, n), msb_bits(2 * code, n + 1), cw(lengths, s).unwrap());
    }
}

pub open spec fn proper_live(lengths: Seq<u8>, bits: Seq<bool>) -> bool {
    exists|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
        && bits.len() < lengths[s] && prefix(bits, cw(lengths, s).unwrap())
}

pub proof fn lemma_complete_proper_live(lengths: Seq<u8>, n: nat, code: nat)
    requires lengths_ok(lengths), Kraft(lengths) == CODE_SPACE,
        n <= MAX_CODE_BITS, code < pow2(n),
    ensures proper_live(lengths, msb_bits(code, n))
        == (next_code(lengths, n) + bl_count(lengths, n) <= code),
{
    lemma_msb_bits_len(code, n);
    if proper_live(lengths, msb_bits(code, n)) {
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && n < lengths[s] && prefix(msb_bits(code, n), cw(lengths, s).unwrap());
        let top = lengths[s] as nat;
        lemma_cw_value_bound(lengths, s); lemma_rank_bound(lengths, s);
        lemma_code_growth(lengths, n, top);
        lemma_msb_prefix_interval(code, n, cw_value(lengths, s), top);
        if code < next_code(lengths, n) + bl_count(lengths, n) {
            lemma_multiply_order((code + 1) as int,
                (next_code(lengths, n) + bl_count(lengths, n)) as int,
                pow2((top - n) as nat) as int);
            assert(false);
        }
    } else if next_code(lengths, n) + bl_count(lengths, n) <= code {
        if n == MAX_CODE_BITS {
            crate::limits::huffman_format_ok(); lemma_kraft_canonical(lengths); assert(false);
        }
        lemma_complete_extension(lengths, n + 1, 2 * code);
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && lengths[s] >= n + 1 && prefix(msb_bits(2 * code, n + 1), cw(lengths, s).unwrap());
        lemma_msb_extend(code, n);
        lemma_prefix_transitive(msb_bits(code, n), msb_bits(2 * code, n + 1), cw(lengths, s).unwrap());
        assert(proper_live(lengths, msb_bits(code, n)));
    }
}

pub proof fn lemma_zero_one_partition(lengths: Seq<u8>, k: nat)
    requires k <= lengths.len(),
    ensures count_prefix(lengths, 0, k) + count_prefix(lengths, 1, k) <= k,
        count_prefix(lengths, 0, k) + count_prefix(lengths, 1, k) == k ==>
            forall|s: int| 0 <= s < k ==> lengths[s] <= 1,
        count_prefix(lengths, 0, k) == k ==>
            forall|s: int| 0 <= s < k ==> lengths[s] == 0,
    decreases k,
{
    if k > 0 {
        lemma_zero_one_partition(lengths, (k - 1) as nat);
        assert forall|s: int| 0 <= s < k && count_prefix(lengths, 0, k) + count_prefix(lengths, 1, k) == k
            implies lengths[s] <= 1 by {}
        assert forall|s: int| 0 <= s < k && count_prefix(lengths, 0, k) == k
            implies lengths[s] == 0 by {}
    }
}

pub proof fn lemma_remaining_small(lengths: Seq<u8>, n: nat, k: nat)
    requires k <= lengths.len(), forall|s: int| 0 <= s < k ==> lengths[s] <= n,
    ensures remaining_mass_prefix(lengths, n, k) == 0,
    decreases k,
{
    if k > 0 { lemma_remaining_small(lengths, n, (k - 1) as nat); }
}

pub open spec fn single_one(lengths: Seq<u8>) -> bool {
    lengths.len() - count_prefix(lengths, 0, lengths.len()) == 1 && bl_count(lengths, 1) == 1
}

pub open spec fn empty_code(lengths: Seq<u8>) -> bool {
    count_prefix(lengths, 0, lengths.len()) == lengths.len()
}

pub proof fn lemma_admissible_shapes(lengths: Seq<u8>, kind: CodeKind, single_accept: bool)
    requires admissible(lengths, kind, single_accept),
    ensures Kraft(lengths) == CODE_SPACE || single_one(lengths) || empty_code(lengths),
{
    match kind { CodeKind::CodeLen => {}, CodeKind::LitLen => {}, CodeKind::Dist => {} }
}

pub proof fn lemma_one_bit_zero()
    ensures msb_bits(0, 1) == seq![false],
{
    reveal_with_fuel(msb_bits, 2);
    assert(msb_bits(0, 1) =~= seq![false]);
}

pub proof fn lemma_single_words(lengths: Seq<u8>)
    requires single_one(lengths),
    ensures (forall|s: int| 0 <= s < lengths.len() ==> lengths[s] <= 1),
        (forall|s: int| 0 <= s < lengths.len() && lengths[s] > 0 ==>
            cw(lengths, s) == Some(seq![false])),
        exists|s: int| 0 <= s < lengths.len() && cw(lengths, s) == Some(seq![false]),
{
    lemma_zero_one_partition(lengths, lengths.len());
    lemma_one_bit_zero();
    assert(next_code(lengths, 1) == 0) by { reveal_with_fuel(next_code, 2); }
    assert forall|s: int| 0 <= s < lengths.len() && lengths[s] > 0 implies
        cw(lengths, s) == Some(seq![false]) by {
        lemma_rank_bound(lengths, s);
        assert(count_prefix(lengths, 1, s as nat) == 0);
        assert(msb_bits(0, 1) =~= seq![false]);
    }
    lemma_rank_exists(lengths, 1, lengths.len(), 0);
    let s = choose|s: int| 0 <= s < lengths.len() && lengths[s] == 1
        && count_prefix(lengths, 1, s as nat) == 0;
    assert(cw(lengths, s) == Some(seq![false]));
}

pub proof fn lemma_single_live(lengths: Seq<u8>, n: nat, code: nat)
    requires single_one(lengths), n <= MAX_CODE_BITS, code < pow2(n),
    ensures proper_live(lengths, msb_bits(code, n)) == (n == 0),
        live(lengths, msb_bits(code, n)) == (n == 0 || (n == 1 && code == 0)),
        live_count(lengths, n) == if n == 0 { 1nat } else { 0nat },
{
    crate::limits::huffman_format_ok(); lemma_single_words(lengths); lemma_one_bit_zero();
    lemma_msb_bits_len(code, n);
    if live(lengths, msb_bits(code, n)) {
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && prefix(msb_bits(code, n), cw(lengths, s).unwrap());
        assert(cw(lengths, s) == Some(seq![false]));
        if n == 1 {
            assert(msb_bits(code, n) =~= msb_bits(0, 1));
            lemma_msb_injective(code, 0, 1);
        }
    }
    if proper_live(lengths, msb_bits(code, n)) {
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && n < lengths[s] && prefix(msb_bits(code, n), cw(lengths, s).unwrap());
        assert(lengths[s] == 1);
    }
    let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s) == Some(seq![false]);
    if n == 0 || (n == 1 && code == 0) {
        assert(prefix(msb_bits(code, n), cw(lengths, s).unwrap()));
        assert(live(lengths, msb_bits(code, n)));
        if n == 0 { assert(proper_live(lengths, msb_bits(code, n))); }
    }
    let unit = pow2((MAX_CODE_BITS - n) as nat);
    lemma_pow2_positive((MAX_CODE_BITS - n) as nat);
    if n > 0 {
        lemma_remaining_small(lengths, n, lengths.len());
        vstd::arithmetic::div_mod::lemma_basic_div((unit - 1) as int, unit as int);
    } else {
        lemma_remaining_small(lengths, 1, lengths.len());
        lemma_remaining_partition(lengths, 1, lengths.len()); lemma_occupied_canonical(lengths, 1);
        lemma_remaining_bounds(lengths, 0, lengths.len());
        let half = pow2((MAX_CODE_BITS - 1) as nat);
        assert(next_code(lengths, 1) == 0) by { reveal_with_fuel(next_code, 2); }
        assert(occupied_prefix(lengths, 1, lengths.len()) == 1);
        assert(remaining_mass(lengths, 0) == half) by (nonlinear_arith)
            requires remaining_mass(lengths, 0) == Kraft(lengths),
                remaining_mass(lengths, 1) == 0,
                occupied_prefix(lengths, 1, lengths.len()) == 1,
                remaining_mass(lengths, 1) + occupied_prefix(lengths, 1, lengths.len()) * half == Kraft(lengths);
        assert(unit == 2 * half);
        lemma_pow2_positive((MAX_CODE_BITS - 1) as nat);
        vstd::arithmetic::div_mod::lemma_div_multiples_vanish_fancy(1, (half - 1) as int, unit as int);
    }
}

pub proof fn lemma_empty_live(lengths: Seq<u8>, n: nat, code: nat)
    requires empty_code(lengths), n <= MAX_CODE_BITS,
    ensures !live(lengths, msb_bits(code, n)), !proper_live(lengths, msb_bits(code, n)),
        live_count(lengths, n) == 0,
{
    lemma_zero_one_partition(lengths, lengths.len());
    assert forall|s: int| 0 <= s < lengths.len() implies cw(lengths, s).is_none() by {}
    lemma_remaining_small(lengths, n, lengths.len());
    let unit = pow2((MAX_CODE_BITS - n) as nat);
    lemma_pow2_positive((MAX_CODE_BITS - n) as nat);
    vstd::arithmetic::div_mod::lemma_basic_div((unit - 1) as int, unit as int);
}

/// The arithmetic interval tested after a failed lookup denotes exactly a
/// proper canonical prefix for all three admissible code shapes.
pub proof fn lemma_admissible_live_interval(lengths: Seq<u8>, kind: CodeKind, single_accept: bool,
    n: nat, code: nat)
    requires admissible(lengths, kind, single_accept), n <= MAX_CODE_BITS, code < pow2(n),
    ensures proper_live(lengths, msb_bits(code, n)) ==
        (next_code(lengths, n) + bl_count(lengths, n) <= code
            && code < next_code(lengths, n) + bl_count(lengths, n) + live_count(lengths, n)),
{
    lemma_admissible_shapes(lengths, kind, single_accept);
    if Kraft(lengths) == CODE_SPACE {
        lemma_complete_live_count(lengths, n); lemma_complete_proper_live(lengths, n, code);
    } else if single_one(lengths) {
        lemma_single_live(lengths, n, code);
    } else { lemma_empty_live(lengths, n, code); }
}

pub proof fn lemma_live_without_exact(lengths: Seq<u8>, bits: Seq<bool>)
    requires lengths_ok(lengths), forall|s: int| !exact(lengths, bits, s),
    ensures live(lengths, bits) == proper_live(lengths, bits),
{
    if live(lengths, bits) {
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && prefix(bits, cw(lengths, s).unwrap());
        lemma_msb_bits_len(cw_value(lengths, s), lengths[s] as nat);
        if bits.len() == lengths[s] {
            assert(bits =~= cw(lengths, s).unwrap());
            assert(exact(lengths, bits, s));
            assert(false);
        }
        assert(proper_live(lengths, bits));
    }
}

pub proof fn lemma_numeric_exact(lengths: Seq<u8>, n: nat, code: nat)
    requires lengths_ok(lengths), Kraft(lengths) <= CODE_SPACE,
        1 <= n <= MAX_CODE_BITS, code < pow2(n),
    ensures (exists|s: int| exact(lengths, msb_bits(code, n), s))
        == (next_code(lengths, n) <= code < next_code(lengths, n) + bl_count(lengths, n)),
{
    lemma_msb_bits_len(code, n);
    if exists|s: int| exact(lengths, msb_bits(code, n), s) {
        let s = choose|s: int| exact(lengths, msb_bits(code, n), s);
        lemma_msb_bits_len(cw_value(lengths, s), lengths[s] as nat);
        assert(lengths[s] == n);
        lemma_cw_value_bound(lengths, s);
        lemma_msb_injective(code, cw_value(lengths, s), n); lemma_rank_bound(lengths, s);
    } else if next_code(lengths, n) <= code < next_code(lengths, n) + bl_count(lengths, n) {
        lemma_rank_exists(lengths, n, lengths.len(), (code - next_code(lengths, n)) as nat);
        let s = choose|s: int| 0 <= s < lengths.len() && lengths[s] == n
            && count_prefix(lengths, n, s as nat) == code - next_code(lengths, n);
        assert(exact(lengths, msb_bits(code, n), s));
    }
}

/// D4 after a failed lookup: the executable subtraction is nonnegative, and
/// its dead-prefix test means no independent canonical word extends the bits.
pub proof fn lemma_admissible_dead_prefix(lengths: Seq<u8>, kind: CodeKind, single_accept: bool,
    n: nat, code: nat)
    requires admissible(lengths, kind, single_accept), 1 <= n <= MAX_CODE_BITS,
        code < pow2(n), forall|s: int| !exact(lengths, msb_bits(code, n), s),
    ensures
        code < next_code(lengths, n) || code >= next_code(lengths, n) + bl_count(lengths, n),
        !live(lengths, msb_bits(code, n)) == (code < next_code(lengths, n)
            || code - next_code(lengths, n) - bl_count(lengths, n) >= live_count(lengths, n)),
{
    lemma_numeric_exact(lengths, n, code);
    lemma_live_without_exact(lengths, msb_bits(code, n));
    lemma_admissible_live_interval(lengths, kind, single_accept, n, code);
}

/// Local MDec error precedence, before another input bit is requested.
/// A dead prefix remains InvalidSymbol even when the input has also ended.
pub proof fn lemma_decode_prefix_errors(lengths: Seq<u8>, z: Seq<u8>, start: nat, bits: Seq<bool>)
    requires lengths_ok(lengths), bits.len() <= MAX_CODE_BITS,
        forall|s: int| !exact(lengths, bits, s),
    ensures
        !live(lengths, bits) ==> decode_prefix(lengths, z, start, bits)
            == (Decoded::InvalidSymbol { position: start + bits.len() }),
        live(lengths, bits) && start + bits.len() >= 8 * z.len() ==>
            decode_prefix(lengths, z, start, bits)
                == (Decoded::TruncatedZlib { position: start + bits.len() }),
{
    reveal(decode_prefix);
}

/// Little-endian value of n consecutive wire bits, independent of any cache.
pub open spec fn field_value(z: Seq<u8>, start: nat, n: nat) -> nat
    decreases n,
{
    if n == 0 { 0 }
    else { field_value(z, start, (n - 1) as nat)
        + if bit(z, (start + n - 1) as nat) { pow2((n - 1) as nat) } else { 0nat } }
}

pub proof fn lemma_field_bound(z: Seq<u8>, start: nat, n: nat)
    ensures field_value(z, start, n) < pow2(n),
    decreases n,
{
    if n > 0 { lemma_field_bound(z, start, (n - 1) as nat); }
}

pub proof fn lemma_field_one(z: Seq<u8>, start: nat)
    ensures field_value(z, start, 1) == if bit(z, start) { 1nat } else { 0nat },
{
    reveal_with_fuel(field_value, 2);
}

pub proof fn lemma_msb_push(code: nat, n: nat, next: nat)
    requires next <= 1,
    ensures msb_bits(2 * code + next, n + 1) == msb_bits(code, n).push(next == 1),
{
    vstd::arithmetic::div_mod::lemma_div_multiples_vanish_fancy(code as int, next as int, 2);
    vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(code as int, next as int, 2);
}

pub proof fn lemma_initial_prefix(lengths: Seq<u8>, kind: CodeKind, single_accept: bool)
    requires admissible(lengths, kind, single_accept),
    ensures (forall|s: int| !exact(lengths, Seq::empty(), s)),
        live(lengths, Seq::empty()) == !empty_code(lengths),
{
    assert forall|s: int| !exact(lengths, Seq::empty(), s) by {
        if 0 <= s < lengths.len() && cw(lengths, s).is_some() {
            lemma_msb_bits_len(cw_value(lengths, s), lengths[s] as nat);
        }
    }
    lemma_admissible_shapes(lengths, kind, single_accept);
    if Kraft(lengths) == CODE_SPACE {
        lemma_complete_extension(lengths, 0, 0);
        let s = choose|s: int| 0 <= s < lengths.len() && cw(lengths, s).is_some()
            && lengths[s] >= 0 && prefix(msb_bits(0, 0), cw(lengths, s).unwrap());
        assert(live(lengths, Seq::empty()));
        if empty_code(lengths) { lemma_empty_live(lengths, 0, 0); assert(false); }
    } else if single_one(lengths) {
        lemma_single_live(lengths, 0, 0);
        assert(!empty_code(lengths));
    } else { lemma_empty_live(lengths, 0, 0); }
}

pub proof fn lemma_decode_prefix_step(lengths: Seq<u8>, z: Seq<u8>, start: nat, bits: Seq<bool>)
    requires bits.len() < MAX_CODE_BITS, live(lengths, bits),
        forall|s: int| !exact(lengths, bits, s), start + bits.len() < 8 * z.len(),
    ensures decode_prefix(lengths, z, start, bits)
        == decode_prefix(lengths, z, start, bits.push(bit(z, start + bits.len()))),
{
    reveal(decode_prefix);
}

pub proof fn lemma_decode_prefix_hit(lengths: Seq<u8>, z: Seq<u8>, start: nat, bits: Seq<bool>, s: int)
    requires lengths_ok(lengths), Kraft(lengths) <= CODE_SPACE, bits.len() <= MAX_CODE_BITS,
        exact(lengths, bits, s),
    ensures decode_prefix(lengths, z, start, bits)
        == (Decoded::Symbol { symbol: s as nat, position: start + bits.len() }),
{
    reveal(decode_prefix);
    let t = choose|t: int| exact(lengths, bits, t);
    lemma_exact_unique(lengths, bits, s, t);
}

} // verus!
