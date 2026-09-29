/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The node budget. The copying desugarings, `||` and `match`, copy their operands, so a
 * chain of them doubles the tree per operator; each result is measured against a budget.
 */

use super::Parser;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;

/** The most nodes a copying desugaring may produce. */
pub(crate) const MAX_EXPR_NODES: usize = 1 << 16;

impl<'a> Parser<'a> {
    /** Refuse an expression past the node budget, pointing at `at`. */
    pub(crate) fn within_budget(&self, e: &Expr, at: usize) -> Result<(), CompileError> {
        if nodes_up_to(e, MAX_EXPR_NODES + 1) > MAX_EXPR_NODES {
            return Err(CompileError::ExpressionTooLarge { at });
        }
        Ok(())
    }
}

/**
 * The node count of an expression, counted no further than `limit`, so measuring a huge
 * tree costs no more than the limit.
 */
fn nodes_up_to(e: &Expr, limit: usize) -> usize {
    let mut stack = alloc::vec![e];
    let mut n = 0usize;
    while let Some(e) = stack.pop() {
        n += 1;
        if n >= limit {
            return n;
        }
        match e {
            Expr::Num(_) | Expr::Var(_) => {}
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Eq(a, b)
            | Expr::Ne(a, b)
            | Expr::Lt(a, b)
            | Expr::Index(a, b, _) => {
                stack.push(a);
                stack.push(b);
            }
            Expr::Neg(a) | Expr::Inv(a) => stack.push(a),
            Expr::Sel(a, b, c) | Expr::If(a, b, c) => {
                stack.push(a);
                stack.push(b);
                stack.push(c);
            }
            Expr::Call(_, xs) | Expr::Array(xs) | Expr::Tuple(xs) => stack.extend(xs.iter()),
            Expr::Block(locals, r) => {
                stack.extend(locals.iter().map(|(_, v)| v));
                stack.push(r);
            }
        }
    }
    n
}
