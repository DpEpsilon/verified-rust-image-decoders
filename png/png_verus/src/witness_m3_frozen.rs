//! Authored frozen-fixture model/header facts, not public-entry gates.
//! Category (a) still needs checked public dispatch; category (b) also
//! needs full checked walk preparation. No pixel/data outcome is claimed.
use vstd::prelude::*;
use crate::{model,header,png::Error,document_model as m,policy::STRICT,png::Events};

verus! {
// container/empty; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_empty() -> Seq<u8> {seq![]}
fn header_container_empty() {
    let b=[];
    proof {assert(b@==bytes_container_empty());
        assert(model::mh(bytes_container_empty())==Err(Error::BadSignature)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::BadSignature)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::BadSignature,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::BadSignature))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::BadSignature))));
}

// container/short-signature; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_short_signature() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8]}
fn header_container_short_signature() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8];
    proof {assert(b@==bytes_container_short_signature());
        assert(model::mh(bytes_container_short_signature())==Err(Error::BadSignature)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::BadSignature)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::BadSignature,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::BadSignature))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::BadSignature))));
}

// container/bad-signature; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_bad_signature() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,10u8,13u8,26u8,10u8]}
fn header_container_bad_signature() {
    let b=[137u8,80u8,78u8,71u8,10u8,13u8,26u8,10u8];
    proof {assert(b@==bytes_container_bad_signature());
        assert(model::mh(bytes_container_bad_signature())==Err(Error::BadSignature)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::BadSignature)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::BadSignature,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::BadSignature))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::BadSignature))));
}

// container/signature-only; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_signature_only() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8]}
fn header_container_signature_only() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8];
    proof {assert(b@==bytes_container_signature_only());
        assert(model::mh(bytes_container_signature_only())==Err(Error::TruncatedChunk)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::TruncatedChunk)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::TruncatedChunk))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::TruncatedChunk))));
}

// container/truncated-ihdr-header; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_truncated_ihdr_header() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8]}
fn header_container_truncated_ihdr_header() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8];
    proof {assert(b@==bytes_container_truncated_ihdr_header());
        assert(model::mh(bytes_container_truncated_ihdr_header())==Err(Error::TruncatedChunk)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::TruncatedChunk)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::TruncatedChunk))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::TruncatedChunk))));
}

// container/truncated-ihdr-crc; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_truncated_ihdr_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8]}
fn header_container_truncated_ihdr_crc() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8];
    proof {assert(b@==bytes_container_truncated_ihdr_crc());
        assert(model::mh(bytes_container_truncated_ihdr_crc())==Err(Error::TruncatedChunk)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::TruncatedChunk)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::TruncatedChunk,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::TruncatedChunk))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::TruncatedChunk))));
}

// container/first-chunk-not-ihdr; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_first_chunk_not_ihdr() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_container_first_chunk_not_ihdr() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,4u8,103u8,65u8,77u8,65u8,0u8,0u8,177u8,143u8,11u8,252u8,97u8,5u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_first_chunk_not_ihdr());
        assert(model::mh(bytes_container_first_chunk_not_ihdr())==Err(Error::MissingIhdr)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::MissingIhdr)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::MissingIhdr,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::MissingIhdr))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::MissingIhdr))));
}

// container/ihdr-length-12; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_ihdr_length_12() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,12u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,185u8,9u8,121u8,60u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_container_ihdr_length_12() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,12u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,185u8,9u8,121u8,60u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_ihdr_length_12());
        assert(model::mh(bytes_container_ihdr_length_12())==Err(Error::InvalidChunkLength)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidChunkLength)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidChunkLength,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidChunkLength))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidChunkLength))));
}

// container/ihdr-crc; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_ihdr_crc() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_container_ihdr_crc() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_ihdr_crc());
        assert(model::mh(bytes_container_ihdr_crc())==Err(Error::CrcMismatch)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::CrcMismatch)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::CrcMismatch,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::CrcMismatch))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::CrcMismatch))));
}

// ihdr/width-0; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_width_0() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,249u8,33u8,74u8,78u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_width_0() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,249u8,33u8,74u8,78u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_width_0());
        assert(model::mh(bytes_ihdr_width_0())==Err(Error::InvalidDimensions)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidDimensions)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidDimensions,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidDimensions))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidDimensions))));
}

// ihdr/height-0; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_height_0() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,8u8,2u8,0u8,0u8,0u8,176u8,28u8,59u8,120u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_height_0() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,8u8,2u8,0u8,0u8,0u8,176u8,28u8,59u8,120u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_height_0());
        assert(model::mh(bytes_ihdr_height_0())==Err(Error::InvalidDimensions)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidDimensions)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidDimensions,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidDimensions))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidDimensions))));
}

// ihdr/width-2_31; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_width_2_31() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,128u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,223u8,223u8,29u8,247u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_width_2_31() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,128u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,223u8,223u8,29u8,247u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_width_2_31());
        assert(model::mh(bytes_ihdr_width_2_31())==Err(Error::InvalidDimensions)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidDimensions)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidDimensions,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidDimensions))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidDimensions))));
}

// ihdr/depth-3; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_depth_3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,3u8,0u8,0u8,0u8,0u8,32u8,13u8,99u8,233u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_depth_3() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,3u8,0u8,0u8,0u8,0u8,32u8,13u8,99u8,233u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_depth_3());
        assert(model::mh(bytes_ihdr_depth_3())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/rgb-depth-4; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_rgb_depth_4() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,4u8,2u8,0u8,0u8,0u8,56u8,36u8,119u8,114u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_rgb_depth_4() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,4u8,2u8,0u8,0u8,0u8,56u8,36u8,119u8,114u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_rgb_depth_4());
        assert(model::mh(bytes_ihdr_rgb_depth_4())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/palette-depth-16; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_palette_depth_16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,16u8,3u8,0u8,0u8,0u8,21u8,248u8,33u8,85u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_palette_depth_16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,16u8,3u8,0u8,0u8,0u8,21u8,248u8,33u8,85u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_palette_depth_16());
        assert(model::mh(bytes_ihdr_palette_depth_16())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/ct-1; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_ct_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,1u8,0u8,0u8,0u8,239u8,97u8,53u8,157u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_ct_1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,1u8,0u8,0u8,0u8,239u8,97u8,53u8,157u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_ct_1());
        assert(model::mh(bytes_ihdr_ct_1())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/ct-5; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_ct_5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,5u8,0u8,0u8,0u8,96u8,3u8,162u8,202u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_ct_5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,5u8,0u8,0u8,0u8,96u8,3u8,162u8,202u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_ct_5());
        assert(model::mh(bytes_ihdr_ct_5())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/ct-7; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_ct_7() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,7u8,0u8,0u8,0u8,202u8,10u8,106u8,65u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_ct_7() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,7u8,0u8,0u8,0u8,202u8,10u8,106u8,65u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_ct_7());
        assert(model::mh(bytes_ihdr_ct_7())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/ga-depth-1; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_ga_depth_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,1u8,4u8,0u8,0u8,0u8,213u8,175u8,167u8,222u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_ga_depth_1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,1u8,4u8,0u8,0u8,0u8,213u8,175u8,167u8,222u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_ga_depth_1());
        assert(model::mh(bytes_ihdr_ga_depth_1())==Err(Error::InvalidColorTypeDepth)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::InvalidColorTypeDepth)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::InvalidColorTypeDepth,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::InvalidColorTypeDepth))));
}

// ihdr/compression-1; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_compression_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,1u8,0u8,0u8,252u8,22u8,240u8,68u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_compression_1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,1u8,0u8,0u8,252u8,22u8,240u8,68u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_compression_1());
        assert(model::mh(bytes_ihdr_compression_1())==Err(Error::UnsupportedCompressionMethod)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::UnsupportedCompressionMethod)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::UnsupportedCompressionMethod,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::UnsupportedCompressionMethod))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::UnsupportedCompressionMethod))));
}

// ihdr/filter-method-1; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_filter_method_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,1u8,0u8,228u8,207u8,171u8,50u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_filter_method_1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,1u8,0u8,228u8,207u8,171u8,50u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_filter_method_1());
        assert(model::mh(bytes_ihdr_filter_method_1())==Err(Error::UnsupportedFilterMethod)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::UnsupportedFilterMethod)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::UnsupportedFilterMethod,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::UnsupportedFilterMethod))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::UnsupportedFilterMethod))));
}

// ihdr/interlace-2; spec-frozen/VECTORS.json sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ihdr_interlace_2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,2u8,19u8,218u8,251u8,95u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_interlace_2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,2u8,19u8,218u8,251u8,95u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_interlace_2());
        assert(model::mh(bytes_ihdr_interlace_2())==Err(Error::UnsupportedInterlaceMethod)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::UnsupportedInterlaceMethod)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::UnsupportedInterlaceMethod,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::UnsupportedInterlaceMethod))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::UnsupportedInterlaceMethod))));
}

// ihdr/width-2_31-1-output; limits-draft/VECTORS-CAPS.json sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_width_2_31_1_output() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,127u8,255u8,255u8,255u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,47u8,84u8,164u8,138u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_width_2_31_1_output() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,127u8,255u8,255u8,255u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,47u8,84u8,164u8,138u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_width_2_31_1_output());
        assert(model::mh(bytes_ihdr_width_2_31_1_output())==Err(Error::OutputTooLarge)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::OutputTooLarge)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::OutputTooLarge,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::OutputTooLarge))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::OutputTooLarge))));
}

// ihdr/67108865x1-output; limits-draft/VECTORS-CAPS.json sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_67108865x1_output() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,4u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,80u8,206u8,57u8,72u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_67108865x1_output() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,4u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,80u8,206u8,57u8,72u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_67108865x1_output());
        assert(model::mh(bytes_ihdr_67108865x1_output())==Err(Error::OutputTooLarge)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::OutputTooLarge)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::OutputTooLarge,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::OutputTooLarge))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::OutputTooLarge))));
}

// ihdr/8192x8193-output; limits-draft/VECTORS-CAPS.json sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_8192x8193_output() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,32u8,1u8,8u8,2u8,0u8,0u8,0u8,54u8,148u8,142u8,171u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_8192x8193_output() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,32u8,1u8,8u8,2u8,0u8,0u8,0u8,54u8,148u8,142u8,171u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_8192x8193_output());
        assert(model::mh(bytes_ihdr_8192x8193_output())==Err(Error::OutputTooLarge)) by (compute);
    }
    let result=header::prefix(&b);
    assert(matches!(result,Err(Error::OutputTooLarge)));
    assert(m::MW_p(b@,m::strict())==m::Outcome::Error(Error::OutputTooLarge,m::no_events()));
    let mut ev=Events {r1:false,r3:false,r4:false};
    let still=crate::callers::still(&b,&STRICT,&mut ev);
    assert(matches!(still,Err(crate::alloc_guard::sites::TryError::Format(Error::OutputTooLarge))));
    let frames=crate::callers::frames(&b,&STRICT,&mut ev);
    assert(matches!(frames,Err(crate::alloc_guard::sites::TryError::Format(Error::OutputTooLarge))));
}

// ihdr/8192x8192-accepted-then-truncated-zlib; limits-draft/VECTORS-CAPS.json sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_8192x8192_accepted_then_truncated_zlib() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,32u8,0u8,8u8,6u8,0u8,0u8,0u8,114u8,170u8,202u8,89u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_8192x8192_accepted_then_truncated_zlib() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,32u8,0u8,8u8,6u8,0u8,0u8,0u8,114u8,170u8,202u8,89u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_8192x8192_accepted_then_truncated_zlib());
        assert(model::mh(bytes_ihdr_8192x8192_accepted_then_truncated_zlib())==Ok((model::Ihdr {width:model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),16),height:model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
    }
    let result=header::prefix(&b);
    assert(result.is_ok());
    assert(result.unwrap().0.view()==model::mh(bytes_ihdr_8192x8192_accepted_then_truncated_zlib()).unwrap().0);
}

// ihdr/67108864x1-accepted-then-truncated-zlib; limits-draft/VECTORS-CAPS.json sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_67108864x1_accepted_then_truncated_zlib() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,4u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,48u8,110u8,197u8,33u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_67108864x1_accepted_then_truncated_zlib() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,4u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,48u8,110u8,197u8,33u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_67108864x1_accepted_then_truncated_zlib());
        assert(model::mh(bytes_ihdr_67108864x1_accepted_then_truncated_zlib())==Ok((model::Ihdr {width:model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),16),height:model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
    }
    let result=header::prefix(&b);
    assert(result.is_ok());
    assert(result.unwrap().0.view()==model::mh(bytes_ihdr_67108864x1_accepted_then_truncated_zlib()).unwrap().0);
}

// ihdr/1x67108864-accepted-then-truncated-zlib; limits-draft/VECTORS-CAPS.json sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_1x67108864_accepted_then_truncated_zlib() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,4u8,0u8,0u8,0u8,8u8,6u8,0u8,0u8,0u8,137u8,165u8,70u8,32u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
fn header_ihdr_1x67108864_accepted_then_truncated_zlib() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,4u8,0u8,0u8,0u8,8u8,6u8,0u8,0u8,0u8,137u8,165u8,70u8,32u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_1x67108864_accepted_then_truncated_zlib());
        assert(model::mh(bytes_ihdr_1x67108864_accepted_then_truncated_zlib())==Ok((model::Ihdr {width:model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),16),height:model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),20),depth:8,color_type:6,interlace:0},33nat))) by (compute);
    }
    let result=header::prefix(&b);
    assert(result.is_ok());
    assert(result.unwrap().0.view()==model::mh(bytes_ihdr_1x67108864_accepted_then_truncated_zlib()).unwrap().0);
}

} // verus!
