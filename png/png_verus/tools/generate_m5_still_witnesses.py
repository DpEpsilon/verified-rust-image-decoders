#!/usr/bin/env python3
"""M5 still data/shape projections and two complete palette-error witnesses.

Frozen expected fields alone supply outcomes. M2 authored raw/token intentions
supply inflate certificates; extra traces come from its forward encoder, never
from a decompressor. Palette reconstruction hints are untrusted certificates:
independent MF byte equations check them, and they supply no expected outcome.
"""
import argparse,ast,binascii,hashlib,importlib.util,json,re,sys
from pathlib import Path
sys.dont_write_bytecode=True
A7=[(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]
NAMES={'trns/grey-d1-masked','trns/grey-d16','trns/rgb-d16-low-byte-differs','trns/palette-d1-2-entries-2-alpha','depth16/rounding-boundaries','image/palette-index-out-of-range','image/palette-index-out-of-range-d2','plte/suggested-palette-in-rgb','ok/ct0-d2-il1-7x5','ok/ct3-d4-il0-9x10','ok/ct4-d16-il1-9x10','ok/ct6-d16-il0-17x2','ok16/ct0-d1-il0-7x5','ok16/ct3-d8-il1-7x5','ok16/ct6-d16-il1-7x5'}
def seq(xs):return 'seq!['+','.join(f'{x}u8' for x in xs)+']'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(n,p):
 sp=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m

def chunks(b):
 p=33
 while p<len(b):
  n=int.from_bytes(b[p:p+4],'big');yield p,b[p+4:p+8],b[p+8:p+8+n];p+=12+n
 assert p==len(b)
def streams(b):
 parts=[bytearray()];active=None
 for p,t,d in chunks(b):
  if t==b'IDAT':parts[0].extend(d)
  if t==b'fcTL':active=None
  if t==b'fdAT':
   if active is None:active=bytearray();parts.append(active)
   active.extend(d[4:])
 return [bytes(x) for x in parts]
def forward_library(root):
 path=root.parent/'inflate_verus/tools/author_extended_classes.py';tree=ast.parse(path.read_text())
 allowed={'pack','scan_intent','literal_body','stored_body','codes','encode','match','greedy','rle'}
 nodes=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in allowed or isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id in {'LBASE','LEXTRA','DBASE','DEXTRA'} for t in n.targets)]
 ns={};exec(compile(ast.Module(body=nodes,type_ignores=[]),str(path),'exec'),ns);return ns,path

def steps_for(tokens,fw):
 out=[];p=19;before=0
 for token in tokens+[256]:
  if isinstance(token,int):
   symbol=token;le=ds=de=length=distance=0;q=p+(8 if symbol<144 else 9 if symbol<256 else 7 if symbol<280 else 8);after=before+(symbol<256)
  else:
   length,distance=token;symbol,le,ln,ds,de,dn=fw['codes'](length,distance);q=p+(7 if symbol<280 else 8)+ln+5+dn;after=before+length
  out.append(dict(pos=p,next=q,before=before,after=after,symbol=symbol,le=le,ds=ds,de=de,length=length,distance=distance));p=q;before=after
 return out

def source_for(root,records,name,z,fw):
 path=root.parent/'inflate_verus/src/witness_profiles.rs';text=path.read_text()
 for profile in ['rounding','subframe']:
  literal=re.search(r'spec fn '+profile+r'_z\(\).*?\{ seq!\[(.*?)\] \}',text).group(1);wire=[int(v.strip().removesuffix('u8')) for v in literal.split(',')]
  if bytes(wire)==z:
   literal=re.search(r'spec fn '+profile+r'_raw\(\).*?\{ seq!\[(.*?)\] \}',text).group(1);raw=[int(v.strip().removesuffix('u8')) for v in literal.split(',')]
   return dict(raw=raw,steps=[],encoding='dynamic',profile=profile,source_path=str(path),source_sha256=digest(path))
 for file in ['witness_extended.json','witness_classes.json']:
  path=root.parent/'inflate_verus/tools'/file
  for source in json.loads(path.read_text()):
   representative=source.get('representative',source['members'][0])
   if representative[0] not in records:continue
   rb=bytes.fromhex(records[representative[0]]['input_hex']);rz=streams(rb)[representative[1]]
   if rz[2:-4]==z[2:-4]:
    source=dict(source);source['source_path']=str(path);source['source_sha256']=digest(path)
    if source['encoding']=='fixed-literals' and not source.get('steps'):source['steps']=steps_for(source['raw'],fw)
    return source
 assert name=='depth16/rounding-boundaries',name
 vals=[0,127,128,383,384,32767,32768,32895,32896,65407,65408,65535]
 raw=fw['scan_intent']([(v,) for v in vals],12,1,0,16,0,0)
 if fw['stored_body'](raw)==z[2:-4]:return dict(raw=raw,steps=[],encoding='stored',source_path='spec-frozen/gen_vectors.py:257-260',source_sha256=digest(root.parent/'spec-frozen/gen_vectors.py'))
 candidates=[[v for v in raw]]+[fw['greedy'](raw,1,lazy,False,origin) for lazy in [False,True] for origin in [0,1]]+[fw['rle'](raw)]
 for tokens in candidates:
  if fw['encode'](tokens)==z[2:-4]:return dict(raw=raw,steps=steps_for(tokens,fw),encoding='fixed-matches',source_path='spec-frozen/gen_vectors.py:257-260',source_sha256=digest(root.parent/'spec-frozen/gen_vectors.py'))
 raise AssertionError('No authored forward encoding matches rounding fixture')

def stream_certificate(ident,z,source,limit):
 raw=source['raw'];n=len(raw)
 if source['encoding']=='dynamic':
  assert limit==n
  return [f'spec fn z_{ident}() -> Seq<u8> {{{seq(z)}}}',f'spec fn raw_{ident}() -> Seq<u8> {{{seq(raw)}}}','#[verifier::spinoff_prover]',f'proof fn inflate_{ident}() ensures zm::Inflate(z_{ident}(),{limit}nat,true,false)==(zm::Inflated {{status:zm::InflateStatus::Done,out:raw_{ident}(),r2:false}}),raw_{ident}().len()=={n}nat,','{',f'    crate::m5_dynamic_witness_model::checked_{source["profile"]}();','}']
 ts=source.get('steps',[]);fixed=source['encoding']!='stored';zf,rf,tf=f'z_{ident}()',f'raw_{ident}()',f'ts_{ident}()'
 sb=sum(raw);sp=n+sum((n-j)*v for j,v in enumerate(raw));checksum=(sp%65521)*65536+(1+sb)%65521;wire=int.from_bytes(z[-4:],'big');cursor=(ts[-1]['next']+7)//8 if fixed else 7+n
 assert cursor+4==len(z) and n<=limit
 out=[f'spec fn z_{ident}() -> Seq<u8> {{{seq(z)}}}',f'spec fn raw_{ident}() -> Seq<u8> {{{seq(raw)}}}']
 if fixed:
  val='seq!['+','.join('t::Step {'+','.join(f'{k}:{v}nat' for k,v in step.items())+'}' for step in ts)+']';out+=[f'spec fn ts_{ident}() -> Seq<t::Step> {{{val}}}']
  for k,a in enumerate(range(0,len(ts),16)):
   b=min(a+16,len(ts));out+=['#[verifier::spinoff_prover]',f'proof fn trace_{ident}_{k}() ensures t::trace({zf},{rf},{tf},{a}nat,{b}nat),', '{',f'    hide(t::valid_step);assert(t::trace({zf},{rf},{tf},{a}nat,{b}nat)) by (compute);','}']
  out+=['#[verifier::spinoff_prover]',f'proof fn trace_{ident}() ensures t::trace({zf},{rf},{tf},0,{len(ts)}nat),','{',f'    hide(z_{ident});hide(raw_{ident});hide(ts_{ident});hide(t::trace);']
  for k,a in enumerate(range(0,len(ts),16)):out+=[f'    trace_{ident}_{k}();']
  # trace uses midpoint splits, so prove the root directly from checked leaves.
  out += [f'    assert(t::trace({zf},{rf},{tf},0,{len(ts)}nat)) by (compute);','}']
 facts=[f'{zf}.len()=={len(z)}nat',f'{rf}.len()=={n}nat',f'{zf}[0]==120u8',f'{zf}[1]=={z[1]}u8',f'zm::field_value({zf},16,1)==1nat',f'zm::field_value({zf},17,2)=={1 if fixed else 0}nat',f'zm::be_value({zf},{cursor}nat,4)=={wire}nat']
 if fixed:facts +=[f'{tf}.len()=={len(ts)}nat',f'{tf}[0].pos==19nat',f'{tf}[0].before==0nat',f'{tf}.last().next=={ts[-1]["next"]}nat']
 else:facts +=[f'{zf}.subrange(7,{7+n})=={rf}',f'{zf}[3] as nat+256*{zf}[4] as nat=={n}nat',f'{zf}[5] as nat+256*{zf}[6] as nat==u16::MAX as nat-{n}nat',f'zm::be_value({zf},0,1)==120nat',f'zm::be_value({zf},1,1)=={z[1]}nat']
 out+=['#[verifier::spinoff_prover]',f'proof fn shape_{ident}() ensures '+',\n        '.join(facts)+',','{',*[f'    assert({v}) by (compute);' for v in facts],'}', '#[verifier::spinoff_prover]',f'proof fn checksum_{ident}() ensures zm::Adler({rf})=={checksum}nat,','{',f'    hide(raw_{ident});hide(zm::sum_bytes);hide(zm::sum_prefixes);',f'    assert({rf}.len()=={n}nat) by (compute);',f'    assert(c::sums({rf},0,{n}nat)==({sb}nat,{sp}nat)) by (compute);',f'    c::sums_correct({rf},0,{n}nat);assert({rf}.subrange(0,{n})=~={rf});','    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);',f'    assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521=={checksum}nat) by (compute);','}']
 status='zm::InflateStatus::Done' if checksum==wire else 'zm::InflateStatus::Error(zm::InflateFault::AdlerMismatch)'
 out+=['#[verifier::spinoff_prover]',f'proof fn inflate_{ident}() ensures zm::Inflate({zf},{limit}nat,true,false)==(zm::Inflated {{status:{status},out:{rf},r2:false}}),{rf}.len()=={n}nat,','{',f'    hide(z_{ident});hide(raw_{ident});hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);'+(f'hide(ts_{ident});' if fixed else ''),f'    shape_{ident}();checksum_{ident}();',f'    assert((120nat*256+{z[1]}nat)%31==0nat && ({z[1]}nat/32)%2==0nat) by (compute);']
 if fixed:out +=[f'    trace_{ident}();ms::fixed_stream({zf},{rf},{tf},{limit}nat,{z[1]}nat,{checksum}nat,{wire}nat);']
 else:
  assert limit==n and checksum==wire;out +=[f'    stream::stored_stream({zf},{rf},{n}nat,120nat,{z[1]}nat,{checksum}nat);']
 return out+['}']

def untrusted_row(ft,filtered,prev,bpp):
 hint=[]
 for i,v in enumerate(filtered):
  a=hint[i-bpp] if i>=bpp else 0;b=prev[i] if prev is not None else 0;c=prev[i-bpp] if prev is not None and i>=bpp else 0
  pa,pb,pc=abs(b-c),abs(a-c),abs(a+b-2*c)
  pred=[0,a,b,(a+b)//2,a if pa<=pb and pa<=pc else b if pb<=pc else c][ft];hint.append((v+pred)%256)
 return hint

def scan_certificate(ident,raw,w,h,ct,depth,il,entries,error):
 rf,af=f'raw_{ident}()',f'a_{ident}()';bits={0:1,2:3,3:1,4:2,6:4}[ct]*depth;bpp=max(1,bits//8);base=0;passes=[]
 for x,y,dx,dy in (A7 if il else [(0,0,1,1)]):
  pw=max(0,(w+dx-1-x)//dx);ph=max(0,(h+dy-1-y)//dy);rb=(pw*bits+7)//8;passes.append((pw,ph,rb,base));base+=ph*(1+rb) if pw and ph else 0
 assert base==len(raw)
 result=f'Err(Error::{error})' if error else 'Ok(())';out=['#[verifier::spinoff_prover]',f'proof fn scan_{ident}() ensures r::scan({rf},{af},0)=={result},','{',f'    hide(raw_{ident});assert({rf}.len()=={len(raw)}nat) by (compute);assert(r::count({af})=={len(passes)}nat) by (compute);'];row_results=[]
 for k,(pw,ph,rb,base) in enumerate(passes):
  out +=[f'    assert(r::pw({af},{k}nat)=={pw}nat && r::ph({af},{k}nat)=={ph}nat && r::rb({af},{k}nat)=={rb}nat && r::base({af},{k}nat)=={base}nat) by (compute);']
  prev=None;rr=[]
  for row in range(ph if pw else 0):
   pos=base+row*(1+rb);ft=raw[pos];assert ft<=4;out +=[f'    assert({rf}[{pos}]=={ft}u8) by (compute);'];bad=False
   if ct==3:
    hint=untrusted_row(ft,raw[pos+1:pos+1+rb],prev,bpp);line=f'r::line({rf},{af},{k}nat,{row}nat)';pv='None' if prev is None else f'Some({seq(prev)})';input=f'f::row_data({rf},{base}nat,{rb}nat,{row}nat)'
    out +=[f'    f::pass_row_equation({rf},{base}nat,{rb}nat,{bpp}nat,{row}nat);',f'    assert(f::previous_row({rf},{base}nat,{rb}nat,{bpp}nat,{row}nat)=={pv});',f'    assert(f::encoded_row({ft}u8,{input},{pv},{bpp}nat,{seq(hint)},0,{rb}nat)) by (compute);',f'    f::checked_row_hint({ft}u8,{input},{pv},{bpp}nat,{seq(hint)});',f'    assert({line}=={seq(hint)});']
    sample=lambda x: (hint[x//(8//depth)]>>(8-depth-depth*(x%(8//depth))))&((1<<depth)-1) if depth<8 else hint[x]
    badx=next((x for x in range(pw) if sample(x)>=entries),None);bad=badx is not None
    if bad:
     out +=[f'    assert(r::palette_sample({seq(hint)},{depth}u8,{badx}nat)=={sample(badx)}u8) by (compute);',f'    assert(r::palette_sample({line},{depth}u8,{badx}nat)=={sample(badx)}u8);',f'    assert(!r::palette_valid({line},{depth}u8,{pw}nat,{entries}nat));']
    else:
     out +=[f'    assert forall|x: nat| x<{pw} implies (#[trigger] r::palette_sample({line},{depth}u8,x) as nat)<{entries} by {{']
     for x in range(pw):out +=[f'        '+('if' if x==0 else 'else if')+f' x=={x} {{assert((r::palette_sample({seq(hint)},{depth}u8,{x}nat) as nat)<{entries}nat) by (compute);assert((r::palette_sample({line},{depth}u8,{x}nat) as nat)<{entries}nat);}}']
     out +=['    }',f'    assert(r::palette_valid({line},{depth}u8,{pw}nat,{entries}nat));']
    prev=hint
   rr.append(bad);out +=[f'    assert(r::row_result({rf},{af},{k}nat,{row}nat)=='+('Err(Error::PaletteIndexOutOfRange)' if bad else 'Ok(())')+');']
   if bad:break
  row_results.append(rr)
  if any(rr):break
 if error:assert any(any(x) for x in row_results)
 else:assert not any(any(x) for x in row_results)
 out +=[f'    r::scan_step({rf},{af},{len(passes)}nat);']
 for k in reversed(range(len(row_results))):
  pw,ph,rb,base=passes[k];rr=row_results[k]
  for row in reversed(range(len(rr)+1)):out +=[f'    r::pass_step({rf},{af},{k}nat,{row}nat);']
  out +=[f'    r::scan_step({rf},{af},{k}nat);']
 return out+['}']

def wire_facts(root,walk,cert,b,ident):
 facts,sites=walk.read_facts(b,ident,None);extra=[]
 for p,tag,data in chunks(b):
  if len(data)<=64:continue
  q=p+8+len(data);unique=ident+'_crc_'+str(p);code=cert.crc_certificate(b,unique,p+4,q);code=[l.replace(f'bytes_{unique}()',f'bytes_{ident}()') for l in code];extra+=code
  prefix=f'            assert(crate::critical_model::read(bytes_{ident}(),{p}nat,phase,false)=='
  indexes=[i for i,l in enumerate(facts) if l.startswith(prefix)];assert len(indexes)==1;i=indexes[0]
  crc=int.from_bytes(b[q:q+4],'big');ty=seq(tag)
  facts[i:i+1]=[f'            assert(crate::model::frame(bytes_{ident}(),{p}nat)==Ok(crate::model::WireChunk {{ty:{ty},data:{p+8}nat,len:{len(data)}nat,next:{q+4}nat}})) by (compute);',f'            assert(crate::model::be32(bytes_{ident}(),{q})=={crc}u32) by (compute);',f'            crc_{unique}();']
 return facts,sites,extra

def walk_certificate(root,walk,cert,b,ident,header,znode,entries):
 if any(t==b'acTL' for p,t,d in chunks(b)):
  frames=load('m5_shared_apng_walk',root/'tools/generate_m5_frames_witnesses.py');symbolic=frames.walk_certificate(None,walk,b,ident,header,False)
  doc=f'doc_{ident}()';bf=f'bytes_{ident}()'
  symbolic+=['#[verifier::spinoff_prover]',f'proof fn project_walk_{ident}() ensures r::default_case({bf},{header},{znode},{entries}nat),','{',f'    walk_{ident}();',f'    assert(m::entries({doc})=={entries}nat) by (compute);',f'    assert(m::default_stream({bf},{doc})==Some({znode})) by (compute);','}']
  return symbolic,[f'    project_walk_{ident}();'],''
 facts,sites,extra=wire_facts(root,walk,cert,b,ident)
 if ident=='plte_suggested_palette_in_rgb':
  facts +=[f'        assert(crate::metadata_model::words16(bytes_{ident}().subrange(653,659),3nat)=~=seq![1u16,2u16,3u16]);',f'        assert(crate::metadata_model::parse(crate::walk_model::Name::Bkgd,bytes_{ident}().subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16]))) by {{bkgd_rgb_certificate(bytes_{ident}().subrange(653,659));}};',f'        assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_{ident}().subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {{reveal(crate::metadata_model::parse);}};']
 if any(t==b'acTL' for p,t,data in chunks(b)):facts+=walk.apng_facts(b,ident)
 req=[];pure=[];i=0
 while i<len(facts):
  line=facts[i]
  if line.startswith('        assert forall|'):
   e=line.strip()[len('assert '):-len(' by {')];req.append('('+e.replace(f'bytes_{ident}()','b').replace(' implies ',' ==> ')+')');i+=1
   while facts[i]!='        }':i+=1
  elif line.startswith('        assert('):
   e=line[len('        assert('):];e=e.rsplit(') by ',1)[0] if ') by ' in e else e[:-2]
   if f'bytes_{ident}()' in e:
    req.append(e.replace(f'bytes_{ident}()','b'))
    if line.endswith(' by {'):
     i+=1
     while facts[i]!='        }':i+=1
   else:
    pure.append(line)
    if line.endswith(' by {'):
     i+=1
     while facts[i]!='        }':pure.append(facts[i]);i+=1
     pure.append(facts[i])
  else:raise AssertionError(line)
  i+=1
 p,tag,z=next((p,t,d) for p,t,d in chunks(b) if t==b'IDAT');payload=f'b.subrange({p+8},{p+8+len(z)})=={znode}';req.append(payload)
 hidden='    '+('hide(crate::metadata_model::parse);' if ident=='plte_suggested_palette_in_rgb' else '')+'hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);'
 symbolic=extra+['#[verifier::spinoff_prover]',f'proof fn walk_{ident}(b: Seq<u8>)','    requires '+',\n        '.join([f'b.len()=={len(b)}',f'crate::model::mh(b)==Ok(({header},33nat))']+req)+',',f'    ensures r::default_case(b,{header},{znode},{entries}nat),','{',hidden,*pure,'    reveal(m::step);reveal(m::ordered);reveal(m::own);',f'    reveal_with_fuel(m::drive,{sites+1});','    reveal_with_fuel(m::concat_ranges,3);','}']
 if ident=='plte_suggested_palette_in_rgb':
  states=[f'm::initial({header},33,m::no_events())',
   f'let s=plte_rgb_state_0();m::State {{pos:645nat,seen:s.seen.insert(crate::walk_model::Name::Plte),doc:m::Doc {{plte:Some(crate::walk_model::Range {{offset:41nat,len:600nat}}),info:m::Info {{palette_entries:200nat,..s.doc.info}},..s.doc}},..s}}',
   'let s=plte_rgb_state_1();m::State {pos:663nat,seen:s.seen.insert(crate::walk_model::Name::Bkgd),doc:m::store(s.doc,crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16]),653nat),..s}',
   'let s=plte_rgb_state_2();m::State {pos:1075nat,seen:s.seen.insert(crate::walk_model::Name::Hist),..s}',
   'let s=plte_rgb_state_3();m::State {pos:1109nat,phase:crate::walk_model::Phase::During,doc:m::Doc {idat:seq![crate::walk_model::Range {offset:1083nat,len:22nat}],..s.doc},..s}']
  extra +=['#[verifier::spinoff_prover]','proof fn bkgd_rgb_certificate(d: Seq<u8>)',
   '    requires d.len()==6,crate::metadata_model::words16(d,3)==seq![1u16,2u16,3u16],',
   '    ensures crate::metadata_model::parse(crate::walk_model::Name::Bkgd,d,2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),',
   '{','    let samples=crate::metadata_model::words16(d,3);',
   '    assert forall|i: int| 0<=i<samples.len() implies (#[trigger] samples[i] as nat)<256nat by {if i==0 {}else if i==1 {}else {assert(i==2);}}',
   '    assert((1u32<<8u8)==256u32) by (compute);','}']
  extra +=[f'spec fn plte_rgb_state_{j}() -> m::State {{{value}}}' for j,value in enumerate(states)]
  common=',\n        '.join([f'b.len()=={len(b)}',f'crate::model::mh(b)==Ok(({header},33nat))']+req)
  for j in range(5):
   end=f'm::Step::Next(plte_rgb_state_{j+1}())' if j<4 else 'm::Step::Done(m::Doc {info:m::Info {frame_count:0nat,..plte_rgb_state_4().doc.info},..plte_rgb_state_4().doc},m::no_events())'
   extra +=['#[verifier::spinoff_prover]',f'proof fn plte_rgb_step_{j}(b: Seq<u8>)','    requires '+common+',',f'    ensures m::step(b,m::strict(),false,false,plte_rgb_state_{j}())=={end},','{',hidden.replace('hide(m::drive);',''),*pure,'    reveal(m::step);reveal(m::ordered);reveal(m::own);','}']
  symbolic=extra+['#[verifier::spinoff_prover]',f'proof fn walk_{ident}(b: Seq<u8>)','    requires '+common+',',f'    ensures r::default_case(b,{header},{znode},{entries}nat),','{','    hide(m::drive);hide(crate::critical_model::read);hide(crate::metadata_model::parse);hide(crate::model::mh);']
  symbolic +=[f'    plte_rgb_step_{j}(b);' for j in range(5)]
  symbolic +=[f'    m::drive_step(b,m::strict(),false,false,plte_rgb_state_{j}());' for j in reversed(range(5))]
  symbolic +=['    reveal_with_fuel(m::concat_ranges,3);','}']
 concrete=[f'    assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by {{reveal(crate::model::mh);assert(crate::model::mh(bytes_{ident}())==Ok(({header},33nat))) by (compute);}}',*facts,f'    assert({payload.replace("b.subrange",f"bytes_{ident}().subrange")}) by (compute);',f'    walk_{ident}(bytes_{ident}());']
 return symbolic,concrete,hidden

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1]);ap.add_argument('--names',nargs='*');a=ap.parse_args();root=a.root;fixture=root.parent/'spec-frozen/VECTORS.json';records={r['name']:r for r in json.loads(fixture.read_text())};selected=[r for r in records.values() if (r['name'] in NAMES or r['name'].startswith(('trns16/','apng/'))) and r['entry'] in ['still','still16'] and (not a.names or r['name'] in a.names)]
 fw,author=forward_library(root);walk=load('m5_existing_walk',root/'tools/generate_m3_walk_witnesses.py');cert=load('m5_existing_crc',root/'tools/generate_m4_complete_witnesses.py');fixture_sha=digest(fixture)
 out=['//! M5 data/shape projections and complete bad-index errors; pixel results deferred by E6.','use vstd::prelude::*;','use crate::{document_model as m,raster_model as r,filter_model as f,png::Error,policy::STRICT,','    m4_trace_witness_model as t,m4_checksum_witness_model as c,m4_stream_witness_model as stream,m5_stream_witness_model as ms,m4_crc_witness_model as crc_cert};','use inflate_verus::model as zm;','verus! {'];index=[]
 for record in selected:
  name=record['name'];ident=re.sub('[^a-zA-Z0-9]','_',name).lower();data=bytes.fromhex(record['input_hex']);ih=data[16:29];w,h=int.from_bytes(ih[:4],'big'),int.from_bytes(ih[4:8],'big');depth,ct,il=ih[8],ih[9],ih[12];entries=next((len(d)//3 for p,t,d in chunks(data) if t==b'PLTE'),0);z=streams(data)[0];source=source_for(root,records,name,z,fw);raw=source['raw'];wide=record['entry']=='still16';error=record['expected'].get('error');header=f'crate::model::Ihdr {{width:{w}u32,height:{h}u32,depth:{depth}u8,color_type:{ct}u8,interlace:{il}u8}}';hf=f'h_{ident}()';af=f'a_{ident}()';zf=f'z_{ident}()';bf=f'bytes_{ident}()'
  expected=record['expected'];expected_bytes=sum(q['count']*(len(q['hex'])//2) for q in expected.get('rgba16_runs',expected.get('rgba_runs',[]))) if not error else None
  goal=f'r::Projection::Error(Error::{error})' if error else f'(r::Projection::Ready {{width:{expected["width"]}u32,height:{expected["height"]}u32,bytes:{expected_bytes}nat}})'
  symbolic,concrete,hidden=walk_certificate(root,walk,cert,data,ident,hf,zf,entries)
  out +=[f'// {name}; frozen SHA256 {fixture_sha}',f'spec fn bytes_{ident}() -> Seq<u8> {{{seq(data)}}}',f'spec fn h_{ident}() -> crate::model::Ihdr {{{header}}}',f'spec fn a_{ident}() -> r::Raster {{r::shape({hf},{w}nat,{h}nat,{entries}nat,STRICT)}}',*stream_certificate(ident,z,source,len(raw)),*symbolic,*scan_certificate(ident,raw,w,h,ct,depth,il,entries,error),'#[verifier::spinoff_prover]',f'proof fn model_{ident}() ensures r::still({bf},STRICT,{str(wide).lower()})=={goal},','{',f'    hide(bytes_{ident});hide(z_{ident});hide(raw_{ident});hide(r::data);',hidden,f'    assert({bf}.len()=={len(data)}nat) by (compute);',*concrete,f'    inflate_{ident}();scan_{ident}();',f'    assert(r::raw({af})=={len(raw)}nat) by (compute);',f'    assert(r::data({zf},{af},STRICT)==Some('+('Err(Error::'+error+')' if error else 'Ok(())')+')) by {reveal(r::data);};',f'    r::default_error({bf},{hf},{zf},{entries}nat,Error::{error});' if error else f'    r::default_ready({bf},{hf},{zf},{entries}nat,{str(wide).lower()});','}',f'fn public_{ident}() {{','    let b=['+','.join(f'{v}u8' for v in data)+'];',f'    proof {{assert(b@=={bf});model_{ident}();}}','    let result=crate::png::'+('try_decode_still16' if wide else 'try_decode_still')+'(&b);']
  if error:out+=['    assert(match result {Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>result==Err(crate::alloc_guard::TryError::Format(Error::'+error+'))});']
  else:out +=[f'    assert(match result {{Ok(s)=>s.width=={expected["width"]}u32 && s.height=={expected["height"]}u32 && s.'+('rgba16' if wide else 'rgba')+f'@.len()=={expected_bytes}nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false}});']
  out+=['}'];index.append(dict(name=name,entry=record['entry'],gate_assignment='complete at M5' if error else 'projection at M5, complete at M6',projection='exact standalone error' if error else 'data success, dimensions and physical RGBA byte length; no channel equality',expected_projection={'error':error} if error else {'width':expected['width'],'height':expected['height'],'bytes':expected_bytes},fixture_sha256=fixture_sha,input_sha256=hashlib.sha256(data).hexdigest(),certificate_source=source['source_path'],certificate_source_sha256=source['source_sha256'],model_function=f'model_{ident}',public_function=f'public_{ident}'))
 out+=['} // verus!',''];(root/'src/witness_m5_still.rs').write_text('\n'.join(out));(root/'evidence/m5-still-witness-index.json').write_text(json.dumps({'status':'Generated candidates; verification credit recorded separately','generator':'tools/generate_m5_still_witnesses.py','records':index},indent=2)+'\n');print('Generated',len(index),'M5 still candidates; no proof credit from generation.')
if __name__=='__main__':main()
