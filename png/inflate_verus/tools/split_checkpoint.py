#!/usr/bin/env python3
"""Pass manifest, coverage check and pass-log checks for split Verus checkpoints.

A checkpoint is a sequence of serial Verus passes over one source tree. The
pinned Verus cannot select modules here: `cargo verus verify` refuses
`--verify-module`/`--verify-root` ("Partial verification must use `cargo verus
focus`", run 585), and rust_verify rejects both under `--no-cheating` (runs 586,
587). Passes therefore select modules at compile time: each leaf witness module
in src/lib.rs carries `#[cfg(any(not(feature = "split"), feature = "split-PASS"))]`.
Without features every module is compiled, exactly as in a monolithic run.
Each pass adds `--output-json`, whose report lists every function of the crate
and states `is-verifying-entire-crate`; that list shows which modules the pass
compiled and verified.

Pass `base` (feature `split`) compiles and verifies the modules that are never
gated: the crate root, the core decoder/model modules and the witness modules
that other witness modules import. Each other pass compiles base plus its own
leaf modules. Every module containing `verus!` code belongs to exactly one pass
in tools/verify_passes.json, and the manifest must agree with the gates.

Because every pass re-verifies base, the plain sum of pass counts counts base
once per pass. The well-defined total is the unique total
    base + sum over leaf passes of (pass count - base count),
which counts each verified function once.

Cargo replays a cached result, without running Verus, when a pass's features
and Rust sources are unchanged since an earlier successful pass (run 590). A
replay prints no JSON report, so its check fails closed: rerunning a checkpoint
needs a changed Rust source.

    python3 -I tools/split_checkpoint.py --check     # coverage only, no verifier
"""
import hashlib
import json
from pathlib import Path
import re
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = 'tools/verify_passes.json'
# Files whose content determines what a checkpoint verified; the tree hash in
# every pass command binds the pass log to them.
TOOL_FILES = ('Cargo.toml', 'Cargo.lock', 'verify.sh', MANIFEST,
              'tools/verify_split.py', 'tools/split_checkpoint.py', 'tools/run_logged.py')
VERUS_FLAGS = ['--num-threads', '2', '--no-cheating', '--output-json']
ROOT_MODULE = '<root>'
GATE = re.compile(r'#\[cfg\(any\(not\(feature = "split"\), feature = "(split-[a-z0-9-]+)"\)\)\]')
MOD = re.compile(r'^((?:#\[[^\n]*\]\n)*)(?:pub\s+)?mod\s+(\w+)\s*;', re.M)


def load_manifest(root=ROOT):
    return json.loads((root / MANIFEST).read_text())


def source_inventory(root=ROOT):
    names = {str(p.relative_to(root)) for p in (root / 'src').rglob('*.rs')} | set(TOOL_FILES)
    return {name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in sorted(names)}


def tree_hash(inventory):
    text = ''.join(f'{name}\0{digest}\n' for name, digest in sorted(inventory.items()))
    return hashlib.sha256(text.encode()).hexdigest()


def gates(root=ROOT):
    """Map each module with verus! code to its split feature, or None if ungated."""
    lib = (root / 'src/lib.rs').read_text()
    assert 2 * len(GATE.findall(lib)) == lib.count('feature = "split'), 'malformed split gate in src/lib.rs'
    declared = {}
    for attrs, name in MOD.findall(lib):
        found = GATE.findall(attrs)
        assert len(found) <= 1, ('module has two split gates', name)
        if found:
            assert '#[cfg(verus_only)]' in attrs, ('split gate outside verus_only', name)
        declared[name] = found[0] if found else None
    assert sum(len(GATE.findall(a)) for a, _ in MOD.findall(lib)) == len(GATE.findall(lib)), 'split gate not on a module declaration'
    result = {}
    for path in sorted((root / 'src').rglob('*.rs')):
        rel = path.relative_to(root / 'src')
        assert len(rel.parts) == 1, ('nested module files need a manifest rule', str(rel))
        text = path.read_text()
        if rel.name != 'lib.rs':
            assert 'feature = "split' not in text, ('split gate outside src/lib.rs', rel.name)
        if 'verus!' not in text:
            continue
        if rel.name == 'lib.rs':
            result[ROOT_MODULE] = None
        else:
            assert rel.stem in declared, ('verus module not declared in src/lib.rs', rel.stem)
            result[rel.stem] = declared[rel.stem]
    return result


def check_coverage(manifest, root=ROOT):
    """Every verus module lies in exactly one pass and its gate matches that pass."""
    passes = manifest['passes']
    assert passes and passes[0]['name'] == 'base' and passes[0]['feature'] == 'split', 'first pass must be base'
    owner = {}
    for p in passes:
        assert re.fullmatch(r'[a-z0-9-]+', p['name']), p['name']
        assert p is passes[0] or p['feature'] == 'split-' + p['name'], ('feature must be split-NAME', p['name'])
        assert p['modules'], ('empty pass', p['name'])
        for m in p['modules']:
            assert m not in owner, ('module in two passes', m, owner.get(m), p['name'])
            owner[m] = p['name']
    assert len({p['name'] for p in passes}) == len(passes), 'duplicate pass name'
    gate = gates(root)
    uncovered = sorted(gate.keys() - owner.keys())
    assert not uncovered, ('verus modules in no pass', uncovered)
    stray = sorted(owner.keys() - gate.keys())
    assert not stray, ('manifest names modules without verus code', stray)
    for p in passes:
        want = None if p is passes[0] else p['feature']
        wrong = sorted(m for m in p['modules'] if gate[m] != want)
        assert not wrong, ('src/lib.rs gate disagrees with the manifest', p['name'], wrong)
    features = tomllib.loads((root / 'Cargo.toml').read_text()).get('features', {})
    assert features == {'split': [], **{p['feature']: ['split'] for p in passes[1:]}}, 'Cargo.toml [features] disagree with the manifest'
    base = set(passes[0]['modules'])
    for p in passes:
        visible = base | set(p['modules'])
        for m in p['modules']:
            if m == ROOT_MODULE:
                continue
            refs = set(re.findall(r'\bcrate::(?:\{[^}]*\}|\w+)', (root / 'src' / f'{m}.rs').read_text()))
            names = {w for r in refs for w in re.findall(r'\b(witness\w*)\b', r)}
            hidden = sorted(n for n in names if n in owner and n not in visible)
            assert not hidden, ('module imports a leaf of another pass', m, hidden)
    return owner


def pass_command(p, digest):
    return ['env', f'VERIFY_SOURCE_TREE={digest}', f'VERIFY_PASS={p["name"]}',
            'cargo', 'verus', 'verify', '--features', p['feature'], '--', *VERUS_FLAGS]


def json_report(text):
    """The single `--output-json` object in a pass log."""
    found = []
    for m in re.finditer(r'^\{$', text, re.M):
        try:
            obj, _ = json.JSONDecoder().raw_decode(text, m.start())
        except ValueError:
            continue
        if isinstance(obj, dict) and 'verification-results' in obj:
            found.append(obj)
    assert len(found) == 1, ('need exactly one --output-json report (a cargo replay prints none)', len(found))
    return found[0]


def module_functions(manifest, report):
    """Map each manifest module to the crate functions the JSON report lists for it."""
    modules = {m for p in manifest['passes'] for m in p['modules']}
    result = {}
    for name in report['func-details']:
        parts = name.split('::')
        if parts[0] != 'inflate_verus':
            continue
        module = parts[1] if len(parts) > 2 and parts[1] in modules else ROOT_MODULE
        result.setdefault(module, set()).add(name)
    return result


def check_pass_log(manifest, p, text, digest):
    """Check one pass log; return its count and the functions it lists per base module."""
    first = text.split('\n', 1)[0]
    assert first == 'command: ' + repr(pass_command(p, digest)), ('pass command or source tree differs', p['name'], first[:300])
    report = json_report(text)
    vr = report['verification-results']
    assert vr['errors'] == 0 and vr['success'] is True, ('pass has errors', p['name'], vr['errors'])
    assert vr['encountered-error'] is False and vr['encountered-vir-error'] is False, ('pass reported an error', p['name'])
    assert vr['is-verifying-entire-crate'] is True, ('pass used a partial-verification selector', p['name'])
    verified = vr['verified']
    assert isinstance(verified, int) and verified > 0, ('pass verified nothing', p['name'])
    funcs = module_functions(manifest, report)
    base = manifest['passes'][0]['modules']
    missing = sorted(m for m in set(base) | set(p['modules']) if not funcs.get(m))
    assert not missing, ('modules absent from the verified crate', p['name'], missing)
    others = sorted(m for q in manifest['passes'][1:] if q is not p for m in q['modules'] if funcs.get(m))
    assert not others, ('modules of other passes were compiled', p['name'], others)
    return {'verified': verified, 'base_functions': {m: funcs[m] for m in base}}


def check_passes(manifest, logs, digest):
    """logs: pass-name -> log text for every manifest pass. Returns per-pass counts and the unique total."""
    names = [p['name'] for p in manifest['passes']]
    assert list(logs) == names, ('checkpoint passes differ from the manifest', list(logs), names)
    checked = {p['name']: check_pass_log(manifest, p, logs[p['name']], digest) for p in manifest['passes']}
    base = checked[names[0]]['base_functions']
    for name in names[1:]:
        assert checked[name]['base_functions'] == base, ('base modules differ between passes', name)
    counts = {name: checked[name]['verified'] for name in names}
    return counts, unique_total(counts)


def unique_total(counts):
    """counts: pass-name -> verified, in manifest order with base first.

    Every pass verifies the identical base modules (check_passes compares their
    function lists), so each leaf pass's own contribution is its count minus the
    base count."""
    names = list(counts)
    base = counts[names[0]]
    for name in names[1:]:
        assert counts[name] > base, ('leaf pass did not verify more than base', name)
    return base + sum(counts[n] - base for n in names[1:])


def main(argv):
    if argv == ['--check']:
        manifest = load_manifest()
        owner = check_coverage(manifest)
        for p in manifest['passes']:
            print(f"{p['name']} ({p['feature']}): {len(p['modules'])} modules")
        print(f'{len(owner)} verus modules covered by {len(manifest["passes"])} passes, each exactly once; gates match')
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
