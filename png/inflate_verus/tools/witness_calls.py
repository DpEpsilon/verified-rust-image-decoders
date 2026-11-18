#!/usr/bin/env python3
"""Parse the restricted straight-line Rust witness language used by the audit.

This is not a Rust compiler. Unsupported expected expressions or control flow
around a harness call are rejected rather than guessed. Proof blocks cannot
supply runtime bindings or public-call credit. No decoder enters this parser.
"""
import re
from functools import lru_cache

TOKEN = re.compile(r'\s+|//[^\n]*|/\*[\s\S]*?\*/|"(?:\\.|[^"\\])*"|0x[0-9a-fA-F_]+(?:u8|u16|u32|u64|usize|nat)?|[0-9][0-9_]*(?:u8|u16|u32|u64|usize|nat)?|[A-Za-z_][A-Za-z_0-9]*|::|->|==|[^\s]')
PAIRS = {'(': ')', '[': ']', '{': '}'}
HARNESS = {'check_at', 'check_public', 'check_policy_parts', 'replay_layout'}

@lru_cache(maxsize=16)
def tokens(text):
    return [t for t in TOKEN.findall(text) if not t.isspace() and not t.startswith(('//','/*'))]

def end_group(ts, start):
    assert ts[start] in PAIRS, ts[start:start+3]
    stack = [PAIRS[ts[start]]]
    for i in range(start+1,len(ts)):
        t = ts[i]
        if t in PAIRS: stack.append(PAIRS[t])
        elif t in PAIRS.values():
            assert stack and stack.pop() == t, ts[start:i+1]
            if not stack: return i+1
    raise AssertionError('unterminated token group')

def split_top(ts, separator=','):
    result=[];start=i=0
    while i<len(ts):
        if ts[i] in PAIRS: i=end_group(ts,i);continue
        if ts[i]==separator:result.append(ts[start:i]);start=i+1
        i+=1
    if start<len(ts):result.append(ts[start:])
    return result

def function(text, name):
    ts=tokens(text);hits=[]
    for i in range(len(ts)-4):
        if ts[i:i+4] == ['fn',name,'(',')'] and ts[i+4]=='{':
            end=end_group(ts,i+4);hits.append(ts[i+5:end-1])
    assert len(hits)==1,(name,'missing or duplicate zero-argument witness function')
    return hits[0]

def declaration(text, name):
    ts=tokens(text);matches=[i for i in range(len(ts)-1) if ts[i:i+2]==['fn',name]]
    assert len(matches)==1,(name,'missing or duplicate declaration')
    i=matches[0];start=ts.index('{',i)
    return ts[i:end_group(ts,start)]

def spec_sequence(text,name):
    ts=tokens(text);needle=['fn',name,'(',')','->','Seq','<','u8','>','{']
    hits=[i for i in range(len(ts)-len(needle)) if ts[i:i+len(needle)]==needle]
    assert len(hits)==1,(name,'missing or duplicate literal sequence spec')
    start=hits[0]+len(needle)-1;end=end_group(ts,start)
    return ts[start+1:end-1]

def number(ts):
    assert len(ts)==1,ts
    return int(re.sub(r'(u8|u16|u32|u64|usize|nat)$','',ts[0]).replace('_',''),0)

def sequence(ts, env, text):
    if len(ts)==1 and ts[0] in env:return sequence(env[ts[0]],env,text)
    if ts[:3]==['seq','!','['] and end_group(ts,2)==len(ts):
        return [number(x) for x in split_top(ts[3:-1])]
    if ts in (tokens('Seq::<u8>::empty()'),tokens('Seq::empty()')):return []
    if ts[:2]==['a_row','('] and ts[-1]==')':
        # This closed evaluator is bound to the authored spec, not an inflate.
        assert tokens('Seq::new(n+1,|i:int| if i == 0 {0u8} else {65u8})') == _a_row_body(text)
        return [0]+[65]*number(ts[2:-1])
    if len(ts)==3 and ts[1:]==['(',')']:
        return sequence(spec_sequence(text,ts[0]),env,text)
    raise AssertionError(('unsupported expected sequence',ts[:12]))

def _a_row_body(text):
    ts=tokens(text);i=ts.index('a_row');j=ts.index('{',i);return ts[j+1:end_group(ts,j)-1]

def array_values(ts, text):
    if len(ts)==3 and ts[1:]==['(',')']:
        all_tokens=tokens(text);matches=[i for i in range(len(all_tokens)-1) if all_tokens[i:i+2]==['fn',ts[0]]]
        assert len(matches)==1,(ts[0],'constructor missing or duplicated')
        start=all_tokens.index('{',matches[0]);ts=all_tokens[start+1:end_group(all_tokens,start)-1]
    assert ts[0]=='[' and ts[-1]==']',('unsupported input constructor',ts[:8])
    return [number(x) for x in split_top(ts[1:-1])]

def byte_array(ts,text):
    return bytes(array_values(ts,text))

def input_parts(name, env, text):
    if name=='wire':return [byte_array(env['wire'],text)]
    parts=env['parts'];assert parts[0]=='[' and parts[-1]==']'
    out=[]
    for ref in split_top(parts[1:-1]):
        assert len(ref)==2 and ref[0]=='&',('unsupported input reference',ref)
        out.append(byte_array(env[ref[1]],text))
    return out

def status(ts):
    if ts==tokens('Status::Done'):return 'Done'
    if ts==tokens('Status::Full'):return 'Full'
    assert len(ts)==8 and ts[:6]==['Status','::','Err','(','E','::'] and ts[-1]==')',ts
    # Path tokenization gives eight tokens for Err(E::Variant).
    return ts[6]

def calls(text,name):
    ts=function(text,name);env={};result=[];i=0
    while i<len(ts):
        if ts[i]=='proof' and i+1<len(ts) and ts[i+1]=='{':
            i=end_group(ts,i+1);continue
        start=i
        while i<len(ts) and ts[i]!=';':
            if ts[i] in PAIRS:i=end_group(ts,i)
            else:i+=1
        stmt=ts[start:i];i+=1
        if not stmt:continue
        if stmt[0]=='let':
            j=1
            while stmt[j] in ('ghost','mut','tracked'):j+=1
            binding=stmt[j];eq=stmt.index('=')
            env[binding]=stmt[eq+1:]
            # Harness calls are required to be standalone statements.
            assert not any(x in HARNESS for x in stmt), (name,'harness in a binding')
            continue
        if stmt[0] not in HARNESS:
            assert not any(x in HARNESS for x in stmt), (name,'harness in unsupported runtime control flow')
            assert stmt[0] in ('hide','reveal','reveal_with_fuel'), (name,'unsupported runtime statement',stmt[:8])
            continue
        harness=stmt[0];assert stmt[1]=='(' and end_group(stmt,1)==len(stmt),(name,stmt[:8])
        args=split_top(stmt[2:-1])
        assert args[0] in (tokens('&wire'),tokens('&parts')),(name,args[0])
        if harness=='replay_layout':
            assert len(args)==5 and args[1]==tokens('&cuts') and args[2]==tokens('&mut parts'),(name,args)
            assert args[3][:2]==['&','mut'] and len(args[3])==3
            array=env[args[3][2]];limit=number(split_top(array[1:-1],';')[1])
            enforce=True;tail=False;expected=tokens('Status::Done');raw=args[4]
        elif harness=='check_public':
            assert len(args)==3
            limit=4;enforce=True;tail=False;expected=args[1];raw=args[2]
        else:
            assert args[1][:2]==['&','mut'] and len(args[1])==3,(name,args[1])
            array=env[args[1][2]]
            assert array[0]=='[' and array[-1]==']', (name,array[:4])
            terms=split_top(array[1:-1],';');assert len(terms)==2
            limit=number(terms[1])
            if harness=='check_at':
                assert len(args)==4
                enforce=True;tail=False;expected=args[2];raw=args[3]
            else:
                assert len(args)==6
                assert args[2] in (['true'],['false']) and args[3] in (['true'],['false'])
                enforce=args[2]==['true'];tail=args[3]==['true'];expected=args[4];raw=args[5]
        assert raw[:2]==['Ghost','('] and raw[-1]==')',(name,raw)
        parts=input_parts(args[0][1],env,text)
        if harness=='replay_layout':
            wire=parts[0];cuts=array_values(env['cuts'],text)
            assert cuts[0]==0 and cuts[-1]==len(wire) and all(a<=b for a,b in zip(cuts,cuts[1:]))
            assert number(split_top(env['parts'][1:-1],';')[1])==len(cuts)-1
            parts=[wire[a:b] for a,b in zip(cuts,cuts[1:])]
        result.append({'input_hex':b''.join(parts).hex(),'segment_lengths':[len(p) for p in parts],'harness':harness,'input':args[0][1], 'limit':limit,'enforce':enforce,'ignore_tail':tail,'status':status(expected),'raw':sequence(raw[2:-1],env,text)})
    return result

def validate(text,name,expected):
    actual=calls(text,name)
    key=lambda x:repr(sorted(x.items()))
    assert sorted(actual,key=key)==sorted(expected,key=key),(name,'public-call descriptors differ',actual,expected)
    assert actual,(name,'no public call')
