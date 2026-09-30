/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Which operators are defined on which types (sections 7.1 to 7.5). An operand whose type
 * is a literal variable may still become an integer, so an integer-only operator on it is
 * left for the end of the body.
 */

use super::cx::FnCx;
use super::deferred::Deferred;
use super::op_sym::sym;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::BinOp;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Check that `op` is defined on operands of type `t`, at `at`. */
    pub(crate) fn op_on(&mut self, op: BinOp, t: TyId, at: Span) {
        let (field, bool_ok) = match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => (true, false),
            BinOp::BitAnd
            | BinOp::BitOr
            | BinOp::BitXor
            | BinOp::Lt
            | BinOp::Le
            | BinOp::Gt
            | BinOp::Ge => (false, true),
            BinOp::Rem | BinOp::Shl | BinOp::Shr => (false, false),
            BinOp::Eq | BinOp::Ne | BinOp::And | BinOp::Or => return,
        };
        let t = self.numeric(t);
        let ok = match self.kind(t) {
            TyKind::Error | TyKind::Never | TyKind::Int(_) => true,
            TyKind::Field => field,
            TyKind::Bool => bool_ok,
            TyKind::Var(_) if !field => {
                self.deferred.push(Deferred::IntOnly {
                    ty: t,
                    span: at,
                    op: sym(op),
                });
                true
            }
            TyKind::Var(_) => true,
            _ => false,
        };
        if !ok {
            self.no_operator(sym(op), t, at);
        }
    }

    /** Report that the operator written `op` is not defined on type `t` (E0301). */
    pub(crate) fn no_operator(&mut self, op: &str, t: TyId, at: Span) {
        let shown = self.show(t);
        let d = Diagnostic::error(
            Code::NO_OPERATOR,
            alloc::format!("`{op}` is not defined on `{shown}`"),
            at,
            alloc::format!("`{op}` on `{shown}`"),
        );
        self.sema.diags.push(d);
    }
}
