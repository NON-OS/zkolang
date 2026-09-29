/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The functions an expression calls. */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::lang::parse::Expr;

/** Push the index of every function `e` calls, looked up in `index`, onto `out`. */
pub(super) fn callees(e: &Expr, index: &BTreeMap<&str, usize>, out: &mut Vec<usize>) {
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
            callees(a, index, out);
            callees(b, index, out);
        }
        Expr::Neg(a) | Expr::Inv(a) => callees(a, index, out),
        Expr::Sel(a, b, c) | Expr::If(a, b, c) => {
            callees(a, index, out);
            callees(b, index, out);
            callees(c, index, out);
        }
        Expr::Call(name, xs) => {
            out.extend(index.get(name.as_str()));
            xs.iter().for_each(|x| callees(x, index, out));
        }
        Expr::Array(xs) | Expr::Tuple(xs) => xs.iter().for_each(|x| callees(x, index, out)),
        Expr::Block(locals, r) => {
            locals.iter().for_each(|(_, v)| callees(v, index, out));
            callees(r, index, out);
        }
    }
}
