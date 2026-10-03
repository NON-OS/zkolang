/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Where the statement forms may stand. `return`, `break`, `continue` and assignment are
 * allowed as an expression statement, as a block's tail and as a `match` arm's body, and
 * are an error inside any larger expression. The check walks the tree once the file is
 * parsed; its depth is bounded by the parser's nesting budget.
 */

use super::types::check_type;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::syntax::ast::{Block, ConstArg, Expr, ExprKind, StmtKind};

pub(super) fn check_block(b: &Block, diags: &mut Diagnostics) {
    for s in &b.stmts {
        match &s.kind {
            StmtKind::Let { ty, init, .. } => {
                ty.iter().for_each(|t| check_type(t, diags));
                check_expr(init, false, diags);
            }
            StmtKind::Assert { cond, .. } => check_expr(cond, false, diags),
            StmtKind::Expr { expr, .. } => check_expr(expr, true, diags),
            StmtKind::Empty => {}
        }
    }
    if let Some(tail) = &b.tail {
        check_expr(tail, true, diags);
    }
}

pub(super) fn check_const(c: &ConstArg, diags: &mut Diagnostics) {
    if let ConstArg::Expr(e) = c {
        check_expr(e, false, diags);
    }
}

/** Check `e`, which stands where a statement form is allowed exactly when `free` holds. */
pub(super) fn check_expr(e: &Expr, free: bool, diags: &mut Diagnostics) {
    if !free && e.kind.is_statement_form() {
        diags.push(
            Diagnostic::error(
                Code::MISPLACED_STATEMENT,
                "this may only stand as a statement",
                e.span,
                "inside a larger expression",
            )
            .with_help("write it as its own statement, or as the last expression of a block"),
        );
    }
    super::expr::check_kind(&e.kind, diags);
}

impl ExprKind {
    /** Whether this form may stand only as a statement: return, break, continue, assignment. */
    pub(super) fn is_statement_form(&self) -> bool {
        matches!(
            self,
            ExprKind::Return(_) | ExprKind::Break | ExprKind::Continue | ExprKind::Assign { .. }
        )
    }
}
