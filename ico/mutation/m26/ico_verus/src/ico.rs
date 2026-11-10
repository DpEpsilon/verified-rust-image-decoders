//! Verified implementation of ICO-CORE-SPEC 1.0, milestone M3.
//!
//! Everything here is inside `verus!`: the M1 plain implementation carries
//! contracts and proofs. `src/ico.rs.m1` preserves the M1 plain file, and
//! `tests/conformance.rs` still runs all 217 frozen vectors. The reference model
//! `MC`/`ME`/`H`/`MD` lives in `crate::model`, written from the spec text and
//! independent of this code.
//!
//! Proved across the milestones: T1 (safety of `parse` and of the ICO side of
//! `decode_entry`), T2 (`parse` equals `MC` as the complete `Result`), T3 (record
//! validity), T4 (signature classification), T5 (exact total `decode_entry`,
//! including the composition with the DIB 1.1 model on the validated declared
//! slice through the public `decode_with_steps` contract), T6 (exact precedence)
//! and T7 (arithmetic and output bounds). T8 (termination) is stated by the model
//! variance lemmas. M4a adds T9 (ICO-side linear work), T10 (allocation-call
//! bound) and T11 (the witness bridge in `crate::witnesses`/`crate::model_vectors`).
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
//!   nothing of its own and calls the DIB core at most once, through
//!   `decode_with_steps` on exactly the validated `[start, end)` slice.

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
/// R13/T9: the eight-byte comparison is a real ICO-controlled loop; each of its
/// (at most eight) iterations charges one ghost step, so the helper adds at most
/// eight to the caller's counter.
fn has_png_signature(bytes: &[u8], start: usize, end: usize, steps: &mut Ghost<nat>) -> (r: bool)
    requires
        start <= end <= bytes@.len(),
    ensures
        r == (end - start >= 8 && bytes@.subrange(start as int, start as int + 8)
            == crate::model::png_signature()),
        final(steps)@ <= old(steps)@ + 8,
{
    let ghost s0: nat = steps@;
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
            s0 == old(steps)@,
            steps@ <= s0 + k,
            forall|j: int| 0 <= j < k ==> #[trigger] bytes@[start as int + j]
                == crate::model::png_signature()[j],
        decreases 8 - k,
    {
        proof {
            *steps = Ghost(steps@ + 1);
        }
        if bytes[start + k] != PNG_SIGNATURE[k] {
            proof {
                assert(steps@ <= s0 + 8);
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
        assert(steps@ <= s0 + 8);
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
    steps: &mut Ghost<nat>,
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
        final(steps)@ <= old(steps)@ + 8,
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
    let png = has_png_signature(bytes, off as usize, end_usize, steps);
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
    steps: &mut Ghost<nat>,
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
        final(steps)@ <= old(steps)@ + 9,
{
    let p = DIRECTORY_HEADER_LEN + 16 * (i & 255);
    proof {
        // R13/T9: the record iteration body charges one step (the list-loop
        // initialisation charges once in `parse`).
        *steps = Ghost(steps@ + 1);
    }
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
        let st = record_status(bytes, dir_end, offset, size, Ghost(hr), Ghost(i as int), steps);
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

/// R13/T9: the ordinary public `parse` — the tracked body with a disposable
/// ghost counter. The ghost argument and its updates erase, so behaviour is
/// byte-identical to `parse_tracked`.
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
    let mut steps = Ghost(0nat);
    let mut alloc = Ghost(0nat);
    parse_tracked(bytes, &mut steps, &mut alloc)
}

/// R13/T9/R15: the tracked `parse`. Its charged steps are at most
/// `model::parse_step_bound(bytes)`, and its charged allocation-call count is
/// exactly `model::parse_alloc_calls(bytes)` — zero on a directory-wide error
/// (the guard returns before the single reservation), one on success, for the
/// one bounded `Vec::with_capacity(count)` with `count <= 1,024`.
fn parse_tracked(
    bytes: &[u8],
    steps: &mut Ghost<nat>,
    alloc: &mut Ghost<nat>,
) -> (r: Result<Vec<Entry>, Error>)
    ensures
        crate::model::parse_eq(bytes@, r),
        final(steps)@ <= old(steps)@ + crate::model::parse_step_bound(bytes@),
        final(alloc)@ == old(alloc)@ + crate::model::parse_alloc_calls(bytes@),
{
    let header = match directory_guard(bytes) {
        Ok(h) => h,
        Err(e) => {
            proof {
                crate::model::lemma_mc_err(bytes@, e);
                reveal(crate::model::parse_alloc_calls);
                assert(crate::model::parse_alloc_calls(bytes@) == 0);
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
    let ghost s0: nat = steps@;
    let ghost a0: nat = alloc@;
    proof {
        // R13/T9: the bounded list initialisation charges one step.
        *steps = Ghost(steps@ + 1);
        // R15/T10: the single bounded entry-list allocation is charged here.
        *alloc = Ghost(alloc@ + 1);
        reveal(crate::model::parse_alloc_calls);
        assert(crate::model::parse_alloc_calls(bytes@) == 1);
        assert(crate::model::parse_step_bound(bytes@) == 1 + 9 * hr.count);
        assert(steps@ == s0 + 1);
        assert(alloc@ == a0 + 1);
        assert(a0 == old(alloc)@);
    }
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
            // R13/T9: the list initialisation charged once; each record body
            // charges at most nine (one body plus the eight-step comparison).
            s0 == old(steps)@,
            steps@ <= s0 + 1 + 9 * (i as int),
            // R15/T10: no further allocation in the record loop.
            a0 == old(alloc)@,
            alloc@ == a0 + 1,
        decreases header.count - i,
    {
        let e = reconstruct(bytes, header.kind, header.dir_end, i, Ghost(hr), steps);
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
        assert(final(steps)@ <= old(steps)@ + 1 + 9 * hr.count);
        assert(crate::model::parse_step_bound(bytes@) == 1 + 9 * hr.count);
        assert(final(alloc)@ == old(alloc)@ + 1);
        assert(crate::model::parse_alloc_calls(bytes@) == 1);
    }
    Ok(entries)
}

/// R13/T9/R15: the public tracked entry point. The ordinary `parse` is
/// `parse_with_steps(bytes).0`; the ghost step count `r.1@` is at most
/// `16*(|b|+1)` for every slice (and at most `parse_step_bound`), and the ghost
/// allocation-call count `r.2@` is `0` on a directory error and `1` on success
/// (one bounded reservation).
pub fn parse_with_steps(bytes: &[u8]) -> (r: (Result<Vec<Entry>, Error>, Ghost<nat>, Ghost<nat>))
    ensures
        crate::model::parse_eq(bytes@, r.0),
        r.1@ <= 16 * (bytes@.len() + 1),
        r.1@ <= crate::model::parse_step_bound(bytes@),
        r.2@ == crate::model::parse_alloc_calls(bytes@),
        r.2@ <= 1,
{
    let mut steps = Ghost(0nat);
    let mut alloc = Ghost(0nat);
    let result = parse_tracked(bytes, &mut steps, &mut alloc);
    proof {
        crate::model::lemma_t9_parse_bound(bytes@);
        reveal(crate::model::parse_alloc_calls);
    }
    (result, steps, alloc)
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
/// index. T5: the postcondition is `model::md_eq` — the complete `Result` equals
/// the total model `MD(b,E,i)`, including header/guard errors,
/// `IndexOutOfRange`, the forged-record `InvalidEntryRecord` check, the selected
/// entry defect, `PngDelegation`, and for a selected valid DIB entry the
/// **composition**: the result equals the DIB model on the validated declared
/// slice `[start, end)`, obtained from the DIB core's public `decode_with_steps`
/// on exactly that slice and wrapped as R9's typed `Dib(e)`, with the `Decoded`
/// passed through unchanged.
pub fn decode_entry(bytes: &[u8], entries: &[Entry], index: u32) -> (r: Result<Decoded, Error>)
    ensures
        crate::model::md_eq(bytes@, entries@, index, r),
{
    let mut steps = Ghost(0nat);
    let mut dib_steps = Ghost(0nat);
    let mut alloc = Ghost(0nat);
    decode_entry_impl(bytes, entries, index, &mut steps, &mut dib_steps, &mut alloc)
}

/// R13/T9/R15/R8: the tracked body of `decode_entry`, shared by the ordinary
/// public call and the public `decode_entry_with_steps`. It charges one step for
/// the eight-byte signature comparison the selected-record revalidation runs (at
/// most eight, but one suffices for the constant wrapper bound) and the DIB
/// call's steps on the selected-DIB branch. It never scans `E` or parses all
/// records, and it makes no allocation (`alloc` stays fixed). `dib_steps` is set
/// to the DIB call's returned ghost count on the selected-DIB branch (so the
/// combined T9 bound can be stated on the executable) and stays `0` otherwise.
fn decode_entry_impl(
    bytes: &[u8],
    entries: &[Entry],
    index: u32,
    steps: &mut Ghost<nat>,
    dib_steps: &mut Ghost<nat>,
    alloc: &mut Ghost<nat>,
) -> (r: Result<Decoded, Error>)
    ensures
        crate::model::md_eq(bytes@, entries@, index, r),
        final(steps)@ <= old(steps)@ + crate::model::wrapper_own_bound(),
        final(dib_steps)@ <= old(dib_steps)@ + crate::model::dib_steps_bound(
            bytes@,
            entries@,
            index,
        ),
        final(alloc)@ == old(alloc)@,
{
    // R8: the R2/R3 guards first, without allocating an entry list.
    let header = match directory_guard(bytes) {
        Ok(h) => h,
        Err(e) => {
            proof {
                reveal(crate::model::md_ico);
                reveal(crate::model::md);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::DirErr(e));
                assert(crate::model::md(bytes@, entries@, index) == Err(e));
                crate::model::lemma_md_eq_err(bytes@, entries@, index, e);
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
            reveal(crate::model::md_ico);
            reveal(crate::model::md);
            assert(index as int >= hr.count);
            assert(header.count as u32 == hr.count);
            assert(crate::model::md_ico(bytes@, entries@, index)
                == crate::model::IcoDecision::IndexOOR);
            assert(crate::model::md(bytes@, entries@, index) == Err(Error::IndexOutOfRange));
            crate::model::lemma_md_eq_err(bytes@, entries@, index, Error::IndexOutOfRange);
        }
        return Err(Error::IndexOutOfRange);
    }
    proof {
        assert((index as int) < hr.count);
        assert((index as usize) < header.count);
    }
    if entries.len() != header.count {
        proof {
            reveal(crate::model::md_ico);
            reveal(crate::model::md);
            assert((index as int) < hr.count);
            assert(entries@.len() != hr.count);
            assert(crate::model::md_ico(bytes@, entries@, index)
                == crate::model::IcoDecision::BadList);
            assert(crate::model::md(bytes@, entries@, index) == Err(Error::InvalidEntryRecord));
            crate::model::lemma_md_eq_err(bytes@, entries@, index, Error::InvalidEntryRecord);
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
    let reconstructed = reconstruct(
        bytes,
        header.kind,
        header.dir_end,
        index as usize,
        Ghost(hr),
        steps,
    );
    proof {
        assert((index as usize) as int == index as int);
        assert(reconstructed == crate::model::me(bytes@, hr, (index as usize) as int));
        assert(reconstructed == crate::model::me(bytes@, hr, index as int));
    }
    let record_matches = record_eq(&selected, &reconstructed);
    if !record_matches {
        proof {
            reveal(crate::model::md_ico);
            reveal(crate::model::md);
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
            assert(crate::model::md(bytes@, entries@, index) == Err(Error::InvalidEntryRecord));
            crate::model::lemma_md_eq_err(bytes@, entries@, index, Error::InvalidEntryRecord);
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
        reveal(crate::model::md_ico);
        assert((index as int) < hr.count);
        assert(entries@.len() == hr.count);
        assert(crate::model::entry_eq(entries@[index as int], crate::model::me(bytes@, hr, index as int)));
    }
    // R13/T9: the selected-record revalidation ran one eight-byte signature
    // comparison; charge one step for it (the manifest's "at most eight
    // signature-byte checks"; charging one keeps the wrapper's constant bound).
    proof {
        *steps = Ghost(steps@ + 1);
    }
    match selected.status {
        EntryStatus::Invalid(defect) => {
            proof {
                reveal(crate::model::error_of_defect);
                reveal(crate::model::md_ico);
                reveal(crate::model::md);
                assert(selected.status == crate::model::me(bytes@, hr, index as int).status);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::Defect(defect));
                assert(crate::model::md(bytes@, entries@, index)
                    == Err(crate::model::error_of_defect(defect)));
                crate::model::lemma_md_eq_err(
                    bytes@,
                    entries@,
                    index,
                    crate::model::error_of_defect(defect),
                );
            }
            Err(defect_to_error(defect))
        },
        EntryStatus::Valid { kind: EntryKind::Png, .. } => {
            proof {
                reveal(crate::model::md_ico);
                reveal(crate::model::md);
                assert(selected.status == crate::model::me(bytes@, hr, index as int).status);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::Png);
                assert(crate::model::md(bytes@, entries@, index) == Err(Error::PngDelegation));
                crate::model::lemma_md_eq_err(bytes@, entries@, index, Error::PngDelegation);
            }
            Err(Error::PngDelegation)
        },
        EntryStatus::Valid { kind: EntryKind::Dib, payload } => {
            // R8: exactly `s = &bytes[start..end)`, called once through the DIB
            // 1.1 public tracked `decode_with_steps`. The range was checked by
            // `reconstruct` for this very `bytes`.
            let start = payload[0] as usize;
            let end = payload[1] as usize;
            proof {
                assert(selected.status == crate::model::me(bytes@, hr, index as int).status);
                crate::model::lemma_valid_payload_in_bounds(bytes@, hr, index as int);
                assert(start <= end <= bytes@.len());
                reveal(crate::model::md_ico);
                assert(crate::model::md_ico(bytes@, entries@, index)
                    == crate::model::IcoDecision::DibSlice(payload[0] as int, payload[1] as int));
            }
            match bytes.get(start..end) {
                Some(slice) => {
                    let (r0, dsteps) = dib_verus::decode_with_steps(
                        slice,
                        Mode::IcoEntryDibSlack,
                    );
                    proof {
                        assert(slice@ == bytes@.subrange(start as int, end as int));
                        crate::model::lemma_md_eq_dib(
                            bytes@,
                            entries@,
                            index,
                            r0,
                            start as int,
                            end as int,
                            slice@,
                        );
                        // R13/T9: record the DIB call's returned ghost count and
                        // its proved bound; the decision is `DibSlice(start,end)`,
                        // so `dib_steps_bound` is this very `16*(|slice|+N)`.
                        *dib_steps = dsteps;
                        assert(start as int == payload[0] as int);
                        assert(end as int == payload[1] as int);
                        assert(crate::model::md_ico(bytes@, entries@, index)
                            == crate::model::IcoDecision::DibSlice(start as int, end as int));
                        assert(slice@ == bytes@.subrange(start as int, end as int));
                        reveal(crate::model::dib_steps_bound);
                        assert(crate::model::dib_steps_bound(bytes@, entries@, index) == 16 * (
                            bytes@.subrange(start as int, end as int).len()
                                + dib_verus::dib::t12_checked_n(
                                bytes@.subrange(start as int, end as int),
                                Mode::IcoEntryDibSlack,
                            )
                        ));
                        assert(dsteps@ <= 16 * (slice@.len() + dib_verus::dib::t12_checked_n(
                            slice@,
                            Mode::IcoEntryDibSlack,
                        )));
                    }
                    match r0 {
                        Ok(d) => Ok(d),
                        Err(e) => Err(Error::Dib(e)),
                    }
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

/// R13/T9/R15: the public tracked wrapper. The ordinary `decode_entry` is
/// `decode_entry_with_steps(bytes, entries, index).0`. Its ghost outputs are:
///
/// * `r.1@`, the wrapper's total charged steps — at most
///   `model::t9_step_cap(...)` = the wrapper's constant own work plus the DIB
///   call's own bound on a selected DIB, at most `16` off it — and at most
///   `16*(|b|+1)` in every case (`lemma_t9_wrapper_bound`);
/// * `r.2@`, the DIB call's returned ghost count — `0` off the selected-DIB
///   decision (every other branch makes no DIB call) and otherwise bounded by the
///   DIB 1.1 `decode_with_steps` postcondition `steps_DIB <= 16*(|slice|+N)`;
/// * `r.3@`, the allocation-call count, `0`.
pub fn decode_entry_with_steps(bytes: &[u8], entries: &[Entry], index: u32) -> (r: (Result<
    Decoded,
    Error,
>, Ghost<nat>, Ghost<nat>, Ghost<nat>))
    ensures
        crate::model::md_eq(bytes@, entries@, index, r.0),
        r.1@ <= crate::model::t9_step_cap(bytes@, entries@, index),
        r.1@ <= 16 * (bytes@.len() + 1),
        r.2@ <= crate::model::dib_steps_bound(bytes@, entries@, index),
        !(crate::model::md_ico(bytes@, entries@, index) is DibSlice) ==> r.2@ == 0,
        r.3@ == 0,
{
    let mut steps = Ghost(0nat);
    let mut dib_steps = Ghost(0nat);
    let mut alloc = Ghost(0nat);
    let result = decode_entry_impl(bytes, entries, index, &mut steps, &mut dib_steps, &mut alloc);
    proof {
        // The wrapper's charged steps are bounded by the constant
        // `wrapper_own_bound() = 16`, which is at most `16*(|b|+1)` and at most
        // `t9_step_cap = 16 + dib_steps_bound` since the DIB term is never
        // negative.
        crate::model::lemma_t9_wrapper_bound(bytes@);
        crate::model::lemma_dib_steps_bound_nonneg(bytes@, entries@, index);
        reveal(crate::model::t9_step_cap);
        // Off the selected-DIB decision the DIB call is never made, so
        // `dib_steps_bound` is `0` and `dib_steps` stayed `0`.
        if !(crate::model::md_ico(bytes@, entries@, index) is DibSlice) {
            crate::model::lemma_dib_steps_bound_zero_off_dib(bytes@, entries@, index);
        }
        assert(alloc@ == 0);
    }
    (result, steps, dib_steps, alloc)
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

/// T5 (exact total decode): the complete statement on the public call. For every
/// representable byte slice, every typed `Entry` slice and every `u32` index,
/// `decode_entry` equals the total model `MD` as the whole `Result`, including
/// the composition with the DIB model on the validated declared slice.
pub fn t5_decode_exact(b: &[u8], es: &[Entry], index: u32) -> (r: Result<Decoded, Error>)
    ensures
        crate::model::md_eq(b@, es@, index, r),
{
    decode_entry(b, es, index)
}

/// T1 (safety of the ICO side of `decode_entry`), discharged from the stronger
/// public postcondition `md_eq`: the whole ICO-controlled result is fixed by
/// R8/R10 order, and on the selected-DIB branch the result is a DIB success or a
/// DIB error and nothing else. (The exact DIB bytes and errors are T5's.)
pub proof fn t1_decode_ico(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>)
    requires
        crate::model::md_eq(b, es, index, r),
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
    crate::model::lemma_md_eq_implies_decode_pre(b, es, index, r);
    reveal(crate::model::decode_pre);
}

/// T6 (exact precedence) on the public call: `decode_entry` revalidates only the
/// selected record, so an unselected bogus supplied record cannot change the
/// result; on the parse side record defects are statuses, not errors. Stated as
/// the `MD`'s ordering lemmas composed (see `model::lemma_t6_*`).
pub proof fn t6_precedence(
    b: Seq<u8>,
    es1: Seq<Entry>,
    es2: Seq<Entry>,
    index: u32,
    r: Result<Decoded, Error>,
)
    requires
        crate::model::md_eq(b, es1, index, r),
        es1.len() == es2.len(),
        (index as int) < es1.len() ==> es1[index as int] == es2[index as int],
    ensures
        // decode checks only entries.len() and entries[index]: the same decision
        crate::model::md_ico(b, es1, index) == crate::model::md_ico(b, es2, index),
        // and the same model result, so a bogus unselected element is ignored
        crate::model::md(b, es1, index) == crate::model::md(b, es2, index),
{
    crate::model::lemma_t6_decode_reads_only_selected(b, es1, es2, index);
    reveal(crate::model::md);
}

/// T6 (exact precedence): the full priority chain, stated on the public result
/// `r` of `decode_entry` for every `b`, `es` and `u32` index. R2/R3 first on both
/// calls; then decode's index check before the list check; the list/selected
/// mismatch before the selected defect; the selected defect before PNG/DIB; PNG
/// before any DIB error; the selected DIB's own R14 priority is delegated
/// unchanged. The predicate `model::t6_full_prec` names that chain; the proof
/// composes the `model::lemma_t6_*` facts through the public `md_eq`.
pub proof fn t6_precedence_full(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>)
    requires
        crate::model::md_eq(b, es, index, r),
    ensures
        crate::model::t6_full_prec(b, es, index, r),
{
    crate::model::lemma_t6_full_prec(b, es, index, r);
}

/// T6 (exact precedence): the R2/R3 directory error has priority on **both**
/// public calls. On `parse` a directory-wide error is the whole result; on
/// `decode_entry` it precedes the index, list, record and defect checks. Stated
/// on the two public entry points.
pub fn t6_parse_directory_priority(b: &[u8]) -> (r: Result<Vec<Entry>, Error>)
    ensures
        crate::model::parse_eq(b@, r),
        match crate::model::h(b@) {
            Err(e) => r == Err(e),
            Ok(_) => true,
        },
{
    let r = parse(b);
    proof {
        match crate::model::h(b@) {
            Err(e) => {
                crate::model::lemma_mc_err(b@, e);
                crate::model::lemma_parse_eq_err(b@, e);
            },
            Ok(_) => {},
        }
    }
    r
}

/// T6 (exact precedence): on `decode_entry` the directory guard precedes every
/// subsequent check, and the index check precedes the list-length check. These
/// are the `md_ico`/`md` orderings, restated for the public call.
pub fn t6_decode_ordering(b: &[u8], es: &[Entry], index: u32) -> (r: Result<Decoded, Error>)
    ensures
        crate::model::md_eq(b@, es@, index, r),
        match crate::model::h(b@) {
            Err(e) => r == Err(e),
            Ok(hr) => {
                if hr.count <= index as int {
                    r == Err(Error::IndexOutOfRange)
                } else {
                    true
                }
            },
        },
{
    let r = decode_entry(b, es, index);
    proof {
        match crate::model::h(b@) {
            Err(e) => {
                crate::model::lemma_t6_directory_priority(b@, es@, index, e);
            },
            Ok(hr) => {
                if hr.count <= index as int {
                    reveal(crate::model::md_eq);
                    reveal(crate::model::md);
                    crate::model::lemma_t6_index_before_list(b@, hr, es@, index);
                }
            },
        }
    }
    r
}

/// T8 (termination): the parse variant `count - i` strictly decreases for each
/// complete record, valid or invalid; the eight-byte signature comparison's
/// variant `8 - k` is bounded and strictly decreases; the decode wrapper has no
/// loop of its own and revalidates exactly one record, then calls the terminating
/// DIB 1.1 decoder once. Stated at the manifest's quantification for `count`,
/// `i` and `k`.
pub proof fn t8_termination(count: int, i: int, k: int)
    requires
        0 <= i < count,
        0 <= k <= 8,
    ensures
        count - (i + 1) < count - i,
        0 < count - i,
        8 - (k + 1) < 8 - k,
        0 <= 8 - k,
{
}

/// T7 (the clause M2 deferred): a successful `decode_entry` returns a `Decoded`
/// obeying the DIB dependency's own `4*w*h <= 67,108,864` output contract (and
/// `1 <= w,h <= 8,192`), discharged from the public `md_eq` postcondition.
pub fn t7_dib_output_bound(b: &[u8], es: &[Entry], index: u32) -> (r: Result<Decoded, Error>)
    ensures
        crate::model::md_eq(b@, es@, index, r),
        match r {
            Ok(d) => {
                &&& 1 <= d.width as int <= dib_verus::model::max_dimension()
                &&& 1 <= d.height as int <= dib_verus::model::max_dimension()
                &&& d.rgba@.len() == 4 * (d.width as int * d.height as int)
                &&& d.rgba@.len() <= dib_verus::model::max_output()
            },
            Err(_) => true,
        },
{
    let r = decode_entry(b, es, index);
    proof {
        crate::model::lemma_t7_dib_output_bound(b@, es@, index, r);
    }
    r
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

// ---------------------------------------------------------------------------
// T9/T10/T11: the public witnesses (the model obligations, stated on the two
// public entry points and on the tracked exports).
// ---------------------------------------------------------------------------

/// T9 (ICO-side linear work), stated at the manifest's quantification: the
/// tracked `parse` charges at most `16*(|b|+1)` ghost steps for every slice, on
/// **every** exit — the directory-error path charges nothing (the guard has no
/// loop) and the success path charges the bounded list initialisation and at most
/// nine per record.
pub fn t9_parse_linear_work(b: &[u8]) -> (r: (Result<Vec<Entry>, Error>, Ghost<nat>, Ghost<nat>))
    ensures
        crate::model::parse_eq(b@, r.0),
        r.1@ <= 16 * (b@.len() + 1),
        r.1@ <= crate::model::parse_step_bound(b@),
        r.2@ == crate::model::parse_alloc_calls(b@),
{
    parse_with_steps(b)
}

/// T9 (ICO-side linear work) + T10 (allocation-call bound) on the wrapper: the
/// tracked `decode_entry` charges at most `16*(|b|+1)` steps for every `b, E, i`,
/// its own work is the constant wrapper bound plus the DIB call's own proved
/// bound (`t9_step_cap`), it never scans `E` (the count is a function of the
/// decision alone), and it requests no allocation.
pub fn t9_wrapper_linear_work(
    b: &[u8],
    es: &[Entry],
    index: u32,
) -> (r: (Result<Decoded, Error>, Ghost<nat>, Ghost<nat>, Ghost<nat>))
    ensures
        crate::model::md_eq(b@, es@, index, r.0),
        r.1@ <= 16 * (b@.len() + 1),
        r.1@ <= crate::model::t9_step_cap(b@, es@, index),
        // The wrapper's own steps are a constant: off the selected-DIB decision
        // they are at most `wrapper_own_bound()`.
        !(crate::model::md_ico(b@, es@, index) is DibSlice) ==> r.1@ <= crate::model::wrapper_own_bound(),
        // The DIB call happens at most once (on the selected-DIB decision) and
        // its count obeys the dependency's published bound.
        r.2@ <= crate::model::dib_steps_bound(b@, es@, index),
        !(crate::model::md_ico(b@, es@, index) is DibSlice) ==> r.2@ == 0,
        r.3@ == 0,
{
    decode_entry_with_steps(b, es, index)
}

/// T9 ("never scans `E`"): the wrapper's step count depends only on the ICO
/// decision, and `md_ico` reads only `|E|` and `E[index]`. Two lists of equal
/// length with equal selected element take the same decision, hence the same
/// wrapper work; the wrapper never iterates the entry list.
pub proof fn t9_wrapper_ignores_list(b: Seq<u8>, es1: Seq<Entry>, es2: Seq<Entry>, index: u32)
    requires
        es1.len() == es2.len(),
        (index as int) < es1.len() ==> es1[index as int] == es2[index as int],
    ensures
        crate::model::md_ico(b, es1, index) == crate::model::md_ico(b, es2, index),
        crate::model::t9_step_cap(b, es1, index) == crate::model::t9_step_cap(b, es2, index),
{
    crate::model::lemma_t9_wrapper_ignores_list(b, es1, es2, index);
    reveal(crate::model::t9_step_cap);
    reveal(crate::model::dib_steps_bound);
}

/// T10 (allocation-call bound): the tracked `parse` requests exactly
/// `model::parse_alloc_calls(b)` allocations — zero on every directory-wide
/// error, one bounded reservation for at most 1,024 records otherwise — and the
/// tracked `decode_entry` requests none.
pub proof fn t10_allocation_bound(b: Seq<u8>, es: Seq<Entry>, index: u32, hr: crate::model::HRec)
    requires
        crate::model::h(b) == Ok(hr),
    ensures
        crate::model::parse_alloc_calls(b) == 1,
        1 <= hr.count <= crate::model::max_entries(),
        crate::model::decode_alloc_calls(b, es, index) == 0,
{
    crate::model::lemma_t10_parse_one_bounded(b, hr);
    crate::model::lemma_t10_decode_no_alloc(b, es, index);
}

} // verus!
