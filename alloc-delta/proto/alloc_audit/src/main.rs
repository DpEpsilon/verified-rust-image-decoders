//! Typed transitive allocation audit — prototype for SPEC.md §6 (gate F1).
//!
//! A `rustc_driver` program that runs `after_analysis` on a crate built on
//! x86_64, walks post-monomorphisation MIR from named root public functions, and
//! reports every path to an allocator entry point except through an allowlist.
//! Unresolved edges fail closed.
//!
//! Usage (as a `RUSTC_WRAPPER`, or directly with rustc arguments):
//!   alloc_audit [rustc args...]
//! Audit configuration comes from environment variables, because a
//! `RUSTC_WORKSPACE_WRAPPER` cannot easily carry its own CLI:
//!   ALLOC_AUDIT_ROOTS   comma-separated `crate::path::fn` roots (default: all
//!                       exported non-generic `pub fn`s)
//!   ALLOC_AUDIT_ALLOWLIST  path to the allowlist TOML (default: none)
//!   ALLOC_AUDIT_REPORT  path to write the JSON report (default: stderr summary)
//!
//! Exit status: 0 = pass, 1 = findings (fail), 2 = tool error.

#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
mod allowlist;
mod report;
mod walk;

use std::process::ExitCode;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;

use crate::report::AuditReport;
use crate::walk::{AuditConfig, AuditEngine};

struct AuditCallbacks {
    config: AuditConfig,
    result: Option<AuditReport>,
}

impl Callbacks for AuditCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let report = AuditEngine::new(tcx, self.config.clone()).run();
        self.result = Some(report);
        // Continue so the normal pipeline still produces the crate's artifact:
        // as a `RUSTC_WORKSPACE_WRAPPER` this must be transparent to cargo.
        Compilation::Continue
    }
}

fn main() -> ExitCode {
    rustc_driver::install_ice_hook(
        "https://github.com/rust-lang/rust/issues/new",
        |_| (),
    );

    let config = match AuditConfig::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("alloc_audit: configuration error: {e}");
            return ExitCode::from(2);
        }
    };

    let mut args: Vec<String> = std::env::args().collect();

    // `RUSTC_WORKSPACE_WRAPPER` protocol: cargo invokes us as
    //   <wrapper> <rustc> <rustc args...>
    // `run_compiler` wants `<argv0> <rustc args...>`, so drop the rustc path.
    if args.len() >= 2 && !args[1].starts_with('-') {
        args.remove(1);
    }

    let mut callbacks = AuditCallbacks { config, result: None };
    let exit_code = rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(&args, &mut callbacks);
    });

    // rustc itself failed (e.g. a type error): that is not an audit verdict.
    if exit_code != ExitCode::SUCCESS {
        eprintln!("alloc_audit: rustc did not succeed; no audit verdict");
        return ExitCode::from(2);
    }

    let report = match callbacks.result {
        Some(r) => r,
        None => {
            eprintln!("alloc_audit: after_analysis did not run; no verdict");
            return ExitCode::from(2);
        }
    };

    report::emit(&report, callbacks.config.report_path.as_deref());

    if report.passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
