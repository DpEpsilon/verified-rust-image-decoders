from pathlib import Path
import shutil
r=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src');b=r.parent.parent/'runs/sol-20261116'
for name in ['gif.rs','parse.rs','lib.rs']:
 dest=b/('t12-identification-before-'+name);assert not dest.exists();shutil.copy2(r/name,dest)
p=r/'parse.rs';s=p.read_text();start=s.index('pub fn parse_mb(');end=s.index('\n}',start)+2;f=s[start:end]
i=f.index('\n{');f=f[:i]+'''\n        (if r is Err {b.len() as int} else {*final(p) as int}) == crate::work_model::mb_cur(b@,header_model(*hdr),*pol,walk_m_at(*old(st),*old(p))),
        (match r {Ok(Some(im))=>(im.fh.rect.width as nat)*(im.fh.rect.height as nat),_=>0nat})
            ==crate::work_model::mb_area(b@,header_model(*hdr),*pol,walk_m_at(*old(st),*old(p))),'''+f[i:]
f=f.replace('    let ghost mut fuel=(b.len()-*p) as nat;','''    let ghost prefix_initial=crate::model::mp_fuel(b@,*pol,walk_m_at(*st,*p),(b.len()-*p) as nat);
    let ghost mut fuel=(b.len()-*p) as nat;''')
f=f.replace('    hide(crate::model::mb_fuel);','    hide(crate::model::mb_fuel);hide(crate::model::mp_fuel);')
f=f.replace('            initial==crate::model::mb_fuel(b@,header_model(*hdr),*pol,walk_m_at(*st,*p),fuel),','''            initial==crate::model::mb_fuel(b@,header_model(*hdr),*pol,walk_m_at(*st,*p),fuel),
            prefix_initial==crate::model::mp_fuel(b@,*pol,walk_m_at(*old(st),*old(p)),(b.len()-*old(p)) as nat),
            prefix_initial==crate::model::mp_fuel(b@,*pol,walk_m_at(*st,*p),fuel),''')
f=f.replace('        proof {reveal_with_fuel(crate::model::mb_fuel,1);}','        proof {reveal_with_fuel(crate::model::mb_fuel,1);reveal_with_fuel(crate::model::mp_fuel,1);}')
s=s[:start]+f+s[end:];p.write_text(s)
print('Added independent structural cursor and area observations to actual MB')
