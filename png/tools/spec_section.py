#!/usr/bin/env python3
"""Print the plain text of one section of the local PNG third-edition source by its id.
Usage: python3 -I spec_section.py ID [MAXCHARS]"""
import html, re, sys
from pathlib import Path
src = (Path(__file__).resolve().parent.parent / 'spec-draft/sources/png-3e-index.html').read_text(encoding='utf8')
sid = sys.argv[1]
i = src.find(f'id="{sid}"')
if i < 0:
    sys.exit('no such id')
i = src.rfind('<', 0, i)
tag = re.match(r'<(\w+)', src[i:]).group(1)
depth, j = 0, i
for m in re.finditer(rf'<(/?){tag}\b', src[i:]):
    depth += -1 if m.group(1) else 1
    if depth == 0:
        j = i + m.end()
        break
s = src[i:j]
s = re.sub(r'<script.*?</script>', '', s, flags=re.S)
s = re.sub(r'</(p|h\d|li|tr|pre|div|dt|dd|section|table)>', '\n', s)
s = re.sub(r'</t[dh]>', ' | ', s)
s = re.sub(r'<[^>]+>', '', s)
s = re.sub(r'[ \t]+', ' ', html.unescape(s))
s = re.sub(r'\n\s*\n+', '\n', s)
print(s[:int(sys.argv[2]) if len(sys.argv) > 2 else None])
