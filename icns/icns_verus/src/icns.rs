use vstd::prelude::*;
use alloc::vec::Vec;

verus! {

// Verified implementation of ICNS-CORE-SPEC 1.1 (milestone M2).
// Behaviour is identical to the plain M1 implementation.

/// The input-size limit of R2: `len(bytes) > 67_108_864` is `FileTooLarge`.
pub const MAX_FILE_LEN: u64 = 67_108_864;

/// Maximum number of accepted entries (R6). A 4,097th entry is rejected.
#[doc(hidden)]
pub const MAX_ENTRIES: usize = 4096;

/// Every legacy side is at most 128 (R8), so a decoded channel has at most
/// 128*128 = 16,384 bytes and an RGB output at most 49,152 bytes (R10).
#[doc(hidden)]
pub const MAX_LEGACY_SIDE: usize = 128;

/// The complete, closed error set of R17. `name()` returns the exact
/// case-sensitive spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
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
    IndexOutOfRange,
    UnsupportedType,
    RleTruncated,
    RleOverrun,
    RleTrailingBytes,
    InvalidMaskLength,
    MissingColor,
    MissingMask,
    TypeMismatch,
}

impl Error {
    /// The R17 spelling table, `open` and field-free so it is well-formed
    /// everywhere, including in `alloc_guard::sites`.
    #[doc(hidden)]
    pub open spec fn name_spec(&self) -> &'static str {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedHeader => "TruncatedHeader",
            Error::BadMagic => "BadMagic",
            Error::InvalidFileLength => "InvalidFileLength",
            Error::TruncatedFile => "TruncatedFile",
            Error::TrailingFileBytes => "TrailingFileBytes",
            Error::TruncatedEntryHeader => "TruncatedEntryHeader",
            Error::InvalidEntryLength => "InvalidEntryLength",
            Error::EntryOutOfBounds => "EntryOutOfBounds",
            Error::TooManyEntries => "TooManyEntries",
            Error::IndexOutOfRange => "IndexOutOfRange",
            Error::UnsupportedType => "UnsupportedType",
            Error::RleTruncated => "RleTruncated",
            Error::RleOverrun => "RleOverrun",
            Error::RleTrailingBytes => "RleTrailingBytes",
            Error::InvalidMaskLength => "InvalidMaskLength",
            Error::MissingColor => "MissingColor",
            Error::MissingMask => "MissingMask",
            Error::TypeMismatch => "TypeMismatch",
        }
    }

    /// The `ensures` ties the executable spelling to `name_spec` for every
    /// variant. `name_spec` is the R17 spec-position table; without this bridge
    /// a mutation of one executable spelling would still verify.
    ///
    /// No `#[verifier::when_used_as_spec]`: the attribute disturbs the
    /// crate-wide encoding and makes an unrelated function report a
    /// resource-limit failure (DIB `alloc-wip-1` run 504 onward; see that
    /// project's `STUCK.md`). `name` is called in exec position only (the test
    /// binaries and `alloc_guard::sites::TryError::name`), so this `ensures` is
    /// the spec-position R17 statement.
    pub fn name(&self) -> (s: &'static str)
        ensures s@ == self.name_spec()@,
    {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedHeader => "TruncatedHeader",
            Error::BadMagic => "BadMagic",
            Error::InvalidFileLength => "InvalidFileLength",
            Error::TruncatedFile => "TruncatedFile",
            Error::TrailingFileBytes => "TrailingFileBytes",
            Error::TruncatedEntryHeader => "TruncatedEntryHeader",
            Error::InvalidEntryLength => "InvalidEntryLength",
            Error::EntryOutOfBounds => "EntryOutOfBounds",
            Error::TooManyEntries => "TooManyEntries",
            Error::IndexOutOfRange => "IndexOutOfRange",
            Error::UnsupportedType => "UnsupportedType",
            Error::RleTruncated => "RleTruncated",
            Error::RleOverrun => "RleOverrun",
            Error::RleTrailingBytes => "RleTrailingBytes",
            Error::InvalidMaskLength => "InvalidMaskLength",
            Error::MissingColor => "MissingColor",
            Error::MissingMask => "MissingMask",
            Error::TypeMismatch => "TypeMismatch",
        }
    }
}

/// R1 entry record. Fields are exactly `{type_hex, payload, opaque_candidate,
/// nominal_side}`. `payload` is the half-open range `[start, end)` into the
/// input slice. Decision D2: `type_hex` is the eight lowercase ASCII hex digits
/// of the four type bytes, held in a stack array; the model projects it through
/// `model::hex_str`. `Entry` derives `Copy`, so its `Clone` no longer allocates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub type_hex: [u8; 8],
    pub payload: [u32; 2],
    pub opaque_candidate: bool,
    pub nominal_side: Option<u32>,
}

impl Entry {
    /// The four raw type bytes, recovered from `type_hex`.
    ///
    /// `requires is_hex8` is kept: the field is public, so a caller can build an
    /// `Entry` with arbitrary bytes. Decision D2 drops the `String`: the field is
    /// an array of eight ASCII bytes, indexed directly.
    pub fn type_code(&self) -> (out: [u8; 4])
        requires
            crate::model::is_hex8(crate::model::hex_str(self.type_hex)),
        ensures
            out == crate::model::code_of(crate::model::hex_str(self.type_hex)),
    {
        proof {
            assert(crate::model::hex_str(self.type_hex).len() == 8);
            assert forall|k: int| 0 <= k < 8 implies 0 <= crate::model::hex_val(
                #[trigger] crate::model::hex_str(self.type_hex)[k],
            ) <= 15 by {}
        }
        let mut out = [0u8; 4];
        let mut i: usize = 0;
        while i < 4
            invariant
                i <= 4,
                crate::model::is_hex8(crate::model::hex_str(self.type_hex)),
                forall|k: int| 0 <= k < i ==> #[trigger] out@[k]
                    == crate::model::code_of(crate::model::hex_str(self.type_hex))[k],
            decreases 4 - i,
        {
            proof {
                assert(self.type_hex[2 * i] as char
                    == crate::model::hex_str(self.type_hex)[2 * (i as int)]);
                assert(self.type_hex[2 * i + 1] as char
                    == crate::model::hex_str(self.type_hex)[2 * (i as int) + 1]);
            }
            let hi = hex_nibble(self.type_hex[2 * i]);
            let lo = hex_nibble(self.type_hex[2 * i + 1]);
            proof {
                lemma_nib_char(crate::model::hex_str(self.type_hex)[2 * (i as int)]);
                lemma_nib_char(crate::model::hex_str(self.type_hex)[2 * (i as int) + 1]);
                assert(hi as int == crate::model::hex_val(
                    crate::model::hex_str(self.type_hex)[2 * (i as int)],
                ));
                assert(lo as int == crate::model::hex_val(
                    crate::model::hex_str(self.type_hex)[2 * (i as int) + 1],
                ));
            }
            let cell: u8 = hi * 16u8 + lo;
            proof {
                assert(hi < 16u8);
                assert(lo < 16u8);
                assert(cell as int == (hi as int) * 16 + (lo as int));
                assert(crate::model::code_of(
                    crate::model::hex_str(self.type_hex),
                )[i as int] == cell);
            }
            out[i] = cell;
            i = i + 1;
        }
        out
    }
}

fn hex_nibble(c: u8) -> (v: u8)
    ensures
        v as int == crate::model::hex_nibble_model(c),
{
    if c >= 48u8 && c <= 57u8 {
        c - 48u8
    } else if c >= 97u8 && c <= 102u8 {
        c - 97u8 + 10u8
    } else {
        0u8
    }
}

/// The byte-level nibble equals the model's `hex_val` for a hex character.
proof fn lemma_nib_char(c: char)
    requires
        0 <= crate::model::hex_val(c) <= 15,
    ensures
        crate::model::hex_nibble_model(c as u8) == crate::model::hex_val(c),
{
    crate::model::lemma_hex_nibble_model(c);
}

/// The parse result type of `parse` and the combine result type of `combine`,
/// aliased so `t13_combine_after_parse` can name them as a tuple field type.
pub type ParseRes = Result<Vec<Entry>, Error>;
pub type CombineRes = Result<Decoded, Error>;

/// An opaque, parsed ICNS container: the input slice it was parsed from, and the
/// entry list that parse produced from exactly that slice.
///
/// This is the whole point of the type. The decoders' Verus contracts require
/// `parsed_inv(bytes@, entries@)`, and `parsed_inv` is erased in the compiled
/// artifact. `Entry` has public fields, so an `(bytes, entries)` pair passed as
/// two arguments can be forged in safe Rust: build any `Vec<Entry>` with
/// out-of-range payloads and call the decoder, and the unchecked
/// `&entries[i]` / `&bytes[start..end]` inside it panics or reads the wrong
/// region. `Parsed` has **private** fields and is constructed only by
/// `try_parse`, so safe Rust outside this crate cannot obtain a value that does
/// not satisfy the invariant. The invariant itself is a Verus type invariant
/// (`inv` below), which every method of the type may assume without a
/// `requires` clause, and which `try_parse` must establish at the one
/// construction site.
///
/// `entries()` is a shared slice, so a caller can read an entry (to choose an
/// index) but cannot mutate the list it will be decoded with.
pub struct Parsed<'a> {
    /// The exact slice `try_parse` parsed. Never handed out as `&mut`.
    bytes: &'a [u8],
    /// The entry list `try_parse` produced from `bytes`.
    entries: Vec<Entry>,
}

impl<'a> Parsed<'a> {
    /// The type invariant: the entry list is the parsed form of the bytes it is
    /// stored with. `pub` so a `pub fn`'s contract may name it (Verus refuses a
    /// `pub` contract that refers to a private spec fn); `#[doc(hidden)]` because
    /// it is proof plumbing, not API.
    #[doc(hidden)]
    #[verifier::type_invariant]
    pub closed spec fn inv(&self) -> bool {
        crate::model::parsed_inv(self.bytes@, self.entries@)
    }

    /// The bytes the handle was parsed from. `closed` and `pub` for the same
    /// reason as `inv`: the `pub` method contracts name it, and it does not
    /// reveal the field (only its view, which `bytes()` already hands out).
    #[doc(hidden)]
    pub closed spec fn spec_bytes(&self) -> Seq<u8> {
        self.bytes@
    }

    /// The parsed entry list, as a sequence. `closed` and `pub`, as `spec_bytes`.
    #[doc(hidden)]
    pub closed spec fn spec_entries(&self) -> Seq<Entry> {
        self.entries@
    }

    /// The parsed entry list, read-only. Callers use this to pick an index; it
    /// is the only view of the entries that leaves the crate.
    pub fn entries(&self) -> (r: &[Entry])
        ensures r@ == self.spec_entries(),
    {
        self.entries.as_slice()
    }

    /// The input bytes this handle was parsed from, read-only. PNG-candidate
    /// classification needs the payload ranges, which are offsets into this
    /// slice; it is the same slice `entries()`'s ranges refer to, by
    /// construction.
    pub fn bytes(&self) -> (r: &'a [u8])
        ensures r@ == self.spec_bytes(),
    {
        self.bytes
    }
}

/// A decoded image: `width`, `height` (equal for a successful decode per R1)
/// and the pixel bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// The eight R8 legacy colour/mask codes, with their square side lengths.
/// Order: the four colour types, then the four mask types.
fn legacy_side(code: [u8; 4]) -> (r: Option<(bool, u32)>)
    ensures
        r == crate::model::legacy_side(code),
{
    if code == [105u8, 115, 51, 50] {            // "is32"
        Some((true, 16))
    } else if code == [105u8, 108, 51, 50] {     // "il32"
        Some((true, 32))
    } else if code == [105u8, 104, 51, 50] {     // "ih32"
        Some((true, 48))
    } else if code == [105u8, 116, 51, 50] {     // "it32"
        Some((true, 128))
    } else if code == [115u8, 56, 109, 107] {    // "s8mk"
        Some((false, 16))
    } else if code == [108u8, 56, 109, 107] {    // "l8mk"
        Some((false, 32))
    } else if code == [104u8, 56, 109, 107] {    // "h8mk"
        Some((false, 48))
    } else if code == [116u8, 56, 109, 107] {    // "t8mk"
        Some((false, 128))
    } else {
        None
    }
}

/// Every legacy side is at most 128 (in fact exactly one of 16, 32, 48, 128).
pub(crate) proof fn lemma_legacy_side_le_128(code: [u8; 4])
    ensures
        match crate::model::legacy_side(code) {
            Some((_, s)) => s <= 128,
            None => true,
        },
{
}

/// R9's fourteen-code table: type code -> nominal side.
fn nominal_side_table(code: [u8; 4]) -> (r: Option<u32>)
    ensures
        r == crate::model::nominal_side(code),
{
    if code == [105u8, 99, 112, 52] {            // "icp4"
        Some(16)
    } else if code == [105u8, 99, 112, 53] {     // "icp5"
        Some(32)
    } else if code == [105u8, 99, 112, 54] {     // "icp6"
        Some(64)
    } else if code == [105u8, 99, 48, 55] {      // "ic07"
        Some(128)
    } else if code == [105u8, 99, 48, 56] {      // "ic08"
        Some(256)
    } else if code == [105u8, 99, 48, 57] {      // "ic09"
        Some(512)
    } else if code == [105u8, 99, 49, 48] {      // "ic10"
        Some(1024)
    } else if code == [105u8, 99, 49, 49] {      // "ic11"
        Some(32)
    } else if code == [105u8, 99, 49, 50] {      // "ic12"
        Some(64)
    } else if code == [105u8, 99, 49, 51] {      // "ic13"
        Some(256)
    } else if code == [105u8, 99, 49, 52] {      // "ic14"
        Some(512)
    } else if code == [105u8, 99, 115, 66] {     // "icsB"
        Some(36)
    } else if code == [115u8, 98, 50, 52] {      // "sb24"
        Some(24)
    } else if code == [83u8, 66, 50, 52] {       // "SB24"
        Some(48)
    } else {
        None
    }
}

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
const JP2_SIGNATURE: [u8; 12] = [
    0x00, 0x00, 0x00, 0x0c, 0x6a, 0x50, 0x20, 0x20, 0x0d, 0x0a, 0x87, 0x0a,
];

spec fn png_sig_spec() -> Seq<u8> {
    seq![0x89u8, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]
}

spec fn jp2_sig_spec() -> Seq<u8> {
    seq![0x00u8, 0x00, 0x00, 0x0c, 0x6a, 0x50, 0x20, 0x20, 0x0d, 0x0a, 0x87, 0x0a]
}

fn payload_starts_with(payload: &[u8], sig: &[u8]) -> (r: bool)
    ensures
        r == (sig@.len() <= payload@.len() && payload@.subrange(0, sig@.len() as int) == sig@),
{
    if payload.len() < sig.len() {
        false
    } else {
        let mut i: usize = 0;
        while i < sig.len()
            invariant
                i <= sig@.len(),
                sig@.len() <= payload@.len(),
                forall|k: int| 0 <= k < i ==> payload@[k] == sig@[k],
            decreases sig@.len() - i,
        {
            if payload[i] != sig[i] {
                assert(!(payload@.subrange(0, sig@.len() as int) == sig@)) by {
                    assert(payload@[i as int] != sig@[i as int]);
                }
                return false;
            }
            i = i + 1;
        }
        assert(payload@.subrange(0, sig@.len() as int) == sig@) by {
            assert forall|k: int| 0 <= k < sig@.len() implies payload@[k] == sig@[k] by {}
        }
        true
    }
}

/// R9 classification of one entry's `opaque_candidate` and `nominal_side`.
fn classify(code: [u8; 4], payload: &[u8]) -> (r: (bool, Option<u32>))
    ensures
        r == crate::model::classify(code, payload@),
{
    if legacy_side(code).is_some() {
        // Every R8 entry is never an opaque candidate and has no nominal side.
        return (false, None);
    }
    if nominal_side_table(code).is_some() {
        // Listed type: opaque by type alone, keeps its table-defined side.
        return (true, nominal_side_table(code));
    }
    // Any other type: opaque only when a full signature matches; no nominal side.
    let png = PNG_SIGNATURE;
    let jp2 = JP2_SIGNATURE;
    let sig = payload_starts_with(payload, &png) || payload_starts_with(payload, &jp2);
    proof {
        assert(png@ =~= png_sig_spec());
        assert(jp2@ =~= jp2_sig_spec());
        assert(png_sig_spec() == crate::model::png_signature());
        assert(jp2_sig_spec() == crate::model::jp2_signature());
        assert(sig == crate::model::has_signature(payload@));
    }
    (sig, None)
}

fn be_u32(bytes: &[u8]) -> (r: u32)
    requires
        bytes@.len() >= 4,
    ensures
        r as int
            == (bytes@[0] as int) * 16777216 + (bytes@[1] as int) * 65536 + (bytes@[2] as int) * 256
                + (bytes@[3] as int),
{
    (bytes[0] as u32) * 16777216 + (bytes[1] as u32) * 65536 + (bytes[2] as u32) * 256
        + (bytes[3] as u32)
}

/// The lowercase hex digit for a nibble `0..16`.
fn hex_digit(v: u8) -> (c: char)
    requires
        v < 16,
    ensures
        c == crate::model::hex_digit(v),
{
    if v == 0 { '0' } else if v == 1 { '1' } else if v == 2 { '2' } else if v == 3 { '3' }
    else if v == 4 { '4' } else if v == 5 { '5' } else if v == 6 { '6' } else if v == 7 { '7' }
    else if v == 8 { '8' } else if v == 9 { '9' } else if v == 10 { 'a' } else if v == 11 { 'b' }
    else if v == 12 { 'c' } else if v == 13 { 'd' } else if v == 14 { 'e' } else { 'f' }
}

/// The lowercase hex digit of a nibble, as a byte (decision D2 drops `String`).
fn hex_digit_byte(v: u8) -> (b: u8)
    requires
        v < 16,
    ensures
        b as char == crate::model::hex_digit(v),
        b == crate::model::hex_digit(v) as u8,
{
    let c = hex_digit(v);
    proof {
        crate::model::lemma_hex_digit_u8(v);
        assert((c as u8) as char == c);
    }
    c as u8
}

/// R1's eight-digit lowercase hex encoding of the four type bytes, written into a
/// stack array (decision D2, no allocation). `hex_str(result) == model::hex4(code)`.
fn hex4_bytes(code: [u8; 4]) -> (a: [u8; 8])
    ensures
        crate::model::hex_str(a) == crate::model::hex4(code),
{
    let mut a = [0u8; 8];
    let mut i: usize = 0;
    while i < 4
        invariant
            i <= 4,
            forall|k: int| 0 <= k < i ==> #[trigger] a@[2 * k]
                == crate::model::hex_digit(code[k] / 16) as u8,
            forall|k: int| 0 <= k < i ==> #[trigger] a@[2 * k + 1]
                == crate::model::hex_digit(code[k] % 16) as u8,
        decreases 4 - i,
    {
        a[2 * i] = hex_digit_byte(code[i] / 16);
        a[2 * i + 1] = hex_digit_byte(code[i] % 16);
        proof {
            assert(a@[2 * i as int] == crate::model::hex_digit(code[i as int] / 16) as u8);
            assert(a@[2 * i as int + 1] == crate::model::hex_digit(code[i as int] % 16) as u8);
        }
        i = i + 1;
    }
    proof {
        assert(a@ =~= crate::model::hex4_bytes_spec(code)@) by {
            assert forall|k: int| 0 <= k < 8 implies #[trigger] a@[k]
                == crate::model::hex4_bytes_spec(code)@[k] by {
                if k % 2 == 0 {
                    assert(a@[2 * (k / 2)] == crate::model::hex_digit(code[k / 2] / 16) as u8);
                    assert(2 * (k / 2) == k);
                    reveal(crate::model::hex4_bytes_spec);
                } else {
                    assert(a@[2 * (k / 2) + 1] == crate::model::hex_digit(code[k / 2] % 16) as u8);
                    assert(2 * (k / 2) + 1 == k);
                    reveal(crate::model::hex4_bytes_spec);
                }
            }
        }
        crate::model::lemma_hex_str_hex4_bytes(code);
    }
    a
}

// ---------------------------------------------------------------------------
// alloc-delta: the guard threading and the `try_*` API (SPEC.md §1-§4,
// THEOREMS-ALLOC-DELTA.md §0/§3, A-ICNS-1..8).
// ---------------------------------------------------------------------------

pub use crate::alloc_guard::sites::TryError;
use crate::alloc_guard::{AllocLog, OutOfMemory};
use crate::alloc_guard::sites::AllocSite;

/// `min(4096, (|b|-8)/8)`, the entry-list request count (SPEC.md §4), as a
/// model-side spec function. `reach_p`'s hypothesis `8 <= |b|` makes `|b|-8`
/// non-negative.
#[doc(hidden)]
pub open spec fn entry_count(b: Seq<u8>) -> nat {
    if b.len() >= 8 {
        if (b.len() - 8) / 8 <= 4096 { ((b.len() - 8) / 8) as nat } else { 4096 }
    } else {
        0
    }
}

/// `Reach_P(b)`: R2-R4 pass (SPEC.md §2), so `try_parse` reserves the entry
/// list: length in `[8, 2^26]`, the ICNS magic, and `BE32(b[4..8]) == |b|`.
/// (The last also gives `BE32 >= 8`, R2-R4's remaining check.)
#[doc(hidden)]
pub open spec fn reach_p(b: Seq<u8>) -> bool {
    &&& 8 <= b.len()
    &&& b.len() <= 67108864
    &&& b.subrange(0, 4) == crate::model::icns_magic()
    &&& (crate::model::be32u(b, 4) as u64) == b.len() as u64
}

/// `Req_P(b)` (SPEC.md §4, A-ICNS-1): the one `(EntryList, count)` request iff
/// R2-R4 pass, else `[]`.
#[doc(hidden)]
pub open spec fn req_p(b: Seq<u8>) -> Seq<(AllocSite, nat)> {
    if reach_p(b) {
        seq![(AllocSite::EntryList, entry_count(b))]
    } else {
        Seq::empty()
    }
}

/// `PreAlloc(b)` (THEOREMS-ALLOC-DELTA.md §0): `Req_P` is empty, so no
/// reservation is made and no `OutOfMemory` can appear.
#[doc(hidden)]
pub open spec fn pre_alloc_p(b: Seq<u8>) -> bool {
    req_p(b).len() == 0
}

/// The per-site element maximum (SPEC.md §4, THEOREMS-ALLOC-DELTA.md §0 G
/// clause 3): the largest `k` a request at this site may carry, from the frozen
/// per-site caps. Every ICNS site but `EntryList` holds `u8`, so the element
/// count is the byte count; `EntryList` holds `Entry` records and is capped by
/// `MAX_ROOM` entries.
#[doc(hidden)]
pub open spec fn site_max(site: AllocSite) -> nat {
    match site {
        AllocSite::EntryList => crate::alloc_guard::sites::MAX_ROOM as nat,
        AllocSite::Channel(_) => 16_384,
        AllocSite::RgbOut => 49_152,
        AllocSite::MaskData => 16_384,
        AllocSite::CombineOut => 65_536,
    }
}

/// Every request in `req` is within its own site's maximum — the frozen G
/// clause 3, stated per site rather than against the global `MAX_BYTES`.
#[doc(hidden)]
pub open spec fn req_within_site_max(req: Seq<(AllocSite, nat)>) -> bool {
    forall|j: int| 0 <= j < req.len() ==> req[j].1 <= site_max(req[j].0)
}

/// Clause 1 of `G` for `try_parse` (THEOREMS-ALLOC-DELTA.md §0): a
/// non-`OutOfMemory` result satisfies `parses_to` on `fmt_part_entries(r)`, has
/// exactly the requests of `Req_P`, and every log entry is `Ok`.
#[doc(hidden)]
pub open spec fn g_p_clause1(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::parses_to(b, crate::alloc_guard::sites::fmt_part_entries(r))
            && crate::alloc_guard::req_of_log(l) == req_p(b)
            && crate::alloc_guard::all_ok(l),
    }
}

/// Clause 2 of `G`: an `OutOfMemory` result has a non-empty log whose requests
/// are a prefix of `Req_P`, every entry but the last `Ok`, and the last entry
/// not `Ok` and equal to the returned `(site, elements)`.
#[doc(hidden)]
pub open spec fn g_p_clause2(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_p(b))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G`: the one request is within its site's maximum, proved from
/// the checked input before the guard call. For the `EntryList` site the site
/// maximum is `MAX_ROOM` entries (`site_max(EntryList)`), so this is already
/// the per-site bound and not the global `MAX_BYTES`.
#[doc(hidden)]
pub open spec fn g_p_clause3(b: Seq<u8>) -> bool {
    forall|j: int| 0 <= j < req_p(b).len()
        ==> req_p(b)[j].1 <= crate::alloc_guard::sites::MAX_ROOM
}

/// Theorem shape `G` for `try_parse` (A-ICNS-1), proved on `try_parse_logged`.
#[doc(hidden)]
pub open spec fn g_p(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_p_clause1(b, r, l) && g_p_clause2(b, r, l) && g_p_clause3(b)
}

/// `Prog` for `try_parse`: if every log entry is `Ok`, the result is not
/// `OutOfMemory`, `parses_to` holds, and `req(L) == Req_P`.
#[doc(hidden)]
pub open spec fn prog_p(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_p_concl(b, r, l)
}

#[doc(hidden)]
pub open spec fn prog_p_concl(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && crate::model::parses_to(b, crate::alloc_guard::sites::fmt_part_entries(r))
        && crate::alloc_guard::req_of_log(l) == req_p(b)
}

/// The log-only facts `at_site_log_p` gives: `req(L)` is `Req_P`, which has at
/// most one element, and a failure is the log's last (only) entry.
#[doc(hidden)]
pub open spec fn at_site_log_p(
    b: Seq<u8>,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::req_of_log(l) == req_p(b)
        && (if r_is_oom {
            l.len() >= 1 && !l[l.len() - 1].2
        } else {
            crate::alloc_guard::all_ok(l)
        })
}

/// The executable count `min(4096, (|b|-8)/8)`, from the length alone.
fn entry_count_exec(bytes: &[u8]) -> (n: usize)
    ensures
        n as nat == entry_count(bytes@),
        n <= crate::alloc_guard::sites::MAX_ROOM,
{
    if bytes.len() < 8 {
        proof { reveal(entry_count); }
        return 0;
    }
    let q = (bytes.len() - 8) / 8;
    proof {
        reveal(entry_count);
        assert(q as nat == ((bytes@.len() - 8) / 8) as nat);
    }
    if q <= 4096 {
        assert(q <= crate::alloc_guard::sites::MAX_ROOM);
        q
    } else {
        4096
    }
}

/// The `try_with_room` preconditions `size_of::<Entry>() >= 1` and
/// `n * size_of::<Entry>() <= isize::MAX` for `n <= MAX_ROOM`, from
/// `lemma_entry_layout` (gate F2, x86_64).
proof fn lemma_room_fits(n: usize)
    requires
        n <= crate::alloc_guard::sites::MAX_ROOM,
    ensures
        1 <= size_of::<Entry>(),
        n * size_of::<Entry>() <= isize::MAX,
{
    crate::alloc_guard::sites::lemma_entry_layout();
    assert(n <= 4096);
    assert(n * size_of::<Entry>() <= 4096 * 28) by (nonlinear_arith)
        requires
            n <= 4096,
            size_of::<Entry>() == 28;
    assert(4096 * 28 <= isize::MAX) by (nonlinear_arith)
        requires
            size_of::<Entry>() == 28;
}

/// A-ICNS-2's `push` precondition: at an accepted entry, the entries already
/// pushed are strictly fewer than the room. Every accepted entry advanced
/// `offset` by `length >= 8` from 8, and `offset + 8 <= |b|` holds (the
/// `TruncatedEntryHeader`/bounds checks), so `k <= (|b|-16)/8 < (|b|-8)/8`;
/// combined with the `TooManyEntries` check (`k < 4096`),
/// `k < min(4096, (|b|-8)/8) = count`.
proof fn lemma_room_headroom(bytes: Seq<u8>, offset: u64, k: nat, count: usize)
    requires
        reach_p(bytes),
        count as nat == entry_count(bytes),
        k < 4096,
        8 + 8 * (k as int) <= offset as int,
        (offset as int) + 8 <= bytes.len(),
    ensures
        k < count,
{
    reveal(entry_count);
    assert(8 <= bytes.len());
    assert(k as int * 8 <= offset as int - 8);
    let q = ((bytes.len() - 8) / 8) as nat;
    assert(k as int * 8 <= bytes.len() as int - 16) by (nonlinear_arith)
        requires
            k as int * 8 <= offset as int - 8,
            (offset as int) + 8 <= bytes.len() as int;
    assert(k < q) by (nonlinear_arith)
        requires
            k as int * 8 <= bytes.len() as int - 16,
            q == ((bytes.len() - 8) / 8) as nat,
            8 <= bytes.len();
    assert((bytes.len() - 8) / 8 <= 4096 ==> count as nat == q);
    assert(!((bytes.len() - 8) / 8 <= 4096) ==> count as nat == 4096);
    if (bytes.len() - 8) / 8 <= 4096 {
        assert(count as nat == q);
        assert(k < count);
    } else {
        assert(count as nat == 4096);
        assert(k < 4096);
        assert(k < count);
    }
}

/// Clause 3's content for `try_parse`: the one request is `count <= MAX_ROOM`.
proof fn lemma_req_p_clause3(b: Seq<u8>)
    ensures g_p_clause3(b),
{
    reveal(req_p);
    reveal(g_p_clause3);
    reveal(entry_count);
}

/// The one-request shape: `Req_P` has at most one element.
proof fn lemma_req_p_len(b: Seq<u8>)
    ensures req_p(b).len() <= 1,
{
    reveal(req_p);
}

/// The log-only facts of `at_site_log_p`, in the form the entry point's proof
/// blocks consume.
proof fn lemma_at_site_log_p_parts(b: Seq<u8>, r_is_oom: bool, l: Seq<(AllocSite, nat, bool)>)
    requires at_site_log_p(b, r_is_oom, l),
    ensures
        crate::alloc_guard::req_of_log(l) == req_p(b),
        req_p(b).len() <= 1,
        l.len() <= 1,
        r_is_oom ==> (l.len() == 1 && !l[0].2),
        !r_is_oom ==> crate::alloc_guard::all_ok(l),
{
    reveal(at_site_log_p);
    reveal(req_p);
    reveal(entry_count);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_p(b));
}

/// The pre-reservation `at_site_log_p` (a header error: R2-R4 failed).
proof fn lemma_at_site_log_p_prealloc(b: Seq<u8>, l: Seq<(AllocSite, nat, bool)>)
    requires
        l.len() == 0,
        !reach_p(b),
    ensures at_site_log_p(b, false, l),
{
    reveal(at_site_log_p);
    reveal(req_p);
    reveal(entry_count);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_p(b));
}

/// The `OutOfMemory` exit of the entry-list reservation.
proof fn lemma_at_site_log_p_oom(
    b: Seq<u8>,
    o: OutOfMemory,
    count: usize,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        reach_p(b),
        count as nat == entry_count(b),
        o.site_spec() == AllocSite::EntryList,
        o.elements_spec() == count,
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::EntryList, count as nat, false)),
    ensures
        at_site_log_p(b, true, l),
        (o.site_spec(), o.elements_spec() as nat) == req_p(b)[0],
{
    reveal(at_site_log_p);
    reveal(req_p);
    reveal(entry_count);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_p(b));
}

/// The `Ok` exit of the entry-list reservation, and every later format-error
/// return inside the loop: `req(L)` is exactly `Req_P` and every entry is `Ok`.
proof fn lemma_at_site_log_p_after_ok(b: Seq<u8>, count: usize, l: Seq<(AllocSite, nat, bool)>)
    requires
        reach_p(b),
        count as nat == entry_count(b),
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::EntryList, count as nat, true)),
    ensures at_site_log_p(b, false, l),
{
    reveal(at_site_log_p);
    reveal(req_p);
    reveal(entry_count);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_p(b));
}

/// R2-R7, R15: parse an ICNS container into entries, through the sealed
/// reservation log.
///
/// The frozen postcondition is `parses_to` on `fmt_part_entries(r)`: the result
/// equals the total reference model `MP` as an exact `Result` — the first error
/// and all four fields of every entry — for every non-`OutOfMemory` result.
///
/// The one reservation is the entry list, `try_with_room(EntryList, count)` with
/// `count = min(4096, (|b|-8)/8)`, made after R2-R4 and before the loop
/// (A-ICNS-1/2). The loop fills the `Reserved<Entry>` by `push`; `len < room`
/// follows from the `TooManyEntries` check and the per-entry bounds check, never
/// from a regrowth.
fn parse_tracked(bytes: &[u8], Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<Vec<Entry>, TryError>)
    requires
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::parses_to(bytes@, crate::alloc_guard::sites::fmt_part_entries(r)),
        at_site_log_p(bytes@, matches!(r, Err(TryError::OutOfMemory(_))), final(log).view()),
        pre_alloc_p(bytes@) ==> final(log).view().len() == 0,
        match r {
            Err(TryError::OutOfMemory(o)) =>
                (o.site_spec(), o.elements_spec() as nat) == req_p(bytes@)[0],
            _ => true,
        },
{
    // R2: input length first, without reading bytes.
    if bytes.len() as u64 > MAX_FILE_LEN {
        proof {
            crate::model::lemma_mp_too_large(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::FileTooLarge);
            reveal(pre_alloc_p);
            reveal(reach_p);
            assert(!reach_p(bytes@));
            lemma_at_site_log_p_prealloc(bytes@, log.view());
        }
        return Err(TryError::Format(Error::FileTooLarge));
    }
    if bytes.len() < 8 {
        proof {
            crate::model::lemma_mp_short(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::TruncatedHeader);
            reveal(pre_alloc_p);
            reveal(reach_p);
            assert(!reach_p(bytes@));
            lemma_at_site_log_p_prealloc(bytes@, log.view());
        }
        return Err(TryError::Format(Error::TruncatedHeader));
    }
    // R3: magic and declared length.
    if bytes[0] != 105 || bytes[1] != 99 || bytes[2] != 110 || bytes[3] != 115 {
        proof {
            reveal(crate::model::icns_magic);
            assert(bytes@.subrange(0, 4) != crate::model::icns_magic()) by {
                if bytes@.subrange(0, 4) == crate::model::icns_magic() {
                    assert(bytes@.subrange(0, 4)[0] == bytes@[0]);
                    assert(bytes@.subrange(0, 4)[1] == bytes@[1]);
                    assert(bytes@.subrange(0, 4)[2] == bytes@[2]);
                    assert(bytes@.subrange(0, 4)[3] == bytes@[3]);
                }
            }
            crate::model::lemma_mp_bad_magic(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::BadMagic);
            reveal(pre_alloc_p);
            reveal(reach_p);
            assert(!reach_p(bytes@));
            lemma_at_site_log_p_prealloc(bytes@, log.view());
        }
        return Err(TryError::Format(Error::BadMagic));
    }
    proof {
        reveal(crate::model::icns_magic);
        assert(bytes@.subrange(0, 4) =~= crate::model::icns_magic()) by {
            assert forall|i: int| 0 <= i < 4 implies
                bytes@.subrange(0, 4)[i] == crate::model::icns_magic()[i] by {
                assert(bytes@.subrange(0, 4)[i] == bytes@[i]);
            }
        }
    }
    let declared = be_u32(&bytes[4..8]) as u64;
    proof {
        crate::model::lemma_be32u_subrange(bytes@, 4, bytes@.subrange(4, 8));
        assert(crate::model::be32u(bytes@, 4) as u64 == declared);
    }
    if declared < 8 {
        proof {
            crate::model::lemma_mp_invalid_len(bytes@);
            crate::model::lemma_parses_to_of_mp_err(
                bytes@, crate::model::MpError::InvalidFileLength);
            reveal(pre_alloc_p);
            reveal(reach_p);
            assert(!reach_p(bytes@));
            lemma_at_site_log_p_prealloc(bytes@, log.view());
        }
        return Err(TryError::Format(Error::InvalidFileLength));
    }
    // R4: declared length must equal len(bytes).
    if declared > bytes.len() as u64 {
        proof {
            crate::model::lemma_mp_truncated_file(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::TruncatedFile);
            reveal(pre_alloc_p);
            reveal(reach_p);
            assert(!reach_p(bytes@));
            lemma_at_site_log_p_prealloc(bytes@, log.view());
        }
        return Err(TryError::Format(Error::TruncatedFile));
    }
    if declared < bytes.len() as u64 {
        proof {
            crate::model::lemma_mp_trailing(bytes@);
            crate::model::lemma_parses_to_of_mp_err(
                bytes@, crate::model::MpError::TrailingFileBytes);
            reveal(pre_alloc_p);
            reveal(reach_p);
            assert(!reach_p(bytes@));
            lemma_at_site_log_p_prealloc(bytes@, log.view());
        }
        return Err(TryError::Format(Error::TrailingFileBytes));
    }

    // R2-R4 pass: `reach_p` holds, so `req_p(bytes) = [(EntryList, count)]`. The
    // count is computed from checked values before the guard call (A-ICNS-1 and
    // clause 3's "before the guard call").
    let count = entry_count_exec(bytes);
    proof {
        reveal(reach_p);
        reveal(crate::model::icns_magic);
        assert(bytes@.subrange(0, 4) =~= crate::model::icns_magic());
        assert(reach_p(bytes@));
        assert(count as nat == entry_count(bytes@));
        assert(count <= crate::alloc_guard::sites::MAX_ROOM);
        lemma_room_fits(count);
        lemma_req_p_clause3(bytes@);
    }
    let mut res = match crate::alloc_guard::sites::try_with_room::<Entry>(
        AllocSite::EntryList,
        count,
        Tracked(log),
    ) {
        Ok(r) => r,
        Err(o) => {
            proof {
                assert(o.site_spec() == AllocSite::EntryList);
                assert(o.elements_spec() == count);
                assert((o.site_spec(), o.elements_spec() as nat) == req_p(bytes@)[0]);
                lemma_at_site_log_p_oom(bytes@, o, count, log.view());
            }
            return Err(TryError::OutOfMemory(o));
        },
    };

    // R5, R6, R15: scan entries in file order. The ghost `os` records the
    // offsets the executable loop has accepted; `walk_pref` says `walk` from 8
    // emits `os` and then continues from `offset`. The entries go into `res`, a
    // `Reserved<Entry>` whose room is fixed by the reservation (A-ICNS-2).
    let ghost post_log: Seq<(AllocSite, nat, bool)> = log.view();
    let mut offset: u64 = 8;
    let mut k: usize = 0;
    let ghost mut os: Seq<u32> = Seq::empty();
    proof {
        crate::model::lemma_pref_init(bytes@, declared);
        assert(crate::model::walk_pref(bytes@, declared, 8u64, Seq::<u32>::empty()));
        // The reservation succeeded, so the log is frozen at `post_log` for the
        // rest of the function: the loop makes no further reservation.
        lemma_at_site_log_p_after_ok(bytes@, count, post_log);
        reveal(pre_alloc_p);
        reveal(req_p);
        assert(req_p(bytes@).len() == 1);
        assert(!pre_alloc_p(bytes@));
    }
    while offset < declared
        invariant
            declared == bytes@.len(),
            bytes@.subrange(0, 4) == crate::model::icns_magic(),
            crate::model::be32u(bytes@, 4) as u64 == declared,
            bytes@.len() <= MAX_FILE_LEN,
            8 <= offset <= declared,
            count as nat == entry_count(bytes@),
            res.room() == count as nat,
            k as nat == res.view().len(),
            k as nat == os.len(),
            8 + 8 * (k as int) <= offset as int,
            crate::model::entries_match(bytes@, res.view(), os),
            crate::model::walk_pref(bytes@, declared, offset, os),
            log.view() == post_log,
            post_log == Seq::<(AllocSite, nat, bool)>::empty()
                .push((AllocSite::EntryList, count as nat, true)),
            at_site_log_p(bytes@, false, post_log),
            !pre_alloc_p(bytes@),
        decreases declared - offset,
    {
        let remaining = declared - offset;
        if remaining < 8 {
            proof {
                crate::model::lemma_pref_error_th(bytes@, declared, offset, os);
                crate::model::lemma_mp_eq_mp_at(bytes@);
                crate::model::lemma_mp_at_walk(bytes@);
                assert(crate::model::mp(bytes@)
                    == Err(crate::model::MpError::TruncatedEntryHeader));
                crate::model::lemma_parses_to_of_mp_err(
                    bytes@, crate::model::MpError::TruncatedEntryHeader);
                lemma_at_site_log_p_after_ok(bytes@, count, post_log);
            }
            return Err(TryError::Format(Error::TruncatedEntryHeader));
        }
        let off = offset as usize;
        let code = [bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]];
        let length = be_u32(&bytes[off + 4..off + 8]) as u64;
        proof {
            // Link the four bytes read above to the model's `code4`.
            assert(code@ =~= bytes@.subrange(off as int, off as int + 4)) by {
                assert(code@[0] == bytes@[off as int]);
                assert(code@[1] == bytes@[off as int + 1]);
                assert(code@[2] == bytes@[off as int + 2]);
                assert(code@[3] == bytes@[off as int + 3]);
            }
            crate::model::lemma_code4_of_subrange(bytes@, off as int, code);
            assert(offset as int == off as int);
            assert(bytes@.len() == declared as int);
            crate::model::lemma_be32u_subrange(bytes@, offset as int + 4,
                bytes@.subrange(offset as int + 4, offset as int + 8));
            assert(crate::model::be32u(bytes@, offset as int + 4) as u64 == length);
        }
        if length < 8 {
            proof {
                crate::model::lemma_pref_error_iel(bytes@, declared, offset, os, length);
                crate::model::lemma_mp_eq_mp_at(bytes@);
                crate::model::lemma_mp_at_walk(bytes@);
                assert(crate::model::mp(bytes@) == Err(crate::model::MpError::InvalidEntryLength));
                crate::model::lemma_parses_to_of_mp_err(
                    bytes@, crate::model::MpError::InvalidEntryLength);
                lemma_at_site_log_p_after_ok(bytes@, count, post_log);
            }
            return Err(TryError::Format(Error::InvalidEntryLength));
        }
        // measured from the first byte of this entry's header
        if length > remaining {
            proof {
                crate::model::lemma_pref_error_oob(bytes@, declared, offset, os, length);
                crate::model::lemma_mp_eq_mp_at(bytes@);
                crate::model::lemma_mp_at_walk(bytes@);
                assert(crate::model::mp(bytes@) == Err(crate::model::MpError::EntryOutOfBounds));
                crate::model::lemma_parses_to_of_mp_err(
                    bytes@, crate::model::MpError::EntryOutOfBounds);
                lemma_at_site_log_p_after_ok(bytes@, count, post_log);
            }
            return Err(TryError::Format(Error::EntryOutOfBounds));
        }
        if k >= MAX_ENTRIES {
            proof {
                crate::model::lemma_pref_error_tme(bytes@, declared, offset, os, length);
                crate::model::lemma_mp_eq_mp_at(bytes@);
                crate::model::lemma_mp_at_walk(bytes@);
                assert(crate::model::mp(bytes@) == Err(crate::model::MpError::TooManyEntries));
                crate::model::lemma_parses_to_of_mp_err(
                    bytes@, crate::model::MpError::TooManyEntries);
                lemma_at_site_log_p_after_ok(bytes@, count, post_log);
            }
            return Err(TryError::Format(Error::TooManyEntries));
        }
        let end = offset + length; // <= declared <= len(bytes)
        let pstart = offset + 8;
        let payload = &bytes[pstart as usize..end as usize];
        let (opaque_candidate, nominal_side) = classify(code, payload);
        let a = hex4_bytes(code);
        let entry = Entry {
            type_hex: a,
            payload: [pstart as u32, end as u32],
            opaque_candidate,
            nominal_side,
        };
        proof {
            // R1: the offsets fit in `u32` without truncation, because the R2 input
            // cap bounds every offset at 67,108,864 < 2^32.
            assert(pstart <= declared);
            assert(end <= declared);
            assert(declared <= MAX_FILE_LEN);
            assert(declared <= 0xffff_ffffu64);
            assert(offset as u32 as int == offset as int);
            assert(pstart as u32 as int == pstart);
            assert(end as u32 as int == end);
            // R5 payload bytes of this entry equal the slice taken above.
            assert(payload@ == bytes@.subrange(pstart as int, end as int));
            // The record is exactly the model's `entry_at` at this offset.
            assert(crate::model::hex_str(entry.type_hex)
                == crate::model::hex4(crate::model::code4(bytes@, offset as int)));
            assert(entry.payload[0] == (offset as int + 8) as u32);
            assert(entry.payload[1] == (offset as int
                + (crate::model::be32u(bytes@, offset as int + 4) as int)) as u32);
            assert(entry.opaque_candidate == crate::model::classify(
                crate::model::code4(bytes@, offset as int),
                bytes@.subrange(offset as int + 8,
                    offset as int + (crate::model::be32u(bytes@, offset as int + 4) as int))).0);
            assert(entry.nominal_side == crate::model::classify(
                crate::model::code4(bytes@, offset as int),
                bytes@.subrange(offset as int + 8,
                    offset as int + (crate::model::be32u(bytes@, offset as int + 4) as int))).1);
            crate::model::lemma_entry_matches_at(bytes@, offset as int, entry);
            // T6's per-entry invariant, from the same facts the parser checked.
            crate::model::lemma_entry_inv_from_at(bytes@, offset as int, entry);
        }
        // A-ICNS-2: the `push` precondition `len < room`, from the two checks.
        proof {
            lemma_room_headroom(bytes@, offset, k as nat, count);
        }
        // Hand the ghost offset list and the executable record to the model.
        proof {
            crate::model::lemma_entries_match_push(bytes@, res.view(), os, entry, offset as u32);
            crate::model::lemma_pref_step(bytes@, declared, offset, os);
        }
        let ghost off_u32: u32 = offset as u32;
        res.push(entry);
        k = k + 1;
        offset = end;
        proof {
            os = os.push(off_u32);
        }
        proof {
            assert(k as nat == res.view().len());
            assert(k as nat == os.len());
            assert(crate::model::walk_pref(bytes@, declared, offset, os));
            assert(crate::model::entries_match(bytes@, res.view(), os));
            assert(8 + 8 * (k as int) <= offset as int) by (nonlinear_arith)
                requires
                    8 + 8 * ((k as int) - 1) <= (offset as int - length as int),
                    length >= 8,
                    8 <= offset as int;
        }
    }
    proof {
        assert(crate::model::walk_pref(bytes@, declared, declared, os));
        crate::model::lemma_pref_done(bytes@, declared, os);
        assert(crate::model::walk(bytes@, declared, 8, 0) == Ok(os));
    }
    let entries = res.into_vec();
    proof {
        assert(entries@.len() == os.len());
        assert(crate::model::entries_match(bytes@, entries@, os));
        crate::model::lemma_parses_to_ok(bytes@, entries@, os);
        crate::model::lemma_parses_to_ok_view(bytes@, entries);
        lemma_at_site_log_p_after_ok(bytes@, count, post_log);
    }
    Ok(entries)
}

/// The logged parse entry point (A-ICNS-1): creates the one log, runs the one
/// entry-list reservation, and returns the log. `Eq_f` is `parses_to`.
pub(crate) fn try_parse_logged(bytes: &[u8])
    -> (r: (Result<Vec<Entry>, TryError>, Tracked<AllocLog>))
    ensures
        g_p(bytes@, r.0, r.1@.view()),
        prog_p(bytes@, r.0, r.1@.view()),
        crate::alloc_guard::sites::log_tied_entries(r.0, r.1@.view()),
        pre_alloc_p(bytes@) ==> !matches!(r.0, Err(TryError::OutOfMemory(_)))
            && crate::model::parses_to(bytes@, crate::alloc_guard::sites::fmt_part_entries(r.0))
            && r.1@.view().len() == 0,
        match r.0 {
            Err(TryError::OutOfMemory(o)) =>
                (o.site_spec(), o.elements_spec() as nat) == req_p(bytes@)[0],
            _ => true,
        },
{
    let tracked mut log = crate::alloc_guard::new_log();
    let result = parse_tracked(bytes, Tracked(&mut log));
    proof {
        reveal(g_p);
        reveal(g_p_clause1);
        reveal(g_p_clause2);
        reveal(g_p_clause3);
        reveal(prog_p);
        reveal(prog_p_concl);
        reveal(crate::alloc_guard::sites::log_tied_entries);
        reveal(pre_alloc_p);
        reveal(req_p);
        reveal(at_site_log_p);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        lemma_req_p_len(bytes@);
        lemma_req_p_clause3(bytes@);
    }
    (result, Tracked(log))
}

/// The parse entry point and the **only** constructor of `Parsed`: it runs
/// `try_parse_logged` and discards the log (decision D1: the frozen name `parse`
/// is removed). `OutOfMemory` names the failed
/// `(EntryList, min(4096, (|b|-8)/8))` reservation.
///
/// The `Ok` arm is the one place a `Parsed` value is built, so it is the one
/// place the type invariant is established: the entries are `parses_to`'s model
/// list for `bytes`, and `lemma_parse_ok_gives_p` + `lemma_p_exec_inv` turn that
/// into `parsed_inv(bytes@, entries@)`. Safe Rust outside this crate cannot
/// construct `Parsed` at all (private fields, no other constructor), so every
/// decoder call made through the handle carries the invariant.
///
/// Every non-`OutOfMemory` result carries the T7 relation: `Ok` gives
/// `parses_to_s` on the handle's entry view (the clause above), and
/// `Err(Format(e))` gives `parses_to_s(bytes@, Err(e))`, so a rejection names
/// the exact first format error and no accepted input can return one. Both are
/// the `fmt_part_entries` projection of the logged internal result
/// `try_parse_logged` proves, carried across the handle boundary.
pub fn try_parse<'a>(bytes: &'a [u8]) -> (r: Result<Parsed<'a>, TryError>)
    ensures
        match r {
            Ok(p) => {
                &&& p.spec_bytes() == bytes@
                &&& crate::model::parses_to_s(bytes@, Ok(p.spec_entries()))
            },
            Err(TryError::Format(e)) => crate::model::parses_to_s(bytes@, Err(e)),
            Err(TryError::OutOfMemory(_)) => true,
        },
        pre_alloc_p(bytes@) ==> !matches!(r, Err(TryError::OutOfMemory(_))),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                o.site_spec() == AllocSite::EntryList
                && o.elements_spec() as nat == entry_count(bytes@),
            _ => true,
        },
{
    let res = try_parse_logged(bytes);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part_entries);
        reveal(g_p);
        reveal(g_p_clause1);
        reveal(req_p);
        reveal(pre_alloc_p);
        reveal(at_site_log_p);
        if let Err(TryError::OutOfMemory(o)) = res.0 {
            assert((o.site_spec(), o.elements_spec() as nat) == req_p(bytes@)[0]);
            assert(req_p(bytes@)[0] == (AllocSite::EntryList, entry_count(bytes@)));
        }
    }
    match res.0 {
        Ok(entries) => {
            proof {
                crate::model::lemma_parse_ok_gives_p(bytes@, entries);
                crate::model::lemma_p_exec_inv(bytes@, entries@);
                crate::model::lemma_parses_to_ok_view(bytes@, entries);
            }
            Ok(Parsed { bytes, entries })
        }
        Err(TryError::Format(e)) => {
            proof {
                // The logged internal result satisfies `g_p_clause1`, which for a
                // non-`OutOfMemory` result is `parses_to` on `fmt_part_entries`;
                // at `Format(e)` that projection is `Err(e)`. `lemma_parses_to_err_view`
                // lifts it to the view form the handle's contract states, so the
                // rejection obligation survives the public boundary.
                assert(crate::model::parses_to(bytes@, Err(e)));
                crate::model::lemma_parses_to_err_view(bytes@, e);
            }
            Err(TryError::Format(e))
        }
        Err(TryError::OutOfMemory(o)) => Err(TryError::OutOfMemory(o)),
    }
}

/// Copy `count` channel bytes from `payload` starting at `cursor` into
/// `channel` at offset `produced`: a literal run copies `count` bytes, a repeat
/// run copies one byte `count` times. Requires `produced + count <= channel.len()`
/// and enough payload bytes. The channel bytes `produced..produced+count` are
/// exactly `rle_bytes`, the model's packet output.
fn rle_copy(
    channel: &mut Vec<u8>,
    payload: &[u8],
    cursor: usize,
    produced: usize,
    count: usize,
    literal: bool,
)
    requires
        produced + count <= old(channel)@.len(),
        literal ==> cursor + count <= payload@.len(),
        !literal ==> cursor + 1 <= payload@.len(),
    ensures
        final(channel)@ == crate::model::write_seq(
            old(channel)@,
            produced as int,
            crate::model::rle_bytes(payload@, cursor as int, count as int, literal),
        ),
{
    let base = channel.len();
    let plen = payload.len();
    let ghost gbase: Seq<u8> = channel@;
    let _ = (base, plen);
    let mut j: usize = 0;
    while j < count
        invariant
            j <= count,
            channel.len() == base,
            base == gbase.len(),
            produced + count <= base,
            literal ==> cursor + count <= plen,
            !literal ==> cursor + 1 <= plen,
            plen == payload@.len(),
            channel@ == crate::model::write_seq(
                gbase,
                produced as int,
                crate::model::rle_bytes(payload@, cursor as int, j as int, literal),
            ),
        decreases count - j,
    {
        let idx = produced + j;
        let src = if literal {
            cursor + j
        } else {
            cursor
        };
        proof {
            assert(src < plen);
            crate::model::lemma_rle_bytes_push(payload@, cursor as int, j as int, literal);
        }
        channel[idx] = payload[src];
        proof {
            assert(payload@[src as int] == crate::model::rle_bytes(
                payload@,
                cursor as int,
                j as int + 1,
                literal,
            )[j as int]);
            crate::model::lemma_write_seq_step(
                gbase,
                produced as int,
                crate::model::rle_bytes(payload@, cursor as int, j as int, literal),
                payload@[src as int],
            );
        }
        j += 1;
    }
}

/// `Req_R(n)`: the three channel reservations, in order (SPEC.md §4, A-ICNS-3).
#[doc(hidden)]
pub open spec fn req_r(n: nat) -> Seq<(AllocSite, nat)> {
    seq![
        (AllocSite::Channel(0u8), n),
        (AllocSite::Channel(1u8), n),
        (AllocSite::Channel(2u8), n),
    ]
}

/// Clause 1 of `G` for `try_rle`: a non-`OutOfMemory` result is `rle_result_eq`
/// on `fmt_part_channels(r)`, has exactly `Req_R`'s requests, every entry `Ok`.
#[doc(hidden)]
pub open spec fn g_r_clause1(
    b: Seq<u8>,
    n: nat,
    r: Result<[Vec<u8>; 3], TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::rle_result_eq(
                crate::alloc_guard::sites::fmt_part_channels(r),
                crate::model::mr(b, n as int),
            )
            && crate::alloc_guard::req_of_log(l) == req_r(n)
            && crate::alloc_guard::all_ok(l),
    }
}

/// Clause 2 of `G` for `try_rle`: the `OutOfMemory` log shape. Here `Req_R` has
/// three elements, so the requests must be a genuine prefix.
#[doc(hidden)]
pub open spec fn g_r_clause2(
    b: Seq<u8>,
    n: nat,
    r: Result<[Vec<u8>; 3], TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_r(n))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G` for `try_rle`: each request is `n <= 16,384`, which is
/// `site_max(Channel(_))` (frozen per-site bound, SPEC.md §4), not the global
/// `MAX_BYTES`.
#[doc(hidden)]
pub open spec fn g_r_clause3(n: nat) -> bool {
    req_within_site_max(req_r(n))
}

#[doc(hidden)]
pub open spec fn g_r(
    b: Seq<u8>,
    n: nat,
    r: Result<[Vec<u8>; 3], TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_r_clause1(b, n, r, l) && g_r_clause2(b, n, r, l) && g_r_clause3(n)
}

#[doc(hidden)]
pub open spec fn prog_r(
    b: Seq<u8>,
    n: nat,
    r: Result<[Vec<u8>; 3], TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_r_concl(b, n, r, l)
}

#[doc(hidden)]
pub open spec fn prog_r_concl(
    b: Seq<u8>,
    n: nat,
    r: Result<[Vec<u8>; 3], TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && crate::model::rle_result_eq(
            crate::alloc_guard::sites::fmt_part_channels(r),
            crate::model::mr(b, n as int),
        )
        && crate::alloc_guard::req_of_log(l) == req_r(n)
}

/// The log-only facts `at_site_log_r` gives: `req(L)` is a prefix of `Req_R`, so
/// a failure is `L`'s last entry, and an all-`Ok` `L` of length three is `Req_R`.
#[doc(hidden)]
pub open spec fn at_site_log_r(
    n: nat,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    if r_is_oom {
        crate::alloc_guard::req_of_log(l).is_prefix_of(req_r(n))
            && l.len() >= 1
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
    } else {
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n, true))
            .push((AllocSite::Channel(1u8), n, true))
            .push((AllocSite::Channel(2u8), n, true))
            && crate::alloc_guard::all_ok(l)
    }
}

/// `Req_R` has exactly three elements.
proof fn lemma_req_r_len(n: nat)
    ensures req_r(n).len() == 3,
{
    reveal(req_r);
}

/// Clause 3's content for `try_rle`: every request is at a `Channel` site with
/// `n <= 16,384 = site_max(Channel(_))`.
proof fn lemma_req_r_clause3(n: nat)
    requires
        n <= 16384,
    ensures g_r_clause3(n),
{
    reveal(g_r_clause3);
    lemma_req_r_elements_bound(n);
}

/// The first reservation failed: `L = [(Channel(0), n, false)]`.
proof fn lemma_at_site_log_r_oom1(n: usize, o: OutOfMemory, l: Seq<(AllocSite, nat, bool)>)
    requires
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::Channel(0u8), n as nat, false)),
        o.site_spec() == AllocSite::Channel(0u8),
        o.elements_spec() == n,
    ensures at_site_log_r(n as nat, true, l),
{
    reveal(at_site_log_r);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_r(n as nat)));
    assert forall|j: int| 0 <= j < l.len() - 1 implies #[trigger] l[j].2 by {};
}

/// The second reservation failed: `L = [(Channel(0), n, true), (Channel(1), n, false)]`.
proof fn lemma_at_site_log_r_oom2(n: usize, o: OutOfMemory, l: Seq<(AllocSite, nat, bool)>)
    requires
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n as nat, true))
            .push((AllocSite::Channel(1u8), n as nat, false)),
        o.site_spec() == AllocSite::Channel(1u8),
        o.elements_spec() == n,
    ensures at_site_log_r(n as nat, true, l),
{
    reveal(at_site_log_r);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_r(n as nat)));
    assert forall|j: int| 0 <= j < l.len() - 1 implies #[trigger] l[j].2 by {};
}

/// The third reservation failed:
/// `L = [(Channel 0, n, true), (Channel 1, n, true), (Channel 2, n, false)]`.
proof fn lemma_at_site_log_r_oom3(n: usize, o: OutOfMemory, l: Seq<(AllocSite, nat, bool)>)
    requires
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n as nat, true))
            .push((AllocSite::Channel(1u8), n as nat, true))
            .push((AllocSite::Channel(2u8), n as nat, false)),
        o.site_spec() == AllocSite::Channel(2u8),
        o.elements_spec() == n,
    ensures at_site_log_r(n as nat, true, l),
{
    reveal(at_site_log_r);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_r(n as nat)));
    assert forall|j: int| 0 <= j < l.len() - 1 implies #[trigger] l[j].2 by {};
}

/// All three reservations succeeded: `L = Req_R`, every entry `Ok`.
proof fn lemma_at_site_log_r_after_ok(n: usize, l: Seq<(AllocSite, nat, bool)>)
    requires
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n as nat, true))
            .push((AllocSite::Channel(1u8), n as nat, true))
            .push((AllocSite::Channel(2u8), n as nat, true)),
    ensures at_site_log_r(n as nat, false, l),
{
    reveal(at_site_log_r);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_r(n as nat));
}

/// R12: decode one payload into three planar channels of `n` bytes each, through
/// the sealed reservation log. R11's optional four-byte prefix removal is applied
/// by the caller. On a non-`OutOfMemory` result it equals `MR(payload, n)` exactly:
/// the three channels or the first error (A-ICNS-3).
///
/// The three channel buffers are `try_zeroed(Channel(k), n)`, three straight-line
/// calls in order, before the loop. If `Channel(0)` or `Channel(1)` fails, the
/// earlier vectors are dropped on return by ordinary ownership (no leak).
fn rle_decode_tracked(payload: &[u8], n: usize, Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<[Vec<u8>; 3], TryError>)
    requires
        n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
        1 <= n,
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rle_result_eq(
                crate::alloc_guard::sites::fmt_part_channels(r),
                crate::model::mr(payload@, n as int),
            ),
        match r {
            Ok(ch) => {
                &&& ch@.len() == 3
                &&& ch@[0]@.len() == n
                &&& ch@[1]@.len() == n
                &&& ch@[2]@.len() == n
            },
            Err(_) => true,
        },
        at_site_log_r(n as nat, matches!(r, Err(TryError::OutOfMemory(_))), final(log).view()),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                o.site_spec() == final(log).view().last().0
                && o.elements_spec() == n
                && final(log).view().last().1 == n as nat,
            _ => true,
        },
{
    // A-ICNS-3: three straight-line reservations, in order.
    let mut c0 = match crate::alloc_guard::try_zeroed(
        AllocSite::Channel(0u8),
        n,
        Tracked(log),
    ) {
        Ok(v) => v,
        Err(o) => {
            proof { lemma_at_site_log_r_oom1(n, o, log.view()); }
            return Err(TryError::OutOfMemory(o));
        },
    };
    let mut c1 = match crate::alloc_guard::try_zeroed(
        AllocSite::Channel(1u8),
        n,
        Tracked(log),
    ) {
        Ok(v) => v,
        Err(o) => {
            proof { lemma_at_site_log_r_oom2(n, o, log.view()); }
            return Err(TryError::OutOfMemory(o));
        },
    };
    let mut c2 = match crate::alloc_guard::try_zeroed(
        AllocSite::Channel(2u8),
        n,
        Tracked(log),
    ) {
        Ok(v) => v,
        Err(o) => {
            proof { lemma_at_site_log_r_oom3(n, o, log.view()); }
            return Err(TryError::OutOfMemory(o));
        },
    };
    proof { reveal(crate::alloc_guard::sites::fmt_part_channels); }
    let mut channels: [Vec<u8>; 3] = [c0, c1, c2];
    let ghost mut acc: Seq<Seq<u8>> = Seq::empty();
    // All three reservations succeeded; the log is frozen from here on.
    let ghost post_log_r: Seq<(AllocSite, nat, bool)> = log.view();
    proof {
        assert(channels@[0]@.len() == n);
        assert(channels@[1]@.len() == n);
        assert(channels@[2]@.len() == n);
        lemma_at_site_log_r_after_ok(n, post_log_r);
        crate::model::lemma_rle_pref_init(payload@, n as int, channels@);
    }
    let mut cursor: usize = 0;
    let mut ci: usize = 0;
    while ci < 3
        invariant
            ci <= 3,
            channels@.len() == 3,
            n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
            forall|k: int| 0 <= k < 3 ==> (#[trigger] channels@[k]@).len() == n,
            cursor <= payload@.len(),
            log.view() == post_log_r,
            post_log_r == Seq::<(AllocSite, nat, bool)>::empty()
                .push((AllocSite::Channel(0u8), n as nat, true))
                .push((AllocSite::Channel(1u8), n as nat, true))
                .push((AllocSite::Channel(2u8), n as nat, true)),
            at_site_log_r(n as nat, false, post_log_r),
            acc == crate::model::chans_prefix_view(channels@, ci as int),
            crate::model::rle_pref(
                payload@,
                n as int,
                ci as int,
                0,
                cursor as int,
                channels@,
                acc,
            ),
        decreases 3 - ci,
    {
        let mut produced: usize = 0;
        while produced < n
            invariant
                produced <= n,
                ci < 3,
                n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
                forall|k: int| 0 <= k < 3 ==> (#[trigger] channels@[k]@).len() == n,
                cursor <= payload@.len(),
                log.view() == post_log_r,
                post_log_r == Seq::<(AllocSite, nat, bool)>::empty()
                    .push((AllocSite::Channel(0u8), n as nat, true))
                    .push((AllocSite::Channel(1u8), n as nat, true))
                    .push((AllocSite::Channel(2u8), n as nat, true)),
                at_site_log_r(n as nat, false, post_log_r),
                crate::model::rle_pref(
                    payload@,
                    n as int,
                    ci as int,
                    produced as int,
                    cursor as int,
                    channels@,
                    acc,
                ),
            decreases n - produced,
        {
            if cursor == payload.len() {
                proof {
                    assert(crate::model::rle_chan_from(
                        payload@,
                        n as int,
                        produced as int,
                        cursor as int,
                    ) == Err(Error::RleTruncated)) by {
                        crate::model::lemma_rle_chan_from_trunc(
                            payload@,
                            n as int,
                            produced as int,
                            cursor as int,
                        );
                    }
                    crate::model::lemma_rle_pref_err_run(
                        payload@,
                        n as int,
                        ci as int,
                        produced as int,
                        cursor as int,
                        channels@,
                        acc,
                        Error::RleTruncated,
                    );
                    lemma_at_site_log_r_after_ok(n, post_log_r);
                }
                return Err(TryError::Format(Error::RleTruncated));
            }
            let ghost cur0: int = cursor as int;
            let control = payload[cursor];
            cursor += 1;
            let (count, literal) = if control < 128 {
                (control as usize + 1, true)
            } else {
                (control as usize - 125, false)
            };
            if count > n - produced {
                proof {
                    assert(crate::model::rle_count(control) == count as int);
                    assert(crate::model::rle_count(control) > n as int - produced as int);
                    crate::model::lemma_rle_chan_from_overrun(
                        payload@,
                        n as int,
                        produced as int,
                        cur0,
                    );
                    crate::model::lemma_rle_pref_err_run(
                        payload@,
                        n as int,
                        ci as int,
                        produced as int,
                        cur0,
                        channels@,
                        acc,
                        Error::RleOverrun,
                    );
                    lemma_at_site_log_r_after_ok(n, post_log_r);
                }
                return Err(TryError::Format(Error::RleOverrun));
            }
            let needed = if literal { count } else { 1 };
            if payload.len() - cursor < needed {
                proof {
                    assert(crate::model::rle_needed(control) == needed as int);
                    assert(cur0 + 1 + crate::model::rle_needed(control)
                        > payload@.len());
                    crate::model::lemma_rle_chan_from_need(
                        payload@,
                        n as int,
                        produced as int,
                        cur0,
                    );
                    crate::model::lemma_rle_pref_err_run(
                        payload@,
                        n as int,
                        ci as int,
                        produced as int,
                        cur0,
                        channels@,
                        acc,
                        Error::RleTruncated,
                    );
                    lemma_at_site_log_r_after_ok(n, post_log_r);
                }
                return Err(TryError::Format(Error::RleTruncated));
            }
            proof {
                assert(literal ==> cursor + count <= payload@.len()) by {
                    if literal {
                        assert(needed == count);
                    }
                }
                assert(!literal ==> cursor + 1 <= payload@.len()) by {
                    if !literal {
                        assert(needed == 1);
                    }
                }
                assert(ci < 3);
                assert(channels@[ci as int]@.len() == n);
                assert(count as int == crate::model::rle_count(control));
                assert(literal == crate::model::rle_literal(control));
                assert(needed as int == crate::model::rle_needed(control));
                crate::model::lemma_rle_bytes_len(payload@, cursor as int, count as int, literal);
            }
            let ghost ch_before: Seq<Vec<u8>> = channels@;
            rle_copy(&mut channels[ci], payload, cursor, produced, count, literal);
            proof {
                crate::model::lemma_write_seq_prefix(
                    ch_before[ci as int]@,
                    produced as int,
                    crate::model::rle_bytes(payload@, cursor as int, count as int, literal),
                );
                assert(channels@[ci as int]@.subrange(0, produced as int + count as int)
                    == ch_before[ci as int]@.subrange(0, produced as int) + crate::model::rle_bytes(
                    payload@,
                    cursor as int,
                    count as int,
                    literal,
                ));
                assert(ch_before[ci as int]@.subrange(0, produced as int)
                    == channels@[ci as int]@.subrange(0, produced as int)) by {
                    assert(channels@[ci as int]@.subrange(0, produced as int)
                        == ch_before[ci as int]@.subrange(0, produced as int));
                }
                assert forall|i: int| 0 <= i < 3 && i != ci implies (#[trigger] channels@[i]@)
                    == ch_before[i]@ by {};
                crate::model::lemma_rle_pref_write(
                    payload@,
                    n as int,
                    ci as int,
                    produced as int,
                    cur0,
                    ch_before,
                    channels@,
                    acc,
                );
                crate::model::lemma_rle_pref_step(
                    payload@,
                    n as int,
                    ci as int,
                    produced as int,
                    cur0,
                    channels@,
                    acc,
                );
            }
            cursor += needed;
            produced += count;
        }
        proof {
            crate::model::lemma_rle_pref_done_chan(
                payload@,
                n as int,
                ci as int,
                cursor as int,
                channels@,
                acc,
            );
            acc = acc.push(channels@[ci as int]@);
        }
        ci += 1;
    }
    proof {
        crate::model::lemma_rle_pref_finish(payload@, n as int, cursor as int, channels@, acc);
        assert(crate::model::rle_run(payload@, n as int) == Ok((acc, cursor as int)));
        assert(acc.len() == 3);
    }
    if cursor != payload.len() {
        proof {
            assert(crate::model::mr(payload@, n as int) == Err(Error::RleTrailingBytes));
            lemma_at_site_log_r_after_ok(n, post_log_r);
        }
        return Err(TryError::Format(Error::RleTrailingBytes));
    }
    proof {
        assert(crate::model::mr(payload@, n as int) == Ok((acc[0], acc[1], acc[2])));
        assert(acc[0] == channels@[0]@);
        assert(acc[1] == channels@[1]@);
        assert(acc[2] == channels@[2]@);
        lemma_at_site_log_r_after_ok(n, post_log_r);
        assert(crate::alloc_guard::sites::log_tied_channels(
            Ok::<[Vec<u8>; 3], TryError>(channels),
            log.view(),
        ));
    }
    Ok(channels)
}

/// The logged RLE entry point (A-ICNS-3): creates the one log, runs the three
/// channel reservations, returns the log. `Eq_f` is `rle_result_eq`. Crate
/// internal: its only caller is the `t9_rle_exact` witness, and its
/// `1 <= n <= 128*128` restriction is a Verus precondition that ordinary Rust
/// callers need not honour (ERRATA E12).
pub(crate) fn try_rle_logged(payload: &[u8], n: usize)
    -> (r: (Result<[Vec<u8>; 3], TryError>, Tracked<AllocLog>))
    requires
        n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
        1 <= n,
    ensures
        g_r(payload@, n as nat, r.0, r.1@.view()),
        prog_r(payload@, n as nat, r.0, r.1@.view()),
        crate::alloc_guard::sites::log_tied_channels(r.0, r.1@.view()),
        match r.0 {
            Ok(ch) => {
                &&& ch@.len() == 3
                &&& ch@[0]@.len() == n
                &&& ch@[1]@.len() == n
                &&& ch@[2]@.len() == n
            },
            Err(_) => true,
        },
{
    let tracked mut log = crate::alloc_guard::new_log();
    let result = rle_decode_tracked(payload, n, Tracked(&mut log));
    proof {
        reveal(g_r);
        reveal(g_r_clause1);
        reveal(g_r_clause2);
        reveal(g_r_clause3);
        reveal(prog_r);
        reveal(prog_r_concl);
        reveal(crate::alloc_guard::sites::log_tied_channels);
        reveal(req_r);
        reveal(at_site_log_r);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        lemma_req_r_len(n as nat);
        lemma_req_r_clause3(n as nat);
        // `at_site_log_r` is the callee's postcondition; unfold it per arm.
        assert(at_site_log_r(n as nat, matches!(result, Err(TryError::OutOfMemory(_))),
            log.view()));
        if let Err(TryError::OutOfMemory(_)) = result {
            assert(crate::alloc_guard::req_of_log(log.view())
                .is_prefix_of(req_r(n as nat)));
        } else {
            assert(log.view() == Seq::<(AllocSite, nat, bool)>::empty()
                .push((AllocSite::Channel(0u8), n as nat, true))
                .push((AllocSite::Channel(1u8), n as nat, true))
                .push((AllocSite::Channel(2u8), n as nat, true)));
            assert(crate::alloc_guard::req_of_log(log.view()) == req_r(n as nat));
            assert(crate::alloc_guard::all_ok(log.view()));
        }
    }
    (result, Tracked(log))
}

/// A successful decode has `width == height == side <= 128` and
/// `data.len() == width*height * per-pixel`, per R1 and R8.
fn payload_range<'a>(bytes: &'a [u8], entry: &Entry) -> (r: &'a [u8])
    requires
        crate::model::entry_payload_safe(bytes@, *entry),
    ensures
        r@ == crate::model::entry_payload(bytes@, *entry),
{
    &bytes[entry.payload[0] as usize..entry.payload[1] as usize]
}

// ---------------------------------------------------------------------------
// MC (decode_rgb) and MA (decode_mask) with the alloc-delta guard
// (A-ICNS-4/5).
// ---------------------------------------------------------------------------

/// `Req_C(b,E,i)`: `[]` on index/type errors, `Req_R(n)` on an RLE error, and
/// `Req_R(n) ++ [(RgbOut, 3n)]` otherwise (SPEC.md §4, A-ICNS-4). Stated in model
/// terms through `mc`: the index/type guards and the RLE outcome come from `mc`.
#[doc(hidden)]
pub open spec fn req_c(b: Seq<u8>, E: Seq<Entry>, i: u32) -> Seq<(AllocSite, nat)> {
    if (i as int) >= (E.len() as int) {
        Seq::empty()
    } else {
        let e = E[i as int];
        let code = crate::model::entry_type_code(e);
        match crate::model::legacy_side(code) {
            Some((true, side)) => {
                let n: nat = ((side as int) * (side as int)) as nat;
                let p2 = crate::model::strip_it32(code, crate::model::entry_payload_bytes(b, e));
                match crate::model::mr(p2, n as int) {
                    Ok(_) => req_r(n).push((AllocSite::RgbOut, 3 * n)),
                    Err(_) => req_r(n),
                }
            },
            _ => Seq::empty(),
        }
    }
}

/// `(site, elements)` is one of the requests in `req`: the public `try_*`'s
/// `OutOfMemory`-naming clause (THEOREMS-ALLOC-DELTA.md §0).
#[doc(hidden)]
pub open spec fn oom_in_req(site: AllocSite, elements: nat, req: Seq<(AllocSite, nat)>) -> bool {
    exists|j: int| 0 <= j < req.len() && req[j] == (site, elements)
}

/// `PreAlloc_C(b,E,i)` (THEOREMS-ALLOC-DELTA.md §0): `Req_C` is empty, so the
/// colour decode reserves nothing and cannot return `OutOfMemory`.
#[doc(hidden)]
pub open spec fn pre_alloc_c(b: Seq<u8>, E: Seq<Entry>, i: u32) -> bool {
    req_c(b, E, i).len() == 0
}

/// `PreAlloc_A(b,E,i)`: as above for `Req_A`.
#[doc(hidden)]
pub open spec fn pre_alloc_a(b: Seq<u8>, E: Seq<Entry>, i: u32) -> bool {
    req_a(b, E, i).len() == 0
}

/// `PreAlloc_J(b,E,ci,mi)`: as above for `Req_J`.
#[doc(hidden)]
pub open spec fn pre_alloc_j(b: Seq<u8>, E: Seq<Entry>, ci: Option<u32>, mi: Option<u32>) -> bool {
    req_j(b, E, ci, mi).len() == 0
}

/// The named `PreAlloc` corollary for the colour decoder (THEOREMS-ALLOC-DELTA.md
/// §0): an empty `Req_C` means the result is not `OutOfMemory`, `rgb_decoded_eq`
/// holds on `fmt_part(r)`, and `L` is empty.
#[doc(hidden)]
pub open spec fn pre_alloc_c_concl(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    pre_alloc_c(b, E, i)
        ==> !matches!(r, Err(TryError::OutOfMemory(_)))
            && crate::model::rgb_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i)
            && l == Seq::<(AllocSite, nat, bool)>::empty()
}

/// The named `PreAlloc` corollary for the mask decoder (A-ICNS-5).
#[doc(hidden)]
pub open spec fn pre_alloc_a_concl(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    pre_alloc_a(b, E, i)
        ==> !matches!(r, Err(TryError::OutOfMemory(_)))
            && crate::model::mask_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i)
            && l == Seq::<(AllocSite, nat, bool)>::empty()
}

/// The named `PreAlloc` corollary for the combine decoder (A-ICNS-6).
#[doc(hidden)]
pub open spec fn pre_alloc_j_concl(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    pre_alloc_j(b, E, ci, mi)
        ==> !matches!(r, Err(TryError::OutOfMemory(_)))
            && crate::model::combine_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi)
            && l == Seq::<(AllocSite, nat, bool)>::empty()
}

#[doc(hidden)]
pub open spec fn g_c_clause1(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::rgb_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i)
            && crate::alloc_guard::req_of_log(l) == req_c(b, E, i)
            && crate::alloc_guard::all_ok(l),
    }
}

#[doc(hidden)]
pub open spec fn g_c_clause2(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_c(b, E, i))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G` for `try_decode_rgb`: every request is within its own site's
/// maximum (16,384 for a channel, 49,152 for `RgbOut`), from `parsed_inv`
/// (legacy sides <= 128), not the global `MAX_BYTES`.
#[doc(hidden)]
pub open spec fn g_c_clause3(b: Seq<u8>, E: Seq<Entry>, i: u32) -> bool {
    req_within_site_max(req_c(b, E, i))
}

#[doc(hidden)]
pub open spec fn g_c(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_c_clause1(b, E, i, r, l) && g_c_clause2(b, E, i, r, l) && g_c_clause3(b, E, i)
}

#[doc(hidden)]
pub open spec fn prog_c(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_c_concl(b, E, i, r, l)
}

#[doc(hidden)]
pub open spec fn prog_c_concl(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && crate::model::rgb_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i)
        && crate::alloc_guard::req_of_log(l) == req_c(b, E, i)
}

#[doc(hidden)]
pub open spec fn at_site_log_c(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    if r_is_oom {
        crate::alloc_guard::req_of_log(l).is_prefix_of(req_c(b, E, i))
            && l.len() >= 1
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
    } else {
        crate::alloc_guard::req_of_log(l) == req_c(b, E, i) && crate::alloc_guard::all_ok(l)
    }
}

/// `Req_A(b,E,i)`: `[(MaskData, n)]` iff `MA`'s length check passes, else `[]`.
#[doc(hidden)]
pub open spec fn req_a(b: Seq<u8>, E: Seq<Entry>, i: u32) -> Seq<(AllocSite, nat)> {
    if (i as int) >= (E.len() as int) {
        Seq::empty()
    } else {
        let e = E[i as int];
        let code = crate::model::entry_type_code(e);
        match crate::model::legacy_side(code) {
            Some((false, side)) => {
                let n: nat = ((side as int) * (side as int)) as nat;
                let p = crate::model::entry_payload_bytes(b, e);
                if p.len() == n as int {
                    seq![(AllocSite::MaskData, n)]
                } else {
                    Seq::empty()
                }
            },
            _ => Seq::empty(),
        }
    }
}

#[doc(hidden)]
pub open spec fn g_a_clause1(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::mask_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i)
            && crate::alloc_guard::req_of_log(l) == req_a(b, E, i)
            && crate::alloc_guard::all_ok(l),
    }
}

#[doc(hidden)]
pub open spec fn g_a_clause2(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_a(b, E, i))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G` for `try_decode_mask`: the one request is at most 16,384,
/// `site_max(MaskData)`, from `parsed_inv`.
#[doc(hidden)]
pub open spec fn g_a_clause3(b: Seq<u8>, E: Seq<Entry>, i: u32) -> bool {
    req_within_site_max(req_a(b, E, i))
}

#[doc(hidden)]
pub open spec fn g_a(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_a_clause1(b, E, i, r, l) && g_a_clause2(b, E, i, r, l) && g_a_clause3(b, E, i)
}

#[doc(hidden)]
pub open spec fn prog_a(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_a_concl(b, E, i, r, l)
}

#[doc(hidden)]
pub open spec fn prog_a_concl(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && crate::model::mask_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i)
        && crate::alloc_guard::req_of_log(l) == req_a(b, E, i)
}

/// `l` starts with `pre` and the requests appended after `pre` are a prefix of
/// `req`. The `pre` parameter is what lets `combine` share one log: the mask
/// decode runs after the colour decode, so its log starts with `Req_C`.
#[doc(hidden)]
pub open spec fn log_extends(
    pre: Seq<(AllocSite, nat, bool)>,
    l: Seq<(AllocSite, nat, bool)>,
    req: Seq<(AllocSite, nat)>,
) -> bool {
    l.len() >= pre.len()
        && l.subrange(0, pre.len() as int) == pre
        && crate::alloc_guard::req_of_log(
            l.subrange(pre.len() as int, l.len() as int),
        ).is_prefix_of(req)
}

/// The mask decode's log shape, relative to the prefix `pre` it starts from.
/// With `pre == []` this is the standalone `try_decode_mask` shape.
#[doc(hidden)]
pub open spec fn at_site_log_a(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    pre: Seq<(AllocSite, nat, bool)>,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    if r_is_oom {
        log_extends(pre, l, req_a(b, E, i))
            && l.len() > pre.len()
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
    } else {
        crate::alloc_guard::req_of_log(l) == crate::alloc_guard::req_of_log(pre) + req_a(b, E, i)
            && crate::alloc_guard::all_ok(l)
    }
}

/// Clause 3 for `try_decode_rgb`: each request is within its own site's bound,
/// from `parsed_inv` (every legacy side is at most 128).
proof fn lemma_req_c_clause3(b: Seq<u8>, E: Seq<Entry>, i: u32)
    ensures g_c_clause3(b, E, i),
{
    reveal(g_c_clause3);
    lemma_req_c_bound(b, E, i);
}

/// Clause 3 for `try_decode_mask`: the one request is within `MaskData`'s bound.
proof fn lemma_req_a_clause3(b: Seq<u8>, E: Seq<Entry>, i: u32)
    ensures g_a_clause3(b, E, i),
{
    reveal(g_a_clause3);
    lemma_req_a_bound(b, E, i);
}

/// The named `PreAlloc` corollary for `try_decode_rgb` (THEOREMS-ALLOC-DELTA.md
/// §0, A-ICNS-4): `Req_C == []` implies the result is not `OutOfMemory`,
/// `rgb_decoded_eq` holds on `fmt_part(r)`, and `L == []`. Proved from `G`:
/// clause 2 would need a non-empty log that is a prefix of the empty `Req_C`.
pub(crate) proof fn lemma_g_rgb_prealloc(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_c(b, E, i, r, l),
    ensures
        pre_alloc_c(b, E, i) ==> !matches!(r, Err(TryError::OutOfMemory(_))),
        pre_alloc_c(b, E, i) ==> crate::model::rgb_decoded_eq(
            crate::alloc_guard::sites::fmt_part(r), b, E, i),
        pre_alloc_c(b, E, i) ==> l == Seq::<(AllocSite, nat, bool)>::empty(),
        pre_alloc_c_concl(b, E, i, r, l),
{
    reveal(g_c);
    reveal(g_c_clause1);
    reveal(g_c_clause2);
    reveal(pre_alloc_c);
    reveal(pre_alloc_c_concl);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    if pre_alloc_c(b, E, i) {
        if let Err(TryError::OutOfMemory(_)) = r {
            assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_c(b, E, i)));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
            assert(l.len() <= req_c(b, E, i).len());
            assert(l.len() >= 1);
            assert(false);
        }
        match r {
            Err(TryError::OutOfMemory(_)) => { assert(false); },
            _ => {
                assert(crate::model::rgb_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(r), b, E, i));
                assert(crate::alloc_guard::req_of_log(l) == req_c(b, E, i));
                assert(req_c(b, E, i).len() == 0);
                assert(l.len() == 0);
                assert(l =~= Seq::<(AllocSite, nat, bool)>::empty());
            },
        }
    }
}

/// The public `try_decode_rgb` contract implies the `PreAlloc` corollary's
/// observable parts. Its `OutOfMemory` naming clause together with `Req_C == []`
/// rules `OutOfMemory` out; the functional relation is the contract's own.
pub(crate) proof fn lemma_public_rgb_prealloc(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
)
    requires
        pre_alloc_c(b, E, i),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                oom_in_req(o.site_spec(), o.elements_spec() as nat, req_c(b, E, i)),
            _ => true,
        },
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rgb_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::rgb_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i),
{
    reveal(pre_alloc_c);
    reveal(oom_in_req);
    if let Err(TryError::OutOfMemory(o)) = r {
        assert(oom_in_req(o.site_spec(), o.elements_spec() as nat, req_c(b, E, i)));
        assert(req_c(b, E, i).len() == 0);
        assert(!oom_in_req(o.site_spec(), o.elements_spec() as nat, req_c(b, E, i)));
        assert(false);
    }
}

/// The named `PreAlloc` corollary for `try_decode_mask` (THEOREMS-ALLOC-DELTA.md
/// §0, A-ICNS-5): `Req_A == []` implies non-`OutOfMemory`, `mask_decoded_eq` on
/// `fmt_part(r)`, and `L == []`.
pub(crate) proof fn lemma_g_mask_prealloc(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_a(b, E, i, r, l),
    ensures
        pre_alloc_a(b, E, i) ==> !matches!(r, Err(TryError::OutOfMemory(_))),
        pre_alloc_a(b, E, i) ==> crate::model::mask_decoded_eq(
            crate::alloc_guard::sites::fmt_part(r), b, E, i),
        pre_alloc_a(b, E, i) ==> l == Seq::<(AllocSite, nat, bool)>::empty(),
        pre_alloc_a_concl(b, E, i, r, l),
{
    reveal(g_a);
    reveal(g_a_clause1);
    reveal(g_a_clause2);
    reveal(pre_alloc_a);
    reveal(pre_alloc_a_concl);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    if pre_alloc_a(b, E, i) {
        if let Err(TryError::OutOfMemory(_)) = r {
            assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_a(b, E, i)));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
            assert(l.len() <= req_a(b, E, i).len());
            assert(l.len() >= 1);
            assert(false);
        }
        match r {
            Err(TryError::OutOfMemory(_)) => { assert(false); },
            _ => {
                assert(crate::model::mask_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(r), b, E, i));
                assert(crate::alloc_guard::req_of_log(l) == req_a(b, E, i));
                assert(req_a(b, E, i).len() == 0);
                assert(l.len() == 0);
                assert(l =~= Seq::<(AllocSite, nat, bool)>::empty());
            },
        }
    }
}

/// The public `try_decode_mask` contract implies the `PreAlloc` corollary's
/// observable parts.
pub(crate) proof fn lemma_public_mask_prealloc(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    r: Result<Decoded, TryError>,
)
    requires
        pre_alloc_a(b, E, i),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                oom_in_req(o.site_spec(), o.elements_spec() as nat, req_a(b, E, i)),
            _ => true,
        },
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::mask_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::mask_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, i),
{
    reveal(pre_alloc_a);
    reveal(oom_in_req);
    if let Err(TryError::OutOfMemory(o)) = r {
        assert(oom_in_req(o.site_spec(), o.elements_spec() as nat, req_a(b, E, i)));
        assert(req_a(b, E, i).len() == 0);
        assert(!oom_in_req(o.site_spec(), o.elements_spec() as nat, req_a(b, E, i)));
        assert(false);
    }
}

/// The empty log is all-`Ok`, and its request list is empty.
proof fn lemma_all_ok_empty()
    ensures
        crate::alloc_guard::all_ok(Seq::<(AllocSite, nat, bool)>::empty()),
        crate::alloc_guard::req_of_log(Seq::<(AllocSite, nat, bool)>::empty())
            == Seq::<(AllocSite, nat)>::empty(),
{
    reveal(crate::alloc_guard::all_ok);
    reveal(crate::alloc_guard::req_of_log);
}

/// `l = pre.push((site, n, false))` with `pre` all-`Ok`: every entry but the
/// last is `Ok`. Isolates the `Seq::push` index reasoning in one place.
proof fn lemma_all_ok_push_false(
    pre: Seq<(AllocSite, nat, bool)>,
    site: AllocSite,
    n: nat,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        l == pre.push((site, n, false)),
        crate::alloc_guard::all_ok(pre),
    ensures
        forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2,
{
    reveal(crate::alloc_guard::all_ok);
    let x = (site, n, false);
    vstd::seq::lemma_seq_push_len(pre, x);
    assert(l == pre.push(x));
    assert(l.len() == pre.len() + 1);
    assert forall|j: int| 0 <= j < l.len() - 1 implies #[trigger] l[j].2 by {
        assert(j < pre.len()) by (nonlinear_arith)
            requires 0 <= j, j < l.len() - 1, l.len() == pre.len() + 1;
        vstd::seq::lemma_seq_push_index_different(pre, x, j);
        assert(pre.push(x)[j] == pre[j]);
        assert(l[j] == pre[j]);
        assert(pre[j].2);
    }
}

/// `l = pre.push((site, n, true))` with `pre` all-`Ok`: every entry is `Ok`.
proof fn lemma_all_ok_push_true(
    pre: Seq<(AllocSite, nat, bool)>,
    site: AllocSite,
    n: nat,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        l == pre.push((site, n, true)),
        crate::alloc_guard::all_ok(pre),
    ensures crate::alloc_guard::all_ok(l),
{
    reveal(crate::alloc_guard::all_ok);
    let x = (site, n, true);
    vstd::seq::lemma_seq_push_len(pre, x);
    assert(l == pre.push(x));
    assert(l.len() == pre.len() + 1);
    assert forall|j: int| 0 <= j < l.len() implies #[trigger] l[j].2 by {
        if j < pre.len() {
            vstd::seq::lemma_seq_push_index_different(pre, x, j);
            assert(pre.push(x)[j] == pre[j]);
            assert(l[j] == pre[j]);
        } else {
            assert(j == pre.len()) by (nonlinear_arith)
                requires 0 <= j, j < l.len(), j >= pre.len(), l.len() == pre.len() + 1;
            vstd::seq::lemma_seq_push_index_same(pre, x, j);
            assert(pre.push(x)[j] == x);
            assert(l[j] == x);
        }
    }
}

/// The index/type-error exit: the log is empty and `Req_C` is empty too.
proof fn lemma_at_site_log_c_empty(b: Seq<u8>, E: Seq<Entry>, i: u32, l: Seq<(AllocSite, nat, bool)>)
    requires
        l == Seq::<(AllocSite, nat, bool)>::empty(),
        req_c(b, E, i).len() == 0,
    ensures at_site_log_c(b, E, i, false, l),
{
    reveal(at_site_log_c);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    lemma_all_ok_empty();
}

/// The RLE-format-error exit: `L = Req_R(n)`, all `Ok`, and `Req_C = Req_R(n)`.
proof fn lemma_at_site_log_c_after_rle_err(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    n: usize,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        (i as int) < (E.len() as int),
        req_c(b, E, i) == req_r(n as nat),
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n as nat, true))
            .push((AllocSite::Channel(1u8), n as nat, true))
            .push((AllocSite::Channel(2u8), n as nat, true)),
    ensures at_site_log_c(b, E, i, false, l),
{
    reveal(at_site_log_c);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_c(b, E, i));
}

/// For a legacy colour entry, `Req_R(n)` is a prefix of `Req_C`: `Req_C` is
/// either `Req_R(n)` (an RLE error) or `Req_R(n) ++ [(RgbOut, 3n)]`.
proof fn lemma_req_r_prefix_req_c(b: Seq<u8>, E: Seq<Entry>, i: u32, side: u32, n: nat)
    requires
        (i as int) < (E.len() as int),
        crate::model::legacy_side(crate::model::entry_type_code(E[i as int]))
            == Some((true, side)),
        n == ((side as int) * (side as int)) as nat,
    ensures
        req_r(n).is_prefix_of(req_c(b, E, i)),
{
    reveal(req_c);
    reveal(req_r);
    let e = E[i as int];
    let code = crate::model::entry_type_code(e);
    assert(crate::model::legacy_side(code) == Some((true, side)));
    let p2 = crate::model::strip_it32(code, crate::model::entry_payload_bytes(b, e));
    match crate::model::mr(p2, n as int) {
        Ok(_) => {
            assert(req_c(b, E, i) == req_r(n).push((AllocSite::RgbOut, 3 * n)));
            assert(req_r(n).is_prefix_of(req_r(n).push((AllocSite::RgbOut, 3 * n))));
        },
        Err(_) => {
            assert(req_c(b, E, i) == req_r(n));
        },
    }
}

/// The channel-failure exit of `try_decode_rgb`: `at_site_log_r` gives it.
proof fn lemma_at_site_log_c_oom_from_r(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    n: usize,
    o: OutOfMemory,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        (i as int) < (E.len() as int),
        at_site_log_r(n as nat, true, l),
        l.len() >= 1,
        o.site_spec() == l.last().0,
        o.elements_spec() == n,
        req_r(n as nat).is_prefix_of(req_c(b, E, i)),
    ensures at_site_log_c(b, E, i, true, l),
{
    reveal(at_site_log_c);
    reveal(at_site_log_r);
    reveal(req_c);
    reveal(req_r);
    assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_r(n as nat)));
    assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_c(b, E, i)));
}

/// The `RgbOut`-failure exit: `L = Req_R(n) ++ [(RgbOut, 3n, false)]` and
/// `Req_C = Req_R(n) ++ [(RgbOut, 3n)]`.
proof fn lemma_at_site_log_c_oom_rgbout(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    n: usize,
    o: OutOfMemory,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        (i as int) < (E.len() as int),
        o.site_spec() == AllocSite::RgbOut,
        o.elements_spec() == 3 * n,
        req_c(b, E, i) == req_r(n as nat).push((AllocSite::RgbOut, 3 * n as nat)),
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n as nat, true))
            .push((AllocSite::Channel(1u8), n as nat, true))
            .push((AllocSite::Channel(2u8), n as nat, true))
            .push((AllocSite::RgbOut, 3 * n as nat, false)),
    ensures at_site_log_c(b, E, i, true, l),
{
    reveal(at_site_log_c);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    assert(crate::alloc_guard::req_of_log(l) =~= req_c(b, E, i));
    assert forall|j: int| 0 <= j < l.len() - 1 implies #[trigger] l[j].2 by {};
}

/// The `Ok` exit of `try_decode_rgb`: `L = Req_R(n) ++ [(RgbOut, 3n)]`, all `Ok`,
/// and `Req_C` is the same list.
proof fn lemma_at_site_log_c_after_ok(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    n: usize,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        (i as int) < (E.len() as int),
        req_c(b, E, i) == req_r(n as nat).push((AllocSite::RgbOut, 3 * n as nat)),
        l == Seq::<(AllocSite, nat, bool)>::empty()
            .push((AllocSite::Channel(0u8), n as nat, true))
            .push((AllocSite::Channel(1u8), n as nat, true))
            .push((AllocSite::Channel(2u8), n as nat, true))
            .push((AllocSite::RgbOut, 3 * n as nat, true)),
    ensures at_site_log_c(b, E, i, false, l),
{
    reveal(at_site_log_c);
    reveal(req_r);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_c(b, E, i));
    assert forall|j: int| 0 <= j < l.len() implies #[trigger] l[j].2 by {};
}

/// R10 and R11: decode a legacy colour entry to interleaved RGB, through the
/// sealed reservation log. On a non-`OutOfMemory` result it equals
/// `MC(bytes, entries, color_index)` exactly (A-ICNS-4).
fn decode_rgb_tracked(
    bytes: &[u8],
    entries: &[Entry],
    color_index: u32,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rgb_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                color_index,
            ),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 3
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 3
            },
            Err(_) => true,
        },
        at_site_log_c(bytes@, entries@, color_index,
            matches!(r, Err(TryError::OutOfMemory(_))), final(log).view()),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                final(log).view().len() >= 1
                && o.site_spec() == final(log).view().last().0
                && o.elements_spec() as nat == final(log).view().last().1,
            _ => true,
        },
{
    if color_index as usize >= entries.len() {
        proof {
            crate::model::lemma_mc_index(bytes@, entries@, color_index);
            reveal(req_c);
            lemma_at_site_log_c_empty(bytes@, entries@, color_index, log.view());
        }
        return Err(TryError::Format(Error::IndexOutOfRange));
    }
    let entry = &entries[color_index as usize];
    proof {
        assert(crate::model::entry_inv(bytes@, entries@[color_index as int]));
    }
    let code = entry.type_code();
    let (is_colour, side) = match legacy_side(code) {
        Some(v) => v,
        None => {
            proof {
                crate::model::lemma_mc_unsupported_none(bytes@, entries@, color_index, code);
                reveal(req_c);
                reveal(crate::model::legacy_side);
                lemma_at_site_log_c_empty(bytes@, entries@, color_index, log.view());
            }
            return Err(TryError::Format(Error::UnsupportedType));
        },
    };
    if !is_colour {
        proof {
            crate::model::lemma_mc_unsupported_mask(bytes@, entries@, color_index, code, side);
            reveal(req_c);
            reveal(crate::model::legacy_side);
            lemma_at_site_log_c_empty(bytes@, entries@, color_index, log.view());
        }
        return Err(TryError::Format(Error::UnsupportedType));
    }
    proof {
        lemma_legacy_side_le_128(code);
        crate::model::lemma_legacy_side_ge_16(code);
        assert(16 <= side && side <= 128);
        assert((side as usize) * (side as usize) <= usize::MAX) by (nonlinear_arith)
            requires 16 <= side, side <= 128;
        crate::model::lemma_mc_colour(bytes@, entries@, color_index, code, side, (side as int)
            * (side as int));
    }
    let n = (side as usize) * (side as usize);
    proof {
        assert(1 <= n) by (nonlinear_arith)
            requires 16 <= side, side <= 128, n == (side as usize) * (side as usize);
        assert(n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE) by (nonlinear_arith)
            requires side <= 128, n == (side as usize) * (side as usize);
        lemma_req_c_clause3(bytes@, entries@, color_index);
    }
    let payload = payload_range(bytes, entry);

    // R11: for it32 only, remove a four-byte all-zero prefix.
    let payload = if code == [105u8, 116, 51, 50]
        && payload.len() >= 4
        && payload[0] == 0
        && payload[1] == 0
        && payload[2] == 0
        && payload[3] == 0
    {
        proof {
            assert(crate::model::strip_it32(code, crate::model::entry_payload_bytes(bytes@, *entry))
                == crate::model::it32_prefix(crate::model::entry_payload_bytes(bytes@, *entry)));
            assert(crate::model::it32_prefix(crate::model::entry_payload_bytes(bytes@, *entry))
                =~= payload@.subrange(4, payload@.len() as int)) by {
                reveal(crate::model::it32_prefix);
            }
        }
        &payload[4..]
    } else {
        proof {
            assert(crate::model::strip_it32(code, crate::model::entry_payload_bytes(bytes@, *entry))
                == crate::model::entry_payload_bytes(bytes@, *entry)) by {
                reveal(crate::model::strip_it32);
                reveal(crate::model::it32_prefix);
                reveal(crate::model::entry_payload_bytes);
            }
        }
        payload
    };
    proof {
        assert(crate::model::strip_it32(code, crate::model::entry_payload_bytes(bytes@, *entry))
            =~= payload@);
    }

    let channels = match rle_decode_tracked(payload, n, Tracked(log)) {
        Ok(ch) => {
            proof {
                assert(ch@[0]@.len() == n);
                assert(ch@[1]@.len() == n);
                assert(ch@[2]@.len() == n);
                reveal(at_site_log_r);
                reveal(req_r);
                reveal(crate::alloc_guard::req_of_log);
                assert(log.view() == Seq::<(AllocSite, nat, bool)>::empty()
                    .push((AllocSite::Channel(0u8), n as nat, true))
                    .push((AllocSite::Channel(1u8), n as nat, true))
                    .push((AllocSite::Channel(2u8), n as nat, true)));
                assert(crate::alloc_guard::req_of_log(log.view()) == req_r(n as nat));
                assert(log.view().len() == 3);
                reveal(crate::model::rle_result_eq);
                reveal(crate::model::legacy_side);
            }
            ch
        },
        Err(TryError::Format(e)) => {
            proof {
                reveal(crate::alloc_guard::sites::fmt_part_channels);
                reveal(req_c);
                reveal(req_r);
                reveal(crate::model::rle_result_eq);
                assert(crate::model::mr(
                    crate::model::strip_it32(code, crate::model::entry_payload_bytes(bytes@, *entry)),
                    n as int,
                ) == Err(e));
                crate::model::lemma_mc_rle_err(bytes@, entries@, color_index, code, side, e);
                lemma_at_site_log_c_after_rle_err(bytes@, entries@, color_index, n, log.view());
            }
            return Err(TryError::Format(e));
        },
        Err(TryError::OutOfMemory(o)) => {
            proof {
                reveal(req_c);
                reveal(req_r);
                lemma_req_r_prefix_req_c(bytes@, entries@, color_index, side, n as nat);
                lemma_at_site_log_c_oom_from_r(bytes@, entries@, color_index, n, o, log.view());
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    // A-ICNS-4: the `RgbOut` reservation, after a successful `MR` and while the
    // three channels are live (peak `6*n` in the colour decode, SPEC.md §4).
    let mut out = match crate::alloc_guard::try_zeroed(AllocSite::RgbOut, 3 * n, Tracked(log)) {
        Ok(v) => v,
        Err(o) => {
            proof {
                reveal(req_c);
                reveal(req_r);
                lemma_at_site_log_c_oom_rgbout(bytes@, entries@, color_index, n, o, log.view());
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    proof { reveal(crate::alloc_guard::sites::fmt_part); }
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
            out@.len() == 3 * n,
            channels@.len() == 3,
            channels@[0]@.len() == n,
            channels@[1]@.len() == n,
            channels@[2]@.len() == n,
            forall|k: int| 0 <= k < i ==> #[trigger] out@[3 * k] == channels@[0]@[k],
            forall|k: int| 0 <= k < i ==> #[trigger] out@[3 * k + 1] == channels@[1]@[k],
            forall|k: int| 0 <= k < i ==> #[trigger] out@[3 * k + 2] == channels@[2]@[k],
        decreases n - i,
    {
        out[3 * i] = channels[0][i];
        out[3 * i + 1] = channels[1][i];
        out[3 * i + 2] = channels[2][i];
        i = i + 1;
    }
    proof {
        crate::model::lemma_interleave_rgb_exact(
            channels@[0]@,
            channels@[1]@,
            channels@[2]@,
            out@,
        );
        crate::model::lemma_rgb_ok(
            bytes@,
            entries@,
            color_index,
            code,
            side,
            n as int,
            payload@,
            channels@[0]@,
            channels@[1]@,
            channels@[2]@,
            out@,
        );
        lemma_at_site_log_c_after_ok(bytes@, entries@, color_index, n, log.view());
    }
    Ok(Decoded {
        width: side,
        height: side,
        data: out,
    })
}

/// The logged colour decoder (A-ICNS-4): creates the one log and returns it.
pub(crate) fn try_decode_rgb_logged(bytes: &[u8], entries: &[Entry], color_index: u32)
    -> (r: (Result<Decoded, TryError>, Tracked<AllocLog>))
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        g_c(bytes@, entries@, color_index, r.0, r.1@.view()),
        prog_c(bytes@, entries@, color_index, r.0, r.1@.view()),
        crate::alloc_guard::sites::log_tied(r.0, r.1@.view()),
        pre_alloc_c_concl(bytes@, entries@, color_index, r.0, r.1@.view()),
        match r.0 {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 3
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 3
            },
            Err(_) => true,
        },
{
    let tracked mut log = crate::alloc_guard::new_log();
    let result = decode_rgb_tracked(bytes, entries, color_index, Tracked(&mut log));
    proof {
        reveal(g_c);
        reveal(g_c_clause1);
        reveal(g_c_clause2);
        reveal(g_c_clause3);
        reveal(prog_c);
        reveal(prog_c_concl);
        reveal(crate::alloc_guard::sites::log_tied);
        reveal(req_c);
        reveal(at_site_log_c);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        reveal(pre_alloc_c);
        reveal(pre_alloc_c_concl);
        assert(at_site_log_c(bytes@, entries@, color_index,
            matches!(result, Err(TryError::OutOfMemory(_))), log.view()));
        if let Err(TryError::OutOfMemory(_)) = result {
            assert(crate::alloc_guard::req_of_log(log.view())
                .is_prefix_of(req_c(bytes@, entries@, color_index)));
        } else {
            assert(crate::alloc_guard::req_of_log(log.view())
                == req_c(bytes@, entries@, color_index));
            assert(crate::alloc_guard::all_ok(log.view()));
        }
        lemma_g_rgb_prealloc(bytes@, entries@, color_index, result, log.view());
    }
    (result, Tracked(log))
}

/// The public colour decoder (A-ICNS-4): `try_decode_rgb_logged` with the log
/// discarded (decision D1: the frozen name `decode_rgb` is removed).
pub(crate) fn try_decode_rgb(bytes: &[u8], entries: &[Entry], color_index: u32)
    -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rgb_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                color_index,
            ),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 3
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 3
            },
            Err(_) => true,
        },
        match r {
            Err(TryError::OutOfMemory(o)) =>
                oom_in_req(o.site_spec(), o.elements_spec() as nat, req_c(bytes@, entries@, color_index)),
            _ => true,
        },
{
    let res = try_decode_rgb_logged(bytes, entries, color_index);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(oom_in_req);
        reveal(g_c);
        reveal(g_c_clause2);
        reveal(req_c);
        reveal(crate::alloc_guard::req_of_log);
        if let Err(TryError::OutOfMemory(o)) = res.0 {
            assert(g_c(bytes@, entries@, color_index, res.0, res.1@.view()));
            assert(g_c_clause2(bytes@, entries@, color_index, res.0, res.1@.view()));
            assert(res.1@.view().len() >= 1);
            assert(crate::alloc_guard::req_of_log(res.1@.view())
                .is_prefix_of(req_c(bytes@, entries@, color_index)));
            assert(res.1@.view().last().0 == o.site_spec());
            assert(res.1@.view().last().1 == o.elements_spec() as nat);
            let j = res.1@.view().len() - 1;
            assert(0 <= j < req_c(bytes@, entries@, color_index).len());
            assert(crate::alloc_guard::req_of_log(res.1@.view())[j]
                == (o.site_spec(), o.elements_spec() as nat));
            assert(req_c(bytes@, entries@, color_index)[j]
                == (o.site_spec(), o.elements_spec() as nat));
            assert(oom_in_req(o.site_spec(), o.elements_spec() as nat,
                req_c(bytes@, entries@, color_index)));
        }
    }
    res.0
}

impl<'a> Parsed<'a> {
    /// The public colour decoder (A-ICNS-4) over the handle: `try_decode_rgb`
    /// with the bytes and entries the handle holds.
    ///
    /// There is no `requires` clause: the handle's type invariant supplies
    /// `parsed_inv(self.bytes@, self.entries@)`, which is exactly the internal
    /// function's precondition. The `ensures` restates the internal
    /// postcondition in handle terms, so a caller sees the model equality, the
    /// output bounds and the named `OutOfMemory` request without naming the
    /// opaque fields.
    pub fn try_decode_rgb(&self, color_index: u32) -> (r: Result<Decoded, TryError>)
        ensures
            !matches!(r, Err(TryError::OutOfMemory(_)))
                ==> crate::model::rgb_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(r),
                    self.spec_bytes(),
                    self.spec_entries(),
                    color_index,
                ),
            match r {
                Ok(d) => {
                    &&& d.width == d.height
                    &&& d.width <= 128
                    &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 3
                    &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 3
                },
                Err(_) => true,
            },
            match r {
                Err(TryError::OutOfMemory(o)) =>
                    oom_in_req(o.site_spec(), o.elements_spec() as nat,
                        req_c(self.spec_bytes(), self.spec_entries(), color_index)),
                _ => true,
            },
    {
        proof { use_type_invariant(self); }
        try_decode_rgb(self.bytes, self.entries.as_slice(), color_index)
    }
}

/// The mask index/type/length-error exits: no request is appended, so the log
/// is exactly the prefix it started from.
proof fn lemma_at_site_log_a_empty(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    pre: Seq<(AllocSite, nat, bool)>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        l == pre,
        req_a(b, E, i).len() == 0,
        crate::alloc_guard::all_ok(pre),
    ensures at_site_log_a(b, E, i, pre, false, l),
{
    reveal(at_site_log_a);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
}

/// The `MaskData`-failure exit: the log is `pre` plus one failed
/// `(MaskData, n)` entry, and `Req_A` is that one request.
proof fn lemma_at_site_log_a_oom(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    n: usize,
    o: OutOfMemory,
    pre: Seq<(AllocSite, nat, bool)>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        (i as int) < (E.len() as int),
        o.site_spec() == AllocSite::MaskData,
        o.elements_spec() == n,
        req_a(b, E, i) == seq![(AllocSite::MaskData, n as nat)],
        l == pre.push((AllocSite::MaskData, n as nat, false)),
        crate::alloc_guard::all_ok(pre),
    ensures at_site_log_a(b, E, i, pre, true, l),
{
    reveal(at_site_log_a);
    reveal(log_extends);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    let x = (AllocSite::MaskData, n as nat, false);
    vstd::seq::lemma_seq_push_len(pre, x);
    assert(l.len() == pre.len() + 1);
    assert(l.subrange(0, pre.len() as int) == pre);
    assert(l.subrange(pre.len() as int, l.len() as int) == seq![x]);
    assert(crate::alloc_guard::req_of_log(l.subrange(pre.len() as int, l.len() as int))
        == req_a(b, E, i));
    lemma_all_ok_push_false(pre, AllocSite::MaskData, n as nat, l);
}

/// The `MaskData`-success exit: the log is `pre` plus one successful
/// `(MaskData, n)` entry, and `Req_A` is that one request.
proof fn lemma_at_site_log_a_after_ok(
    b: Seq<u8>,
    E: Seq<Entry>,
    i: u32,
    n: usize,
    pre: Seq<(AllocSite, nat, bool)>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        (i as int) < (E.len() as int),
        req_a(b, E, i) == seq![(AllocSite::MaskData, n as nat)],
        l == pre.push((AllocSite::MaskData, n as nat, true)),
        crate::alloc_guard::all_ok(pre),
    ensures at_site_log_a(b, E, i, pre, false, l),
{
    reveal(at_site_log_a);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l)
        =~= crate::alloc_guard::req_of_log(pre) + req_a(b, E, i));
    lemma_all_ok_push_true(pre, AllocSite::MaskData, n as nat, l);
}

/// R13: decode a legacy mask entry to `N` alpha bytes, through the sealed
/// reservation log. On a non-`OutOfMemory` result it equals
/// `MA(bytes, entries, mask_index)` exactly (A-ICNS-5).
fn decode_mask_tracked(
    bytes: &[u8],
    entries: &[Entry],
    mask_index: u32,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
        crate::alloc_guard::all_ok(old(log).view()),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::mask_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                mask_index,
            ),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE
                &&& d.data@.len() == (d.width as usize) * (d.width as usize)
            },
            Err(_) => true,
        },
        at_site_log_a(bytes@, entries@, mask_index, old(log).view(),
            matches!(r, Err(TryError::OutOfMemory(_))), final(log).view()),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                final(log).view().len() >= 1
                && o.site_spec() == final(log).view().last().0
                && o.elements_spec() as nat == final(log).view().last().1,
            _ => true,
        },
{
    // The log this decode starts from: `combine` passes the colour decode's log.
    let ghost pre_a: Seq<(AllocSite, nat, bool)> = log.view();
    if mask_index as usize >= entries.len() {
        proof {
            crate::model::lemma_ma_index(bytes@, entries@, mask_index);
            reveal(req_a);
            lemma_at_site_log_a_empty(bytes@, entries@, mask_index, pre_a, log.view());
        }
        return Err(TryError::Format(Error::IndexOutOfRange));
    }
    let entry = &entries[mask_index as usize];
    proof {
        assert(crate::model::entry_inv(bytes@, entries@[mask_index as int]));
    }
    let code = entry.type_code();
    let (is_colour, side) = match legacy_side(code) {
        Some(v) => v,
        None => {
            proof {
                crate::model::lemma_ma_unsupported_none(bytes@, entries@, mask_index, code);
                reveal(req_a);
                reveal(crate::model::legacy_side);
                lemma_at_site_log_a_empty(bytes@, entries@, mask_index, pre_a, log.view());
            }
            return Err(TryError::Format(Error::UnsupportedType));
        },
    };
    if is_colour {
        proof {
            crate::model::lemma_ma_unsupported_colour(bytes@, entries@, mask_index, code, side);
            reveal(req_a);
            reveal(crate::model::legacy_side);
            lemma_at_site_log_a_empty(bytes@, entries@, mask_index, pre_a, log.view());
        }
        return Err(TryError::Format(Error::UnsupportedType));
    }
    proof {
        lemma_legacy_side_le_128(code);
        crate::model::lemma_legacy_side_ge_16(code);
        assert(16 <= side && side <= 128);
        assert((side as usize) * (side as usize) <= usize::MAX) by (nonlinear_arith)
            requires 16 <= side, side <= 128;
    }
    let n = (side as usize) * (side as usize);
    proof {
        assert(1 <= n) by (nonlinear_arith)
            requires 16 <= side, side <= 128, n == (side as usize) * (side as usize);
        assert(n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE) by (nonlinear_arith)
            requires side <= 128, n == (side as usize) * (side as usize);
    }
    let payload = payload_range(bytes, entry);
    if payload.len() != n {
        proof {
            crate::model::lemma_ma_length(
                bytes@,
                entries@,
                mask_index,
                code,
                side,
                n as int,
            );
            reveal(req_a);
            crate::model::lemma_entry_payload_bytes_len(bytes@, *entry);
            reveal(crate::model::entry_payload_bytes);
            lemma_at_site_log_a_empty(bytes@, entries@, mask_index, pre_a, log.view());
        }
        return Err(TryError::Format(Error::InvalidMaskLength));
    }
    // A-ICNS-5: the `MaskData` copy, `n <= 16384` bytes, iff the length check
    // passed (it just did).
    proof {
        reveal(req_a);
        crate::model::lemma_entry_payload_bytes_len(bytes@, *entry);
        reveal(crate::model::entry_payload_bytes);
        lemma_req_a_clause3(bytes@, entries@, mask_index);
        assert(req_a(bytes@, entries@, mask_index)
            == seq![(AllocSite::MaskData, n as nat)]);
    }
    let data = match crate::alloc_guard::sites::try_copy(AllocSite::MaskData, payload, Tracked(log)) {
        Ok(v) => v,
        Err(o) => {
            proof {
                lemma_at_site_log_a_oom(bytes@, entries@, mask_index, n, o, pre_a, log.view());
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        assert(data@ =~= payload@);
        crate::model::lemma_ma_ok(bytes@, entries@, mask_index, code, side, n as int, payload@);
        lemma_at_site_log_a_after_ok(bytes@, entries@, mask_index, n, pre_a, log.view());
    }
    Ok(Decoded {
        width: side,
        height: side,
        data,
    })
}

/// The logged mask decoder (A-ICNS-5): creates the one log and returns it.
pub(crate) fn try_decode_mask_logged(bytes: &[u8], entries: &[Entry], mask_index: u32)
    -> (r: (Result<Decoded, TryError>, Tracked<AllocLog>))
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        g_a(bytes@, entries@, mask_index, r.0, r.1@.view()),
        prog_a(bytes@, entries@, mask_index, r.0, r.1@.view()),
        crate::alloc_guard::sites::log_tied(r.0, r.1@.view()),
        pre_alloc_a_concl(bytes@, entries@, mask_index, r.0, r.1@.view()),
        match r.0 {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE
                &&& d.data@.len() == (d.width as usize) * (d.width as usize)
            },
            Err(_) => true,
        },
{
    let tracked mut log = crate::alloc_guard::new_log();
    let result = decode_mask_tracked(bytes, entries, mask_index, Tracked(&mut log));
    proof {
        reveal(g_a);
        reveal(g_a_clause1);
        reveal(g_a_clause2);
        reveal(g_a_clause3);
        reveal(prog_a);
        reveal(prog_a_concl);
        reveal(crate::alloc_guard::sites::log_tied);
        reveal(req_a);
        reveal(at_site_log_a);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        reveal(pre_alloc_a);
        reveal(pre_alloc_a_concl);
        assert(at_site_log_a(bytes@, entries@, mask_index,
            Seq::<(AllocSite, nat, bool)>::empty(),
            matches!(result, Err(TryError::OutOfMemory(_))), log.view()));
        if let Err(TryError::OutOfMemory(_)) = result {
            reveal(log_extends);
            assert(crate::alloc_guard::req_of_log(
                log.view().subrange(0, log.view().len() as int))
                == crate::alloc_guard::req_of_log(log.view()));
            assert(crate::alloc_guard::req_of_log(log.view())
                .is_prefix_of(req_a(bytes@, entries@, mask_index)));
        } else {
            assert(crate::alloc_guard::req_of_log(log.view())
                == req_a(bytes@, entries@, mask_index));
            assert(crate::alloc_guard::all_ok(log.view()));
        }
        lemma_g_mask_prealloc(bytes@, entries@, mask_index, result, log.view());
    }
    (result, Tracked(log))
}

/// The public mask decoder (A-ICNS-5): `try_decode_mask_logged` with the log
/// discarded (decision D1: the frozen name `decode_mask` is removed).
///
/// `OutOfMemory` names its failed request: `(o.site(), o.elements())` is one of
/// `Req_A`'s requests. `try_decode_mask_logged` already establishes the `site`,
/// the size, the ordered prefix and the failed last event; this clause reads
/// them out of that log.
pub(crate) fn try_decode_mask(bytes: &[u8], entries: &[Entry], mask_index: u32)
    -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::mask_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                mask_index,
            ),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE
                &&& d.data@.len() == (d.width as usize) * (d.width as usize)
            },
            Err(_) => true,
        },
        match r {
            Err(TryError::OutOfMemory(o)) =>
                oom_in_req(o.site_spec(), o.elements_spec() as nat,
                    req_a(bytes@, entries@, mask_index)),
            _ => true,
        },
{
    let res = try_decode_mask_logged(bytes, entries, mask_index);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(oom_in_req);
        reveal(g_a);
        reveal(g_a_clause2);
        reveal(req_a);
        reveal(crate::alloc_guard::req_of_log);
        if let Err(TryError::OutOfMemory(o)) = res.0 {
            assert(g_a(bytes@, entries@, mask_index, res.0, res.1@.view()));
            assert(g_a_clause2(bytes@, entries@, mask_index, res.0, res.1@.view()));
            assert(res.1@.view().len() >= 1);
            assert(crate::alloc_guard::req_of_log(res.1@.view())
                .is_prefix_of(req_a(bytes@, entries@, mask_index)));
            assert(res.1@.view().last().0 == o.site_spec());
            assert(res.1@.view().last().1 == o.elements_spec() as nat);
            let j = res.1@.view().len() - 1;
            assert(0 <= j < req_a(bytes@, entries@, mask_index).len());
            assert(crate::alloc_guard::req_of_log(res.1@.view())[j]
                == (o.site_spec(), o.elements_spec() as nat));
            assert(req_a(bytes@, entries@, mask_index)[j]
                == (o.site_spec(), o.elements_spec() as nat));
            assert(oom_in_req(o.site_spec(), o.elements_spec() as nat,
                req_a(bytes@, entries@, mask_index)));
        }
    }
    res.0
}

impl<'a> Parsed<'a> {
    /// The public mask decoder (A-ICNS-5) over the handle: `try_decode_mask`
    /// with the bytes and entries the handle holds. No `requires` clause; the
    /// type invariant supplies `parsed_inv`, and the `ensures` restates the
    /// internal postcondition in handle terms.
    pub fn try_decode_mask(&self, mask_index: u32) -> (r: Result<Decoded, TryError>)
        ensures
            !matches!(r, Err(TryError::OutOfMemory(_)))
                ==> crate::model::mask_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(r),
                    self.spec_bytes(),
                    self.spec_entries(),
                    mask_index,
                ),
            match r {
                Ok(d) => {
                    &&& d.width == d.height
                    &&& d.width <= 128
                    &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE
                    &&& d.data@.len() == (d.width as usize) * (d.width as usize)
                },
                Err(_) => true,
            },
            match r {
                Err(TryError::OutOfMemory(o)) =>
                    oom_in_req(o.site_spec(), o.elements_spec() as nat,
                        req_a(self.spec_bytes(), self.spec_entries(), mask_index)),
                _ => true,
            },
    {
        proof { use_type_invariant(self); }
        try_decode_mask(self.bytes, self.entries.as_slice(), mask_index)
    }
}

/// R14: combine a colour and a mask entry into interleaved RGBA. The result
/// equals `MJ(bytes, entries, color_index, mask_index)` exactly (T12): the
/// absent-index precedence, RGB before mask error precedence, the side check
/// and every RGBA byte.
// ---------------------------------------------------------------------------
// MJ (combine) with the alloc-delta guard (A-ICNS-6).
// ---------------------------------------------------------------------------

/// `Req_J(b,E,ci,mi)` (SPEC.md §4, A-ICNS-6): `Req_C(ci)`, then `Req_A(mi)` if
/// the colour decode is `Ok`, then `(CombineOut, 4n)` if both are `Ok` and the
/// sides match. An absent index gives `[]` (`MJ`'s `MissingColor`/`MissingMask`
/// precedence). Stated in terms of `mc`/`ma`, so it is a model statement.
#[doc(hidden)]
pub open spec fn req_j(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
) -> Seq<(AllocSite, nat)> {
    match ci {
        None => Seq::empty(),
        Some(c) => match mi {
            None => Seq::empty(),
            Some(m) => {
                let rc = req_c(b, E, c);
                if crate::model::mc(b, E, c) is Ok {
                    let mcv = crate::model::mc(b, E, c)->Ok_0;
                    let ra = req_a(b, E, m);
                    if crate::model::ma(b, E, m) is Ok {
                        let mav = crate::model::ma(b, E, m)->Ok_0;
                        if mcv.0 == mav.0 && mcv.1 == mav.1 {
                            (rc + ra).push((
                                AllocSite::CombineOut,
                                4 * (mcv.0 as nat) * (mcv.1 as nat),
                            ))
                        } else {
                            rc + ra
                        }
                    } else {
                        rc + ra
                    }
                } else {
                    rc
                }
            },
        },
    }
}

#[doc(hidden)]
pub open spec fn g_j_clause1(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::combine_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi)
            && crate::alloc_guard::req_of_log(l) == req_j(b, E, ci, mi)
            && crate::alloc_guard::all_ok(l),
    }
}

#[doc(hidden)]
pub open spec fn g_j_clause2(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_j(b, E, ci, mi))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G` for `try_combine`: every request is within its own site's
/// maximum (channels 16,384, `RgbOut` 49,152, `MaskData` 16,384, `CombineOut`
/// 65,536), from `parsed_inv` (legacy sides <= 128, so `4n <= 65536`).
#[doc(hidden)]
pub open spec fn g_j_clause3(b: Seq<u8>, E: Seq<Entry>, ci: Option<u32>, mi: Option<u32>) -> bool {
    req_within_site_max(req_j(b, E, ci, mi))
}

#[doc(hidden)]
pub open spec fn g_j(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_j_clause1(b, E, ci, mi, r, l) && g_j_clause2(b, E, ci, mi, r, l)
        && g_j_clause3(b, E, ci, mi)
}

#[doc(hidden)]
pub open spec fn prog_j(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_j_concl(b, E, ci, mi, r, l)
}

#[doc(hidden)]
pub open spec fn prog_j_concl(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && crate::model::combine_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi)
        && crate::alloc_guard::req_of_log(l) == req_j(b, E, ci, mi)
}

#[doc(hidden)]
pub open spec fn at_site_log_j(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    if r_is_oom {
        crate::alloc_guard::req_of_log(l).is_prefix_of(req_j(b, E, ci, mi))
            && l.len() >= 1
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
    } else {
        crate::alloc_guard::req_of_log(l) == req_j(b, E, ci, mi)
            && crate::alloc_guard::all_ok(l)
    }
}

/// Every element of `Req_R(n)` is at a `Channel` site with `n <= 16384`, which
/// is `site_max(Channel(_))`, for a legacy side `s` with `n = s*s`. This is the
/// base of `Req_C`'s and `Req_A`'s per-site bounds.
proof fn lemma_req_r_elements_bound(n: nat)
    requires n <= 16384,
    ensures req_within_site_max(req_r(n)),
{
    reveal(req_r);
    reveal(site_max);
    reveal(req_within_site_max);
    assert forall|j: int| 0 <= j < req_r(n).len()
        implies #[trigger] req_r(n)[j].1 <= site_max(req_r(n)[j].0) by {
        if j == 0 {
            assert(req_r(n)[0] == (AllocSite::Channel(0u8), n));
        } else if j == 1 {
            assert(req_r(n)[1] == (AllocSite::Channel(1u8), n));
        } else {
            assert(j == 2);
            assert(req_r(n)[2] == (AllocSite::Channel(2u8), n));
        }
    }
}

/// Appending one request to a within-bounds list keeps it within bounds, when
/// the new request is within its own site's maximum.
proof fn lemma_within_site_max_push(req: Seq<(AllocSite, nat)>, site: AllocSite, k: nat)
    requires
        req_within_site_max(req),
        k <= site_max(site),
    ensures
        req_within_site_max(req.push((site, k))),
{
    reveal(req_within_site_max);
    reveal(site_max);
    let x = (site, k);
    vstd::seq::lemma_seq_push_len(req, x);
    assert forall|j: int| 0 <= j < req.push(x).len()
        implies #[trigger] req.push(x)[j].1 <= site_max(req.push(x)[j].0) by {
        if j < req.len() {
            vstd::seq::lemma_seq_push_index_different(req, x, j);
            assert(req.push(x)[j] == req[j]);
        } else {
            assert(j == req.len());
            vstd::seq::lemma_seq_push_index_same(req, x, j);
            assert(req.push(x)[j] == x);
        }
    }
}

/// Every element of `Req_C(b,E,i)` is within its own site's maximum (an RLE
/// channel `n <= 16384` or `RgbOut` `3n <= 49152`), from `parsed_inv`.
proof fn lemma_req_c_bound(b: Seq<u8>, E: Seq<Entry>, i: u32)
    ensures req_within_site_max(req_c(b, E, i)),
{
    reveal(req_c);
    reveal(req_r);
    reveal(site_max);
    reveal(req_within_site_max);
    if (i as int) < (E.len() as int) {
        let e = E[i as int];
        let code = crate::model::entry_type_code(e);
        crate::model::lemma_legacy_side_n_bound(code);
        match crate::model::legacy_side(code) {
            Some((true, side)) => {
                let n: nat = ((side as int) * (side as int)) as nat;
                assert(n <= 16384);
                lemma_req_r_elements_bound(n);
                let p2 = crate::model::strip_it32(code, crate::model::entry_payload_bytes(b, e));
                match crate::model::mr(p2, n as int) {
                    Ok(_) => {
                        assert(req_c(b, E, i) == req_r(n).push((AllocSite::RgbOut, 3 * n)));
                        assert(3 * n <= 49152) by (nonlinear_arith) requires n <= 16384;
                        assert(3 * n <= site_max(AllocSite::RgbOut));
                        lemma_within_site_max_push(req_r(n), AllocSite::RgbOut, 3 * n);
                    },
                    Err(_) => {
                        assert(req_c(b, E, i) == req_r(n));
                    },
                }
            },
            _ => {},
        }
    }
}

/// Every element of `Req_A(b,E,i)` is at a `MaskData` site with `n <= 16384`,
/// from `parsed_inv`.
proof fn lemma_req_a_bound(b: Seq<u8>, E: Seq<Entry>, i: u32)
    ensures req_within_site_max(req_a(b, E, i)),
{
    reveal(req_a);
    reveal(site_max);
    reveal(req_within_site_max);
    if (i as int) < (E.len() as int) {
        let e = E[i as int];
        let code = crate::model::entry_type_code(e);
        crate::model::lemma_legacy_side_n_bound(code);
        match crate::model::legacy_side(code) {
            Some((false, side)) => {
                let n: nat = ((side as int) * (side as int)) as nat;
                let p = crate::model::entry_payload_bytes(b, e);
                if p.len() == n as int {
                    assert(req_a(b, E, i) == seq![(AllocSite::MaskData, n)]);
                    assert(n <= 16384);
                } else {
                    assert(req_a(b, E, i) == Seq::empty());
                }
            },
            _ => {},
        }
    }
}

/// Clause 3 for `try_combine`: every request is within its own site's maximum,
/// from `parsed_inv` (legacy sides <= 128, so `4n <= 65536`).
proof fn lemma_req_j_clause3(b: Seq<u8>, E: Seq<Entry>, ci: Option<u32>, mi: Option<u32>)
    ensures g_j_clause3(b, E, ci, mi),
{
    reveal(g_j_clause3);
    reveal(req_j);
    reveal(req_c);
    reveal(req_a);
    reveal(req_r);
    reveal(site_max);
    reveal(req_within_site_max);
    if let Some(c) = ci {
        lemma_req_c_bound(b, E, c);
    }
    if let Some(m) = mi {
        lemma_req_a_bound(b, E, m);
    }
    assert forall|j: int| 0 <= j < req_j(b, E, ci, mi).len()
        implies req_j(b, E, ci, mi)[j].1 <= site_max(req_j(b, E, ci, mi)[j].0) by {
        match ci {
            None => {},
            Some(c) => {
                match mi {
                    None => {},
                    Some(m) => {
                        if crate::model::mc(b, E, c) is Ok && crate::model::ma(b, E, m) is Ok {
                            let mcv = crate::model::mc(b, E, c)->Ok_0;
                            let mav = crate::model::ma(b, E, m)->Ok_0;
                            if mcv.0 == mav.0 && mcv.1 == mav.1 {
                                assert(mcv.0 <= 128) by {
                                    reveal(crate::model::mc);
                                    if (c as int) < (E.len() as int) {
                                        crate::model::lemma_legacy_side_bound(
                                            crate::model::entry_type_code(E[c as int]));
                                    }
                                }
                                assert(4 * (mcv.0 as nat) * (mcv.1 as nat) <= 65536) by (nonlinear_arith)
                                    requires mcv.0 <= 128, mcv.1 <= 128;
                            }
                        }
                    },
                }
            },
        }
    }
}

/// The named `PreAlloc` corollary for `try_combine` (THEOREMS-ALLOC-DELTA.md §0,
/// A-ICNS-6): `Req_J == []` implies non-`OutOfMemory`, `combine_decoded_eq` on
/// `fmt_part(r)`, and `L == []`.
pub(crate) proof fn lemma_g_combine_prealloc(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_j(b, E, ci, mi, r, l),
    ensures
        pre_alloc_j(b, E, ci, mi) ==> !matches!(r, Err(TryError::OutOfMemory(_))),
        pre_alloc_j(b, E, ci, mi) ==> crate::model::combine_decoded_eq(
            crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi),
        pre_alloc_j(b, E, ci, mi) ==> l == Seq::<(AllocSite, nat, bool)>::empty(),
        pre_alloc_j_concl(b, E, ci, mi, r, l),
{
    reveal(g_j);
    reveal(g_j_clause1);
    reveal(g_j_clause2);
    reveal(pre_alloc_j);
    reveal(pre_alloc_j_concl);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    if pre_alloc_j(b, E, ci, mi) {
        if let Err(TryError::OutOfMemory(_)) = r {
            assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_j(b, E, ci, mi)));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
            assert(l.len() <= req_j(b, E, ci, mi).len());
            assert(l.len() >= 1);
            assert(false);
        }
        match r {
            Err(TryError::OutOfMemory(_)) => { assert(false); },
            _ => {
                assert(crate::model::combine_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi));
                assert(crate::alloc_guard::req_of_log(l) == req_j(b, E, ci, mi));
                assert(req_j(b, E, ci, mi).len() == 0);
                assert(l.len() == 0);
                assert(l =~= Seq::<(AllocSite, nat, bool)>::empty());
            },
        }
    }
}

/// The public `try_combine` contract implies the `PreAlloc` corollary's
/// observable parts.
pub(crate) proof fn lemma_public_combine_prealloc(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    r: Result<Decoded, TryError>,
)
    requires
        pre_alloc_j(b, E, ci, mi),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                oom_in_req(o.site_spec(), o.elements_spec() as nat, req_j(b, E, ci, mi)),
            _ => true,
        },
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::combine_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::combine_decoded_eq(crate::alloc_guard::sites::fmt_part(r), b, E, ci, mi),
{
    reveal(pre_alloc_j);
    reveal(oom_in_req);
    if let Err(TryError::OutOfMemory(o)) = r {
        assert(oom_in_req(o.site_spec(), o.elements_spec() as nat, req_j(b, E, ci, mi)));
        assert(req_j(b, E, ci, mi).len() == 0);
        assert(!oom_in_req(o.site_spec(), o.elements_spec() as nat, req_j(b, E, ci, mi)));
        assert(false);
    }
}

/// With both decodes `Ok` at matching sides, `Req_J` is `Req_C ++ Req_A` plus
/// the one `CombineOut` element. Stated from the `mc`/`ma` results so the
/// `CombineOut` exit and the `Ok` exit can unfold `req_j` without re-deriving
/// the model's `Ok` shapes.
proof fn lemma_req_j_ok_eq_mcv(
    b: Seq<u8>,
    E: Seq<Entry>,
    c: u32,
    m: u32,
    w: u32,
    h: u32,
    n: nat,
)
    requires
        crate::model::mc(b, E, c) is Ok,
        crate::model::mc(b, E, c)->Ok_0.0 == w,
        crate::model::mc(b, E, c)->Ok_0.1 == h,
        crate::model::ma(b, E, m) is Ok,
        crate::model::ma(b, E, m)->Ok_0.0 == w,
        crate::model::ma(b, E, m)->Ok_0.1 == h,
        n == ((w as int) * (h as int)) as nat,
    ensures
        req_j(b, E, Some(c), Some(m))
            == (req_c(b, E, c) + req_a(b, E, m)).push((AllocSite::CombineOut, 4 * n)),
{
    reveal(req_j);
    reveal(req_c);
    reveal(req_a);
    let mcv = crate::model::mc(b, E, c)->Ok_0;
    let mav = crate::model::ma(b, E, m)->Ok_0;
    assert(mcv.0 == mav.0 && mcv.1 == mav.1);
    assert(4 * (mcv.0 as nat) * (mcv.1 as nat) == 4 * n) by (nonlinear_arith)
        requires mcv.0 == w, mcv.1 == h, n == ((w as int) * (h as int)) as nat;
}

/// `req_of_log` of `pre ++ tail` when `pre`'s requests are exactly `rc` and
/// `tail`'s are a prefix of `ra`: a prefix of `rc ++ ra`. This is the `combine`
/// mask-OOM case, where the mask decode appended a prefix of `Req_A` to the
/// colour decode's `Req_C`.
proof fn lemma_req_prefix_compose(
    pre: Seq<(AllocSite, nat, bool)>,
    tail: Seq<(AllocSite, nat, bool)>,
    rc: Seq<(AllocSite, nat)>,
    ra: Seq<(AllocSite, nat)>,
)
    requires
        crate::alloc_guard::req_of_log(pre) == rc,
        crate::alloc_guard::req_of_log(tail).is_prefix_of(ra),
    ensures
        crate::alloc_guard::req_of_log(pre + tail).is_prefix_of(rc + ra),
{
    reveal(crate::alloc_guard::req_of_log);
    lemma_req_of_log_add(pre, tail);
    assert(crate::alloc_guard::req_of_log(pre + tail)
        == crate::alloc_guard::req_of_log(pre) + crate::alloc_guard::req_of_log(tail));
    assert((crate::alloc_guard::req_of_log(pre) + crate::alloc_guard::req_of_log(tail))
        .is_prefix_of(rc + ra)) by {
        assert(crate::alloc_guard::req_of_log(tail).len() <= ra.len());
        assert(crate::alloc_guard::req_of_log(pre + tail).len() <= (rc + ra).len());
        assert forall|j: int| 0 <= j < crate::alloc_guard::req_of_log(pre + tail).len()
            implies (#[trigger] crate::alloc_guard::req_of_log(pre + tail)[j]
                == (rc + ra)[j]) by {
            if j < crate::alloc_guard::req_of_log(pre).len() {
                assert(crate::alloc_guard::req_of_log(pre).len() == rc.len());
                assert(crate::alloc_guard::req_of_log(pre + tail)[j]
                    == crate::alloc_guard::req_of_log(pre)[j]);
            } else {
                assert(crate::alloc_guard::req_of_log(pre + tail)[j]
                    == crate::alloc_guard::req_of_log(tail)[j - rc.len()]);
            }
        }
    }
}

/// `req_of_log` distributes over concatenation: the requests of `a + b` are the
/// requests of `a` followed by the requests of `b`. This is what lets `combine`
/// read the mask decode's log as `Req_C ++ Req_A`.
proof fn lemma_req_of_log_add(a: Seq<(AllocSite, nat, bool)>, b: Seq<(AllocSite, nat, bool)>)
    ensures
        crate::alloc_guard::req_of_log(a + b)
            == crate::alloc_guard::req_of_log(a) + crate::alloc_guard::req_of_log(b),
{
    reveal(crate::alloc_guard::req_of_log);
    assert(crate::alloc_guard::req_of_log(a + b)
        =~= crate::alloc_guard::req_of_log(a) + crate::alloc_guard::req_of_log(b)) by {
        assert forall|j: int| 0 <= j < (a + b).len() implies
            #[trigger] crate::alloc_guard::req_of_log(a + b)[j]
                == (crate::alloc_guard::req_of_log(a) + crate::alloc_guard::req_of_log(b))[j] by {
            if j < a.len() {
                assert((a + b)[j] == a[j]);
            } else {
                assert((a + b)[j] == b[j - a.len()]);
            }
        }
    }
}

/// The absent-index exit: `Req_J` is `[]`, so is the log.
proof fn lemma_at_site_log_j_empty(
    b: Seq<u8>,
    E: Seq<Entry>,
    ci: Option<u32>,
    mi: Option<u32>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        l == Seq::<(AllocSite, nat, bool)>::empty(),
        req_j(b, E, ci, mi).len() == 0,
    ensures at_site_log_j(b, E, ci, mi, false, l),
{
    reveal(at_site_log_j);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    lemma_all_ok_empty();
}

/// R14: combine a colour and a mask entry into interleaved RGBA, through the
/// sealed reservation log. On a non-`OutOfMemory` result it equals
/// `MJ(bytes, entries, color_index, mask_index)` exactly (A-ICNS-6): the
/// absent-index precedence, RGB before mask error precedence, the side check and
/// every RGBA byte.
///
/// The order is `Req_C(ci)`, then `Req_A(mi)` (only if the colour decode was
/// `Ok`), then `(CombineOut, 4n)` (only if both are `Ok` and the sides match).
fn combine_tracked(
    bytes: &[u8],
    entries: &[Entry],
    color_index: Option<u32>,
    mask_index: Option<u32>,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::combine_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                color_index,
                mask_index,
            ),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 4
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 4
            },
            Err(_) => true,
        },
        at_site_log_j(bytes@, entries@, color_index, mask_index,
            matches!(r, Err(TryError::OutOfMemory(_))), final(log).view()),
        match r {
            Err(TryError::OutOfMemory(o)) =>
                final(log).view().len() >= 1
                && o.site_spec() == final(log).view().last().0
                && o.elements_spec() as nat == final(log).view().last().1,
            _ => true,
        },
{
    let ci = match color_index {
        Some(i) => i,
        None => {
            proof {
                assert(crate::model::mj(bytes@, entries@, None, mask_index)
                    == Err(Error::MissingColor));
                reveal(req_j);
                reveal(at_site_log_j);
                reveal(crate::alloc_guard::req_of_log);
                reveal(crate::alloc_guard::all_ok);
                assert(req_j(bytes@, entries@, None, mask_index) == Seq::<(AllocSite, nat)>::empty());
                assert(crate::alloc_guard::req_of_log(log.view()) == req_j(bytes@, entries@, None, mask_index));
                assert(crate::alloc_guard::all_ok(log.view()));
            }
            return Err(TryError::Format(Error::MissingColor));
        },
    };
    let mi = match mask_index {
        Some(i) => i,
        None => {
            proof {
                assert(crate::model::mj(bytes@, entries@, Some(ci), None)
                    == Err(Error::MissingMask));
                reveal(req_j);
                reveal(at_site_log_j);
                reveal(crate::alloc_guard::req_of_log);
                reveal(crate::alloc_guard::all_ok);
                assert(req_j(bytes@, entries@, Some(ci), None) == Seq::<(AllocSite, nat)>::empty());
                assert(crate::alloc_guard::req_of_log(log.view()) == req_j(bytes@, entries@, Some(ci), None));
                assert(crate::alloc_guard::all_ok(log.view()));
            }
            return Err(TryError::Format(Error::MissingMask));
        },
    };
    proof { lemma_req_j_clause3(bytes@, entries@, Some(ci), Some(mi)); }
    // A-ICNS-6: the colour decode first, sharing the log. Its `at_site_log_c` is
    // exactly `Req_C`'s prefix, which is also `Req_J`'s prefix.
    let rgb = match decode_rgb_tracked(bytes, entries, ci, Tracked(log)) {
        Ok(d) => {
            proof {
                reveal(at_site_log_c);
                reveal(crate::alloc_guard::req_of_log);
                reveal(crate::alloc_guard::all_ok);
                assert(crate::alloc_guard::req_of_log(log.view()) == req_c(bytes@, entries@, ci));
                assert(crate::alloc_guard::all_ok(log.view()));
            }
            d
        },
        Err(TryError::Format(e)) => {
            proof {
                reveal(at_site_log_j);
                reveal(crate::alloc_guard::sites::fmt_part);
                reveal(req_j);
                reveal(crate::model::decoded_eq);
                crate::model::lemma_mj_mc_err(bytes@, entries@, ci, mi, e);
                assert(final(log).view().len() <= req_j(bytes@, entries@, Some(ci), Some(mi)).len());
            }
            return Err(TryError::Format(e));
        },
        Err(TryError::OutOfMemory(o)) => {
            proof {
                reveal(at_site_log_j);
                reveal(req_j);
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    // The colour decode's log is the prefix the mask decode starts from.
    let ghost pre_mask: Seq<(AllocSite, nat, bool)> = log.view();
    proof {
        reveal(at_site_log_c);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        assert(crate::alloc_guard::req_of_log(pre_mask) == req_c(bytes@, entries@, ci));
        assert(crate::alloc_guard::all_ok(pre_mask));
    }
    let mask = match decode_mask_tracked(bytes, entries, mi, Tracked(log)) {
        Ok(d) => {
            proof {
                reveal(at_site_log_a);
                reveal(crate::alloc_guard::req_of_log);
                reveal(crate::alloc_guard::all_ok);
                assert(crate::alloc_guard::req_of_log(log.view())
                    == crate::alloc_guard::req_of_log(pre_mask) + req_a(bytes@, entries@, mi));
                assert(crate::alloc_guard::all_ok(log.view()));
            }
            d
        },
        Err(TryError::Format(e)) => {
            proof {
                reveal(at_site_log_j);
                reveal(crate::alloc_guard::sites::fmt_part);
                reveal(req_j);
                reveal(crate::model::decoded_eq);
                crate::model::lemma_mj_ma_err(bytes@, entries@, ci, mi, rgb.width, rgb.height, rgb.data@, e);
            }
            return Err(TryError::Format(e));
        },
        Err(TryError::OutOfMemory(o)) => {
            proof {
                reveal(at_site_log_j);
                reveal(at_site_log_a);
                reveal(log_extends);
                reveal(req_j);
                reveal(req_c);
                reveal(req_a);
                reveal(crate::alloc_guard::req_of_log);
                let tail = log.view().subrange(pre_mask.len() as int, log.view().len() as int);
                assert(crate::alloc_guard::req_of_log(
                    log.view().subrange(pre_mask.len() as int, log.view().len() as int))
                    .is_prefix_of(req_a(bytes@, entries@, mi)));
                assert(log.view() =~= pre_mask + tail) by {
                    assert forall|j: int| 0 <= j < log.view().len() implies
                        #[trigger] log.view()[j] == (pre_mask + tail)[j] by {
                        if j < pre_mask.len() {
                            assert((pre_mask + tail)[j] == pre_mask[j]);
                            assert(log.view().subrange(0, pre_mask.len() as int)[j]
                                == pre_mask[j]);
                        } else {
                            assert((pre_mask + tail)[j] == tail[j - pre_mask.len()]);
                        }
                    }
                }
                lemma_req_prefix_compose(
                    pre_mask, tail,
                    req_c(bytes@, entries@, ci), req_a(bytes@, entries@, mi));
                assert(crate::alloc_guard::req_of_log(log.view())
                    .is_prefix_of(req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi)));
                assert((req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi))
                    .is_prefix_of(req_j(bytes@, entries@, Some(ci), Some(mi))));
                assert forall|j: int| 0 <= j < log.view().len() - 1
                    implies #[trigger] log.view()[j].2 by {};
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    if rgb.width != mask.width || rgb.height != mask.height {
        proof {
            assert(crate::model::mc(bytes@, entries@, ci) == Ok((rgb.width, rgb.height, rgb.data@)));
            assert(crate::model::ma(bytes@, entries@, mi) == Ok((mask.width, mask.height, mask.data@)));
            assert(crate::model::mj(bytes@, entries@, Some(ci), Some(mi))
                == Err(Error::TypeMismatch));
            reveal(at_site_log_j);
            reveal(req_j);
            reveal(req_c);
            reveal(req_a);
            reveal(req_r);
            reveal(crate::alloc_guard::req_of_log);
            reveal(crate::alloc_guard::all_ok);
            assert(crate::alloc_guard::req_of_log(log.view())
                == req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi));
            // `req_j` is `Req_C ++ Req_A` here: both decodes are `Ok` but the
            // sides differ, so no `CombineOut` element is appended.
            assert(req_j(bytes@, entries@, Some(ci), Some(mi))
                == req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi));
            assert(crate::alloc_guard::req_of_log(log.view())
                .is_prefix_of(req_j(bytes@, entries@, Some(ci), Some(mi))));
            assert forall|j: int| 0 <= j < log.view().len() - 1 implies #[trigger] log.view()[j].2 by {};
        }
        return Err(TryError::Format(Error::TypeMismatch));
    }
    let n = (rgb.width as usize) * (rgb.height as usize);
    proof {
        reveal(req_j);
        reveal(req_c);
        reveal(req_a);
        reveal(req_r);
        assert(crate::model::mc(bytes@, entries@, ci) == Ok((rgb.width, rgb.height, rgb.data@)));
        assert(crate::model::ma(bytes@, entries@, mi) == Ok((mask.width, mask.height, mask.data@)));
        assert(rgb.width == mask.width && rgb.height == mask.height);
        // `rgb.width` is a legacy side, so `16 <= width <= 128` and `n >= 1`.
        assert(1 <= rgb.width && rgb.width <= 128) by {
            reveal(crate::model::mc);
            if (ci as int) < (entries@.len() as int) {
                crate::model::lemma_legacy_side_ge_16(
                    crate::model::entry_type_code(entries@[ci as int]));
            }
        }
        assert(1 <= n) by (nonlinear_arith)
            requires 1 <= rgb.width, 1 <= rgb.height, n == (rgb.width as usize) * (rgb.height as usize);
        assert(n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE) by (nonlinear_arith)
            requires rgb.width <= 128, rgb.height <= 128,
                n == (rgb.width as usize) * (rgb.height as usize);
    }
    // A-ICNS-6: the `CombineOut` reservation, after both sub-decodes and the
    // matching side check, while both buffers are live (peak `8n`).
    let mut out = match crate::alloc_guard::try_zeroed(AllocSite::CombineOut, 4 * n, Tracked(log)) {
        Ok(v) => v,
        Err(o) => {
            proof {
                reveal(at_site_log_j);
                reveal(req_j);
                reveal(req_c);
                reveal(req_a);
                reveal(req_r);
                reveal(crate::alloc_guard::req_of_log);
                reveal(crate::alloc_guard::all_ok);
                assert(crate::alloc_guard::req_of_log(log.view())
                    == (req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi))
                        .push((AllocSite::CombineOut, 4 * n as nat)));
                lemma_req_j_ok_eq_mcv(
                    bytes@, entries@, ci, mi, rgb.width, rgb.height, n as nat);
                assert(req_j(bytes@, entries@, Some(ci), Some(mi))
                    == (req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi))
                        .push((AllocSite::CombineOut, 4 * n as nat)));
                assert(crate::alloc_guard::req_of_log(log.view())
                    .is_prefix_of(req_j(bytes@, entries@, Some(ci), Some(mi))));
                assert forall|j: int| 0 <= j < log.view().len() - 1 implies #[trigger] log.view()[j].2 by {};
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    proof { reveal(crate::alloc_guard::sites::fmt_part); }
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
            out@.len() == 4 * n,
            rgb.data@.len() == 3 * n,
            mask.data@.len() == n,
            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k] == rgb.data@[3 * k],
            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k + 1] == rgb.data@[3 * k + 1],
            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k + 2] == rgb.data@[3 * k + 2],
            forall|k: int| 0 <= k < i ==> #[trigger] out@[4 * k + 3] == mask.data@[k],
        decreases n - i,
    {
        out[4 * i] = rgb.data[3 * i];
        out[4 * i + 1] = rgb.data[3 * i + 1];
        out[4 * i + 2] = rgb.data[3 * i + 2];
        out[4 * i + 3] = mask.data[i];
        i = i + 1;
    }
    let d = Decoded {
        width: rgb.width,
        height: rgb.height,
        data: out,
    };
    proof {
        crate::model::lemma_interleave_rgba_exact(rgb.data@, mask.data@, d.data@);
        assert(crate::model::mc(bytes@, entries@, ci) == Ok((rgb.width, rgb.height, rgb.data@)));
        assert(crate::model::ma(bytes@, entries@, mi) == Ok((mask.width, mask.height, mask.data@)));
        assert(crate::model::mj(bytes@, entries@, Some(ci), Some(mi))
            == Ok((rgb.width, rgb.height, crate::model::interleave_rgba(rgb.data@, mask.data@))));
        reveal(at_site_log_j);
        reveal(req_j);
        reveal(req_c);
        reveal(req_a);
        reveal(req_r);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        lemma_req_j_ok_eq_mcv(
            bytes@, entries@, ci, mi, rgb.width, rgb.height, n as nat);
        assert(req_j(bytes@, entries@, Some(ci), Some(mi))
            == (req_c(bytes@, entries@, ci) + req_a(bytes@, entries@, mi))
                .push((AllocSite::CombineOut, 4 * n as nat)));
        assert(crate::alloc_guard::req_of_log(log.view())
            == req_j(bytes@, entries@, Some(ci), Some(mi)));
        assert forall|j: int| 0 <= j < log.view().len() implies #[trigger] log.view()[j].2 by {};
    }
    Ok(d)
}

/// The logged combine decoder (A-ICNS-6): creates the one log and returns it.
pub(crate) fn try_combine_logged(
    bytes: &[u8],
    entries: &[Entry],
    color_index: Option<u32>,
    mask_index: Option<u32>,
) -> (r: (Result<Decoded, TryError>, Tracked<AllocLog>))
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        g_j(bytes@, entries@, color_index, mask_index, r.0, r.1@.view()),
        prog_j(bytes@, entries@, color_index, mask_index, r.0, r.1@.view()),
        crate::alloc_guard::sites::log_tied(r.0, r.1@.view()),
        pre_alloc_j_concl(bytes@, entries@, color_index, mask_index, r.0, r.1@.view()),
        match r.0 {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 4
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 4
            },
            Err(_) => true,
        },
{
    let tracked mut log = crate::alloc_guard::new_log();
    let result = combine_tracked(bytes, entries, color_index, mask_index, Tracked(&mut log));
    proof {
        reveal(g_j);
        reveal(g_j_clause1);
        reveal(g_j_clause2);
        reveal(g_j_clause3);
        reveal(prog_j);
        reveal(prog_j_concl);
        reveal(crate::alloc_guard::sites::log_tied);
        reveal(req_j);
        reveal(at_site_log_j);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        reveal(pre_alloc_j);
        reveal(pre_alloc_j_concl);
        assert(at_site_log_j(bytes@, entries@, color_index, mask_index,
            matches!(result, Err(TryError::OutOfMemory(_))), log.view()));
        assert(g_j_clause3(bytes@, entries@, color_index, mask_index)) by {
            lemma_req_j_clause3(bytes@, entries@, color_index, mask_index);
        }
        match result {
            Err(TryError::OutOfMemory(o)) => {
                reveal(crate::alloc_guard::sites::fmt_part);
                reveal(g_j_clause2);
                assert(crate::alloc_guard::req_of_log(log.view())
                    .is_prefix_of(req_j(bytes@, entries@, color_index, mask_index)));
                // `combine_tracked`'s own postcondition names the failed request.
                assert(log.view().len() >= 1);
                assert(o.site_spec() == log.view().last().0);
                assert(o.elements_spec() as nat == log.view().last().1);
                assert(!log.view()[log.view().len() - 1].2);
                assert forall|j: int| 0 <= j < log.view().len() - 1
                    implies #[trigger] log.view()[j].2 by {};
            },
            _ => {
                reveal(crate::alloc_guard::sites::fmt_part);
                reveal(g_j_clause1);
                reveal(prog_j_concl);
                reveal(crate::model::combine_decoded_eq);
                assert(crate::alloc_guard::req_of_log(log.view())
                    == req_j(bytes@, entries@, color_index, mask_index));
                assert(crate::alloc_guard::all_ok(log.view()));
            },
        }
        lemma_g_combine_prealloc(bytes@, entries@, color_index, mask_index, result, log.view());
    }
    (result, Tracked(log))
}

/// The public combine decoder (A-ICNS-6): `try_combine_logged` with the log
/// discarded (decision D1: the frozen name `combine` is removed).
///
/// `OutOfMemory` names its failed request: `(o.site(), o.elements())` is one of
/// `Req_J`'s requests. `try_combine_logged` already establishes the `site`, the
/// size, the ordered prefix and the failed last event; this clause reads them
/// out of that log.
pub(crate) fn try_combine(
    bytes: &[u8],
    entries: &[Entry],
    color_index: Option<u32>,
    mask_index: Option<u32>,
) -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::combine_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                color_index,
                mask_index,
            ),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 4
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 4
            },
            Err(_) => true,
        },
        match r {
            Err(TryError::OutOfMemory(o)) =>
                oom_in_req(o.site_spec(), o.elements_spec() as nat,
                    req_j(bytes@, entries@, color_index, mask_index)),
            _ => true,
        },
{
    let res = try_combine_logged(bytes, entries, color_index, mask_index);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(oom_in_req);
        reveal(g_j);
        reveal(g_j_clause2);
        reveal(req_j);
        reveal(crate::alloc_guard::req_of_log);
        if let Err(TryError::OutOfMemory(o)) = res.0 {
            assert(g_j(bytes@, entries@, color_index, mask_index, res.0, res.1@.view()));
            assert(g_j_clause2(bytes@, entries@, color_index, mask_index, res.0, res.1@.view()));
            assert(res.1@.view().len() >= 1);
            assert(crate::alloc_guard::req_of_log(res.1@.view())
                .is_prefix_of(req_j(bytes@, entries@, color_index, mask_index)));
            assert(res.1@.view().last().0 == o.site_spec());
            assert(res.1@.view().last().1 == o.elements_spec() as nat);
            let j = res.1@.view().len() - 1;
            assert(0 <= j < req_j(bytes@, entries@, color_index, mask_index).len());
            assert(crate::alloc_guard::req_of_log(res.1@.view())[j]
                == (o.site_spec(), o.elements_spec() as nat));
            assert(req_j(bytes@, entries@, color_index, mask_index)[j]
                == (o.site_spec(), o.elements_spec() as nat));
            assert(oom_in_req(o.site_spec(), o.elements_spec() as nat,
                req_j(bytes@, entries@, color_index, mask_index)));
        }
    }
    res.0
}

impl<'a> Parsed<'a> {
    /// The public combine decoder (A-ICNS-6) over the handle: `try_combine` with
    /// the bytes and entries the handle holds. No `requires` clause; the type
    /// invariant supplies `parsed_inv`, and the `ensures` restates the internal
    /// postcondition in handle terms.
    pub fn try_combine(
        &self,
        color_index: Option<u32>,
        mask_index: Option<u32>,
    ) -> (r: Result<Decoded, TryError>)
        ensures
            !matches!(r, Err(TryError::OutOfMemory(_)))
                ==> crate::model::combine_decoded_eq(
                    crate::alloc_guard::sites::fmt_part(r),
                    self.spec_bytes(),
                    self.spec_entries(),
                    color_index,
                    mask_index,
                ),
            match r {
                Ok(d) => {
                    &&& d.width == d.height
                    &&& d.width <= 128
                    &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 4
                    &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 4
                },
                Err(_) => true,
            },
            match r {
                Err(TryError::OutOfMemory(o)) =>
                    oom_in_req(o.site_spec(), o.elements_spec() as nat,
                        req_j(self.spec_bytes(), self.spec_entries(), color_index, mask_index)),
                _ => true,
            },
    {
        proof { use_type_invariant(self); }
        try_combine(self.bytes, self.entries.as_slice(), color_index, mask_index)
    }
}

// ---------------------------------------------------------------------------
// T6/T9..T13 call-site witnesses, retargeted to the `try_*` surface (alloc-delta
// SPEC.md §1.2: the frozen names are removed; each frozen witness is rewritten
// over `try_*`). `try_parse`'s own postcondition discharges each decoder's
// precondition from a concrete successful call.
// ---------------------------------------------------------------------------

/// T6: for every byte slice `bytes` with `|bytes| <= 67,108,864`, `try_parse`
/// either fails, or succeeds with a handle whose entry view satisfies
/// `parsed_inv(bytes@, entries@)` (at most 4,096 entries, each with
/// `8 <= start <= end <= |bytes|`, classification matching the parsed bytes).
/// The proof is the handle's type invariant: `try_parse` is its only
/// constructor, and it establishes it.
#[allow(dead_code)]
fn parse_success_invariant(bytes: &[u8]) -> (r: Result<Parsed<'_>, TryError>)
    ensures
        match r {
            Ok(p) => crate::model::parsed_inv(bytes@, p.spec_entries()),
            Err(TryError::Format(_)) => true,
            Err(TryError::OutOfMemory(_)) => true,
        },
{
    match try_parse(bytes) {
        Ok(p) => {
            proof { use_type_invariant(&p); reveal(Parsed::inv); }
            Ok(p)
        }
        Err(TryError::Format(e)) => Err(TryError::Format(e)),
        Err(TryError::OutOfMemory(o)) => Err(TryError::OutOfMemory(o)),
    }
}

/// T8 at an actual `try_parse` call, for every candidate entry sequence, both
/// directions, over the **full** result. A successful handle (`Ok(p)`) matches a
/// candidate `es` by all R1 fields exactly when `P(bytes, es)`; a rejection
/// (`Format(e)`) matches **no** candidate, so `P(bytes, es)` is false for every
/// `es`. This is the pre-change witness's `!OutOfMemory ==> forall es.
/// parse_returned(fmt_part_entries(r), es) <==> p_exec(bytes, es)` carried to the
/// handle's result type: `parse_returned` is `entries_same_fields` on `Ok` and
/// `false` on `Err`, and the two arms below are that statement written out per
/// arm, so the rejection obligation the public `try_parse` contract now carries
/// is exercised by an actual call, not only by the contract. `OutOfMemory` is
/// neither an accepted nor a rejected parse and is left unconstrained.
#[allow(dead_code)]
fn parse_with_t8(bytes: &[u8]) -> (r: Result<Parsed<'_>, TryError>)
    ensures
        match r {
            Ok(p) => forall|es: Seq<Entry>| #[trigger] crate::model::entries_same_fields(
                p.spec_entries(), es,
            ) <==> crate::model::p_exec(bytes@, es),
            Err(TryError::Format(_)) => forall|es: Seq<Entry>| !crate::model::p_exec(bytes@, es),
            Err(TryError::OutOfMemory(_)) => true,
        },
{
    let r = try_parse(bytes);
    proof {
        match &r {
            Ok(p) => {
                assert forall|es: Seq<Entry>| #[trigger] crate::model::entries_same_fields(
                    p.spec_entries(), es,
                ) <==> crate::model::p_exec(bytes@, es) by {
                    crate::model::lemma_t8_view(bytes@, p.spec_entries(), es);
                }
            }
            Err(TryError::Format(e)) => {
                // The contract's error clause gives `parses_to_s(bytes@, Err(e))`,
                // hence `!accepted(bytes@)`; `p_exec` implies `accepted`, so no
                // candidate is a parse of these bytes.
                assert(crate::model::parses_to_s(bytes@, Err(*e)));
                crate::model::lemma_parse_err_gives_not_accepted(bytes@, *e);
                assert forall|es: Seq<Entry>| !crate::model::p_exec(bytes@, es) by {
                    reveal(crate::model::p_exec);
                    if crate::model::p_exec(bytes@, es) {
                        match crate::model::mp(bytes@) {
                            Ok(ms) => assert(crate::model::accepted(bytes@)),
                            Err(_) => assert(false),
                        }
                    }
                }
            }
            Err(TryError::OutOfMemory(_)) => {}
        }
    }
    r
}

/// T9 stated over every payload and every required `n`: the exact `MR` result on
/// the executable type, via `try_rle`.
#[allow(dead_code)]
fn t9_rle_exact(payload: &[u8], n: usize) -> (r: Result<[Vec<u8>; 3], TryError>)
    requires
        1 <= n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rle_result_eq(
                crate::alloc_guard::sites::fmt_part_channels(r),
                crate::model::mr(payload@, n as int),
            ),
{
    let logged = try_rle_logged(payload, n);
    proof { reveal(crate::alloc_guard::sites::fmt_part_channels); }
    logged.0
}

/// T9 at a concrete call site, the shape the frozen `rle_after_call` had.
#[allow(dead_code)]
fn rle_after_call(payload: &[u8], n: usize) -> (r: Result<[Vec<u8>; 3], TryError>)
    requires
        1 <= n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rle_result_eq(
                crate::alloc_guard::sites::fmt_part_channels(r),
                crate::model::mr(payload@, n as int),
            ),
{
    t9_rle_exact(payload, n)
}

/// T10 stated over `P(b,E)`: the internal decoder equals `MC(b,E,i)` for a
/// `parsed_inv` entry list. This is the statement in terms of the parts; the
/// public surface states the same fact through `Parsed`, whose invariant is
/// `parsed_inv` on the parts it holds.
#[allow(dead_code)]
fn t10_rgb_exact_for_p(bytes: &[u8], entries: &[Entry], i: u32)
    -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::rgb_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                i,
            ),
{
    try_decode_rgb(bytes, entries, i)
}

/// T11 stated over `P(b,E)`.
#[allow(dead_code)]
fn t11_mask_exact_for_p(bytes: &[u8], entries: &[Entry], i: u32)
    -> (r: Result<Decoded, TryError>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_)))
            ==> crate::model::mask_decoded_eq(
                crate::alloc_guard::sites::fmt_part(r),
                bytes@,
                entries@,
                i,
            ),
{
    try_decode_mask(bytes, entries, i)
}

/// T13's shape on the handle: `try_parse` succeeds, then `try_combine` runs on
/// the returned handle. The premise of the statement is `parse(b) = Ok(E)`;
/// `P(b,E)` is obtained inside the body from T7/T8, not assumed, and the
/// decoders are reached through the only constructor of `Parsed`.
#[allow(dead_code)]
fn combine_after_parse(bytes: &[u8], ci: Option<u32>, mi: Option<u32>)
    -> (r: Result<Decoded, TryError>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize) * 4,
            Err(_) => true,
        },
{
    let p = match try_parse(bytes) {
        Ok(p) => p,
        Err(TryError::Format(e)) => return Err(TryError::Format(e)),
        Err(TryError::OutOfMemory(o)) => return Err(TryError::OutOfMemory(o)),
    };
    p.try_combine(ci, mi)
}

/// T10 at a concrete decoder call site, with the successful `try_parse` in
/// front: the handle's `try_decode_rgb` sees `parsed_inv` through the type
/// invariant, so no precondition is assumed anywhere in the chain.
#[allow(dead_code)]
fn decode_rgb_after_parse(bytes: &[u8], i: u32) -> (r: Result<Decoded, TryError>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize) * 3,
            Err(_) => true,
        },
{
    let p = match try_parse(bytes) {
        Ok(p) => p,
        Err(TryError::Format(e)) => return Err(TryError::Format(e)),
        Err(TryError::OutOfMemory(o)) => return Err(TryError::OutOfMemory(o)),
    };
    p.try_decode_rgb(i)
}

/// T11 at a concrete decoder call site.
#[allow(dead_code)]
fn decode_mask_after_parse(bytes: &[u8], i: u32) -> (r: Result<Decoded, TryError>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize),
            Err(_) => true,
        },
{
    let p = match try_parse(bytes) {
        Ok(p) => p,
        Err(TryError::Format(e)) => return Err(TryError::Format(e)),
        Err(TryError::OutOfMemory(o)) => return Err(TryError::OutOfMemory(o)),
    };
    p.try_decode_mask(i)
}

/// The accessor a caller uses to choose an index: `entries()` hands out exactly
/// the sequence the decoders' contracts name, so an index chosen from the view
/// is an index into the decoded list.
#[allow(dead_code)]
fn entries_view_len_after_parse(bytes: &[u8]) -> (r: Result<usize, TryError>)
    ensures
        match r {
            Ok(n) => n == crate::model::parse_len(bytes@),
            Err(_) => true,
        },
{
    let p = match try_parse(bytes) {
        Ok(p) => p,
        Err(TryError::Format(e)) => return Err(TryError::Format(e)),
        Err(TryError::OutOfMemory(o)) => return Err(TryError::OutOfMemory(o)),
    };
    let view = p.entries();
    let n = view.len();
    proof {
        use_type_invariant(&p);
        reveal(Parsed::inv);
        assert(view@ == p.spec_entries());
        crate::model::lemma_parses_to_len(bytes@, p.spec_entries());
    }
    Ok(n)
}

} // verus!
