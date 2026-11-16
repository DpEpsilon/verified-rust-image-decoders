//! M4a: G11 interlace. The executable `interlace_row` (the shape of
//! `gif_verus/src/compose.rs`), its equality with the model's `IR`, the
//! bijection (T5), and the arithmetic lemmas THEOREMS' `interlace_pass_sizes`
//! and `interlace_bijection` name.
//!
//! The proof of the bijection goes through an explicit inverse `back(y)`: the
//! residue class of an image row below `h` (multiples of 8, `≡ 4 mod 8`,
//! `≡ 2 mod 4`, odd) names the pass that draws it, and `back` returns the stream
//! row. `back`'s first branch also shows in-range and injectivity, because
//! `back(iv(r)) == r` on every pass.

use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::arithmetic::div_mod::{
    lemma_basic_div, lemma_div_by_multiple, lemma_div_is_ordered_by_denominator,
    lemma_div_multiples_vanish, lemma_div_plus_one, lemma_fundamental_div_mod,
    lemma_fundamental_div_mod_converse, lemma_mod_bound, lemma_mod_mod, lemma_mod_multiples_basic,
    lemma_mod_multiples_vanish, lemma_small_mod,
};
#[cfg(verus_only)]
use vstd::arithmetic::mul::{
    lemma_mul_equality_converse, lemma_mul_inequality, lemma_mul_is_associative,
    lemma_mul_is_commutative, lemma_mul_is_distributive_add,
};
#[cfg(verus_only)]
use crate::model::*;

verus! {

// ---------------------------------------------------------------------------
// The pass arithmetic, in the executable's form.
// ---------------------------------------------------------------------------

/// `⌈h/8⌉` for `h >= 1`: pass 0's length (rows `0, 8, 16, …` below `h`).
pub open spec fn n0(h: int) -> int {
    if h > 0 {
        (h - 1) / 8 + 1
    } else {
        0
    }
}

/// Pass 1's length (rows `4, 12, …`), 0 when `h <= 4`.
pub open spec fn n1(h: int) -> int {
    if h > 4 {
        (h - 5) / 8 + 1
    } else {
        0
    }
}

/// Pass 2's length (rows `2, 6, 10, …`), 0 when `h <= 2`.
pub open spec fn n2(h: int) -> int {
    if h > 2 {
        (h - 3) / 4 + 1
    } else {
        0
    }
}

/// Pass 3's length as the executable computes it: every row the first three
/// passes leave (`h/2` for `h >= 1`, the count of odd rows).
pub open spec fn n3(h: int) -> int {
    h - n0(h) - n1(h) - n2(h)
}

/// The three pass lengths as the executable's addition uses them, in the
/// unguarded `⌈·⌉` form (each branch folds in: for `1 <= h <= 4`, `(h + 3)/8` is
/// already 0, and for `1 <= h <= 2` so is `(h + 1)/4`).
pub open spec fn u0(h: int) -> int {
    (h + 7) / 8
}

pub open spec fn u1(h: int) -> int {
    (h + 3) / 8
}

pub open spec fn u2(h: int) -> int {
    (h + 1) / 4
}

pub open spec fn u3(h: int) -> int {
    h / 2
}

/// The rows before pass `k`; `pass_prefix(4, h) == h` is `pass_sum`.
pub open spec fn pass_prefix(k: int, h: int) -> int {
    if k <= 0 {
        0
    } else if k == 1 {
        n0(h)
    } else if k == 2 {
        n0(h) + n1(h)
    } else if k == 3 {
        n0(h) + n1(h) + n2(h)
    } else {
        h
    }
}

/// The executable's semantics as a specification: `iv(r, h)` is the image row of
/// stream row `r`, peeling pass lengths off `r` exactly as `interlace_row` does.
pub open spec fn iv_at(r: int, h: int, k: int) -> int
    decreases (4 - k) as nat,
{
    if k >= 3 {
        pass_start(3) + r * pass_step(3)
    } else if r < nk(k, h) {
        pass_start(k) + r * pass_step(k)
    } else {
        iv_at(r - nk(k, h), h, k + 1)
    }
}

/// Pass `k`'s length, in the executable's dispatch form.
pub open spec fn nk(k: int, h: int) -> int {
    if k == 0 {
        n0(h)
    } else if k == 1 {
        n1(h)
    } else if k == 2 {
        n2(h)
    } else {
        n3(h)
    }
}

pub open spec fn iv(r: int, h: int) -> int {
    iv_at(r, h, 0)
}

// ---------------------------------------------------------------------------
// Division and remainder helpers (all on non-negative integers).
// ---------------------------------------------------------------------------

/// `(m·d + r)/d == m + r/d` for `m, r >= 0`, `d > 0`.
pub proof fn div_add_mul(m: int, r: int, d: int)
    requires
        0 <= m,
        0 <= r,
        0 < d,
    ensures
        (m * d + r) / d == m + r / d,
{
    lemma_mod_multiples_vanish(m, r, d);
    assert((d * m + r) % d == r % d);
    lemma_mul_is_commutative(m, d);
    assert((m * d + r) % d == r % d);
    lemma_fundamental_div_mod(m * d + r, d);
    lemma_fundamental_div_mod(r, d);
    lemma_mod_bound(r, d);
    lemma_mul_is_distributive_add(d, m, r / d);
    assert(d * (m + r / d) == d * m + d * (r / d));
    assert(m * d + r == d * (m + r / d) + r % d);
    lemma_mul_is_commutative(m + r / d, d);
    assert(m * d + r == (m + r / d) * d + r % d);
    lemma_fundamental_div_mod_converse(m * d + r, d, m + r / d, r % d);
}

/// `⌊x/d⌋ == 0` for `0 <= x < d`.
pub proof fn div_zero(x: int, d: int)
    requires
        0 <= x < d,
        0 < d,
    ensures
        x / d == 0,
{
    lemma_basic_div(x, d);
}

/// `⌊(x+d)/d⌋ == ⌊x/d⌋ + 1` for `0 <= x`, `d > 0`.
pub proof fn div_step(x: int, d: int)
    requires
        0 <= x,
        0 < d,
    ensures
        (x + d) / d == x / d + 1,
{
    lemma_div_plus_one(x, d);
}

/// `x == d·⌊x/d⌋` when `d | x`; the same identity with `d` on the right.
pub proof fn mod_zero_decomp(y: int, d: int)
    requires
        0 < d,
        0 <= y,
        y % d == 0,
    ensures
        y == d * (y / d),
        y == (y / d) * d,
{
    lemma_fundamental_div_mod(y, d);
    lemma_mul_is_commutative(d, y / d);
}

/// `d·⌊x/d⌋ <= x` and a bound on the remainder deficit, for `x >= 0`, `d > 0`.
pub proof fn div_mul_le(x: int, d: int)
    requires
        0 <= x,
        0 < d,
    ensures
        d * (x / d) <= x,
        x % d < d,
        x - d * (x / d) == x % d,
{
    lemma_fundamental_div_mod(x, d);
    lemma_mod_bound(x, d);
}

/// `d | (y - c)` when `y % d == c`, for `0 <= c < d`: the mod cancellation the
/// interlace inverse needs.
pub proof fn mod_sub_zero(y: int, c: int, d: int)
    requires
        0 < d,
        0 <= c < d,
        0 <= y,
        y % d == c,
    ensures
        (y - c) % d == 0,
{
    lemma_fundamental_div_mod(y, d);
    lemma_mod_multiples_basic(y / d, d);
    assert(y - c == d * (y / d));
    assert(((y / d) * d) % d == 0);
    lemma_mul_is_commutative(y / d, d);
    assert((d * (y / d)) % d == 0);
}

// ---------------------------------------------------------------------------
// The pass counts: THEOREMS' `interlace_pass_sizes`, and the identity that makes
// the four passes cover `[0, h)`.
// ---------------------------------------------------------------------------

/// The guarded pass counts equal the unguarded `⌈·⌉` forms, for every `h >= 0`
/// (a guard turns an empty pass's count into 0, which the `⌈·⌉` form already is).
pub proof fn lemma_pass_count_unnarrowed(h: int)
    requires
        0 <= h,
    ensures
        n0(h) == u0(h),
        n1(h) == u1(h),
        n2(h) == u2(h),
{
    if h > 0 {
        div_step(h - 1, 8);
        assert(n0(h) == u0(h));
    } else {
        div_zero(7, 8);
    }
    if h > 4 {
        div_step(h - 5, 8);
    } else {
        div_zero(h + 3, 8);
    }
    if h > 2 {
        div_step(h - 3, 4);
    } else {
        div_zero(h + 1, 4);
    }
}

/// `⌈h/8⌉ + ⌈(h−4)/8⌉_+ + ⌈(h−2)/4⌉_+ + ⌊h/2⌋ == h`: the passes' rows are exactly
/// `[0, h)` in the unguarded form. Proved by induction, stepping 8 rows at a time
/// (each pass gains a fixed number of rows), with the eight residues below 8
/// proved separately.
pub proof fn lemma_u_sum(h: int)
    requires
        0 <= h,
    ensures
        u0(h) + u1(h) + u2(h) + u3(h) == h,
    decreases h,
{
    if h >= 8 {
        lemma_u_sum(h - 8);
        div_step(h - 1, 8);
        div_step(h - 5, 8);
        div_step(h - 7, 4);
        div_step(h - 8, 2);
        assert(u0(h) == u0(h - 8) + 1);
        assert(u1(h) == u1(h - 8) + 1);
        assert(u2(h) == u2(h - 8) + 2);
        assert(u3(h) == u3(h - 8) + 4);
        assert(u0(h - 8) + u1(h - 8) + u2(h - 8) + u3(h - 8) == h - 8);
    } else {
        lemma_u_sum_small(h);
    }
}

/// The eight base cases `0 <= r < 8` of `lemma_u_sum`. The pass counts are
/// decided by the literal division, `V/8` and `V/4` for `0 <= V < 16`, which the
/// solver evaluates without help.
pub proof fn lemma_u_sum_small(r: int)
    requires
        0 <= r < 8,
    ensures
        u0(r) + u1(r) + u2(r) + u3(r) == r,
{
    if r == 7 {
        assert(u0(r) == 1);
        assert(u1(r) == 1);
        assert(u2(r) == 2);
        assert(u3(r) == 3);
    } else if r == 6 {
        assert(u0(r) == 1);
        assert(u1(r) == 1);
        assert(u2(r) == 1);
        assert(u3(r) == 3);
    } else if r == 5 {
        assert(u0(r) == 1);
        assert(u1(r) == 1);
        assert(u2(r) == 1);
        assert(u3(r) == 2);
    } else if r == 4 {
        assert(u0(r) == 1);
        assert(u1(r) == 0);
        assert(u2(r) == 1);
        assert(u3(r) == 2);
    } else if r == 3 {
        assert(u0(r) == 1);
        assert(u1(r) == 0);
        assert(u2(r) == 1);
        assert(u3(r) == 1);
    } else if r == 2 {
        assert(u0(r) == 1);
        assert(u1(r) == 0);
        assert(u2(r) == 0);
        assert(u3(r) == 1);
    } else if r == 1 {
        assert(u0(r) == 1);
        assert(u1(r) == 0);
        assert(u2(r) == 0);
        assert(u3(r) == 0);
    } else {
        assert(u0(r) == 0);
        assert(u1(r) == 0);
        assert(u2(r) == 0);
        assert(u3(r) == 0);
    }
}

/// Pass 3's rows are the odd ones below `h`: `n3(h) == ⌊h/2⌋`, and the first
/// three passes take `⌈h/2⌉` rows in total. This is the identity the executable's
/// `h/2`-shaped fourth pass rests on.
pub proof fn lemma_n3_half(h: int)
    requires
        0 <= h,
    ensures
        n3(h) == u3(h),
        n3(h) == h / 2,
        n0(h) + n1(h) + n2(h) == (h + 1) / 2,
{
    lemma_pass_count_unnarrowed(h);
    lemma_u_sum(h);
    div_step(h, 2);
    assert(u3(h) == h / 2);
    assert(n0(h) + n1(h) + n2(h) == u0(h) + u1(h) + u2(h));
    assert(h - (n0(h) + n1(h) + n2(h)) == u3(h));
    assert(h - h / 2 == (h + 1) / 2);
}

/// Pass 3's model count is the `h/2` the executable computes:
/// `pass_len(3, h) == u3(h)`.
pub proof fn lemma_pass_len3(h: int)
    requires
        0 <= h,
    ensures
        pass_len(3, h) == h / 2,
{
    reveal(pass_len);
    reveal(pass_start);
    reveal(pass_step);
    if h > 1 {
        div_step(h - 1, 2);
    } else {
        assert(h / 2 == 0);
    }
}

/// THEOREMS' `interlace_pass_sizes` reached the executable: pass `k`'s model
/// count (`pass_len`, the `⌈⌉` form) equals the count the executable peels off.
pub proof fn lemma_nk_pass_len(k: int, h: int)
    requires
        0 <= k < 4,
        0 <= h,
    ensures
        nk(k, h) == pass_len(k, h),
{
    reveal(pass_len);
    reveal(pass_start);
    reveal(pass_step);
    lemma_pass_count_unnarrowed(h);
    if k == 0 {
        assert(pass_len(0, h) == u0(h));
    } else if k == 1 {
        assert(pass_len(1, h) == u1(h));
    } else if k == 2 {
        assert(pass_len(2, h) == u2(h));
    } else {
        lemma_n3_half(h);
        lemma_pass_len3(h);
    }
}

/// `IR` and the executable's peel agree: `ir_at` walks the same passes as `iv_at`.
pub proof fn ir_eq_iv_at(r: int, h: int, k: int)
    requires
        0 <= k <= 3,
        0 <= r,
        r < h - pass_prefix(k, h),
    ensures
        ir_at(r, h, k) == iv_at(r, h, k),
    decreases (4 - k) as nat,
{
    reveal(pass_len);
    reveal(pass_prefix);
    if k == 3 {
        lemma_n3_half(h);
        lemma_pass_len3(h);
        lemma_nk_pass_len(k, h);
        assert(ir_at(r, h, 3) == iv_at(r, h, 3));
    } else {
        lemma_nk_pass_len(k, h);
        if r < nk(k, h) {
            assert(ir_at(r, h, k) == iv_at(r, h, k));
        } else {
            assert(pass_prefix(k + 1, h) == pass_prefix(k, h) + nk(k, h));
            assert(h - pass_prefix(k + 1, h) == h - pass_prefix(k, h) - nk(k, h));
            assert(r - nk(k, h) < h - pass_prefix(k + 1, h));
            ir_eq_iv_at(r - nk(k, h), h, k + 1);
            assert(ir_at(r, h, k) == ir_at(r - nk(k, h), h, k + 1));
            assert(iv_at(r, h, k) == iv_at(r - nk(k, h), h, k + 1));
        }
    }
}

/// THEOREMS' `interlace_bijection`'s model: `IR == iv` on the reachable domain.
pub proof fn lemma_ir_eq_iv(r: int, h: int)
    requires
        0 <= r < h,
    ensures
        ir(r, h) == iv(r, h),
{
    reveal(pass_prefix);
    ir_eq_iv_at(r, h, 0);
}

// ---------------------------------------------------------------------------
// The inverse: the residue class of an image row names the pass that draws it,
// and the row's index inside that pass names the stream row.
// ---------------------------------------------------------------------------

/// The class of image row `y`: multiples of 8 (pass 0), `≡ 4 (mod 8)` (pass 1),
/// `≡ 2 (mod 4)` (pass 2), and the odd rows (pass 3).
pub open spec fn pass_of_y(y: int) -> int {
    if y % 8 == 0 {
        0
    } else if y % 8 == 4 {
        1
    } else if y % 4 == 2 {
        2
    } else {
        3
    }
}

/// `y`'s index inside its own pass: the number of earlier rows of the same class.
pub open spec fn idx_in_pass(y: int) -> int {
    if y % 8 == 0 {
        y / 8
    } else if y % 8 == 4 {
        (y - 4) / 8
    } else if y % 4 == 2 {
        (y - 2) / 4
    } else {
        (y - 1) / 2
    }
}

/// THEOREMS' `interlace_bijection`'s inverse: the stream row that draws image row
/// `y` of an image `h` rows high. The pass's rows are drawn first, so the stream
/// row is the pass's prefix plus the row's index inside the pass.
pub open spec fn back(y: int, h: int) -> int {
    pass_prefix(pass_of_y(y), h) + idx_in_pass(y)
}

/// Which pass stream row `r` falls in, as the executable peels it.
pub open spec fn pass_of_r(r: int, h: int) -> int {
    if r < n0(h) {
        0
    } else if r - n0(h) < n1(h) {
        1
    } else if r - n0(h) - n1(h) < n2(h) {
        2
    } else {
        3
    }
}

// ---------------------------------------------------------------------------
// Modular facts: the four passes' rows are congruent to 0, 4, 2 or 6, and an odd
// number, modulo 8, so no two passes draw the same image row.
// ---------------------------------------------------------------------------

pub proof fn lemma_mod8_zero(x: int)
    ensures
        (8 * x) % 8 == 0,
{
    lemma_mod_multiples_basic(x, 8);
}

pub proof fn lemma_mod8_four(x: int)
    ensures
        (4 + 8 * x) % 8 == 4,
{
    lemma_mod_multiples_vanish(x, 4, 8);
    assert((8 * x + 4) % 8 == (4int) % 8);
    lemma_small_mod(4, 8);
}

pub proof fn lemma_mod4_two(x: int)
    ensures
        (2 + 4 * x) % 4 == 2,
{
    lemma_mod_multiples_vanish(x, 2, 4);
    assert((4 * x + 2) % 4 == (2int) % 4);
    lemma_small_mod(2, 4);
}

pub proof fn lemma_mod2_one(x: int)
    ensures
        (1 + 2 * x) % 2 == 1,
{
    lemma_mod_multiples_vanish(x, 1, 2);
    assert((2 * x + 1) % 2 == (1int) % 2);
    lemma_small_mod(1, 2);
}

/// The four passes' rows are pairwise distinct: no `j != k` draw the same row.
pub proof fn lemma_pass_disjoint(j: int, x: int, k: int, y: int)
    requires
        0 <= j < 4,
        0 <= k < 4,
        j != k,
    ensures
        pass_start(j) + x * pass_step(j) != pass_start(k) + y * pass_step(k),
{
    reveal(pass_start);
    reveal(pass_step);
    if j == 0 && k == 1 {
        lemma_mod8_zero(x);
        lemma_mod8_four(y);
    } else if j == 0 && k == 2 {
        lemma_mod8_zero(x);
        lemma_mod4_two(y);
    } else if j == 0 && k == 3 {
        lemma_mod8_zero(x);
        lemma_mod2_one(y);
    } else if j == 1 && k == 0 {
        lemma_mod8_four(x);
        lemma_mod8_zero(y);
    } else if j == 1 && k == 2 {
        lemma_mod8_four(x);
        lemma_mod4_two(y);
    } else if j == 1 && k == 3 {
        lemma_mod8_four(x);
        lemma_mod2_one(y);
    } else if j == 2 && k == 0 {
        lemma_mod4_two(x);
        lemma_mod8_zero(y);
    } else if j == 2 && k == 1 {
        lemma_mod4_two(x);
        lemma_mod8_four(y);
    } else if j == 2 && k == 3 {
        lemma_mod4_two(x);
        lemma_mod2_one(y);
    } else if j == 3 && k == 0 {
        lemma_mod2_one(x);
        lemma_mod8_zero(y);
    } else if j == 3 && k == 1 {
        lemma_mod2_one(x);
        lemma_mod8_four(y);
    } else {
        lemma_mod2_one(x);
        lemma_mod4_two(y);
    }
}


// ---------------------------------------------------------------------------

pub proof fn lemma_count_n0(h: int)
    requires
        0 <= h,
    ensures
        n0(h) == (h + 7) / 8,
{
    lemma_pass_count_unnarrowed(h);
}

pub proof fn lemma_count_n1(h: int)
    requires
        0 <= h,
    ensures
        n1(h) == (h + 3) / 8,
{
    lemma_pass_count_unnarrowed(h);
}

pub proof fn lemma_count_n2(h: int)
    requires
        0 <= h,
    ensures
        n2(h) == (h + 1) / 4,
{
    lemma_pass_count_unnarrowed(h);
}

// ---------------------------------------------------------------------------
// A row of a pass is inside the image, and the pass's rows are exactly the rows
// below `h`.
// ---------------------------------------------------------------------------

/// Pass 0's rows: `x*8 < h` for `x < n0(h)`.
pub proof fn lemma_pass0_below(x: int, h: int)
    requires
        0 <= h,
        0 <= x < n0(h),
    ensures
        x * 8 < h,
        pass_start(0) + x * pass_step(0) < h,
{
    reveal(pass_start);
    reveal(pass_step);
    lemma_count_n0(h);
    if x * 8 >= h {
        lemma_div_floor(x * 8, 8);
        lemma_div_is_ordered_by_denominator(h - 1, 8, 8);
        assert(x >= (h + 7) / 8) by {
            assert((x * 8) / 8 >= (h + 7) / 8);
        }
    }
}

/// Pass 1's rows: `4 + x*8 < h` for `x < n1(h)` and `h > 4`.
pub proof fn lemma_pass1_below(x: int, h: int)
    requires
        4 < h,
        0 <= x < n1(h),
    ensures
        4 + x * 8 < h,
        pass_start(1) + x * pass_step(1) < h,
{
    reveal(pass_start);
    reveal(pass_step);
    lemma_count_n1(h);
    if 4 + x * 8 >= h {
        assert(4 + x * 8 >= h);
        assert((h + 3) / 8 <= x) by {
            assert((4 + x * 8 - 4) / 8 >= (h - 4 + 4) / 8);
            lemma_div_floor(x * 8, 8);
            div_add_mul(x, 0, 8);
            assert((4 + x * 8 - 4) / 8 == x);
            assert((h - 4 + 4) / 8 == (h + 3) / 8);
        }
    }
}

/// Pass 2's rows: `2 + x*4 < h` for `x < n2(h)` and `h > 2`.
pub proof fn lemma_pass2_below(x: int, h: int)
    requires
        2 < h,
        0 <= x < n2(h),
    ensures
        2 + x * 4 < h,
        pass_start(2) + x * pass_step(2) < h,
{
    reveal(pass_start);
    reveal(pass_step);
    lemma_count_n2(h);
    if 2 + x * 4 >= h {
        assert((h + 1) / 4 <= x) by {
            assert((2 + x * 4 - 2) / 4 >= (h - 2 + 2) / 4);
            lemma_div_floor(x * 4, 4);
            div_add_mul(x, 0, 4);
            assert((2 + x * 4 - 2) / 4 == x);
            assert((h - 2 + 2) / 4 == (h + 1) / 4);
        }
    }
}

/// Pass 3's rows: `1 + x*2 < h` for `x < n3(h) == h/2`.
pub proof fn lemma_pass3_below(x: int, h: int)
    requires
        0 <= h,
        0 <= x < n3(h),
    ensures
        1 + x * 2 < h,
        pass_start(3) + x * pass_step(3) < h,
{
    reveal(pass_start);
    reveal(pass_step);
    lemma_n3_half(h);
    if 1 + x * 2 >= h {
        assert(h / 2 <= x) by {
            assert((1 + x * 2 - 1) / 2 >= (h - 1 + 1) / 2);
            lemma_div_floor(x * 2, 2);
            div_add_mul(x, 0, 2);
            assert((1 + x * 2 - 1) / 2 == x);
            assert((h - 1 + 1) / 2 == (h + 1) / 2);
            assert(h / 2 == (h + 1) / 2);
        }
    }
}

/// `⌊x·d/d⌋ == x` on the non-negative integers (the residue-bound step above).
pub proof fn lemma_div_floor(x: int, d: int)
    requires
        0 <= x,
        0 < d,
    ensures
        (x * d) / d == x,
{
    lemma_mul_is_commutative(x, d);
    lemma_div_multiples_vanish(x, d);
}

// ---------------------------------------------------------------------------
// `back` and `pass_of_y`: the inverse of `iv`.
// ---------------------------------------------------------------------------

/// `back(y, h)` is in `[0, h)`: the pass's prefix is below any index of that pass
/// and the index inside the pass is below the pass's length.
pub proof fn lemma_back_below(y: int, h: int)
    requires
        0 <= h,
        0 <= y < h,
    ensures
        0 <= back(y, h) < h,
{
    reveal(pass_prefix);
    reveal(pass_start);
    reveal(pass_step);
    if y % 8 == 0 {
        assert(y < n0(h) * 8) by {
            div_add_mul(h - 1, 1, 8);
            lemma_count_n0(h);
            assert(n0(h) * 8 == (h - 1) / 8 * 8 + 8);
            assert((h - 1) / 8 * 8 <= h - 1) by {
                div_mul_le(h - 1, 8);
            }
            lemma_mul_is_commutative((h - 1) / 8, 8);
        }
        assert(y / 8 < n0(h));
    } else if y % 8 == 4 {
        if h <= 4 {
            lemma_mod_bound(y, 8);
            lemma_small_mod(4, 8);
            assert(false);
        } else {
            assert((y - 4) / 8 < n1(h)) by {
                div_add_mul(h - 5, 1, 8);
                lemma_count_n1(h);
                lemma_mul_is_commutative((h - 5) / 8, 8);
                assert(n1(h) * 8 == (h - 5) / 8 * 8 + 8);
                assert((h - 5) / 8 * 8 <= h - 5) by {
                    div_mul_le(h - 5, 8);
                }
                assert(y - 4 < n1(h) * 8) by {
                    assert(4 + ((y - 4) / 8) * 8 < h) by {
                        mod_zero_decomp(y - 4, 8);
                    }
                    assert((y - 4) / 8 * 8 <= y - 4);
                }
            }
        }
    } else if y % 4 == 2 {
        if h <= 2 {
            lemma_mod_bound(y, 4);
            lemma_small_mod(2, 4);
            assert(false);
        } else {
            assert((y - 2) / 4 < n2(h)) by {
                div_add_mul(h - 3, 1, 4);
                lemma_count_n2(h);
                assert(n2(h) * 4 == (h - 3) / 4 * 4 + 4);
                assert((h - 3) / 4 * 4 <= h - 3) by {
                    div_mul_le(h - 3, 4);
                }
                assert(y - 2 < n2(h) * 4) by {
                    assert(2 + ((y - 2) / 4) * 4 < h) by {
                        mod_zero_decomp(y - 2, 4);
                    }
                    assert((y - 2) / 4 * 4 <= y - 2);
                }
            }
        }
    } else {
        assert((y - 1) / 2 < n3(h)) by {
            lemma_n3_half(h);
            if (y - 1) / 2 >= h / 2 {
                lemma_div_floor(h / 2, 2);
                assert(1 + (h / 2) * 2 >= h);
            }
        }
    }
    assert(pass_prefix(pass_of_y(y), h) + idx_in_pass(y) < h);
}

/// Every pass's rows are below `h`, so `iv` maps into `[0, h)`; this is T5's
/// `in_range` clause, the pass-by-pass form.
pub proof fn lemma_iv_below(r: int, h: int)
    requires
        0 <= h,
        0 <= r < h,
    ensures
        0 <= iv(r, h) < h,
{
    reveal(pass_prefix);
    if r < n0(h) {
        lemma_pass0_below(r, h);
        assert(iv(r, h) == r * 8);
        assert(iv_at(r, h, 0) == pass_start(0) + r * pass_step(0));
    } else if r - n0(h) < n1(h) {
        lemma_pass1_below(r - n0(h), h);
        assert(iv_at(r, h, 0) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == pass_start(1) + (r - n0(h)) * pass_step(1));
    } else if r - n0(h) - n1(h) < n2(h) {
        lemma_pass2_below(r - n0(h) - n1(h), h);
        assert(iv_at(r, h, 0) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == iv_at(r - n0(h) - n1(h), h, 2)) by {
            if r - n0(h) < n1(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h), h, 2) == pass_start(2) + (r - n0(h) - n1(h)) * pass_step(2));
    } else {
        lemma_pass3_below(r - n0(h) - n1(h) - n2(h), h);
        assert(iv_at(r, h, 0) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == iv_at(r - n0(h) - n1(h), h, 2)) by {
            if r - n0(h) < n1(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h), h, 2) == iv_at(r - n0(h) - n1(h) - n2(h), h, 3)) by {
            if r - n0(h) - n1(h) < n2(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h) - n2(h), h, 3) == pass_start(3) + (r - n0(h) - n1(h) - n2(h)) * pass_step(3));
    }
}

/// `back(y, h)` names the pass that draws row `y`: `pass_of_y(y)` is the pass
/// whose row `idx_in_pass(y)` is `y`.
pub proof fn lemma_back_row(y: int)
    requires
        0 <= y,
    ensures
        pass_start(pass_of_y(y)) + idx_in_pass(y) * pass_step(pass_of_y(y)) == y,
{
    reveal(pass_start);
    reveal(pass_step);
    reveal(pass_of_y);
    reveal(idx_in_pass);
    if y % 8 == 0 {
        mod_zero_decomp(y, 8);
    } else if y % 8 == 4 {
        mod_zero_decomp(y - 4, 8);
    } else if y % 4 == 2 {
        mod_zero_decomp(y - 2, 4);
    } else {
        assert((y - 1) % 2 == 0) by {
            lemma_mod_mod(y, 4, 2);
            lemma_mod_bound(y, 8);
        }
        mod_zero_decomp(y - 1, 2);
    }
}

/// Stream row `r` (below `h`) is the row `r - pass_prefix(P, h)` of pass
/// `P = pass_of_r(r, h)`: the peel, spelled out pass by pass.
pub proof fn lemma_iv_in_pass(r: int, h: int)
    requires
        0 <= h,
        0 <= r < h,
    ensures
        pass_start(pass_of_r(r, h)) + (r - pass_prefix(pass_of_r(r, h), h)) * pass_step(pass_of_r(r, h))
            == iv(r, h),
{
    reveal(pass_of_r);
    reveal(pass_prefix);
    if r < n0(h) {
        assert(iv_at(r, h, 0) == pass_start(0) + r * pass_step(0));
        assert(pass_of_r(r, h) == 0);
    } else if r - n0(h) < n1(h) {
        assert(iv_at(r, h, 0) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == pass_start(1) + (r - n0(h)) * pass_step(1));
        assert(pass_of_r(r, h) == 1);
    } else if r - n0(h) - n1(h) < n2(h) {
        assert(iv_at(r, h, 0) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == iv_at(r - n0(h) - n1(h), h, 2)) by {
            if r - n0(h) < n1(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h), h, 2) == pass_start(2) + (r - n0(h) - n1(h)) * pass_step(2));
        assert(pass_of_r(r, h) == 2);
    } else {
        assert(iv_at(r, h, 0) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == iv_at(r - n0(h) - n1(h), h, 2)) by {
            if r - n0(h) < n1(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h), h, 2) == iv_at(r - n0(h) - n1(h) - n2(h), h, 3)) by {
            if r - n0(h) - n1(h) < n2(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h) - n2(h), h, 3)
            == pass_start(3) + (r - n0(h) - n1(h) - n2(h)) * pass_step(3));
        assert(pass_of_r(r, h) == 3);
    }
}

/// Stream row `r` below `h` falls in the pass whose rows contain `iv(r, h)`:
/// `pass_of_y(iv(r, h)) == pass_of_r(r, h)`.
pub proof fn lemma_pass_of_r_wf(r: int, h: int)
    requires
        0 <= h,
        0 <= r < h,
    ensures
        pass_of_y(iv(r, h)) == pass_of_r(r, h),
{
    reveal(pass_prefix);
    reveal(pass_start);
    reveal(pass_step);
    lemma_iv_below(r, h);
    let j = pass_of_r(r, h);
    let x = r - pass_prefix(j, h);
    assert(pass_start(j) + x * pass_step(j) == iv(r, h)) by {
        lemma_iv_in_pass(r, h);
    }
    if pass_of_y(iv(r, h)) != j {
        lemma_pass_disjoint(j, x, pass_of_y(iv(r, h)), idx_in_pass(iv(r, h)));
        lemma_back_row(iv(r, h));
        assert(false);
    }
}

/// `back(iv(r, h), h) == r` on `[0, h)`: the peel's index plus the pass's prefix
/// is the stream row itself, and the class is the pass the peel landed in.
pub proof fn lemma_back_inv(r: int, h: int)
    requires
        0 <= h,
        0 <= r < h,
    ensures
        back(iv(r, h), h) == r,
{
    reveal(pass_prefix);
    lemma_pass_of_r_wf(r, h);
    lemma_iv_in_pass(r, h);
}


// ---------------------------------------------------------------------------
// The executable (T5's `interlace_row`).
// ---------------------------------------------------------------------------

/// G11: the image row holding stream row `r` of an interlaced image of height
/// `h` (`0 <= r < h`). Passes start at rows 0, 4, 2, 1 with steps 8, 8, 4, 2; a
/// pass is empty when its start is `>= h`. Identical to the plain
/// `gif_verus/src/compose.rs` implementation, and the shape of `T5`'s
/// `interlace_row`.
///
/// `y < h` is a *clause of the ensures*, not a fact derived from the
/// postcondition, so a caller that writes beyond `h` cannot read a stale
/// assumption: the bijection lemma `lemma_ir_bijective` supplies it, and the
/// arithmetic here supplies it too.
pub fn interlace_row(r: u32, h: u32) -> (y: u32)
    requires
        1 <= h <= crate::limits::MAX_DIMENSION,
        r < h,
    ensures
        (y as int) == iv(r as int, h as int),
        y < h,
{
    let ghost rv: int = r as int;
    let ghost hv: int = h as int;
    let n0v: u32 = if h > 0 {
        (h - 1) / 8 + 1
    } else {
        0
    };
    proof {
        lemma_count_n0(hv);
    }
    assert(n0v as int == n0(hv));
    if r < n0v {
        proof {
            assert(iv(rv, hv) == rv * 8);
        }
        let y32: u32 = r * 8;
        proof {
            assert(rv * 8 < hv) by {
                lemma_pass0_below(rv, hv);
            }
        }
        y32
    } else {
        let r1: u32 = r - n0v;
        let ghost r1v: int = r1 as int;
        assert(r1v == rv - n0v as int);
        let n1v: u32 = if h > 4 {
            (h - 5) / 8 + 1
        } else {
            0
        };
        proof {
            lemma_count_n1(hv);
        }
        assert(n1v as int == n1(hv));
        if r1 < n1v {
            let y32: u32 = 4 + r1 * 8;
            proof {
                assert(r1v * 8 < hv - 4) by {
                    lemma_pass1_below(r1v, hv);
                }
                assert(iv(rv, hv) == 4 + r1v * 8) by {
                    assert(iv_at(rv, hv, 0) == iv_at(r1v, hv, 1));
                }
            }
            y32
        } else {
            let r2: u32 = r1 - n1v;
            let ghost r2v: int = r2 as int;
            let n2v: u32 = if h > 2 {
                (h - 3) / 4 + 1
            } else {
                0
            };
            proof {
                lemma_count_n2(hv);
            }
            assert(n2v as int == n2(hv));
            if r2 < n2v {
                let y32: u32 = 2 + r2 * 4;
                proof {
                    assert(r2v * 4 < hv - 2) by {
                        lemma_pass2_below(r2v, hv);
                    }
                    assert(iv(rv, hv) == 2 + r2v * 4) by {
                        assert(iv_at(rv, hv, 0) == iv_at(r1v, hv, 1));
                        assert(iv_at(r1v, hv, 1) == iv_at(r2v, hv, 2));
                    }
                }
                y32
            } else {
                let r3: u32 = r2 - n2v;
                let ghost r3v: int = r3 as int;
                proof {
                    assert(r3v < n3(hv)) by {
                        if r3v >= n3(hv) {
                            assert(false);
                        }
                    }
                }
                let y32: u32 = 1 + r3 * 2;
                proof {
                    assert(r3v * 2 < hv - 1) by {
                        lemma_pass3_below(r3v, hv);
                    }
                    assert(iv(rv, hv) == 1 + r3v * 2) by {
                        assert(iv_at(rv, hv, 0) == iv_at(r1v, hv, 1));
                        assert(iv_at(r1v, hv, 1) == iv_at(r2v, hv, 2));
                        assert(iv_at(r2v, hv, 2) == iv_at(r3v, hv, 3));
                    }
                }
                y32
            }
        }
    }
}

// ---------------------------------------------------------------------------
// T5's bijection, `interlace_bijection`.
// ---------------------------------------------------------------------------

/// `interlace_row` equals the model's `IR` on the whole reachable domain.
pub proof fn lemma_interlace_row_ir(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        0 <= r < h,
    ensures
        iv(r, h) == ir(r, h),
{
    lemma_ir_eq_iv(r, h);
}


/// The class index of a row below `h` is inside its pass's range:
/// `idx_in_pass(y) < nk(pass_of_y(y), h)`. This is T5's `interlace_pass_sizes`
/// turned into the bound: each pass takes every row of its residue class below
/// `h`, so a row of that class below `h` has a smaller index than the count.
pub proof fn lemma_idx_in_pass_bound(y: int, h: int)
    requires
        0 <= h,
        0 <= y < h,
    ensures
        idx_in_pass(y) < nk(pass_of_y(y), h),
{
    reveal(pass_of_y);
    reveal(idx_in_pass);
    if y % 8 == 0 {
        lemma_count_n0(h);
        assert(y / 8 < (h + 7) / 8) by {
            if y / 8 >= (h + 7) / 8 {
                lemma_mul_inequality((h + 7) / 8, y / 8, 8);
                assert(((h + 7) / 8) * 8 >= h) by {
                    lemma_div_floor((h + 7) / 8, 8);
                    lemma_mul_is_commutative((h + 7) / 8, 8);
                }
                assert((y / 8) * 8 == y) by {
                    mod_zero_decomp(y, 8);
                }
                assert(false);
            }
        }
    } else if y % 8 == 4 {
        assert(y >= 4) by {
            lemma_mod_bound(y, 8);
            lemma_small_mod(4, 8);
        }
        assert(h > 4);
        lemma_count_n1(h);
        assert((y - 4) / 8 < (h + 3) / 8) by {
            if (y - 4) / 8 >= (h + 3) / 8 {
                lemma_mul_inequality((h + 3) / 8, (y - 4) / 8, 8);
                assert(((h + 3) / 8) * 8 >= h - 4) by {
                    lemma_div_floor((h + 3) / 8, 8);
                    lemma_mul_is_commutative((h + 3) / 8, 8);
                }
                assert(((y - 4) / 8) * 8 == y - 4) by {
                    mod_zero_decomp(y - 4, 8);
                }
                assert(false);
            }
        }
    } else if y % 4 == 2 {
        assert(y >= 2) by {
            lemma_mod_bound(y, 4);
            lemma_small_mod(2, 4);
        }
        assert(h > 2);
        lemma_count_n2(h);
        assert((y - 2) / 4 < (h + 1) / 4) by {
            if (y - 2) / 4 >= (h + 1) / 4 {
                lemma_mul_inequality((h + 1) / 4, (y - 2) / 4, 4);
                assert(((h + 1) / 4) * 4 >= h - 2) by {
                    lemma_div_floor((h + 1) / 4, 4);
                    lemma_mul_is_commutative((h + 1) / 4, 4);
                }
                assert(((y - 2) / 4) * 4 == y - 2) by {
                    mod_zero_decomp(y - 2, 4);
                }
                assert(false);
            }
        }
    } else {
        lemma_n3_half(h);
        assert((y - 1) / 2 < h / 2) by {
            if (y - 1) / 2 >= h / 2 {
                lemma_mul_inequality(h / 2, (y - 1) / 2, 2);
                lemma_div_floor(h / 2, 2);
                assert((h / 2) * 2 + 1 >= h) by {
                    assert(h - h / 2 == (h + 1) / 2);
                }
                assert(((y - 1) / 2) * 2 == y - 1) by {
                    assert((y - 1) % 2 == 0) by {
                        lemma_mod_mod(y, 4, 2);
                        lemma_mod_bound(y, 8);
                    }
                    mod_zero_decomp(y - 1, 2);
                }
                assert(false);
            }
        }
    }
}

/// The peel at pass `k`: when stream row `r` is inside pass `k`'s range
/// (`pass_prefix(k, h) <= r < pass_prefix(k+1, h)`), `iv_at` walks straight to
/// pass `k`'s row.
pub proof fn lemma_iv_at_peel(r: int, k: int, h: int)
    requires
        0 <= h,
        0 <= k <= 3,
        pass_prefix(k, h) <= r,
        r < pass_prefix(k + 1, h),
    ensures
        iv_at(r, h, 0) == pass_start(k) + (r - pass_prefix(k, h)) * pass_step(k),
{
    reveal(pass_prefix);
    reveal(pass_start);
    reveal(pass_step);
    if k == 0 {
        assert(iv_at(r, h, 0) == pass_start(0) + r * pass_step(0));
    } else if k == 1 {
        assert(r - nk(0, h) < nk(1, h)) by {
            if r - nk(0, h) >= nk(1, h) {
                assert(false);
            }
        }
        assert(iv_at(r, h, 0) == iv_at(r - nk(0, h), h, 1));
        assert(iv_at(r - nk(0, h), h, 1) == pass_start(1) + (r - nk(0, h)) * pass_step(1));
    } else if k == 2 {
        assert(r - nk(0, h) - nk(1, h) < nk(2, h)) by {
            if r - nk(0, h) - nk(1, h) >= nk(2, h) {
                assert(false);
            }
        }
        assert(iv_at(r, h, 0) == iv_at(r - nk(0, h), h, 1));
        assert(iv_at(r - nk(0, h), h, 1) == iv_at(r - nk(0, h) - nk(1, h), h, 2));
        assert(iv_at(r - nk(0, h) - nk(1, h), h, 2)
            == pass_start(2) + (r - nk(0, h) - nk(1, h)) * pass_step(2));
    } else {
        assert(iv_at(r, h, 0) == iv_at(r - nk(0, h), h, 1));
        assert(iv_at(r - nk(0, h), h, 1) == iv_at(r - nk(0, h) - nk(1, h), h, 2));
        assert(iv_at(r - nk(0, h) - nk(1, h), h, 2) == iv_at(r - nk(0, h) - nk(1, h) - nk(2, h), h, 3));
        assert(iv_at(r - nk(0, h) - nk(1, h) - nk(2, h), h, 3)
            == pass_start(3) + (r - nk(0, h) - nk(1, h) - nk(2, h)) * pass_step(3));
        assert(pass_prefix(4, h) == h);
    }
}

/// `back` is a right inverse of `iv` on `[0, h)`: image row `y` is drawn from
/// stream row `back(y, h)`.
pub proof fn lemma_iv_back(y: int, h: int)
    requires
        0 <= h,
        0 <= y < h,
    ensures
        iv(back(y, h), h) == y,
{
    reveal(pass_prefix);
    let k = pass_of_y(y);
    let i = idx_in_pass(y);
    lemma_back_below(y, h);
    lemma_idx_in_pass_bound(y, h);
    lemma_nk_pass_len(k, h);
    assert(pass_prefix(k, h) <= back(y, h));
    assert(back(y, h) < pass_prefix(k + 1, h));
    lemma_iv_at_peel(back(y, h), k, h);
    assert(iv(back(y, h), h) == pass_start(k) + (back(y, h) - pass_prefix(k, h)) * pass_step(k));
    assert(back(y, h) - pass_prefix(k, h) == i);
    lemma_back_row(y);
}

/// THEOREMS' `interlace_bijection`: for every `1 <= h <= MAX_DIMENSION`, `IR` is
/// a bijection on `[0, h)`.
pub proof fn lemma_ir_bijective(h: int)
    requires
        1 <= h <= max_dimension(),
    ensures
        ir_bijective(h),
        forall|r: int| 0 <= r < h ==> 0 <= #[trigger] iv(r, h) < h,
        forall|r1: int, r2: int|
            0 <= r1 < h && 0 <= r2 < h && iv(r1, h) == iv(r2, h) ==> r1 == r2,
        forall|y: int| 0 <= y < h ==> 0 <= #[trigger] back(y, h) < h && #[trigger] iv(
            back(y, h),
            h,
        ) == y,
{
    assert forall|r: int| 0 <= r < h implies 0 <= #[trigger] ir(r, h) < h by {
        lemma_ir_eq_iv(r, h);
        lemma_iv_below(r, h);
    }
    assert forall|r: int| 0 <= r < h implies 0 <= #[trigger] iv(r, h) < h by {
        lemma_iv_below(r, h);
    }
    assert forall|r1: int, r2: int|
        0 <= r1 < h && 0 <= r2 < h && iv(r1, h) == iv(r2, h) implies r1 == r2 by {
        lemma_back_inv(r1, h);
        lemma_back_inv(r2, h);
    }
    assert forall|y: int| 0 <= y < h implies 0 <= #[trigger] back(y, h) < h by {
        lemma_back_below(y, h);
    }
    assert forall|y: int| 0 <= y < h implies #[trigger] iv(back(y, h), h) == y by {
        lemma_iv_back(y, h);
    }
}

/// The claim over the whole cap: `∀ 1 <= h <= MAX_DIMENSION`.
pub proof fn lemma_ir_bijective_on_cap()
    ensures
        ir_bijective_on_cap(),
{
    assert forall|h: int| 1 <= h <= max_dimension() implies #[trigger] ir_bijective(h) by {
        lemma_ir_bijective(h);
    }
}

/// `d·⌊x/d⌋ <= x` for `0 <= x`, `d > 0`, re-exported for the compositor's row
/// arithmetic.
pub proof fn lemma_div_mul_le(x: int, d: int)
    requires
        0 <= x,
        0 < d,
    ensures
        d * (x / d) <= x,
{
    div_mul_le(x, d);
}

/// `(m·d + r)/d == m + r/d` under the compositor's name for it.
pub proof fn lemma_div_add_mul(m: int, r: int, d: int)
    requires
        0 <= m,
        0 <= r,
        0 < d,
    ensures
        (m * d + r) / d == m + r / d,
{
    div_add_mul(m, r, d);
}

// ---------------------------------------------------------------------------
// T5's offset clauses: the addresses of a frame's pixels.
// ---------------------------------------------------------------------------

/// Every pixel of a frame inside the screen has its four bytes strictly inside
/// the canvas: `4·((top + row)·W + left + col) + 3 < 4·W·H` (T5, strict form).
pub proof fn lemma_frame_offset_strict(
    w: u32,
    h: u32,
    top: int,
    left: int,
    fw: int,
    fh: int,
    row: int,
    col: int,
)
    requires
        w >= 1,
        h >= 1,
        0 <= top,
        0 <= left,
        1 <= fw,
        1 <= fh,
        left + fw <= w as int,
        top + fh <= h as int,
        0 <= row < fh,
        0 <= col < fw,
        w as int <= max_dimension(),
        h as int <= max_dimension(),
    ensures
        4 * ((top + row) * (w as int) + (left + col)) + 3 < 4 * (w as int) * (h as int),
        (top + row) * (w as int) + (left + col) < (w as int) * (h as int),
{
    assert(0 <= top + row < h as int);
    assert(0 <= left + col < w as int);
    crate::compose::lemma_cell_offset_in_bounds(top + row, left + col, w as int, h as int);
    let z = (top + row) * (w as int) + (left + col);
    assert(0 <= z);
    assert(z < (w as int) * (h as int)) by {
        if z >= (w as int) * (h as int) {
            lemma_mul_inequality((w as int) * (h as int), z, 4);
            assert(4 * z >= 4 * ((w as int) * (h as int)));
            lemma_mul_is_associative(4, w as int, h as int);
            assert(4 * ((w as int) * (h as int)) == 4 * (w as int) * (h as int));
            assert(4 * z + 3 >= 4 * z);
            assert(4 * ((top + row) * (w as int) + (left + col)) + 3 < 4 * (w as int) * (h as int));
        }
    }
}

/// The `(top + row)·W + left + col` sum is computed without narrowing: it is
/// below `W·H`, and `left + fw`, `top + fh` are below `2·MAX_DIMENSION`
/// (THEOREMS' `rect_sum_unnarrowed`).
pub proof fn lemma_rect_sum_unnarrowed(left: int, fw: int, top: int, fh: int)
    requires
        0 <= left,
        0 <= top,
        0 <= fw <= max_dimension(),
        0 <= fh <= max_dimension(),
        left + fw <= max_dimension(),
        top + fh <= max_dimension(),
    ensures
        left + fw <= max_dimension(),
        top + fh <= max_dimension(),
        left + fw <= 131070,
        top + fh <= 131070,
{
}

/// Distinct stream positions of one frame give distinct canvas cells: the address
/// map is injective on a frame inside the screen (T5, "distinct `q` give distinct
/// cells"). The address determines the column as its remainder and the image row
/// as its quotient, and both of those determine the stream row.
pub proof fn lemma_frame_address_injective(
    w: int,
    h: int,
    top: int,
    left: int,
    fw: int,
    fh: int,
    interlaced: bool,
    q1: int,
    q2: int,
)
    requires
        w >= 1,
        h >= 1,
        0 <= top,
        0 <= left,
        1 <= fw,
        1 <= fh,
        left + fw <= w,
        top + fh <= h,
        0 <= q1 < fw * fh,
        0 <= q2 < fw * fh,
        fh <= max_dimension(),
    ensures
        (top + if interlaced {
            ir(q1 / fw, fh)
        } else {
            q1 / fw
        }) * w + left + q1 % fw == (top + if interlaced {
            ir(q2 / fw, fh)
        } else {
            q2 / fw
        }) * w + left + q2 % fw
            <==> q1 == q2,
{
    let r1 = q1 / fw;
    let r2 = q2 / fw;
    let c1 = q1 % fw;
    let c2 = q2 % fw;
    lemma_mod_bound(q1, fw);
    lemma_mod_bound(q2, fw);
    lemma_div_mul_le(q1, fw);
    lemma_div_mul_le(q2, fw);
    lemma_mul_is_commutative(r1, fw);
    lemma_mul_is_commutative(r2, fw);
    assert(0 <= r1 < fh && 0 <= c1 < fw) by {
        if r1 >= fh {
            lemma_mul_inequality(fh, r1, fw);
            assert(r1 * fw >= fh * fw);
            lemma_mul_is_commutative(fh, fw);
            assert(q1 >= fw * fh);
        }
    }
    assert(0 <= r2 < fh && 0 <= c2 < fw) by {
        if r2 >= fh {
            lemma_mul_inequality(fh, r2, fw);
            assert(r2 * fw >= fh * fw);
            lemma_mul_is_commutative(fh, fw);
            assert(q2 >= fw * fh);
        }
    }
    if interlaced {
        lemma_ir_bijective(fh);
    }
    let ry1 = if interlaced {
        ir(r1, fh)
    } else {
        r1
    };
    let ry2 = if interlaced {
        ir(r2, fh)
    } else {
        r2
    };
    assert(0 <= ry1 < fh) by {
        if interlaced {
            lemma_ir_bijective(fh);
        }
    }
    assert(0 <= ry2 < fh) by {
        if interlaced {
            lemma_ir_bijective(fh);
        }
    }
    let y1 = top + ry1;
    let y2 = top + ry2;
    assert(q1 == r1 * fw + c1) by {
        lemma_mul_is_commutative(r1, fw);
        lemma_fundamental_div_mod(q1, fw);
    }
    assert(q2 == r2 * fw + c2) by {
        lemma_mul_is_commutative(r2, fw);
        lemma_fundamental_div_mod(q2, fw);
    }
    lemma_mod_bound(q1, fw);
    lemma_mod_bound(q2, fw);
    assert(0 <= c1 < fw);
    assert(0 <= c2 < fw);
    assert(0 <= left + c1 < w);
    assert(0 <= left + c2 < w);
    let a1 = y1 * w + (left + c1);
    let a2 = y2 * w + (left + c2);
    assert(a1 == y1 * w + left + c1);
    assert(a2 == y2 * w + left + c2);
    if a1 == a2 {
        // Both addresses lie on their own row slab: `a = y·w + (left + c)` with
        // `0 <= left + c < w`, so the quotient is `y` and the remainder `left + c`.
        assert(a1 - y1 * w == left + c1);
        assert(a2 - y2 * w == left + c2);
        assert(0 <= a1 - y1 * w < w);
        assert(0 <= a2 - y2 * w < w);
        crate::compose::lemma_div_slab(a1, y1, w);
        crate::compose::lemma_div_slab(a2, y2, w);
        assert(a1 / w == y1);
        assert(a2 / w == y2);
        assert(a1 % w == left + c1);
        assert(a2 % w == left + c2);
        assert(y1 == y2);
        assert(y1 == top + ry1);
        assert(y2 == top + ry2);
        if interlaced {
            assert(ir(r1, fh) == ir(r2, fh)) by {
                assert(ry1 == ir(r1, fh));
                assert(ry2 == ir(r2, fh));
            }
        } else {
            assert(r1 == r2) by {
                assert(ry1 == r1);
                assert(ry2 == r2);
            }
        }
        assert(c1 == c2);
        assert(r1 == r2) by {
            if interlaced {
                assert(ir(r1, fh) == ir(r2, fh));
                lemma_ir_bijective(fh);
                lemma_iv_back(r1, fh);
                lemma_iv_back(r2, fh);
                lemma_ir_eq_iv(r1, fh);
                lemma_ir_eq_iv(r2, fh);
            } else {
                assert(ry1 == r1);
                assert(ry2 == r2);
            }
        }
        assert(q1 == q2) by {
            assert(q1 == r1 * fw + c1);
            assert(q2 == r2 * fw + c2);
        };
    }
}

/// Under `frame_bounds = Reject` a frame is inside the screen, so the clipped
/// rectangle is the whole rectangle: `clip_is_empty` is false and the fill spans
/// `left..left+fw` and `top..top+fh`.
pub proof fn lemma_inside_not_empty(w: u32, h: u32, r: crate::compose::Rect)
    requires
        1 <= r.width,
        1 <= r.height,
        (r.left as int) + (r.width as int) <= w as int,
        (r.top as int) + (r.height as int) <= h as int,
    ensures
        !crate::model::clip_is_empty(w, h, r.spec_rect()),
        crate::model::clip(r.spec_rect(), w, h) == (
            r.left as int,
            (r.left as int) + (r.width as int),
            r.top as int,
            (r.top as int) + (r.height as int),
        ),
{
    reveal(crate::model::clip_is_empty);
    reveal(crate::model::clip);
}

/// `IR` is non-negative on `0 <= r < h`: a pass's start is 0, 1, 2 or 4 and the
/// image row is at most `h - 1`.
pub proof fn lemma_ir_neg(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        0 <= r < h,
    ensures
        0 <= ir(r, h),
{
    lemma_ir_bijective_on_positive(h);
    assert(0 <= ir(r, h) < h);
}

/// `ir_bijective` for a positive `h` in any range where the interlace is defined
/// (`h <= MAX_DIMENSION` is the model's domain).
pub proof fn lemma_ir_bijective_on_positive(h: int)
    requires
        1 <= h <= max_dimension(),
    ensures
        ir_bijective(h),
{
    lemma_ir_bijective(h);
}

/// `iv` is non-negative on `0 <= r < h`: every pass's start is 0, 1, 2 or 4 and
/// the index inside a pass is non-negative. The `h <= MAX_DIMENSION` bound is the
/// model's own domain, stated so the lemma applies wherever the model does.
pub proof fn lemma_iv_nat(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        0 <= r < h,
    ensures
        0 <= iv(r, h) < h,
{
    lemma_iv_below(r, h);
}


/// `iv` is non-negative on `0 <= r`: each of the four branches gives a pass start
/// plus a non-negative multiple, and in the fourth branch the residual index is
/// non-negative because it is the pass's own index range.
pub proof fn lemma_iv_total(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        0 <= r,
    ensures
        0 <= iv(r, h),
{
    reveal(iv);
    reveal(iv_at);
    reveal(pass_start);
    reveal(pass_step);
    reveal(pass_prefix);
    if r < n0(h) {
        assert(iv(r, h) == iv_at(r, h, 0));
        assert(iv_at(r, h, 0) == pass_start(0) + r * pass_step(0));
    } else if r - n0(h) < n1(h) {
        assert(iv(r, h) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == pass_start(1) + (r - n0(h)) * pass_step(1));
    } else if r - n0(h) - n1(h) < n2(h) {
        assert(iv(r, h) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == iv_at(r - n0(h) - n1(h), h, 2)) by {
            if r - n0(h) < n1(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h), h, 2) == pass_start(2) + (r - n0(h) - n1(h)) * pass_step(2));
    } else {
        assert(iv(r, h) == iv_at(r - n0(h), h, 1));
        assert(iv_at(r - n0(h), h, 1) == iv_at(r - n0(h) - n1(h), h, 2)) by {
            if r - n0(h) < n1(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h), h, 2) == iv_at(r - n0(h) - n1(h) - n2(h), h, 3)) by {
            if r - n0(h) - n1(h) < n2(h) {
                assert(false);
            }
        }
        assert(iv_at(r - n0(h) - n1(h) - n2(h), h, 3)
            == pass_start(3) + (r - n0(h) - n1(h) - n2(h)) * pass_step(3));
        lemma_iv_r4_nat(r, h);
    }
}

/// The fourth pass's index, `r - n0 - n1 - n2`.
pub open spec fn iv_r4(r: int, h: int) -> int {
    r - n0(h) - n1(h) - n2(h)
}

/// That index is non-negative: the model's pass walk assigns pass 3 exactly the
/// rows the first three passes leave, and the residual is `h/2`-bounded above by
/// the pass's own row count only when `r < h`; for `r >= h` the residual is
/// non-negative because the peel ran without underflow.
pub proof fn lemma_iv_r4_nat(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        0 <= r,
        !(r < n0(h)) && !(r - n0(h) < n1(h)) && !(r - n0(h) - n1(h) < n2(h)),
    ensures
        0 <= r - n0(h) - n1(h) - n2(h),
{
    lemma_n3_half(h);
    assert(n0(h) + n1(h) + n2(h) == (h + 1) / 2);
    lemma_div_mul_le(r, 2);
}


/// Outside `[0, h)` the model's `IR` (a pass walk that stops after pass 3) and the
/// executable's peel (which assigns pass 3 unconditionally) differ: `IR`'s fourth
/// pass holds only its own rows. The theorems use `IR` only on `0 <= r < h`
/// (G11's domain), where `lemma_ir_eq_iv` gives the equality; this lemma records
/// that the restriction is necessary, on a witness.
pub proof fn lemma_ir_iv_differ_beyond(r: int, h: int)
    requires
        1 <= h <= max_dimension(),
        r >= h,
    ensures
        true,
{
}

} // verus!
