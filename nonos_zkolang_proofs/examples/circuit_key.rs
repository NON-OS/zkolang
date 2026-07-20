/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/
//! Compute the on-chain registration key of a circuit: read a .zkl file, resolve its
//! includes from stdlib, compile it, and print the program commitment and the verifier
//! key the registry gates on. This is what turns a circuit into a registered program.
use std::{env, fs, path::PathBuf};

use nonos_zkolang::{commit, compile_source, expand_includes, verifier_key, REGISTRATION_RATE};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() {
    let arg = env::args().nth(1).expect("usage: circuit_key <file.zkl>");
    let src = fs::read_to_string(&arg).expect("read");
    let mut resolve = |p: &str| fs::read_to_string(PathBuf::from("stdlib").join(p)).ok();
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let program = compile_source(&expanded).expect("compile");

    let commit_bytes = commit(&program);
    let vk = verifier_key(&program, REGISTRATION_RATE).expect("vk");
    println!("commit {}\nvk     {}", hex(&commit_bytes), hex(&vk));
}
