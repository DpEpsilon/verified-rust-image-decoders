"""Independent forward Huffman candidate, never a decoder."""
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
import author_extended_classes as a

def lengths(freq):
    freq=list(freq);limit=len(freq);depth=[0]*(2*limit+1);parent=[None]*(2*limit+1)
    heap=[0]+[i for i,v in enumerate(freq) if v]
    for i in range(limit):
        if len(heap)>2:break
        if not freq[i]:freq[i]=1;heap.append(i)
    freq.extend([0]*(limit+1))
    def smaller(x,y):return freq[x]<freq[y] or freq[x]==freq[y] and depth[x]<=depth[y]
    def down(k):
        v=heap[k];j=k*2
        while j<len(heap):
            if j+1<len(heap) and smaller(heap[j+1],heap[j]):j+=1
            if smaller(v,heap[j]):break
            heap[k]=heap[j];k=j;j*=2
        heap[k]=v
    for k in range((len(heap)-1)//2,0,-1):down(k)
    node=limit
    while len(heap)>2:
        n=heap[1];heap[1]=heap[-1];heap.pop();down(1);m=heap[1]
        freq[node]=freq[n]+freq[m];depth[node]=max(depth[n],depth[m])+1
        parent[n]=parent[m]=node;heap[1]=node;node+=1;down(1)
    out=[]
    for s in range(limit):
        d=0;p=s
        while parent[p] is not None:d+=1;p=parent[p]
        out.append(d)
    return out

def runs(values):
    out=[];previous=-1;count=0;nextlen=values[0];maximum,minimum=(138,3) if nextlen==0 else (7,4)
    for i in range(len(values)):
        cur=nextlen;nextlen=values[i+1] if i+1<len(values) else -1;count+=1
        if count<maximum and cur==nextlen:continue
        if count<minimum:out.extend((cur,0,0) for _ in range(count))
        elif cur!=0:
            if cur!=previous:out.append((cur,0,0));count-=1
            out.append((16,count-3,2))
        elif count<=10:out.append((17,count-3,3))
        else:out.append((18,count-11,7))
        count=0;previous=cur
        maximum,minimum=(138,3) if nextlen==0 else (6,3) if cur==nextlen else (7,4)
    return out

def words(ls):
    counts=[ls.count(n) for n in range(16)];counts[0]=0;nextcode=[0]*16
    for n in range(1,16):nextcode[n]=(nextcode[n-1]+counts[n-1])*2
    out=[]
    for n in ls:
        out.append(nextcode[n] if n else 0)
        if n:nextcode[n]+=1
    return out

def encode(tokens):
    lf=[0]*286;df=[0]*30;lf[256]=1
    for t in tokens:
        if isinstance(t,int):lf[t]+=1
        else:s,le,ln,ds,de,dn=a.codes(*t);lf[s]+=1;df[ds]+=1
    lit=lengths(lf);dist=lengths(df)
    lit=lit[:max(i for i,v in enumerate(lit) if v)+1];dist=dist[:max(i for i,v in enumerate(dist) if v)+1]
    rr=runs(lit)+runs(dist);cf=[0]*19
    for s,_,_ in rr:cf[s]+=1
    cl=lengths(cf);order=[16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15]
    nc=max(4,max(i for i,s in enumerate(order) if cl[s])+1)
    bits=[1,0,1]
    def field(v,n):bits.extend((v>>i)&1 for i in range(n))
    def code(s,ls):
        w=words(ls)[s];bits.extend((w>>i)&1 for i in range(ls[s]-1,-1,-1))
    field(len(lit)-257,5);field(len(dist)-1,5);field(nc-4,4)
    for s in order[:nc]:field(cl[s],3)
    for s,ex,n in rr:code(s,cl);field(ex,n)
    start=16+len(bits)
    for t in tokens+[256]:
        if isinstance(t,int):code(t,lit)
        else:s,le,ln,ds,de,dn=a.codes(*t);code(s,lit);field(le,ln);code(ds,dist);field(de,dn)
    end=16+len(bits)
    wire=bytes(sum(v<<j for j,v in enumerate(bits[i:i+8])) for i in range(0,len(bits),8))
    return wire,lit,dist,cl,rr,start,end

