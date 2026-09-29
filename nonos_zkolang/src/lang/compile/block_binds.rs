/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Every name a block inside an expression binds, however deep the block sits. */

use crate::lang::parse::Expr;

/** Call `f` with each name bound by a block local anywhere inside `e`. */
pub(super) fn block_binds<E>(e: &Expr, f: &mut impl FnMut(&str) -> Result<(), E>) -> Result<(), E> {
    match e {
        Expr::Num(_) | Expr::Var(_) => Ok(()),
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Eq(a, b)
        | Expr::Ne(a, b)
        | Expr::Lt(a, b)
        | Expr::Index(a, b, _) => {
            block_binds(a, f)?;
            block_binds(b, f)
        }
        Expr::Neg(a) | Expr::Inv(a) => block_binds(a, f),
        Expr::Sel(a, b, c) | Expr::If(a, b, c) => {
            block_binds(a, f)?;
            block_binds(b, f)?;
            block_binds(c, f)
        }
        Expr::Call(_, xs) | Expr::Array(xs) | Expr::Tuple(xs) => {
            xs.iter().try_for_each(|x| block_binds(x, f))
        }
        Expr::Block(locals, r) => {
            for (names, value) in locals {
                block_binds(value, f)?;
                names.iter().filter(|n| *n != "_").try_for_each(|n| f(n))?;
            }
            block_binds(r, f)
        }
    }
}
