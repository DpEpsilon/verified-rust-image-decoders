//! `Segments` readers over the borrowed input (T-SEG, L-REWALK, L-SEG-PNG):
//! the image data of P5 (the IDAT run) and of one APNG frame (its fdAT
//! payloads after their sequence numbers, P14.1) are re-walked by offset from
//! the chunk framing the structural walk already validated. No chunk list is
//! kept (SPEC P20). Every chunk visited, empty or not, occupies at least 12
//! bytes of the input, which is what pays for the segment advances.
//!
//! L-SEG-FRAME (ERRATA E2, condition (i)): each reader holds one shared borrow
//! of the input and plain cursor integers; no `&mut`, raw pointer, interior
//! mutability or I/O. The cursor fields are `pos`, `end`, `next` and `done`.

use inflate_verus::Segments;

use crate::walk::{be32, crc_matches, frame_at};

/// The IDAT run starting at the chunk at `first` (an IDAT chunk). The run ends
/// at the first chunk that is not IDAT, or at the end of the input
/// (`missing_iend = AtEof`).
pub(crate) struct IdatReader<'a> {
    b: &'a [u8],
    /// Next payload byte to return.
    pos: usize,
    /// End of the current chunk's payload.
    end: usize,
    /// Offset of the chunk after the current one.
    next: usize,
    done: bool,
}

impl<'a> IdatReader<'a> {
    pub(crate) fn new(b: &'a [u8], first: usize) -> IdatReader<'a> {
        let mut r = IdatReader { b, pos: 0, end: 0, next: first, done: false };
        r.enter();
        r
    }

    /// Move to the chunk at `next` if it is an IDAT chunk; otherwise finish.
    fn enter(&mut self) -> bool {
        match frame_at(self.b, self.next) {
            Ok(c) if &c.ty == b"IDAT" => {
                self.pos = c.data;
                self.end = c.data + c.len;
                self.next = c.next;
                true
            }
            _ => {
                self.done = true;
                false
            }
        }
    }
}

impl<'a> Segments for IdatReader<'a> {
    fn read(&mut self) -> Option<u8> {
        loop {
            if self.done {
                return None;
            }
            if self.pos < self.end {
                let x = self.b[self.pos];
                self.pos += 1;
                return Some(x);
            }
            if !self.enter() {
                return None;
            }
        }
    }
}

/// The data of one fdAT-based frame: the payloads (after the 4-byte sequence
/// number) of the fdAT chunks from the chunk at `start` up to the next fcTL
/// whose CRC matches, or IEND, or the end of the input. Other chunks are
/// skipped; fdAT chunks of one frame need not be adjacent (P14.1). A fcTL with
/// a bad CRC was discarded by the walk (`crc_ancillary = SkipExceptFdat`), so it
/// does not end the frame here either.
pub(crate) struct FdatReader<'a> {
    b: &'a [u8],
    pos: usize,
    end: usize,
    next: usize,
    done: bool,
}

/// Whether the chunk at `p` ends a frame's data, and its framing.
pub(crate) fn frame_boundary(b: &[u8], p: usize) -> Option<(bool, crate::walk::Chunk)> {
    match frame_at(b, p) {
        Ok(c) => {
            let stop = &c.ty == b"IEND" || (&c.ty == b"fcTL" && crc_matches(b, &c));
            Some((stop, c))
        }
        Err(_) => None,
    }
}

impl<'a> FdatReader<'a> {
    pub(crate) fn new(b: &'a [u8], start: usize) -> FdatReader<'a> {
        FdatReader { b, pos: 0, end: 0, next: start, done: false }
    }

    /// Advance to the next fdAT chunk of this frame; finish at a boundary.
    fn advance(&mut self) -> bool {
        loop {
            match frame_boundary(self.b, self.next) {
                Some((false, c)) => {
                    self.next = c.next;
                    if &c.ty == b"fdAT" && c.len >= 4 {
                        // Sequence number checked by the walk; the payload follows it.
                        let _seq = be32(self.b, c.data);
                        self.pos = c.data + 4;
                        self.end = c.data + c.len;
                        return true;
                    }
                }
                _ => {
                    self.done = true;
                    return false;
                }
            }
        }
    }
}

impl<'a> Segments for FdatReader<'a> {
    fn read(&mut self) -> Option<u8> {
        loop {
            if self.done {
                return None;
            }
            if self.pos < self.end {
                let x = self.b[self.pos];
                self.pos += 1;
                return Some(x);
            }
            if !self.advance() {
                return None;
            }
        }
    }
}

/// The offset of the next fcTL with a matching CRC at or after `p`, before
/// IEND; `None` when IEND or the end of the input comes first.
pub(crate) fn next_fctl(b: &[u8], mut p: usize) -> Option<usize> {
    loop {
        match frame_at(b, p) {
            Ok(c) => {
                if &c.ty == b"IEND" {
                    return None;
                }
                if &c.ty == b"fcTL" && crc_matches(b, &c) {
                    return Some(p);
                }
                p = c.next;
            }
            Err(_) => return None,
        }
    }
}
