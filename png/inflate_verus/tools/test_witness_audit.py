#!/usr/bin/env python3
"""Negative call-binding tests and a runs-free evidence checkout replay."""
import contextlib
import copy
import io
import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import audit_witnesses as audit

class WitnessAuditTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.vectors={v['name']:v for v in json.loads((ROOT.parent/'spec-frozen/VECTORS.json').read_text())}
        cls.policy={v['name']:v for v in json.loads((ROOT.parent/'policy-draft/VECTORS-POLICY.json').read_text())}

    def validate(self, **kwargs):
        with contextlib.redirect_stdout(io.StringIO()):
            audit.validate_bindings(self.vectors,self.policy,**kwargs)

    def test_current_call_bindings(self):
        self.validate()

    def test_removed_public_call_is_rejected(self):
        name='witness_repeats.rs';text=(ROOT/'src'/name).read_text()
        call='check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));'
        self.assertIn(call,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(call,'',1)})

    def test_changed_limit_is_rejected(self):
        name='witness_repeats.rs';text=(ROOT/'src'/name).read_text()
        self.assertIn('[0xa5u8;4]',text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace('[0xa5u8;4]','[0xa5u8;5]',1)})

    def test_duplicated_option_tuple_is_rejected(self):
        name='witness_policy_cases.rs';text=(ROOT/'src'/name).read_text()
        old='check_policy_parts(&parts,&mut output,true,true,Status::Done,Ghost(raw));'
        self.assertIn(old,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(old,old.replace('true,true','true,false'),1)})

    def test_changed_manifest_status_is_rejected(self):
        name='witness_special.json';claims=json.loads((ROOT/'tools'/name).read_text())
        self.assertEqual(claims[0]['status'],'TruncatedZlib');claims[0]['status']='Done'
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_new_finishing_call_removal_is_rejected(self):
        name='witness_finish.rs';text=(ROOT/'src'/name).read_text()
        old='check_policy_parts(&parts,&mut output,true,false,Status::Err(E::AdlerMismatch),Ghost(raw));'
        self.assertIn(old,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(old,'',1)})

    def test_control_flow_cannot_supply_call_credit(self):
        name='witness_repeats.rs';text=(ROOT/'src'/name).read_text()
        call='check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));'
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(call,'if false {'+call+'}',1)})

    def test_changed_individual_status_is_rejected(self):
        name='witness_individual.json';claims=json.loads((ROOT/'tools'/name).read_text())
        claims[0]['calls'][0]['status']='Done'
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_original_layout_call_removal_is_rejected(self):
        name='witness_extended.rs';text=(ROOT/'src'/name).read_text()
        old='replay_layout(&wire,&cuts,&mut parts,&mut output,Ghost(raw));'
        self.assertIn(old,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(old,'',1)})

    def test_changed_layout_cuts_are_rejected(self):
        name='witness_layouts.json';claims=json.loads((ROOT/'tools'/name).read_text())
        claims[0]['cuts'][1]=2
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_evidence_checkout_needs_no_runs_directory(self):
        with tempfile.TemporaryDirectory(prefix='audit-checkout-',dir=ROOT/'backups') as temp:
            root=Path(temp)
            shutil.copytree(ROOT/'evidence',root/'evidence')
            (root/'tools').mkdir();shutil.copy2(ROOT/'tools/witness_blocked.json',root/'tools/witness_blocked.json')
            items=json.loads((ROOT/'evidence/historical-blocked.json').read_text())+json.loads((ROOT/'tools/witness_blocked.json').read_text())
            for item in items:
                dest=root/item['source_snapshot'];dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/item['source_snapshot'],dest)
            for record in json.loads((root/'evidence/manifest.json').read_text()):
                for name in record.get('source_sha256',{}):
                    dest=root/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/name,dest)
            self.assertFalse((root/'runs').exists())
            with contextlib.redirect_stdout(io.StringIO()):audit.validate_evidence(root)
            first=json.loads((root/'evidence/manifest.json').read_text())[0]
            (root/first['path']).write_text('altered diagnostic')
            with self.assertRaises(AssertionError):audit.validate_evidence(root)

if __name__=='__main__':unittest.main(verbosity=2)
