use crate::{bits::Bits, limits::*, InflateError, Segments, Work};

pub(crate) use crate::classification::Kind;

/// Counts, first canonical code, first symbol index, and symbol order. Only
/// symbols in 0..nsyms are readable; rebuilding never clears unused symbol slots.
pub(crate) struct Huffman {
    count: [u16; MAX_CODE_BITS + 1],
    first: [u32; MAX_CODE_BITS + 1],
    offset: [u16; MAX_CODE_BITS + 1],
    symbol: [u16; FIXED_LIT_COUNT],
    live: [u32; MAX_CODE_BITS + 1],
    nsyms: usize,
}
impl Huffman {
    /// Dynamic acceptance is separate from representation construction: every
    /// non-oversubscribed vector receives canonical tables, even if this kind
    /// subsequently rejects the vector as incomplete.
    pub fn build(&mut self, lengths: &[u8], kind: Kind, w: &mut Work) -> Result<(), InflateError> {
        let kraft = self.build_canonical(lengths, w)?;
        let admissible =
            crate::classification::is_admissible(kind, kraft, self.nsyms, self.count[1], true);
        if admissible {
            Ok(())
        } else {
            Err(InflateError::InvalidCodeLengths)
        }
    }

    pub fn new(w: &mut Work) -> Self {
        w.add(4 * (MAX_CODE_BITS + 1) + FIXED_LIT_COUNT);
        Self {
            count: [0; MAX_CODE_BITS + 1],
            first: [0; MAX_CODE_BITS + 1],
            offset: [0; MAX_CODE_BITS + 1],
            symbol: [0; FIXED_LIT_COUNT],
            live: [0; MAX_CODE_BITS + 1],
            nsyms: 0,
        }
    }
    fn build_canonical(&mut self, lengths: &[u8], w: &mut Work) -> Result<u32, InflateError> {
        let (built, _work) = crate::canonical::build_tables(
            lengths,
            &mut self.count,
            &mut self.first,
            &mut self.offset,
            &mut self.symbol,
        );
        // M1a diagnostics are outside the erased proof-counter surface.
        let (kraft, nonzero) = match built {
            Ok(value) => {
                w.add(CANON_FIXED_STEPS + 2 * lengths.len());
                value
            }
            Err(error) => {
                match error {
                    crate::canonical::BuildError::TooManySymbols => {}
                    crate::canonical::BuildError::InvalidLength(index) => {
                        w.add(MAX_CODE_BITS + 1 + index + 1)
                    }
                    crate::canonical::BuildError::Oversubscribed => {
                        w.add(MAX_CODE_BITS + 1 + lengths.len())
                    }
                }
                return Err(InflateError::InvalidCodeLengths);
            }
        };
        self.nsyms = nonzero;
        // In canonical order the proper prefixes immediately follow length-n
        // codewords. Round the remaining Kraft mass up to n-bit prefixes.
        let mut higher = 0u32;
        let mut n = MAX_CODE_BITS;
        loop {
            w.add(1);
            let unit = 1u32 << (MAX_CODE_BITS - n);
            self.live[n] = higher.div_ceil(unit);
            if n == 0 {
                break;
            }
            higher += u32::from(self.count[n]) * unit;
            n -= 1;
        }
        Ok(kraft)
    }
    pub fn decode<S: Segments>(
        &self,
        br: &mut Bits<'_, S>,
        w: &mut Work,
    ) -> Result<u16, InflateError> {
        if self.nsyms == 0 {
            return Err(InflateError::InvalidSymbol);
        }
        let mut code = 0u32;
        let mut n = 1;
        while n <= MAX_CODE_BITS {
            w.add(1);
            code = (code << 1) | br.get(1, w)?;
            let first = self.first[n];
            let count = u32::from(self.count[n]);
            if let Some(symbol) = crate::canonical::lookup(
                &self.count,
                &self.first,
                &self.offset,
                &self.symbol,
                n,
                code,
            ) {
                return Ok(symbol);
            }
            if code < first || code - first - count >= self.live[n] {
                return Err(InflateError::InvalidSymbol);
            }
            n += 1;
        }
        Err(InflateError::InvalidSymbol)
    }
}

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
