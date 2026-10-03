/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The format 7 commands end to end: `statement` writes the image and statement of a
 * program, `prove` writes a proof `nox_verify` accepted beside the same statement, and
 * `verify` accepts it for the inputs and outputs of the run and refuses other outputs.
 */

use std::path::{Path, PathBuf};
use std::process::Command;

const SUM: &str = "fn main(x: public u32, y: secret u32) -> u32 {
    assert y < 100, \"y is small\";
    declassify(x * x + y)
}
";

fn zkolang(dir: &Path, args: &[&str]) -> (bool, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_zkolang"));
    let out = cmd
        .current_dir(dir)
        .args(args)
        .arg("--edition")
        .arg("2026")
        .output();
    let out = out.expect("run zkolang");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    (
        out.status.success(),
        text + &String::from_utf8_lossy(&out.stderr),
    )
}

fn dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-f7-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    std::fs::write(dir.join("sum.zkl"), SUM).expect("write");
    dir
}

#[test]
fn a_proof_made_by_prove_is_verified_by_verify() {
    let d = dir();
    let (ok, text) = zkolang(&d, &["statement", "sum.zkl", "--out", "pinned"]);
    assert!(ok && text.contains("format 7\n"), "{text}");
    let pinned = std::fs::read(d.join("pinned/program.bin")).expect("an image");
    let run = [
        "prove", "sum.zkl", "--public", "12", "--secret", "5", "--out", "run",
    ];
    let (ok, text) = zkolang(&d, &run);
    assert!(ok && text.contains("verified by nox_verify"), "{text}");
    assert!(text.contains("outputs [149]"), "{text}");
    assert_eq!(
        std::fs::read(d.join("run/program.bin")).expect("an image"),
        pinned
    );
    let check = |outputs: &str| {
        let args = [
            "verify",
            "sum.zkl",
            "--proof",
            "run/proof.bin",
            "--public",
            "12",
        ];
        zkolang(&d, &[&args[..], &["--outputs", outputs]].concat())
    };
    let (ok, text) = check("149");
    assert!(ok && text.contains("verified by nox_verify"), "{text}");
    let (ok, text) = check("150");
    assert!(!ok && text.contains("refused"), "{text}");
}
