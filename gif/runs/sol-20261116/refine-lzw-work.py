from pathlib import Path
root=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src')
p=root/'lzw.rs';s=p.read_text()
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
rep('        1 <= m <= 11,\n        m_ok(lzw_ps(m,opts), m as int, area as int),','        1 <= m <= 11, area >= 1,\n        m_ok(lzw_ps(m,opts), m as int, area as int),')
rep('    proof { assert(stop == (opts.tail == Tail::StopAtRaster)); reveal(m_ok); assert(area >= 1); }','    proof { assert(stop == (opts.tail == Tail::StopAtRaster)); reveal(reader_ready); }')
rep('            reader.fetched as int >= fetched0,','            reader.fetched as int >= fetched0, fetched0 >= 0,')
rep('            CodeStep::Halt {error,pos} => {\n                proof {','            CodeStep::Halt {error,pos} => {\n                proof {\n                    reveal(reader_ready);\n                    assert(steps@ as int <= 5 * data_len as int + 2 * area as int) by(linear_arith);')
rep('        if len>area-q {\n            proof {','        if len>area-q {\n            proof {\n                reveal(reader_ready);\n                assert(steps@ as int <= 5 * data_len as int + 2 * area as int) by(linear_arith);')
rep('    requires b.len()<=crate::limits::MAX_INPUT,area<=crate::limits::MAX_TOTAL_PIXELS,','    requires b.len()<=crate::limits::MAX_INPUT,1<=area<=crate::limits::MAX_TOTAL_PIXELS,')
p.write_text(s)
p=root/'compose_new.rs';s=p.read_text();a='    requires b.len()<=crate::limits::MAX_INPUT,area<=crate::limits::MAX_TOTAL_PIXELS,';assert s.count(a)==2;s=s.replace(a,'    requires b.len()<=crate::limits::MAX_INPUT,1<=area<=crate::limits::MAX_TOTAL_PIXELS,');p.write_text(s)
p=root/'gif.rs';s=p.read_text();a='    crate::compose_new::decode_canvas(b,data,area32,opts,pol.lzw_min,t,cv,colors,fr,hdr.width as u32,hdr.height as u32)';assert s.count(a)==1;s=s.replace(a,'''    let rep=crate::compose_new::decode_canvas(b,data,area32,opts,pol.lzw_min,t,cv,colors,fr,hdr.width as u32,hdr.height as u32);
    proof {
        assert(2 * area32 as int == 2 * (fh.rect.width as int) * (fh.rect.height as int)) by(nonlinear_arith)
            requires area32 as int == (fh.rect.width as int) * (fh.rect.height as int);
    }
    rep''');p.write_text(s)
