"""ICO/CUR oracle behavior, per-entry selection, generator, provenance tests."""
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
import format_harness as h
import engine

class ICOTests(unittest.TestCase):
    def test_selection_ids_survive_identical_pixels(self):
        a={'status':'ok','width':16,'height':16,'rgba_sha256':'same','selection':'entry:0:dib'}
        b={**a,'selection':'entry:1:dib'}
        self.assertNotEqual(engine.outcome_signature({'first':a}),engine.outcome_signature({'first':b}))
    def test_oracle_and_entry_decode(self):
        vectors=json.loads(h.VECTORS['ico'].read_text())
        v=next(v for v in vectors if v['operation']=='decode' and 'rgba_runs' in v['expected']
               and v.get('index')==0)
        data=bytes.fromhex(v['input_hex'])
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            p=Path(d)/'test.ico';p.write_bytes(data)
            oracle=h.invoke('ico',p,'oracle-first',5)
            self.assertEqual(oracle['status'],'ok',oracle)
            self.assertEqual((oracle['width'],oracle['height']),
                             (v['expected']['width'],v['expected']['height']))
            crate=h.invoke('ico',p,'crate-entry:0',5)
            self.assertEqual(crate['status'],'ok',crate)
    def test_png_delegation_not_spec_rejection(self):
        vector=next(v for v in json.loads(h.VECTORS['ico'].read_text())
                    if v['operation']=='decode' and v['expected'].get('error')=='PngDelegation')
        with tempfile.TemporaryDirectory(dir='/tmp/claude-0') as d:
            path=Path(d)/'png.ico';path.write_bytes(bytes.fromhex(vector['input_hex']))
            r=h.invoke('ico',path,'oracle-entry-'+str(vector['index']),4)
            self.assertEqual(r['error_class'],'PngDelegation')
    def test_deterministic_and_local_corpus_unmodified(self):
        a,base,_=h.generate('ico',300,731)
        b,_,_=h.generate('ico',300,731)
        self.assertGreater(base,217)
        self.assertEqual([(hashlib.sha256(x).digest(),m) for x,m in a],
                         [(hashlib.sha256(x).digest(),m) for x,m in b])
        self.assertTrue(any(m['source']=='local-ico-cur-corpus' for _,m in a))

if __name__=='__main__':unittest.main()
