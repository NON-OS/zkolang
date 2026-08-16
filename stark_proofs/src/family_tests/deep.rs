// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The DEEP family: every trace value the check batches is the one query k's
/// opening authenticates. Cut one loose and the batch still evaluates, so only
/// this binding refuses it.
#[test]
fn a_trace_value_off_its_opening_rejects() {
    let asm = assemble_capped(Tamper::ReboundTraceValue, 0, 2);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "a DEEP trace value cut loose from its opening verified"
    );
}
