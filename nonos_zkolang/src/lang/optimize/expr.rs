/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Fold an expression. Sub-expressions over constants collapse to one constant, computed
 * in the field so the fold agrees with the run, and the algebraic identities that add
 * nothing to a trace are removed: adding or subtracting zero, multiplying by one,
 * multiplying by zero, and selecting on a constant condition. An identity that would
 * discard an operand applies only when that operand carries no constraint. Ordered
 * comparison is left alone, because its bit decomposition is not a constant even when its
 * operands are.
 */

use alloc::boxed::Box;

use crate::lang::parse::Expr;
use nonos_stark::field::Fp;

fn num(f: Fp) -> Expr {
    Expr::Num(f.value())
}

fn as_num(e: &Expr) -> Option<Fp> {
    if let Expr::Num(v) = e {
        Some(Fp::from_u64(*v))
    } else {
        None
    }
}

/*
 * Whether evaluating an expression carries no constraint, so dropping it changes nothing a
 * program proves. Plain arithmetic, equality, names and literals qualify; an inverse, a
 * division, an ordered comparison, a select (its condition must be a bit), an index, a block
 * and a call (either may hold any of those) do not.
 */
fn constraint_free(e: &Expr) -> bool {
    match e {
        Expr::Num(_) | Expr::Var(_) => true,
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Eq(a, b) | Expr::Ne(a, b) => {
            constraint_free(a) && constraint_free(b)
        }
        Expr::Neg(x) => constraint_free(x),
        _ => false,
    }
}

/*
 * A select on a constant condition returns the arm it takes, but only when the other arm
 * carries no constraint: both arms are evaluated, so the arm not taken still constrains.
 */
fn select(c: Expr, a: Expr, b: Expr, ctor: fn(Box<Expr>, Box<Expr>, Box<Expr>) -> Expr) -> Expr {
    match as_num(&c) {
        Some(v) if v == Fp::ZERO && constraint_free(&a) => b,
        Some(v) if v == Fp::ONE && constraint_free(&b) => a,
        _ => ctor(Box::new(c), Box::new(a), Box::new(b)),
    }
}

pub(super) fn fold(e: &Expr) -> Expr {
    match e {
        Expr::Num(v) => Expr::Num(*v),
        Expr::Var(n) => Expr::Var(n.clone()),
        Expr::Add(a, b) => {
            let (a, b) = (fold(a), fold(b));
            match (as_num(&a), as_num(&b)) {
                (Some(x), Some(y)) => num(x + y),
                (_, Some(y)) if y == Fp::ZERO => a,
                (Some(x), _) if x == Fp::ZERO => b,
                _ => Expr::Add(Box::new(a), Box::new(b)),
            }
        }
        Expr::Sub(a, b) => {
            let (a, b) = (fold(a), fold(b));
            match (as_num(&a), as_num(&b)) {
                (Some(x), Some(y)) => num(x - y),
                (_, Some(y)) if y == Fp::ZERO => a,
                _ => Expr::Sub(Box::new(a), Box::new(b)),
            }
        }
        Expr::Mul(a, b) => {
            let (a, b) = (fold(a), fold(b));
            match (as_num(&a), as_num(&b)) {
                (Some(x), Some(y)) => num(x * y),
                (_, Some(y)) if y == Fp::ZERO && constraint_free(&a) => num(Fp::ZERO),
                (Some(x), _) if x == Fp::ZERO && constraint_free(&b) => num(Fp::ZERO),
                (_, Some(y)) if y == Fp::ONE => a,
                (Some(x), _) if x == Fp::ONE => b,
                _ => Expr::Mul(Box::new(a), Box::new(b)),
            }
        }
        Expr::Div(a, b) => {
            let (a, b) = (fold(a), fold(b));
            match (as_num(&a), as_num(&b)) {
                (Some(x), Some(y)) if y != Fp::ZERO => num(x * y.inv()),
                (_, Some(y)) if y == Fp::ONE => a,
                _ => Expr::Div(Box::new(a), Box::new(b)),
            }
        }
        Expr::Neg(x) => {
            let x = fold(x);
            match as_num(&x) {
                Some(v) => num(Fp::ZERO - v),
                None => Expr::Neg(Box::new(x)),
            }
        }
        Expr::Eq(a, b) => {
            let (a, b) = (fold(a), fold(b));
            match (as_num(&a), as_num(&b)) {
                (Some(x), Some(y)) => num(if x == y { Fp::ONE } else { Fp::ZERO }),
                _ => Expr::Eq(Box::new(a), Box::new(b)),
            }
        }
        Expr::Ne(a, b) => {
            let (a, b) = (fold(a), fold(b));
            match (as_num(&a), as_num(&b)) {
                (Some(x), Some(y)) => num(if x != y { Fp::ONE } else { Fp::ZERO }),
                _ => Expr::Ne(Box::new(a), Box::new(b)),
            }
        }
        Expr::Lt(a, b) => Expr::Lt(Box::new(fold(a)), Box::new(fold(b))),
        Expr::Inv(x) => {
            let x = fold(x);
            match as_num(&x) {
                Some(v) if v != Fp::ZERO => num(v.inv()),
                _ => Expr::Inv(Box::new(x)),
            }
        }
        Expr::Sel(c, a, b) => select(fold(c), fold(a), fold(b), Expr::Sel),
        Expr::If(c, a, b) => select(fold(c), fold(a), fold(b), Expr::If),
        Expr::Call(n, args) => Expr::Call(n.clone(), args.iter().map(fold).collect()),
        Expr::Index(base, idx, at) => Expr::Index(Box::new(fold(base)), Box::new(fold(idx)), *at),
        Expr::Array(xs) => Expr::Array(xs.iter().map(fold).collect()),
        // Fold inside the bindings and the result; the scope is untouched, since folding
        // only collapses constant arithmetic and never moves a name across the braces.
        Expr::Block(locals, r) => Expr::Block(
            locals.iter().map(|(n, e)| (n.clone(), fold(e))).collect(),
            Box::new(fold(r)),
        ),
        Expr::Tuple(xs) => Expr::Tuple(xs.iter().map(fold).collect()),
    }
}
