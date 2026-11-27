#!/usr/bin/env python3
"""Temporarily add pass-only resource probes; never add complete/public witnesses."""
import argparse,hashlib,importlib.util,json,shutil,subprocess
from pathlib import Path
W=Path(__file__).resolve().parent;R=Path('/home/ws/projects/verified-decoders/png/png_verus');B=W/'canonical-five'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def no_verifier():
 rows=subprocess.check_output(['ps','-eo','comm'],text=True).splitlines()
 assert not set(rows)&{'rust_verify','verus','z3'},'A verifier is active; no proof input may change'
def restore():
 no_verifier()
 for name in ['src/witness_m4_interlace_projection.rs','src/witness_m6_m4_a7.rs','src/lib.rs']:shutil.copyfile(B/name,R/name)
 original=json.loads((W/'recovered-source-sha256.json').read_text())
 assert all(sha(R/name)==digest for name,digest in original.items())
 print('Restored byte-identical recovered five-record canonical proof inputs')
def deploy():
 no_verifier();original=json.loads((W/'recovered-source-sha256.json').read_text());assert all(sha(R/name)==digest for name,digest in original.items())
 q=W/'resource-input'/ 'png_verus';(q/'src').mkdir(parents=True);(q/'evidence').mkdir();(q/'tools').symlink_to(R/'tools',target_is_directory=True)
 (q.parent/'spec-frozen').symlink_to(R.parent/'spec-frozen',target_is_directory=True);(q.parent/'inflate_verus').symlink_to(R.parent/'inflate_verus',target_is_directory=True)
 shutil.copyfile(B/'src/witness_m4_interlace_projection.rs',q/'src/witness_m4_interlace_projection.rs')
 scheduled={r['name'] for r in json.loads((R/'evidence/m4-witness-schedule.json').read_text())['records'] if r['name'].startswith('interlace/') and r['gate_assignment'].startswith('projection')}
 selected={r['name'] for r in json.loads((R/'evidence/m6-e6-m4-a7-c1-witness-index.json').read_text())['records']};assert len(scheduled)==36 and len(selected)==5
 spec=importlib.util.spec_from_file_location('resource_generator',W/'resource_generator.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.NAMES=sorted(scheduled-selected);assert len(m.NAMES)==31;m.generate(q)
 extra=(q/'src/witness_m6_a7_resource_probe.rs').read_text().split('verus! {',1)[1].rsplit('} // verus!',1)[0]
 main=(B/'src/witness_m6_m4_a7.rs').read_text();main=main.rsplit('} // verus!',1)[0]+'\n// Diagnostic resource probe literals only; no new complete/public witness.\n'+extra+'\n} // verus!\n'
 assert main.count('pub fn public_complete_')==5 and main.count('pub proof fn model_complete_')==5
 (R/'src/witness_m6_m4_a7.rs').write_text(main);shutil.copyfile(q/'src/witness_m4_interlace_projection.rs',R/'src/witness_m4_interlace_projection.rs')
 for name in ['src/witness_m6_m4_a7.rs','src/witness_m4_interlace_projection.rs']:shutil.copyfile(R/name,q/name)
 inventory={str(p.relative_to(R)):sha(p) for p in sorted((R/'src').rglob('*.rs'))+sorted((R/'tools').glob('*.py'))+[R/n for n in ['Cargo.toml','Cargo.lock','verify.sh']]}
 (W/'resource-probe-manifest.json').write_text(json.dumps(dict(kind='Diagnostic-only resource envelope; no additional complete/public witnesses',selected_records=sorted(selected),resource_only_records=m.NAMES,source_sha256=inventory,index='resource-input/png_verus/evidence/a7-resource-probe-index.json'),indent=2)+'\n')
 print('Deployed pass-only probes for remaining 31 records; five complete/public witnesses remain')
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('action',choices=['deploy','restore']);a=p.parse_args();deploy() if a.action=='deploy' else restore()
