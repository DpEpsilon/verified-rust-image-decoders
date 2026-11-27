#!/usr/bin/env python3
"""Independent frozen filter literal audit; no generator or decoder imports."""
import argparse
import hashlib
import json
from pathlib import Path
import re

def wire(text):
 alphabet='0123456789abcdef';out=bytearray()
 for at in range(0,len(text),2):out.append(16*alphabet.index(text[at].lower())+alphabet.index(text[at+1].lower()))
 return bytes(out)
def expected_bytes(record):
 expected=record['expected'];assert set(expected)=={'width','height','rgba_runs'};out=bytearray()
 for run in expected['rgba_runs']:
  block=wire(run['hex']);remaining=run['count']
  while remaining:
   for octet in block:out.append(octet)
   remaining-=1
 assert len(out)==4*expected['width']*expected['height'];return bytes(out)
def definition(source,name):
 m=re.search(r'\bfn '+re.escape(name)+r'\([^\n]*?\)\s*->[^\n{]+\{([^{}]*)\}',source);assert m,name
 return m[1]
def parse_sequence(text):
 text=text.strip();assert text.startswith('seq![') and text.endswith(']')
 words=text[5:-1].split(',') if text[5:-1] else [];out=[]
 for word in words:
  assert re.fullmatch(r'\d+u8',word);out.append(int(word[:-2]))
 return bytes(out)
def audit(record,source):
 ident=re.sub('[^A-Za-z0-9]','_',record['name']).lower();expected=record['expected'];pixels=expected_bytes(record)
 assert parse_sequence(definition(source,'bytes_'+ident))==wire(record['input_hex'])
 assert parse_sequence(definition(source,'expected_'+ident))==pixels
 predicate=re.sub(r'\s+','',definition(source,'expected_value_'+ident))
 assert predicate==f'value.width=={expected["width"]}u32&&value.height=={expected["height"]}u32&&value.pixels==expected_{ident}()'
 start=source.index('pub fn public_complete_'+ident+'(');end=source.index('\n}',start);public=source[start:end]
 actual=re.search(r'let b=\[([^]]*)\];',public);assert actual
 assert parse_sequence('seq!['+actual[1]+']')==wire(record['input_hex'])
 assert public.count('crate::png::try_decode_still(&b)')==1
 assert 'requires' not in public
 return dict(name=record['name'],width=expected['width'],height=expected['height'],pixels=expected['width']*expected['height'],bytes=len(pixels),sha256=hashlib.sha256(pixels).hexdigest())
SCHEDULED={f'interlace/ct{ct}-d{depth}-{width}x{height}' for ct,depth in [(0,1),(3,4),(6,16)] for width,height in [(1,1),(1,2),(1,5),(1,9),(2,1),(2,2),(3,3),(4,4),(5,1),(5,5),(8,8),(9,1)]}
def check_schedule(index):
 names=[r['name'] for r in index['records']]
 assert len(names)==len(set(names)), 'Duplicate witness index entry'
 assert set(names)==SCHEDULED, 'Witness index does not equal independent full E6 A7 schedule'
def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,required=True);a=p.parse_args();r=a.root
 vectors={x['name']:x for x in json.loads((r.parent/'spec-frozen/VECTORS.json').read_text())};index=json.loads((r/'evidence/m6-e6-m4-a7-witness-index.json').read_text());source=(r/'src/witness_m6_m4_a7.rs').read_text()
 check_schedule(index);checked=[]
 for row in index['records']:
  record=vectors[row['name']];assert row['expected']==record['expected'];checked.append(audit(record,source))
 first=vectors[index['records'][0]['name']];ident=re.sub('[^A-Za-z0-9]','_',first['name']).lower();literal=definition(source,'expected_'+ident);m=re.search(r'\d+u8',literal);assert m
 changed=literal[:m.start()]+str((int(m[0][:-2])+1)%256)+'u8'+literal[m.end():]
 variants={'pixel_byte':source.replace(literal,changed,1),'width':source.replace('value.width==8u32','value.width==7u32',1),'height':source.replace('value.height==8u32','value.height==7u32',1)};rejected=[]
 for name,mutated in variants.items():
  assert mutated!=source
  try:audit(first,mutated)
  except AssertionError:rejected.append(name)
  else:raise AssertionError('Auditor accepted changed '+name)
 index_rejected=[]
 for label,names in [('duplicate',[r['name'] for r in index['records'][:-1]]+[index['records'][0]['name']]),('omitted',[r['name'] for r in index['records'][:-1]]),('unexpected',[r['name'] for r in index['records'][:-1]]+['filter/not-scheduled'])]:
  try:check_schedule({'records':[{'name':name} for name in names]})
  except AssertionError:index_rejected.append(label)
  else:raise AssertionError('Auditor accepted '+label+' schedule')
 print(json.dumps(dict(status='PASS',negative_index_controls_rejected=index_rejected,independent_scheduled_names=sorted(SCHEDULED),records=len(checked),fresh_pixels=sum(x['pixels'] for x in checked),physical_bytes=sum(x['bytes'] for x in checked),negative_literal_controls_rejected=rejected,scope='Independent frozen literal audit only; not a decoder mutation campaign',audited=checked),indent=2))
if __name__=='__main__':main()
