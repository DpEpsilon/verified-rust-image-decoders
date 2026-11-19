#!/usr/bin/env python3
"""Run one implementation-only proof mutation in a durable disposable copy.

The copy shares the pre-seeded target directory; this script never removes a
source or target directory. Every verifier invocation uses the copied verify.sh,
with its original flags, lock and per-pass timeout. Logs append to the main run inventory.
"""
import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CASES = {
    'length-extra-value-plus-one': ('let length = LEN_BASE[index] as u32 + extra;', 'let length = LEN_BASE[index] as u32 + extra + 1;'),
    'window-boundary-equality-rejected': ('if distance > output.produced || distance > window {', 'if distance > output.produced || distance >= window {'),
    'omit-nlen': ('if len != !nlen { return Err(Status::Err(E::StoredLengthMismatch)); }',
                  'if false { return Err(Status::Err(E::StoredLengthMismatch)); }'),
    'wrong-length-285': ('163, 195, 227, 258,', '163, 195, 227, 257,'),
    'omit-window-guard': ('if distance > output.produced || distance > window {',
                          'if distance > output.produced {'),
    'wrong-copy-offset': ('let byte = output.previous(distance);',
                          'let byte = output.previous(1);'),
    'full-at-limit': ('if self.produced == self.limit {',
                      'if self.produced + 1 == self.limit {'),
    'omit-copy-charge': ('w.add(1);\n        let byte = output.previous(distance);',
                         'let byte = output.previous(distance);'),
    'reserved-length-286-admitted': ('if sym > 285 {', 'if sym > 286 {'),
    'early-bfinal': ('Ok(last) => if last { return Ok(()); },',
                     'Ok(last) => if true { return Ok(()); },'),
    'omit-adler-comparison': ('if expected != output.adler.finish() {',
                             'if false {'),
    'ignore-short-tail': ('if ignore_tail && output.produced == output.limit {',
                          'if ignore_tail {'),
    'omit-fcheck': ('if (cmf as u16 * 256 + flg as u16) % 31 != 0 {',
                    'if false {'),
    'little-endian-checksum': ('expected = (expected << 8) | byte as u32;',
                               'expected = (expected >> 8) | ((byte as u32) << 24);'),
}


FILE_CASES = {
    'stored-truncation-drop-prefix': ('inflate.rs', 'proof { assert(cursor == z.len()); }\n                return Err(Status::Err(e));', 'proof { assert(cursor == z.len()); }\n                output.produced = 0;\n                return Err(Status::Err(e));'),
    'length-extra-width-plus-one': ('inflate.rs', 'br.get(LEN_EXTRA[index], w)', 'br.get(LEN_EXTRA[index] + 1, w)'),
    'distance-extra-width-plus-one': ('inflate.rs', 'br.get(DIST_EXTRA[ds], w)', 'br.get(DIST_EXTRA[ds] + 1, w)'),
    'bits-partial-eof-invent-buffered-bit': ('bits.rs', 'Err(e) => return Err(e),\n                };\n                proof {', 'Err(e) => { self.remaining = 1; return Err(e); },\n                };\n                proof {'),
    'reader-eof-advances-byte-position': ('segments.rs', 'return None;\n            }\n            let part', 'self.position += 1;\n                return None;\n            }\n            let part'),
    'single-literal-rejected': ('huffman.rs', 'is_admissible(kind, kraft, self.nsyms, self.count[1], single_accept)', 'is_admissible(kind, kraft, self.nsyms, self.count[1], false)'),
    'single-literal-two-bit-admitted': ('huffman.rs', 'is_admissible(kind, kraft, self.nsyms, self.count[1], single_accept)', 'is_admissible(kind, kraft, self.nsyms, self.nsyms as u16, single_accept)'),
    'single-distance-empty-rejected': ('huffman.rs', 'if admissible {', 'if admissible && self.nsyms > 0 {'),
    'live-count-floor': ('canonical.rs', 'live[n] = (higher + unit - 1) / unit;', 'live[n] = higher / unit;'),
    'live-count-reverse-shift': ('canonical.rs', 'let unit = 1u32 << (MAX_CODE_BITS - n);', 'let unit = 1u32 << n;'),
    'live-count-reverse-depth': ('canonical.rs', 'higher += counts[n] as u32 * unit;', 'higher += counts[n - 1] as u32 * unit;'),
    'live-count-omit-depth-zero': ('canonical.rs', 'if n == 0 { return Ghost(steps); }', 'if n == 1 { return Ghost(steps); }'),
    'dead-prefix-lower-bound-inclusive': ('huffman.rs', 'if code < first || code - first - count >= self.live[n] {', 'if code <= first || code - first - count >= self.live[n] {'),
    'dead-prefix-upper-bound-exclusive': ('huffman.rs', 'if code < first || code - first - count >= self.live[n] {', 'if code < first || code - first - count > self.live[n] {'),
    'length-extras-omitted': ('inflate.rs', 'br.get(LEN_EXTRA[index], w)', 'br.get(0, w)'),
    'late-adler-deferral': ('adler.rs', 'if self.pending == ADLER_NMAX {', 'if self.pending == ADLER_NMAX + 1 {'),
    'bit-position-u32-narrowing': ('bits.rs', '(8 * self.src.pos() - self.remaining) as nat', '((8 * self.src.pos() - self.remaining) as u32) as nat'),
    'reader-byte-as-segment-work': ('segments.rs', 'proof { self.read_work = Ghost(self.read_work@ + 1); }\n                return Some(byte);', 'proof { self.seg_work = Ghost(self.seg_work@ + 1); }\n                return Some(byte);'),
    'omit-symbol-iteration-charge': ('inflate.rs', 'let ghost symbol_iteration = w.observed@;\n    w.add(1);', 'let ghost symbol_iteration = w.observed@;'),
    'omit-block-iteration-charge': ('inflate.rs', 'let ghost block_iteration = w.observed@;\n    w.add(1);', 'let ghost block_iteration = w.observed@;'),
    'omit-cl-construction-charge': ('inflate.rs', 'w.add(NCL);\n    proof { assert(w.observed@ == cl_zero_fill', 'w.add(0);\n    proof { assert(w.observed@ == cl_zero_fill'),
    'omit-dynamic-symbol-charge': ('inflate.rs', 'let ghost header_iteration = w.observed@;\n            w.add(1);', 'let ghost header_iteration = w.observed@;'),
    'omit-huffman-decode-charge': ('huffman.rs', 'let ghost decode_iteration = w.observed@;\n            w.add(1);', 'let ghost decode_iteration = w.observed@;'),
    'omit-bits-loop-charge': ('bits.rs', 'w.add(1);\n            proof { self.loop_work', 'w.add(0);\n            proof { self.loop_work'),
    'reader-omit-byte-charge': ('segments.rs', 'proof { self.read_work = Ghost(self.read_work@ + 1); }\n                return Some(byte);', 'return Some(byte);'),
    'reader-wrong-byte': ('segments.rs', 'let byte = part[self.offset];', 'let byte = 0u8;'),
    'reader-skip-empty-position': ('segments.rs', 'self.offset = 0;\n            proof { self.seg_work', 'self.offset = 1;\n            proof { self.seg_work'),
    'reader-first-eof-double-charge': ('segments.rs', 'self.end_reported = true;\n                    proof', 'self.end_reported = true;\n                    proof { self.read_work = Ghost(self.read_work@ + 1); }\n                    proof'),
    'reader-omit-segment-charge': ('segments.rs', 'self.offset = 0;\n            proof { self.seg_work = Ghost(self.seg_work@ + 1); }', 'self.offset = 0;'),
    'bits-wrong-shift': ('bits.rs', 'self.current >>= 1;', 'self.current <<= 1;'),
    'bits-wrong-alignment': ('bits.rs', 'self.current = 0;\n        self.remaining = 0;', 'self.current = 0;\n        self.remaining = 1;'),
    'bits-misclassify-first-eof': ('bits.rs', 'w.observed@ + self.src.core_steps() - before', 'w.observed@ + 1'),
    'huffman-lsb-first': ('huffman.rs', 'code = code * 2 + bit;', 'code = code + (bit << n);'),
    'huffman-accept-incomplete': ('huffman.rs', 'if admissible {', 'if true {'),
    'huffman-skip-dead-prefix': ('huffman.rs', 'if code < first || code - first - count >= self.live[n] {', 'if false {'),
    'huffman-empty-as-truncated': ('huffman.rs', 'return Err(InflateError::InvalidSymbol);\n        }\n        let mut code', 'return Err(InflateError::TruncatedZlib);\n        }\n        let mut code'),
    'huffman-omit-init-charge': ('huffman.rs', 'w.add(HUFF_INIT_STEPS);', 'w.add(0);'),
    'dynamic-cl-permutation': ('inflate.rs', 'cl[CL_ORDER[i]] = value as u8;', 'cl[i] = value as u8;'),
    'dynamic-repeat-overflow': ('inflate.rs', 'if repeat > total - i {', 'if false {'),
    'dynamic-wrong-repeat-fill': ('inflate.rs', 'write_length_repeat(&mut lengths, i, repeat, value, w)', 'write_length_repeat(&mut lengths, i, repeat, 0, w)'),
    'dynamic-omit-eob-check': ('inflate.rs', 'if lengths[256] == 0 {', 'if false {'),
    'dynamic-count-before-third': ('inflate.rs', 'let ncode = match br.get(4, w)', 'if nlen > NLIT_MAX || ndist > NDIST_MAX { return Err(E::InvalidCodeLengths); }\n        let ncode = match br.get(4, w)'),
    'dynamic-repeat16-read-before-previous': ('inflate.rs', 'if i == 0 { return (Err(E::InvalidCodeLengths), Ghost(construction)); }\n                    let extra = match br.get(2, w) {\n                        Ok(extra) => extra, Err(e) => return (Err(e), Ghost(construction)),\n                    };', 'let extra = match br.get(2, w) {\n                        Ok(extra) => extra, Err(e) => return (Err(e), Ghost(construction)),\n                    };\n                    if i == 0 { return (Err(E::InvalidCodeLengths), Ghost(construction)); }'),
    'fixed-only-thirty-distances': ('inflate.rs', 'let dist_lengths = [5u8; FIXED_DIST_COUNT];', 'let dist_lengths = [5u8; NDIST_MAX];'),
    'btype-swap-fixed-dynamic': ('inflate.rs', 'match typ {', 'match if typ == 1 { 2 } else if typ == 2 { 1 } else { typ } {'),
    'distance-output-plus-one': ('inflate.rs', 'if distance > output.produced || distance > window {', 'if distance > output.produced + 1 || distance > window {'),
    'distance-wrong-base': ('inflate.rs', 'let distance = DIST_BASE[ds] as u64 + de as u64;', 'let distance = DIST_BASE[ds] as u64 + de as u64 + 1;'),
    'distance-wrong-extras': ('inflate.rs', 'br.get(DIST_EXTRA[ds], w)', 'br.get(0, w)'),
    'reserved-distance-admitted': ('inflate.rs', 'if ds >= NDIST_MAX {', 'if ds > NDIST_MAX {'),
    'reader-work-in-construction-budget': ('inflate.rs', 'let ghost cost = NCL + ncode + w.observed@ - before_build;', 'let ghost cost = w.observed@ - cl_zero_fill;'),
    'decode-work-in-construction-budget': ('inflate.rs', 'let ghost after_code = br.position();', 'proof { construction = (construction + w.observed@ - header_iteration - 1) as nat; }\n            let ghost after_code = br.position();'),
    'dynamic-admit-before-eob': ('inflate.rs', '''        if lengths[256] == 0 { return (Err(E::InvalidCodeLengths), Ghost(construction)); }
        // The coding-cost invariant applies even when construction rejects.
        // Pin the frozen EOB-before-admission boundary at the real build call.
        proof { assert(lengths@[256] > 0); }
        proof { self.header_table_start = Ghost(Some(br.position())); }
        let ghost before_lit = w.observed@;
        let lit_result = self.lit.build(&lengths[..nlen], Kind::LitLen, w);''', '''        // The coding-cost invariant applies even when construction rejects.
        // Pin the frozen EOB-before-admission boundary at the real build call.
        proof { assert(lengths@[256] > 0); }
        proof { self.header_table_start = Ghost(Some(br.position())); }
        let ghost before_lit = w.observed@;
        let lit_result = self.lit.build(&lengths[..nlen], Kind::LitLen, w);
        if lengths[256] == 0 { return (Err(E::InvalidCodeLengths), Ghost(construction)); }'''),
    'dynamic-distance-admission-first': ('inflate.rs', '''        let lit_result = self.lit.build(&lengths[..nlen], Kind::LitLen, w);
        proof { construction = (construction + w.observed@ - before_lit) as nat; }
        match lit_result { Ok(()) => {}, Err(e) => return (Err(e), Ghost(construction)), }
        proof { assert(self.lit.ready@); }
        let ghost before_dist = w.observed@;
        let dist_result = self.dist.build(&lengths[nlen..total], Kind::Dist, w);''', '''        let lit_result = self.dist.build(&lengths[nlen..total], Kind::Dist, w);
        proof { construction = (construction + w.observed@ - before_lit) as nat; }
        match lit_result { Ok(()) => {}, Err(e) => return (Err(e), Ghost(construction)), }
        proof { assert(self.lit.ready@); }
        let ghost before_dist = w.observed@;
        let dist_result = self.lit.build(&lengths[..nlen], Kind::LitLen, w);'''),
    'output-omit-limit-guard': ('inflate.rs', 'if self.produced == self.limit {', 'if false {'),
}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('case', choices=list(CASES) + list(FILE_CASES))
    parser.add_argument('--case-root', type=Path, default=ROOT / 'mutation-cases',
                        help='durable disposable-copy parent, inside this crate')
    args = parser.parse_args()
    case_root = args.case_root.resolve()
    assert case_root.is_relative_to(ROOT.resolve()), 'mutation copies must remain inside this crate'
    if args.case in FILE_CASES:
        filename, before, after = FILE_CASES[args.case]
    else:
        filename = 'inflate.rs'
        before, after = CASES[args.case]
    text = (ROOT / 'src' / filename).read_text()
    assert text.count(before) == 1, (args.case, text.count(before))
    dest = case_root / args.case
    assert not dest.exists(), f'Keep existing diagnostics; choose another case: {dest}'
    dest.mkdir(parents=True)
    for name in ('src', 'tools', 'examples', 'tests'):
        shutil.copytree(ROOT / name, dest / name)
    for name in ('Cargo.toml', 'Cargo.lock', 'verify.sh'):
        shutil.copy2(ROOT / name, dest / name)
    for name in ('target', 'runs', 'RUNLOG.tsv'):
        (dest / name).symlink_to(ROOT / name, target_is_directory=name != 'RUNLOG.tsv')
    source = dest / 'src' / filename
    source.write_text(text.replace(before, after))
    # verify.sh runs the split passes serially, each bounded at 25 minutes, and
    # stops at the first failing pass; bound the whole sequence accordingly.
    passes = len(json.loads((dest / 'tools/verify_passes.json').read_text())['passes'])
    result = subprocess.run(['timeout', f'{passes * 1530}s', str(dest / 'verify.sh'),
                             f'Proof mutation {args.case} (disposable source copy)'],
                            timeout=passes * 1530 + 30)
    row = (ROOT / 'RUNLOG.tsv').read_text().splitlines()[-1]
    match = re.search(r'verification results:: (\d+) verified, (\d+) errors', row)
    assert result.returncode != 0 and match and int(match[2]) > 0, row
    print(f'MUTATION KILLED: {args.case}; {match[1]} verified, {match[2]} errors')


if __name__ == '__main__':
    main()
