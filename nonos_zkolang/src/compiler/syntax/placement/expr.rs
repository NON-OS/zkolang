/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The placement check inside each expression form. */

use super::check::{check_const, check_expr};
use super::control::check_control;
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::ExprKind;

impl ExprKind {
    /** Whether this form may stand only as a statement: return, break, continue, assignment. */
    pub(super) fn is_statement_form(&self) -> bool {
        matches!(
            self,
            ExprKind::Return(_) | ExprKind::Break | ExprKind::Continue | ExprKind::Assign { .. }
        )
    }
}

/** Check the expressions an expression holds; none of them stands as a statement. */
pub(super) fn check_kind(k: &ExprKind, diags: &mut Diagnostics) {
    let mut sub = |e: &crate::compiler::syntax::ast::Expr| check_expr(e, false, diags);
    match k {
        ExprKind::Struct { fields, .. } => {
            fields.iter().filter_map(|f| f.value.as_ref()).for_each(sub)
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
        ExprKind::Cast(x, _) => sub(x),
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
        ExprKind::MethodCall { receiver, args, .. } => {
            sub(receiver);
            args.iter().for_each(sub);
        }
        ExprKind::Block(_)
        | ExprKind::If { .. }
        | ExprKind::Match { .. }
        | ExprKind::For { .. }
        | ExprKind::While { .. } => check_control(k, diags),
        ExprKind::Lit(_) | ExprKind::Path(_) | ExprKind::Unit => {}
        ExprKind::Break | ExprKind::Continue | ExprKind::Error => {}
    }
}
