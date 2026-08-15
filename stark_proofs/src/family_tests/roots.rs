// NONOS Operating System (AGPL-3.0-or-later)

use crate::recursion_assembly::{assemble_capped, Tamper};
use crate::witness_satisfies::satisfies;

/// The roots family: each opening checkpoint equals the transcript-absorbed root
/// it authenticates under. Authenticating the deep and comp values under each
/// other's root keeps both Merkle walks valid, so only this binding separates them.
#[test]
fn an_opening_under_the_wrong_root_rejects() {
    let asm = assemble_capped(Tamper::SwappedRoot, 0, 2);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "values authenticated under each other's root verified"
    );
}
