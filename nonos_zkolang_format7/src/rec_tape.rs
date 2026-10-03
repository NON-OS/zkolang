/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A tape begun, marked, and taken. */

use nonos_stark::field::{Fp, Fp2};

use super::rec::{constant, Tape, TAPE};

/** Note that an inversion was asked for, which no program image can carry. */
pub(crate) fn inverted() {
    TAPE.with(|t| t.borrow_mut().inverted = true);
}

/** Start a fresh tape, zero and one at the handles `Rec::ZERO` and `Rec::ONE` name. */
pub(crate) fn fresh() {
    TAPE.with(|t| *t.borrow_mut() = Tape::default());
    constant(Fp2::from_base(Fp::ZERO));
    constant(Fp2::from_base(Fp::ONE));
}

/** The tape recorded since `fresh`, leaving an empty one. */
pub(crate) fn take() -> Tape {
    TAPE.with(|t| std::mem::take(&mut *t.borrow_mut()))
}
