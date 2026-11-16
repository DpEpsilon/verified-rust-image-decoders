//! G10 variable-length-code LZW over the image data's sub-blocks.
//!
//! The decoder works on the caller-provided `LzwTable` reservation — G18's one
//! 20,480-byte `try_zeroed(LzwTable, LZW_TABLE_BYTES)` region, viewed as three
//! areas: prefix codes as little-endian `u16` pairs (code `c` at byte `2c`,
//! bytes `[0, 8192)`), suffix bytes (`[8192, 12288)`) and string lengths as
//! little-endian `u16` pairs (code `c` at byte `12288 + 2c`, bytes
//! `[12288, 20480)`); 4,096 entries each. It hands every index to a
//! caller-provided `Sink` with its pixel position, and never allocates.
//!
//! Literal codes are decided by rule (G10.1, decision D20): a code below
//! `clear = 2^m` denotes the single index `[c]`, its suffix is `c` and its
//! length is 1, and the decoder reads no stored byte for it. Entries exist only
//! for codes `>= clear + 2`, so no frame initialises anything and a literal
//! code's stored bytes are unspecified — zero after the reservation, or left
//! from an earlier frame with a smaller `m`.
//!
//! Strings are not materialised. A code's string is the chain
//! `code -> prefix[code] -> ...` read backwards (the chain yields the last index
//! first), and the stored length gives its length, so the decoder emits the
//! indices of a string at positions `q+len-1, ..., q` in that order. The sink is
//! told the position of each index; because every error discards the frame
//! (G12.4), the order of the writes is unobservable, and the decoder reports the
//! error that G10.2/G12.2's forward order gives: the colour error at the
//! smallest position below `A`, else `TooManyPixels` when the string runs past
//! `A`.
//!
//! Table invariant (proved in M2): for every code `c` in `[clear+2, next)`,
//! `prefix[c] < c`, `prefix[c]` is not `clear` or `clear+1`, and
//! `stored_len[c] = code_len(prefix[c]) + 1 <= 4096`, where `code_len(p) = 1`
//! for `p < clear`. Every value written to `prefix` is a code below 4096, so
//! every chain index stays below `TABLE_SIZE`.

use crate::gif::Error;

use vstd::prelude::*;

verus! {

/// Entries of the LZW code table (12-bit codes).
pub const TABLE_SIZE: usize = 4096;

/// G18's `LZW_TABLE_BYTES = 4096*(2 + 1 + 2)`: the one `LzwTable` request.
pub const LZW_TABLE_BYTES: usize = 20_480;

/// Byte offset of the suffix area (prefix codes occupy `[0, 8192)`).
pub const SUFFIX_OFF: usize = 8192;

/// Byte offset of the string-length area (`u16` pairs; `[8192, 12288)` is the
/// suffix area).
pub const LENGTH_OFF: usize = 12_288;

} // verus!


/// Code `c`'s stored prefix link (LE16 at byte `2c`).
fn prefix_get(t: &[u8], c: u32) -> u16 {
    let i = 2 * (c as usize);
    match (t.get(i), t.get(i + 1)) {
        (Some(a), Some(b)) => (*a as u16) | ((*b as u16) << 8),
        _ => 0,
    }
}

/// Store code `c`'s prefix link.
fn prefix_set(t: &mut [u8], c: u32, v: u16) {
    let i = 2 * (c as usize);
    t[i] = v as u8;
    t[i + 1] = (v >> 8) as u8;
}

/// Code `c`'s stored suffix byte.
fn suffix_get(t: &[u8], c: u32) -> u8 {
    match t.get(SUFFIX_OFF + c as usize) {
        Some(v) => *v,
        None => 0,
    }
}

/// Store code `c`'s suffix byte.
fn suffix_set(t: &mut [u8], c: u32, v: u8) {
    t[SUFFIX_OFF + c as usize] = v;
}

/// Code `c`'s stored string length (LE16 at byte `12288 + 2c`).
fn stored_len(t: &[u8], c: u32) -> u16 {
    let i = LENGTH_OFF + 2 * (c as usize);
    match (t.get(i), t.get(i + 1)) {
        (Some(a), Some(b)) => (*a as u16) | ((*b as u16) << 8),
        _ => 0,
    }
}

/// Store code `c`'s string length.
fn stored_len_set(t: &mut [u8], c: u32, v: u16) {
    let i = LENGTH_OFF + 2 * (c as usize);
    t[i] = v as u8;
    t[i + 1] = (v >> 8) as u8;
}

/// G10.1: the string length of code `c`. A literal code's is 1 by rule; every
/// other code's is the stored length, read only for codes `>= clear + 2`.
fn code_len(t: &[u8], clear: u32, c: u32) -> u16 {
    if c < clear {
        1
    } else {
        stored_len(t, c)
    }
}

/// Receives the decoded indices. `put(q, v)` handles index `v` at pixel
/// position `q < A`; it returns `false` for a colour error
/// (`ColorIndexOutOfRange`, G12.2), `true` otherwise.
pub trait Sink {
    fn put(&mut self, q: u32, v: u8) -> bool;
}

/// What happens after the raster is complete (policy field `lzw_tail`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tail {
    /// G10.2 steps 1, 4, 5, 8.
    Strict,
    /// Stop when `q = A`; indices past `A` are discarded (event E4).
    StopAtRaster,
}

/// Options of one LZW call that come from the policy record.
#[derive(Clone, Copy)]
pub struct LzwOptions {
    pub tail: Tail,
    /// `lzw_min = Range1to11Checked`: a literal code in `256..clear` is
    /// `ColorIndexOutOfRange` when read.
    pub checked_literals: bool,
}

/// LSB-first bit reader over a data sub-block sequence whose structure was
/// already checked by `parse::scan_image_data` (G9): the sequence starts with a
/// size byte at `pos`, holds `data_len` payload bytes, and ends with a zero size
/// byte. `bits_left` counts the bits of the payload not yet consumed; a read is
/// made only when `bits_left >= width`, so it never needs a byte past the
/// payload. A byte that is not there reads as 0 (unreachable after the scan;
/// the fallback keeps every access in bounds without relying on it).
pub struct BitReader<'a> {
    b: &'a [u8],
    /// Index in `b` of the next byte to fetch (payload byte or size byte).
    pos: usize,
    /// Payload bytes left in the current sub-block.
    left_in_block: usize,
    acc: u32,
    nacc: u32,
    bits_left: u32,
    total_bits: u32,
}

impl<'a> BitReader<'a> {
    /// `first` is the index of the first sub-block's size byte; `data_len` the
    /// payload length (`|D|`, at most `MAX_INPUT`, so `8·|D|` fits in `u32`).
    pub fn new(b: &'a [u8], first: usize, data_len: u32) -> BitReader<'a> {
        let total_bits = data_len.saturating_mul(8);
        BitReader { b, pos: first, left_in_block: 0, acc: 0, nacc: 0, bits_left: total_bits, total_bits }
    }

    fn fetch_byte(&mut self) -> u8 {
        if self.left_in_block == 0 {
            let size = match self.b.get(self.pos) {
                Some(s) => *s,
                None => 0,
            };
            self.pos = self.pos.saturating_add(1);
            self.left_in_block = size as usize;
        }
        let v = match self.b.get(self.pos) {
            Some(x) => *x,
            None => 0,
        };
        self.pos = self.pos.saturating_add(1);
        if self.left_in_block > 0 {
            self.left_in_block -= 1;
        }
        v
    }

    /// Read `width` bits (`1 <= width <= 12`); requires `bits_left >= width`.
    fn read(&mut self, width: u32) -> u32 {
        while self.nacc < width {
            let byte = self.fetch_byte() as u32;
            self.acc |= byte << self.nacc;
            self.nacc += 8;
        }
        let code = self.acc & ((1u32 << width) - 1);
        self.acc >>= width;
        self.nacc -= width;
        self.bits_left -= width;
        code
    }

    /// Bit position after the codes read so far (`e` in G10.2 step 5).
    fn bit_pos(&self) -> u32 {
        self.total_bits - self.bits_left
    }
}

/// Decode one frame's image data (G10) with minimum code size `m`
/// (`1 <= m <= 11`; the parser has checked the policy's range) into `area`
/// pixel positions, using the `LzwTable` reservation `t`. Returns
/// `Ok(overrun)`, where `overrun` is event E4 (only under
/// `Tail::StopAtRaster`), or the first error of G10.2/G12.2.
pub fn decode<S: Sink>(
    reader: &mut BitReader,
    data_len: u32,
    m: u32,
    area: u32,
    opts: LzwOptions,
    t: &mut [u8],
    sink: &mut S,
) -> Result<bool, Error> {
    if m < 1 || m > 11 {
        return Err(Error::InvalidLzwMinCodeSize);
    }
    let clear: u32 = 1u32 << m;
    let eoi: u32 = clear + 1;
    // G10.1: no per-frame initialisation — codes below `clear` are literals by
    // rule, and the table holds entries only for codes `>= clear + 2`.
    let mut next: u32 = clear + 2;
    let mut width: u32 = m + 1;
    let mut have_prev = false;
    let mut prev: u32 = 0;
    let mut prev_first: u8 = 0;
    let mut q: u32 = 0;
    loop {
        if opts.tail == Tail::StopAtRaster && q == area {
            return Ok(false);
        }
        // Step 1.
        if reader.bits_left < width {
            if q < area {
                return Err(Error::TooFewPixels);
            }
            return Err(Error::MissingEndOfInformation);
        }
        // Step 2.
        let code = reader.read(width);
        // Step 3.
        if code == clear {
            next = clear + 2;
            width = m + 1;
            have_prev = false;
            continue;
        }
        // Step 4.
        if q == area && code != eoi {
            return Err(Error::MissingEndOfInformation);
        }
        // Step 5.
        if code == eoi {
            if q < area {
                return Err(Error::TooFewPixels);
            }
            let e = reader.bit_pos();
            if (e / 8) + (if e % 8 != 0 { 1 } else { 0 }) < data_len {
                return Err(Error::DataAfterEndOfInformation);
            }
            return Ok(false);
        }
        if opts.checked_literals && code < clear && code >= 256 {
            return Err(Error::ColorIndexOutOfRange);
        }
        // Step 6. `code < next` excludes nothing but clear/eoi (handled above).
        let kwkwk;
        if code < next {
            kwkwk = false;
        } else if code == next && have_prev && next < TABLE_SIZE as u32 {
            kwkwk = true;
        } else {
            return Err(Error::InvalidLzwCode);
        }
        // Step 7 for the KwKwK case: the new entry is `code` itself and is
        // needed to emit it, so it is added first (its suffix is first(prev)).
        if kwkwk {
            add_entry(t, &mut next, &mut width, prev, prev_first, clear);
        }
        // Step 8: emit the string of `code` at positions q .. q+len-1.
        let len = if code < clear { 1u32 } else { code_len(t, clear, code) as u32 };
        let mut cur = code;
        let mut j = len;
        let mut bad = false;
        let mut first: u8 = 0;
        while j > 0 {
            j -= 1;
            let v = if cur < clear { cur as u8 } else { suffix_get(t, cur) };
            first = v;
            // q + j < q + len <= A + 4096, no overflow: q <= A <= 2^30.
            let pos = q + j;
            if pos < area && !sink.put(pos, v) {
                bad = true; // later (smaller) positions overwrite: the earliest wins
            }
            if j > 0 {
                cur = prefix_get(t, cur) as u32;
            }
        }
        if bad {
            return Err(Error::ColorIndexOutOfRange);
        }
        if q + len > area {
            match opts.tail {
                Tail::Strict => return Err(Error::TooManyPixels),
                Tail::StopAtRaster => return Ok(true),
            }
        }
        // Step 7 for the ordinary case: entry next = prev + first(s).
        if !kwkwk && have_prev && next < TABLE_SIZE as u32 {
            add_entry(t, &mut next, &mut width, prev, first, clear);
        }
        q += len;
        have_prev = true;
        prev = code;
        prev_first = first;
    }
}

/// G10.2 step 7: add entry `next = prev + [s0]`; widen when `next` reaches
/// `2^width` below width 12. Requires `next < 4096` and, for `prev >= clear`,
/// `prev < next`. The new length is `code_len(prev) + 1`: a literal prefix
/// (`prev < clear`) contributes 1 by rule, not a stored byte.
fn add_entry(t: &mut [u8], next: &mut u32, width: &mut u32, prev: u32, s0: u8, clear: u32) {
    let n = *next as usize;
    if n >= TABLE_SIZE {
        return;
    }
    prefix_set(t, *next, prev as u16);
    suffix_set(t, *next, s0);
    let plen = code_len(t, clear, prev);
    stored_len_set(t, *next, plen.saturating_add(1));
    *next += 1;
    if *next == (1u32 << *width) && *width < 12 {
        *width += 1;
    }
}

