#[cfg(verus_only)]
use crate::model as m;
use crate::{InflateError, Segments, Work};
use vstd::prelude::*;

verus! {

pub(crate) proof fn lemma_cache_pop(z: Seq<u8>, cursor: nat, remaining: nat, current: u8)
    requires 1 <= remaining <= 8, 1 <= cursor <= z.len(),
        current == z[(cursor - 1) as int] as nat / m::pow2((8 - remaining) as nat),
    ensures
        m::bit(z, (8 * cursor - remaining) as nat) == (current % 2 == 1),
        remaining > 1 ==> current as nat / 2
            == z[(cursor - 1) as int] as nat / m::pow2((9 - remaining) as nat),
        remaining == 1 ==> current / 2 == 0,
{
    let used = (8 - remaining) as nat;
    let position = (8 * cursor - remaining) as nat;
    assert(position == 8 * (cursor - 1) + used);
    vstd::arithmetic::div_mod::lemma_div_multiples_vanish_fancy((cursor - 1) as int, used as int, 8);
    vstd::arithmetic::div_mod::lemma_mod_multiples_vanish((cursor - 1) as int, used as int, 8);
    m::lemma_pow2_positive(used);
    vstd::arithmetic::div_mod::lemma_div_denominator(z[(cursor - 1) as int] as int,
        m::pow2(used) as int, 2);
    assert(m::pow2(used + 1) == 2 * m::pow2(used));
    if remaining == 1 {
        assert(m::pow2(7) == 128) by (compute);
        assert(current < 2);
    }
}

/// The real one-byte cache. Origins measure work deltas, never reset the source.
pub(crate) struct Bits<'a, S: Segments> {
    pub src: &'a mut S,
    pub(crate) current: u8,
    pub(crate) remaining: u8,
    pub(crate) origin_seg: Ghost<nat>,
    pub(crate) origin_core: Ghost<nat>,
    pub(crate) origin_rank: Ghost<nat>,
    pub(crate) origin_count: Ghost<nat>,
    #[cfg(verus_only)]
    pub(crate) origin_source: Ghost<S::Frame>,
    pub(crate) loop_work: Ghost<nat>,
}

impl<'a, S: Segments> Bits<'a, S> {
    pub(crate) closed spec fn concat(&self) -> Seq<u8> { self.src.concat() }
    pub(crate) closed spec fn position(&self) -> nat { (8 * self.src.pos() - self.remaining) as nat }
    pub(crate) closed spec fn measured_seg(&self) -> nat { (self.src.seg_steps() - self.origin_seg@) as nat }
    pub(crate) closed spec fn measured_core(&self) -> nat {
        self.loop_work@ + (self.src.core_steps() - self.origin_core@) as nat
    }
    closed spec fn cache_wf(&self) -> bool {
        self.src.wf() && self.remaining <= 8 && self.remaining <= 8 * self.src.pos()
        && (if self.remaining == 0 { self.current == 0 }
            else { self.src.pos() > 0 && self.current
                == self.src.concat()[(self.src.pos() - 1) as int] as nat
                    / m::pow2((8 - self.remaining) as nat) })
        && self.src.seg_count() == self.origin_count@
        && self.src.source_frame() == self.origin_source@
        && self.src.seg_steps() >= self.origin_seg@
        && self.src.core_steps() >= self.origin_core@
        && self.src.advance_rank() >= self.origin_rank@
        && self.src.seg_steps() - self.origin_seg@ == self.src.advance_rank() - self.origin_rank@
    }

    #[verifier::prophetic]
    pub(crate) closed spec fn frame(&self) -> (S, nat, nat, nat, nat, S::Frame) {
        (*final(self.src), self.origin_seg@, self.origin_core@, self.origin_rank@, self.origin_count@, self.origin_source@)
    }

    pub(crate) proof fn lemma_frame(&self)
        ensures self.frame().1 == self.origin_seg@, self.frame().2 == self.origin_core@,
            self.frame().3 == self.origin_rank@, self.frame().4 == self.origin_count@,
            self.frame().5 == self.origin_source@,
    {}

    pub(crate) closed spec fn wf(&self) -> bool { self.cache_wf() && self.remaining < 8 }

    pub(crate) proof fn lemma_bounds(&self)
        requires self.wf(),
        ensures self.position() <= 8 * self.concat().len(),
            self.remaining == 0 ==> self.position() % 8 == 0,
            self.measured_seg() <= crate::limits::C_SEG * (self.origin_count@ + 1),
    {
        assert(self.src.wf());
        self.src.lemma_wf();
        assert(self.src.pos() <= self.src.concat().len());
        assert(self.src.advance_rank() <= self.src.seg_count() + 1);
    }

    pub(crate) fn new(src: &'a mut S) -> (b: Self)
        requires old(src).wf(),
        ensures b.wf(), b.concat() == old(src).concat(), b.position() == 8 * old(src).pos(),
            b.measured_seg() == 0, b.measured_core() == 0,
            b.remaining == 0,
            b.frame() == (*final(src), old(src).seg_steps(), old(src).core_steps(), old(src).advance_rank(), old(src).seg_count(), old(src).source_frame()),
    {
        let origin_seg = Ghost(src.seg_steps());
        let origin_core = Ghost(src.core_steps());
        let origin_rank = Ghost(src.advance_rank());
        let origin_count = Ghost(src.seg_count());
        let origin_source = Ghost(src.source_frame());
        Self { src, current: 0, remaining: 0, origin_seg, origin_core, origin_rank,
            origin_count, #[cfg(verus_only)] origin_source, loop_work: Ghost(0nat) }
    }

    /// Release the source borrow and export observations relative to this reader's origins.
    pub(crate) fn complete(self) -> (steps: Ghost<nat>)
        requires self.wf(),
        ensures self.frame().0.wf(), self.frame().0.concat() == self.concat(),
            self.frame().0.pos() <= self.concat().len(),
            self.frame().0.seg_count() == self.origin_count@,
            self.frame().0.source_frame() == self.frame().5,
            steps@ == self.frame().0.seg_steps() - self.origin_seg@,
            steps@ <= crate::limits::C_SEG * (self.origin_count@ + 1),
    {
        proof { self.lemma_bounds(); self.src.lemma_wf(); }
        Ghost(self.measured_seg())
    }

    pub(crate) fn byte(&mut self, w: &mut Work) -> (r: Result<u8, InflateError>)
        requires old(self).wf(), old(self).remaining == 0,
            old(w).wf(),
        ensures final(self).wf(), final(self).frame() == old(self).frame(), final(w).wf(),
            final(self).remaining == 0, final(self).current == 0,
            final(self).origin_seg == old(self).origin_seg,
            final(self).origin_core == old(self).origin_core,
            final(self).origin_rank == old(self).origin_rank,
            final(self).origin_count == old(self).origin_count,
            final(self).loop_work == old(self).loop_work,
            final(self).concat() == old(self).concat(),
            old(self).measured_core() <= final(self).measured_core() <= old(self).measured_core() + 1,
            final(w).observed@ - old(w).observed@
                == final(self).measured_core() - old(self).measured_core(),
            old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 1,
            final(self).measured_seg() <= crate::limits::C_SEG * (final(self).src.seg_count() + 1),
            r.is_ok() ==> final(w).observed@ == old(w).observed@ + 1,
            match r {
                Ok(byte) => old(self).position() < 8 * old(self).concat().len()
                    && byte == old(self).concat()[(old(self).position() / 8) as int]
                    && final(self).position() == old(self).position() + 8,
                Err(e) => e == InflateError::TruncatedZlib
                    && old(self).position() == 8 * old(self).concat().len()
                    && final(self).position() == old(self).position(),
            },
    {
        let ghost before = self.src.core_steps();
        let read = self.src.read();
        proof { w.observed = Ghost((w.observed@ + self.src.core_steps() - before) as nat); }
        match read {
            Some(byte) => Ok(byte),
            None => Err(InflateError::TruncatedZlib),
        }
    }

    pub(crate) fn get(&mut self, n: u8, w: &mut Work) -> (r: Result<u32, InflateError>)
        requires old(self).wf(), n <= 32, old(w).wf(),
        ensures final(self).wf(), final(self).frame() == old(self).frame(), final(w).wf(), final(self).concat() == old(self).concat(),
            old(self).position() <= final(self).position() <= old(self).position() + n,
            final(self).position() <= 8 * final(self).concat().len(),
            final(self).measured_seg() <= crate::limits::C_SEG * (final(self).src.seg_count() + 1),
            old(w).observed@ <= final(w).observed@ <= old(w).observed@ + 2 * n,
            final(w).observed@ <= old(w).observed@ + 2 * (final(self).position() - old(self).position() + 1),
            r.is_ok() ==> final(w).observed@ <= old(w).observed@ + 2 * (final(self).position() - old(self).position()),
            final(w).observed@ - old(w).observed@
                == final(self).measured_core() - old(self).measured_core(),
            match r {
                Ok(value) => old(self).position() + n <= 8 * old(self).concat().len()
                    && final(self).position() == old(self).position() + n
                    && value == m::field_value(old(self).concat(), old(self).position(), n as nat)
                    && value < m::pow2(n as nat),
                Err(e) => e == InflateError::TruncatedZlib
                    && old(self).position() + n > 8 * old(self).concat().len()
                    && final(self).position() == 8 * old(self).concat().len(),
            },
    {
        let ghost initial = *self;
        let mut value = 0u32;
        let mut i = 0u8;
        while i < n
            invariant initial == *old(self), self.wf(), self.frame() == initial.frame(), w.wf(), i <= n <= 32,
                self.concat() == initial.concat(),
                self.position() == initial.position() + i,
                value == m::field_value(initial.concat(), initial.position(), i as nat),
                old(w).observed@ <= w.observed@ <= old(w).observed@ + 2 * i,
                w.observed@ - old(w).observed@ == self.measured_core() - initial.measured_core(),
            decreases n - i,
        {
            w.add(1);
            proof { self.loop_work = Ghost(self.loop_work@ + 1); }
            if self.remaining == 0 {
                let byte = match self.byte(w) {
                    Ok(byte) => byte,
                    Err(e) => return Err(e),
                };
                proof {
                    assert(self.cache_wf());
                    assert(self.src.pos() > 0);
                    assert(byte == self.concat()[(self.src.pos() - 1) as int]);
                    assert(m::pow2(0) == 1) by (compute);
                }
                self.current = byte;
                self.remaining = 8;
                proof {
                    assert(self.src.wf());
                    assert(self.remaining <= 8 && self.remaining <= 8 * self.src.pos());
                    assert(self.current == self.src.concat()[(self.src.pos() - 1) as int] as nat / m::pow2(0));
                    assert(self.src.seg_count() == self.origin_count@);
                    assert(self.src.seg_steps() >= self.origin_seg@);
                    assert(self.src.core_steps() >= self.origin_core@);
                    assert(self.src.advance_rank() >= self.origin_rank@);
                    assert(self.src.seg_steps() - self.origin_seg@ == self.src.advance_rank() - self.origin_rank@);
                    assert(self.cache_wf()); assert(self.position() == initial.position() + i);
                }
            }
            let ghost before = self.position();
            let ghost previous_value = value;
            let low = self.current & 1;
            proof {
                assert(self.cache_wf());
                assert(self.src.wf());
                self.src.lemma_wf();
                assert(1 <= self.src.pos() <= self.concat().len());
                lemma_cache_pop(self.concat(), self.src.pos(), self.remaining as nat, self.current);
                let current = self.current;
                assert(current & 1u8 == current % 2u8) by (bit_vector);
                assert(m::bit(initial.concat(), initial.position() + i as nat) == (low == 1));
                m::lemma_field_bound(initial.concat(), initial.position(), i as nat);
                m::lemma_pow2_monotone(i as nat, 31);
                m::lemma_pow2_monotone((i + 1) as nat, 32);
                m::lemma_pow2_vstd(i as nat);
                assert(m::pow2(31) <= u32::MAX) by (compute);
                assert(m::pow2(32) == u32::MAX + 1) by (compute);
                assert(low <= 1);
                assert(low as nat * m::pow2(i as nat) <= u32::MAX) by (nonlinear_arith)
                    requires low <= 1, m::pow2(i as nat) <= u32::MAX;
                vstd::bits::lemma_u32_shl_is_mul(low as u32, i as u32);
                assert((value as nat) + (low as nat) * m::pow2(i as nat) < m::pow2((i + 1) as nat)) by (nonlinear_arith)
                    requires value < m::pow2(i as nat), low <= 1,
                        m::pow2((i + 1) as nat) == 2 * m::pow2(i as nat);
            }
            value += (low as u32) << i;
            proof {
                let current = self.current;
                assert(current >> 1u8 == current / 2u8) by (bit_vector);
            }
            self.current >>= 1;
            self.remaining -= 1;
            proof {
                assert(self.position() == before + 1);
                assert(value == previous_value + low as nat * m::pow2(i as nat));
                assert(m::field_value(initial.concat(), initial.position(), (i + 1) as nat)
                    == previous_value + if low == 1 { m::pow2(i as nat) } else { 0nat });
                if low == 1 {} else { assert(low == 0); }
                assert(value == m::field_value(initial.concat(), initial.position(), (i + 1) as nat));
            }
            i += 1;
        }
        proof { self.lemma_bounds(); m::lemma_field_bound(initial.concat(), initial.position(), n as nat); }
        Ok(value)
    }

    pub(crate) fn align(&mut self)
        requires old(self).wf(),
        ensures final(self).wf(), final(self).frame() == old(self).frame(), final(self).concat() == old(self).concat(),
            final(self).remaining == 0,
            final(self).position() == old(self).position() + old(self).remaining,
            final(self).position() % 8 == 0,
            final(self).position() == ((old(self).position() + 7) / 8) * 8,
            final(self).measured_core() == old(self).measured_core(),
            final(self).measured_seg() == old(self).measured_seg(),
    {
        self.current = 0;
        self.remaining = 0;
    }
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SegmentSlices;

    #[test]
    fn full_width_read_crosses_empty_segments_and_eof_is_stable() {
        let a = [0x78, 0x56];
        let b = [0x34, 0x12];
        let parts: &[&[u8]] = &[&[], &a, &[], &b, &[]];
        let mut src = SegmentSlices::new(parts).unwrap();
        let mut br = Bits::new(&mut src);
        let mut w = Work::new();
        assert_eq!(br.get(32, &mut w), Ok(0x12345678));
        assert_eq!(br.get(1, &mut w), Err(InflateError::TruncatedZlib));
        let advances = br.src.seg_index();
        assert_eq!(br.get(1, &mut w), Err(InflateError::TruncatedZlib));
        assert_eq!(br.src.seg_index(), advances);
    }

    #[test]
    fn failed_read_consumes_the_remaining_cached_prefix() {
        let input = [0b10101101];
        let parts: &[&[u8]] = &[&input];
        let mut src = SegmentSlices::new(parts).unwrap();
        let mut br = Bits::new(&mut src);
        let mut w = Work::new();
        assert_eq!(br.get(3, &mut w), Ok(5));
        assert_eq!(br.get(6, &mut w), Err(InflateError::TruncatedZlib));
        assert_eq!(br.remaining, 0);
        assert_eq!(br.current, 0);
        assert_eq!(br.get(0, &mut w), Ok(0));
    }
}
