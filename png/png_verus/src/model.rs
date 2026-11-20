//! Independent P3 checksum definition: reflected polynomial division, one bit
//! at a time. The model never reads the executable lookup table.
use vstd::prelude::*;

verus! {

pub open spec fn crc_bit(register: u32) -> u32 {
    if register & 1u32 != 0 { 0xEDB8_8320u32 ^ (register >> 1u32) }
    else { register >> 1u32 }
}

pub open spec fn crc_bits(register: u32, count: nat) -> u32
    decreases count,
{
    if count == 0 { register }
    else { crc_bit(crc_bits(register, (count - 1) as nat)) }
}

pub open spec fn crc_register(bytes: Seq<u8>) -> u32
    decreases bytes.len(),
{
    if bytes.len() == 0 { 0xFFFF_FFFFu32 }
    else {
        crc_bits(crc_register(bytes.drop_last()) ^ bytes.last() as u32, 8)
    }
}

pub open spec fn crc(bytes: Seq<u8>) -> u32 {
    crc_register(bytes) ^ 0xFFFF_FFFFu32
}

/// P3 wire framing before the CRC policy is applied. Offsets and lengths are
/// mathematical naturals, including on malformed inputs.
pub struct WireChunk {
    pub ty: Seq<u8>,
    pub data: nat,
    pub len: nat,
    pub next: nat,
}

pub open spec fn be32(bytes: Seq<u8>, p: int) -> u32 {
    ((bytes[p] as u32) << 24u32) | ((bytes[p + 1] as u32) << 16u32)
        | ((bytes[p + 2] as u32) << 8u32) | bytes[p + 3] as u32
}

pub open spec fn letter(c: u8) -> bool {
    0x41 <= c <= 0x5A || 0x61 <= c <= 0x7A
}

pub open spec fn frame(bytes: Seq<u8>, p: nat) -> Result<WireChunk, crate::png::Error> {
    use crate::png::Error;
    if p + 8 > bytes.len() { Err(Error::TruncatedChunk) }
    else {
        let n = be32(bytes, p as int) as nat;
        let ty = bytes.subrange(p as int + 4, p as int + 8);
        if n > crate::limits::PNG_WIRE_MAX { Err(Error::ChunkTooLong) }
        else if !(letter(ty[0]) && letter(ty[1]) && letter(ty[2]) && letter(ty[3])) {
            Err(Error::InvalidChunkType)
        } else if p + 12 + n > bytes.len() { Err(Error::TruncatedChunk) }
        else { Ok(WireChunk { ty, data: p + 8, len: n, next: p + 12 + n }) }
    }
}

/// A segment in a lengths-and-types re-walk. Non-fdAT chunks inside a frame
/// are empty segments, so their traversal is charged to the container too.
pub struct ReaderChunk {
    pub begin: nat,
    pub end: nat,
    pub next: nat,
}

pub open spec fn reader_chunk(bytes: Seq<u8>, p: nat, fdat: bool) -> Option<ReaderChunk> {
    match frame(bytes, p) {
        Err(_) => None,
        Ok(c) => {
            if fdat {
                if c.ty == seq![b'I', b'E', b'N', b'D'] || c.ty == seq![b'f', b'c', b'T', b'L'] {
                    None
                } else if c.ty == seq![b'f', b'd', b'A', b'T'] && c.len >= 4 {
                    Some(ReaderChunk { begin: c.data + 4, end: c.data + c.len, next: c.next })
                } else { Some(ReaderChunk { begin: c.data, end: c.data, next: c.next }) }
            } else if c.ty == seq![b'I', b'D', b'A', b'T'] {
                Some(ReaderChunk { begin: c.data, end: c.data + c.len, next: c.next })
            } else { None }
        },
    }
}

pub proof fn reader_chunk_bounds(bytes: Seq<u8>, p: nat, fdat: bool)
    ensures match reader_chunk(bytes, p, fdat) {
        None => true,
        Some(c) => p + 8 <= c.begin <= c.end && c.end + 4 <= c.next <= bytes.len()
            && p + 12 <= c.next,
    },
{
}

pub open spec fn reader_bytes(bytes: Seq<u8>, p: nat, fdat: bool) -> Seq<u8>
    decreases bytes.len() - p,
{
    match reader_chunk(bytes, p, fdat) {
        None => Seq::empty(),
        Some(c) => {
            proof { reader_chunk_bounds(bytes, p, fdat); }
            bytes.subrange(c.begin as int, c.end as int) + reader_bytes(bytes, c.next, fdat)
        },
    }
}

pub open spec fn reader_count(bytes: Seq<u8>, p: nat, fdat: bool) -> nat
    decreases bytes.len() - p,
{
    match reader_chunk(bytes, p, fdat) {
        None => 0,
        Some(c) => {
            proof { reader_chunk_bounds(bytes, p, fdat); }
            1 + reader_count(bytes, c.next, fdat)
        },
    }
}

pub proof fn reader_shape(bytes: Seq<u8>, p: nat, fdat: bool)
    ensures reader_bytes(bytes, p, fdat).len() <= if p <= bytes.len() { bytes.len() - p } else { 0 },
        12 * reader_count(bytes, p, fdat) <= if p <= bytes.len() { bytes.len() - p } else { 0 },
    decreases bytes.len() - p,
{
    reader_chunk_bounds(bytes, p, fdat);
    match reader_chunk(bytes, p, fdat) {
        None => {},
        Some(c) => { reader_shape(bytes, c.next, fdat); },
    }
}

pub struct Ihdr {
    pub width: u32,
    pub height: u32,
    pub depth: u8,
    pub color_type: u8,
    pub interlace: u8,
}

pub open spec fn valid_depth(ct: u8, depth: u8) -> bool {
    (ct == 0 && (depth == 1 || depth == 2 || depth == 4 || depth == 8 || depth == 16))
    || ((ct == 2 || ct == 4 || ct == 6) && (depth == 8 || depth == 16))
    || (ct == 3 && (depth == 1 || depth == 2 || depth == 4 || depth == 8))
}

/// P4 checks, in their specified order, on the thirteen IHDR data bytes.
pub open spec fn ihdr(bytes: Seq<u8>) -> Result<Ihdr, crate::png::Error> {
    use crate::png::Error;
    use crate::limits::{MAX_DIMENSION, MAX_OUTPUT};
    if bytes.len() != 13 { Err(Error::InvalidChunkLength) }
    else {
        let w = be32(bytes, 0);
        let h = be32(bytes, 4);
        if !(1 <= w <= MAX_DIMENSION && 1 <= h <= MAX_DIMENSION) { Err(Error::InvalidDimensions) }
        else if 4 * (w as int) * (h as int) > MAX_OUTPUT { Err(Error::OutputTooLarge) }
        else if !valid_depth(bytes[9], bytes[8]) { Err(Error::InvalidColorTypeDepth) }
        else if bytes[10] != 0 { Err(Error::UnsupportedCompressionMethod) }
        else if bytes[11] != 0 { Err(Error::UnsupportedFilterMethod) }
        else if bytes[12] > 1 { Err(Error::UnsupportedInterlaceMethod) }
        else { Ok(Ihdr { width: w, height: h, depth: bytes[8], color_type: bytes[9], interlace: bytes[12] }) }
    }
}

pub open spec fn signature(bytes: Seq<u8>) -> bool {
    bytes.len() >= 8 && bytes[0] == 0x89 && bytes[1] == 0x50 && bytes[2] == 0x4E && bytes[3] == 0x47
        && bytes[4] == 0x0D && bytes[5] == 0x0A && bytes[6] == 0x1A && bytes[7] == 0x0A
}

/// MW's P2–P4 prefix (MH); success also gives the next chunk boundary.
pub open spec fn mh(bytes: Seq<u8>) -> Result<(Ihdr, nat), crate::png::Error> {
    use crate::png::Error;
    if bytes.len() > crate::limits::MAX_INPUT { Err(Error::FileTooLarge) }
    else if !signature(bytes) { Err(Error::BadSignature) }
    else {
        match frame(bytes, 8) {
            Err(e) => Err(e),
            Ok(c) => {
                if crc(bytes.subrange(c.data as int - 4, (c.data + c.len) as int))
                    != be32(bytes, (c.data + c.len) as int) { Err(Error::CrcMismatch) }
                else if c.ty != seq![b'I', b'H', b'D', b'R'] { Err(Error::MissingIhdr) }
                else if c.len != 13 { Err(Error::InvalidChunkLength) }
                else {
                    match ihdr(bytes.subrange(c.data as int, (c.data + 13) as int)) {
                        Err(e) => Err(e),
                        Ok(h) => Ok((h, c.next)),
                    }
                }
            },
        }
    }
}

/// P8.2 is a predicate over the complete byte sequence, not a scan state.
pub open spec fn keyword(bytes: Seq<u8>) -> bool {
    1 <= bytes.len() <= 79
    && bytes[0] != 0x20 && bytes[bytes.len() - 1] != 0x20
    && (forall|i: int| 0 <= i < bytes.len() ==> (0x20 <= #[trigger] bytes[i] <= 0x7E || bytes[i] >= 0xA1))
    && (forall|i: int| 1 <= i < bytes.len() ==> !(#[trigger] bytes[i] == 0x20 && bytes[i - 1] == 0x20))
}

pub open spec fn first_nul(bytes: Seq<u8>, from: nat) -> Option<nat>
    decreases bytes.len() - from,
{
    if from >= bytes.len() { None }
    else if bytes[from as int] == 0 { Some(from) }
    else { first_nul(bytes, from + 1) }
}

pub open spec fn split_keyword(bytes: Seq<u8>, error: crate::png::Error) -> Result<nat, crate::png::Error> {
    match first_nul(bytes, 0) {
        Some(z) => if keyword(bytes.subrange(0, z as int)) { Ok(z) } else { Err(error) },
        None => Err(error),
    }
}

} // verus!
