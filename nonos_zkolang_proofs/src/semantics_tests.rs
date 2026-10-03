/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The semantics tests, written in zKølang: every `#[test]` function of every program under
 * `semantics/` must pass on the reference interpreter, and every program check cleanly.
 */

use std::fs;

use nonos_zkolang::compiler::diag::{render, Diagnostics};
use nonos_zkolang::compiler::interp::run_tests;
use nonos_zkolang::compiler::sema::check_tests;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

use crate::golden_corpus::repo_root;

/** The steps one test may take. */
const BUDGET: u64 = 10_000_000;

/** Check and run the tests of `src`; the problems found, and how many tests ran. */
fn run_file(name: &str, src: &str) -> (String, usize) {
    let mut map = SourceMap::new();
    let id = map.add(String::from(name), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    let ast = parse_file(id, src, &lexed, &mut diags, &mut 0);
    let (program, more) = check_tests(&mut map, &ast);
    diags.extend(more);
    if !diags.items().is_empty() {
        let shown: String = diags.items().iter().map(|d| render(&map, d)).collect();
        return (format!("{name} does not check cleanly:\n{shown}"), 0);
    }
    let mut problems = String::new();
    let runs = run_tests(&program, BUDGET);
    for run in runs.iter().filter(|r| !r.passed()) {
        let test = program
            .fns
            .get(run.f.0 as usize)
            .map_or("?", |f| f.name.as_str());
        problems.push_str(&format!("{name}: {test}: {:?}\n", run.outcome));
    }
    if runs.is_empty() {
        problems.push_str(&format!("{name} has no tests\n"));
    }
    (problems, runs.len())
}

#[test]
fn every_semantics_test_passes() {
    let root = repo_root().join("nonos_zkolang_proofs/semantics");
    let mut files: Vec<_> = fs::read_dir(&root)
        .expect("semantics directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "zkl"))
        .collect();
    files.sort();
    let (mut problems, mut ran) = (String::new(), 0);
    for path in &files {
        let src = fs::read_to_string(path).expect("read a semantics test");
        let name = path
            .file_name()
            .map_or(String::new(), |n| n.to_string_lossy().into_owned());
        let (p, n) = run_file(&name, &src);
        problems.push_str(&p);
        ran += n;
    }
    assert!(ran > 0, "no semantics tests ran under {}", root.display());
    assert!(problems.is_empty(), "{problems}");
}
