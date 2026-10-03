/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `build --edition 2026` writes a C file and a Python script that run the program
 * natively: both print the result `run` proves, and fail with status 3 where the run
 * fails. `--target asm` is refused for edition 2026.
 */

use std::path::{Path, PathBuf};
use std::process::Command;

const SQUARE: &str = "fn main(x: public u32) -> u32 {\n    x * x\n}\n";

/** `zkolang build square.zkl --edition 2026 --target TARGET` and then `more`, in `dir`. */
fn build(dir: &Path, target: &str, more: &[&str]) -> (bool, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_zkolang"));
    let args = ["build", "square.zkl", "--edition", "2026", "--target"];
    let out = cmd
        .current_dir(dir)
        .args(args)
        .arg(target)
        .args(more)
        .output();
    let out = out.expect("run zkolang");
    let why = String::from_utf8_lossy(&out.stderr).to_string();
    (out.status.success(), why)
}

/** What the target `bin` in `dir`, run on input `x`, exits with and prints. */
fn ran(dir: &Path, bin: &str, x: &str) -> (Option<i32>, String) {
    let (program, script) = match bin.ends_with(".py") {
        true => (PathBuf::from("python3"), Some(dir.join(bin))),
        false => (dir.join(bin), None),
    };
    let out = Command::new(program).args(script).arg(x).output();
    let out = out.expect("run the target");
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (out.status.code(), text)
}

fn dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-build-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    std::fs::write(dir.join("square.zkl"), SQUARE).expect("write");
    dir
}

#[test]
fn edition_2026_builds_to_c_and_python() {
    let dir = dir();
    for (target, file) in [("c", "square.c"), ("python", "square.py")] {
        let (ok, why) = build(&dir, target, &["--out", file]);
        assert!(ok, "{target}: {why}");
    }
    let mut cc = Command::new("cc");
    let cc = cc.current_dir(&dir).args(["-o", "square", "square.c"]);
    assert!(cc.status().expect("cc").success());
    for bin in ["square", "square.py"] {
        assert_eq!(ran(&dir, bin, "12"), (Some(0), "144".into()), "{bin}");
        assert_eq!(ran(&dir, bin, "70000").0, Some(3), "{bin}");
        assert_eq!(ran(&dir, bin, "-1").0, Some(1), "{bin}");
    }
    let (ok, why) = build(&dir, "asm", &[]);
    assert!(!ok && why.contains("expected c or python"), "{why}");
}
