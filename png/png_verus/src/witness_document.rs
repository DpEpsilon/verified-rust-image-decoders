//! Production-call witnesses for stored metadata and policy projection, plus
//! a mathematical empty-fdAT trace witness. These are not whole-input gates.
use vstd::prelude::*;
use crate::{header::Ihdr, png::{Info,Meta,Events,Error}, kind::Kind};
use crate::{info, document_model as m, walk_model::{Name,Phase,Source,Frame}, apng_model as apng};

verus! {

fn stored_metadata_and_discard() {
    let h=Ihdr {w:1,h:1,depth:8,ct:2,interlace:0};
    let mut info=Info::new(&h);
    info::discard(&mut info,Kind::Gama);
    proof {
        assert(info.view().slots[Name::Gama]==m::Slot::Discarded);
        assert(matches!(info.gama,Meta::Discarded));
    }
    let zero=[0u8,0,0,0];
    let bad=info::parse_and_store(&mut info,Kind::Gama,&zero,0,4,0);
    proof {
        assert(zero@==seq![0u8,0u8,0u8,0u8]);
        assert(crate::model::be32(seq![0u8,0u8,0u8,0u8],0)==0u32) by (compute);
        assert(matches!(bad,Err(Error::InvalidGamma)));
        assert(matches!(info.gama,Meta::Discarded));
    }
    let one=[0u8,0,0,1];
    let good=info::parse_and_store(&mut info,Kind::Gama,&one,0,4,0);
    proof {
        assert(one@==seq![0u8,0u8,0u8,1u8]);
        assert(crate::model::be32(seq![0u8,0u8,0u8,1u8],0)==1u32) by (compute);
        assert(good==Ok(None));
        assert(info.view().slots[Name::Gama]==m::Slot::Present(m::Stored::Raw(crate::metadata_model::Value::Gama(1))));
        assert(matches!(info.gama,Meta::Present(1)));
    }
    info::discard(&mut info,Kind::Gama);
    let mut events=Events {r1:false,r3:true,r4:false};
    events.discard_trns(Kind::Trns,2);
    proof {
        assert(info.view().slots[Name::Gama]==m::Slot::Present(m::Stored::Raw(crate::metadata_model::Value::Gama(1))));
        assert(matches!(info.gama,Meta::Present(1)));
        assert(events.r1 && events.r3 && !events.r4);
    }
}

fn borrowed_metadata_ranges() {
    let h=Ihdr {w:1,h:1,depth:8,ct:2,interlace:0};
    let mut info=Info::new(&h);
    let b=[99u8,b'a',0,0,7,99];
    let profile=info::parse_and_store(&mut info,Kind::Iccp,&b,1,4,0);
    proof {
        assert(b@.subrange(1,5)==seq![b'a',0u8,0u8,7u8]);
        assert(crate::model::first_nul(b@.subrange(1,5),1)==Some(1nat));
        assert(crate::model::first_nul(b@.subrange(1,5),0)==Some(1nat));
        assert(crate::metadata_model::keyword_method(b@.subrange(1,5),Error::InvalidIccp)==Ok((1nat,3nat,1nat)));
        assert(profile==Ok(Some((4usize,1usize))));
        assert(info.view().slots[Name::Iccp]==m::Slot::Present(m::Stored::Bytes(crate::walk_model::Range {offset:1,len:1})));
        assert(matches!(info.iccp_name,Meta::Present((1usize,1usize))));
        assert(info.ranges_wf(6));
    }
    proof {
        assert(info.view().slots[Name::Exif]==m::Slot::Absent);
        assert(info.ranges_wf(5));
    }
    let exif=[99u8,b'I',b'I',42,0];
    let parsed=info::parse_and_store(&mut info,Kind::Exif,&exif,1,4,0);
    proof {
        assert(exif@.subrange(1,5)==seq![b'I',b'I',42u8,0u8]);
        assert(crate::metadata_model::parse(Name::Exif,exif@.subrange(1,5),2,8,0)
            ==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Exif(4)));
        assert(parsed==Ok(None));
        assert(info.view().slots[Name::Exif]==m::Slot::Present(m::Stored::Bytes(crate::walk_model::Range {offset:1,len:4})));
        assert(matches!(info.exif,Meta::Present((1usize,4usize))));
        assert(info.ranges_wf(5));
    }
}

fn policy_projection_calls() {
    let strict=crate::policy::STRICT.walk_rules();
    let beta=crate::policy::RECOMMENDED_BETA.walk_rules();
    proof {
        crate::walk_policy::profiles_project_exactly();
        assert(strict.view()==m::strict());
        assert(beta.view()==m::recommended_beta());
    }
}

proof fn empty_fdat_trace() {
    let b=seq![0u8,0u8,0u8,0u8];
    let h=crate::model::Ihdr {width:1,height:1,depth:8,color_type:2,interlace:0};
    let initial=m::initial(h,4,m::no_events());
    let frame=Frame {control_offset:0,width:1,height:1,x:0,y:0,delay_num:0,delay_den:0,dispose:0,blend:0,
        source:Source::Fdat(Seq::empty())};
    let state=m::State {phase:Phase::After,control:apng::Control {sequence:0,current:apng::Current::Frame(false),
        frame_count:1,fctl_before_idat:false,pixel_total:1},
        doc:m::Doc {actl:Some((1u32,0u32)),frames:seq![frame],..initial.doc},..initial};
    reveal(m::own);
    assert(crate::model::be32(seq![0u8,0u8,0u8,0u8],0)==0u32) by (compute);
    match m::own(b,m::strict(),false,state,Name::Fdat,0,4) {
        m::Step::Next(next)=> {
            assert(next.control.sequence==1);
            assert(next.control.current==apng::Current::Frame(true));
            assert(next.doc.frames[0].source==Source::Fdat(seq![crate::walk_model::Range {offset:4,len:0}]));
            reveal_with_fuel(m::concat_ranges,2);
            let ranges=seq![crate::walk_model::Range {offset:4,len:0}];
            assert(ranges.drop_first()==Seq::<crate::walk_model::Range>::empty());
            let empty=Seq::<crate::walk_model::Range>::empty();
            assert(m::concat_ranges(b,empty)==Some(Seq::<u8>::empty()));
            assert(b.subrange(4,4)==Seq::<u8>::empty());
            assert(m::concat_ranges(b,seq![crate::walk_model::Range {offset:4,len:0}])==Some(Seq::<u8>::empty()));
            assert(m::frame_stream(b,next.doc,0)==Some(Seq::<u8>::empty()));
        },
        _=>assert(false),
    }
}

} // verus!
