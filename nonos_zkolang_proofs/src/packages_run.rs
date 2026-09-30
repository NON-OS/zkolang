/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One program of several packages, loaded from the manifest `root/zkolang.toml` of its
 * directory and checked: the `/*~ CODE */` comments of every file loaded, manifests
 * included, state the diagnostics on their lines, and a program that states none must
 * check cleanly and pass every `#[test]` of its root package.
 */

use std::fs;
use std::path::{Path, PathBuf};

use nonos_zkolang::compiler::diag::{render, Diagnostics};
use nonos_zkolang::compiler::interp::run_tests;
use nonos_zkolang::compiler::package::load_package;
use nonos_zkolang::compiler::sema::check_crates;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::Files;

use crate::modules_expect::{reported, stated};

/** The files under one directory. */
struct Dir(PathBuf);

impl Files for Dir {
    fn read(&self, path: &str) -> Option<String> {
        fs::read_to_string(self.0.join(path)).ok()
    }
}

/** The problems with the program in `dir`, named `name`. */
pub(crate) fn program_problems(name: &str, dir: &Path) -> String {
    let (mut map, mut diags) = (SourceMap::new(), Diagnostics::new());
    let Ok(text) = fs::read_to_string(dir.join("root/zkolang.toml")) else {
        return format!("{name}: no root/zkolang.toml\n");
    };
    let files = Dir(dir.to_path_buf());
    let crates = load_package(
        &files,
        ("root/zkolang.toml", text),
        None,
        &mut map,
        &mut diags,
    );
    let program = (!diags.has_errors()).then(|| {
        let (program, more) = check_crates(&mut map, &crates, true);
        diags.extend(more);
        program
    });
    let (got, want) = (reported(&map, &diags), stated(&map));
    if got != want {
        let shown: String = diags.items().iter().map(|d| render(&map, d)).collect();
        return format!("{name}\n  expected {want:?}\n  reported {got:?}\n{shown}\n");
    }
    let Some(program) = program.filter(|_| want.is_empty()) else {
        return String::new();
    };
    let runs = run_tests(&program, 10_000_000);
    let failed = runs.iter().filter(|r| !r.passed()).count();
    match (runs.len(), failed) {
        (0, _) => format!("{name}: no tests\n"),
        (_, 0) => String::new(),
        (_, n) => format!("{name}: {n} tests failed\n"),
    }
}
