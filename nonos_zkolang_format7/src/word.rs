/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The public words of a zKølang statement: all of them for a run, and the one a boundary reads. */

use nonos_zkolang::compiler::driver::abi::{encode, AbiError};
use nonos_zkolang::compiler::driver::Built;
use nonos_zkolang::{commit_limbs, Op, Pin};

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
    let mut w = head(&b.compiled.machine.ops, st.log_t).to_vec();
    let slots = encode(&b.public, public)?
        .into_iter()
        .chain(encode(&b.output, outputs)?);
    w.extend(slots.map(|f| f.value()));
    Ok(w)
}

/** The words every run of `ops` begins with: its commitment's limbs, then `2^log_t`. */
pub(crate) fn head(ops: &[Op], log_t: u32) -> [u64; 5] {
    let l = commit_limbs(ops);
    [
        l[0].value(),
        l[1].value(),
        l[2].value(),
        l[3].value(),
        1u64 << log_t,
    ]
}
