//! The reachability walk over monomorphised MIR.
//!
//! From a set of root `Instance`s we follow every `Call`, `Drop`, `Assert`/
//! panic and closure/fn-pointer/vtable mention. Each followed edge either
//! resolves to an instance with a body (keep walking), or is a leaf that needs
//! an allowlist entry. Edges we refuse to reason about (indirect calls, dyn
//! dispatch, FFI, inline asm, `-Zshare-generics`) are recorded as findings.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def::DefKind;
use rustc_middle::mir::{
    Body, CastKind, InlineAsmOperand, Operand, Rvalue, StatementKind, TerminatorKind,
};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{
    self, GenericArgKind, Instance, InstanceKind, Ty, TyCtxt, TypingEnv, VtblEntry,
};
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::Span;

use crate::allowlist::Allowlist;
use crate::report::{AllocHit, AuditReport, Edge, Finding, Leaf, LeafKind};

/// Allocator entry points: reaching one is the audit's target of interest.
const ALLOC_ENTRY_PATHS: &[&str] = &[
    "alloc::alloc::alloc",
    "alloc::alloc::alloc_zeroed",
    "alloc::alloc::realloc",
    "alloc::alloc::dealloc",
    "alloc::alloc::exchange_malloc",
    "alloc::alloc::handle_alloc_error",
    "alloc::alloc::__rust_alloc",
    "alloc::alloc::__rust_alloc_zeroed",
    "alloc::alloc::__rust_realloc",
    "alloc::alloc::__rust_dealloc",
];

/// Symbol-name substrings marking an allocator shim reached via a trait method.
const ALLOC_SYMBOL_SUBSTRINGS: &[&str] = &[
    "__rust_alloc",
    "__rust_realloc",
    "__rust_dealloc",
    "__rust_alloc_error_handler",
    "handle_alloc_error",
    "exchange_malloc",
];

#[derive(Clone)]
pub struct AuditConfig {
    pub roots: Vec<String>,
    pub allowlist_path: Option<String>,
    pub report_path: Option<String>,
    /// Whether the run was started with the explicit prototype flag, which is
    /// the only way a `prototype-fixture` reviewer value is accepted.
    pub prototype_mode: bool,
}

impl AuditConfig {
    pub fn from_env() -> Result<Self, String> {
        let roots = std::env::var("ALLOC_AUDIT_ROOTS")
            .ok()
            .map(|s| {
                s.split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        Ok(AuditConfig {
            roots,
            allowlist_path: std::env::var("ALLOC_AUDIT_ALLOWLIST").ok(),
            report_path: std::env::var("ALLOC_AUDIT_REPORT").ok(),
            prototype_mode: std::env::var_os("ALLOC_AUDIT_PROTOTYPE").is_some(),
        })
    }
}

pub struct AuditEngine<'tcx> {
    tcx: TyCtxt<'tcx>,
    config: AuditConfig,
    allowlist: Allowlist,
    visited: BTreeSet<String>,
    queue: VecDeque<(Instance<'tcx>, String)>,
    edges: Vec<Edge>,
    leaves: BTreeMap<String, Leaf>,
    findings: Vec<Finding>,
    allocations: Vec<AllocHit>,
    node_count: usize,
    /// Labels of instances defined in the audited crate itself (local def ids).
    /// Only these are "callers" the allowlist constrains; a std node is not.
    audited_labels: BTreeSet<String>,
    /// Allocator entry-point labels reached, deduplicated.
    alloc_entries: BTreeSet<String>,
    /// Root instance labels (the starts of the traversal).
    root_labels: BTreeSet<String>,
}

impl<'tcx> AuditEngine<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, config: AuditConfig) -> Self {
        let allowlist = match &config.allowlist_path {
            Some(p) if !p.is_empty() => Allowlist::load(tcx, p, config.prototype_mode),
            _ => Allowlist::empty(),
        };
        AuditEngine {
            tcx,
            config,
            allowlist,
            visited: BTreeSet::new(),
            queue: VecDeque::new(),
            edges: Vec::new(),
            leaves: BTreeMap::new(),
            findings: Vec::new(),
            allocations: Vec::new(),
            node_count: 0,
            audited_labels: BTreeSet::new(),
            alloc_entries: BTreeSet::new(),
            root_labels: BTreeSet::new(),
        }
    }

    pub fn run(mut self) -> AuditReport {
        if self.tcx.sess.opts.share_generics() {
            self.findings.push(Finding::new(
                "share-generics",
                "-Zshare-generics is enabled: generic std instantiations come from upstream crates"
                    .into(),
            ));
        }
        if self.allowlist.commit_mismatch {
            self.findings.push(Finding::new(
                "allowlist-commit",
                format!(
                    "allowlist records rustc commit {:?}; running compiler is {:?} (full {:?})",
                    self.allowlist.recorded_commit,
                    self.allowlist.running_commit,
                    self.allowlist.actual_commit_full
                ),
            ));
        }
        if self.allowlist.commit_missing {
            self.findings.push(Finding::new(
                "allowlist-commit-missing",
                format!(
                    "allowlist records no rustc_commit; the running compiler is {:?} (full {:?})",
                    self.allowlist.running_commit, self.allowlist.actual_commit_full
                ),
            ));
        }
        if self.allowlist.target_mismatch {
            self.findings.push(Finding::new(
                "allowlist-target",
                format!(
                    "allowlist records target {:?}; running compiler targets {:?}",
                    self.allowlist.recorded_target, self.allowlist.running_target
                ),
            ));
        }
        if self.allowlist.target_missing {
            self.findings.push(Finding::new(
                "allowlist-target-missing",
                format!(
                    "allowlist records no target; the running target is {:?}",
                    self.allowlist.running_target
                ),
            ));
        }
        if self.allowlist.load_error {
            self.findings.push(Finding::new(
                "allowlist-load",
                "allowlist file could not be parsed".into(),
            ));
        }
        if self.allowlist.profile_missing {
            self.findings.push(Finding::new(
                "allowlist-profile-missing",
                format!(
                    "allowlist records no profile; this compilation is {:?} (class {})",
                    self.allowlist.detected_profile, self.allowlist.profile_class
                ),
            ));
        }
        if self.allowlist.profile_mismatch {
            self.findings.push(Finding::new(
                "allowlist-profile",
                format!(
                    "allowlist records profile {:?}; this compilation is {:?} (class {})",
                    self.allowlist.recorded_profile,
                    self.allowlist.detected_profile,
                    self.allowlist.profile_class
                ),
            ));
        }
        if self.allowlist.source_hash_missing {
            self.findings.push(Finding::new(
                "allowlist-source-hash-missing",
                format!(
                    "allowlist records no source_hash; the audited sources hash to {:?}",
                    self.allowlist.source_hash_actual
                ),
            ));
        }
        if self.allowlist.source_hash_mismatch {
            self.findings.push(Finding::new(
                "allowlist-source-hash",
                format!(
                    "allowlist records source_hash {:?}; the audited sources hash to {:?}",
                    self.allowlist.recorded_source_hash, self.allowlist.source_hash_actual
                ),
            ));
        }
        if self.allowlist.reviewer_invalid {
            let why = if self.allowlist.reviewer.as_deref().map(str::trim)
                == Some(crate::allowlist::PROTOTYPE_REVIEWER)
            {
                "reviewer is the prototype placeholder; pass --prototype to accept it"
            } else {
                "reviewer is missing or empty"
            };
            self.findings.push(Finding::new(
                "allowlist-reviewer",
                format!("{why} (recorded {:?})", self.allowlist.reviewer),
            ));
        }
        if self.allowlist.vstd_missing {
            self.findings.push(Finding::new(
                "allowlist-vstd-missing",
                format!(
                    "the audited crate links vstd but the allowlist records no vstd_version (linked version {:?})",
                    self.allowlist.vstd_actual
                ),
            ));
        }
        if self.allowlist.vstd_undetermined {
            self.findings.push(Finding::new(
                "allowlist-vstd-undetermined",
                format!(
                    "the audited crate links vstd but the tool cannot determine the linked version; allowlist records {:?} (set ALLOC_AUDIT_VSTD_VERSION or provide the crate's Cargo.lock)",
                    self.allowlist.recorded_vstd
                ),
            ));
        }
        if self.allowlist.vstd_mismatch {
            self.findings.push(Finding::new(
                "allowlist-vstd",
                format!(
                    "allowlist records vstd_version {:?}; the linked vstd is {:?}",
                    self.allowlist.recorded_vstd, self.allowlist.vstd_actual
                ),
            ));
        }
        if self.allowlist.vstd_not_declared_none {
            self.findings.push(Finding::new(
                "allowlist-vstd-not-declared-none",
                "the audited crate does not link vstd; the allowlist must declare `vstd = \"none\"`"
                    .into(),
            ));
        }
        if self.allowlist.undeclared_build_script {
            self.findings.push(Finding::new(
                "undeclared-build-script",
                "the audited crate has a build script; its output is not covered by the source hash. Declare `[[entry]] kind = \"build-script\" item = \"build.rs\"` to admit it"
                    .into(),
            ));
        }
        for c in self.allowlist.undeclared_proc_macros.clone() {
            self.findings.push(Finding::new(
                "undeclared-proc-macro",
                format!(
                    "the audited crate links proc-macro crate {c}; its output is not covered by the source hash. Declare `[[entry]] kind = \"proc-macro\" item = \"{c}\"` to admit it"
                ),
            ));
        }
        for inc in self.allowlist.undeclared_includes.clone() {
            self.findings.push(Finding::new(
                "undeclared-include",
                format!(
                    "the audited source contains {inc}; the included bytes are not covered by the source hash. Declare `[[entry]] kind = \"include\" item = \"{inc}\"` to admit it"
                ),
            ));
        }
        // With no allowlist, nothing is declared, so every build input is
        // undeclared. Reporting them here (rather than only from `Allowlist::load`)
        // makes the fail-closed behaviour independent of whether an allowlist was
        // supplied.
        if self.allowlist.path.is_none() {
            let inputs = crate::allowlist::detect_build_inputs(self.tcx);
            if inputs.build_script {
                self.findings.push(Finding::new(
                    "undeclared-build-script",
                    "the audited crate has a build script; its output is not covered by the source hash (no allowlist declares it)"
                        .into(),
                ));
            }
            for c in &inputs.proc_macro_crates {
                self.findings.push(Finding::new(
                    "undeclared-proc-macro",
                    format!(
                        "the audited crate links proc-macro crate {c}; its output is not covered by the source hash (no allowlist declares it)"
                    ),
                ));
            }
            for inc in &inputs.includes {
                self.findings.push(Finding::new(
                    "undeclared-include",
                    format!(
                        "the audited source contains {inc}; the included bytes are not covered by the source hash (no allowlist declares it)"
                    ),
                ));
            }
        }

        let (roots, root_findings) = self.discover_roots();
        self.findings.extend(root_findings);
        if roots.is_empty() {
            self.findings.push(Finding::new(
                "no-roots",
                "no root functions found; nothing was audited".into(),
            ));
        }
        for (inst, why) in roots {
            let label = instance_label(self.tcx, inst);
            self.root_labels.insert(label.clone());
            self.queue.push_back((inst, why));
        }

        self.drain_queue();
        self.authorize_allocations();

        let node_count = self.node_count;
        let mut report = AuditReport::new(
            self.visited,
            self.edges,
            self.leaves,
            self.findings,
            self.allocations,
            node_count,
        );
        report.allowlist_note = self.allowlist.summary();
        report.prototype_mode = self.config.prototype_mode;
        report.source_hashes = self.allowlist.source_hashes.clone();
        report.source_hash_actual = self.allowlist.source_hash_actual.clone();
        report.build_script = self.allowlist.build_script;
        report.proc_macro_crates = self.allowlist.proc_macro_crates.clone();
        report.includes = self.allowlist.includes.clone();
        // The profile is a property of the compilation, not of the allowlist, so
        // detect it here too when no allowlist was loaded; the header then always
        // records the profile the audit saw.
        if self.allowlist.detected_profile.is_empty() {
            let (prof, class) = crate::allowlist::detect_profile(self.tcx);
            report.detected_profile = prof;
            report.profile_class = class;
        } else {
            report.detected_profile = self.allowlist.detected_profile.clone();
            report.profile_class = self.allowlist.profile_class.clone();
        }
        if report.source_hashes.is_empty() {
            let (files, combined) = crate::allowlist::source_hash_of(self.tcx);
            report.source_hashes = files;
            report.source_hash_actual = combined;
        }
        if self.allowlist.path.is_none() {
            // No allowlist loaded: still detect the build inputs, so the report
            // header states what the (absent) hash would not cover.
            let inputs = crate::allowlist::detect_build_inputs(self.tcx);
            report.build_script = inputs.build_script;
            report.proc_macro_crates = inputs.proc_macro_crates;
            report.includes = inputs.includes;
        }
        report
    }

    fn discover_roots(&mut self) -> (Vec<(Instance<'tcx>, String)>, Vec<Finding>) {
        let tcx = self.tcx;
        let mut out = Vec::new();
        let mut findings = Vec::new();

        let all_locals: Vec<LocalDefId> = tcx.iter_local_def_id().collect();
        let label_of = |did: DefId| tcx.def_path_str(did);

        if !self.config.roots.is_empty() {
            let mut matched = BTreeSet::new();
            for name in &self.config.roots {
                let mut hit = None;
                for &local in &all_locals {
                    let did = local.to_def_id();
                    if !matches!(tcx.def_kind(local), DefKind::Fn) {
                        continue;
                    }
                    let p = label_of(did);
                    if p == *name || p.ends_with(&format!("::{name}")) {
                        hit = Some((local, p));
                        break;
                    }
                }
                match hit {
                    Some((local, p)) => {
                        matched.insert(name.clone());
                        let inst = Instance::mono(tcx, local.to_def_id());
                        out.push((inst, format!("root {p}")));
                    }
                    None => findings.push(Finding::new(
                        "missing-root",
                        format!("root {name} was not found among the crate's functions"),
                    )),
                }
            }
        } else {
            for &local in &all_locals {
                if !matches!(tcx.def_kind(local), DefKind::Fn) {
                    continue;
                }
                let did = local.to_def_id();
                let vis = tcx.effective_visibilities(());
                if !vis.is_exported(local) {
                    continue;
                }
                if tcx.generics_of(did).requires_monomorphization(tcx) {
                    findings.push(Finding::new(
                        "generic-pub-fn",
                        format!(
                            "exported generic function {} cannot be a root (SPEC §6 step 1)",
                            label_of(did)
                        ),
                    ));
                    continue;
                }
                let inst = Instance::mono(tcx, did);
                out.push((inst, format!("root {}", label_of(did))));
            }
        }
        (out, findings)
    }

    fn drain_queue(&mut self) {
        const MAX_NODES: usize = 200_000;
        while let Some((instance, from)) = self.queue.pop_front() {
            if self.node_count > MAX_NODES {
                self.findings.push(Finding::new(
                    "node-cap",
                    format!("traversal exceeded {MAX_NODES} nodes; aborted (fail closed)"),
                ));
                break;
            }
            let label = instance_label(self.tcx, instance);
            if self.visited.contains(&label) {
                continue;
            }
            self.visited.insert(label.clone());
            self.node_count += 1;
            // A node defined in the audited crate is a prospective "caller" the
            // allowlist constrains. `is_local` is true exactly for defs of the
            // crate under compilation, so this captures free functions, impl
            // methods and closures alike.
            if instance.def.def_id().is_local() {
                self.audited_labels.insert(label.clone());
            }

            if is_alloc_entry(self.tcx, instance) {
                // Record the entry and stop; authorization is decided after the
                // whole graph is known, over every incoming path (see
                // `authorize_allocations`). Deciding here from the first BFS
                // parent was the HIGH defect: a shared instance reached first
                // from an allowed caller hid a second, unapproved caller.
                self.alloc_entries.insert(label.clone());
                continue;
            }

            match self.body_for(instance) {
                BodyOutcome::Body(body) => self.walk_body(&label, body),
                BodyOutcome::Leaf(kind) => {
                    if !self.allowlist.covers_leaf(&kind) {
                        self.leaves
                            .entry(label.clone())
                            .or_insert_with(|| Leaf { item: label.clone(), kind });
                    }
                }
                BodyOutcome::VirtualFail => {
                    self.findings.push(Finding::new(
                        "virtual-dispatch",
                        format!("trait-object (virtual) call reached: {label} from {from}"),
                    ));
                }
            }
        }
    }

    /// Fetch a monomorphised body, or classify why we cannot.
    ///
    /// The body is instantiated with the instance's generic arguments under
    /// `TypingEnv::fully_monomorphized()`, i.e. it is the post-monomorphisation
    /// MIR (SPEC §6 step 2). A body that cannot be instantiated fails closed.
    fn body_for(&self, instance: Instance<'tcx>) -> BodyOutcome<'tcx> {
        let tcx = self.tcx;
        match instance.def {
            InstanceKind::Intrinsic(did) | InstanceKind::LlvmIntrinsic(did) => {
                BodyOutcome::Leaf(LeafKind::Intrinsic(tcx.def_path_str(did)))
            }
            InstanceKind::Virtual(did, _) => {
                let _ = did;
                BodyOutcome::VirtualFail
            }
            InstanceKind::Item(did) => {
                if tcx.is_foreign_item(did) {
                    return BodyOutcome::Leaf(LeafKind::Foreign(tcx.def_path_str(did)));
                }
                if !tcx.is_mir_available(did) {
                    return BodyOutcome::Leaf(LeafKind::NoMir(tcx.def_path_str(did)));
                }
                self.instantiate_body(instance, tcx.instance_mir(instance.def))
            }
            InstanceKind::Shim(_) => self.instantiate_body(instance, tcx.instance_mir(instance.def)),
        }
    }

    fn instantiate_body(
        &self,
        instance: Instance<'tcx>,
        body: &'tcx Body<'tcx>,
    ) -> BodyOutcome<'tcx> {
        let tcx = self.tcx;
        let bound = rustc_middle::ty::EarlyBinder::bind(tcx, body.clone());
        match instance.try_instantiate_mir_and_normalize_erasing_regions(
            tcx,
            TypingEnv::fully_monomorphized(),
            bound,
        ) {
            Ok(mono) => BodyOutcome::Body(tcx.arena.alloc(mono)),
            Err(_) => BodyOutcome::Leaf(LeafKind::NoMir(format!(
                "{} (instantiation failed)",
                tcx.def_path_str(instance.def.def_id())
            ))),
        }
    }

    fn walk_body(&mut self, label: &str, body: &'tcx Body<'tcx>) {
        for bbdata in body.basic_blocks.iter() {
            for stmt in &bbdata.statements {
                if let StatementKind::Assign(assign) = &stmt.kind {
                    let (_, rv) = &**assign;
                    self.walk_rvalue(label, rv, stmt.source_info.span, body);
                }
            }
            let term = bbdata.terminator();
            self.walk_terminator(label, &term.kind, term.source_info.span, body);
        }
    }

    fn walk_terminator(
        &mut self,
        from: &str,
        kind: &TerminatorKind<'tcx>,
        span: Span,
        body: &'tcx Body<'tcx>,
    ) {
        let tcx = self.tcx;
        match kind {
            TerminatorKind::Call { func, args, .. }
            | TerminatorKind::TailCall { func, args, .. } => {
                for arg in args {
                    self.walk_operand(from, &arg.node, span, body);
                }
                let callee_ty = self.instantiate_ty(func.ty(body, tcx));
                match *callee_ty.kind() {
                    ty::FnDef(def_id, gargs) => {
                        let gargs = gargs.no_bound_vars().expect("bound vars in FnDef args");
                        match Instance::try_resolve(
                            tcx,
                            TypingEnv::fully_monomorphized(),
                            def_id,
                            gargs,
                        ) {
                            Ok(Some(inst)) => {
                                self.push_edge(from, &inst, span, "call");
                                self.queue.push_back((inst, from.to_string()));
                            }
                            Ok(None) => self.findings.push(Finding::new(
                                "unresolved-call",
                                format!(
                                    "{from} calls {} which did not resolve to a single instance at {}",
                                    tcx.def_path_str(def_id),
                                    self.span_str(span)
                                ),
                            )),
                            Err(_) => self.findings.push(Finding::new(
                                "unresolved-call",
                                format!(
                                    "{from} calls {} whose resolution errored at {}",
                                    tcx.def_path_str(def_id),
                                    self.span_str(span)
                                ),
                            )),
                        }
                    }
                    ty::FnPtr(..) => {
                        let targets = self.fn_ptr_targets(body);
                        let n = targets.len();
                        for inst in targets {
                            self.push_edge(from, &inst, span, "fn-ptr-target");
                            self.queue.push_back((inst, from.to_string()));
                        }
                        self.findings.push(Finding::new(
                            "fn-ptr-call",
                            format!(
                                "{from} makes an indirect call through a function pointer at {}; followed {n} candidate target(s) gathered from this body, but the call is not statically resolved",
                                self.span_str(span)
                            ),
                        ));
                    }
                    ty::Dynamic(..) => self.findings.push(Finding::new(
                        "dyn-call",
                        format!("{from} makes a trait-object call at {}", self.span_str(span)),
                    )),
                    _ => self.findings.push(Finding::new(
                        "unknown-callee",
                        format!(
                            "{from} calls an operand of type {callee_ty} that is not a statically resolvable callee at {}",
                            self.span_str(span)
                        ),
                    )),
                }
            }
            TerminatorKind::Drop { place, .. } => {
                let ty = self.instantiate_ty(place.ty(body, tcx).ty);
                match self.resolve_drop_glue(ty) {
                    Some(inst) => {
                        self.push_edge(from, &inst, span, "drop");
                        self.queue.push_back((inst, from.to_string()));
                    }
                    None => self.findings.push(Finding::new(
                        "unresolved-drop",
                        format!(
                            "{from} drops a value of type {ty} whose drop glue did not resolve at {}",
                            self.span_str(span)
                        ),
                    )),
                }
            }
            TerminatorKind::Assert { msg, .. } => {
                // `AssertKind::panic_function` bug!()s on `BoundsCheck` and
                // `MisalignedPointerDereference`; those call dedicated lang
                // items with dynamic arguments, so name them directly.
                let name = match &**msg {
                    rustc_middle::mir::AssertKind::BoundsCheck { .. } => {
                        "panic::PanicBoundsCheck".to_string()
                    }
                    rustc_middle::mir::AssertKind::MisalignedPointerDereference { .. } => {
                        "panic::PanicMisalignedPointerDereference".to_string()
                    }
                    other => format!("panic::{:?}", other.panic_function()),
                };
                self.push_edge_name(from, &name, span, "panic");
                if !self.allowlist.covers_leaf(&LeafKind::Panic(name.clone())) {
                    self.leaves
                        .entry(format!("panic@{from}@{name}"))
                        .or_insert_with(|| Leaf {
                            item: format!("{name} in {from}"),
                            kind: LeafKind::Panic(name.clone()),
                        });
                }
            }
            TerminatorKind::UnwindTerminate(reason) => {
                let name = format!("unwind-terminate::{reason:?}");
                self.push_edge_name(from, &name, span, "unwind");
                if !self.allowlist.covers_leaf(&LeafKind::Panic(name.clone())) {
                    self.leaves
                        .entry(format!("unwind@{from}"))
                        .or_insert_with(|| Leaf {
                            item: format!("{name} in {from}"),
                            kind: LeafKind::Panic(name.clone()),
                        });
                }
            }
            TerminatorKind::InlineAsm { operands, .. } => {
                self.findings.push(Finding::new(
                    "inline-asm",
                    format!("{from} contains inline assembly at {}", self.span_str(span)),
                ));
                for op in operands {
                    if let InlineAsmOperand::SymFn { value } = op {
                        let fty = self.instantiate_ty(value.const_.ty());
                        self.queue_fn_def(from, fty, span, true);
                    }
                }
            }
            TerminatorKind::SwitchInt { discr, .. } => {
                self.walk_operand(from, discr, span, body);
            }
            _ => {}
        }
    }

    fn walk_rvalue(&mut self, from: &str, rv: &Rvalue<'tcx>, span: Span, body: &'tcx Body<'tcx>) {
        let tcx = self.tcx;
        match rv {
            Rvalue::Cast(
                CastKind::PointerCoercion(
                    PointerCoercion::ReifyFnPointer(_) | PointerCoercion::ClosureFnPointer(_),
                    _,
                ),
                op,
                _,
            ) => {
                self.queue_fn_def(from, self.instantiate_ty(op.ty(body, tcx)), span, false);
            }
            Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize, _), op, target) => {
                let src = self.instantiate_ty(op.ty(body, tcx));
                let target = self.instantiate_ty(*target);
                // Unsize to a trait object: `target` may be `&dyn Trait` or
                // `Box<dyn Trait>`, so test the unpeeled tail, not `target`.
                let (_, target_tail) = unpeel_to_trait(src, target);
                if target_tail.is_trait() {
                    self.vtable_edges(from, src, target, span);
                }
            }
            Rvalue::Use(op, _)
            | Rvalue::Repeat(op, _)
            | Rvalue::UnaryOp(_, op)
            | Rvalue::Cast(_, op, _) => {
                self.walk_operand(from, op, span, body);
            }
            Rvalue::BinaryOp(_, ops) => {
                self.walk_operand(from, &ops.0, span, body);
                self.walk_operand(from, &ops.1, span, body);
            }
            Rvalue::Aggregate(_, ops) => {
                for op in ops {
                    self.walk_operand(from, op, span, body);
                }
            }
            _ => {}
        }
    }

    fn walk_operand(&mut self, from: &str, op: &Operand<'tcx>, span: Span, body: &'tcx Body<'tcx>) {
        let ty = self.instantiate_ty(op.ty(body, self.tcx));
        if let ty::FnDef(_, _) = ty.kind() {
            self.queue_fn_def(from, ty, span, false);
        }
    }

    fn queue_fn_def(&mut self, from: &str, ty: Ty<'tcx>, span: Span, direct: bool) {
        let tcx = self.tcx;
        if let ty::FnDef(def_id, gargs) = *ty.kind() {
            let gargs = gargs.no_bound_vars().expect("bound vars in fn use");
            let inst = if direct {
                Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), def_id, gargs)
                    .ok()
                    .flatten()
            } else {
                Instance::resolve_for_fn_ptr(tcx, TypingEnv::fully_monomorphized(), def_id, gargs)
            };
            match inst {
                Some(inst) => {
                    self.push_edge(from, &inst, span, "fn-use");
                    self.queue.push_back((inst, from.to_string()));
                }
                None => self.findings.push(Finding::new(
                    "unresolved-fn-use",
                    format!(
                        "{from} uses {} as a function pointer but it did not resolve at {}",
                        tcx.def_path_str(def_id),
                        self.span_str(span)
                    ),
                )),
            }
        }
    }

    /// Distinct `Instance`s mentioned by fn-pointer casts in a body. Used to
    /// recover candidates for an indirect call; the call itself still fails
    /// closed because the target set is not provably complete.
    fn fn_ptr_targets(&self, body: &'tcx Body<'tcx>) -> Vec<Instance<'tcx>> {
        let tcx = self.tcx;
        let mut out = Vec::new();
        let mut seen: FxHashSet<String> = FxHashSet::default();
        for bbdata in body.basic_blocks.iter() {
            for stmt in &bbdata.statements {
                let StatementKind::Assign(assign) = &stmt.kind else {
                    continue;
                };
                let (_, rv) = &**assign;
                if let Rvalue::Cast(
                    CastKind::PointerCoercion(
                        PointerCoercion::ReifyFnPointer(_) | PointerCoercion::ClosureFnPointer(_),
                        _,
                    ),
                    op,
                    _,
                ) = rv
                {
                    let ty = self.instantiate_ty(op.ty(body, tcx));
                    if let ty::FnDef(did, gargs) = *ty.kind() {
                        if let Some(gargs) = gargs.no_bound_vars() {
                            if let Some(inst) = Instance::resolve_for_fn_ptr(
                                tcx,
                                TypingEnv::fully_monomorphized(),
                                did,
                                gargs,
                            ) {
                                if seen.insert(instance_label(tcx, inst)) {
                                    out.push(inst);
                                }
                            }
                        }
                    }
                }
            }
        }
        out
    }

    /// Vtable edges from an unsizing coercion to a `dyn Trait`. Following the
    /// vtable methods is the reason to look here at all; the dynamic call itself
    /// is recorded as a finding by `walk_terminator`.
    fn vtable_edges(&mut self, from: &str, src: Ty<'tcx>, target: Ty<'tcx>, span: Span) {
        let tcx = self.tcx;
        // Unpeel `&T`/`Box<T>` so we reach the concrete type behind the trait
        // object pointer, then build the `dyn Trait` principal.
        let (src_tail, target_tail) = unpeel_to_trait(src, target);
        if let ty::Dynamic(preds, _) = target_tail.kind() {
            if let Some(principal) = preds.principal() {
                let trait_ref =
                    tcx.instantiate_bound_regions_with_erased(principal.with_self_ty(tcx, src_tail));
                for entry in tcx.vtable_entries(trait_ref) {
                    if let VtblEntry::Method(inst) = entry {
                        let inst = *inst;
                        self.push_edge(from, &inst, span, "vtable-method");
                        self.queue.push_back((inst, from.to_string()));
                    }
                }
            }
        }
        self.findings.push(Finding::new(
            "vtable-unsize",
            format!(
                "{from} unsized {} to trait object {} at {}; vtable methods followed, dynamic dispatch fails closed",
                src,
                target,
                self.span_str(span)
            ),
        ));
    }

    /// Resolve drop glue without ICEing if it fails (`Instance::resolve_drop_glue`
    /// uses `expect_resolve`, which aborts the compiler on failure).
    fn resolve_drop_glue(&self, ty: Ty<'tcx>) -> Option<Instance<'tcx>> {
        let tcx = self.tcx;
        let def_id = tcx.lang_items().get(rustc_hir::attrs::lang_items::LangItem::DropGlue)?;
        let args = tcx.mk_args(&[ty.into()]);
        Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), def_id, args)
            .ok()
            .flatten()
    }

    fn instantiate_ty(&self, ty: Ty<'tcx>) -> Ty<'tcx> {
        let tcx = self.tcx;
        tcx.try_normalize_erasing_regions(
            TypingEnv::fully_monomorphized(),
            rustc_middle::ty::Unnormalized::new_wip(ty),
        )
        .unwrap_or(ty)
    }

    fn push_edge(&mut self, from: &str, to: &Instance<'tcx>, span: Span, kind: &str) {
        let to_label = instance_label(self.tcx, *to);
        self.push_edge_name(from, &to_label, span, kind);
    }

    fn push_edge_name(&mut self, from: &str, to: &str, span: Span, kind: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
            kind: kind.to_string(),
            span: self.span_str(span),
        });
    }

    fn span_str(&self, span: Span) -> String {
        if span.is_dummy() {
            return "<no span>".into();
        }
        self.tcx
            .sess
            .source_map()
            .span_to_string(span, rustc_span::RemapPathScopeComponents::DIAGNOSTICS)
    }

    /// Forward adjacency (deduplicated) over the recorded edges.
    fn forward_adjacency(&self) -> BTreeMap<String, BTreeSet<String>> {
        let mut fwd: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for e in &self.edges {
            fwd.entry(e.from.clone()).or_default().insert(e.to.clone());
        }
        fwd
    }

    /// Decide authorization for every reachable allocator entry point.
    ///
    /// The HIGH defect this replaces: the walk visited each monomorphised
    /// instance once and kept only its first (shortest) BFS parent. When an
    /// allowed guard and an unapproved helper both call the SAME allocation-
    /// capable instance — for example `RawVecInner::<A>::reserve_exact<Global>`,
    /// which is parameterised by the allocator only, so a `Vec<u8>` and a
    /// `Vec<u32>` share it — the first path could be the guard's, and the tool
    /// then reported the shared allocator endpoint as allowed without ever
    /// examining the helper's incoming path.
    ///
    /// Here every root-to-entry path is enumerated (bounded; exceeding the bound
    /// fails closed) and each one must be justified: the last allocation-capable
    /// item on the path must be named by an `alloc` entry, and the nearest
    /// audited-crate function before that item must be the entry's `allow` (or
    /// `*`). An unjustified path makes the entry a finding, naming each
    /// offending path.
    fn authorize_allocations(&mut self) {
        // Snapshot the alloc entries as (item, allow) pairs so we can call
        // `&self` helpers without fighting the borrow checker.
        let entries: Vec<(String, String)> = self
            .allowlist
            .entries
            .iter()
            .filter(|e| e.kind == "alloc")
            .map(|e| (e.item.clone(), e.allow.clone()))
            .collect();
        let fwd = self.forward_adjacency();
        let audited = self.audited_labels.clone();
        let roots: Vec<String> = self.root_labels.iter().cloned().collect();
        let entries_list: Vec<String> = self.alloc_entries.iter().cloned().collect();

        const MAX_PATHS: usize = 20000;
        const MAX_DEPTH: usize = 80;
        const MAX_SHOWN: usize = 8;

        let mut hits: Vec<AllocHit> = Vec::new();
        let mut findings: Vec<Finding> = Vec::new();

        for entry in &entries_list {
            let mut justified = 0usize;
            let mut offending: Vec<(String, String, String)> = Vec::new();
            let mut shown = 0usize;
            // The representative path shown in the report is the first
            // *justified* one when one exists, so the `via`/`path` line names the
            // allowlist entry that admits it; otherwise the first path found.
            let mut first_path: Option<(String, Option<String>, Option<String>)> = None;
            let mut first_ok: Option<(String, Option<String>, Option<String>)> = None;
            let mut truncated = false;

            for root in &roots {
                let mut budget = MAX_PATHS;
                let mut paths: Vec<Vec<String>> = Vec::new();
                if collect_paths(root, entry, &fwd, &mut paths, &mut budget, MAX_DEPTH) {
                    truncated = true;
                }
                for p in &paths {
                    let (ok, item, caller) = judge_path(p, &entries, &audited);
                    let rendered = (render_path(p), item.clone(), caller.clone());
                    if first_path.is_none() {
                        first_path = Some(rendered.clone());
                    }
                    if ok {
                        justified += 1;
                        if first_ok.is_none() {
                            first_ok = Some(rendered);
                        }
                    } else if shown < MAX_SHOWN {
                        offending.push((
                            rendered.0,
                            item.clone().unwrap_or_else(|| "<none>".into()),
                            caller.clone().unwrap_or_else(|| "<none>".into()),
                        ));
                        shown += 1;
                    }
                }
            }

            if truncated {
                findings.push(Finding::new(
                    "path-bound",
                    format!(
                        "path enumeration for {entry} exceeded the {MAX_PATHS}-path bound; failing closed"
                    ),
                ));
            }

            let (path, via_item, via_caller) = first_ok
                .or(first_path)
                .unwrap_or_else(|| (format!("{entry} (no root path?)"), None, None));
            let ok = offending.is_empty() && justified > 0 && !truncated;
            for (op, oi, oc) in &offending {
                findings.push(Finding::new(
                    "unauthorized-alloc-path",
                    format!(
                        "{entry} is reachable on an unauthorized path: {op} (allocation-capable item {oi}, audited-crate caller {oc})"
                    ),
                ));
            }
            hits.push(AllocHit {
                entry: entry.clone(),
                path,
                via_item,
                via_caller,
                justified: ok,
                offending,
            });
        }

        self.allocations.extend(hits);
        self.findings.extend(findings);
    }
}

/// Enumerate every simple path (no repeated node) from `root` to `target` in
/// the forward edge graph, appending each to `out`. Returns `true` if the
/// `budget` ran out (the caller must then fail closed). `budget` is decremented
/// per path emitted, so a combinatorial blow-up is bounded.
fn collect_paths(
    root: &str,
    target: &str,
    fwd: &BTreeMap<String, BTreeSet<String>>,
    out: &mut Vec<Vec<String>>,
    budget: &mut usize,
    max_depth: usize,
) -> bool {
    let mut stack: Vec<String> = vec![root.to_string()];
    let mut on_path: BTreeSet<String> = BTreeSet::new();
    on_path.insert(root.to_string());
    let mut truncated = false;
    dfs_paths(root, target, fwd, &mut stack, &mut on_path, out, budget, max_depth, &mut truncated);
    truncated
}

#[allow(clippy::too_many_arguments)]
fn dfs_paths(
    cur: &str,
    target: &str,
    fwd: &BTreeMap<String, BTreeSet<String>>,
    stack: &mut Vec<String>,
    on_path: &mut BTreeSet<String>,
    out: &mut Vec<Vec<String>>,
    budget: &mut usize,
    max_depth: usize,
    truncated: &mut bool,
) {
    if *budget == 0 {
        *truncated = true;
        return;
    }
    if cur == target {
        if stack.len() > 1 {
            out.push(stack.clone());
            *budget -= 1;
        }
        return;
    }
    if stack.len() >= max_depth {
        *truncated = true;
        return;
    }
    let Some(nexts) = fwd.get(cur) else {
        return;
    };
    for nxt in nexts {
        if on_path.contains(nxt) {
            continue; // simple paths only; cycles contribute nothing new
        }
        stack.push(nxt.clone());
        on_path.insert(nxt.clone());
        dfs_paths(nxt, target, fwd, stack, on_path, out, budget, max_depth, truncated);
        on_path.remove(nxt);
        stack.pop();
        if *budget == 0 {
            *truncated = true;
            return;
        }
    }
}

/// Judge one root-to-entry path. Returns `(justified, via_item, via_caller)`.
///
/// The path is `root .. entry` in execution order. The allocation-capable item
/// is the last element before the entry that matches an `alloc` entry's `item`.
/// The nearest *audited-crate* function at or before that item is the caller the
/// entry must admit; a std node between the item and the audited caller is
/// skipped, and if no audited crate function is on the path the item's own
/// `allow` must be `*`.
fn judge_path(
    path: &[String],
    entries: &[(String, String)],
    audited: &BTreeSet<String>,
) -> (bool, Option<String>, Option<String>) {
    let is_audited = |label: &str| audited.contains(label);
    // Last element before the entry (path's final node is the entry).
    let body = &path[..path.len().saturating_sub(1)];
    // Find the last allocation-capable item in the body that any entry names.
    for i in (0..body.len()).rev() {
        let label = &body[i];
        // Every (item, allow) pair whose item names this node. A path is
        // justified when *any* of them admits it: two guard functions that call
        // the same monomorphised instance can each have their own entry, so the
        // matcher must not stop at the first match.
        let matching: Vec<&(String, String)> = entries
            .iter()
            .filter(|(item, _)| crate::allowlist::def_path_matches(label, item))
            .collect();
        if matching.is_empty() {
            continue;
        }
        // Nearest audited-crate caller at or before `i`.
        let caller = (0..=i)
            .rev()
            .find(|&j| is_audited(&body[j]))
            .map(|j| body[j].clone());
        let admits = |allow: &str| match &caller {
            Some(c) => allow == "*" || crate::allowlist::caller_matches(c, allow),
            None => allow == "*",
        };
        let admitted = matching.iter().find(|(_, allow)| admits(allow));
        return match admitted {
            Some((item, _)) => (true, Some(item.clone()), caller),
            None => (false, Some(matching[0].0.clone()), caller),
        };
    }
    (false, None, None)
}

fn render_path(path: &[String]) -> String {
    let mut s = path.join(" -> ");
    if path.len() > 14 {
        let head = path[..6].join(" -> ");
        let tail = path[path.len() - 6..].join(" -> ");
        s = format!("{head} -> ... -> {tail}");
    }
    s
}

enum BodyOutcome<'tcx> {
    Body(&'tcx Body<'tcx>),
    Leaf(LeafKind),
    VirtualFail,
}

/// Unpeel `&T`, `&mut T`, `*const T`, `*mut T` and `Box<T>` from `src` and
/// `target` until a type remains, so a trait-object pointer can be resolved to
/// its concrete self type.
fn unpeel_to_trait<'tcx>(mut src: Ty<'tcx>, mut target: Ty<'tcx>) -> (Ty<'tcx>, Ty<'tcx>) {
    loop {
        let s = match *src.kind() {
            ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => inner,
            _ => match src.boxed_ty() {
                Some(inner) => inner,
                None => break,
            },
        };
        let t = match *target.kind() {
            ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => inner,
            _ => match target.boxed_ty() {
                Some(inner) => inner,
                None => break,
            },
        };
        src = s;
        target = t;
    }
    (src, target)
}

fn is_alloc_entry(tcx: TyCtxt<'_>, instance: Instance<'_>) -> bool {
    let did = instance.def.def_id();
    let path = tcx.def_path_str(did);
    if ALLOC_ENTRY_PATHS
        .iter()
        .any(|p| path == *p || path.starts_with(&format!("{p}::")))
    {
        return true;
    }
    if let Some(sym) = tcx.codegen_fn_attrs(did).symbol_name {
        let s = sym.as_str();
        if ALLOC_SYMBOL_SUBSTRINGS.iter().any(|sub| s.contains(sub)) {
            return true;
        }
    }
    false
}

fn instance_label(tcx: TyCtxt<'_>, instance: Instance<'_>) -> String {
    let base = tcx.def_path_str(instance.def.def_id());
    let args: Vec<String> = instance
        .args
        .iter()
        .filter_map(|a| match a.kind() {
            GenericArgKind::Type(t) => Some(format!("{t}")),
            GenericArgKind::Const(c) => Some(format!("{c:?}")),
            GenericArgKind::Lifetime(_) => None,
        })
        .collect();
    if args.is_empty() {
        base
    } else {
        format!("{base}<{}>", args.join(","))
    }
}
