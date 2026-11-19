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

verus! {
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
proof fn lemma_length_tables(index: int)
    requires 0 <= index < 29,
    ensures LEN_BASE@[index] == m::length_base((index + 257) as nat),
        LEN_EXTRA@[index] == m::length_extra((index + 257) as nat),
        LEN_EXTRA@[index] <= 5, 3 <= LEN_BASE@[index],
        LEN_BASE@[index] + m::pow2(LEN_EXTRA@[index] as nat) - 1 <= LEN_MAX,
{
    assert(LEN_BASE@ == seq![3u16, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258]);
    assert(LEN_EXTRA@ == seq![0u8, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0]);
    reveal_with_fuel(m::pow2, 16);
    assert(m::pow2(0) == 1 && m::pow2(1) == 2 && m::pow2(2) == 4 && m::pow2(3) == 8
        && m::pow2(4) == 16 && m::pow2(5) == 32 && m::pow2(6) == 64 && m::pow2(7) == 128) by (compute);
    assert(m::length_base(257) == 3 && m::length_base(258) == 4 && m::length_base(259) == 5
        && m::length_base(260) == 6 && m::length_base(261) == 7 && m::length_base(262) == 8
        && m::length_base(263) == 9 && m::length_base(264) == 10 && m::length_base(265) == 11
        && m::length_base(266) == 13 && m::length_base(267) == 15 && m::length_base(268) == 17
        && m::length_base(269) == 19 && m::length_base(270) == 23 && m::length_base(271) == 27
        && m::length_base(272) == 31 && m::length_base(273) == 35 && m::length_base(274) == 43
        && m::length_base(275) == 51 && m::length_base(276) == 59 && m::length_base(277) == 67
        && m::length_base(278) == 83 && m::length_base(279) == 99 && m::length_base(280) == 115
        && m::length_base(281) == 131 && m::length_base(282) == 163 && m::length_base(283) == 195
        && m::length_base(284) == 227 && m::length_base(285) == LEN_MAX) by (compute);
    if index == 0 {} else if index == 1 {} else if index == 2 {} else if index == 3 {}
    else if index == 4 {} else if index == 5 {} else if index == 6 {} else if index == 7 {}
    else if index == 8 {} else if index == 9 {} else if index == 10 {} else if index == 11 {}
    else if index == 12 {} else if index == 13 {} else if index == 14 {} else if index == 15 {}
    else if index == 16 {} else if index == 17 {} else if index == 18 {} else if index == 19 {}
    else if index == 20 {} else if index == 21 {} else if index == 22 {} else if index == 23 {}
    else if index == 24 {} else if index == 25 {} else if index == 26 {} else if index == 27 {}
    else {}
}

proof fn lemma_distance_tables(symbol: int)
    requires 0 <= symbol < NDIST_MAX,
    ensures DIST_BASE@[symbol] == m::distance_base(symbol as nat),
        DIST_EXTRA@[symbol] == m::distance_extra(symbol as nat),
        DIST_EXTRA@[symbol] <= 13, 1 <= DIST_BASE@[symbol],
        DIST_BASE@[symbol] + m::pow2(DIST_EXTRA@[symbol] as nat) - 1 <= WINDOW_MAX,
{
    assert(DIST_BASE@ == seq![1u16, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577]);
    assert(DIST_EXTRA@ == seq![0u8, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13]);
    reveal_with_fuel(m::pow2, 16);
    assert(m::pow2(0) == 1 && m::pow2(1) == 2 && m::pow2(2) == 4 && m::pow2(3) == 8
        && m::pow2(4) == 16 && m::pow2(5) == 32 && m::pow2(6) == 64 && m::pow2(7) == 128
        && m::pow2(8) == 256 && m::pow2(9) == 512 && m::pow2(10) == 1024
        && m::pow2(11) == 2048 && m::pow2(12) == 4096 && m::pow2(13) == 8192
        && m::pow2(14) == 16384) by (compute);
    assert(m::distance_base(0) == 1 && m::distance_base(1) == 2 && m::distance_base(2) == 3
        && m::distance_base(3) == 4 && m::distance_base(4) == 5 && m::distance_base(5) == 7
        && m::distance_base(6) == 9 && m::distance_base(7) == 13 && m::distance_base(8) == 17
        && m::distance_base(9) == 25 && m::distance_base(10) == 33 && m::distance_base(11) == 49
        && m::distance_base(12) == 65 && m::distance_base(13) == 97 && m::distance_base(14) == 129
        && m::distance_base(15) == 193 && m::distance_base(16) == 257 && m::distance_base(17) == 385
        && m::distance_base(18) == 513 && m::distance_base(19) == 769 && m::distance_base(20) == 1025
        && m::distance_base(21) == 1537 && m::distance_base(22) == 2049 && m::distance_base(23) == 3073
        && m::distance_base(24) == 4097 && m::distance_base(25) == 6145 && m::distance_base(26) == 8193
        && m::distance_base(27) == 12289 && m::distance_base(28) == 16385 && m::distance_base(29) == 24577) by (compute);
    if symbol < 4 { assert(DIST_EXTRA@[symbol] == 0); }
    else if symbol < 6 { assert(DIST_EXTRA@[symbol] == 1); }
    else if symbol < 8 { assert(DIST_EXTRA@[symbol] == 2); }
    else if symbol < 10 { assert(DIST_EXTRA@[symbol] == 3); }
    else if symbol < 12 { assert(DIST_EXTRA@[symbol] == 4); }
    else if symbol < 14 { assert(DIST_EXTRA@[symbol] == 5); }
    else if symbol < 16 { assert(DIST_EXTRA@[symbol] == 6); }
    else if symbol < 18 { assert(DIST_EXTRA@[symbol] == 7); }
    else if symbol < 20 { assert(DIST_EXTRA@[symbol] == 8); }
    else if symbol < 22 { assert(DIST_EXTRA@[symbol] == 9); }
    else if symbol < 24 { assert(DIST_EXTRA@[symbol] == 10); }
    else if symbol < 26 { assert(DIST_EXTRA@[symbol] == 11); }
    else if symbol < 28 { assert(DIST_EXTRA@[symbol] == 12); }
    else { assert(DIST_EXTRA@[symbol] == 13); }
    if symbol == 0 {} else if symbol == 1 {} else if symbol == 2 {} else if symbol == 3 {}
    else if symbol == 4 {} else if symbol == 5 {} else if symbol == 6 {} else if symbol == 7 {}
    else if symbol == 8 {} else if symbol == 9 {} else if symbol == 10 {} else if symbol == 11 {}
    else if symbol == 12 {} else if symbol == 13 {} else if symbol == 14 {} else if symbol == 15 {}
    else if symbol == 16 {} else if symbol == 17 {} else if symbol == 18 {} else if symbol == 19 {}
    else if symbol == 20 {} else if symbol == 21 {} else if symbol == 22 {} else if symbol == 23 {}
    else if symbol == 24 {} else if symbol == 25 {} else if symbol == 26 {} else if symbol == 27 {}
    else if symbol == 28 {} else {}
    assert(WINDOW_MAX == 32768) by (compute);
    assert(DIST_EXTRA@[symbol] <= 13);
    assert(DIST_BASE@[symbol] <= 24577);
    m::lemma_pow2_monotone(DIST_EXTRA@[symbol] as nat, 13);
}

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

// Public measuring requires ring.len() == WINDOW_MAX; Output::wf preserves it.
// Both public entries require only wf and pos()==0 from the source, with no
// index or counter reset. No stream-driven allocation.
verus! {

/// Representation only: the independent model never reads this predicate.
pub open spec fn ring_repr(ring: Seq<u8>, out: Seq<u8>) -> bool {
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
    Ring(&'a mut [u8]),
}

closed spec fn sink_view(sink: Sink) -> Seq<u8> {
    match sink { Sink::Into(out) => out@, Sink::Ring(ring) => ring@ }
}

/// The borrowed sink's eventual contents and variant, not its mutable current view.
#[verifier::prophetic]
closed spec fn sink_identity(sink: Sink) -> (bool, Seq<u8>) {
    match sink { Sink::Into(out) => (true, final(out)@), Sink::Ring(ring) => (false, final(ring)@) }
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
            s.initial@ == sink_view(sink), sink_identity(s.sink) == sink_identity(sink),
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
            sink_identity(final(self).sink) == sink_identity(old(self).sink),
            match m::MEmit(old(self).out@, old(self).limit as nat, byte) {
                m::OutputEffect::Next { out } => r.is_ok() && final(self).out@ == out
                    && final(self).produced == old(self).produced + 1,
                m::OutputEffect::Full { out } => r == Err(Status::Full) && final(self).out@ == out
                    && final(self).produced == old(self).produced,
                _ => false,
            },
    {
        proof { caps_ok(); }
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
        m::StoredEffect::Error { error, out: expected, position: end } => out == expected && position == end
            && match result { Err(Status::Err(e)) => fault_eq(error, e), _ => false },
        m::StoredEffect::OutsideDomain => false,
    }
}

/// The production stored-payload loop, after validated LEN/NLEN and alignment.
fn stored_payload<S: Segments>(br: &mut Bits<'_, S>, output: &mut Output<'_>, count: u16, w: &mut Work)
    -> (r: Result<(), Status>)
    requires old(br).wf(), old(br).remaining == 0, old(br).position() % 8 == 0,
        old(output).wf(), old(w).wf(),
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).remaining == 0, final(br).concat() == old(br).concat(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), final(w).wf(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        old(output).produced <= final(output).produced,
        old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 2 * count,
        final(w).observed@ <= old(w).observed@ + 2 * (final(output).produced - old(output).produced + 1),
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
            br.wf(), br.frame() == old(br).frame(), br.remaining == 0, br.concat() == z, z == old(br).concat(),
            start == old(br).position() / 8, start <= z.len(),
            br.position() == 8 * (start + count - remaining), br.position() <= 8 * z.len(),
            br.position() >= old(br).position(),
            output.wf(), output.out@.len() <= limit, output.limit == limit, limit == old(output).limit,
            original == old(output).out@, output.initial == old(output).initial,
            sink_identity(output.sink) == sink_identity(old(output).sink),
            output.produced == old(output).produced + count - remaining,
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

/// The production stored header, followed by the same verified payload loop.
fn stored_block<S: Segments>(br: &mut Bits<'_, S>, output: &mut Output<'_>, w: &mut Work)
    -> (r: Result<(), Status>)
    requires old(br).wf(), old(output).wf(), old(w).wf(),
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).remaining == 0, final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), old(output).produced <= final(output).produced,
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + 4 + 2 * (final(output).produced - old(output).produced + 1),
        stored_eq(m::MStoredBlock(old(br).concat(), old(br).position(), old(output).out@,
            old(output).limit as nat), r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(m::MStoredPayload);
    let ghost z = br.concat();
    let ghost start = br.position();
    proof { br.lemma_bounds(); output.lemma_observation(); }
    br.align();
    let ghost cursor = br.position() / 8;
    let lo = match br.byte(w) { Ok(b) => b as u16, Err(e) => return Err(Status::Err(e)), };
    let hi = match br.byte(w) { Ok(b) => b as u16, Err(e) => return Err(Status::Err(e)), };
    let nlo = match br.byte(w) { Ok(b) => b as u16, Err(e) => return Err(Status::Err(e)), };
    let nhi = match br.byte(w) { Ok(b) => b as u16, Err(e) => return Err(Status::Err(e)), };
    let len = lo | hi << 8;
    let nlen = nlo | nhi << 8;
    proof {
        assert(len as int == lo as int + 256 * hi as int) by (bit_vector)
            requires lo <= 255, hi <= 255, len == lo | hi << 8;
        assert(nlen as int == nlo as int + 256 * nhi as int) by (bit_vector)
            requires nlo <= 255, nhi <= 255, nlen == nlo | nhi << 8;
        assert((len == !nlen) == (len as int + nlen as int == u16::MAX)) by (bit_vector);
    }
    if len != !nlen { return Err(Status::Err(E::StoredLengthMismatch)); }
    stored_payload(br, output, len, w)
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

    closed spec fn wf(&self) -> bool {
        self.fixed_wf() && self.lit.wf() && self.dist.wf() && self.code.wf()
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
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(w).wf(), final(code).wf(),
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
    let ghost cl_zero_fill = w.observed@;
    let mut cl = [0u8; NCL];
    w.add(NCL);
    proof { assert(w.observed@ == cl_zero_fill + NCL); }
    proof { assert(cl@ =~= Seq::new(NCL as nat, |i: int| 0u8)); }
    let mut i = 0;
    while i < ncode
        invariant br.wf(), br.frame() == old(br).frame(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
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
        ensures final(br).wf(), final(br).frame() == old(br).frame(), final(w).wf(), final(br).concat() == old(br).concat(),
            old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
            final(self).code == old(self).code,
            final(self).fixed_lit == old(self).fixed_lit, final(self).fixed_dist == old(self).fixed_dist,
            final(self).header_construction == old(self).header_construction,
            final(self).header_table_start@.is_some() ==> final(self).header_table_start@ == Some(final(br).position())
                && final(br).position() >= old(br).position() + 16,
            ret.0.is_ok() ==> final(self).header_table_start@.is_some(),
            final(self).lit.wf(), final(self).dist.wf(),
            ret.1@ <= DYNAMIC_LENGTHS + 4 * (nlen + ndist) + 2 * (CANON_FIXED_STEPS + MAX_CODE_BITS + 1),
            final(self).header_table_start@.is_none() ==> ret.1@ <= 3 * DYNAMIC_LENGTHS,
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
            invariant br.wf(), br.frame() == old(br).frame(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
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
            let ghost header_iteration = w.observed@;
            w.add(1);
            proof { assert(w.observed@ == header_iteration + 1); construction = (construction + 1) as nat; }
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
        // Pin the frozen EOB-before-admission boundary at the real build call.
        proof { assert(lengths@[256] > 0); }
        proof { self.header_table_start = Ghost(Some(br.position())); }
        let ghost before_lit = w.observed@;
        let lit_result = self.lit.build(&lengths[..nlen], Kind::LitLen, w);
        proof { construction = (construction + w.observed@ - before_lit) as nat; }
        match lit_result { Ok(()) => {}, Err(e) => return (Err(e), Ghost(construction)), }
        proof { assert(self.lit.ready@); }
        let ghost before_dist = w.observed@;
        let dist_result = self.dist.build(&lengths[nlen..total], Kind::Dist, w);
        proof { construction = (construction + w.observed@ - before_dist) as nat; }
        (dist_result, Ghost(construction))
    }

    fn dynamic<S: Segments>(&mut self, br: &mut Bits<'_, S>, w: &mut Work) -> (r: Result<(), E>)
        requires old(br).wf(), old(w).wf(), old(self).code.wf(), old(self).lit.wf(), old(self).dist.wf(),
        ensures final(br).wf(), final(br).frame() == old(br).frame(), final(w).wf(), final(br).concat() == old(br).concat(),
            old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
            final(self).fixed_lit == old(self).fixed_lit, final(self).fixed_dist == old(self).fixed_dist,
            final(self).code.wf(), final(self).lit.wf(), final(self).dist.wf(),
            final(self).header_construction@ < 2048,
            final(self).header_construction@ <= 48 * (final(br).position() - old(br).position()),
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
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(w).wf(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        final(tables).fixed_lit == old(tables).fixed_lit, final(tables).fixed_dist == old(tables).fixed_dist,
        final(tables).code.wf(), final(tables).lit.wf(), final(tables).dist.wf(),
        final(tables).header_construction@ < 2048,
        final(tables).header_construction@ <= 48 * (final(br).position() - old(br).position()),
        final(tables).header_table_start@.is_some() ==>
            final(tables).header_table_start@ == Some(final(br).position())
            && final(br).position() >= old(br).position() + 45,
        old(w).observed@ <= final(w).observed@ <= old(w).observed@ + final(tables).header_construction@
            + 3 * (final(br).position() - old(br).position() + 1),
        r.is_ok() ==> old(br).position() + 3 <= final(br).position(),
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

verus! {

pub open spec fn fault_eq(error: m::InflateFault, e: E) -> bool {
    match error {
        m::InflateFault::TruncatedZlib => e == E::TruncatedZlib,
        m::InflateFault::InvalidZlibHeader => e == E::InvalidZlibHeader,
        m::InflateFault::UnsupportedZlibMethod => e == E::UnsupportedZlibMethod,
        m::InflateFault::ZlibPresetDictionary => e == E::ZlibPresetDictionary,
        m::InflateFault::InvalidBlockType => e == E::InvalidBlockType,
        m::InflateFault::StoredLengthMismatch => e == E::StoredLengthMismatch,
        m::InflateFault::InvalidCodeLengths => e == E::InvalidCodeLengths,
        m::InflateFault::InvalidSymbol => e == E::InvalidSymbol,
        m::InflateFault::DistanceTooFar => e == E::DistanceTooFar,
        m::InflateFault::AdlerMismatch => e == E::AdlerMismatch,
        m::InflateFault::TrailingZlibData => e == E::TrailingZlibData,
    }
}

closed spec fn copy_eq(effect: m::OutputEffect, result: Result<(), Status>, out: Seq<u8>) -> bool {
    match effect {
        m::OutputEffect::Next { out: expected } => result.is_ok() && out == expected,
        m::OutputEffect::Full { out: expected } => result == Err(Status::Full) && out == expected,
        m::OutputEffect::Error { error, out: expected } => out == expected
            && match result { Err(Status::Err(e)) => fault_eq(error, e), _ => false },
        m::OutputEffect::OutsideDomain => false,
    }
}

/// Actual distance guards and overlapping-copy loop, shared by both sinks.
fn match_copy(output: &mut Output<'_>, window: u64, distance: u64, length: u32, w: &mut Work)
    -> (r: Result<(), Status>)
    requires old(output).wf(), old(w).wf(), 1 <= window <= WINDOW_MAX,
        1 <= distance <= WINDOW_MAX, 3 <= length <= LEN_MAX,
    ensures final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), old(output).produced <= final(output).produced,
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + final(output).produced - old(output).produced + 1,
        r.is_ok() ==> final(w).observed@ == old(w).observed@ + length,
        copy_eq(m::MCopy(old(output).out@, old(output).limit as nat, window as nat, distance as nat,
            length as nat), r, final(output).out@),
{
    hide(Output::wf);
    hide(m::copy_distance);
    let ghost original = output.out@;
    let ghost limit = output.limit;
    proof { output.lemma_observation(); }
    if distance > output.produced || distance > window { return Err(Status::Err(E::DistanceTooFar)); }
    proof { reveal(m::copy_distance); assert(m::copy_distance(original, distance as nat, 0) == original); }
    let mut j = 0u32;
    while j < length
        invariant output.wf(), output.limit == limit, limit == old(output).limit,
            original == old(output).out@, output.initial == old(output).initial,
            sink_identity(output.sink) == sink_identity(old(output).sink),
            1 <= distance <= original.len(), distance <= window <= WINDOW_MAX,
            3 <= length <= LEN_MAX, j <= length,
            output.out@ == m::copy_distance(original, distance as nat, j as nat),
            output.produced == old(output).produced + j,
            w.wf(), w.observed@ == old(w).observed@ + j,
        decreases length - j,
    {
        proof { m::lemma_copy_distance(original, distance as nat, j as nat); output.lemma_observation(); }
        let ghost before = output.out@;
        w.add(1);
        let byte = output.previous(distance);
        match output.append_byte(byte) {
            Ok(()) => {},
            Err(status) => return Err(status),
        }
        proof {
            reveal(m::copy_distance);
            assert(m::copy_distance(original, distance as nat, (j + 1) as nat) == before.push(byte));
        }
        j += 1;
    }
    proof { output.lemma_observation(); }
    Ok(())
}

enum SymbolStep { Continue, End }

closed spec fn symbol_eq(effect: m::SymbolEffect, result: Result<SymbolStep, Status>, out: Seq<u8>, position: nat) -> bool {
    match effect {
        m::SymbolEffect::Next { out: expected, position: end } => result == Ok(SymbolStep::Continue) && out == expected && position == end,
        m::SymbolEffect::End { out: expected, position: end } => result == Ok(SymbolStep::End) && out == expected && position == end,
        m::SymbolEffect::Full { out: expected } => result == Err(Status::Full) && out == expected,
        m::SymbolEffect::Error { error, out: expected, position: end } => out == expected && position == end
            && match result { Err(Status::Err(e)) => fault_eq(error, e), _ => false },
        m::SymbolEffect::OutsideDomain => false,
    }
}

/// One real production symbol iteration. Reserved symbols are rejected before
/// their nonexistent extra bits, and match_copy checks distance before emission.
fn compressed_step<S: Segments>(br: &mut Bits<'_, S>, output: &mut Output<'_>,
    lit: &Huffman, dist: &Huffman, window: u64, w: &mut Work) -> (r: Result<SymbolStep, Status>)
    requires old(br).wf(), old(output).wf(), old(w).wf(),
        lit.wf(), lit.ready@, dist.wf(), dist.ready@, 1 <= window <= WINDOW_MAX,
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        r.is_ok() ==> old(br).position() < final(br).position(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), old(output).produced <= final(output).produced,
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + 8 * (final(br).position() - old(br).position() + 1)
            + final(output).produced - old(output).produced,
        r.is_ok() ==> final(w).observed@ <= old(w).observed@ + 8 * (final(br).position() - old(br).position())
            + final(output).produced - old(output).produced,
        symbol_eq(m::MCompressedStep(old(br).concat(), old(br).position(), old(output).out@,
            old(output).limit as nat, window as nat, lit.lengths@, dist.lengths@), r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(m::MDec);
    hide(m::MMatch);
    let ghost z = br.concat();
    proof { br.lemma_bounds(); output.lemma_observation(); }
    let ghost symbol_iteration = w.observed@;
    w.add(1);
    proof { assert(w.observed@ == symbol_iteration + 1); }
    let sym = match lit.decode(br, w) { Ok(s) => s, Err(e) => return Err(Status::Err(e)), };
    if sym < 256 {
        match output.append_byte(sym as u8) { Ok(()) => {}, Err(s) => return Err(s), }
        Ok(SymbolStep::Continue)
    } else if sym == 256 { Ok(SymbolStep::End) }
    else {
        if sym > 285 { return Err(Status::Err(E::InvalidSymbol)); }
        let index = (sym - 257) as usize;
        proof { lemma_length_tables(index as int); reveal(m::MMatch); }
        let extra = match br.get(LEN_EXTRA[index], w) { Ok(n) => n, Err(e) => return Err(Status::Err(e)), };
        let length = LEN_BASE[index] as u32 + extra;
        let ds = match dist.decode(br, w) { Ok(s) => s as usize, Err(e) => return Err(Status::Err(e)), };
        if ds >= NDIST_MAX { return Err(Status::Err(E::InvalidSymbol)); }
        proof { lemma_distance_tables(ds as int); }
        let de = match br.get(DIST_EXTRA[ds], w) { Ok(n) => n, Err(e) => return Err(Status::Err(e)), };
        let distance = DIST_BASE[ds] as u64 + de as u64;
        match match_copy(output, window, distance, length, w) {
            Ok(()) => Ok(SymbolStep::Continue),
            Err(status) => Err(status),
        }
    }
}

fn compressed<S: Segments>(br: &mut Bits<'_, S>, output: &mut Output<'_>,
    lit: &Huffman, dist: &Huffman, window: u64, w: &mut Work) -> (r: Result<(), Status>)
    requires old(br).wf(), old(output).wf(), old(w).wf(),
        lit.wf(), lit.ready@, dist.wf(), dist.ready@, 1 <= window <= WINDOW_MAX,
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        r.is_ok() ==> old(br).position() < final(br).position(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), old(output).produced <= final(output).produced,
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + 8 * (final(br).position() - old(br).position() + 1)
            + final(output).produced - old(output).produced,
        stored_eq(m::MCompressed(old(br).concat(), old(br).position(), old(output).out@,
            old(output).limit as nat, window as nat, lit.lengths@, dist.lengths@), r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(m::MCompressed);
    hide(m::MCompressedStep);
    let ghost z = br.concat();
    let ghost start = br.position();
    let ghost original = output.out@;
    let ghost limit = output.limit;
    let ghost initial = m::MCompressed(z, start, original, limit as nat, window as nat, lit.lengths@, dist.lengths@);
    proof { br.lemma_bounds(); output.lemma_observation(); }
    loop
        invariant br.wf(), br.frame() == old(br).frame(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
            start <= br.position() <= 8 * z.len(), output.wf(), output.limit == limit, limit == old(output).limit,
            original == old(output).out@, output.initial == old(output).initial,
            sink_identity(output.sink) == sink_identity(old(output).sink), old(output).produced <= output.produced,
            lit.wf(), lit.ready@, dist.wf(), dist.ready@, 1 <= window <= WINDOW_MAX,
            w.wf(), old(w).observed@ <= w.observed@ <= old(w).observed@ + 8 * (br.position() - start)
                + output.produced - old(output).produced,
            initial == m::MCompressed(z, start, original, limit as nat, window as nat, lit.lengths@, dist.lengths@),
            initial == m::MCompressed(z, br.position(), output.out@, limit as nat, window as nat, lit.lengths@, dist.lengths@),
        decreases 8 * z.len() - br.position(),
    {
        proof { reveal(m::MCompressed); }
        match compressed_step(br, output, lit, dist, window, w) {
            Ok(SymbolStep::Continue) => {},
            Ok(SymbolStep::End) => return Ok(()),
            Err(status) => return Err(status),
        }
    }
}

closed spec fn block_eq(effect: (bool, m::StoredEffect), result: Result<bool, Status>, out: Seq<u8>, position: nat) -> bool {
    stored_eq(effect.1, match result { Ok(_) => Ok(()), Err(s) => Err(s) }, out, position)
        && (result.is_ok() ==> result.unwrap() == effect.0)
}

/// The real BTYPE dispatch. All payloads run through the verified production
/// helpers, while MOneBlock selects mathematical code-length profiles only.
fn one_block<S: Segments>(tables: &mut Tables, br: &mut Bits<'_, S>, output: &mut Output<'_>,
    window: u64, w: &mut Work) -> (r: Result<bool, Status>)
    requires old(tables).wf(), old(br).wf(), old(output).wf(), old(w).wf(), 1 <= window <= WINDOW_MAX,
    ensures final(tables).wf(), final(br).wf(), final(br).frame() == old(br).frame(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        r.is_ok() ==> old(br).position() + 3 <= final(br).position(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), old(output).produced <= final(output).produced,
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + 64 * (final(br).position() - old(br).position() + 1)
            + 2 * (final(output).produced - old(output).produced),
        r.is_ok() ==> final(w).observed@ <= old(w).observed@ + 64 * (final(br).position() - old(br).position())
            + 2 * (final(output).produced - old(output).produced),
        block_eq(m::MOneBlock(old(br).concat(), old(br).position(), old(output).out@,
            old(output).limit as nat, window as nat, true), r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(m::MDynamic);
    hide(m::MCompressed);
    hide(m::MStoredBlock);
    proof { br.lemma_bounds(); output.lemma_observation(); }
    let ghost block_iteration = w.observed@;
    w.add(1);
    proof { assert(w.observed@ == block_iteration + 1); }
    let (last, typ) = match read_block_header(tables, br, w) {
        Ok(pair) => pair, Err(e) => return Err(Status::Err(e)),
    };
    match typ {
        0 => match stored_block(br, output, w) { Ok(()) => {}, Err(s) => return Err(s), },
        1 => match compressed(br, output, &tables.fixed_lit, &tables.fixed_dist, window, w) {
            Ok(()) => {}, Err(s) => return Err(s),
        },
        2 => match compressed(br, output, &tables.lit, &tables.dist, window, w) {
            Ok(()) => {}, Err(s) => return Err(s),
        },
        _ => return Err(Status::Err(E::InvalidBlockType)),
    }
    Ok(last != 0)
}

/// Actual block loop, before the zlib checksum/tail. Success means a BFINAL
/// block ended, not that the zlib envelope has been accepted.
fn blocks<S: Segments>(tables: &mut Tables, br: &mut Bits<'_, S>, output: &mut Output<'_>,
    window: u64, w: &mut Work) -> (r: Result<(), Status>)
    requires old(tables).wf(), old(br).wf(), old(output).wf(), old(w).wf(), 1 <= window <= WINDOW_MAX,
    ensures final(tables).wf(), final(br).wf(), final(br).frame() == old(br).frame(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        r.is_ok() ==> old(br).position() + 3 <= final(br).position(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink), old(output).produced <= final(output).produced,
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + 64 * (final(br).position() - old(br).position() + 1)
            + 2 * (final(output).produced - old(output).produced),
        r.is_ok() ==> final(w).observed@ <= old(w).observed@ + 64 * (final(br).position() - old(br).position())
            + 2 * (final(output).produced - old(output).produced),
        stored_eq(m::MBlk(old(br).concat(), old(br).position(), old(output).out@,
            old(output).limit as nat, window as nat, true), r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(m::MBlk);
    hide(m::MOneBlock);
    let ghost z = br.concat();
    let ghost start = br.position();
    let ghost original = output.out@;
    let ghost limit = output.limit;
    let ghost initial = m::MBlk(z, start, original, limit as nat, window as nat, true);
    proof {
        br.lemma_bounds(); output.lemma_observation();
        assert(start <= 8 * z.len());
    }
    loop
        invariant tables.wf(), br.wf(), br.frame() == old(br).frame(), br.concat() == z, z == old(br).concat(), start == old(br).position(),
            start <= br.position() <= 8 * z.len(), output.wf(), output.limit == limit, limit == old(output).limit,
            original == old(output).out@, output.initial == old(output).initial,
            sink_identity(output.sink) == sink_identity(old(output).sink), old(output).produced <= output.produced,
            1 <= window <= WINDOW_MAX, w.wf(), old(w).observed@ <= w.observed@,
            w.observed@ <= old(w).observed@ + 64 * (br.position() - start) + 2 * (output.produced - old(output).produced),
            initial == m::MBlk(z, start, original, limit as nat, window as nat, true),
            initial == m::MBlk(z, br.position(), output.out@, limit as nat, window as nat, true),
        decreases 8 * z.len() - br.position(),
    {
        proof { reveal(m::MBlk); }
        match one_block(tables, br, output, window, w) {
            Ok(last) => if last { return Ok(()); },
            Err(s) => return Err(s),
        }
    }
}

/// Production policy selection after the CMF method/CINFO guard.
fn selected_window(cinfo: u8, enforce: bool) -> (window: u64)
    requires cinfo <= 7,
    ensures window == m::window_size(cinfo as nat, enforce), 1 <= window <= WINDOW_MAX,
{
    proof {
        assert(WINDOW_MAX == m::pow2(15)) by (compute);
        m::lemma_pow2_positive(cinfo as nat + 8);
        m::lemma_pow2_monotone(cinfo as nat + 8, 15);
    }
    if enforce {
        proof {
            m::lemma_pow2_vstd(cinfo as nat + 8);
            vstd::bits::lemma_u64_shl_is_mul(1, (cinfo + 8) as u64);
        }
        1u64 << (cinfo + 8)
    } else { WINDOW_MAX as u64 }
}

/// Actual ordered four-byte checksum read, with a mathematical BE prefix.
fn checksum<S: Segments>(br: &mut Bits<'_, S>, w: &mut Work) -> (r: Result<u32, E>)
    requires old(br).wf(), old(br).remaining == 0, old(w).wf(),
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).remaining == 0, final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        final(w).wf(), old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 8,
        match r {
            Ok(value) => old(br).position() / 8 + 4 <= old(br).concat().len()
                && value == m::be_value(old(br).concat(), old(br).position() / 8, 4)
                && final(br).position() == old(br).position() + 32,
            Err(e) => e == E::TruncatedZlib && old(br).position() / 8 + 4 > old(br).concat().len()
                && final(br).position() == 8 * old(br).concat().len(),
        },
{
    let ghost z = br.concat();
    let ghost start = br.position();
    let ghost cursor = start / 8;
    let mut expected = 0u32;
    let mut i = 0u8;
    proof { br.lemma_bounds(); }
    while i < 4
        invariant br.wf(), br.frame() == old(br).frame(), br.remaining == 0, br.concat() == z, z == old(br).concat(),
            start == old(br).position(), cursor == start / 8, start == 8 * cursor,
            br.position() == start + 8 * i, cursor + i <= z.len(), i <= 4,
            expected == m::be_value(z, cursor, i as nat), expected < m::pow2(8 * i as nat),
            w.wf(), old(w).observed@ <= w.observed@ <= old(w).observed@ + 2 * i,
        decreases 4 - i,
    {
        w.add(1);
        let byte = match br.byte(w) { Ok(b) => b, Err(e) => return Err(e), };
        proof {
            m::lemma_pow2_monotone(8 * i as nat, 24);
            assert(m::pow2(24) == u32::MAX / 256 + 1) by (compute);
            assert(m::pow2(0) == 1 && m::pow2(8) == 256 && m::pow2(16) == u16::MAX as nat + 1
                && m::pow2(24) == u32::MAX / 256 + 1 && m::pow2(32) == 4294967296) by (compute);
            assert(m::pow2(8 * (i + 1) as nat) == 256 * m::pow2(8 * i as nat)) by {
                if i == 0 {} else if i == 1 {} else if i == 2 {} else {}
            }
            let previous = expected;
            assert(((previous << 8) | byte as u32) as int == 256 * previous as int + byte as int) by (bit_vector)
                requires previous < u32::MAX / 256 + 1;
        }
        expected = (expected << 8) | byte as u32;
        i += 1;
    }
    Ok(expected)
}

/// Actual checksum enforcement and exactly-full IgnoreTail decision.
fn finish<S: Segments>(br: &mut Bits<'_, S>, output: &Output<'_>, ignore_tail: bool, w: &mut Work)
    -> (r: Result<(), Status>)
    requires old(br).wf(), output.wf(), old(w).wf(),
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).concat() == old(br).concat(),
        old(br).position() <= final(br).position() <= 8 * final(br).concat().len(),
        final(w).wf(), old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 9,
        stored_eq(m::MFinish(old(br).concat(), old(br).position(), output.out@,
            output.limit as nat, ignore_tail), r, output.out@, final(br).position()),
{
    hide(Output::wf);
    proof { output.lemma_observation(); br.lemma_bounds(); }
    br.align();
    let expected = match checksum(br, w) { Ok(value) => value, Err(e) => return Err(Status::Err(e)), };
    proof { reveal(Output::wf); }
    if expected != output.adler.finish() { return Err(Status::Err(E::AdlerMismatch)); }
    if ignore_tail && output.produced == output.limit { return Ok(()); }
    if br.byte(w).is_ok() { return Err(Status::Err(E::TrailingZlibData)); }
    Ok(())
}

fn run<S: Segments>(br: &mut Bits<'_, S>, output: &mut Output<'_>, o: InflateOptions, w: &mut Work)
    -> (r: Result<(), Status>)
    requires old(br).wf(), old(br).position() == 0, old(br).remaining == 0,
        old(output).wf(), old(output).produced == 0, old(w).wf(),
    ensures final(br).wf(), final(br).frame() == old(br).frame(), final(br).concat() == old(br).concat(),
        final(br).position() <= 8 * final(br).concat().len(),
        final(output).wf(), final(output).limit == old(output).limit,
        final(output).initial == old(output).initial,
        sink_identity(final(output).sink) == sink_identity(old(output).sink),
        final(w).wf(), old(w).observed@ <= final(w).observed@,
        final(w).observed@ <= old(w).observed@ + C_IN * old(br).concat().len()
            + C_OUT * final(output).produced + C_INFL_FIXED,
        stored_eq(m::MEnvelope(old(br).concat(), old(output).limit as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail),
            r, final(output).out@, final(br).position()),
{
    hide(Output::wf);
    hide(m::MBlk);
    hide(m::MFinish);
    proof { output.lemma_observation(); assert(output.out@ =~= Seq::<u8>::empty()); br.lemma_bounds(); }
    let cmf = match br.byte(w) { Ok(b) => b, Err(e) => return Err(Status::Err(e)), };
    let flg = match br.byte(w) { Ok(b) => b, Err(e) => return Err(Status::Err(e)), };
    if (cmf as u16 * 256 + flg as u16) % 31 != 0 { return Err(Status::Err(E::InvalidZlibHeader)); }
    proof {
        assert((cmf & 15) == cmf % 16 && (cmf >> 4) == cmf / 16) by (bit_vector);
        assert((flg & 0x20 != 0) == ((flg as nat / 32) % 2 != 0)) by (bit_vector);
    }
    if cmf & 15 != 8 || cmf >> 4 > 7 { return Err(Status::Err(E::UnsupportedZlibMethod)); }
    if flg & 0x20 != 0 { return Err(Status::Err(E::ZlibPresetDictionary)); }
    let cinfo = cmf >> 4;
    proof { assert(cinfo as nat == cmf as nat / 16) by (bit_vector)
        requires cinfo == cmf >> 4; }
    let window = selected_window(cinfo, matches!(o.window, Window::Enforce));
    proof {
        assert(cinfo as nat == cmf as nat / 16);
        assert(window == m::window_size(cinfo as nat, o.window == Window::Enforce));
        assert(window == m::window_size(cmf as nat / 16, o.window == Window::Enforce));
    }
    let mut tables = match Tables::new(w) { Ok(t) => t, Err(e) => return Err(Status::Err(e)), };
    let ghost z = br.concat();
    let ghost limit = output.limit;
    let ghost effect = m::MBlk(z, 16, Seq::empty(), limit as nat, window as nat, true);
    proof {
        assert(z.len() >= 2);
        assert(cmf == z[0] && flg == z[1]);
        assert((256 * z[0] as nat + z[1] as nat) % 31 == 0);
        assert(z[0] as nat % 16 == 8 && z[0] as nat / 16 <= 7);
        assert((z[1] as nat / 32) % 2 == 0);
        assert(window == m::window_size(z[0] as nat / 16, o.window == Window::Enforce));
        assert(m::MEnvelope(z, limit as nat, o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail)
            == match effect {
                m::StoredEffect::End { out, position } => m::MFinish(z, position, out, limit as nat, o.zlib_tail == ZlibTail::IgnoreTail),
                other => other,
            });
    }
    match blocks(&mut tables, br, output, window, w) {
        Ok(()) => { proof { assert(effect == m::StoredEffect::End { out: output.out@, position: br.position() }); } },
        Err(s) => return Err(s),
    }
    finish(br, output, matches!(o.zlib_tail, ZlibTail::IgnoreTail), w)
}

} // verus!

verus! {

pub open spec fn status_eq(status: Status, expected: m::InflateStatus) -> bool {
    match expected {
        m::InflateStatus::Done => status == Status::Done,
        m::InflateStatus::Full => status == Status::Full,
        m::InflateStatus::Error(fault) => match status { Status::Err(e) => fault_eq(fault, e), _ => false },
        m::InflateStatus::OutsideDomain => false,
    }
}

fn entry<S: Segments>(src: &mut S, sink: Sink<'_>, limit: u64, o: InflateOptions) -> (r: InflateResult)
    requires old(src).wf(), old(src).pos() == 0,
        match sink { Sink::Into(buffer) => buffer.len() == limit, Sink::Ring(ring) => ring.len() == WINDOW_MAX },
    ensures final(src).wf(), final(src).concat() == old(src).concat(),
        final(src).source_frame() == old(src).source_frame(),
        final(src).pos() <= final(src).concat().len(), final(src).seg_count() == old(src).seg_count(),
        status_eq(r.status, m::Inflate(old(src).concat(), limit as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).status),
        r.produced == m::Inflate(old(src).concat(), limit as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out.len(),
        r.produced <= limit, sink_identity(sink).1.len() == sink_view(sink).len(),
        if sink_identity(sink).0 {
            sink_identity(sink).1.take(r.produced as int) == m::Inflate(old(src).concat(), limit as nat,
                o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out
            && sink_identity(sink).1.skip(r.produced as int) == sink_view(sink).skip(r.produced as int)
        } else {
            ring_repr(sink_identity(sink).1, m::Inflate(old(src).concat(), limit as nat,
                o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out)
        },
        r.core_steps@ <= C_IN * old(src).concat().len() + C_OUT * r.produced + C_INFL_FIXED,
        r.seg_steps@ <= C_SEG * (old(src).seg_count() + 1),
        r.core_steps@ + r.seg_steps@ <= C_IN * old(src).concat().len() + C_OUT * r.produced
            + C_INFL_FIXED + C_SEG * (old(src).seg_count() + 1),
        r.seg_steps@ == final(src).seg_steps() - old(src).seg_steps(),
{
    proof { caps_ok(); }
    let ghost count = src.seg_count();
    let ghost origin_seg = src.seg_steps();
    let mut output = Output::new(sink, limit);
    let mut work = Work::new();
    let mut br = Bits::new(src);
    let ghost frame = br.frame();
    proof { assert(frame.4 == count && frame.1 == origin_seg); }
    let status = match run(&mut br, &mut output, o, &mut work) {
        Ok(()) => Status::Done,
        Err(s) => s,
    };
    proof {
        assert(br.frame() == frame);
        br.lemma_frame();
        assert(br.origin_count@ == count);
        br.lemma_bounds(); reveal(Output::wf);
    }
    let seg_steps = br.complete();
    InflateResult {
        status,
        produced: output.produced,
        core_steps: Ghost(work.observed@),
        seg_steps,
    }
}

/// Write at most out.len() bytes; preserve the caller's original unwritten suffix.
/// For S=SegmentSlices, source_frame preserves the complete shared input borrow;
/// all remaining runtime source fields are cursor state. SegmentSlices owns no
/// other mutable borrow, so only out and the source cursor can change.
/// An arbitrary Segments implementation may mutate state it owns: its value
/// frame is not a universal effect restriction (PNG erratum E2, approved with amendment).
pub fn inflate_zlib_into<S: Segments>(src: &mut S, out: &mut [u8], o: InflateOptions) -> (r: InflateResult)
    requires old(src).wf(), old(src).pos() == 0,
    ensures final(src).wf(), final(src).concat() == old(src).concat(),
        final(src).source_frame() == old(src).source_frame(),
        final(src).pos() <= final(src).concat().len(), final(src).seg_count() == old(src).seg_count(),
        status_eq(r.status, m::Inflate(old(src).concat(), old(out).len() as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).status),
        r.produced == m::Inflate(old(src).concat(), old(out).len() as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out.len(),
        r.produced <= old(out).len(), final(out).len() == old(out).len(),
        final(out)@.take(r.produced as int) == m::Inflate(old(src).concat(), old(out).len() as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out,
        final(out)@.skip(r.produced as int) == old(out)@.skip(r.produced as int),
        r.core_steps@ <= C_IN * old(src).concat().len() + C_OUT * r.produced + C_INFL_FIXED,
        r.seg_steps@ <= C_SEG * (old(src).seg_count() + 1),
        r.core_steps@ + r.seg_steps@ <= C_IN * old(src).concat().len() + C_OUT * r.produced
            + C_INFL_FIXED + C_SEG * (old(src).seg_count() + 1),
        r.seg_steps@ == final(src).seg_steps() - old(src).seg_steps(),
{
    let limit = out.len() as u64;
    entry(src, Sink::Into(out), limit, o)
}

/// Measure with exactly WINDOW_MAX caller-owned bytes, initially arbitrary.
/// For S=SegmentSlices, only ring and the source cursor can change; the
/// source_frame postcondition preserves the complete shared input borrow.
/// Generic readers retain their own effects, as documented on into.
pub fn inflate_zlib_measure<S: Segments>(src: &mut S, ring: &mut [u8], limit: u64, o: InflateOptions)
    -> (r: InflateResult)
    requires old(src).wf(), old(src).pos() == 0, old(ring).len() == WINDOW_MAX,
    ensures final(src).wf(), final(src).concat() == old(src).concat(),
        final(src).source_frame() == old(src).source_frame(),
        final(src).pos() <= final(src).concat().len(), final(src).seg_count() == old(src).seg_count(),
        status_eq(r.status, m::Inflate(old(src).concat(), limit as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).status),
        r.produced == m::Inflate(old(src).concat(), limit as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out.len(),
        r.produced <= limit, final(ring).len() == WINDOW_MAX,
        ring_repr(final(ring)@, m::Inflate(old(src).concat(), limit as nat,
            o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out),
        forall|j: int| 0 <= j < r.produced && j < WINDOW_MAX ==>
            #[trigger] final(ring)@[(r.produced as int - 1 - j) % (WINDOW_MAX as int)]
                == m::Inflate(old(src).concat(), limit as nat,
                    o.window == Window::Enforce, o.zlib_tail == ZlibTail::IgnoreTail).out[r.produced as int - 1 - j],
        r.core_steps@ <= C_IN * old(src).concat().len() + C_OUT * r.produced + C_INFL_FIXED,
        r.seg_steps@ <= C_SEG * (old(src).seg_count() + 1),
        r.core_steps@ + r.seg_steps@ <= C_IN * old(src).concat().len() + C_OUT * r.produced
            + C_INFL_FIXED + C_SEG * (old(src).seg_count() + 1),
        r.seg_steps@ == final(src).seg_steps() - old(src).seg_steps(),
{
    assert!(ring.len() == WINDOW_MAX, "inflate measure requires WINDOW_MAX ring bytes");
    entry(src, Sink::Ring(ring), limit, o)
}

} // verus!
