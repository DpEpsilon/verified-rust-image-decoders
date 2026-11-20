//! Whole-input selected-frame witness. Compressed fields are empty, so this
//! proves structural transport/counting, not successful image inflation.
use vstd::prelude::*;
use inflate_verus::Segments;
use crate::{model,document_model as m,walk_model::{Name,Phase,Frame,Source,Range},apng_model as apng,
    critical_model as critical,png::Events,policy::STRICT};

verus! {
spec fn file() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8,144u8,119u8,83u8,222u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,180u8,45u8,233u8,160u8,0u8,0u8,0u8,0u8,73u8,68u8,65u8,84u8,53u8,175u8,6u8,30u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,106u8,136u8,156u8,182u8,0u8,0u8,0u8,0u8,118u8,112u8,65u8,103u8,1u8,85u8,194u8,179u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,114u8,207u8,158u8,247u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn header() -> model::Ihdr {model::Ihdr {width:1,height:1,depth:8,color_type:2,interlace:0}}
spec fn s0() -> m::State {m::initial(header(),33,m::no_events())}
spec fn s1() -> m::State {
    let s=s0();m::State {pos:53,seen:s.seen.insert(Name::Actl),doc:m::Doc {actl:Some((1u32,0u32)),..s.doc},..s}
}
spec fn s2() -> m::State {
    let s=s1();m::State {pos:65,phase:Phase::During,doc:m::Doc {idat:seq![Range {offset:61,len:0}],..s.doc},..s}
}
spec fn s3() -> m::State {
    let s=s2();m::State {pos:103,phase:Phase::After,control:apng::Control {sequence:1,current:apng::Current::Frame(false),
        frame_count:1,fctl_before_idat:false,pixel_total:1},doc:m::Doc {frames:seq![Frame {control_offset:65,width:1,height:1,x:0,y:0,
            delay_num:0,delay_den:0,dispose:0,blend:0,source:Source::Fdat(Seq::empty())}],..s.doc},..s}
}
spec fn s4() -> m::State {m::State {pos:115,..s3()}}
spec fn s5() -> m::State {
    let s=s4();m::State {pos:131,control:apng::Control {sequence:2,current:apng::Current::Frame(true),..s.control},
        doc:m::Doc {frames:seq![Frame {source:Source::Fdat(seq![Range {offset:127,len:0}]),..s.doc.frames[0]}],..s.doc},..s}
}
spec fn output() -> m::Doc {m::Doc {info:m::Info {frame_count:1,..s5().doc.info},..s5().doc}}

proof fn prefix()
    requires file().len()<=crate::limits::MAX_INPUT,
    ensures model::mh(file())==Ok((header(),33nat)),
{
    crate::limits::caps_ok();
    assert(model::signature(file())) by (compute);
    assert(model::frame(file(),8)==Ok(model::WireChunk {ty:seq![b'I',b'H',b'D',b'R'],data:16,len:13,next:33})) by (compute);
    assert(model::crc(seq![73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8])==0x9077_53deu32) by (compute);
    assert(file().subrange(12,29)==seq![73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,2u8,0u8,0u8,0u8]);
    assert(model::be32(file(),29)==0x9077_53deu32) by (compute);
    assert(model::be32(file().subrange(16,29),0)==1u32) by (compute);
    assert(model::be32(file().subrange(16,29),4)==1u32) by (compute);
    assert(model::ihdr(file().subrange(16,29))==Ok(header()));
}

proof fn actl_step()
    ensures m::step(file(),m::strict(),true,false,s0())==m::Step::Next(s1()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(critical::read(file(),33,Phase::Before,false)==Ok(critical::Candidate {ty:seq![97u8,99u8,84u8,76u8],
        data:41,len:8,next:53,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==Name::Actl) by (compute);
    crate::limits::caps_ok();
    assert(model::be32(file().subrange(41,49),0)==1u32) by (compute);
    assert(model::be32(file().subrange(41,49),4)==0u32) by (compute);
}

proof fn idat_step()
    ensures m::step(file(),m::strict(),true,false,s1())==m::Step::Next(s2()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(critical::read(file(),53,Phase::Before,false)==Ok(critical::Candidate {ty:seq![73u8,68u8,65u8,84u8],
        data:61,len:0,next:65,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==Name::Idat) by (compute);
}

proof fn fctl_step()
    requires crate::limits::MAX_TOTAL_PIXELS>0,
    ensures m::step(file(),m::strict(),true,false,s2())==m::Step::Next(s3()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(critical::read(file(),65,Phase::During,false)==Ok(critical::Candidate {ty:seq![102u8,99u8,84u8,76u8],
        data:73,len:26,next:103,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==Name::Fctl) by (compute);
    assert(apng::parse_fctl(file().subrange(73,99))==apng::FrameControl {width:1,height:1,x:0,y:0,delay_num:0,delay_den:0,dispose:0,blend:0}) by (compute);
    assert(model::be32(file().subrange(73,99),0)==0u32) by (compute);
    assert(1int*1int==1int) by (nonlinear_arith);
}

proof fn metadata_step()
    ensures m::step(file(),m::strict(),true,false,s3())==m::Step::Next(s4()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(critical::read(file(),103,Phase::After,false)==Ok(critical::Candidate {ty:seq![118u8,112u8,65u8,103u8],
        data:111,len:0,next:115,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![118u8,112u8,65u8,103u8])==Name::Unknown(true)) by (compute);
}

proof fn fdat_step()
    ensures m::step(file(),m::strict(),true,false,s4())==m::Step::Next(s5()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(critical::read(file(),115,Phase::After,false)==Ok(critical::Candidate {ty:seq![102u8,100u8,65u8,84u8],
        data:123,len:4,next:131,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==Name::Fdat) by (compute);
    assert(model::be32(file().subrange(123,127),0)==1u32) by (compute);
    let range=Range {offset:127,len:0};
    assert(Seq::<Range>::empty().push(range)==seq![range]);
    let changed=Frame {source:Source::Fdat(seq![range]),..s4().doc.frames[0]};
    assert(s4().doc.frames.update(0,changed) =~= seq![changed]);
}

proof fn iend_step()
    ensures m::step(file(),m::strict(),true,false,s5())==m::Step::Done(output(),m::no_events()),
{
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    assert(critical::read(file(),131,Phase::After,false)==Ok(critical::Candidate {ty:seq![73u8,69u8,78u8,68u8],
        data:139,len:0,next:143,crc_ok:true,synthetic:false})) by (compute);
    assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==Name::Iend) by (compute);
}

proof fn accepted()
    requires file().len()<=crate::limits::MAX_INPUT,crate::limits::MAX_TOTAL_PIXELS>0,
    ensures m::MWF_p(file(),m::strict())==m::Outcome::Done(output(),m::no_events()),
{
    prefix();actl_step();idat_step();fctl_step();metadata_step();fdat_step();iend_step();
    m::drive_step(file(),m::strict(),true,false,s0());m::drive_step(file(),m::strict(),true,false,s1());
    m::drive_step(file(),m::strict(),true,false,s2());m::drive_step(file(),m::strict(),true,false,s3());
    m::drive_step(file(),m::strict(),true,false,s4());m::drive_step(file(),m::strict(),true,false,s5());
}

fn empty_fdat_with_metadata() {
    let b=[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,1,0,0,0,1,8,2,0,0,0,144,119,83,222,0,0,0,8,97,99,84,76,0,0,0,1,0,0,0,0,180,45,233,160,0,0,0,0,73,68,65,84,53,175,6,30,0,0,0,26,102,99,84,76,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,106,136,156,182,0,0,0,0,118,112,65,103,1,85,194,179,0,0,0,4,102,100,65,84,0,0,0,1,114,207,158,247,0,0,0,0,73,69,78,68,174,66,96,130];
    if b.len() as u64<=crate::limits::MAX_INPUT && crate::limits::MAX_TOTAL_PIXELS>0 {
        let mut events=Events {r1:false,r3:false,r4:false};
        let result=crate::walk::frames(&b,&STRICT,&mut events);
        proof {assert(b@==file());crate::walk_policy::profiles_project_exactly();accepted();assert(result.is_ok());}
        match result {Err(_)=>{},Ok(doc)=> {
            proof {
                assert(doc.view()==output());
                assert(doc.info.frame_count==1 && !doc.info.default_is_frame);
                assert(crate::frame_transport_model::header_end(b@)==33);
            }
            let control=crate::reader::next_selected_fctl(&b,&doc,0,33);
            proof {assert(control==Some(65usize));}
            let mut reader=crate::reader::FdatReader::for_frame(&b,&doc,0,103);
            proof {
                assert(model::reader_count(file(),103,true)==2nat) by (compute);
                assert(model::reader_bytes(file(),103,true)==Seq::<u8>::empty()) by (compute);
                assert(reader.seg_count()==2 && reader.concat()==Seq::<u8>::empty());
                assert(doc.frame_ranges@[0].source==Source::Fdat(seq![Range {offset:127,len:0}]));
            }
            let eof=reader.read();
            proof {assert(eof.is_none());assert(reader.seg_steps()==3 && reader.core_steps()==0);}
        }}
    }
}

} // verus!
