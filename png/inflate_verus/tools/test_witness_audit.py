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

    def test_cached_function_lookup_rejects_duplicates(self):
        text='fn sample() {} fn sample() {}'
        with self.assertRaises(AssertionError):audit.witness_calls.function(text,'sample')

    def test_cached_spec_lookup_rejects_duplicates(self):
        text='spec fn raw() -> Seq<u8> {seq![1u8]} spec fn raw() -> Seq<u8> {seq![1u8]}'
        with self.assertRaises(AssertionError):audit.witness_calls.spec_sequence(text,'raw')

    def test_cached_constructor_lookup_rejects_duplicates(self):
        text='fn wire() -> [u8;1] {[1u8]} fn wire() -> [u8;1] {[1u8]}'
        with self.assertRaises(AssertionError):audit.witness_calls.array_values(['wire','(',')'],text)

    def test_complete_audit_main(self):
        with contextlib.redirect_stdout(io.StringIO()):audit.main()

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

    def test_configured_call_removal_is_rejected(self):
        name='witness_config_cases.rs';text=(ROOT/'src'/name).read_text()
        call='check_policy_parts(&parts,&mut output,true,false,Status::Full,Ghost(expected));'
        self.assertIn(call,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(call,'',1)})

    def test_configured_call_status_is_rejected(self):
        name='witness_config.json';claims=json.loads((ROOT/'tools'/name).read_text())
        claims[0]['calls'][0]['status']='Full'
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_configured_policy_tuple_duplication_is_rejected(self):
        name='witness_config.json';claims=json.loads((ROOT/'tools'/name).read_text())
        claim=next(c for c in claims if c['domain']=='policy')
        claim['calls'][1]=copy.deepcopy(claim['calls'][0])
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_configured_policy_raw_prefix_is_rejected(self):
        name='witness_config.json';claims=json.loads((ROOT/'tools'/name).read_text())
        claim=next(c for c in claims if c['domain']=='policy')
        claim['calls'][0]['raw'][0]^=1
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_dynamic_profile_call_removal_is_rejected(self):
        name='witness_profiles.rs';text=(ROOT/'src'/name).read_text()
        call='check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));'
        self.assertIn(call,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(call,'',1)})

    def test_dynamic_profile_length_change_is_rejected(self):
        claims=json.loads((ROOT/'tools/witness_profiles.json').read_text())
        text=(ROOT/'src/witness_profiles.rs').read_text()
        claims[0]['lit'][0]+=1
        with self.assertRaises(AssertionError):audit.validate_profile_transcriptions(claims,text,self.vectors)

    def test_dynamic_repeat_position_change_is_rejected(self):
        claims=json.loads((ROOT/'tools/witness_profiles.json').read_text())
        text=(ROOT/'src/witness_profiles.rs').read_text()
        claims[0]['repeat_positions'][1]+=1
        with self.assertRaises(AssertionError):audit.validate_profile_transcriptions(claims,text,self.vectors)

    def test_flush_call_removal_is_rejected(self):
        name='witness_blocks.rs';text=(ROOT/'src'/name).read_text()
        call='check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));'
        self.assertIn(call,text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={name:text.replace(call,'',1)})

    def test_cut_source_prefix_change_is_rejected(self):
        claims=json.loads((ROOT/'tools/witness_blocks.json').read_text())
        text=(ROOT/'src/witness_blocks.rs').read_text()
        claims[1]['model_raw'][0]^=1
        with self.assertRaises(AssertionError):audit.validate_block_transcriptions(claims,text,self.vectors)

    def test_unregistered_witness_module_is_rejected(self):
        text=(ROOT/'src/lib.rs').read_text()
        self.assertIn('mod witness_profiles;',text)
        with self.assertRaises(AssertionError):self.validate(source_overrides={'lib.rs':text.replace('mod witness_profiles;','',1)})

    def test_declared_window_descriptor_change_is_rejected(self):
        name='witness_config.json';claims=json.loads((ROOT/'tools'/name).read_text())
        claim=next(c for c in claims if c['class']=='declared_beyond')
        claim['calls'][0]['window']*=2
        with self.assertRaises(AssertionError):self.validate(manifest_overrides={name:claims})

    def test_dynamic_copy_distance_change_is_rejected(self):
        claims=json.loads((ROOT/'tools/witness_profiles.json').read_text())
        text=(ROOT/'src/witness_profiles.rs').read_text()
        next(s for s in claims[0]['steps'] if s['symbol']>256)['distance']+=1
        with self.assertRaises(AssertionError):audit.validate_profile_transcriptions(claims,text,self.vectors)

    def test_evidence_checkout_needs_no_runs_directory(self):
        with tempfile.TemporaryDirectory(prefix='audit-checkout-',dir=ROOT/'backups') as temp:
            root=Path(temp)
            shutil.copytree(ROOT/'evidence',root/'evidence')
            (root/'tools').mkdir();shutil.copy2(ROOT/'tools/witness_blocked.json',root/'tools/witness_blocked.json')
            items=json.loads((ROOT/'evidence/historical-blocked.json').read_text())+json.loads((ROOT/'tools/witness_blocked.json').read_text())
            for item in items:
                dest=root/item['source_snapshot'];dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/item['source_snapshot'],dest)
            for record in json.loads((root/'evidence/manifest.json').read_text()):
                for name in set(record.get('source_sha256',{})) | set(record.get('source_archives_sha256',{})):
                    dest=root/name;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/name,dest)
            self.assertFalse((root/'runs').exists())
            with contextlib.redirect_stdout(io.StringIO()):audit.validate_evidence(root)
            first=json.loads((root/'evidence/manifest.json').read_text())[0]
            original=(root/first['path']).read_bytes()
            (root/first['path']).write_text('altered diagnostic')
            with self.assertRaises(AssertionError):audit.validate_evidence(root)
            (root/first['path']).write_bytes(original)
            archives=next(record['source_archives_sha256'] for record in json.loads((root/'evidence/manifest.json').read_text()) if record.get('source_archives_sha256'))
            archive=root/next(iter(archives));original_archive=archive.read_bytes()
            archive.write_bytes(b'altered source archive')
            with self.assertRaises(AssertionError):audit.validate_evidence(root)
            archive.write_bytes(original_archive)
            manifest=json.loads((root/'evidence/manifest.json').read_text())
            for record in manifest:
                if record.get('role')=='current-final-source-count':record['role']='earlier-source-count'
            (root/'evidence/manifest.json').write_text(json.dumps(manifest))
            with self.assertRaises(AssertionError):audit.validate_evidence(root)

if __name__=='__main__':unittest.main(verbosity=2)
