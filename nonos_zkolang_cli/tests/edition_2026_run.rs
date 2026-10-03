/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `run --edition 2026` proves an honest run and reports its result, refuses a failing run
 * at the failing line, and refuses inputs that do not fit `main`.
 */

use std::path::PathBuf;
use std::process::Command;

const ROOT: &str = "fn main(y: public u32, x: secret u32) -> public u32 {\n    assert x * x == y, \"x is a root\";\n    y + 1\n}\n";

fn program(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-2026r-{name}-{}", std::process::id()));
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
fn run_proves_an_honest_run_and_refuses_the_rest() {
    let p = program("run", ROOT);
    let p = p.to_str().expect("utf-8");
    let run = |public: &str, secret: &str| {
        zkolang(&[
            "run",
            p,
            "--edition",
            "2026",
            "--public",
            public,
            "--secret",
            secret,
        ])
    };
    let (ok, text) = run("49", "7");
    assert!(
        ok && text.contains("verified") && text.contains("outputs [50]"),
        "{text}"
    );
    let (ok, text) = run("49", "6");
    assert!(
        !ok && text.contains("error[E0905]") && text.contains("x is a root"),
        "{text}"
    );
    let (ok, text) = run("4294967296", "7");
    assert!(!ok && text.contains("error[E0906]"), "{text}");
    let (ok, text) = run("49", "");
    assert!(!ok && text.contains("takes 1 secret value,"), "{text}");
    let (ok, text) = run("49", "7x");
    assert!(
        !ok && text.contains("`7x` is not a decimal integer"),
        "{text}"
    );
    let (ok, text) = zkolang(&["run", p, "--edition", "2026", "--input", "49"]);
    assert!(
        !ok && text.contains("--input is not read in edition 2026"),
        "{text}"
    );
}
