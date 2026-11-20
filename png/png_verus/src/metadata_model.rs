//! Independent P6–P8 value rules. Byte ranges are relative to the chunk data;
//! storing them in a returned Info remains a separate composition theorem.
use vstd::prelude::*;
use crate::model::{be16, be32, first_nul, split_keyword, valid_depth};
use crate::png::Error;
use crate::walk_model::{Name, fixed_length};

verus! {

pub enum Value {
    Ignored, Gama(u32), Chrm(Seq<u32>), Srgb(u8),
    Iccp { name_len: nat, profile_start: nat, profile_len: nat },
    Sbit(Seq<u8>), Cicp(Seq<u8>), Mdcv(Seq<u8>), Clli(u32, u32), Bkgd(Seq<u16>),
    Phys(u32, u32, u8), Exif(nat), Time(u16, u8, u8, u8, u8, u8),
}
pub enum Outcome { Value(Value), Error(Error), OutsideDomain }

pub open spec fn keyword_method(d: Seq<u8>, error: Error) -> Result<(nat, nat, nat), Error> {
    match split_keyword(d, error) {
        Err(e) => Err(e),
        Ok(z) => if z + 1 >= d.len() || d[z as int + 1] != 0 { Err(error) }
            else { Ok((z, z + 2, (d.len() - z - 2) as nat)) },
    }
}
pub open spec fn sbit_length(ct: u8) -> nat {
    match ct { 0 => 1, 2 | 3 => 3, 4 => 2, _ => 4 }
}
pub open spec fn bkgd_length(ct: u8) -> nat {
    match ct { 0 | 4 => 2, 2 | 6 => 6, _ => 1 }
}
pub open spec fn words16(d: Seq<u8>, count: nat) -> Seq<u16> {
    Seq::new(count, |i: int| be16(d, 2 * i))
}
pub open spec fn exif_prefix(d: Seq<u8>) -> bool {
    d.len() >= 4 && ((d[0] == b'I' && d[1] == b'I' && d[2] == 42 && d[3] == 0)
        || (d[0] == b'M' && d[1] == b'M' && d[2] == 0 && d[3] == 42))
}

pub open spec fn parse(name: Name, d: Seq<u8>, ct: u8, depth: u8, entries: nat) -> Outcome {
    if !valid_depth(ct, depth) || entries > 256 { Outcome::OutsideDomain }
    else if fixed_length(name).is_some() && d.len() != fixed_length(name).unwrap() {
        Outcome::Error(Error::InvalidChunkLength)
    } else {
        match name {
            Name::Gama => if be32(d,0) == 0 { Outcome::Error(Error::InvalidGamma) }
                else { Outcome::Value(Value::Gama(be32(d,0))) },
            Name::Chrm => Outcome::Value(Value::Chrm(Seq::new(8, |i: int| be32(d,4*i)))),
            Name::Srgb => if d[0] > 3 { Outcome::Error(Error::InvalidSrgb) }
                else { Outcome::Value(Value::Srgb(d[0])) },
            Name::Iccp => match keyword_method(d, Error::InvalidIccp) {
                Err(e) => Outcome::Error(e),
                Ok((name_len,profile_start,profile_len)) => Outcome::Value(Value::Iccp {name_len,profile_start,profile_len}),
            },
            Name::Sbit => if d.len() != sbit_length(ct) { Outcome::Error(Error::InvalidChunkLength) }
                else if !(forall|i: int| 0 <= i < d.len() ==> 1 <= #[trigger] d[i] <= if ct == 3 {8u8} else {depth}) {
                    Outcome::Error(Error::InvalidSbit)
                } else { Outcome::Value(Value::Sbit(d)) },
            Name::Cicp => if d[2] != 0 || d[3] > 1 { Outcome::Error(Error::InvalidCicp) }
                else { Outcome::Value(Value::Cicp(d)) },
            Name::Mdcv => Outcome::Value(Value::Mdcv(d)),
            Name::Clli => Outcome::Value(Value::Clli(be32(d,0),be32(d,4))),
            Name::Bkgd => if d.len() != bkgd_length(ct) { Outcome::Error(Error::InvalidChunkLength) }
                else if ct == 3 {
                    if d[0] as nat >= entries { Outcome::Error(Error::InvalidBackground) }
                    else { Outcome::Value(Value::Bkgd(seq![d[0] as u16])) }
                } else {
                    let samples = words16(d, d.len()/2);
                    if !(forall|i: int| 0 <= i < samples.len() ==> (#[trigger] samples[i] as nat) < (1u32 << depth) as nat) {
                        Outcome::Error(Error::InvalidBackground)
                    } else { Outcome::Value(Value::Bkgd(samples)) }
                },
            Name::Hist => if d.len() != 2 * entries { Outcome::Error(Error::InvalidHistogram) }
                else { Outcome::Value(Value::Ignored) },
            Name::Phys => if d[8] > 1 { Outcome::Error(Error::InvalidPhys) }
                else { Outcome::Value(Value::Phys(be32(d,0),be32(d,4),d[8])) },
            Name::Splt => match split_keyword(d, Error::InvalidSplt) {
                Err(e) => Outcome::Error(e),
                Ok(z) => if z + 1 >= d.len() || !(d[z as int + 1] == 8 || d[z as int + 1] == 16) {
                    Outcome::Error(Error::InvalidSplt)
                } else if (d.len() - z - 2) % (if d[z as int + 1] == 8 {6int} else {10int}) != 0 {
                    Outcome::Error(Error::InvalidSplt)
                } else { Outcome::Value(Value::Ignored) },
            },
            Name::Exif => if !exif_prefix(d) { Outcome::Error(Error::InvalidExif) }
                else { Outcome::Value(Value::Exif(d.len())) },
            Name::Time => if !(1 <= d[2] <= 12 && 1 <= d[3] <= 31 && d[4] <= 23 && d[5] <= 59 && d[6] <= 60) {
                    Outcome::Error(Error::InvalidTime)
                } else { Outcome::Value(Value::Time(be16(d,0),d[2],d[3],d[4],d[5],d[6])) },
            Name::Text => match split_keyword(d, Error::InvalidText) {
                Err(e) => Outcome::Error(e), Ok(_) => Outcome::Value(Value::Ignored),
            },
            Name::Ztxt => match keyword_method(d, Error::InvalidText) {
                Err(e) => Outcome::Error(e), Ok(_) => Outcome::Value(Value::Ignored),
            },
            Name::Itxt => match split_keyword(d, Error::InvalidText) {
                Err(e) => Outcome::Error(e),
                Ok(z) => if z + 2 >= d.len() || d[z as int + 1] > 1 || (d[z as int + 1] == 1 && d[z as int + 2] != 0) {
                    Outcome::Error(Error::InvalidText)
                } else {
                    match first_nul(d, z + 3) {
                        None => Outcome::Error(Error::InvalidText),
                        Some(language_end) => if first_nul(d, language_end + 1).is_none() {
                            Outcome::Error(Error::InvalidText)
                        } else { Outcome::Value(Value::Ignored) },
                    }
                },
            },
            _ => Outcome::Value(Value::Ignored),
        }
    }
}

/// P6/P7 decisions: drop is a policy result, not a format success with data.
pub enum Decision { OutsideDomain, Keep(nat), Drop, Error(Error) }

pub open spec fn palette(length: nat, ct: u8, depth: u8, has_idat: bool,
    after_plte_seen: bool, ignore_unexpected: bool, allow_large: bool) -> Decision
{
    if !valid_depth(ct,depth) { Decision::OutsideDomain }
    else if has_idat || after_plte_seen {
        if ignore_unexpected && ct != 3 { Decision::Drop } else { Decision::Error(Error::ChunkOutOfOrder) }
    } else if ct == 0 || ct == 4 {
        if ignore_unexpected { Decision::Drop } else { Decision::Error(Error::UnexpectedPalette) }
    } else if length == 0 || length % 3 != 0 || length > 768 {
        Decision::Error(Error::InvalidPalette)
    } else if ct == 3 && length / 3 > (1u32 << depth) as nat && !allow_large {
        Decision::Error(Error::InvalidPalette)
    } else { Decision::Keep(length / 3) }
}

pub open spec fn transparency(length: nat, ct: u8, entries: nat, drop_invalid: bool) -> Decision {
    if !matches!(ct,0|2|3|4|6) || entries > 256 { Decision::OutsideDomain }
    else if ct == 4 || ct == 6 || (ct == 0 && length != 2) || (ct == 2 && length != 6)
        || (ct == 3 && !(1 <= length <= entries)) {
        if drop_invalid { Decision::Drop } else { Decision::Error(Error::InvalidTransparency) }
    } else { Decision::Keep(length) }
}

} // verus!
