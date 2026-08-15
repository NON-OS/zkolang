// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The fold family: the betas the chain descends on are the ones the FRI
/// transcript squeezed. The chain is built from the beta it uses, so its own
/// algebra holds and only the transcript binding is left to catch a free choice.
#[test]
fn a_fold_on_an_unsqueezed_beta_rejects() {
    let asm = assemble_capped(Tamper::OffTranscriptBeta, 0, 2);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "a fold chain on a beta the transcript never squeezed verified"
    );
}
