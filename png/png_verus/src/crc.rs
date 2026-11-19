//! P3 CRC-32 (ISO 3309 / ITU-T V.42): reflected polynomial `EDB88320`, register
//! preset to all ones, result complemented. The table is built at compile time
//! from the bitwise definition (L-CRC-TABLE at M3).

const POLY: u32 = 0xEDB8_8320;

const fn make_table() -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut n = 0usize;
    while n < 256 {
        let mut c = n as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { POLY ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        t[n] = c;
        n += 1;
    }
    t
}

static TABLE: [u32; 256] = make_table();

/// CRC-32 over `data` (the chunk type and data bytes, P3 step 4).
pub(crate) fn crc32(data: &[u8]) -> u32 {
    let mut c: u32 = 0xFFFF_FFFF;
    for &x in data {
        c = TABLE[((c ^ x as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}
