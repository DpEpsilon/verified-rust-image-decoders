//! Borrowed segmented input with the actual T-SEG read contract.
use crate::limits::{C_SEG, SEGMENT_BYTES_MAX};
use vstd::prelude::*;

verus! {

/// Concatenation of a prefix of the borrowed segment sequence.
pub open spec fn parts_prefix(parts: Seq<&[u8]>, k: nat) -> Seq<u8>
    decreases k,
{
    if k == 0 || k > parts.len() { Seq::empty() }
    else { parts_prefix(parts, (k - 1) as nat) + parts[(k - 1) as int]@ }
}

pub proof fn lemma_parts_monotone(parts: Seq<&[u8]>, a: nat, b: nat)
    requires a <= b <= parts.len(),
    ensures parts_prefix(parts, a).len() <= parts_prefix(parts, b).len(),
    decreases b - a,
{
    if a < b { lemma_parts_monotone(parts, a, (b - 1) as nat); }
}

pub proof fn lemma_parts_byte(parts: Seq<&[u8]>, index: nat, j: nat, k: nat)
    requires index < k <= parts.len(), j < parts[index as int]@.len(),
    ensures parts_prefix(parts, index).len() + j < parts_prefix(parts, k).len(),
        parts_prefix(parts, k)[(parts_prefix(parts, index).len() + j) as int]
            == parts[index as int]@[j as int],
    decreases k - index,
{
    if k > index + 1 { lemma_parts_byte(parts, index, j, (k - 1) as nat); }
}

/// Work observations are mathematical counters, not reset preconditions.
/// advance_rank accounts for the optional one-time final EOF advance.
pub trait Segments {
    #[cfg(verus_only)]
    type Frame;
    spec fn concat(&self) -> Seq<u8>;
    spec fn pos(&self) -> nat;
    spec fn seg_index(&self) -> nat;
    spec fn seg_count(&self) -> nat;
    spec fn seg_steps(&self) -> nat;
    spec fn core_steps(&self) -> nat;
    spec fn advance_rank(&self) -> nat;
    spec fn state_wf(&self) -> bool;
    /// Implementation-selected non-cursor state. This value frame does not
    /// quantify over mutable referents owned by an arbitrary implementation.
    spec fn source_frame(&self) -> Self::Frame;
    open spec fn wf(&self) -> bool {
        self.state_wf() && self.pos() <= self.concat().len()
        && self.concat().len() <= SEGMENT_BYTES_MAX
        && self.seg_index() <= self.seg_count()
        && self.seg_index() <= self.advance_rank() <= self.seg_index() + 1
        && self.advance_rank() <= self.seg_count() + 1
    }

    /// Implementations prove the wf laws; a default spec body alone can be
    /// overridden and cannot supply these facts to a generic consumer.
    proof fn lemma_wf(&self)
        requires self.wf(),
        ensures self.pos() <= self.concat().len(), self.concat().len() <= SEGMENT_BYTES_MAX,
            self.seg_index() <= self.seg_count(),
            self.seg_index() <= self.advance_rank() <= self.seg_index() + 1,
            self.advance_rank() <= self.seg_count() + 1,
    ;

    fn read(&mut self) -> (r: Option<u8>)
        requires old(self).wf(),
        ensures
            final(self).wf(), final(self).concat() == old(self).concat(),
            final(self).source_frame() == old(self).source_frame(),
            final(self).seg_count() == old(self).seg_count(),
            final(self).pos() <= final(self).concat().len(),
            final(self).seg_index() <= final(self).seg_count(),
            final(self).concat().len() <= SEGMENT_BYTES_MAX,
            old(self).seg_index() <= final(self).seg_index(),
            old(self).advance_rank() <= final(self).advance_rank() <= final(self).seg_count() + 1,
            old(self).seg_steps() <= final(self).seg_steps(),
            final(self).seg_steps() - old(self).seg_steps()
                == final(self).advance_rank() - old(self).advance_rank(),
            final(self).seg_steps() - old(self).seg_steps() <= C_SEG * (old(self).seg_count() + 1),
            old(self).core_steps() <= final(self).core_steps() <= old(self).core_steps() + 1,
            // A first EOF returns from its segment-advance iteration, not a core iteration.
            final(self).core_steps() == old(self).core_steps()
                + if r.is_some() || old(self).advance_rank() == old(self).seg_count() + 1 { 1nat } else { 0nat },
            r.is_none() ==> final(self).seg_index() == final(self).seg_count()
                && final(self).advance_rank() == final(self).seg_count() + 1,
            if old(self).pos() < old(self).concat().len() {
                r == Some(old(self).concat()[old(self).pos() as int])
                    && final(self).pos() == old(self).pos() + 1
            } else { r.is_none() && final(self).pos() == old(self).pos() },
    ;

}

pub struct SegmentSlices<'a> {
    parts: &'a [&'a [u8]],
    index: usize,
    offset: usize,
    position: u64,
    end_reported: bool,
    seg_work: Ghost<nat>,
    read_work: Ghost<nat>,
}

impl<'a> SegmentSlices<'a> {
    /// Complete runtime non-cursor equality for this concrete representation.
    pub closed spec fn preserves_noncursor(&self, before: Self) -> bool {
        self.parts == before.parts
    }

    /// Traversal precedes inflate and does not enter inflate's work counters.
    pub fn new(parts: &'a [&'a [u8]]) -> (r: Option<Self>)
        ensures match r {
            Some(s) => s.wf() && Segments::pos(&s) == 0
                && s.concat() == parts_prefix(parts@, parts@.len())
                && Segments::seg_count(&s) == parts.len()
                && Segments::seg_steps(&s) == 0 && s.core_steps() == 0,
            None => parts_prefix(parts@, parts@.len()).len() > SEGMENT_BYTES_MAX,
        },
    {
        let mut total = 0u64;
        let mut k = 0usize;
        while k < parts.len()
            invariant k <= parts.len(), total <= SEGMENT_BYTES_MAX,
                total == parts_prefix(parts@, k as nat).len(),
            decreases parts.len() - k,
        {
            let part = parts[k];
            let len = part.len() as u64;
            if len > SEGMENT_BYTES_MAX - total {
                proof { lemma_parts_monotone(parts@, (k + 1) as nat, parts@.len()); }
                return None;
            }
            total += len;
            k += 1;
        }
        let ghost zero: nat = 0;
        Some(Self { parts, index: 0, offset: 0, position: 0,
            end_reported: false, seg_work: Ghost(zero), read_work: Ghost(zero) })
    }
}

impl<'a> Segments for SegmentSlices<'a> {
    #[cfg(verus_only)]
    type Frame = &'a [&'a [u8]];
    closed spec fn concat(&self) -> Seq<u8> { parts_prefix(self.parts@, self.parts@.len()) }
    closed spec fn pos(&self) -> nat { self.position as nat }
    closed spec fn seg_index(&self) -> nat { self.index as nat }
    closed spec fn seg_count(&self) -> nat { self.parts@.len() }
    closed spec fn seg_steps(&self) -> nat { self.seg_work@ }
    closed spec fn core_steps(&self) -> nat { self.read_work@ }
    closed spec fn advance_rank(&self) -> nat { self.index as nat + if self.end_reported { 1nat } else { 0nat } }
    /// All runtime non-cursor state is this complete shared input borrow.
    closed spec fn source_frame(&self) -> Self::Frame { self.parts }
    closed spec fn state_wf(&self) -> bool {
        self.index <= self.parts.len()
        && (if self.index < self.parts.len() { self.offset <= self.parts@[self.index as int].len() }
            else { self.offset == 0 })
        && self.position == parts_prefix(self.parts@, self.index as nat).len() + self.offset
        && parts_prefix(self.parts@, self.parts@.len()).len() <= SEGMENT_BYTES_MAX
        && (self.end_reported ==> self.index == self.parts.len())
        && self.seg_work@ == self.index + if self.end_reported { 1nat } else { 0nat }
    }

    proof fn lemma_wf(&self) {}

    // L-SEG-FRAME(iv): parts is the only non-cursor runtime field. The
    // enumerated cursor fields are index, offset, position and end_reported;
    // seg_work/read_work are erased observations. This postcondition covers
    // Some and None returns; absence of other effects also uses type inspection
    // and Rust's aliasing rules, not a generic heap-effect theorem.
    fn read(&mut self) -> (r: Option<u8>)
        ensures final(self).preserves_noncursor(*old(self)),
    {
        let ghost initial = *self;
        loop
            invariant
                self.wf(), initial.wf(), initial == *old(self), self.parts == initial.parts,
                self.position == initial.position, self.read_work@ == initial.read_work@,
                self.index >= initial.index, self.end_reported == initial.end_reported,
                self.seg_work@ - initial.seg_work@ == self.advance_rank() - initial.advance_rank(),
                self.advance_rank() >= initial.advance_rank(),
            decreases self.parts.len() - self.index,
        {
            if self.index == self.parts.len() {
                if !self.end_reported {
                    self.end_reported = true;
                    proof { self.seg_work = Ghost(self.seg_work@ + 1); }
                } else {
                    proof { self.read_work = Ghost(self.read_work@ + 1); }
                }
                return None;
            }
            let part = self.parts[self.index];
            if self.offset < part.len() {
                proof { lemma_parts_byte(self.parts@, self.index as nat, self.offset as nat, self.parts@.len()); }
                let byte = part[self.offset];
                self.offset += 1;
                self.position += 1;
                proof { self.read_work = Ghost(self.read_work@ + 1); }
                return Some(byte);
            }
            self.index += 1;
            self.offset = 0;
            proof { self.seg_work = Ghost(self.seg_work@ + 1); }
        }
    }
}

// These executable verifier witnesses pin the exact partition, including repeated EOF.
#[cfg(verus_only)]
fn zero_segments_accounting() {
    let parts: [&[u8]; 0] = [];
    proof { assert(parts_prefix(parts@, 0) == Seq::<u8>::empty()); }
    let mut src = match SegmentSlices::new(&parts) {
        Some(src) => src,
        None => { proof { assert(false); } return; },
    };
    let first = src.read();
    proof { assert(first.is_none()); assert(src.core_steps() == 0); assert(src.seg_steps() == 1); }
    let second = src.read();
    proof { assert(second.is_none()); assert(src.core_steps() == 1); assert(src.seg_steps() == 1); }
}

#[cfg(verus_only)]
fn one_empty_segment_accounting() {
    let empty: [u8; 0] = [];
    let parts: [&[u8]; 1] = [&empty];
    proof { reveal_with_fuel(parts_prefix, 2); assert(parts_prefix(parts@, 1) =~= Seq::<u8>::empty()); }
    let mut src = match SegmentSlices::new(&parts) {
        Some(src) => src,
        None => { proof { assert(false); } return; },
    };
    let first = src.read();
    proof { assert(first.is_none()); assert(src.core_steps() == 0); assert(src.seg_steps() == 2); }
    let second = src.read();
    proof { assert(second.is_none()); assert(src.core_steps() == 1); assert(src.seg_steps() == 2); }
}

// Cursor accessors remain executable; the work observation is erased.
impl SegmentSlices<'_> {
    pub fn pos(&self) -> (r: u64)
        ensures r == Segments::pos(self),
    { self.position }
    pub fn seg_index(&self) -> (r: usize)
        ensures r == Segments::seg_index(self),
    { self.index }
    pub fn seg_count(&self) -> (r: usize)
        ensures r == Segments::seg_count(self),
    { self.parts.len() }
    pub fn work(&self) -> (r: Ghost<nat>)
        ensures r@ == Segments::seg_steps(self),
    { Ghost(self.seg_work@) }
}

} // verus!
