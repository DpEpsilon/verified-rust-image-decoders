//! Formal reference model for M2 (spec functions and lemmas only).
//!
//! Written independently of the executable code in `crate::icns`; the executables
//! carry postconditions that link their results to these definitions. M2 needs the
//! R8/R9 classification model and the parse-success invariant (T6). The full
//! `MP`/`MR`/`MC`/`MA`/`MJ` reference model is the subject of M3/M4.
use vstd::prelude::*;
use alloc::vec::Vec;
#[cfg(verus_only)]
use vstd::seq::{lemma_seq_new_len, lemma_seq_new_index, lemma_seq_subrange_composition, lemma_seq_subrange_index, lemma_seq_subrange_len};
#[cfg(verus_only)]
use vstd::seq::{lemma_seq_push_index_same, lemma_seq_push_index_different};
#[cfg(verus_only)]
use vstd::seq_lib::lemma_concat_associative;
#[cfg(verus_only)]
use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_small_mod};
#[cfg(verus_only)]
use vstd::seq::{lemma_seq_add_len, lemma_seq_add_index1, lemma_seq_add_index2};

verus! {

#[cfg(verus_only)]
use crate::icns::{Entry, Error, Decoded, MAX_ENTRIES, MAX_FILE_LEN};
#[cfg(verus_only)]
use crate::alloc_guard::sites::{AllocSite, TryError};

// ---------------------------------------------------------------------------
// Hexadecimal encoding of a four-byte type code (R1: lowercase, eight digits).
// ---------------------------------------------------------------------------

pub open spec fn hex_digit(v: u8) -> char {
    if v == 0 { '0' } else if v == 1 { '1' } else if v == 2 { '2' } else if v == 3 { '3' }
    else if v == 4 { '4' } else if v == 5 { '5' } else if v == 6 { '6' } else if v == 7 { '7' }
    else if v == 8 { '8' } else if v == 9 { '9' } else if v == 10 { 'a' } else if v == 11 { 'b' }
    else if v == 12 { 'c' } else if v == 13 { 'd' } else if v == 14 { 'e' } else { 'f' }
}

pub open spec fn hex_val(c: char) -> int {
    if c == '0' { 0 } else if c == '1' { 1 } else if c == '2' { 2 } else if c == '3' { 3 }
    else if c == '4' { 4 } else if c == '5' { 5 } else if c == '6' { 6 } else if c == '7' { 7 }
    else if c == '8' { 8 } else if c == '9' { 9 } else if c == 'a' { 10 } else if c == 'b' { 11 }
    else if c == 'c' { 12 } else if c == 'd' { 13 } else if c == 'e' { 14 } else if c == 'f' { 15 }
    else { -1 }
}

/// `hex4(code)` is the eight-character lowercase hex encoding of the four bytes.
pub open spec fn hex4(code: [u8; 4]) -> Seq<char> {
    seq![
        hex_digit(code[0] / 16), hex_digit(code[0] % 16),
        hex_digit(code[1] / 16), hex_digit(code[1] % 16),
        hex_digit(code[2] / 16), hex_digit(code[2] % 16),
        hex_digit(code[3] / 16), hex_digit(code[3] % 16),
    ]
}

/// Eight lowercase hex digits.
pub open spec fn is_hex8(s: Seq<char>) -> bool {
    &&& s.len() == 8
    &&& forall|i: int| 0 <= i < 8 ==> 0 <= hex_val(#[trigger] s[i]) <= 15
}

/// The four bytes recovered from an eight-character hex string, two digits per byte.
pub open spec fn code_of(s: Seq<char>) -> [u8; 4] {
    [
        (hex_val(s[0]) * 16 + hex_val(s[1])) as u8,
        (hex_val(s[2]) * 16 + hex_val(s[3])) as u8,
        (hex_val(s[4]) * 16 + hex_val(s[5])) as u8,
        (hex_val(s[6]) * 16 + hex_val(s[7])) as u8,
    ]
}

/// `hex_str(a)`: the model projection of the executable `Entry::type_hex`
/// (`[u8; 8]`, decision D2). The eight ASCII bytes read as eight characters;
/// equal to the frozen `MpEntry.type_hex` for every array the parser builds.
pub open spec fn hex_str(a: [u8; 8]) -> Seq<char> {
    Seq::new(8, |i: int| a[i] as char)
}

/// The spec form of the executable `hex4_bytes`: the eight lowercase hex digits
/// of a four-byte code, as a byte array.
pub open spec fn hex4_bytes_spec(code: [u8; 4]) -> [u8; 8] {
    [
        hex_digit(code[0] / 16) as u8,
        hex_digit(code[0] % 16) as u8,
        hex_digit(code[1] / 16) as u8,
        hex_digit(code[1] % 16) as u8,
        hex_digit(code[2] / 16) as u8,
        hex_digit(code[2] % 16) as u8,
        hex_digit(code[3] / 16) as u8,
        hex_digit(code[3] % 16) as u8,
    ]
}

// ---------------------------------------------------------------------------
// R8/R9 classification.
// ---------------------------------------------------------------------------

/// A four-byte type code equals the four given bytes.
pub open spec fn is_code(code: [u8; 4], b0: u8, b1: u8, b2: u8, b3: u8) -> bool {
    code[0] == b0 && code[1] == b1 && code[2] == b2 && code[3] == b3
}

/// R8: the eight legacy colour/mask codes with their square sides `(is_colour, side)`.
pub open spec fn legacy_side(code: [u8; 4]) -> Option<(bool, u32)> {
    if is_code(code, 105u8, 115, 51, 50) {            // "is32"
        Some((true, 16))
    } else if is_code(code, 105u8, 108, 51, 50) {     // "il32"
        Some((true, 32))
    } else if is_code(code, 105u8, 104, 51, 50) {     // "ih32"
        Some((true, 48))
    } else if is_code(code, 105u8, 116, 51, 50) {     // "it32"
        Some((true, 128))
    } else if is_code(code, 115u8, 56, 109, 107) {    // "s8mk"
        Some((false, 16))
    } else if is_code(code, 108u8, 56, 109, 107) {    // "l8mk"
        Some((false, 32))
    } else if is_code(code, 104u8, 56, 109, 107) {    // "h8mk"
        Some((false, 48))
    } else if is_code(code, 116u8, 56, 109, 107) {    // "t8mk"
        Some((false, 128))
    } else {
        None
    }
}

/// R9's fourteen-code table: `nominal_side`.
pub open spec fn nominal_side(code: [u8; 4]) -> Option<u32> {
    if is_code(code, 105u8, 99, 112, 52) {            // "icp4"
        Some(16)
    } else if is_code(code, 105u8, 99, 112, 53) {     // "icp5"
        Some(32)
    } else if is_code(code, 105u8, 99, 112, 54) {     // "icp6"
        Some(64)
    } else if is_code(code, 105u8, 99, 48, 55) {      // "ic07"
        Some(128)
    } else if is_code(code, 105u8, 99, 48, 56) {      // "ic08"
        Some(256)
    } else if is_code(code, 105u8, 99, 48, 57) {      // "ic09"
        Some(512)
    } else if is_code(code, 105u8, 99, 49, 48) {      // "ic10"
        Some(1024)
    } else if is_code(code, 105u8, 99, 49, 49) {      // "ic11"
        Some(32)
    } else if is_code(code, 105u8, 99, 49, 50) {      // "ic12"
        Some(64)
    } else if is_code(code, 105u8, 99, 49, 51) {      // "ic13"
        Some(256)
    } else if is_code(code, 105u8, 99, 49, 52) {      // "ic14"
        Some(512)
    } else if is_code(code, 105u8, 99, 115, 66) {     // "icsB"
        Some(36)
    } else if is_code(code, 115u8, 98, 50, 52) {      // "sb24"
        Some(24)
    } else if is_code(code, 83u8, 66, 50, 52) {       // "SB24"
        Some(48)
    } else {
        None
    }
}

pub open spec fn png_signature() -> Seq<u8> {
    seq![0x89u8, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]
}

pub open spec fn jp2_signature() -> Seq<u8> {
    seq![0x00u8, 0x00, 0x00, 0x0c, 0x6a, 0x50, 0x20, 0x20, 0x0d, 0x0a, 0x87, 0x0a]
}

/// `p` begins with the full eight-byte PNG or twelve-byte JP2 signature.
pub open spec fn has_signature(p: Seq<u8>) -> bool {
    ||| (p.len() >= 8 && p.subrange(0, 8) == png_signature())
    ||| (p.len() >= 12 && p.subrange(0, 12) == jp2_signature())
}

/// R9 classification: `(opaque_candidate, nominal_side)` for a type code and payload.
pub open spec fn classify(code: [u8; 4], payload: Seq<u8>) -> (bool, Option<u32>) {
    if legacy_side(code).is_some() {
        (false, None)
    } else if nominal_side(code).is_some() {
        (true, nominal_side(code))
    } else {
        (has_signature(payload), None)
    }
}

// ---------------------------------------------------------------------------
// Entry-record model.
// ---------------------------------------------------------------------------

/// The four type bytes recovered from the entry's eight-digit `type_hex` field
/// (decision D2: `[u8; 8]`, read as characters through `hex_str`).
pub open spec fn entry_code(e: Entry) -> [u8; 4] {
    code_of(hex_str(e.type_hex))
}

/// The payload bytes an entry names, as a subrange of `b`.
pub open spec fn entry_payload(b: Seq<u8>, e: Entry) -> Seq<u8> {
    b.subrange(e.payload[0] as int, e.payload[1] as int)
}

/// An entry's payload range is a valid subrange of `b` (the R1 range rule).
pub open spec fn entry_payload_safe(b: Seq<u8>, e: Entry) -> bool {
    0 <= e.payload[0] <= e.payload[1] <= b.len()
}

/// The per-entry part of T6's success invariant.
pub open spec fn entry_inv(b: Seq<u8>, e: Entry) -> bool {
    &&& 8 <= e.payload[0] <= e.payload[1] <= b.len()
    &&& is_hex8(hex_str(e.type_hex))
    &&& hex_str(e.type_hex) == hex4(entry_code(e))
    &&& e.opaque_candidate == classify(entry_code(e), entry_payload(b, e)).0
    &&& e.nominal_side == classify(entry_code(e), entry_payload(b, e)).1
}

/// T6's parse-success invariant (also T1's structural postcondition).
#[allow(non_snake_case)]
pub open spec fn parsed_inv(b: Seq<u8>, E: Seq<Entry>) -> bool {
    &&& b.len() <= MAX_FILE_LEN
    &&& E.len() <= MAX_ENTRIES
    &&& forall|j: int| 0 <= j < E.len() ==> #[trigger] entry_inv(b, E[j])
}

// ---------------------------------------------------------------------------
// Lemmas about the hex encoding.
// ---------------------------------------------------------------------------

proof fn lemma_hex_digit_val(v: u8)
    requires
        v < 16,
    ensures
        hex_val(hex_digit(v)) == v,
{
    if v == 0 {} else if v == 1 {} else if v == 2 {} else if v == 3 {}
    else if v == 4 {} else if v == 5 {} else if v == 6 {} else if v == 7 {}
    else if v == 8 {} else if v == 9 {} else if v == 10 {} else if v == 11 {}
    else if v == 12 {} else if v == 13 {} else if v == 14 {} else {}
}

/// The byte-level nibble of a hex character equals its model value, in the
/// `u8`-to-`char` direction `hex_str` needs. `hex_digit(v) as u8 == v` for every
/// nibble, and the cast back is the identity on the eight ASCII digits.
pub proof fn lemma_hex_digit_u8(v: u8)
    requires
        v < 16,
    ensures
        (hex_digit(v) as u8) < 128,
        ((hex_digit(v) as u8) as char) == hex_digit(v),
{
    if v == 0 {} else if v == 1 {} else if v == 2 {} else if v == 3 {}
    else if v == 4 {} else if v == 5 {} else if v == 6 {} else if v == 7 {}
    else if v == 8 {} else if v == 9 {} else if v == 10 {} else if v == 11 {}
    else if v == 12 {} else if v == 13 {} else if v == 14 {} else {}
}

/// `hex_str` of the spec construction `hex4_bytes_spec` is exactly `hex4`: the
/// bridge between the executable `[u8; 8]` field and the frozen `Seq<char>`.
pub proof fn lemma_hex_str_hex4_bytes(code: [u8; 4])
    ensures
        hex_str(hex4_bytes_spec(code)) == hex4(code),
{
    reveal(hex_str);
    reveal(hex4_bytes_spec);
    reveal(hex4);
    lemma_hex_digit_u8(code[0] / 16);
    lemma_hex_digit_u8(code[0] % 16);
    lemma_hex_digit_u8(code[1] / 16);
    lemma_hex_digit_u8(code[1] % 16);
    lemma_hex_digit_u8(code[2] / 16);
    lemma_hex_digit_u8(code[2] % 16);
    lemma_hex_digit_u8(code[3] / 16);
    lemma_hex_digit_u8(code[3] % 16);
    assert(hex_str(hex4_bytes_spec(code)) =~= hex4(code));
}

/// `code_of` recovers the four bytes from `hex_str` of the spec construction:
/// the array side of `lemma_code_of_hex4`.
pub proof fn lemma_code_of_hex_str_hex4_bytes(code: [u8; 4])
    ensures
        code_of(hex_str(hex4_bytes_spec(code))) == code,
{
    lemma_hex_str_hex4_bytes(code);
    lemma_code_of_hex4(code);
}

/// `hex4` produces exactly eight lowercase hex digits.
pub proof fn lemma_hex4_is_hex8(code: [u8; 4])
    ensures
        is_hex8(hex4(code)),
{
    assert(hex4(code).len() == 8);
    assert forall|i: int| 0 <= i < 8 implies 0 <= hex_val(#[trigger] hex4(code)[i]) <= 15 by {
        if i % 2 == 0 {
            assert(hex4(code)[i] == hex_digit(code[i / 2] / 16));
            lemma_hex_digit_val(code[i / 2] / 16);
        } else {
            assert(hex4(code)[i] == hex_digit(code[i / 2] % 16));
            lemma_hex_digit_val(code[i / 2] % 16);
        }
    }
}

/// For any byte `v`, the two nibble digits re-encode to `v`.
proof fn lemma_byte_nibbles(v: u8)
    ensures
        ((v / 16) * 16 + (v % 16)) as u8 == v,
{
    assert(v / 16 * 16 + v % 16 == v as int) by {
        assert(v % 16 == v as int - (v as int / 16) * 16);
    }
}

/// The byte-level nibble of a hex character equals its model value.
/// Mirrors the executable `hex_nibble`.
pub open spec fn hex_nibble_model(c: u8) -> int {
    if c >= 48u8 && c <= 57u8 {
        c as int - 48
    } else if c >= 97u8 && c <= 102u8 {
        c as int - 97 + 10
    } else {
        0
    }
}

pub proof fn lemma_hex_nibble_model(c: char)
    requires
        0 <= hex_val(c) <= 15,
    ensures
        hex_nibble_model(c as u8) == hex_val(c),
{
    assert(('0' <= c && c <= '9') || ('a' <= c && c <= 'f')) by {
        if c == '0' {} else if c == '1' {} else if c == '2' {} else if c == '3' {}
        else if c == '4' {} else if c == '5' {} else if c == '6' {} else if c == '7' {}
        else if c == '8' {} else if c == '9' {} else if c == 'a' {} else if c == 'b' {}
        else if c == 'c' {} else if c == 'd' {} else if c == 'e' {} else if c == 'f' {}
        else { assert(hex_val(c) == -1); }
    }
}

/// Decoding `hex4(code)` with `code_of` recovers `code`.
pub proof fn lemma_code_of_hex4(code: [u8; 4])
    ensures
        code_of(hex4(code)) == code,
{
    lemma_hex_digit_val(code[0] / 16);
    lemma_hex_digit_val(code[0] % 16);
    lemma_hex_digit_val(code[1] / 16);
    lemma_hex_digit_val(code[1] % 16);
    lemma_hex_digit_val(code[2] / 16);
    lemma_hex_digit_val(code[2] % 16);
    lemma_hex_digit_val(code[3] / 16);
    lemma_hex_digit_val(code[3] % 16);
    lemma_byte_nibbles(code[0]);
    lemma_byte_nibbles(code[1]);
    lemma_byte_nibbles(code[2]);
    lemma_byte_nibbles(code[3]);
    assert(hex4(code)[0] == hex_digit(code[0] / 16));
    assert(hex4(code)[1] == hex_digit(code[0] % 16));
    assert(hex4(code)[2] == hex_digit(code[1] / 16));
    assert(hex4(code)[3] == hex_digit(code[1] % 16));
    assert(hex4(code)[4] == hex_digit(code[2] / 16));
    assert(hex4(code)[5] == hex_digit(code[2] % 16));
    assert(hex4(code)[6] == hex_digit(code[3] / 16));
    assert(hex4(code)[7] == hex_digit(code[3] % 16));
    assert(code_of(hex4(code))[0] == code[0]);
    assert(code_of(hex4(code))[1] == code[1]);
    assert(code_of(hex4(code))[2] == code[2]);
    assert(code_of(hex4(code))[3] == code[3]);
}

/// An eight-digit lowercase hex string is ASCII.
pub proof fn lemma_hex8_is_ascii(s: Seq<char>)
    requires
        is_hex8(s),
    ensures
        vstd::utf8::is_ascii_chars(s),
{
    assert forall|i: int| 0 <= i < s.len() implies '\0' <= #[trigger] s[i] <= '\u{7f}' by {
        assert(0 <= hex_val(s[i]) <= 15);
        if s[i] == '0' {} else if s[i] == '1' {} else if s[i] == '2' {} else if s[i] == '3' {}
        else if s[i] == '4' {} else if s[i] == '5' {} else if s[i] == '6' {} else if s[i] == '7' {}
        else if s[i] == '8' {} else if s[i] == '9' {} else if s[i] == 'a' {} else if s[i] == 'b' {}
        else if s[i] == 'c' {} else if s[i] == 'd' {} else if s[i] == 'e' {} else {}
    }
}


// ===========================================================================
// M3 — the total reference model MP (THEOREMS.md "Total reference model").
// Written independently of the executable parser in `crate::icns`; the
// executable carries the postcondition `parse(bytes) == mp(bytes@)@` (T7).
// ===========================================================================

pub enum MpError {
    FileTooLarge,
    TruncatedHeader,
    BadMagic,
    InvalidFileLength,
    TruncatedFile,
    TrailingFileBytes,
    TruncatedEntryHeader,
    InvalidEntryLength,
    EntryOutOfBounds,
    TooManyEntries,
}

pub struct MpEntry {
    pub type_hex: Seq<char>,
    pub payload: (u32, u32),
    pub opaque_candidate: bool,
    pub nominal_side: Option<u32>,
}

/// `U32BE(b,p)`, defined when `b[p..p+4]` exists.
pub open spec fn be32u(b: Seq<u8>, p: int) -> u32 {
    ((b[p] as int) * 16777216 + (b[p+1] as int) * 65536 + (b[p+2] as int) * 256
        + (b[p+3] as int)) as u32
}

/// `BE32(x)`, the four-byte big-endian encoding of a `u32` value.
pub open spec fn be32_bytes(x: u32) -> Seq<u8> {
    seq![
        ((x as int / 16777216) % 256) as u8,
        ((x as int / 65536) % 256) as u8,
        ((x as int / 256) % 256) as u8,
        (x as int % 256) as u8,
    ]
}

pub open spec fn code4(b: Seq<u8>, off: int) -> [u8; 4] {
    [b[off], b[off + 1], b[off + 2], b[off + 3]]
}

/// The entry recorded for a header at `off` (payload `[off+8, off+q)`).
pub open spec fn entry_at(b: Seq<u8>, off: int) -> MpEntry {
    MpEntry {
        type_hex: hex4(code4(b, off)),
        payload: ((off + 8) as u32, (off + (be32u(b, off + 4) as int)) as u32),
        opaque_candidate: classify(
            code4(b, off),
            b.subrange(off + 8, off + (be32u(b, off + 4) as int)),
        ).0,
        nominal_side: classify(
            code4(b, off),
            b.subrange(off + 8, off + (be32u(b, off + 4) as int)),
        ).1,
    }
}

pub open spec fn icns_magic() -> Seq<u8> {
    seq![105u8, 99, 110, 115]
}

pub open spec fn cons<T>(x: T, s: Seq<T>) -> Seq<T> {
    Seq::new(s.len() + 1, |i: int| if i == 0 { x } else { s[i - 1] })
}

pub proof fn lemma_cons_is_singleton_add<T>(x: T, s: Seq<T>)
    ensures cons(x, s) == seq![x] + s,
{
    reveal(cons);
    assert(cons(x, s).len() == (seq![x] + s).len()) by {
        lemma_seq_add_len(seq![x], s);
        lemma_seq_new_len(s.len() + 1, |i: int| if i == 0 { x } else { s[i - 1] });
    }
    assert forall|i: int| 0 <= i < cons(x, s).len() implies
        #[trigger] cons(x, s)[i] == (seq![x] + s)[i] by {
        lemma_seq_new_index(s.len() + 1, |j: int| if j == 0 { x } else { s[j - 1] }, i);
        if i == 0 {
            lemma_seq_add_index1(seq![x], s, 0);
        } else {
            lemma_seq_add_index2(seq![x], s, i);
        }
    }
    assert(cons(x, s) =~= seq![x] + s);
}

pub open spec fn entries_at(b: Seq<u8>, os: Seq<u32>) -> Seq<MpEntry> {
    os.map_values(|o: u32| entry_at(b, o as int))
}

/// The entry scan of MP, from offset `off` with `acc` entries already accepted.
pub open spec fn scan(b: Seq<u8>, L: u64, off: u64, acc: Seq<MpEntry>) -> Result<Seq<MpEntry>, MpError>
    decreases (L - off) as nat,
{
    if off >= L {
        Ok(acc)
    } else if L - off < 8 {
        Err(MpError::TruncatedEntryHeader)
    } else {
        let q = be32u(b, off as int + 4) as u64;
        if q < 8 {
            Err(MpError::InvalidEntryLength)
        } else if q > L - off {
            Err(MpError::EntryOutOfBounds)
        } else if acc.len() >= 4096 {
            Err(MpError::TooManyEntries)
        } else {
            scan(b, L, (off + q) as u64, acc.push(entry_at(b, off as int)))
        }
    }
}

/// `MP(b)`: the ordered, finite algorithm of the total reference model.
pub open spec fn mp(b: Seq<u8>) -> Result<Seq<MpEntry>, MpError> {
    if b.len() > 67108864 {
        Err(MpError::FileTooLarge)
    } else if b.len() < 8 {
        Err(MpError::TruncatedHeader)
    } else if b.subrange(0, 4) != icns_magic() {
        Err(MpError::BadMagic)
    } else {
        let L = be32u(b, 4) as u64;
        if L < 8 {
            Err(MpError::InvalidFileLength)
        } else if L > b.len() as u64 {
            Err(MpError::TruncatedFile)
        } else if L < b.len() as u64 {
            Err(MpError::TrailingFileBytes)
        } else {
            scan(b, L, 8, Seq::empty())
        }
    }
}

/// `Accepted(b) ⇔ ∃E. MP(b) = Ok(E)`; `P(b,E) ⇔ MP(b) = Ok(E)`.
pub open spec fn accepted(b: Seq<u8>) -> bool {
    exists|E: Seq<MpEntry>| #[trigger] mp(b) == Ok(E)
}

pub open spec fn p_of(b: Seq<u8>, E: Seq<MpEntry>) -> bool {
    mp(b) == Ok(E)
}

/// The count-aware walk: the offsets of the entries `scan` accepts, `j` accepted.
pub open spec fn walk(b: Seq<u8>, L: u64, off: u64, j: nat) -> Result<Seq<u32>, MpError>
    decreases (L - off) as nat,
{
    if off >= L {
        Ok(Seq::empty())
    } else if L - off < 8 {
        Err(MpError::TruncatedEntryHeader)
    } else {
        let q = be32u(b, off as int + 4) as u64;
        if q < 8 {
            Err(MpError::InvalidEntryLength)
        } else if q > L - off {
            Err(MpError::EntryOutOfBounds)
        } else if j >= 4096 {
            Err(MpError::TooManyEntries)
        } else {
            match walk(b, L, (off + q) as u64, j + 1) {
                Ok(rest) => Ok(cons(off as u32, rest)),
                Err(e) => Err(e),
            }
        }
    }
}

/// `mp_at(b)`: MP on a slice whose declared length equals its actual length.
pub open spec fn mp_at(b: Seq<u8>) -> Result<Seq<MpEntry>, MpError> {
    scan(b, b.len() as u64, 8, Seq::empty())
}

// ---- The bridge: scan == walk, appended ------------------------------------

proof fn lemma_add_empty(a: Seq<MpEntry>)
    ensures a + Seq::empty() == a,
{
    assert(a + Seq::empty() =~= a);
}

pub proof fn lemma_entries_at_cons(b: Seq<u8>, os: Seq<u32>, o: u32)
    ensures entries_at(b, cons(o, os)) == cons(entry_at(b, o as int), entries_at(b, os)),
{
    reveal(entries_at);
    assert(cons(o, os).len() == os.len() + 1) by {
        lemma_seq_new_len(os.len() + 1, |i: int| if i == 0 { o } else { os[i - 1] });
    }
    assert forall|i: int| 0 <= i < entries_at(b, cons(o, os)).len()
        implies #[trigger] entries_at(b, cons(o, os))[i]
            == cons(entry_at(b, o as int), entries_at(b, os))[i] by {
        assert(entries_at(b, cons(o, os))[i] == entry_at(b, cons(o, os)[i] as int)) by {
            lemma_seq_new_index(os.len() + 1, |j: int| entry_at(b, cons(o, os)[j] as int), i);
        }
        assert(cons(o, os)[i] == (if i == 0 { o } else { os[i - 1] })) by {
            lemma_seq_new_index(os.len() + 1, |j: int| if j == 0 { o } else { os[j - 1] }, i);
        }
        assert(cons(entry_at(b, o as int), entries_at(b, os))[i]
            == (if i == 0 { entry_at(b, o as int) } else { entries_at(b, os)[i - 1] })) by {
            lemma_seq_new_index(os.len() + 1,
                |j: int| if j == 0 { entry_at(b, o as int) } else { entries_at(b, os)[j - 1] }, i);
        }
        if i != 0 {
            assert(entries_at(b, os)[i - 1] == entry_at(b, os[i - 1] as int)) by {
                lemma_seq_new_index(os.len(), |j: int| entry_at(b, os[j] as int), i - 1);
            }
        }
    }
}

pub proof fn lemma_scan_walk(b: Seq<u8>, L: u64, off: u64, acc: Seq<MpEntry>, k: nat)
    requires
        k == acc.len(),
        L <= 0xffff_ffffu64,
    ensures
        scan(b, L, off, acc) == match walk(b, L, off, k) {
            Ok(os) => Ok(acc + entries_at(b, os)),
            Err(e) => Err(e),
        },
    decreases (L - off) as nat,
{
    reveal(scan); reveal(walk);
    if off >= L {
        reveal(walk);
        assert(walk(b, L, off, k) == Ok(Seq::empty()));
        assert(entries_at(b, Seq::<u32>::empty()) =~= Seq::<MpEntry>::empty()) by {
            lemma_seq_new_len(0, |i: int| entry_at(b, Seq::<u32>::empty()[i] as int));
        }
        assert(acc + entries_at(b, Seq::<u32>::empty()) == acc) by {
            assert(acc + Seq::<MpEntry>::empty() =~= acc);
        }
    } else if L - off < 8 {
        reveal(walk);
    } else {
        let q = be32u(b, off as int + 4) as u64;
        if q < 8 {
            reveal(walk);
        } else if q > L - off {
            reveal(walk);
        } else if k >= 4096 {
            reveal(walk);
        } else {
            reveal(walk);
            let acc2 = acc.push(entry_at(b, off as int));
            assert(acc2.len() == k + 1);
            lemma_scan_walk(b, L, (off + q) as u64, acc2, k + 1);
            match walk(b, L, (off + q) as u64, k + 1) {
                Ok(rest) => {
                    lemma_entries_at_cons(b, rest, off as u32);
                    assert(acc + cons(entry_at(b, off as int), entries_at(b, rest))
                        =~= acc2 + entries_at(b, rest));
                },
                Err(e) => {},
            }
        }
    }
}

/// `mp_at(b)` in terms of `walk`: it is `Ok(entries_at …)` or the first error.
pub proof fn lemma_mp_at_walk(b: Seq<u8>)
    requires
        b.len() <= 0xffff_ffffu64,
    ensures
        mp_at(b) == match walk(b, b.len() as u64, 8, 0) {
            Ok(os) => Ok(entries_at(b, os)),
            Err(e) => Err(e),
        },
{
    reveal(mp_at);
    lemma_add_empty(Seq::<MpEntry>::empty());
    lemma_scan_walk(b, b.len() as u64, 8u64, Seq::empty(), 0);
}

/// `mp(b) == mp_at(b)` whenever the header is well formed and L == |b|.
pub proof fn lemma_mp_eq_mp_at(b: Seq<u8>)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) == icns_magic(),
        (be32u(b, 4) as u64) == b.len() as u64,
    ensures
        mp(b) == mp_at(b),
{
    reveal(mp); reveal(mp_at);
}

// ---- be32_bytes round-trip --------------------------------------------------

pub proof fn lemma_be32_bytes_len(x: u32)
    ensures be32_bytes(x).len() == 4,
{
    lemma_seq_new_len(4, |i: int| 0u8);
}

pub proof fn lemma_be32_bytes_index(x: u32)
    ensures
        be32_bytes(x)[0] == (((x as int / 16777216) % 256) as u8)
        && be32_bytes(x)[1] == (((x as int / 65536) % 256) as u8)
        && be32_bytes(x)[2] == (((x as int / 256) % 256) as u8)
        && be32_bytes(x)[3] == ((x as int % 256) as u8),
{
    lemma_seq_new_index(4, |i: int| 0u8, 0);
    lemma_seq_new_index(4, |i: int| 0u8, 1);
    lemma_seq_new_index(4, |i: int| 0u8, 2);
    lemma_seq_new_index(4, |i: int| 0u8, 3);
}

pub proof fn lemma_be32_roundtrip(x: u32)
    ensures be32u(be32_bytes(x), 0) == x,
{
    lemma_be32_bytes_len(x);
    lemma_be32_bytes_index(x);
    let a = x as int;
    lemma_fundamental_div_mod(a, 256);
    lemma_fundamental_div_mod(a / 256, 256);
    lemma_fundamental_div_mod(a / 65536, 256);
    lemma_fundamental_div_mod(a / 16777216, 256);
    assert(a / 16777216 < 256);
}

// ---- The dot-string family H(k,s) ------------------------------------------

/// One empty entry: `bytes("none") · BE32(8)`.
pub open spec fn empty_entry_bytes() -> Seq<u8> {
    seq![110u8, 111, 110, 101, 0, 0, 0, 8]
}

pub proof fn lemma_empty_entry_len()
    ensures empty_entry_bytes().len() == 8,
{
    assert(empty_entry_bytes().len() == 8);
}

pub open spec fn e_pow(k: nat) -> Seq<u8>
    decreases k,
{
    if k == 0 {
        Seq::empty()
    } else {
        empty_entry_bytes() + e_pow((k - 1) as nat)
    }
}

pub proof fn lemma_e_pow_len(k: nat)
    ensures e_pow(k).len() == 8 * k,
    decreases k,
{
    reveal(e_pow);
    if k == 0 {
        assert(e_pow(0) =~= Seq::<u8>::empty());
    } else {
        lemma_e_pow_len((k - 1) as nat);
        lemma_empty_entry_len();
        lemma_seq_add_len(empty_entry_bytes(), e_pow((k - 1) as nat));
    }
}

/// The offsets `off, off+8, …, off+8(m−1)`, ascending.
pub open spec fn offs(off: int, m: nat) -> Seq<u32>
    decreases m,
{
    if m == 0 {
        Seq::empty()
    } else {
        cons(off as u32, offs(off + 8, (m - 1) as nat))
    }
}

pub proof fn lemma_offs_len(off: int, m: nat)
    ensures offs(off, m).len() == m,
    decreases m,
{
    reveal(offs);
    if m > 0 {
        lemma_offs_len(off + 8, (m - 1) as nat);
        assert(cons(off as u32, offs(off + 8, (m - 1) as nat)).len() == m) by {
            lemma_seq_new_len(m, |i: int| if i == 0 { off as u32 } else { offs(off + 8, (m - 1) as nat)[i - 1] });
        }
    }
}

pub open spec fn none_entry_at(j: nat) -> MpEntry {
    MpEntry {
        type_hex: seq!['6', 'e', '6', 'f', '6', 'e', '6', '5'],
        payload: ((16 + 8 * j) as u32, (16 + 8 * j) as u32),
        opaque_candidate: false,
        nominal_side: None,
    }
}

/// `H(k,s) = bytes("icns") · BE32(8+8k+|s|) · e^k · s`.
pub open spec fn h_string(k: nat, s: Seq<u8>) -> Seq<u8> {
    icns_magic() + be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s
}

pub proof fn lemma_h_len(k: nat, s: Seq<u8>)
    ensures h_string(k, s).len() == 8 + 8 * k + s.len(),
{
    reveal(h_string);
    lemma_e_pow_len(k);
    lemma_be32_bytes_len((8 + 8 * k + s.len()) as u32);
    lemma_seq_add_len(icns_magic(), be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s);
    lemma_seq_add_len(be32_bytes((8 + 8 * k + s.len()) as u32), e_pow(k) + s);
    lemma_seq_add_len(e_pow(k), s);
}

pub proof fn lemma_h_subranges(k: nat, s: Seq<u8>)
    ensures
        h_string(k, s).subrange(0, 4) == icns_magic(),
        h_string(k, s).subrange(4, 8) == be32_bytes((8 + 8 * k + s.len()) as u32),
        h_string(k, s).subrange(8, (8 + 8 * k) as int) == e_pow(k),
        h_string(k, s).subrange((8 + 8 * k) as int, (8 + 8 * k + s.len()) as int) == s,
{
    reveal(h_string);
    lemma_e_pow_len(k);
    lemma_be32_bytes_len((8 + 8 * k + s.len()) as u32);
    lemma_seq_add_len(icns_magic(), be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s);
    lemma_seq_add_len(be32_bytes((8 + 8 * k + s.len()) as u32), e_pow(k) + s);
    lemma_seq_add_len(e_pow(k), s);
    // 0..4
    assert(h_string(k, s).subrange(0, 4) =~= icns_magic()) by {
        lemma_seq_subrange_composition(h_string(k, s), 0int, 4int, 0int, 4int);
        assert forall|i: int| 0 <= i < 4 implies
            #[trigger] h_string(k, s).subrange(0, 4)[i] == icns_magic()[i] by {
            lemma_seq_add_index1(icns_magic(), be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s, i);
            lemma_seq_subrange_index(h_string(k, s), 0int, 4int, i);
        }
    }
    // 4..8
    assert(h_string(k, s).subrange(4, 8) =~= be32_bytes((8 + 8 * k + s.len()) as u32)) by {
        assert forall|i: int| 0 <= i < 4 implies
            #[trigger] h_string(k, s).subrange(4, 8)[i]
                == be32_bytes((8 + 8 * k + s.len()) as u32)[i] by {
            lemma_seq_add_index2(icns_magic(), be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s, i + 4);
            lemma_seq_add_index1(be32_bytes((8 + 8 * k + s.len()) as u32), e_pow(k) + s, i);
            lemma_seq_subrange_index(h_string(k, s), 4int, 8int, i);
        }
    }
    // 8..8+8k
    assert(h_string(k, s).subrange(8, (8 + 8 * k) as int) =~= e_pow(k)) by {
        assert forall|i: int| 0 <= i < 8 * k implies
            #[trigger] h_string(k, s).subrange(8, (8 + 8 * k) as int)[i] == e_pow(k)[i] by {
            lemma_seq_add_index2(icns_magic(), be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s, i + 8);
            lemma_seq_add_index2(be32_bytes((8 + 8 * k + s.len()) as u32), e_pow(k) + s, i + 8 - 4);
            lemma_seq_add_index1(e_pow(k), s, i);
            lemma_seq_subrange_index(h_string(k, s), 8int, (8 + 8 * k) as int, i);
        }
    }
    // 8+8k..8+8k+|s|
    assert(h_string(k, s).subrange((8 + 8 * k) as int, (8 + 8 * k + s.len()) as int) =~= s) by {
        assert forall|i: int| 0 <= i < s.len() implies
            #[trigger] h_string(k, s).subrange((8 + 8 * k) as int, (8 + 8 * k + s.len()) as int)[i] == s[i] by {
            lemma_seq_add_index2(icns_magic(), be32_bytes((8 + 8 * k + s.len()) as u32) + e_pow(k) + s, i + 8 + 8 * k);
            lemma_seq_add_index2(be32_bytes((8 + 8 * k + s.len()) as u32), e_pow(k) + s, i + 8 + 8 * k - 4);
            lemma_seq_add_index2(e_pow(k), s, i + 8 + 8 * k - 8);
            lemma_seq_subrange_index(h_string(k, s), (8 + 8 * k) as int, (8 + 8 * k + s.len()) as int, i);
        }
    }
}

pub open spec fn e_pow_entries(j: nat, m: nat) -> Seq<MpEntry>
    decreases m,
{
    if m == 0 {
        Seq::empty()
    } else {
        cons(none_entry_at(j), e_pow_entries(j + 1, (m - 1) as nat))
    }
}

/// The entry an all-empty 8-byte header at `off` produces.
pub open spec fn empty_entry_at(off: int) -> MpEntry {
    MpEntry {
        type_hex: seq!['6', 'e', '6', 'f', '6', 'e', '6', '5'],
        payload: ((off + 8) as u32, (off + 8) as u32),
        opaque_candidate: false,
        nominal_side: None,
    }
}

/// `e_pow(k)[i] == empty_entry_bytes()[i % 8]` for `i < 8k`.
pub proof fn lemma_e_pow_index(k: nat, i: int)
    requires 0 <= i < 8 * k,
    ensures e_pow(k)[i] == empty_entry_bytes()[i % 8],
    decreases k,
{
    reveal(e_pow);
    if k == 0 {
    } else {
        let r = e_pow((k - 1) as nat);
        lemma_e_pow_len((k - 1) as nat);
        lemma_empty_entry_len();
        if i < 8 {
            lemma_seq_add_index1(empty_entry_bytes(), r, i);
            assert(i % 8 == i) by { lemma_small_mod(i as nat, 8); }
        } else {
            assert(i - 8 < 8 * (k - 1)) by (nonlinear_arith)
                requires i < 8 * k, 8 * (k - 1) == 8 * k - 8;
            lemma_e_pow_index((k - 1) as nat, i - 8);
            lemma_seq_add_index2(empty_entry_bytes(), r, i);
            assert(i % 8 == (i - 8) % 8) by {
                lemma_fundamental_div_mod(i, 8);
                lemma_fundamental_div_mod(i - 8, 8);
            }
        }
    }
}

/// For `i < k`, the eight bytes at `8+8i` of the dot-string are one empty entry.
pub proof fn lemma_dot_slice(b: Seq<u8>, k: nat, i: int)
    requires
        0 <= i < k,
        8 + 8 * k <= b.len(),
        b.subrange(8, (8 + 8 * k) as int) == e_pow(k),
    ensures
        b.subrange(8 + 8 * i, 8 + 8 * i + 8) == empty_entry_bytes(),
{
    lemma_e_pow_len(k);
    lemma_empty_entry_len();
    lemma_seq_subrange_composition(b, 8int, (8 + 8 * k) as int, 8 * i, 8 * i + 8);
    assert(b.subrange(8 + 8 * i, 8 + 8 * i + 8)
        == e_pow(k).subrange(8 * i, 8 * i + 8));
    assert(e_pow(k).subrange(8 * i, 8 * i + 8) =~= empty_entry_bytes()) by {
        assert forall|j: int| 0 <= j < 8 implies
            #[trigger] e_pow(k).subrange(8 * i, 8 * i + 8)[j] == empty_entry_bytes()[j] by {
            lemma_seq_subrange_index(e_pow(k), 8 * i, 8 * i + 8, j);
            assert(0 <= 8 * i + j < 8 * k) by (nonlinear_arith)
                requires 0 <= i < k, 0 <= j < 8;
            lemma_e_pow_index(k, 8 * i + j);
            assert((8 * i + j) % 8 == j) by {
                lemma_fundamental_div_mod(8 * i + j, 8);
                lemma_fundamental_div_mod(j, 8);
                lemma_small_mod(j as nat, 8);
            }
        }
        assert(e_pow(k).subrange(8 * i, 8 * i + 8).len() == 8) by {
            lemma_seq_subrange_len(e_pow(k), 8 * i, 8 * i + 8 as int);
        }
        assert(empty_entry_bytes().len() == 8);
    }
}

/// `entry_at(b, 8+8i)` is `empty_entry_at(8+8i)` for the dot-string.
pub proof fn lemma_dot_entry(b: Seq<u8>, k: nat, i: int)
    requires
        0 <= i < k,
        8 + 8 * k <= b.len(),
        b.subrange(8, (8 + 8 * k) as int) == e_pow(k),
    ensures
        entry_at(b, 8 + 8 * i) == empty_entry_at(8 + 8 * i),
{
    assert(b.subrange(8 + 8 * i, 8 + 8 * i + 8) == empty_entry_bytes()) by {
        lemma_dot_slice(b, k, i);
    }
    lemma_entry_at_empty(b, 8 + 8 * i);
}

/// `entry_at` of the `i`th empty entry is `none_entry_at(i)`.
pub proof fn lemma_dot_none(b: Seq<u8>, k: nat, i: int)
    requires
        0 <= i < k,
        8 + 8 * k <= b.len(),
        b.subrange(8, (8 + 8 * k) as int) == e_pow(k),
    ensures
        entry_at(b, 8 + 8 * i) == none_entry_at(i as nat),
{
    lemma_dot_entry(b, k, i);
    reveal(empty_entry_at); reveal(none_entry_at);
    assert(empty_entry_at(8 + 8 * i) == none_entry_at(i as nat));
}

/// The entry an all-empty 8-byte header at `off` produces.
pub proof fn lemma_entry_at_empty(b: Seq<u8>, off: int)
    requires
        0 <= off,
        off + 8 <= b.len(),
        b.subrange(off, off + 8) == empty_entry_bytes(),
    ensures
        entry_at(b, off) == empty_entry_at(off),
{
    reveal(entry_at); reveal(empty_entry_at); reveal(empty_entry_bytes);
    lemma_empty_entry_len();
    assert(b[off] == 110u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 0);
        assert(empty_entry_bytes()[0] == 110u8);
    }
    assert(b[off + 1] == 111u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 1);
        assert(empty_entry_bytes()[1] == 111u8);
    }
    assert(b[off + 2] == 110u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 2);
        assert(empty_entry_bytes()[2] == 110u8);
    }
    assert(b[off + 3] == 101u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 3);
        assert(empty_entry_bytes()[3] == 101u8);
    }
    assert(b[off + 4] == 0u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 4);
        assert(empty_entry_bytes()[4] == 0u8);
    }
    assert(b[off + 5] == 0u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 5);
        assert(empty_entry_bytes()[5] == 0u8);
    }
    assert(b[off + 6] == 0u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 6);
        assert(empty_entry_bytes()[6] == 0u8);
    }
    assert(b[off + 7] == 8u8) by {
        lemma_seq_subrange_index(b, off, off + 8, 7);
        assert(empty_entry_bytes()[7] == 8u8);
    }
    assert(be32u(b, off + 4) == 8u32);
    assert(code4(b, off) == [110u8, 111u8, 110u8, 101u8]);
    assert(hex4([110u8, 111u8, 110u8, 101u8])
        == seq!['6', 'e', '6', 'f', '6', 'e', '6', '5']) by {
        reveal(hex4); reveal(hex_digit);
    }
    assert(b.subrange(off + 8, off + 8) == Seq::<u8>::empty()) by {
        lemma_seq_subrange_len(b, off + 8, off + 8 as int);
        assert(b.subrange(off + 8, off + 8).len() == 0);
        assert(b.subrange(off + 8, off + 8) =~= Seq::<u8>::empty());
    }
    assert(classify([110u8, 111u8, 110u8, 101u8], b.subrange(off + 8, off + 8))
        == (false, None)) by {
        reveal(classify); reveal(legacy_side); reveal(nominal_side); reveal(has_signature);
        reveal(is_code);
    }
    assert(entry_at(b, off).type_hex == empty_entry_at(off).type_hex);
    assert(entry_at(b, off).payload == empty_entry_at(off).payload);
    assert(entry_at(b, off).opaque_candidate == empty_entry_at(off).opaque_candidate);
    assert(entry_at(b, off).nominal_side == empty_entry_at(off).nominal_side);
    assert(entry_at(b, off) == empty_entry_at(off));
}

/// `offs(off,m)[i] == off + 8i` for `i < m`.
pub proof fn lemma_offs_index(off: int, m: nat, i: int)
    requires 0 <= i < m,
    ensures offs(off, m)[i] == (off + 8 * i) as u32,
    decreases m,
{
    reveal(offs); reveal(cons);
    assert(m > 0);
    let rest = offs(off + 8, (m - 1) as nat);
    assert(offs(off, m) == cons(off as u32, rest));
    assert(offs(off, m) == Seq::new(m, |j: int| if j == 0 { off as u32 } else { rest[j - 1] })) by {
        assert(cons(off as u32, rest) == Seq::new(rest.len() + 1,
            |j: int| if j == 0 { off as u32 } else { rest[j - 1] }));
        lemma_offs_len(off + 8, (m - 1) as nat);
    }
    if i == 0 {
        lemma_seq_new_index(m, |j: int| if j == 0 { off as u32 } else { rest[j - 1] }, 0);
        assert(off + 8 * 0 == off);
    } else {
        lemma_offs_index(off + 8, (m - 1) as nat, i - 1);
        assert(rest[i - 1] == (off + 8 + 8 * (i - 1)) as u32);
        lemma_seq_new_index(m, |j: int| if j == 0 { off as u32 } else { rest[j - 1] }, i);
        assert(off + 8 + 8 * (i - 1) == off + 8 * i);
    }
}

/// The manifest's `E_k = [(none,[16+8j,16+8j],false,None)]_{j<k}`.
pub open spec fn big_e(k: nat) -> Seq<MpEntry> {
    Seq::new(k, |i: int| none_entry_at(i as nat))
}

/// L4 core: for the all-empty dot-string of length `8+8k`, `entries_at` is `big_e`.
pub proof fn lemma_entries_big_e(b: Seq<u8>, k: nat)
    requires
        8 + 8 * k <= b.len(),
        b.len() <= 0xffff_ffffu64,
        b.subrange(8, (8 + 8 * k) as int) == e_pow(k),
    ensures
        entries_at(b, offs(8, k)) == big_e(k),
{
    reveal(entries_at); reveal(big_e);
    lemma_offs_len(8, k);
    assert(offs(8, k).len() == k);
    assert(entries_at(b, offs(8, k)).len() == k) by {
        assert(Seq::<u32>::empty().len() == 0);
    }
    assert forall|i: int| 0 <= i < k implies
        #[trigger] entries_at(b, offs(8, k))[i] == big_e(k)[i] by {
        lemma_seq_new_index(k, |j: int| entry_at(b, offs(8, k)[j] as int), i);
        lemma_offs_index(8, k, i);
        assert(offs(8, k)[i] == (8 + 8 * i) as u32);
        assert((8 + 8 * i) as u32 as int == 8 + 8 * i) by (nonlinear_arith)
            requires 0 <= i < k, 8 + 8 * k <= b.len(), b.len() <= 0xffff_ffffu64;
        lemma_dot_none(b, k, i);
        assert(entry_at(b, 8 + 8 * i) == none_entry_at(i as nat));
        lemma_seq_new_index(k, |j: int| none_entry_at(j as nat), i);
    }
    assert(entries_at(b, offs(8, k)) =~= big_e(k));
}


// ---- T7: linking the executable `parse` result to `mp` ----------------------

/// The executable error for a model error (the first nine `MpError`s share names).
pub open spec fn exec_err(m: MpError) -> Error {
    match m {
        MpError::FileTooLarge => Error::FileTooLarge,
        MpError::TruncatedHeader => Error::TruncatedHeader,
        MpError::BadMagic => Error::BadMagic,
        MpError::InvalidFileLength => Error::InvalidFileLength,
        MpError::TruncatedFile => Error::TruncatedFile,
        MpError::TrailingFileBytes => Error::TrailingFileBytes,
        MpError::TruncatedEntryHeader => Error::TruncatedEntryHeader,
        MpError::InvalidEntryLength => Error::InvalidEntryLength,
        MpError::EntryOutOfBounds => Error::EntryOutOfBounds,
        MpError::TooManyEntries => Error::TooManyEntries,
    }
}

/// The executable record matches its model record exactly (R1's four fields).
pub open spec fn entry_matches(b: Seq<u8>, e: Entry, m: MpEntry) -> bool {
    &&& hex_str(e.type_hex) == m.type_hex
    &&& e.payload[0] == m.payload.0
    &&& e.payload[1] == m.payload.1
    &&& e.opaque_candidate == m.opaque_candidate
    &&& e.nominal_side == m.nominal_side
}

/// `es` matches the entries at the offsets `os`, one for one.
pub open spec fn entries_match(b: Seq<u8>, es: Seq<Entry>, os: Seq<u32>) -> bool {
    &&& es.len() == os.len()
    &&& forall|i: int| 0 <= i < es.len() ==> #[trigger] entry_eq_fields(es[i], entry_at(b, os[i] as int))
}

/// The `parse` loop state: `walk` from 8 produces `os` followed by `walk` from `off`.
pub open spec fn walk_pref(b: Seq<u8>, L: u64, off: u64, os: Seq<u32>) -> bool {
    walk(b, L, 8, 0) == match walk(b, L, off, os.len()) {
        Ok(rest) => Ok(os + rest),
        Err(e) => Err(e),
    }
}

pub proof fn lemma_pref_init(b: Seq<u8>, L: u64)
    ensures walk_pref(b, L, 8, Seq::empty()),
{
    reveal(walk_pref);
    reveal(walk);
    if 8 >= L {
        assert(walk(b, L, 8, 0) == Ok(Seq::empty()));
        assert(Seq::<u32>::empty() + Seq::<u32>::empty() == Seq::<u32>::empty()) by {
            assert(Seq::<u32>::empty() + Seq::<u32>::empty() =~= Seq::<u32>::empty());
        }
    } else if L - 8 < 8 {
    } else {
        let q = be32u(b, 8int + 4) as u64;
        if q < 8 || q > L - 8 {
        } else if 0 >= 4096 {
        } else {
            match walk(b, L, (8 + q) as u64, 1) {
                Ok(rest) => {
                    assert(Seq::<u32>::empty() + cons(8u32, rest) =~= cons(8u32, rest));
                },
                Err(e) => {},
            }
        }
    }
}

/// One accepted empty-header-free step: `walk` at `off` opens with `off`.
pub proof fn lemma_walk_step(b: Seq<u8>, L: u64, off: u64, k: nat)
    requires
        off < L,
        8 <= L - off,
        (be32u(b, off as int + 4) as u64) >= 8,
        (be32u(b, off as int + 4) as u64) <= L - off,
        k < 4096,
    ensures
        walk(b, L, off, k) == match walk(b, L, (off + (be32u(b, off as int + 4) as u64)) as u64, k + 1) {
            Ok(rest) => Ok(cons(off as u32, rest)),
            Err(e) => Err(e),
        },
{
    reveal(walk);
}

pub proof fn lemma_pref_step(b: Seq<u8>, L: u64, off: u64, os: Seq<u32>)
    requires
        walk_pref(b, L, off, os),
        os.len() < 4096,
        off < L,
        8 <= L - off,
        (be32u(b, off as int + 4) as u64) >= 8,
        (be32u(b, off as int + 4) as u64) <= L - off,
    ensures
        walk_pref(b, L, (off + (be32u(b, off as int + 4) as u64)) as u64, os.push(off as u32)),
{
    reveal(walk_pref);
    lemma_walk_step(b, L, off, os.len());
    let q = be32u(b, off as int + 4) as u64;
    let next = (off + q) as u64;
    match walk(b, L, next, os.len() + 1) {
        Ok(rest) => {
            assert(walk(b, L, off, os.len()) == Ok(cons(off as u32, rest)));
            assert(walk(b, L, 8, 0) == Ok(os + cons(off as u32, rest)));
            lemma_cons_is_singleton_add(off as u32, rest);
            assert(os + cons(off as u32, rest) == os + (seq![off as u32] + rest));
            lemma_concat_associative(os, seq![off as u32], rest);
            assert(os + (seq![off as u32] + rest) == (os + seq![off as u32]) + rest);
            assert(os.push(off as u32) == os + seq![off as u32]) by {
                assert(os.push(off as u32) =~= os + seq![off as u32]);
            }
            assert(os + cons(off as u32, rest) == os.push(off as u32) + rest);
        },
        Err(e) => {
            assert(walk(b, L, off, os.len()) == Err(e));
            assert(walk(b, L, 8, 0) == Err(e));
        },
    }
}

/// The final loop state: `walk` from 8 and from `L` agree on `os`.
pub proof fn lemma_pref_done(b: Seq<u8>, L: u64, os: Seq<u32>)
    requires walk_pref(b, L, L, os),
    ensures
        walk(b, L, 8, 0) == Ok(os),
{
    reveal(walk_pref);
    assert(walk(b, L, L, os.len()) == Ok(Seq::empty())) by { reveal(walk); }
    assert(os + Seq::empty() == os) by { assert(os + Seq::<u32>::empty() =~= os); }
}


// ---- T7/T8 statement helpers ------------------------------------------------

/// Two records have identical R1 fields.
pub open spec fn entry_eq_fields(e: Entry, m: MpEntry) -> bool {
    &&& hex_str(e.type_hex) == m.type_hex
    &&& e.payload[0] == m.payload.0
    &&& e.payload[1] == m.payload.1
    &&& e.opaque_candidate == m.opaque_candidate
    &&& e.nominal_side == m.nominal_side
}

/// `es` is exactly the model list `ms`, field for field, in order.
pub open spec fn entries_eq_models(es: Seq<Entry>, ms: Seq<MpEntry>) -> bool {
    &&& es.len() == ms.len()
    &&& forall|i: int| 0 <= i < es.len() ==> #[trigger] entry_eq_fields(es[i], ms[i])
}

/// The exact `Result` relation T7 asserts, on `parse`'s own result type.
pub open spec fn parses_to(b: Seq<u8>, r: Result<Vec<Entry>, Error>) -> bool {
    match mp(b) {
        Ok(ms) => match r {
            Ok(es) => entries_eq_models(es@, ms),
            Err(_) => false,
        },
        Err(m) => match r {
            Ok(_) => false,
            Err(e) => e == exec_err(m),
        },
    }
}

// ---- Early-branch lemmas for `parse` (T7) ----------------------------------

pub proof fn lemma_mp_too_large(b: Seq<u8>)
    requires b.len() > 67108864,
    ensures mp(b) == Err(MpError::FileTooLarge),
{
    reveal(mp);
}

pub proof fn lemma_mp_short(b: Seq<u8>)
    requires b.len() <= 67108864, b.len() < 8,
    ensures mp(b) == Err(MpError::TruncatedHeader),
{
    reveal(mp);
}

pub proof fn lemma_mp_bad_magic(b: Seq<u8>)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) != icns_magic(),
    ensures
        mp(b) == Err(MpError::BadMagic),
{
    reveal(mp);
}

pub proof fn lemma_mp_invalid_len(b: Seq<u8>)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) == icns_magic(),
        (be32u(b, 4) as u64) < 8,
    ensures
        mp(b) == Err(MpError::InvalidFileLength),
{
    reveal(mp);
}

pub proof fn lemma_mp_truncated_file(b: Seq<u8>)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) == icns_magic(),
        (be32u(b, 4) as u64) > b.len() as u64,
    ensures
        mp(b) == Err(MpError::TruncatedFile),
{
    reveal(mp);
}

pub proof fn lemma_mp_trailing(b: Seq<u8>)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) == icns_magic(),
        8 <= (be32u(b, 4) as u64),
        (be32u(b, 4) as u64) < b.len() as u64,
    ensures
        mp(b) == Err(MpError::TrailingFileBytes),
{
    reveal(mp);
}



// ---- prefix-error lemmas used at `parse`'s early returns --------------------

pub proof fn lemma_pref_error_th(b: Seq<u8>, L: u64, off: u64, os: Seq<u32>)
    requires
        walk_pref(b, L, off, os),
        off < L,
        L - off < 8,
    ensures
        walk(b, L, 8, 0) == Err(MpError::TruncatedEntryHeader),
{
    reveal(walk_pref); reveal(walk);
}

pub proof fn lemma_pref_error_iel(b: Seq<u8>, L: u64, off: u64, os: Seq<u32>, q: u64)
    requires
        walk_pref(b, L, off, os),
        off < L,
        8 <= L - off,
        q == be32u(b, off as int + 4) as u64,
        q < 8,
    ensures
        walk(b, L, 8, 0) == Err(MpError::InvalidEntryLength),
{
    reveal(walk_pref); reveal(walk);
}

pub proof fn lemma_pref_error_oob(b: Seq<u8>, L: u64, off: u64, os: Seq<u32>, q: u64)
    requires
        walk_pref(b, L, off, os),
        off < L,
        8 <= L - off,
        q == be32u(b, off as int + 4) as u64,
        q >= 8,
        q > L - off,
    ensures
        walk(b, L, 8, 0) == Err(MpError::EntryOutOfBounds),
{
    reveal(walk_pref); reveal(walk);
}

pub proof fn lemma_pref_error_tme(b: Seq<u8>, L: u64, off: u64, os: Seq<u32>, q: u64)
    requires
        walk_pref(b, L, off, os),
        off < L,
        8 <= L - off,
        q == be32u(b, off as int + 4) as u64,
        q >= 8,
        q <= L - off,
        os.len() >= 4096,
    ensures
        walk(b, L, 8, 0) == Err(MpError::TooManyEntries),
{
    reveal(walk_pref); reveal(walk);
}

// ---- entries_match bookkeeping, and the parse-facing wrappers --------------

/// The freshly spliced executable `Entry` equals `entry_at` at the same offset.
pub proof fn lemma_entries_match_unpack(b: Seq<u8>, es: Seq<Entry>, os: Seq<u32>, i: int)
    requires
        entries_match(b, es, os),
        0 <= i < es.len(),
    ensures
        entry_eq_fields(es[i], entry_at(b, os[i] as int)),
{
    reveal(entries_match);
}

pub proof fn lemma_entries_match_push(b: Seq<u8>, es: Seq<Entry>, os: Seq<u32>, e: Entry, off: u32)
    requires
        entries_match(b, es, os),
        entry_eq_fields(e, entry_at(b, off as int)),
    ensures
        entries_match(b, es.push(e), os.push(off)),
{
    reveal(entries_match);
    assert(es.len() == os.len());
    assert forall|i: int| 0 <= i < es.push(e).len() implies
        #[trigger] entry_eq_fields(es.push(e)[i], entry_at(b, os.push(off)[i] as int)) by {
        if i < es.len() {
            lemma_seq_push_index_different(es, e, i);
            lemma_seq_push_index_different(os, off, i);
            lemma_entries_match_unpack(b, es, os, i);
        } else {
            assert(i == es.len());
            lemma_seq_push_index_same(es, e, i);
            lemma_seq_push_index_same(os, off, i);
        }
    }
}

pub proof fn lemma_entries_match_final(b: Seq<u8>, es: Seq<Entry>, os: Seq<u32>)
    requires
        entries_match(b, es, os),
        walk(b, b.len() as u64, 8, 0) == Ok(os),
    ensures
        entries_eq_models(es, entries_at(b, os)),
{
    reveal(entries_match); reveal(entries_eq_models); reveal(entries_at);
    assert forall|i: int| 0 <= i < es.len() implies
        #[trigger] entry_eq_fields(es[i], entries_at(b, os)[i]) by {
        lemma_seq_new_index(os.len(), |j: int| entry_at(b, os[j] as int), i);
        assert(entries_at(b, os)[i] == entry_at(b, os[i] as int));
        lemma_entries_match_unpack(b, es, os, i);
    }
}

/// The freshly spliced executable `Entry` equals `entry_at` at the same offset.
#[allow(non_snake_case)]
pub proof fn lemma_entry_matches_at(b: Seq<u8>, off: int, e: Entry)
    requires
        0 <= off,
        off + 8 <= b.len(),
        hex_str(e.type_hex) == hex4(code4(b, off)),
        e.payload[0] == (off + 8) as u32,
        e.payload[1] == (off + (be32u(b, off + 4) as int)) as u32,
        e.opaque_candidate == classify(
            code4(b, off), b.subrange(off + 8, off + (be32u(b, off + 4) as int))).0,
        e.nominal_side == classify(
            code4(b, off), b.subrange(off + 8, off + (be32u(b, off + 4) as int))).1,
    ensures
        entry_eq_fields(e, entry_at(b, off)),
{
    reveal(entry_at); reveal(entry_eq_fields);
}


// ===========================================================================
// The manifest's stated lemmas L1, L2, L4, L5, L6, with their instantiations.
// ===========================================================================

/// A four-byte literal read as `[u8; 4]` by `b[off..off+4] == seq![...]`.
pub proof fn lemma_code4_of_subrange(b: Seq<u8>, off: int, code: [u8; 4])
    requires
        0 <= off,
        off + 4 <= b.len(),
        b.subrange(off, off + 4) == seq![code[0], code[1], code[2], code[3]],
    ensures
        code4(b, off) == code,
{
    reveal(code4);
    assert(b[off] == code[0]) by { lemma_seq_subrange_index(b, off, off + 4, 0); }
    assert(b[off + 1] == code[1]) by { lemma_seq_subrange_index(b, off, off + 4, 1); }
    assert(b[off + 2] == code[2]) by { lemma_seq_subrange_index(b, off, off + 4, 2); }
    assert(b[off + 3] == code[3]) by { lemma_seq_subrange_index(b, off, off + 4, 3); }
}

/// `be32u` of a literal four-byte big-endian encoding.
pub proof fn lemma_be32u_of_bytes(b: Seq<u8>, off: int, x: u32)
    requires
        0 <= off,
        off + 4 <= b.len(),
        b.subrange(off, off + 4) == be32_bytes(x),
    ensures
        be32u(b, off) == x,
{
    reveal(be32u);
    lemma_be32_bytes_len(x);
    lemma_be32_bytes_index(x);
    assert(b[off] == be32_bytes(x)[0]) by { lemma_seq_subrange_index(b, off, off + 4, 0); }
    assert(b[off + 1] == be32_bytes(x)[1]) by { lemma_seq_subrange_index(b, off, off + 4, 1); }
    assert(b[off + 2] == be32_bytes(x)[2]) by { lemma_seq_subrange_index(b, off, off + 4, 2); }
    assert(b[off + 3] == be32_bytes(x)[3]) by { lemma_seq_subrange_index(b, off, off + 4, 3); }
    lemma_fundamental_div_mod(x as int, 256);
    lemma_fundamental_div_mod(x as int / 256, 256);
    lemma_fundamental_div_mod(x as int / 65536, 256);
    lemma_fundamental_div_mod(x as int / 16777216, 256);
    assert(x as int / 16777216 < 256);
    assert((x as int / 16777216) * 16777216 + (x as int / 65536 % 256) * 65536
        + (x as int / 256 % 256) * 256 + (x as int % 256) == x as int) by (nonlinear_arith)
        requires
            x as int == (x as int / 256) * 256 + x as int % 256,
            x as int / 256 == (x as int / 65536) * 256 + x as int / 256 % 256,
            x as int / 65536 == (x as int / 16777216) * 256 + x as int / 65536 % 256,
            x as int / 16777216 < 256;
}

/// A repeated byte string is its singleton repetition.
pub open spec fn repeat_byte(v: u8, n: nat) -> Seq<u8> {
    Seq::new(n, |i: int| v)
}

pub proof fn lemma_repeat_byte_len(v: u8, n: nat)
    ensures repeat_byte(v, n).len() == n,
{
    lemma_seq_new_len(n, |i: int| v);
}

// ---- L1 — oversized input ---------------------------------------------------

/// L1: `∀b, |b|>67,108,864: MP(b)=Err(FileTooLarge)`.
pub proof fn lemma_l1_oversized(b: Seq<u8>)
    requires
        b.len() > 67108864,
    ensures
        mp(b) == Err(MpError::FileTooLarge),
{
    reveal(mp);
}

/// L1's manifest instantiation `file-too-large`.
pub proof fn lemma_l1_file_too_large()
    ensures
        mp(repeat_byte(0u8, 67108865)) == Err(MpError::FileTooLarge),
        exec_err(MpError::FileTooLarge) == Error::FileTooLarge,
{
    lemma_repeat_byte_len(0u8, 67108865);
    reveal(exec_err);
    lemma_l1_oversized(repeat_byte(0u8, 67108865));
}

// ---- L2 — exact-limit bad magic --------------------------------------------

/// L2: `∀b, |b|=67,108,864 ∧ b[0:4]≠bytes("icns"): MP(b)=Err(BadMagic)`.
pub proof fn lemma_l2_exact_limit_bad_magic(b: Seq<u8>)
    requires
        b.len() == 67108864,
        8 <= b.len(),
        b.subrange(0, 4) != icns_magic(),
    ensures
        mp(b) == Err(MpError::BadMagic),
{
    assert(!(b.len() > 67108864));
    assert(!(b.len() < 8));
    reveal(mp);
}

/// L2's manifest instantiation `file-size-at-limit`: 67,108,864 zero bytes.
/// Its expected error is `BadMagic`: the size test is strict `>`.
pub proof fn lemma_l2_file_size_at_limit()
    ensures
        mp(repeat_byte(0u8, 67108864)) == Err(MpError::BadMagic),
        exec_err(MpError::BadMagic) == Error::BadMagic,
{
    lemma_repeat_byte_len(0u8, 67108864);
    reveal(icns_magic);
    assert(repeat_byte(0u8, 67108864).subrange(0, 4) != icns_magic()) by {
        assert(repeat_byte(0u8, 67108864)[0] == 0u8) by {
            lemma_seq_subrange_index(repeat_byte(0u8, 67108864), 0, 4, 0);
        }
        assert(icns_magic()[0] == 105u8);
    }
    reveal(exec_err);
    lemma_l2_exact_limit_bad_magic(repeat_byte(0u8, 67108864));
}

// ===========================================================================
// T7 support: every offset `walk` accepts names an in-range entry header, and
// the executable records built at those offsets satisfy `entry_inv` (T6).
// ===========================================================================

/// `be32u` at `off` equals any `u32` whose value is the four-byte big-endian sum.
pub proof fn lemma_be32u_eq(b: Seq<u8>, off: int, v: u32)
    requires
        0 <= off,
        off + 4 <= b.len(),
        v as int == (b[off] as int) * 16777216 + (b[off + 1] as int) * 65536
            + (b[off + 2] as int) * 256 + (b[off + 3] as int),
    ensures
        be32u(b, off) == v,
{
    reveal(be32u);
    assert((v as int) as u32 == v);
}

/// `be32u` of a four-byte subrange equals `be32u` at the subrange's start.
pub proof fn lemma_be32u_subrange(b: Seq<u8>, off: int, s: Seq<u8>)
    requires
        s == b.subrange(off, off + 4),
        0 <= off,
        off + 4 <= b.len(),
    ensures
        be32u(s, 0) == be32u(b, off),
{
    reveal(be32u);
    lemma_seq_subrange_index(b, off, off + 4, 0);
    lemma_seq_subrange_index(b, off, off + 4, 1);
    lemma_seq_subrange_index(b, off, off + 4, 2);
    lemma_seq_subrange_index(b, off, off + 4, 3);
}

/// A `u32` whose `int` value is `e` equals `e as u32`.
pub proof fn lemma_u32_of_int(v: u32, e: int)
    requires
        v as int == e,
        0 <= e <= 0xffff_ffff,
    ensures
        v == e as u32,
{
    assert((e as u32) as int == e) by (nonlinear_arith)
        requires 0 <= e <= 0xffff_ffff;
    assert(v as int == (e as u32) as int);
}

/// The `i`th offset of a `walk` result is a well-formed entry header at `L`.
pub proof fn lemma_walk_offset_ok(b: Seq<u8>, L: u64, off: u64, j: nat, os: Seq<u32>, i: int)
    requires
        walk(b, L, off, j) == Ok(os),
        0 <= i < os.len(),
        8 <= off as int,
        off <= L,
        L <= 0xffff_ffffu64,
    ensures
        8 <= os[i] as int,
        os[i] as int + 8 <= L as int,
        (be32u(b, os[i] as int + 4) as u64) >= 8,
        os[i] as int + (be32u(b, os[i] as int + 4) as int) <= L as int,
    decreases (L - off) as nat,
{
    reveal(walk);
    if off >= L {
        assert(walk(b, L, off, j) == Ok(Seq::<u32>::empty()));
        assert(false);
    } else if L - off < 8 {
        assert(false);
    } else {
        let q = be32u(b, off as int + 4) as u64;
        if q < 8 || q > L - off || j >= 4096 {
            assert(false);
        } else {
            match walk(b, L, (off + q) as u64, j + 1) {
                Ok(rest) => {
                    assert(walk(b, L, off, j) == Ok(cons(off as u32, rest)));
                    assert(os == cons(off as u32, rest));
                    if i == 0 {
                        assert(os[0] == off as u32) by { reveal(cons); }
                        assert(off as u32 as int == off as int) by (nonlinear_arith)
                            requires off <= L, L <= 0xffff_ffffu64, 0 <= off;
                        assert(be32u(b, off as int + 4) >= 8) by {
                            assert(be32u(b, off as int + 4) == q as u32);
                        }
                        assert(off as int + (be32u(b, off as int + 4) as int) <= L as int) by {
                            assert(be32u(b, off as int + 4) == q as u32);
                        }
                    } else {
                        lemma_walk_offset_ok(b, L, (off + q) as u64, j + 1, rest, i - 1);
                        assert(os[i] == rest[i - 1]) by { reveal(cons); }
                        assert(off as int + q >= 8) by (nonlinear_arith)
                            requires q >= 8;
                        assert((off + q) as u64 <= L) by (nonlinear_arith)
                            requires q <= L - off, off <= L;
                    }
                },
                Err(_) => { assert(false); },
            }
        }
    }
}

/// `walk` accepts at most `4096 − j` entries: the count cap holds.
pub proof fn lemma_walk_len(b: Seq<u8>, L: u64, off: u64, j: nat, os: Seq<u32>)
    requires
        j <= 4096,
        walk(b, L, off, j) == Ok(os),
    ensures
        os.len() + j <= 4096,
    decreases (L - off) as nat,
{
    reveal(walk);
    if off >= L {
        assert(os =~= Seq::<u32>::empty()) by {
            assert(walk(b, L, off, j) == Ok(Seq::<u32>::empty()));
        }
    } else if L - off < 8 {
        assert(false);
    } else {
        let q = be32u(b, off as int + 4) as u64;
        if q < 8 || q > L - off || j >= 4096 {
            assert(false);
        } else {
            match walk(b, L, (off + q) as u64, j + 1) {
                Ok(rest) => {
                    assert(os == cons(off as u32, rest));
                    assert(cons(off as u32, rest).len() == rest.len() + 1) by {
                        reveal(cons);
                        lemma_seq_new_len(rest.len() + 1,
                            |i: int| if i == 0 { off as u32 } else { rest[i - 1] });
                    }
                    lemma_walk_len(b, L, (off + q) as u64, j + 1, rest);
                    assert(rest.len() + (j + 1) <= 4096);
                    assert(os =~= cons(off as u32, rest)) by {
                        assert(walk(b, L, off, j) == Ok(cons(off as u32, rest)));
                    }
                    assert(os.len() == cons(off as u32, rest).len());
                    assert(cons(off as u32, rest).len() == rest.len() + 1) by {
                        reveal(cons);
                        lemma_seq_new_len(rest.len() + 1,
                            |i: int| if i == 0 { off as u32 } else { rest[i - 1] });
                    }
                    assert(os.len() + j <= 4096) by (nonlinear_arith)
                        requires rest.len() + (j + 1) <= 4096, os.len() == rest.len() + 1;
                },
                Err(_) => { assert(false); },
            }
        }
    }
}

/// Every entry `entry_at` records at an accepted offset satisfies `entry_inv`.
pub proof fn lemma_entry_inv_from_at(b: Seq<u8>, off: int, e: Entry)
    requires
        entry_eq_fields(e, entry_at(b, off)),
        0 <= off,
        off + 8 <= b.len(),
        (be32u(b, off + 4) as u64) >= 8,
        off + (be32u(b, off + 4) as int) <= b.len(),
        b.len() <= 0xffff_ffffu64,
    ensures
        entry_inv(b, e),
{
    reveal(entry_inv); reveal(entry_eq_fields); reveal(entry_at);
    reveal(entry_code); reveal(entry_payload);
    let q = be32u(b, off + 4) as int;
    assert(be32u(b, off + 4) as int == q);
    assert(hex_str(e.type_hex) == hex4(code4(b, off)));
    lemma_hex4_is_hex8(code4(b, off));
    assert(entry_code(e) == code4(b, off)) by {
        assert(entry_code(e) == code_of(hex4(code4(b, off))));
        lemma_code_of_hex4(code4(b, off));
    }
    assert(e.payload[0] == (off + 8) as u32);
    assert(e.payload[1] == (off + q) as u32);
    assert((off + 8) as u32 as int == off + 8) by (nonlinear_arith)
        requires 0 <= off, off + 8 <= b.len(), b.len() <= 0xffff_ffffu64;
    assert(off + q <= 0xffff_ffffu64) by (nonlinear_arith)
        requires 0 <= off, off + q <= b.len(), b.len() <= 0xffff_ffffu64;
    assert((off + q) as u32 as int == off + q);
    assert(8 <= e.payload[0]);
    assert(e.payload[0] <= e.payload[1]);
    assert(e.payload[1] <= b.len());
    assert(entry_payload(b, e) == b.subrange(off + 8, off + q));
    assert(entry_at(b, off).opaque_candidate == classify(
        code4(b, off), b.subrange(off + 8, off + q)).0);
    assert(entry_at(b, off).nominal_side == classify(
        code4(b, off), b.subrange(off + 8, off + q)).1);
}

/// `parses_to` lifted to the view of the result, so spec code can name `Ok(e@)`.
pub open spec fn parses_to_s(b: Seq<u8>, r: Result<Seq<Entry>, Error>) -> bool {
    match mp(b) {
        Ok(ms) => match r {
            Ok(es) => entries_eq_models(es, ms),
            Err(_) => false,
        },
        Err(m) => match r {
            Ok(_) => false,
            Err(e) => e == exec_err(m),
        },
    }
}

/// `parses_to` on an `Ok` result equals its lifted form.
pub proof fn lemma_parses_to_ok_view(b: Seq<u8>, v: Vec<Entry>)
    ensures
        parses_to(b, Ok(v)) == parses_to_s(b, Ok(v@)),
{
    reveal(parses_to); reveal(parses_to_s);
}

/// `parses_to` on an `Err` result equals its lifted form.
pub proof fn lemma_parses_to_err_view(b: Seq<u8>, e: Error)
    ensures
        parses_to(b, Err(e)) == parses_to_s(b, Err(e)),
{
    reveal(parses_to); reveal(parses_to_s);
}

/// T7 at `parse`'s success return: `Ok(es)` matches `mp`, and `parsed_inv` holds.
pub proof fn lemma_parses_to_ok(b: Seq<u8>, es: Seq<Entry>, os: Seq<u32>)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) == icns_magic(),
        (be32u(b, 4) as u64) == b.len() as u64,
        walk(b, b.len() as u64, 8, 0) == Ok(os),
        entries_match(b, es, os),
    ensures
        parses_to_s(b, Ok(es)),
        parsed_inv(b, es),
{
    reveal(parses_to_s); reveal(parsed_inv); reveal(entries_match);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
    assert(mp(b) == mp_at(b));
    assert(mp_at(b) == Ok(entries_at(b, os)));
    assert(mp(b) == Ok(entries_at(b, os)));
    lemma_entries_match_final(b, es, os);
    assert(entries_eq_models(es, entries_at(b, os)));
    assert(parses_to_s(b, Ok(es)));
    assert(es.len() == os.len());
    assert(b.len() <= MAX_FILE_LEN);
    lemma_walk_len(b, b.len() as u64, 8u64, 0, os);
    assert(es.len() <= MAX_ENTRIES) by { assert(os.len() + 0 <= 4096); }
    assert forall|i: int| 0 <= i < es.len() implies #[trigger] entry_inv(b, es[i]) by {
        lemma_entries_match_unpack(b, es, os, i);
        lemma_walk_offset_ok(b, b.len() as u64, 8u64, 0, os, i);
        lemma_entry_inv_from_at(b, os[i] as int, es[i]);
    }
}

/// T7 at an error return stated directly over `mp`.
pub proof fn lemma_parses_to_of_mp_err(b: Seq<u8>, m: MpError)
    requires mp(b) == Err(m),
    ensures parses_to(b, Err(exec_err(m))),
{
    reveal(parses_to);
}

/// T7 at a success return stated directly over `mp`.
pub proof fn lemma_parses_to_of_mp_ok(b: Seq<u8>, v: Vec<Entry>, ms: Seq<MpEntry>)
    requires
        mp(b) == Ok(ms),
        entries_eq_models(v@, ms),
    ensures
        parses_to(b, Ok(v)),
{
    reveal(parses_to);
}

/// T7 at `parse`'s error return: `walk` from 8 reports first error `e`.
pub proof fn lemma_parses_to_err(b: Seq<u8>, e: MpError)
    requires
        b.len() <= 67108864,
        8 <= b.len(),
        b.subrange(0, 4) == icns_magic(),
        (be32u(b, 4) as u64) == b.len() as u64,
        walk(b, b.len() as u64, 8, 0) == Err(e),
    ensures
        parses_to_s(b, Err(exec_err(e))),
{
    reveal(parses_to_s);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
    assert(mp(b) == Err(e));
}

// ===========================================================================
// T8 — `parse(b) = Ok(E) ⇔ P(b,E)`.
// ===========================================================================

/// `P(b, es)` in executable terms: `mp(b)` returns a model list that the
/// executable list `es` matches field for field, in order.
pub open spec fn p_exec(b: Seq<u8>, es: Seq<Entry>) -> bool {
    match mp(b) {
        Ok(ms) => entries_eq_models(es, ms),
        Err(_) => false,
    }
}

/// T8: for every byte sequence and every entry list, `P` holds exactly when
/// `parse` can return `Ok` with that list.
pub proof fn lemma_t8(b: Seq<u8>, es: Seq<Entry>)
    ensures
        p_exec(b, es) <==> parses_to_s(b, Ok(es)),
{
    reveal(p_exec); reveal(parses_to_s);
}

/// T8 in the direction used at a decoder call site: a successful `parse` gives
/// `P(b, E)` for the same `b` and the returned `E`.
pub proof fn lemma_parse_ok_gives_p(b: Seq<u8>, v: Vec<Entry>)
    requires parses_to(b, Ok(v)),
    ensures p_exec(b, v@),
{
    lemma_parses_to_ok_view(b, v);
    lemma_t8(b, v@);
}

/// If `parse` fails, no successful model run exists.
pub proof fn lemma_parse_err_gives_not_accepted(b: Seq<u8>, e: Error)
    requires parses_to_s(b, Err(e)),
    ensures !accepted(b),
{
    reveal(parses_to_s); reveal(accepted); reveal(p_exec);
    if accepted(b) {
        let ms = choose|ms: Seq<MpEntry>| #[trigger] mp(b) == Ok(ms);
        assert(false);
    }
}

/// T8 at the concrete decoder boundary: exact model provenance gives T6's
/// structural and classification invariant without an independent assumption.
pub proof fn lemma_p_exec_inv(b: Seq<u8>, es: Seq<Entry>)
    requires p_exec(b, es),
    ensures parsed_inv(b, es),
{
    reveal(p_exec);
    let ms = match mp(b) {
        Ok(ms) => ms,
        Err(_) => { assert(false); Seq::empty() },
    };
    assert(entries_eq_models(es, ms));
    reveal(mp);
    assert(b.len() <= 67108864);
    assert(b.len() >= 8);
    assert(b.subrange(0, 4) == icns_magic());
    assert((be32u(b, 4) as u64) == b.len() as u64);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
    let w = walk(b, b.len() as u64, 8, 0);
    match w {
        Ok(os) => {
            assert(mp(b) == Ok(entries_at(b, os)));
            assert(ms == entries_at(b, os));
            assert(es.len() == os.len());
            assert(entries_match(b, es, os)) by {
                reveal(entries_match);
                reveal(entries_eq_models);
                reveal(entries_at);
                assert forall|i: int| 0 <= i < es.len() implies
                    #[trigger] entry_eq_fields(es[i], entry_at(b, os[i] as int)) by {
                    lemma_seq_new_index(os.len(), |j: int| entry_at(b, os[j] as int), i);
                    assert(entry_eq_fields(es[i], ms[i]));
                    assert(ms[i] == entries_at(b, os)[i]);
                    assert(entries_at(b, os)[i] == entry_at(b, os[i] as int));
                }
            }
            lemma_parses_to_ok(b, es, os);
        },
        Err(_) => { assert(false); },
    }
}

/// The first `m` header offsets, extended by the next eight-byte header.
pub proof fn lemma_offs_push(m: nat)
    ensures offs(8, m + 1) == offs(8, m).push((8 + 8 * m) as u32),
{
    lemma_offs_len(8, m);
    lemma_offs_len(8, m + 1);
    assert forall|i: int| 0 <= i < m + 1 implies
        #[trigger] offs(8, m + 1)[i] == offs(8, m).push((8 + 8 * m) as u32)[i] by {
        lemma_offs_index(8, m + 1, i);
        if i < m {
            lemma_offs_index(8, m, i);
            lemma_seq_push_index_different(offs(8, m), (8 + 8 * m) as u32, i);
        } else {
            lemma_seq_push_index_same(offs(8, m), (8 + 8 * m) as u32, i);
        }
    }
    assert(offs(8, m + 1) =~= offs(8, m).push((8 + 8 * m) as u32));
}

/// Every complete header in the repeated prefix has length eight.
pub proof fn lemma_dot_q8(b: Seq<u8>, k: nat, i: int)
    requires
        0 <= i < k,
        8 + 8 * k <= b.len(),
        b.subrange(8, (8 + 8 * k) as int) == e_pow(k),
    ensures be32u(b, 8 + 8 * i + 4) == 8,
{
    lemma_dot_slice(b, k, i);
    let o = 8 + 8 * i;
    assert(b[o + 4] == 0u8) by { lemma_seq_subrange_index(b, o, o + 8, 4); }
    assert(b[o + 5] == 0u8) by { lemma_seq_subrange_index(b, o, o + 8, 5); }
    assert(b[o + 6] == 0u8) by { lemma_seq_subrange_index(b, o, o + 8, 6); }
    assert(b[o + 7] == 8u8) by { lemma_seq_subrange_index(b, o, o + 8, 7); }
    reveal(be32u);
}

/// Consume any prefix of at most 4,096 valid empty entries without inspecting
/// the next entry or the suffix. This holds even when the next header is bad.
pub proof fn lemma_walk_empty_prefix(b: Seq<u8>, k: nat, m: nat)
    requires
        m <= k,
        m <= 4096,
        8 + 8 * k <= b.len() <= 67108864,
        b.subrange(8, (8 + 8 * k) as int) == e_pow(k),
    ensures
        walk_pref(b, b.len() as u64, (8 + 8 * m) as u64, offs(8, m)),
    decreases m,
{
    if m == 0 {
        lemma_pref_init(b, b.len() as u64);
        assert(offs(8, 0) == Seq::<u32>::empty());
    } else {
        lemma_walk_empty_prefix(b, k, (m - 1) as nat);
        lemma_dot_q8(b, k, (m - 1) as int);
        lemma_offs_len(8, (m - 1) as nat);
        let o = (8 + 8 * (m - 1)) as u64;
        assert(o < b.len() as u64);
        assert(8 <= b.len() as u64 - o);
        assert(offs(8, (m - 1) as nat).len() < 4096);
        assert(be32u(b, o as int + 4) as u64 == 8);
        lemma_pref_step(b, b.len() as u64, o, offs(8, (m - 1) as nat));
        lemma_offs_push((m - 1) as nat);
        assert((o + 8) as u64 == (8 + 8 * m) as u64);
        assert(offs(8, (m - 1) as nat).push(o as u32) == offs(8, m));
    }
}

/// L4: every permitted repeated-entry count yields all four exact fields.
pub proof fn lemma_l4_repeated_entries(k: nat)
    requires k <= 4096,
    ensures mp(h_string(k, Seq::<u8>::empty())) == Ok(big_e(k)),
{
    let b = h_string(k, Seq::<u8>::empty());
    lemma_h_len(k, Seq::<u8>::empty());
    lemma_h_subranges(k, Seq::<u8>::empty());
    assert(8 + 8 * k <= 67108864);
    lemma_walk_empty_prefix(b, k, k);
    lemma_pref_done(b, b.len() as u64, offs(8, k));
    lemma_entries_big_e(b, k);
    lemma_be32u_of_bytes(b, 4, (8 + 8 * k) as u32);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
}

/// The compact `entry-count-exactly-4096` fixture is L4 at its bound.
pub proof fn lemma_l4_entry_count_exactly_4096()
    ensures mp(h_string(4096, Seq::<u8>::empty())) == Ok(big_e(4096)),
{
    lemma_l4_repeated_entries(4096);
}

/// L5: a valid 4,097th header takes precedence over every later suffix.
pub proof fn lemma_l5_count_before_suffix(s: Seq<u8>)
    requires s.len() <= 67108864 - 32784,
    ensures mp(h_string(4097, s)) == Err(MpError::TooManyEntries),
{
    let b = h_string(4097, s);
    lemma_h_len(4097, s);
    lemma_h_subranges(4097, s);
    assert(b.len() <= 67108864);
    lemma_walk_empty_prefix(b, 4097, 4096);
    lemma_dot_q8(b, 4097, 4096);
    lemma_offs_len(8, 4096);
    assert(be32u(b, 32780int) as u64 == 8);
    lemma_pref_error_tme(b, b.len() as u64, 32776, offs(8, 4096), 8);
    lemma_be32u_of_bytes(b, 4, (32784 + s.len()) as u32);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
}

/// Eight-byte suffix with the header of the next entry, including malformed q.
pub open spec fn suffix_entry(q: u32) -> Seq<u8> {
    seq![110u8, 111, 110, 101] + be32_bytes(q)
}

pub proof fn lemma_suffix_len(q: u32)
    ensures suffix_entry(q).len() == 8,
{
    lemma_be32_bytes_len(q);
    lemma_seq_add_len(seq![110u8, 111, 110, 101], be32_bytes(q));
}

/// The three compact L5 witnesses: no suffix, stray BAD, and a later
/// malformed entry that the count limit prevents the model from reading.
pub proof fn lemma_l5_vectors()
    ensures
        mp(h_string(4097, Seq::<u8>::empty())) == Err(MpError::TooManyEntries),
        mp(h_string(4097, seq![66u8, 65, 68])) == Err(MpError::TooManyEntries),
        mp(h_string(4097, suffix_entry(7))) == Err(MpError::TooManyEntries),
{
    lemma_l5_count_before_suffix(Seq::<u8>::empty());
    lemma_l5_count_before_suffix(seq![66u8, 65, 68]);
    lemma_suffix_len(7);
    lemma_l5_count_before_suffix(suffix_entry(7));
}

/// The manifest's B(q) with its declared length fixed at 32,784.
pub open spec fn b_q(q: u32) -> Seq<u8> {
    icns_magic() + be32_bytes(32784) + e_pow(4096) + suffix_entry(q)
}

pub proof fn lemma_b_q_h(q: u32)
    ensures b_q(q) == h_string(4096, suffix_entry(q)),
{
    lemma_suffix_len(q);
    reveal(b_q); reveal(h_string);
}

/// The four bytes of the 4,097th header encode q, independently of q's value.
pub proof fn lemma_h_suffix_q(q: u32)
    ensures be32u(h_string(4096, suffix_entry(q)), 32780) == q,
{
    let b = h_string(4096, suffix_entry(q));
    lemma_suffix_len(q);
    lemma_h_len(4096, suffix_entry(q));
    lemma_h_subranges(4096, suffix_entry(q));
    let s = suffix_entry(q);
    lemma_be32_bytes_len(q);
    assert(b.subrange(32776, 32784) == s);
    assert(b.subrange(32780, 32784) =~= be32_bytes(q)) by {
        assert forall|i: int| 0 <= i < 4 implies
            #[trigger] b.subrange(32780, 32784)[i] == be32_bytes(q)[i] by {
            lemma_seq_subrange_index(b, 32780, 32784, i);
            lemma_seq_subrange_index(b, 32776, 32784, i + 4);
            lemma_seq_add_index2(seq![110u8, 111, 110, 101], be32_bytes(q), i + 4);
            assert(s[i + 4] == be32_bytes(q)[i]);
        }
    }
    lemma_be32u_of_bytes(b, 32780, q);
}

/// L6: invalid length and out-of-bounds outrank the count limit at entry 4,097.
pub proof fn lemma_l6_invalid_before_count(q: u32)
    requires q < 8,
    ensures mp(b_q(q)) == Err(MpError::InvalidEntryLength),
{
    lemma_b_q_h(q);
    let b = h_string(4096, suffix_entry(q));
    lemma_suffix_len(q);
    lemma_h_len(4096, suffix_entry(q));
    lemma_h_subranges(4096, suffix_entry(q));
    lemma_walk_empty_prefix(b, 4096, 4096);
    lemma_h_suffix_q(q);
    lemma_pref_error_iel(b, b.len() as u64, 32776, offs(8, 4096), q as u64);
    lemma_be32u_of_bytes(b, 4, 32784);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
}

pub proof fn lemma_l6_oob_before_count(q: u32)
    requires q > 8,
    ensures mp(b_q(q)) == Err(MpError::EntryOutOfBounds),
{
    lemma_b_q_h(q);
    let b = h_string(4096, suffix_entry(q));
    lemma_suffix_len(q);
    lemma_h_len(4096, suffix_entry(q));
    lemma_h_subranges(4096, suffix_entry(q));
    lemma_walk_empty_prefix(b, 4096, 4096);
    lemma_h_suffix_q(q);
    lemma_pref_error_oob(b, b.len() as u64, 32776, offs(8, 4096), q as u64);
    lemma_be32u_of_bytes(b, 4, 32784);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
}

/// The same fixed-size B(q) has a valid last header when q=8.
pub proof fn lemma_l6_eight_before_count()
    ensures mp(b_q(8)) == Err(MpError::TooManyEntries),
{
    lemma_b_q_h(8);
    let b = h_string(4096, suffix_entry(8));
    lemma_suffix_len(8);
    lemma_h_len(4096, suffix_entry(8));
    lemma_h_subranges(4096, suffix_entry(8));
    lemma_walk_empty_prefix(b, 4096, 4096);
    lemma_h_suffix_q(8);
    lemma_offs_len(8, 4096);
    lemma_pref_error_tme(b, b.len() as u64, 32776, offs(8, 4096), 8);
    lemma_be32u_of_bytes(b, 4, 32784);
    lemma_mp_eq_mp_at(b);
    lemma_mp_at_walk(b);
}

/// Both L6 malformed-length vectors and the q=8 boundary from L5.
pub proof fn lemma_l6_vectors()
    ensures
        mp(b_q(7)) == Err(MpError::InvalidEntryLength),
        mp(b_q(0xffff_ffffu32)) == Err(MpError::EntryOutOfBounds),
        mp(b_q(8)) == Err(MpError::TooManyEntries),
{
    lemma_l6_invalid_before_count(7);
    lemma_l6_oob_before_count(0xffff_ffffu32);
    lemma_l6_eight_before_count();
}

// ===========================================================================
// L3 — the large 128x128 exact mask (`mask-t8mk`).
// ===========================================================================

/// `b = bytes("icns") · BE32(16,400) · bytes("t8mk") · BE32(16,392) · a`, the
/// manifest's `t8mk` container with a `16,384`-byte mask `a`.
pub open spec fn l3_string(a: Seq<u8>) -> Seq<u8> {
    icns_magic() + be32_bytes(16400) + seq![116u8, 56, 109, 107] + be32_bytes(16392) + a
}

/// The single model entry L3 expects: `("74386d6b", [16, 16,400], false, None)`.
pub open spec fn l3_entry() -> MpEntry {
    MpEntry {
        type_hex: seq!['7', '4', '3', '8', '6', 'd', '6', 'b'],
        payload: (16u32, 16400u32),
        opaque_candidate: false,
        nominal_side: None,
    }
}

/// L3: for every 16,384-byte `a`, `MP(l3_string(a)) = Ok([l3_entry])` and
/// `MA(l3_string(a), [l3_entry], 0) = Ok(128,128,a)`.
pub proof fn lemma_l3_large_mask(a: Seq<u8>)
    requires a.len() == 16384,
    ensures
        mp(l3_string(a)) == Ok(seq![l3_entry()]),
        ma_ms(l3_string(a), seq![l3_entry()], 0) == Ok((128u32, 128u32, a)),
{
    reveal(l3_string);
    reveal(l3_entry);
    reveal(ma_ms);
    reveal(icns_magic);
    lemma_be32_bytes_len(16400);
    lemma_be32_bytes_len(16392);
    lemma_seq_add_len(icns_magic(), be32_bytes(16400) + seq![116u8, 56, 109, 107] + be32_bytes(
        16392,
    ) + a);
    lemma_seq_add_len(be32_bytes(16400), seq![116u8, 56, 109, 107] + be32_bytes(16392) + a);
    lemma_seq_add_len(seq![116u8, 56, 109, 107], be32_bytes(16392) + a);
    lemma_seq_add_len(be32_bytes(16392), a);
    assert(l3_string(a).len() == 16400);
    // Subranges: magic, declared length, type, entry length, payload.
    assert(l3_string(a).subrange(0, 4) =~= icns_magic()) by {
        lemma_seq_subrange_index(l3_string(a), 0, 4, 0);
        lemma_seq_subrange_index(l3_string(a), 0, 4, 1);
        lemma_seq_subrange_index(l3_string(a), 0, 4, 2);
        lemma_seq_subrange_index(l3_string(a), 0, 4, 3);
    }
    lemma_be32u_subrange(l3_string(a), 4, l3_string(a).subrange(4, 8));
    assert(be32u(l3_string(a), 4) == 16400u32) by {
        lemma_be32_bytes_index(16400u32);
        assert(l3_string(a).subrange(4, 8) =~= be32_bytes(16400)) by {
            assert forall|i: int| 0 <= i < 4 implies (#[trigger] l3_string(a).subrange(4, 8)[i])
                == be32_bytes(16400)[i] by {
                lemma_seq_subrange_index(l3_string(a), 4, 8, i);
            }
        }
    }
    // The first entry's header is at offset 8.
    assert(l3_string(a).subrange(8, 12) =~= seq![116u8, 56, 109, 107]) by {
        assert forall|i: int| 0 <= i < 4 implies (#[trigger] l3_string(a).subrange(8, 12)[i])
            == seq![116u8, 56, 109, 107][i] by {
            lemma_seq_subrange_index(l3_string(a), 8, 12, i);
        }
    }
    lemma_be32u_subrange(l3_string(a), 12, l3_string(a).subrange(12, 16));
    assert(be32u(l3_string(a), 12) == 16392u32) by {
        lemma_be32_bytes_index(16392u32);
        assert(l3_string(a).subrange(12, 16) =~= be32_bytes(16392)) by {
            assert forall|i: int| 0 <= i < 4 implies (#[trigger] l3_string(a).subrange(12, 16)[i])
                == be32_bytes(16392)[i] by {
                lemma_seq_subrange_index(l3_string(a), 12, 16, i);
            }
        }
    }
    assert(l3_string(a).subrange(16, 16400) == a) by {
        assert forall|i: int| 0 <= i < 16384 implies (#[trigger] l3_string(a).subrange(16, 16400)[i])
            == a[i] by {
            lemma_seq_subrange_index(l3_string(a), 16, 16400, i);
        }
        lemma_seq_subrange_len(l3_string(a), 16, 16400);
    }
    // `scan` accepts exactly one entry.
    reveal(scan);
    reveal(entry_at);
    reveal(hex4);
    assert(code4(l3_string(a), 8) == [116u8, 56, 109, 107]) by {
        reveal(code4);
    }
    assert(entry_at(l3_string(a), 8) == l3_entry()) by {
        reveal(l3_entry);
        reveal(entry_at);
        reveal(code4);
        reveal(hex4);
        reveal(hex_digit);
        reveal(classify);
        reveal(legacy_side);
        reveal(be32u);
    }
    assert(scan(l3_string(a), 16400, 8, Seq::empty()) == Ok(seq![l3_entry()])) by {
        reveal(scan);
        assert(be32u(l3_string(a), 12) == 16392u32) by {
            assert(l3_string(a).subrange(12, 16) =~= be32_bytes(16392));
        }
        assert(8 + 16392 == 16400);
        assert(scan(l3_string(a), 16400, 16400, seq![l3_entry()]) == Ok(seq![l3_entry()])) by {
            reveal(scan);
        }
    }
    reveal(mp);
    assert(mp(l3_string(a)) == Ok(seq![l3_entry()]));
    // `MA` on that model entry returns the payload verbatim.
    assert((l3_string(a).subrange(l3_entry().payload.0 as int, l3_entry().payload.1 as int)) == a) by {
        reveal(l3_entry);
    }
    assert(code_of(l3_entry().type_hex) == [116u8, 56, 109, 107]) by {
        reveal(l3_entry);
        reveal(code_of);
        reveal(hex_val);
    }
    assert(legacy_side(code_of(l3_entry().type_hex)) == Some((false, 128u32))) by {
        reveal(legacy_side);
        reveal(is_code);
        reveal(code_of);
        reveal(hex_val);
        reveal(l3_entry);
    }
}

/// L3's manifest instantiation `mask-t8mk`: `a` repeats `7f` 16,384 times.
pub proof fn lemma_l3_mask_t8mk()
    ensures
        mp(l3_string(repeat_byte(0x7fu8, 16384))) == Ok(seq![l3_entry()]),
        ma_ms(l3_string(repeat_byte(0x7fu8, 16384)), seq![l3_entry()], 0) == Ok((
            128u32,
            128u32,
            repeat_byte(0x7fu8, 16384),
        )),
{
    lemma_repeat_byte_len(0x7fu8, 16384);
    lemma_l3_large_mask(repeat_byte(0x7fu8, 16384));
}

/// Exact equality of R1 record contents (a String's logical contents are its view).
pub open spec fn entries_same_fields(a: Seq<Entry>, b: Seq<Entry>) -> bool {
    &&& a.len() == b.len()
    &&& forall|i: int| 0 <= i < a.len() ==> {
        &&& hex_str(a[i].type_hex) == hex_str(b[i].type_hex)
        &&& a[i].payload[0] == b[i].payload[0]
        &&& a[i].payload[1] == b[i].payload[1]
        &&& a[i].opaque_candidate == b[i].opaque_candidate
        &&& a[i].nominal_side == b[i].nominal_side
    }
}

/// A candidate E is the actual parse result, compared by the four R1 fields.
pub open spec fn parse_returned(r: Result<Vec<Entry>, Error>, es: Seq<Entry>) -> bool {
    match r {
        Ok(v) => entries_same_fields(v@, es),
        Err(_) => false,
    }
}

pub proof fn lemma_exact_entry_equivalence(a: Seq<Entry>, ms: Seq<MpEntry>, es: Seq<Entry>)
    requires entries_eq_models(a, ms),
    ensures entries_same_fields(a, es) <==> entries_eq_models(es, ms),
{
    reveal(entries_eq_models);
    reveal(entries_same_fields);
    reveal(entry_eq_fields);
    if entries_same_fields(a, es) {
        assert forall|i: int| 0 <= i < es.len() implies
            #[trigger] entry_eq_fields(es[i], ms[i]) by {
            assert(entry_eq_fields(a[i], ms[i]));
            assert(hex_str(a[i].type_hex) == hex_str(es[i].type_hex));
            assert(a[i].payload[0] == es[i].payload[0]);
            assert(a[i].payload[1] == es[i].payload[1]);
            assert(a[i].opaque_candidate == es[i].opaque_candidate);
            assert(a[i].nominal_side == es[i].nominal_side);
        }
    }
    if entries_eq_models(es, ms) {
        assert forall|i: int| 0 <= i < a.len() implies {
            &&& hex_str(a[i].type_hex) == hex_str(es[i].type_hex)
            &&& a[i].payload[0] == es[i].payload[0]
            &&& a[i].payload[1] == es[i].payload[1]
            &&& a[i].opaque_candidate == es[i].opaque_candidate
            &&& a[i].nominal_side == es[i].nominal_side
        } by {
            assert(entry_eq_fields(a[i], ms[i]));
            assert(entry_eq_fields(es[i], ms[i]));
        }
    }
}

// ===========================================================================
// M4a — the reference models MR (RLE), MC (decode_rgb) and MA (decode_mask).
//
// Written independently of the executable code: `mr`, `mc`, `ma` are recursive
// definitions over the mathematical bytes with the exact R10-R13/R15 error
// precedence. The executables carry postconditions phrased against them.
// ===========================================================================

// ---- R12 packet helpers ---------------------------------------------------

/// R12's run count: `control+1` below 128, `control-125` otherwise.
pub open spec fn rle_count(control: u8) -> int {
    if control < 128 {
        control as int + 1
    } else {
        control as int - 125
    }
}

/// The result of writing `vs` into `s` at `off` (length preserved).
pub open spec fn write_seq<T>(s: Seq<T>, off: int, vs: Seq<T>) -> Seq<T> {
    Seq::new(s.len(), |i: int| if off <= i < off + vs.len() {
        vs[i - off]
    } else {
        s[i]
    })
}

pub proof fn lemma_write_seq_len<T>(s: Seq<T>, off: int, vs: Seq<T>)
    ensures
        write_seq(s, off, vs).len() == s.len(),
{
    reveal(write_seq);
    lemma_seq_new_len(s.len(), |i: int| if off <= i < off + vs.len() {
        vs[i - off]
    } else {
        s[i]
    });
}

/// Writing `vs` at `off` extends the prefix `s[0..off]` to
/// `s[0..off] + vs`, provided `off + vs.len() <= s.len()`.
pub proof fn lemma_write_seq_prefix<T>(s: Seq<T>, off: int, vs: Seq<T>)
    requires
        0 <= off,
        off + vs.len() <= s.len(),
    ensures
        write_seq(s, off, vs).subrange(0, off + vs.len()) == s.subrange(0, off) + vs,
{
    reveal(write_seq);
    lemma_write_seq_len(s, off, vs);
    lemma_seq_subrange_len(s, 0, off);
    lemma_seq_add_len(s.subrange(0, off), vs);
    assert forall|k: int| 0 <= k < off + vs.len() implies (#[trigger]write_seq(s, off, vs).subrange(
        0,
        off + vs.len(),
    )[k]) == (s.subrange(0, off) + vs)[k] by {
        lemma_seq_subrange_index(write_seq(s, off, vs), 0, off + vs.len(), k);
        reveal(write_seq);
        lemma_seq_new_index(s.len(), |i: int| if off <= i < off + vs.len() {
            vs[i - off]
        } else {
            s[i]
        }, k);
        if k < off {
            lemma_seq_subrange_index(s, 0, off, k);
            lemma_seq_add_index1(s.subrange(0, off), vs, k);
        } else {
            lemma_seq_add_index2(s.subrange(0, off), vs, k);
        }
    }
}

/// Writing `X.push(x)` equals writing `X` then `x` at the next position.
pub proof fn lemma_write_seq_push<T>(s: Seq<T>, off: int, xseq: Seq<T>, x: T)
    requires
        0 <= off,
        off + xseq.len() < s.len(),
    ensures
        write_seq(s, off, xseq.push(x)) == write_seq(s, off, xseq).update(off + xseq.len() as int, x),
{
    reveal(write_seq);
    let a = write_seq(s, off, xseq.push(x));
    let b = write_seq(s, off, xseq).update(off + xseq.len() as int, x);
    lemma_write_seq_len(s, off, xseq.push(x));
    vstd::seq::lemma_seq_push_len(xseq, x);
    lemma_write_seq_len(s, off, xseq);
    vstd::seq::lemma_seq_update_len(write_seq(s, off, xseq), off + xseq.len() as int, x);
    assert(a =~= b) by {
        assert forall|k: int| 0 <= k < a.len() implies (#[trigger]a[k]) == b[k] by {
            reveal(write_seq);
            lemma_seq_new_index(s.len(), |i: int| if off <= i < off + xseq.push(x).len() {
                xseq.push(x)[i - off]
            } else {
                s[i]
            }, k);
            if off <= k < off + xseq.len() {
                assert(a[k] == xseq[k - off]);
                vstd::seq::lemma_seq_update_different(write_seq(s, off, xseq), k, off
                    + xseq.len(), x);
                lemma_seq_new_index(s.len(), |i: int| if off <= i < off + xseq.len() {
                    xseq[i - off]
                } else {
                    s[i]
                }, k);
            } else if k == off + xseq.len() {
                assert(a[k] == xseq.push(x)[xseq.len() as int]);
                vstd::seq::lemma_seq_push_index_same(xseq, x, xseq.len() as int);
                vstd::seq::lemma_seq_update_same(write_seq(s, off, xseq), off + xseq.len() as int, x);
            } else {
                assert(!(off <= k < off + xseq.push(x).len()));
                assert(a[k] == s[k]);
                assert(!(off <= k < off + xseq.len()));
                vstd::seq::lemma_seq_update_different(write_seq(s, off, xseq), k, off
                    + xseq.len(), x);
                assert(write_seq(s, off, xseq)[k] == s[k]);
            }
        }
    }
}

/// `rle_bytes` grows one byte at a time.
pub proof fn lemma_rle_bytes_push(s: Seq<u8>, start: int, j: int, literal: bool)
    requires
        0 <= j,
        0 <= start,
        literal ==> start + j + 1 <= s.len(),
        !literal ==> start < s.len(),
    ensures
        rle_bytes(s, start, j + 1, literal) == rle_bytes(s, start, j, literal).push(rle_bytes(
            s,
            start,
            j + 1,
            literal,
        )[j]),
        rle_bytes(s, start, j + 1, literal)[j] == (if literal {
            s[start + j]
        } else {
            s[start]
        }),
{
    reveal(rle_bytes);
    if literal {
        assert(rle_bytes(s, start, j + 1, literal) == s.subrange(start, start + j + 1));
        assert(s.subrange(start, start + j + 1) == s.subrange(start, start + j).push(s[start + j])) by {
            lemma_seq_subrange_len(s, start, start + j);
            assert forall|k: int|
                0 <= k < s.subrange(start, start + j + 1).len() implies (#[trigger]s.subrange(
                start,
                start + j + 1,
            )[k]) == s.subrange(start, start + j).push(s[start + j])[k] by {
                lemma_seq_subrange_index(s, start, start + j + 1, k);
                if k < j {
                    lemma_seq_subrange_index(s, start, start + j, k);
                    vstd::seq::lemma_seq_push_index_different(s.subrange(start, start + j), s[start
                        + j], k);
                } else {
                    vstd::seq::lemma_seq_push_index_same(s.subrange(start, start + j), s[start + j], k);
                }
            }
        }
        assert(s.subrange(start, start + j + 1)[j] == s[start + j]) by {
            lemma_seq_subrange_index(s, start, start + j + 1, j);
        }
    } else {
        assert(rle_bytes(s, start, j + 1, literal) == repeat_byte(s[start], (j + 1) as nat));
        assert(rle_bytes(s, start, j, literal) == repeat_byte(s[start], j as nat));
        assert(repeat_byte(s[start], (j + 1) as nat) == repeat_byte(s[start], j as nat).push(s[start])) by {
            reveal(repeat_byte);
            lemma_seq_new_len((j + 1) as nat, |i: int| s[start]);
            lemma_seq_new_len(j as nat, |i: int| s[start]);
            assert forall|k: int|
                0 <= k < repeat_byte(s[start], (j + 1) as nat).len() implies (#[trigger]repeat_byte(
                s[start],
                (j + 1) as nat,
            )[k]) == repeat_byte(s[start], j as nat).push(s[start])[k] by {
                reveal(repeat_byte);
                lemma_seq_new_index((j + 1) as nat, |i: int| s[start], k);
                if k < j {
                    lemma_seq_new_index(j as nat, |i: int| s[start], k);
                    vstd::seq::lemma_seq_push_index_different(repeat_byte(s[start], j as nat), s[start], k);
                } else {
                    vstd::seq::lemma_seq_push_index_same(repeat_byte(s[start], j as nat), s[start], k);
                }
            }
        }
        assert(repeat_byte(s[start], (j + 1) as nat)[j] == s[start]) by {
            reveal(repeat_byte);
            lemma_seq_new_index((j + 1) as nat, |i: int| s[start], j);
        }
    }
}

/// Composing two writes at adjacent offsets equals one write of the pushed
/// sequence.
pub proof fn lemma_write_seq_step<T>(s: Seq<T>, off: int, vs: Seq<T>, v: T)
    requires
        0 <= off,
        off + vs.len() < s.len(),
    ensures
        write_seq(write_seq(s, off, vs), off + vs.len(), seq![v]) == write_seq(s, off, vs.push(v)),
{
    lemma_write_seq_push(s, off, vs, v);
    reveal(write_seq);
    assert(write_seq(s, off, vs).update(off + vs.len() as int, v) == write_seq(
        write_seq(s, off, vs),
        off + vs.len(),
        seq![v],
    )) by {
        // write_seq with a singleton is exactly an update at that index
        assert forall|k: int|
            0 <= k < write_seq(s, off, vs).len() implies (#[trigger]write_seq(
            write_seq(s, off, vs),
            off + vs.len(),
            seq![v],
        )[k]) == write_seq(s, off, vs).update(off + vs.len() as int, v)[k] by {
            reveal(write_seq);
            lemma_seq_new_index(write_seq(s, off, vs).len(), |i: int|
                if off + vs.len() <= i < off + vs.len() + seq![v].len() {
                    seq![v][i - (off + vs.len())]
                } else {
                    write_seq(s, off, vs)[i]
                }, k);
            if k == off + vs.len() {
                vstd::seq::lemma_seq_update_same(write_seq(s, off, vs), off + vs.len() as int, v);
                assert(seq![v][0] == v);
            } else {
                vstd::seq::lemma_seq_update_different(write_seq(s, off, vs), k, off + vs.len() as int, v);
            }
        }
    }
}

/// `rle_pref` survives a write into channel `ci` beyond the produced prefix.
pub proof fn lemma_rle_pref_write(
    s: Seq<u8>,
    n: int,
    ci: int,
    produced: int,
    cur: int,
    channels: Seq<Vec<u8>>,
    channels2: Seq<Vec<u8>>,
    acc: Seq<Seq<u8>>,
)
    requires
        rle_pref(s, n, ci, produced, cur, channels, acc),
        channels.len() == 3,
        channels2.len() == 3,
        ci < 3,
        0 <= produced,
        forall|i: int| 0 <= i < 3 ==> (#[trigger] channels[i]@).len() == n,
        forall|i: int| 0 <= i < 3 ==> (#[trigger] channels2[i]@).len() == n,
        channels2[ci]@.subrange(0, produced) == channels[ci]@.subrange(0, produced),
        forall|i: int| 0 <= i < 3 && i != ci ==> (#[trigger] channels2[i]@) == channels[i]@,
    ensures
        rle_pref(s, n, ci, produced, cur, channels2, acc),
{
    reveal(rle_pref);
    reveal(chans_prefix_view);
    assert(chans_prefix_view(channels2, ci) == chans_prefix_view(channels, ci)) by {
        lemma_seq_new_len(ci as nat, |i: int| channels2[i]@);
        lemma_seq_new_len(ci as nat, |i: int| channels[i]@);
        assert forall|i: int| 0 <= i < ci implies (#[trigger] chans_prefix_view(channels2, ci)[i])
            == chans_prefix_view(channels, ci)[i] by {
            reveal(chans_prefix_view);
            lemma_seq_new_index(ci as nat, |j: int| channels2[j]@, i);
            lemma_seq_new_index(ci as nat, |j: int| channels[j]@, i);
        }
    }
    assert(channels2[ci]@.subrange(0, produced) == channels[ci]@.subrange(0, produced));
}

/// The channel loop's missing-control error.
pub proof fn lemma_rle_chan_from_trunc(s: Seq<u8>, n: int, p: int, cur: int)
    requires
        p < n,
        cur >= s.len(),
    ensures
        rle_chan_from(s, n, p, cur) == Err(Error::RleTruncated),
{
    reveal(rle_chan_from);
}

/// The channel loop's overrun error (checked before the availability test).
pub proof fn lemma_rle_chan_from_overrun(s: Seq<u8>, n: int, p: int, cur: int)
    requires
        p < n,
        cur < s.len(),
        rle_count(s[cur]) > n - p,
    ensures
        rle_chan_from(s, n, p, cur) == Err(Error::RleOverrun),
{
    reveal(rle_chan_from);
}

/// The channel loop's missing-data error.
pub proof fn lemma_rle_chan_from_need(s: Seq<u8>, n: int, p: int, cur: int)
    requires
        p < n,
        cur < s.len(),
        rle_count(s[cur]) <= n - p,
        cur + 1 + rle_needed(s[cur]) > s.len(),
    ensures
        rle_chan_from(s, n, p, cur) == Err(Error::RleTruncated),
{
    reveal(rle_chan_from);
}

/// `rle_pref` propagates a channel loop error to `rle_run`.
pub proof fn lemma_rle_pref_err_run(
    s: Seq<u8>,
    n: int,
    ci: int,
    produced: int,
    cur: int,
    channels: Seq<Vec<u8>>,
    acc: Seq<Seq<u8>>,
    e: Error,
)
    requires
        rle_pref(s, n, ci, produced, cur, channels, acc),
        ci < 3,
        produced < n,
        channels[ci]@.subrange(0, produced).len() == produced,
        rle_chan_from(s, n, produced, cur) == Err(e),
    ensures
        rle_run(s, n) == Err(e),
{
    reveal(rle_pref);
    reveal(rle_rest);
}

/// A control below 128 is a literal run (R12).
pub open spec fn rle_literal(control: u8) -> bool {
    control < 128
}

/// The payload bytes the packet needs after its control byte.
pub open spec fn rle_needed(control: u8) -> int {
    if control < 128 {
        rle_count(control)
    } else {
        1
    }
}

/// The `count` channel bytes of one RLE packet whose data byte is at `start`.
pub open spec fn rle_bytes(s: Seq<u8>, start: int, count: int, literal: bool) -> Seq<u8> {
    if literal {
        s.subrange(start, start + count)
    } else {
        repeat_byte(s[start], count as nat)
    }
}

pub proof fn lemma_rle_count_bounds(control: u8)
    ensures
        1 <= rle_count(control) <= 130,
        1 <= rle_needed(control) <= rle_count(control),
{
    reveal(rle_count);
    reveal(rle_needed);
}

pub proof fn lemma_rle_bytes_len(s: Seq<u8>, start: int, count: int, literal: bool)
    requires
        0 <= count,
        0 <= start,
        literal ==> start + count <= s.len(),
        !literal ==> 0 <= start < s.len(),
    ensures
        rle_bytes(s, start, count, literal).len() == count,
{
    reveal(rle_bytes);
    if literal {
        lemma_seq_subrange_len(s, start, start + count);
    } else {
        assert(0 <= start < s.len());
        lemma_repeat_byte_len(s[start], count as nat);
    }
}

/// R12's single channel loop: produce bytes `p..n` of one channel, starting at
/// cursor `cur`, and return the produced suffix with the cursor past it.
pub open spec fn rle_chan_from(s: Seq<u8>, n: int, p: int, cur: int) -> Result<
    (Seq<u8>, int),
    Error,
>
    decreases (n - p) as nat,
{
    if p >= n {
        Ok((Seq::empty(), cur))
    } else if cur >= s.len() {
        Err(Error::RleTruncated)
    } else {
        let control = s[cur];
        if rle_count(control) > n - p {
            Err(Error::RleOverrun)
        } else if cur + 1 + rle_needed(control) > s.len() {
            Err(Error::RleTruncated)
        } else {
            match rle_chan_from(s, n, p + rle_count(control), cur + 1 + rle_needed(control)) {
                Err(e) => Err(e),
                Ok((rest, c)) => Ok((
                    rle_bytes(s, cur + 1, rle_count(control), rle_literal(control)) + rest,
                    c,
                )),
            }
        }
    }
}

/// `MR`'s remaining channel work from channel `ci`, with `prefix` bytes of
/// channel `ci` already produced: the completed channels `ci..3` (channel `ci`
/// as `prefix` followed by its suffix) and the final cursor.
pub open spec fn rle_rest(s: Seq<u8>, n: int, ci: int, prefix: Seq<u8>, cur: int) -> Result<
    (Seq<Seq<u8>>, int),
    Error,
>
    decreases (3 - ci) as nat,
{
    if ci >= 3 {
        Ok((Seq::empty(), cur))
    } else {
        match rle_chan_from(s, n, prefix.len() as int, cur) {
            Err(e) => Err(e),
            Ok((sfx, c)) => match rle_rest(s, n, ci + 1, Seq::empty(), c) {
                Err(e) => Err(e),
                Ok((rest, cf)) => Ok((cons(prefix + sfx, rest), cf)),
            },
        }
    }
}

/// `MR`'s channel walk from a fresh start, without the trailing test.
pub open spec fn rle_run(s: Seq<u8>, n: int) -> Result<(Seq<Seq<u8>>, int), Error> {
    rle_rest(s, n, 0, Seq::empty(), 0)
}

/// `MR(s,n)`: the complete R12 decode — every error the first encountered by
/// this order, and `RleTrailingBytes` when the cursor does not end at `|s|`.
pub open spec fn mr(s: Seq<u8>, n: int) -> Result<(Seq<u8>, Seq<u8>, Seq<u8>), Error> {
    match rle_run(s, n) {
        Err(e) => Err(e),
        Ok((chs, c)) => if c == s.len() && chs.len() == 3 {
            Ok((chs[0], chs[1], chs[2]))
        } else {
            Err(Error::RleTrailingBytes)
        },
    }
}

// ---- MR lemmas ------------------------------------------------------------

/// `cons(x,s) == seq![x] + s`, shifted past an accumulator.
pub proof fn lemma_push_cons_shift<T>(acc: Seq<T>, x: T, rest: Seq<T>)
    ensures
        acc.push(x) + rest == acc + cons(x, rest),
{
    lemma_cons_is_singleton_add(x, rest);
    assert(acc.push(x) == acc + seq![x]) by {
        assert(acc.push(x) =~= acc + seq![x]);
    }
    assert(acc + cons(x, rest) == acc + (seq![x] + rest));
    lemma_concat_associative(acc, seq![x], rest);
    assert(acc + (seq![x] + rest) == (acc + seq![x]) + rest);
}

/// The channel loop's suffix has exactly `n-p` bytes (when `p <= n`).
pub proof fn lemma_rle_chan_from_len(s: Seq<u8>, n: int, p: int, cur: int)
    requires
        p <= n,
        0 <= cur,
    ensures
        match rle_chan_from(s, n, p, cur) {
            Ok((sfx, c)) => sfx.len() == n - p && c >= 0,
            Err(_) => true,
        },
    decreases (n - p) as nat,
{
    reveal(rle_chan_from);
    if p >= n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur];
        if rle_count(control) > n - p {
        } else if cur + 1 + rle_needed(control) > s.len() {
        } else {
            lemma_rle_count_bounds(control);
            assert(p + rle_count(control) <= n);
            assert(0 <= cur + 1 + rle_needed(control));
            lemma_rle_chan_from_len(s, n, p + rle_count(control), cur + 1 + rle_needed(control));
            reveal(rle_needed); reveal(rle_literal);
            assert(rle_bytes(s, cur + 1, rle_count(control), rle_literal(control)).len()
                == rle_count(control)) by {
                lemma_rle_bytes_len(s, cur + 1, rle_count(control), rle_literal(control));
            }
            match rle_chan_from(s, n, p + rle_count(control), cur + 1 + rle_needed(control)) {
                Ok((rest, c)) => {
                    assert(rest.len() == n - (p + rle_count(control)));
                },
                Err(_) => {},
            }
        }
    }
}

/// One packet step of the channel loop: the loop at `p` is the loop at
/// `p+count` with this packet's bytes prepended to its suffix.
pub proof fn lemma_rle_chan_from_step(s: Seq<u8>, n: int, p: int, cur: int)
    requires
        p < n,
        0 <= cur < s.len(),
        rle_count(s[cur]) <= n - p,
        cur + 1 + rle_needed(s[cur]) <= s.len(),
    ensures
        rle_chan_from(s, n, p, cur) == match rle_chan_from(
            s,
            n,
            p + rle_count(s[cur]),
            cur + 1 + rle_needed(s[cur]),
        ) {
            Err(e) => Err(e),
            Ok((rest, c)) => Ok((
                rle_bytes(s, cur + 1, rle_count(s[cur]), rle_literal(s[cur])) + rest,
                c,
            )),
        },
{
    reveal(rle_chan_from);
}

/// One packet step of the remaining-work walk: growing the prefix with the
/// packet's bytes leaves the result unchanged.
pub proof fn lemma_rle_rest_step(s: Seq<u8>, n: int, ci: int, prefix: Seq<u8>, cur: int)
    requires
        ci < 3,
        prefix.len() < n,
        0 <= cur < s.len(),
        rle_count(s[cur]) <= n - prefix.len(),
        cur + 1 + rle_needed(s[cur]) <= s.len(),
    ensures
        rle_rest(s, n, ci, prefix, cur) == rle_rest(
            s,
            n,
            ci,
            prefix + rle_bytes(s, cur + 1, rle_count(s[cur]), rle_literal(s[cur])),
            cur + 1 + rle_needed(s[cur]),
        ),
{
    reveal(rle_rest);
    lemma_rle_chan_from_step(s, n, prefix.len() as int, cur);
    let rb = rle_bytes(s, cur + 1, rle_count(s[cur]), rle_literal(s[cur]));
    let c2 = cur + 1 + rle_needed(s[cur]);
    let count = rle_count(s[cur]);
    assert((prefix + rb).len() == prefix.len() + count);
    match rle_chan_from(s, n, prefix.len() as int + count, c2) {
        Err(e) => {
            assert(rle_chan_from(s, n, prefix.len() as int, cur) == Err(e));
            assert(rle_chan_from(s, n, (prefix + rb).len() as int, c2) == Err(e));
        },
        Ok((rest, c)) => {
            assert(rle_chan_from(s, n, prefix.len() as int, cur) == Ok((rb + rest, c)));
            assert(rle_chan_from(s, n, (prefix + rb).len() as int, c2) == Ok((rest, c)));
            assert(prefix + (rb + rest) == (prefix + rb) + rest);
        },
    }
}

/// Completing a channel: once `prefix` has all `n` bytes, the loop's base case
/// fires and channel `ci+1` begins.
pub proof fn lemma_rle_rest_done_chan(s: Seq<u8>, n: int, ci: int, prefix: Seq<u8>, cur: int)
    requires
        ci < 3,
        prefix.len() == n,
    ensures
        match rle_rest(s, n, ci + 1, Seq::empty(), cur) {
            Err(e) => rle_rest(s, n, ci, prefix, cur) == Err(e),
            Ok((rest, cf)) => rle_rest(s, n, ci, prefix, cur) == Ok((cons(prefix, rest), cf)),
        },
{
    reveal(rle_rest);
    reveal(rle_chan_from);
    assert(rle_chan_from(s, n, n, cur) == Ok((Seq::<u8>::empty(), cur)));
    assert(prefix + Seq::<u8>::empty() == prefix);
}

/// The remaining work of `rle_rest` yields `3-ci` channels, each of `n` bytes.
pub proof fn lemma_rle_rest_len(s: Seq<u8>, n: int, ci: int, prefix: Seq<u8>, cur: int)
    requires
        0 <= ci <= 3,
        prefix.len() <= n,
        0 <= cur,
    ensures
        match rle_rest(s, n, ci, prefix, cur) {
            Ok((chs, _)) => chs.len() == 3 - ci && (ci < 3 ==> chs[0].len() == n),
            Err(_) => true,
        },
    decreases (3 - ci) as nat,
{
    reveal(rle_rest);
    if ci >= 3 {
    } else {
        match rle_chan_from(s, n, prefix.len() as int, cur) {
            Err(_) => {},
            Ok((sfx, c)) => {
                lemma_rle_chan_from_len(s, n, prefix.len() as int, cur);
                assert(sfx.len() == n - prefix.len());
                assert((prefix + sfx).len() == n);
                lemma_rle_rest_len(s, n, ci + 1, Seq::empty(), c);
                match rle_rest(s, n, ci + 1, Seq::empty(), c) {
                    Ok((rest, cf)) => {
                        assert(rest.len() == 3 - (ci + 1));
                        assert(cons(prefix + sfx, rest).len() == 3 - ci);
                    },
                    Err(_) => {},
                }
            },
        }
    }
}

// ---- The executable's RLE loop state (`rle_pref`, cf. `walk_pref`) --------

/// The views of the first `ci` executable channels.
pub open spec fn chans_prefix_view(channels: Seq<Vec<u8>>, ci: int) -> Seq<Seq<u8>> {
    Seq::new(ci as nat, |i: int| channels[i]@)
}

pub proof fn lemma_chans_prefix_push(channels: Seq<Vec<u8>>, ci: int)
    requires
        0 <= ci < channels.len(),
    ensures
        chans_prefix_view(channels, ci).push(channels[ci]@) == chans_prefix_view(channels, ci + 1),
{
    reveal(chans_prefix_view);
    lemma_seq_new_len(ci as nat, |i: int| channels[i]@);
    lemma_seq_new_len((ci + 1) as nat, |i: int| channels[i]@);
    assert(chans_prefix_view(channels, ci + 1) =~= chans_prefix_view(channels, ci).push(
        channels[ci]@,
    )) by {
        assert forall|i: int|
            0 <= i < chans_prefix_view(channels, ci + 1).len() implies (#[trigger]chans_prefix_view(
            channels,
            ci + 1,
        )[i]) == chans_prefix_view(channels, ci).push(channels[ci]@)[i] by {
            lemma_seq_new_index((ci + 1) as nat, |j: int| channels[j]@, i);
            if i < ci {
                lemma_seq_new_index(ci as nat, |j: int| channels[j]@, i);
                vstd::seq::lemma_seq_push_index_different(chans_prefix_view(channels, ci), channels[ci]@, i);
            } else {
                vstd::seq::lemma_seq_push_index_same(chans_prefix_view(channels, ci), channels[ci]@, i);
            }
        }
    }
}

/// The executable's RLE loop state. `channels[ci]` is preallocated to `n` bytes
/// and its first `produced` bytes are the produced prefix of channel `ci`;
/// `acc` is the assembled sequence of the first `ci` channel views. The
/// predicate says the global model run from a fresh start agrees with this
/// state: the completed channels are `acc`, then the remaining model work.
pub open spec fn rle_pref(
    s: Seq<u8>,
    n: int,
    ci: int,
    produced: int,
    cur: int,
    channels: Seq<Vec<u8>>,
    acc: Seq<Seq<u8>>,
) -> bool {
    &&& channels.len() == 3
    &&& 0 <= ci <= 3
    &&& 0 <= produced <= n
    &&& 0 <= cur
    &&& (forall|i: int| 0 <= i < 3 ==> (#[trigger] channels[i]@).len() == n)
    &&& acc == chans_prefix_view(channels, ci)
    &&& match rle_rest(s, n, ci, channels[ci]@.subrange(0, produced), cur) {
        Err(e) => rle_run(s, n) == Err(e),
        Ok((chs, c)) => chs.len() == 3 - ci && rle_run(s, n) == Ok((acc + chs, c)),
    }
}

pub proof fn lemma_rle_pref_init(s: Seq<u8>, n: int, channels: Seq<Vec<u8>>)
    requires
        channels.len() == 3,
        forall|i: int| 0 <= i < 3 ==> (#[trigger] channels[i]@).len() == n,
        0 <= n,
    ensures
        rle_pref(s, n, 0, 0, 0, channels, Seq::empty()),
{
    reveal(rle_pref);
    reveal(rle_run);
    reveal(chans_prefix_view);
    lemma_seq_new_len(0nat, |i: int| channels[i]@);
    assert(chans_prefix_view(channels, 0) =~= Seq::<Seq<u8>>::empty());
    assert(Seq::<u8>::empty().len() as int == 0);
    assert(channels[0]@.subrange(0, 0) =~= Seq::<u8>::empty());
    assert(rle_run(s, n) == rle_rest(s, n, 0, Seq::empty(), 0));
    lemma_rle_rest_len(s, n, 0, Seq::empty(), 0);
    assert(Seq::<Seq<u8>>::empty() + (match rle_run(s, n) {
        Ok((chs, _)) => chs,
        Err(_) => Seq::empty(),
    }) == (match rle_run(s, n) {
        Ok((chs, _)) => chs,
        Err(_) => Seq::empty(),
    }));
}

pub proof fn lemma_rle_pref_step(
    s: Seq<u8>,
    n: int,
    ci: int,
    produced: int,
    cur: int,
    channels: Seq<Vec<u8>>,
    acc: Seq<Seq<u8>>,
)
    requires
        rle_pref(s, n, ci, produced, cur, channels, acc),
        ci < 3,
        produced + rle_count(s[cur]) <= n,
        cur + 1 + rle_needed(s[cur]) <= s.len(),
        0 <= cur < s.len(),
        channels[ci]@.subrange(0, produced + rle_count(s[cur])) == channels[ci]@.subrange(
            0,
            produced,
        ) + rle_bytes(s, cur + 1, rle_count(s[cur]), rle_literal(s[cur])),
    ensures
        rle_pref(s, n, ci, produced + rle_count(s[cur]), cur + 1 + rle_needed(s[cur]), channels, acc),
{
    reveal(rle_pref);
    let rb = rle_bytes(s, cur + 1, rle_count(s[cur]), rle_literal(s[cur]));
    let count = rle_count(s[cur]);
    assert(channels[ci]@.subrange(0, produced) + rb == channels[ci]@.subrange(0, produced + count));
    lemma_rle_rest_step(s, n, ci, channels[ci]@.subrange(0, produced), cur);
    assert(rle_rest(s, n, ci, channels[ci]@.subrange(0, produced + count), cur + 1 + rle_needed(
        s[cur],
    )) == rle_rest(s, n, ci, channels[ci]@.subrange(0, produced) + rb, cur + 1 + rle_needed(
        s[cur],
    )));
}

pub proof fn lemma_rle_pref_done_chan(
    s: Seq<u8>,
    n: int,
    ci: int,
    cur: int,
    channels: Seq<Vec<u8>>,
    acc: Seq<Seq<u8>>,
)
    requires
        rle_pref(s, n, ci, n, cur, channels, acc),
        ci < 3,
    ensures
        rle_pref(s, n, ci + 1, 0, cur, channels, acc.push(channels[ci]@)),
{
    reveal(rle_pref);
    reveal(rle_run);
    let prefix = channels[ci]@.subrange(0, n);
    assert(prefix == channels[ci]@);
    lemma_rle_rest_done_chan(s, n, ci, prefix, cur);
    lemma_chans_prefix_push(channels, ci);
    assert(channels[ci + 1]@.subrange(0, 0) =~= Seq::<u8>::empty());
    let r_next = rle_rest(s, n, ci + 1, Seq::empty(), cur);
    let r_cur = rle_rest(s, n, ci, prefix, cur);
    match r_next {
        Err(e) => {
            assert(r_cur == Err(e));
        },
        Ok((rest, cf)) => {
            assert(r_cur == Ok((cons(prefix, rest), cf)));
            assert(rle_run(s, n) == Ok((acc + cons(prefix, rest), cf)));
            lemma_push_cons_shift(acc, prefix, rest);
            assert(acc.push(prefix) + rest == acc + cons(prefix, rest));
            assert(channels[ci]@ == prefix);
            assert(acc.push(channels[ci]@) + rest == acc + cons(prefix, rest));
        },
    }
}

/// At `ci == 3` the model run is fully assembled.
pub proof fn lemma_rle_pref_finish(
    s: Seq<u8>,
    n: int,
    cur: int,
    channels: Seq<Vec<u8>>,
    acc: Seq<Seq<u8>>,
)
    requires
        rle_pref(s, n, 3, 0, cur, channels, acc),
    ensures
        rle_run(s, n) == Ok((acc, cur)),
        acc.len() == 3,
        acc == chans_prefix_view(channels, 3),
{
    reveal(rle_pref);
    reveal(rle_rest);
    reveal(chans_prefix_view);
    assert(rle_rest(s, n, 3, Seq::empty(), cur) == Ok((Seq::<Seq<u8>>::empty(), cur)));
    assert(Seq::<Seq<u8>>::empty().len() == 0);
}
// ===========================================================================
// M4a — MC (decode_rgb) and MA (decode_mask).
// ===========================================================================

/// The four type bytes of an entry, from its `type_hex` string (R1).
pub open spec fn entry_type_code(e: Entry) -> [u8; 4] {
    code_of(hex_str(e.type_hex))
}

/// The payload an entry names, as a subrange of `b`.
pub open spec fn entry_payload_bytes(b: Seq<u8>, e: Entry) -> Seq<u8> {
    b.subrange(e.payload[0] as int, e.payload[1] as int)
}

/// R11's prefix removal: for a payload beginning with four zero bytes, drop
/// exactly those four.
pub open spec fn it32_prefix(p: Seq<u8>) -> Seq<u8> {
    if p.len() >= 4 && p[0] == 0 && p[1] == 0 && p[2] == 0 && p[3] == 0 {
        p.subrange(4, p.len() as int)
    } else {
        p
    }
}

/// R11 applies only to `it32`.
pub open spec fn strip_it32(code: [u8; 4], p: Seq<u8>) -> Seq<u8> {
    if is_code(code, 105u8, 116, 51, 50) {
        it32_prefix(p)
    } else {
        p
    }
}

/// R10's interleave: `out[3k..3k+3] = [R[k],G[k],B[k]]`, `k = y*d+x`.
pub open spec fn interleave_rgb(R: Seq<u8>, G: Seq<u8>, B: Seq<u8>) -> Seq<u8> {
    Seq::new(3 * R.len(), |k: int|
        if k % 3 == 0 {
            R[k / 3]
        } else if k % 3 == 1 {
            G[k / 3]
        } else {
            B[k / 3]
        })
}

/// R14's RGBA interleave: `out[4k..4k+4] = [rgb[3k],rgb[3k+1],rgb[3k+2],alpha[k]]`,
/// `k = y*d+x`, increasing x then y.
pub open spec fn interleave_rgba(rgb: Seq<u8>, alpha: Seq<u8>) -> Seq<u8> {
    Seq::new(4 * alpha.len(), |j: int|
        if j % 4 == 3 {
            alpha[j / 4]
        } else {
            rgb[3 * (j / 4) + (j % 4)]
        })
}

/// `MC(b,E,i)`: index test, then the four legacy colour types, then R11's
/// `it32` prefix and `MR` on the payload; exact three-channel interleave.
pub open spec fn mc(b: Seq<u8>, E: Seq<Entry>, i: u32) -> Result<(u32, u32, Seq<u8>), Error> {
    if i as int >= E.len() {
        Err(Error::IndexOutOfRange)
    } else {
        let e = E[i as int];
        let code = entry_type_code(e);
        match legacy_side(code) {
            Some((true, side)) => {
                let n: int = (side as int) * (side as int);
                let p2 = strip_it32(code, entry_payload_bytes(b, e));
                match mr(p2, n) {
                    Ok((r, g, bl)) => Ok((side, side, interleave_rgb(r, g, bl))),
                    Err(err) => Err(err),
                }
            },
            _ => Err(Error::UnsupportedType),
        }
    }
}

/// `MA(b,E,i)`: index test, then the four legacy mask types, then the exact
/// payload length `d²`; the payload is returned verbatim.
pub open spec fn ma(b: Seq<u8>, E: Seq<Entry>, i: u32) -> Result<(u32, u32, Seq<u8>), Error> {
    if i as int >= E.len() {
        Err(Error::IndexOutOfRange)
    } else {
        let e = E[i as int];
        let code = entry_type_code(e);
        match legacy_side(code) {
            Some((false, side)) => {
                let n: int = (side as int) * (side as int);
                let p = entry_payload_bytes(b, e);
                if p.len() == n {
                    Ok((side, side, p))
                } else {
                    Err(Error::InvalidMaskLength)
                }
            },
            _ => Err(Error::UnsupportedType),
        }
    }
}

/// A byte sequence equal to `interleave_rgb(R,G,B)` when each index `3k+c`
/// carries the right channel byte.
pub proof fn lemma_interleave_rgb_exact(
    R: Seq<u8>,
    G: Seq<u8>,
    B: Seq<u8>,
    out: Seq<u8>,
)
    requires
        R.len() == G.len(),
        G.len() == B.len(),
        out.len() == 3 * R.len(),
        forall|k: int| 0 <= k < R.len() ==> #[trigger] out[3 * k] == R[k],
        forall|k: int| 0 <= k < G.len() ==> #[trigger] out[3 * k + 1] == G[k],
        forall|k: int| 0 <= k < B.len() ==> #[trigger] out[3 * k + 2] == B[k],
    ensures
        out == interleave_rgb(R, G, B),
{
    reveal(interleave_rgb);
    lemma_seq_new_len((3 * R.len()) as nat, |k: int|
        if k % 3 == 0 {
            R[k / 3]
        } else if k % 3 == 1 {
            G[k / 3]
        } else {
            B[k / 3]
        });
    assert forall|j: int| 0 <= j < out.len() implies (#[trigger] out[j]) == interleave_rgb(
        R,
        G,
        B,
    )[j] by {
        reveal(interleave_rgb);
        lemma_seq_new_index((3 * R.len()) as nat, |k: int|
            if k % 3 == 0 {
                R[k / 3]
            } else if k % 3 == 1 {
                G[k / 3]
            } else {
                B[k / 3]
            }, j);
        let k = j / 3;
        lemma_fundamental_div_mod(j, 3);
        assert(j % 3 == j - 3 * k);
        assert(k < R.len()) by (nonlinear_arith)
            requires j < 3 * R.len(), 3 * k <= j, k == j / 3;
        if j % 3 == 0 {
            assert(interleave_rgb(R, G, B)[j] == R[k]);
            assert(j == 3 * k) by {
                lemma_fundamental_div_mod(j, 3);
            }
        } else if j % 3 == 1 {
            assert(interleave_rgb(R, G, B)[j] == G[k]);
            assert(j == 3 * k + 1) by {
                lemma_fundamental_div_mod(j, 3);
            }
        } else {
            assert(interleave_rgb(R, G, B)[j] == B[k]);
            assert(j == 3 * k + 2) by {
                lemma_fundamental_div_mod(j, 3);
                lemma_small_mod((j % 3) as nat, 3);
            }
        }
    }
}

/// `MC` on a colour entry whose `MR` succeeds, with the executable channel and
/// output views supplied.
pub proof fn lemma_rgb_ok(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
    p2: Seq<u8>,
    R: Seq<u8>,
    G: Seq<u8>,
    B: Seq<u8>,
    out: Seq<u8>,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
        n == (side as int) * (side as int),
        p2 == strip_it32(code, entry_payload_bytes(b, E[i as int])),
        mr(p2, n) == Ok((R, G, B)),
        out == interleave_rgb(R, G, B),
    ensures
        mc(b, E, i) == Ok((side, side, out)),
{
    reveal(mc);
}

/// `MC` on a colour entry whose `MR` fails: the error propagates.
pub proof fn lemma_rgb_err(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
    p2: Seq<u8>,
    e: Error,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
        n == (side as int) * (side as int),
        p2 == strip_it32(code, entry_payload_bytes(b, E[i as int])),
        mr(p2, n) == Err(e),
    ensures
        mc(b, E, i) == Err(e),
{
    reveal(mc);
}

/// `MA` at an out-of-range index.
pub proof fn lemma_ma_index(b: Seq<u8>, E: Seq<Entry>, i: u32)
    requires
        (i as int) >= (E.len() as int),
    ensures
        ma(b, E, i) == Err(Error::IndexOutOfRange),
{
    reveal(ma);
}

/// `MA` when the selected entry has no legacy side at all.
pub proof fn lemma_ma_unsupported_none(b: Seq<u8>, E: Seq<Entry>, i: u32, code: [u8; 4])
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code).is_none(),
    ensures
        ma(b, E, i) == Err(Error::UnsupportedType),
{
    reveal(ma);
}

/// `MA` when the selected entry is a legacy colour type.
pub proof fn lemma_ma_unsupported_colour(b: Seq<u8>, E: Seq<Entry>, i: u32, code: [u8; 4], side: u32)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
    ensures
        ma(b, E, i) == Err(Error::UnsupportedType),
{
    reveal(ma);
}

/// `MA` when the payload length is not `d²`.
pub proof fn lemma_ma_length(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((false, side)),
        n == (side as int) * (side as int),
        entry_payload_bytes(b, E[i as int]).len() != n,
    ensures
        ma(b, E, i) == Err(Error::InvalidMaskLength),
{
    reveal(ma);
}

/// `MA` on a successful mask decode.
pub proof fn lemma_ma_ok(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
    p: Seq<u8>,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((false, side)),
        n == (side as int) * (side as int),
        p == entry_payload_bytes(b, E[i as int]),
        p.len() == n,
    ensures
        ma(b, E, i) == Ok((side, side, p)),
{
    reveal(ma);
}

/// `MC` on a model entry list, the form T10 quantifies over in model terms.
pub open spec fn mc_ms(b: Seq<u8>, ms: Seq<MpEntry>, i: u32) -> Result<(u32, u32, Seq<u8>), Error> {
    if i as int >= ms.len() {
        Err(Error::IndexOutOfRange)
    } else {
        let e = ms[i as int];
        let code = code_of(e.type_hex);
        match legacy_side(code) {
            Some((true, side)) => {
                let n: int = (side as int) * (side as int);
                let p2 = strip_it32(code, b.subrange(e.payload.0 as int, e.payload.1 as int));
                match mr(p2, n) {
                    Ok((r, g, bl)) => Ok((side, side, interleave_rgb(r, g, bl))),
                    Err(err) => Err(err),
                }
            },
            _ => Err(Error::UnsupportedType),
        }
    }
}

/// `MA` on a model entry list.
pub open spec fn ma_ms(b: Seq<u8>, ms: Seq<MpEntry>, i: u32) -> Result<(u32, u32, Seq<u8>), Error> {
    if i as int >= ms.len() {
        Err(Error::IndexOutOfRange)
    } else {
        let e = ms[i as int];
        let code = code_of(e.type_hex);
        match legacy_side(code) {
            Some((false, side)) => {
                let n: int = (side as int) * (side as int);
                let p = b.subrange(e.payload.0 as int, e.payload.1 as int);
                if p.len() == n {
                    Ok((side, side, p))
                } else {
                    Err(Error::InvalidMaskLength)
                }
            },
            _ => Err(Error::UnsupportedType),
        }
    }
}

/// The executable decoded result matches the model's `Ok(side,side,bytes)` or
/// the exact error.
pub open spec fn decoded_eq(
    r: Result<Decoded, Error>,
    m: Result<(u32, u32, Seq<u8>), Error>,
) -> bool {
    match m {
        Ok(x) => match r {
            Ok(d) => d.width == x.0 && d.height == x.1 && d.data@ == x.2,
            Err(_) => false,
        },
        Err(e) => match r {
            Err(e2) => e2 == e,
            Ok(_) => false,
        },
    }
}

/// The executable RLE result matches the model's three channels or error.
pub open spec fn rle_result_eq(
    r: Result<[Vec<u8>; 3], Error>,
    m: Result<(Seq<u8>, Seq<u8>, Seq<u8>), Error>,
) -> bool {
    match m {
        Ok(x) => match r {
            Ok(ch) => ch@.len() == 3 && ch@[0]@ == x.0 && ch@[1]@ == x.1 && ch@[2]@ == x.2,
            Err(_) => false,
        },
        Err(e) => match r {
            Err(e2) => e2 == e,
            Ok(_) => false,
        },
    }
}

/// `MC` at an out-of-range index.
pub proof fn lemma_mc_index(b: Seq<u8>, E: Seq<Entry>, i: u32)
    requires
        (i as int) >= (E.len() as int),
    ensures
        mc(b, E, i) == Err(Error::IndexOutOfRange),
{
    reveal(mc);
}

/// `MC` when the selected entry has no legacy side at all.
pub proof fn lemma_mc_unsupported_none(b: Seq<u8>, E: Seq<Entry>, i: u32, code: [u8; 4])
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code).is_none(),
    ensures
        mc(b, E, i) == Err(Error::UnsupportedType),
{
    reveal(mc);
}

/// `MC` when the selected entry is a legacy mask type.
pub proof fn lemma_mc_unsupported_mask(b: Seq<u8>, E: Seq<Entry>, i: u32, code: [u8; 4], side: u32)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((false, side)),
    ensures
        mc(b, E, i) == Err(Error::UnsupportedType),
{
    reveal(mc);
}

/// `MC` on a colour entry: it decodes the payload's `it32`-stripped bytes with
/// `MR` and interleaves the three channels.
pub proof fn lemma_mc_colour(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
        n == (side as int) * (side as int),
    ensures
        mc(b, E, i) == match mr(strip_it32(code, entry_payload_bytes(b, E[i as int])), n) {
            Ok(t) => Ok((side, side, interleave_rgb(t.0, t.1, t.2))),
            Err(e) => Err(e),
        },
{
    reveal(mc);
}

/// `MC` on a colour entry whose `MR` succeeds.
pub proof fn lemma_mc_ok(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
        n == (side as int) * (side as int),
    ensures
        mc(b, E, i) == match mr(strip_it32(code, entry_payload_bytes(b, E[i as int])), n) {
            Ok(t) => Ok((side, side, interleave_rgb(t.0, t.1, t.2))),
            Err(e) => Err(e),
        },
{
    reveal(mc);
}

/// `MC` on a colour entry whose `MR` fails: the error propagates.
pub proof fn lemma_mc_err(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    n: int,
    e: Error,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
        n == (side as int) * (side as int),
        mr(strip_it32(code, entry_payload_bytes(b, E[i as int])), n) == Err(e),
    ensures
        mc(b, E, i) == Err(e),
{
    reveal(mc);
}

/// The executable `decode_mask` result is exactly `MA(b,E,i)`.
pub open spec fn mask_decoded_eq(
    r: Result<Decoded, Error>,
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
) -> bool {
    decoded_eq(r, ma(b, E, i))
}

/// `MC` depends only on the entry records' type bytes and payload range, so
/// `entries_eq_models` carries `mc` to `mc_ms`.
pub proof fn lemma_mc_ms_eq(b: Seq<u8>, es: Seq<Entry>, ms: Seq<MpEntry>, i: u32)
    requires
        entries_eq_models(es, ms),
    ensures
        mc(b, es, i) == mc_ms(b, ms, i),
{
    reveal(entries_eq_models);
    reveal(entry_eq_fields);
    reveal(mc);
    reveal(mc_ms);
    if (i as int) < (es.len() as int) {
        let e = es[i as int];
        let m = ms[i as int];
        assert(entry_eq_fields(e, m));
        assert(hex_str(e.type_hex) == m.type_hex);
        assert(entry_type_code(e) == code_of(m.type_hex)) by {
            reveal(entry_type_code);
        }
        assert(entry_payload_bytes(b, e) == b.subrange(m.payload.0 as int, m.payload.1 as int)) by {
            reveal(entry_payload_bytes);
        }
    }
}

/// `MA` depends only on the same fields.
pub proof fn lemma_ma_ms_eq(b: Seq<u8>, es: Seq<Entry>, ms: Seq<MpEntry>, i: u32)
    requires
        entries_eq_models(es, ms),
    ensures
        ma(b, es, i) == ma_ms(b, ms, i),
{
    reveal(entries_eq_models);
    reveal(entry_eq_fields);
    reveal(ma);
    reveal(ma_ms);
    if (i as int) < (es.len() as int) {
        let e = es[i as int];
        let m = ms[i as int];
        assert(entry_eq_fields(e, m));
        assert(hex_str(e.type_hex) == m.type_hex);
        assert(entry_type_code(e) == code_of(m.type_hex)) by {
            reveal(entry_type_code);
        }
        assert(entry_payload_bytes(b, e) == b.subrange(m.payload.0 as int, m.payload.1 as int)) by {
            reveal(entry_payload_bytes);
        }
    }
}

/// The executable `decode_rgb` result is exactly `MC(b,E,i)`.
pub open spec fn rgb_decoded_eq(r: Result<Decoded, Error>, b: Seq<u8>, E: Seq<Entry>, i: u32) -> bool {
    decoded_eq(r, mc(b, E, i))
}

/// `stripping the it32 prefix` on the executable payload view agrees.
pub proof fn lemma_strip_it32_len(code: [u8; 4], p: Seq<u8>)
    ensures
        strip_it32(code, p).len() <= p.len(),
{
    reveal(strip_it32); reveal(it32_prefix);
}

/// T8 for an arbitrary candidate E and a concrete parse result r. A caller
/// supplies r via `parse(bytes)`; its T7 postcondition discharges the premise.
pub proof fn lemma_t8_result(b: Seq<u8>, r: Result<Vec<Entry>, Error>, es: Seq<Entry>)
    requires parses_to(b, r),
    ensures parse_returned(r, es) <==> p_exec(b, es),
{
    reveal(parses_to);
    reveal(parse_returned);
    reveal(p_exec);
    match mp(b) {
        Ok(ms) => match r {
            Ok(v) => { lemma_exact_entry_equivalence(v@, ms, es); },
            Err(_) => { assert(false); },
        },
        Err(_) => match r {
            Ok(_) => { assert(false); },
            Err(_) => {},
        },
    }
}

// ===========================================================================
// M4b — the reference model MJ (R14, combine) and T12/T13.
//
// `mj` is written independently of `combine`: the two index tests in R14's
// order (MissingColor before MissingMask), then `MC` (colour) and `MA` (mask)
// with their errors propagated in that exact order, then the side comparison
// (TypeMismatch), then R14's four-byte interleave.
// ===========================================================================

/// `MJ(b,E,ci,mi)`: R14's combine with the exact error precedence.
pub open spec fn mj(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
) -> Result<(u32, u32, Seq<u8>), Error> {
    match ci {
        None => Err(Error::MissingColor),
        Some(i) => match mi {
            None => Err(Error::MissingMask),
            Some(j) => match mc(b, E, i) {
                Err(e) => Err(e),
                Ok(rc) => match ma(b, E, j) {
                    Err(e) => Err(e),
                    Ok(rm) => if rc.0 != rm.0 || rc.1 != rm.1 {
                        Err(Error::TypeMismatch)
                    } else {
                        Ok((rc.0, rc.1, interleave_rgba(rc.2, rm.2)))
                    },
                },
            },
        },
    }
}

/// The executable `combine` result is exactly `MJ(b,E,ci,mi)`.
pub open spec fn combine_decoded_eq(
    r: Result<Decoded, Error>,
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
) -> bool {
    decoded_eq(r, mj(b, E, ci, mi))
}

/// `interleave_rgba` has length `4 * |alpha|`.
pub proof fn lemma_interleave_rgba_len(rgb: Seq<u8>, alpha: Seq<u8>)
    ensures
        interleave_rgba(rgb, alpha).len() == 4 * alpha.len(),
{
    reveal(interleave_rgba);
    lemma_seq_new_len((4 * alpha.len()) as nat, |j: int|
        if j % 4 == 3 {
            alpha[j / 4]
        } else {
            rgb[3 * (j / 4) + (j % 4)]
        });
}

/// A byte sequence equal to `interleave_rgba(rgb,alpha)` when each index `4k+c`
/// carries the right byte.
pub proof fn lemma_interleave_rgba_exact(
    rgb: Seq<u8>,
    alpha: Seq<u8>,
    out: Seq<u8>,
)
    requires
        rgb.len() == 3 * alpha.len(),
        out.len() == 4 * alpha.len(),
        forall|k: int| 0 <= k < alpha.len() ==> #[trigger] out[4 * k] == rgb[3 * k],
        forall|k: int| 0 <= k < alpha.len() ==> #[trigger] out[4 * k + 1] == rgb[3 * k + 1],
        forall|k: int| 0 <= k < alpha.len() ==> #[trigger] out[4 * k + 2] == rgb[3 * k + 2],
        forall|k: int| 0 <= k < alpha.len() ==> #[trigger] out[4 * k + 3] == alpha[k],
    ensures
        out == interleave_rgba(rgb, alpha),
{
    reveal(interleave_rgba);
    lemma_interleave_rgba_len(rgb, alpha);
    assert forall|j: int| 0 <= j < out.len() implies (#[trigger] out[j]) == interleave_rgba(
        rgb,
        alpha,
    )[j] by {
        reveal(interleave_rgba);
        lemma_seq_new_index((4 * alpha.len()) as nat, |j: int|
            if j % 4 == 3 {
                alpha[j / 4]
            } else {
                rgb[3 * (j / 4) + (j % 4)]
            }, j);
        let k = j / 4;
        lemma_fundamental_div_mod(j, 4);
        assert(j % 4 == j - 4 * k);
        assert(k < alpha.len()) by (nonlinear_arith)
            requires j < 4 * alpha.len(), 4 * k <= j, k == j / 4;
        if j % 4 == 0 {
            assert(3 * k < rgb.len()) by (nonlinear_arith)
                requires k < alpha.len(), rgb.len() == 3 * alpha.len();
            assert(interleave_rgba(rgb, alpha)[j] == rgb[3 * k]);
            assert(j == 4 * k) by {
                lemma_fundamental_div_mod(j, 4);
            }
        } else if j % 4 == 1 {
            assert(3 * k + 1 < rgb.len()) by (nonlinear_arith)
                requires k < alpha.len(), rgb.len() == 3 * alpha.len();
            assert(interleave_rgba(rgb, alpha)[j] == rgb[3 * k + 1]);
            assert(j == 4 * k + 1) by {
                lemma_fundamental_div_mod(j, 4);
                lemma_small_mod((j % 4) as nat, 4);
            }
        } else if j % 4 == 2 {
            assert(3 * k + 2 < rgb.len()) by (nonlinear_arith)
                requires k < alpha.len(), rgb.len() == 3 * alpha.len();
            assert(interleave_rgba(rgb, alpha)[j] == rgb[3 * k + 2]);
            assert(j == 4 * k + 2) by {
                lemma_fundamental_div_mod(j, 4);
                lemma_small_mod((j % 4) as nat, 4);
            }
        } else {
            assert(k < alpha.len());
            assert(interleave_rgba(rgb, alpha)[j] == alpha[k]);
            assert(j == 4 * k + 3) by {
                lemma_fundamental_div_mod(j, 4);
                lemma_small_mod((j % 4) as nat, 4);
            }
        }
    }
}

/// `MJ` at an absent colour index: `MissingColor` before anything else.
pub proof fn lemma_mj_missing_color(b: Seq<u8>, E: Seq<Entry>, mi: Option<u32>)
    ensures mj(b, E, None, mi) == Err(Error::MissingColor),
{
    reveal(mj);
}

/// `MJ` at an absent mask index with a present colour index: `MissingMask`.
pub proof fn lemma_mj_missing_mask(b: Seq<u8>, E: Seq<Entry>, i: u32)
    ensures mj(b, E, Some(i), None) == Err(Error::MissingMask),
{
    reveal(mj);
}

/// `MJ` when `MC` fails: its error propagates before `MA` is consulted.
pub proof fn lemma_mj_mc_err(b: Seq<u8>, E: Seq<Entry>, i: u32, j: u32, e: Error)
    requires mc(b, E, i) == Err(e),
    ensures mj(b, E, Some(i), Some(j)) == Err(e),
{
    reveal(mj);
}

/// `MJ` when `MC` succeeds and `MA` fails: `MA`'s error propagates.
pub proof fn lemma_mj_ma_err(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    j: u32,
    w: u32,
    h: u32,
    c: Seq<u8>,
    e: Error,
)
    requires
        mc(b, E, i) == Ok((w, h, c)),
        ma(b, E, j) == Err(e),
    ensures
        mj(b, E, Some(i), Some(j)) == Err(e),
{
    reveal(mj);
}

/// `MJ` when both decodes succeed but the sides differ: `TypeMismatch`.
pub proof fn lemma_mj_mismatch(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    j: u32,
    w: u32,
    h: u32,
    c: Seq<u8>,
    w2: u32,
    h2: u32,
    a: Seq<u8>,
)
    requires
        mc(b, E, i) == Ok((w, h, c)),
        ma(b, E, j) == Ok((w2, h2, a)),
        w != w2 || h != h2,
    ensures
        mj(b, E, Some(i), Some(j)) == Err(Error::TypeMismatch),
{
    reveal(mj);
}

/// `MJ` on a successful combine: the exact RGBA interleave.
pub proof fn lemma_mj_ok(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    j: u32,
    w: u32,
    h: u32,
    c: Seq<u8>,
    a: Seq<u8>,
)
    requires
        mc(b, E, i) == Ok((w, h, c)),
        ma(b, E, j) == Ok((w, h, a)),
    ensures
        mj(b, E, Some(i), Some(j)) == Ok((w, h, interleave_rgba(c, a))),
{
    reveal(mj);
}

/// `decoded_eq` on a failed executable result pins the model to the same error.
pub proof fn lemma_decoded_eq_err(
    r: Result<Decoded, Error>,
    m: Result<(u32, u32, Seq<u8>), Error>,
    e: Error,
)
    requires decoded_eq(r, m), r == Err(e),
    ensures m == Err(e),
{
    reveal(decoded_eq);
}


/// Any code the R8 table classifies has a side at most 128: a property of the
/// `legacy_side` table alone, so it needs no hypothesis on the entry.
pub proof fn lemma_legacy_side_bound(code: [u8; 4])
    ensures
        match legacy_side(code) {
            Some((_, s)) => s <= 128,
            None => true,
        },
{
    reveal(legacy_side);
    reveal(is_code);
}

/// The R8 side bound in the squared form `Req_C`/`Req_A`/`Req_J` need:
/// `s*s <= 16384` for every legacy code.
pub proof fn lemma_legacy_side_n_bound(code: [u8; 4])
    ensures
        match legacy_side(code) {
            Some((_, s)) => (s as int) * (s as int) <= 16384,
            None => true,
        },
{
    reveal(legacy_side);
    reveal(is_code);
    if legacy_side(code) is Some {
        let s = legacy_side(code).unwrap().1;
        assert(s as int <= 128);
        assert((s as int) * (s as int) <= 16384) by (nonlinear_arith)
            requires 0 <= s as int, s as int <= 128;
    }
}

/// The R8 table's smallest side is 16, so every classified code has
/// `16 <= s <= 128` and `s*s >= 1`. The lower bound is what `assert(1 <= n)`
/// in the decoders needs.
pub proof fn lemma_legacy_side_ge_16(code: [u8; 4])
    ensures
        match legacy_side(code) {
            Some((_, s)) => 16 <= s <= 128,
            None => true,
        },
{
    reveal(legacy_side);
    reveal(is_code);
}

/// The RLE error propagates through `mc`: if the strip-and-RLE step of entry
/// `i` is `Err(e)`, then `mc(b, E, i) == Err(e)` too (A-ICNS-4's error exit).
pub proof fn lemma_mc_rle_err(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    code: [u8; 4],
    side: u32,
    e: Error,
)
    requires
        (i as int) < (E.len() as int),
        code == entry_type_code(E[i as int]),
        legacy_side(code) == Some((true, side)),
        mr(strip_it32(code, entry_payload_bytes(b, E[i as int])),
            (side as int) * (side as int)) == Err(e),
    ensures
        mc(b, E, i) == Err(e),
{
    reveal(mc);
}

/// The payload byte length an entry names equals `payload[1] - payload[0]`.
pub proof fn lemma_entry_payload_bytes_len(b: Seq<u8>, e: Entry)
    requires
        e.payload[0] as int <= e.payload[1] as int,
        e.payload[1] as int <= b.len(),
    ensures
        entry_payload_bytes(b, e).len()
            == (e.payload[1] as int) - (e.payload[0] as int),
{
    reveal(entry_payload_bytes);
    lemma_seq_subrange_len(b, e.payload[0] as int, e.payload[1] as int);
}
/// `decoded_eq` on a successful executable result pins width, height and bytes.
pub proof fn lemma_decoded_eq_ok(
    r: Result<Decoded, Error>,
    m: Result<(u32, u32, Seq<u8>), Error>,
    d: Decoded,
)
    requires decoded_eq(r, m), r == Ok(d),
    ensures m == Ok((d.width, d.height, d.data@)),
{
    reveal(decoded_eq);
}

/// `MC` depends only on the entries' R1 fields, so an `entries_same_fields`
/// pair gives the same `MC`.
pub proof fn lemma_mc_fields_eq(b: Seq<u8>, a: Seq<Entry>, a2: Seq<Entry>, i: u32)
    requires
        entries_same_fields(a, a2),
    ensures
        mc(b, a, i) == mc(b, a2, i),
{
    reveal(entries_same_fields);
    reveal(mc);
    reveal(entry_type_code);
    reveal(entry_payload_bytes);
    if (i as int) < a.len() {
        let e1 = a[i as int];
        let e2 = a2[i as int];
        assert(hex_str(e1.type_hex) == hex_str(e2.type_hex));
        assert(e1.payload[0] == e2.payload[0]);
        assert(e1.payload[1] == e2.payload[1]);
    }
}

/// `MA` depends only on the same fields.
pub proof fn lemma_ma_fields_eq(b: Seq<u8>, a: Seq<Entry>, a2: Seq<Entry>, i: u32)
    requires
        entries_same_fields(a, a2),
    ensures
        ma(b, a, i) == ma(b, a2, i),
{
    reveal(entries_same_fields);
    reveal(ma);
    reveal(entry_type_code);
    reveal(entry_payload_bytes);
    if (i as int) < a.len() {
        let e1 = a[i as int];
        let e2 = a2[i as int];
        assert(hex_str(e1.type_hex) == hex_str(e2.type_hex));
        assert(e1.payload[0] == e2.payload[0]);
        assert(e1.payload[1] == e2.payload[1]);
    }
}

/// `MJ` depends only on the entries' R1 fields.
pub proof fn lemma_mj_fields_eq(
    b: Seq<u8>,
    a: Seq<Entry>,
    a2: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
)
    requires
        entries_same_fields(a, a2),
    ensures
        mj(b, a, ci, mi) == mj(b, a2, ci, mi),
{
    reveal(mj);
    match ci {
        None => {},
        Some(i) => match mi {
            None => {},
            Some(j) => {
                lemma_mc_fields_eq(b, a, a2, i);
                lemma_ma_fields_eq(b, a, a2, j);
            },
        },
    }
}

/// `MJ` on a model entry list: the same R14 order over `MpEntry` records, so the
/// model can be evaluated against a fixture's `entries` field directly.
pub open spec fn mj_ms(
    b: Seq<u8>,
    ms: Seq<MpEntry>,
    ci: Option<u32>,
    mi: Option<u32>,
) -> Result<(u32, u32, Seq<u8>), Error> {
    match ci {
        None => Err(Error::MissingColor),
        Some(i) => match mi {
            None => Err(Error::MissingMask),
            Some(j) => match mc_ms(b, ms, i) {
                Err(e) => Err(e),
                Ok(rc) => match ma_ms(b, ms, j) {
                    Err(e) => Err(e),
                    Ok(rm) => if rc.0 != rm.0 || rc.1 != rm.1 {
                        Err(Error::TypeMismatch)
                    } else {
                        Ok((rc.0, rc.1, interleave_rgba(rc.2, rm.2)))
                    },
                },
            },
        },
    }
}

/// `mj_ms` agrees with `mj` on the same model entries the executable relates to.
pub proof fn lemma_mj_ms_eq(
    b: Seq<u8>,
    es: Seq<Entry>,
    ms: Seq<MpEntry>,
    ci: Option<u32>,
    mi: Option<u32>,
)
    requires
        entries_eq_models(es, ms),
    ensures
        mj(b, es, ci, mi) == mj_ms(b, ms, ci, mi),
{
    reveal(mj); reveal(mj_ms);
    match ci {
        None => {},
        Some(i) => match mi {
            None => {},
            Some(j) => {
                lemma_mc_ms_eq(b, es, ms, i);
                lemma_ma_ms_eq(b, es, ms, j);
            },
        },
    }
}

/// T12 stated over `P(b,E)`: `try_combine` equals `MJ` exactly on every
/// non-`OutOfMemory` result. The `P(b,E)` premise is supplied by the caller (a
/// successful `try_parse` via T8, or directly).
#[allow(dead_code)]
fn t12_combine_exact_for_p(
    bytes: &[u8],
    entries: &[crate::icns::Entry],
    ci: Option<u32>,
    mi: Option<u32>,
) -> (r: Result<crate::icns::Decoded, crate::alloc_guard::sites::TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        !matches!(r, Err(crate::alloc_guard::sites::TryError::OutOfMemory(_)))
            ==> crate::model::combine_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                ci,
                mi,
            ),
{
    crate::try_combine(bytes, entries, ci, mi)
}

/// T13 at a concrete call path: `try_parse` runs, then `try_combine` runs on the
/// same bytes and the entries `try_parse` returned. The premise of the statement
/// is `parse(b) = Ok(E)`; `P(b,E)` is obtained inside the body from T7/T8, not
/// assumed. An `OutOfMemory` from either call makes the statement vacuous.
#[allow(dead_code)]
fn t13_combine_after_parse(
    bytes: &[u8],
    ci: Option<u32>,
    mi: Option<u32>,
) -> (out: (Result<Vec<crate::icns::Entry>, crate::alloc_guard::sites::TryError>,
           Result<crate::icns::Decoded, crate::alloc_guard::sites::TryError>))
    ensures
        !matches!(out.0, Err(crate::alloc_guard::sites::TryError::OutOfMemory(_)))
            && !matches!(out.1, Err(crate::alloc_guard::sites::TryError::OutOfMemory(_)))
            ==> match out.0 {
                Ok(entries) => crate::model::combine_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(out.1),
                    bytes@,
                    entries@,
                    ci,
                    mi,
                ),
                Err(_) => true,
            },
{
    use crate::alloc_guard::sites::TryError;
    proof { reveal(crate::alloc_guard::sites::fmt_part_entries); }
    let entries = match crate::try_parse(bytes) {
        Ok(v) => v,
        Err(TryError::Format(e)) => return (Err(TryError::Format(e)), Err(TryError::Format(e))),
        Err(TryError::OutOfMemory(o)) => {
            return (Err(TryError::OutOfMemory(o)), Err(TryError::Format(crate::icns::Error::BadMagic)));
        },
    };
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    let r = crate::try_combine(bytes, &entries, ci, mi);
    (Ok(entries), r)
}

} // verus!
