/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `explain` prints a code's description; `check --json` prints an edition 2026 program's
 * diagnostics as one JSON array on standard output, and fails when one is an error.
 */

use std::path::PathBuf;
use std::process::Command;

fn program(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-json-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("p.zkl");
    std::fs::write(&path, src).expect("write program");
    path
}

fn zkolang(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .args(args)
        .output()
        .expect("run zkolang");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    (
        out.status.success(),
        stdout,
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn explain_prints_a_description() {
    let (ok, out, _) = zkolang(&["explain", "e0300"]);
    assert!(ok && out.starts_with("E0300: "), "{out}");
    let (ok, _, err) = zkolang(&["explain", "E9999x"]);
    assert!(!ok && err.contains("no diagnostic code"), "{err}");
}

#[test]
fn check_json_prints_the_diagnostics_alone() {
    let bad = program(
        "bad",
        "fn main(a: public u8) -> public u8 {\n    let _b: u16 = a;\n    a\n}\n",
    );
    let bad = bad.to_str().expect("utf-8");
    let (ok, out, _) = zkolang(&["check", bad, "--edition", "2026", "--json"]);
    assert!(
        !ok && out.starts_with("[{\"severity\":\"error\",\"code\":\"E0300\""),
        "{out}"
    );
    let good = program("good", "fn main(a: public u8) -> public u8 {\n    a\n}\n");
    let good = good.to_str().expect("utf-8");
    let (ok, out, _) = zkolang(&["check", good, "--edition", "2026", "--json"]);
    assert!(ok && out.trim() == "[]", "{out}");
    let (ok, _, err) = zkolang(&["check", good, "--json"]);
    assert!(!ok && err.contains("edition 2026"), "{err}");
}
