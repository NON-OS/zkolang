/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use super::document::document;
use std::fs;
use std::path::PathBuf;

fn path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push("spec/shield-key-hierarchy.json");
    p
}

#[test]
#[ignore]
fn emit_shield_key_hierarchy() {
    fs::write(path(), document()).expect("write vector");
}

/// The vector is the contract between the circuit, the wallet and the client.
/// Regenerating it changes what every note derives from, so drift fails here
/// before it reaches anyone.
#[test]
fn the_key_hierarchy_vector_is_frozen() {
    let on_disk = fs::read_to_string(path()).expect("vector missing; emit with --ignored");
    assert_eq!(
        on_disk,
        document(),
        "the key hierarchy drifted from the emitted vector"
    );
}
