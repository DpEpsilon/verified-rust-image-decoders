//! Formal reference model for ICO-CORE-SPEC 1.0 (milestones M2/M3).
//!
//! This file is written *independently of the executable code*: every definition
//! below is a transcription of the frozen specification text (`spec-frozen/SPEC.md`,
//! R2..R11) over `Seq<u8>` and mathematical integers. Nothing here calls,
//! mirrors the control flow of, or is derived from `crate::ico`. The executable
//! carries postconditions phrased in terms of these definitions; the definitions
//! themselves are never justified by running the executable.
//!
//! M2 supplies the total directory model `H`, the selected-record model `ME`, the
//! parse model `MC`, the equality model and the arithmetic lemmas. M3 adds the
//! total decode model `MD` (R8/R11: the ICO-controlled `md_ico` plus the
//! composition with the DIB 1.1 model `dib_verus::model::mp` on the validated
//! declared slice), the exact statement `md_eq`, and the T5/T6/T8 lemmas.
use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::seq::{lemma_seq_new_index, lemma_seq_new_len, lemma_seq_push_index_different, lemma_seq_push_index_same, lemma_seq_push_len};

#[allow(unused_imports)]
use crate::ico::{Entry, EntryDefect, EntryKind, EntryStatus, Error};
#[allow(unused_imports)]
use dib_verus::{Decoded, Mode};

verus! {

// ---------------------------------------------------------------------------
// R2, R7: the spec's numeric limits, as mathematical integers.
// ---------------------------------------------------------------------------

/// R2: `MAX_INPUT = 67,108,864` bytes. Exactly this length passes the cap test.
pub open spec fn max_input() -> int {
    67108864
}

/// R2: `MAX_ENTRIES = 1,024` records. The u16 wire count permits 65,535.
pub open spec fn max_entries() -> int {
    1024
}

/// R3: the shared directory header is six bytes.
pub open spec fn dir_header_len() -> int {
    6
}

/// R4: one directory record is 16 bytes.
pub open spec fn entry_record_len() -> int {
    16
}

/// R7/T7: `6 + 16*count <= 6 + 16*1024 = 16,390` for every admissible `count`.
pub open spec fn max_directory_len() -> int {
    16390
}

/// R6: `SIG = 89 50 4e 47 0d 0a 1a 0a`, the eight-byte PNG signature.
pub open spec fn png_signature() -> Seq<u8> {
    seq![0x89u8, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]
}

// ---------------------------------------------------------------------------
// R3, R4: guarded little-endian reads, as mathematical integers.
// ---------------------------------------------------------------------------

/// `U8(b,p) = b[p]`. Defined (meaningful) once `0 <= p < |b|`; every caller
/// below proves that before use.
pub open spec fn b8(b: Seq<u8>, p: int) -> int {
    b[p] as int
}

/// `U16(b,p) = b[p] + 256*b[p+1]`. Defined once `p+2 <= |b|`.
pub open spec fn le16(b: Seq<u8>, p: int) -> int {
    b[p] as int + 256 * (b[p + 1] as int)
}

/// `U32(b,p) = b[p] + 256*b[p+1] + 65536*b[p+2] + 16777216*b[p+3]`. Defined once
/// `p+4 <= |b|`.
pub open spec fn le32(b: Seq<u8>, p: int) -> int {
    b[p] as int + 256 * (b[p + 1] as int) + 65536 * (b[p + 2] as int) + 16777216 * (b[p + 3] as int)
}

// ---------------------------------------------------------------------------
// R3: the shared directory guard H(b).
// ---------------------------------------------------------------------------

/// The `H(b)` success record: the directory type and record count, and the end
/// of the whole directory table.
pub struct HRec {
    /// R3: 1 (ICO) or 2 (CUR).
    pub typ: int,
    /// R3: the wire count, an admissible value in `[1, 1024]`.
    pub count: int,
    /// R3: `6 + 16*count`.
    pub dir_end: int,
}

/// `H(b)`: the R2 cap, the six-byte availability, then R3's reserved word, type,
/// non-empty count, count cap and whole-table guard, in exactly that order.
/// Directory-wide failure only; no record is inspected.
pub open spec fn h(b: Seq<u8>) -> Result<HRec, Error> {
    if b.len() > max_input() {
        Err(Error::FileTooLarge)
    } else if b.len() < dir_header_len() {
        Err(Error::TruncatedDirectory)
    } else if le16(b, 0) != 0 {
        Err(Error::InvalidReserved)
    } else if !(le16(b, 2) == 1 || le16(b, 2) == 2) {
        Err(Error::UnsupportedType)
    } else if le16(b, 4) == 0 {
        Err(Error::EmptyDirectory)
    } else if le16(b, 4) > max_entries() {
        Err(Error::TooManyEntries)
    } else if dir_header_len() + entry_record_len() * le16(b, 4) > b.len() {
        Err(Error::TruncatedEntries)
    } else {
        Ok(HRec { typ: le16(b, 2), count: le16(b, 4), dir_end: 6 + 16 * le16(b, 4) })
    }
}

// ---------------------------------------------------------------------------
// R4..R6: the selected-record model ME(b,H,i).
// ---------------------------------------------------------------------------

/// R4: `w0 = 0` means 256, otherwise the raw byte.
pub open spec fn nom_dim(w0: int) -> int {
    if w0 == 0 {
        256
    } else {
        w0
    }
}

/// R4: the first-matching-defect test on the raw words of a complete record, in
/// R4's order: nonzero reserved byte, zero size, offset before the end of the
/// whole directory, then an out-of-file range. Returns the defect, or `None`
/// when the record is valid.
pub open spec fn defect_of(b: Seq<u8>, dir_end: int, reserved: int, size: int, offset: int)
    -> Option<EntryDefect> {
    if reserved != 0 {
        Some(EntryDefect::InvalidEntryReserved)
    } else if size == 0 {
        Some(EntryDefect::EmptyEntry)
    } else if offset < dir_end {
        Some(EntryDefect::EntryOverlapsDirectory)
    } else if offset > b.len() || size > b.len() - offset {
        Some(EntryDefect::EntryOutOfBounds)
    } else {
        None
    }
}

/// R6: is the payload `[offset, offset+size)` at least eight bytes long and its
/// first eight bytes exactly `SIG`? Never reads a byte at or beyond `end`.
pub open spec fn kind_of(b: Seq<u8>, offset: int, size: int) -> EntryKind {
    if size >= 8 && b.subrange(offset, offset + 8) == png_signature() {
        EntryKind::Png
    } else {
        EntryKind::Dib
    }
}

/// R4/R5/R6: the complete record `ME(b,H,i)` of the record at `p = 6 + 16*i`.
/// Reads all sixteen record bytes, derives every public field, chooses the first
/// R4 defect or a valid range with its R6 kind. `ME` is total on every `i`; the
/// `H` record supplies the type, the directory end and the validity of `i`.
pub open spec fn me(b: Seq<u8>, hr: HRec, i: int) -> Entry {
    let p = 6 + 16 * i;
    let w0 = b8(b, p);
    let h0 = b8(b, p + 1);
    let color_count = b8(b, p + 2);
    let reserved = b8(b, p + 3);
    let lo = le16(b, p + 4);
    let hi = le16(b, p + 6);
    let size = le32(b, p + 8);
    let offset = le32(b, p + 12);
    let status = match defect_of(b, hr.dir_end, reserved, size, offset) {
        Some(d) => EntryStatus::Invalid(d),
        None => EntryStatus::Valid {
            kind: kind_of(b, offset, size),
            payload: [offset as u32, (offset + size) as u32],
        },
    };
    let (planes, bit_count, hotspot_x, hotspot_y) = if hr.typ == 1 {
        (Some(lo as u16), Some(hi as u16), None, None)
    } else {
        (None, None, Some(lo as u16), Some(hi as u16))
    };
    Entry {
        width: nom_dim(w0) as u16,
        height: nom_dim(h0) as u16,
        color_count: color_count as u8,
        raw_offset: offset as u32,
        raw_size: size as u32,
        planes,
        bit_count,
        hotspot_x,
        hotspot_y,
        status,
    }
}

// ---------------------------------------------------------------------------
// R11: the total parse model MC(b).
// ---------------------------------------------------------------------------

/// `MC(b)`: `H` first; on success the **complete** ordered list of `count`
/// records, every `ME`, including every invalid status. No record defect makes
/// `MC` fail.
pub open spec fn mc(b: Seq<u8>) -> Result<Seq<Entry>, Error> {
    match h(b) {
        Err(e) => Err(e),
        Ok(hr) => Ok(Seq::new(hr.count as nat, |i: int| me(b, hr, i))),
    }
}

/// T2's exact statement: the executable `parse` result equals `MC(b)` as the
/// complete `Result` — the same directory-wide error, or the same list of
/// records in order.
pub open spec fn parse_eq(b: Seq<u8>, r: Result<Vec<Entry>, Error>) -> bool {
    match mc(b) {
        Ok(es) => match r {
            Ok(rs) => rs@ == es,
            Err(_) => false,
        },
        Err(e) => match r {
            Ok(_) => false,
            Err(e2) => e2 == e,
        },
    }
}

// ---------------------------------------------------------------------------
// T3: the success-record shape (record validity).
// ---------------------------------------------------------------------------

/// T3: on `parse(b) = Ok(E)`, `|E| = count` with `count` in `[1, 1024]`, and
/// every `E[j]` is exactly its wire-derived `ME` record, with a valid entry's
/// payload satisfying `dir_end <= start < end <= |b| <= 67,108,864`. An invalid
/// entry carries raw offset/size but no validated payload, so no range is
/// asserted for it. Derivable from `MC` alone.
pub open spec fn entries_well_formed(b: Seq<u8>, es: Seq<Entry>) -> bool {
    match h(b) {
        Err(_) => false,
        Ok(hr) => {
            &&& 1 <= hr.count <= max_entries()
            &&& es.len() == hr.count
            &&& forall|j: int| 0 <= j < es.len() ==> #[trigger] es[j] == me(b, hr, j)
            &&& forall|j: int| 0 <= j < es.len() ==> #[trigger] entry_range_ok(b, hr, es[j])
        },
    }
}

/// T3's range clause for one record: a valid entry's payload is `dir_end <=
/// start < end <= |b| <= 67,108,864`; an invalid entry is unconstrained.
pub open spec fn entry_range_ok(b: Seq<u8>, hr: HRec, e: Entry) -> bool {
    match e.status {
        EntryStatus::Valid { .. } => {
            &&& hr.dir_end <= e.raw_offset as int
            &&& (e.raw_offset as int) < (e.raw_offset as int) + (e.raw_size as int)
            &&& (e.raw_offset as int) + (e.raw_size as int) <= b.len()
            &&& b.len() <= max_input()
        },
        EntryStatus::Invalid(_) => true,
    }
}

// ---------------------------------------------------------------------------
// T4: signature classification on valid entries.
// ---------------------------------------------------------------------------

/// T4 for one record: if the record is valid, `kind = Png` iff `raw_size >= 8`
/// and the eight bytes at `raw_offset` equal `SIG`. An invalid record asserts
/// nothing about its kind.
pub open spec fn entry_class_ok(b: Seq<u8>, e: Entry) -> bool {
    match e.status {
        EntryStatus::Valid { kind, payload } => {
            let start = e.raw_offset as int;
            let size = e.raw_size as int;
            &&& payload[0] as int == start
            &&& payload[1] as int == start + size
            &&& (kind == EntryKind::Png) == (size >= 8 && b.subrange(start, start + 8)
                == png_signature())
        },
        EntryStatus::Invalid(_) => true,
    }
}

/// T4: every record of `es` satisfies [`entry_class_ok`].
pub open spec fn classification_ok(b: Seq<u8>, es: Seq<Entry>) -> bool {
    forall|j: int| 0 <= j < es.len() ==> #[trigger] entry_class_ok(b, es[j])
}

// ---------------------------------------------------------------------------
// Arithmetic lemmas (used only to justify the executable's bounds).
// ---------------------------------------------------------------------------

/// `le16(b,p) < 2^16` whenever both bytes exist: two `u8` reads cannot reach
/// `256*256`.
pub proof fn lemma_le16_lt(b: Seq<u8>, p: int)
    requires
        p + 2 <= b.len(),
    ensures
        0 <= le16(b, p) < 65536,
{
    assert(0 <= b[p] as int);
    assert((b[p] as int) < 256);
    assert(0 <= b[p + 1] as int);
    assert((b[p + 1] as int) < 256);
    assert(le16(b, p) < 65536) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            0 <= b[p] as int,
            (b[p] as int) < 256,
            0 <= b[p + 1] as int,
            (b[p + 1] as int) < 256,
    ;
    assert(le16(b, p) >= 0) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            0 <= b[p] as int,
            0 <= b[p + 1] as int,
    ;
}

/// `le32(b,p) < 2^32` whenever all four bytes exist.
pub proof fn lemma_le32_lt(b: Seq<u8>, p: int)
    requires
        p + 4 <= b.len(),
    ensures
        0 <= le32(b, p) < 4294967296,
{
    assert(0 <= b[p] as int);
    assert((b[p] as int) < 256);
    assert(0 <= b[p + 1] as int);
    assert((b[p + 1] as int) < 256);
    assert(0 <= b[p + 2] as int);
    assert((b[p + 2] as int) < 256);
    assert(0 <= b[p + 3] as int);
    assert((b[p + 3] as int) < 256);
    assert(le32(b, p) < 4294967296) by (nonlinear_arith)
        requires
            le32(b, p) == b[p] as int + 256 * (b[p + 1] as int) + 65536 * (b[p + 2] as int)
                + 16777216 * (b[p + 3] as int),
            0 <= b[p] as int,
            (b[p] as int) < 256,
            0 <= b[p + 1] as int,
            (b[p + 1] as int) < 256,
            0 <= b[p + 2] as int,
            (b[p + 2] as int) < 256,
            0 <= b[p + 3] as int,
            (b[p + 3] as int) < 256,
    ;
    assert(le32(b, p) >= 0) by (nonlinear_arith)
        requires
            le32(b, p) == b[p] as int + 256 * (b[p + 1] as int) + 65536 * (b[p + 2] as int)
                + 16777216 * (b[p + 3] as int),
            0 <= b[p] as int,
            0 <= b[p + 1] as int,
            0 <= b[p + 2] as int,
            0 <= b[p + 3] as int,
    ;
}

/// T7: `dir_end` bound. For every admissible count the table is at most 16,390
/// bytes, on 32- and 64-bit `usize` alike (the value is far below both maxima).
pub proof fn lemma_dir_end_bound(count: int)
    requires
        0 <= count <= 1024,
    ensures
        dir_header_len() + entry_record_len() * count <= max_directory_len(),
{
    assert(dir_header_len() + entry_record_len() * count <= max_directory_len()) by (nonlinear_arith)
        requires
            dir_header_len() == 6,
            entry_record_len() == 16,
            max_directory_len() == 16390,
            0 <= count <= 1024,
    ;
}

/// T7: the whole-table end and the widest wire range both fit `u32`, hence fit
/// `usize` on either target. The input cap already forces a 32-bit-safe slice
/// length, so a comparison in `u64` never wraps on 32- or 64-bit `usize`.
pub proof fn lemma_u32_width(b: Seq<u8>)
    requires
        b.len() <= max_input(),
    ensures
        max_directory_len() + 16 <= 4294967296,
        b.len() + 4294967296 <= 8589934592,
        max_input() <= 4294967296,
{
    assert(max_directory_len() + 16 <= 4294967296) by (nonlinear_arith)
        requires
            max_directory_len() == 16390,
    ;
    assert(b.len() + 4294967296 <= 8589934592) by (nonlinear_arith)
        requires
            b.len() <= max_input(),
            max_input() == 67108864,
    ;
    assert(max_input() <= 4294967296) by (nonlinear_arith)
        requires
            max_input() == 67108864,
    ;
}

/// `nom_dim` is always in `[1, 256]`, so the narrowed `u16` is exact.
pub proof fn lemma_nom_dim(w0: int)
    requires
        0 <= w0 < 256,
    ensures
        1 <= nom_dim(w0) <= 256,
{
    reveal(nom_dim);
    if w0 == 0 {
    } else {
    }
}


// ---------------------------------------------------------------------------
// R1/R8: record equality, field for field and status for status.
// ---------------------------------------------------------------------------

/// R6: `Png` and `Dib` as distinct codes.
pub open spec fn kind_code(k: EntryKind) -> u8 {
    match k {
        EntryKind::Png => 0u8,
        EntryKind::Dib => 1u8,
    }
}

/// R4: the four defect names as distinct codes.
pub open spec fn defect_code(d: EntryDefect) -> u8 {
    match d {
        EntryDefect::InvalidEntryReserved => 0u8,
        EntryDefect::EmptyEntry => 1u8,
        EntryDefect::EntryOverlapsDirectory => 2u8,
        EntryDefect::EntryOutOfBounds => 3u8,
    }
}

/// R1: `Option<u16>` equality.
pub open spec fn opt_u16_eq(a: Option<u16>, b: Option<u16>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

/// R1/R8: status equality, kind for kind and payload pair for payload pair.
pub open spec fn status_eq(a: EntryStatus, b: EntryStatus) -> bool {
    match a {
        EntryStatus::Invalid(d1) => match b {
            EntryStatus::Invalid(d2) => defect_code(d1) == defect_code(d2),
            _ => false,
        },
        EntryStatus::Valid { kind: k1, payload: p1 } => match b {
            EntryStatus::Valid { kind: k2, payload: p2 } => kind_code(k1) == kind_code(k2)
                && p1[0] == p2[0] && p1[1] == p2[1],
            _ => false,
        },
    }
}

/// R8: the exact field-for-field, status-for-status equality of a supplied
/// record and its wire reconstruction — every R1 field, nothing else.
pub open spec fn entry_eq(a: Entry, b: Entry) -> bool {
    &&& a.width == b.width
    &&& a.height == b.height
    &&& a.color_count == b.color_count
    &&& a.raw_offset == b.raw_offset
    &&& a.raw_size == b.raw_size
    &&& opt_u16_eq(a.planes, b.planes)
    &&& opt_u16_eq(a.bit_count, b.bit_count)
    &&& opt_u16_eq(a.hotspot_x, b.hotspot_x)
    &&& opt_u16_eq(a.hotspot_y, b.hotspot_y)
    &&& status_eq(a.status, b.status)
}

// ---------------------------------------------------------------------------
// R9: the entry-defect -> public-error mapping used by decode_entry.
// ---------------------------------------------------------------------------

/// R9: the exact `Error` a selected defect returns.
pub open spec fn error_of_defect(d: EntryDefect) -> Error {
    match d {
        EntryDefect::InvalidEntryReserved => Error::InvalidEntryReserved,
        EntryDefect::EmptyEntry => Error::EmptyEntry,
        EntryDefect::EntryOverlapsDirectory => Error::EntryOverlapsDirectory,
        EntryDefect::EntryOutOfBounds => Error::EntryOutOfBounds,
    }
}

// ---------------------------------------------------------------------------
// R8/R10: the ICO-side decision of decode_entry (the M2 part of MD).
// ---------------------------------------------------------------------------

/// The ICO-controlled decision `decode_entry` takes, in R10 order. The DIB case
/// names the validated declared slice `[start, end)`; its pixel/error content is
/// the DIB core's own contract and the M3 theorem T5 that names `MD`.
pub enum IcoDecision {
    /// R2/R3 directory-wide error.
    DirErr(Error),
    /// `index >= count`.
    IndexOOR,
    /// `entries.len() != count` or the selected record is not `ME`.
    BadList,
    /// The selected record's first defect.
    Defect(EntryDefect),
    /// The selected valid record is `Png`.
    Png,
    /// The selected valid record is `Dib`; decode the declared slice.
    DibSlice(int, int),
}

/// `decode_entry`'s ICO-side decision, exactly in R8/R10 order: directory
/// guards, index before list length, list length before selected equality,
/// selected equality before the selected status, PNG before DIB.
pub open spec fn md_ico(b: Seq<u8>, es: Seq<Entry>, index: u32) -> IcoDecision {
    match h(b) {
        Err(e) => IcoDecision::DirErr(e),
        Ok(hr) => {
            if index as int >= hr.count {
                IcoDecision::IndexOOR
            } else if es.len() != hr.count {
                IcoDecision::BadList
            } else if !entry_eq(es[index as int], me(b, hr, index as int)) {
                IcoDecision::BadList
            } else {
                match me(b, hr, index as int).status {
                    EntryStatus::Invalid(d) => IcoDecision::Defect(d),
                    EntryStatus::Valid { kind: EntryKind::Png, .. } => IcoDecision::Png,
                    EntryStatus::Valid { kind: EntryKind::Dib, payload } => IcoDecision::DibSlice(
                        payload[0] as int,
                        payload[1] as int,
                    ),
                }
            }
        }
    }
}

/// T1/T5 (ICO side): the M2 postcondition of `decode_entry`. Every ICO-controlled
/// outcome is fixed exactly; on the selected-DIB branch the result is a DIB
/// success or a DIB error and nothing else. M3 strengthens this branch to the
/// exact `MD` value, including the composition with the DIB model.
pub open spec fn decode_pre(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>)
    -> bool {
    match md_ico(b, es, index) {
        IcoDecision::DirErr(e) => r == Err(e),
        IcoDecision::IndexOOR => r == Err(Error::IndexOutOfRange),
        IcoDecision::BadList => r == Err(Error::InvalidEntryRecord),
        IcoDecision::Defect(d) => r == Err(error_of_defect(d)),
        IcoDecision::Png => r == Err(Error::PngDelegation),
        IcoDecision::DibSlice(_start, _end) => match r {
            Ok(_) => true,
            Err(Error::Dib(_)) => true,
            Err(_) => false,
        },
    }
}

// ---------------------------------------------------------------------------
// T3/T4 bridges from the parse result, and the small lemmas they need.
// ---------------------------------------------------------------------------

/// T3 (record validity), stated on the model: if `parse(b) = Ok(E)` then
/// `|E| = count` with `count` in `[1, 1024]`, every record is its `ME`, and each
/// valid record's payload lies in `[dir_end, |b|]` inside the 64 MiB cap. The
/// bridge from `parse_eq` to this predicate is `lemma_parse_ok_well_formed`.
pub proof fn lemma_parse_ok_well_formed(b: Seq<u8>, es: Seq<Entry>)
    requires
        match mc(b) {
            Ok(ms) => es == ms,
            Err(_) => false,
        },
    ensures
        entries_well_formed(b, es),
{
    reveal(mc);
    reveal(h);
    reveal(entries_well_formed);
    reveal(entry_range_ok);
    reveal(defect_of);
    if !(b.len() > max_input()) && !(b.len() < dir_header_len()) && le16(b, 0) == 0 && (le16(b, 2)
        == 1 || le16(b, 2) == 2) && le16(b, 4) != 0 && le16(b, 4) <= max_entries() && dir_header_len()
        + entry_record_len() * le16(b, 4) <= b.len() {
        let hr = HRec { typ: le16(b, 2), count: le16(b, 4), dir_end: 6 + 16 * le16(b, 4) };
        assert(h(b) == Ok(hr));
        assert(es == Seq::new(hr.count as nat, |i: int| me(b, hr, i)));
        assert(1 <= hr.count <= max_entries());
        assert(es.len() == hr.count);
        assert forall|j: int| 0 <= j < es.len() implies #[trigger] es[j] == me(b, hr, j) by {
            lemma_seq_new_index(hr.count as nat, |i: int| me(b, hr, i), j);
        }
        assert forall|j: int| 0 <= j < es.len() implies #[trigger] entry_range_ok(b, hr, es[j]) by {
            let e = es[j];
            assert(e == me(b, hr, j));
            let p = 6 + 16 * j;
            assert(0 <= j < hr.count);
            assert(p + 16 <= b.len()) by (nonlinear_arith)
                requires
                    0 <= j,
                    j < hr.count,
                    hr.dir_end == 6 + 16 * hr.count,
                    hr.dir_end <= b.len(),
                    p == 6 + 16 * j,
            ;
            lemma_le32_lt(b, p + 8);
            lemma_le32_lt(b, p + 12);
            reveal(me);
            reveal(nom_dim);
            if defect_of(b, hr.dir_end, b8(b, p + 3), le32(b, p + 8), le32(b, p + 12)) is None {
                assert(es[j].status is Valid);
                assert(hr.dir_end <= le32(b, p + 12));
                assert(le32(b, p + 12) > 0);
                assert(le32(b, p + 12) <= b.len());
                assert(le32(b, p + 8) <= b.len() - le32(b, p + 12));
                assert(le32(b, p + 12) + le32(b, p + 8) <= b.len()) by (nonlinear_arith)
                    requires
                        le32(b, p + 8) <= b.len() - le32(b, p + 12),
                ;
            }
        }
    }
}

/// T4 (signature classification) on the model: every record that `parse` returns
/// classifies by the eight signature bytes inside its own declared range.
pub proof fn lemma_parse_ok_classified(b: Seq<u8>, es: Seq<Entry>)
    requires
        match mc(b) {
            Ok(ms) => es == ms,
            Err(_) => false,
        },
    ensures
        classification_ok(b, es),
{
    lemma_parse_ok_well_formed(b, es);
    reveal(mc);
    reveal(h);
    reveal(classification_ok);
    reveal(entry_class_ok);
    reveal(entries_well_formed);
    let hr = match h(b) {
        Ok(hr) => hr,
        Err(_) => return,
    };
    assert(es == Seq::new(hr.count as nat, |i: int| me(b, hr, i)));
    assert forall|j: int| 0 <= j < es.len() implies #[trigger] entry_class_ok(b, es[j]) by {
        let p = 6 + 16 * j;
        assert(es[j] == me(b, hr, j));
        reveal(me);
        reveal(nom_dim);
        reveal(kind_of);
        reveal(defect_of);
        lemma_le32_lt(b, p + 8);
        lemma_le32_lt(b, p + 12);
        assert(0 <= j < hr.count);
        assert(p + 16 <= b.len()) by (nonlinear_arith)
            requires
                0 <= j,
                j < hr.count,
                hr.dir_end == 6 + 16 * hr.count,
                hr.dir_end <= b.len(),
                p == 6 + 16 * j,
        ;
        if defect_of(b, hr.dir_end, b8(b, p + 3), le32(b, p + 8), le32(b, p + 12)) is None {
            assert(me(b, hr, j).status is Valid);
            assert(me(b, hr, j).raw_offset == le32(b, p + 12) as u32);
            assert(me(b, hr, j).raw_size == le32(b, p + 8) as u32);
        }
    }
}

/// The valid-entry payload lies inside the file, so `decode_entry`'s declared
/// slice `[start, end)` is obtainable from `bytes`.
pub proof fn lemma_valid_payload_in_bounds(b: Seq<u8>, hr: HRec, i: int)
    requires
        h(b) == Ok(hr),
        0 <= i < hr.count,
        me(b, hr, i).status is Valid,
    ensures
        ({
            let st = me(b, hr, i);
            match st.status {
                EntryStatus::Valid { payload, .. } => (payload[0] as int) <= (payload[1] as int)
                    && (payload[1] as int) <= b.len(),
                _ => false,
            }
        }),
{
    reveal(h);
    reveal(me);
    reveal(defect_of);
    let p = 6 + 16 * i;
    assert(p + 16 <= b.len()) by (nonlinear_arith)
        requires
            0 <= i,
            i < hr.count,
            hr.dir_end == 6 + 16 * hr.count,
            hr.dir_end <= b.len(),
            p == 6 + 16 * i,
    ;
    lemma_le32_lt(b, p + 8);
    lemma_le32_lt(b, p + 12);
    assert(me(b, hr, i).status is Valid);
}

/// T2's hypothesis gives the model list: `parse_eq(b, Ok(v))` fixes `MC(b)`.
pub proof fn lemma_parse_eq_ok(b: Seq<u8>, v: Vec<Entry>)
    requires
        parse_eq(b, Ok(v)),
    ensures
        mc(b) == Ok(v@),
{
    reveal(parse_eq);
    reveal(mc);
    match mc(b) {
        Ok(ms) => {
            assert(ms == v@);
        },
        Err(e) => {},
    }
}

/// T2's hypothesis on the `Err` branch: `parse_eq(b, Err(e))` fixes `MC(b)`.
pub proof fn lemma_parse_eq_err(b: Seq<u8>, e: Error)
    requires
        parse_eq(b, Err(e)),
    ensures
        mc(b) == Err(e),
{
    reveal(parse_eq);
    reveal(mc);
    match mc(b) {
        Ok(_) => {},
        Err(e2) => {
            assert(e2 == e);
        },
    }
}

/// `MC(b)` is `Err(e)` exactly when `H(b)` is.
pub proof fn lemma_mc_err(b: Seq<u8>, e: Error)
    requires
        h(b) == Err(e),
    ensures
        mc(b) == Err(e),
{
    reveal(mc);
}

/// `MC(b)` is `Ok` with the `ME` list exactly when `H(b)` is `Ok`.
pub proof fn lemma_mc_ok(b: Seq<u8>, hr: HRec)
    requires
        h(b) == Ok(hr),
    ensures
        mc(b) == Ok(Seq::new(hr.count as nat, |i: int| me(b, hr, i))),
{
    reveal(mc);
}

/// T7 on the parse result: every valid record's range is a `u32` pair inside the
/// file and inside the 64 MiB cap. Stated for one record, for reuse in M3.
pub proof fn lemma_me_range_u32(b: Seq<u8>, hr: HRec, i: int)
    requires
        h(b) == Ok(hr),
        0 <= i < hr.count,
        me(b, hr, i).status is Valid,
    ensures
        (me(b, hr, i).raw_offset as int) <= 4294967295,
        (me(b, hr, i).raw_offset as int) + (me(b, hr, i).raw_size as int) <= b.len(),
{
    lemma_valid_payload_in_bounds(b, hr, i);
    reveal(me);
}


/// `Option<u16>` equality is reflexive.
pub proof fn lemma_opt_u16_eq_refl(a: Option<u16>)
    ensures
        opt_u16_eq(a, a),
{
    reveal(opt_u16_eq);
}

/// `status_eq` is reflexive.
pub proof fn lemma_status_eq_refl(a: EntryStatus)
    ensures
        status_eq(a, a),
{
    reveal(status_eq);
    reveal(kind_code);
    reveal(defect_code);
}

/// `status_eq(a,b)` gives `a == b` for the concrete `EntryStatus`.
pub proof fn lemma_status_eq_implies_eq(a: EntryStatus, b: EntryStatus)
    requires
        status_eq(a, b),
    ensures
        a == b,
{
    reveal(status_eq);
    reveal(kind_code);
    reveal(defect_code);
    match a {
        EntryStatus::Invalid(d1) => match b {
            EntryStatus::Invalid(d2) => {
                assert(defect_code(d1) == defect_code(d2));
                assert(d1 == d2) by {
                    if d1 == EntryDefect::InvalidEntryReserved {
                        assert(d2 == EntryDefect::InvalidEntryReserved);
                    } else if d1 == EntryDefect::EmptyEntry {
                        assert(d2 == EntryDefect::EmptyEntry);
                    } else if d1 == EntryDefect::EntryOverlapsDirectory {
                        assert(d2 == EntryDefect::EntryOverlapsDirectory);
                    } else {
                        assert(d2 == EntryDefect::EntryOutOfBounds);
                    }
                }
            },
            _ => {},
        },
        EntryStatus::Valid { kind: k1, payload: p1 } => match b {
            EntryStatus::Valid { kind: k2, payload: p2 } => {
                assert(kind_code(k1) == kind_code(k2));
                assert(k1 == k2) by {
                    if k1 == EntryKind::Png {
                        assert(k2 == EntryKind::Png);
                    } else {
                        assert(k2 == EntryKind::Dib);
                    }
                }
                assert(p1[0] == p2[0]);
                assert(p1[1] == p2[1]);
                assert(p1 == p2);
            },
            _ => {},
        },
    }
}

/// `a == b` gives `status_eq(a,b)`.
pub proof fn lemma_status_eq_of_eq(a: EntryStatus, b: EntryStatus)
    requires
        a == b,
    ensures
        status_eq(a, b),
{
    reveal(status_eq);
    reveal(kind_code);
    reveal(defect_code);
    assert(a == b);
}

/// T3/T4/R8: `entry_eq` is exactly the concrete record equality.
pub proof fn lemma_entry_eq_iff(a: Entry, b: Entry)
    ensures
        entry_eq(a, b) <==> a == b,
{
    if entry_eq(a, b) {
        reveal(entry_eq);
        lemma_opt_u16_eq_refl(a.planes);
        lemma_opt_u16_eq_refl(a.bit_count);
        lemma_opt_u16_eq_refl(a.hotspot_x);
        lemma_opt_u16_eq_refl(a.hotspot_y);
        assert(opt_u16_eq(a.planes, b.planes));
        reveal(opt_u16_eq);
        assert(opt_u16_eq(a.planes, b.planes));
        assert(a.planes == b.planes);
        assert(a.bit_count == b.bit_count);
        assert(a.hotspot_x == b.hotspot_x);
        assert(a.hotspot_y == b.hotspot_y);
        lemma_status_eq_implies_eq(a.status, b.status);
        assert(a.status == b.status);
        assert(a == b);
    }
    if a == b {
        lemma_eq_implies_entry_eq(a, b);
    }
}

/// `a == b` gives `entry_eq(a,b)`.
pub proof fn lemma_eq_implies_entry_eq(a: Entry, b: Entry)
    requires
        a == b,
    ensures
        entry_eq(a, b),
{
    reveal(entry_eq);
    reveal(opt_u16_eq);
    lemma_status_eq_of_eq(a.status, b.status);
    assert(a == b);
}

// ---------------------------------------------------------------------------
// R11/R8: the total decode model MD, and the composition statement.
// ---------------------------------------------------------------------------

/// R8: the exact `Error` the ICO core returns when the selected DIB call fails:
/// the DIB crate's error, wrapped as R9's typed `Dib(e)`.
pub open spec fn dib_wrap(r: Result<Decoded, dib_verus::Error>) -> Result<Decoded, Error> {
    match r {
        Ok(d) => Ok(d),
        Err(e) => Err(Error::Dib(e)),
    }
}

/// R8/R11: `MD(b,E,i)` — R8 in R10 order, total for every typed `E` and every
/// `u32` index. The ICO-controlled branches are `md_ico`; the selected valid DIB
/// branch calls the **mathematical DIB 1.1** `MP_1.1(b.subrange(start,end),
/// IcoEntryDibSlack)` and maps its exact result: a success `(w,h,rgba)` to the
/// public `Decoded`, and every DIB error to the typed `Dib(e)`. The result is
/// the manifest's `Result<Decoded, Error>` re-encoded as the tuple `(w,h,rgba)`
/// on success, so the statement names every decoded dimension and RGBA byte.
pub open spec fn md(b: Seq<u8>, es: Seq<Entry>, index: u32) -> Result<
    (int, int, Seq<u8>),
    Error,
> {
    match md_ico(b, es, index) {
        IcoDecision::DirErr(e) => Err(e),
        IcoDecision::IndexOOR => Err(Error::IndexOutOfRange),
        IcoDecision::BadList => Err(Error::InvalidEntryRecord),
        IcoDecision::Defect(d) => Err(error_of_defect(d)),
        IcoDecision::Png => Err(Error::PngDelegation),
        IcoDecision::DibSlice(start, end) => match dib_verus::model::mp(
            b.subrange(start, end),
            Mode::IcoEntryDibSlack,
        ) {
            Ok(t) => Ok(t),
            Err(e) => Err(Error::Dib(e)),
        },
    }
}

/// T5: does the decoded image `d` equal the model tuple `t = (w,h,rgba)` field for
/// field, including every RGBA byte? `Decoded`'s `Vec<u8>` payload is viewed as
/// the `Seq<u8>` `d.rgba@`.
pub open spec fn decoded_matches(d: Decoded, t: (int, int, Seq<u8>)) -> bool {
    d.width as int == t.0 && d.height as int == t.1 && d.rgba@ == t.2
}

/// T5: the exact statement — `decode_entry(b,E,i) = MD(b,E,i)` as the complete
/// `Result`, on every byte slice, every typed record list and every `u32` index,
/// with the success carrying every decoded dimension and RGBA byte.
pub open spec fn md_eq(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>) -> bool {
    match md(b, es, index) {
        Ok(t) => match r {
            Ok(d) => decoded_matches(d, t),
            Err(_) => false,
        },
        Err(e) => match r {
            Ok(_) => false,
            Err(e2) => e2 == e,
        },
    }
}

/// T5 (composition): if the ICO-side decision is the selected DIB declared slice,
/// the DIB public contract holds on that exact slice, and `r0` is the DIB call's
/// raw result, then the wrapped result satisfies `md_eq` — the DIB model's exact
/// `Ok(w,h,rgba)`/`Err(e)` mapped into `MD`'s type. This is the discharge of the
/// composition statement from `decode_mp_eq`, stated on the same slice.
pub proof fn lemma_md_eq_dib(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r0: Result<Decoded, dib_verus::Error>,
    start: int,
    end: int,
    slice: Seq<u8>,
)
    requires
        md_ico(b, es, index) == IcoDecision::DibSlice(start, end),
        slice == b.subrange(start, end),
        dib_verus::dib::decode_mp_eq(r0, slice, Mode::IcoEntryDibSlack),
    ensures
        md_eq(b, es, index, dib_wrap(r0)),
{
    reveal(md_eq);
    reveal(md);
    reveal(dib_wrap);
    reveal(dib_verus::dib::decode_mp_eq);
    assert(md(b, es, index) == match dib_verus::model::mp(slice, Mode::IcoEntryDibSlack) {
        Ok(t) => Ok(t),
        Err(e) => Err(Error::Dib(e)),
    });
    match dib_verus::model::mp(slice, Mode::IcoEntryDibSlack) {
        Ok(t) => {
            match r0 {
                Ok(d) => {
                    assert(t == (d.width as int, d.height as int, d.rgba@));
                    reveal(decoded_matches);
                },
                Err(_) => {},
            }
        },
        Err(e) => {
            match r0 {
                Ok(_) => {},
                Err(e2) => {
                    assert(e2 == e);
                },
            }
        },
    }
}

/// T5: on the `Err` branch, if `MD(b,E,i) = Err(e)` then the executable's
/// `Err(e)` satisfies `md_eq`. Used by `decode_entry` for its four ICO-controlled
/// error branches (directory guard, index, list/record mismatch, defect, PNG).
pub proof fn lemma_md_eq_err(b: Seq<u8>, es: Seq<Entry>, index: u32, e: Error)
    requires
        md(b, es, index) == Err(e),
    ensures
        md_eq(b, es, index, Err(e)),
{
    reveal(md_eq);
}

/// T11: the converse direction used by the concrete witness bridge — if
/// `md_eq(b,E,i,r)` holds and `MD(b,E,i) = Err(e)`, then the executable result
/// is exactly `Err(e)`.
pub proof fn lemma_md_eq_to_err(b: Seq<u8>, es: Seq<Entry>, index: u32, e: Error, r: Result<
    Decoded,
    Error,
>)
    requires
        md(b, es, index) == Err(e),
        md_eq(b, es, index, r),
    ensures
        r == Err(e),
{
    reveal(md_eq);
}

/// T11: if `md_eq(b,E,i,r)` holds and `MD(b,E,i) = Ok(t)`, then the executable
/// result is a success whose dimensions and RGBA bytes are exactly `t`.
pub proof fn lemma_md_eq_to_ok(b: Seq<u8>, es: Seq<Entry>, index: u32, t: (int, int, Seq<u8>), r:
    Result<Decoded, Error>)
    requires
        md(b, es, index) == Ok(t),
        md_eq(b, es, index, r),
    ensures
        r is Ok,
        match r {
            Ok(d) => decoded_matches(d, t),
            Err(_) => false,
        },
{
    reveal(md_eq);
}

/// T6 (exact precedence): the full priority chain of R10, as a predicate on the
/// public `decode_entry` result `r`. Naming the chain once keeps the statement in
/// one place: R2/R3 first on both calls; then decode's index check before the
/// list check; the list/selected mismatch before the selected defect; the
/// selected defect before PNG/DIB; PNG before any DIB error. The selected DIB's
/// own R14 internal precedence is delegated unchanged to the DIB call.
pub open spec fn t6_full_prec(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>)
    -> bool {
    match h(b) {
        Err(e) => r == Err(e),
        Ok(hr) => {
            &&& (hr.count <= index as int ==> r == Err(Error::IndexOutOfRange))
            &&& ((index as int) < hr.count && es.len() != hr.count ==> r == Err(
                Error::InvalidEntryRecord,
            ))
            &&& ((index as int) < hr.count && es.len() == hr.count && !entry_eq(
                es[index as int],
                me(b, hr, index as int),
            ) ==> r == Err(Error::InvalidEntryRecord))
            &&& ((index as int) < hr.count && es.len() == hr.count && entry_eq(
                es[index as int],
                me(b, hr, index as int),
            ) ==> match me(b, hr, index as int).status {
                EntryStatus::Invalid(d) => r == Err(error_of_defect(d)),
                EntryStatus::Valid { kind: EntryKind::Png, .. } => r == Err(Error::PngDelegation),
                EntryStatus::Valid { kind: EntryKind::Dib, .. } => match r {
                    Ok(_) => true,
                    Err(Error::Dib(_)) => true,
                    Err(_) => false,
                },
            })
        },
    }
}

/// T6: `md_eq` gives the full precedence chain `t6_full_prec`.
pub proof fn lemma_t6_full_prec(b: Seq<u8>, es: Seq<Entry>, index: u32, r: Result<Decoded, Error>)
    requires
        md_eq(b, es, index, r),
    ensures
        t6_full_prec(b, es, index, r),
{
    reveal(md_eq);
    reveal(md);
    reveal(md_ico);
    reveal(t6_full_prec);
    match h(b) {
        Err(e) => {
            assert(md(b, es, index) == Err(e));
        },
        Ok(hr) => {
            if hr.count <= index as int {
                assert(md_ico(b, es, index) == IcoDecision::IndexOOR);
                assert(md(b, es, index) == Err(Error::IndexOutOfRange));
            } else if es.len() != hr.count {
                assert(md_ico(b, es, index) == IcoDecision::BadList);
                assert(md(b, es, index) == Err(Error::InvalidEntryRecord));
            } else if !entry_eq(es[index as int], me(b, hr, index as int)) {
                assert(md_ico(b, es, index) == IcoDecision::BadList);
                assert(md(b, es, index) == Err(Error::InvalidEntryRecord));
            } else {
                match me(b, hr, index as int).status {
                    EntryStatus::Invalid(d) => {
                        assert(md_ico(b, es, index) == IcoDecision::Defect(d));
                        assert(md(b, es, index) == Err(error_of_defect(d)));
                    },
                    EntryStatus::Valid { kind: EntryKind::Png, .. } => {
                        assert(md_ico(b, es, index) == IcoDecision::Png);
                        assert(md(b, es, index) == Err(Error::PngDelegation));
                    },
                    EntryStatus::Valid { kind: EntryKind::Dib, payload } => {
                        assert(md_ico(b, es, index) == IcoDecision::DibSlice(
                            payload[0] as int,
                            payload[1] as int,
                        ));
                        match dib_verus::model::mp(
                            b.subrange(payload[0] as int, payload[1] as int),
                            Mode::IcoEntryDibSlack,
                        ) {
                            Ok(t) => {
                                match r {
                                    Ok(_) => {},
                                    Err(_) => {},
                                }
                            },
                            Err(e) => {
                                match r {
                                    Ok(_) => {},
                                    Err(e2) => {
                                        assert(e2 == Error::Dib(e));
                                    },
                                }
                            },
                        }
                    },
                }
            }
        },
    }
}

/// T1/T5: `md_eq` implies the M2 postcondition `decode_pre` (the ICO-side
/// decision and the DIB-result shape), so the M2 T1 witness keeps its exact
/// hypothesis while the public postcondition is the stronger `md_eq`.
pub proof fn lemma_md_eq_implies_decode_pre(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, Error>,
)
    requires
        md_eq(b, es, index, r),
    ensures
        decode_pre(b, es, index, r),
{
    reveal(md_eq);
    reveal(md);
    reveal(decode_pre);
    reveal(md_ico);
    match md_ico(b, es, index) {
        IcoDecision::DirErr(e) => {},
        IcoDecision::IndexOOR => {},
        IcoDecision::BadList => {},
        IcoDecision::Defect(d) => {},
        IcoDecision::Png => {},
        IcoDecision::DibSlice(start, end) => {
            match dib_verus::model::mp(b.subrange(start, end), Mode::IcoEntryDibSlack) {
                Ok(t) => {
                    match r {
                        Ok(_) => {},
                        Err(_) => {},
                    }
                },
                Err(e) => {
                    match r {
                        Ok(_) => {},
                        Err(e2) => {
                            assert(e2 == Error::Dib(e));
                        },
                    }
                },
            }
        },
    }
}

/// T7 (the clause M2 deferred): a successful `decode_entry` obeys the DIB
/// dependency's own output contract — the `Decoded` has exactly `4*w*h` RGBA
/// bytes with `4*w*h <= max_output()` (67,108,864), and its dimensions are within
/// `1..=8192`. Discharged from `md_eq` through the DIB model's public
/// `lemma_mp_ok_shape` on the same validated slice.
pub proof fn lemma_t7_dib_output_bound(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, Error>,
)
    requires
        md_eq(b, es, index, r),
    ensures
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
    reveal(md_eq);
    reveal(md);
    reveal(md_ico);
    reveal(decoded_matches);
    match r {
        Ok(d) => {
            assert(md(b, es, index) is Ok);
            let t = match md(b, es, index) {
                Ok(t) => t,
                Err(_) => (0, 0, Seq::empty()),
            };
            assert(decoded_matches(d, t));
            assert(d.width as int == t.0);
            assert(d.height as int == t.1);
            assert(d.rgba@ == t.2);
            // `md` is `Ok` only through the DIB branch, whose value is
            // `mp(subrange, Slack)`.
            assert(md_ico(b, es, index) is DibSlice);
            let (start, end) = match md_ico(b, es, index) {
                IcoDecision::DibSlice(s, e) => (s, e),
                _ => (0, 0),
            };
            assert(dib_verus::model::mp(b.subrange(start, end), Mode::IcoEntryDibSlack) == Ok(t));
            dib_verus::model::lemma_mp_ok_shape(
                b.subrange(start, end),
                Mode::IcoEntryDibSlack,
                t.0,
                t.1,
                t.2,
            );
        },
        Err(_) => {},
    }
}

// ---------------------------------------------------------------------------
// T6: exact precedence, as properties of the model.
// ---------------------------------------------------------------------------

/// T6: the R2/R3 directory guard has priority on **both** public calls — an `H`
/// error is `MC`'s error and `decode_entry`'s directory error, before any record,
/// index or list is examined.
pub proof fn lemma_t6_directory_priority(b: Seq<u8>, es: Seq<Entry>, index: u32, e: Error)
    requires
        h(b) == Err(e),
    ensures
        mc(b) == Err(e),
        md_ico(b, es, index) == IcoDecision::DirErr(e),
        md(b, es, index) == Err(e),
{
    reveal(mc);
    reveal(md_ico);
    reveal(md);
}

/// T6: decode checks the index against `count` before the supplied list length.
/// Whatever the list is, an out-of-range index is `IndexOutOfRange`.
pub proof fn lemma_t6_index_before_list(b: Seq<u8>, hr: HRec, es: Seq<Entry>, index: u32)
    requires
        h(b) == Ok(hr),
        hr.count <= index as int,
    ensures
        md_ico(b, es, index) == IcoDecision::IndexOOR,
{
    reveal(md_ico);
}

/// T6: the list-length check precedes the selected-record comparison and the
/// selected defect. A list whose length differs from `count` is
/// `InvalidEntryRecord` no matter what its elements are.
pub proof fn lemma_t6_list_before_record(b: Seq<u8>, hr: HRec, es: Seq<Entry>, index: u32)
    requires
        h(b) == Ok(hr),
        (index as int) < hr.count,
        es.len() != hr.count,
    ensures
        md_ico(b, es, index) == IcoDecision::BadList,
{
    reveal(md_ico);
}

/// T6: the selected-record comparison precedes the selected defect, PNG
/// delegation and the DIB call. A selected record that is not exactly `ME` is
/// `InvalidEntryRecord`, whatever its status.
pub proof fn lemma_t6_record_before_defect(b: Seq<u8>, hr: HRec, es: Seq<Entry>, index: u32)
    requires
        h(b) == Ok(hr),
        (index as int) < hr.count,
        es.len() == hr.count,
        !entry_eq(es[index as int], me(b, hr, index as int)),
    ensures
        md_ico(b, es, index) == IcoDecision::BadList,
{
    reveal(md_ico);
}

/// T6: with the guards passed and the selected record equal, a selected defect
/// wins over PNG classification and the DIB call: the decision names the defect.
pub proof fn lemma_t6_defect_before_png_dib(b: Seq<u8>, hr: HRec, es: Seq<Entry>, index: u32)
    requires
        h(b) == Ok(hr),
        (index as int) < hr.count,
        es.len() == hr.count,
        entry_eq(es[index as int], me(b, hr, index as int)),
    ensures
        match me(b, hr, index as int).status {
            EntryStatus::Invalid(d) => md_ico(b, es, index) == IcoDecision::Defect(d),
            EntryStatus::Valid { kind: EntryKind::Png, .. } => md_ico(b, es, index)
                == IcoDecision::Png,
            EntryStatus::Valid { kind: EntryKind::Dib, payload } => md_ico(b, es, index)
                == IcoDecision::DibSlice(payload[0] as int, payload[1] as int),
        },
{
    reveal(md_ico);
}

/// T6: `decode_entry` inspects **only** `entries.len()` and `entries[index]`. Two
/// lists of the same length whose selected element agrees produce the same
/// decision, so a bogus *unselected* supplied or wire record cannot change the
/// selected result.
pub proof fn lemma_t6_decode_reads_only_selected(
    b: Seq<u8>,
    es1: Seq<Entry>,
    es2: Seq<Entry>,
    index: u32,
)
    requires
        es1.len() == es2.len(),
        (index as int) < es1.len() ==> es1[index as int] == es2[index as int],
    ensures
        md_ico(b, es1, index) == md_ico(b, es2, index),
{
    reveal(md_ico);
    match h(b) {
        Err(e) => {
            assert(md_ico(b, es1, index) == IcoDecision::DirErr(e));
            assert(md_ico(b, es2, index) == IcoDecision::DirErr(e));
        },
        Ok(hr) => {
            if index as int >= hr.count {
            } else if es1.len() != hr.count {
                assert(es2.len() != hr.count);
            } else {
                assert((index as int) < es1.len());
                assert(es1[index as int] == es2[index as int]);
                assert(entry_eq(es1[index as int], me(b, hr, index as int)) == entry_eq(
                    es2[index as int],
                    me(b, hr, index as int),
                ));
            }
        },
    }
}

/// T6: `parse` reports record defects as statuses, never as parse errors — a
/// valid directory with defective records still returns `Ok` with all `count`
/// records. (The defect itself is R4's, and `MC` fails only on `H`.)
pub proof fn lemma_t6_parse_defects_are_statuses(b: Seq<u8>, hr: HRec, j: int)
    requires
        h(b) == Ok(hr),
        0 <= j < hr.count,
    ensures
        mc(b) == Ok(Seq::new(hr.count as nat, |i: int| me(b, hr, i))),
{
    reveal(mc);
    reveal(h);
}

// ---------------------------------------------------------------------------
// T8: termination.
// ---------------------------------------------------------------------------

/// T8: the parse loop's variant. For each complete record the iteration index
/// `i` advances to `i+1` and `count - i` strictly decreases and stays positive,
/// whether the record was valid or invalid (`reconstruct` never fails). Also the
/// eight-byte signature comparison's variant `8 - k` is bounded and strictly
/// decreases. Stated for the parse loop and the signature loop as the manifest
/// quantifies them.
pub proof fn lemma_t8_variants(count: int, i: int, k: int)
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

/// T8: an input whose wire count exceeds `MAX_ENTRIES` is `TooManyEntries`, so
/// `parse` returns from the directory guard before the record loop is entered —
/// an input with a count of 65,535 never iterates a record. (The count cap is
/// checked before the whole-table guard, so it wins over a short table.)
pub proof fn lemma_t8_count_cap_before_loop(b: Seq<u8>)
    requires
        b.len() <= max_input(),
        b.len() >= dir_header_len(),
        le16(b, 0) == 0,
        le16(b, 2) == 1 || le16(b, 2) == 2,
        le16(b, 4) > max_entries(),
    ensures
        h(b) == Err(Error::TooManyEntries),
        mc(b) == Err(Error::TooManyEntries),
{
    reveal(h);
    reveal(mc);
}

/// T8: `parse` completes exactly `count` iterations — the model list `MC` has
/// length `count`, every record present with its status (valid or invalid). This
/// is the loop's own invariant `acc.len() == i` restated on the model, so it
/// captures that no record, valid or invalid, aborts the iteration. `decode_entry`
/// revalidates exactly one record and has no loop of its own; the one bounded loop
/// in the ICO decode path is the eight-byte signature test bounded by
/// `lemma_t8_variants`.
pub proof fn lemma_t8_parse_iterates_count(b: Seq<u8>, hr: HRec)
    requires
        h(b) == Ok(hr),
    ensures
        mc(b) == Ok(Seq::new(hr.count as nat, |i: int| me(b, hr, i))),
        match mc(b) {
            Ok(es) => es.len() == hr.count,
            Err(_) => false,
        },
{
    reveal(mc);
    reveal(h);
}

// ---------------------------------------------------------------------------
// M4a · R2/R13: the ICO-side linear-work model (T9)
// ---------------------------------------------------------------------------
//
// THEOREMS.md T9: attach a ghost step to **every ICO-controlled real loop
// iteration** — the directory loop, the record loop, the eight-byte signature
// comparison, and any library bulk fill — with `steps(parse) <= 16*(|b|+1)` and
// `steps(wrapper) <= 16*(|b|+1)` for every `b,E,i`. The wrapper's constant
// selected-record work counts at most eight signature-byte checks and never
// scans `E` or parses all records. On the shipped DIB 1.0 the manifest permits
// only the separately proved DIB T12 as external evidence; the DIB 1.1 crate
// this core depends on exports the tracked `decode_with_steps` whose bound
// `steps_DIB <= 16*(|slice|+N)` is a proved postcondition, so ICO **does**
// derive the combined bound from both counters (see `t9_combined_steps`).

/// R13: the parse path charges one step for the bounded entry-list
/// initialisation (`Vec::with_capacity`, the "list initialisation" the manifest
/// names) and, per record, one for the record body plus up to eight for the
/// record/signature interior loop — but only after the directory guard success.
/// The directory guard itself has no loop (its word reads are straight-line), so
/// a directory-wide failure charges nothing. `parse_step_bound` is the ceiling
/// the executable charges on a slice `b`: `0` on `H`'s error, else
/// `1 + 9*count`.
pub open spec fn parse_step_bound(b: Seq<u8>) -> int {
    match h(b) {
        Err(_) => 0,
        Ok(hr) => 1 + 9 * hr.count,
    }
}

/// R13/T9: the wrapper's own ICO-controlled steps — everything `decode_entry`
/// charges that is not the DIB call's `steps_DIB`. It is bounded by 16 for every
/// decision: the directory guard has no loop (zero), the selected-record
/// revalidation runs one record body whose only loop is the eight-byte signature
/// comparison (at most eight), and the wrapper charges one further step for that
/// comparison, so the actual own work is at most ten and 16 is a safe constant
/// ceiling. It never scans `E` and never parses all records: at most one record
/// is revalidated, so the count does not grow with `count`.
pub open spec fn wrapper_own_bound() -> int {
    16
}

/// R13/T9: the DIB call's own published bound `steps_DIB <= 16*(|slice|+N)`,
/// evaluated on the selected-DIB decision; `0` on every other decision (no DIB
/// call is made). `N` is the DIB crate's `t12_checked_n`, exported for exactly
/// this purpose.
pub open spec fn dib_steps_bound(b: Seq<u8>, es: Seq<Entry>, index: u32) -> int {
    match md_ico(b, es, index) {
        IcoDecision::DibSlice(start, end) => 16 * (b.subrange(start, end).len()
            + dib_verus::dib::t12_checked_n(b.subrange(start, end), Mode::IcoEntryDibSlack)),
        _ => 0,
    }
}

/// `dib_steps_bound` is never negative (it is `16*(...)` or `0`).
pub proof fn lemma_dib_steps_bound_nonneg(b: Seq<u8>, es: Seq<Entry>, index: u32)
    ensures
        0 <= dib_steps_bound(b, es, index),
{
    reveal(dib_steps_bound);
    reveal(md_ico);
    match md_ico(b, es, index) {
        IcoDecision::DibSlice(start, end) => {
            assert(b.subrange(start, end).len() >= 0);
            assert(dib_verus::dib::t12_checked_n(b.subrange(start, end), Mode::IcoEntryDibSlack)
                >= 0);
        },
        _ => {},
    }
}

/// On a selected-DIB decision `dib_steps_bound` is the DIB call's own bound; on
/// every other decision it is `0`.
pub proof fn lemma_dib_steps_bound_zero_off_dib(b: Seq<u8>, es: Seq<Entry>, index: u32)
    requires
        !(md_ico(b, es, index) is DibSlice),
    ensures
        dib_steps_bound(b, es, index) == 0,
{
    reveal(dib_steps_bound);
    reveal(md_ico);
}

/// R13/T9: the wrapper's total charged steps — its own constant work plus the
/// DIB call when a selected DIB is decoded — are within the wrapper ceiling plus
/// the DIB call's own proved bound. `t9_step_cap` is that ceiling; on every
/// non-DIB decision the DIB term is `0` and the cap is the constant wrapper
/// bound. This is the manifest's combined statement: the ICO-side counter summed
/// with the DIB `decode_with_steps` counter stays within the sum of their two
/// proved bounds.
pub open spec fn t9_step_cap(b: Seq<u8>, es: Seq<Entry>, index: u32) -> int {
    wrapper_own_bound() + dib_steps_bound(b, es, index)
}

/// T9 (ICO-side linear work): `parse`'s charged steps are within `16*(|b|+1)` for
/// every slice. On the directory-error path the guard charged at most seven; on
/// success `count` is in `[1, 1024]` and `6 + 16*count <= |b|`, so
/// `8 + 25*count <= 16*(6 + 16*count + 1) <= 16*(|b|+1)`.
pub proof fn lemma_t9_parse_bound(b: Seq<u8>)
    ensures
        parse_step_bound(b) <= 16 * (b.len() + 1),
{
    reveal(h);
    match h(b) {
        Err(_) => {},
        Ok(hr) => {
            assert(hr.count >= 1);
            assert(hr.dir_end == 6 + 16 * hr.count);
            assert(hr.count <= max_entries());
            assert(1 + 9 * hr.count <= 16 * (6 + 16 * hr.count + 1)) by (nonlinear_arith)
                requires
                    1 <= hr.count <= max_entries(),
                    max_entries() == 1024,
            ;
            assert(16 * (6 + 16 * hr.count + 1) <= 16 * (b.len() + 1)) by (nonlinear_arith)
                requires
                    hr.dir_end == 6 + 16 * hr.count,
                    hr.dir_end <= b.len(),
            ;
        },
    }
}

/// T9 (wrapper): the wrapper's own steps are at most `16*(|b|+1)` for every
/// slice, and its eight-byte signature comparison charges at most eight, for
/// every decision `decode_entry` can take. The count does not depend on `count`
/// nor on `es`: `md_ico` is a function of `|es|`, `es[index]` and the wire.
pub proof fn lemma_t9_wrapper_bound(b: Seq<u8>)
    ensures
        wrapper_own_bound() <= 16 * (b.len() + 1),
        wrapper_own_bound() == 16,
{
    assert(wrapper_own_bound() == 16);
    assert(16 <= 16 * (b.len() + 1)) by (nonlinear_arith)
        requires
            b.len() >= 0,
    ;
}

/// T9 (wrapper, "never scans `E`"): the wrapper's step **bound** is a function
/// of the ICO decision alone, and `md_ico` reads only `|E|` and `E[index]`
/// (`lemma_t6_decode_reads_only_selected`). Two lists of equal length with equal
/// selected element therefore take the same decision, hence the same step
/// **ceiling** (`t9_step_cap`); the wrapper never iterates the entry list. This
/// proves equality of the bound `t9_step_cap`, not of a real step count: the
/// charged count is not observable here, and the two lists need not charge the
/// same number of steps.
pub proof fn lemma_t9_wrapper_ignores_list(b: Seq<u8>, es1: Seq<Entry>, es2: Seq<Entry>, index: u32)
    requires
        es1.len() == es2.len(),
        (index as int) < es1.len() ==> es1[index as int] == es2[index as int],
    ensures
        md_ico(b, es1, index) == md_ico(b, es2, index),
{
    lemma_t6_decode_reads_only_selected(b, es1, es2, index);
}

/// T9 (the combined statement): the DIB 1.1 `decode_with_steps` postcondition
/// `steps_DIB <= 16*(|slice|+N)` on the selected slice, composed with the
/// wrapper's constant own-work bound, gives `t9_step_cap`. This is the
/// manifest's "combined bound from both counters" — the DIB half is the
/// dependency's own proved contract on the very function ICO calls.
pub proof fn lemma_t9_total(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    steps: int,
    dib_steps: int,
)
    requires
        steps <= wrapper_own_bound() + dib_steps,
        match md_ico(b, es, index) {
            IcoDecision::DibSlice(start, end) => dib_steps <= 16 * (b.subrange(start, end).len()
                + dib_verus::dib::t12_checked_n(
                b.subrange(start, end),
                Mode::IcoEntryDibSlack,
            )),
            _ => dib_steps == 0,
        },
    ensures
        steps <= t9_step_cap(b, es, index),
{
    reveal(t9_step_cap);
    reveal(dib_steps_bound);
    reveal(md_ico);
    match md_ico(b, es, index) {
        IcoDecision::DibSlice(start, end) => {
            assert(dib_steps <= dib_steps_bound(b, es, index));
            assert(steps <= wrapper_own_bound() + dib_steps_bound(b, es, index))
                by (nonlinear_arith)
                requires
                    steps <= wrapper_own_bound() + dib_steps,
                    dib_steps <= dib_steps_bound(b, es, index),
            ;
        },
        _ => {
            assert(dib_steps_bound(b, es, index) == 0);
        },
    }
}

// ---------------------------------------------------------------------------
// M4a · R2/R13/R15: the allocation-call model (T10)
// ---------------------------------------------------------------------------
//
// THEOREMS.md T10: header-error paths request no entry-list allocation; after
// the directory/count/table guards request at most one bounded allocation for at
// most 1,024 records; the decode wrapper makes no allocation and invokes DIB at
// most once. A ghost call counter charged at the documented sites expresses the
// shape; it cannot detect a new uncharged `vec!`/clone/collect, so the source
// audit `tools/audit_alloc.sh` is the enforcement (see the report).

/// R15/T10: the number of entry-list allocations `parse` requests — zero on
/// every directory-wide error (the guard returns before the one reservation),
/// one on a successful directory guard.
pub open spec fn parse_alloc_calls(b: Seq<u8>) -> int {
    match h(b) {
        Err(_) => 0,
        Ok(_) => 1,
    }
}

/// R15/T10: the wrapper (`decode_entry`) requests no allocation.
pub open spec fn decode_alloc_calls(b: Seq<u8>, es: Seq<Entry>, index: u32) -> int {
    0
}

/// T10: the header-error paths request no entry-list allocation.
pub proof fn lemma_t10_parse_header_error_no_alloc(b: Seq<u8>, e: Error)
    requires
        h(b) == Err(e),
    ensures
        parse_alloc_calls(b) == 0,
        mc(b) == Err(e),
{
    reveal(h);
    reveal(parse_alloc_calls);
    reveal(mc);
}

/// T10: after the guards `parse` requests at most one bounded allocation, and its
/// size is at most `max_entries` records.
pub proof fn lemma_t10_parse_one_bounded(b: Seq<u8>, hr: HRec)
    requires
        h(b) == Ok(hr),
    ensures
        parse_alloc_calls(b) == 1,
        1 <= hr.count <= max_entries(),
{
    reveal(h);
    reveal(parse_alloc_calls);
}

/// T10: `decode_entry` requests no allocation.
pub proof fn lemma_t10_decode_no_alloc(b: Seq<u8>, es: Seq<Entry>, index: u32)
    ensures
        decode_alloc_calls(b, es, index) == 0,
{
    reveal(decode_alloc_calls);
}

// ---------------------------------------------------------------------------
// M4a · the symbolic boundary lemmas L1-L4 (spec-frozen "Non-vacuity and
// symbolic boundaries"; never by expanding a 64 MiB verifier term)
// ---------------------------------------------------------------------------

/// L1: every slice longer than `MAX_INPUT` is `FileTooLarge` on `parse` **and**
/// on `decode_entry` for every `E` and every `i`, decided by the length alone
/// before any read. (`too-large-before-bad-header` is the worst case: a
/// single-byte seed repeated to 67,108,865 bytes.)
pub proof fn lemma_l1_over_cap(b: Seq<u8>, es: Seq<Entry>, index: u32)
    requires
        b.len() > max_input(),
    ensures
        h(b) == Err(Error::FileTooLarge),
        mc(b) == Err(Error::FileTooLarge),
        md_ico(b, es, index) == IcoDecision::DirErr(Error::FileTooLarge),
        md(b, es, index) == Err(Error::FileTooLarge),
{
    reveal(h);
    reveal(mc);
    reveal(md_ico);
    reveal(md);
}

/// The 30-byte prefix of `file-size-67108864`: a one-record ICO directory whose
/// single record is a valid 8-byte PNG at `[22, 30)`, followed by the signature.
pub open spec fn l2_prefix() -> Seq<u8> {
    seq![
        0x00u8, 0x00, 0x01, 0x00, 0x01, 0x00, // reserved=0, type=1, count=1
        0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, // w=1,h=1,cc=0,res=0,planes=1,bits=24
        0x08, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, // size=8, offset=22
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, // PNG signature
    ]
}

/// The one validated PNG record `file-size-67108864` parses to.
pub open spec fn l2_record() -> Entry {
    Entry {
        width: 1,
        height: 1,
        color_count: 0,
        raw_offset: 22,
        raw_size: 8,
        planes: Some(1),
        bit_count: Some(24),
        hotspot_x: None,
        hotspot_y: None,
        status: EntryStatus::Valid { kind: EntryKind::Png, payload: [22, 30] },
    }
}

/// `b[start + i] == lit[i]` for `i < len` whenever `b`'s subrange at `start` is
/// `lit` — the fact that lets L2 read the fixture's own bytes from its prefix.
pub proof fn lemma_subrange_index_at(b: Seq<u8>, start: int, len: int, lit: Seq<u8>)
    requires
        0 <= start,
        start + len <= b.len(),
        lit.len() == len,
        b.subrange(start, start + len) == lit,
    ensures
        forall|i: int| 0 <= i < len ==> #[trigger] b[start + i] == lit[i],
{
    broadcast use vstd::seq::group_seq_axioms;

    assert forall|i: int| 0 <= i < len implies #[trigger] b[start + i] == lit[i] by {
        assert(b.subrange(start, start + len)[i] == b[start + i]);
    }
}

/// The reverse: if `b`'s bytes at `start` equal `lit`, the subrange equals `lit`.
pub proof fn lemma_subrange_eq_lit(b: Seq<u8>, start: int, lit: Seq<u8>)
    requires
        0 <= start,
        start + lit.len() <= b.len(),
        forall|i: int| 0 <= i < lit.len() ==> #[trigger] b[start + i] == lit[i],
    ensures
        b.subrange(start, start + lit.len()) == lit,
{
    broadcast use vstd::seq::group_seq_axioms;

    assert(b.subrange(start, start + lit.len()) =~= lit) by {
        assert forall|i: int| 0 <= i < lit.len() implies
            b.subrange(start, start + lit.len())[i] == lit[i] by {
            assert(b.subrange(start, start + lit.len())[i] == b[start + i]);
        }
    }
}

/// L2: the exact `file-size-67108864` length, a 30-byte one-PNG-entry prefix and
/// zero suffix: it passes the cap and yields exactly one validated PNG range
/// `[22, 30)`. Stated for every slice of that length whose first 30 bytes are the
/// prefix (the suffix — zero bytes in the fixture — cannot change the result,
/// since `H` and `ME` read only bytes below 30). This is the symbolic length
/// lemma, not a 64 MiB verifier expansion.
pub proof fn lemma_l2_exact_cap(b: Seq<u8>)
    requires
        b.len() == 67108864,
        b.subrange(0, 30) == l2_prefix(),
    ensures
        mc(b) == Ok(seq![l2_record()]),
{
    reveal(h);
    reveal(mc);
    reveal(me);
    reveal(defect_of);
    reveal(kind_of);
    reveal(nom_dim);
    lemma_subrange_index_at(b, 0, 30, l2_prefix());
    assert(b[0] == 0u8);
    assert(b[1] == 0u8);
    assert(b[2] == 1u8);
    assert(b[3] == 0u8);
    assert(b[4] == 1u8);
    assert(b[5] == 0u8);
    assert(b[8] == 0u8);
    assert(b[9] == 0u8);
    // The declared range `[22, 30)` holds the eight signature bytes.
    assert(forall|k: int| 0 <= k < 8 ==> #[trigger] b[22 + k] == png_signature()[k]) by {
        assert forall|k: int| 0 <= k < 8 implies #[trigger] b[22 + k] == png_signature()[k] by {
            assert(b[22 + k] == l2_prefix()[22 + k]);
        }
    }
    lemma_subrange_eq_lit(b, 22, png_signature());
    assert(b.subrange(22, 30) == png_signature());
    assert(le16(b, 0) == 0);
    assert(le16(b, 2) == 1);
    assert(le16(b, 4) == 1);
    // Record 0 at p = 6: `size = le32(b, 14) = 8`, `offset = le32(b, 18) = 22`.
    assert(le32(b, 14) == 8);
    assert(le32(b, 18) == 22);
    let hr0 = HRec { typ: 1, count: 1, dir_end: 22 };
    assert(h(b) == Ok(hr0));
    assert(mc(b) == Ok(Seq::new(hr0.count as nat, |i: int| me(b, hr0, i))));
    assert(me(b, hr0, 0) == l2_record()) by {
        assert(le32(b, 14) == 8);
        assert(le32(b, 18) == 22);
    }
    assert(Seq::new(hr0.count as nat, |i: int| me(b, hr0, i)) == seq![l2_record()]) by {
        assert(Seq::new(hr0.count as nat, |i: int| me(b, hr0, i))[0] == me(b, hr0, 0));
        assert(Seq::new(hr0.count as nat, |i: int| me(b, hr0, i)).len() == 1);
        assert(Seq::new(hr0.count as nat, |i: int| me(b, hr0, i)) =~= seq![l2_record()]);
    }
}

/// L3: `parse`'s record-loop induction. `MC` on a successful directory is the
/// complete ordered list of `count` records, each exactly its mathematical `ME`;
/// its `j`-th element is `ME(b,H,j)` for every `j < count`, including invalid
/// statuses — irrespective of any record defect, so a bad record never aborts
/// the list. (The executable proves the same prefix equality as its loop
/// invariant `acc == Seq::new(i, ME)`; the runtime `count-exact-1024` fixture
/// exercises 1,024 distinct records without a verifier term.)
pub proof fn lemma_l3_record_induction(b: Seq<u8>, hr: HRec, j: int)
    requires
        h(b) == Ok(hr),
        0 <= j < hr.count,
    ensures
        mc(b) == Ok(Seq::new(hr.count as nat, |i: int| me(b, hr, i))),
        Seq::new(hr.count as nat, |i: int| me(b, hr, i))[j] == me(b, hr, j),
{
    reveal(mc);
    lemma_seq_new_index(hr.count as nat, |i: int| me(b, hr, i), j);
}

/// L4: for every valid record the range satisfies `offset >= dir_end`,
/// `size > 0`, `offset <= |b|` and `size <= |b| - offset`, so the end fits the
/// file; the invalid records assert no range. This distinguishes a 16-bit
/// narrowing and a u32 wrap (`wide-size-10000`, `wide-offset-10016`,
/// `wide-sum-wrap`), which are exercised as parse vectors.
pub proof fn lemma_l4_valid_range(b: Seq<u8>, hr: HRec, i: int)
    requires
        h(b) == Ok(hr),
        0 <= i < hr.count,
        me(b, hr, i).status is Valid,
    ensures
        hr.dir_end <= (me(b, hr, i).raw_offset as int),
        (me(b, hr, i).raw_size as int) > 0,
        (me(b, hr, i).raw_offset as int) <= b.len(),
        (me(b, hr, i).raw_size as int) <= b.len() - (me(b, hr, i).raw_offset as int),
{
    lemma_valid_payload_in_bounds(b, hr, i);
    reveal(me);
    reveal(defect_of);
    assert(me(b, hr, i).status is Valid);
}

} // verus!
