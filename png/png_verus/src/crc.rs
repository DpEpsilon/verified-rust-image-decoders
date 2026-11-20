//! P3 CRC-32, proved against the independent bitwise model (L-CRC-TABLE).
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::model::{crc, crc_bit, crc_bits, crc_register};

verus! {

const POLY: u32 = 0xEDB8_8320;

/// A table entry is eight rounds of reflected polynomial division, without
/// preset or complement: those belong to the whole-message CRC instead.
const fn make_table() -> (t: [u32; 256])
    ensures forall|i: int| 0 <= i < 256 ==> t@[i] == crc_bits(i as u32, 8),
{
    let mut t = [0u32; 256];
    let mut n = 0usize;
    while n < 256
        invariant n <= 256,
            forall|i: int| 0 <= i < n ==> t@[i] == crc_bits(i as u32, 8),
        decreases 256 - n,
    {
        let mut c = n as u32;
        let mut k = 0usize;
        while k < 8
            invariant n < 256, k <= 8, c == crc_bits(n as u32, k as nat),
            decreases 8 - k,
        {
            c = if c & 1 != 0 { POLY ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        t[n] = c;
        n += 1;
    }
    t
}

exec static TABLE: [u32; 256]
    ensures forall|i: int| 0 <= i < 256 ==> TABLE@[i] == crc_bits(i as u32, 8),
{
    make_table()
}

/// L-CRC-TABLE: the full 256-entry production table, not a sampled witness.
proof fn lemma_crc_table(table: &[u32; 256], i: u8)
    requires forall|j: int| 0 <= j < 256 ==> table@[j] == crc_bits(j as u32, 8),
    ensures table@[i as int] == crc_bits(i as u32, 8),
{
}

/// Eight bitwise rounds distribute into the low-byte table remainder and the
/// untouched high 24 bits. No table appears in this algebraic lemma.
proof fn lemma_crc_update(register: u32, byte: u8)
    ensures crc_bits(register ^ byte as u32, 8)
        == crc_bits(((register ^ byte as u32) & 0xFFu32), 8) ^ (register >> 8u32),
{
    reveal_with_fuel(crc_bits, 9);
    assert(crc_bits(register ^ byte as u32, 8)
        == crc_bits(((register ^ byte as u32) & 0xFFu32), 8) ^ (register >> 8u32)) by (bit_vector);
}

/// CRC-32 over the actual borrowed type-and-data slice, with no precondition.
pub(crate) fn crc32(data: &[u8]) -> (result: u32)
    ensures result == crc(data@),
{
    let table = &TABLE;
    let mut c: u32 = 0xFFFF_FFFF;
    let mut i = 0usize;
    while i < data.len()
        invariant i <= data.len(), c == crc_register(data@.subrange(0, i as int)),
            forall|j: int| 0 <= j < 256 ==> table@[j] == crc_bits(j as u32, 8),
        decreases data.len() - i,
    {
        let x = data[i];
        let index = ((c ^ x as u32) & 0xFF) as usize;
        proof {
            assert(((c ^ x as u32) & 0xFFu32) <= 255u32) by (bit_vector);
            lemma_crc_table(table, index as u8);
            lemma_crc_update(c, x);
            assert(data@.subrange(0, i as int + 1).drop_last()
                == data@.subrange(0, i as int));
        }
        c = table[index] ^ (c >> 8);
        i += 1;
    }
    proof { assert(data@.subrange(0, data@.len() as int) == data@); }
    c ^ 0xFFFF_FFFF
}

} // verus!
