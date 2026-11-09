#!/usr/bin/env python3
"""Independent, bounds-checked DIB byte oracle. Run with python3 -I."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
MAX_INPUT = MAX_OUTPUT = 67_108_864
MAX_DIMENSION = 8192
ERRORS = set('FileTooLarge TruncatedFileHeader BadMagic InvalidFileSize InvalidPixelOffset TruncatedDibHeader UnsupportedHeaderSize TruncatedHeader InvalidPlanes InvalidDimensions UnsupportedPixelDepth UnsupportedCompression InvalidCompressionDepth UnsupportedOrientation InvalidColorCount OutputTooLarge TruncatedMasks InvalidBitfields TruncatedPalette TruncatedPixels PaletteIndexOutOfRange RleTruncated RleBounds InvalidImageSize TruncatedAndMask TrailingBytes'.split())

class Invalid(Exception):
    pass

def fail(name):
    raise Invalid(name)

def u16(b, p):
    return int.from_bytes(b[p:p+2], 'little')

def u32(b, p):
    return int.from_bytes(b[p:p+4], 'little')

def s32(b, p):
    return int.from_bytes(b[p:p+4], 'little', signed=True)

def stride(width, bits):
    return ((width * bits + 31) // 32) * 4

def masks_valid(masks, bits):
    used = 0
    for i, m in enumerate(masks):
        if i < 3 and m == 0:
            return False
        if m >> bits or m & used:
            return False
        if m:
            low = m & -m
            compact = m // low
            if compact & (compact + 1):
                return False
        used |= m
    return True

def channel(word, mask):
    if not mask:
        return 255
    shift = (mask & -mask).bit_length()-1
    n = (mask >> shift).bit_length()
    return (((word & mask) >> shift) * 255 + ((1 << n)-1)//2) // ((1 << n)-1)

def decode(b, mode):
    if len(b) > MAX_INPUT:
        fail('FileTooLarge')
    if mode not in ('BmpFile', 'BareDib', 'IcoEntryDib'):
        raise ValueError('mode is an API enum, not a format error')
    start = 0
    offset = None
    if mode == 'BmpFile':
        if len(b) < 14: fail('TruncatedFileHeader')
        if b[:2] != b'BM': fail('BadMagic')
        if (u32(b, 2) != 0 and u32(b, 2) > len(b)) or u16(b, 6) or u16(b, 8): fail('InvalidFileSize')
        offset = u32(b, 10)
        if offset < 18 or offset > len(b): fail('InvalidPixelOffset')
        start = 14
    if len(b)-start < 4: fail('TruncatedDibHeader')
    size = u32(b, start)
    if size not in (12, 16, 40, 52, 56, 64, 108, 124): fail('UnsupportedHeaderSize')
    if len(b)-start < size: fail('TruncatedHeader')
    core = size == 12
    if core:
        width, signed_height, planes, bits = u16(b,start+4), u16(b,start+6), u16(b,start+8), u16(b,start+10)
        compression, used, declared_image = 0, 0, 0
    elif size == 16:
        width, signed_height = s32(b,start+4), s32(b,start+8)
        planes, bits = u16(b,start+12), u16(b,start+14)
        compression, used, declared_image = 0, 0, 0
    else:
        width, signed_height = s32(b,start+4), s32(b,start+8)
        planes, bits = u16(b,start+12), u16(b,start+14)
        compression, declared_image, used = u32(b,start+16), u32(b,start+20), u32(b,start+32)
    if planes != 1: fail('InvalidPlanes')
    if width <= 0 or signed_height == 0 or signed_height == -2147483648: fail('InvalidDimensions')
    if mode == 'IcoEntryDib' and (signed_height < 0 or signed_height % 2): fail('InvalidDimensions')
    height = (abs(signed_height)//2 if mode == 'IcoEntryDib' else abs(signed_height))
    if width > MAX_DIMENSION or height > MAX_DIMENSION: fail('InvalidDimensions')
    if bits not in (1,4,8,16,24,32): fail('UnsupportedPixelDepth')
    if compression not in (0,1,2,3): fail('UnsupportedCompression')
    if (compression == 1 and bits != 8) or (compression == 2 and bits != 4) or (compression == 3 and bits not in (16,32)):
        fail('InvalidCompressionDepth')
    if signed_height < 0 and compression in (1,2): fail('UnsupportedOrientation')
    if mode == 'IcoEntryDib' and compression in (1,2): fail('UnsupportedOrientation')
    if bits <= 8:
        if used > (1 << bits): fail('InvalidColorCount')
    elif used and mode != 'BmpFile':
        fail('InvalidColorCount')
    n = width*height
    if 4*n > MAX_OUTPUT: fail('OutputTooLarge')
    pos = start+size
    masks = None
    if compression == 3:
        if size in (40,64):
            if len(b)-pos < 12: fail('TruncatedMasks')
            masks = tuple(u32(b,pos+i*4) for i in range(3))+(0,)
            pos += 12
        else:
            masks = tuple(u32(b,start+40+i*4) for i in range(4 if size >=56 else 3))
            if size == 52: masks += (0,)
        if not masks_valid(masks,bits): fail('InvalidBitfields')
    count = (used or 1 << bits) if bits <= 8 else 0
    pal_stride = 3 if core else 4
    table_count = count if bits <= 8 else used
    # Python integer arithmetic models the required u64 product before any
    # narrowing: 4*biClrUsed may exceed 32-bit usize even for a short BMP.
    palette_length = table_count * pal_stride
    palette_end = pos + palette_length
    if mode == 'BmpFile' and offset < palette_end: fail('InvalidPixelOffset')
    if len(b) < palette_end: fail('TruncatedPalette')
    palette = b[pos:palette_end]
    pos = offset if mode == 'BmpFile' else palette_end
    out = bytearray(4*n)
    if count:
        initial = bytes((palette[2],palette[1],palette[0],255))
        if compression in (1,2): out[:] = initial*n
    def colour(ix):
        if ix >= count: fail('PaletteIndexOutOfRange')
        p = ix*pal_stride
        return bytes((palette[p+2],palette[p+1],palette[p],255))
    def put(x,y,c):
        yy = y if signed_height < 0 else height-1-y
        p = 4*(yy*width+x)
        out[p:p+4] = c
    pixel_start = pos
    if compression in (0,3):
        line = stride(width,bits)
        for y in range(height):
            if len(b)-pos < line: fail('TruncatedPixels')
            for x in range(width):
                p = pos+(x*bits)//8
                if bits <= 8:
                    idx = (b[p] >> (8-bits-(x*bits)%8)) & ((1<<bits)-1)
                    c = colour(idx)
                elif bits == 24:
                    c = bytes((b[p+2],b[p+1],b[p],255))
                else:
                    word = int.from_bytes(b[p:p+bits//8], 'little')
                    if compression == 3:
                        c = bytes(channel(word,m) for m in masks)
                    elif bits == 16:
                        c = bytes((channel(word,0x7c00),channel(word,0x03e0),channel(word,0x001f),255))
                    else:
                        c = bytes((b[p+2],b[p+1],b[p],b[p+3] if mode == 'IcoEntryDib' else 255))
                put(x,y,c)
            pos += line
    else:
        x=y=0
        while True:
            if len(b)-pos < 2: fail('RleTruncated')
            quantity, code = b[pos:pos+2]
            pos += 2
            if quantity:
                if y >= height or x+quantity > width: fail('RleBounds')
                seq = [(code if bits==8 else (code >> (4 if j%2==0 else 0)) & 15) for j in range(quantity)]
            elif code == 1:
                break
            elif code == 0:
                if y >= height: fail('RleBounds')
                x=0; y+=1
                continue
            elif code == 2:
                if len(b)-pos < 2: fail('RleTruncated')
                dx,dy=b[pos:pos+2]; pos+=2
                if y+dy >= height or x+dx >= width: fail('RleBounds')
                x+=dx; y+=dy
                continue
            else:
                if y >= height or x+code > width: fail('RleBounds')
                payload=(code if bits==8 else (code+1)//2)
                padded=payload+(payload&1)
                if len(b)-pos < padded: fail('RleTruncated')
                seq = [b[pos+j] if bits==8 else (b[pos+j//2] >> (4 if j%2==0 else 0)) & 15 for j in range(code)]
                pos+=padded
            for ix in seq:
                put(x,y,colour(ix)); x+=1
    image_len=pos-pixel_start
    mask_stride = stride(width,1) if mode == 'IcoEntryDib' else 0
    and_bytes = mask_stride*height
    valid_sizes = (image_len, image_len+and_bytes) if mode == 'IcoEntryDib' else (image_len,)
    if declared_image and declared_image not in valid_sizes: fail('InvalidImageSize')
    if mode == 'IcoEntryDib':
        if len(b)-pos < and_bytes: fail('TruncatedAndMask')
        has_xor_alpha = bits==32 and (compression==0 or masks[3]!=0)
        use_and = not has_xor_alpha or not any(out[i] for i in range(3,len(out),4))
        if use_and:
            for y in range(height):
                for x in range(width):
                    yy=height-1-y
                    a=4*(yy*width+x)+3
                    if b[pos+y*mask_stride+x//8] & (0x80 >> (x%8)):
                        out[a]=0
                    elif has_xor_alpha:
                        out[a]=255
        pos += and_bytes
    if mode != 'BmpFile' and pos != len(b): fail('TrailingBytes')
    return width,height,bytes(out)

def runs(pixels):
    result=[]
    for p in range(0,len(pixels),4):
        h=pixels[p:p+4].hex()
        if result and result[-1]['hex']==h: result[-1]['count']+=1
        else: result.append({'count':1,'hex':h})
    return result

def actual(b,mode):
    try:
        w,h,rgba=decode(b,mode)
        return {'width':w,'height':h,'rgba_runs':runs(rgba)}
    except Invalid as e:
        return {'error':str(e)}

def check_vector(v):
    allowed={'name','mode','input_hex','expected','input_repeat','repeat_from','input_suffix_hex'}
    assert {'name','mode','input_hex','expected'} <= set(v) <= allowed
    assert ('repeat_from' in v) == ('input_suffix_hex' in v)
    assert v['mode'] in ('BmpFile','BareDib','IcoEntryDib')
    h=v['input_hex']; assert isinstance(h,str) and h==h.lower() and len(h)%2==0
    repeat=v.get('input_repeat',1)
    assert type(repeat)==int and repeat>0
    if 'error' in v['expected']:
        assert set(v['expected'])=={'error'} and v['expected']['error'] in ERRORS
    else:
        ex=v['expected']; assert set(ex)=={'width','height','rgba_runs'}
        rr=ex['rgba_runs']; assert sum(r['count'] for r in rr)==ex['width']*ex['height']
        assert all(set(r)=={'count','hex'} and len(r['hex'])==8 and r['count']>0 and (i==0 or rr[i-1]['hex'] != r['hex']) for i,r in enumerate(rr))
    raw=bytes.fromhex(h)
    if 'repeat_from' in v:
        cut=v['repeat_from']
        assert type(cut)==int and 0<=cut<len(raw) and repeat>0
        suffix=v['input_suffix_hex']
        assert suffix==suffix.lower() and len(suffix)%2==0
        length=cut+(len(raw)-cut)*repeat+len(suffix)//2
        data=None if length>MAX_INPUT else raw[:cut]+raw[cut:]*repeat+bytes.fromhex(suffix)
    else:
        length=len(raw)*repeat
        data=None if length>MAX_INPUT else raw*repeat
    got={'error':'FileTooLarge'} if length>MAX_INPUT else actual(data,v['mode'])
    if got!=v['expected']:
        raise AssertionError(f"{v['name']}: {v['expected']} != {got}")

def main():
    vectors=json.loads((HERE/'VECTORS.json').read_text())
    assert len({v['name'] for v in vectors})==len(vectors)
    for v in vectors: check_vector(v)
    print(f'{len(vectors)} independent DIB conformance vectors passed')

if __name__=='__main__': main()
