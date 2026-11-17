//! Borrowed segmented input. The mathematical concat/wf contract is documented
//! here at M1a; M2 must encode and prove it as the T-SEG trait specification.
use crate::limits::SEGMENT_BYTES_MAX;

/// Implementations must preserve their segment concatenation throughout a call.
/// A fresh cursor has pos() = seg_index() = seg_steps() = 0. read() returns the
/// next concatenated byte and increments pos(), or None at EOF without changing
/// pos(). It skips empty/exhausted segments, never decreases seg_index(), and
/// keeps seg_index() <= seg_count(). Total length is at most SEGMENT_BYTES_MAX.
///
/// Each read has at most one non-advance loop iteration. seg_steps() counts
/// segment-advance iterations, including at most one final EOF advance, and is
/// bounded by seg_count()+1 across all reads from a fresh cursor. No allocation
/// or recursion is allowed. Public inflate calls require this contract.
pub trait Segments {
    fn read(&mut self) -> Option<u8>;
    fn pos(&self) -> u64;
    fn seg_index(&self) -> usize;
    fn seg_count(&self) -> usize;
    fn seg_steps(&self) -> u128;
}

/// A fresh, allocation-free reader over borrowed segments, empty ones included.
pub struct SegmentSlices<'a> {
    parts: &'a [&'a [u8]],
    index: usize,
    offset: usize,
    position: u64,
    advances: u128,
    end_reported: bool,
}
impl<'a> SegmentSlices<'a> {
    /// None means the concatenation exceeds T-SEG's domain, not a zlib error.
    /// Constructor traversal is outside the inflate call's work counters.
    pub fn new(parts: &'a [&'a [u8]]) -> Option<Self> {
        let mut total = 0u64;
        for part in parts {
            let len = u64::try_from(part.len()).ok()?;
            if len > SEGMENT_BYTES_MAX - total {
                return None;
            }
            total += len;
        }
        Some(Self {
            parts,
            index: 0,
            offset: 0,
            position: 0,
            advances: 0,
            end_reported: false,
        })
    }
}
impl Segments for SegmentSlices<'_> {
    fn read(&mut self) -> Option<u8> {
        loop {
            if self.index == self.parts.len() {
                if !self.end_reported {
                    self.advances += 1;
                    self.end_reported = true;
                }
                return None;
            }
            let part = self.parts[self.index];
            if self.offset < part.len() {
                let byte = part[self.offset];
                self.offset += 1;
                self.position += 1;
                return Some(byte);
            }
            self.advances += 1;
            self.index += 1;
            self.offset = 0;
        }
    }
    fn pos(&self) -> u64 {
        self.position
    }
    fn seg_index(&self) -> usize {
        self.index
    }
    fn seg_count(&self) -> usize {
        self.parts.len()
    }
    fn seg_steps(&self) -> u128 {
        self.advances
    }
}
