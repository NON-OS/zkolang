/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One crate of several files, loaded from its directory and checked: the `/*~ CODE */`
 * comments of every file loaded state the diagnostics on their lines, and a crate that
 * states none must check cleanly and pass every `#[test]`.
 */

use std::fs;
use std::path::{Path, PathBuf};

use nonos_zkolang::compiler::diag::{render, Diagnostics};
use nonos_zkolang::compiler::interp::run_tests;
use nonos_zkolang::compiler::sema::check_tests;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::{load, Files};

use crate::modules_expect::{reported, stated};

/** The files of one directory. */
struct Dir(PathBuf);

impl Files for Dir {
    fn read(&self, path: &str) -> Option<String> {
        fs::read_to_string(self.0.join(path)).ok()
    }
}

/** The problems with the crate in `dir`, named `name`. */
pub(crate) fn crate_problems(name: &str, dir: &Path) -> String {
    let (mut map, mut diags) = (SourceMap::new(), Diagnostics::new());
    let Ok(text) = fs::read_to_string(dir.join("main.zkl")) else {
        return format!("{name}: no main.zkl\n");
    };
    let ast = load(
        &Dir(dir.to_path_buf()),
        ("main.zkl", text),
        &mut map,
        &mut diags,
    );
    let (program, more) = check_tests(&mut map, &ast);
    diags.extend(more);
    let (got, want) = (reported(&map, &diags), stated(&map));
    if got != want {
        let shown: String = diags.items().iter().map(|d| render(&map, d)).collect();
        return format!("{name}\n  expected {want:?}\n  reported {got:?}\n{shown}\n");
    }
    if !want.is_empty() {
        return String::new();
    }
    let runs = run_tests(&program, 10_000_000);
    let failed = runs.iter().filter(|r| !r.passed()).count();
    match (runs.len(), failed) {
        (0, _) => format!("{name}: no tests\n"),
        (_, 0) => String::new(),
        (_, n) => format!("{name}: {n} tests failed\n"),
    }
}
