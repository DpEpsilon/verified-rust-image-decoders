impl Huffman {
    /// Decode on the actual reader; the invariant records the reachable model
    /// prefix before each attempted bit, including failed-lookup liveness.
    pub(crate) fn decode<S: Segments>(&self, br: &mut Bits<'_, S>, w: &mut Work)
        -> (r: Result<u16, InflateError>)
        requires self.wf(), self.ready@, old(br).wf(), old(w).wf(),
            old(w).core + 3 * MAX_CODE_BITS <= u128::MAX,
        ensures final(br).wf(), final(w).wf(), final(br).concat() == old(br).concat(),
            final(w).core >= old(w).core, final(w).core <= old(w).core + 3 * MAX_CODE_BITS,
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
        let ghost lengths = self.lengths@;
        let ghost kind = crate::classification::model_kind(self.kind@);
        let ghost start = br.position();
        let ghost z = br.concat();
        proof {
            br.lemma_bounds();
            m::lemma_initial_prefix(lengths, kind, true);
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
                m::admissible(lengths, kind, true),
                br.wf(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
                start <= 8 * z.len(), br.position() == start + n,
                w.wf(), w.core >= old(w).core, w.core <= old(w).core + 3 * n,
                old(w).core + 3 * MAX_CODE_BITS <= u128::MAX,
                n <= MAX_CODE_BITS, code < m::pow2(n as nat), bits == m::msb_bits(code as nat, n as nat),
                bits.len() == n, m::live(lengths, bits), forall|s: int| !m::exact(lengths, bits, s),
                m::decode_prefix(lengths, z, start, bits) == m::MDec(lengths, z, start),
            decreases MAX_CODE_BITS - n,
        {
            w.add(1);
            let bit = match br.get(1, w) {
                Ok(bit) => bit,
                Err(e) => {
                    proof { m::lemma_decode_prefix_errors(lengths, z, start, bits); }
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
            proof { m::lemma_bucket_bound(lengths, n as nat); }
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
                    self.offset@, self.symbol@, self.live@, kind, true, n as nat, code as nat);
                m::lemma_numeric_exact(lengths, n as nat, code as nat);
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

