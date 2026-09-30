/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What `test` prints of each test: its name, whether it passed, and why not. */

use nonos_zkolang::compiler::diag::render;
use nonos_zkolang::compiler::driver::TestReport;
use nonos_zkolang::compiler::source::SourceMap;

use crate::out::paint;

/** Print each report, its diagnostics rendered against `map`; how many failed. */
pub(super) fn print_reports(map: &SourceMap, reports: &[TestReport]) -> usize {
    let noun = if reports.len() == 1 { "test" } else { "tests" };
    println!("running {} {noun}", reports.len());
    let mut failed = 0;
    for r in reports {
        let passed = r.failure.items().is_empty();
        let word = match passed {
            true => paint("ok", "1;32"),
            false => paint("FAILED", "1;31"),
        };
        println!("test {} ... {word}", r.name);
        r.failure
            .items()
            .iter()
            .for_each(|x| print!("{}", render(map, x)));
        failed += usize::from(!passed);
    }
    failed
}
