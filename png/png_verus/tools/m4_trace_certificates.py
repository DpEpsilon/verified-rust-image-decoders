"""Emit checked certificates from M2's authored forward-encoder intentions.

Read only raw/steps/adler and the trace partition, never M2 call outcomes.
No private imported theorem, decoder, or oracle supplies an expected PNG result.
"""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT.parent / 'inflate_verus/tools/witness_config.json'


def seq(xs):
    return 'seq![' + ','.join(f'{x}u8' for x in xs) + ']'


def certificate(ident, znode, anode, name):
    records = json.loads(SOURCE.read_text())
    selected = [x for x in records if [name, 0] in x['members']]
    assert len(selected) == 1
    source = selected[0]
    raw, steps, ranges = source['raw'], source['steps'], source['trace_chunks']
    n, size = len(raw), len(steps)
    assert n == 1023 and source['intent'] == {'schedule': 'base-or-window-authored', 'record': name}
    rawfn, tsfn = f'raw_{ident}()', f'ts_{ident}()'
    ts = 'seq![' + ','.join('t::Step {' + ','.join(f'{k}:{v}nat' for k, v in s.items()) + '}' for s in steps) + ']'
    end = steps[-1]['next']
    cursor = (end + 7) // 8
    sb = sum(raw)
    sp = sum((n - j) * v for j, v in enumerate(raw)) + n
    adler = (sp % 65521) * 65536 + (1 + sb) % 65521
    assert adler == source['adler']
    done = name != 'precedence/adler-before-filter'
    stat = 'zm::InflateStatus::Done' if done else 'zm::InflateStatus::Error(zm::InflateFault::AdlerMismatch)'
    block = f'(zm::StoredEffect::End {{out:{rawfn},position:{end}nat}})'
    finished = (f'(zm::StoredEffect::End {{out:{rawfn},position:{8*(cursor+4)}nat}})' if done else
                f'(zm::StoredEffect::Error {{error:zm::InflateFault::AdlerMismatch,out:{rawfn},position:{8*(cursor+4)}nat}})')
    lines = [f'// M2 authored certificate source sha256 {hashlib.sha256(SOURCE.read_bytes()).hexdigest()}',
             f'spec fn raw_{ident}() -> Seq<u8> {{{seq(raw)}}}',
             f'spec fn ts_{ident}() -> Seq<t::Step> {{{ts}}}']
    for j, (lo, hi) in enumerate(ranges):
        lines += ['#[verifier::spinoff_prover]', f'proof fn trace_{ident}_{j}()',
                  f'    ensures t::trace({znode},{rawfn},{tsfn},{lo}nat,{hi}nat),',
                  '{', '    hide(t::valid_step);',
                  f'    assert(t::trace({znode},{rawfn},{tsfn},{lo}nat,{hi}nat)) by (compute);', '}']
    lines += ['#[verifier::spinoff_prover]', f'proof fn trace_{ident}()',
              f'    ensures t::trace({znode},{rawfn},{tsfn},0,{size}nat),', '{',
              f'    hide(z_{ident});hide(raw_{ident});hide(ts_{ident});hide(t::valid_step);hide(t::trace);']
    lines += [f'    trace_{ident}_{j}();' for j in range(len(ranges))]
    def join(lo, hi):
        if [lo, hi] in ranges:
            return
        mid = lo + (hi - lo) // 2
        join(lo, mid)
        join(mid, hi)
        lines.append(f'    t::join_trace({znode},{rawfn},{tsfn},{lo}nat,{hi}nat);')
    join(0, size)
    lines += ['}', '#[verifier::spinoff_prover]', f'proof fn checksum_{ident}()',
              f'    ensures zm::Adler({rawfn})=={adler}nat,', '{',
              '    hide(zm::sum_bytes);hide(zm::sum_prefixes);',
              f'    assert(c::sums({rawfn},0,{n}nat)==({sb}nat,{sp}nat)) by (compute);',
              f'    c::sums_correct({rawfn},0,{n}nat);',
              f'    assert({rawfn}.subrange(0,{n}) =~= {rawfn});',
              '    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);',
              '    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);',
              f'    assert(({sp}nat%65521)*(u16::MAX as nat+1)+(1+{sb}nat)%65521=={adler}nat) by (compute);', '}',
              '#[verifier::spinoff_prover]', f'proof fn shape_{ident}()',
              f'    ensures {rawfn}.len()=={n}nat,{znode}.len()=={cursor+4}nat,{tsfn}.len()=={size}nat,',
              f'        {znode}[0]==120u8,{znode}[1]==156u8,',
              f'        {tsfn}[0].pos==19nat,{tsfn}[0].before==0nat,{tsfn}.last().next=={end}nat,',
              f'        zm::be_value({znode},{cursor}nat,4)=={adler if done else 0}nat,',
              f'        zm::field_value({znode},16,1)==1nat,zm::field_value({znode},17,2)==1nat,',
              '{', f'    assert(zm::be_value({znode},{cursor}nat,4)=={adler if done else 0}nat) by (compute);',
              f'    assert(zm::field_value({znode},16,1)==1nat && zm::field_value({znode},17,2)==1nat) by (compute);', '}',
              '#[verifier::spinoff_prover]', f'proof fn inflate_{ident}()',
              f'    ensures r::inflated({znode},{anode},STRICT)==(zm::Inflated {{status:{stat},out:{rawfn},r2:false}}),', '{',
              f'    hide(z_{ident});hide(raw_{ident});hide(ts_{ident});',
              '    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MBlk);hide(zm::MOneBlock);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(zm::MFinish);',
              f'    shape_{ident}();trace_{ident}();checksum_{ident}();',
              f'    assert(r::raw({anode})=={n}nat) by (compute);',
              f'    t::effects({znode},{rawfn},{tsfn},0,{size}nat);',
              f'    t::expand({znode},{rawfn},{tsfn},0);',
              f'    assert({rawfn}.take(0) =~= Seq::<u8>::empty());',
              f'    assert(zm::MCompressed({znode},19,Seq::empty(),{n}nat,32768,zm::fixed_lit_lengths(),zm::fixed_dist_lengths())=={block});',
              f'    assert(zm::MFinish({znode},{end}nat,{rawfn},{n}nat,false)=={finished}) by {{reveal(zm::MFinish);}}',
              f'    fixed::envelope({znode},{n}nat,{block},{finished});',
              f'    assert(r::inflated({znode},{anode},STRICT)==(zm::Inflated {{status:{stat},out:{rawfn},r2:false}})) by {{reveal(zm::Inflate);}}', '}']
    return lines
