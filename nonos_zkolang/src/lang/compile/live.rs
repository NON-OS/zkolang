/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Liveness of top-level bindings. A name is single-assignment in the source, and
//! its register can go back to the pool once no later statement reads it. This walk
//! collects every name a statement reads, so the lowering can free a binding after
//! its last use instead of holding it to the end of the program. It matches the AST
//! exhaustively on purpose: a missed reference would let a live binding be freed, so
//! the compiler must account for every place a name can appear.
//!
//! The collection is deliberately blind to scope. A name written by a block-local
//! `let` still counts as read wherever it textually appears, which can only keep an
//! outer binding of the same name alive longer than strictly needed, never free one
//! early. Erring that way keeps the analysis sound with no aliasing reasoning.

use alloc::string::String;
use alloc::vec::Vec;

use crate::lang::parse::{Expr, Stmt};

/// Push every name read by an expression onto `out`.
pub(crate) fn reads_of_expr(e: &Expr, out: &mut Vec<String>) {
    match e {
        Expr::Num(_) => {}
        Expr::Var(name) => out.push(name.clone()),
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Eq(a, b)
        | Expr::Ne(a, b)
        | Expr::Lt(a, b) => {
            reads_of_expr(a, out);
            reads_of_expr(b, out);
        }
        Expr::Neg(a) | Expr::Inv(a) => reads_of_expr(a, out),
        Expr::Sel(a, b, c) | Expr::If(a, b, c) => {
            reads_of_expr(a, out);
            reads_of_expr(b, out);
            reads_of_expr(c, out);
        }
        // A call names a function, not a variable, so only its arguments read names.
        Expr::Call(_, args) => {
            for a in args {
                reads_of_expr(a, out);
            }
        }
        Expr::Index(base, index, _) => {
            reads_of_expr(base, out);
            reads_of_expr(index, out);
        }
        Expr::Array(items) | Expr::Tuple(items) => {
            for it in items {
                reads_of_expr(it, out);
            }
        }
        Expr::Block(bindings, result) => {
            for (_, value) in bindings {
                reads_of_expr(value, out);
            }
            reads_of_expr(result, out);
        }
    }
}

/// Push every name read by a statement, descending into a loop body. A loop counts
/// each name its body reads as read at the loop, so a binding used only inside stays
/// live across the whole unrolling.
pub(crate) fn reads_of_stmt(s: &Stmt, out: &mut Vec<String>) {
    match s {
        Stmt::Let(_, e) | Stmt::LetTuple(_, e) | Stmt::Assert(e) | Stmt::Output(e) => {
            reads_of_expr(e, out)
        }
        Stmt::Input(_) | Stmt::Secret(_) => {}
        Stmt::For { body, .. } => {
            for s in body {
                reads_of_stmt(s, out);
            }
        }
    }
}
