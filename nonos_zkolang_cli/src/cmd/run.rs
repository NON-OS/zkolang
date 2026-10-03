/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `run`: compile, prove, verify and report. */

use nonos_zkolang::prove_source_with_witness;

use super::edition::modern;
use super::prepare::source_and_inputs;
use crate::line::Line;
use crate::out::paint;
use crate::render_run::render_run;

const USAGE: &str = "usage: zkolang run <file> [--input a,b] [--witness x,y]\n       zkolang run <file> --edition 2026 [--public a,b] [--secret x,y]";

pub(crate) fn run(args: &[String]) -> Result<(), String> {
    let flags = ["--input", "--witness", "--edition", "--public", "--secret"];
    let line = Line::parse(args, &flags, USAGE)?;
    let modern = modern(&line)?;
    let (ours, theirs) = match modern {
        true => (["--public", "--secret"], ["--input", "--witness"]),
        false => (["--input", "--witness"], ["--public", "--secret"]),
    };
    if let Some(f) = theirs.iter().find(|f| line.value(f).is_some()) {
        let e = if modern { 2026 } else { 2025 };
        return Err(format!(
            "{f} is not read in edition {e}; its values go in {}\n{USAGE}",
            ours.join(" and ")
        ));
    }
    if modern {
        return super::run_2026::run(&line);
    }
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
