/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Which subtrees may be shared, and which are visible to sharing at all. */

use alloc::vec;
use alloc::vec::Vec;

use crate::lang::parse::Expr;

fn is_pure(e: &Expr) -> bool {
    match e {
        Expr::Num(_) | Expr::Var(_) => true,
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Eq(a, b) | Expr::Ne(a, b) => {
            is_pure(a) && is_pure(b)
        }
        Expr::Neg(x) => is_pure(x),
        _ => false,
    }
}

/** Whether `e` is pure arithmetic worth a binding of its own. */
pub(super) fn hoistable(e: &Expr) -> bool {
    is_pure(e) && !matches!(e, Expr::Num(_) | Expr::Var(_))
}

/**
 * The subtrees sharing looks into. An index must fold to a compile-time constant, so
 * nothing inside one may be hoisted into a binding, and a block carries its own scope, so
 * its interior is opaque to sharing across it.
 */
pub(super) fn children(e: &Expr) -> Vec<&Expr> {
    match e {
        Expr::Num(_) | Expr::Var(_) | Expr::Index(_, _, _) | Expr::Block(_, _) => Vec::new(),
        Expr::Neg(x) | Expr::Inv(x) => vec![x.as_ref()],
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Eq(a, b)
        | Expr::Ne(a, b)
        | Expr::Lt(a, b) => vec![a.as_ref(), b.as_ref()],
        Expr::Sel(c, a, b) | Expr::If(c, a, b) => vec![c.as_ref(), a.as_ref(), b.as_ref()],
        Expr::Call(_, args) | Expr::Array(args) | Expr::Tuple(args) => args.iter().collect(),
    }
}
