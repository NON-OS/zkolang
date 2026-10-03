/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every edition 2026 file of the tree that parses is laid out as the formatter lays it
 * out: the standard library, the semantics and ui tests, the module and package trees.
 */

use std::fs;
use std::path::{Path, PathBuf};

use nonos_zkolang::compiler::fmt::{format, FmtError};

use crate::golden_corpus::repo_root;

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("a directory").flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "zkl") {
            out.push(path);
        }
    }
}

#[test]
fn the_tree_is_formatted() {
    let root = repo_root();
    let mut files = Vec::new();
    for dir in [
        "std",
        "nonos_zkolang_proofs/semantics",
        "nonos_zkolang_proofs/ui",
    ] {
        collect(&root.join(dir), &mut files);
    }
    for dir in ["modules", "packages", "doc"] {
        collect(&root.join("nonos_zkolang_proofs").join(dir), &mut files);
    }
    let (mut formatted, mut untidy) = (0, Vec::new());
    for path in &files {
        let src = fs::read_to_string(path).expect("read");
        match format(&src) {
            Ok(out) if out == src => formatted += 1,
            Ok(_) => untidy.push(path.display().to_string()),
            Err(FmtError::Changed) => untidy.push(format!("{} (guard)", path.display())),
            Err(FmtError::Syntax(_)) => {}
        }
    }
    assert!(untidy.is_empty(), "not formatted: {untidy:?}");
    assert!(formatted > 100, "only {formatted} files formatted");
}
