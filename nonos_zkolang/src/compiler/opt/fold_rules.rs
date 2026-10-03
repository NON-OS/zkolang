/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The folding rules, each keeping `ssa::eval`'s semantics. */

use nonos_stark::field::Fp;

use crate::compiler::ssa::{Inst, V};

/** What an instruction folds to. */
pub(super) enum Folded {
    Keep,
    /** An earlier value. */
    Value(V),
    Const(u64),
    /** Nothing: a constraint that holds. */
    Drop,
}

/** How `inst` folds, `def` giving the instruction that defines each value so far. */
pub(super) fn rule(inst: Inst, def: &dyn Fn(V) -> Option<Inst>) -> Folded {
    let k = |v: V| match def(v) {
        Some(Inst::Const(c)) => Some(Fp::from_u64(c)),
        _ => None,
    };
    let fp = |x: Fp| Folded::Const(x.value());
    let (zero, one) = (Some(Fp::ZERO), Some(Fp::ONE));
    match inst {
        Inst::Add(a, b) => match (k(a), k(b)) {
            (Some(x), Some(y)) => fp(x + y),
            (_, y) if y == zero => Folded::Value(a),
            (x, _) if x == zero => Folded::Value(b),
            _ => Folded::Keep,
        },
        Inst::Sub(a, b) => match (k(a), k(b)) {
            (Some(x), Some(y)) => fp(x - y),
            (_, y) if y == zero => Folded::Value(a),
            _ if a == b => Folded::Const(0),
            _ => Folded::Keep,
        },
        Inst::Mul(a, b) => match (k(a), k(b)) {
            (Some(x), Some(y)) => fp(x * y),
            (x, y) if x == zero || y == zero => Folded::Const(0),
            (_, y) if y == one => Folded::Value(a),
            (x, _) if x == one => Folded::Value(b),
            _ => Folded::Keep,
        },
        Inst::Inv(a) => match k(a) {
            Some(x) if x != Fp::ZERO => fp(x.inv()),
            _ => Folded::Keep,
        },
        Inst::Sel(c, a, b) => match k(c) {
            x if x == one => Folded::Value(a),
            x if x == zero => Folded::Value(b),
            None if a == b
                && matches!(
                    def(c),
                    Some(Inst::Eq(..) | Inst::Bit(..) | Inst::FieldBit(..))
                ) =>
            {
                Folded::Value(a)
            }
            _ => Folded::Keep,
        },
        Inst::Eq(a, b) => match (k(a), k(b)) {
            (Some(x), Some(y)) => Folded::Const(u64::from(x == y)),
            _ if a == b => Folded::Const(1),
            _ => Folded::Keep,
        },
        _ => super::fold_checks::rule(inst, &k),
    }
}
