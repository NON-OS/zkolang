/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The public words of a zKølang statement, and the one a boundary reads. */

use nonos_zkolang::Pin;

/**
 * The public word `pin` reads, in a statement of `inputs` public input slots: the
 * program commitment's four limbs and the trace length come first, then the inputs, then
 * the outputs. `None` for a value the program fixes.
 */
pub fn word(pin: Pin, inputs: usize) -> Option<usize> {
    match pin {
        Pin::Value(_) => None,
        Pin::Input(k) => Some(5 + k),
        Pin::Output(k) => Some(5 + inputs + k),
    }
}
