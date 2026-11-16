from pathlib import Path
import shutil
r=Path('/home/ws/projects/verified-decoders/gif');p=r/'gif_verus/src/model_vectors.rs';s=p.read_text();dst=r/'runs/sol-20261116/model-vectors-eight-compute.rs';assert not dst.exists();shutil.copy2(p,dst)
a='// Frozen record basic-4x3-gct.';assert s.count(a)==1;s=s[:s.index(a)]+'} // verus!\n'
start=s.index('pub proof fn model_two_colour_m2()');brace=s.index('\n{',start);end=s.index('\n}',brace)+2
s=s[:brace]+'''
{
    hide(crate::lzw_model::ML);hide(crate::mc::draw_fold);
    let b=bytes_two_colour_m2();
    let p=crate::policy::profile_record(crate::policy::Profile::Strict);
    let h=crate::model::HeaderM {width:3,height:1,version:1,background_index:0,aspect:0,
        global:Some(crate::model::ColorTableM {off:13,entries:2}),cursor:19};
    let w=crate::model::WalkM {p:19,gce:None,frames_seen:0,total_pixels:0,loop_count:None};
    let info=crate::model::InfoM {width:3,height:1,version:1,background_index:0,aspect:0,loop_count:None};
    let fh=crate::model::FrameHeaderM {left:0,top:0,w:3,h:1,interlaced:false,
        table:crate::model::ColorTableM {off:13,entries:2},inside:true};
    let after=crate::model::WalkM {p:34,gce:None,frames_seen:1,total_pixels:3,loop_count:None};
    assert(crate::model::mh(b)==Ok(h)) by(compute);
    assert(crate::model::mp(b,p)==Ok((info,w))) by(compute);
    assert(crate::model::mb(b,h,p,w)==Ok(crate::model::MbOut::Image(fh,None,2,seq![68u8,80u8],after))) by(compute);
    // These indices are read from the frozen expected RGBA using its distinct
    // white/black palette, not obtained from an oracle or decoder run.
    let idx=seq![0u8,1u8,0u8];
    assert(crate::lzw_model::ML(2,seq![68u8,80u8],3,crate::frame_model::params(p)).idx==idx
        && crate::lzw_model::ML(2,seq![68u8,80u8],3,crate::frame_model::params(p)).error is None) by(compute);
    let cv=Seq::new(12,|_i:int|0u8);
    let table=seq![255u8,255u8,255u8,0u8,0u8,0u8];
    let rgba=seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8];
    assert(crate::mc::draw_fold(cv,crate::frame_model::frame(fh,None),table,3,1,idx,0)==Ok(rgba)) by(compute);
    assert(b.subrange(13,19)==table) by(compute);
}
'''+s[end:];p.write_text(s)
print('Retained five negative witnesses and staged the positive two-colour gate without raising limits')
