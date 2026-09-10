/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The opcode gates: each row's result equals the operation its selector names.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Felt;

use super::Cols;

pub(super) fn opcode_constraints<F: Felt>(c: &Cols<F>) -> Vec<F> {
    let one = F::ONE;
    let diff = c.a - c.b;
    vec![
        // Arithmetic: the result is the field operation on the operands.
        c.s_imm * (c.d - c.imm),
        c.s_add * (c.d - (c.a + c.b)),
        c.s_sub * (c.d - (c.a - c.b)),
        c.s_mul * (c.d - c.a * c.b),
        /*
         * Invert. The prover witnesses aux and the first constraint forces a * aux = 1,
         * which has no solution when a is zero, so an inverse of zero leaves the trace
         * unprovable rather than returning a wrong value. The second copies the witnessed
         * inverse into the result.
         */
        c.s_inv * (c.a * c.aux - one),
        c.s_inv * (c.d - c.aux),
        /*
         * Equality, as a bit with one witnessed inverse. When a equals b the difference is
         * zero: the first constraint holds for any d, and the second collapses to d = 1.
         * When a and b differ the first forces d = 0, and the second is satisfied only by
         * aux = 1 / (a - b), a witness that exists precisely because the difference is
         * nonzero. So d is one exactly when the operands agree, with no branch.
         */
        c.s_eq * (c.d * diff),
        c.s_eq * (c.d + diff * c.aux - one),
        // Select: c is boolean and d = c ? a : b, written c*a + b - c*b.
        c.s_sel * (c.c * (c.c - one)),
        c.s_sel * (c.d - (c.c * c.a + c.b - c.c * c.b)),
        // Constraint opcodes: a is boolean, or a is zero.
        c.s_bool * (c.a * (c.a - one)),
        c.s_assert * c.a,
        // Input: the register takes the immediate the boundary pins.
        c.s_inp * (c.d - c.imm),
    ]
}
