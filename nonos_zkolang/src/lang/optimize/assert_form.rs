/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Normalise an assertion without changing which relation it states. */

use super::env::Env;
use super::subst::{bx, norm};
use crate::lang::parse::{Expr, Stmt};

/*
 * An assertion's own meaning depends on its top node: `assert a == b` and `assert a != b`
 * state the relation, any other expression is required to be zero. Folding the relation to
 * its bit would turn it into a claim that the bit is zero, the opposite statement, so the
 * top relation is kept and only its sides are normalised. Between two constants the
 * relation is decided here: a true one needs no row, a false one keeps an assertion that
 * can never hold.
 */
pub(super) fn norm_assert(e: &Expr, env: &Env) -> Option<Stmt> {
    let (l, r, equal) = match e {
        Expr::Eq(l, r) => (l, r, true),
        Expr::Ne(l, r) => (l, r, false),
        _ => return Some(Stmt::Assert(norm(e, env))),
    };
    let (l, r) = (norm(l, env), norm(r, env));
    if let (Expr::Num(a), Expr::Num(b)) = (&l, &r) {
        let same = nonos_stark::field::Fp::from_u64(*a) == nonos_stark::field::Fp::from_u64(*b);
        return if same == equal {
            None
        } else {
            Some(Stmt::Assert(Expr::Num(1)))
        };
    }
    let (l, r) = (bx(l), bx(r));
    Some(Stmt::Assert(if equal {
        Expr::Eq(l, r)
    } else {
        Expr::Ne(l, r)
    }))
}
