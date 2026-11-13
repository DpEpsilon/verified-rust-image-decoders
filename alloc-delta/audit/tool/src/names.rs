//! Canonical, display-only names for DefIds and Instances, and the parsing of
//! allowlist path values. Nothing in the audit compares these strings to
//! decide identity between graph nodes: nodes are keyed by `Instance`. The
//! strings are used (a) for the report and (b) to resolve an allowlist value
//! to the set of DefIds whose canonical path it spells (`allowlist.rs`).

use rustc_middle::ty::print::{with_crate_prefix, with_no_trimmed_paths, with_no_visible_paths};
use rustc_middle::ty::{GenericArgKind, Instance, InstanceKind, ShimKind, TyCtxt};
use rustc_span::def_id::DefId;

/// Canonical def-path: the defining crate's real module path (no re-export
/// "visible" path, no trimming); local items carry a leading `crate::`.
pub fn canon(tcx: TyCtxt<'_>, did: DefId) -> String {
    with_no_trimmed_paths!(with_no_visible_paths!(with_crate_prefix!(tcx.def_path_str(did))))
}

/// Printed generic arguments of an instance, `<A,B>`, or empty.
pub fn args_str<'tcx>(tcx: TyCtxt<'tcx>, inst: Instance<'tcx>) -> String {
    let _ = tcx;
    let parts: Vec<String> = with_no_trimmed_paths!(with_no_visible_paths!(with_crate_prefix!(
        inst.args
            .iter()
            .filter_map(|a| match a.kind() {
                GenericArgKind::Type(t) => Some(format!("{t}")),
                GenericArgKind::Const(c) => Some(format!("{c}")),
                GenericArgKind::Lifetime(_) => None,
            })
            .collect()
    )));
    if parts.is_empty() {
        String::new()
    } else {
        format!("<{}>", parts.join(","))
    }
}

/// Display label of an instance: canonical path, generic args, and a shim tag
/// for non-`Item` kinds so that e.g. a reify shim and the item it wraps are
/// visibly different nodes (they are different `Instance`s).
pub fn label<'tcx>(tcx: TyCtxt<'tcx>, inst: Instance<'tcx>) -> String {
    let base = format!("{}{}", canon(tcx, inst.def.def_id()), args_str(tcx, inst));
    match inst.def {
        InstanceKind::Item(_) => base,
        InstanceKind::Intrinsic(_) => format!("{base} [intrinsic]"),
        InstanceKind::LlvmIntrinsic(_) => format!("{base} [llvm-intrinsic]"),
        InstanceKind::Virtual(_, n) => format!("{base} [virtual #{n}]"),
        InstanceKind::Shim(s) => {
            let tag = match s {
                ShimKind::VTable(_) => "vtable-shim".to_string(),
                ShimKind::Reify(..) => "reify-shim".to_string(),
                ShimKind::FnPtr(..) => "fn-ptr-shim".to_string(),
                ShimKind::ClosureOnce { .. } => "closure-once-shim".to_string(),
                ShimKind::DropGlue(..) => "drop-glue".to_string(),
                ShimKind::Clone(..) => "clone-shim".to_string(),
                ShimKind::FnPtrAddr(..) => "fn-ptr-addr-shim".to_string(),
                ShimKind::ThreadLocal(..) => "thread-local-shim".to_string(),
                other => format!("{other:?}"),
            };
            format!("{base} [{tag}]")
        }
    }
}

/// An allowlist path value split into its base path and an optional trailing
/// generic-argument group. The trailing group is recognised only when it is
/// one balanced `<...>` running to the end of the string and directly follows
/// an identifier character, so `RawVecInner::<A>::reserve_exact` keeps its
/// inner `::<A>` (path text) and `f<u8>` splits into `f` + `<u8>`.
pub struct PathValue {
    pub base: String,
    pub args: Option<String>,
}

pub fn parse_value(v: &str) -> PathValue {
    let v = v.trim();
    if v.ends_with('>') {
        // Find the '<' that balances the final '>'.
        let bytes: Vec<char> = v.chars().collect();
        let mut depth = 0i32;
        let mut i = bytes.len();
        while i > 0 {
            i -= 1;
            match bytes[i] {
                '>' => depth += 1,
                '<' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
        if depth == 0 && i > 0 {
            let prev = bytes[i - 1];
            if prev.is_alphanumeric() || prev == '_' || prev == ']' || prev == '}' {
                let base: String = bytes[..i].iter().collect();
                let args: String = bytes[i..].iter().collect();
                return PathValue { base, args: Some(args) };
            }
        }
    }
    PathValue { base: v.to_string(), args: None }
}
