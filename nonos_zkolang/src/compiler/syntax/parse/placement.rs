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

use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::syntax::ast::{Block, ConstArg, Expr, Item, ItemKind, StmtKind};

/** Report every statement form in a file that stands inside a larger expression. */
pub(super) fn check_items(items: &[Item], diags: &mut Diagnostics) {
    for item in items {
        match &item.kind {
            ItemKind::Fn(f) => check_block(&f.body, diags),
            ItemKind::Const(c) => check_expr(&c.value, false, diags),
            ItemKind::Mod(m) => check_items(m.body.as_deref().unwrap_or(&[]), diags),
            ItemKind::Impl(i) => check_items(&i.items, diags),
            _ => {}
        }
    }
}

pub(super) fn check_block(b: &Block, diags: &mut Diagnostics) {
    for s in &b.stmts {
        match &s.kind {
            StmtKind::Let { init, .. } => check_expr(init, false, diags),
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
    super::placement_expr::check_kind(&e.kind, diags);
}
