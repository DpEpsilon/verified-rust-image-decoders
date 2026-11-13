//! Invocation classification and audit configuration.

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct AuditConfig {
    pub crate_name: String,
    pub allowlist_path: Option<PathBuf>,
    pub report_path: Option<PathBuf>,
    /// Extra roots named on the command line / environment.
    pub roots: Vec<String>,
    pub prototype_mode: bool,
    pub continue_on_fail: bool,
    pub emit_edges: bool,
    /// `ALLOC_AUDIT_VSTD_VERSION`, if set (refused outside prototype mode).
    pub vstd_override: Option<String>,
    /// The full rustc command line the audited compilation was started with.
    pub rustc_args: Vec<String>,
    /// How the tool was invoked: `wrapper` (by cargo) or `direct`.
    pub invocation: &'static str,
}

pub enum Invocation {
    /// Not a crate under audit and invoked as a wrapper: exec the real rustc.
    Passthrough { real_rustc: String, args: Vec<String> },
    /// Not a crate under audit, invoked directly: compile normally.
    Compile { args: Vec<String> },
    Audit { args: Vec<String>, config: AuditConfig },
}

fn is_rustc_path(s: &str) -> bool {
    Path::new(s).file_name().map(|f| f == "rustc").unwrap_or(false)
}

fn crate_name_of(args: &[String]) -> Option<String> {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--crate-name" {
            return it.next().cloned();
        }
        if let Some(v) = a.strip_prefix("--crate-name=") {
            return Some(v.to_string());
        }
    }
    None
}

fn has_input(args: &[String]) -> bool {
    args.iter().skip(1).any(|a| a.ends_with(".rs"))
}

pub fn classify(raw: &[String]) -> Result<Invocation, String> {
    let wrapper = raw.len() >= 2 && is_rustc_path(&raw[1]);
    let (real_rustc, args): (Option<String>, Vec<String>) = if wrapper {
        let mut a = vec![raw[0].clone()];
        a.extend(raw[2..].iter().cloned());
        (Some(raw[1].clone()), a)
    } else {
        (None, raw.to_vec())
    };
    let pass = |args: Vec<String>| match &real_rustc {
        Some(r) => Invocation::Passthrough { real_rustc: r.clone(), args },
        None => Invocation::Compile { args },
    };
    if !has_input(&args) {
        return Ok(pass(args));
    }
    let Some(crate_name) = crate_name_of(&args) else {
        return Ok(pass(args));
    };
    let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    let prototype_mode = std::env::var_os("ALLOC_AUDIT_PROTOTYPE").is_some();
    let roots: Vec<String> = env("ALLOC_AUDIT_ROOTS")
        .map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect())
        .unwrap_or_default();
    let mut config = AuditConfig {
        crate_name: crate_name.clone(),
        allowlist_path: None,
        report_path: None,
        roots,
        prototype_mode,
        continue_on_fail: std::env::var_os("ALLOC_AUDIT_CONTINUE").is_some(),
        emit_edges: std::env::var_os("ALLOC_AUDIT_EDGES").is_some(),
        vstd_override: std::env::var("ALLOC_AUDIT_VSTD_VERSION").ok(),
        rustc_args: args.clone(),
        invocation: if wrapper { "wrapper" } else { "direct" },
    };
    if let Some(plan) = env("ALLOC_AUDIT_PLAN") {
        let text = std::fs::read_to_string(&plan).map_err(|e| format!("cannot read plan {plan}: {e}"))?;
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split_whitespace().collect();
            if f.len() != 3 {
                return Err(format!("malformed plan line: {line:?}"));
            }
            if f[0] == crate_name {
                config.allowlist_path = if f[1] == "-" { None } else { Some(PathBuf::from(f[1])) };
                config.report_path = Some(PathBuf::from(f[2]));
                return Ok(Invocation::Audit { args, config });
            }
        }
        return Ok(pass(args));
    }
    if wrapper {
        // A wrapper run without a plan audits nothing; run_audit.sh always
        // sets a plan and treats a missing report as a failure.
        return Ok(pass(args));
    }
    config.allowlist_path = env("ALLOC_AUDIT_ALLOWLIST").map(PathBuf::from);
    config.report_path = env("ALLOC_AUDIT_REPORT").map(PathBuf::from);
    Ok(Invocation::Audit { args, config })
}
