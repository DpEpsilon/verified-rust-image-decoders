from pathlib import Path
import re,shutil
root=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src')
backup=root.parent.parent/'runs/sol-20261116'
for name in ['gif.rs','lzw.rs','compose_new.rs','parse.rs']:
 dest=backup/('t12-pre-parser-'+name);assert not dest.exists();shutil.copy2(root/name,dest)
p=root/'parse.rs';s=p.read_text()
names=['read_sub_blocks','loop_count_of','skip_ext','plain_text_ext','app_ext','parse_extension_verified','parse_extension','scan_image_data','parse_data_step','parse_image_step','parse_mb','parse_mp']
# Extend concrete helper interfaces and every concrete parser call.
edits=[]
for name in names:
 hits=list(re.finditer(r'(?<![\w:])'+name+r'\(',s));assert hits,name
 for m in hits:
  o=m.end()-1;depth=1;j=o+1
  while depth:
   if s[j]=='(':depth+=1
   if s[j]==')':depth-=1
   j+=1
  definition=s[max(0,m.start()-4):m.start()].endswith('fn ')
  edits.append((j-1,', steps: &mut Ghost<nat>' if definition else ', steps'))
for i,t in sorted(edits,reverse=True):s=s[:i]+t+s[i:]
def transform(name,fn):
 global s
 start=s.index('fn '+name+'(');end=s.index('\n}',start)+2
 f=s[start:end];nf=fn(f);assert nf!=f,name;s=s[:start]+nf+s[end:]
def contract(f,origin,end,coeff=1,extra=0):
 i=f.index('\n{')
 f=f[:i]+f'''\n        old(steps)@ <= final(steps)@,
        match r {{
            Ok(v) => {origin} as int <= {end} as int <= b.len() as int
                && final(steps)@ as int <= old(steps)@ as int + {coeff} * ({end} as int - {origin} as int) + {extra},
            Err(_) => final(steps)@ as int <= old(steps)@ as int + {coeff} * (b.len() as int - {origin} as int + 1),
        }},'''+f[i:]
 return f
# Exactly one step per body, allowing the final failing availability test.
def sub(f):
 f=contract(f,'q','v.0')
 f=f.replace('    let mut p = q;','    let mut p = q;')
 f=f.replace('            q <= p <= b.len(),','''            q <= p <= b.len(),
            old(steps)@ <= steps@,
            steps@ as int <= old(steps)@ as int + p as int - q as int,
            total as int <= p as int-q as int,''',1)
 i=f.index('\n    {',f.index('    loop'));f=f[:i]+f[i:].replace('\n    {','\n    {\n        proof { steps@ = steps@ + 1; }',1)
 i=f.index('\n{');f=f[:i]+'\n        r is Ok ==> r->Ok_0.1 as int <= r->Ok_0.0 as int-q as int,'+f[i:]
 return f
transform('read_sub_blocks',sub)
def loops(f):
 f=contract(f,'q','v.0',2)
 f=f.replace('    let (end, _) = sub;','    let (end, _) = sub;\n    let ghost scan_cost=steps@;')
 f=f.replace('            q <= p <= b.len(),','''            q <= p <= end <= b.len(),
            old(steps)@ <= scan_cost <= steps@,
            scan_cost as int <= old(steps)@ as int + end as int-q as int,
            steps@ as int <= scan_cost as int+p as int-q as int,''',1)
 i=f.index('\n    {',f.index('    loop'));f=f[:i]+f[i:].replace('\n    {','\n    {\n        proof { steps@ = steps@ + 1; }',1)
 return f
transform('loop_count_of',loops)
for name in ['skip_ext','plain_text_ext']:
 transform(name,lambda f:contract(f,'q','v.0'))
def app(f):
 f=contract(f,'q','v.0',3)
 f=f.replace('    let mut is_netscape = size == 11;','    let ghost initial_cost=steps@;\n    let mut is_netscape = size == 11;')
 f=f.replace('                i <= 11,','''                i <= 11,
                steps@ == initial_cost + i as nat,
                initial_cost == old(steps)@,''',1)
 i=f.index('\n        {',f.index('        while i'));f=f[:i]+f[i:].replace('\n        {','\n        {\n            proof { steps@ = steps@ + 1; }',1)
 return f
transform('app_ext',app)
transform('parse_extension_verified',lambda f:contract(f,'p','v.0',3))
transform('parse_extension',lambda f:contract(f,'p','v',3))
transform('scan_image_data',lambda f:contract(f,'p','v.end'))
transform('parse_data_step',lambda f:contract(f,'c','v.0.end'))
transform('parse_image_step',lambda f:contract(f,'p','v.data.end'))
# Scan payload size is bounded by the newly scanned bytes, not the whole input.
for name,expr in [('scan_image_data','r->Ok_0.data_len as int <= r->Ok_0.end as int-p as int'),('parse_data_step','r->Ok_0.0.data_len as int <= r->Ok_0.0.end as int-c as int'),('parse_image_step','r->Ok_0.data.data_len as int <= r->Ok_0.data.end as int-p as int')]:
 transform(name,lambda f,expr=expr:f[:f.index('\n{')]+'\n        r is Ok ==> '+expr+','+f[f.index('\n{'):])
def walk(f):
 i=f.index('\n{')
 f=f[:i]+'''\n        *old(p) <= *final(p), old(steps)@ <= final(steps)@,
        final(steps)@ as int <= old(steps)@ as int + 4 *
            ((if r is Err {b.len()} else {*final(p)}) as int - *old(p) as int) + 1,'''+f[i:]
 f=f.replace('            fuel>=b.len()-*p,','''            fuel>=b.len()-*p,
            *old(p)<=*p,old(steps)@<=steps@,
            steps@ as int <= old(steps)@ as int + 4 * (*p as int-*old(p) as int),''',1)
 i=f.index('\n    {',f.index('    loop'));f=f[:i]+f[i:].replace('\n    {','\n    {\n        proof { steps@ = steps@ + 1; }',1)
 return f
for name in ['parse_mb','parse_mp']:transform(name,walk)
transform('parse_mb',lambda f:f[:f.index('\n{')]+'''\n        r is Ok && r->Ok_0 is Some ==> r->Ok_0->Some_0.data.data_len as int <= *final(p) as int-*old(p) as int,'''+f[f.index('\n{'):])
p.write_text(s)
# Connect actual parser calls to the real decoder's cumulative counter.
p=root/'gif.rs';s=p.read_text()
def rep(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a));s=s.replace(a,b)
rep('    log: Tracked<AllocLog>,','    log: Tracked<AllocLog>,\n    work: Ghost<nat>,')
rep('saved:None,prev:None,state,still,log:Tracked(log)}','saved:None,prev:None,state,still,log:Tracked(log),work:Ghost(0nat)}')
rep('parse::parse_mb(self.b,&mut self.p,&self.hdr,&self.pol,&mut self.st)','parse::parse_mb(self.b,&mut self.p,&self.hdr,&self.pol,&mut self.st,&mut self.work)')
rep('    let stop=match crate::parse::parse_mp(b,&mut p,&pol,&mut st) {','    let mut work=Ghost(0nat);\n    let stop=match crate::parse::parse_mp(b,&mut p,&pol,&mut st,&mut work) {')
p.write_text(s)
print('Instrumented parser passes and connected MB counter to Frames')
