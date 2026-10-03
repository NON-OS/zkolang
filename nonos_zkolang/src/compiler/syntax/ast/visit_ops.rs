/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Visiting the operands of operators, calls, indexing and assignment. */

use super::{Expr, ExprKind};

impl Expr {
    /** Call `f` on every expression among this expression's operands. */
    pub(super) fn each_operand(&self, f: &mut dyn FnMut(&Expr)) {
        match &self.kind {
            ExprKind::Binary(a, links) => {
                a.each_expr(f);
                links.iter().for_each(|(_, x)| x.each_expr(f));
            }
            ExprKind::Call(callee, args) => {
                callee.each_expr(f);
                args.iter().for_each(|x| x.each_expr(f));
            }
            ExprKind::MethodCall { receiver, args, .. } => {
                receiver.each_expr(f);
                args.iter().for_each(|x| x.each_expr(f));
            }
            ExprKind::Index(a, i) => {
                a.each_expr(f);
                i.each_expr(f);
            }
            ExprKind::Assign { place, value, .. } => {
                place.each_expr(f);
                value.each_expr(f);
            }
            _ => {}
        }
    }
}
