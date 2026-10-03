/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What pins each boundary cell: a value the program fixes, or a public input or output.
 * A verifier that holds the circuit apart from the statement, such as one reading a
 * program image, names the public words a boundary reads rather than carrying their
 * values, so one image serves every run of a program.
 */

use alloc::vec::Vec;

use nonos_stark::air::Air;
use nonos_stark::field::Fp;

use super::step_air::StepAir;

/** What a boundary pins its cell to. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    /** A value the program fixes. */
    Value(Fp),
    /** Public input slot `k`. */
    Input(usize),
    /** Output slot `k`. */
    Output(usize),
}

impl StepAir {
    /**
     * `boundary()` in its order, each entry with what pins it in place of its value. The
     * public bindings come last, after the cells the program fixes.
     */
    pub fn boundary_pins(&self) -> Vec<(usize, usize, Pin)> {
        let all = self.boundary();
        let fixed = all.len() - self.pins.len();
        let pin = |i: usize, v: Fp| i.checked_sub(fixed).map_or(Pin::Value(v), |k| self.pins[k]);
        all.into_iter()
            .enumerate()
            .map(|(i, (c, r, v))| (c, r, pin(i, v)))
            .collect()
    }
}
