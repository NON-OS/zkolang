/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The verifier keys circuits/shield/README.md lists are the circuits' keys. */

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, verifier_key, REGISTRATION_RATE, TRACE_WIDTH};

fn base() -> PathBuf {
    let mut b = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    b.pop();
    b
}

fn resolve(path: &str) -> Option<String> {
    fs::read_to_string(base().join("stdlib").join(path)).ok()
}

fn key_of(name: &str) -> String {
    let src = fs::read_to_string(base().join("circuits/shield").join(name)).expect("read");
    let src = expand_includes(&src, &mut resolve).expect("expand");
    let program = nonos_zkolang::compile_source(&src).expect("compile");
    let vk = verifier_key(&program, REGISTRATION_RATE).expect("key");
    vk.iter().map(|b| format!("{b:02x}")).collect()
}

/* Each row of the key table names a circuit and its key, written as the first and the
 * last eight hex digits; both must be those of the circuit as this compiler and the
 * prover it depends on build it. The rate and the width the README states are checked
 * with them. */
#[test]
fn the_readme_keys_are_the_circuits_keys() {
    let readme = fs::read_to_string(base().join("circuits/shield/README.md")).expect("read");
    assert!(readme.contains("registration rate three and trace width fifty one"));
    assert_eq!((REGISTRATION_RATE, TRACE_WIDTH), (3, 51));
    let mut rows = 0;
    for line in readme.lines().filter(|l| l.starts_with("| `")) {
        let cells: Vec<&str> = line.split('`').collect();
        let (name, listed) = (cells[1], cells[3]);
        let (head, tail) = listed.split_once('…').expect("an abbreviated key");
        let key = key_of(name);
        assert!(
            key.starts_with(head) && key.ends_with(tail),
            "{name}: the README lists {listed}, the circuit's key is {key}"
        );
        rows += 1;
    }
    assert_eq!(rows, 2, "the README's key table");
}
