//! Checked projection of the public policy record into the walk's rules.
use vstd::prelude::*;
use crate::policy::*;
use crate::ordering::Rules as OrderRules;
#[cfg(verus_only)]
use crate::{document_model as m, walk_model};

verus! {

#[derive(Clone,Copy)]
pub(crate) struct Rules {
    pub order: OrderRules,
    pub skip_ancillary_crc: bool,
    pub allow_eof_iend: bool,
    pub ignore_trailing: bool,
    pub allow_large_palette: bool,
    pub drop_invalid_trns: bool,
    pub ignore_apng_in_still: bool,
    pub static_apng_errors: bool,
}
impl Rules {
    pub(crate) open spec fn view(self) -> m::Policy {
        m::Policy {order:self.order.view(),skip_ancillary_crc:self.skip_ancillary_crc,
            allow_eof_iend:self.allow_eof_iend,ignore_trailing:self.ignore_trailing,
            allow_large_palette:self.allow_large_palette,drop_invalid_trns:self.drop_invalid_trns,
            ignore_apng_in_still:self.ignore_apng_in_still,static_apng_errors:self.static_apng_errors}
    }
}
impl Policy {
    pub(crate) open spec fn walk_view(self) -> m::Policy {
        m::Policy {order:walk_model::Rules {
            first_duplicate_wins:matches!(self.ancillary_duplicate,AncillaryDuplicate::FirstWins),
            drop_misordered:matches!(self.ancillary_order,AncillaryOrder::Drop),
            drop_invalid:matches!(self.ancillary_values,AncillaryValues::Drop),
            ignore_unexpected_plte:matches!(self.plte_unexpected,PlteUnexpected::Ignore),
            lenient_iend:matches!(self.iend,Iend::Lenient)},
            skip_ancillary_crc:matches!(self.crc_ancillary,CrcAncillary::SkipExceptFdat),
            allow_eof_iend:matches!(self.missing_iend,MissingIend::AtEof),
            ignore_trailing:matches!(self.trailing,Trailing::Ignore),
            allow_large_palette:matches!(self.palette_size,PaletteSize::Allow),
            drop_invalid_trns:matches!(self.trns_invalid,TrnsInvalid::Drop),
            ignore_apng_in_still:matches!(self.apng_in_still,ApngInStill::Ignore),
            static_apng_errors:matches!(self.apng_errors,ApngErrors::Static)}
    }
    pub(crate) fn walk_rules(&self) -> (r: Rules)
        ensures r.view()==self.walk_view(),
    {
        Rules {order:OrderRules {
            first_duplicate_wins:matches!(self.ancillary_duplicate,AncillaryDuplicate::FirstWins),
            drop_misordered:matches!(self.ancillary_order,AncillaryOrder::Drop),
            drop_invalid:matches!(self.ancillary_values,AncillaryValues::Drop),
            ignore_unexpected_plte:matches!(self.plte_unexpected,PlteUnexpected::Ignore),
            lenient_iend:matches!(self.iend,Iend::Lenient)},
            skip_ancillary_crc:matches!(self.crc_ancillary,CrcAncillary::SkipExceptFdat),
            allow_eof_iend:matches!(self.missing_iend,MissingIend::AtEof),
            ignore_trailing:matches!(self.trailing,Trailing::Ignore),
            allow_large_palette:matches!(self.palette_size,PaletteSize::Allow),
            drop_invalid_trns:matches!(self.trns_invalid,TrnsInvalid::Drop),
            ignore_apng_in_still:matches!(self.apng_in_still,ApngInStill::Ignore),
            static_apng_errors:matches!(self.apng_errors,ApngErrors::Static)}
    }
}

pub(crate) proof fn profiles_project_exactly()
    ensures STRICT.walk_view()==m::strict(),RECOMMENDED_BETA.walk_view()==m::recommended_beta(),
{}

} // verus!
