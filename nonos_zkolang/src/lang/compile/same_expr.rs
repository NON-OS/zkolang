/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Whether two expressions are the same code, wherever each was written. */

use crate::lang::parse::Expr;

/**
 * Structural equality that ignores source offsets. An index carries the offset it was
 * written at, so two copies of one definition from different places differ only there.
 */
pub(super) fn same(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Add(x, y), Expr::Add(u, v))
        | (Expr::Sub(x, y), Expr::Sub(u, v))
        | (Expr::Mul(x, y), Expr::Mul(u, v))
        | (Expr::Div(x, y), Expr::Div(u, v))
        | (Expr::Eq(x, y), Expr::Eq(u, v))
        | (Expr::Ne(x, y), Expr::Ne(u, v))
        | (Expr::Lt(x, y), Expr::Lt(u, v))
        | (Expr::Index(x, y, _), Expr::Index(u, v, _)) => same(x, u) && same(y, v),
        (Expr::Neg(x), Expr::Neg(u)) | (Expr::Inv(x), Expr::Inv(u)) => same(x, u),
        (Expr::Sel(x, y, z), Expr::Sel(u, v, w)) | (Expr::If(x, y, z), Expr::If(u, v, w)) => {
            same(x, u) && same(y, v) && same(z, w)
        }
        (Expr::Call(f, xs), Expr::Call(g, ys)) => f == g && all(xs, ys),
        (Expr::Array(xs), Expr::Array(ys)) | (Expr::Tuple(xs), Expr::Tuple(ys)) => all(xs, ys),
        (Expr::Block(ls, r), Expr::Block(ms, s)) => {
            ls.len() == ms.len()
                && ls
                    .iter()
                    .zip(ms)
                    .all(|((n, e), (m, f))| n == m && same(e, f))
                && same(r, s)
        }
        _ => a == b,
    }
}

fn all(xs: &[Expr], ys: &[Expr]) -> bool {
    xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| same(x, y))
}
