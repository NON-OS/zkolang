// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The statement family: the DEEP batching coefficients are the ones the STARK
/// transcript squeezed. A free coefficient batches just as well, which is the
/// whole point of binding them.
#[test]
fn an_unsqueezed_batching_coefficient_rejects() {
    let asm = assemble_capped(Tamper::OffTranscriptCoeff, 0, 2);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "a DEEP coefficient the transcript never squeezed verified"
    );
}
