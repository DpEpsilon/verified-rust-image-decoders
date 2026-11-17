use crate::{bits::Bits, limits::*, InflateError, Segments, Work};
#[cfg(verus_only)]
use crate::model as m;
use vstd::prelude::*;

pub(crate) use crate::classification::Kind;

verus! {

/// Unused symbol slots remain unspecified after rebuilding.
pub(crate) struct Huffman {
    pub(crate) count: [u16; MAX_CODE_BITS + 1],
    pub(crate) first: [u32; MAX_CODE_BITS + 1],
    pub(crate) offset: [u16; MAX_CODE_BITS + 1],
    pub(crate) symbol: [u16; FIXED_LIT_COUNT],
    pub(crate) live: [u32; MAX_CODE_BITS + 1],
    pub(crate) nsyms: usize,
    pub(crate) lengths: Ghost<Seq<u8>>,
    pub(crate) kind: Ghost<Kind>,
    pub(crate) ready: Ghost<bool>,
}

impl Huffman {
    pub(crate) closed spec fn repr(&self) -> bool {
        crate::canonical::table_repr(self.lengths@, self.count@, self.first@, self.offset@, self.symbol@)
        && self.nsyms == self.lengths@.len() - m::count_prefix(self.lengths@, 0, self.lengths@.len())
        && (forall|n: int| 0 <= n <= MAX_CODE_BITS ==> self.live@[n] == m::live_count(self.lengths@, n as nat))
    }

    pub(crate) closed spec fn wf(&self) -> bool {
        self.ready@ ==> self.repr() && m::admissible(self.lengths@,
            crate::classification::model_kind(self.kind@), true)
    }

    /// Representation exists before the separate kind-specific admission test.
    pub(crate) fn build(&mut self, lengths: &[u8], kind: Kind, w: &mut Work) -> (r: Result<(), InflateError>)
        requires old(w).wf(), old(w).core + HUFF_BUILD_STEPS <= u128::MAX,
        ensures final(self).wf(), final(w).wf(),
            final(w).core >= old(w).core, final(w).core <= old(w).core + HUFF_BUILD_STEPS,
            final(self).ready@ == r.is_ok(),
            r.is_ok() == m::admissible(lengths@, crate::classification::model_kind(kind), true),
            r.is_err() ==> r.err() == Some(InflateError::InvalidCodeLengths),
            (m::lengths_ok(lengths@) && m::Kraft(lengths@) <= CODE_SPACE) ==>
                final(self).repr() && final(self).lengths@ == lengths@,
            r.is_ok() ==> final(self).kind@ == kind,
    {
        let kraft = match self.build_canonical(lengths, w) {
            Ok(kraft) => kraft,
            Err(e) => return Err(e),
        };
        proof {
            crate::classification::lemma_admissible_from_counts(lengths@, kind, true);
            self.kind = Ghost(kind);
        }
        let admissible = crate::classification::is_admissible(kind, kraft, self.nsyms, self.count[1], true);
        if admissible {
            proof { self.ready = Ghost(true); }
            Ok(())
        } else { Err(InflateError::InvalidCodeLengths) }
    }

    pub(crate) fn new(w: &mut Work) -> (h: Self)
        requires old(w).wf(), old(w).core + HUFF_INIT_STEPS <= u128::MAX,
        ensures h.wf(), !h.ready@, final(w).wf(), final(w).core == old(w).core + HUFF_INIT_STEPS,
    {
        w.add(HUFF_INIT_STEPS);
        Self {
            count: [0; MAX_CODE_BITS + 1], first: [0; MAX_CODE_BITS + 1],
            offset: [0; MAX_CODE_BITS + 1], symbol: [0; FIXED_LIT_COUNT],
            live: [0; MAX_CODE_BITS + 1], nsyms: 0,
            lengths: Ghost(Seq::empty()), kind: Ghost(Kind::Dist), ready: Ghost(false),
        }
    }

    fn build_canonical(&mut self, lengths: &[u8], w: &mut Work) -> (r: Result<u32, InflateError>)
        requires old(w).wf(), old(w).core + HUFF_BUILD_STEPS <= u128::MAX,
        ensures final(w).wf(), final(w).core >= old(w).core,
            final(w).core <= old(w).core + HUFF_BUILD_STEPS,
            !final(self).ready@,
            r.is_ok() == (m::lengths_ok(lengths@) && m::Kraft(lengths@) <= CODE_SPACE),
            match r {
                Ok(kraft) => final(self).repr() && final(self).lengths@ == lengths@
                    && kraft == m::Kraft(lengths@),
                Err(e) => e == InflateError::InvalidCodeLengths,
            },
    {
        proof { self.ready = Ghost(false); }
        let (built, table_work) = crate::canonical::build_tables(lengths,
            &mut self.count, &mut self.first, &mut self.offset, &mut self.symbol);
        let (kraft, nonzero) = match built {
            Ok(value) => {
                w.add(CANON_FIXED_STEPS + 2 * lengths.len());
                value
            }
            Err(error) => {
                match error {
                    crate::canonical::BuildError::TooManySymbols => {}
                    crate::canonical::BuildError::InvalidLength(index) => {
                        w.add(MAX_CODE_BITS + 1 + index + 1);
                    }
                    crate::canonical::BuildError::Oversubscribed => {
                        w.add(MAX_CODE_BITS + 1 + lengths.len());
                    }
                }
                return Err(InflateError::InvalidCodeLengths);
            }
        };
        self.nsyms = nonzero;
        let live_work = crate::canonical::live_counts(Ghost(lengths@), &self.count, &mut self.live);
        w.add(MAX_CODE_BITS + 1);
        proof { self.lengths = Ghost(lengths@); }
        Ok(kraft)
    }
}

} // verus!

// The reader-loop refinement is the next layer-2 obligation.
impl Huffman {
