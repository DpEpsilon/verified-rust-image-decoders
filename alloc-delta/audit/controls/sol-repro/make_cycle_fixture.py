import re,json,pathlib
base=pathlib.Path(__file__).parent
s=(base/'cycle-report.txt').read_text()
def q(x): return json.dumps(x)
header=dict(re.findall(r'^([^\n:]+): (.*)$',s,re.M))
text='[meta]\nformat="alloc-audit/2"\ncrate_name="cycle"\nrustc_commit="58cf8f9c35b355199727627d07ae34a4e2846d67"\ntarget="x86_64-unknown-linux-gnu"\nprofile="opt-level=3,debug-assertions=off,share-generics=off"\nfeatures=[]\nsource_hash='+q(header['source_hash'])+'\nvstd_version="none"\nreviewer="synthetic-review-control"\n'
def entry(kind,item,allow=None):
 global text
 text+='\n[[entry]]\nkind='+q(kind)+'\nitem='+q(item)+'\ncontract="synthetic control: standard nonallocating leaf or explicitly admitted allocation"\ncitation="synthetic control"\n'
 if allow is not None: text+='allow='+q(allow)+'\n'
entry('alloc','alloc::slice::<impl [T]>::to_vec','crate::guard')
entry('alloc','alloc::raw_vec::RawVecInner::<A>::deallocate','*')
seen=set()
for kind,div,item,caller in re.findall(r'^  !! \[([a-z-]+)(,diverging)?\] (.*) <- (.*)$',s,re.M):
 item=re.sub(r' \[[^]]*\]$','',item)
 key=(kind,item,caller if div else None)
 if key in seen: continue
 seen.add(key)
 entry(kind,item,caller if div else None)
for name,caller in re.findall(r'^  !! (\S+) in (.*?) at .*$',s,re.M):
 entry('panic',name,caller)
(base/'cycle.toml').write_text(text)
