/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The placement check inside types and paths, which hold expressions in their constant
 * arguments: `[u8; { return 1 }]` is as wrong as `1 + return`.
 */

use super::check::check_const;
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::{GenericArg, GenericParam, Path, Type, TypeKind};

/** Check the constant arguments a type holds. */
pub(super) fn check_type(t: &Type, diags: &mut Diagnostics) {
    match &t.kind {
        TypeKind::Tuple(ts) => ts.iter().for_each(|t| check_type(t, diags)),
        TypeKind::Array(elem, len) => {
            check_type(elem, diags);
            check_const(len, diags);
        }
        TypeKind::Path(p) => check_path(p, diags),
        TypeKind::RefMut(t) | TypeKind::Labelled(_, t) => check_type(t, diags),
        TypeKind::Field
        | TypeKind::Bool
        | TypeKind::Int(_)
        | TypeKind::Unit
        | TypeKind::SelfType
        | TypeKind::Error => {}
    }
}

/** Check the generic arguments of a path. */
pub(super) fn check_path(p: &Path, diags: &mut Diagnostics) {
    for s in &p.segments {
        check_generic_args(s.generics.as_deref().unwrap_or(&[]), diags);
    }
}

/** Check a list of generic arguments. */
pub(super) fn check_generic_args(args: &[GenericArg], diags: &mut Diagnostics) {
    for a in args {
        match a {
            GenericArg::Type(t) => check_type(t, diags),
            GenericArg::Const(c) => check_const(c, diags),
        }
    }
}

/** Check the types of constant generic parameters. */
pub(super) fn check_generic_params(params: &[GenericParam], diags: &mut Diagnostics) {
    for p in params {
        if let GenericParam::Const { ty, .. } = p {
            check_type(ty, diags);
        }
    }
}
