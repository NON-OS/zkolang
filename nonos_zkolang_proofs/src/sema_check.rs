/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Check a program and run its functions with the reference interpreter, for tests. */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::interp::{Failure, Interp, Value};
use nonos_zkolang::compiler::sema::check;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;
use nonos_zkolang::compiler::tir::{FnId, TProgram};

/** The checked program `src`, and the codes of every diagnostic. */
pub(crate) fn checked(src: &str) -> (TProgram, Vec<&'static str>) {
    let mut map = SourceMap::new();
    let id = map.add(String::from("t.zkl"), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    let ast = parse_file(id, src, &lexed, &mut diags, &mut 0);
    let (program, more) = check(&ast);
    diags.extend(more);
    (program, diags.items().iter().map(|d| d.code.0).collect())
}

/** Run the function `name` of `src`, which must check without errors, on `args`. */
pub(crate) fn run(src: &str, name: &str, args: Vec<Value>) -> Result<Value, Failure> {
    let (program, codes) = checked(src);
    assert!(codes.iter().all(|c| c.starts_with('W')), "{codes:?}");
    let f = program
        .fns
        .iter()
        .position(|f| f.name == name)
        .expect("no such function");
    let span = program.fns[f].span;
    Interp::new(&program, 100_000_000).call(FnId(f as u32), args, span)
}

/** An integer value. */
pub(crate) fn int(v: i128) -> Value {
    Value::Int(v)
}
