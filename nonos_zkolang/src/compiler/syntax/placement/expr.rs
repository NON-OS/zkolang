/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The placement check inside each expression form. */

use super::check::{check_const, check_expr};
use super::control::check_control;
use super::types::{check_generic_args, check_path, check_type};
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::ExprKind;

/** Check the expressions an expression holds; none of them stands as a statement. */
pub(super) fn check_kind(k: &ExprKind, diags: &mut Diagnostics) {
    let mut sub = |e: &crate::compiler::syntax::ast::Expr| check_expr(e, false, diags);
    match k {
        ExprKind::Struct { path, fields } => {
            fields.iter().filter_map(|f| f.value.as_ref()).for_each(sub);
            check_path(path, diags);
        }
        ExprKind::Tuple(xs) | ExprKind::Array(xs) => xs.iter().for_each(sub),
        ExprKind::Repeat(x, n) => {
            sub(x);
            check_const(n, diags);
        }
        ExprKind::Paren(x)
        | ExprKind::Declassify(x)
        | ExprKind::Unary(_, x)
        | ExprKind::RefMut(x) => sub(x),
        ExprKind::Cast(x, t) => {
            sub(x);
            check_type(t, diags);
        }
        ExprKind::Field(x, _) | ExprKind::TupleField(x, _, _) => sub(x),
        ExprKind::Return(x) => x.iter().for_each(|x| sub(x)),
        ExprKind::Binary(first, rest) => {
            sub(first);
            rest.iter().for_each(|(_, e)| sub(e));
        }
        ExprKind::Index(a, b) => {
            sub(a);
            sub(b);
        }
        ExprKind::Assign { place, value, .. } => {
            sub(place);
            sub(value);
        }
        ExprKind::Call(f, args) => {
            sub(f);
            args.iter().for_each(sub);
        }
        ExprKind::MethodCall {
            receiver,
            args,
            generics,
            ..
        } => {
            sub(receiver);
            args.iter().for_each(sub);
            check_generic_args(generics.as_deref().unwrap_or(&[]), diags);
        }
        ExprKind::Block(_)
        | ExprKind::If { .. }
        | ExprKind::Match { .. }
        | ExprKind::For { .. }
        | ExprKind::While { .. } => check_control(k, diags),
        ExprKind::Path(p) => check_path(p, diags),
        ExprKind::Lit(_) | ExprKind::Unit => {}
        ExprKind::Break | ExprKind::Continue | ExprKind::Error => {}
    }
}
