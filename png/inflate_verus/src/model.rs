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

/// Independent output effects. No executable output buffer or ring appears in
/// these definitions; every successful byte extends a mathematical sequence.
pub enum InflateFault {
    TruncatedZlib,
    InvalidZlibHeader,
    UnsupportedZlibMethod,
    ZlibPresetDictionary,
    InvalidBlockType,
    StoredLengthMismatch,
    InvalidCodeLengths,
    InvalidSymbol,
    DistanceTooFar,
    AdlerMismatch,
    TrailingZlibData,
}

pub enum OutputEffect {
    Next { out: Seq<u8> },
    Full { out: Seq<u8> },
    Error { error: InflateFault, out: Seq<u8> },
    OutsideDomain,
}

pub open spec fn effect_output(effect: OutputEffect) -> Seq<u8> {
    match effect {
        OutputEffect::Next { out } => out,
        OutputEffect::Full { out } => out,
        OutputEffect::Error { error: _, out } => out,
        OutputEffect::OutsideDomain => Seq::empty(),
    }
}

pub open spec fn MEmit(out: Seq<u8>, limit: nat, byte: u8) -> OutputEffect {
    if out.len() > limit { OutputEffect::OutsideDomain }
    else if out.len() == limit { OutputEffect::Full { out } }
    else { OutputEffect::Next { out: out.push(byte) } }
}

/// Resolve each next match byte against the sequence after all earlier bytes
/// of the same match were appended. This defines overlap without a ring model.
pub open spec fn copy_distance(out: Seq<u8>, distance: nat, count: nat) -> Seq<u8>
    decreases count,
{
    if count == 0 || distance == 0 || distance > out.len() { out }
    else {
        let previous = copy_distance(out, distance, (count - 1) as nat);
        previous.push(previous[(previous.len() - distance) as int])
    }
}

pub open spec fn MCopy(out: Seq<u8>, limit: nat, window: nat, distance: nat, count: nat) -> OutputEffect {
    if out.len() > limit || distance == 0 || count < 3 || count > crate::limits::LEN_MAX
        || window == 0 || window > crate::limits::WINDOW_MAX {
        OutputEffect::OutsideDomain
    } else if distance > out.len() || distance > window {
        OutputEffect::Error { error: InflateFault::DistanceTooFar, out }
    } else {
        let fit = if count <= limit - out.len() { count } else { (limit - out.len()) as nat };
        let extended = copy_distance(out, distance, fit);
        if fit < count { OutputEffect::Full { out: extended } }
        else { OutputEffect::Next { out: extended } }
    }
}

/// Stored payload effects carry an input position only when it is observable
/// in a normal continuation or format error; Full needs only the output prefix.
pub enum StoredEffect {
    End { out: Seq<u8>, position: nat },
    Full { out: Seq<u8> },
    Error { error: InflateFault, out: Seq<u8>, position: nat },
    OutsideDomain,
}

/// Aligned stored payload, after LEN/NLEN have already been checked. Availability
/// precedes the output-limit test for every byte, including the byte beyond L.
pub open spec fn MStoredPayload(z: Seq<u8>, cursor: nat, out: Seq<u8>, limit: nat, count: nat) -> StoredEffect
    decreases count,
{
    if cursor > z.len() || out.len() > limit || count > u16::MAX {
        StoredEffect::OutsideDomain
    } else if count == 0 {
        StoredEffect::End { out, position: 8 * cursor }
    } else if cursor == z.len() {
        StoredEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * cursor }
    } else if out.len() == limit {
        StoredEffect::Full { out }
    } else {
        MStoredPayload(z, cursor + 1, out.push(z[cursor as int]), limit, (count - 1) as nat)
    }
}

pub proof fn lemma_copy_distance(out: Seq<u8>, distance: nat, count: nat)
    requires 1 <= distance <= out.len(),
    ensures
        copy_distance(out, distance, count).len() == out.len() + count,
        copy_distance(out, distance, count).take(out.len() as int) == out,
        forall|j: int| 0 <= j < count ==>
            #[trigger] copy_distance(out, distance, count)[out.len() + j]
                == copy_distance(out, distance, count)[out.len() + j - distance],
    decreases count,
{
    if count > 0 {
        lemma_copy_distance(out, distance, (count - 1) as nat);
        let previous = copy_distance(out, distance, (count - 1) as nat);
        let extended = copy_distance(out, distance, count);
        assert(previous.len() >= distance);
        assert(extended.take(out.len() as int) =~= out);
        assert forall|j: int| 0 <= j < count implies
            #[trigger] extended[out.len() + j] == extended[out.len() + j - distance] by {
            if j < count - 1 {
                assert(0 <= out.len() + j - distance < previous.len());
                assert(previous[out.len() + j] == previous[out.len() + j - distance]);
            } else {
                assert(out.len() + j == previous.len());
            }
        }
    }
}

pub proof fn lemma_copy_prefix(out: Seq<u8>, distance: nat, a: nat, b: nat)
    requires 1 <= distance <= out.len(), a <= b,
    ensures copy_distance(out, distance, b).take((out.len() + a) as int) == copy_distance(out, distance, a),
    decreases b - a,
{
    lemma_copy_distance(out, distance, b);
    if a < b {
        lemma_copy_prefix(out, distance, a, (b - 1) as nat);
        lemma_copy_distance(out, distance, (b - 1) as nat);
    }
}

pub proof fn lemma_MCopy_shape(out: Seq<u8>, limit: nat, window: nat, distance: nat, count: nat)
    requires out.len() <= limit, 1 <= distance, 3 <= count <= crate::limits::LEN_MAX,
        1 <= window <= crate::limits::WINDOW_MAX,
    ensures match MCopy(out, limit, window, distance, count) {
        OutputEffect::Next { out: extended } => distance <= out.len() && distance <= window
            && extended == copy_distance(out, distance, count)
            && extended.len() == out.len() + count && extended.len() <= limit
            && extended.take(out.len() as int) == out,
        OutputEffect::Full { out: extended } => distance <= out.len() && distance <= window
            && out.len() + count > limit && extended.len() == limit
            && extended == copy_distance(out, distance, (limit - out.len()) as nat)
            && extended.take(out.len() as int) == out,
        OutputEffect::Error { error, out: unchanged } => error == InflateFault::DistanceTooFar
            && (distance > out.len() || distance > window) && unchanged == out,
        OutputEffect::OutsideDomain => false,
    },
{
    if distance <= out.len() && distance <= window {
        let fit = if count <= limit - out.len() { count } else { (limit - out.len()) as nat };
        lemma_copy_distance(out, distance, fit);
    }
}

pub proof fn lemma_push_subrange(out: Seq<u8>, z: Seq<u8>, a: nat, b: nat)
    requires a < b <= z.len(),
    ensures out.push(z[a as int]) + z.subrange((a + 1) as int, b as int)
        == out + z.subrange(a as int, b as int),
{
    assert(out.push(z[a as int]) + z.subrange((a + 1) as int, b as int)
        =~= out + z.subrange(a as int, b as int));
}

/// D5's exact stream-order output prefix and first limit/truncation outcome.
pub proof fn lemma_stored_payload(z: Seq<u8>, cursor: nat, out: Seq<u8>, limit: nat, count: nat)
    requires cursor <= z.len(), out.len() <= limit, count <= u16::MAX,
    ensures match MStoredPayload(z, cursor, out, limit, count) {
        StoredEffect::End { out: extended, position } => cursor + count <= z.len()
            && out.len() + count <= limit && position == 8 * (cursor + count)
            && extended == out + z.subrange(cursor as int, (cursor + count) as int),
        StoredEffect::Full { out: extended } => limit - out.len() < count
            && limit - out.len() < z.len() - cursor
            && extended == out + z.subrange(cursor as int, (cursor + limit - out.len()) as int)
            && extended.len() == limit,
        StoredEffect::Error { error, out: extended, position } => error == InflateFault::TruncatedZlib
            && z.len() - cursor < count && out.len() + z.len() - cursor <= limit
            && position == 8 * z.len() && extended == out + z.subrange(cursor as int, z.len() as int),
        StoredEffect::OutsideDomain => false,
    },
    decreases count,
{
    if count > 0 && cursor < z.len() && out.len() < limit {
        lemma_stored_payload(z, cursor + 1, out.push(z[cursor as int]), limit, (count - 1) as nat);
        let effect = MStoredPayload(z, cursor + 1, out.push(z[cursor as int]), limit, (count - 1) as nat);
        match effect {
            StoredEffect::End { out: extended, position } => {
                lemma_push_subrange(out, z, cursor, cursor + count);
            },
            StoredEffect::Full { out: extended } => {
                lemma_push_subrange(out, z, cursor, (cursor + limit - out.len()) as nat);
            },
            StoredEffect::Error { error, out: extended, position } => {
                lemma_push_subrange(out, z, cursor, z.len());
            },
            StoredEffect::OutsideDomain => {},
        }
    } else {
        assert(z.subrange(cursor as int, cursor as int) =~= Seq::<u8>::empty());
        assert(out + Seq::<u8>::empty() =~= out);
    }
}

/// Mathematical Adler sums: no deferred-modulo state or executable accumulator.
pub open spec fn sum_bytes(out: Seq<u8>) -> nat
    decreases out.len(),
{
    if out.len() == 0 { 0 }
    else { sum_bytes(out.drop_last()) + out.last() as nat }
}

pub open spec fn sum_prefixes(out: Seq<u8>) -> nat
    decreases out.len(),
{
    if out.len() == 0 { 0 }
    else { sum_prefixes(out.drop_last()) + 1 + sum_bytes(out) }
}

pub open spec fn Adler(out: Seq<u8>) -> nat {
    (sum_prefixes(out) % (crate::limits::ADLER_MOD as nat)) * pow2(16)
        + (1 + sum_bytes(out)) % (crate::limits::ADLER_MOD as nat)
}

pub proof fn lemma_adler_sums_push(out: Seq<u8>, byte: u8)
    ensures sum_bytes(out.push(byte)) == sum_bytes(out) + byte,
        sum_prefixes(out.push(byte)) == sum_prefixes(out) + 1 + sum_bytes(out.push(byte)),
{
    assert(out.push(byte).drop_last() =~= out);
}

pub proof fn lemma_stored_unfold(z: Seq<u8>, cursor: nat, out: Seq<u8>, limit: nat, count: nat)
    requires cursor <= z.len(), out.len() <= limit, count <= u16::MAX,
    ensures
        count == 0 ==> MStoredPayload(z, cursor, out, limit, count)
            == (StoredEffect::End { out, position: 8 * cursor }),
        count > 0 && cursor == z.len() ==> MStoredPayload(z, cursor, out, limit, count)
            == (StoredEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * cursor }),
        count > 0 && cursor < z.len() && out.len() == limit ==>
            MStoredPayload(z, cursor, out, limit, count) == (StoredEffect::Full { out }),
        count > 0 && cursor < z.len() && out.len() < limit ==>
            MStoredPayload(z, cursor, out, limit, count)
                == MStoredPayload(z, cursor + 1, out.push(z[cursor as int]), limit, (count - 1) as nat),
{
    reveal(MStoredPayload);
}

/// RFC 1951 fixed code lengths, defined without executable table state.
pub open spec fn fixed_lit_lengths() -> Seq<u8> {
    Seq::new(FIXED_LIT_COUNT as nat, |s: int| if s < 144 { 8u8 }
        else if s < 256 { 9u8 } else if s < 280 { 7u8 } else { 8u8 })
}

pub open spec fn fixed_dist_lengths() -> Seq<u8> {
    Seq::new(crate::limits::FIXED_DIST_COUNT as nat, |s: int| 5u8)
}

pub open spec fn fixed_lit_mass(k: nat) -> nat {
    let a = pow2((MAX_CODE_BITS - 8) as nat);
    let b = pow2((MAX_CODE_BITS - 9) as nat);
    let c = pow2((MAX_CODE_BITS - 7) as nat);
    if k <= 144 { k * a }
    else if k <= 256 { 144 * a + ((k - 144) as nat) * b }
    else if k <= 280 { 144 * a + 112 * b + ((k - 256) as nat) * c }
    else { 144 * a + 112 * b + 24 * c + ((k - 280) as nat) * a }
}

pub proof fn lemma_fixed_lit_mass(k: nat)
    requires k <= FIXED_LIT_COUNT,
    ensures kraft_prefix(fixed_lit_lengths(), k) == fixed_lit_mass(k),
    decreases k,
{
    assert(pow2((MAX_CODE_BITS - 8) as nat) == 128) by (compute);
    assert(pow2((MAX_CODE_BITS - 9) as nat) == 64) by (compute);
    assert(pow2((MAX_CODE_BITS - 7) as nat) == 256) by (compute);
    if k > 0 { lemma_fixed_lit_mass((k - 1) as nat); }
}

pub proof fn lemma_fixed_dist_mass(k: nat)
    requires k <= crate::limits::FIXED_DIST_COUNT,
    ensures kraft_prefix(fixed_dist_lengths(), k) == k * pow2((MAX_CODE_BITS - 5) as nat),
    decreases k,
{
    assert(pow2((MAX_CODE_BITS - 5) as nat) == 1024) by (compute);
    if k > 0 { lemma_fixed_dist_mass((k - 1) as nat); }
}

pub proof fn lemma_fixed_codes(single_accept: bool)
    ensures lengths_ok(fixed_lit_lengths()), lengths_ok(fixed_dist_lengths()),
        Kraft(fixed_lit_lengths()) == CODE_SPACE, Kraft(fixed_dist_lengths()) == CODE_SPACE,
        admissible(fixed_lit_lengths(), CodeKind::LitLen, single_accept),
        admissible(fixed_dist_lengths(), CodeKind::Dist, single_accept),
{
    lemma_fixed_lit_mass(FIXED_LIT_COUNT as nat);
    lemma_fixed_dist_mass(crate::limits::FIXED_DIST_COUNT as nat);
    assert(fixed_lit_mass(FIXED_LIT_COUNT as nat) == CODE_SPACE) by (compute);
    assert(crate::limits::FIXED_DIST_COUNT * pow2((MAX_CODE_BITS - 5) as nat) == CODE_SPACE) by (compute);
    assert forall|s: int| 0 <= s < fixed_lit_lengths().len() implies
        fixed_lit_lengths()[s] <= MAX_CODE_BITS by {}
    assert forall|s: int| 0 <= s < fixed_dist_lengths().len() implies
        fixed_dist_lengths()[s] <= MAX_CODE_BITS by {}
}

/// Independently ordered dynamic-header parsing. These states contain only
/// length sequences and bit positions, never executable Huffman tables.
pub enum DynamicHeader {
    Tables { lit: Seq<u8>, dist: Seq<u8>, position: nat },
    Error { error: InflateFault, position: nat },
    OutsideDomain,
}

pub open spec fn code_length_order() -> Seq<int> {
    seq![16int, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]
}

pub open spec fn dynamic_sizes_ok(nlen: nat, ndist: nat) -> bool {
    257 <= nlen <= crate::limits::NLIT_MAX && 1 <= ndist <= crate::limits::NDIST_MAX
}

pub open spec fn length_entries_ok(lengths: Seq<u8>) -> bool {
    forall|s: int| 0 <= s < lengths.len() ==> lengths[s] <= MAX_CODE_BITS
}

pub open spec fn finish_dynamic(lengths: Seq<u8>, nlen: nat, ndist: nat, position: nat,
    single_accept: bool) -> DynamicHeader {
    if !dynamic_sizes_ok(nlen, ndist) || lengths.len() != nlen + ndist || !length_entries_ok(lengths) {
        DynamicHeader::OutsideDomain
    } else if lengths[256] == 0 {
        DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position }
    } else {
        let lit = lengths.take(nlen as int);
        let dist = lengths.skip(nlen as int);
        if !admissible(lit, CodeKind::LitLen, single_accept) {
            DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position }
        } else if !admissible(dist, CodeKind::Dist, single_accept) {
            DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position }
        } else { DynamicHeader::Tables { lit, dist, position } }
    }
}

pub open spec fn MDynamicLengths(z: Seq<u8>, position: nat, code: Seq<u8>, lengths: Seq<u8>,
    nlen: nat, ndist: nat, single_accept: bool) -> DynamicHeader
    decreases if lengths.len() <= nlen + ndist { nlen + ndist - lengths.len() } else { 0 },
{
    let total = nlen + ndist;
    if position > 8 * z.len() || !dynamic_sizes_ok(nlen, ndist) || lengths.len() > total
        || !length_entries_ok(lengths) || code.len() != crate::limits::NCL
        || !admissible(code, CodeKind::CodeLen, true) {
        DynamicHeader::OutsideDomain
    } else if lengths.len() == total {
        finish_dynamic(lengths, nlen, ndist, position, single_accept)
    } else {
        match MDec(code, z, position) {
            Decoded::InvalidSymbol { position: end } =>
                DynamicHeader::Error { error: InflateFault::InvalidSymbol, position: end },
            Decoded::TruncatedZlib { position: end } =>
                DynamicHeader::Error { error: InflateFault::TruncatedZlib, position: end },
            Decoded::OutsideDomain => DynamicHeader::OutsideDomain,
            Decoded::Symbol { symbol, position: after_code } => {
                if symbol < 16 {
                    MDynamicLengths(z, after_code, code, lengths.push(symbol as u8), nlen, ndist, single_accept)
                } else if symbol > 18 {
                    DynamicHeader::OutsideDomain
                } else if symbol == 16 && lengths.len() == 0 {
                    DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position: after_code }
                } else {
                    let width = if symbol == 16 { 2nat } else if symbol == 17 { 3nat } else { 7nat };
                    if after_code + width > 8 * z.len() {
                        DynamicHeader::Error { error: InflateFault::TruncatedZlib, position: 8 * z.len() }
                    } else {
                        let repeat = (if symbol == 18 { 11nat } else { 3nat }) + field_value(z, after_code, width);
                        let value = if symbol == 16 { lengths.last() } else { 0u8 };
                        if repeat > total - lengths.len() {
                            DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position: after_code + width }
                        } else {
                            MDynamicLengths(z, after_code + width, code,
                                lengths + Seq::new(repeat, |i: int| value), nlen, ndist, single_accept)
                        }
                    }
                }
            },
        }
    }
}

pub open spec fn MDynamicCL(z: Seq<u8>, position: nat, code: Seq<u8>, i: nat, ncode: nat,
    nlen: nat, ndist: nat, single_accept: bool) -> DynamicHeader
    decreases if i <= ncode { ncode - i } else { 0 },
{
    if position > 8 * z.len() || !dynamic_sizes_ok(nlen, ndist) || i > ncode
        || ncode < 4 || ncode > crate::limits::NCL || code.len() != crate::limits::NCL
        || !(forall|s: int| 0 <= s < code.len() ==> code[s] <= 7) {
        DynamicHeader::OutsideDomain
    } else if i == ncode {
        if !admissible(code, CodeKind::CodeLen, true) {
            DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position }
        } else { MDynamicLengths(z, position, code, Seq::empty(), nlen, ndist, single_accept) }
    } else if position + 3 > 8 * z.len() {
        DynamicHeader::Error { error: InflateFault::TruncatedZlib, position: 8 * z.len() }
    } else {
        MDynamicCL(z, position + 3, code.update(code_length_order()[i as int], field_value(z, position, 3) as u8),
            i + 1, ncode, nlen, ndist, single_accept)
    }
}

pub open spec fn MDynamic(z: Seq<u8>, position: nat, single_accept: bool) -> DynamicHeader {
    if position > 8 * z.len() { DynamicHeader::OutsideDomain }
    else if position + 14 > 8 * z.len() {
        DynamicHeader::Error { error: InflateFault::TruncatedZlib, position: 8 * z.len() }
    } else {
        let nlen = field_value(z, position, 5) + 257;
        let ndist = field_value(z, position + 5, 5) + 1;
        let ncode = field_value(z, position + 10, 4) + 4;
        if nlen > crate::limits::NLIT_MAX || ndist > crate::limits::NDIST_MAX {
            DynamicHeader::Error { error: InflateFault::InvalidCodeLengths, position: position + 14 }
        } else {
            MDynamicCL(z, position + 14, Seq::new(crate::limits::NCL as nat, |i: int| 0u8),
                0, ncode, nlen, ndist, single_accept)
        }
    }
}

pub proof fn lemma_dynamic_count_fields(z: Seq<u8>, position: nat)
    ensures field_value(z, position, 5) + 257 <= FIXED_LIT_COUNT,
        1 <= field_value(z, position + 5, 5) + 1 <= crate::limits::FIXED_DIST_COUNT,
        4 <= field_value(z, position + 10, 4) + 4 <= crate::limits::NCL,
{
    lemma_field_bound(z, position, 5);
    lemma_field_bound(z, position + 5, 5);
    lemma_field_bound(z, position + 10, 4);
    assert(pow2(5) == 32) by (compute);
    assert(pow2(4) == 16) by (compute);
}

pub proof fn lemma_finish_dynamic(lengths: Seq<u8>, nlen: nat, ndist: nat, position: nat, single_accept: bool)
    ensures match finish_dynamic(lengths, nlen, ndist, position, single_accept) {
        DynamicHeader::Tables { lit, dist, position: end } => dynamic_sizes_ok(nlen, ndist)
            && lit.len() == nlen && dist.len() == ndist
            && lengths.len() == nlen + ndist && lengths.len() <= crate::limits::DYNAMIC_LENGTHS
            && lit == lengths.take(nlen as int) && dist == lengths.skip(nlen as int)
            && lit[256] > 0 && end == position
            && admissible(lit, CodeKind::LitLen, single_accept)
            && admissible(dist, CodeKind::Dist, single_accept),
        DynamicHeader::Error { error, position: end } => error == InflateFault::InvalidCodeLengths && end == position,
        DynamicHeader::OutsideDomain => true,
    },
{}

/// L-HDR-BITS arithmetic, tied to the real header loop's construction marker.
/// Large repeats are symbol 18; all other entries obey 5*bits >= 2*entries.
pub proof fn lemma_header_coding_cost(large: nat, large_entries: nat, other_entries: nat,
    large_bits: nat, other_bits: nat)
    requires large_entries + other_entries >= 257 + 1,
        large_entries <= 138 * large, large_bits >= 8 * large,
        5 * other_bits >= 2 * other_entries,
    ensures large_bits + other_bits >= 16,
{
    if large >= 2 {} else {
        assert(other_entries >= 120);
        assert(other_bits >= 48);
    }
}

/// P10.2 stored block after BTYPE, including padding and ordered LEN/NLEN reads.
/// The complement test is mathematical addition of the two little-endian words.
pub open spec fn MStoredBlock(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat) -> StoredEffect {
    let cursor = (position + 7) / 8;
    if position > 8 * z.len() || out.len() > limit { StoredEffect::OutsideDomain }
    else if cursor + 4 > z.len() {
        StoredEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * z.len() }
    } else {
        let len = z[cursor as int] as nat + 256 * z[(cursor + 1) as int] as nat;
        let nlen = z[(cursor + 2) as int] as nat + 256 * z[(cursor + 3) as int] as nat;
        if len + nlen != u16::MAX {
            StoredEffect::Error { error: InflateFault::StoredLengthMismatch, out, position: 8 * (cursor + 4) }
        } else { MStoredPayload(z, cursor + 4, out, limit, len) }
    }
}

/// RFC 1951 length and distance groups, defined arithmetically rather than by
/// the executable lookup arrays. Outside the decoded symbol domains they are 0.
pub open spec fn length_extra(symbol: nat) -> nat {
    if symbol < 265 || symbol >= 285 { 0 } else { (symbol - 261) as nat / 4 }
}
pub open spec fn length_base(symbol: nat) -> nat {
    if symbol < 257 || symbol > 285 { 0 }
    else if symbol < 265 { (symbol - 254) as nat }
    else if symbol == 285 { crate::limits::LEN_MAX as nat }
    else { let e = length_extra(symbol);
        3 + pow2(e + 2) + (symbol - 261 - 4 * e) as nat * pow2(e) }
}
pub open spec fn distance_extra(symbol: nat) -> nat {
    if symbol < 4 || symbol >= crate::limits::NDIST_MAX { 0 } else { (symbol / 2 - 1) as nat }
}
pub open spec fn distance_base(symbol: nat) -> nat {
    if symbol >= crate::limits::NDIST_MAX { 0 }
    else if symbol < 4 { symbol + 1 }
    else { let e = distance_extra(symbol); 1 + pow2(e + 1) + (symbol % 2) * pow2(e) }
}

pub enum SymbolEffect {
    Next { out: Seq<u8>, position: nat },
    End { out: Seq<u8>, position: nat },
    Full { out: Seq<u8> },
    Error { error: InflateFault, out: Seq<u8>, position: nat },
    OutsideDomain,
}

/// One match after a literal/length symbol: extras, distance code, extras, then
/// the output/window check in MCopy, before any emission or limit decision.
pub open spec fn MMatch(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat,
    window: nat, dist: Seq<u8>, symbol: nat) -> SymbolEffect {
    let le = length_extra(symbol);
    if symbol < 257 || symbol > 285 || position > 8 * z.len()
        || out.len() > limit || window == 0 || window > crate::limits::WINDOW_MAX {
        SymbolEffect::OutsideDomain
    } else if position + le > 8 * z.len() {
        SymbolEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * z.len() }
    } else {
        let count = length_base(symbol) + field_value(z, position, le);
        match MDec(dist, z, position + le) {
            Decoded::OutsideDomain => SymbolEffect::OutsideDomain,
            Decoded::InvalidSymbol { position: end } => SymbolEffect::Error { error: InflateFault::InvalidSymbol, out, position: end },
            Decoded::TruncatedZlib { position: end } => SymbolEffect::Error { error: InflateFault::TruncatedZlib, out, position: end },
            Decoded::Symbol { symbol: ds, position: after_code } => {
                if ds >= crate::limits::NDIST_MAX {
                    SymbolEffect::Error { error: InflateFault::InvalidSymbol, out, position: after_code }
                } else {
                    let de = distance_extra(ds);
                    if after_code + de > 8 * z.len() {
                        SymbolEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * z.len() }
                    } else {
                        let distance = distance_base(ds) + field_value(z, after_code, de);
                        match MCopy(out, limit, window, distance, count) {
                            OutputEffect::Next { out: extended } => SymbolEffect::Next { out: extended, position: after_code + de },
                            OutputEffect::Full { out: extended } => SymbolEffect::Full { out: extended },
                            OutputEffect::Error { error, out: unchanged } => SymbolEffect::Error { error, out: unchanged, position: after_code + de },
                            OutputEffect::OutsideDomain => SymbolEffect::OutsideDomain,
                        }
                    }
                }
            },
        }
    }
}

pub open spec fn MCompressedStep(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat,
    window: nat, lit: Seq<u8>, dist: Seq<u8>) -> SymbolEffect {
    if position > 8 * z.len() || out.len() > limit || window == 0 || window > crate::limits::WINDOW_MAX {
        SymbolEffect::OutsideDomain
    } else { match MDec(lit, z, position) {
        Decoded::OutsideDomain => SymbolEffect::OutsideDomain,
        Decoded::InvalidSymbol { position: end } => SymbolEffect::Error { error: InflateFault::InvalidSymbol, out, position: end },
        Decoded::TruncatedZlib { position: end } => SymbolEffect::Error { error: InflateFault::TruncatedZlib, out, position: end },
        Decoded::Symbol { symbol, position: end } => {
            if symbol < 256 { match MEmit(out, limit, symbol as u8) {
                OutputEffect::Next { out: extended } => SymbolEffect::Next { out: extended, position: end },
                OutputEffect::Full { out: unchanged } => SymbolEffect::Full { out: unchanged },
                _ => SymbolEffect::OutsideDomain,
            } } else if symbol == 256 { SymbolEffect::End { out, position: end } }
            else if symbol > 285 { SymbolEffect::Error { error: InflateFault::InvalidSymbol, out, position: end } }
            else { MMatch(z, end, out, limit, window, dist, symbol) }
        },
    } }
}

pub open spec fn MCompressed(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat,
    window: nat, lit: Seq<u8>, dist: Seq<u8>) -> StoredEffect
    decreases 8 * z.len() - position,
{
    match MCompressedStep(z, position, out, limit, window, lit, dist) {
        SymbolEffect::Next { out: extended, position: end } => {
            if end <= position || end > 8 * z.len() { StoredEffect::OutsideDomain }
            else { MCompressed(z, end, extended, limit, window, lit, dist) }
        },
        SymbolEffect::End { out: extended, position: end } => StoredEffect::End { out: extended, position: end },
        SymbolEffect::Full { out: extended } => StoredEffect::Full { out: extended },
        SymbolEffect::Error { error, out: extended, position: end } => StoredEffect::Error { error, out: extended, position: end },
        SymbolEffect::OutsideDomain => StoredEffect::OutsideDomain,
    }
}

/// One complete block, including its header and independent table selection.
/// The bool is BFINAL; only a successful End continuation consults it.
pub open spec fn MOneBlock(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat,
    window: nat, single_accept: bool) -> (bool, StoredEffect) {
    if position > 8 * z.len() || out.len() > limit || window == 0 || window > crate::limits::WINDOW_MAX {
        (false, StoredEffect::OutsideDomain)
    } else if position + 3 > 8 * z.len() {
        (false, StoredEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * z.len() })
    } else {
        let last = field_value(z, position, 1) != 0;
        let typ = field_value(z, position + 1, 2);
        let effect = if typ == 0 { MStoredBlock(z, position + 3, out, limit) }
        else if typ == 1 { MCompressed(z, position + 3, out, limit, window, fixed_lit_lengths(), fixed_dist_lengths()) }
        else if typ == 2 { match MDynamic(z, position + 3, single_accept) {
            DynamicHeader::Tables { lit, dist, position: end } => MCompressed(z, end, out, limit, window, lit, dist),
            DynamicHeader::Error { error, position: end } => StoredEffect::Error { error, out, position: end },
            DynamicHeader::OutsideDomain => StoredEffect::OutsideDomain,
        } } else { StoredEffect::Error { error: InflateFault::InvalidBlockType, out, position: position + 3 } };
        (last, effect)
    }
}

/// P10 block loop: every continuation advances the mathematical bit position.
pub open spec fn MBlk(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat,
    window: nat, single_accept: bool) -> StoredEffect
    decreases 8 * z.len() - position,
{
    let (last, effect) = MOneBlock(z, position, out, limit, window, single_accept);
    match effect {
        StoredEffect::End { out: extended, position: end } => {
            if end <= position || end > 8 * z.len() { StoredEffect::OutsideDomain }
            else if last { effect }
            else { MBlk(z, end, extended, limit, window, single_accept) }
        },
        _ => effect,
    }
}

/// P9/P10 admissible match distance for the two implemented window policies.
pub open spec fn window_size(cinfo: nat, enforce: bool) -> nat {
    if cinfo > 7 { 0 }
    else if enforce { pow2(cinfo + 8) }
    else { crate::limits::WINDOW_MAX as nat }
}

/// Mathematical big-endian prefix value, without a byte buffer or machine shifts.
pub open spec fn be_value(z: Seq<u8>, cursor: nat, count: nat) -> nat
    decreases count,
{
    if count == 0 || cursor + count > z.len() { 0 }
    else { 256 * be_value(z, cursor, (count - 1) as nat) + z[(cursor + count - 1) as int] as nat }
}

/// P9 steps 5–6 after a final DEFLATE block: padding, checksum, then tail.
pub open spec fn MFinish(z: Seq<u8>, position: nat, out: Seq<u8>, limit: nat,
    ignore_tail: bool) -> StoredEffect {
    let cursor = (position + 7) / 8;
    if position > 8 * z.len() || out.len() > limit { StoredEffect::OutsideDomain }
    else if cursor + 4 > z.len() {
        StoredEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * z.len() }
    } else if be_value(z, cursor, 4) != Adler(out) {
        StoredEffect::Error { error: InflateFault::AdlerMismatch, out, position: 8 * (cursor + 4) }
    } else if ignore_tail && out.len() == limit {
        StoredEffect::End { out, position: 8 * (cursor + 4) }
    } else if cursor + 4 < z.len() {
        StoredEffect::Error { error: InflateFault::TrailingZlibData, out, position: 8 * (cursor + 5) }
    } else { StoredEffect::End { out, position: 8 * (cursor + 4) } }
}

/// P9 in its operational check order. Only mathematical bytes, positions,
/// canonical codewords and the evolving sequence output enter this envelope.
pub open spec fn MEnvelope(z: Seq<u8>, limit: nat, enforce_window: bool,
    ignore_tail: bool) -> StoredEffect {
    let out = Seq::<u8>::empty();
    if z.len() < 2 {
        StoredEffect::Error { error: InflateFault::TruncatedZlib, out, position: 8 * z.len() }
    } else if (256 * z[0] as nat + z[1] as nat) % 31 != 0 {
        StoredEffect::Error { error: InflateFault::InvalidZlibHeader, out, position: 16 }
    } else if z[0] as nat % 16 != 8 || z[0] as nat / 16 > 7 {
        StoredEffect::Error { error: InflateFault::UnsupportedZlibMethod, out, position: 16 }
    } else if (z[1] as nat / 32) % 2 != 0 {
        StoredEffect::Error { error: InflateFault::ZlibPresetDictionary, out, position: 16 }
    } else { match MBlk(z, 16, out, limit, window_size(z[0] as nat / 16, enforce_window), true) {
        StoredEffect::End { out: extended, position: end } => MFinish(z, end, extended, limit, ignore_tail),
        effect => effect,
    } }
}

pub enum InflateStatus { Done, Full, Error(InflateFault), OutsideDomain }
pub struct Inflated { pub status: InflateStatus, pub out: Seq<u8>, pub r2: bool }

/// The single public inflate model for the implemented Strict/RB field domain.
/// The remaining three option fields each have one inhabitant; R2 is false.
pub open spec fn Inflate(z: Seq<u8>, limit: nat, enforce_window: bool,
    ignore_tail: bool) -> Inflated {
    match MEnvelope(z, limit, enforce_window, ignore_tail) {
        StoredEffect::End { out, .. } => Inflated { status: InflateStatus::Done, out, r2: false },
        StoredEffect::Full { out } => Inflated { status: InflateStatus::Full, out, r2: false },
        StoredEffect::Error { error, out, .. } => Inflated { status: InflateStatus::Error(error), out, r2: false },
        StoredEffect::OutsideDomain => Inflated { status: InflateStatus::OutsideDomain, out: Seq::empty(), r2: false },
    }
}

} // verus!
