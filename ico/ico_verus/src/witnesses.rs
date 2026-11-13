//! T11 (witness bridge): ICO-only public-call witnesses, hand-written (not
//! generated).  THEOREMS.md T11 asks for
//!
//! * `MC` versus a small subset of independently authored expected records — the
//!   `model_vectors.rs` parse agreements supply these (e.g. `short-dir-0`,
//!   `valid-then-bad-oob`, `directory-order-large-first`);
//! * one public `decode_entry` call on a file containing a valid selected record
//!   beside an invalid unselected record (`t11_valid_selected_beside_invalid_unselected`);
//! * one forged selected record producing `InvalidEntryRecord`
//!   (`t11_forged_selected_invalid`).
//!
//! These are ICO-only model obligations: no proof-gate witness needs a concrete
//! `MP_1.1` DIB pixel evaluation.  The selected records in the two public calls
//! below are PNG (the manifest's non-vacuity allowance: "`forged-unselected-ignored`
//! can use a selected PNG signature"), so every branch is decided by the
//! ICO-controlled model and the `Dib(...)` pixels stay a runtime fixture bridged
//! by the generic T5 slice-model lemma.  Expected values come from
//! `spec-frozen/VECTORS.json` fixtures (`forged-wrong-kind`) and from an
//! independently transcribed two-record file, never from the executable.
//!
//! The two public calls take the file and the supplied entries as parameters
//! pinned to the fixture by `requires`; this is the same shape the other public
//! witnesses (`t5_decode_exact`, `t6_parse_directory_priority`) use, and it keeps
//! `bytes@` a parameter view (an exec-local array/slice `.view()` trips a Verus
//! internal error, so the fixtures are stated as spec sequences and the exec
//! literals live with the caller).
use vstd::prelude::*;

#[allow(unused_imports)]
use crate::alloc_guard::TryError;
#[allow(unused_imports)]
use crate::ico::{try_decode_entry, Entry, EntryDefect, EntryKind, EntryStatus, Error};
#[allow(unused_imports)]
use dib_verus::Decoded;

verus! {

/// The 46-byte two-record file: record 0 is a valid PNG (`[38, 46)`), record 1
/// is an invalid out-of-file record (`size 1`, `offset 65558`).  Record 0 is
/// selected; record 1 is the invalid unselected record.
pub open spec fn w_vsbiu_input() -> Seq<u8> {
    seq![
        0x00u8, 0x00, 0x01, 0x00, 0x02, 0x00, // reserved=0, type=1, count=2
        0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, // rec0 w=1 h=1 cc=0 res=0 planes=1 bits=24
        0x08, 0x00, 0x00, 0x00, 0x26, 0x00, 0x00, 0x00, // rec0 size=8, offset=38
        0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, // rec1 w=1 h=1 cc=0 res=0 planes=1 bits=24
        0x01, 0x00, 0x00, 0x00, 0x16, 0x00, 0x01, 0x00, // rec1 size=1, offset=65558
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, // rec0 payload: PNG signature
    ]
}

/// The supplied entry list: record 0's wire reconstruction (valid PNG) beside
/// record 1's wire reconstruction (invalid out of file), as `parse` would build
/// them.
pub open spec fn w_vsbiu_entries() -> Seq<Entry> {
    seq![
        Entry {
            width: 1,
            height: 1,
            color_count: 0,
            raw_offset: 38,
            raw_size: 8,
            planes: Some(1),
            bit_count: Some(24),
            hotspot_x: None,
            hotspot_y: None,
            status: EntryStatus::Valid { kind: EntryKind::Png, payload: [38, 46] },
        },
        Entry {
            width: 1,
            height: 1,
            color_count: 0,
            raw_offset: 65558,
            raw_size: 1,
            planes: Some(1),
            bit_count: Some(24),
            hotspot_x: None,
            hotspot_y: None,
            status: EntryStatus::Invalid(EntryDefect::EntryOutOfBounds),
        },
    ]
}

/// T11: a public `decode_entry` call on the file with a valid selected record
/// beside an invalid unselected record.  `MD` is `Err(PngDelegation)` for the
/// selected PNG, so the invalid unselected record 1 cannot change the result;
/// the executable call returns exactly that error.
pub fn t11_valid_selected_beside_invalid_unselected(bytes: &[u8], entries: &[Entry]) -> (r:
    Result<Decoded, TryError>)
    requires
        bytes@ == w_vsbiu_input(),
        entries@ == w_vsbiu_entries(),
    ensures
        crate::alloc_guard::sites::fmt_part_decoded(r) == Err(Error::PngDelegation),
{
    proof {
        assert(crate::model::md(w_vsbiu_input(), w_vsbiu_entries(), 0u32) == Err(
            Error::PngDelegation,
        )) by (compute);
    }
    proof {
        reveal(crate::model::md_ico);
        assert(!(crate::model::md_ico(bytes@, entries@, 0u32) is DibSlice));
        crate::alloc_guard::sites::lemma_req_d_empty_off_dib(bytes@, entries@, 0u32);
    }
    let r = try_decode_entry(bytes, entries, 0u32);
    proof {
        let l = choose|l: Seq<(crate::alloc_guard::AllocSite, nat, bool)>|
            crate::alloc_guard::sites::g_decode(bytes@, entries@, 0u32, r, l);
        crate::alloc_guard::sites::lemma_g_decode_format(
            bytes@, entries@, 0u32, r, l, Error::PngDelegation,
        );
    }
    r
}

/// The 70-byte `forged-wrong-kind` fixture: one valid DIB record whose supplied
/// (forged) kind is `Png`.
pub open spec fn w_forged_wrong_kind_input() -> Seq<u8> {
    seq![
        0x00u8, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x30, 0x00,
        0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]
}

/// The forged supplied record: the wire record at offset 22, size 48, is a valid
/// `Dib`, but the supplied record claims `Png`.
pub open spec fn w_forged_wrong_kind_entries() -> Seq<Entry> {
    seq![
        Entry {
            width: 1,
            height: 1,
            color_count: 0,
            raw_offset: 22,
            raw_size: 48,
            planes: Some(1),
            bit_count: Some(24),
            hotspot_x: None,
            hotspot_y: None,
            status: EntryStatus::Valid { kind: EntryKind::Png, payload: [22, 70] },
        },
    ]
}

/// T11: a public `decode_entry` call on the `forged-wrong-kind` fixture — a
/// forged selected record whose sole disagreement is the kind.  `MD` is
/// `Err(InvalidEntryRecord)` (the field-for-field, status-for-status selected
/// comparison fails), and the executable call returns exactly that error without
/// decoding any DIB.
pub fn t11_forged_selected_invalid(bytes: &[u8], entries: &[Entry]) -> (r: Result<Decoded, TryError>)
    requires
        bytes@ == w_forged_wrong_kind_input(),
        entries@ == w_forged_wrong_kind_entries(),
    ensures
        crate::alloc_guard::sites::fmt_part_decoded(r) == Err(Error::InvalidEntryRecord),
{
    proof {
        assert(crate::model::md(w_forged_wrong_kind_input(), w_forged_wrong_kind_entries(), 0u32)
            == Err(Error::InvalidEntryRecord)) by (compute);
    }
    proof {
        reveal(crate::model::md_ico);
        assert(!(crate::model::md_ico(bytes@, entries@, 0u32) is DibSlice));
        crate::alloc_guard::sites::lemma_req_d_empty_off_dib(bytes@, entries@, 0u32);
    }
    let r = try_decode_entry(bytes, entries, 0u32);
    proof {
        let l = choose|l: Seq<(crate::alloc_guard::AllocSite, nat, bool)>|
            crate::alloc_guard::sites::g_decode(bytes@, entries@, 0u32, r, l);
        crate::alloc_guard::sites::lemma_g_decode_format(
            bytes@, entries@, 0u32, r, l, Error::InvalidEntryRecord,
        );
    }
    r
}

/// The `forged-unselected-ignored` witness file and supplied list: record 0 is a
/// valid PNG (`[38, 46)`, the manifest's allowance that this witness "can use a
/// selected PNG signature"), and record 1 is a *forged* supplied record that
/// disagrees with the wire (the wire record 1 is an out-of-file record; this one
/// claims a valid `Dib` at `[38, 86)`).  Record 0 is selected, so the forged
/// unselected record 1 must not change the result.
pub open spec fn w_forged_unselected_entries() -> Seq<Entry> {
    seq![
        Entry {
            width: 1,
            height: 1,
            color_count: 0,
            raw_offset: 38,
            raw_size: 8,
            planes: Some(1),
            bit_count: Some(24),
            hotspot_x: None,
            hotspot_y: None,
            status: EntryStatus::Valid { kind: EntryKind::Png, payload: [38, 46] },
        },
        Entry {
            width: 2,
            height: 1,
            color_count: 0,
            raw_offset: 38,
            raw_size: 48,
            planes: Some(1),
            bit_count: Some(24),
            hotspot_x: None,
            hotspot_y: None,
            status: EntryStatus::Valid { kind: EntryKind::Dib, payload: [38, 86] },
        },
    ]
}

/// T11 (`forged-unselected-ignored`): a public `decode_entry` call on the file
/// with a valid selected record (PNG) beside a forged unselected record.  `MD`
/// is `Err(PngDelegation)` for the selected PNG and reads only `entries[0]`, so
/// the forged unselected record 1 cannot change the result; the executable call
/// returns exactly that error.  This is the same decision `MD` takes on the
/// all-wire list, so the forged record is genuinely ignored.
pub fn t11_forged_unselected_ignored(bytes: &[u8], entries: &[Entry]) -> (r: Result<Decoded, TryError>)
    requires
        bytes@ == w_vsbiu_input(),
        entries@ == w_forged_unselected_entries(),
    ensures
        crate::alloc_guard::sites::fmt_part_decoded(r) == Err(Error::PngDelegation),
{
    proof {
        assert(crate::model::md(w_vsbiu_input(), w_forged_unselected_entries(), 0u32) == Err(
            Error::PngDelegation,
        )) by (compute);
        // The forged unselected element is ignored: `MD` on the all-wire list
        // takes the same decision.
        assert(crate::model::md_ico(w_vsbiu_input(), w_forged_unselected_entries(), 0u32)
            == crate::model::md_ico(w_vsbiu_input(), w_vsbiu_entries(), 0u32)) by (compute);
    }
    proof {
        reveal(crate::model::md_ico);
        assert(!(crate::model::md_ico(bytes@, entries@, 0u32) is DibSlice));
        crate::alloc_guard::sites::lemma_req_d_empty_off_dib(bytes@, entries@, 0u32);
    }
    let r = try_decode_entry(bytes, entries, 0u32);
    proof {
        let l = choose|l: Seq<(crate::alloc_guard::AllocSite, nat, bool)>|
            crate::alloc_guard::sites::g_decode(bytes@, entries@, 0u32, r, l);
        crate::alloc_guard::sites::lemma_g_decode_format(
            bytes@, entries@, 0u32, r, l, Error::PngDelegation,
        );
    }
    r
}

} // verus!
