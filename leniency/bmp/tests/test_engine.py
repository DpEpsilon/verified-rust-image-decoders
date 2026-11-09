import hashlib
import json
from pathlib import Path
import sys
import unittest

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import engine
import harness


class HarnessTests(unittest.TestCase):
    def test_classifications(self):
        a={'status':'ok','width':1,'height':1,'rgba_sha256':'a'}
        b={'status':'ok','width':1,'height':1,'rgba_sha256':'b'}
        e={'status':'error','error_class':'invalid'}
        self.assertEqual(engine.classify({'x':a,'y':a}), 'all-accept-identical-pixels')
        self.assertEqual(engine.classify({'x':a,'y':b}), 'accept-different-pixels')
        self.assertEqual(engine.classify({'x':a,'y':e}), 'accept-vs-reject')
        self.assertEqual(engine.classify({'x':e,'y':e}), 'all-reject')
        self.assertEqual(engine.classify({'x':e,'y':{'status':'timeout'}}), 'crash-or-timeout')
        another={'status':'ok','width':1,'height':1,'rgba_sha256':'z'}
        self.assertEqual(harness.relation_signature({'x':a,'y':a}),
                         harness.relation_signature({'x':another,'y':another}))
        self.assertNotEqual(harness.relation_signature({'x':a,'y':b}),
                            harness.relation_signature({'x':a,'y':a}))

    def test_frozen_bmp_vector(self):
        vector=next(v for v in json.loads(harness.VECTORS.read_text())
                    if v['name']=='header-40-BmpFile')
        data=bytes.fromhex(vector['input_hex'])
        self.assertEqual(data[:2],b'BM')
        self.assertEqual(harness.seed_key(data),(40,24,0))
        self.assertEqual(harness.u(data,10,4),54)

    def test_core_header_wrapper(self):
        dib=bytes.fromhex('0c00000001000100010018000000ff00')
        bmp=harness.wrap_dib(dib)
        self.assertEqual(bmp[14:],dib)
        self.assertEqual(harness.u(bmp,10,4),26)

    def test_real_decoders_agree_on_core_header(self):
        dib=bytes.fromhex('0c00000001000100010018000000ff00')
        from tempfile import TemporaryDirectory
        with TemporaryDirectory(prefix='leniency-test-',dir='/tmp/claude-0') as tmp:
            path=Path(tmp)/'sample.bmp';path.write_bytes(harness.wrap_dib(dib))
            results={name:harness.invoke(path,name,10) for name in harness.RUNNERS}
            self.assertEqual(engine.classify(results),'all-accept-identical-pixels',results)

    def test_seed_is_deterministic(self):
        a,_,_=harness.generate(300,731)
        b,_,_=harness.generate(300,731)
        self.assertEqual([(hashlib.sha256(data).digest(),m) for data,m in a],
                         [(hashlib.sha256(data).digest(),m) for data,m in b])


if __name__=='__main__': unittest.main()
