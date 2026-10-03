/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the proving commands share: the program and its inputs. */

use nonos_zkolang::{compile_source_full, render_error, Compiled};

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
pub(super) fn compiled(line: &Line) -> Result<(String, Compiled), String> {
    let src = load(line.file)?;
    let compiled = compile_source_full(&src).map_err(|e| render_error(&src, &e))?;
    Ok((src, compiled))
}
