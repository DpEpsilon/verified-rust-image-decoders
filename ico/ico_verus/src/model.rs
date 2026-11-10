//! Formal reference model for ICO-CORE-SPEC 1.0 (milestone M2).
//!
//! This file is written *independently of the executable code*: every definition
//! below is a transcription of the frozen specification text (`spec-frozen/SPEC.md`,
//! R2..R7, R11) over `Seq<u8>` and mathematical integers. Nothing here calls,
//! mirrors the control flow of, or is derived from `crate::ico`. The executable
//! carries postconditions phrased in terms of these definitions; the definitions
//! themselves are never justified by running the executable.
//!
//! M2 needs exactly the total directory model `H`, the selected-record model
//! `ME` and the parse model `MC` of `THEOREMS.md` ("Total mathematical models"),
//! plus the arithmetic lemmas that bound a successful directory. `MD` and the
//! DIB composition are M3.
use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::seq::{lemma_seq_new_index, lemma_seq_new_len, lemma_seq_push_index_different, lemma_seq_push_index_same, lemma_seq_push_len};

#[allow(unused_imports)]
use crate::ico::{Entry, EntryDefect, EntryKind, EntryStatus, Error};
#[allow(unused_imports)]
use dib_verus::Decoded;

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

} // verus!
