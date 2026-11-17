use crate::limits::{ADLER_MOD, ADLER_NMAX};

pub(crate) struct Adler {
    a: u32,
    b: u32,
    pending: usize,
}
impl Adler {
    pub fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            pending: 0,
        }
    }
    pub fn update_byte(&mut self, byte: u8) {
        self.a += u32::from(byte);
        self.b += self.a;
        self.pending += 1;
        if self.pending == ADLER_NMAX {
            self.a %= ADLER_MOD;
            self.b %= ADLER_MOD;
            self.pending = 0;
        }
    }
    pub fn finish(&self) -> u32 {
        ((self.b % ADLER_MOD) << 16) | (self.a % ADLER_MOD)
    }
}
