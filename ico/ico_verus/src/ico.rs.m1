//! Plain (unverified) implementation of ICO-CORE-SPEC 1.0, milestone M1.
//!
//! Everything here is ordinary Rust, outside any `verus!` block: no contracts and
//! no proofs yet. It exists so that the conformance harness in
//! `tests/conformance.rs` can run all 217 frozen vectors against the API that
//! SPEC R1 fixes, and so that later milestones have an executable to re-state.
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

// ---------------------------------------------------------------------------
// Limits (SPEC R2)
// ---------------------------------------------------------------------------

/// R2: `MAX_INPUT = 67,108,864` bytes. Exactly this length passes the cap test.
pub const MAX_INPUT: usize = 67_108_864;
/// R2: `MAX_ENTRIES = 1,024` records. The u16 wire count permits 65,535.
pub const MAX_ENTRIES: usize = 1_024;
/// R3: the shared directory header is six bytes; `U16(b,0)`, `U16(b,2)`, `U16(b,4)`.
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
    // Directory-wide errors (R3).
    FileTooLarge,
    TruncatedDirectory,
    InvalidReserved,
    UnsupportedType,
    EmptyDirectory,
    TooManyEntries,
    TruncatedEntries,
    // Entry defects, returned by `decode_entry` for the selected record only
    // (in `parse` they appear inside a status).
    InvalidEntryReserved,
    EmptyEntry,
    EntryOverlapsDirectory,
    EntryOutOfBounds,
    // Decode-only errors.
    IndexOutOfRange,
    InvalidEntryRecord,
    PngDelegation,
    // Wrapped DIB error.
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
// Guarded wire reads (R3, R4)
// ---------------------------------------------------------------------------

/// `U16(b, p)` under a guard, little-endian. Returns `None` unless both bytes
/// `p` and `p+1` exist in `b` and `p+1` does not overflow `usize`.
fn read_u16(b: &[u8], p: usize) -> Option<u16> {
    let hi = p.checked_add(1)?;
    if hi >= b.len() {
        return None;
    }
    let lo_byte = *b.get(p)?;
    let hi_byte = *b.get(hi)?;
    Some(u16::from_le_bytes([lo_byte, hi_byte]))
}

/// `U32(b, p)` under a guard, little-endian. Returns `None` unless bytes
/// `p..p+4` all exist in `b` and `p+3` does not overflow `usize`.
fn read_u32(b: &[u8], p: usize) -> Option<u32> {
    let last = p.checked_add(3)?;
    if last >= b.len() {
        return None;
    }
    let v0 = *b.get(p)?;
    let v1 = *b.get(p + 1)?;
    let v2 = *b.get(p + 2)?;
    let v3 = *b.get(last)?;
    Some(u32::from_le_bytes([v0, v1, v2, v3]))
}

/// `b[p]` under a guard.
fn read_u8(b: &[u8], p: usize) -> Option<u8> {
    b.get(p).copied()
}

// ---------------------------------------------------------------------------
// The shared directory guard H (R3)
// ---------------------------------------------------------------------------

/// The `H(b)` result: the two metadata words and the end of the whole directory.
struct Header {
    kind: u16,
    count: usize,
    dir_end: u32,
}

/// R3, in this exact order: the length cap, six-byte availability, reserved
/// word, type, non-empty count, count cap, and the whole-table guard. The count
/// cap is tested before the table guard, so a count of 1,025 wins over an
/// incomplete table.
fn directory_guard(bytes: &[u8]) -> Result<Header, Error> {
    // R2: the cap is tested before the buffer is read at all.
    if bytes.len() > MAX_INPUT {
        return Err(Error::FileTooLarge);
    }
    // R3: all six header bytes must exist before any of the six-byte reads.
    if bytes.len() < DIRECTORY_HEADER_LEN {
        return Err(Error::TruncatedDirectory);
    }
    let reserved = match read_u16(bytes, 0) {
        Some(v) => v,
        None => return Err(Error::TruncatedDirectory),
    };
    if reserved != 0 {
        return Err(Error::InvalidReserved);
    }
    let kind = match read_u16(bytes, 2) {
        Some(v) => v,
        None => return Err(Error::TruncatedDirectory),
    };
    if kind != TYPE_ICO && kind != TYPE_CUR {
        return Err(Error::UnsupportedType);
    }
    let count_word = match read_u16(bytes, 4) {
        Some(v) => v,
        None => return Err(Error::TruncatedDirectory),
    };
    if count_word == 0 {
        return Err(Error::EmptyDirectory);
    }
    let count = count_word as usize;
    if count > MAX_ENTRIES {
        return Err(Error::TooManyEntries);
    }
    // R7/T7: `6 + 16*count <= 6 + 16*1,024 = 16,390`, computed in `u64` so the
    // multiplication cannot wrap on any target.
    let dir_end = DIRECTORY_HEADER_LEN as u64 + ENTRY_RECORD_LEN as u64 * count_word as u64;
    if dir_end > bytes.len() as u64 {
        return Err(Error::TruncatedEntries);
    }
    // `dir_end <= 16,390` and the input is at most 64 MiB, so this narrowing is
    // exact; the bound is kept explicit rather than relying on the `as` cast.
    if dir_end > u32::MAX as u64 {
        return Err(Error::TruncatedEntries);
    }
    Ok(Header {
        kind,
        count,
        dir_end: dir_end as u32,
    })
}

// ---------------------------------------------------------------------------
// Per-record reconstruction ME (R4-R6)
// ---------------------------------------------------------------------------

/// R4/R5/R6: reconstruct record `i` of a directory that already passed `H`.
/// The returned offset and size are the wire words; for a valid record they also
/// satisfy `dir_end <= start < end <= |b|`.
fn record_status(bytes: &[u8], dir_end: u32, offset_word: u32, len_word: u32) -> EntryStatus {
    // R4: first matching defect wins, and no later test overrides an earlier one.
    if len_word == 0 {
        return EntryStatus::Invalid(EntryDefect::EmptyEntry);
    }
    if offset_word < dir_end {
        return EntryStatus::Invalid(EntryDefect::EntryOverlapsDirectory);
    }
    // R4: the range test is `offset > |b| || size > |b| - offset`, evaluated in
    // `u64`. Forming `offset + size` directly would wrap in `u32`.
    let off = offset_word as u64;
    let len = bytes.len() as u64;
    if off > len || len_word as u64 > len - off {
        return EntryStatus::Invalid(EntryDefect::EntryOutOfBounds);
    }
    // R6: classify from the payload range alone; a seven-byte prefix must never
    // be completed by bytes beyond `end`. The range was just proved in bounds, so
    // `end` does not overflow and the slice cannot fail.
    let end = off + len_word as u64; // <= bytes.len(), so this cannot overflow
    let kind = match has_png_signature(bytes, offset_word as usize, end as usize) {
        true => EntryKind::Png,
        false => EntryKind::Dib,
    };
    EntryStatus::Valid {
        kind,
        payload: [offset_word, offset_word + len_word],
    }
}

/// R6: `true` iff the payload `[start, end)` is at least eight bytes long and its
/// first eight bytes equal [`PNG_SIGNATURE`]. Reads only bytes below `end`.
fn has_png_signature(bytes: &[u8], start: usize, end: usize) -> bool {
    match end.checked_sub(start) {
        Some(n) if n >= PNG_SIGNATURE.len() => {}
        _ => return false,
    }
    let mut k = 0usize;
    while k < PNG_SIGNATURE.len() {
        match bytes.get(start + k) {
            Some(v) => {
                if *v != PNG_SIGNATURE[k] {
                    return false;
                }
            }
            None => return false,
        }
        k += 1;
    }
    true
}

/// R4/R5/R6: reconstruct record `i` at `p = 6 + 16*i` from the wire. This is the
/// `ME` reconstruction; the read offsets are inside the already-guarded
/// directory, so the guarded reads below never fail for a directory that passed
/// `H`.
fn reconstruct(bytes: &[u8], typ: u16, dir_end: u32, i: usize) -> Option<Entry> {
    let p = DIRECTORY_HEADER_LEN.checked_add(ENTRY_RECORD_LEN.checked_mul(i)?)?;
    let w0 = read_u8(bytes, p)?;
    let h0 = read_u8(bytes, p + 1)?;
    let color_count = read_u8(bytes, p + 2)?;
    let reserved = read_u8(bytes, p + 3)?;
    let lo = read_u16(bytes, p + 4)?;
    let hi = read_u16(bytes, p + 6)?;
    let size = read_u32(bytes, p + 8)?;
    let offset = read_u32(bytes, p + 12)?;

    // R5: a zero nominal dimension means 256, in either word.
    let width = if w0 == 0 { 256u16 } else { w0 as u16 };
    let height = if h0 == 0 { 256u16 } else { h0 as u16 };

    // R4: the whole-directory guard has already succeeded, so the reserved byte
    // chooses between two defects and the range branch. The "first matching
    // defect" order is:
    //   reserved != 0, size = 0, offset < dir_end, offset/size out of file.
    let status = if reserved != 0 {
        EntryStatus::Invalid(EntryDefect::InvalidEntryReserved)
    } else {
        record_status(bytes, dir_end, offset, size)
    };

    // R5: ICO keeps `lo`/`hi` as planes/bit depth; CUR keeps them as the two raw
    // hotspot words. Both are uninterpreted raw words, never clipped.
    let (planes, bit_count, hotspot_x, hotspot_y) = if typ == TYPE_ICO {
        (Some(lo), Some(hi), None, None)
    } else {
        (None, None, Some(lo), Some(hi))
    };

    Some(Entry {
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
    })
}

// ---------------------------------------------------------------------------
// parse (R1, R3, R4, R10)
// ---------------------------------------------------------------------------

/// R1: `parse(bytes) = MC(bytes)`. `Err` only for the R2/R3 directory-wide
/// errors; otherwise every `count` records in wire order, invalid ones included.
pub fn parse(bytes: &[u8]) -> Result<Vec<Entry>, Error> {
    let header = directory_guard(bytes)?;
    // R2/R13: one bounded reservation, only after directory/count/table guards.
    let mut entries: Vec<Entry> = Vec::with_capacity(header.count);
    let mut i = 0usize;
    while i < header.count {
        match reconstruct(bytes, header.kind, header.dir_end, i) {
            Some(e) => entries.push(e),
            None => return Err(Error::TruncatedEntries),
        }
        i += 1;
    }
    Ok(entries)
}

// ---------------------------------------------------------------------------
// decode_entry (R1, R8, R10)
// ---------------------------------------------------------------------------

/// R1/R8: total for every byte slice, every typed `Entry` slice and every `u32`
/// index. It revalidates exactly one wire record, then delegates to the DIB core
/// on precisely that record's declared slice, or reports PNG delegation.
pub fn decode_entry(bytes: &[u8], entries: &[Entry], index: u32) -> Result<Decoded, Error> {
    // R8: the R2/R3 guards first, without allocating an entry list.
    let header = directory_guard(bytes)?;
    // R8/R10: index before list length, and the index is kept `u32`-wide.
    if index >= header.count as u32 {
        return Err(Error::IndexOutOfRange);
    }
    if entries.len() != header.count {
        return Err(Error::InvalidEntryRecord);
    }
    // R8: only `entries.len()` and `entries[index]` may be inspected.
    let selected = match entries.get(index as usize) {
        Some(e) => e,
        None => return Err(Error::InvalidEntryRecord),
    };
    // R8/R10: reconstruct the selected record from the current wire bytes and
    // require field-for-field, status-for-status equality.
    let reconstructed = match reconstruct(bytes, header.kind, header.dir_end, index as usize) {
        Some(e) => e,
        None => return Err(Error::InvalidEntryRecord),
    };
    if *selected != reconstructed {
        return Err(Error::InvalidEntryRecord);
    }
    match selected.status {
        EntryStatus::Invalid(defect) => Err(match defect {
            EntryDefect::InvalidEntryReserved => Error::InvalidEntryReserved,
            EntryDefect::EmptyEntry => Error::EmptyEntry,
            EntryDefect::EntryOverlapsDirectory => Error::EntryOverlapsDirectory,
            EntryDefect::EntryOutOfBounds => Error::EntryOutOfBounds,
        }),
        EntryStatus::Valid { kind: EntryKind::Png, .. } => Err(Error::PngDelegation),
        EntryStatus::Valid {
            kind: EntryKind::Dib,
            payload,
        } => {
            // R8: exactly `s = &bytes[start..end)`, called once. The range was
            // checked by `reconstruct` for this very `bytes`, so the slice is
            // in bounds; the `match` keeps that fact local instead of assuming it.
            let start = payload[0] as usize;
            let end = payload[1] as usize;
            match bytes.get(start..end) {
                Some(slice) => match dib_verus::decode(slice, Mode::IcoEntryDibSlack) {
                    Ok(d) => Ok(d),
                    Err(e) => Err(Error::Dib(e)),
                },
                None => Err(Error::EntryOutOfBounds),
            }
        }
    }
}
