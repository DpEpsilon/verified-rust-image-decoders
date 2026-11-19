#!/usr/bin/env python3
"""Check authored witness input provenance without consulting an inflate oracle."""
import ast
import hashlib
import sys
import json
from pathlib import Path
import re
import tarfile

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
    lib = source_overrides.get('lib.rs',(ROOT/'src/lib.rs').read_text())
    # A split-checkpoint gate (tools/split_checkpoint.py) only narrows which
    # pass compiles the module; without features every gated module is active.
    active = set(re.findall(r'#\[cfg\(verus_only\)\]\s*(?:#\[cfg\(any\(not\(feature = "split"\), feature = "split-[a-z0-9-]+"\)\)\]\s*)?(?:pub\s+)?mod\s+(\w+)\s*;',lib))
    def source(name):
        assert Path(name).stem in active, ('witness module is not active',name)
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
    for filename,src in [('witness_config.json','witness_config_cases.rs'),('witness_profiles.json','witness_profiles.rs'),('witness_blocks.json','witness_blocks.rs')]:
        for claim in manifest(filename):
            domain = vectors if claim['domain'] == 'core' else policy_vectors
            name,index = claim['members'][0]
            r,parts,_ = stream_shapes(bytes.fromhex(domain[name]['input_hex']))[index]
            assert r == claim['r'] and [len(p) for p in parts] == claim['segment_lengths'], name
            tuples = [(c['limit'],c['enforce'],c['ignore_tail']) for c in claim['calls']]
            options = [(L,e,t) for L in (r,r+1) for e in (True,False) for t in (False,True)] if claim['domain'] == 'policy' else [(r,True,False)]
            assert sorted(tuples) == sorted(options), name
            for call in claim['calls']:
                if 'window' in call:
                    cmf = b''.join(parts)[0]
                    assert call['window'] == (2**(cmf//16+8) if call['enforce'] else 32768), name
            expected = [expected_call(parts,c['limit'],c['enforce'],c['ignore_tail'],c['status'],c['raw']) for c in claim['calls']]
            witness_calls.validate(source(src),claim['class'],expected)
            for member,index in claim['members']:
                mr,mp,_ = stream_shapes(bytes.fromhex(domain[member]['input_hex']))[index]
                assert mr == r and mp == parts, member
    print('Parsed public-call descriptors bind all credited witnesses to exact input, limit, options, status and raw')
    return individual_keys


def validate_profile_transcriptions(claims, text, vectors):
    keys=set()
    for claim in claims:
        ident=claim['class']
        assert witness_calls.sequence(witness_calls.spec_sequence(text,ident+'_raw'),{},text) == claim['raw'], ident
        for kind in ('lit','dist'):
            values=claim[kind]
            expr=f'Seq::new({len(values)},|i:int| '+''.join(f'if i == {i} {{{n}u8}} else ' for i,n in enumerate(values) if n)+'{0u8})'
            assert witness_calls.spec_sequence(text,ident+'_'+kind) == witness_calls.tokens(expr), (ident,kind)
        assert witness_calls.sequence(witness_calls.spec_sequence(text,ident+'_cl'),{},text) == claim['cl'], ident
        def seq(xs,ty='u8'):
            return 'seq!['+', '.join(str(x)+(ty if i==0 else '') for i,x in enumerate(xs))+']'
        repeat=f'd::repeat_trace({seq(witness_calls.sequence(witness_calls.spec_sequence(text,ident+"_z"),{},text))},{seq(claim["cl"])},{seq(claim["repeat_symbols"])},{seq(claim["repeat_positions"],"nat")},{seq(claim["repeat_offsets"],"nat")},{seq(claim["lit"]+claim["dist"])},0,{len(claim["repeat_symbols"])})'
        compact=' '.join(witness_calls.tokens(text))
        assert ' '.join(witness_calls.tokens(repeat)) in compact, ident
        for step in claim['steps']:
            assert ' '.join(witness_calls.tokens(f'follow_word(lit,z,{step["pos"]},{step["symbol"]},0)')) in compact, ident
            out='raw' if step['symbol']==256 else f'raw.take({step["after"]})'
            variant='End' if step['symbol']==256 else 'Next'
            expr=f'm::MCompressedStep(z,{step["pos"]},raw.take({step["before"]}),{claim["r"]},32768,lit,dist) == (m::SymbolEffect::{variant} {{out:{out},position:{step["next"]}}})'
            assert ' '.join(witness_calls.tokens(expr)) in compact, (ident,step)
            if step['symbol']>256:
                expr=f't::copy_bytes({seq(claim["raw"])},{step["before"]},{step["distance"]},0,{step["length"]})'
                assert ' '.join(witness_calls.tokens(expr)) in compact, (ident,step)
                lp=step['pos']+claim['lit'][step['symbol']]
                ln=0 if step['symbol']<265 or step['symbol']>=285 else (step['symbol']-261)//4
                dp=lp+ln
                assert ' '.join(witness_calls.tokens(f'follow_word(dist,z,{dp},{step["ds"]},0)')) in compact, (ident,step)

        for name,index in claim['members']:
            r,parts,_=stream_shapes(bytes.fromhex(vectors[name]['input_hex']))[index]
            assert r == claim['r'] and [len(p) for p in parts] == claim['segment_lengths'], name
            assert bytes(witness_calls.sequence(witness_calls.spec_sequence(text,ident+'_z'),{},text)) == b''.join(parts), name
            assert (name,index) not in keys, name
            keys.add((name,index))
    return keys


def validate_block_transcriptions(claims,text,vectors):
    keys=set()
    for claim in claims:
        ident=claim['class']
        for suffix,key in [('raw','raw'),('source','model_raw')]:
            assert witness_calls.sequence(witness_calls.spec_sequence(text,ident+'_'+suffix),{},text) == claim[key], (ident,key)
        spec=re.search(r'spec fn '+re.escape(ident)+r'_ts\(\) -> Seq<t::Step> \{ (seq!\[.*?\]) \}',text)
        helper=re.search(r'proof fn '+re.escape(ident)+r'_trace\(\).*?\n\{(.*?)(?=\n\})',text,re.S)
        assert spec and helper, ident
        authored=[dict((k,int(v)) for k,v in re.findall(r'(\w+):(\d+)',step)) for step in re.findall(r't::Step \{([^}]*)\}',spec[1]+helper[1])]
        assert authored == claim['steps'], ident
        chunks=claim['trace_chunks']
        assert chunks[0][0] == 0 and chunks[-1][1] == claim['trace_prefix'], ident
        assert all(a[1] == b[0] for a,b in zip(chunks,chunks[1:])), ident
        for j,(lo,hi) in enumerate(chunks):
            chunk=re.search(r'proof fn '+re.escape(ident)+r'_trace_chunk_'+str(j)+r'\(\).*?\n\{(.*?)(?=\n\})',text,re.S)
            assert chunk and f'assert(t::trace({ident}_z(),{ident}_source(),{ident}_ts(),{lo},{hi})) by (compute);' in chunk[1], ident
            assert f'{ident}_trace_chunk_{j}();' in helper[1], ident
        for name,index in claim['members']:
            r,parts,_=stream_shapes(bytes.fromhex(vectors[name]['input_hex']))[index]
            assert r == claim['r'] and [len(p) for p in parts] == claim['segment_lengths'], name
            assert bytes(witness_calls.sequence(witness_calls.spec_sequence(text,ident+'_z'),{},text)) == b''.join(parts), name
            assert (name,index) not in keys, name
            keys.add((name,index))
    return keys


def strip_split_gates(lib):
    """src/lib.rs as it reads without the split-checkpoint cfg gates."""
    return re.sub(r'^#\[cfg\(any\(not\(feature = "split"\), feature = "split-[a-z0-9-]+"\)\)\]\n', '', lib, flags=re.M)


def strip_split_features(cargo):
    """Cargo.toml as it reads without the split-checkpoint [features] table."""
    return re.sub(r'\n\[features\]\n(?:#[^\n]*\n|split[\w-]* = \[[^\n]*\]\n)*', '', cargo)


def equivalent_monolithic_sources(root, sources):
    """True when the reference run's sources equal the current tree, ignoring only
    verify.sh and the split gates/features, which a feature-free build does not see."""
    for name, digest in sources.items():
        data = (root/name).read_bytes() if (root/name).exists() else b''
        if name == 'src/lib.rs':
            data = strip_split_gates(data.decode()).encode()
        elif name == 'Cargo.toml':
            data = strip_split_features(data.decode()).encode()
        elif name == 'verify.sh':
            continue
        if hashlib.sha256(data).hexdigest() != digest:
            return False
    current = {str(p.relative_to(root)) for p in (root/'src').rglob('*.rs')}
    return current == {n for n in sources if n.startswith('src/')}


def validate_split_checkpoint(root, record, by_path):
    """A split checkpoint: every manifest pass, 0 errors, one source tree, one unique total."""
    import split_checkpoint as sc
    checkpoint = json.loads((root/record['path']).read_text())
    assert checkpoint['kind'] == 'split-checkpoint', record['path']
    assert checkpoint['source_sha256'] == record['source_sha256'], 'checkpoint and manifest source bindings differ'
    digest = sc.tree_hash(record['source_sha256'])
    assert checkpoint['source_tree_sha256'] == digest, 'checkpoint tree hash does not match its source inventory'
    manifest = sc.load_manifest(root)
    sc.check_coverage(manifest, root)
    names = [p['name'] for p in manifest['passes']]
    assert [q['pass'] for q in checkpoint['passes']] == names, ('checkpoint passes differ from the manifest', names)
    assert [q['run'] for q in checkpoint['passes']] == record['pass_runs'], 'pass runs differ'
    logs = {}
    for q in checkpoint['passes']:
        path = f'evidence/run-{q["run"]:04d}.txt'
        log_record = by_path.get(path)
        assert log_record and log_record.get('role') == 'split-pass' and log_record.get('checkpoint') == record['path'], ('pass log not retained', path)
        assert log_record['run'] == q['run'] and log_record['pass'] == q['pass'], path
        logs[q['pass']] = (root/path).read_text()
    counts, unique = sc.check_passes(manifest, logs, digest)
    for q in checkpoint['passes']:
        line = f'verification results:: {counts[q["pass"]]} verified, 0 errors'
        assert q['verified'] == counts[q['pass']] and q['verification_results'] == line, q['pass']
        assert by_path[f'evidence/run-{q["run"]:04d}.txt']['verification_results'] == line, q['pass']
    summed = sum(counts.values())
    assert checkpoint['verified_sum'] == record['verified_sum'] == summed, 'plain pass sum differs'
    assert checkpoint['verified_unique_total'] == record['verified_unique_total'] == unique, 'unique total differs'
    return unique


def validate_target_replay(root, record, final, by_path):
    """A split checkpoint replayed for another target (I686-CACHE.md): the same
    source tree and unique total as the current final count, every pass compiled
    for record['target'] against the recorded vstd cache, and vstd not rebuilt."""
    assert record.get('kind') == 'split-checkpoint' and final.get('kind') == 'split-checkpoint', record['path']
    assert record['source_sha256'] == final['source_sha256'], 'target replay ran on a different source tree'
    unique = validate_split_checkpoint(root, record, by_path)
    assert unique == final['verified_unique_total'], ('target replay unique total differs from the final count', unique, final['verified_unique_total'])
    target, vstd = record['target'], record['vstd_rmeta']
    assert re.fullmatch(r'[a-z0-9_]+(-[a-z0-9_]+)+', target) and target != 'x86_64-unknown-linux-gnu', target
    assert re.fullmatch(rf'target/[\w.-]+/{re.escape(target)}/debug/deps/libvstd-[0-9a-f]{{16}}\.rmeta', vstd), vstd
    for run in record['pass_runs']:
        text = (root/f'evidence/run-{run:04d}.txt').read_text()
        crate = [l for l in text.splitlines() if l.lstrip().startswith('Running `') and '--crate-name inflate_verus ' in l]
        assert len(crate) == 1, ('target replay pass needs one verbose inflate_verus rustc line', run, len(crate))
        assert f' --target {target} ' in crate[0], ('target replay pass not compiled for the target', run)
        assert re.search(rf' --extern vstd=/\S+/inflate_verus/{re.escape(vstd)} ', crate[0]), ('target replay pass used another vstd', run)
        assert not re.search(r'^\s*Compiling vstd ', text, re.M), ('target replay pass rebuilt vstd', run)
        assert re.search(r'^\s*Fresh vstd ', text, re.M), ('target replay pass does not show the cached vstd', run)
    return unique


# Frozen THEOREMS mutation numbers, not a count of all useful experiments.
M2_MUTATION_SELECTORS = {
    1: {'btype-swap-fixed-dynamic'}, 2: {'huffman-lsb-first'},
    3: {'huffman-accept-incomplete'}, 4: {'omit-nlen'}, 5: {'wrong-length-285'},
    6: {'reserved-distance-admitted', 'fixed-only-thirty-distances'},
    7: {'distance-output-plus-one'}, 8: {'omit-window-guard'},
    9: {'late-adler-deferral'}, 10: {'omit-adler-comparison'},
    11: {'full-at-limit', 'output-omit-limit-guard'}, 12: {'bit-position-u32-narrowing'},
    37: {'huffman-skip-dead-prefix'}, 39: {'stored-truncation-drop-prefix'},
}


def validate_mutation_index(root, by_path, index=None):
    """Check numbered coverage, real failed proofs, retained revisions and scope.

    Historical snapshots identify the affected implementation revision, not a
    retroactive source-hash binding of the whole historical verifier process.
    """
    if index is None:
        index = json.loads((root/'evidence/mutation-index.json').read_text())
    assert index['schema'] == 1
    selector_source = (root/'tools/proof_mutations.py').read_bytes()
    assert hashlib.sha256(selector_source).hexdigest() == index['selector_definitions_sha256'], 'mutation selector definitions changed'
    definitions = {}
    for node in ast.parse(selector_source).body:
        if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            if node.targets[0].id in ('CASES', 'FILE_CASES'):
                definitions[node.targets[0].id] = ast.literal_eval(node.value)
    selectors = {key: ('inflate.rs', *value) for key, value in definitions['CASES'].items()}
    selectors.update(definitions['FILE_CASES'])
    requirements = index['requirements']
    assert [r['number'] for r in requirements] == list(range(1, 41)), 'mutation requirements must be exactly 1–40'
    runs = set()
    for requirement in requirements:
        number = requirement['number']
        wanted = M2_MUTATION_SELECTORS.get(number, set())
        assert requirement['milestone'] == ('M2' if wanted else 'not M2'), ('mutation milestone differs', number)
        assert requirement['status'] == ('verifier-killed' if wanted else 'not credited here; later PNG/allocation/policy milestone'), ('mutation status differs', number)
        assert {m['selector'] for m in requirement['mutants']} == wanted, ('required mutation selectors differ', number)
        if number == 12:
            assert 'Ghost representation narrowing' in requirement.get('qualification', ''), 'mutation 12 needs its ghost qualification'
        for mutant in requirement['mutants']:
            file, before, after = selectors[mutant['selector']]
            assert mutant['mutation'] == {'file':'src/'+file, 'before':before, 'after':after}, 'mutation replacement differs from its selector'
            revision = mutant['source_revision']
            assert re.fullmatch(r'[0-9a-f]{40}', revision['evidence_commit']), 'missing mutation evidence revision'
            archive = revision['implementation_snapshot']
            record = by_path.get(mutant['log'], {})
            assert archive in record.get('source_archives_sha256', {}), 'mutation implementation snapshot is not retained'
            with tarfile.open(root/archive) as snapshot:
                implementation = snapshot.extractfile(mutant['mutation']['file']).read()
                if 'source_tree_sha256' in revision:
                    import split_checkpoint as sc
                    names = {p.name for p in snapshot.getmembers() if p.name.startswith('src/') and p.name.endswith('.rs')} | set(sc.TOOL_FILES)
                    inventory = {name:hashlib.sha256(snapshot.extractfile(name).read()).hexdigest() for name in names}
                    assert sc.tree_hash(inventory) == revision['source_tree_sha256'], 'mutation archive does not reproduce its split source hash'
            assert hashlib.sha256(implementation).hexdigest() == revision['implementation_sha256'], 'mutation implementation revision differs'
            image = mutant['mutation']['after' if number == 39 else 'before']
            assert implementation.decode().count(image) == 1, 'mutation replacement does not match its implementation revision'
            assert mutant['failed_function'], 'mutation failed function missing'
            if number == 12:
                assert 'Ghost representation narrowing' in mutant.get('qualification', ''), 'mutation 12 needs its ghost qualification'
    all_mutants = [m for r in requirements for m in r['mutants']] + index['additional_mutations']
    for mutant in all_mutants:
        run = mutant['run']
        assert run not in runs, ('mutation run counted twice', run)
        runs.add(run)
        record = by_path.get(mutant['log'])
        assert record and record['run'] == run and record['mutation'] == mutant['selector'], 'mutation log is not retained with its selector'
        text = (root/mutant['log']).read_text()
        assert mutant['failed_proof'] in text and mutant['failed_proof'].startswith('error: '), 'mutation diagnostic missing from log'
        assert re.search(r'^error: (postcondition not satisfied|precondition not satisfied|assertion failed|invariant not satisfied|possible arithmetic underflow/overflow)', mutant['failed_proof']), 'mutation needs a real failed proof'
        summary = mutant['verification_results']
        assert summary == record['verification_results'] and re.fullmatch(r'verification results:: [1-9]\d* verified, [1-9]\d* errors', summary), 'mutation needs positive verified and failed-proof counts'
        if run == 613:
            import split_checkpoint as sc
            report = sc.json_report(text)['verification-results']
            assert report['verified'] > 0 and report['errors'] > 0 and not report['encountered-vir-error'], 'mutation needs a real failed JSON proof'
            assert 'VERIFY_SOURCE_TREE=' + mutant['source_revision']['source_tree_sha256'] in text
        else:
            assert summary in text, 'mutation count missing from log'
    assert index['historical_kill_total'] == len(runs), 'mutation kill total differs'
    assert all(e['run'] not in runs for e in index['excluded_attempts']), 'excluded attempt counted as a mutation kill'
    assert {e['run'] for e in index['excluded_attempts']} == {453}, 'retain the type-check-only exclusion'
    print(f'{len(M2_MUTATION_SELECTORS)} M2 mutation requirements checked; {len(runs)} historical kills (mutation 12 is ghost representation narrowing)')


def validate_evidence(root=ROOT):
    records = json.loads((root/'evidence/manifest.json').read_text())
    assert sum(record.get('role') == 'current-final-source-count' for record in records) == 1, 'one fresh source-bound final count is required'
    assert sum(record.get('role') == 'monolithic-reference-count' for record in records) <= 1, 'at most one monolithic reference'
    paths = set()
    by_path = {}
    for record in records:
        path = record['path']
        assert path.startswith('evidence/') and '..' not in Path(path).parts and path not in paths, path
        paths.add(path)
        by_path[path] = record
        data = (root/path).read_bytes()
        assert hashlib.sha256(data).hexdigest() == record['sha256'], path
        if 'verification_results' in record and record.get('role') != 'split-pass':
            if record.get('kind') == 'proof-mutation-json':
                import split_checkpoint as sc
                result = sc.json_report(data.decode())['verification-results']
                summary = f'verification results:: {result["verified"]} verified, {result["errors"]} errors'
                assert summary == record['verification_results'], path
            else:
                assert record['verification_results'] in data.decode(), path
        for name,digest in record.get('source_archives_sha256',{}).items():
            assert '..' not in Path(name).parts and name.startswith('backups/'), name
            archive = (root/name).read_bytes()
            assert len(archive) < 1_000_000 and hashlib.sha256(archive).hexdigest() == digest, name
    for record in records:
        if record.get('role') == 'current-final-source-count':
            sources = record['source_sha256']
            if record.get('kind') == 'split-checkpoint':
                import split_checkpoint as sc
                actual = set(sc.source_inventory(root)) if (root/'src').exists() else set()
            else:
                actual = {str(p.relative_to(root)) for p in (root/'src').rglob('*.rs')} | {'Cargo.toml','Cargo.lock','verify.sh'}
            assert set(sources) == actual, 'final-count source inventory changed'
            for name,digest in sources.items():
                assert '..' not in Path(name).parts
                assert hashlib.sha256((root/name).read_bytes()).hexdigest() == digest, ('final-count source changed',name)
            if record.get('kind') == 'split-checkpoint':
                unique = validate_split_checkpoint(root, record, by_path)
                reference = [r for r in records if r.get('role') == 'monolithic-reference-count']
                if reference:
                    # The split total is compared with the monolithic count, not
                    # hidden: on equivalent sources both must count the same functions.
                    ref = reference[0]
                    assert equivalent_monolithic_sources(root, ref['source_sha256']), 'monolithic reference is not source-equivalent; drop or replace it'
                    expected = int(re.fullmatch(r'verification results:: (\d+) verified, 0 errors', ref['verification_results'])[1])
                    assert unique == expected, ('split unique total differs from the monolithic reference', unique, expected)
    final = [r for r in records if r.get('role') == 'current-final-source-count']
    for record in records:
        if record.get('role') == 'target-replay-count':
            validate_target_replay(root, record, final[0], by_path)
    split_passes = [r for r in records if r.get('role') == 'split-pass']
    current = [r for r in records if r.get('role') in ('current-final-source-count', 'target-replay-count') and r.get('kind') == 'split-checkpoint']
    for r in split_passes:
        assert r['checkpoint'] in by_path, ('split pass of an unretained checkpoint', r['path'])
        for c in current:
            if r['checkpoint'] == c['path']:
                assert r['run'] in c['pass_runs'], ('stray pass log for the current checkpoint', r['path'])
    for item in json.loads((root/'evidence/historical-blocked.json').read_text()) + json.loads((root/'tools/witness_blocked.json').read_text()):
        assert item['diagnostics'] in paths, item
        log = (root/item['diagnostics']).read_text()
        assert any(f'fn {q}(' in log for q in item.get('queries',[item['class']])) and 'resource limit (rlimit) exceeded' in log, item
        assert (root/item['source_snapshot']).stat().st_size < 1_000_000, item
    assert {'evidence/run-0507.txt','evidence/run-0508.txt','evidence/run-0510.txt'} <= paths
    index_records = [r for r in records if r.get('role') == 'requirement-mutation-index']
    assert len(index_records) == 1 and index_records[0]['path'] == 'evidence/mutation-index.json', 'one requirement-indexed mutation table is required'
    validate_mutation_index(root, by_path)
    print(f'{len(records)} retained evidence records hash-checked without reading ignored runs/')


def main():
    vectors = {v['name']: v for v in json.loads((ROOT.parent / 'spec-frozen/VECTORS.json').read_text())}
    policy_vectors = {v['name']: v for v in json.loads((ROOT.parent/'policy-draft/VECTORS-POLICY.json').read_text())}
    validate_evidence()
    individual_keys = validate_bindings(vectors, policy_vectors)
    for name in ('author_more_witnesses.py', 'author_dynamic_witnesses.py', 'author_d4_witnesses.py', 'author_window_witnesses.py', 'author_invalid_dynamic_witnesses.py', 'author_repeat_witnesses.py', 'author_literal_classes.py', 'author_extended_classes.py', 'author_special_witnesses.py', 'author_policy_witnesses.py', 'author_single_literal_witness.py', 'author_finish_witnesses.py', 'author_config_witnesses.py', 'author_profile_witnesses.py', 'author_huffman_intent.py', 'author_block_witnesses.py'):
        tree = ast.parse((ROOT / 'tools' / name).read_text())
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                allowed = {'json', 'pathlib', 'sys', 're', 'author_extended_classes', 'author_huffman_intent'} if name in ('author_config_witnesses.py','author_profile_witnesses.py','author_huffman_intent.py','author_block_witnesses.py') else {'json', 'pathlib', 'random'} if name in ('author_literal_classes.py', 'author_extended_classes.py') else {'json', 'pathlib'}
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
    config = json.loads((ROOT/'tools/witness_config.json').read_text())
    config_text = (ROOT/'src/witness_config_cases.rs').read_text()
    config_policy = set()
    for claim in config:
        ident = claim['class']
        assert witness_calls.sequence(witness_calls.spec_sequence(config_text,ident+'_raw'),{},config_text) == claim['raw'], ident
        name,index = claim['members'][0]
        domain = vectors if claim['domain'] == 'core' else policy_vectors
        z = streams(bytes.fromhex(domain[name]['input_hex']))[index]
        assert bytes(witness_calls.sequence(witness_calls.spec_sequence(config_text,ident+'_z'),{},config_text)) == z, ident
        spec = re.search(r'spec fn '+re.escape(ident)+r'_ts\(\) -> Seq<t::Step> \{ (seq!\[.*?\]) \}',config_text)
        helper = re.search(r'proof fn '+re.escape(ident)+r'_trace\(\).*?\n\{(.*?)(?=\n\})',config_text,re.S)
        assert spec and helper, ident
        authored = [dict((k,int(v)) for k,v in re.findall(r'(\w+):(\d+)',step)) for step in re.findall(r't::Step \{([^}]*)\}',spec[1]+helper[1])]
        assert authored == claim['steps'], ident
        trace_ranges=claim['trace_chunks']
        assert trace_ranges[0][0] == 0 and trace_ranges[-1][1] == len(claim['steps']), ident
        assert all(a[1] == b[0] for a,b in zip(trace_ranges,trace_ranges[1:])), ident
        for j,(lo,hi) in enumerate(trace_ranges):
            chunk=re.search(r'proof fn '+re.escape(ident)+r'_trace_chunk_'+str(j)+r'\(\).*?\n\{(.*?)(?=\n\})',config_text,re.S)
            assert chunk and f'assert(t::trace({ident}_z(),{ident}_raw(),{ident}_ts(),{lo},{hi})) by (compute);' in chunk[1], ident
            assert f'{ident}_trace_chunk_{j}();' in helper[1], ident
        for name,index in claim['members']:
            key = (name,index)
            if claim['domain'] == 'core':
                assert key not in covered, key
                covered.add(key)
            else:
                assert key not in config_policy, key
                config_policy.add(key)
    print(f'{len(config)} fixed configuration classes audited; {len(config_policy)} policy records outside the core numerator')
    profiles = json.loads((ROOT/'tools/witness_profiles.json').read_text())
    profile_keys = validate_profile_transcriptions(profiles,(ROOT/'src/witness_profiles.rs').read_text(),vectors)
    assert not profile_keys & covered
    covered.update(profile_keys)
    print(f'{len(profiles)} complete dynamic profile/word/effect witnesses audited')
    blocks = json.loads((ROOT/'tools/witness_blocks.json').read_text())
    block_keys = validate_block_transcriptions(blocks,(ROOT/'src/witness_blocks.rs').read_text(),vectors)
    assert not block_keys & covered
    covered.update(block_keys)
    print(f'{len(blocks)} flush/EOF witnesses audited with original parts and independent prefixes')
    blocked = json.loads((ROOT / 'tools/witness_blocked.json').read_text())
    blocked_keys = {}
    for item in blocked:
        log = (ROOT / item['diagnostics']).read_text()
        assert any(f'fn {q}(' in log for q in item.get('queries',[item['class']])) and 'resource limit (rlimit) exceeded' in log, item
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
