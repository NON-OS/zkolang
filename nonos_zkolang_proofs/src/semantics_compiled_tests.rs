/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The semantics tests compiled: every `#[test]` function under `semantics/`, lowered as the
 * entry of a program, compiled to machine code and run on the VM, must be accepted exactly
 * when it passes on the interpreter, rejected when it is marked `#[should_fail]`.
 */

use std::fs;

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::sema::check_tests;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

use crate::compile_run::run;
use crate::golden_corpus::repo_root;

#[test]
fn every_semantics_test_runs_compiled_as_it_does_interpreted() {
    let root = repo_root().join("nonos_zkolang_proofs/semantics");
    let mut files: Vec<_> = fs::read_dir(&root)
        .expect("semantics")
        .flatten()
        .map(|e| e.path())
        .collect();
    files.sort();
    let (mut problems, mut ran, mut unsupported) = (Vec::new(), 0, Vec::new());
    for path in files
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "zkl"))
    {
        let src = fs::read_to_string(path).expect("read");
        let name = path
            .file_name()
            .map_or(String::new(), |n| n.to_string_lossy().into_owned());
        let mut map = SourceMap::new();
        let id = map.add(name.clone(), src.clone());
        let mut diags = Diagnostics::new();
        let lexed = lex(id, &src, &mut diags);
        let ast = parse_file(id, &src, &lexed, &mut diags, &mut 0);
        let (program, _) = check_tests(&ast);
        for &(f, should_fail) in &program.tests {
            let test = program
                .fns
                .get(f.0 as usize)
                .map_or(String::new(), |b| b.name.clone());
            let compiled = match crate::compile_run::compile_test(&program, f) {
                Ok(c) => c,
                Err((true, what)) => {
                    unsupported.push(format!("{name}: {test}: {what}"));
                    continue;
                }
                Err((false, what)) => {
                    problems.push(format!("{name}: {test}: {what}"));
                    continue;
                }
            };
            ran += 1;
            let accepted = run(&compiled, &[], 0).is_some();
            if accepted == should_fail {
                problems.push(format!(
                    "{name}: {test}: accepted {accepted}, marked should_fail {should_fail}"
                ));
            }
        }
    }
    assert!(ran > 30, "only {ran} tests compiled");
    assert!(problems.is_empty(), "{problems:#?}");
    assert!(unsupported.is_empty(), "{unsupported:#?}");
}
