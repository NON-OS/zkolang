/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The programs under `packages/`, each a directory of packages whose root package is
 * `root/` (section 4.1).
 */

use std::fs;
use std::path::PathBuf;

use crate::golden_corpus::repo_root;
use crate::packages_run::program_problems;

#[test]
fn every_program_of_packages_loads_and_checks() {
    let root = repo_root().join("nonos_zkolang_proofs/packages");
    let mut dirs: Vec<PathBuf> = fs::read_dir(&root)
        .expect("packages directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    assert!(dirs.len() >= 5, "too few programs under {}", root.display());
    let problems: String = dirs
        .iter()
        .map(|d| program_problems(&d.display().to_string(), d))
        .collect();
    assert!(problems.is_empty(), "{problems}");
}
