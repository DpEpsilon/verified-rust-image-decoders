use crate::limits::{ADLER_MOD, ADLER_NMAX};
#[cfg(verus_only)]
use crate::model as m;
use vstd::prelude::*;

verus! {

proof fn adler_format_ok()
    ensures 0 < ADLER_MOD <= u16::MAX, 0 < ADLER_NMAX < usize::MAX,
        (ADLER_MOD - 1) + (u8::MAX as nat) * ADLER_NMAX <= u32::MAX,
        2 * (ADLER_MOD - 1) + 2 * ADLER_NMAX * (ADLER_MOD - 1)
            + (u8::MAX as nat) * ADLER_NMAX * (ADLER_NMAX + 1) <= 2 * (u32::MAX as nat),
{
    assert((ADLER_MOD - 1) + (u8::MAX as nat) * ADLER_NMAX <= u32::MAX) by (compute);
    assert(2 * (ADLER_MOD - 1) + 2 * ADLER_NMAX * (ADLER_MOD - 1)
        + (u8::MAX as nat) * ADLER_NMAX * (ADLER_NMAX + 1) <= 2 * (u32::MAX as nat)) by (compute);
}

proof fn lemma_bound_step(a: nat, b: nat, pending: nat, byte: nat)
    requires pending < ADLER_NMAX, byte <= u8::MAX,
        a <= ADLER_MOD - 1 + (u8::MAX as nat) * pending,
        2 * b <= 2 * (ADLER_MOD - 1) + 2 * pending * (ADLER_MOD - 1)
            + (u8::MAX as nat) * pending * (pending + 1),
    ensures a + byte <= u32::MAX, b + a + byte <= u32::MAX,
        a + byte <= ADLER_MOD - 1 + (u8::MAX as nat) * (pending + 1),
        2 * (b + a + byte) <= 2 * (ADLER_MOD - 1) + 2 * (pending + 1) * (ADLER_MOD - 1)
            + (u8::MAX as nat) * (pending + 1) * (pending + 2),
{
    adler_format_ok();
    let bound_a = (ADLER_MOD - 1 + (u8::MAX as nat) * (pending + 1)) as nat;
    let bound_b = (2 * (ADLER_MOD - 1) + 2 * (pending + 1) * (ADLER_MOD - 1)
        + (u8::MAX as nat) * (pending + 1) * (pending + 2)) as nat;
    assert(a + byte <= bound_a && 2 * (b + a + byte) <= bound_b) by (nonlinear_arith)
        requires a <= ADLER_MOD - 1 + (u8::MAX as nat) * pending,
            2 * b <= 2 * (ADLER_MOD - 1) + 2 * pending * (ADLER_MOD - 1)
                + (u8::MAX as nat) * pending * (pending + 1), byte <= u8::MAX,
            bound_a == ADLER_MOD - 1 + (u8::MAX as nat) * (pending + 1),
            bound_b == 2 * (ADLER_MOD - 1) + 2 * (pending + 1) * (ADLER_MOD - 1)
                + (u8::MAX as nat) * (pending + 1) * (pending + 2);
    assert(bound_a <= ADLER_MOD - 1 + (u8::MAX as nat) * ADLER_NMAX) by (nonlinear_arith)
        requires pending < ADLER_NMAX,
            bound_a == ADLER_MOD - 1 + (u8::MAX as nat) * (pending + 1);
    assert(bound_b <= 2 * (ADLER_MOD - 1) + 2 * ADLER_NMAX * (ADLER_MOD - 1)
        + (u8::MAX as nat) * ADLER_NMAX * (ADLER_NMAX + 1)) by (nonlinear_arith)
        requires pending < ADLER_NMAX, ADLER_MOD > 0,
            bound_b == 2 * (ADLER_MOD - 1) + 2 * (pending + 1) * (ADLER_MOD - 1)
                + (u8::MAX as nat) * (pending + 1) * (pending + 2);
}

proof fn lemma_mod_add_equal(a: nat, b: nat, c: nat, d: nat)
    requires a % (ADLER_MOD as nat) == b % (ADLER_MOD as nat), c % (ADLER_MOD as nat) == d % (ADLER_MOD as nat),
    ensures (a + c) % (ADLER_MOD as nat) == (b + d) % (ADLER_MOD as nat),
{
    adler_format_ok();
    vstd::arithmetic::div_mod::lemma_mod_adds(a as int, c as int, ADLER_MOD as int);
    vstd::arithmetic::div_mod::lemma_mod_adds(b as int, d as int, ADLER_MOD as int);
}

pub(crate) struct Adler {
    pub(crate) a: u32,
    pub(crate) b: u32,
    pub(crate) pending: usize,
    pub(crate) bytes: Ghost<Seq<u8>>,
}
impl Adler {
    pub(crate) closed spec fn data(&self) -> Seq<u8> { self.bytes@ }
    pub(crate) closed spec fn wf(&self) -> bool {
        self.pending < ADLER_NMAX
        && self.a <= ADLER_MOD - 1 + (u8::MAX as nat) * self.pending
        && 2 * self.b <= 2 * (ADLER_MOD - 1) + 2 * self.pending * (ADLER_MOD - 1)
            + (u8::MAX as nat) * self.pending * (self.pending + 1)
        && self.a % ADLER_MOD == (1 + m::sum_bytes(self.bytes@)) % (ADLER_MOD as nat)
        && self.b % ADLER_MOD == m::sum_prefixes(self.bytes@) % (ADLER_MOD as nat)
    }

    pub(crate) fn new() -> (s: Self)
        ensures s.wf(), s.data() == Seq::<u8>::empty(),
    {
        proof { adler_format_ok(); }
        Self { a: 1, b: 0, pending: 0, bytes: Ghost(Seq::empty()) }
    }

    pub(crate) fn update_byte(&mut self, byte: u8)
        requires old(self).wf(),
        ensures final(self).wf(), final(self).data() == old(self).data().push(byte),
    {
        let ghost previous = *self;
        proof {
            adler_format_ok();
            lemma_bound_step(self.a as nat, self.b as nat, self.pending as nat, byte as nat);
            m::lemma_adler_sums_push(self.bytes@, byte);
            lemma_mod_add_equal(self.a as nat, 1 + m::sum_bytes(self.bytes@), byte as nat, byte as nat);
        }
        self.a += byte as u32;
        proof {
            lemma_mod_add_equal(self.b as nat, m::sum_prefixes(self.bytes@), self.a as nat,
                1 + m::sum_bytes(self.bytes@.push(byte)));
        }
        self.b += self.a;
        self.pending += 1;
        if self.pending == ADLER_NMAX {
            proof {
                vstd::arithmetic::div_mod::lemma_mod_twice(self.a as int, ADLER_MOD as int);
                vstd::arithmetic::div_mod::lemma_mod_twice(self.b as int, ADLER_MOD as int);
            }
            self.a %= ADLER_MOD;
            self.b %= ADLER_MOD;
            self.pending = 0;
        }
        proof { self.bytes = Ghost(previous.bytes@.push(byte)); }
    }

    pub(crate) fn finish(&self) -> (checksum: u32)
        requires self.wf(),
        ensures checksum == m::Adler(self.data()),
    {
        proof { adler_format_ok(); }
        let a = self.a % ADLER_MOD;
        let b = self.b % ADLER_MOD;
        proof {
            assert(a <= u16::MAX as u32 && b <= u16::MAX as u32);
            assert((b << 16) | a == b * (u16::MAX as u32 + 1) + a) by (bit_vector)
                requires a <= u16::MAX as u32, b <= u16::MAX as u32;
            m::lemma_pow2_vstd(16);
            assert(m::pow2(16) == u16::MAX + 1) by (compute);
            assert((b as nat) * m::pow2(16) <= u32::MAX) by (nonlinear_arith)
                requires b <= ADLER_MOD - 1, ADLER_MOD <= u16::MAX,
                    m::pow2(16) == u16::MAX + 1;
            vstd::bits::lemma_u32_shl_is_mul(b, 16);
        }
        (b << 16) | a
    }
}

} // verus!
