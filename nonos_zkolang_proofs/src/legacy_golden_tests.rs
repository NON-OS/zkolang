/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The compiled form of every program in the tree, pinned. A registered circuit is fixed by
 * its program commitment, so a compiler change that moves the ops of a program that
 * already compiled correctly would silently move a registered verifier key. The table is
 * held against a committed copy; `ZKOLANG_BLESS=1` rewrites it, and its diff names every
 * program whose compiled form moved.
 */

use std::fs;

use crate::golden_corpus::{corpus_table, repo_root};

#[test]
fn every_program_compiles_to_its_pinned_form() {
    let table = corpus_table();
    let golden = repo_root().join("nonos_zkolang_proofs/golden/legacy-commitments.txt");
    if std::env::var_os("ZKOLANG_BLESS").is_some() {
        fs::write(&golden, &table).expect("write the golden table");
        return;
    }
    let pinned = fs::read_to_string(&golden).expect("read the golden table");
    for (want, got) in pinned.lines().zip(table.lines()) {
        assert_eq!(got, want, "a program's compiled form moved");
    }
    assert_eq!(
        pinned.lines().count(),
        table.lines().count(),
        "the set of programs changed"
    );
}
