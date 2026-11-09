"""Determinism, provenance isolation, and decoder smoke tests."""
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
import engine
import format_harness as h

class FormatTests(unittest.TestCase):
    def test_seeded_generation(self):
        a,base,seeds=h.generate('tga',300,731)
        b,_,_=h.generate('tga',300,731)
        self.assertGreater(base,169)
        self.assertGreater(seeds,10)
        self.assertEqual([(hashlib.sha256(x).digest(),m) for x,m in a],
                         [(hashlib.sha256(x).digest(),m) for x,m in b])
    def test_valid_image(self):
        data=next(x for x,m in h.bases('tga') if m['name']=='raw-red')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            path=Path(d)/'red.tga';path.write_bytes(data)
            a=h.invoke('tga',path,'verified',3)
            b=h.invoke('tga',path,'image',3)
            c=h.invoke('tga',path,'tiny',3)
            self.assertEqual(engine.classify({'a':a,'b':b,'c':c}),'all-accept-identical-pixels')
    def test_grayscale_glycin_is_normalized(self):
        data=next(x for x,m in h.bases('tga') if m['name']=='gray8')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            path=Path(d)/'gray.tga';path.write_bytes(data)
            results=[h.invoke('tga',path,n,3) for n in ('verified','image','glycin')]
            self.assertTrue(all(r['status']=='ok' for r in results),results)
            self.assertEqual(len({r['rgba_sha256'] for r in results}),1,results)
    def test_gdk_forced_detection(self):
        data=next(x for x,m in h.bases('tga') if m['name']=='raw-red')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            path=Path(d)/'red.dat';path.write_bytes(data)
            self.assertEqual(h.invoke('tga',path,'gdk-forced-byte',3)['status'],'ok')
    def test_classification_and_provenance(self):
        a={'status':'ok','width':1,'height':1,'rgba_sha256':'a'}
        b={'status':'ok','width':1,'height':1,'rgba_sha256':'b'}
        self.assertEqual(engine.classify({'a':a,'b':b}),'accept-different-pixels')
        self.assertEqual(engine.relation_signature({'a':a,'b':a}),
                         engine.relation_signature({'a':b,'b':b}))
        self.assertEqual(set(h.EXT),{'tga','icns','ico'})

if __name__=='__main__':unittest.main()
