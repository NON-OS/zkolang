/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `check --edition 2026`: build without running, and report the rows. */

use super::modern::built;
use crate::line::Line;
use crate::out::paint;

/** Build the program `line` names and print its cost. */
pub(super) fn check(line: &Line) -> Result<(), String> {
    let (_, b) = built(line)?;
    let rows = b.compiled.machine.ops.len();
    println!("{}  {rows} rows", paint("ok", "1;32"));
    Ok(())
}
