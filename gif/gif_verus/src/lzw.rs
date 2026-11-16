//! G10 variable-length-code LZW over the image data's sub-blocks.
//!
//! The decoder works on the caller-provided `LzwTable` reservation — G18's one
//! 20,480-byte `try_zeroed(LzwTable, LZW_TABLE_BYTES)` region, viewed as three
//! areas: prefix codes as little-endian `u16` pairs (code `c` at byte `2c`,
//! bytes `[0, 8192)`), suffix bytes (`[8192, 12288)`) and string lengths as
//! little-endian `u16` pairs (code `c` at byte `12288 + 2c`, bytes
//! `[12288, 20480)`); 4,096 entries each. It hands every index to a
//! caller-provided `Sink` with its pixel position, and never allocates.
//!
//! Literal codes are decided by rule (G10.1, decision D20): a code below
//! `clear = 2^m` denotes the single index `[c]`, its suffix is `c` and its
//! length is 1, and the decoder reads no stored byte for it. Entries exist only
//! for codes `>= clear + 2`, so no frame initialises anything and a literal
//! code's stored bytes are unspecified — zero after the reservation, or left
//! from an earlier frame with a smaller `m`.
//!
//! Strings are not materialised. A code's string is the chain
//! `code -> prefix[code] -> ...` read backwards (the chain yields the last index
//! first), and the stored length gives its length, so the decoder emits the
//! indices of a string at positions `q+len-1, ..., q` in that order. The sink is
//! told the position of each index; because every error discards the frame
//! (G12.4), the order of the writes is unobservable, and the decoder reports the
//! error that G10.2/G12.2's forward order gives: the colour error at the
//! smallest position below `A`, else `TooManyPixels` when the string runs past
//! `A`.
//!
//! Table invariant (proved in M2): for every code `c` in `[clear+2, next)`,
//! `prefix[c] < c`, `prefix[c]` is not `clear` or `clear+1`, and
//! `stored_len[c] = code_len(prefix[c]) + 1 <= 4096`, where `code_len(p) = 1`
//! for `p < clear`. Every value written to `prefix` is a code below 4096, so
//! every chain index stays below `TABLE_SIZE`.

use crate::gif::Error;

use vstd::prelude::*;

verus! {

/// Entries of the LZW code table (12-bit codes).
pub const TABLE_SIZE: usize = 4096;

/// G18's `LZW_TABLE_BYTES = 4096*(2 + 1 + 2)`: the one `LzwTable` request.
pub const LZW_TABLE_BYTES: usize = 20_480;

/// Byte offset of the suffix area (prefix codes occupy `[0, 8192)`).
pub const SUFFIX_OFF: usize = 8192;

/// Byte offset of the string-length area (`u16` pairs; `[8192, 12288)` is the
/// suffix area).
pub const LENGTH_OFF: usize = 12_288;

// ---------------------------------------------------------------------------
// L-TABLE-VIEW (G18, D4, R11): the raw storage accessors of the one
// 20,480-byte `try_zeroed(LzwTable)` reservation, and the region separation
// its three setters enjoy.
// ---------------------------------------------------------------------------

/// L-TABLE-VIEW's `prefix(t, c) = t[2c] + 256·t[2c+1]`: code `c`'s stored
/// prefix link, the LE16 pair at byte `2c` (region `[0, 8192)`).
pub open spec fn raw_prefix(t: Seq<u8>, c: int) -> int {
    t[2 * c] as int + 256 * (t[2 * c + 1] as int)
}

/// L-TABLE-VIEW's `suffix(t, c) = t[8192 + c]`: code `c`'s stored suffix byte
/// (region `[8192, 12288)`).
pub open spec fn raw_suffix(t: Seq<u8>, c: int) -> int {
    t[SUFFIX_OFF as int + c] as int
}

/// L-TABLE-VIEW's `stored_length(t, c) = t[12288 + 2c] + 256·t[12288 + 2c + 1]`:
/// code `c`'s stored string length, the LE16 pair at byte `12288 + 2c` (region
/// `[12288, 20480)`).
pub open spec fn raw_length(t: Seq<u8>, c: int) -> int {
    t[LENGTH_OFF as int + 2 * c] as int + 256 * (t[LENGTH_OFF as int + 2 * c + 1] as int)
}

/// The reservation has G18's exact length, `LZW_TABLE_BYTES`. Every accessor and
/// setter below is stated over such a `t`, so every offset it forms is in range
/// for every code below `LZW_CODES` (no read past the region, no wrapping).
pub open spec fn table_len_ok(t: Seq<u8>) -> bool {
    t.len() == LZW_TABLE_BYTES as int
}

/// Code `c`'s stored prefix link (LE16 at byte `2c`). The `.get` fallback is
/// unreachable under `table_len_ok` and `c < LZW_CODES`.
fn prefix_get(t: &[u8], c: u32) -> (r: u16)
    requires
        table_len_ok(t@),
        c < TABLE_SIZE,
    ensures
        r as int == raw_prefix(t@, c as int),
{
    let i = 2 * (c as usize);
    proof {
        reveal(raw_prefix);
        reveal(table_len_ok);
        assert(2 * (c as int) + 1 < t@.len());
    }
    match (t.get(i), t.get(i + 1)) {
        (Some(a), Some(b)) => {
            let r = (*a as u16) | ((*b as u16) << 8);
            proof {
                assert(r as int == (*a as int) + 256 * (*b as int)) by (bit_vector)
                    requires
                        r == ((*a as u16) | ((*b as u16) << 8)),
                ;
                assert(*a == t@[2 * (c as int)]);
                assert(*b == t@[2 * (c as int) + 1]);
            }
            r
        },
        _ => 0,
    }
}

/// Store code `c`'s prefix link. L-TABLE-VIEW: only bytes `2c` and `2c + 1`
/// change.
fn prefix_set(t: &mut [u8], c: u32, v: u16)
    requires
        table_len_ok(old(t)@),
        c < TABLE_SIZE,
    ensures
        final(t)@ == old(t)@.update(2 * (c as int), v as u8).update(2 * (c as int) + 1, (v >> 8) as u8),
{
    let i = 2 * (c as usize);
    proof {
        reveal(table_len_ok);
        assert(i + 1 < t@.len());
    }
    t[i] = v as u8;
    t[i + 1] = (v >> 8) as u8;
}

/// Code `c`'s stored suffix byte. The `.get` fallback is unreachable under
/// `table_len_ok` and `c < LZW_CODES`.
fn suffix_get(t: &[u8], c: u32) -> (r: u8)
    requires
        table_len_ok(t@),
        c < TABLE_SIZE,
    ensures
        r as int == raw_suffix(t@, c as int),
{
    proof {
        reveal(raw_suffix);
        reveal(table_len_ok);
        assert((SUFFIX_OFF as int) + (c as int) < t@.len());
    }
    match t.get(SUFFIX_OFF + c as usize) {
        Some(v) => *v,
        None => 0,
    }
}

/// Store code `c`'s suffix byte. L-TABLE-VIEW: only byte `8192 + c` changes.
fn suffix_set(t: &mut [u8], c: u32, v: u8)
    requires
        table_len_ok(old(t)@),
        c < TABLE_SIZE,
    ensures
        final(t)@ == old(t)@.update(SUFFIX_OFF as int + c as int, v),
{
    proof {
        reveal(table_len_ok);
        assert((SUFFIX_OFF as int) + (c as int) < t@.len());
    }
    t[SUFFIX_OFF + c as usize] = v;
}

/// Code `c`'s stored string length (LE16 at byte `12288 + 2c`). The `.get`
/// fallback is unreachable under `table_len_ok` and `c < LZW_CODES`.
fn stored_len(t: &[u8], c: u32) -> (r: u16)
    requires
        table_len_ok(t@),
        c < TABLE_SIZE,
    ensures
        r as int == raw_length(t@, c as int),
{
    let i = LENGTH_OFF + 2 * (c as usize);
    proof {
        reveal(raw_length);
        reveal(table_len_ok);
        assert((LENGTH_OFF as int) + 2 * (c as int) + 1 < t@.len());
    }
    match (t.get(i), t.get(i + 1)) {
        (Some(a), Some(b)) => {
            let r = (*a as u16) | ((*b as u16) << 8);
            proof {
                assert(r as int == (*a as int) + 256 * (*b as int)) by (bit_vector)
                    requires
                        r == ((*a as u16) | ((*b as u16) << 8)),
                ;
                assert(*a == t@[LENGTH_OFF + 2 * (c as int)]);
                assert(*b == t@[LENGTH_OFF + 2 * (c as int) + 1]);
            }
            r
        },
        _ => 0,
    }
}

/// Store code `c`'s string length. L-TABLE-VIEW: only bytes `12288 + 2c` and
/// `12288 + 2c + 1` change.
fn stored_len_set(t: &mut [u8], c: u32, v: u16)
    requires
        table_len_ok(old(t)@),
        c < TABLE_SIZE,
    ensures
        final(t)@ == old(t)@.update(LENGTH_OFF as int + 2 * (c as int), v as u8).update(
            LENGTH_OFF as int + 2 * (c as int) + 1,
            (v >> 8) as u8,
        ),
{
    let i = LENGTH_OFF + 2 * (c as usize);
    proof {
        reveal(table_len_ok);
        assert(i + 1 < t@.len());
    }
    t[i] = v as u8;
    t[i + 1] = (v >> 8) as u8;
}

/// The three regions do not overlap: for `0 <= c, c2 < LZW_CODES` the prefix
/// pair `{2·c, 2·c+1}`, the suffix byte `SUFFIX_OFF + c2` and the length pair
/// `{LENGTH_OFF + 2·c2, LENGTH_OFF + 2·c2 + 1}` are pairwise distinct. No
/// hypothesis on `c` vs `c2` is needed: the suffix and length regions start
/// above the largest prefix offset (`2·4095 + 1 = 8191`).
pub proof fn lemma_regions_disjoint(c: int, c2: int)
    requires
        0 <= c < TABLE_SIZE,
        0 <= c2 < TABLE_SIZE,
    ensures
        2 * c != SUFFIX_OFF + c2,
        2 * c + 1 != SUFFIX_OFF + c2,
        2 * c2 != SUFFIX_OFF + c,
        2 * c2 + 1 != SUFFIX_OFF + c,
        2 * c2 != LENGTH_OFF + 2 * c,
        2 * c2 + 1 != LENGTH_OFF + 2 * c,
        2 * c2 != LENGTH_OFF + 2 * c + 1,
        2 * c2 + 1 != LENGTH_OFF + 2 * c + 1,
        2 * c != LENGTH_OFF + 2 * c2,
        2 * c + 1 != LENGTH_OFF + 2 * c2,
        2 * c != LENGTH_OFF + 2 * c2 + 1,
        2 * c + 1 != LENGTH_OFF + 2 * c2 + 1,
        SUFFIX_OFF + c2 != LENGTH_OFF + 2 * c,
        SUFFIX_OFF + c2 != LENGTH_OFF + 2 * c + 1,
        SUFFIX_OFF + c != LENGTH_OFF + 2 * c2,
        SUFFIX_OFF + c != LENGTH_OFF + 2 * c2 + 1,
{
    assert(TABLE_SIZE as int == 4096);
    assert(SUFFIX_OFF as int == 8192);
    assert(LENGTH_OFF as int == 12288);
}

/// Two codes' prefix pairs are distinct when the codes are: `2·c` and `2·c+1`
/// meet neither `2·c2` nor `2·c2+1` for `c2 != c`.
pub proof fn lemma_prefix_pairs_disjoint(c: int, c2: int)
    requires
        0 <= c < TABLE_SIZE,
        0 <= c2 < TABLE_SIZE,
        c2 != c,
    ensures
        2 * c2 != 2 * c,
        2 * c2 + 1 != 2 * c,
        2 * c2 != 2 * c + 1,
        2 * c2 + 1 != 2 * c + 1,
{
    assert(TABLE_SIZE as int == 4096);
}

/// Two codes' length pairs are distinct when the codes are.
pub proof fn lemma_length_pairs_disjoint(c: int, c2: int)
    requires
        0 <= c < TABLE_SIZE,
        0 <= c2 < TABLE_SIZE,
        c2 != c,
    ensures
        LENGTH_OFF + 2 * c2 != LENGTH_OFF + 2 * c,
        LENGTH_OFF + 2 * c2 + 1 != LENGTH_OFF + 2 * c,
        LENGTH_OFF + 2 * c2 != LENGTH_OFF + 2 * c + 1,
        LENGTH_OFF + 2 * c2 + 1 != LENGTH_OFF + 2 * c + 1,
{
    assert(TABLE_SIZE as int == 4096);
    assert(LENGTH_OFF as int == 12288);
}

/// L-TABLE-VIEW, the prefix setter: writing code `c`'s prefix link sets
/// `prefix(t, c) = v`, changes no other code's prefix, and changes neither
/// `suffix` nor `stored_length` for any code.
pub proof fn lemma_prefix_set_region(t0: Seq<u8>, t1: Seq<u8>, c: int, v: u16)
    requires
        table_len_ok(t0),
        0 <= c < TABLE_SIZE,
        t1 == t0.update(2 * c, v as u8).update(2 * c + 1, (v >> 8) as u8),
    ensures
        raw_prefix(t1, c) == v as int,
        forall|c2: int| 0 <= c2 < TABLE_SIZE && c2 != c ==> #[trigger] raw_prefix(t1, c2)
            == raw_prefix(t0, c2),
        forall|c2: int| 0 <= c2 < TABLE_SIZE ==> #[trigger] raw_suffix(t1, c2) == raw_suffix(t0, c2),
        forall|c2: int| 0 <= c2 < TABLE_SIZE ==> #[trigger] raw_length(t1, c2) == raw_length(t0, c2),
{
    reveal(raw_prefix);
    reveal(raw_suffix);
    reveal(raw_length);
    reveal(table_len_ok);
    let lo = v as u8;
    let hi = (v >> 8) as u8;
    assert(t1[2 * c] == lo);
    assert(t1[2 * c + 1] == hi);
    assert(lo as int + 256 * (hi as int) == v as int) by (bit_vector)
        requires
            lo == (v as u8),
            hi == ((v >> 8) as u8),
    ;
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE && c2 != c implies #[trigger] raw_prefix(t1, c2)
        == raw_prefix(t0, c2) by {
        lemma_prefix_pairs_disjoint(c, c2);
        assert(t1[2 * c2] == t0[2 * c2]);
        assert(t1[2 * c2 + 1] == t0[2 * c2 + 1]);
    }
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE implies #[trigger] raw_suffix(t1, c2)
        == raw_suffix(t0, c2) by {
        lemma_regions_disjoint(c, c2);
        assert(t1[SUFFIX_OFF + c2] == t0[SUFFIX_OFF + c2]);
    }
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE implies #[trigger] raw_length(t1, c2)
        == raw_length(t0, c2) by {
        lemma_regions_disjoint(c2, c);
        assert(t1[LENGTH_OFF + 2 * c2] == t0[LENGTH_OFF + 2 * c2]);
        assert(t1[LENGTH_OFF + 2 * c2 + 1] == t0[LENGTH_OFF + 2 * c2 + 1]);
    }
}

/// L-TABLE-VIEW, the suffix setter.
pub proof fn lemma_suffix_set_region(t0: Seq<u8>, t1: Seq<u8>, c: int, v: u8)
    requires
        table_len_ok(t0),
        0 <= c < TABLE_SIZE,
        t1 == t0.update(SUFFIX_OFF + c, v),
    ensures
        raw_suffix(t1, c) == v as int,
        forall|c2: int| 0 <= c2 < TABLE_SIZE && c2 != c ==> #[trigger] raw_suffix(t1, c2)
            == raw_suffix(t0, c2),
        forall|c2: int| 0 <= c2 < TABLE_SIZE ==> #[trigger] raw_prefix(t1, c2) == raw_prefix(t0, c2),
        forall|c2: int| 0 <= c2 < TABLE_SIZE ==> #[trigger] raw_length(t1, c2) == raw_length(t0, c2),
{
    reveal(raw_prefix);
    reveal(raw_suffix);
    reveal(raw_length);
    reveal(table_len_ok);
    assert(t1[SUFFIX_OFF + c] == v);
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE && c2 != c implies #[trigger] raw_suffix(t1, c2)
        == raw_suffix(t0, c2) by {
        assert(SUFFIX_OFF + c2 != SUFFIX_OFF + c);
    }
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE implies #[trigger] raw_prefix(t1, c2)
        == raw_prefix(t0, c2) by {
        lemma_regions_disjoint(c2, c);
        assert(t1[2 * c2] == t0[2 * c2]);
        assert(t1[2 * c2 + 1] == t0[2 * c2 + 1]);
    }
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE implies #[trigger] raw_length(t1, c2)
        == raw_length(t0, c2) by {
        lemma_regions_disjoint(c2, c);
        assert(t1[LENGTH_OFF + 2 * c2] == t0[LENGTH_OFF + 2 * c2]);
        assert(t1[LENGTH_OFF + 2 * c2 + 1] == t0[LENGTH_OFF + 2 * c2 + 1]);
    }
}

/// L-TABLE-VIEW, the length setter.
pub proof fn lemma_length_set_region(t0: Seq<u8>, t1: Seq<u8>, c: int, v: u16)
    requires
        table_len_ok(t0),
        0 <= c < TABLE_SIZE,
        t1 == t0.update(LENGTH_OFF + 2 * c, v as u8).update(
            LENGTH_OFF + 2 * c + 1,
            (v >> 8) as u8,
        ),
    ensures
        raw_length(t1, c) == v as int,
        forall|c2: int| 0 <= c2 < TABLE_SIZE && c2 != c ==> #[trigger] raw_length(t1, c2)
            == raw_length(t0, c2),
        forall|c2: int| 0 <= c2 < TABLE_SIZE ==> #[trigger] raw_prefix(t1, c2) == raw_prefix(t0, c2),
        forall|c2: int| 0 <= c2 < TABLE_SIZE ==> #[trigger] raw_suffix(t1, c2) == raw_suffix(t0, c2),
{
    reveal(raw_prefix);
    reveal(raw_suffix);
    reveal(raw_length);
    reveal(table_len_ok);
    let lo = v as u8;
    let hi = (v >> 8) as u8;
    assert(t1[LENGTH_OFF + 2 * c] == lo);
    assert(t1[LENGTH_OFF + 2 * c + 1] == hi);
    assert(lo as int + 256 * (hi as int) == v as int) by (bit_vector)
        requires
            lo == (v as u8),
            hi == ((v >> 8) as u8),
    ;
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE && c2 != c implies #[trigger] raw_length(t1, c2)
        == raw_length(t0, c2) by {
        lemma_length_pairs_disjoint(c, c2);
        assert(t1[LENGTH_OFF + 2 * c2] == t0[LENGTH_OFF + 2 * c2]);
        assert(t1[LENGTH_OFF + 2 * c2 + 1] == t0[LENGTH_OFF + 2 * c2 + 1]);
    }
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE implies #[trigger] raw_prefix(t1, c2)
        == raw_prefix(t0, c2) by {
        lemma_regions_disjoint(c, c2);
        assert(t1[2 * c2] == t0[2 * c2]);
        assert(t1[2 * c2 + 1] == t0[2 * c2 + 1]);
    }
    assert forall|c2: int| 0 <= c2 < TABLE_SIZE implies #[trigger] raw_suffix(t1, c2)
        == raw_suffix(t0, c2) by {
        lemma_regions_disjoint(c2, c);
        assert(t1[SUFFIX_OFF + c2] == t0[SUFFIX_OFF + c2]);
    }
}

// ---------------------------------------------------------------------------
// G9/G10 policy record and G10.2's closed error kinds, adapted from
// `gif/lzw_proof/src/lzw.rs` (M2) so the integrated core can state refinement
// against `crate::lzw_model::ML`. The two executable constructors (`strict`,
// `recommended_beta`) live outside `verus!` below, with the spec records here;
// `lemma_strict_params_eq` pins them.
// ---------------------------------------------------------------------------


/// G9's minimum-code-size policy (`policy-draft/OPTIONS.md` field `lzw_min`) plus
/// the G10.2 switches this milestone's proofs depend on. Strict and RecommendedBeta
/// instantiate the record; nothing in the decoder tests a profile by name, so the
/// model covers every record in the option table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LzwParams {
    /// Smallest accepted `m` (`lzw_min`: 2 for `Spec`, 1 for `Range1to11*`).
    pub m_lo: u8,
    /// Largest accepted `m` (8 for `Spec`, 11 otherwise).
    pub m_hi: u8,
    /// `lzw_min = Range1to11Checked`'s size-1 rule (Q6): `m = 1` only at `A = 1`.
    pub size1_only_one_pixel: bool,
    /// `Range1to11Checked`: a literal above 255 is `ColorIndexOutOfRange`.
    pub checked_literals: bool,
    /// Retained policy bit. Both values reject code 4096: a 12-bit reader cannot
    /// produce it. Entry insertion always stops at 4096, as the policy oracle does;
    /// this bit has no observable effect in the represented policy subset.
    pub deferred_clear: bool,
    /// `lzw_tail = StopAtRaster` (RecommendedBeta): the loop returns as soon as the
    /// raster is complete, so no tail error is reported. `false` is Strict's
    /// `lzw_tail = Strict`.
    pub stop_at_raster: bool,
}

/// Strict's record: G9 keeps `2 <= m <= 8`, `lzw_tail = Strict`, no checked
/// literals, the deferred clear.
pub open spec fn strict_params() -> LzwParams {
    LzwParams {
        m_lo: 2,
        m_hi: 8,
        size1_only_one_pixel: false,
        checked_literals: false,
        deferred_clear: true,
        stop_at_raster: false,
    }
}

/// RecommendedBeta's `lzw_min = Range1to11Checked` plus `lzw_tail = StopAtRaster`.
pub open spec fn recommended_beta_params() -> LzwParams {
    LzwParams {
        m_lo: 1,
        m_hi: 11,
        size1_only_one_pixel: true,
        checked_literals: true,
        deferred_clear: true,
        stop_at_raster: true,
    }
}


// ---------------------------------------------------------------------------
// L-TABLE-VIEW's code-level view of the table (`THEOREMS.md` T-LZW
// sub-obligations, R11): the string a code denotes, read from the three regions
// through `raw_prefix`/`raw_suffix`/`raw_length` (the code-level functions
// `chain::str_of`/`root_of`/`code_len` do the walk).
// ---------------------------------------------------------------------------

/// `code_suffix(t, m, c) = if c < clear then c else suffix(t, c)`: a literal's
/// suffix is the code value, not a stored byte (R11).
pub open spec fn code_suffix(t: Seq<u8>, m: int, c: int) -> int {
    if c < crate::chain::ipow2(m) {
        c
    } else {
        raw_suffix(t, c)
    }
}

/// `code_first(t, m, c)`: the first index of code `c`'s string. A literal's is
/// the code value; a stored code's is the chain root, computed by the walk.
pub open spec fn code_first(t: Seq<u8>, m: int, c: int, fuel: nat) -> int {
    crate::chain::root_of_from_raw(t, crate::chain::ipow2(m), c, fuel)
}

/// `chain(t, m, c)`: the index string code `c` denotes, in emitting order, as
/// the code-level walk over the raw accessors.
pub open spec fn chain(t: Seq<u8>, m: int, c: int, fuel: nat) -> Seq<u8> {
    crate::chain::str_of_raw(t, crate::chain::ipow2(m), c, fuel)
}

/// G10.1: the string length of code `c`. A literal code's is 1 by rule; every
/// other code's is the stored length, read only for codes `>= clear + 2`.
fn code_len(t: &[u8], clear: u32, c: u32) -> (r: u16)
    requires
        table_len_ok(t@),
        c < TABLE_SIZE,
    ensures
        c < clear ==> r == 1,
        c >= clear ==> r as int == raw_length(t@, c as int),
{
    if c < clear {
        1
    } else {
        stored_len(t, c)
    }
}

} // verus!

// `Sink`, `Tail` and `LzwOptions` are inside the second `verus!` block below:
// the trait's contract names its ghost call log and validity predicate, and
// `decode`'s contract names the two `LzwOptions` fields.

verus! {

/// Receives the decoded indices. `put(q, v)` handles index `v` at pixel
/// position `q < A`; it returns `false` for a colour error
/// (`ColorIndexOutOfRange`, G12.2), `true` otherwise.
///
/// The contract `decode` needs is the sink's **ghost call log** `H` and its
/// validity predicate: the sink's `H` records `(q, v)` for every completed call
/// (in call order), and its Boolean is `ok(v)` — a function of the offered
/// index alone, independent of earlier calls and of call order. A sink that
/// decided its Boolean from accumulated state would make `decode`'s fault
/// tracking unsound, so the log and `ok` are part of the trait.
///
/// `Drawer`'s instances are the compositor's; the blame-free `draw_step`
/// relation between `put` and the canvas is M4b's (design memo step 2's Drawer
/// clause) and is not stated here, where the compositor's model is absent.
pub trait Sink {
    /// The sink's immutable configuration: the raster `A`. `decode` offers
    /// positions below it only, so this is what makes `put`'s precondition
    /// discharge at every call.
    spec fn sink_area(&self) -> int;

    /// The sink's own state invariant; `put` requires it and preserves it.
    spec fn sink_inv(&self) -> bool;

    /// The index-validity predicate `ok`: whether an offered index is accepted.
    spec fn sink_ok(&self, v: u8) -> bool;

    spec fn sink_log(&self) -> Seq<(u32, u8)>;

    /// Current private drawing state, observed without exposing an error canvas.
    spec fn sink_view(&self) -> Seq<u8>;

    /// Immutable configuration's action on one put; rejection may be identity.
    spec fn sink_effect(&self) -> spec_fn(Seq<u8>, u32, u8) -> Seq<u8>;

    /// Endpoint of a borrowed drawing target. Preserving this observation
    /// prevents a sink call from replacing the caller's mutable borrow.
    #[verifier::prophetic]
    open spec fn sink_future(&self) -> Seq<u8> { Seq::empty() }

    fn put(&mut self, q: u32, v: u8) -> (r: bool)
        requires
            (q as int) < old(self).sink_area(),
            old(self).sink_inv(),
        ensures
            r == final(self).sink_ok(v),
            final(self).sink_log() == old(self).sink_log().push((q, v)),
            final(self).sink_inv(),
            final(self).sink_area() == old(self).sink_area(),
            forall|x: u8| final(self).sink_ok(x) == old(self).sink_ok(x),
            final(self).sink_future() == old(self).sink_future(),
            final(self).sink_effect() == old(self).sink_effect(),
            final(self).sink_view() == old(self).sink_effect()(old(self).sink_view(),q,v),
            final(self).sink_view().len() == old(self).sink_view().len(),
    ;
}

/// What happens after the raster is complete (policy field `lzw_tail`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tail {
    /// G10.2 steps 1, 4, 5, 8.
    Strict,
    /// Stop when `q = A`; indices past `A` are discarded (event E4).
    StopAtRaster,
}

/// Options of one LZW call that come from the policy record.
#[derive(Clone, Copy)]
pub struct LzwOptions {
    pub tail: Tail,
    /// `lzw_min = Range1to11Checked`: a literal code in `256..clear` is
    /// `ColorIndexOutOfRange` when read.
    pub checked_literals: bool,
}

/// LSB-first bit reader over a data sub-block sequence whose structure was
/// already checked by `parse::scan_image_data` (G9): the sequence starts with a
/// size byte at `pos`, holds `data_len` payload bytes, and ends with a zero size
/// byte. `bits_left` counts the bits of the payload not yet consumed; a read is
/// made only when `bits_left >= width`, so it never needs a byte past the
/// payload. A byte that is not there reads as 0 (unreachable after the scan;
/// the fallback keeps every access in bounds without relying on it).
///
/// L-BITS (`THEOREMS.md` T6): `seq_start` is the index of the sequence's first
/// size byte and `fetched` the number of payload bytes the reader has returned,
/// so `crate::model::rd_state(b@, seq_start, fetched)` is its `(pos, left_in_block)`
/// and `crate::model::acc_view(b@, seq_start, fetched, nacc, acc)` says what its low
/// `nacc` accumulator bits hold. `bits_left` counts the payload bits `read` has
/// not consumed, so `bit_pos() == 8·fetched − nacc`.
pub struct BitReader<'a> {
    /// The input; `b@` is the model's byte sequence.
    pub b: &'a [u8],
    /// Index in `b` of the next byte to fetch (payload byte or size byte).
    pub pos: usize,
    /// Payload bytes left in the current sub-block.
    pub left_in_block: usize,
    pub acc: u32,
    pub nacc: u32,
    pub bits_left: u32,
    pub total_bits: u32,
    /// Ghost: the index in `b` of the first sub-block's size byte.
    pub ghost seq_start: usize,
    /// Ghost: the payload bytes fetched so far.
    pub ghost fetched: u32,
    /// Ghost: `|D|`, the payload length of the sequence (G9's scan total).
    pub ghost data_len: u32,
}

impl<'a> BitReader<'a> {
    /// Bit position after the codes read so far (`e` in G10.2 step 5): the
    /// payload bits consumed, `8·fetched − nacc`.
    pub closed spec fn bit_pos_spec(&self) -> u32 {
        (self.total_bits - self.bits_left) as u32
    }

    /// `first` is the index of the first sub-block's size byte; `data_len` the
    /// payload length (`|D|`, at most `MAX_INPUT`, so `8·|D|` fits in `u32`).
    pub fn new(b: &'a [u8], first: usize, data_len: u32) -> (r: BitReader<'a>)
        requires
            data_len <= crate::limits::MAX_INPUT,
        ensures
            r.seq_start == first,
            r.fetched == 0u32,
            r.data_len == data_len,
            r.pos == first,
            r.left_in_block == 0,
            r.nacc == 0u32,
            r.acc == 0u32,
            r.total_bits == data_len * 8,
            r.bits_left == data_len * 8,
            r.bit_pos_spec() == 0u32,
            r.b == b,
    {
        let total_bits = data_len * 8;
        BitReader {
            b,
            pos: first,
            left_in_block: 0,
            acc: 0,
            nacc: 0,
            bits_left: total_bits,
            total_bits,
            seq_start: first,
            fetched: 0,
            data_len,
        }
    }

    /// Fetch the reader's current payload byte (`crate::model::payload_byte` at
    /// `fetched`) and move `pos`/`left_in_block` to the reader state after that
    /// byte. `fetched` is not advanced here: `read` advances it once the byte is
    /// in the accumulator, so `acc_view` and `lemma_acc_push` line up.
    /// `decode` establishes the precondition from G9's structural scan
    /// (`crate::model::lemma_rd_step_ok_all`).
    fn fetch_byte(&mut self) -> (v: u8)
        requires
            old(self).pos as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
            ).0,
            old(self).left_in_block as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
            ).1,
            old(self).b@.len() == old(self).b.len(),
            old(self).b.len() <= crate::limits::MAX_INPUT,
            crate::model::rd_step_ok(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
            ),
        ensures
            final(self).fetched == old(self).fetched,
            final(self).seq_start == old(self).seq_start,
            final(self).b == old(self).b,
            final(self).acc == old(self).acc,
            final(self).nacc == old(self).nacc,
            final(self).total_bits == old(self).total_bits,
            final(self).bits_left == old(self).bits_left,
            final(self).data_len == old(self).data_len,
            final(self).b@.len() == final(self).b.len(),
            final(self).pos as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int + 1,
            ).0,
            final(self).left_in_block as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int + 1,
            ).1,
            v as int == crate::model::payload_byte(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
            ),
    {
        let bp = self.b;
        let ghost bs = self.seq_start as int;
        let ghost bf = self.fetched as int;
        let ghost p = self.pos as int;
        let ghost l = self.left_in_block as int;
        proof {
            reveal(crate::model::rd_step_ok);
            reveal(crate::model::rd_state);
            reveal(crate::model::rd_next_off);
            reveal(crate::model::payload_byte);
            reveal(crate::model::have);
        }
        let v: u8;
        if self.left_in_block == 0 {
            // A size byte at `pos`; `rd_step_ok` gives it positive with its
            // payload present, so the following byte is the block's first
            // payload byte.
            let size = self.b[self.pos];
            proof {
                assert(crate::model::have(bp@, p, 1));
                assert(bp@[p] as int == size as int);
                assert(size as int >= 1);
                assert(crate::model::have(bp@, p + 1, size as int));
                assert(p + 2 <= bp@.len()) by (nonlinear_arith)
                    requires
                        p + 1 + (size as int) <= bp@.len(),
                        size as int >= 1,
                ;
            }
            v = self.b[self.pos + 1];
            self.pos = self.pos + 2;
            self.left_in_block = (size as usize) - 1;
            proof {
                reveal(crate::model::rd_state);
                assert(crate::model::rd_state(bp@, bs, bf) == (p, 0));
                assert(crate::model::rd_state(bp@, bs, bf + 1) == (
                    p + 2,
                    if size as int > 0 {
                        (size as int) - 1
                    } else {
                        0
                    },
                ));
                assert(v as int == bp@[p + 1]);
                assert(crate::model::payload_byte(bp@, bs, bf) == v) by {
                    reveal(crate::model::payload_byte);
                    reveal(crate::model::have);
                    assert(bp@[p] as int == size as int);
                }
            }
        } else {
            proof {
                assert(crate::model::have(bp@, p, 1));
                assert(p + 1 <= bp@.len()) by (nonlinear_arith)
                    requires
                        p + 1 <= bp@.len(),
                ;
            }
            v = self.b[self.pos];
            self.pos = self.pos + 1;
            self.left_in_block = self.left_in_block - 1;
            proof {
                reveal(crate::model::rd_state);
                assert(l >= 1);
                assert(crate::model::rd_state(bp@, bs, bf) == (p, l));
                assert(crate::model::rd_state(bp@, bs, bf + 1) == (p + 1, l - 1));
                assert(v as int == bp@[p]);
                assert(crate::model::payload_byte(bp@, bs, bf) == v) by {
                    reveal(crate::model::payload_byte);
                    reveal(crate::model::have);
                }
            }
        }
        v
    }

    /// Read `width` bits (`1 <= width <= 12`); `bits_left >= width` is the
    /// caller's, established by `decode`'s G10.2 step 1. Returns the code at the
    /// reader's position and advances the position by `width`.
    ///
    /// `fetched <= data_len`, not `<`: a read whose bits are all already in the
    /// accumulator after the last payload byte was fetched consumes no byte, so
    /// `fetched == data_len` is a reachable entry state (the EOI read of
    /// `m = 2, D = [140, 11], A = 5`). The fill loop proves `fetched < data_len`
    /// itself whenever it needs another byte, from `bits_left >= width` and
    /// `nacc < width`.
    ///
    /// Contracts are stated on the fields' integer values: the position in
    /// payload bits is `(total_bits as int) - (bits_left as int)`, and the last
    /// precondition is the caller's standing hypothesis from G9's scan of the
    /// whole sub-block sequence (`crate::model::lemma_rd_step_ok_all`).
    fn read(&mut self, width: u32) -> (code: u32)
        requires
            old(self).bits_left >= width,
            1 <= width <= 12,
            old(self).b@.len() == old(self).b.len(),
            old(self).b.len() <= crate::limits::MAX_INPUT,
            old(self).pos as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
            ).0,
            old(self).left_in_block as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
            ).1,
            old(self).bits_left <= old(self).total_bits,
            old(self).total_bits == 8 * (old(self).data_len as int),
            old(self).nacc as int == 8 * (old(self).fetched as int) - (
            (old(self).total_bits as int) - (old(self).bits_left as int)),
            old(self).nacc <= 32,
            old(self).fetched <= old(self).data_len,
            crate::model::acc_view(
                old(self).b@,
                old(self).seq_start as int,
                old(self).fetched as int,
                old(self).nacc as int,
                old(self).acc,
            ),
            forall|j: int|
                old(self).fetched as int <= j && j < old(self).data_len as int ==> #[trigger] crate::model::rd_step_ok(
                    old(self).b@,
                    old(self).seq_start as int,
                    j,
                ),
        ensures
            final(self).b == old(self).b,
            final(self).seq_start == old(self).seq_start,
            final(self).data_len == old(self).data_len,
            final(self).total_bits == old(self).total_bits,
            final(self).bits_left == old(self).bits_left - width,
            final(self).bits_left <= final(self).total_bits,
            final(self).total_bits == 8 * (final(self).data_len as int),
            final(self).pos as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                final(self).fetched as int,
            ).0,
            final(self).left_in_block as int == crate::model::rd_state(
                old(self).b@,
                old(self).seq_start as int,
                final(self).fetched as int,
            ).1,
            final(self).fetched <= final(self).data_len,
            final(self).nacc as int == 8 * (final(self).fetched as int) - (
            (final(self).total_bits as int) - (final(self).bits_left as int)),
            final(self).nacc <= 32,
            crate::model::acc_view(
                old(self).b@,
                old(self).seq_start as int,
                final(self).fetched as int,
                final(self).nacc as int,
                final(self).acc,
            ),
            forall|j: int|
                final(self).fetched as int <= j && j < final(self).data_len as int ==> #[trigger] crate::model::rd_step_ok(
                    old(self).b@,
                    old(self).seq_start as int,
                    j,
                ),
            code as int == crate::model::payload_code(
                old(self).b@,
                old(self).seq_start as int,
                (old(self).total_bits as int) - (old(self).bits_left as int),
                width as int,
                0,
            ),
    {
        let gb = self.b;
        let ghost gs = self.seq_start as int;
        let ghost gf = self.fetched as int;
        let ghost gd = self.data_len as int;
        let ga0 = self.nacc;
        proof {
            assert(ga0 as int >= 0);
        }
        let ghost ga = ga0 as int;
        let ghost gp = (self.total_bits as int) - (self.bits_left as int);
        let ebits = self.total_bits;
        let ebl = self.bits_left;
        proof {
            reveal(crate::model::acc_view);
            reveal(crate::model::rd_state);
            reveal(crate::model::have);
            assert(8 * (gf) == gp + ga);
            assert(self.bits_left == ebl);
            assert(ebl as int == (ebits as int) - gp);
            assert(self.bits_left <= self.total_bits);
            assert(self.nacc as int == 8 * (self.fetched as int) - gp);
            // `nacc = 8·fetched − gp` with `fetched <= data_len` gives the entry
            // bound `nacc <= bits_left`: `fetched == data_len` is the state where
            // every payload bit is buffered and no fill is needed at all.
            assert(self.nacc <= self.bits_left) by (nonlinear_arith)
                requires
                    self.nacc as int == 8 * (self.fetched as int) - gp,
                    self.bits_left as int == 8 * (self.data_len as int) - gp,
                    self.fetched <= self.data_len,
            ;
        }
        let mut bytes = 0u32;
        while self.nacc < width
            invariant
                1 <= width <= 12,
                self.b == gb,
                self.b@.len() == self.b.len(),
                self.b.len() <= crate::limits::MAX_INPUT,
                self.total_bits == ebits,
                self.data_len == old(self).data_len,
                self.total_bits == 8 * (self.data_len as int),
                self.seq_start == old(self).seq_start,
                (self.seq_start as int) == gs,
                self.fetched as int == (gf) + (bytes as int),
                self.nacc as int == (ga) + 8 * (bytes as int),
                ga >= 0,
                self.bits_left == ebl,
                self.bits_left >= width,
                self.bits_left <= self.total_bits,
                (gd) == self.data_len as int,
                self.bits_left as int == (ebits as int) - gp,
                self.nacc as int == 8 * (self.fetched as int) - gp,
                self.fetched <= self.data_len,
                bytes <= 2u32,
                self.nacc <= 32,
                self.pos as int == crate::model::rd_state(
                    self.b@,
                    gs,
                    self.fetched as int,
                ).0,
                self.left_in_block as int == crate::model::rd_state(
                    self.b@,
                    gs,
                    self.fetched as int,
                ).1,
                crate::model::acc_view(self.b@, gs, self.fetched as int, self.nacc as int, self.acc),
                forall|j: int|
                    self.fetched as int <= j && j < (gd) ==> #[trigger] crate::model::rd_step_ok(
                        self.b@,
                        gs,
                        j,
                    ),
            decreases 32u32 - self.nacc,
        {
            proof {
                // 8·fetched = gp + nacc with nacc < width and gp = 8·|D| − bits_left
                // at bits_left >= width, so fetched + 1 <= |D|.
                assert(8 * (self.fetched as int) == gp + (self.nacc as int)) by (nonlinear_arith)
                    requires
                        self.nacc as int == (ga as int) + 8 * (bytes as int),
                        self.fetched as int == (gf as int) + (bytes as int),
                        8 * (gf as int) == gp + (ga as int),
                ;
                assert((self.fetched as int) + 1 <= gd) by (nonlinear_arith)
                    requires
                        self.bits_left as int == (ebits as int) - gp,
                        ebits as int == 8 * (gd),
                        self.bits_left >= width,
                        self.nacc < width,
                        8 * (self.fetched as int) == gp + (self.nacc as int),
                ;
                assert(crate::model::rd_step_ok(self.b@, gs, self.fetched as int));
            }
            assert(self.pos as int == crate::model::rd_state(
                self.b@,
                gs,
                self.fetched as int,
            ).0);
            assert(self.left_in_block as int == crate::model::rd_state(
                self.b@,
                gs,
                self.fetched as int,
            ).1);
            let byte = self.fetch_byte();
            proof {
                assert(byte as int == crate::model::payload_byte(
                    self.b@,
                    gs,
                    self.fetched as int,
                ));
                reveal(crate::model::acc_view);
                assert(self.nacc + 8 <= 32) by (nonlinear_arith)
                    requires
                        self.nacc < width,
                        width <= 12,
                ;
                crate::model::lemma_acc_push(
                    self.b@,
                    gs,
                    self.fetched as int,
                    self.nacc as int,
                    self.acc,
                    byte,
                );
            }
            self.acc = self.acc | ((byte as u32) << self.nacc);
            proof {
                assert(self.nacc <= 32);
                assert((self.nacc as int) + 8 <= 32) by (nonlinear_arith)
                    requires
                        self.nacc < width,
                        width <= 12,
                ;
                assert(8 * (bytes as int) <= self.nacc as int) by (nonlinear_arith)
                    requires
                        self.nacc as int == (ga) + 8 * (bytes as int),
                ga >= 0,
                        ga >= 0,
                ;
                assert(bytes as int <= 1) by (nonlinear_arith)
                    requires
                        8 * (bytes as int) <= self.nacc as int,
                        self.nacc < width,
                        width <= 12,
                ;
                assert(bytes as int + 1 <= 2) by (nonlinear_arith)
                    requires
                        bytes as int <= 1,
                ;
            }
            self.nacc += 8;
            proof {
                self.fetched = (self.fetched + 1) as u32;
                reveal(crate::model::rd_state);
            }
            bytes += 1;
        }
        let nacc2 = self.nacc;
        proof {
            crate::model::lemma_acc_take(
                self.b@,
                gs,
                self.fetched as int,
                self.nacc as int,
                self.acc,
                width as int,
            );
            reveal(crate::model::mask32);
        }
        let maskw: u32 = (1u32.wrapping_shl(width)).wrapping_sub(1u32);
        let code = self.acc & maskw;
        let accv = self.acc >> width;
        let naccv = nacc2 - width;
        let ebl2 = self.bits_left;
        let blv = ebl2 - width;
        proof {
            assert(maskw == crate::model::mask32(width)) by {
                reveal(crate::model::mask32);
                assert(((1u32 << width) - 1u32) == maskw) by (bit_vector)
                    requires
                        width <= 12,
                        maskw == (1u32.wrapping_shl(width)).wrapping_sub(1u32),
                ;
            }
            assert(code == self.acc & crate::model::mask32(width));
            reveal(crate::model::acc_view);
            assert(blv as int == (ebl2 as int) - (width as int)) by (nonlinear_arith)
                requires
                    blv == ebl2 - width,
            ;
            assert(naccv as int == (nacc2 as int) - (width as int)) by (nonlinear_arith)
                requires
                    naccv == nacc2 - width,
            ;
            assert(nacc2 as int == 8 * (self.fetched as int) - gp);
            assert(naccv as int == 8 * (self.fetched as int) - (gp + (width as int))) by (
            nonlinear_arith)
                requires
                    naccv as int == (nacc2 as int) - (width as int),
                    nacc2 as int == 8 * (self.fetched as int) - gp,
            ;
            assert((self.total_bits as int) - (blv as int) == gp + (width as int)) by (nonlinear_arith)
                requires
                    self.total_bits == ebits,
                    ebl == (ebits as int) - gp,
                    ebl2 == ebl,
                    blv as int == (ebl2 as int) - (width as int),
            ;
            assert(crate::model::acc_view(self.b@, gs, self.fetched as int, naccv as int, accv));
        }
        self.acc = accv;
        self.nacc = naccv;
        self.bits_left = blv;
        code
    }

    /// Bit position after the codes read so far (`e` in G10.2 step 5).
    fn bit_pos(&self) -> (e: u32)
        requires
            self.bits_left <= self.total_bits,
        ensures
            e == self.bit_pos_spec(),
    {
        self.total_bits - self.bits_left
    }
}


// ---------------------------------------------------------------------------
// Step 2 of the `lzw::decode` contract: the `Sink` observation.
//
// `decode` writes each index through `Sink::put(q, v)` and keeps no buffer
// (G18). The observable is the sequence of (position, index) pairs it offers,
// in the order G10.2 step 8 walks the string: the string's indices at positions
// `q+len-1` down to `q`, each offered only when the position is below the raster
// `A` (`pos < area`; G12.2's clip). `sink_pairs` is that sequence as a spec
// function, so a ghost log accumulated in `decode` is compared against it.
// ---------------------------------------------------------------------------

/// The pairs G10.2 step 8 offers the sink for a string `s` from position `q`
/// with raster `a`: `(q + len - 1, s[len - 1])` first, then down to `(q, s[0])`,
/// skipping positions at or above `a`. `k` is the walk's step count, so `k = 0`
/// is the whole sequence.
pub open spec fn sink_pairs(s: Seq<u8>, q: int, a: int, k: int) -> Seq<(u32, u8)>
    decreases s.len() - k,
{
    if k >= s.len() {
        Seq::empty()
    } else {
        let j = s.len() - 1 - k;
        let pos = q + j;
        let rest = sink_pairs(s, q, a, k + 1);
        if pos < a {
            seq![(pos as u32, s[j])] + rest
        } else {
            rest
        }
    }
}

/// Every offered pair is one of the string's index/position pairs.
pub proof fn lemma_sink_pairs_ok(s: Seq<u8>, q: int, a: int, k: int)
    requires
        0 <= k,
        q >= 0,
    ensures
        forall|i: int|
            0 <= i < sink_pairs(s, q, a, k).len() ==> (exists|j: int|
                0 <= j < s.len() && #[trigger] sink_pairs(s, q, a, k)[i] == (
                    (q + j) as u32,
                    s[j],
                )),
    decreases s.len() - k,
{
    reveal(sink_pairs);
    if k >= s.len() {
    } else {
        lemma_sink_pairs_ok(s, q, a, k + 1);
        let j0 = s.len() - 1 - k;
        let rest = sink_pairs(s, q, a, k + 1);
        if q + j0 < a {
            assert(sink_pairs(s, q, a, k) == seq![((q + j0) as u32, s[j0])] + rest);
            assert forall|i: int|
                0 <= i < sink_pairs(s, q, a, k).len() implies (exists|j: int|
                    0 <= j < s.len() && #[trigger] sink_pairs(s, q, a, k)[i] == (
                        (q + j) as u32,
                        s[j],
                    )) by {
                if i == 0 {
                } else {
                    assert(sink_pairs(s, q, a, k)[i] == rest[i - 1]);
                }
            }
        } else {
            assert(sink_pairs(s, q, a, k) == rest);
        }
    }
}

/// Every index of the string at a position below the raster is offered once, as
/// long as the walk has not yet passed it. The domain `k <= |s| - 1 - j` is
/// exactly "step `k` still has `j` ahead": the walk visits `s[|s|-1]` at `k = 0`
/// and `s[j]` at `k = |s| - 1 - j`, so the induction decreases `|s| - 1 - j - k`.
pub proof fn lemma_sink_pairs_complete(s: Seq<u8>, q: int, a: int, k: int, j: int)
    requires
        0 <= k,
        0 <= j < s.len(),
        k <= s.len() - 1 - j,
        q >= 0,
        q + j < a,
    ensures
        exists|i: int|
            0 <= i < sink_pairs(s, q, a, k).len() && #[trigger] sink_pairs(
                s,
                q,
                a,
                k,
            )[i].0 == (q + j) as u32,
    decreases (s.len() - 1 - j) - k,
{
    reveal(sink_pairs);
    let j0 = s.len() - 1 - k;
    let rest = sink_pairs(s, q, a, k + 1);
    if j0 == j {
        assert(sink_pairs(s, q, a, k) == seq![((q + j) as u32, s[j])] + rest);
        assert(sink_pairs(s, q, a, k).len() >= 1);
        assert(sink_pairs(s, q, a, k)[0].0 == (q + j) as u32);
    } else {
        assert(k < s.len() - 1 - j);
        lemma_sink_pairs_complete(s, q, a, k + 1, j);
        let i0 = choose|i: int| 0 <= i < rest.len() && #[trigger] rest[i].0 == (q + j) as u32;
        assert(0 <= i0 < rest.len() && rest[i0].0 == (q + j) as u32);
        if q + j0 < a {
            let head = seq![((q + j0) as u32, s[j0])];
            assert(sink_pairs(s, q, a, k) == head + rest);
            vstd::seq::lemma_seq_add_index2(head, rest, i0 + 1);
            assert((head + rest)[i0 + 1] == rest[i0]);
            assert(0 <= i0 + 1 < (head + rest).len());
        } else {
            assert(sink_pairs(s, q, a, k) == rest);
        }
    }
}




#[cfg(verus_only)]
use crate::chain::{chain_ok, fuel_above};
#[cfg(verus_only)]
use crate::lzw_model::{ErrK, MRes};

// ---------------------------------------------------------------------------
// The bridge step 3 needs: the chain walk stated over the array form
// (`chain::str_of`/`root_of` on `pf`/`sf`) and over the reservation
// (`chain::str_of_raw`/`root_of_from_raw` on `raw_prefix`/`raw_suffix`) name the
// same string whenever the two agree on the readable range `[clear+2, c]`. This
// is `refinement::table_agreement` with the prefix/suffix sequences built from
// the raw accessors, so every M2 chain lemma applies to `decode`'s `t`.
// ---------------------------------------------------------------------------

/// Agreement on the readable stored range: `t`'s raw prefix and suffix of every
/// entry below `c` equal the array form's `pf`/`sf`.
pub open spec fn table_agrees(t: Seq<u8>, pf: Seq<u16>, sf: Seq<u8>, clear: int, c: int) -> bool {
    forall|x: int|
        clear + 2 <= x <= c ==> #[trigger] crate::lzw::raw_prefix(t, x) == pf[x] as int
            && crate::lzw::raw_suffix(t, x) == sf[x] as int
}

/// The raw prefix sequence of a reservation, as the array `table_agreement` takes.
pub open spec fn raw_pf_seq(t: Seq<u8>) -> Seq<u16> {
    Seq::new(crate::lzw::TABLE_SIZE as nat, |i: int| crate::lzw::raw_prefix(t, i) as u16)
}

/// The raw suffix sequence of a reservation.
pub open spec fn raw_sf_seq(t: Seq<u8>) -> Seq<u8> {
    Seq::new(crate::lzw::TABLE_SIZE as nat, |i: int| crate::lzw::raw_suffix(t, i) as u8)
}

/// The raw-form link invariant: every stored entry at or below `c` has a parent
/// link that is a strictly smaller code (the raw counterpart of `chain::link_ok`,
/// which is the fact `decode`'s table maintains).
pub open spec fn raw_link_ok(t: Seq<u8>, clear: int, c: int) -> bool {
    forall|x: int|
        clear + 2 <= x <= c ==> 0 <= #[trigger] crate::chain::raw_pf(t, x) && (
        crate::chain::raw_pf(t, x) < clear || clear + 2 <= crate::chain::raw_pf(t, x) < x)
}

/// The raw walk is the array walk over `raw_pf_seq`/`raw_sf_seq`: both recurse on
/// the parent code and the entry's suffix, so the agreement is definitional once
/// the parent links match.
pub proof fn lemma_raw_walk_eq_seq(t: Seq<u8>, clear: int, c: int, fuel: nat)
    requires
        0 <= c < crate::lzw::TABLE_SIZE,
        c < clear || clear + 2 <= c,
        fuel_above(c, fuel),
        raw_link_ok(t, clear, c),
    ensures
        crate::chain::str_of_raw(t, clear, c, fuel) == crate::chain::str_of(
            raw_sf_seq(t),
            raw_pf_seq(t),
            clear,
            c,
            fuel,
        ),
        crate::chain::root_of_from_raw(t, clear, c, fuel) == crate::chain::root_of(
            raw_pf_seq(t),
            clear,
            c,
            fuel,
        ),
    decreases fuel,
{
    reveal(raw_link_ok);
    reveal(fuel_above);
    if fuel == 0 {
    } else if c < clear {
    } else {
        let p = crate::chain::raw_pf(t, c);
        assert(raw_link_ok(t, clear, c));
        assert(clear + 2 <= c) by (nonlinear_arith)
            requires
                !(c < clear),
                c < clear || clear + 2 <= c,
        ;
        assert(0 <= p);
        if p >= clear {
            assert(clear + 2 <= p < c);
            assert(raw_link_ok(t, clear, p)) by {
                assert forall|x: int| clear + 2 <= x <= p implies 0 <= #[trigger] crate::chain::raw_pf(
                    t,
                    x,
                ) && (crate::chain::raw_pf(t, x) < clear || clear + 2 <= crate::chain::raw_pf(
                    t,
                    x,
                ) < x) by {
                    assert(x <= c);
                }
            }
        } else {
            assert(raw_link_ok(t, clear, p)) by {
                assert forall|x: int| clear + 2 <= x <= p implies 0 <= #[trigger] crate::chain::raw_pf(
                    t,
                    x,
                ) && (crate::chain::raw_pf(t, x) < clear || clear + 2 <= crate::chain::raw_pf(
                    t,
                    x,
                ) < x) by {
                    assert(x <= c);
                }
            }
        }
        reveal(raw_pf_seq);
        reveal(raw_sf_seq);
        assert(raw_pf_seq(t)[c] as int == p);
        assert(raw_sf_seq(t)[c] as int == crate::lzw::raw_suffix(t, c));
        assert(fuel_above(p, (fuel - 1) as nat));
        lemma_raw_walk_eq_seq(t, clear, p, (fuel - 1) as nat);
    }
}

/// The array walk and the raw walk name the same string and root when the two
/// agree on `[clear+2, c]`. The bridge `decode` cites: its table is the
/// reservation `t`, while the model's `semit`/`first_of` read `pf`/`sf`.
pub proof fn lemma_walk_agrees(
    t: Seq<u8>,
    pf: Seq<u16>,
    sf: Seq<u8>,
    clear: int,
    c: int,
    fuel: nat,
)
    requires
        0 <= c < crate::lzw::TABLE_SIZE,
        chain_ok(pf, clear, c),
        fuel_above(c, fuel),
        pf.len() == crate::lzw::TABLE_SIZE,
        sf.len() == crate::lzw::TABLE_SIZE,
        table_agrees(t, pf, sf, clear, c),
    ensures
        crate::chain::str_of(sf, pf, clear, c, fuel) == crate::chain::str_of_raw(t, clear, c, fuel),
        crate::chain::root_of(pf, clear, c, fuel) == crate::chain::root_of_from_raw(t, clear, c, fuel),
{
    reveal(table_agrees);
    reveal(crate::chain::chain_ok);
    reveal(crate::chain::link_ok);
    assert(raw_link_ok(t, clear, c)) by {
        assert forall|x: int| clear + 2 <= x <= c implies 0 <= #[trigger] crate::chain::raw_pf(
            t,
            x,
        ) && (crate::chain::raw_pf(t, x) < clear || clear + 2 <= crate::chain::raw_pf(t, x) < x) by {
            assert(crate::lzw::raw_prefix(t, x) == pf[x] as int);
            assert(crate::chain::link_ok(pf, clear, x));
        }
    }
    lemma_raw_walk_eq_seq(t, clear, c, fuel);
    crate::refinement::table_agreement(
        sf,
        pf,
        raw_sf_seq(t),
        raw_pf_seq(t),
        clear,
        c,
        fuel,
    );
}



/// The walk stepped by one: `sink_pairs` at step `k + 1` over `s` is step `k`
/// over `s`'s first `|s| - 1` indices (the position `q + |s| - 1` is skipped and
/// every later position is unchanged). This is the induction step the count lemma
/// runs on.
pub proof fn lemma_sink_pairs_peel(s: Seq<u8>, q: int, a: int, k: int)
    requires
        s.len() >= 1,
        0 <= k,
    ensures
        sink_pairs(s, q, a, k + 1) == sink_pairs(s.take((s.len() - 1) as int), q, a, k),
    decreases (s.len() + 1) as nat - (k as nat),
{
    reveal(sink_pairs);
    let t = s.take((s.len() - 1) as int);
    if k + 1 >= s.len() {
        assert(sink_pairs(s, q, a, k + 1) == Seq::< (u32,u8) >::empty());
        assert(k >= t.len());
        assert(sink_pairs(t, q, a, k) == Seq::< (u32,u8) >::empty());
    } else {
        if k == t.len() {
            assert(sink_pairs(s, q, a, k + 1) == Seq::< (u32,u8) >::empty());
            assert(sink_pairs(t, q, a, k) == Seq::< (u32,u8) >::empty());
        } else {
            lemma_sink_pairs_peel(s, q, a, k + 1);
        }
    }
}

/// The count: `sink_pairs` from step `k` offers `min(|s| - k, A - q)` pairs, the
/// string indices still ahead of the walk whose position is below the raster.
pub proof fn lemma_sink_pairs_len(s: Seq<u8>, q: int, a: int, k: int)
    requires
        0 <= k <= s.len(),
        q <= a,
    ensures
        sink_pairs(s, q, a, k).len() == (if s.len() - k <= a - q { s.len() - k } else { a - q }),
    decreases s.len() - k,
{
    reveal(sink_pairs);
    if k == s.len() {
        assert(sink_pairs(s, q, a, k) == Seq::< (u32,u8) >::empty());
        assert(s.len() - k == 0);
    } else {
        let j = s.len() - 1 - k;
        let rest = sink_pairs(s, q, a, k + 1);
        lemma_sink_pairs_len(s, q, a, k + 1);
        assert(s.len() - (k + 1) == s.len() - k - 1);
        if q + j < a {
            assert(sink_pairs(s, q, a, k) == seq![((q + j) as u32, s[j])] + rest);
            assert(sink_pairs(s, q, a, k).len() == 1 + rest.len());
        } else {
            assert(sink_pairs(s, q, a, k) == rest);
        }
    }
}

// ---------------------------------------------------------------------------
// The ghost call log H (design memo step 4): `log_complete` says the log names
// every position below `q` exactly once with `O`'s value at it, which is what
// makes the report's `written` and `colour` clauses meaningful. `log_block`
// describes the pairs one code appends, in call order, so `decode` can step the
// invariant once per completed code with `lemma_log_complete_append`.
//
// Every position comparison is on the pair's `.0 as int` against an `int`, so no
// `u32` cast of a quantified variable appears (extends/truncation noise would
// otherwise block the `choose` calls).
// ---------------------------------------------------------------------------

/// Some pair of `log` has logical position `p`.
pub open spec fn log_has_at(log: Seq<(u32, u8)>, p: int) -> bool {
    exists|i: int| 0 <= i < log.len() && #[trigger] (log[i].0 as int) == p
}

/// Some pair of `block` has logical position `p`.
pub open spec fn log_block_at(block: Seq<(u32, u8)>, p: int) -> bool {
    exists|i: int| 0 <= i < block.len() && #[trigger] (block[i].0 as int) == p
}

/// Every logged pair's position is below `n`.
pub open spec fn log_pos_below(log: Seq<(u32, u8)>, n: int) -> bool {
    forall|i: int| 0 <= i < log.len() ==> #[trigger] (log[i].0 as int) < n
}

/// Every logged pair's value is `o`'s at that pair's position.
pub open spec fn log_vals(log: Seq<(u32, u8)>, o: Seq<u8>) -> bool {
    forall|i: int| 0 <= i < log.len() ==> #[trigger] log[i].1 == o[log[i].0 as int]
}

/// The two value clauses together: every logged pair's position is below `n` and
/// its value is `o`'s at that position.
pub open spec fn log_below_ok(log: Seq<(u32, u8)>, o: Seq<u8>, n: int) -> bool {
    log_pos_below(log, n) && log_vals(log, o)
}

/// Every position below `n` occurs at least once in the log (with `log.len() ==
/// n` this is exactly once per position and no other).
pub open spec fn log_below_covers(log: Seq<(u32, u8)>, n: int) -> bool {
    forall|p: int| 0 <= p < n ==> #[trigger] log_has_at(log, p)
}

/// The finished log's four clauses (design memo §3): `|H| == q == |O|`, every
/// pair's value is `O[p]` at its position, and every position below `q` occurs.
pub open spec fn log_complete(log: Seq<(u32, u8)>, o: Seq<u8>, q: int) -> bool {
    &&& log.len() == q
    &&& o.len() == q
    &&& log_pos_below(log, q)
    &&& log_vals(log, o)
    &&& log_below_covers(log, q)
}

/// The pairs one code appends: `|block|` positions from `q` upward, each offered
/// once, each carrying `o`'s value at its position (G10.2 step 8 offers
/// `q .. q+|block|-1`, in the walk's order, so the block is a permutation of
/// those pairs; the order does not matter to `log_complete`).
pub open spec fn log_block(block: Seq<(u32, u8)>, o: Seq<u8>, q: int) -> bool {
    &&& q + block.len() <= o.len()
    &&& (forall|i: int| 0 <= i < block.len() ==> #[trigger] (block[i].0 as int) < q + block.len())
    &&& (forall|i: int| 0 <= i < block.len() ==> #[trigger] block[i].1 == o[block[i].0 as int])
    &&& (forall|p: int| q <= p < q + block.len() ==> #[trigger] log_block_at(block, p))
}

/// Position bounds (`log_pos_below`, `log_vals`) are preserved by appending.
pub proof fn lemma_log_below_ok_append(
    pre: Seq<(u32, u8)>,
    block: Seq<(u32, u8)>,
    o: Seq<u8>,
    q: int,
)
    requires
        pre.len() == q,
        q >= 0,
        o.len() == q + block.len(),
        log_below_ok(pre, o, q),
        log_block(block, o, q),
    ensures
        log_below_ok(pre + block, o, q + block.len()),
{
    reveal(log_below_ok);
    reveal(log_pos_below);
    reveal(log_vals);
    reveal(log_block);
    let n = block.len();
    vstd::seq::lemma_seq_add_len(pre, block);
    assert((pre + block).len() == q + n);
    assert forall|i: int| 0 <= i < (pre + block).len() implies #[trigger] (
        (pre + block)[i].0 as int) < q + n by {
        if i < pre.len() {
            assert((pre[i].0 as int) < q);
            vstd::seq::lemma_seq_add_index1_alt(pre, block, i);
            assert((pre + block)[i] == pre[i]);
        } else {
            let i2 = i - pre.len();
            assert(0 <= i2 < n);
            assert((block[i2].0 as int) < q + n);
            vstd::seq::lemma_seq_add_index2_alt(pre, block, i2);
            assert((pre + block)[i2 + pre.len()] == block[i2]);
            assert(i2 + pre.len() == i);
        }
    }
    assert forall|i: int| 0 <= i < (pre + block).len() implies #[trigger] (pre + block)[i].1
        == o[(pre + block)[i].0 as int] by {
        if i < pre.len() {
            assert(pre[i].1 == o[pre[i].0 as int]);
            vstd::seq::lemma_seq_add_index1_alt(pre, block, i);
            assert((pre + block)[i] == pre[i]);
        } else {
            let i2 = i - pre.len();
            assert(0 <= i2 < n);
            assert(block[i2].1 == o[block[i2].0 as int]);
            vstd::seq::lemma_seq_add_index2_alt(pre, block, i2);
            assert((pre + block)[i2 + pre.len()] == block[i2]);
            assert(i2 + pre.len() == i);
        }
    }
    assert(log_pos_below(pre + block, q + n));
    assert(log_vals(pre + block, o));
}

/// Coverage is preserved by appending.
pub proof fn lemma_log_covers_append(
    pre: Seq<(u32, u8)>,
    block: Seq<(u32, u8)>,
    q: int,
)
    requires
        pre.len() == q,
        q >= 0,
        log_below_covers(pre, q),
        forall|p: int| q <= p < q + block.len() ==> #[trigger] log_block_at(block, p),
    ensures
        log_below_covers(pre + block, q + block.len()),
{
    reveal(log_below_covers);
    reveal(log_has_at);
    reveal(log_block_at);
    let n = block.len();
    vstd::seq::lemma_seq_add_len(pre, block);
    assert forall|p: int| 0 <= p < q + n implies #[trigger] log_has_at(pre + block, p) by {
        if p < q {
            assert(log_has_at(pre, p));
            let i = choose|i: int| 0 <= i < pre.len() && #[trigger] (pre[i].0 as int) == p;
            vstd::seq::lemma_seq_add_index1_alt(pre, block, i);
            assert((pre + block)[i] == pre[i]);
            assert(log_has_at(pre + block, p));
        } else {
            assert(log_block_at(block, p));
            let i2 = choose|j: int| 0 <= j < block.len() && #[trigger] (block[j].0 as int) == p;
            vstd::seq::lemma_seq_add_index2_alt(pre, block, i2);
            assert((pre + block)[i2 + pre.len()] == block[i2]);
            assert(log_has_at(pre + block, p));
        }
    }
}

/// The log invariant steps once per completed code: appending a code's block of
/// pairs (`log_block`) to a complete log keeps it complete for the grown `O`.
pub proof fn lemma_log_complete_append(
    pre: Seq<(u32, u8)>,
    block: Seq<(u32, u8)>,
    o: Seq<u8>,
    q: int,
)
    requires
        pre.len() == q,
        q >= 0,
        o.len() == q + block.len(),
        log_below_ok(pre, o, q),
        log_below_covers(pre, q),
        log_block(block, o, q),
    ensures
        log_complete(pre + block, o, q + block.len()),
{
    reveal(log_complete);
    reveal(log_block);
    vstd::seq::lemma_seq_add_len(pre, block);
    lemma_log_below_ok_append(pre, block, o, q);
    lemma_log_covers_append(pre, block, q);
    assert((pre + block).len() == q + block.len());
}

/// A `u32` round trip for a value in range: `(x as u32) as int == x`. Every
/// `sink_pairs` position is below `MAX_TOTAL_PIXELS` and `decode`'s `q + j` is at
/// most `A + 4096`, so both callers are inside the bound.
pub proof fn lemma_u32_roundtrip(x: int)
    requires
        0 <= x <= 0xffff_ffff,
    ensures
        (x as u32) as int == x,
{
    assert(x as u32 as int == x) by (nonlinear_arith)
        requires
            0 <= x <= 0xffff_ffff,
    ;
}

/// Every offered pair's position is below the raster (`sink_pairs` appends a pair
/// only when its position is below `a`).
pub proof fn lemma_sink_pairs_pos_below(s: Seq<u8>, q: int, a: int, k: int)
    requires
        0 <= k,
        0 <= q,
        q + s.len() <= 0xffff_ffff,
    ensures
        forall|i: int|
            0 <= i < sink_pairs(s, q, a, k).len() ==> #[trigger] (
            sink_pairs(s, q, a, k)[i].0 as int) < a,
    decreases s.len() - k,
{
    reveal(sink_pairs);
    if k >= s.len() {
    } else {
        lemma_sink_pairs_pos_below(s, q, a, k + 1);
        let j = s.len() - 1 - k;
        let rest = sink_pairs(s, q, a, k + 1);
        if q + j < a {
            assert(sink_pairs(s, q, a, k) == seq![((q + j) as u32, s[j])] + rest);
            assert(0 <= q + j) by (nonlinear_arith)
                requires
                    0 <= q,
                    0 <= j,
            ;
            assert(q + j <= 0xffff_ffff) by (nonlinear_arith)
                requires
                    j < s.len(),
                    q + s.len() <= 0xffff_ffff,
            ;
            assert((q + j) as u32 as int == q + j) by {
                lemma_u32_roundtrip(q + j);
            }
            assert forall|i: int| 0 <= i < sink_pairs(s, q, a, k).len() implies #[trigger] (
                sink_pairs(s, q, a, k)[i].0 as int) < a by {
                if i == 0 {
                    assert(sink_pairs(s, q, a, k)[0] == ((q + j) as u32, s[j]));
                    assert(sink_pairs(s, q, a, k)[0].0 == (q + j) as u32);
                } else {
                    let head = seq![((q + j) as u32, s[j])];
                    assert(0 <= i - 1 < rest.len());
                    vstd::seq::lemma_seq_add_index2_alt(head, rest, i - 1);
                    assert((head + rest)[(i - 1) + head.len()] == rest[i - 1]);
                    assert((i - 1) + head.len() == i);
                    assert(sink_pairs(s, q, a, k)[i] == rest[i - 1]);
                }
            }
        } else {
            assert(sink_pairs(s, q, a, k) == rest);
        }
    }
}

/// The block a completed code appends is `log_block`: `sink_pairs(s, q, a, 0)`
/// offers `s[j]` at position `q + j` for every `j` below `min(|s|, a - q)`, and
/// nothing else. `o` is the emitted sequence the code's string is being written
/// into, so `o[q + j] == s[j]` on the string's range is the caller's fact. With
/// `lemma_log_complete_append` this is the loop's per-code log step.
pub proof fn lemma_log_block_sink_pairs(
    s: Seq<u8>,
    o: Seq<u8>,
    q: int,
    a: int,
)
    requires
        0 <= q,
        q <= a,
        q + s.len() <= 0xffff_ffff,
        q + s.len() <= o.len(),
        forall|j: int| 0 <= j < s.len() ==> #[trigger] o[q + j] == s[j],
    ensures
        log_block(sink_pairs(s, q, a, 0), o, q),
{
    reveal(log_block);
    reveal(log_block_at);
    let sp = sink_pairs(s, q, a, 0);
    let ghost sl: int = s.len() as int;
    let ghost aq: int = a - q;
    lemma_sink_pairs_len(s, q, a, 0);
    lemma_sink_pairs_pos_below(s, q, a, 0);
    assert(sl == s.len() as int);
    assert(aq == a - q);
    assert(sp.len() == (if sl <= aq { sl } else { aq }) as nat);
    assert(sp.len() <= s.len()) by {
        assert(sp.len() == (if sl <= aq { sl } else { aq }) as nat);
    }
    assert(q + sp.len() <= o.len()) by (nonlinear_arith)
        requires
            sp.len() <= s.len(),
            q + s.len() <= o.len(),
    ;
    assert forall|i: int| 0 <= i < sp.len() implies #[trigger] (sp[i].0 as int) < q + sp.len() by {
        lemma_sink_pairs_ok(s, q, a, 0);
        let j = choose|j: int| 0 <= j < sl && #[trigger] sp[i] == ((q + j) as u32, s[j]);
        assert(sp[i] == ((q + j) as u32, s[j]));
        assert(0 <= q + j <= 0xffff_ffff) by (nonlinear_arith)
            requires
                0 <= q,
                0 <= j,
                j < sl,
                q + sl <= 0xffff_ffff,
        ;
        assert((sp[i].0 as int) == q + j) by {
            lemma_u32_roundtrip(q + j);
            assert(sp[i].0 == (q + j) as u32);
        }
        assert((sp[i].0 as int) < a);
        assert(j < aq) by (nonlinear_arith)
            requires
                (sp[i].0 as int) == q + j,
                (sp[i].0 as int) < a,
                aq == a - q,
        ;
        assert(j < sp.len()) by (nonlinear_arith)
            requires
                sp.len() == (if sl <= aq { sl } else { aq }) as nat,
                j < sl,
                j < aq,
        ;
    }
    assert forall|i: int| 0 <= i < sp.len() implies #[trigger] sp[i].1 == o[sp[i].0 as int] by {
        lemma_sink_pairs_ok(s, q, a, 0);
        let j = choose|j: int| 0 <= j < sl && #[trigger] sp[i] == ((q + j) as u32, s[j]);
        assert(sp[i] == ((q + j) as u32, s[j]));
        assert(0 <= q + j <= 0xffff_ffff) by (nonlinear_arith)
            requires
                0 <= q,
                0 <= j,
                j < sl,
                q + sl <= 0xffff_ffff,
        ;
        assert((sp[i].0 as int) == q + j) by {
            lemma_u32_roundtrip(q + j);
            assert(sp[i].0 == (q + j) as u32);
        }
        assert(o[q + j] == s[j]);
    }
    assert forall|p: int| q <= p < q + sp.len() implies #[trigger] log_block_at(sp, p) by {
        let j = p - q;
        assert(0 <= j < sl);
        let ghost spn: int = sp.len() as int;
        assert(spn == sp.len() as int);
        assert(spn == (if sl <= aq { sl } else { aq }));
        assert(spn <= aq) by (nonlinear_arith)
            requires
                spn == (if sl <= aq { sl } else { aq }),
        ;
        assert(j < aq) by (nonlinear_arith)
            requires
                p < q + sp.len(),
                sp.len() <= aq,
                j == p - q,
        ;
        assert(q + j < a) by (nonlinear_arith)
            requires
                j < aq,
                aq == a - q,
        ;
        lemma_sink_pairs_complete(s, q, a, 0, j);
        let i = choose|i: int| 0 <= i < sp.len() && #[trigger] sp[i].0 == (q + j) as u32;
        assert(sp[i].0 == (q + j) as u32);
        assert(0 <= q + j <= 0xffff_ffff) by (nonlinear_arith)
            requires
                0 <= q,
                0 <= j,
                j < sl,
                q + sl <= 0xffff_ffff,
        ;
        assert((sp[i].0 as int) == p) by {
            lemma_u32_roundtrip(q + j);
            assert((sp[i].0 as int) == q + j);
        }
    }
}

// ---------------------------------------------------------------------------
// The decode report and its model-parameter bridge (design memo steps 3/4).
// The integrated decoder and scanned-data entry below consume this report.
// ---------------------------------------------------------------------------

/// `LzwReport`: the pure core observation the memo's `DecodeReport` carries —
/// the emitted index count, the pure G10.2 outcome, the consumed-code-bit
/// position and E4 — plus the ghost call log `H` the frame adapter's colour
/// clause reads. `colour` is the memo's `Option<ColourFault>`.
pub struct LzwReport {
    pub written: u32,
    pub error: Option<Error>,
    pub pos: u32,
    pub e4: bool,
    /// Ghost: the sink's call log for this decode (`|H| == written`).
    pub h: Ghost<Seq<(u32, u8)>>,
    pub times: Ghost<Seq<u32>>,
    pub colour: Option<ColourFault>,
}

#[derive(Clone, Copy)]
pub struct ColourFault {
    pub q: u32,
    pub pos: u32,
}

impl LzwReport {
    pub fn public_pos(&self) -> (pos: u32)
        ensures pos == projected_pos(*self),
    {
        match self.colour { Some(c) => c.pos, None => self.pos }
    }

    /// G12.2 rejects any invalid emitted index before exposing the pure outcome.
    pub fn public_result(self) -> (r: Result<bool, Error>)
        ensures match r {
            Ok(e4) => projected_error(self) is None && e4 == self.e4,
            Err(e) => projected_error(self) == Some(e),
        },
    {
        match self.colour {
            Some(_) => Err(Error::ColorIndexOutOfRange),
            None => match self.error { Some(e) => Err(e), None => Ok(self.e4) },
        }
    }
}

/// `D`, the concatenated payload of the sub-block sequence whose first size byte
/// is at `q`: `sub_scan`'s `ps` concatenated, or empty when the scan fails. This
/// is the `d` `ML` reads (`THEOREMS.md` G9: `D` is the concatenation).
pub open spec fn read_d(b: Seq<u8>, q: int) -> Seq<u8> {
    match crate::model::sub_scan(b, q) {
        Some((ps, e)) => crate::model::concat(ps),
        None => Seq::empty(),
    }
}

/// The model parameters selected by the parser's literal-check mode. `m` is
/// retained in this interface; `m_ok` checks its range and the size-1 area rule.
/// Both tail modes are supported for each minimum-code-size policy.
pub open spec fn lzw_ps(m: u32, opts: LzwOptions) -> LzwParams {
    LzwParams {
        m_lo: if opts.checked_literals { 1 } else { 2 },
        m_hi: if opts.checked_literals { 11 } else { 8 },
        size1_only_one_pixel: opts.checked_literals,
        checked_literals: opts.checked_literals,
        deferred_clear: true,
        stop_at_raster: opts.tail == Tail::StopAtRaster,
    }
}

/// The derived minimum-size policy's range; `m_ok` also checks the frame area.
pub open spec fn lzw_ps_known(m: u32, opts: LzwOptions) -> bool {
    2 <= m <= 8 && !opts.checked_literals
        || 1 <= m <= 11 && opts.checked_literals
}

/// The error a pure model outcome projects to.
pub open spec fn errk_to_error(e: ErrK) -> Error {
    match e {
        ErrK::InvalidLzwMinCodeSize => Error::InvalidLzwMinCodeSize,
        ErrK::InvalidLzwCode => Error::InvalidLzwCode,
        ErrK::TooManyPixels => Error::TooManyPixels,
        ErrK::TooFewPixels => Error::TooFewPixels,
        ErrK::MissingEndOfInformation => Error::MissingEndOfInformation,
        ErrK::DataAfterEndOfInformation => Error::DataAfterEndOfInformation,
        ErrK::ColorIndexOutOfRange => Error::ColorIndexOutOfRange,
    }
}

/// `lzw_proof`'s `err_of`, restated for the report.
pub open spec fn err_of(e: ErrK) -> Error {
    errk_to_error(e)
}

/// The report's pure observation equals a model observation: `written` is its
/// index count, `error` its outcome projected to a G16 name, `pos` its position
/// and `e4` its flag. The log half is `log_complete` separately.
pub open spec fn report_matches(r: LzwReport, mr: MRes) -> bool {
    &&& r.written as int == mr.idx.len()
    &&& r.pos as int == mr.pos
    &&& r.e4 == mr.e4
    &&& (match mr.error {
        None => r.error.is_none(),
        Some(e) => r.error == Some(err_of(e)),
    })
}

} // verus!


verus! {

/// Typed length view of the raw reservation; literal bytes remain arbitrary.
pub open spec fn raw_ls_seq(t: Seq<u8>) -> Seq<u16> {
    Seq::new(TABLE_SIZE as nat, |i: int| raw_length(t, i) as u16)
}

pub proof fn lemma_raw_views(t: Seq<u8>)
    requires table_len_ok(t),
    ensures
        forall|c: int| 0 <= c < TABLE_SIZE ==> #[trigger] raw_pf_seq(t)[c] as int == raw_prefix(t, c),
        forall|c: int| 0 <= c < TABLE_SIZE ==> #[trigger] raw_ls_seq(t)[c] as int == raw_length(t, c),
        forall|c: int| 0 <= c < TABLE_SIZE ==> #[trigger] raw_sf_seq(t)[c] as int == raw_suffix(t, c),
{
    assert forall|c: int| 0 <= c < TABLE_SIZE implies #[trigger] raw_pf_seq(t)[c] as int == raw_prefix(t, c) by {
        assert(0 <= raw_prefix(t, c) <= 65535);
    }
    assert forall|c: int| 0 <= c < TABLE_SIZE implies #[trigger] raw_ls_seq(t)[c] as int == raw_length(t, c) by {
        assert(0 <= raw_length(t, c) <= 65535);
    }
}

/// G10.2 step 7 changes only the newly readable entry, including its length.
fn add_entry(t: &mut [u8], next: &mut u32, width: &mut u32, prev: u32, s0: u8, clear: u32)
    requires
        table_len_ok(old(t)@),
        *old(next) < TABLE_SIZE,
        1 <= *old(width) <= 12,
        prev < *old(next),
        crate::dictionary::valid(clear as int, *old(next) as int, prev as int),
        crate::dictionary::table_ok(raw_pf_seq(old(t)@), raw_ls_seq(old(t)@), clear as int, *old(next) as int),
    ensures
        table_len_ok(final(t)@),
        *final(next) == *old(next) + 1,
        *final(width) == if *old(width) < 12 && *final(next) == (1u32 << *old(width)) { *old(width) as int + 1 } else { *old(width) as int },
        1 <= *final(width) <= 12,
        raw_pf_seq(final(t)@) == raw_pf_seq(old(t)@).update(*old(next) as int, prev as u16),
        raw_sf_seq(final(t)@) == raw_sf_seq(old(t)@).update(*old(next) as int, s0),
        raw_ls_seq(final(t)@) == raw_ls_seq(old(t)@).update(*old(next) as int,
            (crate::refinement::clen(raw_ls_seq(old(t)@), clear as int, prev as int) + 1) as u16),
        crate::refinement::clen(raw_ls_seq(old(t)@), clear as int, prev as int) + 1 <= TABLE_SIZE,
        crate::dictionary::table_ok(raw_pf_seq(final(t)@), raw_ls_seq(final(t)@), clear as int, *final(next) as int),
        raw_link_ok(final(t)@,clear as int,*final(next) as int-1),
        crate::chain::str_of(raw_sf_seq(final(t)@), raw_pf_seq(final(t)@), clear as int, *old(next) as int, 4096)
            == crate::chain::str_of(raw_sf_seq(old(t)@), raw_pf_seq(old(t)@), clear as int, prev as int, 4096).push(s0),
{
    let e = *next;
    let ghost ot = t@;
    let ghost op = raw_pf_seq(ot);
    let ghost os = raw_sf_seq(ot);
    let ghost ol = raw_ls_seq(ot);
    proof {
        lemma_raw_views(ot);
        crate::dictionary::chain_valid(op, ol, clear as int, e as int, prev as int);
        crate::refinement::cache_shape(os, op, ol, clear as int, prev as int, 4096);
        crate::chain::lemma_str_len_bound(os, op, clear as int, prev as int, 4096);
    }
    let plen = code_len(t, clear, prev);
    proof { assert(plen as int + 1 <= 4096); }
    prefix_set(t, e, prev as u16);
    let ghost t1 = t@;
    proof { lemma_prefix_set_region(ot, t1, e as int, prev as u16); }
    suffix_set(t, e, s0);
    let ghost t2 = t@;
    proof { lemma_suffix_set_region(t1, t2, e as int, s0); }
    stored_len_set(t, e, plen + 1);
    proof {
        lemma_length_set_region(t2, t@, e as int, (plen + 1) as u16);
        lemma_raw_views(t@);
        assert(raw_pf_seq(t@) =~= op.update(e as int, prev as u16));
        assert(raw_sf_seq(t@) =~= os.update(e as int, s0));
        assert(raw_ls_seq(t@) =~= ol.update(e as int, (plen + 1) as u16));
        assert(crate::dictionary::table_ok(raw_pf_seq(t@), raw_ls_seq(t@), clear as int, e as int + 1)) by {
            assert forall|x: int| clear + 2 <= x < e + 1 implies #[trigger] crate::chain::link_ok(raw_pf_seq(t@), clear as int, x)
                && crate::refinement::clen(raw_ls_seq(t@), clear as int, x) == crate::refinement::clen(raw_ls_seq(t@), clear as int, raw_pf_seq(t@)[x] as int) + 1
                && 1 <= crate::refinement::clen(raw_ls_seq(t@), clear as int, x) <= 4096 by {
                if x == e {} else { assert(crate::chain::link_ok(op, clear as int, x)); }
            }
        }
        lemma_raw_table_links(t@,clear as int,e as int+1);
        crate::dictionary::chain_valid(raw_pf_seq(t@), raw_ls_seq(t@), clear as int, e as int + 1, e as int);
        crate::chain::lemma_chain_step(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, e as int, 4096);
        crate::refinement::fresh_frame(os, op, clear as int, e as int, prev as int, s0, prev as int);
        crate::refinement::fuel_irrelevant(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, prev as int, 4095, 4096);
    }
    *next += 1;
    if *next == (1u32 << *width) && *width < 12 { *width += 1; }
}

} // verus!

verus! {

/// `fault` is the least rejected raster position in this physical call log.
pub open spec fn min_rejected(h: Seq<(u32, u8)>, fault: Option<u32>, ok: spec_fn(u8) -> bool) -> bool {
    match fault {
        None => forall|i: int| 0 <= i < h.len() ==> ok(#[trigger] h[i].1),
        Some(p) => {
            &&& exists|i: int| 0 <= i < h.len() && h[i].0 == p && !ok(h[i].1)
            &&& forall|i: int| 0 <= i < h.len() && !ok(#[trigger] h[i].1) ==> p <= h[i].0
        },
    }
}

pub proof fn lemma_min_rejected_push(h: Seq<(u32,u8)>, f: Option<u32>, ok: spec_fn(u8)->bool,
    p: u32, v: u8, nf: Option<u32>)
    requires min_rejected(h, f, ok),
        nf == if ok(v) { f } else { match f { Some(x) => Some(if p < x {p} else {x}), None => Some(p) } },
    ensures min_rejected(h.push((p,v)), nf, ok),
{
    match nf {
        None => {},
        Some(x) => {
            if !ok(v) && x == p { assert(h.push((p,v))[h.len() as int] == (p,v)); }
            else {
                let i = choose|i: int| 0 <= i < h.len() && h[i].0 == x && !ok(h[i].1);
                assert(h.push((p,v))[i] == h[i]);
            }
        },
    }
}

/// Walk a verified raw chain last-first, without allocating an index buffer.
/// The log equation records performed calls plus the still-unvisited suffix.
fn emit_raw<S: Sink>(t: &[u8], clear: u32, code: u32, q: u32, area: u32, len: u32,
    sink: &mut S) -> (r: (u8, Option<u32>))
    requires
        table_len_ok(t@), clear < TABLE_SIZE, code < TABLE_SIZE,
        crate::chain::chain_ok(raw_pf_seq(t@), clear as int, code as int),
        q <= area <= crate::limits::MAX_TOTAL_PIXELS,
        len == crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, code as int, 4096).len(),
        old(sink).sink_inv(), old(sink).sink_area() == area as int,
    ensures
        final(sink).sink_future()==old(sink).sink_future(),
        final(sink).sink_effect()==old(sink).sink_effect(),
        final(sink).sink_view().len()==old(sink).sink_view().len(),
        final(sink).sink_view()==effect_fold(old(sink).sink_view(),sink_pairs(crate::chain::str_of(raw_sf_seq(t@),raw_pf_seq(t@),clear as int,code as int,4096),q as int,area as int,0),old(sink).sink_effect()),
        r.0 == crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, code as int, 4096)[0],
        final(sink).sink_inv(), final(sink).sink_area() == old(sink).sink_area(),
        forall|v: u8| final(sink).sink_ok(v) == old(sink).sink_ok(v),
        final(sink).sink_log() == old(sink).sink_log() + sink_pairs(
            crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, code as int, 4096), q as int, area as int, 0),
        min_rejected(sink_pairs(crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, code as int, 4096), q as int, area as int, 0),
            r.1, |v: u8| old(sink).sink_ok(v)),
{
    let ghost s = crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, code as int, 4096);
    let ghost h0 = sink.sink_log();
    let ghost cv0=sink.sink_view();
    let ghost effect=sink.sink_effect();
    let ghost ok = |v: u8| sink.sink_ok(v);
    let ghost mut h = Seq::<(u32,u8)>::empty();
    let mut fault: Option<u32> = None;
    let mut cur = code;
    let mut j = len;
    let mut fuel: u32 = 4096;
    let mut first: u8 = 0;
    proof {
        lemma_raw_views(t@);
        crate::chain::lemma_str_len_bound(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, code as int, 4096);
    }
    while j > 0
        invariant
            table_len_ok(t@), clear < TABLE_SIZE, code < TABLE_SIZE,
            q <= area <= crate::limits::MAX_TOTAL_PIXELS,
            0 <= j <= len <= TABLE_SIZE, s.len() == len, len >= 1,
            cur < TABLE_SIZE,
            crate::chain::chain_ok(raw_pf_seq(t@), clear as int, cur as int),
            crate::chain::fuel_above(cur as int, fuel as nat),
            j > 0 ==> s.take(j as int) == crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, cur as int, fuel as nat),
            j == 0 ==> first == s[0],
            sink.sink_inv(), sink.sink_area() == area as int,
            forall|v: u8| sink.sink_ok(v) == ok(v),
            sink.sink_log() == h0 + h,
            cv0==old(sink).sink_view(),effect==old(sink).sink_effect(),
            sink.sink_future()==old(sink).sink_future(),
            sink.sink_effect()==effect,sink.sink_view()==effect_fold(cv0,h,effect),
            sink.sink_view().len()==cv0.len(),
            h + sink_pairs(s, q as int, area as int, len as int - j as int) == sink_pairs(s, q as int, area as int, 0),
            min_rejected(h, fault, ok),
        decreases j,
    {
        let ghost k = len as int - j as int;
        proof {
            if cur < clear {
                assert(crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, cur as int, fuel as nat) == seq![cur as u8]);
                assert(j == 1);
            } else {
                crate::chain::lemma_chain_step(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, cur as int, fuel as nat);
                crate::chain::lemma_str_len_bound(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, raw_pf_seq(t@)[cur as int] as int, (fuel - 1) as nat);
                assert(j >= 2);
            }
        }
        j -= 1;
        let v = if cur < clear { cur as u8 } else { suffix_get(t, cur) };
        first = v;
        proof {
            lemma_raw_views(t@);
            if cur >= clear {
                let inner = crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, raw_pf_seq(t@)[cur as int] as int, (fuel - 1) as nat);
                assert(s.take(j as int + 1) == inner.push(v));
                assert(inner.len() == j);
                assert(s.take(j as int + 1)[j as int] == inner.push(v)[j as int]);
            } else { assert(s.take(1)[0] == v); }
            assert(s[j as int] == v);
            assert(crate::limits::MAX_TOTAL_PIXELS + TABLE_SIZE <= u32::MAX) by (compute);
            reveal_with_fuel(sink_pairs, 1);
            assert(sink_pairs(s, q as int, area as int, k) == if q + j < area {
                seq![((q + j) as u32, v)] + sink_pairs(s, q as int, area as int, k + 1)
            } else { sink_pairs(s, q as int, area as int, k + 1) });
        }
        let p = q + j;
        if p < area {
            let accepted = sink.put(p, v);
            let ghost oldfault = fault;
            if !accepted {
                fault = match fault { Some(x) => Some(if p < x {p} else {x}), None => Some(p) };
            }
            proof {
                lemma_min_rejected_push(h, oldfault, ok, p, v, fault);
                lemma_effect_push(cv0,h,p,v,effect);
                h = h.push((p,v));
            }
        }
        if j > 0 {
            let parent = prefix_get(t, cur) as u32;
            proof {
                let inner = crate::chain::str_of(raw_sf_seq(t@), raw_pf_seq(t@), clear as int, parent as int, (fuel - 1) as nat);
                assert(s.take(j as int + 1) == inner.push(v));
                assert(inner.len() == j);
                assert(s.take(j as int) =~= inner);
            }
            cur = parent;
            fuel -= 1;
        }
    }
    proof { assert(h == sink_pairs(s, q as int, area as int, 0)); }
    (first, fault)
}

} // verus!

verus! {
#[cfg(verus_only)]
use crate::lzw_model::{St, Vd, ML, L, init_st, clear_st, emit_st, semit, first_of, verdict, nbits, m_ok, lzw_ipow2, lzw_clear_of, lzw_eoi_of, lemma_L_unfold, lemma_L_step_pos};

/// Reader state consumed and preserved by each code read.
pub open spec fn reader_ready(r: &BitReader) -> bool {
    &&& r.b.len() <= crate::limits::MAX_INPUT
    &&& r.data_len <= crate::limits::MAX_INPUT
    &&& r.bits_left <= r.total_bits
    &&& r.total_bits == 8 * (r.data_len as int)
    &&& r.pos as int == crate::model::rd_state(r.b@, r.seq_start as int, r.fetched as int).0
    &&& r.left_in_block as int == crate::model::rd_state(r.b@, r.seq_start as int, r.fetched as int).1
    &&& r.nacc as int == 8 * (r.fetched as int) - ((r.total_bits as int) - (r.bits_left as int))
    &&& r.nacc <= 32
    &&& r.fetched <= r.data_len
    &&& crate::model::acc_view(r.b@, r.seq_start as int, r.fetched as int, r.nacc as int, r.acc)
    &&& forall|j: int| 0 <= j < r.data_len as int ==> #[trigger] crate::model::rd_step_ok(r.b@, r.seq_start as int, j)
}

pub proof fn lemma_shift_power(m: u32)
    requires m <= 12,
    ensures (1u32 << m) as int == lzw_ipow2(m as int),
        1 <= lzw_ipow2(m as int) <= TABLE_SIZE,
        m <= 11 ==> lzw_ipow2(m as int) <= 2048,
        m >= 2 ==> lzw_ipow2(m as int) >= 4,
        m == 1 ==> lzw_ipow2(m as int) == 2,
{
    reveal_with_fuel(lzw_ipow2, 13);
    assert((1u32 << m) == match m {0=>1u32,1=>2,2=>4,3=>8,4=>16,5=>32,6=>64,7=>128,8=>256,9=>512,10=>1024,11=>2048,_=>4096}) by (bit_vector)
        requires m <= 12;
}

pub proof fn lemma_code_models(d: Seq<u8>, p: int, w: int, i: int)
    requires 0 <= i <= w,
    ensures crate::model::concat_code(d,p,w,i) == crate::lzw_model::code_at(d,p,w,i),
    decreases w-i,
{
    if i < w { lemma_code_models(d,p,w,i+1); }
}

pub proof fn lemma_lzw_code_bound(d: Seq<u8>, p: int, w: int, i: int)
    requires 0 <= i <= w <= 12, 0 <= p, p+w <= nbits(d),
    ensures 0 <= crate::lzw_model::code_at(d,p,w,i) < lzw_ipow2(w-i),
    decreases w-i,
{
    if i < w {
        assert(0 <= crate::lzw_model::lzw_bit_at(d,p+i) <= 1) by {
            let b = d[(p+i)/8]; let j = ((p+i)%8) as u8;
            assert(((b >> j) & 1u8) <= 1) by (bit_vector);
        }
        lemma_lzw_code_bound(d,p,w,i+1);
    }
}

pub open spec fn raster(h: Seq<(u32,u8)>) -> Seq<u8> {
    Seq::new(h.len(), |p: int| h[choose|i: int| 0 <= i < h.len() && (h[i].0 as int) == p].1)
}

pub proof fn lemma_raster_complete(h: Seq<(u32,u8)>, o: Seq<u8>, q: int)
    requires log_complete(h,o,q), o.len() == q,
    ensures raster(h) == o,
{
    assert(raster(h) =~= o) by {
        assert forall|p: int| 0 <= p < o.len() implies #[trigger] raster(h)[p] == o[p] by {
            assert(log_has_at(h,p));
            let i = choose|i: int| 0 <= i < h.len() && (h[i].0 as int) == p;
            assert(h[i].1 == o[h[i].0 as int]);
        }
    }
}

/// After-code position for each as-yet un-emitted index of the pure fold.
pub open spec fn code_stamps(m: int, d: Seq<u8>, a: int, ps: LzwParams, st: St) -> Seq<u32>
    decreases nbits(d)-st.pos,
{
    match verdict(m,d,a,ps,st) {
        Vd::Halt(r) => Seq::new((r.idx.len()-st.o.len()) as nat, |_i: int| r.pos as u32),
        Vd::Step(ns) => if ns.pos > st.pos {
            Seq::new((ns.o.len()-st.o.len()) as nat, |_i: int| ns.pos as u32) + code_stamps(m,d,a,ps,ns)
        } else { Seq::empty() },
    }
}

pub open spec fn fault_matches(h: Seq<(u32,u8)>, ts: Seq<u32>, f: Option<ColourFault>, ok: spec_fn(u8)->bool) -> bool {
    &&& h.len() == ts.len()
    &&& min_rejected(h, match f { Some(c) => Some(c.q), None => None }, ok)
    &&& match f {
        None => true,
        Some(c) => exists|i: int| 0 <= i < h.len() && h[i].0 == c.q && !ok(h[i].1) && ts[i] == c.pos,
    }
}

fn merge_colour(f: Option<ColourFault>, block: Option<u32>, after: u32) -> (nf: Option<ColourFault>)
    ensures nf == match block {
        None => f,
        Some(p) => match f { Some(c) => if p < c.q {Some(ColourFault {q:p,pos:after})} else {f}, None => Some(ColourFault {q:p,pos:after}) },
    },
{
    match block {
        None => f,
        Some(p) => match f { Some(c) => if p < c.q {Some(ColourFault {q:p,pos:after})} else {f}, None => Some(ColourFault {q:p,pos:after}) },
    }
}

pub proof fn lemma_fault_append(h: Seq<(u32,u8)>, ts: Seq<u32>, f: Option<ColourFault>,
    b: Seq<(u32,u8)>, bf: Option<u32>, after: u32, nf: Option<ColourFault>, ok: spec_fn(u8)->bool)
    requires fault_matches(h,ts,f,ok), min_rejected(b,bf,ok),
        nf == match bf { None => f, Some(p) => match f { Some(c) => if p<c.q {Some(ColourFault {q:p,pos:after})} else {f}, None => Some(ColourFault {q:p,pos:after}) } },
    ensures fault_matches(h+b, ts+Seq::new(b.len(), |_i:int| after), nf, ok),
{
    let nt = ts+Seq::new(b.len(), |_i:int| after);
    match nf {
        None => {},
        Some(c) => {
            if bf == Some(c.q) && (f is None || c.q < f.unwrap().q) {
                let i = choose|i:int| 0<=i<b.len() && b[i].0==c.q && !ok(b[i].1);
                assert((h+b)[h.len()+i] == b[i]);
                assert(nt[h.len()+i] == after);
            } else {
                let i = choose|i:int| 0<=i<h.len() && h[i].0==c.q && !ok(h[i].1) && ts[i]==c.pos;
                assert((h+b)[i] == h[i]);
                assert(nt[i] == ts[i]);
            }
        },
    }
    assert forall|i:int| 0<=i<(h+b).len() implies #[trigger] (h+b)[i] == if i<h.len() {h[i]} else {b[i-h.len()]} by {};
}

/// Append a completed code's reverse call block to the logical raster prefix.
pub proof fn lemma_emitted_log(h: Seq<(u32,u8)>, o: Seq<u8>, s: Seq<u8>, q: int, a: int)
    requires log_complete(h,o,q), o.len()==q, 0<=q<=a, a<=crate::limits::MAX_TOTAL_PIXELS, s.len() <= TABLE_SIZE,
    ensures log_complete(h+sink_pairs(s,q,a,0), o+s.take(if s.len()<=a-q {s.len() as int} else {a-q}),
        q+if s.len()<=a-q {s.len() as int} else {a-q}),
{
    let n = if s.len() <= a-q {s.len() as int} else {a-q};
    let no = o+s.take(n);
    assert(crate::limits::MAX_TOTAL_PIXELS + TABLE_SIZE <= u32::MAX) by (compute);
    assert forall|j:int| 0<=j<s.len() implies #[trigger] (o+s)[q+j]==s[j] by {};
    lemma_log_block_sink_pairs(s,o+s,q,a);
    let b = sink_pairs(s,q,a,0);
    lemma_sink_pairs_len(s,q,a,0);
    assert forall|i:int| 0<=i<b.len() implies #[trigger] b[i].1 == no[b[i].0 as int] by {
        assert((b[i].0 as int) < q+n);
    }
    assert forall|i:int| 0<=i<h.len() implies #[trigger] h[i].1 == no[h[i].0 as int] by {};
    lemma_log_complete_append(h,b,no,q);
}

} // verus!

verus! {

/// The integrated core completes the pure fold after a sink rejection; the
/// frame adapter projects the saved colour fault over the pure observation.
#[verifier::spinoff_prover]
pub fn decode<S: Sink>(reader: &mut BitReader, data_len: u32, m: u32, area: u32,
    opts: LzwOptions, t: &mut [u8], sink: &mut S) -> (rep: LzwReport)
    requires
        1 <= m <= 11,
        m_ok(lzw_ps(m,opts), m as int, area as int),
        area <= crate::limits::MAX_TOTAL_PIXELS,
        old(sink).sink_inv(), old(sink).sink_area() == area as int,
        table_len_ok(old(t)@),
        reader_ready(old(reader)), old(reader).data_len == data_len,
        old(reader).total_bits == old(reader).bits_left,
        crate::model::sub_scan(old(reader).b@,old(reader).seq_start as int) is Some,
        read_d(old(reader).b@,old(reader).seq_start as int).len() == data_len,
    ensures
        final(sink).sink_future()==old(sink).sink_future(),
        final(sink).sink_effect()==old(sink).sink_effect(),
        final(sink).sink_view().len()==old(sink).sink_view().len(),
        final(sink).sink_view()==effect_fold(old(sink).sink_view(),rep.h@,old(sink).sink_effect()),
        table_len_ok(final(t)@), reader_ready(final(reader)),
        final(reader).b == old(reader).b, final(reader).seq_start == old(reader).seq_start,
        final(reader).data_len == data_len,
        final(sink).sink_inv(), final(sink).sink_area() == area as int,
        forall|v:u8| final(sink).sink_ok(v) == old(sink).sink_ok(v),
        final(sink).sink_log() == old(sink).sink_log() + rep.h@,
        rep.written <= area,
        rep.pos as int <= 8 * (data_len as int),
        report_matches(rep, ML(m as int,read_d(old(reader).b@,old(reader).seq_start as int),area as int,lzw_ps(m,opts))),
        log_complete(rep.h@, ML(m as int,read_d(old(reader).b@,old(reader).seq_start as int),area as int,lzw_ps(m,opts)).idx,rep.written as int),
        raster(rep.h@) == ML(m as int,read_d(old(reader).b@,old(reader).seq_start as int),area as int,lzw_ps(m,opts)).idx,
        rep.times@ == code_stamps(m as int,read_d(old(reader).b@,old(reader).seq_start as int),area as int,lzw_ps(m,opts),init_st(m as int)),
        fault_matches(rep.h@,rep.times@,rep.colour,|v:u8| old(sink).sink_ok(v)),
        log_time_agrees(rep.h@,rep.times@),
        colour_observation(rep, ML(m as int,read_d(old(reader).b@,old(reader).seq_start as int),area as int,lzw_ps(m,opts)).idx,
            |v:u8| old(sink).sink_ok(v)),
{
    hide(L);
    hide(verdict);
    hide(effect_fold);
    hide(code_stamps);
    hide(raw_pf_seq);
    hide(raw_sf_seq);
    hide(raw_ls_seq);
    hide(crate::dictionary::table_ok);
    hide(table_agrees);
    hide(raw_link_ok);
    hide(reader_ready);
    hide(log_complete);
    hide(fault_matches);
    hide(min_rejected);
    hide(raster);
    hide(log_time_agrees);
    hide(crate::chain::str_of);
    hide(crate::chain::root_of);
    let input = reader.b;
    let stop = match opts.tail { Tail::StopAtRaster => true, Tail::Strict => false };
    proof { assert(stop == (opts.tail == Tail::StopAtRaster)); }
    let ghost b0 = reader.b@;
    let ghost start = reader.seq_start;
    let ghost d = read_d(b0,start as int);
    let ghost ps = lzw_ps(m,opts);
    let ghost h0 = sink.sink_log();
    let ghost cv0=sink.sink_view();
    let ghost effect=sink.sink_effect();
    let ghost ok = |v:u8| sink.sink_ok(v);
    let ghost mut h = Seq::<(u32,u8)>::empty();
    let ghost mut times = Seq::<u32>::empty();
    let mut colour: Option<ColourFault> = None;
    proof { lemma_shift_power(m); }
    let clear = 1u32 << m;
    let eoi = clear + 1;
    let mut next = clear + 2;
    let mut width = m + 1;
    let mut have_prev = false;
    let mut prev: u32 = 0;
    let mut prev_first: u8 = 0;
    let mut q: u32 = 0;
    let ghost mut st = init_st(m as int);
    proof {
        lemma_table_reset(t@,clear as int,st.pf,st.sf);
        lemma_empty_observation(ok);
        lemma_effect_empty(cv0,effect);
        crate::dictionary::history_empty(raw_pf_seq(t@),raw_ls_seq(t@),clear as int,0,opts.checked_literals);
        assert(st.pos == (reader.total_bits as int)-(reader.bits_left as int));
        lemma_shift_power(width);
    }
    loop
        invariant
            1 <= m <= 11, m_ok(ps,m as int,area as int), ps == lzw_ps(m,opts),
            stop == (opts.tail == Tail::StopAtRaster),
            clear as int == lzw_clear_of(m as int), eoi as int == lzw_eoi_of(m as int), clear <= 2048,
            lzw_ipow2(m as int+1) == 2*clear,
            m>=2 ==> clear>=4, m==1 ==> clear==2,
            m+1 <= width <= 12,
            m>=2 ==> (next as int)<lzw_ipow2(width as int) || width==12,
            m==1 && area==1 && opts.tail==Tail::StopAtRaster ==> next==4 && width==2,
            table_len_ok(t@), clear+2 <= next <= TABLE_SIZE,
            raw_link_ok(t@,clear as int,next as int-1),
            crate::dictionary::table_ok(raw_pf_seq(t@),raw_ls_seq(t@),clear as int,next as int),
            have_prev ==> q>=1,
            have_prev ==> crate::dictionary::valid(clear as int,next as int,prev as int) && prev<next,
            have_prev ==> prev_first == crate::chain::root_of(raw_pf_seq(t@),clear as int,prev as int,4096) as u8,
            crate::dictionary::history(raw_pf_seq(t@),raw_ls_seq(t@),clear as int,next as int,q as int,have_prev,prev as int,opts.checked_literals),
            !have_prev ==> next==clear+2,
            reader_ready(reader), reader.b==input, reader.b@==b0, reader.seq_start==start, reader.data_len==data_len,
            input==old(reader).b, start==old(reader).seq_start,
            crate::model::sub_scan(b0,start as int) is Some, d==read_d(b0,start as int), d.len()==data_len,
            q<=area<=crate::limits::MAX_TOTAL_PIXELS,
            st.pos == (reader.total_bits as int)-(reader.bits_left as int),
            st.width==width as int, st.next==next as int,
            st.prev==if have_prev {prev as int} else {-1}, st.q==q as int, st.o.len()==q,
            st.pf.len()==4096, st.sf.len()==4096,
            table_agrees(t@,st.pf,st.sf,clear as int,next as int-1),
            ML(m as int,d,area as int,ps)==L(m as int,d,area as int,ps,st),
            log_complete(h,st.o,q as int), sink.sink_log()==h0+h,
            h0==old(sink).sink_log(),
            cv0==old(sink).sink_view(),effect==old(sink).sink_effect(),
            sink.sink_future()==old(sink).sink_future(),
            sink.sink_effect()==effect,sink.sink_view()==effect_fold(cv0,h,effect),
            sink.sink_view().len()==cv0.len(),
            forall|v:u8| ok(v)==old(sink).sink_ok(v),
            sink.sink_inv(), sink.sink_area()==area as int,
            forall|v:u8| sink.sink_ok(v)==ok(v),
            times + code_stamps(m as int,d,area as int,ps,st) == code_stamps(m as int,d,area as int,ps,init_st(m as int)),
            fault_matches(h,times,colour,ok),
            log_time_agrees(h,times),
        decreases reader.bits_left,
    {
        let ghost op = raw_pf_seq(t@);
        let ghost os = raw_sf_seq(t@);
        let ghost ol = raw_ls_seq(t@);
        proof {
            assert(ok =~= |v:u8| old(sink).sink_ok(v));
            assert(ok =~= |v:u8| sink.sink_ok(v));
            lemma_L_unfold(m as int,d,area as int,ps,st);
            lemma_L_step_pos(m as int,d,area as int,ps,st);
            reveal_with_fuel(code_stamps,1);
        }
        let decision = read_step(reader,data_len,m,clear,width,q,area,next,have_prev,opts,Ghost(st));
        let (code,after) = match decision {
            CodeStep::Halt {error,pos} => {
                proof {
                    lemma_raster_complete(h,st.o,q as int);
                    lemma_colour_projection(h,times,st.o,q as int,colour,ok);
                }
                return LzwReport {written:q,error,pos,e4:false,h:Ghost(h),times:Ghost(times),colour};
            },
            CodeStep::Clear {after} => {
                proof {
                    st=clear_st(m as int,st,after as int);
                    lemma_table_reset(t@,clear as int,st.pf,st.sf);
                    crate::dictionary::history_empty(op,ol,clear as int,q as int,opts.checked_literals);
                }
                next=clear+2; width=m+1; have_prev=false; prev=0;
                continue;
            },
            CodeStep::Emit {code,after} => (code,after),
        };
        proof {
            lemma_control_emit(m as int,d,area as int,ps,st,code,after);
            lemma_resolved_agrees(t@,m as int,st,code as int);
            if m==1 && area==1 && stop { assert(!have_prev); assert(code<clear); }
        }
        let ghost s = semit(m as int,st,code as int);
        let ghost grow = have_prev && next<TABLE_SIZE;
        let ghost ot = t@;
        let (len,first,blockfault) = run_code(t,&mut next,&mut width,clear,code,have_prev,prev,prev_first,q,area,opts.checked_literals,sink);
        proof {
            assert(s==resolved_raw(ot,clear as int,st.next,st.prev,code as int));
            assert(s.len()==len);
            lemma_sink_pairs_len(s,q as int,area as int,0);
        }
        let ghost before_times=times;
        let ghost oldcolour = colour;
        colour=merge_colour(colour,blockfault,after);
        proof {
            let block = sink_pairs(s,q as int,area as int,0);
            lemma_fault_append(h,times,oldcolour,block,blockfault,after,colour,ok);
            lemma_effect_append(cv0,h,block,effect);
            lemma_emission_times(h,times,st.o,s,q as int,area as int,after);
            lemma_emitted_log(h,st.o,s,q as int,area as int);
            h=h+block;
            times=times+Seq::new(block.len(),|_i:int| after);
        }
        if len>area-q {
            proof {
                let no = st.o+s.take(area as int-q as int);
                let mr=crate::lzw_model::observation(no,if stop {None} else {Some(ErrK::TooManyPixels)},after as int,stop);
                assert(verdict(m as int,d,area as int,ps,st)==Vd::Halt(mr));
                lemma_stamps_halt(m as int,d,area as int,ps,st,mr);
                assert(times==before_times+code_stamps(m as int,d,area as int,ps,st));
                lemma_raster_complete(h,no,area as int);
                lemma_colour_projection(h,times,no,area as int,colour,ok);
            }
            return LzwReport {written:area,error:if stop {None} else {Some(Error::TooManyPixels)},pos:after,e4:stop,h:Ghost(h),times:Ghost(times),colour};
        }
        let ghost ns = emit_st(m as int,st,after as int,code as int,s,first,grow);
        proof {
            assert(first==first_of(m as int,st,code as int));
            assert(verdict(m as int,d,area as int,ps,st)==Vd::Step(ns));
            assert(s.take(s.len() as int)==s);
            assert(log_complete(h,ns.o,q as int+len as int));
            lemma_stamps_step(m as int,d,area as int,ps,st,ns);
            assert(times+code_stamps(m as int,d,area as int,ps,ns)==before_times+code_stamps(m as int,d,area as int,ps,st));
        }
        proof {
            lemma_agrees_grow(ot,t@,st,ns,clear as int,grow,prev as u16,first);
            lemma_shift_power(width);
            st=ns;
        }
        q+=len; have_prev=true; prev=code; prev_first=first;
    }
}

} // verus!

verus! {

pub open spec fn resolved_raw(t: Seq<u8>, clear: int, next: int, prev: int, code: int) -> Seq<u8> {
    let base = if code==next {prev} else {code};
    let s = crate::chain::str_of(raw_sf_seq(t),raw_pf_seq(t),clear,base,4096);
    if code==next {s.push(s[0])} else {s}
}

/// Resolve, emit and insert one ordinary/KwKwK code. Keeping table evolution
/// outside the fold query prevents its raw-byte quantifiers expanding at exits.
fn run_code<S: Sink>(t: &mut [u8], next: &mut u32, width: &mut u32, clear: u32,
    code: u32, have: bool, prev: u32, prev_first: u8, q: u32, area: u32, checked: bool,
    sink: &mut S) -> (r: (u32,u8,Option<u32>))
    requires
        table_len_ok(old(t)@), clear<TABLE_SIZE, 1<=*old(width)<=12,
        crate::dictionary::table_ok(raw_pf_seq(old(t)@),raw_ls_seq(old(t)@),clear as int,*old(next) as int),
        have ==> crate::dictionary::valid(clear as int,*old(next) as int,prev as int) && prev<*old(next),
        have ==> prev_first==crate::chain::root_of(raw_pf_seq(old(t)@),clear as int,prev as int,4096) as u8,
        crate::dictionary::history(raw_pf_seq(old(t)@),raw_ls_seq(old(t)@),clear as int,*old(next) as int,q as int,have,prev as int,checked),
        code<TABLE_SIZE,
        crate::dictionary::valid(clear as int,*old(next) as int,code as int) || code==*old(next) && have && *old(next)<TABLE_SIZE,
        checked && code<clear ==> code<256,
        q<area<=crate::limits::MAX_TOTAL_PIXELS,
        old(sink).sink_inv(), old(sink).sink_area()==area as int,
    ensures
        final(sink).sink_future()==old(sink).sink_future(),
        final(sink).sink_effect()==old(sink).sink_effect(),
        final(sink).sink_view().len()==old(sink).sink_view().len(),
        final(sink).sink_view()==effect_fold(old(sink).sink_view(),sink_pairs(resolved_raw(old(t)@,clear as int,*old(next) as int,prev as int,code as int),q as int,area as int,0),old(sink).sink_effect()),
        table_len_ok(final(t)@),
        crate::dictionary::table_ok(raw_pf_seq(final(t)@),raw_ls_seq(final(t)@),clear as int,*final(next) as int),
        raw_link_ok(final(t)@,clear as int,*final(next) as int-1),
        *final(next)==*old(next)+if have && *old(next)<TABLE_SIZE {1int} else {0int},
        *final(width) as int==if have && *old(next)<TABLE_SIZE {crate::lzw_model::widen(*old(width) as int,*old(next) as int+1)} else {*old(width) as int},
        1<=*final(width)<=12,
        raw_pf_seq(final(t)@)==if have && *old(next)<TABLE_SIZE {raw_pf_seq(old(t)@).update(*old(next) as int,prev as u16)} else {raw_pf_seq(old(t)@)},
        raw_sf_seq(final(t)@)==if have && *old(next)<TABLE_SIZE {raw_sf_seq(old(t)@).update(*old(next) as int,r.1)} else {raw_sf_seq(old(t)@)},
        r.0 as int==resolved_raw(old(t)@,clear as int,*old(next) as int,prev as int,code as int).len(),
        1<=r.0<=TABLE_SIZE, r.0<=q as int+1,
        r.1==resolved_raw(old(t)@,clear as int,*old(next) as int,prev as int,code as int)[0],
        r.1==crate::chain::root_of(raw_pf_seq(old(t)@),clear as int,if code==*old(next) {prev as int} else {code as int},4096) as u8,
        r.1==crate::chain::root_of(raw_pf_seq(final(t)@),clear as int,code as int,4096) as u8,
        crate::dictionary::valid(clear as int,*final(next) as int,code as int),
        crate::dictionary::history(raw_pf_seq(final(t)@),raw_ls_seq(final(t)@),clear as int,*final(next) as int,q as int+r.0 as int,true,code as int,checked),
        final(sink).sink_inv(), final(sink).sink_area()==area as int,
        forall|v:u8| final(sink).sink_ok(v)==old(sink).sink_ok(v),
        final(sink).sink_log()==old(sink).sink_log()+sink_pairs(resolved_raw(old(t)@,clear as int,*old(next) as int,prev as int,code as int),q as int,area as int,0),
        min_rejected(sink_pairs(resolved_raw(old(t)@,clear as int,*old(next) as int,prev as int,code as int),q as int,area as int,0),r.2,|v:u8| old(sink).sink_ok(v)),
{
    let ghost ot=t@;
    let ghost op=raw_pf_seq(t@);
    let ghost os=raw_sf_seq(t@);
    let ghost ol=raw_ls_seq(t@);
    let ghost e=*next;
    let ghost w=*width;
    let kw=code==*next;
    let base=if kw {prev} else {code};
    proof {
        lemma_raw_views(t@);
        crate::dictionary::chain_valid(op,ol,clear as int,*next as int,base as int);
        crate::refinement::cache_shape(os,op,ol,clear as int,base as int,4096);
        crate::dictionary::history_read(op,ol,clear as int,*next as int,q as int,have,prev as int,checked,base as int);
    }
    let ghost s=resolved_raw(t@,clear as int,*next as int,prev as int,code as int);
    if kw {add_entry(t,next,width,prev,prev_first,clear);}
    proof {
        lemma_raw_views(t@);
        crate::dictionary::chain_valid(raw_pf_seq(t@),raw_ls_seq(t@),clear as int,*next as int,code as int);
        crate::refinement::cache_shape(raw_sf_seq(t@),raw_pf_seq(t@),raw_ls_seq(t@),clear as int,code as int,4096);
        assert(s==crate::chain::str_of(raw_sf_seq(t@),raw_pf_seq(t@),clear as int,code as int,4096));
    }
    let len=code_len(t,clear,code) as u32;
    let (first,fault)=emit_raw(t,clear,code,q,area,len,sink);
    if !kw && have && *next<TABLE_SIZE as u32 {
        proof {crate::refinement::fresh_frame(os,op,clear as int,*next as int,prev as int,first,code as int);}
        add_entry(t,next,width,prev,first,clear);
    }
    proof {
        let grow=have && e<TABLE_SIZE;
        lemma_raw_views(t@);
        lemma_raw_table_links(t@,clear as int,*next as int);
        assert(raw_pf_seq(t@)==if grow {op.update(e as int,prev as u16)} else {op});
        assert(raw_sf_seq(t@)==if grow {os.update(e as int,first)} else {os});
        assert(raw_ls_seq(t@)==if grow {ol.update(e as int,(crate::refinement::clen(ol,clear as int,prev as int)+1) as u16)} else {ol});
        crate::dictionary::history_advance(op,ol,raw_pf_seq(t@),raw_ls_seq(t@),clear as int,e as int,q as int,have,prev as int,checked,code as int,len as int);
        lemma_shift_power(w);
    }
    (len,first,fault)
}

} // verus!

verus! {

pub proof fn lemma_resolved_agrees(t: Seq<u8>, m: int, st: St, code: int)
    requires table_len_ok(t),
        crate::dictionary::table_ok(raw_pf_seq(t),raw_ls_seq(t),lzw_clear_of(m),st.next),
        st.pf.len()==4096, st.sf.len()==4096,
        table_agrees(t,st.pf,st.sf,lzw_clear_of(m),st.next-1),
        crate::dictionary::valid(lzw_clear_of(m),st.next,code) || code==st.next && st.next<4096 && crate::dictionary::valid(lzw_clear_of(m),st.next,st.prev),
    ensures semit(m,st,code)==resolved_raw(t,lzw_clear_of(m),st.next,st.prev,code),
        first_of(m,st,code)==crate::chain::root_of(raw_pf_seq(t),lzw_clear_of(m),if code==st.next {st.prev} else {code},4096) as u8,
{
    let c=if code==st.next {st.prev} else {code};
    let clear=lzw_clear_of(m);
    lemma_raw_views(t);
    crate::dictionary::chain_valid(raw_pf_seq(t),raw_ls_seq(t),clear,st.next,c);
    assert forall|x:int| clear+2<=x<=c implies #[trigger] st.pf[x]==raw_pf_seq(t)[x] && st.sf[x]==raw_sf_seq(t)[x] by {
        assert(raw_prefix(t,x)==st.pf[x] as int);
        assert(raw_suffix(t,x)==st.sf[x] as int);
    }
    assert forall|x:int| clear+2<=x<=c implies #[trigger] raw_pf_seq(t)[x]==st.pf[x] && raw_sf_seq(t)[x]==st.sf[x] by {
        assert(st.pf[x]==raw_pf_seq(t)[x] && st.sf[x]==raw_sf_seq(t)[x]);
    }
    crate::refinement::table_agreement(raw_sf_seq(t),raw_pf_seq(t),st.sf,st.pf,clear,c,4096);
    lemma_walk_agrees(t,st.pf,st.sf,clear,c,4096);
}

} // verus!

verus! {
pub enum CodeStep {
    Halt { error: Option<Error>, pos: u32 },
    Clear { after: u32 },
    Emit { code: u32, after: u32 },
}

pub open spec fn control_matches(c: CodeStep, m: int, d: Seq<u8>, a: int, ps: LzwParams, st: St) -> bool {
    match c {
        CodeStep::Halt {error,pos} => match verdict(m,d,a,ps,st) {
            Vd::Halt(r) => r.idx==st.o && r.pos==pos as int && !r.e4 && match r.error {
                None => error is None, Some(e) => error==Some(err_of(e)),
            },
            _ => false,
        },
        CodeStep::Clear {after} => after as int==st.pos+st.width && verdict(m,d,a,ps,st)==Vd::Step(clear_st(m,st,after as int)),
        CodeStep::Emit {code,after} => {
            &&& after as int==st.pos+st.width
            &&& st.pos+st.width<=nbits(d)
            &&& code as int==crate::lzw_model::code_at(d,st.pos,st.width,0)
            &&& code<TABLE_SIZE
            &&& code as int!=lzw_clear_of(m) && code as int!=lzw_eoi_of(m)
            &&& st.q<a
            &&& crate::lzw_model::str_ok(m,st.next,st.prev,code as int)
            &&& ps.checked_literals && (code as int)<lzw_clear_of(m) ==> code<256
        },
    }
}

/// G10.2's control checks, with no table or sink quantifiers in this query.
fn read_step(reader: &mut BitReader, data_len: u32, m: u32, clear: u32, width: u32,
    q: u32, area: u32, next: u32, have: bool, opts: LzwOptions, state: Ghost<crate::lzw_model::St>) -> (r: CodeStep)
    requires reader_ready(old(reader)), old(reader).data_len==data_len,
        crate::model::sub_scan(old(reader).b@,old(reader).seq_start as int) is Some,
        read_d(old(reader).b@,old(reader).seq_start as int).len()==data_len,
        1<=m<=11, clear as int==lzw_clear_of(m as int), clear<=2048, 2<=width<=12,
        q<=area, state@.q==q as int, state@.width==width as int,
        state@.pos==(old(reader).total_bits as int)-(old(reader).bits_left as int),
        state@.next==next as int, clear+2<=next<=TABLE_SIZE, have==(state@.prev>=0), state@.prev>=-1,
    ensures reader_ready(final(reader)), final(reader).b==old(reader).b,
        final(reader).seq_start==old(reader).seq_start, final(reader).data_len==data_len,
        control_matches(r,m as int,read_d(old(reader).b@,old(reader).seq_start as int),area as int,lzw_ps(m,opts),state@),
        match r {
            CodeStep::Halt {pos,..} => pos as int <= 8*(data_len as int),
            CodeStep::Clear {after} => final(reader).bits_left<old(reader).bits_left && after as int==(final(reader).total_bits as int)-(final(reader).bits_left as int),
            CodeStep::Emit {after,..} => final(reader).bits_left<old(reader).bits_left && after as int==(final(reader).total_bits as int)-(final(reader).bits_left as int),
        },
{
    let stop=match opts.tail {Tail::StopAtRaster=>true,Tail::Strict=>false};
    let ghost st=state@;
    let ghost d=read_d(reader.b@,reader.seq_start as int);
    proof { reveal(BitReader::bit_pos_spec); }
    if stop && q==area {return CodeStep::Halt {error:None,pos:reader.bit_pos()};}
    if reader.bits_left<width {
        return CodeStep::Halt {error:Some(if q<area {Error::TooFewPixels} else {Error::MissingEndOfInformation}),pos:reader.bit_pos()};
    }
    let code=reader.read(width);
    let after=reader.bit_pos();
    proof {
        let scanned=crate::model::sub_scan(reader.b@,reader.seq_start as int).unwrap();
        crate::model::lemma_concat_len(scanned.0,Seq::empty());
        crate::model::lemma_lbits_read(reader.b@,reader.seq_start as int,scanned.0,scanned.1,st.pos,width as int);
        lemma_code_models(d,st.pos,width as int,0);
        lemma_lzw_code_bound(d,st.pos,width as int,0);
        lemma_shift_power(width);
    }
    let eoi=clear+1;
    if code==clear {return CodeStep::Clear {after};}
    if q==area && code!=eoi {return CodeStep::Halt {error:Some(Error::MissingEndOfInformation),pos:after};}
    if code==eoi {
        if q<area {return CodeStep::Halt {error:Some(Error::TooFewPixels),pos:after};}
        if !stop && (after+7)/8<data_len {return CodeStep::Halt {error:Some(Error::DataAfterEndOfInformation),pos:after};}
        return CodeStep::Halt {error:None,pos:after};
    }
    if opts.checked_literals && code<clear && code>=256 {return CodeStep::Halt {error:Some(Error::ColorIndexOutOfRange),pos:after};}
    if !(code<next || code==next && have && next<TABLE_SIZE as u32) {
        return CodeStep::Halt {error:Some(Error::InvalidLzwCode),pos:after};
    }
    CodeStep::Emit {code,after}
}
} // verus!

verus! {
pub proof fn lemma_agrees_grow(t: Seq<u8>, nt: Seq<u8>, st: St, ns: St, clear: int, grow: bool, p: u16, v: u8)
    requires table_len_ok(t), table_len_ok(nt),
        st.pf.len()==4096,st.sf.len()==4096,0<=clear,clear+2<=st.next<=4096,
        grow ==> st.next<4096,
        table_agrees(t,st.pf,st.sf,clear,st.next-1),
        ns.next==st.next+if grow {1int} else {0int},
        ns.pf==if grow {st.pf.update(st.next,p)} else {st.pf},
        ns.sf==if grow {st.sf.update(st.next,v)} else {st.sf},
        raw_pf_seq(nt)==if grow {raw_pf_seq(t).update(st.next,p)} else {raw_pf_seq(t)},
        raw_sf_seq(nt)==if grow {raw_sf_seq(t).update(st.next,v)} else {raw_sf_seq(t)},
    ensures table_agrees(nt,ns.pf,ns.sf,clear,ns.next-1),
{
    lemma_raw_views(t); lemma_raw_views(nt);
    assert forall|x:int| clear+2<=x<ns.next implies #[trigger] raw_prefix(nt,x)==ns.pf[x] as int && raw_suffix(nt,x)==ns.sf[x] as int by {
        if grow && x==st.next {} else {
            assert(raw_prefix(t,x)==st.pf[x] as int);
            assert(raw_suffix(t,x)==st.sf[x] as int);
            assert(raw_pf_seq(t)[x] as int==raw_prefix(t,x));
            assert(raw_sf_seq(t)[x] as int==raw_suffix(t,x));
        }
        assert(raw_pf_seq(nt)[x]==ns.pf[x]);
        assert(raw_sf_seq(nt)[x]==ns.sf[x]);
        assert(raw_pf_seq(nt)[x] as int==raw_prefix(nt,x));
        assert(raw_sf_seq(nt)[x] as int==raw_suffix(nt,x));
    }
}
} // verus!

verus! {
pub proof fn lemma_table_reset(t: Seq<u8>, clear: int, pf: Seq<u16>, sf: Seq<u8>)
    requires table_len_ok(t), 0<=clear<=2048,
    ensures crate::dictionary::table_ok(raw_pf_seq(t),raw_ls_seq(t),clear,clear+2),
        table_agrees(t,pf,sf,clear,clear+1),
        raw_link_ok(t,clear,clear+1),
{}
} // verus!

verus! {
pub proof fn lemma_empty_observation(ok: spec_fn(u8)->bool)
    ensures log_complete(Seq::empty(),Seq::empty(),0),
        fault_matches(Seq::empty(),Seq::empty(),None,ok),
        log_time_agrees(Seq::empty(),Seq::empty()),
{}
} // verus!

verus! {
pub proof fn lemma_stamps_step(m:int,d:Seq<u8>,a:int,ps:LzwParams,st:St,ns:St)
    requires verdict(m,d,a,ps,st)==Vd::Step(ns), ns.pos>st.pos,
    ensures code_stamps(m,d,a,ps,st)==Seq::new((ns.o.len()-st.o.len()) as nat,|_i:int|ns.pos as u32)+code_stamps(m,d,a,ps,ns),
{reveal(code_stamps);}
pub proof fn lemma_stamps_halt(m:int,d:Seq<u8>,a:int,ps:LzwParams,st:St,r:MRes)
    requires verdict(m,d,a,ps,st)==Vd::Halt(r),
    ensures code_stamps(m,d,a,ps,st)==Seq::new((r.idx.len()-st.o.len()) as nat,|_i:int|r.pos as u32),
{reveal(code_stamps);}
} // verus!

verus! {
/// Physical call order and logical raster order carry the same per-code time.
pub open spec fn log_time_agrees(h: Seq<(u32,u8)>, ts: Seq<u32>) -> bool {
    &&& h.len()==ts.len()
    &&& forall|i:int| 0<=i<h.len() ==> #[trigger] ts[i]==ts[h[i].0 as int]
}

pub proof fn lemma_emission_times(h: Seq<(u32,u8)>, ts: Seq<u32>, o: Seq<u8>, s: Seq<u8>, q:int,a:int,after:u32)
    requires log_complete(h,o,q),log_time_agrees(h,ts),
        0<=q<=a<=crate::limits::MAX_TOTAL_PIXELS, s.len()<=TABLE_SIZE,
    ensures log_time_agrees(h+sink_pairs(s,q,a,0),ts+Seq::new(sink_pairs(s,q,a,0).len(),|_i:int|after)),
{
    let b=sink_pairs(s,q,a,0);
    let nt=ts+Seq::new(b.len(),|_i:int|after);
    lemma_sink_pairs_len(s,q,a,0);
    lemma_sink_pairs_ok(s,q,a,0);
    assert(crate::limits::MAX_TOTAL_PIXELS+TABLE_SIZE<=u32::MAX) by(compute);
    assert forall|j:int| 0<=j<s.len() implies #[trigger] (o+s)[q+j]==s[j] by {};
    lemma_log_block_sink_pairs(s,o+s,q,a);
    assert forall|i:int| 0<=i<(h+b).len() implies #[trigger] nt[i]==nt[(h+b)[i].0 as int] by {
        if i<h.len() {
            assert((h+b)[i]==h[i]);
            assert((h[i].0 as int)<q);
            assert(nt[h[i].0 as int]==ts[h[i].0 as int]);
        } else {
            let k=i-h.len();
            assert((h+b)[i]==b[k]);
            let j=choose|j:int| 0<=j<s.len() && b[k]==((q+j) as u32,s[j]);
            lemma_u32_roundtrip(q+j);
            assert(q<=(b[k].0 as int)<q+b.len());
            assert(nt[b[k].0 as int]==after);
        }
    }
}

/// L-INTERLEAVE's colour projection and R12's first offending code timestamp.
pub proof fn lemma_colour_projection(h: Seq<(u32,u8)>, ts: Seq<u32>, o: Seq<u8>, q:int,
    f: Option<ColourFault>,ok:spec_fn(u8)->bool)
    requires log_complete(h,o,q),log_time_agrees(h,ts),fault_matches(h,ts,f,ok),
    ensures
        (f is None) <==> (forall|p:int| 0<=p<o.len() ==> ok(#[trigger] o[p])),
        match f {
            None=>true,
            Some(c)=> {
                &&& 0<=c.q<o.len()
                &&& !ok(o[c.q as int])
                &&& forall|p:int| 0<=p<(c.q as int) ==> ok(#[trigger] o[p])
                &&& c.pos==ts[c.q as int]
            },
        },
{
    match f {
        None=> {
            assert forall|p:int| 0<=p<o.len() implies ok(#[trigger] o[p]) by {
                assert(log_has_at(h,p));
                let i=choose|i:int| 0<=i<h.len() && (h[i].0 as int)==p;
                assert(h[i].1==o[p]);
            }
        },
        Some(c)=> {
            let i=choose|i:int| 0<=i<h.len() && h[i].0==c.q && !ok(h[i].1) && ts[i]==c.pos;
            assert(h[i].1==o[c.q as int]);
            assert forall|p:int| 0<=p<(c.q as int) implies ok(#[trigger] o[p]) by {
                assert(log_has_at(h,p));
                let j=choose|j:int| 0<=j<h.len() && (h[j].0 as int)==p;
                assert(h[j].1==o[p]);
            }
        },
    }
}
} // verus!

verus! {
pub open spec fn projected_error(r: LzwReport) -> Option<Error> {
    match r.colour { Some(_) => Some(Error::ColorIndexOutOfRange), None => r.error }
}
pub open spec fn projected_pos(r: LzwReport) -> u32 {
    match r.colour { Some(c) => c.pos, None => r.pos }
}
pub open spec fn colour_observation(r: LzwReport, o: Seq<u8>, ok: spec_fn(u8)->bool) -> bool {
    &&& (r.colour is None) <==> (forall|p:int| 0<=p<o.len() ==> ok(#[trigger] o[p]))
    &&& match r.colour {
        None => true,
        Some(c) => {
            &&& c.q<o.len()
            &&& !ok(o[c.q as int])
            &&& forall|p:int| 0<=p<(c.q as int) ==> ok(#[trigger] o[p])
            &&& c.pos==r.times@[c.q as int]
        },
    }
}

/// Palette rejection precedes the pure LZW result without adding a palette
/// branch to the pure G10 model's `verdict`.
pub proof fn lemma_public_projection(r:LzwReport, mr:MRes, ok:spec_fn(u8)->bool)
    requires report_matches(r,mr), colour_observation(r,mr.idx,ok),
    ensures
        projected_error(r)==if exists|p:int| 0<=p<mr.idx.len() && !ok(#[trigger] mr.idx[p]) {
            Some(Error::ColorIndexOutOfRange)
        } else {match mr.error {None=>None,Some(e)=>Some(err_of(e))}},
        match r.colour {
            None=> projected_pos(r) as int==mr.pos,
            Some(c)=> projected_error(r)==Some(Error::ColorIndexOutOfRange)
                && projected_pos(r)==r.times@[c.q as int],
        },
{}
} // verus!

#[cfg(test)]
#[path = "lzw_tests.rs"]
mod tests;

verus! {
impl<'a> BitReader<'a> {
    /// G9's structural scan supplies every future payload-fetch precondition.
    pub fn new_scanned(b:&'a [u8],first:usize,data_len:u32)->(r:BitReader<'a>)
        requires b.len()<=crate::limits::MAX_INPUT,data_len<=crate::limits::MAX_INPUT,
            crate::model::sub_scan(b@,first as int) is Some,
            read_d(b@,first as int).len()==data_len,
        ensures reader_ready(&r),r.b==b,r.seq_start==first,r.data_len==data_len,
            r.total_bits==r.bits_left,r.fetched==0,r.nacc==0,r.acc==0,
    {
        let r=BitReader::new(b,first,data_len);
        proof {
            let scanned=crate::model::sub_scan(b@,first as int).unwrap();
            crate::model::lemma_concat_len(scanned.0,Seq::empty());
            crate::model::lemma_rd_step_ok_all(b@,first as int,scanned.0,scanned.1);
            assert(crate::model::concat_len(scanned.0)==data_len);
            assert(crate::model::rd_state(b@,first as int,0)==(first as int,0));
            assert((0u32 >> 0u32)==0u32) by(bit_vector);
            assert(crate::model::acc_view(b@,first as int,0,0,0));
            assert forall|j:int| 0<=j<data_len as int implies #[trigger] crate::model::rd_step_ok(r.b@,r.seq_start as int,j) by {};
            assert(reader_ready(&r));
        }
        r
    }
}

pub proof fn lemma_min_admissibility(kind:crate::policy::LzwMin,m:u32,area:u32,opts:LzwOptions)
    requires crate::parse::lzw_min_ok(kind,m as int,area as int),
        opts.checked_literals==(kind==crate::policy::LzwMin::Range1to11Checked),
    ensures 1<=m<=11,m_ok(lzw_ps(m,opts),m as int,area as int),lzw_ps_known(m,opts),
{}

/// Export the successful G9 parser's facts in the exact core-entry vocabulary.
pub proof fn lemma_image_data_ready(b:Seq<u8>,p:int,pol:crate::policy::Policy,area:u32,data:crate::parse::ImageData)
    requires crate::parse::image_data_post(b,p,pol,area as int,Ok(data)),
    ensures crate::parse::lzw_min_ok(pol.lzw_min,data.min_code_size as int,area as int),
        crate::model::sub_scan(b,data.first as int) is Some,
        read_d(b,data.first as int).len()==data.data_len,
        data.data_len<=crate::limits::MAX_INPUT,
{}

/// The frame path uses the G9 record directly: no contiguous payload copy and
/// no index buffer. The preceding scan establishes minimum-size admissibility.
pub fn decode_image_data<S:Sink>(b:&[u8],data:crate::parse::ImageData,area:u32,opts:LzwOptions,
    kind:crate::policy::LzwMin,t:&mut [u8],sink:&mut S)->(rep:LzwReport)
    requires b.len()<=crate::limits::MAX_INPUT,area<=crate::limits::MAX_TOTAL_PIXELS,
        crate::parse::lzw_min_ok(kind,data.min_code_size as int,area as int),
        opts.checked_literals==(kind==crate::policy::LzwMin::Range1to11Checked),
        crate::model::sub_scan(b@,data.first as int) is Some,
        read_d(b@,data.first as int).len()==data.data_len,data.data_len<=crate::limits::MAX_INPUT,
        table_len_ok(old(t)@),old(sink).sink_inv(),old(sink).sink_area()==area as int,
    ensures
        final(sink).sink_future()==old(sink).sink_future(),
        final(sink).sink_effect()==old(sink).sink_effect(),
        final(sink).sink_view().len()==old(sink).sink_view().len(),
        final(sink).sink_view()==effect_fold(old(sink).sink_view(),rep.h@,old(sink).sink_effect()),
        table_len_ok(final(t)@),final(sink).sink_inv(),final(sink).sink_area()==area as int,
        forall|v:u8| final(sink).sink_ok(v)==old(sink).sink_ok(v),
        final(sink).sink_log()==old(sink).sink_log()+rep.h@,
        report_matches(rep,ML(data.min_code_size as int,read_d(b@,data.first as int),area as int,lzw_ps(data.min_code_size,opts))),
        log_complete(rep.h@,ML(data.min_code_size as int,read_d(b@,data.first as int),area as int,lzw_ps(data.min_code_size,opts)).idx,rep.written as int),
        raster(rep.h@)==ML(data.min_code_size as int,read_d(b@,data.first as int),area as int,lzw_ps(data.min_code_size,opts)).idx,
        rep.times@==code_stamps(data.min_code_size as int,read_d(b@,data.first as int),area as int,lzw_ps(data.min_code_size,opts),init_st(data.min_code_size as int)),
        fault_matches(rep.h@,rep.times@,rep.colour,|v:u8|old(sink).sink_ok(v)),log_time_agrees(rep.h@,rep.times@),
        colour_observation(rep,ML(data.min_code_size as int,read_d(b@,data.first as int),area as int,lzw_ps(data.min_code_size,opts)).idx,|v:u8|old(sink).sink_ok(v)),
        rep.written<=area,rep.pos as int<=8*(data.data_len as int),
{
    proof {lemma_min_admissibility(kind,data.min_code_size,area,opts);}
    let mut reader=BitReader::new_scanned(b,data.first,data.data_len);
    decode(&mut reader,data.data_len,data.min_code_size,area,opts,t,sink)
}
} // verus!

verus! {
pub proof fn lemma_raw_table_links(t:Seq<u8>,clear:int,next:int)
    requires table_len_ok(t),crate::dictionary::table_ok(raw_pf_seq(t),raw_ls_seq(t),clear,next),
    ensures raw_link_ok(t,clear,next-1),
{
    lemma_raw_views(t);
    assert forall|x:int| clear+2<=x<next implies 0<=#[trigger] crate::chain::raw_pf(t,x)
        && (crate::chain::raw_pf(t,x)<clear || clear+2<=crate::chain::raw_pf(t,x)<x) by {
        assert(crate::chain::link_ok(raw_pf_seq(t),clear,x));
        assert(raw_pf_seq(t)[x] as int==raw_prefix(t,x));
    }
}
} // verus!

verus! {
pub open spec fn effect_fold(cv:Seq<u8>,h:Seq<(u32,u8)>,effect:spec_fn(Seq<u8>,u32,u8)->Seq<u8>)->Seq<u8>
    decreases h.len(),
{
    if h.len()==0 {cv} else {effect_fold(effect(cv,h[0].0,h[0].1),h.drop_first(),effect)}
}
pub proof fn lemma_effect_append(cv:Seq<u8>,h:Seq<(u32,u8)>,b:Seq<(u32,u8)>,effect:spec_fn(Seq<u8>,u32,u8)->Seq<u8>)
    ensures effect_fold(cv,h+b,effect)==effect_fold(effect_fold(cv,h,effect),b,effect),
    decreases h.len(),
{
    if h.len()>0 {
        assert((h+b).drop_first()==h.drop_first()+b);
        lemma_effect_append(effect(cv,h[0].0,h[0].1),h.drop_first(),b,effect);
    }
}
pub proof fn lemma_effect_push(cv:Seq<u8>,h:Seq<(u32,u8)>,p:u32,v:u8,effect:spec_fn(Seq<u8>,u32,u8)->Seq<u8>)
    ensures effect_fold(cv,h.push((p,v)),effect)==effect(effect_fold(cv,h,effect),p,v),
{
    assert(h.push((p,v))==h+seq![(p,v)]);
    lemma_effect_append(cv,h,seq![(p,v)],effect);
    reveal_with_fuel(effect_fold,2);
}
pub proof fn lemma_effect_empty(cv:Seq<u8>,effect:spec_fn(Seq<u8>,u32,u8)->Seq<u8>)
    ensures effect_fold(cv,Seq::empty(),effect)==cv,
{}
} // verus!

verus! {
pub proof fn lemma_control_emit(m:int,d:Seq<u8>,a:int,ps:LzwParams,st:St,code:u32,after:u32)
    requires control_matches(CodeStep::Emit {code,after},m,d,a,ps,st),
    ensures verdict(m,d,a,ps,st)=={
        let s=semit(m,st,code as int);
        if st.q+s.len()>a {Vd::Halt(crate::lzw_model::observation(st.o+s.take(a-st.q),if ps.stop_at_raster {None} else {Some(ErrK::TooManyPixels)},after as int,ps.stop_at_raster))}
        else {Vd::Step(emit_st(m,st,after as int,code as int,s,first_of(m,st,code as int),st.prev>=0 && st.next<4096))}
    },
{}
} // verus!

verus! {
pub open spec fn canonical_log(o:Seq<u8>)->Seq<(u32,u8)> {
    Seq::new(o.len(),|p:int|(p as u32,o[p]))
}
/// Coverage, count and values imply the exact forward pair multiset.
pub proof fn lemma_log_canonical_multiset(h:Seq<(u32,u8)>,o:Seq<u8>)
    requires log_complete(h,o,o.len() as int),o.len()<=u32::MAX,
    ensures h.to_multiset()==canonical_log(o).to_multiset(),
{
    broadcast use vstd::multiset::group_multiset_axioms;
    let f=canonical_log(o);
    assert(f.no_duplicates()) by {
        assert forall|i:int,j:int| 0<=i<j<f.len() implies #[trigger] f[i]!=#[trigger] f[j] by {
            lemma_u32_roundtrip(i);lemma_u32_roundtrip(j);
        }
    }
    f.lemma_multiset_has_no_duplicates();
    assert(f.to_multiset().subset_of(h.to_multiset())) by {
        assert forall|v:(u32,u8)| #[trigger] f.to_multiset().count(v)<=#[trigger] h.to_multiset().count(v) by {
            vstd::seq_lib::to_multiset_contains(f,v);
            vstd::seq_lib::to_multiset_contains(h,v);
            if f.to_multiset().count(v)>0 {
                let p=f.lemma_contains_to_index(v);
                assert(log_has_at(h,p));
                let i=choose|i:int| 0<=i<h.len() && (h[i].0 as int)==p;
                assert(h[i].1==o[p]);
                assert(h[i]==v);
                assert(h.contains(v));
            }
        }
    }
    vstd::seq_lib::to_multiset_len(f);vstd::seq_lib::to_multiset_len(h);
    assert(h.to_multiset().sub(f.to_multiset()).len()==0);
    assert(h.to_multiset() =~= f.to_multiset()) by {
        assert forall|v:(u32,u8)| #[trigger] h.to_multiset().count(v)==#[trigger] f.to_multiset().count(v) by {
            assert(h.to_multiset().sub(f.to_multiset()).count(v)==0);
        }
    }
}
} // verus!
