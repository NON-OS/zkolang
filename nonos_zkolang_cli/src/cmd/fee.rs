/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `fee`: the pay-to-prove cost of a run. */

use nonos_zkolang::{prove_source_with_witness, quote};

use super::prepare::source_and_inputs;
use crate::line::Line;
use crate::render_run::render_run;

const USAGE: &str = "usage: zkolang fee <file> [--input a,b] [--witness x,y]";

pub(crate) fn fee(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--input", "--witness"], USAGE)?;
    let (src, inputs, witness) = source_and_inputs(&line)?;
    let r = prove_source_with_witness(&src, &inputs, &witness).map_err(|e| render_run(&src, &e))?;
    if !r.verified {
        return Err(String::from("proof did not verify"));
    }
    let q = quote(&r);
    println!(
        "cells {}  ({} rows x {} width)",
        q.cells, r.trace_len, r.trace_width
    );
    println!(
        "base {} + compute {} = {} micronox",
        q.base_micronox, q.compute_micronox, q.total_micronox
    );
    println!(
        "protocol {} micronox, prover {} micronox",
        q.protocol_fee_micronox, q.prover_micronox
    );
    Ok(())
}
