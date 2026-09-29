/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the proving commands share: the program and its inputs. */

use nonos_zkolang::{compile_source, render_error, Op};

use crate::args::nums;
use crate::line::Line;
use crate::load::load;

/** The expanded source, the public inputs and the secrets a command line names. */
pub(super) fn source_and_inputs(line: &Line) -> Result<(String, Vec<u64>, Vec<u64>), String> {
    let src = load(line.file)?;
    let inputs = nums(line, "--input")?;
    let witness = nums(line, "--witness")?;
    Ok((src, inputs, witness))
}

/** The compiled program the command line names, and its expanded source. */
pub(super) fn compiled(line: &Line) -> Result<(String, Vec<Op>), String> {
    let src = load(line.file)?;
    let ops = compile_source(&src).map_err(|e| render_error(&src, &e))?;
    Ok((src, ops))
}
