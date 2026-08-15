// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The positive case the three forgeries are read against. A tamper rejecting
/// only means a cell is constrained to something; without this it does not mean
/// it is constrained to the right thing.
#[test]
fn the_capped_assembly_satisfies_every_binding() {
    let asm = assemble_capped(Tamper::None, 0, 2);
    assert_eq!(asm.lay.n_q, 2, "cap did not take");
    assert!(satisfies(&asm.wired, &asm.witness), "the honest assembly failed a binding");
}
