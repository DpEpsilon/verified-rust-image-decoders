#!/usr/bin/env python3
"""Check authored witness input provenance without consulting an inflate oracle."""
import ast
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


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


def main():
    vectors = {v['name']: v for v in json.loads((ROOT.parent / 'spec-frozen/VECTORS.json').read_text())}
    for name in ('author_more_witnesses.py', 'author_dynamic_witnesses.py', 'author_d4_witnesses.py', 'author_window_witnesses.py', 'author_invalid_dynamic_witnesses.py', 'author_repeat_witnesses.py', 'author_literal_classes.py', 'author_extended_classes.py', 'author_special_witnesses.py', 'author_policy_witnesses.py', 'author_single_literal_witness.py'):
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
        ctor = re.search(r'fn ' + re.escape(ident) + r'_bytes\(\) -> \(wire: \[u8;(\d+)\]\)\s*ensures wire@ == seq!\[([^\]]*)\],\s*\{\s*\[([^\]]*)\]\s*\}', extended_text)
        assert ctor, ident
        wire = bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[3].split(',') if x.strip())
        assert len(wire) == int(ctor[1]), ident
        assert wire == bytes(int(x.strip().removesuffix('u8'), 0) for x in ctor[2].split(',') if x.strip()), ident
        body = re.search(r'fn ' + re.escape(ident) + r'\(\) \{(.*?)(?=\n\})', extended_text, re.S)
        assert body and f'let wire = {ident}_bytes();' in body[1], ident
        for prefix in ('let raw = ', 'let ghost raw = '):
            authored = re.search(re.escape(prefix) + r'seq!\[([^\]]*)\]', body[1])
            assert authored, ident
            assert [int(x.strip().removesuffix('u8'), 0) for x in authored[1].split(',') if x.strip()] == raw, ident
        assert f'let mut output = [0xa5u8;{len(raw)}]' in body[1], ident
        assert 'check_at(&wire,&mut output,Status::Done,Ghost(raw))' in body[1], ident
        steps = [dict((k, int(v)) for k, v in re.findall(r'(\w+):(\d+)', step))
                 for step in re.findall(r't::Step \{([^}]*)\}', body[1])]
        # The trace is authored twice in the query: its binding and compute check.
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
