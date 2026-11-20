//! P3 wire framing and CRC comparison, shared by the walk and re-walks.
use vstd::prelude::*;
use crate::png::Error;
use crate::crc::crc32;
#[cfg(verus_only)]
use crate::model;

verus! {

#[inline]
pub(crate) fn be32(b: &[u8], p: usize) -> (r: u32)
    requires p <= b.len(), 4 <= b.len() - p,
    ensures r == model::be32(b@, p as int),
{
    ((b[p] as u32) << 24) | ((b[p + 1] as u32) << 16) | ((b[p + 2] as u32) << 8) | (b[p + 3] as u32)
}

#[inline]
pub(crate) fn be16(b: &[u8], p: usize) -> u16
    requires p <= b.len(), 2 <= b.len() - p,
{
    ((b[p] as u16) << 8) | (b[p + 1] as u16)
}

#[derive(Clone, Copy)]
pub(crate) struct Chunk {
    pub ty: [u8; 4],
    pub data: usize,
    pub len: usize,
    pub next: usize,
    pub crc_ok: bool,
}

impl Chunk {
    pub(crate) open spec fn wf(self, bytes: Seq<u8>) -> bool {
        8 <= self.data && self.data <= bytes.len()
        && self.len <= bytes.len() - self.data
        && self.data + self.len + 4 == self.next <= bytes.len()
    }
}

fn letter(c: u8) -> (r: bool)
    ensures r == model::letter(c),
{
    (0x41 <= c && c <= 0x5A) || (0x61 <= c && c <= 0x7A)
}

/// P3 steps 1–3, with the exact first error, for every input and offset.
pub(crate) fn frame_at(b: &[u8], p: usize) -> (r: Result<Chunk, Error>)
    ensures match r {
        Err(e) => model::frame(b@, p as nat) == Err(e),
        Ok(c) => c.wf(b@) && !c.crc_ok && p + 12 <= c.next
            && match model::frame(b@, p as nat) {
                Ok(m) => c.ty@ == m.ty && c.data == m.data && c.len == m.len && c.next == m.next,
                Err(_) => false,
            },
    },
{
    let len = b.len();
    if p > len || len - p < 8 { return Err(Error::TruncatedChunk); }
    let n = be32(b, p);
    let ty = [b[p + 4], b[p + 5], b[p + 6], b[p + 7]];
    proof { assert(ty@ == b@.subrange(p as int + 4, p as int + 8)); }
    if n > crate::limits::PNG_WIRE_MAX { return Err(Error::ChunkTooLong); }
    if !(letter(ty[0]) && letter(ty[1]) && letter(ty[2]) && letter(ty[3])) {
        return Err(Error::InvalidChunkType);
    }
    let n = n as usize;
    if ((len - p - 8) as u64) < n as u64 + 4 { return Err(Error::TruncatedChunk); }
    Ok(Chunk { ty, data: p + 8, len: n, next: p + 12 + n, crc_ok: false })
}

pub(crate) fn crc_matches(b: &[u8], c: &Chunk) -> (r: bool)
    requires c.wf(b@),
    ensures r == (model::crc(b@.subrange(c.data as int - 4, c.data as int + c.len as int))
        == model::be32(b@, c.data as int + c.len as int)),
{
    crc32(&b[c.data - 4..c.data + c.len]) == be32(b, c.data + c.len)
}

/// The caller chooses whether a CRC mismatch is fatal under its policy.
pub(crate) fn chunk_at(b: &[u8], p: usize) -> (r: Result<Chunk, Error>)
    ensures match r {
        Err(e) => model::frame(b@, p as nat) == Err(e),
        Ok(c) => c.wf(b@) && p + 12 <= c.next
            && match model::frame(b@, p as nat) {
                Ok(m) => c.ty@ == m.ty && c.data == m.data && c.len == m.len && c.next == m.next
                    && c.crc_ok == (model::crc(b@.subrange(m.data as int - 4, (m.data + m.len) as int))
                        == model::be32(b@, (m.data + m.len) as int)),
                Err(_) => false,
            },
    },
{
    let mut c = match frame_at(b, p) { Ok(c) => c, Err(e) => return Err(e) };
    c.crc_ok = crc_matches(b, &c);
    Ok(c)
}

} // verus!
