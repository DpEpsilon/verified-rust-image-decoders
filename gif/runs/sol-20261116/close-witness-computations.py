from pathlib import Path
import re
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/model_vectors.rs');s=p.read_text();start=s.index('pub proof fn model_two_colour_m2()');end=s.index('\npub fn executable_two_colour_m2',start);f=s[start:end]
consts={}
for name in ['b','p','h','w','info','fh','after','idx','cv','table','rgba']:
 hits=re.findall(r'\blet '+name+r'=(.*?);',f,re.S);assert len(hits)==1,(name,hits);consts[name]=hits[0].strip()
# Compute assertions must receive closed terms, not let-bound SMT variables.
pat=re.compile(r'\b('+ '|'.join(consts)+r')\b')
seen=[0]
def change(m):
 seen[0]+=1
 return 'assert('+pat.sub(lambda x:'('+consts[x.group(0)]+')',m.group(1))+') by(compute);'
f=re.sub(r'assert\((.*?)\) by\(compute\);',change,f,flags=re.S)
assert seen[0]==6,seen
s=s[:start]+f+s[end:];p.write_text(s)
print('Closed all six staged compute expressions over concrete frozen terms')
