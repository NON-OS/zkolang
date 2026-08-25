// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The index family, which the assembly calls its forgery-critical seam: the x
/// query k's DEEP terms divide by is derived from the index k's own openings
/// authenticate. The point chain is an honest walk of whatever index it is given,
/// so nothing but this binding ties the divisor to the opened path.
#[test]
fn a_deep_divisor_off_the_opened_index_rejects() {
    let asm = assemble_capped(Tamper::ForeignConsistencyIndex, 0, 2);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "a DEEP divisor derived from an unopened index verified"
    );
}
