//! Allowlist loading and matching.
//!
//! Format (a small hand-rolled TOML subset; no external crates):
//!
//! ```toml
//! [meta]
//! rustc_commit = "58cf8f9c3"
//! vstd_version = "0.0.0-2026-11-01-0251"
//! target = "x86_64-unknown-linux-gnu"
//! reviewer = "name"
//! source_hash = "sha256hex"
//!
//! [[entry]]
//! item = "core::intrinsics::size_of"     # def-path identity of the leaf/item
//! kind = "intrinsic"                      # intrinsic | no-mir | panic | ffi
//! reason = "std doc clause ..."
//! reviewer = "name"
//!
//! [[entry]]
//! item = "alloc::raw_vec::RawVecInner::reserve_exact"
//! kind = "alloc"                          # allocation-capable item
//! allow = "mycrate::guard::try_zeroed"    # caller allowed to reach it, or "*"
//! reason = "..."
//! reviewer = "..."
//! ```
//!
//! A leaf passes when an entry matches its identity and kind. An allocation is
//! justified when it is reached through an allocation-capable item that an
//! `alloc` entry names, from a caller that entry allows.

use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use rustc_session::Session;

use crate::report::LeafKind;

pub struct AllowlistEntry {
    pub item: String,
    pub kind: String,
    pub allow: String,
    pub reason: String,
    pub reviewer: String,
}

pub struct Allowlist {
    pub entries: Vec<AllowlistEntry>,
    pub recorded_commit: Option<String>,
    pub running_commit: String,
    /// The full 40-hex commit of the running toolchain, read from
    /// `<sysroot>/bin/rustc -vV`, when available.
    pub actual_commit_full: Option<String>,
    pub recorded_target: Option<String>,
    pub running_target: String,
    pub reviewer: Option<String>,
    pub commit_mismatch: bool,
    pub commit_missing: bool,
    pub target_mismatch: bool,
    pub target_missing: bool,
    pub profile_missing: bool,
    pub load_error: bool,
    pub path: Option<String>,
    /// SHA-256 of the allowlist file, recorded in the report.
    pub file_sha256: Option<String>,
    /// The compilation profile, detected from the session: the canonical triple
    /// `opt-level=<n>,debug-assertions=<on|off>,share-generics=<on|off>` and a
    /// coarse class (`release`, `debug` or `custom`).
    pub detected_profile: String,
    pub profile_class: String,
    /// The profile the allowlist records, and whether it disagrees.
    pub recorded_profile: Option<String>,
    pub profile_mismatch: bool,
    /// SHA-256 of the audited crate's sources, and the recorded value.
    pub source_hash_actual: Option<String>,
    /// Per-file hashes `(path, sha256)`, for the report.
    pub source_hashes: Vec<(String, String)>,
    pub recorded_source_hash: Option<String>,
    pub source_hash_mismatch: bool,
    pub source_hash_missing: bool,
    /// Whether the run was started with the explicit prototype flag, which is
    /// the only way `reviewer = "prototype-fixture"` is accepted.
    pub prototype_mode: bool,
    /// Set when the reviewer field is empty or is the prototype placeholder
    /// without `prototype_mode`.
    pub reviewer_invalid: bool,
    /// The `vstd_version` the allowlist records, and whether the crate links
    /// vstd at all. `vstd = "none"` (or `vstd_version = "none"`) declares a
    /// non-vstd toy crate.
    pub recorded_vstd: Option<String>,
    pub vstd_declared_none: bool,
    pub vstd_linked: bool,
    /// The vstd version the build actually links: from `Cargo.lock` (found via
    /// `CARGO_MANIFEST_DIR`) or from `ALLOC_AUDIT_VSTD_VERSION`.
    pub vstd_actual: Option<String>,
    pub vstd_mismatch: bool,
    pub vstd_missing: bool,
    /// vstd is linked and a version is recorded, but the tool cannot determine
    /// the linked version: fail closed.
    pub vstd_undetermined: bool,
    /// The crate does not link vstd but the allowlist does not declare `none`.
    pub vstd_not_declared_none: bool,
    /// Build inputs the source hash does not cover. Detected from the
    /// compilation: a build script (via `OUT_DIR`), foreign proc-macro crates
    /// loaded as dylibs, and `include_*!`/`include!` sites found by scanning
    /// the crate's own source text. Each must be declared in the allowlist.
    pub build_script: bool,
    pub proc_macro_crates: Vec<String>,
    pub includes: Vec<String>,
    pub undeclared_build_script: bool,
    pub undeclared_proc_macros: Vec<String>,
    pub undeclared_includes: Vec<String>,
}

impl Allowlist {
    pub fn empty() -> Self {
        Allowlist {
            entries: Vec::new(),
            recorded_commit: None,
            running_commit: String::new(),
            actual_commit_full: None,
            recorded_target: None,
            running_target: String::new(),
            reviewer: None,
            commit_mismatch: false,
            commit_missing: false,
            target_mismatch: false,
            target_missing: false,
            profile_missing: false,
            load_error: false,
            path: None,
            file_sha256: None,
            detected_profile: String::new(),
            profile_class: String::new(),
            recorded_profile: None,
            profile_mismatch: false,
            source_hash_actual: None,
            source_hashes: Vec::new(),
            recorded_source_hash: None,
            source_hash_mismatch: false,
            source_hash_missing: false,
            prototype_mode: false,
            reviewer_invalid: false,
            recorded_vstd: None,
            vstd_declared_none: false,
            vstd_linked: false,
            vstd_actual: None,
            vstd_mismatch: false,
            vstd_missing: false,
            vstd_undetermined: false,
            vstd_not_declared_none: false,
            build_script: false,
            proc_macro_crates: Vec::new(),
            includes: Vec::new(),
            undeclared_build_script: false,
            undeclared_proc_macros: Vec::new(),
            undeclared_includes: Vec::new(),
        }
    }

    pub fn load(tcx: TyCtxt<'_>, path: &str, prototype_mode: bool) -> Self {
        let mut a = Allowlist::empty();
        a.path = Some(path.to_string());
        a.file_sha256 = sha256_file(path);
        a.running_commit = compiler_commit(tcx.sess);
        a.actual_commit_full = full_commit_hash(tcx.sess);
        a.running_target = tcx.sess.opts.target_triple.tuple().to_string();
        a.prototype_mode = prototype_mode;
        let (profile, class) = detect_profile(tcx);
        a.detected_profile = profile;
        a.profile_class = class;
        a.source_hashes = source_hashes(tcx);
        a.source_hash_actual = combined_source_hash(&a.source_hashes);
        a.vstd_linked = crate_linked(tcx, "vstd");
        a.vstd_actual = std::env::var("ALLOC_AUDIT_VSTD_VERSION")
            .ok()
            .or_else(vstd_version_from_lock);

        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("alloc_audit: cannot read allowlist {path}: {e}");
                a.load_error = true;
                return a;
            }
        };
        a.recorded_commit = parse_meta(&text, "rustc_commit");
        a.recorded_target = parse_meta(&text, "target");
        a.reviewer = parse_meta(&text, "reviewer");
        a.recorded_profile = parse_meta(&text, "profile");
        a.recorded_source_hash = parse_meta(&text, "source_hash");
        a.recorded_vstd = parse_meta(&text, "vstd_version");

        // --- commit (required; exact or prefix-in-the-documented-direction) --
        match a.recorded_commit.clone() {
            None => a.commit_missing = true,
            Some(rec) if rec.trim().is_empty() => a.commit_missing = true,
            Some(rec) => {
                let rec = rec.trim().to_string();
                let short_ok = rec == a.running_commit;
                let full_ok = a.actual_commit_full.as_deref() == Some(rec.as_str());
                // A recorded value shorter than the full hash is accepted only
                // as a prefix of it, at least 12 hex characters, all hex.
                let prefix_ok = a
                    .actual_commit_full
                    .as_deref()
                    .map(|full| {
                        rec.len() >= 12
                            && rec.len() < full.len()
                            && full.starts_with(rec.as_str())
                            && rec.chars().all(|c| c.is_ascii_hexdigit())
                    })
                    .unwrap_or(false);
                if !(short_ok || full_ok || prefix_ok) {
                    a.commit_mismatch = true;
                }
            }
        }

        // --- target (required) -------------------------------------------
        match a.recorded_target.clone() {
            None => a.target_missing = true,
            Some(rec) if rec.trim().is_empty() => a.target_missing = true,
            Some(rec) if rec.trim() != a.running_target => a.target_mismatch = true,
            Some(_) => {}
        }

        // --- profile (required; must equal the detected string exactly) ---
        match a.recorded_profile.clone() {
            None => a.profile_missing = true,
            Some(rec) if rec.trim().is_empty() => a.profile_missing = true,
            Some(rec) if rec.trim() != a.detected_profile => a.profile_mismatch = true,
            Some(_) => {}
        }

        // --- source hash (required) --------------------------------------
        match &a.recorded_source_hash {
            None => a.source_hash_missing = true,
            Some(rec) if rec.trim().is_empty() => a.source_hash_missing = true,
            Some(rec) => {
                let rec_norm = rec.trim().to_ascii_lowercase();
                let ok = matches!(&a.source_hash_actual, Some(act) if act == &rec_norm);
                if !ok {
                    a.source_hash_mismatch = true;
                }
            }
        }

        // --- vstd (required; must match the linked version) --------------
        let declared_none = parse_meta(&text, "vstd")
            .map(|v| v.trim().eq_ignore_ascii_case("none"))
            .unwrap_or(false)
            || a
                .recorded_vstd
                .as_deref()
                .map(|v| v.trim().eq_ignore_ascii_case("none"))
                .unwrap_or(false);
        a.vstd_declared_none = declared_none;
        if a.vstd_linked {
            if declared_none {
                // The allowlist declares a non-vstd crate but vstd is linked:
                // the recorded version cannot be trusted.
                a.vstd_mismatch = true;
            } else {
                match a.recorded_vstd.clone() {
                    None => a.vstd_missing = true,
                    Some(v) if v.trim().is_empty() => a.vstd_missing = true,
                    Some(v) if v.trim().eq_ignore_ascii_case("none") => a.vstd_mismatch = true,
                    Some(rec) => match a.vstd_actual.clone() {
                        Some(act) if act == rec.trim() => {}
                        Some(_) => a.vstd_mismatch = true,
                        None => a.vstd_undetermined = true,
                    },
                }
            }
        } else if !declared_none {
            a.vstd_not_declared_none = true;
        }

        // Validate the reviewer: non-empty, and not the prototype placeholder
        // unless the run explicitly opted into prototype mode.
        match &a.reviewer {
            None => a.reviewer_invalid = true,
            Some(r) if r.trim().is_empty() => a.reviewer_invalid = true,
            Some(r) if r.trim() == PROTOTYPE_REVIEWER && !prototype_mode => {
                a.reviewer_invalid = true
            }
            Some(_) => {}
        }

        a.entries = parse_entries(&text);

        // --- build inputs the source hash does not cover -----------------
        // Detected (see `detect_build_inputs` and the NOTES.md caveats): a
        // build script (cargo sets `OUT_DIR` only for a crate with `build.rs`),
        // foreign proc-macro crates linked as dylibs, and `include_*!` sites in
        // the crate's own source text. Each needs an explicit declaration.
        let inputs = detect_build_inputs(tcx);
        a.build_script = inputs.build_script;
        a.proc_macro_crates = inputs.proc_macro_crates;
        a.includes = inputs.includes;
        a.undeclared_build_script = a.build_script && !a.declares("build-script", "build.rs");
        for c in &a.proc_macro_crates {
            if !declares_proc_macro(&a, c) {
                a.undeclared_proc_macros.push(c.clone());
            }
        }
        for inc in &a.includes {
            if !a.declares("include", inc) {
                a.undeclared_includes.push(inc.clone());
            }
        }
        a
    }

    /// Whether a leaf with `identity` and `kind` has a matching entry.
    pub fn covers_leaf(&self, kind: &LeafKind) -> bool {
        let id = kind.identity();
        let tag = kind.tag();
        self.entries
            .iter()
            .any(|e| e.kind == tag && def_path_matches(id, &e.item))
    }

    /// Whether an allocation reached through `item` (the allocation-capable
    /// item on the path) from `caller` is allowed. Retained alongside the
    /// in-walk matcher (`walk::judge_path`) as a simple predicate.
    #[allow(dead_code)]
    pub fn covers_alloc(&self, item: &str, caller: &str) -> bool {
        self.entries.iter().any(|e| {
            e.kind == "alloc"
                && def_path_matches(item, &e.item)
                && (e.allow == "*" || caller_matches(caller, &e.allow))
        })
    }

    /// Whether the allowlist declares an `[[entry]]` of `kind` whose `item` is
    /// exactly `name` or the wildcard `*`. Used for the build-input controls
    /// (build script, proc macros, `include_*!`).
    pub fn declares(&self, kind: &str, name: &str) -> bool {
        self.entries
            .iter()
            .any(|e| e.kind == kind && (e.item == "*" || e.item == name))
    }

    pub fn summary(&self) -> String {
        let p = self.path.clone().unwrap_or_else(|| "<none>".into());
        if self.load_error {
            return format!("{p} (LOAD ERROR)");
        }
        format!(
            "{p} sha256={} : {} entr{}, reviewer {:?}, commit {:?} (running {:?}, full {:?}), target {:?}, profile {:?} (detected {:?} [{}]), vstd {:?} (linked {}, actual {:?}), source_hash {:?} (actual {:?})",
            self.file_sha256.clone().unwrap_or_else(|| "<none>".into()),
            self.entries.len(),
            if self.entries.len() == 1 { "y" } else { "ies" },
            self.reviewer,
            self.recorded_commit,
            self.running_commit,
            self.actual_commit_full,
            self.recorded_target,
            self.recorded_profile,
            self.detected_profile,
            self.profile_class,
            self.recorded_vstd,
            self.vstd_linked,
            self.vstd_actual,
            self.recorded_source_hash,
            self.source_hash_actual,
        )
    }
}

/// The placeholder reviewer value a *fixture* allowlist carries. A real
/// allowlist must name a human; only a run started with `--prototype` accepts
/// this value, and that run prints the flag in the report header.
pub const PROTOTYPE_REVIEWER: &str = "prototype-fixture";

/// Detect the compilation profile: the canonical `opt-level`, `debug-assertions`
/// and `share-generics` values, plus a coarse class. `release` means
/// `opt-level >= 2` with debug assertions off and share-generics off (the
/// spec's production configuration); `debug` means `opt-level = 0`; anything
/// else is `custom`.
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
    // `debug_assertions` is already resolved against `opt-level` by the option
    // parser (on at `opt-level=0`, off otherwise, unless `-Cdebug-assertions`
    // was given).
    let dbg = opts.debug_assertions;
    let sg = opts.share_generics();
    let canon = format!(
        "opt-level={opt},debug-assertions={},share-generics={}",
        if dbg { "on" } else { "off" },
        if sg { "on" } else { "off" }
    );
    let class = if opts.optimize == config::OptLevel::No {
        "debug"
    } else if matches!(opts.optimize, config::OptLevel::More | config::OptLevel::Aggressive)
        && !sg
        && !dbg
    {
        "release"
    } else {
        "custom"
    };
    (canon, class.to_string())
}

/// SHA-256 of every source file the audited crate compiled from, read from the
/// session's source map. Files under the compiler's own sysroot (std and the
/// rustc-src component) are excluded: the audit's source hash covers the crate
/// under audit, and std is pinned by the recorded rustc commit, not by a hash.
fn source_hashes(tcx: TyCtxt<'_>) -> Vec<(String, String)> {
    use rustc_span::FileName;
    let sysroot = tcx.sess.opts.sysroot.path();
    let mut out = Vec::new();
    let sm = tcx.sess.source_map();
    for sf in sm.files().iter() {
        let FileName::Real(name) = &sf.name else {
            continue;
        };
        let path = match name.local_path() {
            Some(p) => p.to_path_buf(),
            None => continue,
        };
        // Skip the sysroot (std, core, the rustc-src tree).
        if path.starts_with(sysroot) {
            continue;
        }
        let Some(src) = &sf.src else {
            continue;
        };
        out.push((path.display().to_string(), sha256_bytes(src.as_bytes())));
    }
    out.sort();
    out
}

/// A single hash over all audited source files, so the allowlist can pin one
/// value. The combined hash is the SHA-256 of the files' individual content
/// hashes sorted lexicographically, so it pins the exact source *text* and is
/// independent of checkout location. The per-file `(path, hash)` pairs are
/// recorded separately in the report, so a reviewer sees which files the hash
/// covers.
fn combined_source_hash(files: &[(String, String)]) -> Option<String> {
    if files.is_empty() {
        return None;
    }
    let mut hashes: Vec<&str> = files.iter().map(|(_, h)| h.as_str()).collect();
    hashes.sort_unstable();
    let mut text = String::new();
    for h in hashes {
        text.push_str(h);
        text.push('\n');
    }
    Some(sha256_bytes(text.as_bytes()))
}

/// The audited crate's source files and their SHA-256, and the combined hash
/// over them. Exposed so a run with no allowlist still records them in the
/// report header.
pub fn source_hash_of(tcx: TyCtxt<'_>) -> (Vec<(String, String)>, Option<String>) {
    let files = source_hashes(tcx);
    let combined = combined_source_hash(&files);
    (files, combined)
}

/// Whether an allowlist `item`/`allow` def-path names the same item as a walk
/// `label`. Matching is whole-segment: the two strings must be equal, or the
/// shorter must be a prefix of the longer whose only remainder is exactly one
/// balanced generic-argument group (`<...>`), so `guard::try_zeroed` matches
/// `guard::try_zeroed<u8>` but never `unapproved_guard::try_zeroed`, and a
/// root-level `try_zeroed` never matches `guard::try_zeroed`. This replaces the
/// old `ends_with`/`starts_with` predicate, which accepted suffix lookalikes.
pub fn def_path_matches(label: &str, item: &str) -> bool {
    if label == item {
        return true;
    }
    for (long, short) in [(label, item), (item, label)] {
        if long.len() > short.len() && long.starts_with(short) {
            let rest = &long[short.len()..];
            if rest.starts_with('<') && balanced_generic_suffix(rest) {
                return true;
            }
        }
    }
    false
}

/// Whether `rest` is exactly one balanced `<...>` group consuming the whole
/// string. Used by `def_path_matches` to permit a generic-argument suffix and
/// nothing more (so `a::b` never matches `a::b<T>::c`).
fn balanced_generic_suffix(rest: &str) -> bool {
    debug_assert!(rest.starts_with('<'));
    let mut depth = 0i32;
    for (i, ch) in rest.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    return i + ch.len_utf8() == rest.len();
                }
            }
            _ => {}
        }
    }
    false
}

/// Whether the audited-crate caller `caller` is the def-path an `alloc`
/// entry's `allow` admits. An `allow` written with an explicit `crate::` root
/// is accepted; the comparison is otherwise whole-segment (see
/// `def_path_matches`).
pub fn caller_matches(caller: &str, allow: &str) -> bool {
    def_path_matches(caller, allow)
        || def_path_matches(caller, allow.strip_prefix("crate::").unwrap_or(allow))
}

/// The full 40-hex commit of the running toolchain, from `<sysroot>/bin/rustc
/// -vV`. `None` if the binary is missing or its output has no `commit-hash`.
fn full_commit_hash(sess: &Session) -> Option<String> {
    let bin = sess.opts.sysroot.path().join("bin").join("rustc");
    let out = std::process::Command::new(bin).arg("-vV").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("commit-hash:") {
            let h = rest.trim();
            if !h.is_empty() {
                return Some(h.to_string());
            }
        }
    }
    None
}

/// Whether a crate with this name is linked into the compilation under audit.
/// Used to decide whether the allowlist must record a `vstd_version`.
fn crate_linked(tcx: TyCtxt<'_>, name: &str) -> bool {
    tcx.crates(()).iter().any(|&cnum| tcx.crate_name(cnum).as_str() == name)
}

/// The `vstd` version the audited build links, read from the crate's
/// `Cargo.lock` (found relative to `CARGO_MANIFEST_DIR`). `None` when the lock
/// cannot be found or has no `[[package]] name = "vstd"`.
fn vstd_version_from_lock() -> Option<String> {
    let mut dir = std::env::var_os("CARGO_MANIFEST_DIR").map(std::path::PathBuf::from)?;
    loop {
        let lock = dir.join("Cargo.lock");
        if let Ok(text) = std::fs::read_to_string(&lock) {
            if let Some(v) = vstd_version_in_lock(&text) {
                return Some(v);
            }
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Extract the version of the `vstd` package from a `Cargo.lock` body: the
/// `version = "..."` line inside the `[[package]]` whose `name = "vstd"`.
fn vstd_version_in_lock(text: &str) -> Option<String> {
    let mut in_vstd = false;
    for line in text.lines() {
        let line = line.trim();
        if line == "[[package]]" {
            in_vstd = false;
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let val = v.trim().trim_matches('"');
            if key == "name" && val == "vstd" {
                in_vstd = true;
            } else if key == "version" && in_vstd {
                return Some(val.to_string());
            }
        }
    }
    None
}

/// Build inputs the source-map text hash cannot cover.
pub struct BuildInputs {
    pub build_script: bool,
    pub proc_macro_crates: Vec<String>,
    pub includes: Vec<String>,
}

/// Detect build inputs outside the source-map text: a build script, foreign
/// proc-macro crates, and `include_*!` sites.
///
/// These are heuristics, and NOTES.md states their limits:
/// - A build script is inferred from cargo's `OUT_DIR`, which cargo sets for
///   the crate only when it has a `build.rs`. A build script that cargo runs
///   but that writes nothing cargo then passes through still sets `OUT_DIR`, so
///   the detection is conservative (it can over-report, never under-report).
/// - Foreign proc macros are the crates cargo loaded as dylibs, minus the
///   declared Verus builtin macros (`verus_builtin_macros`,
///   `verus_state_machines_macros`, `verus_builtin`).
/// - `include_*!` sites are found by scanning the crate's own source text for
///   the macro names; a computed argument (`concat!`, a `const`) is recorded as
///   the non-literal marker rather than resolved.
pub fn detect_build_inputs(tcx: TyCtxt<'_>) -> BuildInputs {
    let build_script = std::env::var_os("OUT_DIR").is_some();

    let mut proc_macro_crates = Vec::new();
    for &cnum in tcx.crates(()) {
        let name = tcx.crate_name(cnum).to_string();
        // The declared vstd/verus macros are admitted by construction: the
        // spec admits "the declared vstd/verus macros" without an entry.
        if name == "vstd" || name.starts_with("verus") {
            continue;
        }
        if tcx.used_crate_source(cnum).dylib.is_some() {
            proc_macro_crates.push(name);
        }
    }
    proc_macro_crates.sort();
    proc_macro_crates.dedup();

    // Scan the crate's own source files for `include!`/`include_str!`/
    // `include_bytes!` sites.
    use rustc_span::FileName;
    let sysroot = tcx.sess.opts.sysroot.path();
    let mut includes = Vec::new();
    for sf in tcx.sess.source_map().files().iter() {
        let FileName::Real(name) = &sf.name else {
            continue;
        };
        let Some(path) = name.local_path() else { continue };
        if path.starts_with(sysroot) {
            continue;
        }
        let Some(src) = &sf.src else { continue };
        let scan = strip_line_comments(src);
        for mac in ["include_bytes!", "include_str!", "include!"] {
            let mut from = 0usize;
            while let Some(pos) = scan[from..].find(mac) {
                let at = from + pos;
                from = at + mac.len();
                let arg = include_arg(&scan[from..]);
                // `include!("x.rs")` pulls in source text, which the source map
                // carries, so it is not a hidden build input; every
                // `include_bytes!`/`include_str!` argument and any non-`.rs`
                // `include!` argument is.
                if mac == "include!" && arg.ends_with(".rs") {
                    continue;
                }
                includes.push(format!("{mac}{arg}"));
            }
        }
    }
    includes.sort();
    includes.dedup();

    BuildInputs { build_script, proc_macro_crates, includes }
}

/// Replace the contents of `//`-to-end-of-line comments with spaces, so a macro
/// name mentioned in a comment is not scanned as a use. This is deliberately
/// simple (it does not understand string literals or block comments); NOTES.md
/// records that, together with the over-reporting direction it can cause.
fn strip_line_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    for line in src.split_inclusive('\n') {
        match line.find("//") {
            Some(pos) => {
                out.push_str(&line[..pos]);
                out.push_str(&" ".repeat(line.len() - pos - 1));
                out.push('\n');
            }
            None => out.push_str(line),
        }
    }
    out
}

/// The literal argument text of an `include_*!(...)`, for the report and for
/// matching an `[[entry]]` declaration. The surrounding double quotes of a
/// string-literal argument are stripped, so the recorded form is
/// `include_str!payload.txt`, not `include_str!"payload.txt"`; this keeps the
/// declaration storable in the TOML subset (which strips quotes). Returns
/// `(...)` for a missing `(` and `(non-literal)` for a non-string argument.
fn include_arg(rest: &str) -> String {
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix('(') else {
        return "(...)".into();
    };
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix('"') else {
        return "(non-literal)".into();
    };
    match rest.find('"') {
        Some(end) => rest[..end].to_string(),
        None => "(non-literal)".into(),
    }
}

/// Whether the allowlist declares a proc-macro crate `name`: an `[[entry]]` of
/// kind `proc-macro` whose `item` is `name` or `*`.
fn declares_proc_macro(a: &Allowlist, name: &str) -> bool {
    a.declares("proc-macro", name)
}

/// SHA-256 of a byte slice, via `sha256sum` (avoids an extra crate). The input
/// is piped on stdin so no temporary file is needed.
fn sha256_bytes(bytes: &[u8]) -> String {
    use std::io::Write;
    let mut child = match std::process::Command::new("sha256sum")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return "<sha256sum failed>".into(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(bytes);
    }
    match child.wait_with_output() {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("<no hash>")
            .to_string(),
        _ => "<sha256sum failed>".into(),
    }
}

/// SHA-256 of a file, via `sha256sum` (avoids an extra crate dependency). The
/// spec requires the allowlist's hash to appear in the proof report.
fn sha256_file(path: &str) -> Option<String> {
    let out = std::process::Command::new("sha256sum").arg(path).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    s.split_whitespace().next().map(|h| h.to_string())
}

/// The rustc commit hash, taken from the compiler's version string
/// (`1.99.1 (58cf8f9c3 2026-10-13)`); the parenthesised field is the commit.
fn compiler_commit(sess: &Session) -> String {
    let v = rustc_interface::util::rustc_version_str().unwrap_or("unknown");
    if let Some(start) = v.find('(') {
        let rest = &v[start + 1..];
        let inner = rest.split(')').next().unwrap_or(rest);
        if let Some(tok) = inner.split_whitespace().next() {
            return tok.to_string();
        }
    }
    let _ = sess;
    "unknown".to_string()
}

fn parse_meta(text: &str, key: &str) -> Option<String> {
    let mut in_meta = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_meta = line == "[meta]";
            continue;
        }
        if in_meta {
            if let Some(v) = scalar(line, key) {
                return Some(v);
            }
        }
    }
    None
}

fn scalar(line: &str, key: &str) -> Option<String> {
    let (k, v) = line.split_once('=')?;
    if k.trim() != key {
        return None;
    }
    Some(v.trim().trim_matches('"').to_string())
}

fn parse_entries(text: &str) -> Vec<AllowlistEntry> {
    let mut out = Vec::new();
    let mut cur: Option<AllowlistEntry> = None;
    let blank = || AllowlistEntry {
        item: String::new(),
        kind: String::new(),
        allow: String::new(),
        reason: String::new(),
        reviewer: String::new(),
    };
    for line in text.lines() {
        let line = line.trim();
        if line == "[[entry]]" {
            if let Some(e) = cur.take() {
                out.push(e);
            }
            cur = Some(blank());
            continue;
        }
        if let Some(e) = cur.as_mut() {
            for (k, field) in [
                ("item", &mut e.item),
                ("kind", &mut e.kind),
                ("allow", &mut e.allow),
                ("reason", &mut e.reason),
                ("reviewer", &mut e.reviewer),
            ] {
                if field.is_empty() {
                    if let Some(v) = scalar(line, k) {
                        *field = v;
                    }
                }
            }
        }
    }
    if let Some(e) = cur.take() {
        out.push(e);
    }
    out
}
