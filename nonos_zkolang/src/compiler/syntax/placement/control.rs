/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The placement check inside blocks, branches and loops, whose bodies hold statements. */

use super::check::{check_block, check_const, check_expr};
use super::expr::check_kind;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::syntax::ast::{AssignOp, Expr, ExprKind, ForIter};

/** Check a block, `if`, `match`, `for` or `while`. */
pub(super) fn check_control(k: &ExprKind, diags: &mut Diagnostics) {
    let mut sub = |e: &Expr| check_expr(e, false, diags);
    match k {
        ExprKind::Block(b) => check_block(b, diags),
        ExprKind::If(branches, else_block) => {
            for b in branches {
                check_cond(&b.cond, diags);
                check_block(&b.block, diags);
            }
            else_block.iter().for_each(|b| check_block(b, diags));
        }
        ExprKind::Match { scrutinee, arms } => {
            sub(scrutinee);
            for arm in arms {
                arm.guard.iter().for_each(|g| check_expr(g, false, diags));
                check_expr(&arm.body, true, diags);
            }
        }
        ExprKind::For { iter, body, .. } => {
            match iter {
                ForIter::Range { lo, hi, .. } => {
                    sub(lo);
                    sub(hi);
                }
                ForIter::Array(x) | ForIter::Enumerate(x) => sub(x),
            }
            check_block(body, diags);
        }
        ExprKind::While { cond, limit, body } => {
            check_cond(cond, diags);
            check_const(limit, diags);
            check_block(body, diags);
        }
        _ => {}
    }
}

/** Check the condition of an `if` or `while`, where `x = y` is most likely `x == y`. */
fn check_cond(e: &Expr, diags: &mut Diagnostics) {
    if !matches!(
        e.kind,
        ExprKind::Assign {
            op: AssignOp::Assign,
            ..
        }
    ) {
        return check_expr(e, false, diags);
    }
    let d = Diagnostic::error(
        Code::MISPLACED_STATEMENT,
        "an assignment is not a condition",
        e.span,
        "this assigns",
    )
    .with_help("compare with `==`");
    diags.push(d);
    check_kind(&e.kind, diags);
}
