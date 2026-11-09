#!/usr/bin/env python3
"""Independent bounded BMP/DIB policy oracle, based on the 1.1 vector oracle.

A compatibility profile is a *byte semantics proposal*, not a wrapper for a
native decoder. Library-specific I/O, sniffing and allocator effects are excluded.
"""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
MAX_INPUT = MAX_OUTPUT = 67_108_864
MAX_DIMENSION = 8192
ERRORS = set('FileTooLarge TruncatedFileHeader BadMagic InvalidFileSize InvalidPixelOffset TruncatedDibHeader UnsupportedHeaderSize TruncatedHeader InvalidPlanes InvalidDimensions UnsupportedPixelDepth UnsupportedCompression InvalidCompressionDepth UnsupportedOrientation InvalidColorCount OutputTooLarge TruncatedMasks InvalidBitfields TruncatedPalette TruncatedPixels PaletteIndexOutOfRange RleTruncated RleBounds InvalidImageSize TruncatedAndMask TrailingBytes'.split())

# Each dimension is independently selectable. Profiles apply to BMP and BareDib;
# ICO retains the frozen semantics until its own policy is reviewed.
STRICT = dict(file_size='AtMostFileLength', offset='Declared', image_size='Exact',
              planes='Must1', high_count='BmpTable',
              short_pixels='Reject', rle_eol='OnlyFinal',
              rle_overrun='Reject', rle_delta='Inside', rle_eob='Required',
              rle_fill='Palette0', palette_index='Reject', masks='Reject',
              scaling='Round', headers='All1_1',
              unsupported='Reject', reserved='Zero',
              bitfield_offset='DeclaredOffset',
              bitfield16_alpha='Mask',incomplete_metadata='Reject',
              short_indexed_gap='Decode',header_bytes='Full')
GDK = dict(STRICT, file_size='Ignore', offset='Gdk', image_size='Ignore',
           planes='Ignore', high_count='GdkBound',
           short_pixels='ZeroFill', rle_eol='StopAtHeight',
           rle_overrun='Clip', rle_delta='ClampStop', rle_eob='AtInputEnd',
           rle_fill='TransparentBlack', palette_index='Black',
           masks='DefaultRGB', scaling='Gdk',
           headers='No16', reserved='Ignore',
           bitfield_offset='MetadataEnd',bitfield16_alpha='Opaque',
           incomplete_metadata='Blank',short_indexed_gap='Blank')
IMAGE = dict(STRICT, file_size='Ignore', offset='AllowAny', image_size='Ignore',
             high_count='Ignore', rle_eol='StopAtHeight', rle_overrun='Image',
             rle_fill='OpaqueBlack', headers='No16No64', reserved='Ignore',
             masks='Independent',palette_index='Black',unsupported='Rgb2',
             scaling='Image',header_bytes='ParsedFields')
RECOMMENDED = dict(STRICT, file_size='Ignore', image_size='Ignore')
PROFILES = {'Strict': STRICT, 'GdkCompat-2.42.12': GDK,
            'ImageCrateCompat-0.25.10': IMAGE, 'Recommended': RECOMMENDED}
OPTIONS = {
    'file_size': ('AtMostFileLength','Ignore'),
    'offset': ('Declared','AllowAny','Gdk'), 'image_size': ('Exact','Ignore'),
    'planes': ('Must1','Ignore'),
    'high_count': ('BmpTable','Ignore','GdkBound'),
    'short_pixels': ('Reject','ZeroFill'), 'rle_eol': ('OnlyFinal','StopAtHeight'),
    'rle_overrun': ('Reject','Clip','Image'), 'rle_delta': ('Inside','ClampStop'),
    'rle_eob': ('Required','AtInputEnd'),
    'rle_fill': ('Palette0','OpaqueBlack','TransparentBlack'),
    'palette_index': ('Reject','Black'),
    'masks': ('Reject','DefaultRGB','Independent'),
    'scaling': ('Round','Image','Gdk'),
    'headers': ('All1_1','No16','No16No64'), 'unsupported': ('Reject','Rgb2'),
    'reserved': ('Zero','Ignore'),
    'bitfield_offset': ('DeclaredOffset','MetadataEnd'),
    'bitfield16_alpha': ('Mask','Opaque'),
    'incomplete_metadata': ('Reject','Blank'),
    'short_indexed_gap': ('Decode','Blank'),
    'header_bytes': ('Full','ParsedFields'),
}
# Public entry points accept only named profiles. This switch controls internal
# custom-record experiments; turning it off leaves every named profile intact.
EXPERIMENTAL_OPTIONS = True

class Invalid(Exception):
    pass

def fail(name):
    raise Invalid(name)

def u16(b,p): return int.from_bytes(b[p:p+2], 'little')
def u32(b,p): return int.from_bytes(b[p:p+4], 'little')
def s32(b,p): return int.from_bytes(b[p:p+4], 'little', signed=True)
def stride(width,bits): return ((width*bits+31)//32)*4

def masks_valid(masks,bits):
    used=0
    for i,m in enumerate(masks):
        if i<3 and not m: return False
        if m>>bits or m&used: return False
        if m:
            compact=m//(m&-m)
            if compact & (compact+1): return False
        used|=m
    return True

def channel(word,mask,scaling='Round',missing=255):
    if not mask: return missing
    shift=(mask&-mask).bit_length()-1
    if scaling=='Gdk':
        n=min(mask.bit_count(),8)
        if mask.bit_count()>8:shift+=mask.bit_count()-8
        value=(word&mask)>>shift
        return ((value<<(8-n)) | (value>>(2*n-8) if n>=4 else 0))&255
    n=(mask>>shift).bit_length()
    value=(word&mask)>>shift
    if scaling=='Image' and n>8:return (value>>(n-8))&255
    return (value*255+((1<<n)-1)//2)//((1<<n)-1)

def _decode(b, mode, policy):
    """Internal pure-byte model; return (width,height,rgba,recovered)."""
    if len(b)>MAX_INPUT: fail('FileTooLarge')
    if mode not in ('BmpFile','BareDib','IcoEntryDib','IcoEntryDibSlack'):
        raise ValueError('invalid typed mode')
    if set(policy)!=set(OPTIONS) or any(policy[k] not in OPTIONS[k] for k in OPTIONS):
        raise ValueError('unknown internal policy record')
    ico=mode in ('IcoEntryDib','IcoEntryDibSlack')
    if ico and policy != STRICT:
        raise ValueError('ICO modes have no policy entry point')
    # The retained option space is internal to the oracle, not a public constructor.
    # Clearing this switch disables experimental single-field combinations.
    if not EXPERIMENTAL_OPTIONS and policy not in PROFILES.values():
        raise ValueError('experimental option records disabled')
    recovered=False
    start=0;offset=None
    if mode=='BmpFile':
        if len(b)<14: fail('TruncatedFileHeader')
        if b[:2]!=b'BM': fail('BadMagic')
        declared=u32(b,2)
        if (policy['file_size']=='AtMostFileLength' and declared>len(b) and declared!=0) or (policy['reserved']=='Zero' and (u16(b,6) or u16(b,8))): fail('InvalidFileSize')
        offset=u32(b,10)
        if policy['offset']=='Declared' and (offset<18 or offset>len(b)): fail('InvalidPixelOffset')
        recovered |= policy['offset']!='Declared' and (offset<18 or offset>len(b))
        start=14
    if len(b)-start<4: fail('TruncatedDibHeader')
    size=u32(b,start)
    allowed=((12,16,40,52,56,64,108,124) if policy['headers']=='All1_1' else
             (12,40,52,56,64,108,124) if policy['headers']=='No16' else
             (12,40,52,56,108,124))
    if size not in allowed: fail('UnsupportedHeaderSize')
    if len(b)-start<(size if policy['header_bytes']=='Full' else (12 if size==12 else 16 if size==16 else 40)):fail('TruncatedHeader')
    recovered |= policy['header_bytes']=='ParsedFields' and len(b)-start<size
    core=size==12
    if core:
        width,signed_height,planes,bits=(u16(b,start+i) for i in (4,6,8,10))
        compression=used=declared_image=0
    elif size==16:
        width,signed_height=s32(b,start+4),s32(b,start+8)
        planes,bits=u16(b,start+12),u16(b,start+14)
        compression=used=declared_image=0
    else:
        width,signed_height=s32(b,start+4),s32(b,start+8)
        planes,bits=u16(b,start+12),u16(b,start+14)
        compression,declared_image,used=u32(b,start+16),u32(b,start+20),u32(b,start+32)
    if policy['planes']=='Must1' and planes!=1: fail('InvalidPlanes')
    if width<=0 or signed_height in (0,-2147483648): fail('InvalidDimensions')
    if ico and (signed_height<0 or signed_height%2): fail('InvalidDimensions')
    height=abs(signed_height)//(2 if ico else 1)
    if width>MAX_DIMENSION or height>MAX_DIMENSION: fail('InvalidDimensions')
    if bits not in ((1,2,4,8,16,24,32) if policy['unsupported']=='Rgb2' and size>=40 else (1,4,8,16,24,32)): fail('UnsupportedPixelDepth')
    if compression not in (0,1,2,3): fail('UnsupportedCompression')
    if bits==2 and compression!=0:fail('InvalidCompressionDepth')
    if (compression==1 and bits!=8) or (compression==2 and bits!=4) or (compression==3 and bits not in (16,32)): fail('InvalidCompressionDepth')
    if (signed_height<0 or ico) and compression in (1,2): fail('UnsupportedOrientation')
    if bits<=8:
        if used>1<<bits: fail('InvalidColorCount')
    elif used and mode!='BmpFile': fail('InvalidColorCount')
    elif policy['high_count']=='GdkBound' and used>((1<<bits)&0xffffffff):fail('InvalidColorCount')
    n=width*height
    if 4*n>MAX_OUTPUT: fail('OutputTooLarge')
    pos=start+size;masks=None
    if compression==3:
        if size in (40,64):
            if len(b)-pos<12:
                if policy['incomplete_metadata']=='Blank':
                    return width,height,(b'\x00\x00\x00\x00' if bits==32 else b'\x00\x00\x00\xff')*n,True
                fail('TruncatedMasks')
            masks=tuple(u32(b,pos+i*4) for i in range(3))+(0,)
            pos+=12
        else:
            needed=(56 if size>=56 else 52)
            if len(b)-start<needed:fail('TruncatedMasks')
            masks=tuple(u32(b,start+40+i*4) for i in range(4 if size>=56 else 3))
            if size==52: masks+=(0,)
        if policy['masks']=='DefaultRGB' and not all(masks[:3]):
            recovered=True
            masks=((0x7c00,0x03e0,0x001f,0) if bits==16 else (0x00ff0000,0x0000ff00,0x000000ff,0xff000000))
        if policy['masks']=='Reject' and not masks_valid(masks,bits): fail('InvalidBitfields')
        if policy['masks']=='Independent':
            if any(m>>bits or (m and ((m//(m&-m)) & ((m//(m&-m))+1))) for m in masks) or not all(masks[:3]):
                fail('InvalidBitfields')
        # Noncanonical GDK bit runs require a separately specified bit-count rule.
        if policy['masks']=='DefaultRGB' and any(m>>bits for m in masks): fail('InvalidBitfields')
    count=(used or 1<<bits) if bits<=8 else 0
    pal_stride=3 if core else 4
    table_count=count if bits<=8 else (used if policy['high_count']=='BmpTable' else 0)
    palette_end=pos+table_count*pal_stride
    if mode=='BmpFile' and policy['offset']=='Declared' and offset<palette_end: fail('InvalidPixelOffset')
    if len(b)<palette_end and (table_count or policy['header_bytes']=='Full'):
        if policy['incomplete_metadata']=='Blank':
            return width,height,(b'\x00\x00\x00\x00' if bits==32 or compression in (1,2) else b'\x00\x00\x00\xff')*n,True
        fail('TruncatedPalette')
    palette=b[pos:palette_end]
    if (mode=='BmpFile' and bits<=8 and used>0 and used<(1<<bits)
        and offset>palette_end and policy['short_indexed_gap']=='Blank'):
        return width,height,(b'\x00\x00\x00\x00' if compression in (1,2) else b'\x00\x00\x00\xff')*n,True
    if mode=='BmpFile' and policy['offset']=='Gdk':
        if bits<=8 and (offset<palette_end or offset>len(b)):
            return width,height,b'\x00\x00\x00\x00'*(n if compression in (1,2) else 0)+b'\x00\x00\x00\xff'*(n if compression not in (1,2) else 0),True
        if bits>8 and compression==0 and offset>len(b):fail('InvalidPixelOffset')
        if bits>8 and compression==0:
            recovered |= offset<palette_end
            offset=max(offset,palette_end)
    pos=offset if mode=='BmpFile' else palette_end
    if mode=='BmpFile' and compression==3 and policy['bitfield_offset']=='MetadataEnd':
        recovered |= pos!=palette_end
        pos=palette_end
    out=bytearray(4*n)
    if policy['short_pixels']=='ZeroFill' and compression in (0,3) and bits!=32:
        out[:]=b'\x00\x00\x00\xff'*n
    if count and compression in (1,2):
        if policy['rle_fill']=='Palette0':
            out[:]=bytes((palette[2],palette[1],palette[0],255))*n
        elif policy['rle_fill']=='OpaqueBlack':out[:]=b'\x00\x00\x00\xff'*n
    def colour(ix):
        nonlocal recovered
        if ix>=count:
            if policy['palette_index']=='Reject': fail('PaletteIndexOutOfRange')
            recovered=True
            return b'\x00\x00\x00\xff'
        p=ix*pal_stride
        return bytes((palette[p+2],palette[p+1],palette[p],255))
    def put(x,y,c):
        yy=y if signed_height<0 else height-1-y
        at=4*(yy*width+x)
        out[at:at+4]=c
    pixel_start=pos
    if compression in (0,3):
        line=stride(width,bits)
        for y in range(height):
            if len(b)-pos<line:
                if policy['short_pixels']=='Reject': fail('TruncatedPixels')
                recovered=True
                # No partially decoded row: complete preceding rows survive;
                # this and subsequent rows retain initialized black pixels.
                pos+=line
                continue
            row=b[pos:pos+line]
            for x in range(width):
                p=x*bits//8
                if bits<=8:
                    idx=(row[p]>>(8-bits-(x*bits)%8))&((1<<bits)-1)
                    c=colour(idx)
                elif bits==24:c=bytes((row[p+2],row[p+1],row[p],255))
                else:
                    word=int.from_bytes(row[p:p+bits//8],'little')
                    if compression==3:
                        c=bytes(channel(word,m,policy['scaling'],0 if i<3 else 255) if i!=3 or bits!=16 or policy['bitfield16_alpha']=='Mask' else 255 for i,m in enumerate(masks))
                    elif bits==16:
                        c=bytes((channel(word,0x7c00,policy['scaling']),channel(word,0x03e0,policy['scaling']),channel(word,0x001f,policy['scaling']),255))
                    else:c=bytes((row[p+2],row[p+1],row[p],row[p+3] if ico else 255))
                put(x,y,c)
            pos+=line
    else:
        x=y=0
        painted_pixels=0
        while True:
            if y>=height and policy['rle_eob']=='AtInputEnd':
                break
            if len(b)-pos<2:
                if policy['rle_eob']=='AtInputEnd':
                    break
                fail('RleTruncated')
            quantity,code=b[pos:pos+2];pos+=2
            if quantity:
                if y>=height:fail('RleBounds')
                if x+quantity>width:
                    if policy['rle_overrun']=='Reject' or (policy['rle_overrun']=='Image' and bits==4):fail('RleBounds')
                    recovered=True
                painted=min(quantity,max(0,width-x))
                seq=[code if bits==8 else (code>>(4 if j%2==0 else 0))&15 for j in range(painted)]
            elif code==1:break
            elif code==0:
                if y>=height:fail('RleBounds')
                x=0;y+=1
                if y==height and policy['rle_eol']=='StopAtHeight':
                    break
                continue
            elif code==2:
                if len(b)-pos<2:fail('RleTruncated')
                dx,dy=b[pos:pos+2];pos+=2
                if y+dy>=height or x+dx>=width:
                    if policy['rle_delta']=='Inside':fail('RleBounds')
                    recovered |= y+dy>height or x+dx>width
                    x=min(width,x+dx);y=min(height,y+dy)
                    continue
                x+=dx;y+=dy;continue
            else:
                if y>=height:fail('RleBounds')
                if x+code>width:
                    if policy['rle_overrun'] in ('Reject','Image'):fail('RleBounds')
                    recovered=True
                payload=code if bits==8 else (code+1)//2
                padded=payload+(payload&1)
                if len(b)-pos<padded:fail('RleTruncated')
                seq=[b[pos+j] if bits==8 else (b[pos+j//2]>>(4 if j%2==0 else 0))&15 for j in range(min(code,max(0,width-x)))]
                pos+=padded
            for ix in seq:
                if x>=width:continue
                put(x,y,colour(ix));x+=1;painted_pixels+=1
        if policy['rle_fill']!='Palette0' and painted_pixels<n:
            recovered=True
    image_len=pos-pixel_start
    and_bytes=stride(width,1)*height if ico else 0
    valid_sizes=(image_len,image_len+and_bytes) if ico else (image_len,)
    if policy['image_size']=='Exact' and declared_image and declared_image not in valid_sizes:fail('InvalidImageSize')
    if ico:
        if len(b)-pos<and_bytes:fail('TruncatedAndMask')
        has_xor_alpha=bits==32 and (compression==0 or masks[3]!=0)
        use_and=not has_xor_alpha or not any(out[i] for i in range(3,len(out),4))
        if use_and:
            for y in range(height):
                for x in range(width):
                    at=4*((height-1-y)*width+x)+3
                    if b[pos+y*stride(width,1)+x//8] & (0x80>>(x%8)):out[at]=0
                    elif has_xor_alpha:out[at]=255
        pos+=and_bytes
    if mode=='BareDib' and (pos<len(b) or (pos>len(b) and policy['short_pixels']=='Reject')):fail('TrailingBytes')
    if mode=='IcoEntryDib' and pos!=len(b):fail('TrailingBytes')
    return width,height,bytes(out),recovered

def decode(b,mode):
    """Stable 1.1 API, including both policy-free ICO modes."""
    w,h,rgba,_=_decode(b,mode,STRICT)
    return w,h,rgba

def decode_with_policy(b,mode,profile):
    """Profiled BMP/BareDib result includes the recovery indicator."""
    if mode not in ('BmpFile','BareDib'):
        raise ValueError('policy is only available for BmpFile and BareDib')
    if profile not in PROFILES:
        raise ValueError('only named versioned profiles are public')
    return _decode(b,mode,PROFILES[profile])

def runs(pixels):
    result=[]
    for p in range(0,len(pixels),4):
        h=pixels[p:p+4].hex()
        if result and result[-1]['hex']==h:result[-1]['count']+=1
        else:result.append({'count':1,'hex':h})
    return result

def actual(b,mode,profile='Strict',with_recovery=False):
    try:
        if isinstance(profile,str):
            if mode in ('IcoEntryDib','IcoEntryDibSlack'):
                if profile!='Strict':raise ValueError('ICO has no policy entry point')
                w,h,rgba,recovered=_decode(b,mode,STRICT)
            else:w,h,rgba,recovered=decode_with_policy(b,mode,profile)
        else:w,h,rgba,recovered=_decode(b,mode,profile)
        result={'width':w,'height':h,'rgba_runs':runs(rgba)}
        if with_recovery:result['recovered']=recovered
        return result
    except Invalid as e:return {'error':str(e)}

def expand(v):
    raw=bytes.fromhex(v['input_hex']);repeat=v.get('input_repeat',1)
    if 'repeat_from' in v:
        cut=v['repeat_from'];length=cut+(len(raw)-cut)*repeat+len(v['input_suffix_hex'])//2
        return length,None if length>MAX_INPUT else raw[:cut]+raw[cut:]*repeat+bytes.fromhex(v['input_suffix_hex'])
    length=len(raw)*repeat
    return length,None if length>MAX_INPUT else raw*repeat

def check_vector(v,profile='Strict'):
    length,b=expand(v)
    got={'error':'FileTooLarge'} if length>MAX_INPUT else actual(b,v['mode'],profile,'recovered' in v['expected'])
    if got!=v['expected']:raise AssertionError(f"{v['name']}: {v['expected']} != {got}")

def main():
    vectors=json.loads((HERE/'STRICT-VECTORS.json').read_text())
    assert len(vectors)==302
    for v in vectors:check_vector(v)
    print('302 unchanged Strict vectors passed')
    path=HERE/'POLICY-VECTORS.json'
    if path.exists():
        pv=json.loads(path.read_text())
        for profile,entries in pv.items():
            for v in entries:check_vector(v,profile)
        print('policy vectors passed:', {k:len(v) for k,v in pv.items()})
    custom=HERE/'CUSTOM-VECTORS.json'
    if custom.exists():
        cv=json.loads(custom.read_text())
        for row in cv:check_vector(row,row['policy'])
        print(len(cv),'custom policy values passed')

if __name__=='__main__':main()
