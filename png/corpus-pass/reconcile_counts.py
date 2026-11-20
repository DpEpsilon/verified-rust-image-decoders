#!/usr/bin/env python3
"""Reconcile the input counts of PLAN.md M0-corpus (748 / 737 / 740 / 767 / 773 / 784) from the files on disk.

  python3 -I reconcile_counts.py [--json OUT]

Definitions (all counts are of synthetic inputs: vectors, probes, witnesses; none is a corpus file):
  748  records in policy-draft/probes/results.json (vec + probe + wit; 14 above 4 KiB stored by reference, hex None).
  N    the Strict bridge of policy-draft/validate.py: distinct byte strings among (a) every vector record without
       `pad_to` in the three vector files and (b) every results.json record with inline hex. Records above 4 KiB
       (hex None) and padded vectors are not bridged.
The historical values are recomputed by removing, newest first, the vector records each change added (names from
policy-draft/DECISIONS.md); the old-cap vectors replaced on 2026-11-17 still contribute through results.json.
Only bytes from the repository's own generated files are read; nothing is written except the optional JSON."""
import json
import sys
from pathlib import Path

PNG = Path(__file__).resolve().parents[1]
FILES = ['spec-frozen/VECTORS.json', 'limits-draft/VECTORS-CAPS.json', 'policy-draft/VECTORS-POLICY.json']

REVIEW_2026_11_17 = ['apng/empty-fdat-payload', 'apng/empty-fdat-then-data', 'apng/empty-fdat-payload-still',
                     'apng/empty-fdat-then-data-still', 'icc/empty-profile', 'deflate/fixed-dist-31',
                     'deflate/fixed-dist-30-then-eof', 'deflate/fixed-dist-31-then-eof', 'limits/exact-max-input-trailing',
                     'deflate/single-literal-code-other-bit-then-eof', 'deflate/single-literal-code-live-prefix-eof',
                     'deflate/length-with-no-distance-codes-then-eof', 'deflate/single-distance-code-other-bit-then-eof',
                     'deflate/single-distance-code-live-prefix-eof']
CAPS_2026_11_17 = ['ihdr/width-2^31-1-output', 'ihdr/67108865x1-output', 'ihdr/8192x8193-output',
                   'ihdr/8192x8192-accepted-then-truncated-zlib', 'ihdr/67108864x1-accepted-then-truncated-zlib',
                   'ihdr/1x67108864-accepted-then-truncated-zlib']
OVER_2026_11_16 = ['apng/blend-over-transparent-source', 'apng/blend-over-transparent-destination',
                   'apng/blend-over-zero-result-alpha']


def main():
    vec = []
    for f in FILES:
        for v in json.loads((PNG / f).read_text()):
            vec.append(dict(v, _file=f))
    names = {v['name'] for v in vec}
    res = json.loads((PNG / 'policy-draft/probes/results.json').read_text())
    inline = [bytes.fromhex(r['hex']) for r in res if r['hex'] is not None]
    out = {'results_json_records': len(res),
           'results_json_by_kind': {k: sum(1 for r in res if r['name'].startswith(k + ':')) for k in ('vec', 'probe', 'wit')},
           'results_json_by_reference_over_4KiB': sum(1 for r in res if r['hex'] is None),
           'results_json_inline': len(inline),
           'vector_records': {f: sum(1 for v in vec if v['_file'] == f) for f in FILES},
           'vector_records_padded': sum(1 for v in vec if 'pad_to' in v)}

    def bridge(drop=(), drop_policy=False):
        s = set(inline)
        for v in vec:
            if 'pad_to' in v or v['name'] in drop or (drop_policy and v['_file'].startswith('policy-draft')):
                continue
            s.add(bytes.fromhex(v['input_hex']))
        return len(s)

    missing = [n for n in REVIEW_2026_11_17 + CAPS_2026_11_17 + OVER_2026_11_16 if n not in names]
    assert not missing, missing
    d1 = set(REVIEW_2026_11_17)
    d2 = d1 | set(CAPS_2026_11_17)
    d3 = d2
    d4 = d2 | set(OVER_2026_11_16)
    steps = [('784', 'now: Strict bridge after the review and council vectors (2026-11-17)', bridge()),
             ('773', 'without the 14 review/council vectors (11 distinct non-padded inputs)', bridge(d1)),
             ('767', 'also without the 6 cap-boundary vectors of 2026-11-17', bridge(d2)),
             ('740', 'also without the 198 profile vectors of VECTORS-POLICY.json', bridge(d3, drop_policy=True)),
             ('737', 'also without the 3 OVER corner vectors of 2026-11-16', bridge(d4, drop_policy=True))]
    out['bridge_steps'] = [{'claimed': c, 'what': w, 'recomputed': n, 'agrees': int(c) == n} for c, w, n in steps]
    # how many distinct inputs each vector group contributes beyond the inline records
    nonpad_core = {bytes.fromhex(v['input_hex']) for v in vec if 'pad_to' not in v and not v['_file'].startswith('policy-draft')
                   and v['name'] not in d4}
    out['distinct_nonpadded_vectors_before_2026_11_16'] = len(nonpad_core)
    out['inline_distinct'] = len(set(inline))
    out['overlap_inline_and_those_vectors'] = len(set(inline) & nonpad_core)
    out['review_vectors_distinct_new_inputs'] = bridge() - bridge(d1)
    out['policy_vectors_distinct_new_inputs'] = bridge(d3) - bridge(d3, drop_policy=True)
    out['records_now'] = len(vec)
    for s in out['bridge_steps']:
        print(f"{s['claimed']:>4}  recomputed {s['recomputed']:>4}  {'ok' if s['agrees'] else 'DIFFERS'}  {s['what']}")
    print(json.dumps({k: v for k, v in out.items() if k != 'bridge_steps'}, indent=1))
    if '--json' in sys.argv:
        Path(sys.argv[sys.argv.index('--json') + 1]).write_text(json.dumps(out, indent=1) + '\n')
    return 0 if all(s['agrees'] for s in out['bridge_steps']) else 1


if __name__ == '__main__':
    sys.exit(main())
