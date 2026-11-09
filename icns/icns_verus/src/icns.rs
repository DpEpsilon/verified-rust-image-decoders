use vstd::prelude::*;

verus! {

// Verified implementation of ICNS-CORE-SPEC 1.1 (milestone M2).
// Behaviour is identical to the plain M1 implementation.

/// The input-size limit of R2: `len(bytes) > 67_108_864` is `FileTooLarge`.
pub const MAX_FILE_LEN: u64 = 67_108_864;

/// Maximum number of accepted entries (R6). A 4,097th entry is rejected.
pub const MAX_ENTRIES: usize = 4096;

/// Every legacy side is at most 128 (R8), so a decoded channel has at most
/// 128*128 = 16,384 bytes and an RGB output at most 49,152 bytes (R10).
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
    pub fn name(&self) -> &'static str {
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
/// input slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub type_hex: String,
    pub payload: [u32; 2],
    pub opaque_candidate: bool,
    pub nominal_side: Option<u32>,
}

impl Entry {
    /// The four raw type bytes, recovered from `type_hex`.
    pub fn type_code(&self) -> (out: [u8; 4])
        requires
            crate::model::is_hex8(self.type_hex@),
        ensures
            out == crate::model::code_of(self.type_hex@),
    {
        broadcast use vstd::string::is_ascii_spec_bytes;
        let bs = self.type_hex.as_str();
        assert(bs@ == self.type_hex@);
        assert(self.type_hex@.len() == 8);
        proof { crate::model::lemma_hex8_is_ascii(self.type_hex@); }
        assert(vstd::utf8::is_ascii_chars(bs@));
        assert(vstd::string::is_ascii(bs));
        let b = bs.as_bytes();
        assert(b@ =~= Seq::new(bs@.len(), |k: int| bs@[k] as u8));
        let mut out = [0u8; 4];
        let mut i: usize = 0;
        while i < 4
            invariant
                i <= 4,
                b@ == Seq::new(self.type_hex@.len(), |k: int| self.type_hex@[k] as u8),
                bs@ == self.type_hex@,
                crate::model::is_hex8(self.type_hex@),
                forall|k: int| 0 <= k < i ==> #[trigger] out@[k] == crate::model::code_of(
                    self.type_hex@,
                )[k],
            decreases 4 - i,
        {
            assert(b[2 * i] == self.type_hex@[2 * (i as int)] as u8);
            assert(b[2 * i + 1] == self.type_hex@[2 * (i as int) + 1] as u8);
            let hi = hex_nibble(b[2 * i]);
            let lo = hex_nibble(b[2 * i + 1]);
            proof {
                lemma_nib_char(self.type_hex@[2 * (i as int)]);
                lemma_nib_char(self.type_hex@[2 * (i as int) + 1]);
                assert(hi as int == crate::model::hex_val(self.type_hex@[2 * (i as int)]));
                assert(lo as int == crate::model::hex_val(self.type_hex@[2 * (i as int) + 1]));
            }
            let cell: u8 = hi * 16u8 + lo;
            proof {
                assert(hi < 16u8);
                assert(lo < 16u8);
                assert(cell as int == (hi as int) * 16 + (lo as int));
                assert(crate::model::code_of(self.type_hex@)[i as int] == cell);
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
pub proof fn lemma_legacy_side_le_128(code: [u8; 4])
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

/// R1's eight-digit lowercase hex encoding of the four type bytes.
fn hex4(code: [u8; 4]) -> (s: String)
    ensures
        s@ == crate::model::hex4(code),
{
    let c00 = hex_digit(code[0] / 16);
    let c01 = hex_digit(code[0] % 16);
    let c10 = hex_digit(code[1] / 16);
    let c11 = hex_digit(code[1] % 16);
    let c20 = hex_digit(code[2] / 16);
    let c21 = hex_digit(code[2] % 16);
    let c30 = hex_digit(code[3] / 16);
    let c31 = hex_digit(code[3] % 16);
    let mut s = String::new();
    s.push(c00);
    s.push(c01);
    s.push(c10);
    s.push(c11);
    s.push(c20);
    s.push(c21);
    s.push(c30);
    s.push(c31);
    assert(s@ =~= seq![c00, c01, c10, c11, c20, c21, c30, c31]);
    s
}

/// R2-R7, R15: parse an ICNS container into entries.
///
/// The postcondition is T7 exactly: the result equals the total reference model
/// `MP` as an exact `Result` — the first error and all four fields of every entry.
pub fn parse(bytes: &[u8]) -> (r: Result<Vec<Entry>, Error>)
    ensures
        crate::model::parses_to(bytes@, r),
{
    // R2: input length first, without reading bytes.
    if bytes.len() as u64 > MAX_FILE_LEN {
        proof {
            crate::model::lemma_mp_too_large(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::FileTooLarge);
        }
        return Err(Error::FileTooLarge);
    }
    if bytes.len() < 8 {
        proof {
            crate::model::lemma_mp_short(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::TruncatedHeader);
        }
        return Err(Error::TruncatedHeader);
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
        }
        return Err(Error::BadMagic);
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
        }
        return Err(Error::InvalidFileLength);
    }
    // R4: declared length must equal len(bytes).
    if declared > bytes.len() as u64 {
        proof {
            crate::model::lemma_mp_truncated_file(bytes@);
            crate::model::lemma_parses_to_of_mp_err(bytes@, crate::model::MpError::TruncatedFile);
        }
        return Err(Error::TruncatedFile);
    }
    if declared < bytes.len() as u64 {
        proof {
            crate::model::lemma_mp_trailing(bytes@);
            crate::model::lemma_parses_to_of_mp_err(
                bytes@, crate::model::MpError::TrailingFileBytes);
        }
        return Err(Error::TrailingFileBytes);
    }

    // R5, R6, R15: scan entries in file order. The ghost `os` records the
    // offsets the executable loop has accepted; `walk_pref` says `walk` from 8
    // emits `os` and then continues from `offset`.
    let mut entries: Vec<Entry> = Vec::new();
    let mut offset: u64 = 8;
    let ghost mut os: Seq<u32> = Seq::empty();
    proof {
        crate::model::lemma_pref_init(bytes@, declared);
        assert(crate::model::walk_pref(bytes@, declared, 8u64, Seq::<u32>::empty()));
    }
    while offset < declared
        invariant
            declared == bytes@.len(),
            bytes@.subrange(0, 4) == crate::model::icns_magic(),
            crate::model::be32u(bytes@, 4) as u64 == declared,
            bytes@.len() <= MAX_FILE_LEN,
            8 <= offset <= declared,
            entries@.len() == os.len(),
            crate::model::entries_match(bytes@, entries@, os),
            crate::model::walk_pref(bytes@, declared, offset, os),
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
            }
            return Err(Error::TruncatedEntryHeader);
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
            }
            return Err(Error::InvalidEntryLength);
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
            }
            return Err(Error::EntryOutOfBounds);
        }
        if entries.len() >= MAX_ENTRIES {
            proof {
                crate::model::lemma_pref_error_tme(bytes@, declared, offset, os, length);
                crate::model::lemma_mp_eq_mp_at(bytes@);
                crate::model::lemma_mp_at_walk(bytes@);
                assert(crate::model::mp(bytes@) == Err(crate::model::MpError::TooManyEntries));
                crate::model::lemma_parses_to_of_mp_err(
                    bytes@, crate::model::MpError::TooManyEntries);
            }
            return Err(Error::TooManyEntries);
        }
        let end = offset + length; // <= declared <= len(bytes)
        let pstart = offset + 8;
        let payload = &bytes[pstart as usize..end as usize];
        let (opaque_candidate, nominal_side) = classify(code, payload);
        let type_hex = hex4(code);
        let entry = Entry {
            type_hex,
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
            assert(entry.type_hex@ == crate::model::hex4(crate::model::code4(bytes@, offset as int)));
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
        // Hand the ghost offset list and the executable record to the model.
        proof {
            crate::model::lemma_entries_match_push(bytes@, entries@, os, entry, offset as u32);
            crate::model::lemma_pref_step(bytes@, declared, offset, os);
        }
        let ghost off_u32: u32 = offset as u32;
        entries.push(entry);
        offset = end;
        proof {
            os = os.push(off_u32);
        }
        proof {
            assert(entries@.len() == os.len());
            assert(crate::model::walk_pref(bytes@, declared, offset, os));
            assert(crate::model::entries_match(bytes@, entries@, os));
        }
    }
    proof {
        assert(crate::model::walk_pref(bytes@, declared, declared, os));
        crate::model::lemma_pref_done(bytes@, declared, os);
        assert(crate::model::walk(bytes@, declared, 8, 0) == Ok(os));
        crate::model::lemma_parses_to_ok(bytes@, entries@, os);
        crate::model::lemma_parses_to_ok_view(bytes@, entries);
    }
    Ok(entries)
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

/// R12: decode one payload into three planar channels of `n` bytes each.
/// R11's optional four-byte prefix removal is applied by the caller. The result
/// equals `MR(payload, n)` exactly: the three channels or the first error.
fn rle_decode(payload: &[u8], n: usize) -> (r: Result<[Vec<u8>; 3], Error>)
    requires
        n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
    ensures
        crate::model::rle_result_eq(r, crate::model::mr(payload@, n as int)),
        match r {
            Ok(ch) => {
                &&& ch@.len() == 3
                &&& ch@[0]@.len() == n
                &&& ch@[1]@.len() == n
                &&& ch@[2]@.len() == n
            },
            Err(_) => true,
        },
{
    let mut channels: [Vec<u8>; 3] = [vec![0u8; n], vec![0u8; n], vec![0u8; n]];
    let ghost mut acc: Seq<Seq<u8>> = Seq::empty();
    proof {
        assert(channels@[0]@.len() == n);
        assert(channels@[1]@.len() == n);
        assert(channels@[2]@.len() == n);
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
                }
                return Err(Error::RleTruncated);
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
                }
                return Err(Error::RleOverrun);
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
                }
                return Err(Error::RleTruncated);
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
        }
        return Err(Error::RleTrailingBytes);
    }
    proof {
        assert(crate::model::mr(payload@, n as int) == Ok((acc[0], acc[1], acc[2])));
        assert(acc[0] == channels@[0]@);
        assert(acc[1] == channels@[1]@);
        assert(acc[2] == channels@[2]@);
    }
    Ok(channels)
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

/// R10 and R11: decode a legacy colour entry to interleaved RGB. The result
/// equals `MC(bytes, entries, color_index)` exactly (T10): the optional
/// zero-only `it32` prefix, each RGB byte, and the precedence of index, type
/// and RLE errors.
pub fn decode_rgb(bytes: &[u8], entries: &[Entry], color_index: u32) -> (r: Result<Decoded, Error>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        crate::model::rgb_decoded_eq(r, bytes@, entries@, color_index),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 3
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 3
            },
            Err(_) => true,
        },
{
    if color_index as usize >= entries.len() {
        proof {
            crate::model::lemma_mc_index(bytes@, entries@, color_index);
        }
        return Err(Error::IndexOutOfRange);
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
            }
            return Err(Error::UnsupportedType);
        },
    };
    if !is_colour {
        proof {
            crate::model::lemma_mc_unsupported_mask(bytes@, entries@, color_index, code, side);
        }
        return Err(Error::UnsupportedType);
    }
    proof {
        lemma_legacy_side_le_128(code);
        crate::model::lemma_mc_colour(bytes@, entries@, color_index, code, side, (side as int)
            * (side as int));
    }
    let n = (side as usize) * (side as usize);
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

    let channels = rle_decode(payload, n)?;
    proof {
        assert(channels@[0]@.len() == n);
        assert(channels@[1]@.len() == n);
        assert(channels@[2]@.len() == n);
    }
    let mut out = vec![0u8; 3 * n];
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
    }
    Ok(Decoded {
        width: side,
        height: side,
        data: out,
    })
}

/// R13: decode a legacy mask entry to `N` alpha bytes. The result equals
/// `MA(bytes, entries, mask_index)` exactly (T11): every alpha byte and the
/// exact error precedence of index, type and length.
pub fn decode_mask(bytes: &[u8], entries: &[Entry], mask_index: u32) -> (r: Result<Decoded, Error>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        crate::model::mask_decoded_eq(r, bytes@, entries@, mask_index),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE
                &&& d.data@.len() == (d.width as usize) * (d.width as usize)
            },
            Err(_) => true,
        },
{
    if mask_index as usize >= entries.len() {
        proof {
            crate::model::lemma_ma_index(bytes@, entries@, mask_index);
        }
        return Err(Error::IndexOutOfRange);
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
            }
            return Err(Error::UnsupportedType);
        },
    };
    if is_colour {
        proof {
            crate::model::lemma_ma_unsupported_colour(bytes@, entries@, mask_index, code, side);
        }
        return Err(Error::UnsupportedType);
    }
    proof { lemma_legacy_side_le_128(code); }
    let n = (side as usize) * (side as usize);
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
        }
        return Err(Error::InvalidMaskLength);
    }
    let mut data: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
            payload@.len() == n,
            data@ == payload@.subrange(0, i as int),
        decreases n - i,
    {
        data.push(payload[i]);
        i += 1;
    }
    proof {
        assert(data@ =~= payload@);
        crate::model::lemma_ma_ok(bytes@, entries@, mask_index, code, side, n as int, payload@);
    }
    Ok(Decoded {
        width: side,
        height: side,
        data,
    })
}

/// R14: combine a colour and a mask entry into interleaved RGBA. The result
/// equals `MJ(bytes, entries, color_index, mask_index)` exactly (T12): the
/// absent-index precedence, RGB before mask error precedence, the side check
/// and every RGBA byte.
pub fn combine(
    bytes: &[u8],
    entries: &[Entry],
    color_index: Option<u32>,
    mask_index: Option<u32>,
) -> (r: Result<Decoded, Error>)
    requires
        crate::model::parsed_inv(bytes@, entries@),
    ensures
        crate::model::combine_decoded_eq(r, bytes@, entries@, color_index, mask_index),
        match r {
            Ok(d) => {
                &&& d.width == d.height
                &&& d.width <= 128
                &&& d.data@.len() <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE * 4
                &&& d.data@.len() == (d.width as usize) * (d.width as usize) * 4
            },
            Err(_) => true,
        },
{
    let ci = match color_index {
        Some(i) => i,
        None => {
            proof {
                assert(crate::model::mj(bytes@, entries@, None, mask_index)
                    == Err(Error::MissingColor));
            }
            return Err(Error::MissingColor);
        },
    };
    let mi = match mask_index {
        Some(i) => i,
        None => {
            proof {
                assert(crate::model::mj(bytes@, entries@, Some(ci), None)
                    == Err(Error::MissingMask));
            }
            return Err(Error::MissingMask);
        },
    };
    let rgb = match decode_rgb(bytes, entries, ci) {
        Ok(d) => { proof {
            crate::model::lemma_decoded_eq_ok(
                Ok::<Decoded, Error>(d),
                crate::model::mc(bytes@, entries@, ci),
                d,
            );
        } d },
        Err(e) => {
            proof {
                crate::model::lemma_decoded_eq_err(
                    Err::<Decoded, Error>(e),
                    crate::model::mc(bytes@, entries@, ci),
                    e,
                );
                assert(crate::model::mj(bytes@, entries@, Some(ci), Some(mi)) == Err(e));
            }
            return Err(e);
        },
    };
    let mask = match decode_mask(bytes, entries, mi) {
        Ok(d) => { proof {
            crate::model::lemma_decoded_eq_ok(
                Ok::<Decoded, Error>(d),
                crate::model::ma(bytes@, entries@, mi),
                d,
            );
        } d },
        Err(e) => {
            proof {
                crate::model::lemma_decoded_eq_err(
                    Err::<Decoded, Error>(e),
                    crate::model::ma(bytes@, entries@, mi),
                    e,
                );
                assert(crate::model::mj(bytes@, entries@, Some(ci), Some(mi)) == Err(e));
            }
            return Err(e);
        },
    };
    if rgb.width != mask.width || rgb.height != mask.height {
        proof {
            assert(crate::model::mc(bytes@, entries@, ci) == Ok((rgb.width, rgb.height, rgb.data@)));
            assert(crate::model::ma(bytes@, entries@, mi) == Ok((mask.width, mask.height, mask.data@)));
            assert(crate::model::mj(bytes@, entries@, Some(ci), Some(mi))
                == Err(Error::TypeMismatch));
        }
        return Err(Error::TypeMismatch);
    }
    let n = (rgb.width as usize) * (rgb.height as usize);
    let mut out = vec![0u8; 4 * n];
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
    }
    Ok(d)
}

// ---------------------------------------------------------------------------
// T6 call-site witnesses: `parse`'s own postcondition discharges each decoder's
// precondition from a concrete successful call. Nothing about `bytes` is assumed
// beyond the R2 input length cap, which is part of `parsed_inv`.
// ---------------------------------------------------------------------------

/// T6: for every byte slice `bytes` with `|bytes| <= 67,108,864`, `parse(bytes)`
/// either fails, or succeeds with at most 4,096 entries whose ranges are
/// `8 <= start <= end <= |bytes|` and whose classification matches the parsed
/// bytes. This is exactly `crate::model::parsed_inv`, which `parse` proves.
#[allow(dead_code)]
fn parse_success_invariant(bytes: &[u8]) -> (r: Result<Vec<Entry>, Error>)
    ensures
        match r {
            Ok(E) => crate::model::parsed_inv(bytes@, E@),
            Err(_) => true,
        },
{
    let entries = parse(bytes)?;
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    Ok(entries)
}

/// A wrapper showing one concrete decoder call site: `parse` succeeds, then
/// `decode_rgb` is called with the *same* slice and the *same* entries, and its
/// precondition `parsed_inv(bytes@, entries@)` is discharged by `parse`'s
/// postcondition, not assumed.
#[allow(dead_code)]
fn decode_rgb_after_parse(bytes: &[u8], i: u32) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize) * 3,
            Err(_) => true,
        },
{
    let entries = parse(bytes)?;
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    decode_rgb(bytes, &entries, i)
}

/// As above for `decode_mask`.
#[allow(dead_code)]
fn decode_mask_after_parse(bytes: &[u8], i: u32) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize),
            Err(_) => true,
        },
{
    let entries = parse(bytes)?;
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    decode_mask(bytes, &entries, i)
}

/// As above for `combine`, including the internal `decode_rgb`/`decode_mask`
/// calls that each discharge the same `parse(b)=Ok(E)` precondition.
#[allow(dead_code)]
fn combine_after_parse(
    bytes: &[u8],
    ci: Option<u32>,
    mi: Option<u32>,
) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize) * 4,
            Err(_) => true,
        },
{
    let entries = parse(bytes)?;
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    combine(bytes, &entries, ci, mi)
}

/// T8 at an actual call to parse, for every candidate entry sequence, in
/// both directions. The candidate and result are compared by all R1 fields.
#[allow(dead_code)]
fn parse_with_t8(bytes: &[u8]) -> (r: Result<Vec<Entry>, Error>)
    ensures
        forall|es: Seq<Entry>| crate::model::parse_returned(r, es)
            <==> crate::model::p_exec(bytes@, es),
{
    let r = parse(bytes);
    proof {
        assert forall|es: Seq<Entry>| crate::model::parse_returned(r, es)
            <==> crate::model::p_exec(bytes@, es) by {
            crate::model::lemma_t8_result(bytes@, r, es);
        }
    }
    r
}

/// T9 at a concrete `rle_decode` call: for every payload and every `n` in the
/// required set, the result is exactly `MR(payload, n)`.
#[allow(dead_code)]
fn rle_after_call(payload: &[u8], n: usize) -> (r: Result<[Vec<u8>; 3], Error>)
    requires
        n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
    ensures
        crate::model::rle_result_eq(r, crate::model::mr(payload@, n as int)),
{
    rle_decode(payload, n)
}

/// T10 at a concrete decoder call site: `parse` succeeds, `P(bytes, E)` holds,
/// and `decode_rgb` equals `MC(bytes, E, i)` exactly. The model list comes from
/// `mp(bytes)`, so `MC` here is over the same `E` the caller passes.
#[allow(dead_code)]
fn decode_rgb_with_t10(bytes: &[u8], i: u32) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize) * 3,
            Err(_) => true,
        },
{
    let entries = parse(bytes)?;
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    decode_rgb(bytes, &entries, i)
}

/// T11 at a concrete decoder call site.
#[allow(dead_code)]
fn decode_mask_with_t11(bytes: &[u8], i: u32) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => d.width == d.height && d.width <= 128 && d.data@.len()
                == (d.width as usize) * (d.width as usize),
            Err(_) => true,
        },
{
    let entries = parse(bytes)?;
    proof {
        crate::model::lemma_parse_ok_gives_p(bytes@, entries);
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    decode_mask(bytes, &entries, i)
}

/// T9 stated as a theorem over every payload and every required `n`: the exact
/// `MR` result, on the executable type, with no precondition on the payload.
#[allow(dead_code)]
fn t9_rle_exact(payload: &[u8], n: usize) -> (r: Result<[Vec<u8>; 3], Error>)
    requires
        n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE,
    ensures
        crate::model::rle_result_eq(r, crate::model::mr(payload@, n as int)),
{
    rle_decode(payload, n)
}

/// T10 stated over `P(b,E)`: `decode_rgb` equals `MC` for the parse result's
/// entries. The `P(b,E)` premise is supplied by the caller's successful parse.
#[allow(dead_code)]
fn t10_rgb_exact_for_p(bytes: &[u8], entries: &[Entry], i: u32) -> (r: Result<Decoded, Error>)
    requires
        crate::model::p_exec(bytes@, entries@),
    ensures
        crate::model::rgb_decoded_eq(r, bytes@, entries@, i),
{
    proof {
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    decode_rgb(bytes, entries, i)
}

/// T11 stated over `P(b,E)`.
#[allow(dead_code)]
fn t11_mask_exact_for_p(bytes: &[u8], entries: &[Entry], i: u32) -> (r: Result<Decoded, Error>)
    requires
        crate::model::p_exec(bytes@, entries@),
    ensures
        crate::model::mask_decoded_eq(r, bytes@, entries@, i),
{
    proof {
        crate::model::lemma_p_exec_inv(bytes@, entries@);
    }
    decode_mask(bytes, entries, i)
}

} // verus!
