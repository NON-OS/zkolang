/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `test` runs each `#[test]` of an edition 2026 crate, compiled, reports each, and fails
 * when one fails; an edition 2025 file is refused.
 */

use std::path::PathBuf;
use std::process::Command;

const PASSING: &str = "fn double(x: u8) -> u8 {\n    x * 2\n}\n\n#[test]\nfn doubles() {\n    assert double(4) == 8;\n}\n\n#[test]\n#[should_fail]\nfn overflows() {\n    let _v = double(200);\n}\n";

const FAILING: &str = "#[test]\nfn wrong() {\n    assert 1u8 + 1 == 3, \"arithmetic\";\n}\n";

fn program(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-2026t-{name}-{}", std::process::id()));
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
fn test_runs_every_test_and_fails_when_one_does() {
    let p = program("pass", PASSING);
    let p = p.to_str().expect("utf-8");
    let (ok, text) = zkolang(&["test", p]);
    assert!(ok, "{text}");
    assert!(
        text.contains("test doubles ... ok") && text.contains("test overflows ... ok"),
        "{text}"
    );
    assert!(
        text.contains("test result: ok. 2 passed; 0 failed"),
        "{text}"
    );
    let f = program("fail", FAILING);
    let (ok, text) = zkolang(&["test", f.to_str().expect("utf-8")]);
    assert!(!ok && text.contains("test wrong ... FAILED"), "{text}");
    assert!(
        text.contains("arithmetic") && text.contains("0 passed; 1 failed"),
        "{text}"
    );
    let (ok, text) = zkolang(&["test", p, "--edition", "2025"]);
    assert!(!ok && text.contains("edition 2026 form"), "{text}");
}
