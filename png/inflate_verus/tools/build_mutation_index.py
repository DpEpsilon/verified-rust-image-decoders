#!/usr/bin/env python3
"""Rebuild the retained, requirement-indexed mutation inventory from local evidence."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    spec = importlib.util.spec_from_file_location('proof_mutations', ROOT/'tools/proof_mutations.py')
    mutations = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mutations)
    manifest_path = ROOT/'evidence/manifest.json'
    manifest = json.loads(manifest_path.read_text())
    by_path = {r['path']: r for r in manifest}
    rows = []
    required_runs = {1:[383], 2:[371], 3:[372], 4:[317], 5:[320], 6:[387,382],
                     7:[384], 8:[321], 9:[450], 10:[318], 11:[323,389], 12:[451], 37:[373], 39:[613]}
    required = {n for runs in required_runs.values() for n in runs}
    for line in (ROOT/'RUNLOG.tsv').read_text().splitlines():
        fields = line.split('\t')
        match = re.search(r'Proof mutation ([\w-]+) \(disposable source copy\)', fields[4])
        if not match or not re.search(r'verified, [1-9]\d* errors', fields[5]):
            continue
        run, selector = int(fields[0]), match[1]
        assert run <= 613, 'Extend the milestone-specific revision mapping before indexing later mutations'
        path = f'evidence/run-{run:04d}.txt'
        if path not in by_path:
            shutil.copy2(ROOT/'runs'/f'{run:04d}.log', ROOT/path)
            record = {'run':run, 'path':path, 'sha256':sha(ROOT/path), 'mutation':selector,
                      'verification_results':fields[5]}
            manifest.append(record); by_path[path] = record
        log = (ROOT/path).read_text()
        start = re.search(r'^error: (postcondition not satisfied|precondition not satisfied|assertion failed|invariant not satisfied|possible arithmetic underflow/overflow)', log, re.M)
        assert start, run
        end = log.find('\n\n', start.start())
        diagnostic = log[start.start():end if end >= 0 else None]
        evidence_commit = '1473bef' if run < 500 else '5f697fa' if run < 580 else 'fdb4026' if run < 613 else 'b34b217'
        full_commit = subprocess.check_output(['git','-C',str(ROOT),'rev-parse',evidence_commit],text=True).strip()
        row = {'selector':selector, 'run':run, 'source_revision':{
                   'evidence_commit':full_commit,
                   'scope':'Historical working-tree run recorded by this commit; not an exact clean-commit source binding.' if run < 613 else 'Disposable mutation of b34b217 production sources; exact source archive and split tree hash retained.',
               }, 'failed_proof':diagnostic, 'log':path, 'verification_results':fields[5]}
        if selector in mutations.FILE_CASES:
            filename,before,after = mutations.FILE_CASES[selector]
        else:
            filename = 'inflate.rs';before,after = mutations.CASES[selector]
        row['mutation'] = {'file':'src/'+filename, 'before':before, 'after':after}
        if run in required:
            archive = ('backups/public-envelope-before-witnesses.tar' if run < 350 else
                       'backups/astra-before-work-mutations.tar' if run < 400 else
                       'backups/sol-before-remaining-mutations.tar' if run < 500 else
                       'backups/m2-review-stored-truncation-drop-prefix.tar.gz')
            row['source_revision']['implementation_snapshot'] = archive
            with tarfile.open(ROOT/archive) as snapshot:
                implementation = snapshot.extractfile('src/'+filename).read()
            row['source_revision']['implementation_sha256'] = hashlib.sha256(implementation).hexdigest()
            image = after if run == 613 else before
            assert implementation.decode().count(image) == 1, (run, archive, image)
            location = re.search(r'--> src/\w+.rs:(\d+)', diagnostic)
            prefix = '\n'.join(implementation.decode().splitlines()[:int(location[1])])
            row['failed_function'] = re.findall(r'\bfn (\w+)\s*(?:<|\()', prefix)[-1]
            by_path[path].setdefault('source_archives_sha256',{})[archive] = sha(ROOT/archive)
        if run == 613:
            by_path[path]['kind'] = 'proof-mutation-json'
            row['source_revision']['source_tree_sha256'] = re.search(r'VERIFY_SOURCE_TREE=([0-9a-f]{64})', log)[1]
        if run == 451:
            row['qualification'] = 'Ghost representation narrowing: implementation-side bit-position observation only, not an executable u32 counter; independent model unchanged.'
        rows.append(row)
    by_run = {r['run']:r for r in rows}
    requirements = []
    for number in range(1,41):
        m2 = number in required_runs
        entry = {'number':number, 'milestone':'M2' if m2 else 'not M2',
                 'status':'verifier-killed' if m2 else 'not credited here; later PNG/allocation/policy milestone',
                 'mutants':[by_run[n] for n in required_runs.get(number,[])]}
        if number == 12:entry['qualification'] = by_run[451]['qualification']
        requirements.append(entry)
    table = {'schema':1, 'requirement_source':'../THEOREMS.md:215–254',
             'selector_definitions_sha256':sha(ROOT/'tools/proof_mutations.py'),
             'provenance_note':'Historical runs predate source-hash-bound checkpoints. evidence_commit is the commit that records the run, not a claim that the mutant ran on that clean commit. implementation_snapshot identifies a retained implementation revision from that phase; it does not retroactively bind every historical proof/helper file. Mutation 39 has an exact disposable archive and split source hash.',
             'requirements':requirements, 'additional_mutations':[r for r in rows if r['run'] not in required],
             'excluded_attempts':[{'run':453,'reason':'Rust type-check failure, no failed Verus proof; not a kill.'}],
             'historical_kill_total':len(rows)}
    (ROOT/'evidence/mutation-index.json').write_text(json.dumps(table,indent=1)+'\n')
    p='evidence/mutation-index.json'
    record={'path':p,'sha256':sha(ROOT/p),'role':'requirement-mutation-index'}
    if p in by_path:by_path[p].update(record)
    else:manifest.append(record)
    manifest_path.write_text(json.dumps(manifest,indent=2)+'\n')
    print(f'{len(requirements)} requirements; {len(required_runs)} M2 requirements; {len(rows)} historical kills; {len(table["additional_mutations"])} additional kills')

if __name__ == '__main__':
    main()
