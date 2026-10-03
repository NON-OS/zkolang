/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `run --edition 2026`: build, run on typed inputs, prove the run hiding the secret inputs,
 * verify, and report the result. The blinding seed is read fresh from the system's source
 * of randomness; without one there is no proof, since a fixed seed would not hide.
 */

use std::fs::File;
use std::io::Read;

use nonos_zkolang::compiler::diag::render;
use nonos_zkolang::compiler::driver::{diagnose, prove, seed_of, SEED_BYTES};

use super::modern::built;
use super::values::values;
use crate::line::Line;
use crate::out::paint;

/** Run and prove the program `line` names on its `--public` and `--secret` values. */
pub(super) fn run(line: &Line) -> Result<(), String> {
    let (map, b) = built(line)?;
    let public = values(line, "--public")?;
    let secret = values(line, "--secret")?;
    let seed = seed_of(&random()?);
    match prove(&b, &public, &secret, &seed) {
        Ok(p) => {
            println!("{}", paint("verified", "1;32"));
            println!("outputs {:?}", p.outputs);
            let r = &p.report;
            println!("rows {}  trace 2^{}", r.steps, r.log_trace_len);
            Ok(())
        }
        Err(f) => {
            eprint!("{}", render(&map, &diagnose(&b, &f)));
            Err(format!("{}: the run was not proved", line.file))
        }
    }
}

/** Fresh random bytes for the blinding seed. */
pub(super) fn random() -> Result<[u8; SEED_BYTES], String> {
    let mut bytes = [0u8; SEED_BYTES];
    File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut bytes))
        .map_err(|e| format!("no randomness for the proof's blinding: /dev/urandom: {e}"))?;
    Ok(bytes)
}
