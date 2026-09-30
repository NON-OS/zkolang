/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `test`: run every `#[test]` of an edition 2026 crate (section 16), each compiled and run
 * on the machine, and report each; any failing test makes the command fail.
 */

use std::fs;

use nonos_zkolang::compiler::diag::render;
use nonos_zkolang::compiler::driver::test as run_tests;
use nonos_zkolang::compiler::source::SourceMap;

use super::disk::Disk;
use super::test_report::print_reports;
use crate::line::Line;
use crate::out::paint;

const USAGE: &str = "usage: zkolang test <file> [--edition 2026]";

/** Run the tests of the crate whose root file `args` names. */
pub(crate) fn test(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--edition"], USAGE)?;
    if let Some(e) = line.value("--edition").filter(|e| *e != "2026") {
        return Err(format!("--edition {e}: tests are an edition 2026 form"));
    }
    let src = fs::read_to_string(line.file).map_err(|e| format!("read {}: {e}", line.file))?;
    let mut map = SourceMap::new();
    let (reports, warnings) = match run_tests(&mut map, &Disk, (line.file, src)) {
        Ok(r) => r,
        Err(d) => {
            d.items()
                .iter()
                .for_each(|x| eprint!("{}", render(&map, x)));
            let n = d.items().iter().filter(|x| x.is_error()).count();
            return Err(format!("{}: {n} errors; no test ran", line.file));
        }
    };
    warnings
        .items()
        .iter()
        .for_each(|x| eprint!("{}", render(&map, x)));
    let failed = print_reports(&map, &reports);
    let tests = |n: usize| if n == 1 { "test" } else { "tests" };
    let passed = reports.len().saturating_sub(failed);
    let head = match failed {
        0 => paint("ok", "1;32"),
        _ => paint("FAILED", "1;31"),
    };
    println!("test result: {head}. {passed} passed; {failed} failed");
    match failed {
        0 => Ok(()),
        n => Err(format!(
            "{}: {n} of {} {} failed",
            line.file,
            reports.len(),
            tests(reports.len())
        )),
    }
}
