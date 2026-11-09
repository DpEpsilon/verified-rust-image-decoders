"""Format-neutral subprocess executor and normalization classifier.

A format adapter provides a mapping name -> (executable path, selector arguments);
its generator supplies bytes and provenance. Each invocation gets the exact same input file.
"""
import json
import subprocess


def invoke(path, runner, timeout=10, dump=None):
    """runner is (executable, arguments); optional last argument requests RGBA8 dump."""
    exe, args=runner
    try:
        cmd=['prlimit','--as=805306368','--core=0','--fsize=67108864','--',
             str(exe),*map(str,args),str(path)]+([str(dump)] if dump else [])
        p=subprocess.run(cmd, stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=timeout,
                         check=False)
        lines=p.stdout.decode('utf-8','replace').splitlines()
        if p.returncode and not lines:
            return {'status':'crash','error_class':'signal' if p.returncode<0 else 'exit',
                    'message':str(p.returncode)+' '+p.stderr.decode('utf-8','replace')[-160:]}
        obj=json.loads(lines[-1]) if lines else {'status':'crash','error_class':'empty-output'}
        if p.returncode and obj.get('status')=='ok':
            return {'status':'crash','error_class':'post-output','message':str(p.returncode)}
        return obj
    except subprocess.TimeoutExpired:
        return {'status':'timeout','error_class':'timeout'}
    except (OSError,ValueError,json.JSONDecodeError) as e:
        return {'status':'crash','error_class':type(e).__name__,'message':str(e)[:150]}


def classify(results):
    statuses=[v['status'] for v in results.values()]
    if 'crash' in statuses or 'timeout' in statuses:return 'crash-or-timeout'
    accepted=[v for v in results.values() if v['status']=='ok']
    if not accepted:return 'all-reject'
    if len(accepted)<len(statuses):return 'accept-vs-reject'
    if len({(v['width'],v['height'],v['rgba_sha256']) for v in accepted})>1:
        return 'accept-different-pixels'
    return 'all-accept-identical-pixels'


def relation_signature(results):
    """Statuses and pairwise pixel equality, independent of actual pixel contents."""
    keys = list(results)
    return (tuple(results[k]['status'] for k in keys),
            tuple((a, b, results[a].get('width') == results[b].get('width'),
                   results[a].get('height') == results[b].get('height'),
                   results[a].get('rgba_sha256') == results[b].get('rgba_sha256'))
                  for i, a in enumerate(keys) for b in keys[i+1:]
                  if results[a]['status'] == results[b]['status'] == 'ok'))


def outcome_signature(results):
    """Preserve each acceptance, selected frame, dimensions and exact pixel hash."""
    return tuple((k, v['status'], v.get('selection'), v.get('width'),
                  v.get('height'), v.get('rgba_sha256')) for k, v in results.items())


def compact(results):
    return {k:{x:v[x] for x in ('status','error_class','message','width','height','rgba_sha256','alpha_present','orig_color_type','selection') if x in v}
            for k,v in results.items()}
