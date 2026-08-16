// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The periodic family: each recomputed P_j(z) is the compose input that consumes
/// it. Recomputing at another point leaves both regions internally consistent, so
/// the composition runs on a prover input unless this binding holds.
#[test]
fn a_periodic_column_off_the_composed_point_rejects() {
    let asm = assemble_capped(Tamper::PeriodicOffPoint, 0, 2);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "periodic columns recomputed at an unused point verified"
    );
}
