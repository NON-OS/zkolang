/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lengthening a machine program to a number of rows. A trace too short cannot carry a
 * blinding of every point a proof opens it at, so it cannot be proved hiding its witness;
 * constants written to a register before the halt give it the rows it needs
 * and change nothing it computes.
 */

use nonos_stark::field::Fp;

use super::machine::{Machine, Origin};
use crate::isa::{Op, REGS};

/** The register padding writes: at the end of a program, no value is needed any more. */
const PAD: u8 = (REGS - 1) as u8;

/** Insert padding before the final halt of `m` until it has at least `rows` instructions. */
pub fn pad(m: &mut Machine, rows: usize) {
    let fits = m.ops.len() == m.origins.len();
    if !fits || !matches!(m.ops.last(), Some(Op::Halt)) || m.ops.len() >= rows {
        return;
    }
    let at = m.ops.len() - 1;
    let n = rows - m.ops.len();
    let op = Op::Imm {
        d: PAD,
        v: Fp::ZERO,
    };
    m.ops.splice(at..at, core::iter::repeat_n(op, n));
    m.origins
        .splice(at..at, core::iter::repeat_n(Origin::Pad, n));
}
