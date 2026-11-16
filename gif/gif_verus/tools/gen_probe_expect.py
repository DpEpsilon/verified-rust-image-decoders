#!/usr/bin/env python3
"""Write tests/data/PROBES.json: the probe inputs of ../policy-draft paired with the
oracles' expectations, in the VECTORS.json schema (SPEC.md G17), for
tests/conformance.rs.

Inputs are the ones validate.py checks: every input of probes/results.json with bytes
(spec-draft vectors, the probes of probes_def.py and the leniency-harness witnesses), plus
any probe in probes_def.py that results.json predates. Vector inputs are kept too (as in
validate.py), so the RecommendedBeta records cover them as well. For each input, four records:
Strict still and frames (gif_oracle.py), RecommendedBeta still and frames
(policy_oracle.py). Expectations are computed by the oracles through
spec-draft/check_vectors.py's `actual`, so the schema is exactly the vectors'.

Usage: python3 -I tools/gen_probe_expect.py [OUT]   (default tests/data/PROBES.json)
"""
import json
import sys
from pathlib import Path

CRATE = Path(__file__).resolve().parent.parent
GIF = CRATE.parent
sys.path.insert(0, str(GIF / 'spec-draft'))
sys.path.insert(0, str(GIF / 'policy-draft'))
sys.path.insert(0, str(GIF / 'policy-draft' / 'probes'))
import check_vectors as CV  # noqa: E402


def inputs():
    seen = {}
    results = json.loads((GIF / 'policy-draft/probes/results.json').read_text())
    for r in results:
        if r['hex'] is not None:
            seen.setdefault(r['hex'], r['name'])
    try:
        from probes_def import PROBES  # noqa: E402
        for name, b, _why in PROBES:
            seen.setdefault(b.hex(), 'probe:' + name)
    except Exception as e:                      # results.json alone still pairs every input
        print(f'probes_def.py not importable ({e}); using results.json only', file=sys.stderr)
    return [(name, h) for h, name in seen.items()]


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else CRATE / 'tests/data/PROBES.json'
    recs = []
    for name, h in inputs():
        for entry in ('still', 'frames'):
            for prof in (None, 'RecommendedBeta'):
                v = {'name': name + ('/frames' if entry == 'frames' else '') + (f'@{prof}' if prof else ''),
                     'entry': entry, 'input_hex': h}
                if prof:
                    v['profile'] = prof
                v['expected'] = {}
                v['expected'] = CV.actual(v)
                recs.append(v)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(recs))
    print(f'{len(recs)} records from {len(recs) // 4} inputs -> {out}')


if __name__ == '__main__':
    main()
