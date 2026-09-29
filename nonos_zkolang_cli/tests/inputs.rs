/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The command line reads inputs strictly. A value it cannot read is refused by name, never
//! dropped: dropping one shifts every later value onto a different input, and the proof
//! would then be about numbers nobody typed.

use std::path::PathBuf;
use std::process::Command;

fn program(dir: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-{dir}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("p.zkl");
    std::fs::write(&path, src).expect("write program");
    path
}

fn run(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .args(args)
        .output()
        .expect("run zkolang");
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}

#[test]
fn an_unreadable_input_is_refused_not_dropped() {
    let p = program("unreadable", "input a;\ninput b;\noutput a + b;\n");
    let p = p.to_str().expect("utf-8 path");
    let (ok, text) = run(&["run", p, "--input", "1,x,3"]);
    assert!(!ok, "a malformed input list proved: {text}");
    assert!(text.contains("`x` is not a decimal number"), "{text}");
    let (ok, text) = run(&["run", p, "--input", "1,18446744069414584321"]);
    assert!(!ok, "an input at the modulus proved: {text}");
    assert!(text.contains("not below the field modulus"), "{text}");
    let (ok, text) = run(&["run", p, "--input"]);
    assert!(!ok, "a flag without its list proved: {text}");
    let (ok, text) = run(&["run", p, "--input", "1,3"]);
    assert!(ok, "{text}");
    assert!(text.contains("outputs [4]"), "{text}");
}
