/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `check --edition 2026` builds a program and reports its rows, or its diagnostics
 * rendered against the source; an unknown edition is refused.
 */

use std::path::PathBuf;
use std::process::Command;

const ROOT: &str = "fn main(y: public u32, x: secret u32) -> public u32 {\n    assert x * x == y, \"x is a root\";\n    y + 1\n}\n";

fn program(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-2026c-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("p.zkl");
    std::fs::write(&path, src).expect("write program");
    path
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
fn check_reports_rows_or_diagnostics() {
    let p = program("check", ROOT);
    let (ok, text) = zkolang(&["check", p.to_str().expect("utf-8"), "--edition", "2026"]);
    assert!(ok && text.contains("ok") && text.contains("rows"), "{text}");
    let bad = program(
        "bad",
        "fn main(a: public u8) -> public u8 {\n    let _b: u16 = a;\n    a\n}\n",
    );
    let (ok, text) = zkolang(&["check", bad.to_str().expect("utf-8"), "--edition", "2026"]);
    assert!(
        !ok && text.contains("error[E0300]") && text.contains("1 error;"),
        "{text}"
    );
    let (ok, text) = zkolang(&["check", bad.to_str().expect("utf-8"), "--edition", "2027"]);
    assert!(!ok && text.contains("`2027` is not an edition"), "{text}");
}
