from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/model_vectors.rs');s=p.read_text()
start=s.index('    assert(crate::lzw_model::ML(2,seq![68u8,80u8]');end=s.index(' by(compute);',start)+len(' by(compute);')
s=s[:start]+'''    // Unfold one code at a time; never raise the compute recursion limit.
    hide(crate::lzw_model::L);
    let ps=crate::frame_model::params(p);
    let st0=crate::lzw_model::init_st(2);
    let st1=crate::lzw_model::clear_st(2,st0,3);
    let st2=crate::lzw_model::emit_st(2,st1,6,0,seq![0u8],0u8,false);
    let st3=crate::lzw_model::emit_st(2,st2,9,1,seq![1u8],1u8,true);
    let st4=crate::lzw_model::emit_st(2,st3,12,0,seq![0u8],0u8,true);
    assert(crate::lzw_model::code_at(seq![68u8,80u8],0,3,0)==4) by(compute);
    assert(crate::lzw_model::code_at(seq![68u8,80u8],3,3,0)==0) by(compute);
    assert(crate::lzw_model::code_at(seq![68u8,80u8],6,3,0)==1) by(compute);
    assert(crate::lzw_model::code_at(seq![68u8,80u8],9,3,0)==0) by(compute);
    assert(crate::lzw_model::code_at(seq![68u8,80u8],12,4,0)==5) by(compute);
    assert(crate::lzw_model::verdict(2,seq![68u8,80u8],3,ps,st0)==crate::lzw_model::Vd::Step(st1));
    assert(crate::lzw_model::verdict(2,seq![68u8,80u8],3,ps,st1)==crate::lzw_model::Vd::Step(st2));
    assert(crate::lzw_model::verdict(2,seq![68u8,80u8],3,ps,st2)==crate::lzw_model::Vd::Step(st3));
    assert(crate::lzw_model::verdict(2,seq![68u8,80u8],3,ps,st3)==crate::lzw_model::Vd::Step(st4));
    assert(crate::lzw_model::verdict(2,seq![68u8,80u8],3,ps,st4)==crate::lzw_model::Vd::Halt(crate::lzw_model::observation(idx,None,16,false)));
    crate::lzw_model::lemma_L_unfold(2,seq![68u8,80u8],3,ps,st0);
    crate::lzw_model::lemma_L_unfold(2,seq![68u8,80u8],3,ps,st1);
    crate::lzw_model::lemma_L_unfold(2,seq![68u8,80u8],3,ps,st2);
    crate::lzw_model::lemma_L_unfold(2,seq![68u8,80u8],3,ps,st3);
    crate::lzw_model::lemma_L_unfold(2,seq![68u8,80u8],3,ps,st4);
    reveal(crate::lzw_model::ML);
    assert(crate::lzw_model::ML(2,seq![68u8,80u8],3,ps).idx==idx && crate::lzw_model::ML(2,seq![68u8,80u8],3,ps).error is None);
'''+s[end:];p.write_text(s)
print('Staged the positive witness LZW fold per concrete code')
