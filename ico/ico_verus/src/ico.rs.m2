//! Verified implementation of ICO-CORE-SPEC 1.0, milestone M2.
//!
//! Everything here is inside `verus!`: the M1 plain implementation now carries
//! contracts and proofs. `src/ico.rs.m1` preserves the M1 plain file, and
//! `tests/conformance.rs` still runs all 217 frozen vectors. The reference model
//! `MC`/`ME`/`H` lives in `crate::model`, written from the spec text and
//! independent of this code.
//!
//! Proved here: T1 (safety of `parse` and of the ICO side of `decode_entry`),
//! T2 (`parse` equals `MC` as the complete `Result`), T3 (record validity),
//! T4 (signature classification) and T7 (arithmetic and output bounds). The DIB
//! composition — T5, T6, T8, `MD` and the exact `decode_entry` result — is M3;
//! `decode_entry` here proves its ICO-side decision (`model::decode_pre`), which
//! fixes every ICO-controlled outcome and the DIB-result *shape* but not yet the
//! DIB bytes.
//!
//! Rules followed throughout:
//!
//! - Directory and range arithmetic uses `u64` intermediates and comparisons in
//!   `u64`; no `offset + size` and no `6 + 16*count` is ever formed in `u32` or
//!   `u16`. Nothing narrows a `u32` index to a smaller type.
//! - Every slice index and every read offset is compared against the buffer
//!   length before it is used, so no input can panic the decoder.
//! - No `unsafe`, no `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!` or
//!   `unwrap_or` path is reachable from any input.
//! - `parse` makes at most one entry-list reservation; `decode_entry` allocates
//!   nothing of its own and calls the DIB core at most once.

use dib_verus::{Decoded, Error as DibError, Mode};
use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::seq::{
    lemma_seq_new_index, lemma_seq_new_len, lemma_seq_push_index_different,
    lemma_seq_push_index_same, lemma_seq_push_len,
};

verus! {

// ---------------------------------------------------------------------------
// Limits (SPEC R2)
// ---------------------------------------------------------------------------

/// R2: `MAX_INPUT = 67,108,864` bytes. Exactly this length passes the cap test.
pub const MAX_INPUT: usize = 67_108_864;

/// R2: `MAX_ENTRIES = 1,024` records. The u16 wire count permits 65,535.
pub const MAX_ENTRIES: usize = 1_024;

/// R3: the shared directory header is six bytes.
pub const DIRECTORY_HEADER_LEN: usize = 6;

/// R4: one directory record is 16 bytes.
pub const ENTRY_RECORD_LEN: usize = 16;

/// R7/T7: a complete maximum-size directory occupies `6 + 16*1,024` bytes.
pub const MAX_DIRECTORY_LEN: usize = DIRECTORY_HEADER_LEN + ENTRY_RECORD_LEN * MAX_ENTRIES;

/// R6: the eight-byte PNG signature `89 50 4e 47 0d 0a 1a 0a`.
pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

/// R3: ICO directory type.
pub const TYPE_ICO: u16 = 1;

/// R3: CUR directory type.
pub const TYPE_CUR: u16 = 2;

// ---------------------------------------------------------------------------
// Public types (SPEC R1)
// ---------------------------------------------------------------------------

/// R1/R6: a valid range's payload kind. The payload range is carried beside it
/// in [`EntryStatus`], per the JSON schema `{"kind":..., "payload":[start,end]}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Png,
    Dib,
}

/// R4: the first matching defect of a complete directory record, in R4's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryDefect {
    InvalidEntryReserved,
    EmptyEntry,
    EntryOverlapsDirectory,
    EntryOutOfBounds,
}

/// R1: `Invalid(EntryDefect)` or `Valid { kind, payload }` with
/// `payload = [start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryStatus {
    Invalid(EntryDefect),
    Valid { kind: EntryKind, payload: [u32; 2] },
}

/// R1: one directory record. `raw_offset`/`raw_size` are retained for invalid
/// records too; an invalid record has no trusted payload range and no kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub width: u16,
    pub height: u16,
    pub color_count: u8,
    pub raw_offset: u32,
    pub raw_size: u32,
    pub planes: Option<u16>,
    pub bit_count: Option<u16>,
    pub hotspot_x: Option<u16>,
    pub hotspot_y: Option<u16>,
    pub status: EntryStatus,
}

/// R9: the closed, case-sensitive error set. The DIB case carries the DIB
/// crate's own error enum, not a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FileTooLarge,
    TruncatedDirectory,
    InvalidReserved,
    UnsupportedType,
    EmptyDirectory,
    TooManyEntries,
    TruncatedEntries,
    InvalidEntryReserved,
    EmptyEntry,
    EntryOverlapsDirectory,
    EntryOutOfBounds,
    IndexOutOfRange,
    InvalidEntryRecord,
    PngDelegation,
    Dib(DibError),
}

impl Error {
    /// R9: the exact spelling used by `VECTORS.json`.
    pub fn name(&self) -> &'static str {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedDirectory => "TruncatedDirectory",
            Error::InvalidReserved => "InvalidReserved",
            Error::UnsupportedType => "UnsupportedType",
            Error::EmptyDirectory => "EmptyDirectory",
            Error::TooManyEntries => "TooManyEntries",
            Error::TruncatedEntries => "TruncatedEntries",
            Error::InvalidEntryReserved => "InvalidEntryReserved",
            Error::EmptyEntry => "EmptyEntry",
            Error::EntryOverlapsDirectory => "EntryOverlapsDirectory",
            Error::EntryOutOfBounds => "EntryOutOfBounds",
            Error::IndexOutOfRange => "IndexOutOfRange",
            Error::InvalidEntryRecord => "InvalidEntryRecord",
            Error::PngDelegation => "PngDelegation",
            Error::Dib(e) => e.name(),
        }
    }
}

impl EntryDefect {
    /// R4/R9: the exact spelling.
    pub fn name(&self) -> &'static str {
        match self {
            EntryDefect::InvalidEntryReserved => "InvalidEntryReserved",
            EntryDefect::EmptyEntry => "EmptyEntry",
            EntryDefect::EntryOverlapsDirectory => "EntryOverlapsDirectory",
            EntryDefect::EntryOutOfBounds => "EntryOutOfBounds",
        }
    }
}

// ---------------------------------------------------------------------------
// Guaranteed wire reads (R3, R4)
// ---------------------------------------------------------------------------

/// `b[p]`; the caller proves that byte exists. Model value `model::b8`.
fn rd8(b: &[u8], p: usize) -> (r: u8)
    requires
        p < b@.len(),
    ensures
        r as int == crate::model::b8(b@, p as int),
{
    b[p]
}

/// `U16(b,p)` little-endian; the caller proves both bytes exist.
fn rd16(b: &[u8], p: usize) -> (r: u16)
    requires
        p + 2 <= b@.len(),
    ensures
        r as int == crate::model::le16(b@, p as int),
{
    let lo = b[p] as u16;
    let hi = b[p + 1] as u16;
    proof {
        assert(crate::model::le16(b@, p as int) == b@[p as int] + 256 * (b@[p as int + 1] as int));
        assert((lo as int) < 256);
        assert((hi as int) < 256);
        assert((lo | (hi << 8)) as int == (lo as int) + ((hi << 8) as int)) by (bit_vector)
            requires
                (lo as int) < 256,
                (hi as int) < 256,
        ;
        assert(((hi << 8) as int) == 256 * (hi as int)) by (bit_vector)
            requires
                (hi as int) < 256,
        ;
    }
    lo | (hi << 8)
}

/// `U32(b,p)` little-endian; the caller proves all four bytes exist.
fn rd32(b: &[u8], p: usize) -> (r: u32)
    requires
        p + 4 <= b@.len(),
    ensures
        r as int == crate::model::le32(b@, p as int),
{
    proof {
        assert(b@.len() == b.len());
        assert(b@.len() <= usize::MAX);
        assert(p + 2 <= usize::MAX) by (nonlinear_arith)
            requires
                p + 4 <= b@.len(),
                b@.len() <= usize::MAX,
        ;
    }
    let a = rd16(b, p) as u32;
    let c = rd16(b, p + 2) as u32;
    proof {
        assert(crate::model::le32(b@, p as int) == b@[p as int] + 256 * (b@[p as int + 1] as int)
            + 65536 * (b@[p as int + 2] as int) + 16777216 * (b@[p as int + 3] as int));
        crate::model::lemma_le16_lt(b@, p as int);
        crate::model::lemma_le16_lt(b@, p as int + 2);
        assert(a as int == crate::model::le16(b@, p as int));
        assert(c as int == crate::model::le16(b@, p as int + 2));
        assert((a as int) < 65536);
        assert((c as int) < 65536);
        assert((a | (c << 16)) as int == (a as int) + ((c << 16) as int)) by (bit_vector)
            requires
                (a as int) < 65536,
                (c as int) < 65536,
        ;
        assert(((c << 16) as int) == 65536 * (c as int)) by (bit_vector)
            requires
                (c as int) < 65536,
        ;
    }
    a | (c << 16)
}

// ---------------------------------------------------------------------------
// The shared directory guard H (R3)
// ---------------------------------------------------------------------------

/// The `H(b)` result: the two metadata words and the end of the whole directory.
pub struct Header {
    pub kind: u16,
    pub count: usize,
    pub dir_end: u32,
}

/// The `H` record a concrete `Header` denotes, field for field.
pub open spec fn hdr_matches(hr: crate::model::HRec, h: Header) -> bool {
    &&& hr.typ == h.kind as int
    &&& hr.count == h.count as int
    &&& hr.dir_end == h.dir_end as int
}

/// R3, in this exact order: the length cap, six-byte availability, reserved
/// word, type, non-empty count, count cap, and the whole-table guard. The count
/// cap is tested before the table guard, so a count of 1,025 wins over an
/// incomplete table. Postcondition: the result is `model::h(bytes)` exactly.
fn directory_guard(bytes: &[u8]) -> (r: Result<Header, Error>)
    ensures
        match crate::model::h(bytes@) {
            Ok(hr) => match r {
                Ok(h) => hdr_matches(hr, h),
                Err(_) => false,
            },
            Err(e) => match r {
                Ok(_) => false,
                Err(e2) => e2 == e,
            },
        },
{
    let len = bytes.len();
    // R2: the cap is tested before the buffer is read at all.
    if len > MAX_INPUT {
        return Err(Error::FileTooLarge);
    }
    proof {
        assert(bytes@.len() <= crate::model::max_input());
    }
    // R3: all six header bytes must exist before any of the six-byte reads.
    if len < DIRECTORY_HEADER_LEN {
        return Err(Error::TruncatedDirectory);
    }
    let reserved = rd16(bytes, 0);
    if reserved != 0 {
        proof {
            assert(crate::model::le16(bytes@, 0) != 0);
        }
        return Err(Error::InvalidReserved);
    }
    proof {
        assert(crate::model::le16(bytes@, 0) == 0);
    }
    let kind = rd16(bytes, 2);
    if kind != TYPE_ICO && kind != TYPE_CUR {
        proof {
            assert(!(crate::model::le16(bytes@, 2) == 1 || crate::model::le16(bytes@, 2) == 2));
        }
        return Err(Error::UnsupportedType);
    }
    proof {
        assert(crate::model::le16(bytes@, 2) == 1 || crate::model::le16(bytes@, 2) == 2);
    }
    let count_word = rd16(bytes, 4);
    if count_word == 0 {
        proof {
            assert(crate::model::le16(bytes@, 4) == 0);
        }
        return Err(Error::EmptyDirectory);
    }
    let count = count_word as usize;
    if count > MAX_ENTRIES {
        proof {
            assert(crate::model::le16(bytes@, 4) > crate::model::max_entries());
        }
        return Err(Error::TooManyEntries);
    }
    proof {
        assert(crate::model::le16(bytes@, 4) <= crate::model::max_entries());
        assert(count as int == crate::model::le16(bytes@, 4));
        crate::model::lemma_dir_end_bound(crate::model::le16(bytes@, 4));
        assert(6 + 16 * (count as int) <= 16390);
        assert(6 + 16 * (count as int) == DIRECTORY_HEADER_LEN as int + ENTRY_RECORD_LEN as int
            * (count_word as int));
    }
    // R7/T7: `6 + 16*count <= 6 + 16*1,024 = 16,390`, computed in `u64` so the
    // multiplication cannot wrap on any target.
    let dir_end = DIRECTORY_HEADER_LEN as u64 + ENTRY_RECORD_LEN as u64 * count_word as u64;
    if dir_end > bytes.len() as u64 {
        proof {
            assert(crate::model::dir_header_len() + crate::model::entry_record_len() * crate::model::le16(
                bytes@,
                4,
            ) > bytes@.len());
        }
        return Err(Error::TruncatedEntries);
    }
    proof {
        assert(crate::model::dir_header_len() + crate::model::entry_record_len() * crate::model::le16(
            bytes@,
            4,
        ) <= bytes@.len());
        assert(crate::model::dir_header_len() == 6);
        assert(crate::model::entry_record_len() == 16);
        assert(dir_end as int == 6 + 16 * (count_word as int));
        assert(dir_end as int <= 16390);
    }
    // `dir_end <= 16,390`, so this narrowing is exact; the M1 defensive
    // `dir_end > u32::MAX` branch is provably dead and dropped.
    let dir_end32 = dir_end as u32;
    proof {
        assert(dir_end32 as int == 6 + 16 * crate::model::le16(bytes@, 4));
        reveal(crate::model::h);
        assert(crate::model::h(bytes@) == Ok(crate::model::HRec {
            typ: crate::model::le16(bytes@, 2),
            count: crate::model::le16(bytes@, 4),
            dir_end: 6 + 16 * crate::model::le16(bytes@, 4),
        }));
    }
    let h = Header {
        kind,
        count,
        dir_end: dir_end32,
    };
    proof {
        reveal(hdr_matches);
        assert(hdr_matches(
            crate::model::HRec {
                typ: crate::model::le16(bytes@, 2),
                count: crate::model::le16(bytes@, 4),
                dir_end: 6 + 16 * crate::model::le16(bytes@, 4),
            },
            h,
        ));
    }
    Ok(h)
}

// ---------------------------------------------------------------------------
// Per-record reconstruction ME (R4-R6)
// ---------------------------------------------------------------------------

/// R6: `true` iff the payload `[start, end)` is at least eight bytes long and its
/// first eight bytes equal [`PNG_SIGNATURE`]. Reads only bytes below `end`.
fn has_png_signature(bytes: &[u8], start: usize, end: usize) -> (r: bool)
    requires
        start <= end <= bytes@.len(),
    ensures
        r == (end - start >= 8 && bytes@.subrange(start as int, start as int + 8)
            == crate::model::png_signature()),
{
    if end - start < 8 {
        proof {
            assert(crate::model::png_signature().len() == 8);
        }
        return false;
    }
    proof {
        assert(PNG_SIGNATURE@ =~= crate::model::png_signature());
    }
    let mut k = 0usize;
    while k < PNG_SIGNATURE.len()
        invariant
            start <= end <= bytes@.len(),
            end - start >= 8,
            k <= 8,
            forall|j: int| 0 <= j < k ==> #[trigger] bytes@[start as int + j]
                == crate::model::png_signature()[j],
        decreases 8 - k,
    {
        if bytes[start + k] != PNG_SIGNATURE[k] {
            proof {
                assert(bytes@[start as int + k as int] != crate::model::png_signature()[k as int]);
                assert(bytes@.subrange(start as int, start as int + 8)
                    != crate::model::png_signature()) by {
                    if bytes@.subrange(start as int, start as int + 8) == crate::model::png_signature(
                    ) {
                        assert(bytes@.subrange(start as int, start as int + 8)[k as int]
                            == crate::model::png_signature()[k as int]);
                    }
                }
            }
            return false;
        }
        k += 1;
    }
    proof {
        assert(bytes@.subrange(start as int, start as int + 8) == crate::model::png_signature()) by {
            assert(crate::model::png_signature().len() == 8);
            assert forall|j: int| 0 <= j < 8 implies
                bytes@.subrange(start as int, start as int + 8)[j]
                    == crate::model::png_signature()[j] by {
                assert(bytes@.subrange(start as int, start as int + 8)[j] == bytes@[start as int + j]);
            }
        }
    }
    true
}

/// R4: the `reserved` byte is already known zero, so `model::defect_of`'s
/// remaining chain applies: zero size, offset before the whole-directory end,
/// then an out-of-file range. The range test is `offset > |b| || size > |b| -
/// offset`, evaluated in `u64`; forming `offset + size` directly would wrap.
fn record_status(
    bytes: &[u8],
    dir_end: u32,
    offset_word: u32,
    len_word: u32,
    Ghost(hr): Ghost<crate::model::HRec>,
    Ghost(i): Ghost<int>,
) -> (r: EntryStatus)
    requires
        hr.dir_end == dir_end as int,
        hr.dir_end == 6 + 16 * hr.count,
        0 <= i < hr.count,
        hr.dir_end <= bytes@.len(),
        bytes@.len() <= crate::model::max_input(),
        crate::model::b8(bytes@, 6 + 16 * i + 3) == 0,
        crate::model::le32(bytes@, 6 + 16 * i + 8) == len_word as int,
        crate::model::le32(bytes@, 6 + 16 * i + 12) == offset_word as int,
    ensures
        r == crate::model::me(bytes@, hr, i).status,
{
    reveal(crate::model::me);
    reveal(crate::model::defect_of);
    assert(crate::model::me(bytes@, hr, i).status == match crate::model::defect_of(bytes@, hr.dir_end,
        0, len_word as int, offset_word as int) {
        Some(d) => EntryStatus::Invalid(d),
        None => EntryStatus::Valid {
            kind: crate::model::kind_of(bytes@, offset_word as int, len_word as int),
            payload: [offset_word, (offset_word as int + len_word as int) as u32],
        },
    });
    if len_word == 0 {
        proof {
            assert(crate::model::defect_of(bytes@, hr.dir_end, 0, len_word as int, offset_word as int)
                == Some(EntryDefect::EmptyEntry));
        }
        return EntryStatus::Invalid(EntryDefect::EmptyEntry);
    }
    if offset_word < dir_end {
        proof {
            assert(crate::model::defect_of(bytes@, hr.dir_end, 0, len_word as int, offset_word as int)
                == Some(EntryDefect::EntryOverlapsDirectory));
        }
        return EntryStatus::Invalid(EntryDefect::EntryOverlapsDirectory);
    }
    let off = offset_word as u64;
    let len = bytes.len() as u64;
    if off > len || len_word as u64 > len - off {
        proof {
            assert(crate::model::defect_of(bytes@, hr.dir_end, 0, len_word as int, offset_word as int)
                == Some(EntryDefect::EntryOutOfBounds));
        }
        return EntryStatus::Invalid(EntryDefect::EntryOutOfBounds);
    }
    proof {
        assert(off as int <= bytes@.len());
        assert((len_word as int) <= bytes@.len() - (offset_word as int));
        assert((offset_word as int) + (len_word as int) <= bytes@.len()) by (nonlinear_arith)
            requires
                (offset_word as int) <= bytes@.len(),
                (len_word as int) <= bytes@.len() - (offset_word as int),
        ;
        assert((offset_word as int) + (len_word as int) <= crate::model::max_input());
        assert((offset_word as int) + (len_word as int) <= u32::MAX) by (nonlinear_arith)
            requires
                (offset_word as int) + (len_word as int) <= crate::model::max_input(),
                crate::model::max_input() == 67108864,
        ;
        assert(crate::model::defect_of(bytes@, hr.dir_end, 0, len_word as int, offset_word as int)
            is None);
    }
    let end = off + (len_word as u64);
    let end_usize = end as usize;
    proof {
        assert(end as int == (offset_word as int) + (len_word as int));
        assert(end_usize as int == end as int);
        assert(end_usize <= bytes@.len());
    }
    let png = has_png_signature(bytes, off as usize, end_usize);
    proof {
        assert(png == (end_usize as int - off as int >= 8 && bytes@.subrange(off as int, off as int
            + 8) == crate::model::png_signature()));
        assert(end_usize as int - off as int == len_word as int);
        reveal(crate::model::kind_of);
        assert(png == (crate::model::kind_of(bytes@, offset_word as int, len_word as int)
            == EntryKind::Png));
    }
    let end32 = end as u32;
    proof {
        assert(end32 as int == (offset_word as int) + (len_word as int));
    }
    let st = if png {
        EntryStatus::Valid {
            kind: EntryKind::Png,
            payload: [offset_word, end32],
        }
    } else {
        EntryStatus::Valid {
            kind: EntryKind::Dib,
            payload: [offset_word, end32],
        }
    };
    proof {
        assert(st == crate::model::me(bytes@, hr, i).status);
    }
    st
}

/// R4/R5/R6: reconstruct record `i` at `p = 6 + 16*i` from the wire. This is the
/// `ME` reconstruction; every read is inside the already-guarded directory, so
/// the reads never fail for a directory that passed `H`. Returns `ME` exactly.
fn reconstruct(
    bytes: &[u8],
    typ: u16,
    dir_end: u32,
    i: usize,
    Ghost(hr): Ghost<crate::model::HRec>,
) -> (r: Entry)
    requires
        hr.typ == typ as int,
        hr.dir_end == dir_end as int,
        hr.dir_end == 6 + 16 * hr.count,
        0 <= i < hr.count,
        hr.count <= crate::model::max_entries(),
        typ == TYPE_ICO || typ == TYPE_CUR,
        hr.dir_end <= bytes@.len(),
        bytes@.len() <= crate::model::max_input(),
    ensures
        r == crate::model::me(bytes@, hr, i as int),
{
    let p = DIRECTORY_HEADER_LEN + ENTRY_RECORD_LEN * i;
    proof {
        assert(6 + 16 * i + 16 <= 6 + 16 * hr.count) by (nonlinear_arith)
            requires
                i < hr.count,
        ;
        assert(p as int == 6 + 16 * (i as int));
        assert(p + 16 <= bytes@.len());
    }
    let w0 = rd8(bytes, p);
    let h0 = rd8(bytes, p + 1);
    let color_count = rd8(bytes, p + 2);
    let reserved = rd8(bytes, p + 3);
    let lo = rd16(bytes, p + 4);
    let hi = rd16(bytes, p + 6);
    let size = rd32(bytes, p + 8);
    let offset = rd32(bytes, p + 12);

    // R5: a zero nominal dimension means 256, in either word.
    let width = if w0 == 0 {
        256u16
    } else {
        w0 as u16
    };
    let height = if h0 == 0 {
        256u16
    } else {
        h0 as u16
    };

    proof {
        reveal(crate::model::me);
        reveal(crate::model::nom_dim);
        assert(crate::model::b8(bytes@, p as int + 0) == w0 as int);
        assert(crate::model::b8(bytes@, p as int + 1) == h0 as int);
        assert(crate::model::b8(bytes@, p as int + 2) == color_count as int);
        assert(crate::model::b8(bytes@, p as int + 3) == reserved as int);
        assert(lo as int == crate::model::le16(bytes@, p as int + 4));
        assert(hi as int == crate::model::le16(bytes@, p as int + 6));
        assert(size as int == crate::model::le32(bytes@, p as int + 8));
        assert(offset as int == crate::model::le32(bytes@, p as int + 12));
        assert(width as int == crate::model::nom_dim(w0 as int));
        assert(height as int == crate::model::nom_dim(h0 as int));
    }

    // R4: the whole-directory guard has already succeeded, so the reserved byte
    // chooses between two defects and the range branch.
    let status = if reserved != 0 {
        proof {
            assert(crate::model::defect_of(bytes@, hr.dir_end, reserved as int, size as int,
                offset as int) == Some(EntryDefect::InvalidEntryReserved));
        }
        EntryStatus::Invalid(EntryDefect::InvalidEntryReserved)
    } else {
        let st = record_status(bytes, dir_end, offset, size, Ghost(hr), Ghost(i as int));
        proof {
            assert(crate::model::defect_of(bytes@, hr.dir_end, reserved as int, size as int,
                offset as int) == crate::model::defect_of(bytes@, hr.dir_end, 0, size as int,
                offset as int));
        }
        st
    };

    // R5: ICO keeps `lo`/`hi` as planes/bit depth; CUR keeps them as the two raw
    // hotspot words.
    let (planes, bit_count, hotspot_x, hotspot_y) = if typ == TYPE_ICO {
        (Some(lo), Some(hi), None, None)
    } else {
        (None, None, Some(lo), Some(hi))
    };

    let e = Entry {
        width,
        height,
        color_count,
        raw_offset: offset,
        raw_size: size,
        planes,
        bit_count,
        hotspot_x,
        hotspot_y,
        status,
    };
    proof {
        assert(typ as int == hr.typ);
        assert(crate::model::me(bytes@, hr, i as int) == Entry {
            width: width,
            height: height,
            color_count: color_count,
            raw_offset: offset,
            raw_size: size,
            planes,
            bit_count,
            hotspot_x,
            hotspot_y,
            status,
        }) by {
            reveal(crate::model::me);
            reveal(crate::model::nom_dim);
            assert(hr.typ == 1 || hr.typ == 2);
        }
    }
    e
}

// ---------------------------------------------------------------------------
// parse (R1, R3, R4, R10)
// ---------------------------------------------------------------------------

/// R1: `parse(bytes) = MC(bytes)`. `Err` only for the R2/R3 directory-wide
/// errors; otherwise every `count` records in wire order, invalid ones included.
/// The postcondition also carries T3 and T4 on the success branch, discharged by
/// the model lemmas `lemma_parse_ok_well_formed` / `lemma_parse_ok_classified`.
pub fn parse(bytes: &[u8]) -> (r: Result<Vec<Entry>, Error>)
    ensures
        crate::model::parse_eq(bytes@, r),
        match r {
            Ok(es) => crate::model::entries_well_formed(bytes@, es@) && crate::model::classification_ok(
                bytes@,
                es@,
            ),
            Err(_) => true,
        },
{
    let header = match directory_guard(bytes) {
        Ok(h) => h,
        Err(e) => {
            proof {
                crate::model::lemma_mc_err(bytes@, e);
            }
            return Err(e);
        },
    };
    let ghost hr =
        choose|hr: crate::model::HRec| crate::model::h(bytes@) == Ok(hr) && hdr_matches(hr, header);
    proof {
        reveal(hdr_matches);
        assert(crate::model::h(bytes@) == Ok(hr));
        assert(hdr_matches(hr, header));
        assert(1 <= hr.count <= crate::model::max_entries());
        assert(hr.count == header.count as int);
        assert(hr.dir_end == header.dir_end as int);
        assert(hr.typ == header.kind as int);
    }
    // R2/R13: one bounded reservation, only after directory/count/table guards.
    let mut entries: Vec<Entry> = Vec::with_capacity(header.count);
    let mut i = 0usize;
    let ghost mut acc: Seq<Entry> = Seq::empty();
    proof {
        assert(acc == Seq::new(0, |j: int| crate::model::me(bytes@, hr, j)));
    }
    while i < header.count
        invariant
            i <= header.count,
            header.count as int == hr.count,
            hr.count <= crate::model::max_entries(),
            crate::model::h(bytes@) == Ok(hr),
            hr.dir_end == 6 + 16 * hr.count,
            hr.dir_end <= bytes@.len(),
            bytes@.len() <= crate::model::max_input(),
            hr.typ == header.kind as int,
            hr.dir_end == header.dir_end as int,
            entries@ == acc,
            acc.len() == i,
            acc == Seq::new(i as nat, |j: int| crate::model::me(bytes@, hr, j)),
        decreases header.count - i,
    {
        let e = reconstruct(bytes, header.kind, header.dir_end, i, Ghost(hr));
        entries.push(e);
        proof {
            acc = acc.push(e);
            lemma_seq_new_snoc(|j: int| crate::model::me(bytes@, hr, j), i as nat);
            assert(acc == Seq::new((i + 1) as nat, |j: int| crate::model::me(bytes@, hr, j)));
        }
        i += 1;
    }
    proof {
        assert(entries@ == Seq::new(hr.count as nat, |j: int| crate::model::me(bytes@, hr, j)));
        crate::model::lemma_mc_ok(bytes@, hr);
        assert(crate::model::parse_eq(bytes@, Ok(entries)));
        crate::model::lemma_parse_ok_well_formed(bytes@, entries@);
        crate::model::lemma_parse_ok_classified(bytes@, entries@);
    }
    Ok(entries)
}

// ---------------------------------------------------------------------------
// decode_entry (R1, R8, R10) — the ICO-side decision
// ---------------------------------------------------------------------------

/// R9: the `Error` a selected defect returns, matching `model::error_of_defect`.
fn defect_to_error(d: EntryDefect) -> (r: Error)
    ensures
        r == crate::model::error_of_defect(d),
{
    match d {
        EntryDefect::InvalidEntryReserved => Error::InvalidEntryReserved,
        EntryDefect::EmptyEntry => Error::EmptyEntry,
        EntryDefect::EntryOverlapsDirectory => Error::EntryOverlapsDirectory,
        EntryDefect::EntryOutOfBounds => Error::EntryOutOfBounds,
    }
}

/// R6: the model's kind code, computed.
fn kind_code_exec(k: EntryKind) -> (r: u8)
    ensures
        r == crate::model::kind_code(k),
{
    match k {
        EntryKind::Png => 0u8,
        EntryKind::Dib => 1u8,
    }
}

/// R4: the model's defect code, computed.
fn defect_code_exec(d: EntryDefect) -> (r: u8)
    ensures
        r == crate::model::defect_code(d),
{
    match d {
        EntryDefect::InvalidEntryReserved => 0u8,
        EntryDefect::EmptyEntry => 1u8,
        EntryDefect::EntryOverlapsDirectory => 2u8,
        EntryDefect::EntryOutOfBounds => 3u8,
    }
}

/// R1/R8: `Option<u16>` equality, matching `model::opt_u16_eq`.
fn opt_u16_eq_exec(a: Option<u16>, b: Option<u16>) -> (r: bool)
    ensures
        r == crate::model::opt_u16_eq(a, b),
{
    reveal(crate::model::opt_u16_eq);
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

/// R1/R8: status equality, matching `model::status_eq`.
fn status_eq_exec(a: EntryStatus, b: EntryStatus) -> (r: bool)
    ensures
        r == crate::model::status_eq(a, b),
{
    reveal(crate::model::status_eq);
    match a {
        EntryStatus::Invalid(d1) => match b {
            EntryStatus::Invalid(d2) => {
                let c1 = defect_code_exec(d1);
                let c2 = defect_code_exec(d2);
                c1 == c2
            },
            _ => false,
        },
        EntryStatus::Valid { kind: k1, payload: p1 } => match b {
            EntryStatus::Valid { kind: k2, payload: p2 } => {
                let c1 = kind_code_exec(k1);
                let c2 = kind_code_exec(k2);
                c1 == c2 && p1[0] == p2[0] && p1[1] == p2[1]
            },
            _ => false,
        },
    }
}

/// R8: `entries[index]` equals its wire reconstruction, field for field and
/// status for status — the model `entry_eq`, computed.
fn record_eq(a: &Entry, b: &Entry) -> (r: bool)
    ensures
        r == crate::model::entry_eq(*a, *b),
{
    proof {
        reveal(crate::model::entry_eq);
    }
    let st = status_eq_exec(a.status, b.status);
    let opt_planes = opt_u16_eq_exec(a.planes, b.planes);
    let opt_bb = opt_u16_eq_exec(a.bit_count, b.bit_count);
    let opt_hx = opt_u16_eq_exec(a.hotspot_x, b.hotspot_x);
    let opt_hy = opt_u16_eq_exec(a.hotspot_y, b.hotspot_y);
    a.width == b.width && a.height == b.height && a.color_count == b.color_count && a.raw_offset
        == b.raw_offset && a.raw_size == b.raw_size && opt_planes && opt_bb && opt_hx && opt_hy && st
}

/// R1/R8: total for every byte slice, every typed `Entry` slice and every `u32`
/// index. The postcondition is the ICO side of R8 exactly — the R2/R3 guards,
/// the index check before the list check, the selected-record mismatch, the
/// selected defect and PNG delegation in R10 order (`model::md_ico`) — and, on
/// the selected DIB path, the DIB core's own public contract applied to the exact
/// declared slice. The model `MD` and the theorem T5 that names it are M3.
pub fn decode_entry(bytes: &[u8], entries: &[Entry], index: u32) -> (r: Result<Decoded, Error>)
    ensures
        crate::model::decode_pre(bytes@, entries@, index, r),
{
    // R8: the R2/R3 guards first, without allocating an entry list.
    let header = match directory_guard(bytes) {
        Ok(h) => h,
        Err(e) => {
            proof {
                reveal(crate::model::decode_pre);
                reveal(crate::model::md_ico);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::DirErr(e));
            }
            return Err(e);
        },
    };
    let ghost hr =
        choose|hr: crate::model::HRec| crate::model::h(bytes@) == Ok(hr) && hdr_matches(hr, header);
    proof {
        reveal(hdr_matches);
        assert(crate::model::h(bytes@) == Ok(hr));
        assert(hdr_matches(hr, header));
        assert(hr.count == header.count as int);
        assert(hr.dir_end == header.dir_end as int);
        assert(hr.typ == header.kind as int);
        assert(1 <= hr.count <= crate::model::max_entries());
    }
    // R8/R10: index before list length, and the index is kept `u32`-wide.
    if index >= header.count as u32 {
        proof {
            reveal(crate::model::decode_pre);
            reveal(crate::model::md_ico);
            assert(index as int >= hr.count);
            assert(header.count as u32 == hr.count);
            assert(crate::model::md_ico(bytes@, entries@, index)
                == crate::model::IcoDecision::IndexOOR);
        }
        return Err(Error::IndexOutOfRange);
    }
    proof {
        assert((index as int) < hr.count);
        assert((index as usize) < header.count);
    }
    if entries.len() != header.count {
        proof {
            reveal(crate::model::decode_pre);
            reveal(crate::model::md_ico);
            assert((index as int) < hr.count);
            assert(entries@.len() != hr.count);
            assert(crate::model::md_ico(bytes@, entries@, index)
                == crate::model::IcoDecision::BadList);
        }
        return Err(Error::InvalidEntryRecord);
    }
    proof {
        assert(entries@.len() == header.count);
        assert(entries@.len() == hr.count);
        assert((index as usize) < entries@.len());
        assert((index as usize) < entries.len());
    }
    // R8: only `entries.len()` and `entries[index]` may be inspected.
    let selected = entries[index as usize];
    proof {
        assert(selected == entries@[index as int]);
    }
    // R8/R10: reconstruct the selected record from the current wire bytes and
    // require field-for-field, status-for-status equality.
    let reconstructed = reconstruct(bytes, header.kind, header.dir_end, index as usize, Ghost(hr));
    proof {
        assert((index as usize) as int == index as int);
        assert(reconstructed == crate::model::me(bytes@, hr, (index as usize) as int));
        assert(reconstructed == crate::model::me(bytes@, hr, index as int));
    }
    let record_matches = record_eq(&selected, &reconstructed);
    if !record_matches {
        proof {
            reveal(crate::model::decode_pre);
            reveal(crate::model::md_ico);
            assert((index as int) < hr.count);
            assert(entries@.len() == hr.count);
            assert(selected == entries@[index as int]);
            assert(!crate::model::entry_eq(entries@[index as int], crate::model::me(
                bytes@,
                hr,
                index as int,
            ))) by {
                crate::model::lemma_entry_eq_iff(selected, reconstructed);
                if crate::model::entry_eq(entries@[index as int], crate::model::me(
                    bytes@,
                    hr,
                    index as int,
                )) {
                    crate::model::lemma_entry_eq_iff(entries@[index as int], crate::model::me(
                        bytes@,
                        hr,
                        index as int,
                    ));
                    assert(entries@[index as int] == crate::model::me(bytes@, hr, index as int));
                    assert(selected == crate::model::me(bytes@, hr, index as int));
                    assert(selected == reconstructed);
                    crate::model::lemma_eq_implies_entry_eq(selected, reconstructed);
                    assert(record_matches);
                }
            }
            assert(crate::model::md_ico(bytes@, entries@, index)
                == crate::model::IcoDecision::BadList);
        }
        return Err(Error::InvalidEntryRecord);
    }
    proof {
        assert(record_matches);
        crate::model::lemma_entry_eq_iff(selected, reconstructed);
        assert(selected == reconstructed);
        assert(selected == crate::model::me(bytes@, hr, index as int));
        assert(selected == entries@[index as int]);
        assert(entries@[index as int] == crate::model::me(bytes@, hr, index as int));
        reveal(crate::model::decode_pre);
        reveal(crate::model::md_ico);
        assert((index as int) < hr.count);
        assert(entries@.len() == hr.count);
        assert(crate::model::entry_eq(entries@[index as int], crate::model::me(bytes@, hr, index as int)));
    }
    match selected.status {
        EntryStatus::Invalid(defect) => {
            proof {
                reveal(crate::model::error_of_defect);
                assert(selected.status == crate::model::me(bytes@, hr, index as int).status);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::Defect(defect));
            }
            Err(defect_to_error(defect))
        },
        EntryStatus::Valid { kind: EntryKind::Png, .. } => {
            proof {
                assert(selected.status == crate::model::me(bytes@, hr, index as int).status);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::Png);
            }
            Err(Error::PngDelegation)
        },
        EntryStatus::Valid { kind: EntryKind::Dib, payload } => {
            // R8: exactly `s = &bytes[start..end)`, called once. The range was
            // checked by `reconstruct` for this very `bytes`.
            let start = payload[0] as usize;
            let end = payload[1] as usize;
            proof {
                assert(selected.status == crate::model::me(bytes@, hr, index as int).status);
                crate::model::lemma_valid_payload_in_bounds(bytes@, hr, index as int);
                assert(start <= end <= bytes@.len());
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::DibSlice(payload[0] as int, payload[1] as int));
            }
            match bytes.get(start..end) {
                Some(slice) => match dib_verus::decode(slice, Mode::IcoEntryDibSlack) {
                    Ok(d) => Ok(d),
                    Err(e) => Err(Error::Dib(e)),
                },
                None => {
                    proof {
                        assert(false);
                    }
                    Err(Error::EntryOutOfBounds)
                },
            }
        },
    }
}

// ---------------------------------------------------------------------------
// T1, T3, T4, T7 witnesses and their supporting lemmas
// ---------------------------------------------------------------------------

/// The `Seq::new`-snoc step used by `parse`'s loop: growing the ghost list by one
/// record appends the next `ME`.
pub proof fn lemma_seq_new_snoc<A>(f: spec_fn(int) -> A, k: nat)
    ensures
        Seq::new(k + 1, f) =~= Seq::new(k, f).push(f(k as int)),
{
    let s = Seq::new(k, f);
    let t = Seq::new(k + 1, f);
    assert forall|i: int| 0 <= i < t.len() implies #[trigger] t[i] == s.push(f(k as int))[i] by {
        if i == k {
            lemma_seq_new_index((k + 1) as nat, f, i);
            lemma_seq_push_index_same(s, f(k as int), i);
        } else {
            lemma_seq_new_index((k + 1) as nat, f, i);
            lemma_seq_new_index(k, f, i);
            lemma_seq_push_index_different(s, f(k as int), i);
        }
    }
    lemma_seq_new_len(k + 1, f);
    lemma_seq_push_len(s, f(k as int));
}


// ---------------------------------------------------------------------------
// T1, T3, T4, T7: the public witness lemmas.
// ---------------------------------------------------------------------------

/// T7 (arithmetic and output bounds), stated at the manifest's quantification.
/// For every byte slice and every admissible directory: the table end is at most
/// 16,390 and every validated range lies inside the file and inside `u32`. The
/// width-generic argument: the cap `|b| <= 67,108,864` makes every `usize`
/// quantity here at most `2^26`, far below `u32::MAX = 2^32 - 1`, which is at
/// most `usize::MAX` on both the 32-bit (2^32 - 1) and the 64-bit (2^64 - 1)
/// target, so none of the `u64` comparisons can wrap and none of the narrowings
/// can lose a bit.
pub proof fn t7_arithmetic_bounds(b: Seq<u8>, hr: crate::model::HRec)
    requires
        crate::model::h(b) == Ok(hr),
    ensures
        hr.dir_end <= 16390,
        1 <= hr.count <= 1024,
        hr.dir_end <= b.len(),
        b.len() <= 67108864,
        crate::model::max_directory_len() + 16 <= 4294967296,
        crate::model::max_input() <= 4294967296,
        forall|i: int| 0 <= i < hr.count && #[trigger] crate::model::me(b, hr, i).status is Valid
            ==> ({
                let st = crate::model::me(b, hr, i);
                &&& hr.dir_end <= (st.raw_offset as int)
                &&& (st.raw_offset as int) < (st.raw_offset as int) + (st.raw_size as int)
                &&& (st.raw_offset as int) + (st.raw_size as int) <= b.len()
            }),
{
    reveal(crate::model::h);
    crate::model::lemma_dir_end_bound(hr.count);
    crate::model::lemma_u32_width(b);
    assert forall|i: int| 0 <= i < hr.count && #[trigger] crate::model::me(b, hr, i).status is Valid
        implies ({
        let st = crate::model::me(b, hr, i);
        &&& hr.dir_end <= (st.raw_offset as int)
        &&& (st.raw_offset as int) < (st.raw_offset as int) + (st.raw_size as int)
        &&& (st.raw_offset as int) + (st.raw_size as int) <= b.len()
    }) by {
        crate::model::lemma_valid_payload_in_bounds(b, hr, i);
        reveal(crate::model::me);
        reveal(crate::model::defect_of);
    }
}

/// T2 (exact parse), stated on the parse result: exactly `MC(b)` as a whole.
pub proof fn t2_parse_exact(b: Seq<u8>, r: Result<Vec<Entry>, Error>)
    requires
        crate::model::parse_eq(b, r),
    ensures
        match crate::model::mc(b) {
            Ok(es) => match r {
                Ok(rs) => rs@ == es,
                Err(_) => false,
            },
            Err(e) => match r {
                Ok(_) => false,
                Err(e2) => e2 == e,
            },
        },
{
    reveal(crate::model::parse_eq);
}

/// T3 (record validity): every success of `parse` has `|E| = count` with `count`
/// in `[1, 1024]`, each record is its wire-derived `ME`, and each valid record's
/// payload lies in `[dir_end, |b|]` under the 64 MiB cap.
pub proof fn t3_record_validity(b: Seq<u8>, v: Vec<Entry>)
    requires
        crate::model::parse_eq(b, Ok(v)),
    ensures
        crate::model::entries_well_formed(b, v@),
{
    crate::model::lemma_parse_eq_ok(b, v);
    crate::model::lemma_parse_ok_well_formed(b, v@);
}

/// T4 (signature classification): for every valid record `E[j]`, `kind = Png`
/// iff `raw_size >= 8` and the eight bytes at `raw_offset` are the signature.
pub proof fn t4_signature_classification(b: Seq<u8>, v: Vec<Entry>)
    requires
        crate::model::parse_eq(b, Ok(v)),
    ensures
        crate::model::classification_ok(b, v@),
{
    crate::model::lemma_parse_eq_ok(b, v);
    crate::model::lemma_parse_ok_classified(b, v@);
}

/// T1 (safety of the ICO side of `decode_entry`): the whole ICO-controlled result
/// is fixed by R8/R10 order, and on the selected-DIB branch the result is a DIB
/// success or a DIB error and nothing else. (The DIB bytes themselves are M3.)
pub proof fn t1_decode_ico(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>)
    requires
        crate::model::decode_pre(b, es, index, r),
    ensures
        match crate::model::md_ico(b, es, index) {
            crate::model::IcoDecision::DirErr(e) => r == Err(e),
            crate::model::IcoDecision::IndexOOR => r == Err(Error::IndexOutOfRange),
            crate::model::IcoDecision::BadList => r == Err(Error::InvalidEntryRecord),
            crate::model::IcoDecision::Defect(d) => r == Err(crate::model::error_of_defect(d)),
            crate::model::IcoDecision::Png => r == Err(Error::PngDelegation),
            crate::model::IcoDecision::DibSlice(_s, _e) => match r {
                Ok(_) => true,
                Err(Error::Dib(_)) => true,
                Err(_) => false,
            },
        },
{
    reveal(crate::model::decode_pre);
}

/// T1 (safety) also states that a forged range, kind, list length or index
/// cannot panic. The proof is Verus verifying the `decode_entry` body, which has
/// no `requires`; this lemma records the fact that the selected index is kept
/// `u32`-wide, so a 256th or 65,536th record is addressed, not narrowed.
pub proof fn t1_index_u32(index: u32)
    ensures
        (index as int) <= 4294967295,
        (index as int) < 4294967296,
{
}

} // verus!
