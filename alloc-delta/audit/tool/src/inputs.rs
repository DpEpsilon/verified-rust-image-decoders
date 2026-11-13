//! Facts about the compilation and its inputs, read from the compiler session
//! (not from a text scan): profile, commit, target, cfgs, the source map, the
//! files and environment variables rustc tracked for dep-info, the crate graph
//! (proc-macro crates by crate kind, dylibs, vstd and its locked source), and
//! the build script (cargo's package metadata in the manifest directory).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rustc_hir::def::DefKind;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use rustc_span::def_id::{CrateNum, LOCAL_CRATE};
use rustc_span::{FileName, Symbol};
use sha2::{Digest, Sha256};

use crate::report::Finding;

pub fn sha256_hex(bytes: &[u8]) -> String {
    let d = Sha256::digest(bytes);
    d.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn sha256_file(p: &Path) -> Option<String> {
    std::fs::read(p).ok().map(|b| sha256_hex(&b))
}

/// `opt-level=<n>,debug-assertions=<on|off>,share-generics=<on|off>` and a
/// class (`release`, `debug`, `custom`).
pub fn detect_profile(tcx: TyCtxt<'_>) -> (String, String) {
    let opts = &tcx.sess.opts;
    let opt = match opts.optimize {
        config::OptLevel::No => "0",
        config::OptLevel::Less => "1",
        config::OptLevel::More => "2",
        config::OptLevel::Aggressive => "3",
        config::OptLevel::Size => "s",
        config::OptLevel::SizeMin => "z",
    };
    let dbg = opts.debug_assertions;
    let sg = opts.share_generics();
    let canon = format!(
        "opt-level={opt},debug-assertions={},share-generics={}",
        if dbg { "on" } else { "off" },
        if sg { "on" } else { "off" }
    );
    let class = if opts.optimize == config::OptLevel::No {
        "debug"
    } else if opts.optimize == config::OptLevel::Aggressive && !sg && !dbg {
        "release"
    } else {
        "custom"
    };
    (canon, class.to_string())
}

pub fn commit_short() -> String {
    let v = rustc_interface::util::rustc_version_str().unwrap_or("unknown");
    v.split('(')
        .nth(1)
        .and_then(|r| r.split_whitespace().next())
        .unwrap_or("unknown")
        .to_string()
}

pub fn commit_full(tcx: TyCtxt<'_>) -> Option<String> {
    let bin = tcx.sess.opts.sysroot.path().join("bin").join("rustc");
    let out = std::process::Command::new(bin).arg("-vV").output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    text.lines()
        .find_map(|l| l.strip_prefix("commit-hash:").map(|h| h.trim().to_string()))
        .filter(|h| !h.is_empty())
}

pub fn rustc_version_line() -> String {
    rustc_interface::util::rustc_version_str().unwrap_or("unknown").to_string()
}

/// Enabled cargo features and other `--cfg`s (sorted), and findings for
/// configurations SPEC §6 *Targets* excludes (`cfg(test)`, `cfg(verus_only)`).
pub fn cfgs(tcx: TyCtxt<'_>, out: &mut Vec<Finding>) -> (Vec<String>, Vec<String>) {
    let mut features = Vec::new();
    let mut other = Vec::new();
    let feature = Symbol::intern("feature");
    for (k, v) in tcx.sess.config.iter() {
        if *k == feature {
            if let Some(v) = v {
                features.push(v.to_string());
            }
        } else {
            match v {
                Some(v) => other.push(format!("{k}={v:?}")),
                None => other.push(k.to_string()),
            }
        }
    }
    features.sort();
    other.sort();
    if tcx.sess.is_test_crate() || other.iter().any(|c| c == "test") {
        out.push(Finding::new("cfg-test", "the audited compilation has cfg(test) on; the audit covers the production configuration only".into()));
    }
    if other.iter().any(|c| c == "verus_only") {
        out.push(Finding::new("cfg-verus-only", "the audited compilation has cfg(verus_only) on".into()));
    }
    (features, other)
}

pub struct SourceFiles {
    /// (path, sha256 of the file on disk, is_module_file)
    pub files: Vec<(String, String, bool)>,
    pub combined: Option<String>,
    /// Files rustc read for `include_bytes!`/`include_str!` (dep-info).
    pub file_depinfo: Vec<String>,
    /// Environment variables rustc tracked (`env!`/`option_env!`, proc-macro
    /// tracked env): (name, value).
    pub env_depinfo: Vec<(String, Option<String>)>,
}

fn rel(p: &Path) -> String {
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        if let Ok(r) = p.strip_prefix(Path::new(&dir)) {
            return r.display().to_string();
        }
    }
    p.display().to_string()
}

/// Every non-imported, real source-map file outside the sysroot, hashed from
/// disk, with rustc's own recorded source hash checked against the disk text
/// (so the hash is of the text that was compiled). A file is a *module file*
/// when it holds the inner span of a local module; every other file
/// (`include!` targets, `include_str!`/`include_bytes!` payloads) is a build
/// input that must be declared.
pub fn source_files(tcx: TyCtxt<'_>, out: &mut Vec<Finding>) -> SourceFiles {
    let sm = tcx.sess.source_map();
    let sysroot = tcx.sess.opts.sysroot.path().to_path_buf();
    // Files that contain a local module's inner span.
    let mut module_files: Vec<PathBuf> = Vec::new();
    for local in tcx.iter_local_def_id() {
        if matches!(tcx.def_kind(local), DefKind::Mod) {
            let (m, _, _) = tcx.hir_get_module(rustc_span::def_id::LocalModId::new_unchecked(local));
            let sp = m.spans.inner_span;
            if let FileName::Real(r) = sm.span_to_filename(sp) {
                if let Some(p) = r.local_path() {
                    module_files.push(p.to_path_buf());
                }
            }
        }
    }
    let mut files = Vec::new();
    for sf in sm.files().iter() {
        if sf.is_imported() {
            continue;
        }
        let FileName::Real(name) = &sf.name else { continue };
        let Some(path) = name.local_path() else { continue };
        if path.starts_with(&sysroot) {
            continue;
        }
        let Ok(bytes) = std::fs::read(path) else {
            out.push(Finding::new("source-unreadable", format!("cannot re-read {} from disk", path.display())));
            continue;
        };
        let is_module = module_files.iter().any(|m| m == path);
        if is_module {
            match std::str::from_utf8(&bytes) {
                Ok(text) if sf.src_hash.matches(text) => {}
                _ => out.push(Finding::new(
                    "source-changed",
                    format!("{} on disk does not match the text rustc compiled (src_hash)", path.display()),
                )),
            }
        }
        files.push((rel(path), sha256_hex(&bytes), is_module));
    }
    files.sort();
    files.dedup();
    let combined = if files.is_empty() {
        None
    } else {
        let mut hs: Vec<&str> = files.iter().map(|f| f.1.as_str()).collect();
        hs.sort_unstable();
        let mut text = String::new();
        for h in hs {
            text.push_str(h);
            text.push('\n');
        }
        Some(sha256_hex(text.as_bytes()))
    };
    let file_depinfo: Vec<String> =
        tcx.sess.file_depinfo.borrow().iter().map(|s| rel(Path::new(s.as_str()))).collect();
    let env_depinfo: Vec<(String, Option<String>)> = tcx
        .sess
        .env_depinfo
        .borrow()
        .iter()
        .map(|(k, v)| (k.to_string(), v.map(|v| v.to_string())))
        .collect();
    SourceFiles { files, combined, file_depinfo, env_depinfo }
}

pub struct CrateInfo {
    pub cnum: CrateNum,
    pub name: String,
    pub svh: String,
    pub dep_kind: String,
    pub proc_macro: bool,
    pub artifacts: Vec<(String, Option<String>)>,
    pub from_sysroot: bool,
}

pub fn crate_graph(tcx: TyCtxt<'_>) -> Vec<CrateInfo> {
    use rustc_metadata::creader::CStore;
    let sysroot = tcx.sess.opts.sysroot.path().to_path_buf();
    let pm_crates: Vec<CrateNum> = {
        let cstore = CStore::from_tcx(tcx);
        let mut v: Vec<CrateNum> = cstore.all_proc_macro_def_ids(tcx).map(|d| d.krate).collect();
        v.sort();
        v.dedup();
        v
    };
    let mut out = Vec::new();
    for &cnum in tcx.crates(()) {
        let src = tcx.used_crate_source(cnum);
        let mut artifacts = Vec::new();
        for p in src.paths() {
            artifacts.push((p.display().to_string(), sha256_file(p)));
        }
        let from_sysroot = src.paths().next().is_some() && src.paths().all(|p| p.starts_with(&sysroot));
        let dep_kind = tcx.crate_dep_kind(cnum);
        let macros_only = matches!(dep_kind, rustc_crate_store::CrateDepKind::MacrosOnly);
        out.push(CrateInfo {
            cnum,
            name: tcx.crate_name(cnum).to_string(),
            svh: tcx.crate_hash(cnum).to_hex(),
            dep_kind: format!("{dep_kind:?}"),
            proc_macro: pm_crates.contains(&cnum) || macros_only,
            artifacts,
            from_sysroot,
        });
    }
    out
}

/// Build-script detection from cargo's package data: `OUT_DIR` (cargo sets it
/// only for a package with a build script), a `build.rs` at the package root
/// (cargo's auto-detection), or `package.build` in `Cargo.toml`.
pub struct BuildScript {
    pub under_cargo: bool,
    pub present: bool,
    pub path: Option<PathBuf>,
    pub why: Vec<String>,
}

pub fn build_script() -> BuildScript {
    let mut why = Vec::new();
    if std::env::var_os("OUT_DIR").is_some() {
        why.push("OUT_DIR is set (cargo sets it only for a package with a build script)".to_string());
    }
    let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from) else {
        return BuildScript { under_cargo: false, present: !why.is_empty(), path: None, why };
    };
    let mut path = None;
    let manifest = dir.join("Cargo.toml");
    let mut explicit: Option<toml::Value> = None;
    if let Ok(text) = std::fs::read_to_string(&manifest) {
        if let Ok(v) = toml::from_str::<toml::Table>(&text) {
            explicit = v.get("package").and_then(|p| p.get("build")).cloned();
        }
    }
    match explicit {
        Some(toml::Value::Boolean(false)) => {}
        Some(toml::Value::String(s)) => {
            why.push(format!("Cargo.toml package.build = {s:?}"));
            path = Some(dir.join(s));
        }
        _ => {
            if dir.join("build.rs").exists() {
                why.push("build.rs exists at the package root (cargo auto-detects it)".into());
                path = Some(dir.join("build.rs"));
            }
        }
    }
    BuildScript { under_cargo: true, present: !why.is_empty(), path, why }
}

/// vstd identity: the version, the crate hash (SVH) of the linked rlib, the
/// locked registry checksum, and the content of the source tree the rlib was
/// compiled from. Problems are returned as strings (each becomes an
/// `allowlist-vstd-artifact` finding in production mode).
pub struct VstdInfo {
    pub linked: bool,
    pub version: Option<String>,
    pub svh: Option<String>,
    pub source_checksum: Option<String>,
    pub rlib_sha256: Option<String>,
    pub source_dir: Option<String>,
    pub problems: Vec<String>,
    pub verus_builtin_svh: Option<String>,
}

fn find_lock() -> Option<(PathBuf, String)> {
    let mut dir = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from)?;
    loop {
        let lock = dir.join("Cargo.lock");
        if let Ok(text) = std::fs::read_to_string(&lock) {
            return Some((lock, text));
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn lock_entry(lock: &str, name: &str) -> Vec<(String, Option<String>, Option<String>)> {
    // (version, source, checksum) of each [[package]] with this name.
    let Ok(v) = toml::from_str::<toml::Table>(lock) else { return vec![] };
    let Some(pkgs) = v.get("package").and_then(|p| p.as_array()) else { return vec![] };
    pkgs.iter()
        .filter(|p| p.get("name").and_then(|n| n.as_str()) == Some(name))
        .map(|p| {
            let s = |k: &str| p.get(k).and_then(|x| x.as_str()).map(|x| x.to_string());
            (s("version").unwrap_or_default(), s("source"), s("checksum"))
        })
        .collect()
}

pub fn vstd_info(tcx: TyCtxt<'_>, crates: &[CrateInfo], prototype: bool, override_v: Option<&str>) -> VstdInfo {
    let vs: Vec<&CrateInfo> = crates.iter().filter(|c| c.name == "vstd").collect();
    let vb: Vec<&CrateInfo> = crates.iter().filter(|c| c.name == "verus_builtin").collect();
    let mut info = VstdInfo {
        linked: !vs.is_empty(),
        version: None,
        svh: None,
        source_checksum: None,
        rlib_sha256: None,
        source_dir: None,
        problems: vec![],
        verus_builtin_svh: vb.first().map(|c| c.svh.clone()),
    };
    if vb.len() > 1 {
        info.problems.push(format!("{} crates named verus_builtin are linked", vb.len()));
    }
    if vs.is_empty() {
        return info;
    }
    if vs.len() > 1 {
        info.problems.push(format!("{} crates named vstd are linked; identity is ambiguous", vs.len()));
    }
    let c = vs[0];
    info.svh = Some(c.svh.clone());
    info.rlib_sha256 = c.artifacts.iter().find_map(|a| a.1.clone());
    let lock = find_lock();
    if prototype {
        // The F1 prototype's rule: override, else the Cargo.lock version.
        info.version = override_v.map(|s| s.to_string()).or_else(|| {
            lock.as_ref().and_then(|(_, t)| lock_entry(t, "vstd").first().map(|e| e.0.clone()))
        });
        return info;
    }
    // Production: the source tree the linked rlib was compiled from, read from
    // the crate root's file name recorded in its metadata.
    let root_span = tcx.def_span(c.cnum.as_def_id());
    let fname = tcx.sess.source_map().span_to_filename(root_span);
    let src_path = match &fname {
        FileName::Real(r) => r.local_path().map(|p| p.to_path_buf()),
        _ => None,
    };
    let Some(src_path) = src_path else {
        info.problems.push(format!("cannot locate the source of the linked vstd (crate root file {fname:?})"));
        return info;
    };
    // The package dir: the nearest ancestor of the crate-root file that has
    // a Cargo.toml (registry packages also carry .cargo-checksum.json there).
    let pkg_dir = src_path.ancestors().skip(1).find(|d| d.join("Cargo.toml").is_file()).map(|p| p.to_path_buf());
    let Some(pkg_dir) = pkg_dir else {
        info.problems.push(format!("no Cargo.toml above the linked vstd's crate root {}", src_path.display()));
        return info;
    };
    info.source_dir = Some(pkg_dir.display().to_string());
    let manifest_version = std::fs::read_to_string(pkg_dir.join("Cargo.toml"))
        .ok()
        .and_then(|t| toml::from_str::<toml::Table>(&t).ok())
        .and_then(|t| t.get("package").and_then(|p| p.get("version")).and_then(|v| v.as_str()).map(|s| s.to_string()));
    let (pkg_sum, tree_problems) = registry_tree_binding(&pkg_dir);
    info.problems.extend(tree_problems);
    // The locked resolution.
    match &lock {
        None => info.problems.push("no Cargo.lock found above CARGO_MANIFEST_DIR; the vstd resolution is not locked".into()),
        Some((lp, text)) => {
            let es = lock_entry(text, "vstd");
            if es.len() != 1 {
                info.problems.push(format!("{} has {} vstd entries", lp.display(), es.len()));
            } else {
                let (ver, source, sum) = &es[0];
                if !source.as_deref().unwrap_or("").starts_with("registry+") {
                    info.problems.push(format!("Cargo.lock vstd source is {source:?}, not a registry"));
                }
                if sum.is_none() || sum != &pkg_sum {
                    info.problems.push(format!(
                        "Cargo.lock vstd checksum {sum:?} differs from the compiled source tree's package checksum {pkg_sum:?}"
                    ));
                }
                if manifest_version.as_deref() != Some(ver.as_str()) {
                    info.problems.push(format!(
                        "Cargo.lock vstd version {ver:?} differs from the compiled source tree's version {manifest_version:?}"
                    ));
                }
                info.source_checksum = sum.clone();
            }
        }
    }
    info.version = manifest_version;
    info
}

/// The `[[package]]` entries of the audited package's Cargo.lock, hashed, for
/// the build-input inventory section of the report.
pub fn lock_summary() -> Option<(String, String)> {
    let (p, t) = find_lock()?;
    Some((p.display().to_string(), sha256_hex(t.as_bytes())))
}

pub fn local_svh(tcx: TyCtxt<'_>) -> String {
    tcx.crate_hash(LOCAL_CRATE).to_hex()
}

pub fn env_snapshot() -> BTreeMap<String, String> {
    let keys = ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_PKG_NAME", "CARGO_PKG_VERSION", "CARGO_MANIFEST_DIR", "OUT_DIR", "RUSTC_BOOTSTRAP"];
    keys.iter()
        .filter_map(|k| std::env::var(k).ok().map(|v| (k.to_string(), v)))
        .collect()
}

/// Bind a registry source tree to its locked checksum: the `.crate` archive in
/// cargo's registry cache must hash to the checksum (cargo's own rule), and the
/// extracted tree the compiler read must be byte-identical to the archive's
/// contents. Vendored sources (`.cargo-checksum.json`) are checked against
/// their per-file list instead. Returns (package checksum, problems).
pub fn registry_tree_binding(pkg_dir: &Path) -> (Option<String>, Vec<String>) {
    let mut problems = Vec::new();
    if let Some(cj) = std::fs::read_to_string(pkg_dir.join(".cargo-checksum.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
    {
        let pkg = cj.get("package").and_then(|v| v.as_str()).map(|s| s.to_string());
        if let Some(files) = cj.get("files").and_then(|v| v.as_object()) {
            for (f, h) in files {
                if sha256_file(&pkg_dir.join(f)).as_deref() != h.as_str() {
                    problems.push(format!("vendored source file {f} differs from .cargo-checksum.json"));
                }
            }
        }
        return (pkg, problems);
    }
    // <CARGO_HOME>/registry/src/<index>/<name-ver>  ->  <CARGO_HOME>/registry/cache/<index>/<name-ver>.crate
    let (Some(name), Some(index_dir)) = (pkg_dir.file_name(), pkg_dir.parent()) else {
        return (None, vec![format!("cannot parse registry path {}", pkg_dir.display())]);
    };
    let (Some(index), Some(src_root)) = (index_dir.file_name(), index_dir.parent()) else {
        return (None, vec![format!("cannot parse registry path {}", pkg_dir.display())]);
    };
    let Some(reg_root) = src_root.parent() else {
        return (None, vec![format!("cannot parse registry path {}", pkg_dir.display())]);
    };
    let archive = reg_root.join("cache").join(index).join(format!("{}.crate", name.to_string_lossy()));
    let Some(sum) = sha256_file(&archive) else {
        return (None, vec![format!("the source tree {} has no registry archive {} to bind it to", pkg_dir.display(), archive.display())]);
    };
    let tmp = std::env::temp_dir().join(format!("alloc-audit-crate-{}-{}", std::process::id(), sum.get(..12).unwrap_or("x")));
    let _ = std::fs::remove_dir_all(&tmp);
    if std::fs::create_dir_all(&tmp).is_err() {
        return (Some(sum), vec![format!("cannot create {}", tmp.display())]);
    }
    let ok = std::process::Command::new("tar")
        .arg("-xzf")
        .arg(&archive)
        .arg("-C")
        .arg(&tmp)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        let _ = std::fs::remove_dir_all(&tmp);
        return (Some(sum), vec![format!("cannot extract {}", archive.display())]);
    }
    let extracted = tmp.join(name);
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();
    collect_tree(&extracted, &extracted, &mut a);
    collect_tree(pkg_dir, pkg_dir, &mut b);
    b.remove(".cargo-ok");
    if a.is_empty() {
        problems.push(format!("archive {} extracted to nothing", archive.display()));
    }
    for (k, v) in &a {
        match b.get(k) {
            Some(w) if w == v => {}
            Some(_) => problems.push(format!("{}: {k} differs from the locked archive", pkg_dir.display())),
            None => problems.push(format!("{}: {k} is missing (present in the locked archive)", pkg_dir.display())),
        }
    }
    for k in b.keys() {
        if !a.contains_key(k) {
            problems.push(format!("{}: {k} is not in the locked archive", pkg_dir.display()));
        }
    }
    let _ = std::fs::remove_dir_all(&tmp);
    (Some(sum), problems)
}

fn collect_tree(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let Ok(ft) = e.file_type() else { continue };
        if ft.is_dir() {
            collect_tree(root, &p, out);
        } else if ft.is_file() {
            let rel = p.strip_prefix(root).unwrap_or(&p).display().to_string();
            out.insert(rel, sha256_file(&p).unwrap_or_default());
        }
    }
}
