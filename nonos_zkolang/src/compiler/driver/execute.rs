/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One run of a built program, checked against the reference interpreter (section 14):
 * both must fail or both return the same result. A failing run has no witness, so the
 * reference run says how and where it fails. A reference run that exhausts its budget
 * checks nothing, and the compiled run stands.
 */

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::abi::{decode, encode};
use super::build::Built;
use super::run::RunFailure;
use super::values::{args_of, leaves_of};
use super::witness::witness;
use crate::compiler::interp::{FailKind, Failure, Interp, Value};
use crate::vm::Vm;

/** A run: every input slot the machine reads, how many are public, and the result. */
pub(super) struct Executed {
    pub(super) full: Vec<Fp>,
    pub(super) n_public: usize,
    pub(super) outputs: Vec<i128>,
}

/** Run `b` on `public` and `secret` leaf values, and the reference run beside it. */
pub(super) fn execute(b: &Built, public: &[i128], secret: &[i128]) -> Result<Executed, RunFailure> {
    let pubs = encode(&b.public, public).map_err(|e| RunFailure::Inputs(e, false))?;
    let secs = encode(&b.secret, secret).map_err(|e| RunFailure::Inputs(e, true))?;
    let n_public = pubs.len();
    let inputs: Vec<Fp> = pubs.into_iter().chain(secs).collect();
    let ran = witness(&b.compiled, &inputs).ok().and_then(|full| {
        let trace = Vm::new()
            .run(&b.compiled.machine.ops, &full, n_public)
            .ok()?;
        let slots: Vec<u64> = trace.public_outputs.iter().map(|f| f.value()).collect();
        Some((full, slots))
    });
    let reference = reference(b, public, secret).ok_or(RunFailure::Disagree)?;
    let (full, slots, expected) = match (ran, reference) {
        (Some((full, slots)), Ok(v)) => {
            let mut expected = Vec::new();
            leaves_of(&v, &mut expected);
            (full, slots, Some(expected))
        }
        (Some((full, slots)), Err(f)) if f.kind == FailKind::Budget => (full, slots, None),
        (None, Err(f)) => return Err(RunFailure::Fails(f)),
        _ => return Err(RunFailure::Disagree),
    };
    let outputs = decode(&b.output, &slots).ok_or(RunFailure::Disagree)?;
    if expected.is_some_and(|e| e != outputs) {
        return Err(RunFailure::Disagree);
    }
    Ok(Executed {
        full,
        n_public,
        outputs,
    })
}

/** The reference run of `b`'s `main`, or `None` if it has none. */
fn reference(b: &Built, public: &[i128], secret: &[i128]) -> Option<Result<Value, Failure>> {
    let main = b.program.main?;
    let span = b.program.fns.get(main.0 as usize)?.span;
    let args = args_of(&b.program, main, public, secret);
    Some(Interp::new(&b.program, 1 << 30).call(main, args, span))
}
