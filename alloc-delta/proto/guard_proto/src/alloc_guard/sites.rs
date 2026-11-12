//! Per-core site declarations (SPEC.md §3). Separately hashed and reviewed in
//! the real delta; a child of `alloc_guard`, so it may read and write the
//! private `ev` field of `AllocLog` and construct `OutOfMemory`.

use vstd::prelude::*;

use super::{AllocLog, OutOfMemory, TryError, log_tied};

verus! {

/// The closed per-core site enum. One variant per reservation (SPEC.md §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSite {
    Output,
    EntryList,
    Channel(u8),
    RgbOut,
    MaskData,
    CombineOut,
}

/// The ICNS `Entry` after decision D2 (`type_hex: [u8; 8]`). Size and alignment
/// asserted below with a `global layout` declaration; rustc checks it as a
/// static assertion, Verus turns it into an axiom for `size_of`/`align_of`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub type_hex: [u8; 8],
    pub payload: [u32; 2],
    pub opaque_candidate: bool,
    pub nominal_side: Option<u32>,
}

/// The ICO `Entry` (`origin/main:ico/ico_verus/src/ico.rs:101`), with its
/// `EntryStatus` collapsed to a two-variant stand-in; the sizes are the real
/// ones (44 bytes, 12-byte status).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IcoEntry {
    pub width: u16,
    pub height: u16,
    pub color_count: u8,
    pub raw_offset: u32,
    pub raw_size: u32,
    pub planes: Option<u16>,
    pub bit_count: Option<u16>,
    pub hotspot_x: Option<u16>,
    pub hotspot_y: Option<u16>,
    pub status: IcoEntryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcoEntryStatus {
    Invalid,
    Valid { kind: u8, payload: [u32; 2] },
}

// Gate F2: the per-target `Entry` layout, checked by rustc as a static
// assertion on x86_64. i686 is not installed (see NOTES.md).
global layout Entry is size == 28, align == 4;
global layout IcoEntry is size == 44, align == 4;

/// ICO's `import_dib` (SPEC.md §3.1): consumes DIB's returned log, appends its
/// events to ICO's log (through `map_dib`), and wraps a DIB `OutOfMemory`. Lives
/// in `sites.rs` because that is the module that may touch `AllocLog`'s private
/// field. In this same-crate prototype the two site types coincide, so
/// `map_dib` is the identity on sites; the cross-crate version is gate F3 (c).
pub open spec fn map_site(s: AllocSite) -> AllocSite {
    s
}

/// Exec counterpart of `map_site`, used by `import_dib`'s body to build the
/// wrapped `OutOfMemory`. In the real ICO core it maps `dib::AllocSite` to
/// `ico::AllocSite::Dib(..)`; here the two enums coincide, so it is the
/// identity.
pub fn map_site_exec(s: AllocSite) -> (r: AllocSite)
    ensures r == map_site(s),
{
    s
}

pub open spec fn map_dib(s: Seq<(AllocSite, nat, bool)>) -> Seq<(AllocSite, nat, bool)> {
    Seq::new(s.len(), |j: int| (map_site(s[j].0), s[j].1, s[j].2))
}

pub fn import_dib(
    r: Result<Vec<u8>, TryError>,
    Tracked(dlog): Tracked<AllocLog>,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (out: Result<Vec<u8>, TryError>)
    requires
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
        log_tied(r, dlog.view()),
    ensures
        final(log).view() == map_dib(dlog.view()),
        match r {
            Ok(d) => out == Ok(d),
            Err(TryError::Format(e)) => out == Err(TryError::Format(e)),
            Err(TryError::OutOfMemory(o)) => match out {
                Err(TryError::OutOfMemory(o2)) =>
                    o2.site_spec() == map_site(o.site_spec())
                    && o2.elements_spec() == o.elements_spec()
                    && final(log).view().last()
                        == (map_site(o.site_spec()), o.elements_spec() as nat, false),
                _ => false,
            },
        },
{
    proof { log.ev = map_dib(dlog.view()); }
    match r {
        Ok(d) => Ok(d),
        Err(TryError::Format(e)) => Err(TryError::Format(e)),
        Err(TryError::OutOfMemory(o)) => {
            Err(TryError::OutOfMemory(OutOfMemory {
                site: map_site_exec(o.site()),
                elements: o.elements(),
            }))
        },
    }
}

/// Gate F2: the layout facts, exported so modules and crates outside `sites.rs`
/// can use the numbers. A `global layout` declaration's axiom is only usable
/// inside the module that declares it, so every other user (the entry-list
/// reservation's `size_of::<T>() >= 1` and `n * size_of::<T>() <= isize::MAX`
/// preconditions among them) goes through this lemma. rustc checks the
/// declaration itself as a static assertion, so the numbers cannot drift.
pub proof fn lemma_entry_layout()
    ensures
        size_of::<Entry>() == 28,
        align_of::<Entry>() == 4,
        size_of::<IcoEntry>() == 44,
        align_of::<IcoEntry>() == 4,
{
    assert(size_of::<Entry>() == 28);
    assert(align_of::<Entry>() == 4);
    assert(size_of::<IcoEntry>() == 44);
    assert(align_of::<IcoEntry>() == 4);
}

} // verus!
