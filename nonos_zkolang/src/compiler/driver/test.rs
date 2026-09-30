/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Running a crate's tests (section 16): each `#[test]` compiled on its own and run on the
 * machine, which decides whether it accepts, and on the reference interpreter, whose run
 * says where a failing test fails. The two must agree; a test they disagree on is
 * reported as a fault of the compiler, not passed.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::test_verdict::{compiled_accepts, verdict};
use crate::compiler::diag::Diagnostics;
use crate::compiler::interp::{run_tests, TestRun};
use crate::compiler::sema::check_tests;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::load::{load, Files};
use crate::compiler::tir::TProgram;

/** The steps the reference run of one test may take. */
const BUDGET: u64 = 10_000_000;

/** How one test ended. */
#[derive(Clone, Debug)]
pub struct TestReport {
    pub name: String,
    pub should_fail: bool,
    /** Why the test did not pass; empty if it passed. */
    pub failure: Diagnostics,
}

/**
 * Run the tests of the crate whose root file is `root`, a path and its text, reading its
 * modules from `files` into `map`: a report per test and the warnings of checking it, or
 * the diagnostics of a crate that does not check.
 */
pub fn test(
    map: &mut SourceMap,
    files: &dyn Files,
    root: (&str, String),
) -> Result<(Vec<TestReport>, Diagnostics), Diagnostics> {
    let mut diags = Diagnostics::new();
    let ast = load(files, root, map, &mut diags);
    if diags.has_errors() {
        return Err(diags);
    }
    let (program, more) = check_tests(map, &ast);
    diags.extend(more);
    if diags.has_errors() {
        return Err(diags);
    }
    let runs = run_tests(&program, BUDGET);
    Ok((runs.iter().map(|r| report(&program, r)).collect(), diags))
}

/** The report of the test `r` of `p`. */
fn report(p: &TProgram, r: &TestRun) -> TestReport {
    let f = p.fns.get(r.f.0 as usize);
    let name = f.map_or_else(String::new, |f| f.name.clone());
    let failure = match compiled_accepts(p, r.f) {
        Ok(accepted) => verdict(p, r, accepted),
        Err(d) => d,
    };
    TestReport {
        name,
        should_fail: r.should_fail,
        failure,
    }
}
