//! ICO's `alloc_guard`, a second sealed log in a second crate.
//!
//! This is the design question gate F3 (c) asks: can ICO's guard consume DIB's
//! returned `Tracked<dib::alloc_guard::AllocLog>` across the crate boundary,
//! read its `view()`, and write the mapped sequence into ICO's own private
//! field?
//!
//! Site type: `ico::AllocSite = EntryList | Dib(dib::AllocSite)` (SPEC.md §4).

use vstd::prelude::*;

pub mod sites;

use sites::ICOSite;

verus! {

/// ICO's sealed log. Its `ghost ev` field is private to *this* module, so DIB
/// cannot write it and ICO cannot write DIB's.
pub tracked struct AllocLog {
    ghost ev: Seq<(ICOSite, nat, bool)>,
}

impl AllocLog {
    pub closed spec fn view(self) -> Seq<(ICOSite, nat, bool)> {
        self.ev
    }
}

pub proof fn new_log() -> (tracked l: AllocLog)
    ensures l.view() == Seq::<(ICOSite, nat, bool)>::empty(),
{
    AllocLog { ev: Seq::empty() }
}

/// ICO's `OutOfMemory`. Same sealing rule as DIB's; its fields are private to
/// this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfMemory {
    site: ICOSite,
    elements: usize,
}

impl OutOfMemory {
    #[verifier::when_used_as_spec(site_spec)]
    pub fn site(&self) -> (s: ICOSite)
        ensures s == Self::site_spec(self),
    {
        self.site
    }

    #[verifier::when_used_as_spec(elements_spec)]
    pub fn elements(&self) -> (n: usize)
        ensures n == Self::elements_spec(self),
    {
        self.elements
    }

    pub closed spec fn site_spec(&self) -> ICOSite {
        self.site
    }

    pub closed spec fn elements_spec(&self) -> usize {
        self.elements
    }
}

/// ICO's outcome type (SPEC.md §1). `IcoError` stands in for the frozen ICO
/// error enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcoError {
    TruncatedDirectory,
    InvalidKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryError {
    Format(IcoError),
    OutOfMemory(OutOfMemory),
}

pub open spec fn fmt_part(r: Result<Vec<u8>, TryError>) -> Result<Vec<u8>, IcoError> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

} // verus!
