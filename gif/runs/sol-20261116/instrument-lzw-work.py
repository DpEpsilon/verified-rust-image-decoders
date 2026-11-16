from pathlib import Path
import shutil
root=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src')
backup=root.parent.parent/'runs/sol-20261116'
for name in ['lzw.rs','compose_new.rs','gif.rs','parse.rs']:
    dest=backup/('t12-before-'+name)
    assert not dest.exists(),dest
    shutil.copy2(root/name,dest)
p=root/'lzw.rs'; s=p.read_text()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(a,s.count(a),n)
    s=s.replace(a,b)
rep('fn read(&mut self, width: u32)', 'fn read(&mut self, width: u32, steps: &mut Ghost<nat>)')
rep('            final(self).bits_left == old(self).bits_left - width,','            final(self).bits_left == old(self).bits_left - width,\n            final(self).fetched >= old(self).fetched,\n            final(steps)@ as int == old(steps)@ as int + final(self).fetched as int - old(self).fetched as int,')
rep('                self.fetched as int == (gf) + (bytes as int),','                self.fetched as int == (gf) + (bytes as int),\n                steps@ == old(steps)@ + bytes as nat,')
# Charge each reader-fill body, not the fetch helper (which has no loop).
rep('        while self.nacc < width','        while self.nacc < width') if False else None
start=s.index('        while self.nacc < width'); body=s.index('\n        {',start)
s=s[:body]+s[body:].replace('\n        {','\n        {\n            proof { steps@ = steps@ + 1; }',1)
rep('    pub colour: Option<ColourFault>,','    pub colour: Option<ColourFault>,\n    /// Measured reader-fill, raw-chain and outer-decoder loop iterations.\n    /// The concrete Drawer put path has no loop or bulk fill/copy call.\n    pub work: Ghost<nat>,')
rep('    sink: &mut S) -> (r: (u8, Option<u32>))','    sink: &mut S, steps: &mut Ghost<nat>) -> (r: (u8, Option<u32>))')
idx=s.index('fn emit_raw'); e=s.index('\n{',idx)
s=s[:e]+'\n        final(steps)@ == old(steps)@ + len as nat,'+s[e:]
rep('            0 <= j <= len <= TABLE_SIZE, s.len() == len, len >= 1,','            0 <= j <= len <= TABLE_SIZE, s.len() == len, len >= 1,\n            steps@ + j as nat == old(steps)@ + len as nat,')
rep('        let ghost k = len as int - j as int;','        proof { steps@ = steps@ + 1; }\n        let ghost k = len as int - j as int;')
rep('    sink: &mut S) -> (r: (u32,u8,Option<u32>))','    sink: &mut S, steps: &mut Ghost<nat>) -> (r: (u32,u8,Option<u32>))')
idx=s.index('fn run_code'); e=s.index('\n{',idx)
s=s[:e]+'\n        final(steps)@ == old(steps)@ + r.0 as nat,'+s[e:]
rep('emit_raw(t,clear,code,q,area,len,sink)','emit_raw(t,clear,code,q,area,len,sink,steps)')
rep('state: Ghost<crate::lzw_model::St>) -> (r: CodeStep)','state: Ghost<crate::lzw_model::St>, steps: &mut Ghost<nat>) -> (r: CodeStep)')
idx=s.index('fn read_step'); e=s.index('\n{',idx)
s=s[:e]+'''\n        final(reader).fetched >= old(reader).fetched,
        final(reader).bits_left <= old(reader).bits_left,
        final(reader).total_bits == old(reader).total_bits,
        final(steps)@ as int == old(steps)@ as int + final(reader).fetched as int - old(reader).fetched as int,
        !(r is Halt) ==> final(reader).bits_left as int + 2 <= old(reader).bits_left as int,'''+s[e:]
rep('reader.read(width)','reader.read(width,steps)')
# The outer invariant charges completed iterations to two consumed bits,
# chain bodies to produced indices, and fill bodies to fetched bytes.
rep('    let input = reader.b;','    let mut steps = Ghost(0nat);\n    let ghost fetched0 = reader.fetched as int;\n    let input = reader.b;')
rep('            1 <= m <= 11, m_ok(ps,m as int,area as int), ps == lzw_ps(m,opts),','''            1 <= m <= 11, m_ok(ps,m as int,area as int), ps == lzw_ps(m,opts),
            area >= 1,
            reader.fetched as int >= fetched0,
            2 * (steps@ as int - reader.fetched as int + fetched0 - q as int)
                <= reader.total_bits as int - reader.bits_left as int,
            reader.total_bits as int == 8 * data_len as int,''')
rep('        let ghost op = raw_pf_seq(t@);','        proof { steps@ = steps@ + 1; }\n        let ghost op = raw_pf_seq(t@);')
rep('opts,Ghost(st))','opts,Ghost(st),&mut steps)')
rep('q,area,opts.checked_literals,sink);','q,area,opts.checked_literals,sink,&mut steps);')
rep('times:Ghost(times),colour}', 'times:Ghost(times),colour,work:Ghost(steps@)}',2)
# Explicit area positivity follows from m_ok, including the restricted m=1 case.
rep('    proof { assert(stop == (opts.tail == Tail::StopAtRaster)); }','    proof { assert(stop == (opts.tail == Tail::StopAtRaster)); reveal(m_ok); assert(area >= 1); }')
idx=s.index('pub fn decode<S: Sink>'); e=s.index('\n{',idx)
s=s[:e]+'\n        rep.work@ <= 5 * data_len as nat + 2 * area as nat,'+s[e:]
idx=s.index('pub fn decode_image_data'); e=s.index('\n{',idx)
s=s[:e]+'\n        rep.work@ <= 5 * data.data_len as nat + 2 * area as nat,'+s[e:]
p.write_text(s)
# Lift the measured report bound, without changing the executable interfaces.
for name,fun,clause in [
    ('compose_new.rs','pub fn decode_drawer','rep.work@ <= 5 * data.data_len as nat + 2 * area as nat,'),
    ('compose_new.rs','pub fn decode_canvas','rep.work@ <= 5 * data.data_len as nat + 2 * area as nat,'),
    ('gif.rs','fn decode_image_pixels','rep.work@ as int <= 5 * im.data.data_len as int + 2 * (im.fh.rect.width as int) * (im.fh.rect.height as int),')]:
    p=root/name; t=p.read_text(); assert t.count(fun)==1
    i=t.index(fun); e=t.index('\n{',i)
    t=t[:e]+'\n        '+clause+t[e:]; p.write_text(t)
print('Instrumented LZW core and lifted report bounds; four source backups saved')
