/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The step AIR pins a row that reads a public input to its committed value, but a row
 * that reads a secret input or an advice slot holds whatever the prover writes there. Two
 * reads of one secret or advice slot can differ, so the machine program reads each at most
 * once, and a value that must be read again is kept in a register or recomputed instead.
 */

use std::collections::BTreeSet;

use nonos_zkolang::compiler::codegen::CodegenError;
use nonos_zkolang::compiler::driver::{backend, BackendError};
use nonos_zkolang::Op;

use crate::prop_gen::Rng;
use crate::ssa_gen::program;

/** The secret input and advice slots `ops` reads more than once. */
pub(crate) fn rereads(ops: &[Op], n_public: usize) -> Vec<u16> {
    let mut seen = BTreeSet::new();
    let mut twice = Vec::new();
    for op in ops {
        if let Op::Inp { idx, .. } = *op {
            if usize::from(idx) >= n_public && !seen.insert(idx) {
                twice.push(idx);
            }
        }
    }
    twice
}

#[test]
fn no_secret_or_advice_slot_is_read_twice() {
    let mut r = Rng(0x5EC2_E7ED_2EAD_0A11);
    let (mut compiled, mut problems) = (0, Vec::new());
    for case in 0..300 {
        let mut ssa = program(&mut r, 20 + (case % 7) * 15);
        /* Inputs 2 and 3 are secret. */
        (ssa.n_public, ssa.n_secret) = (2, 2);
        match backend(&ssa) {
            Ok(c) => {
                compiled += 1;
                let twice = rereads(&c.machine.ops, 2);
                if !twice.is_empty() {
                    problems.push(format!("case {case}: slots {twice:?} are read again"));
                }
            }
            Err(BackendError::Codegen(CodegenError::Pressure(_))) => {}
            Err(e) => problems.push(format!("case {case}: {e:?}")),
        }
    }
    let shown: Vec<&String> = problems.iter().take(6).collect();
    assert!(
        problems.is_empty(),
        "{} problems, first {shown:#?}",
        problems.len()
    );
    assert!(compiled > 250, "only {compiled} programs compiled");
}
