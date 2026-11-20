#!/usr/bin/env python3
"""Generate whole-walk/public-error witnesses from authored frozen fixtures.

The status inventory chooses records; expectations come from the fixture itself.
No decoder, policy oracle or third-party corpus supplies an expected result.
"""
import argparse
import ast
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




def limit_hint(name):
    """Read a central limit only to suggest a certificate; Verus checks it."""
    text = (ROOT/'src/limits.rs').read_text()
    expression = re.search(r'pub const '+name+r'\s*:[^=]+=(.*?);', text).group(1)
    def value(node):
        if isinstance(node, ast.Constant) and isinstance(node.value, int):
            return node.value
        assert isinstance(node, ast.BinOp)
        left, right = value(node.left), value(node.right)
        if isinstance(node.op, ast.LShift): return left << right
        if isinstance(node.op, ast.RShift): return left >> right
        if isinstance(node.op, ast.Add): return left + right
        if isinstance(node.op, ast.Sub): return left - right
        if isinstance(node.op, ast.Mult): return left * right
        raise AssertionError('Unsupported central-limit expression')
    return value(ast.parse(expression.strip(), mode='eval').body)


def apng_facts(data, ident):
    """Suggest pure local APNG transitions, never a public expected outcome."""
    width, height = int.from_bytes(data[16:20], 'big'), int.from_bytes(data[20:24], 'big')
    assert width < 32768 and height < 32768
    facts = [f'        assert(crate::model::be32(bytes_{ident}(),16)=={width}u32) by (compute);',
             f'        assert(crate::model::be32(bytes_{ident}(),20)=={height}u32) by (compute);']
    sequence, current, frames, before_flag, pixels = 0, 'None', 0, False, 0
    area_terms = []
    declared, plays, had_idat = None, 0, False
    def control():
        total = '+'.join(area_terms) or '0nat'
        return (f'crate::apng_model::Control {{sequence:{sequence}nat,current:crate::apng_model::Current::{current},'
                f'frame_count:{frames}nat,fctl_before_idat:{str(before_flag).lower()},pixel_total:{total}}}')
    pos = 33
    while pos+12 <= len(data):
        size = int.from_bytes(data[pos:pos+4], 'big')
        if pos+size+12 > len(data): break
        tag = data[pos+4:pos+8]
        payload = data[pos+8:pos+8+size]
        d = f'bytes_{ident}().subrange({pos+8},{pos+8+size})'
        before = control()
        before_idat = not had_idat
        error, structural, value = None, True, None
        if tag == b'acTL':
            if size != 8: error = 'InvalidChunkLength'
            else:
                count = int.from_bytes(payload[:4],'big')
                play = int.from_bytes(payload[4:8],'big')
                if count == 0: error = 'InvalidAnimationControl'
                elif count > limit_hint('MAX_FRAMES'):
                    error, structural = 'TooManyFrames', False
                else: declared, plays = count, play
            expr = f'crate::apng_model::actl({d})'
            value = (f'Err((Error::{error},{str(structural).lower()}))' if error else f'Ok(({declared}u32,{plays}u32))')
        elif tag == b'IDAT':
            if current != 'None': current = 'Default'
            had_idat = True
            expr, value = f'crate::apng_model::begin_idat({before})', control()
        elif tag == b'fcTL':
            expr = f'crate::apng_model::fctl({d},{str(declared is not None).lower()},{str(before_idat).lower()},{width}u32,{height}u32,{before})'
            if declared is None: error = 'MissingAnimationControl'
            elif before_idat and before_flag: error = 'ChunkOutOfOrder'
            elif size != 26: error = 'InvalidChunkLength'
            else:
                seq,w,h,x,y = [int.from_bytes(payload[i:i+4],'big') for i in [0,4,8,12,16]]
                delay_num, delay_den = int.from_bytes(payload[20:22],'big'), int.from_bytes(payload[22:24],'big')
                dispose, blend = payload[24], payload[25]
                if seq != sequence: error = 'SequenceError'
                elif current == 'Frame(false)': error = 'MissingFrameData'
                elif not w or not h or x+w > width or y+h > height or dispose > 2 or blend > 1: error = 'InvalidFrameControl'
                elif before_idat and (x or y or w != width or h != height): error = 'InvalidFrameControl'
                elif pixels+w*h > limit_hint('MAX_TOTAL_PIXELS'): error, structural = 'PixelBudgetExceeded', False
                else:
                    frame = (f'crate::apng_model::FrameControl {{width:{w}u32,height:{h}u32,x:{x}u32,y:{y}u32,'
                             f'delay_num:{delay_num}u16,delay_den:{delay_den}u16,dispose:{dispose}u8,blend:{blend}u8}}')
                    sequence += 1;current = 'Frame(false)';frames += 1;before_flag |= before_idat;pixels += w*h
                    area_terms.append(f'({w}nat*{h}nat)')
                    value = f'Ok(({frame},{control()}))'
            if error: value = f'Err((Error::{error},{str(structural).lower()}))'
        elif tag == b'fdAT':
            expr = f'crate::apng_model::fdat({d},{str(declared is not None).lower()},{str(before_idat).lower()},{before})'
            if declared is None: error = 'MissingAnimationControl'
            elif before_idat: error = 'ChunkOutOfOrder'
            elif size < 4: error = 'InvalidChunkLength'
            elif int.from_bytes(payload[:4],'big') != sequence: error = 'SequenceError'
            elif not current.startswith('Frame('): error = 'UnexpectedFrameData'
            else:
                sequence += 1;current = 'Frame(true)';value = f'Ok({control()})'
            if error: value = f'Err((Error::{error},true))'
        elif tag == b'IEND':
            count = 'None' if declared is None else f'Some({declared}u32)'
            expr = f'crate::apng_model::finish({before},{count})'
            error = 'MissingFrameData' if current == 'Frame(false)' else ('FrameCountMismatch' if declared is not None and frames != declared else None)
            value = f'Err((Error::{error},true))' if error else 'Ok(())'
        if value is not None:
            method = expr.split('crate::apng_model::',1)[1].split('(',1)[0]
            facts += [f'        assert({expr}=={value}) by {{',
                      f'            reveal(crate::apng_model::{method});',
                      f'            assert({expr}=={value}) by (compute);', '        }']
        pos += size+12
    return facts


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
            if tag in [b'iCCP',b'tEXt',b'zTXt',b'iTXt',b'sPLT']:
                nul = payload.find(0)
                expected_nul = 'None' if nul < 0 else f'Some({nul}nat)'
                facts += [f'        assert(crate::model::first_nul({payload_expr},0)=={expected_nul}) by (compute);']
                if nul >= 0:
                    key_bytes = payload[:nul]
                    key = ','.join(f'{x}u8' for x in key_bytes)
                    valid = (1 <= len(key_bytes) <= 79 and key_bytes[0] != 32 and key_bytes[-1] != 32
                             and all(32 <= x <= 126 or x >= 161 for x in key_bytes)
                             and b'  ' not in key_bytes)
                    facts += [f'        assert({payload_expr}.subrange(0,{nul})==seq![{key}]) by (compute);',
                              f'        assert({"" if valid else "!"}crate::model::keyword(seq![{key}]));']
                    if tag == b'iTXt':
                        start = nul+3
                        for _ in range(2):
                            end = payload.find(0,start)
                            expected_end = 'None' if end < 0 else f'Some({end}nat)'
                            facts += [f'        assert(crate::model::first_nul({payload_expr},{start}nat)=={expected_end}) by (compute);']
                            if end < 0:
                                break
                            start = end+1
            if tag in [b'cICP',b'eXIf',b'tIME',b'iTXt',b'bKGD',b'sBIT']:
                for index, value in enumerate(payload):
                    facts += [f'        assert({payload_expr}[{index}]=={value}u8) by (compute);']
            if tag == b'bKGD' and ct != 3:
                count = size//2
                facts += [f'        assert(crate::metadata_model::words16({payload_expr},{count}nat).len()=={count});']
                for index in range(count):
                    value = int.from_bytes(payload[2*index:2*index+2],'big')
                    facts += [f'        assert(crate::model::be16({payload_expr},{2*index})=={value}u16) by (compute);',
                              f'        assert(crate::metadata_model::words16({payload_expr},{count}nat)[{index}]=={value}u16);']
            if tag in [b'cICP',b'eXIf',b'tIME'] and frozen_error in ['InvalidCicp','InvalidExif','InvalidTime']:
                expression = (f'crate::metadata_model::parse(crate::walk_model::Name::{NAMES[tag]},{payload_expr},'
                              f'{ct}u8,{depth}u8,{entries}nat)==crate::metadata_model::Outcome::Error(Error::{frozen_error})')
                facts += [f'        assert({expression}) by (compute);']
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

#[verifier::spinoff_prover]
proof fn invalid_iccp_keyword(d: Seq<u8>,ct: u8,depth: u8,z: nat)
    requires crate::model::valid_depth(ct,depth),crate::model::first_nul(d,0)==Some(z),
        !crate::model::keyword(d.subrange(0,z as int)),
    ensures crate::metadata_model::parse(crate::walk_model::Name::Iccp,d,ct,depth,0)==crate::metadata_model::Outcome::Error(Error::InvalidIccp),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn first_iccp_error(b: Seq<u8>,h: crate::model::Ihdr,length: nat)
    requires 41+length<=b.len(),crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![105u8,67u8,67u8,80u8],data:41,len:length,next:45+length,crc_ok:true,synthetic:false}),
        crate::metadata_model::parse(crate::walk_model::Name::Iccp,b.subrange(41,(41+length) as int),h.color_type,h.depth,0)==crate::metadata_model::Outcome::Error(Error::InvalidIccp),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::InvalidIccp,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::metadata_model::parse);hide(m::drive);hide(m::initial_info);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let s=m::initial(h,33,m::no_events());
    assert(m::step(b,m::strict(),false,false,s)==m::Step::Error(Error::InvalidIccp,m::no_events()));
    m::drive_step(b,m::strict(),false,false,s);
}
#[verifier::spinoff_prover]
proof fn idat_then_iccp_order(b: Seq<u8>,h: crate::model::Ihdr,idat_len: nat,iccp_len: nat)
    requires 57+idat_len+iccp_len<=b.len(),h.color_type!=3,crate::model::mh(b)==Ok((h,33nat)),
        crate::critical_model::read(b,33,crate::walk_model::Phase::Before,false)==Ok(crate::critical_model::Candidate {
            ty:seq![73u8,68u8,65u8,84u8],data:41,len:idat_len,next:45+idat_len,crc_ok:true,synthetic:false}),
        crate::critical_model::read(b,45+idat_len,crate::walk_model::Phase::During,false)==Ok(crate::critical_model::Candidate {
            ty:seq![105u8,67u8,67u8,80u8],data:53+idat_len,len:iccp_len,next:57+idat_len+iccp_len,crc_ok:true,synthetic:false}),
    ensures m::MW_p(b,m::strict())==m::Outcome::Error(Error::ChunkOutOfOrder,m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);
    let s0=m::initial(h,33,m::no_events());
    let s1=m::State {pos:45+idat_len,phase:crate::walk_model::Phase::During,
        control:crate::apng_model::begin_idat(s0.control),
        doc:m::Doc {idat:s0.doc.idat.push(crate::walk_model::Range {offset:41,len:idat_len}),..s0.doc},..s0};
    assert(m::step(b,m::strict(),false,false,s0)==m::Step::Next(s1)) by {
        reveal(m::step);reveal(m::ordered);reveal(m::own);
    }
    assert(m::step(b,m::strict(),false,false,s1)==m::Step::Error(Error::ChunkOutOfOrder,m::no_events())) by {
        reveal(m::step);reveal(m::ordered);reveal(m::own);
    }
    m::drive_step(b,m::strict(),false,false,s1);
    m::drive_step(b,m::strict(),false,false,s0);
}

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
        if name.startswith('apng-bad/'):
            facts += apng_facts(data, ident)
        header = (f'Ok((crate::model::Ihdr {{width:crate::model::be32(bytes_{ident}(),16),'
                  f'height:crate::model::be32(bytes_{ident}(),20),depth:{data[24]},'
                  f'color_type:{data[25]},interlace:{data[28]}}},33nat))')
        steps_before_error = {
            'container/duplicate-ihdr':0,'apng-bad/fctl-width-0':2,
            'apng-bad/fewer-frames-than-declared':5,'apng-bad/more-frames-than-declared':5,
            'apng-bad/seq-fdat-repeats':5,'apng-bad/fdat-for-idat-frame':3,
            'apng-bad/fdat-for-idat-frame-still':3,'apng-bad/fdat-len-3':4,
            'apng-bad/fctl-no-fdat-then-iend':4,'apng-bad/fctl-no-fdat-then-fctl':4,
        }.get(name)
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
        if name in ['anc-bad/iccp-name-80','anc-bad/iccp-leading-space']:
            length = int.from_bytes(data[33:37],'big')
            nul = data[41:41+length].index(0)
            composition = [f'        invalid_iccp_keyword(bytes_{ident}().subrange(41,{41+length}),{data[25]}u8,{data[24]}u8,{nul}nat);',
                           f'        first_iccp_error(bytes_{ident}(),crate::model::mh(bytes_{ident}()).unwrap().0,{length}nat);']
        if name == 'order/iCCP-after-idat':
            composition = [f'        idat_then_iccp_order(bytes_{ident}(),crate::model::mh(bytes_{ident}()).unwrap().0,22,287);']

        control_hides = '    '+''.join(f'hide(crate::apng_model::{method});' for method in ['actl','fctl','fdat','finish']
                                      if any(f'crate::apng_model::{method}(' in line for line in facts))
        symbolic = name.startswith('apng-bad/') and name != 'apng-bad/fctl-width-0'
        if symbolic:
            requirements = []
            pure = []
            i = 0
            while i < len(facts):
                line = facts[i]
                if line.startswith('        assert forall|'):
                    expression = line.strip()[len('assert '):-len(' by {')]
                    requirements.append('('+expression.replace(f'bytes_{ident}()', 'b')+')')
                    i += 1
                    while facts[i] != '        }': i += 1
                elif line.startswith('        assert('):
                    expression = line[len('        assert('):]
                    expression = expression.rsplit(') by ',1)[0] if ') by ' in expression else expression[:-2]
                    if f'bytes_{ident}()' in expression:
                        requirements.append(expression.replace(f'bytes_{ident}()', 'b'))
                        if line.endswith(' by {'):
                            i += 1
                            while facts[i] != '        }': i += 1
                    else:
                        pure.append(line)
                        if line.endswith(' by {'):
                            i += 1
                            while facts[i] != '        }':
                                pure.append(facts[i]);i += 1
                            pure.append(facts[i])
                else:
                    raise AssertionError(line)
                i += 1
            lines += ['#[verifier::spinoff_prover]',f'proof fn symbolic_{ident}(b: Seq<u8>)',
                      f'    requires b.len()=={len(data)},crate::model::mh(b)=={header.replace(f"bytes_{ident}()", "b")},',
                      *[f'        {r},' for r in requirements],
                      f'    ensures m::{model}(b,m::strict())==m::Outcome::Error(Error::{error},m::no_events()),',
                      '{','    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);',
                      control_hides,*pure,'    reveal(m::step);reveal(m::ordered);reveal(m::own);',
                      *[r.replace(f'bytes_{ident}()', 'b') for r in composition],'}','']
            composition = [f'        symbolic_{ident}(bytes_{ident}());']
        lines += [
            f'// {name.replace(chr(94), chr(95))}; fixture sha256 {digest}',
            f'spec fn bytes_{ident}() -> Seq<u8> {{seq![{values}]}}',
            *(['#[verifier::spinoff_prover]'] if name == 'container/non-consecutive-idat' or name.startswith('apng-bad/') else []),
            f'proof fn model_{ident}()',
            f'    ensures m::{model}(bytes_{ident}(),m::strict())==m::Outcome::Error(Error::{error},m::no_events()),',
            '{',
            '    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);',
            *([control_hides] if name.startswith('apng-bad/') else []),
            *([f'    hide(m::{model});'] if symbolic else []),
            f'        assert(crate::model::mh(bytes_{ident}())=={header}) by {{',
            '            reveal(crate::model::mh);',
            f'            assert(crate::model::mh(bytes_{ident}())=={header}) by (compute);',
            '        }',
            *facts,
            *([] if symbolic or name in ['apng-bad/fctl-width-0','plte/257-entries','anc-bad/iccp-name-80','anc-bad/iccp-leading-space','order/iCCP-after-idat'] else ['        reveal(m::step);reveal(m::ordered);reveal(m::own);']),
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
