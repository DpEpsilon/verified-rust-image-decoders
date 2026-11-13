//! alloc_audit (production): typed, transitive allocation audit for the
//! verified decoder cores (alloc-delta SPEC.md §6, §8, §9; ERRATA.md E7).
//!
//! A `rustc_driver` program. In `after_analysis` it walks the
//! post-monomorphisation MIR call graph from every exported function of the
//! crate under audit, through every dependency body rustc can give it, and
//! checks that every path to an allocator sink passes through an allowlisted
//! allocation-capable item from an admitted caller. Nodes are keyed by
//! `Instance` (DefId + generic arguments + instance kind), never by a printed
//! string; printed paths are for display and for writing allowlist values,
//! which are resolved to DefIds before they are compared.
//!
//! Invocation:
//!   * as `RUSTC_WRAPPER`/`RUSTC_WORKSPACE_WRAPPER` (cargo passes
//!     `<this> <path-to-rustc> <rustc args...>`). With `ALLOC_AUDIT_PLAN` set,
//!     only the crates named in the plan are audited; every other invocation
//!     (version probes, dependencies, build scripts) is passed to the real
//!     rustc unchanged by `exec`.
//!   * directly with rustc arguments (the controls do this).
//!
//! Environment:
//!   ALLOC_AUDIT_PLAN       file with lines `<crate_name> <allowlist|-> <report>`
//!   ALLOC_AUDIT_ALLOWLIST  allowlist (direct mode; absent => `allowlist-absent`)
//!   ALLOC_AUDIT_REPORT     report path (direct mode)
//!   ALLOC_AUDIT_ROOTS      extra required roots (production: added to the
//!                          exported set; prototype mode: replaces it)
//!   ALLOC_AUDIT_PROTOTYPE  prototype mode (fixture reviewer accepted; header
//!                          says `mode: PROTOTYPE`; never backs a proof report)
//!   ALLOC_AUDIT_VSTD_VERSION  prototype-only override; refused otherwise
//!   ALLOC_AUDIT_CONTINUE   exit 0 after writing a FAIL report, so cargo goes on
//!   ALLOC_AUDIT_EDGES      also write the full edge list into the report
//!
//! Exit status: 0 pass, 1 findings, 2 tool error (rustc failed; no verdict).

#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_crate_store;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod allowlist;
mod config;
mod inputs;
mod names;
mod report;
mod walk;

use std::process::ExitCode;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;

use crate::config::{AuditConfig, Invocation};
use crate::report::Report;

struct AuditCallbacks {
    config: AuditConfig,
    result: Option<Report>,
}

impl Callbacks for AuditCallbacks {
    fn after_analysis<'tcx>(&mut self, _c: &interface::Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        self.result = Some(walk::run_audit(tcx, &self.config));
        // Continue: as a cargo wrapper the tool must still emit the artifact,
        // so the audited compilation is the one that produces the rlib.
        Compilation::Continue
    }
}

struct NoopCallbacks;
impl Callbacks for NoopCallbacks {}

fn main() -> ExitCode {
    rustc_driver::install_ice_hook("https://github.com/rust-lang/rust/issues/new", |_| ());
    let raw: Vec<String> = std::env::args().collect();

    let inv = match config::classify(&raw) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("alloc_audit: configuration error: {e}");
            return ExitCode::from(2);
        }
    };
    let (args, config) = match inv {
        Invocation::Passthrough { real_rustc, args } => {
            // Not a crate under audit: hand the invocation to the real rustc.
            use std::os::unix::process::CommandExt;
            let err = std::process::Command::new(real_rustc).args(&args[1..]).exec();
            eprintln!("alloc_audit: exec of the real rustc failed: {err}");
            return ExitCode::from(2);
        }
        Invocation::Compile { args } => {
            let code = rustc_driver::catch_with_exit_code(|| {
                rustc_driver::run_compiler(&args, &mut NoopCallbacks)
            });
            return code;
        }
        Invocation::Audit { args, config } => (args, config),
    };

    let mut cb = AuditCallbacks { config, result: None };
    let code = rustc_driver::catch_with_exit_code(|| rustc_driver::run_compiler(&args, &mut cb));
    if code != ExitCode::SUCCESS {
        eprintln!("alloc_audit: rustc did not succeed; no audit verdict");
        return ExitCode::from(2);
    }
    let Some(report) = cb.result else {
        eprintln!("alloc_audit: after_analysis did not run; no audit verdict");
        return ExitCode::from(2);
    };
    if let Err(e) = report.emit(cb.config.report_path.as_deref(), cb.config.emit_edges) {
        eprintln!("alloc_audit: cannot write report: {e}");
        return ExitCode::from(2);
    }
    if report.passed() || cb.config.continue_on_fail {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
