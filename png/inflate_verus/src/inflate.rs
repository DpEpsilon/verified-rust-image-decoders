#[cfg(verus_only)]
use crate::model as m;
use crate::{
    adler::Adler,
    bits::Bits,
    huffman::{Huffman, Kind},
    limits::*,
    InflateError as E, InflateOptions, InflateResult, Segments, Status, Window, Work, ZlibTail,
};
use vstd::prelude::*;

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; NDIST_MAX] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; NDIST_MAX] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
verus! {
const CL_ORDER: [usize; NCL] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

proof fn lemma_cl_order(i: int)
    requires 0 <= i < NCL,
    ensures CL_ORDER@[i] == m::code_length_order()[i], CL_ORDER@[i] < NCL,
{
    assert(CL_ORDER@ == seq![16usize, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]);
}
}

// The public signatures require ring.len() == WINDOW_MAX for measuring, and
// a fresh source satisfying Segments' contract. A typed array inside the sink
// prevents accidental indexing of a shorter ring. No stream-driven allocation.
verus! {

/// Representation only: the independent model never reads this predicate.
closed spec fn ring_repr(ring: Seq<u8>, out: Seq<u8>) -> bool {
    ring.len() == WINDOW_MAX && forall|i: int|
        0 <= i < out.len() && out.len() - WINDOW_MAX <= i ==>
            #[trigger] ring[i % WINDOW_MAX as int] == out[i]
}

proof fn lemma_recent_distinct(k: nat, distance: nat)
    requires 1 <= distance < WINDOW_MAX, distance <= k,
    ensures (k as int) % (WINDOW_MAX as int) != (k - distance) % (WINDOW_MAX as int),
{
    vstd::arithmetic::div_mod::lemma_mod_equivalence(k as int, (k - distance) as int, WINDOW_MAX as int);
    vstd::arithmetic::div_mod::lemma_small_mod(distance, WINDOW_MAX as nat);
}

proof fn lemma_skip_later(a: Seq<u8>, b: Seq<u8>, p: int, q: int)
    requires a.len() == b.len(), 0 <= p <= q <= a.len(), a.skip(p) == b.skip(p),
    ensures a.skip(q) == b.skip(q),
{
    vstd::seq_lib::lemma_seq_skip_of_skip(a, p, q - p);
    vstd::seq_lib::lemma_seq_skip_of_skip(b, p, q - p);
    assert(a.skip(q) =~= a.skip(p).skip(q - p));
    assert(b.skip(q) =~= b.skip(p).skip(q - p));
}

proof fn lemma_ring_append(ring: Seq<u8>, out: Seq<u8>, byte: u8)
    requires ring_repr(ring, out),
    ensures ring_repr(ring.update((out.len() % WINDOW_MAX as nat) as int, byte), out.push(byte)),
{
    let slot = (out.len() % WINDOW_MAX as nat) as int;
    let next = ring.update(slot, byte);
    assert forall|i: int| 0 <= i < out.len() + 1 && out.len() + 1 - WINDOW_MAX <= i implies
        #[trigger] next[i % WINDOW_MAX as int] == out.push(byte)[i] by {
        if i < out.len() {
            lemma_recent_distinct(out.len(), (out.len() - i) as nat);
            assert(ring[i % WINDOW_MAX as int] == out[i]);
        } else { assert(i == out.len()); }
    }
}

enum Sink<'a> {
    Into(&'a mut [u8]),
    Ring(&'a mut [u8; WINDOW_MAX]),
}

closed spec fn sink_view(sink: Sink) -> Seq<u8> {
    match sink { Sink::Into(out) => out@, Sink::Ring(ring) => ring@ }
}

struct Output<'a> {
    sink: Sink<'a>,
    produced: u64,
    limit: u64,
    adler: Adler,
    out: Ghost<Seq<u8>>,
    initial: Ghost<Seq<u8>>,
}
impl<'a> Output<'a> {
    closed spec fn wf(&self) -> bool {
        self.produced == self.out@.len() && self.produced <= self.limit
        && self.adler.wf() && self.adler.data() == self.out@
        && match self.sink {
            Sink::Into(buffer) => buffer.len() == self.limit && self.initial@.len() == buffer.len()
                && buffer@.take(self.produced as int) == self.out@
                && buffer@.skip(self.produced as int) == self.initial@.skip(self.produced as int),
            Sink::Ring(ring) => ring_repr(ring@, self.out@),
        }
    }

    proof fn lemma_observation(&self)
        requires self.wf(),
        ensures self.produced == self.out@.len(), self.out@.len() <= self.limit,
    {}

    fn new(sink: Sink<'a>, limit: u64) -> (s: Self)
        requires match sink {
            Sink::Into(buffer) => buffer.len() == limit,
            Sink::Ring(ring) => ring@.len() == WINDOW_MAX,
        },
        ensures s.wf(), s.produced == 0, s.limit == limit, s.out@ == Seq::<u8>::empty(),
    {
        let initial = Ghost(sink_view(sink));
        let adler = Adler::new();
        proof {
            match &sink {
                Sink::Into(buffer) => {
                    assert(buffer@.take(0) =~= Seq::<u8>::empty());
                    assert(buffer@.skip(0) =~= initial@);
                },
                Sink::Ring(_) => {},
            }
        }
        Self { sink, produced: 0, limit, adler, out: Ghost(Seq::empty()), initial }
    }

    fn append_byte(&mut self, byte: u8) -> (r: Result<(), Status>)
        requires old(self).wf(),
        ensures final(self).wf(), final(self).limit == old(self).limit,
            final(self).initial == old(self).initial,
            match m::MEmit(old(self).out@, old(self).limit as nat, byte) {
                m::OutputEffect::Next { out } => r.is_ok() && final(self).out@ == out
                    && final(self).produced == old(self).produced + 1,
                m::OutputEffect::Full { out } => r == Err(Status::Full) && final(self).out@ == out
                    && final(self).produced == old(self).produced,
                _ => false,
            },
    {
        if self.produced == self.limit { return Err(Status::Full); }
        let ghost before = self.out@;
        match &mut self.sink {
            Sink::Into(out) => {
                let ghost previous = out@;
                proof {
                    assert(previous.skip(self.produced as int) == self.initial@.skip(self.produced as int));
                    lemma_skip_later(previous, self.initial@, self.produced as int, (self.produced + 1) as int);
                }
                out[self.produced as usize] = byte;
                proof {
                    assert(out@.take((self.produced + 1) as int) =~= before.push(byte));
                    assert(out@.skip((self.produced + 1) as int)
                        =~= self.initial@.skip((self.produced + 1) as int));
                }
            },
            Sink::Ring(ring) => {
                proof { lemma_ring_append(ring@, before, byte); }
                ring[(self.produced % WINDOW_MAX as u64) as usize] = byte;
            },
        }
        self.produced += 1;
        self.adler.update_byte(byte);
        proof { self.out = Ghost(before.push(byte)); }
        Ok(())
    }

    fn previous(&self, distance: u64) -> (byte: u8)
        requires self.wf(), 1 <= distance <= self.produced, distance <= WINDOW_MAX,
        ensures byte == self.out@[(self.produced - distance) as int],
    {
        let index = self.produced - distance;
        match &self.sink {
            Sink::Into(out) => out[index as usize],
            Sink::Ring(ring) => ring[(index % WINDOW_MAX as u64) as usize],
        }
    }
}

closed spec fn stored_eq(effect: m::StoredEffect, result: Result<(), Status>, out: Seq<u8>, position: nat) -> bool {
    match effect {
        m::StoredEffect::End { out: expected, position: end } => result.is_ok() && out == expected && position == end,
        m::StoredEffect::Full { out: expected } => result == Err(Status::Full) && out == expected,
        m::StoredEffect::Error { error, out: expected, position: end } => error == m::InflateFault::TruncatedZlib
            && result == Err(Status::Err(E::TruncatedZlib)) && out == expected && position == end,
        m::StoredEffect::OutsideDomain => false,
    }
}

/// The production stored-payload loop, after validated LEN/NLEN and alignment.
fn stored_payload<S: Segments>(br: &mut Bits<'_, S>, output: &mut Output<'_>, count: u16, w: &mut Work)
    -> (r: Result<(), Status>)
    requires old(br).wf(), old(br).remaining == 0, old(br).position() % 8 == 0,
        old(output).wf(), old(w).wf(),
    ensures final(br).wf(), final(br).remaining == 0, final(br).concat() == old(br).concat(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial, final(w).wf(),
        old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 2 * count,
        stored_eq(m::MStoredPayload(old(br).concat(), old(br).position() / 8,
            old(output).out@, old(output).limit as nat, count as nat), r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(ring_repr);
    hide(m::MStoredPayload);
    let ghost z = br.concat();
    let ghost start = br.position() / 8;
    let ghost original = output.out@;
    let ghost limit = output.limit;
    proof { br.lemma_bounds(); output.lemma_observation(); }
    let mut remaining = count;
    while remaining > 0
        invariant
            br.wf(), br.remaining == 0, br.concat() == z, z == old(br).concat(),
            start == old(br).position() / 8, start <= z.len(),
            br.position() == 8 * (start + count - remaining), br.position() <= 8 * z.len(),
            output.wf(), output.out@.len() <= limit, output.limit == limit, limit == old(output).limit,
            original == old(output).out@, output.initial == old(output).initial,
            w.wf(), old(w).observed@ <= w.observed@,
            w.observed@ == old(w).observed@ + 2 * (count - remaining),
            remaining <= count,
            m::MStoredPayload(z, start, original, limit as nat, count as nat)
                == m::MStoredPayload(z, br.position() / 8, output.out@, limit as nat, remaining as nat),
        decreases remaining,
    {
        let ghost cursor = br.position() / 8;
        let ghost before = output.out@;
        proof { m::lemma_stored_unfold(z, cursor, before, limit as nat, remaining as nat); }
        w.add(1);
        let byte = match br.byte(w) {
            Ok(byte) => byte,
            Err(e) => {
                proof { assert(cursor == z.len()); }
                return Err(Status::Err(e));
            }
        };
        match output.append_byte(byte) {
            Ok(()) => {},
            Err(status) => return Err(status),
        }
        remaining -= 1;
    }
    proof { m::lemma_stored_unfold(z, br.position() / 8, output.out@, limit as nat, 0); }
    Ok(())
}

} // verus!

verus! {

struct Tables {
    fixed_lit: Huffman,
    fixed_dist: Huffman,
    lit: Huffman,
    dist: Huffman,
    code: Huffman,
    header_construction: Ghost<nat>,
    header_table_start: Ghost<Option<nat>>,
}
impl Tables {
    closed spec fn fixed_wf(&self) -> bool {
        self.fixed_lit.wf() && self.fixed_lit.ready@ && self.fixed_lit.lengths@ == m::fixed_lit_lengths()
        && self.fixed_dist.wf() && self.fixed_dist.ready@ && self.fixed_dist.lengths@ == m::fixed_dist_lengths()
    }

    fn new(w: &mut Work) -> (r: Result<Self, E>)
        requires old(w).wf(),
        ensures final(w).wf(), old(w).observed@ <= final(w).observed@ <= old(w).observed@ + C_INFL_FIXED,
            r.is_ok(), match r {
                Ok(tables) => tables.fixed_wf() && tables.lit.wf() && tables.dist.wf() && tables.code.wf(),
                Err(_) => false,
            },
    {
        let ghost start = w.observed@;
        proof { m::lemma_fixed_codes(true); }
        let mut tables = Self {
            fixed_lit: Huffman::new(w),
            fixed_dist: Huffman::new(w),
            lit: Huffman::new(w),
            dist: Huffman::new(w),
            code: Huffman::new(w),
            header_construction: Ghost(0nat),
            header_table_start: Ghost(None),
        };
        let mut lengths = [0u8; FIXED_LIT_COUNT];
        w.add(FIXED_LIT_COUNT);
        let mut i = 0;
        while i < FIXED_LIT_COUNT
            invariant i <= FIXED_LIT_COUNT, w.wf(),
                w.observed@ == start + 5 * HUFF_INIT_STEPS + FIXED_LIT_COUNT + i,
                forall|j: int| 0 <= j < i ==> lengths@[j] == m::fixed_lit_lengths()[j],
            decreases FIXED_LIT_COUNT - i,
        {
            w.add(1);
            lengths[i] = if i < 144 {
                8
            } else if i < 256 {
                9
            } else if i < 280 {
                7
            } else {
                8
            };
            i += 1;
        }
        proof { assert(lengths@ =~= m::fixed_lit_lengths()); }
        match tables.fixed_lit.build(&lengths, Kind::LitLen, w) {
            Ok(()) => {}, Err(error) => return Err(error),
        }
        w.add(FIXED_DIST_COUNT);
        let dist_lengths = [5u8; FIXED_DIST_COUNT];
        proof { assert(dist_lengths@ =~= m::fixed_dist_lengths()); }
        match tables.fixed_dist.build(&dist_lengths, Kind::Dist, w) {
            Ok(()) => {}, Err(error) => return Err(error),
        }
        proof {
            assert(5 * HUFF_INIT_STEPS + 2 * FIXED_LIT_COUNT + 2 * HUFF_BUILD_STEPS + FIXED_DIST_COUNT
                <= C_INFL_FIXED) by (compute);
        }
        Ok(tables)
    }
}

} // verus!

verus! {

closed spec fn dynamic_error(error: m::InflateFault, e: E) -> bool {
    match error {
        m::InflateFault::InvalidCodeLengths => e == E::InvalidCodeLengths,
        m::InflateFault::InvalidSymbol => e == E::InvalidSymbol,
        m::InflateFault::TruncatedZlib => e == E::TruncatedZlib,
        _ => false,
    }
}

closed spec fn dynamic_eq(effect: m::DynamicHeader, result: Result<(), E>, lit: Seq<u8>, dist: Seq<u8>, position: nat) -> bool {
    match effect {
        m::DynamicHeader::Tables { lit: l, dist: d, position: end } => result.is_ok()
            && lit == l && dist == d && position == end,
        m::DynamicHeader::Error { error, position: end } => result.is_err()
            && dynamic_error(error, result.unwrap_err()) && position == end,
        m::DynamicHeader::OutsideDomain => false,
    }
}

/// The production CL permutation loop and its canonical admission.
fn dynamic_code<S: Segments>(code: &mut Huffman, br: &mut Bits<'_, S>, ncode: usize,
    nlen: usize, ndist: usize, w: &mut Work) -> (ret: (Result<(), E>, Ghost<nat>))
    requires old(br).wf(), old(w).wf(), old(code).wf(), 4 <= ncode <= NCL,
        m::dynamic_sizes_ok(nlen as nat, ndist as nat),
    ensures final(br).wf(), final(w).wf(), final(code).wf(),
        final(br).concat() == old(br).concat(), old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        ret.1@ <= NCL + ncode + CANON_FIXED_STEPS + 2 * NCL + MAX_CODE_BITS + 1,
        old(w).observed@ <= final(w).observed@ <= old(w).observed@ + ret.1@ + 2 * (final(br).position() - old(br).position() + 1),
        ret.0.is_ok() ==> final(w).observed@ <= old(w).observed@ + ret.1@ + 2 * (final(br).position() - old(br).position()),
        ret.0.is_ok() ==> final(code).ready@ && final(code).kind@ == Kind::CodeLen
            && final(code).single_accept@ && final(code).lengths@.len() == NCL
            && final(br).position() == old(br).position() + 3 * ncode
            && m::admissible(final(code).lengths@, m::CodeKind::CodeLen, true),
        match ret.0 {
            Ok(()) => m::MDynamicCL(old(br).concat(), old(br).position(), Seq::new(NCL as nat, |i: int| 0u8),
                0, ncode as nat, nlen as nat, ndist as nat, true)
                == m::MDynamicLengths(final(br).concat(), final(br).position(), final(code).lengths@,
                    Seq::empty(), nlen as nat, ndist as nat, true),
            Err(e) => dynamic_eq(m::MDynamicCL(old(br).concat(), old(br).position(), Seq::new(NCL as nat, |i: int| 0u8),
                0, ncode as nat, nlen as nat, ndist as nat, true), Err(e), Seq::empty(), Seq::empty(), final(br).position()),
        },
        ret.0.is_err() ==> ret.0 == Err(E::TruncatedZlib) || ret.0 == Err(E::InvalidCodeLengths),
{
    hide(m::MDynamicCL);
    hide(m::MDynamicLengths);
    let ghost z = br.concat();
    let ghost start = br.position();
    let ghost initial = m::MDynamicCL(z, start, Seq::new(NCL as nat, |i: int| 0u8),
        0, ncode as nat, nlen as nat, ndist as nat, true);
    let mut cl = [0u8; NCL];
    w.add(NCL);
    proof { assert(cl@ =~= Seq::new(NCL as nat, |i: int| 0u8)); }
    let mut i = 0;
    while i < ncode
        invariant br.wf(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
            code.wf(), 4 <= ncode <= NCL, m::dynamic_sizes_ok(nlen as nat, ndist as nat),
            initial == m::MDynamicCL(z, start, Seq::new(NCL as nat, |i: int| 0u8),
                0, ncode as nat, nlen as nat, ndist as nat, true),
            br.position() == start + 3 * i, w.wf(), i <= ncode,
            old(w).observed@ <= w.observed@ <= old(w).observed@ + NCL + i + 6 * i,
            forall|s: int| 0 <= s < NCL ==> cl@[s] <= 7,
            initial == m::MDynamicCL(z, br.position(), cl@, i as nat, ncode as nat, nlen as nat, ndist as nat, true),
        decreases ncode - i,
    {
        proof { reveal(m::MDynamicCL); lemma_cl_order(i as int); }
        w.add(1);
        let value = match br.get(3, w) {
            Ok(value) => value,
            Err(e) => return (Err(e), Ghost((NCL + i + 1) as nat)),
        };
        proof { m::lemma_field_bound(z, (start + 3 * i) as nat, 3); assert(m::pow2(3) == 8) by (compute); }
        cl[CL_ORDER[i]] = value as u8;
        i += 1;
    }
    proof {
        br.lemma_bounds(); reveal(m::MDynamicCL);
        assert(initial == if m::admissible(cl@, m::CodeKind::CodeLen, true) {
            m::MDynamicLengths(z, br.position(), cl@, Seq::empty(), nlen as nat, ndist as nat, true)
        } else { m::DynamicHeader::Error { error: m::InflateFault::InvalidCodeLengths, position: br.position() } });
    }
    let ghost before_build = w.observed@;
    let result = code.build(&cl, Kind::CodeLen, w);
    let ghost cost = NCL + ncode + w.observed@ - before_build;
    (result, Ghost(cost as nat))
}

/// The real bounded repeat-fill loop, separated from input processing.
fn write_length_repeat(lengths: &mut [u8; DYNAMIC_LENGTHS], start: usize, count: usize, value: u8, w: &mut Work)
    -> (end: usize)
    requires start <= DYNAMIC_LENGTHS, count <= DYNAMIC_LENGTHS - start,
        value <= MAX_CODE_BITS, m::length_entries_ok(old(lengths)@), old(w).wf(),
    ensures end == start + count, m::length_entries_ok(final(lengths)@), final(w).wf(),
        final(w).observed@ == old(w).observed@ + count,
        final(lengths)@.take(end as int) == old(lengths)@.take(start as int) + Seq::new(count as nat, |j: int| value),
{
    let ghost next = lengths@.take(start as int) + Seq::new(count as nat, |j: int| value);
    let end = start + count;
    let mut i = start;
    while i < end
        invariant start <= i <= end <= DYNAMIC_LENGTHS, end == start + count, value <= MAX_CODE_BITS,
            next == old(lengths)@.take(start as int) + Seq::new(count as nat, |j: int| value),
            lengths@.take(i as int) == next.take(i as int), m::length_entries_ok(lengths@),
            w.wf(), w.observed@ == old(w).observed@ + i - start,
        decreases end - i,
    {
        w.add(1);
        lengths[i] = value;
        proof { assert(lengths@.take((i + 1) as int) =~= next.take((i + 1) as int)); }
        i += 1;
    }
    proof { assert(lengths@.take(end as int) =~= next); }
    end
}

impl Tables {
    /// The production expansion loop, EOB check and ordered table admissions.
    #[verifier::spinoff_prover]
    fn dynamic_lengths<S: Segments>(&mut self, br: &mut Bits<'_, S>, nlen: usize, ndist: usize,
        w: &mut Work) -> (ret: (Result<(), E>, Ghost<nat>))
        requires old(br).wf(), old(w).wf(), m::dynamic_sizes_ok(nlen as nat, ndist as nat),
            old(self).code.wf(), old(self).code.ready@, old(self).code.lengths@.len() == NCL,
            m::admissible(old(self).code.lengths@, m::CodeKind::CodeLen, true),
            old(self).lit.wf(), old(self).dist.wf(),
        ensures final(br).wf(), final(w).wf(), final(br).concat() == old(br).concat(),
            old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
            final(self).code == old(self).code,
            final(self).fixed_lit == old(self).fixed_lit, final(self).fixed_dist == old(self).fixed_dist,
            final(self).header_construction == old(self).header_construction,
            final(self).header_table_start@.is_some() ==> final(self).header_table_start@ == Some(final(br).position())
                && final(br).position() >= old(br).position() + 16,
            ret.0.is_ok() ==> final(self).header_table_start@.is_some(),
            final(self).lit.wf(), final(self).dist.wf(),
            ret.1@ <= DYNAMIC_LENGTHS + 4 * (nlen + ndist) + 2 * (CANON_FIXED_STEPS + MAX_CODE_BITS + 1),
            old(w).observed@ <= final(w).observed@ <= old(w).observed@ + ret.1@ + 3 * (final(br).position() - old(br).position() + 1),
            ret.0.is_ok() ==> final(self).lit.ready@ && final(self).dist.ready@
                && final(self).lit.kind@ == Kind::LitLen && final(self).dist.kind@ == Kind::Dist
                && final(self).lit.single_accept@ && final(self).dist.single_accept@
                && final(br).position() >= old(br).position() + 16,
            dynamic_eq(m::MDynamicLengths(old(br).concat(), old(br).position(), old(self).code.lengths@,
                Seq::empty(), nlen as nat, ndist as nat, true), ret.0,
                final(self).lit.lengths@, final(self).dist.lengths@, final(br).position()),
    {
        hide(m::MDynamicLengths);
        hide(m::MDec);
        let ghost z = br.concat();
        let ghost start = br.position();
        let ghost code = self.code.lengths@;
        let ghost initial = m::MDynamicLengths(z, start, code, Seq::empty(), nlen as nat, ndist as nat, true);
        proof { br.lemma_bounds(); self.header_table_start = Ghost(None); }
        let mut lengths = [0u8; DYNAMIC_LENGTHS];
        w.add(DYNAMIC_LENGTHS);
        let total = nlen + ndist;
        proof { assert(lengths@.take(0) =~= Seq::<u8>::empty()); }
        let mut i = 0usize;
        let ghost mut construction: nat = DYNAMIC_LENGTHS as nat;
        let ghost mut large: nat = 0;
        let ghost mut large_entries: nat = 0;
        let ghost mut other_entries: nat = 0;
        let ghost mut large_bits: nat = 0;
        let ghost mut other_bits: nat = 0;
        while i < total
            invariant br.wf(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
                start <= br.position() <= 8 * z.len(), w.wf(),
                self.code == old(self).code, self.code.wf(), self.code.ready@, code == self.code.lengths@,
                code.len() == NCL, m::admissible(code, m::CodeKind::CodeLen, true),
                self.lit == old(self).lit, self.dist == old(self).dist,
                self.lit.wf(), self.dist.wf(), self.header_table_start@.is_none(),
                m::dynamic_sizes_ok(nlen as nat, ndist as nat),
                initial == m::MDynamicLengths(z, start, code, Seq::empty(), nlen as nat, ndist as nat, true),
                i <= total, total == nlen + ndist, total <= DYNAMIC_LENGTHS,
                m::length_entries_ok(lengths@),
                DYNAMIC_LENGTHS <= construction <= DYNAMIC_LENGTHS + 2 * i,
                old(w).observed@ <= w.observed@ <= old(w).observed@ + construction + 3 * (br.position() - start),
                initial == m::MDynamicLengths(z, br.position(), code, lengths@.take(i as int), nlen as nat, ndist as nat, true),
                large_entries + other_entries == i, large_entries <= 138 * large,
                large_bits >= 8 * large, 5 * other_bits >= 2 * other_entries,
                br.position() == start + large_bits + other_bits,
            decreases total - i,
        {
            let ghost cursor = br.position();
            let ghost prefix = lengths@.take(i as int);
            proof { reveal(m::MDynamicLengths); }
            w.add(1);
            proof { construction = (construction + 1) as nat; }
            let sym = match self.code.decode(br, w) {
                Ok(sym) => sym,
                Err(e) => return (Err(e), Ghost(construction)),
            };
            let ghost after_code = br.position();
            if sym < 16 {
                lengths[i] = sym as u8;
                proof {
                    assert(lengths@.take((i + 1) as int) =~= prefix.push(sym as u8));
                    other_entries = (other_entries + 1) as nat;
                    other_bits = (other_bits + after_code - cursor) as nat;
                }
                i += 1;
            } else {
                let (value, repeat) = if sym == 16 {
                    if i == 0 { return (Err(E::InvalidCodeLengths), Ghost(construction)); }
                    let extra = match br.get(2, w) {
                        Ok(extra) => extra, Err(e) => return (Err(e), Ghost(construction)),
                    };
                    proof { m::lemma_field_bound(z, after_code, 2); assert(m::pow2(2) == 4) by (compute); }
                    (lengths[i - 1], 3 + extra as usize)
                } else if sym == 17 {
                    let extra = match br.get(3, w) {
                        Ok(extra) => extra, Err(e) => return (Err(e), Ghost(construction)),
                    };
                    proof { m::lemma_field_bound(z, after_code, 3); assert(m::pow2(3) == 8) by (compute); }
                    (0u8, 3 + extra as usize)
                } else {
                    let extra = match br.get(7, w) {
                        Ok(extra) => extra, Err(e) => return (Err(e), Ghost(construction)),
                    };
                    proof { m::lemma_field_bound(z, after_code, 7); assert(m::pow2(7) == 128) by (compute); }
                    (0u8, 11 + extra as usize)
                };
                if repeat > total - i { return (Err(E::InvalidCodeLengths), Ghost(construction)); }
                let ghost next = prefix + Seq::new(repeat as nat, |j: int| value);
                i = write_length_repeat(&mut lengths, i, repeat, value, w);
                proof { construction = construction + repeat as nat; }
                proof {
                    assert(lengths@.take(i as int) =~= next);
                    if sym == 18 { large = (large + 1) as nat; large_entries = (large_entries + repeat as nat) as nat; large_bits = (large_bits + br.position() - cursor) as nat; }
                    else { other_entries = (other_entries + repeat as nat) as nat; other_bits = (other_bits + br.position() - cursor) as nat; }
                }
            }
        }
        proof {
            reveal(m::MDynamicLengths);
            assert(lengths@.take(total as int).len() == total);
            assert(lengths@.take(total as int).take(nlen as int) =~= lengths@.take(nlen as int));
            assert(lengths@.take(total as int).skip(nlen as int) =~= lengths@.subrange(nlen as int, total as int));
            m::lemma_header_coding_cost(large, large_entries, other_entries, large_bits, other_bits);
            assert(br.position() >= start + 16);
        }
        if lengths[256] == 0 { return (Err(E::InvalidCodeLengths), Ghost(construction)); }
        // The coding-cost invariant applies even when construction rejects.
        proof { self.header_table_start = Ghost(Some(br.position())); }
        let ghost before_lit = w.observed@;
        let lit_result = self.lit.build(&lengths[..nlen], Kind::LitLen, w);
        proof { construction = (construction + w.observed@ - before_lit) as nat; }
        match lit_result { Ok(()) => {}, Err(e) => return (Err(e), Ghost(construction)), }
        let ghost before_dist = w.observed@;
        let dist_result = self.dist.build(&lengths[nlen..total], Kind::Dist, w);
        proof { construction = (construction + w.observed@ - before_dist) as nat; }
        (dist_result, Ghost(construction))
    }

    fn dynamic<S: Segments>(&mut self, br: &mut Bits<'_, S>, w: &mut Work) -> (r: Result<(), E>)
        requires old(br).wf(), old(w).wf(), old(self).code.wf(), old(self).lit.wf(), old(self).dist.wf(),
        ensures final(br).wf(), final(w).wf(), final(br).concat() == old(br).concat(),
            old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
            final(self).fixed_lit == old(self).fixed_lit, final(self).fixed_dist == old(self).fixed_dist,
            final(self).code.wf(), final(self).lit.wf(), final(self).dist.wf(),
            final(self).header_construction@ < 2048,
            final(self).header_table_start@.is_some() ==> final(self).header_table_start@ == Some(final(br).position())
                && final(br).position() >= old(br).position() + 42,
            r.is_ok() ==> final(self).header_table_start@.is_some(),
            old(w).observed@ <= final(w).observed@ <= old(w).observed@ + final(self).header_construction@
                + 3 * (final(br).position() - old(br).position() + 1),
            r.is_ok() ==> final(self).lit.ready@ && final(self).dist.ready@
                && final(br).position() >= old(br).position() + 42,
            dynamic_eq(m::MDynamic(old(br).concat(), old(br).position(), true), r,
                final(self).lit.lengths@, final(self).dist.lengths@, final(br).position()),
    {
        hide(m::MDynamicCL);
        hide(m::MDynamicLengths);
        let ghost z = br.concat();
        let ghost start = br.position();
        proof {
            br.lemma_bounds(); self.header_construction = Ghost(0nat); self.header_table_start = Ghost(None);
            m::lemma_dynamic_count_fields(z, start);
            assert(m::pow2(5) == 32 && m::pow2(4) == 16) by (compute);
        }
        let nlen = match br.get(5, w) { Ok(n) => n as usize + 257, Err(e) => return Err(e), };
        let ndist = match br.get(5, w) { Ok(n) => n as usize + 1, Err(e) => return Err(e), };
        let ncode = match br.get(4, w) { Ok(n) => n as usize + 4, Err(e) => return Err(e), };
        proof { m::lemma_dynamic_count_fields(z, start); }
        if nlen > NLIT_MAX || ndist > NDIST_MAX { return Err(E::InvalidCodeLengths); }
        let (code_result, code_cost) = dynamic_code(&mut self.code, br, ncode, nlen, ndist, w);
        proof { self.header_construction = code_cost; }
        match code_result { Ok(()) => {}, Err(e) => return Err(e), }
        let (result, length_cost) = self.dynamic_lengths(br, nlen, ndist, w);
        proof { self.header_construction = Ghost(self.header_construction@ + length_cost@); }
        result
    }
}

/// Actual block-header reads and dynamic-table selection. The block payload is
/// processed by the caller; no MBlk or whole-inflate claim is made here.
fn read_block_header<S: Segments>(tables: &mut Tables, br: &mut Bits<'_, S>, w: &mut Work)
    -> (r: Result<(u32, u32), E>)
    requires old(br).wf(), old(w).wf(), old(tables).code.wf(), old(tables).lit.wf(), old(tables).dist.wf(),
    ensures final(br).wf(), final(w).wf(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        final(tables).fixed_lit == old(tables).fixed_lit, final(tables).fixed_dist == old(tables).fixed_dist,
        final(tables).code.wf(), final(tables).lit.wf(), final(tables).dist.wf(),
        final(tables).header_construction@ < 2048,
        final(tables).header_table_start@.is_some() ==>
            final(tables).header_table_start@ == Some(final(br).position())
            && final(br).position() >= old(br).position() + 45,
        old(w).observed@ <= final(w).observed@ <= old(w).observed@ + final(tables).header_construction@
            + 3 * (final(br).position() - old(br).position() + 1),
        match r {
            Ok((last, typ)) => last == m::field_value(old(br).concat(), old(br).position(), 1)
                && typ == m::field_value(old(br).concat(), old(br).position() + 1, 2) && typ <= 2
                && if typ == 2 { final(tables).lit.ready@ && final(tables).dist.ready@
                    && dynamic_eq(m::MDynamic(old(br).concat(), old(br).position() + 3, true), Ok(()),
                        final(tables).lit.lengths@, final(tables).dist.lengths@, final(br).position()) }
                    else { final(br).position() == old(br).position() + 3 },
            Err(e) => if old(br).position() + 3 > 8 * old(br).concat().len() {
                e == E::TruncatedZlib && final(br).position() == 8 * old(br).concat().len()
            } else if m::field_value(old(br).concat(), old(br).position() + 1, 2) == 3 {
                e == E::InvalidBlockType && final(br).position() == old(br).position() + 3
            } else { m::field_value(old(br).concat(), old(br).position() + 1, 2) == 2
                && dynamic_eq(m::MDynamic(old(br).concat(), old(br).position() + 3, true), Err(e),
                    final(tables).lit.lengths@, final(tables).dist.lengths@, final(br).position()) },
        },
{
    hide(m::MDynamic);
    proof { br.lemma_bounds(); tables.header_construction = Ghost(0nat); tables.header_table_start = Ghost(None); }
    let final_block = match br.get(1, w) { Ok(n) => n, Err(e) => return Err(e), };
    let block_type = match br.get(2, w) { Ok(n) => n, Err(e) => return Err(e), };
    proof { assert(m::pow2(2) == 4) by (compute); }
    if block_type == 3 { return Err(E::InvalidBlockType); }
    if block_type == 2 {
        match tables.dynamic(br, w) { Ok(()) => {}, Err(e) => return Err(e), }
    }
    proof { br.lemma_bounds(); }
    Ok((final_block, block_type))
}

} // verus!

fn compressed<S: Segments>(
    br: &mut Bits<'_, S>,
    output: &mut Output<'_>,
    lit: &Huffman,
    dist: &Huffman,
    window: u64,
    w: &mut Work,
) -> Result<(), Status> {
    loop {
        w.add(1);
        let sym = lit.decode(br, w).map_err(Status::Err)?;
        if sym < 256 {
            output.append_byte(sym as u8)?;
        } else if sym == 256 {
            return Ok(());
        } else {
            if sym > 285 {
                return Err(Status::Err(E::InvalidSymbol));
            }
            let index = usize::from(sym - 257);
            let length =
                u32::from(LEN_BASE[index]) + br.get(LEN_EXTRA[index], w).map_err(Status::Err)?;
            let ds = usize::from(dist.decode(br, w).map_err(Status::Err)?);
            if ds >= NDIST_MAX {
                return Err(Status::Err(E::InvalidSymbol));
            }
            let distance = u64::from(DIST_BASE[ds])
                + u64::from(br.get(DIST_EXTRA[ds], w).map_err(Status::Err)?);
            if distance > output.produced || distance > window {
                return Err(Status::Err(E::DistanceTooFar));
            }
            let mut j = 0;
            while j < length {
                w.add(1);
                // Resolve against the evolving output/ring, including overlap.
                let byte = output.previous(distance);
                output.append_byte(byte)?;
                j += 1;
            }
        }
    }
}

fn run<S: Segments>(
    br: &mut Bits<'_, S>,
    output: &mut Output<'_>,
    o: InflateOptions,
    w: &mut Work,
) -> Result<(), Status> {
    let cmf = br.byte(w).map_err(Status::Err)?;
    let flg = br.byte(w).map_err(Status::Err)?;
    if (u16::from(cmf) * 256 + u16::from(flg)) % 31 != 0 {
        return Err(Status::Err(E::InvalidZlibHeader));
    }
    if cmf & 15 != 8 || cmf >> 4 > 7 {
        return Err(Status::Err(E::UnsupportedZlibMethod));
    }
    if flg & 0x20 != 0 {
        return Err(Status::Err(E::ZlibPresetDictionary));
    }
    let window = match o.window {
        Window::Enforce => 1u64 << ((cmf >> 4) + 8),
        Window::Ignore => WINDOW_MAX as u64,
    };
    let mut tables = Tables::new(w).map_err(Status::Err)?;
    loop {
        w.add(1);
        let (final_block, block_type) =
            read_block_header(&mut tables, br, w).map_err(Status::Err)?;
        match block_type {
            0 => {
                br.align();
                let lo = u16::from(br.byte(w).map_err(Status::Err)?);
                let hi = u16::from(br.byte(w).map_err(Status::Err)?);
                let nlo = u16::from(br.byte(w).map_err(Status::Err)?);
                let nhi = u16::from(br.byte(w).map_err(Status::Err)?);
                let len = lo | hi << 8;
                if len != !(nlo | nhi << 8) {
                    return Err(Status::Err(E::StoredLengthMismatch));
                }
                // The verified payload loop preserves D5's read-before-limit order.
                stored_payload(br, output, len, w)?;
            }
            1 => compressed(br, output, &tables.fixed_lit, &tables.fixed_dist, window, w)?,
            2 => {
                compressed(br, output, &tables.lit, &tables.dist, window, w)?;
            }
            _ => return Err(Status::Err(E::InvalidBlockType)),
        }
        if final_block != 0 {
            break;
        }
    }
    br.align();
    let mut expected = 0u32;
    let mut i = 0;
    while i < 4 {
        w.add(1);
        expected = (expected << 8) | u32::from(br.byte(w).map_err(Status::Err)?);
        i += 1;
    }
    if expected != output.adler.finish() {
        return Err(Status::Err(E::AdlerMismatch));
    }
    // IgnoreTail is inert for a short raster: only exactly-L output may ignore
    // bytes after the checked Adler-32 (OPTIONS.md's RecommendedBeta condition).
    if o.zlib_tail == ZlibTail::IgnoreTail && output.produced == output.limit {
        return Ok(());
    }
    w.add(1);
    if br.src.read().is_some() {
        return Err(Status::Err(E::TrailingZlibData));
    }
    Ok(())
}

fn entry<S: Segments>(src: &mut S, sink: Sink<'_>, limit: u64, o: InflateOptions) -> InflateResult {
    let mut output = Output::new(sink, limit);
    let mut work = Work::default();
    let mut br = Bits::new(src);
    let status = match run(&mut br, &mut output, o, &mut work) {
        Ok(()) => Status::Done,
        Err(s) => s,
    };
    InflateResult {
        status,
        produced: output.produced,
        core_steps: work.core,
        seg_steps: br.src.diagnostic_seg_steps(),
    }
}

/// Write at most out.len() bytes; preserve the unwritten suffix on every result.
/// Requires a fresh source satisfying Segments' contract. No allocation.
pub fn inflate_zlib_into<S: Segments>(
    src: &mut S,
    out: &mut [u8],
    o: InflateOptions,
) -> InflateResult {
    let limit = out.len() as u64;
    entry(src, Sink::Into(out), limit, o)
}

/// Measure using exactly WINDOW_MAX caller-owned bytes, whose initial contents
/// may be arbitrary. Requires a fresh source and ring.len() == WINDOW_MAX.
/// An invalid ring is a caller-contract violation, not an input-format error.
pub fn inflate_zlib_measure<S: Segments>(
    src: &mut S,
    ring: &mut [u8],
    limit: u64,
    o: InflateOptions,
) -> InflateResult {
    let ring: &mut [u8; WINDOW_MAX] = ring
        .try_into()
        .expect("inflate measure requires WINDOW_MAX ring bytes");
    entry(src, Sink::Ring(ring), limit, o)
}
