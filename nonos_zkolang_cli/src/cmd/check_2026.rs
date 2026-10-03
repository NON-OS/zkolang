/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `check --edition 2026`: build without running, and report the rows. */

use nonos_zkolang::compiler::driver::declassified;

use super::cost::print_cost;
use super::library::library;
use super::modern::built;
use crate::line::Line;
use crate::out::paint;

/** Build the program `line` names and print its cost, unless `--json` asks for the diagnostics alone. */
pub(super) fn check(line: &Line) -> Result<(), String> {
    if library(line)? {
        return Ok(());
    }
    let (map, b) = built(line)?;
    if line.switch("--declassify") {
        for at in declassified(&b.program) {
            let (file, row, col) = map.locate(at).unwrap_or(("?", 0, 0));
            println!("{file}:{row}:{col}: {}", map.snippet(at));
        }
    }
    if line.switch("--cost") {
        print_cost(&map, &b);
    }
    let rows = b.compiled.machine.ops.len();
    if !line.switch("--json") {
        println!("{}  {rows} rows", paint("ok", "1;32"));
    }
    Ok(())
}
