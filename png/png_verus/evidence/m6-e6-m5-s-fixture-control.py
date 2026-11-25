"""Independent literal audit; never imports a witness generator or its printer."""
import re

def wire_bytes(hex_text):
    assert isinstance(hex_text,str) and len(hex_text)%2==0
    digits='0123456789abcdef';result=bytearray()
    for at in range(0,len(hex_text),2):
        high=digits.index(hex_text[at].lower());low=digits.index(hex_text[at+1].lower())
        result.append(16*high+low)
    return bytes(result)

def frozen_pixels(record):
    expected=record['expected'];wide=record['entry']=='still16'
    key='rgba16_runs' if wide else 'rgba_runs'
    assert set(expected)=={'width','height',key}
    result=bytearray()
    for run in expected[key]:
        assert set(run)=={'hex','count'} and isinstance(run['count'],int) and run['count']>=0
        block=wire_bytes(run['hex'])
        for _ in range(run['count']):
            for byte in block:result.append(byte)
    assert len(result)==expected['width']*expected['height']*(8 if wide else 4)
    return bytes(result)

def literal(source,name):
    expression=re.compile(r'pub\s+open\s+spec\s+fn\s+'+re.escape(name)+r'\(\)\s*->\s*Seq<u8>\s*\{\s*seq!\[([^\]]*)\]\s*\}')
    matches=expression.findall(source);assert len(matches)==1,name
    text=matches[0].strip()
    if not text:return b''
    result=[]
    for token in text.split(','):
        token=token.strip();assert re.fullmatch(r'[0-9]+u8',token),(name,token)
        value=int(token[:-2]);assert 0<=value<=255;result.append(value)
    return bytes(result)

def audit(record,row,source):
    ident=re.sub('[^a-zA-Z0-9]','_',record['name']).lower()
    m3=row['witness_source']=='src/witness_m6_m3.rs'
    byte_name=('bytes_' if m3 else 'e6_bytes_')+ident
    expected_name=('expected_' if m3 else 'e6_expected_')+ident
    expected=frozen_pixels(record);actual=literal(source,expected_name)
    assert actual==expected,(record['name'],'expected byte literal differs from independent frozen-run expansion')
    assert literal(source,byte_name)==wire_bytes(record['input_hex'])
    goal=re.compile(r'pub\s+open\s+spec\s+fn\s+expected_value_'+re.escape(ident)+r'\(value:\s*sm::Value\)\s*->\s*bool\s*\{([^}]+)\}')
    matches=goal.findall(source);assert len(matches)==1
    exact=f'value.width=={record["expected"]["width"]}u32&&value.height=={record["expected"]["height"]}u32&&value.pixels=={expected_name}()'
    assert ''.join(matches[0].split())==exact,(record['name'],'incomplete frozen-result predicate')
    if record['entry']=='still16':
        for at in range(0,len(expected),2):
            word=expected[at]*256+expected[at+1]
            assert actual[at]==word//256 and actual[at+1]==word%256,(record['name'],'sixteen-bit physical byte order')
    return len(expected)


def rejects_swapped_word(record,row,source):
    """Literal-only in-memory negative control, not a decoder mutation run."""
    assert record['entry']=='still16'
    expected=frozen_pixels(record)
    at=next(i for i in range(0,len(expected),2) if expected[i]!=expected[i+1])
    changed=list(expected);changed[at],changed[at+1]=changed[at+1],changed[at]
    ident=re.sub('[^a-zA-Z0-9]','_',record['name']).lower()
    pattern=re.compile(r'(pub\s+open\s+spec\s+fn\s+e6_expected_'+re.escape(ident)+r'\(\)\s*->\s*Seq<u8>\s*\{\s*seq!\[)([^\]]*)(\]\s*\})')
    match=pattern.search(source);assert match
    replacement=','.join(str(value)+'u8' for value in changed)
    altered=source[:match.start(2)]+replacement+source[match.end(2):]
    try:audit(record,row,altered)
    except AssertionError:return True
    raise AssertionError('independent audit accepted a swapped nonidentical sixteen-bit byte pair')
