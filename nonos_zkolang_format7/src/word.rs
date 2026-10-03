/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The public words of a zKølang statement: all of them for a run, and the one a boundary reads. */

use nonos_zkolang::compiler::driver::abi::{encode, AbiError};
use nonos_zkolang::compiler::driver::Built;
use nonos_zkolang::{commit_limbs, Pin};

use super::statement::Statement;

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

/**
 * The public words of a run of `b` that took `public` and gave `outputs`, as a verifier
 * computes them for itself: the program commitment's limbs, the trace length `st` names,
 * then the inputs and the outputs, encoded as the program's slots.
 */
pub fn words(
    b: &Built,
    st: &Statement,
    public: &[i128],
    outputs: &[i128],
) -> Result<Vec<u64>, AbiError> {
    let limbs = commit_limbs(&b.compiled.machine.ops);
    let mut w: Vec<u64> = limbs.iter().map(|f| f.value()).collect();
    w.push(1u64 << st.log_t);
    let slots = encode(&b.public, public)?
        .into_iter()
        .chain(encode(&b.output, outputs)?);
    w.extend(slots.map(|f| f.value()));
    Ok(w)
}
