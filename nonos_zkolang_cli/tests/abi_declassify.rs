/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `abi` prints the layout of `main`'s inputs and result (section 12.2); `check
 * --declassify` lists each `declassify` of a program (section 13.2).
 */

use std::path::PathBuf;
use std::process::Command;

const PROGRAM: &str = "fn main(x: public u64, flag: secret bool, e: secret Option<u8>) -> u8 {\n    let v = match e {\n        Some(y) => y,\n        None => 0,\n    };\n    let w = if flag { v } else { 1 };\n    declassify(w).wrapping_add(u8::wrapping_from(x))\n}\n";

fn program() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-abi-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("p.zkl");
    std::fs::write(&path, PROGRAM).expect("write program");
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
fn abi_and_declassify_describe_a_program() {
    let p = program();
    let p = p.to_str().expect("utf-8");
    let (ok, text) = zkolang(&["abi", p, "--edition", "2026"]);
    assert!(ok, "{text}");
    assert!(
        text.contains("public inputs, 2 slots\n  x: u64  [u64]"),
        "{text}"
    );
    assert!(
        text.contains("secret inputs, 3 slots\n  flag: bool  [bool]"),
        "{text}"
    );
    assert!(
        text.contains("e: Option<u8>  [tag below 2, slot]"),
        "{text}"
    );
    assert!(text.contains("output, 1 slot\n  [u8]"), "{text}");
    let (ok, text) = zkolang(&["check", p, "--edition", "2026", "--declassify"]);
    assert!(ok && text.contains("p.zkl:7:5: declassify(w)"), "{text}");
}
