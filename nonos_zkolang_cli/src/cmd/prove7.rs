/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `prove`: prove a run of an edition 2026 program in STARKs format 7, check the proof
 * with `nox_verify`, and write it beside the program's statement. The blinding seed is
 * read fresh from the system's source of randomness.
 */

use std::fs;
use std::path::Path;

use nonos_zkolang::compiler::diag::render;
use nonos_zkolang::compiler::driver::{diagnose, seed_of};
use nonos_zkolang_format7::{prove as prove7, verify, Error};

use super::format7::{needs_2026, write};
use super::format7_why::why;
use super::modern::built;
use super::run_2026::random;
use super::values::values;
use crate::line::Line;
use crate::out::paint;

const USAGE: &str =
    "usage: zkolang prove <file> --edition 2026 [--public a,b] [--secret x,y] [--out dir]";

/** Prove the run `args` names and write `proof.bin`, `program.bin` and `statement.txt`. */
pub(crate) fn prove(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--edition", "--public", "--secret", "--out"], USAGE)?;
    needs_2026(&line)?;
    let (map, b) = built(&line)?;
    let (public, secret) = (values(&line, "--public")?, values(&line, "--secret")?);
    let p = match prove7(&b, &public, &secret, &seed_of(&random()?)) {
        Ok(p) => p,
        Err(Error::Run(f)) => {
            eprint!("{}", render(&map, &diagnose(&b, &f)));
            return Err(format!("{}: the run was not proved", line.file));
        }
        Err(e) => return Err(format!("{}: {}", line.file, why(&e))),
    };
    verify(&p.statement, &p.bytes, &p.words).map_err(|(_, w)| format!("not verified: {w}"))?;
    let dir = line.value("--out").unwrap_or(".");
    write(dir, &p.statement)?;
    let path = Path::new(dir).join("proof.bin");
    fs::write(&path, &p.bytes).map_err(|e| format!("write {}: {e}", path.display()))?;
    println!("{}", paint("verified by nox_verify", "1;32"));
    println!("outputs {:?}", p.outputs);
    let (n, lg) = (p.bytes.len(), p.statement.log_t);
    println!("proof {n} bytes  trace 2^{lg}  in {}", path.display());
    Ok(())
}
