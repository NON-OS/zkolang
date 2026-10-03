/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `check` of a crate with no `fn main`, a library, checks it and says so rather than
 * failing for want of a `main`; `run` of it still refuses with E0900.
 */

use std::path::PathBuf;
use std::process::Command;

const MANIFEST: &str = "[package]\nname = \"tally\"\nversion = \"0.1.0\"\nedition = \"2026\"\nentry = \"src/lib.zkl\"\n";
const LIB: &str = "pub fn yes<const N: usize>(votes: [bool; N]) -> u32 {\n    let mut n = 0;\n    for i in 0..N {\n        if votes[i] {\n            n += 1;\n        }\n    }\n    n\n}\n";
const BROKEN: &str = "pub fn yes() -> u32 {\n    true\n}\n";

fn package(name: &str, lib: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-lib-{name}-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("src")).expect("temp dir");
    std::fs::write(dir.join("zkolang.toml"), MANIFEST).expect("write manifest");
    std::fs::write(dir.join("src/lib.zkl"), lib).expect("write library");
    dir.join("src/lib.zkl")
}

fn zkolang(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .args(args)
        .output()
        .expect("run zkolang");
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}

#[test]
fn a_library_checks_without_a_main() {
    let lib = package("ok", LIB);
    let (ok, text) = zkolang(&["check", lib.to_str().expect("utf-8")]);
    assert!(ok, "{text}");
    assert!(text.contains("ok  a library"), "{text}");
    let (ran, text) = zkolang(&["run", lib.to_str().expect("utf-8")]);
    assert!(!ran && text.contains("E0900"), "{text}");
}

#[test]
fn a_library_that_does_not_check_is_refused() {
    let lib = package("broken", BROKEN);
    let (ok, text) = zkolang(&["check", lib.to_str().expect("utf-8")]);
    assert!(!ok, "{text}");
    assert!(
        text.contains("error[E") && !text.contains("E0900"),
        "{text}"
    );
}
