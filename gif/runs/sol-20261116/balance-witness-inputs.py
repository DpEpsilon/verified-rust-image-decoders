from pathlib import Path
import json,re,shutil
r=Path('/home/ws/projects/verified-decoders/gif');p=r/'gif_verus/src/model_vectors.rs';s=p.read_text();backup=r/'runs/sol-20261116/model-vectors-six-staged-deep-input.rs';assert not backup.exists();shutil.copy2(p,backup)
vs={v['name']:v for v in json.loads((r/'spec-frozen/VECTORS.json').read_text())}
def tree(data,lo,hi):
 if hi-lo==1:return str(data[lo])+'u8'
 mid=(lo+hi)//2
 return f'if i<{mid} {{'+tree(data,lo,mid)+'} else {'+tree(data,mid,hi)+'}'
for name in ['empty','header-12','bad-signature','version-88a','header-only-gct-trailer/still','two-colour-m2']:
 key=re.sub('[^a-z0-9]+','_',name).strip('_');data=bytes.fromhex(vs[name]['input_hex']);body='Seq::empty()' if not data else f'Seq::new({len(data)},|i:int|'+tree(data,0,len(data))+')'
 pat=rf'pub open spec fn bytes_{key}\(\)->Seq<u8> \{{[^\n]*\}}';assert len(re.findall(pat,s))==1
 s=re.sub(pat,f'pub open spec fn bytes_{key}()->Seq<u8> {{{body}}}',s)
p.write_text(s)
print('Represented fixed input bytes with bounded-depth lookup trees, without changing any expected result')
