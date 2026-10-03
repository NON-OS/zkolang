/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Replace every visible occurrence of one subtree with the name it was bound to. */

use alloc::string::String;
use alloc::vec;

use super::ids::Ids;
use crate::lang::parse::Expr;

/** A rewrite of the subtrees with id `target` to `name`. */
pub(super) struct Rewrite<'a> {
    pub(super) ids: &'a mut Ids,
    pub(super) target: usize,
    pub(super) name: String,
}

impl Rewrite<'_> {
    /**
     * The rewritten `e` and the id `e` had. Children are rewritten first; a node equal to
     * the target has no descendant equal to it, so matching after them replaces the same
     * nodes a top-down walk would. An index or a block is left as it is.
     */
    pub(super) fn go(&mut self, e: &Expr) -> (Expr, usize) {
        let (out, kids) = match e {
            Expr::Num(_) | Expr::Var(_) | Expr::Index(..) | Expr::Block(..) => (e.clone(), vec![]),
            Expr::Add(a, b) => self.two(a, b, Expr::Add),
            Expr::Sub(a, b) => self.two(a, b, Expr::Sub),
            Expr::Mul(a, b) => self.two(a, b, Expr::Mul),
            Expr::Div(a, b) => self.two(a, b, Expr::Div),
            Expr::Eq(a, b) => self.two(a, b, Expr::Eq),
            Expr::Ne(a, b) => self.two(a, b, Expr::Ne),
            Expr::Lt(a, b) => self.two(a, b, Expr::Lt),
            Expr::Neg(x) => self.one(x, Expr::Neg),
            Expr::Inv(x) => self.one(x, Expr::Inv),
            Expr::Sel(c, a, b) => self.three(c, a, b, Expr::Sel),
            Expr::If(c, a, b) => self.three(c, a, b, Expr::If),
            Expr::Call(f, xs) => self.many(xs, |xs| Expr::Call(f.clone(), xs)),
            Expr::Array(xs) => self.many(xs, Expr::Array),
            Expr::Tuple(xs) => self.many(xs, Expr::Tuple),
        };
        let id = self.ids.node(e, kids);
        let out = if id == self.target {
            Expr::Var(self.name.clone())
        } else {
            out
        };
        (out, id)
    }
}
