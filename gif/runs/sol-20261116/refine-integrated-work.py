from pathlib import Path
root=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src')
p=root/'compose_new.rs';s=p.read_text();a='work@ as int <= 2 * (r.width as int) * (r.height as int),';assert s.count(a)==2;s=s.replace(a,'work@ as int <= 2 * ((r.width as int) * (r.height as int)),');p.write_text(s)
p=root/'gif.rs';s=p.read_text()
a='rep.work@ as int <= 5 * im.data.data_len as int + 2 * (im.fh.rect.width as int) * (im.fh.rect.height as int),';assert s.count(a)==1;s=s.replace(a,'rep.work@ as int <= 5 * im.data.data_len as int + 2 * ((im.fh.rect.width as int) * (im.fh.rect.height as int)),')
a='        assert(header_pixels(b@) as int==(hdr.width as int)*(hdr.height as int));';assert s.count(a)==1
s=s.replace(a,a+'''
        assert(4*(hdr.width as int)*(hdr.height as int)==4*header_pixels(b@) as int) by(nonlinear_arith)
            requires header_pixels(b@) as int==(hdr.width as int)*(hdr.height as int);
''')
p.write_text(s)
