//! Whole-input production-walk witness using locally synthesized bytes.
//! Empty compressed data is structurally valid, not a successful pixel stream.
use vstd::prelude::*;
use crate::{walk,png::{Events,Error},policy::STRICT,document_model as m,model};
use crate::walk_model::{Phase,Range};

verus! {

spec fn file() -> Seq<u8> {seq![0x89u8, 0x50u8, 0x4eu8, 0x47u8, 0x0du8, 0x0au8, 0x1au8, 0x0au8, 0x00u8, 0x00u8, 0x00u8, 0x0du8, 0x49u8, 0x48u8, 0x44u8, 0x52u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x08u8, 0x02u8, 0x00u8, 0x00u8, 0x00u8, 0x90u8, 0x77u8, 0x53u8, 0xdeu8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x49u8, 0x44u8, 0x41u8, 0x54u8, 0x35u8, 0xafu8, 0x06u8, 0x1eu8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x49u8, 0x45u8, 0x4eu8, 0x44u8, 0xaeu8, 0x42u8, 0x60u8, 0x82u8]}
spec fn header() -> model::Ihdr {model::Ihdr {width:1,height:1,depth:8,color_type:2,interlace:0}}
spec fn before() -> m::State {m::initial(header(),33,m::no_events())}
spec fn after() -> m::State {
    let state=before();
    m::State {pos:45,phase:Phase::During,doc:m::Doc {idat:seq![Range {offset:41,len:0}],..state.doc},..state}
}

proof fn prefix()
    requires file().len()<=crate::limits::MAX_INPUT,
    ensures model::mh(file())==Ok((header(),33nat)),
{
    crate::limits::caps_ok();
    assert(model::crc(seq![0x49u8, 0x48u8, 0x44u8, 0x52u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x08u8, 0x02u8, 0x00u8, 0x00u8, 0x00u8])==0x9077_53deu32) by (compute);
    assert(file().subrange(12,29)==seq![0x49u8, 0x48u8, 0x44u8, 0x52u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x08u8, 0x02u8, 0x00u8, 0x00u8, 0x00u8]);
    assert(model::frame(file(),8)==Ok(model::WireChunk {ty:seq![b'I',b'H',b'D',b'R'],data:16,len:13,next:33})) by (compute);
    assert(model::be32(file().subrange(16,29),0)==1u32) by (compute);
    assert(model::be32(file().subrange(16,29),4)==1u32) by (compute);
    assert(model::ihdr(file().subrange(16,29))==Ok(header()));
    assert(model::signature(file())) by (compute);
    assert(model::be32(file(),29)==0x9077_53deu32) by (compute);
}

proof fn idat_step()
    ensures m::step(file(),m::strict(),false,false,before())==m::Step::Next(after()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(model::crc(seq![b'I',b'D',b'A',b'T'])==0x35af_061eu32) by (compute);
    assert(file().subrange(37,41)==seq![b'I',b'D',b'A',b'T']);
    assert(crate::critical_model::read(file(),33,Phase::Before,false)==Ok(crate::critical_model::Candidate {
        ty:seq![b'I',b'D',b'A',b'T'],data:41,len:0,next:45,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![b'I',b'D',b'A',b'T'])==crate::walk_model::Name::Idat) by (compute);
    assert(Seq::<Range>::empty().push(Range {offset:41,len:0})==seq![Range {offset:41,len:0}]);
}

proof fn iend_step()
    ensures m::step(file(),m::strict(),false,false,after())==m::Step::Done(after().doc,m::no_events()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(model::crc(seq![b'I',b'E',b'N',b'D'])==0xae42_6082u32) by (compute);
    assert(file().subrange(49,53)==seq![b'I',b'E',b'N',b'D']);
    assert(crate::critical_model::read(file(),45,Phase::During,false)==Ok(crate::critical_model::Candidate {
        ty:seq![b'I',b'E',b'N',b'D'],data:53,len:0,next:57,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![b'I',b'E',b'N',b'D'])==crate::walk_model::Name::Iend) by (compute);
}

proof fn accepted()
    requires file().len()<=crate::limits::MAX_INPUT,
    ensures model::MW(file())==m::Outcome::Done(after().doc,m::no_events()),
{
    prefix();idat_step();iend_step();
    m::drive_step(file(),m::strict(),false,false,before());
    m::drive_step(file(),m::strict(),false,false,after());
}

fn empty_idat_document() {
    let b=[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x44, 0x41, 0x54, 0x35, 0xaf, 0x06, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82];
    let mut events=Events {r1:false,r3:false,r4:false};
    let result=walk::still(&b,&STRICT,&mut events);
    proof {
        assert(b@==file());
        crate::walk_policy::profiles_project_exactly();
        if b.len() as u64<=crate::limits::MAX_INPUT {accepted();assert(result.is_ok());}
        else{assert(matches!(result,Err(Error::FileTooLarge)));}
    }
    match result {
        Err(_)=>{},Ok(doc)=> {proof {
            assert(doc.idat_first==33);
            assert(doc.view().idat==seq![Range {offset:41,len:0}]);
            assert(doc.frame_ranges@.len()==0);
            assert(!events.r1 && !events.r3 && !events.r4);
        }},
    }
}

} // verus!
