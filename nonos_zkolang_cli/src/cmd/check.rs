/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `check`: compile without proving. */

use nonos_zkolang::program_log_t;

use super::prepare::compiled;
use crate::line::Line;
use crate::out::paint;

const USAGE: &str = "usage: zkolang check <file>";

/**
 * Compile the program, and refuse one the prover cannot size a trace for, as `run` and
 * `key` refuse it, rather than report as fine a program that can never be proven.
 */
pub(crate) fn check(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &[], USAGE)?;
    let (_, ops) = compiled(&line)?;
    if program_log_t(&ops).is_none() {
        let n = ops.len();
        return Err(format!(
            "the program needs {n} steps, more than a proof can hold"
        ));
    }
    println!("{}  {} instructions", paint("ok", "1;32"), ops.len());
    Ok(())
}
