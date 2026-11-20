#!/usr/bin/env python3
"""Generate whole-walk/public-error witnesses from authored frozen fixtures.

The status inventory chooses records; expectations come from the fixture itself.
No decoder, policy oracle or third-party corpus supplies an expected result.
"""
import argparse
import binascii
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_NAMES = ['container/duplicate-ihdr', 'apng-bad/fctl-width-0']
NAMES = dict(zip(
    [b'IHDR',b'PLTE',b'IDAT',b'IEND',b'tRNS',b'cHRM',b'gAMA',b'iCCP',b'sBIT',b'sRGB',b'cICP',b'mDCv',b'cLLi',b'bKGD',b'hIST',b'pHYs',b'sPLT',b'eXIf',b'tIME',b'tEXt',b'zTXt',b'iTXt',b'acTL',b'fcTL',b'fdAT'],
    ['Ihdr','Plte','Idat','Iend','Trns','Chrm','Gama','Iccp','Sbit','Srgb','Cicp','Mdcv','Clli','Bkgd','Hist','Phys','Splt','Exif','Time','Text','Ztxt','Itxt','Actl','Fctl','Fdat']))



def read_facts(data, ident, frozen_error):
    """Print checked intermediate wire certificates, not predicted outcomes.

    CRC booleans are certificate suggestions; each is proved from the independent
    bitwise model. Only the frozen fixture supplies the public expected error.
    """
    depth, ct = data[24], data[25]
    bound = '(u16::MAX as u32)+1' if depth == 16 else f'{1 << depth}u32'
    facts = [f'        assert((1u32 << {depth}u8)=={bound}) by (compute);']
    entries = 0
    had_idat = False
    after_plte_seen = False
    pos = 33
    sites = 0
    while True:
        sites += 1
        if pos == len(data):
            expected = 'Err(Error::MissingIend)'
            terminal = True
        elif pos + 8 > len(data):
            expected = 'Err(Error::TruncatedChunk)'
            terminal = True
        else:
            size = int.from_bytes(data[pos:pos+4], 'big')
            tag = data[pos+4:pos+8]
            if size > 0x7fffffff:
                expected = 'Err(Error::ChunkTooLong)'
                terminal = True
            elif any(not (65 <= x <= 90 or 97 <= x <= 122) for x in tag):
                expected = 'Err(Error::InvalidChunkType)'
                terminal = True
            elif pos + 12 + size > len(data):
                expected = 'Err(Error::TruncatedChunk)'
                terminal = True
            else:
                wire_crc = int.from_bytes(data[pos+8+size:pos+12+size], 'big')
                crc_ok = binascii.crc32(data[pos+4:pos+8+size]) == wire_crc
                ty = ','.join(f'{x}u8' for x in tag)
                expected = (f'Ok(crate::critical_model::Candidate {{ty:seq![{ty}],'
                            f'data:{pos+8}nat,len:{size}nat,next:{pos+12+size}nat,'
                            f'crc_ok:{str(crc_ok).lower()},synthetic:false}})')
                terminal = tag == b'IEND'
        expression = f'crate::critical_model::read(bytes_{ident}(),{pos}nat,phase,false)=={expected}'
        facts += [f'        assert forall|phase: crate::walk_model::Phase| {expression} by {{',
                  '            reveal(crate::critical_model::read);',
                  f'            assert({expression}) by (compute);', '        }']
        if pos+8 <= len(data):
            facts += [f'        assert(({tag[0]}u8 & 32u8)=={tag[0] & 32}u8) by (bit_vector);']
            name = NAMES.get(tag, f'Unknown({str(bool(tag[0] & 32)).lower()})')
            ty = ','.join(f'{x}u8' for x in tag)
            expression = f'crate::walk_model::classify(seq![{ty}])==crate::walk_model::Name::{name}'
            facts += [f'        assert({expression}) by {{',
                      '            reveal(crate::walk_model::classify);',
                      f'            assert({expression}) by (compute);', '        }']

        if pos+12+size <= len(data):
            payload = data[pos+8:pos+8+size]
            payload_expr = f'bytes_{ident}().subrange({pos+8},{pos+8+size})'
            if tag == b'PLTE':
                if had_idat or after_plte_seen:
                    decision = 'Error(Error::ChunkOutOfOrder)'
                elif ct in [0,4]:
                    decision = 'Error(Error::UnexpectedPalette)'
                elif size == 0 or size % 3 or size > 768 or (ct == 3 and size//3 > 1 << depth):
                    decision = 'Error(Error::InvalidPalette)'
                else:
                    decision = f'Keep({size//3}nat)'
                expression = (f'crate::metadata_model::palette({size}nat,{ct}u8,{depth}u8,'
                              f'{str(had_idat).lower()},{str(after_plte_seen).lower()},false,false)'
                              f'==crate::metadata_model::Decision::{decision}')
                facts += [f'        assert({expression}) by (compute);']
                entries = size//3
            if tag == b'tRNS':
                if entries > 256:
                    decision = 'OutsideDomain'
                elif ct in [4,6] or (ct == 0 and size != 2) or (ct == 2 and size != 6) or (ct == 3 and not 1 <= size <= entries):
                    decision = 'Error(Error::InvalidTransparency)'
                else:
                    decision = f'Keep({size}nat)'
                expression = (f'crate::metadata_model::transparency({size}nat,{ct}u8,{entries}nat,false)'
                              f'==crate::metadata_model::Decision::{decision}')
                facts += [f'        assert({expression}) by (compute);']
            if tag == b'tEXt' and frozen_error != 'InvalidText':
                nul = payload.index(0)
                key = ','.join(f'{x}u8' for x in payload[:nul])
                facts += [f'        assert(crate::model::first_nul({payload_expr},0)==Some({nul}nat)) by (compute);',
                          f'        assert({payload_expr}.subrange(0,{nul})==seq![{key}]) by (compute);',
                          f'        assert(crate::model::keyword(seq![{key}]));']
            if tag == b'gAMA' and size == 4:
                value = int.from_bytes(payload,'big')
                outcome = 'Error(Error::InvalidGamma)' if value == 0 else f'Value(crate::metadata_model::Value::Gama({value}u32))'
                expression = (f'crate::metadata_model::parse(crate::walk_model::Name::Gama,{payload_expr},'
                              f'{ct}u8,{depth}u8,{entries}nat)==crate::metadata_model::Outcome::{outcome}')
                facts += [f'        assert({expression}) by (compute);']
            had_idat = had_idat or tag == b'IDAT'
            after_plte_seen = after_plte_seen or tag in [b'tRNS',b'bKGD',b'hIST']
        if pos+8 <= len(data) and tag in [b'acTL', b'fcTL', b'fdAT'] and pos+12+size <= len(data):
            payload = data[pos+8:pos+8+size]
            # Small literal words keep payload parsing out of the large input term.
            for offset in range(0, min(size, 20), 4):
                if offset+4 <= size:
                    word = ','.join(f'{x}u8' for x in payload[offset:offset+4])
                    value = str(int.from_bytes(payload[offset:offset+4], 'big'))
                    expression = (f'crate::model::be32(bytes_{ident}().subrange({pos+8},{pos+8+size}),{offset})=={value}')
                    facts += [f'        assert({expression}) by (compute);']
        if pos+8 <= len(data) and tag == b'acTL' and size == 8 and pos+12+size <= len(data):
            count = int.from_bytes(data[pos+8:pos+12], 'big')
            relation = '>' if frozen_error == 'TooManyFrames' else '<='
            facts += [f'        assert({count}u32{relation}crate::limits::MAX_FRAMES) by (bit_vector);']
        if pos+8 <= len(data) and tag == b'fcTL' and size == 26 and pos+12+size <= len(data):
            width = int.from_bytes(data[pos+12:pos+16], 'big')
            height = int.from_bytes(data[pos+16:pos+20], 'big')
            facts += [f'        assert(({width}u64)*({height}u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);']
        if terminal:
            break
        pos += size + 12
    return facts, sites



MODEL_HELPERS = r"""

// This proof never expands the large palette payload; the caller checks its wire.
#[verifier::spinoff_prover]
proof fn first_palette_error(b: Seq<u8>,h: crate::model::Ihdr,length: nat,error: Error)
    requires 41+length<=b.len(),crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![80u8,76u8,84u8,69u8],data:41,len:length,next:45+length,crc_ok:true,synthetic:false}),
        crate::metadata_model::palette(length,h.color_type,h.depth,false,false,false,false)==crate::metadata_model::Decision::Error(error),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(error,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::metadata_model::palette);hide(m::drive);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let s=m::initial(h,33,m::no_events());
    assert(m::step(b,m::strict(),false,false,s)==m::Step::Error(error,m::no_events()));
    m::drive_step(b,m::strict(),false,false,s);
}
// Symbolic prefix states prevent the fixture's byte array entering transition solving.
spec fn zero_s0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn zero_s1(h: crate::model::Ihdr) -> m::State {
    let s=zero_s0(h);
    m::State {pos:53,seen:s.seen.insert(crate::walk_model::Name::Actl),
        doc:m::Doc {actl:Some((2u32,0u32)),..s.doc},..s}
}
spec fn zero_s2(h: crate::model::Ihdr) -> m::State {
    let s=zero_s1(h);
    m::State {pos:88,phase:crate::walk_model::Phase::During,control:crate::apng_model::begin_idat(s.control),
        doc:m::Doc {idat:s.doc.idat.push(crate::walk_model::Range {offset:61,len:23}),..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn zero_actl_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false}),
        crate::model::be32(b.subrange(41,49),0)==2,crate::model::be32(b.subrange(41,49),4)==0,
    ensures m::step(b,m::strict(),true,false,zero_s0(h))==m::Step::Next(zero_s1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::be32);hide(m::initial_info);
    assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn zero_idat_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,h.color_type==6,
        crate::critical_model::read(b,53,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![73u8,68u8,65u8,84u8],data:61,len:23,next:88,crc_ok:true,synthetic:false}),
    ensures m::step(b,m::strict(),true,false,zero_s1(h))==m::Step::Next(zero_s2(h)),
{
    hide(crate::critical_model::read);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn zero_fctl_step(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,
        crate::critical_model::read(b,88,crate::walk_model::Phase::During,false)==Ok(crate::critical_model::Candidate {
            ty:seq![102u8,99u8,84u8,76u8],data:96,len:26,next:126,crc_ok:true,synthetic:false}),
        crate::model::be32(b.subrange(96,122),0)==0,crate::model::be32(b.subrange(96,122),4)==0,
    ensures m::step(b,m::strict(),true,false,zero_s2(h))==m::Step::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::be32);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn late_zero_width_control(b: Seq<u8>,h: crate::model::Ihdr)
    requires b.len()>=126,h.color_type==6,crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![97u8,99u8,84u8,76u8],data:41,len:8,next:53,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,53,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![73u8,68u8,65u8,84u8],data:61,len:23,next:88,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,88,crate::walk_model::Phase::During,false)==Ok(crate::critical_model::Candidate {
            ty:seq![102u8,99u8,84u8,76u8],data:96,len:26,next:126,crc_ok:true,synthetic:false}),
        crate::model::be32(b.subrange(41,49),0)==2,crate::model::be32(b.subrange(41,49),4)==0,
        crate::model::be32(b.subrange(96,122),0)==0,crate::model::be32(b.subrange(96,122),4)==0,
    ensures m::MWF_p(b,m::strict())==m::Outcome::Error(Error::InvalidFrameControl,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::model::be32);hide(m::drive);hide(m::initial_info);
    zero_actl_step(b,h);zero_idat_step(b,h);zero_fctl_step(b,h);
    m::drive_step(b,m::strict(),true,false,zero_s2(h));
    m::drive_step(b,m::strict(),true,false,zero_s1(h));
    m::drive_step(b,m::strict(),true,false,zero_s0(h));
}
"""


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--all-a', action='store_true')
    parser.add_argument('--names', nargs='+', default=DEFAULT_NAMES)
    args = parser.parse_args()
    inventory = json.loads((ROOT/'evidence/m3-witness-status-0248.json').read_text())
    lines = [
        '//! Whole-walk/public-entry error witnesses from authored frozen fixtures.',
        '//! Exact errors come from frozen records, not an executable oracle.',
        'use vstd::prelude::*;',
        'use crate::{document_model as m,png::Error,alloc_guard::sites::TryError};',
        '', 'verus! {', MODEL_HELPERS,
    ]
    rows = []
    for scheduled in inventory['records']:
        if scheduled['category'] != 'a' or scheduled['complete_public_entry_witness']:
            continue
        name = scheduled['name']
        if not args.all_a and name not in args.names:
            continue
        path = Path(scheduled['source'])
        fixture = next(r for r in json.loads(path.read_text()) if r['name'] == name)
        assert 'pad_to' not in fixture
        data = bytes.fromhex(fixture['input_hex'])
        error = fixture['expected']['error']
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        ident = re.sub('[^a-zA-Z0-9]', '_', name).lower()
        values = ','.join(f'{x}u8' for x in data)
        frames = scheduled['entry'] == 'frames'
        model = 'MWF_p' if frames else 'MW_p'
        public = 'crate::png::Frames::try_new' if frames else 'crate::png::try_decode_still'
        facts, sites = read_facts(data, ident, error)
        header = (f'Ok((crate::model::Ihdr {{width:crate::model::be32(bytes_{ident}(),16),'
                  f'height:crate::model::be32(bytes_{ident}(),20),depth:{data[24]},'
                  f'color_type:{data[25]},interlace:{data[28]}}},33nat))')
        steps_before_error = {'container/duplicate-ihdr': 0, 'apng-bad/fctl-width-0': 2}.get(name)
        if steps_before_error is None:
            composition = [f'        reveal_with_fuel(m::drive,{sites+1});',
                           f'        assert(m::{model}(bytes_{ident}(),m::strict())==m::Outcome::Error(Error::{error},m::no_events()));']
        else:
            f = str(frames).lower()
            call = lambda i: f'm::step(bytes_{ident}(),m::strict(),{f},false,s{i})'
            composition = [f'        let ghost s0=m::initial(crate::model::mh(bytes_{ident}()).unwrap().0,33,m::no_events());']
            for i in range(steps_before_error):
                composition += [f'        let ghost s{i+1}=match {call(i)} {{m::Step::Next(next)=>next,_=>s{i}}};',
                                f'        assert({call(i)}==m::Step::Next(s{i+1}));',
                                f'        m::step_progress(bytes_{ident}(),m::strict(),{f},false,s{i});']
            composition += [f'        assert({call(steps_before_error)}==m::Step::Error(Error::{error},m::no_events()));']
            for i in reversed(range(steps_before_error+1)):
                composition += [f'        m::drive_step(bytes_{ident}(),m::strict(),{f},false,s{i});',
                                f'        assert(m::drive(bytes_{ident}(),m::strict(),{f},false,s{i})==m::Outcome::Error(Error::{error},m::no_events()));']
            composition += [f'        assert(m::{model}(bytes_{ident}(),m::strict())==m::Outcome::Error(Error::{error},m::no_events()));']
        if name == 'apng-bad/fctl-width-0':
            composition = [f'        late_zero_width_control(bytes_{ident}(),crate::model::mh(bytes_{ident}()).unwrap().0);']
        if name == 'plte/257-entries':
            composition = [f'        first_palette_error(bytes_{ident}(),crate::model::mh(bytes_{ident}()).unwrap().0,771,Error::InvalidPalette);']
        lines += [
            f'// {name.replace(chr(94), chr(95))}; fixture sha256 {digest}',
            f'spec fn bytes_{ident}() -> Seq<u8> {{seq![{values}]}}',
            f'proof fn model_{ident}()',
            f'    ensures m::{model}(bytes_{ident}(),m::strict())==m::Outcome::Error(Error::{error},m::no_events()),',
            '{',
            '    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);',
            f'        assert(crate::model::mh(bytes_{ident}())=={header}) by {{',
            '            reveal(crate::model::mh);',
            f'            assert(crate::model::mh(bytes_{ident}())=={header}) by (compute);',
            '        }',
            *facts,
            *([] if name in ['apng-bad/fctl-width-0','plte/257-entries'] else ['        reveal(m::step);reveal(m::ordered);reveal(m::own);']),
            *composition,
            '}',
            f'fn walk_{ident}() {{',
            f'    hide(m::{model});',
            f'    let b=[{values}];',
            f'    proof {{assert(b@==bytes_{ident}());model_{ident}();}}',
            f'    let public_result={public}(&b);',
            f'    assert(matches!(public_result,Err(TryError::Format(Error::{error}))));',
            '}', '',
        ]
        rows.append(dict(name=name, entry=scheduled['entry'], function='walk_'+ident,
                         source=str(path), source_sha256=digest,
                         input_sha256=hashlib.sha256(data).hexdigest(),
                         frozen_expected=fixture['expected'], category='a',
                         complete_public_entry_witness=True,
                         status='complete M3 category-a whole-model/public-entry error witness'))
    assert rows
    lines += ['} // verus!', '']
    (ROOT/'src/witness_m3_walk_frozen.rs').write_text('\n'.join(lines))
    (ROOT/'runs/m3-walk-witness-inputs.json').write_text(json.dumps(rows, indent=2)+'\n')
    print(f'Generated {len(rows)} whole-walk/public-error witnesses.')


if __name__ == '__main__':
    main()
