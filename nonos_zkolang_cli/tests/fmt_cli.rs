/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `fmt` lays a file out in place; `fmt --check` fails on a file not laid out and writes nothing. */

use std::process::Command;

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
fn fmt_lays_out_in_place_and_check_reports() {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-fmt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("p.zkl");
    let messy = "fn main(a: public u8) -> u8 {\n\n  a + 1   \n}\n\n";
    std::fs::write(&path, messy).expect("write");
    let p = path.to_str().expect("utf-8");
    let (ok, text) = zkolang(&["fmt", p, "--check"]);
    assert!(!ok && text.contains("not formatted"), "{text}");
    assert_eq!(std::fs::read_to_string(&path).expect("read"), messy);
    let (ok, text) = zkolang(&["fmt", p]);
    assert!(ok && text.contains("laid out"), "{text}");
    let tidy = "fn main(a: public u8) -> u8 {\n    a + 1\n}\n";
    assert_eq!(std::fs::read_to_string(&path).expect("read"), tidy);
    assert!(zkolang(&["fmt", p, "--check"]).0);
    std::fs::write(&path, "fn f( {\n").expect("write");
    let (ok, text) = zkolang(&["fmt", p]);
    assert!(!ok && text.contains("does not parse"), "{text}");
}
