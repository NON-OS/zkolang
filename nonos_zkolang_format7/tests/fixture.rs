/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The committed format 7 fixture holds. The statement made from `fixture/sum.zkl` now is
 * the one pinned beside the proof, the run's public words are the pinned ones, and
 * `nox_verify` accepts the pinned proof and refuses it with a byte flipped or another
 * output. A change to the compiler, the image or the STARKs pin that moves any of them
 * fails here; `cargo run --release -p nonos_zkolang_format7 --example fixture` writes the
 * fixture again.
 */

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::compiler::driver::{build, Source};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang_format7::{statement, text, verify, words};

fn read(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixture")
        .join(name);
    fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn the_pinned_proof_verifies_against_the_statement_made_now() {
    let src = String::from_utf8(read("sum.zkl")).expect("utf-8");
    let b = build(
        &mut SourceMap::new(),
        &NoFiles,
        Source::file("sum.zkl", src),
    );
    let b = b.expect("the fixture's program builds");
    let st = statement(&b).expect("a statement");
    assert!(st.image == read("program.bin"), "the image moved");
    assert_eq!(
        text(&st).as_bytes(),
        &read("statement.txt")[..],
        "the statement moved"
    );
    let pinned: Vec<u64> = String::from_utf8(read("words.txt"))
        .expect("utf-8")
        .lines()
        .map(|l| l.parse().expect("a word"))
        .collect();
    let w = words(&b, &st, &[12], &[149]).expect("words");
    assert_eq!(w, pinned, "the words moved");
    let proof = read("proof.bin");
    verify(&st, &proof, &w).expect("nox_verify accepts the pinned proof");

    let mut bad = proof.clone();
    let at = bad.len() / 3;
    bad[at] ^= 1;
    assert!(verify(&st, &bad, &w).is_err(), "a flipped byte");
    let other = words(&b, &st, &[12], &[150]).expect("words");
    assert!(verify(&st, &proof, &other).is_err(), "another output");
}
