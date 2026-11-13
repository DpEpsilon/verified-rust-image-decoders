//! The call-graph walk over post-monomorphisation MIR, keyed by `Instance`,
//! and the audit verdict built on it.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use rustc_hir::def::DefKind;
use rustc_middle::mono::MonoItem;
use rustc_middle::mir::{
    AssertKind, Body, CastKind, InlineAsmOperand, Operand, Rvalue, StatementKind, TerminatorKind,
};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{self, Instance, InstanceKind, Ty, TyCtxt, TypeVisitableExt, TypingEnv, VtblEntry};
use rustc_span::def_id::{CrateNum, DefId};
use rustc_span::Span;

use crate::allowlist::{self, Allowlist, Entry, Facts};
use crate::config::AuditConfig;
use crate::inputs;
use crate::names;
use crate::report::{Finding, Report};

const MAX_NODES: usize = 200_000;
const MAX_PATHS: usize = 20_000;
const MAX_DEPTH: usize = 80;
const MAX_SHOWN: usize = 8;

/// Allocator sinks, by exact canonical def-path in the sysroot `alloc` crate.
/// Reaching one ends the walk on that path; every path to one is judged.
const SINK_ALLOC: &[&str] = &[
    "alloc::alloc::__rust_alloc",
    "alloc::alloc::__rust_alloc_zeroed",
    "alloc::alloc::__rust_realloc",
];
const SINK_DEALLOC: &[&str] = &["alloc::alloc::__rust_dealloc"];
/// Abort-on-allocation-failure entry points (SPEC §6: no edge to
/// `handle_alloc_error` outside an admitted contract).
const SINK_ABORT: &[&str] = &[
    "alloc::alloc::handle_alloc_error",
    "alloc::alloc::__rust_alloc_error_handler",
    "alloc::raw_vec::handle_error",
    "alloc::raw_vec::capacity_overflow",
];
/// Crates whose instances may be reached (SPEC §9 audit rule); identity of
/// core/alloc/compiler_builtins is "loaded from the sysroot", of vstd and
/// verus_builtin the crate hashes bound in `[meta]`.
const SYSROOT_CRATES: &[&str] = &["core", "alloc", "compiler_builtins"];
const VERUS_CRATES: &[&str] = &["vstd", "verus_builtin"];
/// Items whose reachability from crate code SPEC §2 rules out.
const FORBIDDEN_FROM_LOCAL: &[&str] = &[
    "core::mem::forget",
    "alloc::boxed::Box::<T, A>::leak",
    "core::mem::manually_drop::ManuallyDrop::<T>::new",
];
const FORBIDDEN_PREFIX: &[&str] = &["alloc::rc::", "alloc::sync::"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SinkKind {
    Alloc,
    Dealloc,
    Abort,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeafKind {
    Intrinsic,
    NoMir,
    Ffi,
}

impl LeafKind {
    fn tag(self) -> &'static str {
        match self {
            LeafKind::Intrinsic => "intrinsic",
            LeafKind::NoMir => "no-mir",
            LeafKind::Ffi => "ffi",
        }
    }
}

#[derive(Clone, Debug)]
enum Class {
    Pending,
    Body,
    Sink(SinkKind),
    Leaf { kind: LeafKind, diverging: bool },
    Virtual,
    Unwalkable(String),
}

struct Node<'tcx> {
    inst: Instance<'tcx>,
    label: String,
    class: Class,
}

pub struct Graph<'tcx> {
    tcx: TyCtxt<'tcx>,
    sysroot_crates: BTreeSet<CrateNum>,
    ids: HashMap<Instance<'tcx>, usize>,
    nodes: Vec<Node<'tcx>>,
    adj: Vec<BTreeSet<usize>>,
    edges: Vec<(usize, usize, &'static str, Span)>,
    queue: VecDeque<usize>,
    findings: Vec<Finding>,
    /// (caller, panic kind name, span) for Assert / UnwindTerminate edges.
    panics: Vec<(usize, String, Span)>,
    capped: bool,
}

impl<'tcx> Graph<'tcx> {
    fn new(tcx: TyCtxt<'tcx>, sysroot_crates: BTreeSet<CrateNum>) -> Self {
        Graph {
            tcx,
            sysroot_crates,
            ids: HashMap::new(),
            nodes: Vec::new(),
            adj: Vec::new(),
            edges: Vec::new(),
            queue: VecDeque::new(),
            findings: Vec::new(),
            panics: Vec::new(),
            capped: false,
        }
    }

    fn node(&mut self, inst: Instance<'tcx>) -> usize {
        if let Some(&id) = self.ids.get(&inst) {
            return id;
        }
        let id = self.nodes.len();
        self.nodes.push(Node { inst, label: names::label(self.tcx, inst), class: Class::Pending });
        self.adj.push(BTreeSet::new());
        self.ids.insert(inst, id);
        self.queue.push_back(id);
        id
    }

    fn edge(&mut self, from: usize, to: Instance<'tcx>, kind: &'static str, span: Span) -> usize {
        let to = self.node(to);
        self.adj[from].insert(to);
        self.edges.push((from, to, kind, span));
        to
    }

    fn span_str(&self, span: Span) -> String {
        if span.is_dummy() {
            return "<no span>".into();
        }
        self.tcx.sess.source_map().span_to_string(span, rustc_span::RemapPathScopeComponents::DIAGNOSTICS)
    }

    fn sink_kind(&self, did: DefId) -> Option<SinkKind> {
        if !self.sysroot_crates.contains(&did.krate) || self.tcx.crate_name(did.krate).as_str() != "alloc" {
            return None;
        }
        let p = names::canon(self.tcx, did);
        if SINK_ALLOC.contains(&p.as_str()) {
            Some(SinkKind::Alloc)
        } else if SINK_DEALLOC.contains(&p.as_str()) {
            Some(SinkKind::Dealloc)
        } else if SINK_ABORT.contains(&p.as_str()) {
            Some(SinkKind::Abort)
        } else {
            None
        }
    }

    fn diverging(&self, did: DefId) -> bool {
        matches!(self.tcx.def_kind(did), DefKind::Fn | DefKind::AssocFn)
            && self.tcx.fn_sig(did).skip_binder().skip_binder().output().is_never()
    }

    fn drain(&mut self) {
        while let Some(id) = self.queue.pop_front() {
            if self.nodes.len() > MAX_NODES {
                if !self.capped {
                    self.capped = true;
                    self.findings.push(Finding::new(
                        "node-cap",
                        format!("traversal exceeded {MAX_NODES} instances; aborted (fail closed)"),
                    ));
                }
                return;
            }
            let inst = self.nodes[id].inst;
            let tcx = self.tcx;
            let class = match inst.def {
                InstanceKind::Intrinsic(did) | InstanceKind::LlvmIntrinsic(did) => {
                    // An intrinsic with a fallback body may be codegened from
                    // that body: walk it as well (the intrinsic stays a leaf).
                    if tcx.is_mir_available(did) && !tcx.is_foreign_item(did) {
                        let fb = Instance { def: InstanceKind::Item(did), args: inst.args };
                        self.edge(id, fb, "intrinsic-fallback", rustc_span::DUMMY_SP);
                    }
                    Class::Leaf { kind: LeafKind::Intrinsic, diverging: self.diverging(did) }
                }
                InstanceKind::Virtual(..) => Class::Virtual,
                InstanceKind::Item(did) => {
                    if let Some(k) = self.sink_kind(did) {
                        Class::Sink(k)
                    } else if tcx.is_foreign_item(did) {
                        // Fail closed on allocator-shaped foreign symbols that
                        // are not on the exact sink list (a renamed shim, or a
                        // crate declaring `__rust_alloc` itself): they are
                        // judged as allocation sinks, never admitted as ffi.
                        let name = tcx.item_name(did).to_string();
                        if name.starts_with("__rust_")
                            && (name.contains("alloc") || name.contains("realloc"))
                            && name != "__rust_no_alloc_shim_is_unstable_v2"
                        {
                            Class::Sink(if name.contains("dealloc") { SinkKind::Dealloc } else { SinkKind::Alloc })
                        } else if ["malloc", "calloc", "realloc", "free", "posix_memalign", "aligned_alloc", "mmap"].contains(&name.as_str()) {
                            Class::Sink(if name == "free" { SinkKind::Dealloc } else { SinkKind::Alloc })
                        } else {
                            Class::Leaf { kind: LeafKind::Ffi, diverging: self.diverging(did) }
                        }
                    } else if !tcx.is_mir_available(did) {
                        Class::Leaf { kind: LeafKind::NoMir, diverging: self.diverging(did) }
                    } else {
                        Class::Body
                    }
                }
                InstanceKind::Shim(_) => Class::Body,
            };
            if matches!(class, Class::Body) {
                let body = tcx.instance_mir(inst.def);
                let bound = ty::EarlyBinder::bind(tcx, body.clone());
                match inst.try_instantiate_mir_and_normalize_erasing_regions(
                    tcx,
                    TypingEnv::fully_monomorphized(),
                    bound,
                ) {
                    Ok(mono) => {
                        self.nodes[id].class = Class::Body;
                        let mono: &'tcx Body<'tcx> = tcx.arena.alloc(mono);
                        self.walk_body(id, mono);
                    }
                    Err(_) => {
                        self.nodes[id].class = Class::Unwalkable("body instantiation failed".into());
                        let l = self.nodes[id].label.clone();
                        self.findings.push(Finding::new(
                            "uninstantiable-body",
                            format!("the body of {l} could not be instantiated; not walked (fail closed)"),
                        ));
                    }
                }
            } else {
                if matches!(class, Class::Virtual) {
                    let l = self.nodes[id].label.clone();
                    self.findings.push(Finding::new("virtual-dispatch", format!("trait-object (virtual) call reached: {l}")));
                }
                self.nodes[id].class = class;
            }
        }
    }

    fn mono_ty(&self, ty: Ty<'tcx>) -> Ty<'tcx> {
        self.tcx
            .try_normalize_erasing_regions(TypingEnv::fully_monomorphized(), ty::Unnormalized::new_wip(ty))
            .unwrap_or(ty)
    }

    fn walk_body(&mut self, id: usize, body: &'tcx Body<'tcx>) {
        // Every inlined callee becomes a node with an edge from its logical
        // caller, even if no statement of it survived optimisation; its own
        // body is then walked in its own right.
        for (sc, data) in body.source_scopes.iter_enumerated() {
            if data.inlined.is_some() {
                self.logical_caller(id, body, sc);
            }
        }
        for bb in body.basic_blocks.iter() {
            for stmt in &bb.statements {
                if let StatementKind::Assign(assign) = &stmt.kind {
                    let (_, rv) = &**assign;
                    let from = self.logical_caller(id, body, stmt.source_info.scope);
                    self.walk_rvalue(from, rv, stmt.source_info.span, body);
                }
            }
            let term = bb.terminator();
            let from = self.logical_caller(id, body, term.source_info.scope);
            self.walk_terminator(from, &term.kind, term.source_info.span, body);
        }
    }

    /// Undo MIR inlining for attribution. rustc records, for every source
    /// scope that came from an inlined callee, the callee `Instance`. A
    /// terminator in such a scope is attributed to the innermost inlined
    /// instance, and the chain `owner -> inlined_1 -> ... -> inlined_k` is
    /// added as edges. So a helper inlined into another function is still its
    /// own caller on every path, and an allowlist `allow` naming function F
    /// never admits code that was merely inlined into F.
    fn logical_caller(&mut self, owner: usize, body: &'tcx Body<'tcx>, scope: rustc_middle::mir::SourceScope) -> usize {
        let mut chain: Vec<(Instance<'tcx>, Span)> = Vec::new();
        let mut s = Some(scope);
        while let Some(sc) = s {
            let data = &body.source_scopes[sc];
            if let Some((inst, sp)) = data.inlined {
                chain.push((inst, sp));
            }
            s = data.parent_scope;
        }
        let mut cur = owner;
        for (inst, sp) in chain.into_iter().rev() {
            if inst.args.has_param() {
                self.fail(cur, "inline-scope-generic", sp, format!("inlined callee {inst:?} is not monomorphic; attribution falls back to the enclosing body"));
                return cur;
            }
            let next = self.node(inst);
            if self.adj[cur].insert(next) {
                self.edges.push((cur, next, "inlined", sp));
            }
            cur = next;
        }
        cur
    }

    fn walk_terminator(&mut self, from: usize, kind: &TerminatorKind<'tcx>, span: Span, body: &'tcx Body<'tcx>) {
        let tcx = self.tcx;
        match kind {
            TerminatorKind::Call { func, args, .. } | TerminatorKind::TailCall { func, args, .. } => {
                for a in args.iter() {
                    self.walk_operand(from, &a.node, span, body);
                }
                let fty = self.mono_ty(func.ty(body, tcx));
                match *fty.kind() {
                    ty::FnDef(def_id, gargs) => {
                        let Some(gargs) = gargs.no_bound_vars() else {
                            self.fail(from, "unresolved-call", span, format!("callee {} has bound vars", names::canon(tcx, def_id)));
                            return;
                        };
                        match Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), def_id, gargs) {
                            Ok(Some(inst)) => {
                                self.edge(from, inst, "call", span);
                            }
                            Ok(None) => self.fail(from, "unresolved-call", span, format!("{} did not resolve to an instance", names::canon(tcx, def_id))),
                            Err(_) => self.fail(from, "unresolved-call", span, format!("resolution of {} errored", names::canon(tcx, def_id))),
                        }
                    }
                    ty::FnPtr(..) => {
                        let n = self.fn_ptr_candidates(from, body, span);
                        self.fail(from, "fn-ptr-call", span, format!("indirect call through a function pointer; {n} candidate target(s) from casts in this body were walked, but the call is not statically resolved"));
                    }
                    _ => self.fail(from, "unknown-callee", span, format!("callee operand of type {fty} is not statically resolvable")),
                }
            }
            TerminatorKind::Drop { place, .. } => {
                let ty = self.mono_ty(place.ty(body, tcx).ty);
                let glue = tcx
                    .lang_items()
                    .get(rustc_hir::attrs::lang_items::LangItem::DropGlue)
                    .and_then(|d| Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), d, tcx.mk_args(&[ty.into()])).ok().flatten());
                match glue {
                    Some(inst) => {
                        self.edge(from, inst, "drop", span);
                    }
                    None => self.fail(from, "unresolved-drop", span, format!("drop glue of {ty} did not resolve")),
                }
            }
            TerminatorKind::Assert { msg, .. } => {
                let name = match &**msg {
                    AssertKind::BoundsCheck { .. } => "PanicBoundsCheck".to_string(),
                    AssertKind::MisalignedPointerDereference { .. } => "PanicMisalignedPointerDereference".to_string(),
                    other => format!("{:?}", other.panic_function()),
                };
                self.panics.push((from, name, span));
            }
            TerminatorKind::UnwindTerminate(reason) => {
                self.panics.push((from, format!("UnwindTerminate::{reason:?}"), span));
            }
            TerminatorKind::InlineAsm { operands, .. } => {
                self.fail(from, "inline-asm", span, "inline assembly".into());
                for op in operands.iter() {
                    match op {
                        InlineAsmOperand::SymFn { value } => {
                            let t = self.mono_ty(value.const_.ty());
                            self.fn_use(from, t, span, true);
                        }
                        InlineAsmOperand::SymStatic { .. } => {
                            self.fail(from, "inline-asm", span, "inline assembly names a static".into())
                        }
                        _ => {}
                    }
                }
            }
            TerminatorKind::SwitchInt { discr, .. } => self.walk_operand(from, discr, span, body),
            TerminatorKind::Yield { .. } | TerminatorKind::CoroutineDrop => {
                self.fail(from, "unsupported-terminator", span, format!("{kind:?}"))
            }
            _ => {}
        }
    }

    fn fail(&mut self, from: usize, tag: &str, span: Span, what: String) {
        let l = self.nodes[from].label.clone();
        let s = self.span_str(span);
        self.findings.push(Finding::new(tag, format!("{l}: {what} at {s}")));
    }

    fn walk_rvalue(&mut self, from: usize, rv: &Rvalue<'tcx>, span: Span, body: &'tcx Body<'tcx>) {
        let tcx = self.tcx;
        match rv {
            Rvalue::Cast(
                CastKind::PointerCoercion(PointerCoercion::ReifyFnPointer(_) | PointerCoercion::ClosureFnPointer(_), _),
                op,
                _,
            ) => {
                let t = self.mono_ty(op.ty(body, tcx));
                self.fn_use(from, t, span, false);
            }
            Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize, _), op, target) => {
                let src = self.mono_ty(op.ty(body, tcx));
                let target = self.mono_ty(*target);
                let (st, tt) = unpeel(src, target);
                if tt.is_trait() {
                    self.vtable(from, st, tt, span);
                }
                self.walk_operand(from, op, span, body);
            }
            Rvalue::Use(op, _) | Rvalue::Repeat(op, _) | Rvalue::UnaryOp(_, op) | Rvalue::Cast(_, op, _) => {
                self.walk_operand(from, op, span, body)
            }
            Rvalue::BinaryOp(_, ops) => {
                self.walk_operand(from, &ops.0, span, body);
                self.walk_operand(from, &ops.1, span, body);
            }
            Rvalue::Aggregate(_, ops) => {
                for op in ops.iter() {
                    self.walk_operand(from, op, span, body);
                }
            }
            _ => {}
        }
    }

    /// A function item used as a value (passed as an argument, stored): walk
    /// the instance it would call, conservatively.
    fn walk_operand(&mut self, from: usize, op: &Operand<'tcx>, span: Span, body: &'tcx Body<'tcx>) {
        let t = self.mono_ty(op.ty(body, self.tcx));
        if let ty::FnDef(..) = t.kind() {
            self.fn_use(from, t, span, false);
        }
    }

    fn fn_use(&mut self, from: usize, t: Ty<'tcx>, span: Span, direct: bool) {
        let tcx = self.tcx;
        if let ty::FnDef(did, gargs) = *t.kind() {
            let Some(gargs) = gargs.no_bound_vars() else {
                self.fail(from, "unresolved-fn-use", span, format!("{} has bound vars", names::canon(tcx, did)));
                return;
            };
            let inst = if direct {
                Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), did, gargs).ok().flatten()
            } else {
                Instance::resolve_for_fn_ptr(tcx, TypingEnv::fully_monomorphized(), did, gargs)
            };
            match inst {
                Some(i) => {
                    self.edge(from, i, "fn-use", span);
                }
                None => self.fail(from, "unresolved-fn-use", span, format!("{} used as a value did not resolve", names::canon(tcx, did))),
            }
        }
    }

    fn fn_ptr_candidates(&mut self, from: usize, body: &'tcx Body<'tcx>, span: Span) -> usize {
        let tcx = self.tcx;
        let mut n = 0;
        for bb in body.basic_blocks.iter() {
            for stmt in &bb.statements {
                let StatementKind::Assign(a) = &stmt.kind else { continue };
                if let Rvalue::Cast(
                    CastKind::PointerCoercion(PointerCoercion::ReifyFnPointer(_) | PointerCoercion::ClosureFnPointer(_), _),
                    op,
                    _,
                ) = &a.1
                {
                    let t = self.mono_ty(op.ty(body, tcx));
                    if let ty::FnDef(did, gargs) = *t.kind() {
                        if let Some(g) = gargs.no_bound_vars() {
                            if let Some(i) = Instance::resolve_for_fn_ptr(tcx, TypingEnv::fully_monomorphized(), did, g) {
                                self.edge(from, i, "fn-ptr-candidate", span);
                                n += 1;
                            }
                        }
                    }
                }
            }
        }
        n
    }

    fn vtable(&mut self, from: usize, src: Ty<'tcx>, target: Ty<'tcx>, span: Span) {
        let tcx = self.tcx;
        if let ty::Dynamic(preds, _) = target.kind() {
            if let Some(principal) = preds.principal() {
                let tr = tcx.instantiate_bound_regions_with_erased(principal.with_self_ty(tcx, src));
                for e in tcx.vtable_entries(tr) {
                    if let VtblEntry::Method(i) = e {
                        self.edge(from, *i, "vtable-method", span);
                    }
                }
            }
        }
        // Drop glue of the concrete type is also in the vtable.
        if let Some(d) = tcx.lang_items().get(rustc_hir::attrs::lang_items::LangItem::DropGlue) {
            if let Ok(Some(i)) = Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), d, tcx.mk_args(&[src.into()])) {
                self.edge(from, i, "vtable-drop", span);
            }
        }
        self.fail(from, "vtable-unsize", span, format!("unsizes {src} to trait object {target}; vtable methods walked, dynamic dispatch fails closed"));
    }

    fn is_local(&self, id: usize) -> bool {
        self.nodes[id].inst.def.def_id().is_local()
    }
}

fn unpeel<'tcx>(mut s: Ty<'tcx>, mut t: Ty<'tcx>) -> (Ty<'tcx>, Ty<'tcx>) {
    loop {
        let ns = match *s.kind() {
            ty::Ref(_, i, _) | ty::RawPtr(i, _) => i,
            _ => match s.boxed_ty() {
                Some(i) => i,
                None => break,
            },
        };
        let nt = match *t.kind() {
            ty::Ref(_, i, _) | ty::RawPtr(i, _) => i,
            _ => match t.boxed_ty() {
                Some(i) => i,
                None => break,
            },
        };
        s = ns;
        t = nt;
    }
    (s, t)
}

// ---------------------------------------------------------------------------
// Allowlist value resolution against DefIds.

enum Resolved {
    Did(DefId, Option<String>),
    Star,
    Bad,
}

struct Resolver {
    by_canon: HashMap<String, HashSet<DefId>>,
}

impl Resolver {
    fn resolve(&self, v: &str, what: &str, out: &mut Vec<Finding>) -> Resolved {
        let v = v.trim();
        if v == "*" {
            return Resolved::Star;
        }
        let pv = names::parse_value(v);
        let mut cands: HashSet<DefId> = HashSet::new();
        let local_spelled = pv.base.starts_with("crate::") || pv.base.starts_with("<crate::");
        let get = |k: &str| self.by_canon.get(k).cloned().unwrap_or_default();
        if local_spelled {
            cands.extend(get(&pv.base).into_iter().filter(|d| d.is_local()));
        } else {
            cands.extend(get(&pv.base).into_iter().filter(|d| !d.is_local()));
            cands.extend(get(&format!("crate::{}", pv.base)).into_iter().filter(|d| d.is_local()));
            if let Some(rest) = pv.base.strip_prefix('<') {
                cands.extend(get(&format!("<crate::{rest}")).into_iter().filter(|d| d.is_local()));
            }
        }
        match cands.len() {
            0 => {
                out.push(Finding::new(
                    "allowlist-unresolved",
                    format!("{what} {v:?} names no item in this compilation's call graph or crate"),
                ));
                Resolved::Bad
            }
            1 => {
                let d = *cands.iter().next().unwrap();
                if d.is_local() && !local_spelled {
                    out.push(Finding::new(
                        "allowlist-local-spelling",
                        format!("{what} {v:?} names a local item; write it as \"crate::{}\"", pv.base),
                    ));
                    return Resolved::Bad;
                }
                Resolved::Did(d, pv.args)
            }
            n => {
                out.push(Finding::new(
                    "allowlist-ambiguous",
                    format!("{what} {v:?} resolves to {n} distinct items (a local item and an external one, or several crates of one name); it matches none of them"),
                ));
                Resolved::Bad
            }
        }
    }
}

struct AllocRule {
    idx: usize,
    item: DefId,
    args: Option<String>,
    allow: Resolved,
}

struct LeafRule {
    idx: usize,
    kind: String,
    item: Option<DefId>,
    panic_name: Option<String>,
    args: Option<String>,
    allow: Option<Resolved>,
}

// ---------------------------------------------------------------------------

pub fn run_audit<'tcx>(tcx: TyCtxt<'tcx>, cfg: &AuditConfig) -> Report {
    let mut rep = Report::default();
    let mut f: Vec<Finding> = Vec::new();

    // --- compilation facts -------------------------------------------------
    let (profile, class) = inputs::detect_profile(tcx);
    if tcx.sess.opts.share_generics() {
        f.push(Finding::new("share-generics", "-Zshare-generics is on: generic std instantiations would come from upstream crates".into()));
    }
    let (features, other_cfgs) = inputs::cfgs(tcx, &mut f);
    let src = inputs::source_files(tcx, &mut f);
    let crates = inputs::crate_graph(tcx);
    let vstd = inputs::vstd_info(tcx, &crates, cfg.prototype_mode, cfg.vstd_override.as_deref());
    let bs = inputs::build_script();
    let target = tcx.sess.opts.target_triple.tuple().to_string();
    let commit_short = inputs::commit_short();
    let commit_full = inputs::commit_full(tcx);
    let local_svh = inputs::local_svh(tcx);

    if !bs.under_cargo && !cfg.prototype_mode {
        f.push(Finding::new(
            "not-under-cargo",
            "CARGO_MANIFEST_DIR is not set: the build script and the locked resolution cannot be checked; production audits run under cargo".into(),
        ));
    }

    // --- allowlist -----------------------------------------------------------
    let al: Option<Allowlist> = match &cfg.allowlist_path {
        Some(p) => allowlist::load(p, &mut f),
        None => {
            f.push(Finding::new(
                "allowlist-absent",
                "no allowlist was supplied: this run is a diagnostic and can never pass".into(),
            ));
            None
        }
    };
    let facts = Facts {
        crate_name: cfg.crate_name.clone(),
        commit_short: commit_short.clone(),
        commit_full: commit_full.clone(),
        target: target.clone(),
        profile: profile.clone(),
        features: features.clone(),
        source_hash: src.combined.clone(),
        vstd_linked: vstd.linked,
        vstd_version: vstd.version.clone(),
        vstd_svh: vstd.svh.clone(),
        vstd_source_checksum: vstd.source_checksum.clone(),
        vstd_problems: vstd.problems.clone(),
        verus_builtin_svh: vstd.verus_builtin_svh.clone(),
        prototype_mode: cfg.prototype_mode,
        vstd_override: cfg.vstd_override.clone(),
    };
    if let Some(a) = &al {
        allowlist::check_meta(a, &facts, &mut f);
    } else if cfg.vstd_override.is_some() && !cfg.prototype_mode {
        f.push(Finding::new("vstd-override-forbidden", "ALLOC_AUDIT_VSTD_VERSION is set without --prototype".into()));
    }
    let al = al.as_ref();

    // --- build inputs (compiler-resolved) -------------------------------------
    let mut bi_lines = Vec::new();
    let declared = |kind: &str, item: &str| -> Vec<&Entry> {
        allowlist::of_kind(al, kind).filter(|e| e.item.trim() == item).collect()
    };
    if bs.present {
        let shown = bs.path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "build.rs".into());
        let hash = bs.path.as_ref().and_then(|p| inputs::sha256_file(p));
        bi_lines.push(format!("build-script {shown} sha256={hash:?} ({})", bs.why.join("; ")));
        let d = declared("build-script", "build.rs");
        if d.is_empty() {
            f.push(Finding::new("undeclared-build-script", format!("the package has a build script ({}); declare kind = \"build-script\", item = \"build.rs\" with its sha256", bs.why.join("; "))));
        } else if !cfg.prototype_mode && !d.iter().any(|e| e.sha256.as_deref() == hash.as_deref() && hash.is_some()) {
            f.push(Finding::new("build-input-hash", format!("build script {shown} hashes to {hash:?}, not the declared sha256")));
        }
    } else {
        bi_lines.push("build-script: none".into());
    }
    let mut inc_files: BTreeMap<String, Option<String>> = BTreeMap::new();
    for (p, h, is_mod) in &src.files {
        if !is_mod {
            inc_files.insert(p.clone(), Some(h.clone()));
        }
    }
    for p in &src.file_depinfo {
        inc_files.entry(p.clone()).or_insert_with(|| inputs::sha256_file(std::path::Path::new(p)));
    }
    if inc_files.is_empty() {
        bi_lines.push("include: none (no file loaded other than module files)".into());
    }
    for (p, h) in &inc_files {
        bi_lines.push(format!("include {p} sha256={h:?}"));
        let d = declared("include", p);
        if d.is_empty() {
            f.push(Finding::new("undeclared-include", format!("the compilation loaded {p} (include!/include_str!/include_bytes!), which is not a module file; declare kind = \"include\", item = {p:?} with its sha256")));
        } else if !cfg.prototype_mode && !d.iter().any(|e| e.sha256.as_deref() == h.as_deref()) {
            f.push(Finding::new("build-input-hash", format!("{p} hashes to {h:?}, not the declared sha256")));
        }
    }
    if src.env_depinfo.is_empty() {
        bi_lines.push("env: none (no env!/option_env!/tracked env read)".into());
    }
    for (k, v) in &src.env_depinfo {
        bi_lines.push(format!("env {k}={v:?}"));
        if declared("env", k).is_empty() {
            f.push(Finding::new("undeclared-env", format!("the compilation read environment variable {k} (env!/option_env!/tracked); declare kind = \"env\", item = {k:?}")));
        }
    }
    let mut pm_seen = false;
    for c in &crates {
        let arts: Vec<String> = c.artifacts.iter().map(|(p, h)| format!("{p} sha256={}", h.clone().unwrap_or_default())).collect();
        if c.proc_macro {
            pm_seen = true;
            let h = c.artifacts.iter().find_map(|a| a.1.clone());
            bi_lines.push(format!("proc-macro {} svh={} {}", c.name, c.svh, arts.join(" ")));
            let d = declared("proc-macro", &c.name);
            if d.is_empty() {
                f.push(Finding::new("undeclared-proc-macro", format!("the compilation loads proc-macro crate {} (crate kind, not dylib presence); declare kind = \"proc-macro\", item = {:?} with its artifact sha256", c.name, c.name)));
            } else if !cfg.prototype_mode && !d.iter().any(|e| e.sha256.is_some() && e.sha256 == h) {
                f.push(Finding::new("proc-macro-artifact", format!("proc-macro crate {} artifact hashes to {h:?}, not the declared sha256 (Verus macro crates are bound by name and artifact hash)", c.name)));
            }
        } else if c.artifacts.iter().any(|(p, _)| p.ends_with(".so") || p.ends_with(".dylib")) && !c.from_sysroot {
            f.push(Finding::new("dylib-dependency", format!("crate {} is linked as a dylib and is not a proc-macro crate", c.name)));
        }
    }
    if !pm_seen {
        bi_lines.push("proc-macro: none".into());
    }

    // --- crate graph -----------------------------------------------------------
    let sysroot_crates: BTreeSet<CrateNum> = crates
        .iter()
        .filter(|c| c.from_sysroot && SYSROOT_CRATES.contains(&c.name.as_str()))
        .map(|c| c.cnum)
        .collect();

    // --- roots -------------------------------------------------------------------
    let vis = tcx.effective_visibilities(());
    let rng = tcx.reachable_non_generics(rustc_span::def_id::LOCAL_CRATE);
    let mut exported: Vec<DefId> = Vec::new();
    for local in tcx.iter_local_def_id() {
        if !matches!(tcx.def_kind(local), DefKind::Fn | DefKind::AssocFn) {
            continue;
        }
        let did = local.to_def_id();
        if !(vis.is_reachable(local) || rng.contains_key(&did)) {
            continue;
        }
        if !tcx.is_mir_available(did) {
            continue; // e.g. a required trait method without a default body
        }
        exported.push(did);
    }
    // Resolution universe (finished after the walk): local fn-like items now.
    let mut universe: HashSet<DefId> = tcx
        .iter_local_def_id()
        .filter(|l| matches!(tcx.def_kind(*l), DefKind::Fn | DefKind::AssocFn | DefKind::Closure))
        .map(|l| l.to_def_id())
        .collect();
    let pre_resolver = make_resolver(tcx, &universe);
    let mut excluded: HashSet<DefId> = HashSet::new();
    for e in allowlist::of_kind(al, "root-exclude") {
        if let Resolved::Did(d, _) = pre_resolver.resolve(&e.item, "root-exclude", &mut f) {
            if !exported.contains(&d) {
                f.push(Finding::new("allowlist-root-exclude", format!("root-exclude {:?} is not an exported function", e.item)));
            }
            excluded.insert(d);
        }
    }
    let mut generic_declared: HashSet<DefId> = HashSet::new();
    for e in allowlist::of_kind(al, "generic-export") {
        if let Resolved::Did(d, _) = pre_resolver.resolve(&e.item, "generic-export", &mut f) {
            generic_declared.insert(d);
        }
    }
    let mut roots: Vec<DefId> = Vec::new();
    let mut generic_exports: Vec<DefId> = Vec::new();
    let explicit: Vec<String> = cfg.roots.iter().cloned().chain(al.and_then(|a| a.meta.required_roots.clone()).unwrap_or_default()).collect();
    let explicit_dids: Vec<(String, Option<DefId>)> = explicit
        .iter()
        .map(|r| {
            let v = if r.starts_with("crate::") { r.clone() } else { format!("crate::{r}") };
            let d = pre_resolver.by_canon.get(&v).and_then(|s| if s.len() == 1 { s.iter().next().copied() } else { None });
            (r.clone(), d)
        })
        .collect();
    let replace = cfg.prototype_mode && !cfg.roots.is_empty();
    let candidates: Vec<DefId> = if replace {
        explicit_dids.iter().filter_map(|(_, d)| *d).collect()
    } else {
        exported.clone()
    };
    for did in candidates {
        if excluded.contains(&did) {
            continue;
        }
        if tcx.generics_of(did).requires_monomorphization(tcx) {
            generic_exports.push(did);
            if !generic_declared.contains(&did) {
                f.push(Finding::new(
                    "generic-pub-fn",
                    format!("exported generic function {} cannot be a root; SPEC §6 step 1 requires no generic export (declare kind = \"generic-export\" with a citation to admit it; its instantiations reached from the roots are audited)", names::canon(tcx, did)),
                ));
            }
            continue;
        }
        roots.push(did);
    }
    for (r, d) in &explicit_dids {
        match d {
            Some(d) if roots.contains(d) => {}
            _ => f.push(Finding::new("missing-root", format!("required root {r} is not an exported non-generic function of this crate"))),
        }
    }
    if roots.is_empty() {
        f.push(Finding::new("no-roots", "no root functions; nothing was audited".into()));
    }

    // --- walk ------------------------------------------------------------------
    let mut g = Graph::new(tcx, sysroot_crates.clone());
    let mut root_ids: Vec<usize> = Vec::new();
    for &d in &roots {
        root_ids.push(g.node(Instance::mono(tcx, d)));
    }
    g.drain();
    // Shadow walk of excluded roots: only to account for their codegen items.
    let mut shadow = Graph::new(tcx, sysroot_crates.clone());
    for &d in &excluded {
        if !tcx.generics_of(d).requires_monomorphization(tcx) {
            shadow.node(Instance::mono(tcx, d));
        }
    }
    shadow.drain();
    f.extend(g.findings.drain(..));

    // --- resolution universe and rules -------------------------------------------
    for n in &g.nodes {
        universe.insert(n.inst.def.def_id());
    }
    let resolver = make_resolver(tcx, &universe);
    let mut alloc_rules: Vec<AllocRule> = Vec::new();
    let mut leaf_rules: Vec<LeafRule> = Vec::new();
    if let Some(a) = al {
        for (idx, e) in a.entries.iter().enumerate() {
            match e.kind.as_str() {
                "alloc" => {
                    if e.item.trim() == "*" {
                        continue;
                    }
                    if let Resolved::Did(d, args) = resolver.resolve(&e.item, "alloc item", &mut f) {
                        let allow = match e.allow.as_deref() {
                            Some(v) => resolver.resolve(v, "alloc allow", &mut f),
                            None => Resolved::Bad,
                        };
                        alloc_rules.push(AllocRule { idx, item: d, args, allow });
                    }
                }
                "intrinsic" | "no-mir" | "ffi" | "panic" => {
                    if e.item.trim() == "*" {
                        continue;
                    }
                    let (item, panic_name, args) = if e.kind == "panic" {
                        (None, Some(e.item.trim().trim_start_matches("panic::").to_string()), None)
                    } else {
                        match resolver.resolve(&e.item, "leaf item", &mut f) {
                            Resolved::Did(d, a) => (Some(d), None, a),
                            _ => continue,
                        }
                    };
                    let allow = e.allow.as_deref().map(|v| resolver.resolve(v, "leaf allow", &mut f));
                    leaf_rules.push(LeafRule { idx, kind: e.kind.clone(), item, panic_name, args, allow });
                }
                _ => {}
            }
        }
    }
    let mut used: BTreeSet<usize> = BTreeSet::new();

    // --- leaves ------------------------------------------------------------------
    // Callers of each node.
    let mut callers: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); g.nodes.len()];
    for &(a, b, _, _) in &g.edges {
        callers[b].insert(a);
    }
    let mut leaf_lines = Vec::new();
    for (id, n) in g.nodes.iter().enumerate() {
        let Class::Leaf { kind, diverging } = n.class else { continue };
        let did = n.inst.def.def_id();
        let args = names::args_str(tcx, n.inst);
        for &c in &callers[id] {
            let cdid = g.nodes[c].inst.def.def_id();
            let hit = leaf_rules.iter().find(|r| {
                r.kind == kind.tag()
                    && r.item == Some(did)
                    && r.args.as_ref().is_none_or(|a| *a == args)
                    && match &r.allow {
                        None => !diverging,
                        Some(Resolved::Did(a, _)) => *a == cdid,
                        _ => false,
                    }
            });
            match hit {
                Some(r) => {
                    used.insert(r.idx);
                    leaf_lines.push(format!("ok [{}{}] {} <- {}", kind.tag(), if diverging { ",diverging" } else { "" }, n.label, g.nodes[c].label));
                }
                None => {
                    leaf_lines.push(format!("!! [{}{}] {} <- {}", kind.tag(), if diverging { ",diverging" } else { "" }, n.label, g.nodes[c].label));
                    f.push(Finding::new(
                        "unexplained-leaf",
                        format!("[{}{}] {} reached from {} has no allowlist entry{}", kind.tag(), if diverging { ", diverging: per-caller entry required" } else { "" }, n.label, g.nodes[c].label, if diverging { "" } else { "" }),
                    ));
                }
            }
        }
    }
    let mut panic_lines = Vec::new();
    let mut panic_seen: BTreeSet<(usize, String)> = BTreeSet::new();
    for (c, name, span) in &g.panics {
        if !panic_seen.insert((*c, name.clone())) {
            continue;
        }
        let cdid = g.nodes[*c].inst.def.def_id();
        let hit = leaf_rules.iter().find(|r| {
            r.kind == "panic"
                && r.panic_name.as_deref() == Some(name.as_str())
                && matches!(&r.allow, Some(Resolved::Did(a, _)) if *a == cdid)
        });
        match hit {
            Some(r) => {
                used.insert(r.idx);
                panic_lines.push(format!("ok {name} in {}", g.nodes[*c].label));
            }
            None => {
                panic_lines.push(format!("!! {name} in {} at {}", g.nodes[*c].label, g.span_str(*span)));
                f.push(Finding::new("unexplained-leaf", format!("[panic] {name} in {} at {} has no per-function allowlist entry (T1/Safe' citation)", g.nodes[*c].label, g.span_str(*span))));
            }
        }
    }

    // --- every-path authorisation of allocator sinks ----------------------------
    let mut sink_lines = Vec::new();
    let sinks: Vec<(usize, SinkKind)> = g
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if let Class::Sink(k) = n.class { Some((i, k)) } else { None })
        .collect();
    for &(s, sk) in &sinks {
        // Nodes that can reach `s` (reverse BFS), to prune the path search.
        let mut can: Vec<bool> = vec![false; g.nodes.len()];
        let mut q = VecDeque::from([s]);
        can[s] = true;
        while let Some(x) = q.pop_front() {
            for &p in &callers[x] {
                if !can[p] {
                    can[p] = true;
                    q.push_back(p);
                }
            }
        }
        let mut justified = 0usize;
        let mut bad_total = 0usize;
        let mut shown: Vec<String> = Vec::new();
        let mut first_ok: Option<String> = None;
        let mut truncated = false;
        for &r in &root_ids {
            if !can[r] {
                continue;
            }
            let mut paths: Vec<Vec<usize>> = Vec::new();
            let mut budget = MAX_PATHS;
            let mut stack = vec![r];
            let mut on: Vec<bool> = vec![false; g.nodes.len()];
            on[r] = true;
            dfs(r, s, &g.adj, &can, &mut stack, &mut on, &mut paths, &mut budget, &mut truncated);
            for p in &paths {
                let (ok, why) = judge(&g, p, sk, &alloc_rules, &mut used);
                let rendered = render(&g, p);
                if ok {
                    justified += 1;
                    if first_ok.is_none() {
                        first_ok = Some(format!("{rendered}  [{why}]"));
                    }
                } else {
                    bad_total += 1;
                    if shown.len() < MAX_SHOWN {
                        shown.push(format!("{rendered}  [{why}]"));
                    }
                }
            }
        }
        let label = &g.nodes[s].label;
        if truncated {
            f.push(Finding::new("path-bound", format!("path enumeration to {label} exceeded {MAX_PATHS} paths per root or depth {MAX_DEPTH}; failing closed")));
        }
        for p in &shown {
            f.push(Finding::new("unauthorized-alloc-path", format!("{label} ({sk:?}) is reachable on an unauthorized path: {p}")));
        }
        if bad_total > shown.len() {
            f.push(Finding::new("unauthorized-alloc-path", format!("{label}: {} further unauthorized path(s) not shown", bad_total - shown.len())));
        }
        let ok = bad_total == 0 && justified > 0 && !truncated;
        sink_lines.push(format!("{} {label} ({sk:?}): {justified} justified path(s), {bad_total} unauthorized", if ok { "OK" } else { "!!" }));
        if let Some(p) = first_ok {
            sink_lines.push(format!("    e.g. {p}"));
        }
        for p in &shown {
            sink_lines.push(format!("    UNAUTHORIZED {p}"));
        }
    }

    // --- crate-origin rule (SPEC §9) ----------------------------------------------
    let mut per_crate: BTreeMap<String, usize> = BTreeMap::new();
    let crate_entries: Vec<&Entry> = allowlist::of_kind(al, "crate").collect();
    let mut admitted_crates: BTreeSet<CrateNum> = BTreeSet::new();
    for e in &crate_entries {
        let name = e.item.trim();
        if name == "std" {
            f.push(Finding::new("allowlist-crate-std", "a kind = \"crate\" entry may not admit std".into()));
            continue;
        }
        let matches: Vec<&inputs::CrateInfo> = crates.iter().filter(|c| c.name == name).collect();
        match matches.len() {
            0 => f.push(Finding::new("allowlist-unresolved", format!("crate entry {name:?} names no crate in this compilation"))),
            1 => {
                admitted_crates.insert(matches[0].cnum);
            }
            n => f.push(Finding::new("allowlist-ambiguous-crate", format!("crate entry {name:?} matches {n} crates in this compilation; it admits none of them"))),
        }
    }
    let mut foreign_seen: BTreeSet<CrateNum> = BTreeSet::new();
    for n in &g.nodes {
        let did = n.inst.def.def_id();
        let name = if did.is_local() { cfg.crate_name.clone() } else { tcx.crate_name(did.krate).to_string() };
        *per_crate.entry(name.clone()).or_default() += 1;
        if did.is_local() {
            continue;
        }
        let k = did.krate;
        if name == "std" {
            if foreign_seen.insert(k) {
                f.push(Finding::new("std-instance", format!("an instance from std is reachable: {}", n.label)));
            }
            continue;
        }
        let ok = (SYSROOT_CRATES.contains(&name.as_str()) && sysroot_crates.contains(&k))
            || (VERUS_CRATES.contains(&name.as_str()) && crates.iter().filter(|c| c.name == name).count() == 1)
            || admitted_crates.contains(&k);
        if !ok && foreign_seen.insert(k) {
            f.push(Finding::new("foreign-crate-instance", format!("an instance from crate {name} (not core/alloc/vstd/verus_builtin/compiler_builtins from their bound sources, and not admitted) is reachable: {}", n.label)));
        }
    }

    // --- dependency contracts --------------------------------------------------
    let mut dep_lines = Vec::new();
    if let Some(a) = al {
        for e in allowlist::of_kind(al, "dependency") {
            let name = e.item.trim();
            let Some(c) = crates.iter().find(|c| c.name == name) else {
                f.push(Finding::new("dependency-contract", format!("dependency {name} is not linked")));
                continue;
            };
            let rp = allowlist::relative_to(a, e.report.as_deref().unwrap_or(""));
            match allowlist::read_report_header(&rp) {
                None => f.push(Finding::new("dependency-contract", format!("dependency {name}: cannot read its audit report {}", rp.display()))),
                Some(h) => {
                    let get = |k: &str| h.get(k).cloned().unwrap_or_default();
                    dep_lines.push(format!("{name}: report {} verdict={} crate-svh={} linked-svh={} source_hash={} findings-other-than-unsigned-reviewer={}", rp.display(), get("verdict"), get("crate-svh"), c.svh, get("source_hash"), get("findings-other-than-unsigned-reviewer")));
                    if get("verdict") != "PASS" {
                        f.push(Finding::new("dependency-contract", format!("dependency {name}: its own audit report {} does not PASS (verdict {:?}, finding tags {:?})", rp.display(), get("verdict"), get("finding-tags"))));
                    }
                    if get("crate-svh") != c.svh {
                        f.push(Finding::new("dependency-contract", format!("dependency {name}: its report audited crate hash {:?}, but the linked {name} has {:?}", get("crate-svh"), c.svh)));
                    }
                    if get("target") != target || get("profile") != profile {
                        f.push(Finding::new("dependency-contract", format!("dependency {name}: its report is for target/profile {:?}/{:?}", get("target"), get("profile"))));
                    }
                }
            }
            if !admitted_crates.contains(&c.cnum) {
                f.push(Finding::new("dependency-contract", format!("dependency {name} has a dependency entry but no kind = \"crate\" entry")));
            }
        }
        for c in &crate_entries {
            if !allowlist::of_kind(al, "dependency").any(|e| e.item.trim() == c.item.trim()) {
                f.push(Finding::new("dependency-contract", format!("crate {} is admitted without a dependency entry naming its own passing audit report", c.item.trim())));
            }
        }
    }

    // --- guard call sites, forbidden items, generic exports ---------------------
    let mut guard_lines = Vec::new();
    for e in allowlist::of_kind(al, "guard-calls") {
        if let Resolved::Did(gd, _) = resolver.resolve(&e.item, "guard-calls item", &mut f) {
            let mut spans: BTreeSet<String> = BTreeSet::new();
            for &(a, b, kind, span) in &g.edges {
                if kind == "call" && g.nodes[b].inst.def.def_id() == gd && g.is_local(a) {
                    spans.insert(format!("{} (in {})", g.span_str(span), g.nodes[a].label));
                }
            }
            let distinct: BTreeSet<String> = spans.iter().map(|s| s.split(" (in ").next().unwrap_or("").to_string()).collect();
            guard_lines.push(format!("{}: {} call site(s), expected {}", e.item, distinct.len(), e.sites.unwrap_or(0)));
            for s in &spans {
                guard_lines.push(format!("    {s}"));
            }
            if distinct.len() as u64 != e.sites.unwrap_or(u64::MAX) {
                f.push(Finding::new("guard-call-sites", format!("{} has {} distinct call site(s) in crate code; the allowlist requires {}", e.item, distinct.len(), e.sites.unwrap_or(0))));
            }
        }
    }
    for &(a, b, _, span) in &g.edges {
        let p = names::canon(tcx, g.nodes[b].inst.def.def_id());
        let from_sysroot = sysroot_crates.contains(&g.nodes[b].inst.def.def_id().krate);
        if from_sysroot && FORBIDDEN_PREFIX.iter().any(|x| p.starts_with(x)) {
            f.push(Finding::new("forbidden-item", format!("{} reaches {} (Rc/Arc) at {}", g.nodes[a].label, g.nodes[b].label, g.span_str(span))));
        }
        if from_sysroot && g.is_local(a) && FORBIDDEN_FROM_LOCAL.contains(&p.as_str()) {
            f.push(Finding::new("forbidden-item", format!("{} calls {} at {} (SPEC §2: no mem::forget, Box::leak, ManuallyDrop)", g.nodes[a].label, g.nodes[b].label, g.span_str(span))));
        }
    }
    let mut gd_sorted: Vec<DefId> = generic_declared.iter().copied().collect();
    gd_sorted.sort_by_key(|d| names::canon(tcx, *d));
    for d in &gd_sorted {
        let insts: Vec<&str> = g.nodes.iter().filter(|n| n.inst.def.def_id() == *d).map(|n| n.label.as_str()).collect();
        if insts.is_empty() {
            f.push(Finding::new("generic-export-unreached", format!("declared generic export {} has no instantiation reached from the roots, so no body of it was audited", names::canon(tcx, *d))));
        } else {
            guard_lines.push(format!("generic export {} audited at: {}", names::canon(tcx, *d), insts.join(", ")));
        }
    }

    // --- SPEC §3: allocation-capable guard functions carry #[inline(never)] ----
    {
        let mut reach_sink: Vec<bool> = vec![false; g.nodes.len()];
        let mut q: VecDeque<usize> = sinks.iter().map(|(s, _)| *s).collect();
        for &(s, _) in &sinks {
            reach_sink[s] = true;
        }
        while let Some(x) = q.pop_front() {
            for &p in &callers[x] {
                if !reach_sink[p] {
                    reach_sink[p] = true;
                    q.push_back(p);
                }
            }
        }
        let mut seen_g: HashSet<DefId> = HashSet::new();
        for (i, n) in g.nodes.iter().enumerate() {
            let did = n.inst.def.def_id();
            if !did.is_local() || !reach_sink[i] || !matches!(n.inst.def, InstanceKind::Item(_)) {
                continue;
            }
            let p = names::canon(tcx, did);
            if !(p.starts_with("crate::alloc_guard::") || p.starts_with("<crate::alloc_guard::")) {
                continue;
            }
            if !matches!(tcx.def_kind(did), DefKind::Fn | DefKind::AssocFn) || !seen_g.insert(did) {
                continue;
            }
            let never = matches!(tcx.codegen_fn_attrs(did).inline, rustc_hir::attrs::InlineAttr::Never);
            guard_lines.push(format!("guard fn {p}: reaches an allocator sink; #[inline(never)] {}", if never { "present" } else { "MISSING" }));
            if !never {
                f.push(Finding::new(
                    "guard-inline-never",
                    format!("{p} is an allocation-capable alloc_guard function without #[inline(never)] (SPEC §3, §8 prerequisite 4); the MIR inliner and LLVM may merge it into its callers, so the binary scan (§6 item 4) cannot attribute its allocation"),
                ));
            }
        }
    }

    // --- independent cross-check against rustc's own mono-item collector -------
    let mut mono_unwalked = Vec::new();
    let parts = tcx.collect_and_partition_mono_items(());
    let mut mono_count = 0usize;
    for cgu in parts.codegen_units {
        for (item, _) in cgu.items() {
            mono_count += 1;
            match item {
                MonoItem::Fn(inst) => {
                    if !g.ids.contains_key(inst) && !shadow.ids.contains_key(inst) {
                        mono_unwalked.push(names::label(tcx, *inst));
                    }
                }
                MonoItem::Static(did) => {
                    f.push(Finding::new("static-item", format!("codegen unit contains static {}; statics are outside the call-graph walk", names::canon(tcx, *did))));
                }
                MonoItem::GlobalAsm(_) => f.push(Finding::new("global-asm", "codegen unit contains global_asm!".into())),
            }
        }
    }
    mono_unwalked.sort();
    mono_unwalked.dedup();
    for m in &mono_unwalked {
        f.push(Finding::new("mono-item-unwalked", format!("rustc's mono-item collector codegens {m}, which the walk from the roots (and excluded roots) never reached")));
    }

    // --- unused entries ----------------------------------------------------------
    if let Some(a) = al {
        for (i, e) in a.entries.iter().enumerate() {
            let k = e.kind.as_str();
            if (k == "alloc" || allowlist::LEAF_KINDS.contains(&k)) && !used.contains(&i) && e.item.trim() != "*" {
                f.push(Finding::new("allowlist-unused-entry", format!("entry #{} (kind {k}, item {:?}, allow {:?}) admits nothing that this compilation reaches", i + 1, e.item, e.allow)));
            }
        }
    }

    // --- report ------------------------------------------------------------------
    rep.h("mode", if cfg.prototype_mode { "PROTOTYPE (--prototype; fixture reviewer accepted; cannot back a proof report)" } else { "PRODUCTION" });
    if al.is_none() {
        rep.h("diagnostic", "no allowlist");
    }
    rep.h("crate", cfg.crate_name.clone());
    rep.h("crate-svh", local_svh);
    rep.h("invocation", cfg.invocation);
    rep.h("rustc", inputs::rustc_version_line());
    rep.h("rustc-commit", format!("{} (full {})", commit_short, commit_full.clone().unwrap_or_default()));
    rep.h("target", target.clone());
    rep.h("profile", profile.clone());
    rep.h("profile-class", class);
    rep.h("features", format!("{features:?}"));
    rep.h("cfgs", format!("{other_cfgs:?}"));
    rep.h("source_hash", src.combined.clone().unwrap_or_default());
    rep.h("source-hash-scope", "sha256 over sorted per-file sha256 of every non-imported source-map file outside the sysroot (module files and loaded include targets); dependency crates are bound by crate hash, vstd additionally by its locked source checksum");
    match al {
        Some(a) => rep.h(
            "allowlist",
            format!("{} sha256={} entries={} reviewer={:?} drafted_by={:?}", a.path.display(), a.sha256, a.entries.len(), a.meta.reviewer.clone().unwrap_or_default(), a.meta.drafted_by.clone().unwrap_or_default()),
        ),
        None => rep.h("allowlist", "<none>"),
    }
    rep.h("vstd", format!(
        "linked={} version={:?} svh={:?} locked_checksum={:?} linked_artifact_sha256={:?} source_dir={:?}",
        vstd.linked, vstd.version, vstd.svh, vstd.source_checksum, vstd.rlib_sha256, vstd.source_dir
    ));
    rep.h("vstd-binding-problems", if vstd.problems.is_empty() { "none".to_string() } else { vstd.problems.join(" | ") });
    rep.h("roots", roots.iter().map(|d| names::canon(tcx, *d)).collect::<Vec<_>>().join(", "));
    let mut ex_sorted: Vec<String> = excluded.iter().map(|d| names::canon(tcx, *d)).collect();
    ex_sorted.sort();
    rep.h("roots-excluded", ex_sorted.join(", "));
    rep.h("generic-exports", generic_exports.iter().map(|d| names::canon(tcx, *d)).collect::<Vec<_>>().join(", "));
    rep.h("instances", g.nodes.len().to_string());
    rep.h("edges", g.edges.len().to_string());
    rep.h("crates-reached", per_crate.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    rep.h("allocator-sinks", format!("{}", sinks.len()));
    rep.h("mono-items", format!("{mono_count} in local codegen units; {} not reached by the walk", mono_unwalked.len()));
    let mut srcl: Vec<String> = src.files.iter().map(|(p, h, m)| format!("{h}  {p}{}", if *m { "" } else { "  (non-module: include target)" })).collect();
    srcl.insert(0, format!("combined {}", src.combined.clone().unwrap_or_default()));
    rep.section("source files", srcl);
    rep.section("build inputs (compiler-resolved)", bi_lines);
    let mut inv = Vec::new();
    inv.push(format!("rustc command line: {}", cfg.rustc_args.join(" ")));
    for (k, v) in inputs::env_snapshot() {
        inv.push(format!("env {k}={v}"));
    }
    if let Some((p, h)) = inputs::lock_summary() {
        inv.push(format!("Cargo.lock {p} sha256={h}"));
    }
    for c in &crates {
        for (p, h) in &c.artifacts {
            inv.push(format!("crate {} svh={} kind={} sysroot={} artifact {} sha256={}", c.name, c.svh, c.dep_kind, c.from_sysroot, p, h.clone().unwrap_or_default()));
        }
    }
    rep.section("linked artifacts and command line", inv);
    rep.section("allocator sinks (every root-to-sink path judged)", sink_lines);
    rep.section("leaves", leaf_lines);
    rep.section("panic edges (per function)", panic_lines);
    rep.section("guard call sites and generic exports", guard_lines);
    rep.section("dependency contracts", dep_lines);
    for (a, b, k, s) in &g.edges {
        rep.edges.push(format!("{} -> {} [{k}] @ {}", g.nodes[*a].label, g.nodes[*b].label, g.span_str(*s)));
    }
    rep.findings = f;
    rep
}

fn make_resolver(tcx: TyCtxt<'_>, universe: &HashSet<DefId>) -> Resolver {
    let mut by_canon: HashMap<String, HashSet<DefId>> = HashMap::new();
    for &d in universe {
        by_canon.entry(names::canon(tcx, d)).or_default().insert(d);
    }
    Resolver { by_canon }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    cur: usize,
    target: usize,
    adj: &[BTreeSet<usize>],
    can: &[bool],
    stack: &mut Vec<usize>,
    on: &mut [bool],
    out: &mut Vec<Vec<usize>>,
    budget: &mut usize,
    truncated: &mut bool,
) {
    if *budget == 0 {
        *truncated = true;
        return;
    }
    if cur == target {
        out.push(stack.clone());
        *budget -= 1;
        return;
    }
    if stack.len() >= MAX_DEPTH {
        *truncated = true;
        return;
    }
    for &n in &adj[cur] {
        if on[n] || !can[n] {
            continue;
        }
        on[n] = true;
        stack.push(n);
        dfs(n, target, adj, can, stack, on, out, budget, truncated);
        stack.pop();
        on[n] = false;
        if *budget == 0 {
            *truncated = true;
            return;
        }
    }
}

/// Judge one root-to-sink path: the last node on it that an `alloc` entry
/// names (by DefId, and generic arguments if the entry writes them) is the
/// allocation-capable item; the nearest node at or before it that is defined
/// in the audited crate is the caller, which must be the entry's `allow`
/// (DefId equality). `allow = "*"` admits deallocation sinks only.
fn judge(g: &Graph<'_>, path: &[usize], sk: SinkKind, rules: &[AllocRule], used: &mut BTreeSet<usize>) -> (bool, String) {
    let body = &path[..path.len().saturating_sub(1)];
    for i in (0..body.len()).rev() {
        let n = &g.nodes[body[i]];
        let did = n.inst.def.def_id();
        let matching: Vec<&AllocRule> = rules
            .iter()
            .filter(|r| r.item == did && r.args.as_ref().is_none_or(|a| *a == names::args_str(g.tcx, n.inst)))
            .collect();
        if matching.is_empty() {
            continue;
        }
        let caller = (0..=i).rev().find(|&j| g.is_local(body[j])).map(|j| body[j]);
        for r in &matching {
            let ok = match &r.allow {
                Resolved::Star => sk == SinkKind::Dealloc,
                Resolved::Did(a, _) => caller.is_some_and(|c| g.nodes[c].inst.def.def_id() == *a),
                Resolved::Bad => false,
            };
            if ok {
                used.insert(r.idx);
                return (true, format!("item {} admitted for caller {}", n.label, caller.map(|c| g.nodes[c].label.clone()).unwrap_or_else(|| "* (deallocation)".into())));
            }
        }
        return (
            false,
            format!(
                "item {} is allowlisted, but not for crate caller {}",
                n.label,
                caller.map(|c| g.nodes[c].label.clone()).unwrap_or_else(|| "<none>".into())
            ),
        );
    }
    (false, "no allocation-capable item on this path is allowlisted".into())
}

fn render(g: &Graph<'_>, p: &[usize]) -> String {
    let parts: Vec<&str> = p.iter().map(|&i| g.nodes[i].label.as_str()).collect();
    if parts.len() > 16 {
        format!("{} -> ... -> {}", parts[..7].join(" -> "), parts[parts.len() - 7..].join(" -> "))
    } else {
        parts.join(" -> ")
    }
}
