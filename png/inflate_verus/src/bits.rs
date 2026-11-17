use crate::{InflateError, Segments, Work};

/// One partially consumed byte; no u32 bit position or input-length product.
/// The mathematical bit position is 8*src.pos()-remaining (M2 obligation).
pub(crate) struct Bits<'a, S: Segments> {
    pub src: &'a mut S,
    current: u8,
    remaining: u8,
}
impl<'a, S: Segments> Bits<'a, S> {
    pub fn new(src: &'a mut S) -> Self {
        Self {
            src,
            current: 0,
            remaining: 0,
        }
    }
    pub fn byte(&mut self, w: &mut Work) -> Result<u8, InflateError> {
        // The reader's one non-advance iteration belongs to core_steps.
        w.add(1);
        self.src.read().ok_or(InflateError::TruncatedZlib)
    }
    pub fn get(&mut self, n: u8, w: &mut Work) -> Result<u32, InflateError> {
        let mut value = 0u32;
        let mut i = 0u8;
        while i < n {
            w.add(1);
            if self.remaining == 0 {
                self.current = self.byte(w)?;
                self.remaining = 8;
            }
            value |= u32::from(self.current & 1) << i;
            self.current >>= 1;
            self.remaining -= 1;
            i += 1;
        }
        Ok(value)
    }
    pub fn align(&mut self) {
        self.current = 0;
        self.remaining = 0;
    }
}
