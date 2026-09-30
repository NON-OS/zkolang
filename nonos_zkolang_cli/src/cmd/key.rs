/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `key`: a circuit's registration commitment and verifier key. */

use nonos_zkolang::{commit, verifier_key, REGISTRATION_RATE};

use super::prepare::compiled;
use crate::line::Line;
use crate::out::hex;

const USAGE: &str = "usage: zkolang key <file>";

pub(crate) fn key(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &[], USAGE)?;
    let program = compiled(&line)?.1.ops;
    let vk = verifier_key(&program, REGISTRATION_RATE).map_err(|e| format!("key error: {e:?}"))?;
    println!("commit {}", hex(&commit(&program)));
    println!("vk     {}", hex(&vk));
    Ok(())
}
