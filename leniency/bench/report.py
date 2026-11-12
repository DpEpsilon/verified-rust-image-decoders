#!/usr/bin/env python3
"""Render results/*.json into markdown tables (stdout)."""
import json, os, sys
sys.dont_write_bytecode = True
H = os.path.dirname(os.path.abspath(__file__))
def row(name, subset, m, ref=None):
    flag = ' NOISY' if m['round_spread'] > 0.05 or m['wall_spread'] > 0.30 else ''
    ratio = f" ({m['ns_per_px']/ref:.2f}x)" if ref else ''
    load = f"{max(x['runnable_others'] for l in m['load'] for x in l)}"
    return (f"| {name} | {subset} | {m['ns_per_px']:.2f}{ratio} | {m['min_ns_per_px']:.2f} | {m['wall_ns_per_px']:.2f} / {m['wall_min_ns_per_px']:.2f} | {m['mbps']:.0f} | "
            f"{m['round_spread']*100:.0f}% / {m['wall_spread']*100:.0f}%{flag} | {m['hwm_mb']:.0f} ({m['hwm_mb']-m['base_mb']:+.0f}) | {load} |")
HDR = ("| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |\n"
       "|---|---|---|---|---|---|---|---|---|")
for fmt in ['bmp', 'tga', 'icns', 'ico']:
    for part in ['corpus', 'large']:
        p = f'{H}/results/{fmt}-{part}.json'
        if not os.path.exists(p): continue
        d = json.load(open(p))
        print(f"\n### {fmt.upper()} {part}\n")
        print(f"files: {d['n_files']}, verified decodes {d['verified_ok']}; rounds={d['rounds']} passes={d['passes']} warmup={d['warmup']}\n")
        print("agreement with verified (per incumbent): " + '; '.join(f"{k}: " + ', '.join(f'{a} {b}' for a, b in v['counts'].items()) for k, v in d['agreement'].items()) + "\n")
        print(HDR)
        for key, t in d['timing'].items():
            if 'decoders' not in t:
                print(f"| {key} | - | {t.get('note')} ({t.get('ver_msg')}) | | | | |"); continue
            ref = t['decoders'].get(next((k for k in t['decoders'] if 'verified' in k), ''), {}).get('ns_per_px')
            subset = f"{key} ({t['n_files']} files)" if part == 'corpus' else key
            for dec, m in t['decoders'].items():
                if 'error' in m: print(f"| {dec} | {subset} | error | | | | |"); continue
                print(row(dec, subset, m, ref if 'verified' not in dec else None))
            for f in t.get('flags', [])[:2]: print(f"<!-- {f} -->")
