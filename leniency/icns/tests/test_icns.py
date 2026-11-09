"""ICNS legacy pair, PNG candidate, and selection tests."""
import hashlib
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
import format_harness as h

class ICNSTests(unittest.TestCase):
    def test_synthetic_png_has_valid_scanlines(self):
        all_inputs={m['name']:b for b,m in h.bases('icns') if m['name'].startswith('compose-')}
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            for side in (16,32,64,128,256):
                path=Path(d)/f'{side}.icns';path.write_bytes(all_inputs[f'compose-png-{side}'])
                for name in ('verified-png','crate','glycin'):
                    result=h.invoke('icns',path,name,4)
                    self.assertEqual((result['status'],result.get('width')),('ok',side),(name,result))
    def test_core_pair_and_largest_selection(self):
        data=next(b for b,m in h.bases('icns') if m['name']=='compose-two-legacy-32-16')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            p=Path(d)/'pair.icns';p.write_bytes(data)
            good=[h.invoke('icns',p,n,4) for n in ('verified-legacy','crate','glycin')]
            self.assertTrue(all(x['status']=='ok' and x['width']==32 for x in good),good)
            self.assertEqual(len({x['rgba_sha256'] for x in good}),1)
    def test_generator_repeatable(self):
        a,_,_=h.generate('icns',300,731)
        b,_,_=h.generate('icns',300,731)
        self.assertEqual([(hashlib.sha256(x).digest(),m) for x,m in a],
                         [(hashlib.sha256(x).digest(),m) for x,m in b])

if __name__=='__main__':unittest.main()
