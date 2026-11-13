#!/usr/bin/env python3
"""Run a command and record peak memory of its whole process tree.

Usage: python3 -I peakrss.py [--out FILE] [--interval SECS] -- CMD [ARGS...]

The command inherits stdout/stderr/stdin untouched and its exit code is
returned. On exit, one line is written to FILE (or stderr if no --out):
  peak_rss_sum_mb=<n>\tpeak_rss_max_proc_mb=<n>:<procname>\twall_s=<n.n>
See peakrss.md. Std-only; polls /proc (falls back to `ps -o rss=`).
"""
import os, signal, subprocess, sys, time

def read_proc(pid):
    """Return (ppid, rss_kb, name) or None. Prefers /proc/<pid>/status."""
    try:
        ppid = rss = None
        name = "?"
        with open("/proc/%d/status" % pid) as f:
            for line in f:
                if line.startswith("Name:"):
                    name = line.split(None, 1)[1].strip()
                elif line.startswith("PPid:"):
                    ppid = int(line.split()[1])
                elif line.startswith("VmRSS:"):
                    rss = int(line.split()[1])
        if ppid is None:
            return None
        if rss is None:  # kernel threads / zombies have no VmRSS
            rss = 0
        return ppid, rss, name
    except (OSError, ValueError):
        return None

def snapshot_proc():
    procs = {}
    for e in os.listdir("/proc"):
        if e.isdigit():
            r = read_proc(int(e))
            if r:
                procs[int(e)] = r
    return procs

def snapshot_ps():
    out = subprocess.run(["ps", "-e", "-o", "pid=,ppid=,rss=,comm="],
                         capture_output=True, text=True).stdout
    procs = {}
    for line in out.splitlines():
        p = line.split(None, 3)
        if len(p) == 4:
            procs[int(p[0])] = (int(p[1]), int(p[2]), p[3].strip())
    return procs

def proc_works():
    r = read_proc(os.getpid())
    return r is not None and r[1] > 0

def descendants(procs, root):
    kids = {}
    for pid, (ppid, _, _) in procs.items():
        kids.setdefault(ppid, []).append(pid)
    seen, stack = [], [root]
    while stack:
        p = stack.pop()
        if p in procs:
            seen.append(p)
        stack.extend(kids.get(p, []))
    return seen

def main():
    a = sys.argv[1:]
    out, interval = None, 0.25
    while a and a[0] != "--":
        if a[0] == "--out" and len(a) > 1:
            out = a[1]; a = a[2:]
        elif a[0] == "--interval" and len(a) > 1:
            interval = float(a[1]); a = a[2:]
        else:
            sys.exit("peakrss: bad option %r" % a[0])
    cmd = a[1:] if a else []
    if not cmd:
        sys.exit("usage: peakrss.py [--out FILE] -- CMD [ARGS...]")
    snap = snapshot_proc if proc_works() else snapshot_ps
    t0 = time.monotonic()
    try:
        child = subprocess.Popen(cmd)
    except OSError as e:
        print("peakrss: cannot run %s: %s" % (cmd[0], e), file=sys.stderr)
        sys.exit(127)
    for s in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(s, lambda n, f: child.send_signal(n))
    peak_sum = peak_one = 0
    peak_name = "?"
    def sample():
        nonlocal peak_sum, peak_one, peak_name
        try:
            procs = snap()
        except Exception:
            return
        tot = 0
        for pid in descendants(procs, child.pid):
            rss, name = procs[pid][1], procs[pid][2]
            tot += rss
            if rss > peak_one:
                peak_one, peak_name = rss, name
        peak_sum = max(peak_sum, tot)
    while child.poll() is None:
        sample()
        try:
            child.wait(timeout=interval)
        except subprocess.TimeoutExpired:
            pass
    wall = time.monotonic() - t0
    line = "peak_rss_sum_mb=%d\tpeak_rss_max_proc_mb=%d:%s\twall_s=%.1f\n" % (
        round(peak_sum / 1024), round(peak_one / 1024),
        peak_name.replace("\t", " ").replace(":", "_") or "?", wall)
    if out:
        with open(out, "w") as f:
            f.write(line)
    else:
        sys.stderr.write(line)
    rc = child.returncode
    sys.exit(rc if rc >= 0 else 128 - rc)

main()
