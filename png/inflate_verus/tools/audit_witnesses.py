#!/usr/bin/env python3
"""Check authored witness input provenance without consulting an inflate oracle."""
import ast
import hashlib
import sys
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
import witness_calls


def idat(data):
    pos = 8
    parts = []
    while pos + 12 <= len(data):
        size = int.from_bytes(data[pos:pos + 4], 'big')
        assert pos + 12 + size <= len(data)
        if data[pos + 4:pos + 8] == b'IDAT':
            parts.append(data[pos + 8:pos + 8 + size])
        pos += 12 + size
    assert parts, 'This checker covers the authored IDAT witness subset only'
    return b''.join(parts)


def chunks(data):
    pos = 8
    while pos + 12 <= len(data):
        size = int.from_bytes(data[pos:pos+4], 'big')
        if size > 1 << 28 or pos+12+size > len(data):
            return
        yield data[pos+4:pos+8], data[pos+8:pos+8+size]
        pos += 12+size


def streams(data):
    """Frozen extractor presence rules, including empty IDAT and fdAT payloads."""
    idat_parts = None
    frames = []
    for kind, payload in chunks(data):
        if kind == b'IDAT':
            if idat_parts is None:
                idat_parts = []
            idat_parts.append(payload)
        elif kind == b'fcTL':
            frames.append(None)
        elif kind == b'fdAT' and frames and len(payload) >= 4:
            if frames[-1] is None:
                frames[-1] = []
            frames[-1].append(payload[4:])
    return [b''.join(parts) for parts in ([] if idat_parts is None else [idat_parts]) + [f for f in frames if f is not None]]


def image_r(data, dimensions=None):
    """Independent legal IHDR/Adam7 row-byte count for IDAT class membership."""
    headers = [p for k,p in chunks(data) if k == b'IHDR' and len(p) == 13]
    if len(headers) != 1:
        return None
    h = headers[0]
    w, height = int.from_bytes(h[:4], 'big'), int.from_bytes(h[4:8], 'big')
    depth, color, compression, filtering, interlace = h[8:]
    legal = {0: (1,2,4,8,16), 2: (8,16), 3: (1,2,4,8), 4: (8,16), 6: (8,16)}
    if not (0 < w < 2**31 and 0 < height < 2**31 and color in legal and depth in legal[color]
            and compression == filtering == 0 and interlace in (0,1)):
        return None
    if dimensions is not None:
        w, height = dimensions
        if not (0 < w < 2**31 and 0 < height < 2**31):
            return None
    channels = {0:1,2:3,3:1,4:2,6:4}[color]
    grids = [(0,0,1,1)] if interlace == 0 else [(0,0,8,8),(4,0,8,8),(0,4,4,8),(2,0,4,4),(0,2,2,4),(1,0,2,2),(0,1,1,2)]
    total = 0
    for x,y,dx,dy in grids:
        cols = len(range(x,w,dx)); rows = len(range(y,height,dy))
        if cols and rows:
            total += rows * (1+(cols*channels*depth+7)//8)
    return total


def stream_shapes(data):
    """Associate every extracted stream with its own original control record."""
    default = None
    frames = []
    for kind, payload in chunks(data):
        if kind == b'IDAT':
            if default is None:
                default = []
            default.append(payload)
        elif kind == b'fcTL':
            frames.append([payload, None])
        elif kind == b'fdAT' and len(payload) >= 4 and frames:
            if frames[-1][1] is None:
                frames[-1][1] = []
            frames[-1][1].append(payload[4:])
    shaped = [] if default is None else [(image_r(data), default, None)]
    for control, parts in frames:
        if parts is None:
            continue
        dims = (int.from_bytes(control[4:8], 'big'), int.from_bytes(control[8:12], 'big'))
        r = image_r(data, dims) if len(control) == 26 else None
        shaped.append((r, parts, control))
    assert [b''.join(parts) for _, parts, _ in shaped] == streams(data)
    return shaped


def expected_call(parts, limit, enforce, tail, status, raw, harness='check_policy_parts'):
    return {'input_hex':b''.join(parts).hex(), 'segment_lengths':[len(p) for p in parts],
            'harness':harness, 'input':'wire' if harness != 'check_policy_parts' else 'parts',
            'limit':limit, 'enforce':enforce, 'ignore_tail':tail, 'status':status, 'raw':raw}


def validate_bindings(vectors, policy_vectors, source_overrides=None, manifest_overrides=None):
    """Bind coverage to parsed, unconditional runtime harness statements."""
    source_overrides = source_overrides or {}
    manifest_overrides = manifest_overrides or {}
    def source(name):
        return source_overrides[name] if name in source_overrides else (ROOT/'src'/name).read_text()
    def manifest(name):
        return manifest_overrides[name] if name in manifest_overrides else json.loads((ROOT/'tools'/name).read_text())
    for harness in manifest('witness_harnesses.json'):
        ts = witness_calls.declaration(source(harness['source']),harness['function'])
        assert hashlib.sha256(' '.join(ts).encode()).hexdigest() == harness['token_sha256'], harness
    individual = manifest('witness_individual.json')
    individual_keys = set()
    assert len(individual) == 48
    for claim in individual:
        name,index = claim['record'],claim['stream']
        assert (name,index) not in individual_keys, name
        individual_keys.add((name,index))
        mapping = re.findall(r'/// ([\w/-]+):[^\n]*\n(?:#\[[^\n]*\]\s*)?fn (\w+)\(\)',source(claim['source']))
        assert (name,claim['function']) in mapping, name
        r,parts,_ = stream_shapes(bytes.fromhex(vectors[name]['input_hex']))[index]
        assert claim['r'] == r and r is not None, name
        assert claim['input_hex'] == b''.join(parts).hex(), name
        assert len(claim['calls']) == 1, name
        call = claim['calls'][0]
        assert call['limit'] == r and call['enforce'] and not call['ignore_tail'], name
        assert call['input_hex'] == claim['input_hex'] and len(call['raw']) <= r, name
        assert call['harness'] in ('check_at','check_public'), name
        witness_calls.validate(source(claim['source']),claim['function'],claim['calls'])
    for filename,src in [('witness_classes.json','witness_classes.rs'),('witness_extended.json','witness_extended.rs')]:
        for claim in manifest(filename):
            name,index = claim.get('representative',claim['members'][0])
            z = streams(bytes.fromhex(vectors[name]['input_hex']))[index]
            call = expected_call([z],len(claim['raw']),True,False,'Done',claim['raw'],'check_at')
            witness_calls.validate(source(src),claim['class'],[call])
    for filename,src in [('witness_special.json','witness_special.rs'),('witness_finish.json','witness_finish.rs')]:
        for claim in manifest(filename):
            domain = claim.get('domain','core')
            name,index = claim['members'][0]
            v = vectors if domain == 'core' else policy_vectors
            r,parts,_ = stream_shapes(bytes.fromhex(v[name]['input_hex']))[index]
            assert claim['r'] == r and r is not None, name
            opts = [(L,e,t) for L in (r,r+1) for e in (True,False) for t in (False,True)] if domain == 'policy' else [(r,True,False)]
            expected = [expected_call(parts,L,e,t,claim['status'],claim['raw']) for L,e,t in opts]
            witness_calls.validate(source(src),claim['class'],expected)
    for claim in manifest('witness_policy.json'):
        name = claim['records'][0]
        r,parts,_ = stream_shapes(bytes.fromhex(policy_vectors[name]['input_hex']))[0]
        assert r == claim['r'], name
        expected = [expected_call(parts,c['limit'],c['enforce'],c['ignore_tail'],c['status'],claim['raw']) for c in claim['calls']]
        assert sorted((c['limit'],c['enforce'],c['ignore_tail']) for c in claim['calls']) == sorted((L,e,t) for L in (r,r+1) for e in (True,False) for t in (False,True)), name
        witness_calls.validate(source('witness_policy_cases.rs'),claim['function'],expected)
    for claim in manifest('witness_layouts.json'):
        r,parts,_ = stream_shapes(bytes.fromhex(vectors[claim['record']]['input_hex']))[0]
        assert r == claim['r'] and [len(p) for p in parts] == claim['segment_lengths'], claim['record']
        cuts = [0]
        for part in parts: cuts.append(cuts[-1]+len(part))
        assert cuts == claim['cuts'], claim['record']
        call = expected_call(parts,r,True,False,'Done',claim['raw'],'replay_layout')
        witness_calls.validate(source('witness_extended.rs'),claim['class'],[call])
    print('Parsed public-call descriptors bind all credited witnesses to exact input, limit, options, status and raw')
    return individual_keys


def validate_evidence(root=ROOT):
    records = json.loads((root/'evidence/manifest.json').read_text())
    paths = set()
    for record in records:
        path = record['path']
        assert path.startswith('evidence/') and '..' not in Path(path).parts and path not in paths, path
        paths.add(path)
        data = (root/path).read_bytes()
        assert hashlib.sha256(data).hexdigest() == record['sha256'], path
        if 'verification_results' in record:
            assert record['verification_results'] in data.decode(), path
        if record.get('role') == 'current-final-source-count':
            sources = record['source_sha256']
            actual = {str(p.relative_to(root)) for p in (root/'src').rglob('*.rs')} | {'Cargo.toml','Cargo.lock','verify.sh'}
            assert set(sources) == actual, 'final-count source inventory changed'
            for name,digest in sources.items():
                assert '..' not in Path(name).parts
                assert hashlib.sha256((root/name).read_bytes()).hexdigest() == digest, ('final-count source changed',name)
    for item in json.loads((root/'evidence/historical-blocked.json').read_text()) + json.loads((root/'tools/witness_blocked.json').read_text()):
        assert item['diagnostics'] in paths, item
        log = (root/item['diagnostics']).read_text()
        assert f'fn {item["class"]}()' in log and 'resource limit (rlimit) exceeded' in log, item
        assert (root/item['source_snapshot']).stat().st_size < 1_000_000, item
    assert {'evidence/run-0507.txt','evidence/run-0508.txt','evidence/run-0510.txt'} <= paths
    print(f'{len(records)} retained proof logs hash-checked without reading ignored runs/')


def main():
    vectors = {v['name']: v for v in json.loads((ROOT.parent / 'spec-frozen/VECTORS.json').read_text())}
    policy_vectors = {v['name']: v for v in json.loads((ROOT.parent/'policy-draft/VECTORS-POLICY.json').read_text())}
    validate_evidence()
    individual_keys = validate_bindings(vectors, policy_vectors)
    for name in ('author_more_witnesses.py', 'author_dynamic_witnesses.py', 'author_d4_witnesses.py', 'author_window_witnesses.py', 'author_invalid_dynamic_witnesses.py', 'author_repeat_witnesses.py', 'author_literal_classes.py', 'author_extended_classes.py', 'author_special_witnesses.py', 'author_policy_witnesses.py', 'author_single_literal_witness.py', 'author_finish_witnesses.py'):
        tree = ast.parse((ROOT / 'tools' / name).read_text())
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                allowed = {'json', 'pathlib', 'random'} if name in ('author_literal_classes.py', 'author_extended_classes.py') else {'json', 'pathlib'}
                assert all(alias.name in allowed for alias in node.names), name
            if isinstance(node, ast.ImportFrom):
                assert node.module == 'pathlib', name
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Name):
                assert node.func.id not in {'eval', 'exec', '__import__', 'compile'}, name
            if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Constant):
                assert node.slice.value not in {'expected', 'expected_error', 'rgba', 'rgba_runs'}, name
    text = '\n'.join((ROOT / 'src' / name).read_text() for name in ('witnesses.rs', 'witness_cases.rs', 'witness_dynamic_cases.rs', 'witness_d4.rs', 'witness_overlap.rs', 'witness_invalid_dynamic.rs', 'witness_repeats.rs', 'witness_single_literal.rs'))
    pattern = re.compile(r'/// ([\w/-]+):[^\n]*\n(?:#\[[^\n]*\]\s*)?fn \w+\(\) \{\s*(?:hide\([^;]*\);\s*)*let wire(?:\s*:\s*\[u8;\s*0\])?\s*=\s*\[([^\]]*)\]', re.M)
    seen = set()
    for name, elements in pattern.findall(text):
        wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in elements.split(',') if x.strip())
        assert name not in seen, name
        assert wire == idat(bytes.fromhex(vectors[name]['input_hex'])), name
        seen.add(name)
    # The large FCHECK fixture uses a separately verified literal constructor;
    # its caller needs only the authored header because rejection precedes BTYPE.
    ctor = re.search(r'fn fcheck_bytes\(\) -> \(wire: \[u8;\d+\]\)\s*ensures[^\{]*\{\s*\[([^\]]*)\]\s*\}', text)
    assert ctor and 'let wire = fcheck_bytes();' in text
    header_wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[1].split(',') if x.strip())
    assert header_wire == idat(bytes.fromhex(vectors['zlib/fcheck']['input_hex']))
    seen.add('zlib/fcheck')
    # This is the authored subset, not a claim of coverage of all 631 streams.
    assert len(seen) == 48, (len(seen), sorted(seen))
    assert individual_keys == {(name,0) for name in seen}
    assert 'zlib/empty-idat-only' in seen
    print(f'{len(seen)} authored witness streams match frozen IDAT extraction, including Z=[]; no oracle used')
    covered = {(name, 0) for name in seen}
    classes = json.loads((ROOT / 'tools/witness_classes.json').read_text())
    class_text = (ROOT / 'src/witness_classes.rs').read_text()
    for claim in classes:
        ident, raw = claim['class'], claim['raw']
        body = re.search(r'fn ' + re.escape(ident) + r'\(\) \{(.*?)(?=\n\})', class_text, re.S)
        assert body, ident
        literal = re.search(r'let wire = \[([^\]]*)\]', body[1])
        assert literal, ident
        wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in literal[1].split(',') if x.strip())
        authored_raw = re.search(r'let ghost raw = seq!\[([^\]]*)\]', body[1])
        assert authored_raw, ident
        assert list(int(x.strip().removesuffix('u8'), 0) for x in authored_raw[1].split(',') if x.strip()) == raw, ident
        assert f'let mut output = [0xa5u8;{len(raw)}]' in body[1], ident
        assert 'check_at(&wire,&mut output,Status::Done,Ghost(raw))' in body[1], ident
        assert claim['members'], ident
        for name, index in claim['members']:
            key = (name, index)
            assert key not in covered, key
            data = bytes.fromhex(vectors[name]['input_hex'])
            assert index == 0 and streams(data)[index] == wire, key
            assert image_r(data) == len(raw), key
            covered.add(key)
    extended = json.loads((ROOT / 'tools/witness_extended.json').read_text())
    extended_text = (ROOT / 'src/witness_extended.rs').read_text()
    extended_fdats = 0
    for claim in extended:
        ident, raw = claim['class'], claim['raw']
        if claim.get('decomposed'):
            ctor = re.search(r'fn ' + re.escape(ident) + r'_bytes\(\) -> \(wire: \[u8;(\d+)\]\)\s*ensures wire@ == ' + re.escape(ident) + r'_z\(\),\s*\{\s*\[([^\]]*)\]\s*\}', extended_text)
            assert ctor, ident
            wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[2].split(',') if x.strip())
            zspec = re.search(r'spec fn ' + re.escape(ident) + r'_z\(\) -> Seq<u8> \{ seq!\[([^\]]*)\] \}', extended_text)
            rspec = re.search(r'spec fn ' + re.escape(ident) + r'_raw\(\) -> Seq<u8> \{ seq!\[([^\]]*)\] \}', extended_text)
            assert zspec and rspec, ident
            assert wire == bytes(int(x.strip().removesuffix('u8'), 0) for x in zspec[1].split(',') if x.strip()), ident
            assert raw == [int(x.strip().removesuffix('u8'), 0) for x in rspec[1].split(',') if x.strip()], ident
        else:
            ctor = re.search(r'fn ' + re.escape(ident) + r'_bytes\(\) -> \(wire: \[u8;(\d+)\]\)\s*ensures wire@ == seq!\[([^\]]*)\],\s*\{\s*\[([^\]]*)\]\s*\}', extended_text)
            assert ctor, ident
            wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[3].split(',') if x.strip())
            assert wire == bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[2].split(',') if x.strip()), ident
        assert len(wire) == int(ctor[1]), ident
        body = re.search(r'fn ' + re.escape(ident) + r'\(\) \{(.*?)(?=\n\})', extended_text, re.S)
        assert body and f'let wire = {ident}_bytes();' in body[1], ident
        if claim.get('decomposed'):
            assert f'let ghost raw = {ident}_raw();' in body[1], ident
            fixed = claim.get('decomposed') == 'fixed'
            for helper in (('trace', 'fields', 'checksum') if fixed else ('payload', 'fields', 'checksum')):
                assert f'{ident}_{helper}();' in body[1], ident
            bridge = f'fixed_stream({ident}_z(),{ident}_raw(),{ident}_ts(),{len(raw)},' if fixed else f'stored_stream({ident}_z(),{ident}_raw(),{len(raw)},'
            assert 'crate::witness_stored_bridge::' + bridge in body[1], ident
        else:
            for prefix in ('let raw = ', 'let ghost raw = '):
                authored = re.search(re.escape(prefix) + r'seq!\[([^\]]*)\]', body[1])
                assert authored, ident
                assert [int(x.strip().removesuffix('u8'), 0) for x in authored[1].split(',') if x.strip()] == raw, ident
        assert f'let mut output = [0xa5u8;{len(raw)}]' in body[1], ident
        assert 'check_at(&wire,&mut output,Status::Done,Ghost(raw))' in body[1], ident
        trace_text = body[1]
        if claim.get('decomposed') == 'fixed':
            spec = re.search(r'spec fn ' + re.escape(ident) + r'_ts\(\) -> Seq<t::Step> \{ (seq!\[.*?\]) \}', extended_text)
            helper = re.search(r'proof fn ' + re.escape(ident) + r'_trace\(\).*?\n\{(.*?)(?=\n\})', extended_text, re.S)
            assert spec and helper, ident
            trace_text = spec[1] + helper[1]
        steps = [dict((k, int(v)) for k, v in re.findall(r'(\w+):(\d+)', step))
                 for step in re.findall(r't::Step \{([^}]*)\}', trace_text)]
        # The trace is authored twice: its binding/spec and concrete compute check.
        assert steps == claim['steps'] * 2, ident
        assert claim['members'] and claim['representative'] in claim['members'], ident
        for name, index in claim['members']:
            key = (name, index)
            assert key not in covered, key
            data = bytes.fromhex(vectors[name]['input_hex'])
            r, parts, control = stream_shapes(data)[index]
            assert b''.join(parts) == wire and r == len(raw), key
            extended_fdats += control is not None
            covered.add(key)
    classes += extended
    print(f'{len(extended)} extended constructors/traces audited, including {extended_fdats} fdAT streams at their own frame R')
    special = json.loads((ROOT / 'tools/witness_special.json').read_text())
    special_text = (ROOT / 'src/witness_special.rs').read_text()
    policy_vectors = {v['name']:v for v in json.loads((ROOT.parent / 'policy-draft/VECTORS-POLICY.json').read_text())}
    for claim in special:
        ident = claim['class']
        body = re.search(r'fn ' + re.escape(ident) + r'\(\) \{(.*?)(?=\n\})', special_text, re.S)
        assert body, ident
        parts = [bytes(int(x.strip().removesuffix('u8'),0) for x in literal.split(',') if x.strip())
                 for literal in re.findall(r'let segment_\d+: \[u8;\d+\] = \[([^\]]*)\]', body[1])]
        assert [len(p) for p in parts] == claim['segment_lengths'], ident
        calls = 8 if claim['domain'] == 'policy' else 1
        assert body[1].count('check_policy_parts(') == calls, ident
        assert f'let mut output = [0xa5u8;{claim["r"]}]' in body[1], ident
        if claim['raw']:
            raw = re.search(r'let ghost raw = seq!\[([^\]]*)\]', body[1])
            assert raw and [int(x.strip().removesuffix('u8'),0) for x in raw[1].split(',') if x.strip()] == claim['raw'], ident
        else:
            assert 'let ghost raw = Seq::<u8>::empty();' in body[1], ident
        for name, index in claim['members']:
            data = bytes.fromhex((vectors if claim['domain'] == 'core' else policy_vectors)[name]['input_hex'])
            r, original, control = stream_shapes(data)[index]
            assert r == claim['r'] and parts == original, (name,index)
            if claim['domain'] == 'core':
                assert (name,index) not in covered, (name,index)
                covered.add((name,index))
    print('Three small error families match original payload boundaries: 3 core streams and 2 window-policy records')
    finishing = json.loads((ROOT / 'tools/witness_finish.json').read_text())
    finish_text = (ROOT / 'src/witness_finish.rs').read_text()
    finish_fdats = 0
    for claim in finishing:
        ident, raw = claim['class'], claim['raw']
        body = re.search(r'fn ' + re.escape(ident) + r'\(\) \{(.*?)(?=\n\})', finish_text, re.S)
        assert body, ident
        parts = [bytes(int(x.strip().removesuffix('u8'), 0) for x in literal.split(',') if x.strip())
                 for literal in re.findall(r'let segment_\d+: \[u8;\d+\] = \[([^\]]*)\]', body[1])]
        assert [len(p) for p in parts] == claim['segment_lengths'], ident
        for prefix in ('let raw = ', 'let ghost raw = '):
            authored = re.search(re.escape(prefix) + r'seq!\[([^\]]*)\]', body[1])
            assert authored, ident
            assert raw == [int(x.strip().removesuffix('u8'), 0) for x in authored[1].split(',') if x.strip()], ident
        steps = [dict((k, int(v)) for k,v in re.findall(r'(\w+):(\d+)', step))
                 for step in re.findall(r't::Step \{([^}]*)\}', body[1])]
        assert steps == claim['steps'] * 2, ident
        assert body[1].count('check_policy_parts(') == 1, ident
        assert f'let mut output = [0xa5u8;{claim["r"]}]' in body[1], ident
        assert f't::effects_at(z,raw,ts,0,{len(claim["steps"])},{claim["r"]})' in body[1], ident
        for name,index in claim['members']:
            key = (name,index)
            assert key not in covered, key
            r,original,control = stream_shapes(bytes.fromhex(vectors[name]['input_hex']))[index]
            assert r == claim['r'] and original == parts, key
            finish_fdats += control is not None
            covered.add(key)
    print(f'{len(finishing)} finishing classes audited with original parts and R, including {finish_fdats} fdAT instances')
    blocked = json.loads((ROOT / 'tools/witness_blocked.json').read_text())
    blocked_keys = {}
    for item in blocked:
        log = (ROOT / item['diagnostics']).read_text()
        assert f'fn {item["class"]}()' in log and 'resource limit (rlimit) exceeded' in log, item
        assert (ROOT / item['source_snapshot']).stat().st_size < 1_000_000, item
        for name,index in item['members']:
            assert (name,index) not in covered, (name,index)
            blocked_keys[name,index] = item
    all_streams = {(v['name'],i): z for v in vectors.values() for i,z in enumerate(streams(bytes.fromhex(v['input_hex'])))}
    assert len(all_streams) == 631, len(all_streams)
    assert covered <= all_streams.keys()
    inventory = []
    for (name,index), z in all_streams.items():
        key = (name,index)
        reason = 'proved authored model/public-entry witness' if key in covered else (
            'awaits authored trace; non-IDAT stream' if index else
            'awaits authored trace; empty/short envelope' if len(z) < 3 else
            ['awaits authored stored-block trace', 'awaits authored fixed-match/literal trace',
             'awaits authored dynamic profile/symbol trace', 'awaits authored invalid-block trace'][(z[2] >> 1) & 3])
        if key not in covered:
            data = bytes.fromhex(vectors[name]['input_hex'])
            if stream_shapes(data)[index][0] is None:
                reason = 'no legal independent image/frame R; malformed or ambiguous IHDR/fcTL'
        if key in blocked_keys:
            item = blocked_keys[key]
            reason = f'blocked at default solver rlimit; run {item["run"]}; {item["class"]}'
        inventory.append({'name':name, 'stream':index, 'bytes':len(z), 'covered':key in covered, 'reason':reason})
    (ROOT / 'tools/witness_inventory.json').write_text(json.dumps(inventory,indent=2)+'\n')
    class_kinds = {}
    for claim in classes:
        kind = claim['encoding']
        class_kinds[kind] = class_kinds.get(kind, 0) + 1
    print(f'{len(classes)} independent stream classes {class_kinds} cover {sum(len(c["members"]) for c in classes)} additional streams')
    print(f'{len(covered)} / 631 frozen core streams covered; {631-len(covered)} remain; every stream inventoried')
    policy_vectors = {v['name']:v for v in json.loads((ROOT.parent / 'policy-draft/VECTORS-POLICY.json').read_text())}
    policy_text = (ROOT / 'src/witness_policy_cases.rs').read_text()
    policy_claims = json.loads((ROOT / 'tools/witness_policy.json').read_text())
    policy_seen = set()
    for claim in policy_claims:
        body = re.search(r'fn ' + re.escape(claim['function']) + r'\(\) \{(.*?)(?=\n\})', policy_text, re.S)
        assert body, claim
        parts = [bytes(int(x.strip().removesuffix('u8'),0) for x in literal.split(',') if x.strip())
                 for literal in re.findall(r'let segment_\d+ = \[([^\]]*)\]', body[1])]
        assert [len(p) for p in parts] == claim['segment_lengths'], claim
        assert body[1].count('check_policy_parts(') == 8, claim
        assert 'let ghost raw = seq![0u8,65,0,66]' in body[1], claim
        for name in claim['records']:
            assert name not in policy_seen, name
            data = bytes.fromhex(policy_vectors[name]['input_hex'])
            assert parts == [p for k,p in chunks(data) if k == b'IDAT'], name
            assert image_r(data) == 4, name
            policy_seen.add(name)
    assert len(policy_seen) == 8, policy_seen
    print('8 policy records match original IDAT segments; four window/tail combinations at R and R+1; not added to the core-stream numerator')


if __name__ == '__main__':
    main()
