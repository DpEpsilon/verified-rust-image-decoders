//! Concrete production-call bindings for the P6–P8 component checkpoint.
//! These do not replace the complete whole-input M3 frozen-record gates.
use vstd::prelude::*;
use crate::{metadata::{self,Value}, kind::Kind, model, png::Error};

verus! {

fn palette_transparency_decisions() {
    let exact = metadata::palette(6,3,1,false,false,false,false);
    let too_many = metadata::palette(9,3,1,false,false,false,false);
    let allowed = metadata::palette(9,3,1,false,false,false,true);
    let order_first = metadata::palette(1,0,8,true,false,false,false);
    let grey_drop = metadata::palette(3,0,8,false,false,true,false);
    let alpha_error = metadata::transparency(0,6,0,false);
    let alpha_drop = metadata::transparency(0,6,0,true);
    let palette_key = metadata::transparency(1,3,2,false);
    proof {
        assert((1u32 << 1u8) == 2u32) by (compute);
        assert(matches!(exact,Ok(Some(2))));
        assert(matches!(too_many,Err(Error::InvalidPalette)));
        assert(matches!(allowed,Ok(Some(3))));
        assert(matches!(order_first,Err(Error::ChunkOutOfOrder)));
        assert(matches!(grey_drop,Ok(None)));
        assert(matches!(alpha_error,Err(Error::InvalidTransparency)));
        assert(matches!(alpha_drop,Ok(false)));
        assert(matches!(palette_key,Ok(true)));
    }
}

fn metadata_sample_ranges() {
    let sbit = [9u8,8,8];
    let high_background = [1u8,0];
    let palette_background = [3u8];
    let a = metadata::parse(Kind::Sbit,&sbit,3,8,3);
    let b = metadata::parse(Kind::Bkgd,&high_background,0,8,0);
    let c = metadata::parse(Kind::Bkgd,&palette_background,3,8,3);
    proof {
        assert(sbit@ == seq![9u8,8u8,8u8]);
        assert(high_background@ == seq![1u8,0u8]);
        assert(palette_background@ == seq![3u8]);
        assert(model::be16(seq![1u8,0u8],0) == 256u16) by (compute);
        assert((1u32 << 8u8) == 256u32) by (compute);
        assert(crate::metadata_model::words16(high_background@,1).len() == 1);
        assert(crate::metadata_model::words16(high_background@,1)[0] == 256u16);
        assert(matches!(a,Err(Error::InvalidSbit)));
        assert(matches!(b,Err(Error::InvalidBackground)));
        assert(matches!(c,Err(Error::InvalidBackground)));
    }
}

fn metadata_text_methods() {
    let uncompressed = [b'k',0,0,9,0,0];
    let compressed_bad = [b'k',0,1,1,0,0];
    let corrupt_payload = [b'k',0,0,255,255];
    let a = metadata::parse(Kind::Itxt,&uncompressed,2,8,0);
    let b = metadata::parse(Kind::Itxt,&compressed_bad,2,8,0);
    let c = metadata::parse(Kind::Ztxt,&corrupt_payload,2,8,0);
    proof {
        assert(uncompressed@ == seq![b'k',0u8,0u8,9u8,0u8,0u8]);
        assert(compressed_bad@ == seq![b'k',0u8,1u8,1u8,0u8,0u8]);
        assert(corrupt_payload@ == seq![b'k',0u8,0u8,255u8,255u8]);
        assert(model::keyword(seq![b'k']));
        assert(model::first_nul(seq![b'k',0u8,0u8,9u8,0u8,0u8],0) == Some(1nat)) by (compute);
        assert(model::first_nul(seq![b'k',0u8,1u8,1u8,0u8,0u8],0) == Some(1nat)) by (compute);
        assert(model::first_nul(seq![b'k',0u8,0u8,255u8,255u8],0) == Some(1nat)) by (compute);
        assert(model::first_nul(seq![b'k',0u8,0u8,9u8,0u8,0u8],4) == Some(4nat)) by (compute);
        assert(model::first_nul(seq![b'k',0u8,0u8,9u8,0u8,0u8],5) == Some(5nat)) by (compute);
        assert(matches!(a,Ok(Value::Ignored)));
        assert(matches!(b,Err(Error::InvalidText)));
        assert(matches!(c,Ok(Value::Ignored)));
    }
}

fn metadata_relative_ranges() {
    let bytes = [99u8,b'k',0,0,1,2,3,99];
    let data = &bytes[1..7];
    let parsed = metadata::parse(Kind::Iccp,data,2,8,0);
    proof {
        assert(data@ == seq![b'k',0u8,0u8,1u8,2u8,3u8]);
        assert(model::keyword(seq![b'k']));
        assert(model::first_nul(seq![b'k',0u8,0u8,1u8,2u8,3u8],0) == Some(1nat)) by (compute);
        assert(matches!(parsed,Ok(Value::Iccp {name_len:1,profile_start:3,profile_len:3})));
    }
    match parsed {
        Ok(Value::Iccp {name_len,profile_start,profile_len}) => {
            let name = metadata::rebase(&bytes,1,6,0,name_len);
            let profile = metadata::rebase(&bytes,1,6,profile_start,profile_len);
            proof {
                assert(name == (1usize,1usize));
                assert(profile == (4usize,3usize));
                assert(bytes@.subrange(profile.0 as int,(profile.0 + profile.1) as int) == seq![1u8,2u8,3u8]);
            }
        },
        _ => {},
    }
}

} // verus!
