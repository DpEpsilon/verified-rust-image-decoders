//! Report data model and rendering.

use std::collections::{BTreeMap, BTreeSet};

/// One traversed call-graph edge.
#[derive(Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub kind: String,
    pub span: String,
}

/// A leaf: an edge with no walkable body, needing an allowlist entry.
#[derive(Clone)]
pub struct Leaf {
    pub item: String,
    pub kind: LeafKind,
}

#[derive(Clone)]
pub enum LeafKind {
    /// A `rustc` intrinsic, allowed per name.
    Intrinsic(String),
    /// An `extern` item (FFI). Fails closed.
    Foreign(String),
    /// Reached an item with no available MIR (precompiled std body, or an
    /// upstream body without exported MIR).
    NoMir(String),
    /// A panic entry point; justified by a theorem citation.
    Panic(String),
}

impl LeafKind {
    pub fn tag(&self) -> &'static str {
        match self {
            LeafKind::Intrinsic(_) => "intrinsic",
            LeafKind::Foreign(_) => "ffi",
            LeafKind::NoMir(_) => "no-mir",
            LeafKind::Panic(_) => "panic",
        }
    }
    /// The identity an allowlist entry must match.
    pub fn identity(&self) -> &str {
        match self {
            LeafKind::Intrinsic(s)
            | LeafKind::Foreign(s)
            | LeafKind::NoMir(s)
            | LeafKind::Panic(s) => s,
        }
    }
}

#[derive(Clone)]
pub struct Finding {
    pub kind: String,
    pub detail: String,
}

impl Finding {
    pub fn new(kind: &str, detail: String) -> Self {
        Finding {
            kind: kind.to_string(),
            detail,
        }
    }
}

/// One reachable allocator entry point.
#[derive(Clone)]
pub struct AllocHit {
    /// The allocator symbol/def-path reached.
    pub entry: String,
    /// The full root-to-entry path.
    pub path: String,
    /// The allocation-capable item on the path that the allowlist names, if any.
    pub via_item: Option<String>,
    /// The caller before `via_item` that the allowlist admits, if any.
    pub via_caller: Option<String>,
    /// Whether every reachable path to this entry is justified.
    pub justified: bool,
    /// Unauthorized paths to this entry, each `(path, item, caller)`. Bounded.
    pub offending: Vec<(String, String, String)>,
}

/// Verdict for the whole audit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    Pass,
    Fail,
}

pub struct AuditReport {
    pub nodes: BTreeSet<String>,
    pub edges: Vec<Edge>,
    pub leaves: BTreeMap<String, Leaf>,
    /// Fail-closed findings (indirect calls, dyn calls, FFI, asm, share-generics).
    pub findings: Vec<Finding>,
    /// Every reachable allocator entry point.
    pub allocations: Vec<AllocHit>,
    pub allowlist_note: String,
    /// The audited crate's source files and their SHA-256, so a reviewer sees
    /// which text the recorded `source_hash` covers.
    pub source_hashes: Vec<(String, String)>,
    /// The combined source hash the allowlist must record.
    pub source_hash_actual: Option<String>,
    /// The compilation profile detected (canonical string and class).
    pub detected_profile: String,
    pub profile_class: String,
    /// Whether the run was started in explicit prototype mode; printed in the
    /// header, so a reader can see the `prototype-fixture` reviewer was accepted
    /// by an explicit opt-in and not by default.
    pub prototype_mode: bool,
    /// Build inputs the source-map text hash does not cover, so the header can
    /// print them: a build script, foreign proc-macro crates, and `include_*!`
    /// sites.
    pub build_script: bool,
    pub proc_macro_crates: Vec<String>,
    pub includes: Vec<String>,
}

impl AuditReport {
    pub fn new(
        nodes: BTreeSet<String>,
        edges: Vec<Edge>,
        leaves: BTreeMap<String, Leaf>,
        findings: Vec<Finding>,
        allocations: Vec<AllocHit>,
        _node_count: usize,
    ) -> Self {
        AuditReport {
            nodes,
            edges,
            leaves,
            findings,
            allocations,
            allowlist_note: String::new(),
            source_hashes: Vec::new(),
            source_hash_actual: None,
            detected_profile: String::new(),
            profile_class: String::new(),
            prototype_mode: false,
            build_script: false,
            proc_macro_crates: Vec::new(),
            includes: Vec::new(),
        }
    }

    pub fn unexplained_allocs(&self) -> impl Iterator<Item = &AllocHit> {
        self.allocations.iter().filter(|a| !a.justified)
    }

    /// Fail-closed: any unresolved/refused edge, any unexplained leaf, or any
    /// allocation not covered by an allowlist entry fails the audit.
    pub fn passed(&self) -> bool {
        self.findings.is_empty()
            && self.leaves.is_empty()
            && self.unexplained_allocs().next().is_none()
    }

    pub fn verdict(&self) -> Verdict {
        if self.passed() {
            Verdict::Pass
        } else {
            Verdict::Fail
        }
    }
}

pub fn emit(report: &AuditReport, report_path: Option<&str>) {
    let mut out = String::new();
    let justified = report.allocations.iter().filter(|a| a.justified).count();
    let unexplained = report.allocations.len() - justified;
    out.push_str("=== alloc_audit report ===\n");
    out.push_str(&format!("verdict: {:?}\n", report.verdict()));
    out.push_str(&format!(
        "profile: {} [{}]\n",
        report.detected_profile, report.profile_class
    ));
    out.push_str(&format!(
        "source_hash (audited crate): {}\n",
        report.source_hash_actual.clone().unwrap_or_else(|| "<none>".into())
    ));
    // State plainly what the hash covers. The hash is over the source-map text
    // of the crate's own files; it does not cover include_bytes!/include_str!
    // payloads, build-script output, proc-macro output, or dependency text.
    out.push_str("source-hash scope: source-map text only (crate's own files; excludes include_bytes!/include_str! payloads, build-script and proc-macro output, and dependency sources)\n");
    for (path, hash) in &report.source_hashes {
        out.push_str(&format!("  source {hash}  {path}\n"));
    }
    if report.build_script {
        out.push_str("build input: build script present (output not covered by source hash)\n");
    }
    for c in &report.proc_macro_crates {
        out.push_str(&format!("build input: proc-macro crate {c} (output not covered by source hash)\n"));
    }
    for inc in &report.includes {
        out.push_str(&format!("build input: {inc} present in source (payload not covered by source hash)\n"));
    }
    if report.prototype_mode {
        out.push_str(
            "mode: PROTOTYPE (--prototype given; the `prototype-fixture` reviewer is accepted)\n",
        );
    }
    out.push_str(&format!(
        "nodes (monomorphised instances walked): {}\n",
        report.nodes.len()
    ));
    out.push_str(&format!("edges: {}\n", report.edges.len()));
    out.push_str(&format!(
        "allocator entry points reachable: {} ({} allowlisted, {} unexplained)\n",
        report.allocations.len(),
        justified,
        unexplained
    ));
    out.push_str(&format!("fail-closed findings: {}\n", report.findings.len()));
    out.push_str(&format!("unexplained leaves: {}\n", report.leaves.len()));
    if !report.allowlist_note.is_empty() {
        out.push_str(&format!("allowlist: {}\n", report.allowlist_note));
    }

    if !report.allocations.is_empty() {
        out.push_str("\n-- allocator entry points --\n");
        for a in &report.allocations {
            let mark = if a.justified { "OK " } else { "!! " };
            out.push_str(&format!("  {mark}{}\n      path: {}\n", a.entry, a.path));
            if let Some(item) = &a.via_item {
                out.push_str(&format!(
                    "      via {item} (allowlist caller {})\n",
                    a.via_caller.clone().unwrap_or_else(|| "<none>".into())
                ));
            }
            for (path, item, caller) in &a.offending {
                out.push_str(&format!(
                    "      !!! unauthorized path: {path}\n          (allocation-capable item {item}, audited-crate caller {caller})\n"
                ));
            }
        }
    }

    if !report.findings.is_empty() {
        out.push_str("\n-- fail-closed findings --\n");
        for f in &report.findings {
            out.push_str(&format!("  [{}] {}\n", f.kind, f.detail));
        }
    }

    if !report.leaves.is_empty() {
        out.push_str("\n-- unexplained leaves (need an allowlist entry) --\n");
        for (_, leaf) in &report.leaves {
            out.push_str(&format!(
                "  [{}] {} (identity {})\n",
                leaf.kind.tag(),
                leaf.item,
                leaf.kind.identity()
            ));
        }
    }

    // Every edge with its source span (SPEC §6 step 3). This is long, so it is
    // emitted only when ALLOC_AUDIT_EDGES is set.
    if std::env::var_os("ALLOC_AUDIT_EDGES").is_some() {
        out.push_str("\n-- reachable edges --\n");
        for e in &report.edges {
            out.push_str(&format!(
                "  [{}] {} -> {}  @ {}\n",
                e.kind, e.from, e.to, e.span
            ));
        }
    }

    eprint!("{out}");

    if let Some(path) = report_path {
        if let Err(e) = std::fs::write(path, &out) {
            eprintln!("alloc_audit: could not write report to {path}: {e}");
        }
    }
}
