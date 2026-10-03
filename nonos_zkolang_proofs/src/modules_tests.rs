/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The crates under `modules/`, each a directory whose `main.zkl` is the root and whose
 * modules are files around it (section 4.2).
 */

use std::fs;
use std::path::PathBuf;

use crate::golden_corpus::repo_root;
use crate::modules_run::crate_problems;

#[test]
fn every_crate_of_several_files_loads_and_checks() {
    let root = repo_root().join("nonos_zkolang_proofs/modules");
    let mut dirs: Vec<PathBuf> = fs::read_dir(&root)
        .expect("modules directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    assert!(dirs.len() >= 2, "too few crates under {}", root.display());
    let problems: String = dirs
        .iter()
        .map(|d| crate_problems(&d.display().to_string(), d))
        .collect();
    assert!(problems.is_empty(), "{problems}");
}
