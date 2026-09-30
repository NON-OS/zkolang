/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A file that a manifest governs takes the manifest's edition without `--edition`, and is
 * built with its package's path dependencies (section 4.1).
 */

use std::path::PathBuf;
use std::process::Command;

fn fixture(rel: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../nonos_zkolang_proofs/packages");
    root.join(rel).to_string_lossy().into_owned()
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
fn a_package_is_built_with_its_dependencies() {
    let main = fixture("diamond/root/src/main.zkl");
    let (ok, text) = zkolang(&["test", &main]);
    assert!(
        ok && text.contains("test dependencies_are_found_by_name ... ok"),
        "{text}"
    );
    assert!(!text.contains("not_run_by_the_root"), "{text}");
    let (ok, text) = zkolang(&["check", &fixture("cycle/root/src/main.zkl")]);
    assert!(
        !ok && text.contains("error[E0902]") && text.contains("closes a dependency cycle"),
        "{text}"
    );
    let (ok, text) = zkolang(&["check", &fixture("broken_manifest/root/src/main.zkl")]);
    assert!(
        !ok && text.contains("`name` is given twice in `[package]`"),
        "{text}"
    );
}
