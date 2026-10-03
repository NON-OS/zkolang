/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `verify`: check a format 7 proof with `nox_verify`. The statement is made from the
 * program and the public words from the public inputs and the outputs the command line
 * claims, never taken from the prover, so a proof verifies only of this program, these
 * inputs and these outputs.
 */

use std::fs;

use nonos_zkolang_format7::{statement, verify as verify7, words};

use super::format7::needs_2026;
use super::format7_why::{abi_why, why};
use super::modern::built;
use super::values::values;
use crate::line::Line;
use crate::out::paint;

const USAGE: &str = "usage: zkolang verify <file> --edition 2026 --proof proof.bin \
                     [--public a,b] [--outputs y,z]";

/** Verify the proof `args` names of the program it names. */
pub(crate) fn verify(args: &[String]) -> Result<(), String> {
    let known = ["--edition", "--proof", "--public", "--outputs"];
    let line = Line::parse(args, &known, USAGE)?;
    needs_2026(&line)?;
    let path = line
        .value("--proof")
        .ok_or(format!("--proof is needed\n{USAGE}"))?;
    let proof = fs::read(path).map_err(|e| format!("read {path}: {e}"))?;
    let (_, b) = built(&line)?;
    let st = statement(&b).map_err(|e| format!("{}: {}", line.file, why(&e)))?;
    let (public, outputs) = (values(&line, "--public")?, values(&line, "--outputs")?);
    let w = words(&b, &st, &public, &outputs);
    let w = w.map_err(|e| abi_why("--public or --outputs", e))?;
    match verify7(&st, &proof, &w) {
        Ok(()) => {
            println!("{}", paint("verified by nox_verify", "1;32"));
            Ok(())
        }
        Err((_, why)) => Err(format!("{path}: refused: {why}")),
    }
}
