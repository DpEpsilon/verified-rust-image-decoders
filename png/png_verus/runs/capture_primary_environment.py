from pathlib import Path
import os,subprocess,signal,time,json,fcntl
r=Path('/home/ws/projects/verified-decoders/png/png_verus')
lock=Path('/home/ws/.cache/verified-decoders/verifier.lock').open('a')
fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
env={k:v for k,v in os.environ.items() if k!='RUST_MIN_STACK'}
env.update(PATH='/opt/cargo/bin:/opt/verus:'+env.get('PATH',''),CARGO_BUILD_JOBS='1',CARGO_TARGET_DIR=str(r/'target'),CARGO_TERM_VERBOSE='true')
cmd=['python3','-I',str(r/'tools/run_logged.py'),'--timeout','120','--label','Cargo primary environment inspection only; terminate before proof work','--','cargo','verus','verify','-vvv','--','--num-threads','2','--no-cheating']
p=subprocess.Popen(cmd,cwd=r,env=env,stdout=subprocess.PIPE,text=True,start_new_session=True)
found=None
try:
 end=time.monotonic()+110
 while time.monotonic()<end and p.poll() is None:
  for f in Path('/proc').glob('[0-9]*/cmdline'):
   try:a=f.read_bytes().split(b'\0')
   except OSError:continue
   if a and a[0]==b'/opt/verus/rust_verify' and b'--crate-name' in a and b'png_verus' in a:
    pid=int(f.parent.name);os.kill(pid,signal.SIGSTOP)
    pairs=(f.parent/'environ').read_bytes().split(b'\0')
    full=dict(x.decode().split('=',1) for x in pairs if b'=' in x)
    filtered={k:v for k,v in full.items() if k.strip().startswith(('CARGO_','__VERUS_','__CARGO_')) or k in ['OUT_DIR','RUSTC_BOOTSTRAP','RUST_MIN_STACK','RUSTC_WRAPPER','RUSTUP_TOOLCHAIN','LD_LIBRARY_PATH']}
    found={'kind':'stopped-primary-cargo-environment-capture','pid':pid,'argv':[x.decode() for x in a if x],'environment':filtered,'cargo_invoked_for_inspection_only':True,'primary_terminated_before_proof_diagnostic':True}
    (r/'runs/cargo-primary-environment-capture.json').write_text(json.dumps(found,indent=2)+'\n')
    os.kill(pid,signal.SIGTERM);os.kill(pid,signal.SIGCONT);break
  if found:break
  time.sleep(.025)
 out,_=p.communicate(timeout=15);print('\n'.join(out.splitlines()[-3:]));assert found,'No primary process captured'
 print('Captured',len(found['environment']),'Cargo/Verus environment variables; primary terminated.')
except BaseException:
 if p.poll() is None:
  os.killpg(p.pid,signal.SIGTERM)
  try:p.wait(timeout=5)
  except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait(timeout=5)
 raise
