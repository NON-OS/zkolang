/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Running a program's tests (section 16). Each `#[test]` function is called with no
 * arguments; it passes when its run accepts, or, marked `#[should_fail]`, when the run
 * fails by a condition of section 14.1. Running out of steps and a failure of the
 * interpreter itself are not such conditions, so they never pass.
 */

use alloc::vec::Vec;

use super::{FailKind, Failure, Interp, Value};
use crate::compiler::tir::{FnId, TProgram};

/** How one test ran. */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestRun {
    pub f: FnId,
    pub should_fail: bool,
    pub outcome: Result<Value, Failure>,
}

impl TestRun {
    /** Whether the test passed. */
    pub fn passed(&self) -> bool {
        match (&self.outcome, self.should_fail) {
            (Ok(_), false) => true,
            (Err(f), true) => !matches!(f.kind, FailKind::Budget | FailKind::Internal),
            _ => false,
        }
    }
}

/** Run every test of `p`, each within `budget` steps. */
pub fn run_tests(p: &TProgram, budget: u64) -> Vec<TestRun> {
    let mut out = Vec::with_capacity(p.tests.len());
    for &(f, should_fail) in &p.tests {
        let Some(body) = p.fns.get(f.0 as usize) else {
            continue;
        };
        let outcome = Interp::new(p, budget).call(f, Vec::new(), body.span);
        out.push(TestRun {
            f,
            should_fail,
            outcome,
        });
    }
    out
}
