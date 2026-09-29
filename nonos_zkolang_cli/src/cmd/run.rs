/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `run`: compile, prove, verify and report. */

use nonos_zkolang::prove_source_with_witness;

use super::prepare::source_and_inputs;
use crate::line::Line;
use crate::out::paint;
use crate::render_run::render_run;

const USAGE: &str = "usage: zkolang run <file> [--input a,b] [--witness x,y]";

pub(crate) fn run(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--input", "--witness"], USAGE)?;
    let (src, inputs, witness) = source_and_inputs(&line)?;
    let r = prove_source_with_witness(&src, &inputs, &witness).map_err(|e| render_run(&src, &e))?;
    if !r.verified {
        return Err(String::from("proof did not verify"));
    }
    println!("{}", paint("verified", "1;32"));
    println!("outputs {:?}", r.outputs);
    println!("steps {}  trace 2^{}", r.steps, r.log_trace_len);
    Ok(())
}
