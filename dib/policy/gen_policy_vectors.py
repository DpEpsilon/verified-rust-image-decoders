#!/usr/bin/env python3
"""Generate reproducible per-profile byte vectors with independent option witnesses."""
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE))
import policy_oracle as oracle
sys.path.insert(0,'/home/ws/projects/leniency/bmp')
import harness

source={v['name']:v for v in json.loads((HERE/'STRICT-VECTORS.json').read_text())}
cases=[]
def add(name,b,mode='BmpFile'):
    if len(b)>8192:return
    cases.append((name,bytes(b),mode))
def base(name):
    v=source[name]
    b=bytes.fromhex(v['input_hex'])
    return b if v['mode']=='BmpFile' else harness.wrap_dib(b)
def field(name,origin,at,size,value):
    b=bytearray(base(origin));b[at:at+size]=value.to_bytes(size,'little');add(name,b)

for n in ('file-size-mismatch','file-reserved','file-offset-small','file-offset-before-palette',
          'header-16-BareDib','palette-short-invalid-index-4','masks-zero','masks-overlap',
          'masks-40-16','masks-108-32',
          'masks-hole','fields-16-alpha-nonzero','palette-truncated-8',
          'bi-rgb-16-630c','bi-rgb-32-11223300','rle8-early-end',
          'rle8-delta-bounds','rle8-missing-end','rle8-last-eol-no-eob',
          'rle8-run-over-row','rle8-eol-past-height','bmp-indexed-gap-4',
          'bmp-fields-40-masks-then-optimal-table-gap','rle8-index-invalid'):
    add('base-'+n,base(n))
field('bfSize-greater-than-slice','bi-rgb-32-11223300',2,4,1000)
field('bfSize-zero','bi-rgb-32-11223300',2,4,0)
field('bfOffBits-zero','bi-rgb-32-11223300',10,4,0)
field('bfOffBits-before-palette','bmp-indexed-gap-4',10,4,54)
field('biSizeImage-wrong','bi-rgb-32-11223300',34,4,1)
field('planes-zero','bi-rgb-32-11223300',26,2,0)
field('planes-two','bi-rgb-32-11223300',26,2,2)
field('indexed-count-one','bmp-indexed-gap-4',46,4,1)
field('indexed-count-too-large','bmp-indexed-gap-4',46,4,17)
field('high-count-one','bi-rgb-32-11223300',46,4,1)
field('important-huge','bi-rgb-32-11223300',50,4,0xffffffff)
field('important-indexed-huge','bmp-indexed-gap-4',50,4,0xffffffff)
field('important-indexed-zero','bmp-indexed-gap-4',50,4,0)
field('unsupported-depth-two','bi-rgb-32-11223300',28,2,2)
field('unsupported-compression-six','bi-rgb-32-11223300',30,4,6)
field('alpha-masks-or-default','masks-56-32',54,4,0)
field('signature-invalid','bi-rgb-32-11223300',0,2,0x4d41)
field('reserved-nonzero','bi-rgb-32-11223300',6,2,1)
add('short-last-row',base('bi-rgb-32-11223300')[:-1])
add('short-first-row',base('bi-rgb-32-11223300')[:-4])
alpha_byte=bytearray(base('bi-rgb-32-11223300'))
alpha_byte[int.from_bytes(alpha_byte[10:14],'little')+3]=0x7f
add('alpha32-stored-nonzero',alpha_byte)
add('rle-early-eob-gap',base('rle8-early-end'))
add('rle-terminal-eol',base('rle8-last-eol-no-eob'))
add('trailing-bmp-bytes',base('bi-rgb-32-11223300')+b'END!')
add('bare-exact-tail',bytes.fromhex(source['bi-rgb-32-11223300']['input_hex']),'BareDib')
add('bare-with-tail',bytes.fromhex(source['bi-rgb-32-11223300']['input_hex'])+b'END!','BareDib')
# A palette lookup beyond biClrUsed and a first-byte gap use different inputs.
field('gap-offset-at-header','bmp-indexed-gap-4',10,4,58)
field('image-size-nonzero-equal','bi-rgb-32-11223300',34,4,4)
field('parsed-fields-past-eof-empty-table','bi-rgb-32-11223300',14,4,124)
field('header-64-rejected-by-image','bi-rgb-32-11223300',14,4,64)
bare_high=bytearray(bytes.fromhex(source['bi-rgb-32-11223300']['input_hex']))
bare_high[32:36]=(1).to_bytes(4,'little')
add('bare-high-count-all-profiles',bare_high,'BareDib')
bare_gdk_bound=bytearray(bare_high)
bare_gdk_bound[14:16]=(24).to_bytes(2,'little')
add('bare-gdk-bound-positive-count-24',bare_gdk_bound,'BareDib')
# ParsedFields requires no availability test when the optional high-depth table is empty.
# With one declared table entry the same out-of-slice span must reject.
field('parsed-fields-past-eof-nonempty-table','bi-rgb-32-11223300',14,4,124)
partial_table=bytearray(base('bi-rgb-32-11223300'))
partial_table[14:18]=(124).to_bytes(4,'little')
partial_table[28:30]=(8).to_bytes(2,'little')
partial_table[46:50]=(1).to_bytes(4,'little')
add('parsed-fields-table-required',partial_table)
short24=bytearray(base('bi-rgb-32-11223300')[:54]+b'\x00\x00\xff\x00\xff\x00\x00\x00')
short24[2:6]=len(short24).to_bytes(4,'little')
short24[18:22]=(2).to_bytes(4,'little')
short24[22:26]=(2).to_bytes(4,'little')
short24[28:30]=(24).to_bytes(2,'little')
short24[34:38]=(16).to_bytes(4,'little')
add('short-raster-logical-image-size',short24)
short24[34:38]=(8).to_bytes(4,'little')
add('short-raster-physical-image-size',short24)
field('rgb2-header16-unsupported','bi-rgb-32-11223300',14,4,16)
rgb2_h16=bytearray(base('bi-rgb-32-11223300'))
rgb2_h16[14:18]=(16).to_bytes(4,'little')
rgb2_h16[28:30]=(2).to_bytes(2,'little')
add('rgb2-header16-combination',rgb2_h16)
add('bare-incomplete-indexed-palette',bytes.fromhex(source['palette-truncated-8']['input_hex']),'BareDib')
wide_mask=bytearray(base('masks-40-16'))
wide_mask[54:58]=(0x03ff).to_bytes(4,'little')
wide_mask[58:62]=(0x1c00).to_bytes(4,'little')
wide_mask[62:66]=(0xe000).to_bytes(4,'little')
wide_mask[66:68]=(3).to_bytes(2,'little')
add('wide-ten-bit-mask-image-scaling',wide_mask)
red_palette0=bytearray(base('rle8-early-end'))
red_palette0[54:58]=b'\x00\x00\xff\x00'
add('rle-early-end-nonblack-palette-zero',red_palette0)
short_mask16=bytearray(base('masks-40-16')[:60])
short_mask16[2:6]=len(short_mask16).to_bytes(4,'little')
short_mask16[10:14]=(54).to_bytes(4,'little')
add('incomplete-external-masks-16',short_mask16)
short_mask32=bytearray(short_mask16)
short_mask32[28:30]=(32).to_bytes(2,'little')
add('incomplete-external-masks-32-alpha',short_mask32)
short_palette32=bytearray(base('bi-rgb-32-11223300'))
short_palette32[46:50]=(2).to_bytes(4,'little')
add('incomplete-high-depth-optional-table',short_palette32)
short_rle_gap=bytearray(base('rle8-early-end'))
short_rle_gap[46:50]=(1).to_bytes(4,'little')
add('rle-short-indexed-gap-transparent',short_rle_gap)
add('rle-one-byte-before-command',base('rle8-missing-end')+b'\x00')
add('rle4-encoded-row-overrun',base('rle4-run-over-row'))
add('rle4-absolute-row-overrun',base('rle4-absolute-row-overrun-precedes-short'))
complete_rle=bytes.fromhex('424d44000000000000003e000000280000000200000001000000010008000100000000000000000000000000000002000000000000000000ff00ff000000020000000001')
add('complete-rle8-eol-and-eob-no-recovery',complete_rle)
add('complete-rle8-eob-no-recovery',bytes.fromhex('424d42000000000000003e000000280000000200000001000000010008000100000000000000000000000000000002000000000000000000ff00ff00000002000001'))
indexed_at_end=bytearray(base('bmp-indexed-gap-4'))
indexed_at_end[10:14]=len(indexed_at_end).to_bytes(4,'little')
add('gdk-indexed-offset-at-eof',indexed_at_end)
indexed_below=bytearray(base('bmp-indexed-gap-4'))
indexed_below[10:14]=(54).to_bytes(4,'little')
add('gdk-indexed-offset-below-palette',indexed_below)
indexed_past=bytearray(base('bmp-indexed-gap-4'))
indexed_past[10:14]=(len(indexed_past)+1).to_bytes(4,'little')
add('gdk-indexed-offset-past-eof',indexed_past)
rgb_past=bytearray(base('bi-rgb-32-11223300'))
rgb_past[10:14]=(len(rgb_past)+1).to_bytes(4,'little')
add('gdk-rgb-offset-past-eof',rgb_past)
bitfield_gap=bytearray(base('masks-40-16'))
bitfield_gap[66:66]=b'\x11\x22'
bitfield_gap[2:6]=len(bitfield_gap).to_bytes(4,'little')
bitfield_gap[10:14]=(68).to_bytes(4,'little')
add('gdk-bitfield-declared-offset-with-gap',bitfield_gap)
threshold16=bytearray(base('masks-40-16'))
threshold16[46:50]=(65536).to_bytes(4,'little')
add('gdk-bound-equal-threshold-16',threshold16)
indexed_rle_bad=bytearray(base('rle8-early-end'))
indexed_rle_bad[10:14]=(54).to_bytes(4,'little')
add('gdk-rle-indexed-invalid-offset-blank-alpha',indexed_rle_bad)
incomplete_rle_palette=bytearray(bytes.fromhex(source['palette-truncated-8']['input_hex']))
incomplete_rle_palette[16:20]=(1).to_bytes(4,'little')
add('bare-rle-incomplete-palette-blank-alpha',incomplete_rle_palette,'BareDib')
wide16=bytearray(base('masks-40-16'))
wide16[66:68]=(0x0400).to_bytes(2,'little')
wide16[54:58]=(0).to_bytes(4,'little')
add('gdk-16-default-555-versus-565',wide16)
overflow_mask=bytearray(base('masks-40-16'))
overflow_mask[54:58]=(0x10000).to_bytes(4,'little')
add('gdk-nonzero-mask-above-depth',overflow_mask)
invalid_alpha=bytearray(base('masks-56-32'))
invalid_alpha[66:70]=(0x0a000000).to_bytes(4,'little')
add('independent-alpha-mask-noncontiguous',invalid_alpha)
bare_short=bytearray(short24[14:])
bare_short[20:24]=(16).to_bytes(4,'little')
add('bare-short-logical-cursor',bare_short,'BareDib')
for name,stream in (('clamped-x-width',b'\x00\x02\xff\x00\x01\x01\x00\x01'),
                    ('clamped-y-height',b'\x00\x02\x00\xff\x01\x01\x00\x01')):
    b=bytearray(base('rle8-delta-bounds')[:70]+stream)
    b[2:6]=len(b).to_bytes(4,'little')
    add(name,b)
clamped_eob=bytearray(base('rle8-delta-bounds')[:70]+b'\x00\x02\x00\x05\x00\x01')
clamped_eob[2:6]=len(clamped_eob).to_bytes(4,'little')
clamped_eob[34:38]=(6).to_bytes(4,'little')
add('clamped-height-then-explicit-eob',clamped_eob)
equal_delta=bytearray(base('rle8-early-end')[:70]+b'\x02\x00\x00\x02\x00\x01\x00\x01')
equal_delta[2:6]=len(equal_delta).to_bytes(4,'little')
add('clamp-delta-exact-height-no-recovery',equal_delta)
internal_short=bytearray(base('masks-56-32')[:58])
internal_short[14:18]=(52).to_bytes(4,'little')
internal_short[2:6]=len(internal_short).to_bytes(4,'little')
add('parsed-internal-mask-short-even-with-blank',internal_short)
rgb2_compressed=bytearray(base('bi-rgb-32-11223300'))
rgb2_compressed[28:30]=(2).to_bytes(2,'little')
rgb2_compressed[30:34]=(1).to_bytes(4,'little')
add('rgb2-non-rgb-compression-first-error',rgb2_compressed)
rgb2_unknown=bytearray(rgb2_compressed)
rgb2_unknown[30:34]=(5).to_bytes(4,'little')
add('rgb2-unknown-compression-first-error',rgb2_unknown)
absolute_clip=bytearray(base('rle8-absolute-even'))
absolute_clip[18:22]=(2).to_bytes(4,'little')
add('rle8-complete-absolute-run-clipped',absolute_clip)
bitfield_bad_offset=bytearray(base('masks-40-16'))
bitfield_bad_offset[10:14]=(0).to_bytes(4,'little')
add('gdk-bitfield-invalid-offset-metadata-start',bitfield_bad_offset)
for filename in ('01-planes.bmp','02-biSizeImage.bmp','08-bitfield-mask.bmp',
                 '09-bfSize.bmp','pixel-01-frozen-BareDib.bmp',
                 'pixel-02-frozen-BmpFile.bmp','pixel-03-rle-escape.bmp'):
    add('matrix-'+filename,Path('/home/ws/projects/leniency/bmp/witnesses',filename).read_bytes())

result={}
for profile in oracle.PROFILES:
    rows=[]
    for name,b,mode in cases:
        row={'name':name,'mode':mode,'input_hex':b.hex(),
             'expected':oracle.actual(b,mode,profile,with_recovery=True)}
        oracle.check_vector(row,profile)
        rows.append(row)
    result[profile]=rows
# This closed event catalog is deliberately independent of the generic
# per-profile grid: each positive and negative flag expectation is explicit.
events={
    'offset-blank':('GdkCompat-2.42.12','gdk-indexed-offset-below-palette'),
    'offset-bitfield-repair':('GdkCompat-2.42.12','gdk-bitfield-invalid-offset-metadata-start'),
    'bitfield-metadata-end-differs-from-valid-offset':('GdkCompat-2.42.12','gdk-bitfield-declared-offset-with-gap'),
    'partial-header':('ImageCrateCompat-0.25.10','parsed-fields-past-eof-empty-table'),
    'blank-metadata':('GdkCompat-2.42.12','incomplete-external-masks-32-alpha'),
    'blank-indexed-gap':('GdkCompat-2.42.12','rle-short-indexed-gap-transparent'),
    'mask-default':('GdkCompat-2.42.12','base-masks-zero'),
    'missing-row':('GdkCompat-2.42.12','short-last-row'),
    'palette-substitution':('ImageCrateCompat-0.25.10','base-palette-short-invalid-index-4'),
    'encoded-clip':('GdkCompat-2.42.12','base-rle8-run-over-row'),
    'absolute-clip':('GdkCompat-2.42.12','rle8-complete-absolute-run-clipped'),
    'delta-clamp':('GdkCompat-2.42.12','clamped-y-height'),
    'early-end-fill':('ImageCrateCompat-0.25.10','base-rle8-early-end'),
}
for event,(profile,name) in events.items():
    expected=next(r['expected'] for r in result[profile] if r['name']==name)
    assert expected.get('recovered') is True,(event,expected)
for profile,rows in result.items():
    expected=next(r['expected'] for r in rows if r['name']=='complete-rle8-eol-and-eob-no-recovery')
    assert expected.get('recovered') is False,(profile,expected)
expected=next(r['expected'] for r in result['GdkCompat-2.42.12'] if r['name']=='clamp-delta-exact-height-no-recovery')
assert expected.get('recovered') is False,expected
for name in ('bfSize-greater-than-slice','biSizeImage-wrong'):
    expected=next(r['expected'] for r in result['Recommended'] if r['name']==name)
    assert expected.get('recovered') is False,(name,expected)
(HERE/'POLICY-VECTORS.json').write_text(json.dumps(result,indent=2)+'\n')
anchors={'file_size':'bfSize-greater-than-slice','offset':'bfOffBits-zero',
         'image_size':'biSizeImage-wrong','planes':'planes-zero',
         'indexed_count':'indexed-count-too-large','high_count':'high-count-one',
         'important':'important-indexed-huge','short_pixels':'short-last-row',
         'rle_eol':'rle-terminal-eol','rle_overrun':'base-rle8-run-over-row',
         'rle_delta':'base-rle8-delta-bounds','rle_eob':'base-rle8-missing-end',
         'rle_fill':'rle-early-eob-gap','palette_index':'base-palette-short-invalid-index-4',
         'masks':'base-masks-zero','scaling':'base-bi-rgb-16-630c',
         'alpha32':'alpha32-stored-nonzero','headers':'base-header-16-BareDib',
         'unsupported':'unsupported-depth-two','signature':'signature-invalid',
         'reserved':'reserved-nonzero','palette_gap':'base-bmp-indexed-gap-4',
         'bitfield_offset':'base-masks-40-16',
         'bitfield16_alpha':'base-fields-16-alpha-nonzero',
         'incomplete_metadata':'base-palette-truncated-8',
         'short_indexed_gap':'base-bmp-indexed-gap-4',
         'header_bytes':'base-masks-108-32'}
lookup={name:(b,mode) for name,b,mode in cases}
custom=[]
undiscriminated=[]
for key,values in oracle.OPTIONS.items():
    for value in values:
        policy={**oracle.STRICT,key:value}
        chosen=next(((name,b,mode) for name,b,mode in cases
                     if mode in ('BmpFile','BareDib') and
                     oracle.actual(b,mode,policy)!=oracle.actual(b,mode,oracle.STRICT)),None)
        if chosen is None:
            selected=anchors[key]
            b,mode=lookup[selected]
            if value!=oracle.STRICT[key]:undiscriminated.append((key,value))
        else:selected,b,mode=chosen
        row={'name':key+'-'+value+'-'+selected,'mode':mode,'input_hex':b.hex(),
             'policy':policy,'expected':oracle.actual(b,mode,policy,with_recovery=True)}
        oracle.check_vector(row,policy)
        custom.append(row)
# Coupled fields require their own witnesses; a one-field witness cannot pin
# image-size precedence after a filled row or EOL after a clamped delta.
for name,changes in (
    ('short-raster-logical-image-size',{'short_pixels':'ZeroFill'}),
    ('short-raster-physical-image-size',{'short_pixels':'ZeroFill'}),
    ('rgb2-header16-combination',{'unsupported':'Rgb2'}),
    ('incomplete-high-depth-optional-table',{'offset':'AllowAny','incomplete_metadata':'Blank'}),
    ('gdk-indexed-offset-at-eof',{'offset':'Gdk','short_pixels':'Reject'}),
    ('gdk-indexed-offset-past-eof',{'offset':'Gdk','short_pixels':'Reject'}),
    ('gdk-bitfield-declared-offset-with-gap',{'offset':'Gdk','bitfield_offset':'DeclaredOffset'}),
    ('gdk-bitfield-invalid-offset-metadata-start',{'offset':'Gdk','bitfield_offset':'DeclaredOffset'}),
    ('rgb2-unknown-compression-first-error',{'unsupported':'Rgb2'}),
    ('bare-high-count-all-profiles',{'high_count':'GdkBound'}),
    ('bare-gdk-bound-positive-count-24',{'high_count':'GdkBound'}),
    ('bare-short-logical-cursor',{'short_pixels':'ZeroFill'}),
    ('parsed-internal-mask-short-even-with-blank',{'offset':'AllowAny','header_bytes':'ParsedFields','incomplete_metadata':'Blank'}),
    ('clamped-height-then-explicit-eob',{'rle_delta':'ClampStop'}),
):
    b,mode=lookup[name];policy={**oracle.STRICT,**changes}
    row={'name':'coupled-'+name,'mode':mode,'input_hex':b.hex(),
         'policy':policy,'expected':oracle.actual(b,mode,policy,with_recovery=True)}
    oracle.check_vector(row,policy);custom.append(row)
clamped_eol=bytearray(base('rle8-delta-bounds')[:70]+b'\x00\x02\x00\x05\x00\x00\x00\x01')
clamped_eol[2:6]=len(clamped_eol).to_bytes(4,'little')
policy={**oracle.STRICT,'rle_delta':'ClampStop','rle_eol':'StopAtHeight'}
row={'name':'coupled-clamped-delta-eol-at-height','mode':'BmpFile',
     'input_hex':clamped_eol.hex(),'policy':policy,
     'expected':oracle.actual(clamped_eol,'BmpFile',policy,with_recovery=True)}
custom.append(row);oracle.check_vector(row,policy)
(HERE/'CUSTOM-VECTORS.json').write_text(json.dumps(custom,indent=2)+'\n')
print('generated',len(cases),'vectors per profile,',len(custom),'internal probes; non-discriminating:',undiscriminated)
