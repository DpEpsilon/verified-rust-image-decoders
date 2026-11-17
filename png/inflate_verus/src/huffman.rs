#[cfg(verus_only)]
use crate::model as m;
use crate::{bits::Bits, limits::*, InflateError, Segments, Work};
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
    pub(crate) single_accept: Ghost<bool>,
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
            crate::classification::model_kind(self.kind@), self.single_accept@)
    }

    pub(crate) proof fn lemma_canon_forward(&self, s: int)
        requires self.repr(), 0 <= s < self.lengths@.len(), self.lengths@[s] > 0,
        ensures crate::canonical::table_lookup(self.count@, self.first@, self.offset@, self.symbol@,
            self.lengths@[s] as nat, m::cw_value(self.lengths@, s)) == Some(s as u16),
    {
        crate::canonical::lemma_lookup_canonical(self.lengths@, self.count@, self.first@,
            self.offset@, self.symbol@, s);
    }

    pub(crate) proof fn lemma_canon_reverse(&self, n: nat, code: nat, s: u16)
        requires self.repr(), 1 <= n <= MAX_CODE_BITS,
            crate::canonical::table_lookup(self.count@, self.first@, self.offset@, self.symbol@, n, code) == Some(s),
        ensures m::cw(self.lengths@, s as int) == Some(m::msb_bits(code, n)),
    {
        crate::canonical::lemma_lookup_word(self.lengths@, self.count@, self.first@,
            self.offset@, self.symbol@, n, code, s);
    }

    /// Representation exists before the separate kind-specific admission test.
    pub(crate) fn build(&mut self, lengths: &[u8], kind: Kind, w: &mut Work) -> (r: Result<(), InflateError>)
        requires old(w).wf(),
        ensures final(self).wf(), final(w).wf(),
            final(w).core >= old(w).core, final(w).core <= old(w).core + HUFF_BUILD_STEPS,
            final(self).ready@ == r.is_ok(),
            r.is_ok() == m::admissible(lengths@, crate::classification::model_kind(kind), true),
            r.is_err() ==> r.err() == Some(InflateError::InvalidCodeLengths),
            (m::lengths_ok(lengths@) && m::Kraft(lengths@) <= CODE_SPACE) ==>
                final(self).repr() && final(self).lengths@ == lengths@,
            r.is_ok() ==> final(self).kind@ == kind && final(self).single_accept@,
    { self.build_with_single(lengths, kind, true, w) }

    /// Representation exists before the separate kind-specific admission test.
    pub(crate) fn build_with_single(&mut self, lengths: &[u8], kind: Kind, single_accept: bool, w: &mut Work) -> (r: Result<(), InflateError>)
        requires old(w).wf(),
        ensures final(self).wf(), final(w).wf(),
            final(w).core >= old(w).core, final(w).core <= old(w).core + HUFF_BUILD_STEPS,
            final(self).ready@ == r.is_ok(),
            r.is_ok() == m::admissible(lengths@, crate::classification::model_kind(kind), single_accept),
            r.is_err() ==> r.err() == Some(InflateError::InvalidCodeLengths),
            (m::lengths_ok(lengths@) && m::Kraft(lengths@) <= CODE_SPACE) ==>
                final(self).repr() && final(self).lengths@ == lengths@,
            r.is_ok() ==> final(self).kind@ == kind && final(self).single_accept@ == single_accept,
    {
        let kraft = match self.build_canonical(lengths, w) {
            Ok(kraft) => kraft,
            Err(e) => return Err(e),
        };
        proof {
            crate::classification::lemma_admissible_from_counts(lengths@, kind, single_accept);
            self.kind = Ghost(kind);
            self.single_accept = Ghost(single_accept);
        }
        let admissible = crate::classification::is_admissible(kind, kraft, self.nsyms, self.count[1], single_accept);
        if admissible {
            proof { self.ready = Ghost(true); }
            Ok(())
        } else { Err(InflateError::InvalidCodeLengths) }
    }

    pub(crate) fn new(w: &mut Work) -> (h: Self)
        requires old(w).wf(),
        ensures h.wf(), !h.ready@, final(w).wf(), old(w).core <= final(w).core <= old(w).core + HUFF_INIT_STEPS,
            final(w).observed@ == old(w).observed@ + HUFF_INIT_STEPS,
    {
        w.add(HUFF_INIT_STEPS);
        Self {
            count: [0; MAX_CODE_BITS + 1], first: [0; MAX_CODE_BITS + 1],
            offset: [0; MAX_CODE_BITS + 1], symbol: [0; FIXED_LIT_COUNT],
            live: [0; MAX_CODE_BITS + 1], nsyms: 0,
            lengths: Ghost(Seq::empty()), kind: Ghost(Kind::Dist),
            single_accept: Ghost(true), ready: Ghost(false),
        }
    }

    fn build_canonical(&mut self, lengths: &[u8], w: &mut Work) -> (r: Result<u32, InflateError>)
        requires old(w).wf(),
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
        let ghost work_at_entry = w.observed@;
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
                proof { assert(w.observed@ == work_at_entry + table_work@); }
                return Err(InflateError::InvalidCodeLengths);
            }
        };
        self.nsyms = nonzero;
        let live_work = crate::canonical::live_counts(Ghost(lengths@), &self.count, &mut self.live);
        w.add(MAX_CODE_BITS + 1);
        proof {
            self.lengths = Ghost(lengths@);
            assert(w.observed@ == work_at_entry + table_work@ + live_work@);
        }
        Ok(kraft)
    }
}

impl Huffman {
    /// Decode on the actual reader; the invariant records the reachable model
    /// prefix before each attempted bit, including failed-lookup liveness.
    pub(crate) fn decode<S: Segments>(&self, br: &mut Bits<'_, S>, w: &mut Work)
        -> (r: Result<u16, InflateError>)
        requires self.wf(), self.ready@, old(br).wf(), old(w).wf(),
        ensures final(br).wf(), final(w).wf(), final(br).concat() == old(br).concat(),
            final(w).core >= old(w).core, final(w).core <= old(w).core + 3 * MAX_CODE_BITS,
            old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 3 * MAX_CODE_BITS,
            match r {
                Ok(s) => m::MDec(self.lengths@, old(br).concat(), old(br).position())
                    == (m::Decoded::Symbol { symbol: s as nat, position: final(br).position() }),
                Err(e) => (e == InflateError::InvalidSymbol
                    && m::MDec(self.lengths@, old(br).concat(), old(br).position())
                        == (m::Decoded::InvalidSymbol { position: final(br).position() }))
                    || (e == InflateError::TruncatedZlib
                    && m::MDec(self.lengths@, old(br).concat(), old(br).position())
                        == (m::Decoded::TruncatedZlib { position: final(br).position() })),
            },
    {
        hide(crate::canonical::table_repr);
        hide(m::decode_prefix);
        let ghost lengths = self.lengths@;
        let ghost kind = crate::classification::model_kind(self.kind@);
        let ghost single_accept = self.single_accept@;
        let ghost start = br.position();
        let ghost z = br.concat();
        proof {
            br.lemma_bounds();
            m::lemma_initial_prefix(lengths, kind, single_accept);
        }
        if self.nsyms == 0 {
            proof { m::lemma_decode_prefix_errors(lengths, z, start, Seq::empty()); }
            return Err(InflateError::InvalidSymbol);
        }
        let mut code = 0u32;
        let mut n = 0usize;
        let ghost mut bits: Seq<bool> = Seq::empty();
        while n < MAX_CODE_BITS
            invariant
                self.wf(), self.ready@, self.repr(),
                lengths == self.lengths@, kind == crate::classification::model_kind(self.kind@),
                single_accept == self.single_accept@, m::admissible(lengths, kind, single_accept),
                br.wf(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
                start <= 8 * z.len(), br.position() == start + n, br.position() <= 8 * z.len(),
                w.wf(), w.core >= old(w).core, w.core <= old(w).core + 3 * n,
                old(w).observed@ <= w.observed@ <= old(w).observed@ + 3 * n,
                n <= MAX_CODE_BITS, code < m::pow2(n as nat), bits == m::msb_bits(code as nat, n as nat),
                bits.len() == n, m::live(lengths, bits), forall|s: int| !m::exact(lengths, bits, s),
                m::decode_prefix(lengths, z, start, bits) == m::MDec(lengths, z, start),
            decreases MAX_CODE_BITS - n,
        {
            w.add(1);
            let bit = match br.get(1, w) {
                Ok(bit) => bit,
                Err(e) => {
                    proof {
                        assert(start + bits.len() == 8 * z.len());
                        m::lemma_decode_prefix_errors(lengths, z, start, bits);
                    }
                    return Err(e);
                }
            };
            proof {
                m::lemma_field_one(z, start + n as nat);
                m::lemma_decode_prefix_step(lengths, z, start, bits);
                m::lemma_msb_push(code as nat, n as nat, bit as nat);
                m::lemma_pow2_monotone((n + 1) as nat, MAX_CODE_BITS as nat);
                huffman_format_ok();
                assert(bit <= 1);
                assert(2 * (code as nat) + bit < m::pow2((n + 1) as nat));
                bits = bits.push(bit == 1);
            }
            code = code * 2 + bit;
            n += 1;
            let first = self.first[n];
            let count = self.count[n] as u32;
            proof { crate::canonical::lemma_lookup_bounds(lengths, self.count@, self.first@,
                self.offset@, self.symbol@, n as nat); }
            let found = crate::canonical::lookup(&self.count, &self.first, &self.offset, &self.symbol, n, code);
            if let Some(symbol) = found {
                proof {
                    crate::canonical::lemma_lookup_word(lengths, self.count@, self.first@,
                        self.offset@, self.symbol@, n as nat, code as nat, symbol);
                    m::lemma_msb_bits_len(code as nat, n as nat);
                    m::lemma_decode_prefix_hit(lengths, z, start, bits, symbol as int);
                }
                return Ok(symbol);
            }
            proof {
                crate::canonical::lemma_dead_prefix_representation(lengths, self.count@, self.first@,
                    self.offset@, self.symbol@, self.live@, kind, single_accept, n as nat, code as nat);
            }
            if code < first || code - first - count >= self.live[n] {
                proof { m::lemma_decode_prefix_errors(lengths, z, start, bits); }
                return Err(InflateError::InvalidSymbol);
            }
        }
        proof {
            m::lemma_live_without_exact(lengths, bits);
            let s = choose|s: int| 0 <= s < lengths.len() && m::cw(lengths, s).is_some()
                && bits.len() < lengths[s] && m::prefix(bits, m::cw(lengths, s).unwrap());
            assert(lengths[s] <= MAX_CODE_BITS);
            assert(false);
        }
        Err(InflateError::InvalidSymbol)
    }
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_nonoversubscribed_vectors_still_get_canonical_representation() {
        let mut w = Work::default();
        let mut h = Huffman::new(&mut w);
        assert_eq!(
            h.build(&[2, 2], Kind::LitLen, &mut w),
            Err(InflateError::InvalidCodeLengths)
        );
        assert_eq!(h.count[2], 2);
        assert_eq!(h.first[2], 0);
        assert_eq!(h.offset[2], 0);
        assert_eq!(&h.symbol[..2], &[0, 1]);
        assert_eq!(h.live[1], 1);
    }

    #[test]
    fn oversubscription_is_rejected_before_canonical_representation() {
        let mut w = Work::default();
        let mut h = Huffman::new(&mut w);
        h.first[1] = 123;
        assert_eq!(
            h.build_canonical(&[1, 1, 1], &mut w),
            Err(InflateError::InvalidCodeLengths)
        );
        assert_eq!(h.first[1], 123);
    }

    #[test]
    fn both_single_code_admission_choices_use_the_production_constructor() {
        for accept in [false, true] {
            let mut w = Work::default();
            let mut h = Huffman::new(&mut w);
            assert_eq!(
                h.build_with_single(&[1], Kind::LitLen, accept, &mut w),
                if accept {
                    Ok(())
                } else {
                    Err(InflateError::InvalidCodeLengths)
                }
            );
            assert_eq!(
                h.build_with_single(&[1, 1], Kind::LitLen, accept, &mut w),
                Ok(())
            );
            assert_eq!(
                h.build_with_single(&[1], Kind::Dist, accept, &mut w),
                Ok(())
            );
            assert_eq!(h.build_with_single(&[], Kind::Dist, accept, &mut w), Ok(()));
        }
    }

    #[test]
    fn bounded_exhaustion_matches_independent_word_assignment() {
        // All 15,625 length vectors of six entries with lengths 0..4. The
        // reference visits symbols by length and assigns successive words;
        // it never reads the executable's counts, offsets or first-code array.
        for encoded in 0..5usize.pow(6) {
            let mut lengths = [0u8; 6];
            let mut value = encoded;
            for len in &mut lengths {
                *len = (value % 5) as u8;
                value /= 5;
            }
            let kraft: u32 = lengths
                .iter()
                .filter(|&&l| l != 0)
                .map(|&l| 1 << (MAX_CODE_BITS - usize::from(l)))
                .sum();
            let mut w = Work::default();
            let mut h = Huffman::new(&mut w);
            let result = h.build_canonical(&lengths, &mut w);
            if kraft > CODE_SPACE {
                assert_eq!(result, Err(InflateError::InvalidCodeLengths));
                continue;
            }
            assert_eq!(result, Ok(kraft));
            let mut word = 0u32;
            let mut index = 0;
            for n in 1..=MAX_CODE_BITS {
                let mut rank = 0;
                for (symbol, &len) in lengths.iter().enumerate() {
                    if usize::from(len) == n {
                        assert_eq!(h.first[n] + rank, word);
                        assert_eq!(usize::from(h.offset[n]) + rank as usize, index);
                        assert_eq!(h.symbol[index] as usize, symbol);
                        word += 1;
                        index += 1;
                        rank += 1;
                    }
                }
                assert_eq!(u32::from(h.count[n]), rank);
                word *= 2;
            }
            assert_eq!(index, h.nsyms);
        }
    }
}
