//! Findings and the text report. The report header is `key: value` lines up to
//! the first `-- ` section marker; the controls runner and dependency checks
//! read it.

use std::fmt::Write as _;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Finding {
    pub kind: String,
    pub detail: String,
}

impl Finding {
    pub fn new(kind: &str, detail: String) -> Self {
        Finding { kind: kind.to_string(), detail }
    }
}

#[derive(Default)]
pub struct Report {
    /// Ordered header lines (key, value).
    pub header: Vec<(String, String)>,
    /// Ordered named sections.
    pub sections: Vec<(String, Vec<String>)>,
    pub findings: Vec<Finding>,
    pub edges: Vec<String>,
}

impl Report {
    pub fn h(&mut self, k: &str, v: impl Into<String>) {
        self.header.push((k.to_string(), v.into()));
    }
    pub fn section(&mut self, name: &str, lines: Vec<String>) {
        self.sections.push((name.to_string(), lines));
    }
    pub fn passed(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn render(&self, with_edges: bool) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "=== alloc_audit report (alloc-delta/audit, production tool) ===");
        let _ = writeln!(s, "verdict: {}", if self.passed() { "PASS" } else { "FAIL" });
        let mut tags: Vec<&str> = self.findings.iter().map(|f| f.kind.as_str()).collect();
        tags.sort();
        tags.dedup();
        let _ = writeln!(s, "findings: {}", self.findings.len());
        let _ = writeln!(s, "finding-tags: {}", if tags.is_empty() { "-".to_string() } else { tags.join(",") });
        let non_rev = self.findings.iter().filter(|f| f.kind != "allowlist-reviewer").count();
        let _ = writeln!(s, "findings-other-than-unsigned-reviewer: {non_rev}");
        for (k, v) in &self.header {
            let _ = writeln!(s, "{k}: {v}");
        }
        let _ = writeln!(s, "\n-- findings ({}) --", self.findings.len());
        for f in &self.findings {
            let _ = writeln!(s, "  [{}] {}", f.kind, f.detail);
        }
        for (name, lines) in &self.sections {
            let _ = writeln!(s, "\n-- {name} ({}) --", lines.len());
            for l in lines {
                let _ = writeln!(s, "  {l}");
            }
        }
        if with_edges {
            let _ = writeln!(s, "\n-- edges ({}) --", self.edges.len());
            for e in &self.edges {
                let _ = writeln!(s, "  {e}");
            }
        }
        s
    }

    pub fn emit(&self, path: Option<&Path>, with_edges: bool) -> std::io::Result<()> {
        let text = self.render(with_edges);
        // Summary on stderr always.
        for line in text.lines().take(6) {
            eprintln!("{line}");
        }
        match path {
            Some(p) => {
                if let Some(d) = p.parent() {
                    std::fs::create_dir_all(d)?;
                }
                std::fs::write(p, text)
            }
            None => {
                eprintln!("{text}");
                Ok(())
            }
        }
    }
}
