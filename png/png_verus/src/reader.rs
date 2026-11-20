//! Borrowed lengths-and-types re-walks. The verified adapter implements M2's
//! complete Segments contract; equality with MW's selected ranges remains a
//! separate L-REWALK obligation. No re-walk recomputes a CRC.
use vstd::prelude::*;
use inflate_verus::Segments;
#[cfg(verus_only)]
use inflate_verus::limits::SEGMENT_BYTES_MAX;
use crate::framing::frame_at;
#[cfg(verus_only)]
use crate::model::{self, reader_bytes, reader_count};

verus! {

fn type_is(t: &[u8; 4], a: u8, b: u8, c: u8, d: u8) -> (r: bool)
    ensures r == (t@ == seq![a, b, c, d]),
{
    let r = t[0] == a && t[1] == b && t[2] == c && t[3] == d;
    proof { if r { assert(t@ =~= seq![a, b, c, d]); } }
    r
}

/// One step of the re-walk. A non-fdAT chunk inside a frame contributes an
/// empty segment, so traversing that chunk is still counted as segment work.
fn segment_at(b: &[u8], p: usize, fdat: bool) -> (r: Option<(usize, usize, usize)>)
    ensures match r {
        None => model::reader_chunk(b@, p as nat, fdat).is_none(),
        Some((begin, end, next)) => p + 8 <= begin <= end && end + 4 <= next <= b.len()
            && p + 12 <= next
            && match model::reader_chunk(b@, p as nat, fdat) {
                None => false,
                Some(c) => c.begin == begin && c.end == end && c.next == next,
            },
    },
{
    let c = match frame_at(b, p) { Ok(c) => c, Err(_) => return None };
    if fdat {
        if type_is(&c.ty, b'I', b'E', b'N', b'D') || type_is(&c.ty, b'f', b'c', b'T', b'L') {
            None
        } else if type_is(&c.ty, b'f', b'd', b'A', b'T') && c.len >= 4 {
            Some((c.data + 4, c.data + c.len, c.next))
        } else { Some((c.data, c.data, c.next)) }
    } else if type_is(&c.ty, b'I', b'D', b'A', b'T') {
        Some((c.data, c.data + c.len, c.next))
    } else { None }
}

/// All runtime non-cursor state is (b, start, fdat). Cursor fields are pos,
/// end, next and done; the remaining fields are erased proof observations.
pub(crate) struct DataReader<'a> {
    b: &'a [u8],
    start: usize,
    fdat: bool,
    pos: usize,
    end: usize,
    next: usize,
    done: bool,
    source: Ghost<Seq<u8>>,
    prefix: Ghost<Seq<u8>>,
    total: Ghost<nat>,
    entered: Ghost<nat>,
    reads: Ghost<nat>,
}

pub(crate) type IdatReader<'a> = DataReader<'a>;
pub(crate) type FdatReader<'a> = DataReader<'a>;

impl<'a> DataReader<'a> {
    fn at(b: &'a [u8], start: usize, fdat: bool) -> (r: Self)
        requires b.len() <= SEGMENT_BYTES_MAX,
        ensures r.wf(), r.pos() == 0, r.seg_steps() == 0, r.core_steps() == 0,
            r.concat() == reader_bytes(b@, start as nat, fdat),
            r.seg_count() == reader_count(b@, start as nat, fdat),
            12 * r.seg_count() <= b.len(),
    {
        proof { model::reader_shape(b@, start as nat, fdat); }
        Self { b, start, fdat, pos: 0, end: 0, next: start, done: false,
            source: Ghost(reader_bytes(b@, start as nat, fdat)), prefix: Ghost(Seq::empty()),
            total: Ghost(reader_count(b@, start as nat, fdat)), entered: Ghost(0nat), reads: Ghost(0nat) }
    }

    /// A fresh IDAT-run cursor. The caller will discharge the input bound from
    /// the successful walk's MAX_INPUT test when entry points are migrated.
    pub(crate) fn new(b: &'a [u8], first: usize) -> (r: Self)
        requires b.len() <= SEGMENT_BYTES_MAX,
        ensures r.wf(), r.pos() == 0, r.seg_steps() == 0, r.core_steps() == 0,
            r.concat() == reader_bytes(b@, first as nat, false),
            12 * r.seg_count() <= b.len(),
    { Self::at(b, first, false) }

    /// A fresh fdAT-span cursor, stopping at the next fcTL or IEND.
    pub(crate) fn new_fdat(b: &'a [u8], start: usize) -> (r: Self)
        requires b.len() <= SEGMENT_BYTES_MAX,
        ensures r.wf(), r.pos() == 0, r.seg_steps() == 0, r.core_steps() == 0,
            r.concat() == reader_bytes(b@, start as nat, true),
            12 * r.seg_count() <= b.len(),
    { Self::at(b, start, true) }
}

impl<'a> Segments for DataReader<'a> {
    #[cfg(verus_only)]
    type Frame = (&'a [u8], usize, bool);
    closed spec fn concat(&self) -> Seq<u8> { self.source@ }
    closed spec fn pos(&self) -> nat { self.prefix@.len() }
    closed spec fn seg_index(&self) -> nat { self.entered@ }
    closed spec fn seg_count(&self) -> nat { self.total@ }
    closed spec fn seg_steps(&self) -> nat { self.entered@ + if self.done { 1nat } else { 0nat } }
    closed spec fn core_steps(&self) -> nat { self.reads@ }
    closed spec fn advance_rank(&self) -> nat { self.entered@ + if self.done { 1nat } else { 0nat } }
    closed spec fn source_frame(&self) -> Self::Frame { (self.b, self.start, self.fdat) }
    closed spec fn state_wf(&self) -> bool {
        self.pos <= self.end <= self.b.len()
        && self.source@ == reader_bytes(self.b@, self.start as nat, self.fdat)
        && self.total@ == reader_count(self.b@, self.start as nat, self.fdat)
        && self.source@.len() <= SEGMENT_BYTES_MAX
        && if self.done {
            self.entered@ == self.total@ && self.source@ == self.prefix@
        } else {
            self.entered@ + reader_count(self.b@, self.next as nat, self.fdat) == self.total@
            && self.source@ == self.prefix@ + self.b@.subrange(self.pos as int, self.end as int)
                + reader_bytes(self.b@, self.next as nat, self.fdat)
        }
    }

    proof fn lemma_wf(&self) {}

    fn read(&mut self) -> (r: Option<u8>) {
        let ghost initial = *self;
        loop
            invariant self.wf(), initial.wf(), initial == *old(self),
                self.b == initial.b, self.start == initial.start, self.fdat == initial.fdat,
                self.source@ == initial.source@, self.total@ == initial.total@,
                self.prefix@ == initial.prefix@, self.reads@ == initial.reads@,
                self.entered@ >= initial.entered@,
                self.advance_rank() >= initial.advance_rank(),
                self.done == initial.done,
            decreases if self.next <= self.b.len() { self.b.len() - self.next + 1 } else { 0 },
        {
            if self.done {
                proof { self.reads = Ghost(self.reads@ + 1); }
                return None;
            }
            if self.pos < self.end {
                let x = self.b[self.pos];
                proof {
                    let active = self.b@.subrange(self.pos as int, self.end as int);
                    let rest = reader_bytes(self.b@, self.next as nat, self.fdat);
                    assert(active.first() == x);
                    assert(active == seq![x] + self.b@.subrange(self.pos as int + 1, self.end as int));
                    assert(self.source@[self.prefix@.len() as int] == x);
                    self.prefix = Ghost(self.prefix@ + seq![x]);
                    self.reads = Ghost(self.reads@ + 1);
                }
                self.pos += 1;
                proof {
                    assert(self.source@ =~= self.prefix@ + self.b@.subrange(self.pos as int, self.end as int)
                        + reader_bytes(self.b@, self.next as nat, self.fdat));
                    assert(self.state_wf());
                }
                return Some(x);
            }
            match segment_at(self.b, self.next, self.fdat) {
                None => {
                    self.done = true;
                    return None;
                },
                Some((begin, end, next)) => {
                    proof {
                        assert(reader_count(self.b@, self.next as nat, self.fdat)
                            == 1 + reader_count(self.b@, next as nat, self.fdat));
                        assert(reader_bytes(self.b@, self.next as nat, self.fdat)
                            == self.b@.subrange(begin as int, end as int)
                                + reader_bytes(self.b@, next as nat, self.fdat));
                        assert(self.source@ =~= self.prefix@ + self.b@.subrange(begin as int, end as int)
                            + reader_bytes(self.b@, next as nat, self.fdat));
                    }
                    self.pos = begin;
                    self.end = end;
                    self.next = next;
                    proof { self.entered = Ghost(self.entered@ + 1); }
                },
            }
        }
    }
}

/// Find a frame boundary using only lengths and types. CRC validation belongs
/// to the successful frames walk, not to this re-walk.
pub(crate) fn next_fctl(b: &[u8], mut p: usize) -> (r: Option<usize>)
    ensures r.is_some() ==> r.unwrap() < b.len(),
{
    loop
        decreases if p <= b.len() { b.len() - p + 1 } else { 0 },
    {
        match frame_at(b, p) {
            Ok(c) => {
                if type_is(&c.ty, b'I', b'E', b'N', b'D') { return None; }
                if type_is(&c.ty, b'f', b'c', b'T', b'L') { return Some(p); }
                p = c.next;
            },
            Err(_) => return None,
        }
    }
}

} // verus!
