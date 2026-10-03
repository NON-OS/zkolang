/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2026 front end over every `.zkl` test under `nonos_zkolang_proofs/ui`.
 * Each file states in `/*~ CODE */` comments the diagnostics it expects, and the front end
 * must report exactly those, each on its line.
 */

use std::fs;
use std::path::{Path, PathBuf};

use crate::golden_corpus::repo_root;
use crate::ui_expect::expected;
use crate::ui_run::report;

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("ui directory").flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "zkl") {
            out.push(path);
        }
    }
}

#[test]
fn every_ui_program_reports_what_it_expects() {
    let root = repo_root().join("nonos_zkolang_proofs/ui");
    let mut files = Vec::new();
    collect(&root, &mut files);
    files.sort();
    assert!(!files.is_empty(), "no ui tests under {}", root.display());
    let mut failures = String::new();
    for path in &files {
        let src = fs::read_to_string(path).expect("read ui test");
        let name = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .display()
            .to_string();
        let got = report(&name, &src);
        let want = expected(&src);
        if got.lines != want {
            failures.push_str(&format!(
                "{name}\n  expected {want:?}\n  reported {:?}\n{}\n",
                got.lines, got.rendered
            ));
        }
    }
    assert!(failures.is_empty(), "{failures}");
}
