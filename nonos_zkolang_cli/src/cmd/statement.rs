/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `statement`: an edition 2026 program's format 7 statement, made from the program alone:
 * the image as `program.bin` and the numbers a gate pins beside it as `statement.txt`.
 */

use nonos_zkolang_format7::{statement as statement7, text};

use super::format7::{needs_2026, write};
use super::format7_why::why;
use super::modern::built;
use crate::line::Line;

const USAGE: &str = "usage: zkolang statement <file> --edition 2026 [--out dir]";

/** Write the statement of the program `args` names, and print it. */
pub(crate) fn statement(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--edition", "--out"], USAGE)?;
    needs_2026(&line)?;
    let (_, b) = built(&line)?;
    let st = statement7(&b).map_err(|e| format!("{}: {}", line.file, why(&e)))?;
    write(line.value("--out").unwrap_or("."), &st)?;
    print!("{}", text(&st));
    Ok(())
}
