/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Upper bounds on values: for each value, a number its canonical representative never
 * exceeds in an accepted run. An operation's bound follows from its operands' when no
 * reduction modulo p can happen; otherwise it is `p - 1`, which says nothing.
 */

use nonos_stark::field::P;

use crate::compiler::ssa::{Inst, V};

/** The bound that says nothing. */
pub(super) const ANY: u128 = P as u128 - 1;

/**
 * The bound of the value `inst` defines, from the bounds `b` of its operands and `k`,
 * which gives an operand's value when it is a constant.
 */
pub(super) fn def_bound(inst: Inst, b: &dyn Fn(V) -> u128, k: &dyn Fn(V) -> Option<u64>) -> u128 {
    let fits = |x: Option<u128>| x.filter(|x| *x <= ANY).unwrap_or(ANY);
    let pow = |n: u8| (1u128 << n.min(64)) - 1;
    match inst {
        Inst::Const(c) => u128::from(c),
        Inst::Add(x, y) => fits(b(x).checked_add(b(y))),
        Inst::Mul(x, y) => fits(b(x).checked_mul(b(y))),
        /* `c - y` with `y <= c` lies in `[0, c]`. */
        Inst::Sub(x, y) => match k(x).map(u128::from) {
            Some(c) if b(y) <= c => c,
            _ => ANY,
        },
        Inst::Sel(_, x, y) => b(x).max(b(y)),
        Inst::Eq(..) | Inst::Bit(..) | Inst::FieldBit(..) => 1,
        Inst::Quot(x, _, n) => b(x).min(pow(n)),
        Inst::Rem(_, y, n) => b(y).saturating_sub(1).min(pow(n)),
        _ => ANY,
    }
}
