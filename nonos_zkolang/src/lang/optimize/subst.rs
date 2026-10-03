/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Substitute known constants for the names bound to them, then fold. */

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::env::{latest, Env};
use super::expr::fold;
use crate::lang::parse::Expr;

pub(super) fn bx(e: Expr) -> Box<Expr> {
    Box::new(e)
}

/* Replace each variable bound to a known constant with that constant, recursively. */
pub(super) fn subst(e: &Expr, env: &Env) -> Expr {
    match e {
        Expr::Num(v) => Expr::Num(*v),
        Expr::Var(n) => match latest(env, n) {
            Some(v) => Expr::Num(v),
            None => Expr::Var(n.clone()),
        },
        Expr::Add(a, b) => Expr::Add(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Sub(a, b) => Expr::Sub(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Mul(a, b) => Expr::Mul(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Div(a, b) => Expr::Div(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Neg(x) => Expr::Neg(bx(subst(x, env))),
        Expr::Eq(a, b) => Expr::Eq(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Ne(a, b) => Expr::Ne(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Lt(a, b) => Expr::Lt(bx(subst(a, env)), bx(subst(b, env))),
        Expr::Inv(x) => Expr::Inv(bx(subst(x, env))),
        Expr::Sel(c, a, b) => Expr::Sel(bx(subst(c, env)), bx(subst(a, env)), bx(subst(b, env))),
        Expr::If(c, a, b) => Expr::If(bx(subst(c, env)), bx(subst(a, env)), bx(subst(b, env))),
        Expr::Call(n, args) => Expr::Call(n.clone(), args.iter().map(|a| subst(a, env)).collect()),
        Expr::Index(base, idx, at) => Expr::Index(bx(subst(base, env)), bx(subst(idx, env)), *at),
        Expr::Array(xs) => Expr::Array(xs.iter().map(|a| subst(a, env)).collect()),
        Expr::Block(locals, r) => subst_block(locals, r, env),
        Expr::Tuple(xs) => Expr::Tuple(xs.iter().map(|a| subst(a, env)).collect()),
    }
}

pub(super) fn norm(e: &Expr, env: &Env) -> Expr {
    fold(&subst(e, env))
}

/**
 * Substitute through a block. Each local binding shadows the outer constants of its names
 * for the rest of the block, so a name a block rebinds is never replaced after that point.
 * A `_` slot of a destructuring binds nothing, as in lowering.
 */
fn subst_block(locals: &[(Vec<String>, Expr)], r: &Expr, env: &Env) -> Expr {
    let mut inner = env.clone();
    let mut out = Vec::with_capacity(locals.len());
    for (names, value) in locals {
        out.push((names.clone(), subst(value, &inner)));
        let tuple = names.len() > 1;
        let bound = names.iter().filter(|n| !(tuple && *n == "_"));
        inner.extend(bound.map(|n| (n.clone(), None)));
    }
    Expr::Block(out, bx(subst(r, &inner)))
}
