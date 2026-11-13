//! Allowlist file format (TOML, parsed by the `toml` crate with unknown keys
//! rejected) and the `[meta]` provenance checks.
//!
//! ```toml
//! [meta]
//! format = "alloc-audit/2"
//! crate_name = "tga_verus"
//! rustc_commit = "58cf8f9c35b355199727627d07ae34a4e2846d67"
//! target = "x86_64-unknown-linux-gnu"
//! profile = "opt-level=3,debug-assertions=off,share-generics=off"
//! features = []
//! source_hash = "<sha256>"
//! vstd_version = "0.0.0-2026-11-01-0251"     # or "none"
//! vstd_svh = "<crate hash of the linked vstd>"
//! vstd_source_checksum = "<Cargo.lock checksum of vstd>"
//! verus_builtin_svh = "<crate hash of the linked verus_builtin>"
//! reviewer = "<named human>"                 # empty until signed
//! drafted_by = "<who drafted it>"
//! required_roots = ["crate::tga::try_decode"]
//!
//! [[entry]]
//! kind = "alloc"            # alloc | intrinsic | no-mir | ffi | panic | crate |
//!                           # proc-macro | build-script | include | env |
//!                           # root-exclude | generic-export | guard-calls | dependency
//! item = "..."              # def-path (resolved to DefIds), crate name, file, ...
//! allow = "crate::..."      # admitted caller (alloc: required; panic: required)
//! contract = "..."          # what the item and everything it calls allocates
//! citation = "..."          # theorem / A-STD-n / std source at the pinned commit
//! reason = "..."
//! sha256 = "..."            # proc-macro, build-script, include
//! sites = 1                 # guard-calls
//! report = "..."            # dependency
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::report::Finding;

pub const PROTOTYPE_REVIEWER: &str = "prototype-fixture";
pub const FORMAT: &str = "alloc-audit/2";

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub format: Option<String>,
    pub crate_name: Option<String>,
    pub rustc_commit: Option<String>,
    pub target: Option<String>,
    pub profile: Option<String>,
    pub features: Option<Vec<String>>,
    pub source_hash: Option<String>,
    pub vstd_version: Option<String>,
    pub vstd: Option<String>,
    pub vstd_svh: Option<String>,
    pub vstd_source_checksum: Option<String>,
    pub verus_builtin_svh: Option<String>,
    pub reviewer: Option<String>,
    pub drafted_by: Option<String>,
    pub required_roots: Option<Vec<String>>,
    pub note: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub kind: String,
    pub item: String,
    pub allow: Option<String>,
    pub contract: Option<String>,
    pub citation: Option<String>,
    pub reason: Option<String>,
    pub reviewer: Option<String>,
    pub sha256: Option<String>,
    pub sites: Option<u64>,
    pub report: Option<String>,
    pub note: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct FileRepr {
    meta: Option<Meta>,
    #[serde(default)]
    entry: Vec<Entry>,
}

pub struct Allowlist {
    pub path: PathBuf,
    pub sha256: String,
    pub meta: Meta,
    pub entries: Vec<Entry>,
}

pub const LEAF_KINDS: &[&str] = &["intrinsic", "no-mir", "ffi", "panic"];
pub const BUILD_INPUT_KINDS: &[&str] = &["proc-macro", "build-script", "include", "env", "crate"];
pub const ALL_KINDS: &[&str] = &[
    "alloc", "intrinsic", "no-mir", "ffi", "panic", "crate", "proc-macro", "build-script",
    "include", "env", "root-exclude", "generic-export", "guard-calls", "dependency",
];

pub fn load(path: &Path, findings: &mut Vec<Finding>) -> Option<Allowlist> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            findings.push(Finding::new("allowlist-load", format!("cannot read {}: {e}", path.display())));
            return None;
        }
    };
    let sha256 = crate::inputs::sha256_hex(&bytes);
    let text = match String::from_utf8(bytes) {
        Ok(t) => t,
        Err(_) => {
            findings.push(Finding::new("allowlist-load", format!("{} is not UTF-8", path.display())));
            return None;
        }
    };
    let repr: FileRepr = match toml::from_str(&text) {
        Ok(r) => r,
        Err(e) => {
            findings.push(Finding::new(
                "allowlist-load",
                format!("{} does not parse as an alloc-audit allowlist: {e}", path.display()),
            ));
            return None;
        }
    };
    let meta = match repr.meta {
        Some(m) => m,
        None => {
            findings.push(Finding::new("allowlist-load", "allowlist has no [meta] table".into()));
            Meta::default()
        }
    };
    Some(Allowlist { path: path.to_path_buf(), sha256, meta, entries: repr.entry })
}

fn nonempty(v: &Option<String>) -> Option<String> {
    v.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// Facts about the running compilation that `[meta]` is compared against.
pub struct Facts {
    pub crate_name: String,
    pub commit_short: String,
    pub commit_full: Option<String>,
    pub target: String,
    pub profile: String,
    pub features: Vec<String>,
    pub source_hash: Option<String>,
    pub vstd_linked: bool,
    pub vstd_version: Option<String>,
    pub vstd_svh: Option<String>,
    pub vstd_source_checksum: Option<String>,
    pub vstd_problems: Vec<String>,
    pub verus_builtin_svh: Option<String>,
    pub prototype_mode: bool,
    pub vstd_override: Option<String>,
}

/// Every `[meta]` check. Each problem is its own fail-closed finding; a
/// missing or empty field is never a skipped check.
pub fn check_meta(a: &Allowlist, f: &Facts, out: &mut Vec<Finding>) {
    let m = &a.meta;
    match nonempty(&m.format) {
        Some(v) if v == FORMAT => {}
        other => out.push(Finding::new(
            "allowlist-format",
            format!("[meta] format is {other:?}; this tool reads {FORMAT:?}"),
        )),
    }
    match nonempty(&m.crate_name) {
        None => out.push(Finding::new("allowlist-crate-missing", format!("[meta] crate_name missing; audited crate is {}", f.crate_name))),
        Some(v) if v != f.crate_name => out.push(Finding::new(
            "allowlist-crate",
            format!("allowlist is for crate {v:?}; audited crate is {:?}", f.crate_name),
        )),
        _ => {}
    }
    // rustc commit: equal to the short token or the full hash, or a >=12-hex
    // prefix of the full hash. One-directional.
    match nonempty(&m.rustc_commit) {
        None => out.push(Finding::new(
            "allowlist-commit-missing",
            format!("no rustc_commit; running compiler is {} (full {:?})", f.commit_short, f.commit_full),
        )),
        Some(rec) => {
            let ok = rec == f.commit_short
                || f.commit_full.as_deref() == Some(rec.as_str())
                || f.commit_full.as_deref().is_some_and(|full| {
                    rec.len() >= 12
                        && rec.len() < full.len()
                        && rec.chars().all(|c| c.is_ascii_hexdigit())
                        && full.starts_with(rec.as_str())
                });
            if !ok {
                out.push(Finding::new(
                    "allowlist-commit",
                    format!("allowlist records rustc commit {rec:?}; running compiler is {} (full {:?})", f.commit_short, f.commit_full),
                ));
            }
        }
    }
    match nonempty(&m.target) {
        None => out.push(Finding::new("allowlist-target-missing", format!("no target; running target is {}", f.target))),
        Some(t) if t != f.target => out.push(Finding::new(
            "allowlist-target",
            format!("allowlist records target {t:?}; running target is {:?}", f.target),
        )),
        _ => {}
    }
    match nonempty(&m.profile) {
        None => out.push(Finding::new("allowlist-profile-missing", format!("no profile; this compilation is {}", f.profile))),
        Some(p) if p != f.profile => out.push(Finding::new(
            "allowlist-profile",
            format!("allowlist records profile {p:?}; this compilation is {:?}", f.profile),
        )),
        _ => {}
    }
    match &m.features {
        None => out.push(Finding::new(
            "allowlist-features-missing",
            format!("no features list; this compilation enables {:?}", f.features),
        )),
        Some(v) => {
            let mut rec = v.clone();
            rec.sort();
            if rec != f.features {
                out.push(Finding::new(
                    "allowlist-features",
                    format!("allowlist records features {rec:?}; this compilation enables {:?}", f.features),
                ));
            }
        }
    }
    match nonempty(&m.source_hash) {
        None => out.push(Finding::new(
            "allowlist-source-hash-missing",
            format!("no source_hash; the audited sources hash to {:?}", f.source_hash),
        )),
        Some(h) if Some(h.to_ascii_lowercase()) != f.source_hash => out.push(Finding::new(
            "allowlist-source-hash",
            format!("allowlist records source_hash {h:?}; the audited sources hash to {:?}", f.source_hash),
        )),
        _ => {}
    }
    // vstd.
    if f.vstd_override.is_some() && !f.prototype_mode {
        out.push(Finding::new(
            "vstd-override-forbidden",
            "ALLOC_AUDIT_VSTD_VERSION is set; the override is accepted only with --prototype (SPEC §6)".into(),
        ));
    }
    let declared_none = [&m.vstd, &m.vstd_version]
        .iter()
        .any(|v| nonempty(v).is_some_and(|s| s.eq_ignore_ascii_case("none")));
    if f.vstd_linked {
        if declared_none {
            out.push(Finding::new("allowlist-vstd", "allowlist declares vstd = \"none\" but vstd is linked".into()));
        } else {
            match nonempty(&m.vstd_version) {
                None => out.push(Finding::new(
                    "allowlist-vstd-missing",
                    format!("vstd is linked but no vstd_version is recorded (linked {:?})", f.vstd_version),
                )),
                Some(rec) => match &f.vstd_version {
                    None => out.push(Finding::new(
                        "allowlist-vstd-undetermined",
                        format!("vstd is linked but the tool cannot determine its version; allowlist records {rec:?}"),
                    )),
                    Some(act) if *act != rec => out.push(Finding::new(
                        "allowlist-vstd",
                        format!("allowlist records vstd_version {rec:?}; the linked vstd is {act:?}"),
                    )),
                    _ => {}
                },
            }
            // Artifact binding (production only; prototype runs compare the
            // version string, as the F1 prototype did).
            if !f.prototype_mode {
                for p in &f.vstd_problems {
                    out.push(Finding::new("allowlist-vstd-artifact", p.clone()));
                }
                match (nonempty(&m.vstd_svh), &f.vstd_svh) {
                    (None, act) => out.push(Finding::new(
                        "allowlist-vstd-artifact",
                        format!("no vstd_svh recorded; the linked vstd has crate hash {act:?}"),
                    )),
                    (Some(rec), Some(act)) if rec == *act => {}
                    (Some(rec), act) => out.push(Finding::new(
                        "allowlist-vstd-artifact",
                        format!("allowlist records vstd_svh {rec:?}; the linked vstd rlib has crate hash {act:?}"),
                    )),
                }
                match (nonempty(&m.vstd_source_checksum), &f.vstd_source_checksum) {
                    (Some(rec), Some(act)) if rec == *act => {}
                    (rec, act) => out.push(Finding::new(
                        "allowlist-vstd-artifact",
                        format!("allowlist records vstd_source_checksum {rec:?}; the linked vstd's locked source checksum is {act:?}"),
                    )),
                }
                match (nonempty(&m.verus_builtin_svh), &f.verus_builtin_svh) {
                    (Some(rec), Some(act)) if rec == *act => {}
                    (rec, act) => out.push(Finding::new(
                        "allowlist-vstd-artifact",
                        format!("allowlist records verus_builtin_svh {rec:?}; the linked verus_builtin has crate hash {act:?}"),
                    )),
                }
            }
        }
    } else if !declared_none {
        out.push(Finding::new(
            "allowlist-vstd-not-declared-none",
            "the audited crate does not link vstd; the allowlist must declare vstd = \"none\"".into(),
        ));
    }
    match nonempty(&m.reviewer) {
        None => out.push(Finding::new(
            "allowlist-reviewer",
            format!("reviewer is missing or empty (recorded {:?}); the allowlist is unsigned", m.reviewer),
        )),
        Some(r) if r == PROTOTYPE_REVIEWER && !f.prototype_mode => out.push(Finding::new(
            "allowlist-reviewer",
            "reviewer is the prototype placeholder; accepted only with --prototype".into(),
        )),
        _ => {}
    }
    // Entries: shape checks.
    for (i, e) in a.entries.iter().enumerate() {
        let id = format!("entry #{} (kind {:?}, item {:?})", i + 1, e.kind, e.item);
        if !ALL_KINDS.contains(&e.kind.as_str()) {
            out.push(Finding::new("allowlist-entry-invalid", format!("{id}: unknown kind")));
            continue;
        }
        if e.item.trim().is_empty() {
            out.push(Finding::new("allowlist-entry-invalid", format!("{id}: empty item")));
            continue;
        }
        if e.item.trim() == "*" {
            let tag = if BUILD_INPUT_KINDS.contains(&e.kind.as_str()) {
                "allowlist-wildcard-build-input"
            } else {
                "allowlist-wildcard"
            };
            out.push(Finding::new(tag, format!("{id}: `*` is accepted only as an alloc entry's `allow`")));
            continue;
        }
        let needs_contract = e.kind == "alloc" || LEAF_KINDS.contains(&e.kind.as_str());
        if needs_contract {
            if nonempty(&e.contract).is_none() {
                out.push(Finding::new("allowlist-entry-incomplete", format!("{id}: no `contract` (what the item allocates)")));
            }
            if nonempty(&e.citation).is_none() {
                out.push(Finding::new("allowlist-entry-incomplete", format!("{id}: no `citation`")));
            }
        }
        if (e.kind == "alloc" || e.kind == "panic") && nonempty(&e.allow).is_none() {
            out.push(Finding::new("allowlist-entry-incomplete", format!("{id}: `allow` (the admitted caller) is required")));
        }
        if let Some(al) = nonempty(&e.allow) {
            if al == "*" && e.kind != "alloc" {
                out.push(Finding::new("allowlist-wildcard", format!("{id}: `allow = \"*\"` is accepted only on alloc entries")));
            }
        }
        if ["proc-macro", "build-script", "include"].contains(&e.kind.as_str())
            && nonempty(&e.sha256).is_none()
            && !f.prototype_mode
        {
            out.push(Finding::new(
                "allowlist-entry-incomplete",
                format!("{id}: a build-input declaration must bind its content with `sha256`"),
            ));
        }
        if e.kind == "guard-calls" && e.sites.is_none() {
            out.push(Finding::new("allowlist-entry-incomplete", format!("{id}: `sites` is required")));
        }
        if e.kind == "dependency" && nonempty(&e.report).is_none() {
            out.push(Finding::new("allowlist-entry-incomplete", format!("{id}: `report` is required")));
        }
        if matches!(e.kind.as_str(), "root-exclude" | "generic-export") && nonempty(&e.citation).is_none() {
            out.push(Finding::new("allowlist-entry-incomplete", format!("{id}: `citation` is required")));
        }
    }
}

/// Entries of one kind.
pub fn of_kind<'a>(a: Option<&'a Allowlist>, kind: &str) -> impl Iterator<Item = &'a Entry> + 'a {
    let kind = kind.to_string();
    a.into_iter().flat_map(move |a| {
        let kind = kind.clone();
        a.entries.iter().filter(move |e| e.kind == kind)
    })
}

/// Resolve a path relative to the allowlist file's directory.
pub fn relative_to(a: &Allowlist, p: &str) -> PathBuf {
    // `$RUN/...` names a file in the current run directory (run_audit.sh sets
    // ALLOC_AUDIT_RUN_DIR); used for a dependency's report from the same run.
    let expanded;
    let p = match (p.strip_prefix("$RUN/"), std::env::var("ALLOC_AUDIT_RUN_DIR")) {
        (Some(rest), Ok(dir)) => {
            expanded = format!("{dir}/{rest}");
            expanded.as_str()
        }
        _ => p,
    };
    let pb = PathBuf::from(p);
    if pb.is_absolute() {
        pb
    } else {
        a.path.parent().unwrap_or(Path::new(".")).join(pb)
    }
}

/// `key = value` lines of a report header, for dependency checks.
pub fn read_report_header(path: &Path) -> Option<BTreeMap<String, String>> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut out = BTreeMap::new();
    for line in text.lines() {
        if line.starts_with("-- ") {
            break;
        }
        if let Some((k, v)) = line.split_once(": ") {
            out.entry(k.trim().to_string()).or_insert_with(|| v.trim().to_string());
        }
    }
    Some(out)
}
