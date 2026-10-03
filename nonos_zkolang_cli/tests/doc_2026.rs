/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `doc` prints a crate's reference from its doc comments, or the standard library's. */

use std::path::PathBuf;
use std::process::Command;

fn zkolang(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .args(args)
        .output()
        .expect("run zkolang");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).to_string(),
    )
}

#[test]
fn doc_prints_a_reference() {
    let (ok, out) = zkolang(&["doc", "--std"]);
    assert!(ok && out.contains("# `std`"), "{out}");
    assert!(
        out.contains("### `pub fn permute(s: field) -> field`"),
        "{out}"
    );
    let sample =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../nonos_zkolang_proofs/doc/sample.zkl");
    let (ok, out) = zkolang(&["doc", sample.to_str().expect("utf-8")]);
    assert!(
        ok && out.contains("### `pub struct Point`") && !out.contains("helper"),
        "{out}"
    );
}
