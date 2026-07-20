/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Build the public statement bound into the proof.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use crate::commit;
use crate::isa::Op;
use crate::trace::Trace;

/// The public statement the transcript seeds: the program commitment, the padded
/// trace length so the fee is checkable, then the public inputs and outputs. The
/// verifier replays exactly this, so a proof is tied to one program, one trace size,
/// and one public input and output.
pub(super) fn build_publics(program: &[Op], trace_len: usize, trace: &Trace) -> Vec<Fp> {
    let mut publics: Vec<Fp> = Vec::new();
    publics.extend_from_slice(&commit::commit_limbs(program));
    publics.push(Fp::from_u64(trace_len as u64));
    publics.extend_from_slice(&trace.public_inputs);
    publics.extend_from_slice(&trace.public_outputs);
    publics
}
