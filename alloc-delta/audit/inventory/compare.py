#!/usr/bin/env python3
"""Classify the captured build inputs and compare them with the audit report.

Usage: compare.py <out-dir> <crate-dir> <crate-name> <target-dir> <sysroot>
Writes <out-dir>/INVENTORY.txt and exits 0 (PASS) or 1 (BLOCK).

What blocks the proof report (each is listed with its evidence):
  untracked-read        a file opened successfully that is not a dep-info input
                        of a local package, not in the locked registry tree, the
                        sysroot, the target dir or a fixed system location, and
                        not declared
  undeclared-env-read   a build script or a compiler process asked for an
                        environment variable that is neither in the declared
                        environment nor a reviewed probe (KNOWN-ENV-PROBES.txt)
  report-mismatch       the audit report's source list / hashes, command line
                        or linked-artifact hashes differ from the captured ones
  no-report / build     the build or the report is missing
"""
import glob
import hashlib
import json
import os
import re
import sys

out, crate_dir, crate, tdir, sysroot = sys.argv[1:6]
here = os.path.dirname(os.path.abspath(__file__))
home = os.environ.get("HOME", "/home/ws")
findings, lines = [], []


def sha(p):
    try:
        return hashlib.sha256(open(p, "rb").read()).hexdigest()
    except OSError:
        return None


def F(tag, msg):
    findings.append((tag, msg))


declared_files, declared_env = set(), {}
for l in open(os.path.join(out, "declared.txt")):
    parts = l.strip().split(None, 1)
    if len(parts) == 2 and parts[0] == "file":
        declared_files.add(os.path.realpath(os.path.join(crate_dir, parts[1])))
    elif len(parts) == 2 and parts[0] == "env":
        k, _, v = parts[1].partition("=")
        declared_env[k] = v
environment = {}
for l in open(os.path.join(out, "environment.txt")):
    k, _, v = l.rstrip("\n").partition("=")
    environment[k] = v

# --- cargo messages: build units, build-script runs, local packages -------
units, scripts, local_pkgs = [], [], set()
for l in open(os.path.join(out, "cargo.json")):
    try:
        m = json.loads(l)
    except ValueError:
        continue
    if m.get("reason") == "compiler-artifact":
        units.append(m)
        pid = m.get("package_id", "")
        if pid.startswith("path+file://"):
            local_pkgs.add(os.path.realpath(pid[len("path+file://"):].split("#")[0]))
    elif m.get("reason") == "build-script-executed":
        scripts.append(m)
lines.append(f"build units (cargo compiler-artifact messages): {len(units)}")
for u in units:
    for fn in u.get("filenames", []):
        lines.append(f"  unit {u['package_id']} {u['target']['kind']} -> {fn} sha256={sha(fn)}")
lines.append(f"build scripts executed: {len(scripts)}")
for s in scripts:
    od = s.get("out_dir", "")
    lines.append(f"  build-script {s['package_id']} cfgs={s.get('cfgs')} env={s.get('env')} out_dir={od}")
    for f in sorted(glob.glob(os.path.join(od, "**"), recursive=True)):
        if os.path.isfile(f):
            lines.append(f"    out_dir file {f} sha256={sha(f)}")

# --- dep-info: rustc-tracked inputs of each local package ------------------
tracked = set()
dep_lists = {}
for d in glob.glob(os.path.join(out, "*.d")):
    for l in open(d):
        if ":" not in l or l.startswith("#"):
            continue
        tgt, deps = l.split(":", 1)
        for p in deps.split():
            if not p.startswith("/"):
                # cargo passes path-package sources relative to the package dir
                bases = [crate_dir, *sorted(local_pkgs)]
                a3 = os.path.realpath(crate_dir)
                while a3 != "/":
                    a3 = os.path.dirname(a3)
                    bases.append(a3)
                cands = [os.path.join(b, p) for b in bases]
                p = next((c for c in cands if os.path.exists(c)), cands[0])
            rp = os.path.realpath(p)
            tracked.add(rp)
            dep_lists.setdefault(os.path.basename(d), set()).add(rp)

# --- strace: execs and successful opens -------------------------------------
exe_of = {}
execs, opens = [], []
rx_exec = re.compile(r'^(\d+)\s+execve\("([^"]*)", \[(.*)\], .*\) = 0')
rx_chdir = re.compile(r'^(\d+)\s+chdir\("([^"]*)"\) = 0')
rx_clone = re.compile(r'^(\d+)\s+(?:(?:clone3?|v?fork)\(.*\)|<\.\.\. (?:clone3?|v?fork) resumed>.*) = (\d+)')
rx_open = re.compile(r'^(\d+)\s+(?:openat\(([^,]+), |open\()"([^"]*)", ([A-Z_|]+)[^)]*\) = (-?\d+\S*)')
cwd_of = {}
first_pid = None
for l in open(os.path.join(out, "strace.log"), errors="replace"):
    pm = re.match(r"^(\d+)\s", l)
    if pm and first_pid is None:
        first_pid = pm.group(1)
        cwd_of[first_pid] = os.path.realpath(crate_dir)
    m = rx_clone.match(l)
    if m:
        cwd_of[m.group(2)] = cwd_of.get(m.group(1), os.path.realpath(crate_dir))
        if m.group(1) in exe_of:
            exe_of[m.group(2)] = exe_of[m.group(1)]
        continue
    m = rx_chdir.match(l)
    if m:
        cwd_of[m.group(1)] = os.path.normpath(os.path.join(cwd_of.get(m.group(1), "/"), m.group(2)))
        continue
    m = rx_exec.match(l)
    if m:
        pid, exe, argv = m.groups()
        exe_of[pid] = exe
        execs.append((pid, exe, argv))
        continue
    m = rx_open.match(l)
    if m and int(m.group(5).split("<")[0]) >= 0 and "O_DIRECTORY" not in m.group(4):
        dirfd, path = m.group(2), m.group(3)
        if not path.startswith("/"):
            dm = re.match(r"^\d+<(.*)>$", dirfd or "")
            base = dm.group(1) if dm else cwd_of.get(m.group(1), os.path.realpath(crate_dir))
            path = os.path.join(base, path)
        opens.append((m.group(1), path, m.group(4)))
lines.append(f"processes exec'd: {len(execs)}; successful file opens: {len(opens)}")

reg = os.path.join(environment.get("CARGO_HOME", home + "/.cargo"), "registry")
allowed_prefixes = [
    os.path.realpath(sysroot) + "/", "/opt/rustup/", "/opt/cargo/", os.path.realpath(tdir) + "/",
    reg + "/", os.path.realpath(reg) + "/", "/proc/", "/sys/", "/dev/", "/lib/", "/lib64/", "/usr/lib/",
    "/usr/lib64/", "/usr/share/zoneinfo/", "/tmp/alloc-audit-crate-", "/tmp/claude-0/alloc-audit/",
    os.path.realpath(out) + "/",
    os.path.realpath(os.path.join(here, "..", "allowlist")) + "/",  # the audit's own allowlists (hashed in the report)
    "/tmp/cc",                      # gcc/collect2 temporaries of the link step
    "/etc/ssl/certs/",              # cargo's TLS setup (offline build; no network)
    os.path.realpath(environment.get("CARGO_HOME", home + "/.cargo")) + "/.package-cache",
    os.path.realpath(environment.get("CARGO_HOME", home + "/.cargo")) + "/.global-cache",
    os.path.realpath(os.path.join(environment.get("HOME", home), ".gitconfig")),
]
allowed_exact = {"/etc/ld.so.cache", "/etc/localtime", "/etc/passwd", "/etc/nsswitch.conf", "/etc/group",
                 "/etc/host.conf", "/etc/resolv.conf", "/etc/gai.conf", "/etc/hosts"}
local_meta = set()
for p in local_pkgs | {os.path.realpath(crate_dir)}:
    for f in ("Cargo.toml", "Cargo.lock", "build.rs"):
        local_meta.add(os.path.join(p, f))
    local_meta.add(os.path.join(p, ".cargo", "config.toml"))
anc = os.path.realpath(crate_dir)
while anc != "/":
    for f in ("Cargo.toml", "Cargo.lock"):
        local_meta.add(os.path.join(anc, f))
    anc = os.path.dirname(anc)
untracked = {}
for pid, path, flags in opens:
    rp = os.path.realpath(path) if path.startswith("/") else path
    if rp in tracked or rp in declared_files or rp in allowed_exact or rp in local_meta:
        continue
    if any(rp.startswith(a) for a in allowed_prefixes):
        continue
    if os.path.isdir(rp):
        continue
    if "/.git/" in rp and os.path.basename(exe_of.get(pid, "")) == "cargo":
        continue  # cargo's VCS probe of the enclosing repository (package metadata only)
    untracked.setdefault(rp, set()).add(os.path.basename(exe_of.get(pid, "?")))
for p, who in sorted(untracked.items()):
    F("untracked-read", f"{p} was opened by {sorted(who)} and is neither a dep-info input, a locked/sysroot/target file, nor declared")

# --- environment reads -------------------------------------------------------
probes = set()
pf = os.path.join(here, "KNOWN-ENV-PROBES.txt")
if os.path.exists(pf):
    for l in open(pf):
        l = l.split("#", 1)[0].strip()
        if l:
            probes.add(l)
def probe_ok(name):
    return any(name == p or (p.endswith("*") and name.startswith(p[:-1])) for p in probes)
script_env = {e[0] for sc in scripts for e in (sc.get("env") or []) if e}
env_reads = {}
gl = os.path.join(out, "getenv.log")
if os.path.exists(gl):
    for l in open(gl, errors="replace"):
        f = l.rstrip("\n").split("\t")
        if len(f) < 4:
            continue
        pid, exe, name, present = f[:4]
        env_reads.setdefault((os.path.basename(exe), name, present), 0)
        env_reads[(os.path.basename(exe), name, present)] += 1
lines.append(f"distinct (process, variable, present) environment reads: {len(env_reads)}")
for (exe, name, present), n in sorted(env_reads.items()):
    script = exe.startswith("build-script-") or exe.startswith("build_script_")
    lines.append(f"  env read {name} present={present} by {exe} x{n}")
    if name in environment or name in declared_env or name in script_env:
        continue
    if probe_ok(name):
        continue
    kind = "build script" if script else "process"
    F("undeclared-env-read", f"{kind} {exe} read environment variable {name} (present={present}); it is neither in the declared environment nor a reviewed probe")

# --- comparison with the audit report of the same build --------------------
rep = os.path.join(out, "report.txt")
cexit = open(os.path.join(out, "cargo.exit")).read().strip()
if cexit != "cargo exit 0":
    F("build", f"the traced build failed ({cexit}); see cargo.stderr")
if not os.path.exists(rep):
    F("no-report", "the audit report of the traced build is missing")
else:
    text = open(rep).read()
    rsrc = dict()
    sec = re.search(r"-- source files \(\d+\) --\n(.*?)\n\n", text, re.S)
    for l in (sec.group(1).splitlines() if sec else []):
        m = re.match(r"^\s+([0-9a-f]{64})\s+(\S+)", l)
        if m:
            rel = m.group(2)
            cands = [os.path.join(crate_dir, rel)]
            a2 = os.path.realpath(crate_dir)
            while a2 != "/":
                a2 = os.path.dirname(a2)
                cands.append(os.path.join(a2, rel))
            rsrc[os.path.realpath(next((c for c in cands if os.path.exists(c)), cands[0]))] = m.group(1)
    # Captured: dep-info inputs of the audited crate's own unit.
    mine = set()
    for dname, s in dep_lists.items():
        if dname.startswith(crate + "-") and dname.endswith(".d"):
            mine |= {p for p in s if not p.startswith(os.path.realpath(sysroot))}
    cap = {p: sha(p) for p in mine}
    lines.append(f"audited crate inputs: dep-info {len(cap)}, report {len(rsrc)}")
    for p in sorted(set(cap) | set(rsrc)):
        a, b = cap.get(p), rsrc.get(p)
        lines.append(f"  input {p} captured={a} report={b}")
        if a != b:
            F("report-mismatch", f"{p}: captured sha256 {a}, report {b}")
    # Command line: the core's rustc exec in the trace vs the report.
    cl = re.search(r"^  rustc command line: (.*)$", text, re.M)
    ex = [argv for pid, exe, argv in execs if f'"--crate-name", "{crate}"' in argv]
    if not ex:
        F("report-mismatch", f"no traced compiler exec for crate {crate}")
    elif cl:
        targs = re.findall(r'"((?:[^"\\]|\\.)*)"', ex[-1])
        rargs = cl.group(1).split(" ")
        if " ".join(targs[2:]) != " ".join(rargs[1:]):
            F("report-mismatch", "the report's rustc command line differs from the traced exec")
        lines.append("traced rustc command line: " + " ".join(targs))
        # Linked artifacts named on that command line, hashed here and in the report.
        for a in targs:
            m = re.match(r"^([A-Za-z0-9_]+)=(/.*\.(?:rlib|rmeta|so))$", a)
            if m:
                h = sha(m.group(2))
                lines.append(f"  extern {m.group(1)} {m.group(2)} sha256={h}")
                if f"{m.group(2)} sha256={h}" not in text:
                    F("report-mismatch", f"linked artifact {m.group(2)} (sha256 {h}) is not listed with that hash in the report")
    v = re.search(r"^verdict: (\S+)", text, re.M)
    lines.append(f"audit report verdict: {v.group(1) if v else '?'}")

verdict = "PASS" if not findings else "BLOCK"
with open(os.path.join(out, "INVENTORY.txt"), "w") as f:
    f.write("=== independent build-input inventory (alloc-delta/audit/inventory) ===\n")
    f.write(f"inventory-verdict: {verdict}\n")
    f.write(f"crate: {crate}\nfindings: {len(findings)}\n")
    f.write("finding-tags: " + (",".join(sorted({t for t, _ in findings})) or "-") + "\n")
    f.write("environment (reduced, declared):\n" + "".join(f"  {k}={v}\n" for k, v in environment.items()))
    f.write("\n-- findings --\n" + "".join(f"  [{t}] {m}\n" for t, m in findings))
    f.write("\n-- inventory --\n" + "".join(l + "\n" for l in lines))
print(f"inventory {crate}: {verdict}, {len(findings)} finding(s); {os.path.join(out, 'INVENTORY.txt')}")
sys.exit(0 if verdict == "PASS" else 1)
