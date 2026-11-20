//! Verified P6–P8 value validation, separate from the plain Info storage layer.
//! Fixed-size arrays hold raw metadata values; variable data remains borrowed
//! through relative byte-range descriptors. No payload is inflated here.
use vstd::prelude::*;
use crate::framing::{be16, be32};
use crate::keywords::{find_nul, split_keyword};
use crate::kind::Kind;
use crate::png::Error;
#[cfg(verus_only)]
use crate::{metadata_model as m, model};

verus! {

pub(crate) enum Value {
    Ignored, Gama(u32), Chrm([u32; 8]), Srgb(u8),
    Iccp { name_len: usize, profile_start: usize, profile_len: usize },
    Sbit([u8; 4], u8), Cicp([u8; 4]), Mdcv([u8; 24]), Clli(u32, u32), Bkgd([u16; 3], u8),
    Phys(u32, u32, u8), Exif(usize), Time(u16, u8, u8, u8, u8, u8),
}
impl Value {
    pub(crate) open spec fn view(self) -> m::Value {
        match self {
            Self::Ignored => m::Value::Ignored, Self::Gama(g) => m::Value::Gama(g),
            Self::Chrm(v) => m::Value::Chrm(v@), Self::Srgb(v) => m::Value::Srgb(v),
            Self::Iccp {name_len,profile_start,profile_len} => m::Value::Iccp {
                name_len: name_len as nat, profile_start: profile_start as nat, profile_len: profile_len as nat },
            Self::Sbit(v,n) => m::Value::Sbit(v@.subrange(0,n as int)),
            Self::Cicp(v) => m::Value::Cicp(v@), Self::Mdcv(v) => m::Value::Mdcv(v@),
            Self::Clli(a,b) => m::Value::Clli(a,b), Self::Bkgd(v,n) => m::Value::Bkgd(v@.subrange(0,n as int)),
            Self::Phys(x,y,u) => m::Value::Phys(x,y,u), Self::Exif(n) => m::Value::Exif(n as nat),
            Self::Time(y,mo,d,h,mi,s) => m::Value::Time(y,mo,d,h,mi,s),
        }
    }
    pub(crate) open spec fn wf(self, size: nat) -> bool {
        match self {
            Self::Iccp {name_len,profile_start,profile_len} => name_len < size
                && profile_start == name_len + 2 && profile_start <= size && profile_len == size - profile_start,
            Self::Sbit(_,n) => n <= 4 && n == size,
            Self::Bkgd(_,n) => n <= 3,
            Self::Exif(n) => n == size,
            _ => true,
        }
    }
}

fn keyword_method(d: &[u8], error: Error) -> (r: Result<(usize, usize, usize), Error>)
    ensures match r {
        Err(e) => m::keyword_method(d@,error) == Err(e),
        Ok((z,start,len)) => m::keyword_method(d@,error) == Ok((z as nat,start as nat,len as nat))
            && z < d.len() && start == z + 2 && start <= d.len() && len == d.len() - start,
    },
{
    let z = match split_keyword(d,error) { Ok(z) => z, Err(e) => return Err(e) };
    if d.len() - z < 2 || d[z + 1] != 0 { return Err(error); }
    Ok((z, z + 2, d.len() - z - 2))
}

fn sbit(d: &[u8], ct: u8, depth: u8, entries: usize) -> (r: Result<Value,Error>)
    requires model::valid_depth(ct,depth), entries <= 256,
    ensures match r {
        Err(e) => m::parse(crate::walk_model::Name::Sbit,d@,ct,depth,entries as nat) == m::Outcome::Error(e),
        Ok(v) => v.wf(d@.len()) && m::parse(crate::walk_model::Name::Sbit,d@,ct,depth,entries as nat) == m::Outcome::Value(v.view()),
    },
{
    let n = d.len();
    let want = match ct { 0 => 1usize, 2 | 3 => 3, 4 => 2, _ => 4 };
    if n != want { return Err(Error::InvalidChunkLength); }
    let top = if ct == 3 {8u8} else {depth};
    let mut v = [0u8; 4];
    let mut i = 0usize;
    while i < n
        invariant i <= n <= 4, n == d.len(), n == m::sbit_length(ct),
            top == if ct == 3 {8u8} else {depth}, model::valid_depth(ct,depth), entries <= 256,
            forall|j: int| 0 <= j < i ==> v@[j] == d@[j] && 1 <= d@[j] <= top,
        decreases n - i,
    {
        let x = d[i];
        if !(1 <= x && x <= top) { return Err(Error::InvalidSbit); }
        v[i] = x;
        i += 1;
    }
    proof { assert(v@.subrange(0,n as int) =~= d@); }
    Ok(Value::Sbit(v,n as u8))
}

fn bkgd(d: &[u8], ct: u8, depth: u8, entries: usize) -> (r: Result<Value,Error>)
    requires model::valid_depth(ct,depth), entries <= 256,
    ensures match r {
        Err(e) => m::parse(crate::walk_model::Name::Bkgd,d@,ct,depth,entries as nat) == m::Outcome::Error(e),
        Ok(v) => v.wf(d@.len()) && m::parse(crate::walk_model::Name::Bkgd,d@,ct,depth,entries as nat) == m::Outcome::Value(v.view()),
    },
{
    let n = d.len();
    let want = match ct { 0 | 4 => 2usize, 2 | 6 => 6, _ => 1 };
    if n != want { return Err(Error::InvalidChunkLength); }
    let mut v = [0u16; 3];
    if ct == 3 {
        if d[0] as usize >= entries { return Err(Error::InvalidBackground); }
        v[0] = d[0] as u16;
        proof { assert(v@.subrange(0,1) =~= seq![d@[0] as u16]); }
        Ok(Value::Bkgd(v,1))
    } else {
        let mut i = 0usize;
        while i < n
            invariant i <= n, n == d.len(), n == m::bkgd_length(ct),
                i % 2 == 0, n == 2 || n == 6, depth <= 16,
                model::valid_depth(ct,depth), ct != 3, entries <= 256,
                forall|j: int| 0 <= j < i / 2 ==> v@[j] == model::be16(d@,2*j)
                    && (v@[j] as nat) < (1u32 << depth) as nat,
            decreases n - i,
        {
            let x = be16(d,i);
            proof {
                assert(((x as u32) >> depth == 0) == ((x as u32) < (1u32 << depth))) by (bit_vector)
                    requires depth <= 16;
            }
            if (x as u32) >> depth != 0 {
                proof {
                    assert(0 <= i / 2 < m::words16(d@,(n/2) as nat).len());
                    assert(m::words16(d@,(n/2) as nat)[(i/2) as int] == x);
                }
                return Err(Error::InvalidBackground);
            }
            v[i / 2] = x;
            i += 2;
        }
        proof { assert(v@.subrange(0,(n/2) as int) =~= m::words16(d@,(n/2) as nat)); }
        Ok(Value::Bkgd(v,(n/2) as u8))
    }
}

pub(crate) fn parse(kind: Kind, d: &[u8], ct: u8, depth: u8, entries: usize) -> (r: Result<Value, Error>)
    requires model::valid_depth(ct,depth), entries <= 256,
    ensures match r {
        Err(e) => m::parse(kind.view(),d@,ct,depth,entries as nat) == m::Outcome::Error(e),
        Ok(v) => v.wf(d@.len()) && m::parse(kind.view(),d@,ct,depth,entries as nat) == m::Outcome::Value(v.view()),
    },
{
    let n = d.len();
    if let Some(want) = kind.fixed_len() {
        if n != want { return Err(Error::InvalidChunkLength); }
    }
    match kind {
        Kind::Gama => {
            let g = be32(d,0);
            if g == 0 { return Err(Error::InvalidGamma); }
            Ok(Value::Gama(g))
        },
        Kind::Chrm => {
            let mut v = [0u32; 8];
            let mut i = 0usize;
            while i < 8
                invariant i <= 8, d.len() == 32,
                    forall|j: int| 0 <= j < i ==> v@[j] == model::be32(d@,4*j),
                decreases 8 - i,
            {
                v[i] = be32(d,4*i);
                i += 1;
            }
            proof { assert(v@ =~= Seq::new(8, |j: int| model::be32(d@,4*j))); }
            Ok(Value::Chrm(v))
        },
        Kind::Srgb => {
            if d[0] > 3 { return Err(Error::InvalidSrgb); }
            Ok(Value::Srgb(d[0]))
        },
        Kind::Iccp => {
            let (name_len,profile_start,profile_len) = match keyword_method(d,Error::InvalidIccp) {
                Ok(v) => v, Err(e) => return Err(e),
            };
            Ok(Value::Iccp {name_len,profile_start,profile_len})
        },
        Kind::Sbit => sbit(d,ct,depth,entries),
        Kind::Cicp => {
            if d[2] != 0 || d[3] > 1 { return Err(Error::InvalidCicp); }
            let v = [d[0],d[1],d[2],d[3]];
            proof { assert(v@ =~= d@); }
            Ok(Value::Cicp(v))
        },
        Kind::Mdcv => {
            let mut v = [0u8; 24];
            let mut i = 0usize;
            while i < 24
                invariant i <= 24, d.len() == 24,
                    forall|j: int| 0 <= j < i ==> v@[j] == d@[j],
                decreases 24 - i,
            { v[i] = d[i]; i += 1; }
            proof { assert(v@ =~= d@); }
            Ok(Value::Mdcv(v))
        },
        Kind::Clli => Ok(Value::Clli(be32(d,0),be32(d,4))),
        Kind::Bkgd => bkgd(d,ct,depth,entries),
        Kind::Hist => {
            if n != 2 * entries { return Err(Error::InvalidHistogram); }
            Ok(Value::Ignored)
        },
        Kind::Phys => {
            if d[8] > 1 { return Err(Error::InvalidPhys); }
            Ok(Value::Phys(be32(d,0),be32(d,4),d[8]))
        },
        Kind::Splt => {
            let z = match split_keyword(d,Error::InvalidSplt) { Ok(z) => z, Err(e) => return Err(e) };
            if n - z < 2 || !(d[z + 1] == 8 || d[z + 1] == 16) { return Err(Error::InvalidSplt); }
            let entry = if d[z + 1] == 8 {6usize} else {10usize};
            if (n - z - 2) % entry != 0 { return Err(Error::InvalidSplt); }
            Ok(Value::Ignored)
        },
        Kind::Exif => {
            if n < 4 || !((d[0] == b'I' && d[1] == b'I' && d[2] == 42 && d[3] == 0)
                || (d[0] == b'M' && d[1] == b'M' && d[2] == 0 && d[3] == 42)) { return Err(Error::InvalidExif); }
            Ok(Value::Exif(n))
        },
        Kind::Time => {
            let (y,mo,day,h,mi,s) = (be16(d,0),d[2],d[3],d[4],d[5],d[6]);
            if !(1 <= mo && mo <= 12 && 1 <= day && day <= 31 && h <= 23 && mi <= 59 && s <= 60) {
                return Err(Error::InvalidTime);
            }
            Ok(Value::Time(y,mo,day,h,mi,s))
        },
        Kind::Text => {
            match split_keyword(d,Error::InvalidText) { Ok(_) => Ok(Value::Ignored), Err(e) => Err(e) }
        },
        Kind::Ztxt => {
            match keyword_method(d,Error::InvalidText) { Ok(_) => Ok(Value::Ignored), Err(e) => Err(e) }
        },
        Kind::Itxt => {
            let z = match split_keyword(d,Error::InvalidText) { Ok(z) => z, Err(e) => return Err(e) };
            if n - z < 3 || d[z + 1] > 1 || (d[z + 1] == 1 && d[z + 2] != 0) { return Err(Error::InvalidText); }
            match find_nul(d,z + 3) {
                Some(language_end) => if find_nul(d,language_end + 1).is_none() { return Err(Error::InvalidText); },
                None => return Err(Error::InvalidText),
            }
            Ok(Value::Ignored)
        },
        _ => Ok(Value::Ignored),
    }
}

pub(crate) fn palette(length: usize, ct: u8, depth: u8, has_idat: bool,
    after_plte_seen: bool, ignore_unexpected: bool, allow_large: bool) -> (r: Result<Option<u32>,Error>)
    requires model::valid_depth(ct,depth),
    ensures match r {
        Err(e) => m::palette(length as nat,ct,depth,has_idat,after_plte_seen,ignore_unexpected,allow_large) == m::Decision::Error(e),
        Ok(None) => m::palette(length as nat,ct,depth,has_idat,after_plte_seen,ignore_unexpected,allow_large) == m::Decision::Drop,
        Ok(Some(entries)) => 1 <= entries <= 256 && 3 * entries == length
            && m::palette(length as nat,ct,depth,has_idat,after_plte_seen,ignore_unexpected,allow_large) == m::Decision::Keep(entries as nat),
    },
{
    if has_idat || after_plte_seen {
        if ignore_unexpected && ct != 3 { return Ok(None); }
        return Err(Error::ChunkOutOfOrder);
    }
    if ct == 0 || ct == 4 {
        if ignore_unexpected { return Ok(None); }
        return Err(Error::UnexpectedPalette);
    }
    if length == 0 || length % 3 != 0 || length > 768 { return Err(Error::InvalidPalette); }
    let entries = (length / 3) as u32;
    if ct == 3 && entries > (1u32 << depth) && !allow_large { return Err(Error::InvalidPalette); }
    Ok(Some(entries))
}

pub(crate) fn transparency(length: usize, ct: u8, entries: usize, drop_invalid: bool) -> (r: Result<bool,Error>)
    requires matches!(ct,0|2|3|4|6), entries <= 256,
    ensures match r {
        Err(e) => m::transparency(length as nat,ct,entries as nat,drop_invalid) == m::Decision::Error(e),
        Ok(false) => m::transparency(length as nat,ct,entries as nat,drop_invalid) == m::Decision::Drop,
        Ok(true) => m::transparency(length as nat,ct,entries as nat,drop_invalid) == m::Decision::Keep(length as nat),
    },
{
    let bad = ct == 4 || ct == 6 || (ct == 0 && length != 2) || (ct == 2 && length != 6)
        || (ct == 3 && !(1 <= length && length <= entries));
    if bad {
        if drop_invalid { Ok(false) } else { Err(Error::InvalidTransparency) }
    } else { Ok(true) }
}

/// Translate a checked payload-relative descriptor into an input range. The
/// content equality is over the actual borrowed slice, without reading/copying.
pub(crate) fn rebase(b: &[u8], base: usize, size: usize, start: usize, count: usize) -> (r: (usize,usize))
    requires base <= b.len(), size <= b.len() - base, start <= size, count <= size - start,
    ensures r.0 == base + start, r.1 == count,
        r.0 <= b.len(), r.1 <= b.len() - r.0,
        b@.subrange(r.0 as int,r.0 as int + r.1 as int)
            == b@.subrange(base as int,base as int + size as int)
                .subrange(start as int,start as int + count as int),
{
    let offset = base + start;
    proof {
        assert(b@.subrange(offset as int,offset as int + count as int)
            =~= b@.subrange(base as int,base as int + size as int)
                .subrange(start as int,start as int + count as int));
    }
    (offset,count)
}

} // verus!
