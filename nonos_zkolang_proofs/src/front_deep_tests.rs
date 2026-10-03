/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Text nested far past the parser's budget parses on a small stack, in every construct. */

use crate::front_check::check;
use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

#[test]
fn deep_mixed_brackets_parse_without_overflow() {
    let run = |src: String| {
        std::thread::Builder::new()
            .stack_size(2 << 20)
            .spawn(move || check(&src))
    };
    let opens = [
        "(",
        "[",
        "{",
        "fn f() {",
        "let x = (",
        "match x { _ => (",
        "<",
        "-",
        "!",
        "(a || b && c == d | e ^ f & g << h + i * j as u8 * ",
        "if a { 0 } else if b { (",
    ];
    for open in opens {
        let src = format!("fn g() {{ {} }}", open.repeat(20_000));
        run(src).expect("spawn").join().expect("no overflow");
    }
    let types = format!(
        "fn h(a: {}u8{}) {{}}",
        "[(".repeat(5_000),
        ",); 2]".repeat(5_000)
    );
    run(types).expect("spawn").join().expect("no overflow");
}

#[test]
fn flat_chains_are_not_nesting() {
    let terms: Vec<String> = (0..10_000).map(|i| format!("a[{i}] * b[{i}]")).collect();
    let sum = format!("fn f() -> field {{ {} }}", terms.join(" + "));
    let branches: Vec<String> = (0..2_000)
        .map(|i| format!("if x == {i} {{ {i} }}"))
        .collect();
    let chain = format!(
        "fn g(x: u32) -> u32 {{ {} else {{ 0 }} }}",
        branches.join(" else ")
    );
    for src in [sum, chain] {
        let mut map = SourceMap::new();
        let id = map.add(String::from("f.zkl"), src.clone());
        let mut diags = Diagnostics::new();
        let lexed = lex(id, &src, &mut diags);
        parse_file(id, &src, &lexed, &mut diags, &mut 0);
        assert!(diags.items().is_empty(), "{:?}", diags.items().first());
    }
}
