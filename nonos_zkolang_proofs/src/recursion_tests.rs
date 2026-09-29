/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every function is checked whether or not it is called. A recursive function used to be
 * reported as too many live values or an unnamed depth error, and not at all while nothing
 * called it; a repeated parameter silently took the last argument.
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, CompileError, NameError};

/** The error both builds report, which must agree. */
fn error(src: &str) -> Option<CompileError> {
    let opt = compile_source(src).err();
    assert_eq!(opt, compile_source_unoptimized(src).err(), "`{src}`");
    opt
}

fn recursive(name: &str) -> Option<CompileError> {
    let name = String::from(name);
    Some(CompileError::Name(NameError::Recursive { name }))
}

#[test]
fn a_recursive_function_is_named() {
    let fact = "fn fact(n) = if n == 0 { 1 } else { n * fact(n - 1) };\ninput x;\noutput fact(x);";
    assert_eq!(error(fact), recursive("fact"));
    let uncalled = "fn f(x) = f(x) + 1;\noutput 1;";
    assert_eq!(error(uncalled), recursive("f"));
    let mutual = "fn f(x) = g(x);\nfn g(x) = { let y = x * 2; f(y) };\noutput 1;";
    assert_eq!(error(mutual), recursive("f"));
    /* A call chain that never returns to its start is fine, up to the inlining depth. */
    let mut chain = String::from("fn f0(x) = x + 1;\n");
    for k in 1..200 {
        chain.push_str(&format!("fn f{k}(x) = f{}(x);\n", k - 1));
    }
    chain.push_str("fn g(x) = f199(x) + f5(x);\ninput x;\noutput g(x);");
    assert!(error(&chain).is_none());
}

#[test]
fn a_repeated_parameter_is_refused() {
    let src = "fn two(a, a) = a;\ninput p;\ninput q;\noutput two(p, q);";
    let name = String::from("a");
    assert_eq!(
        error(src),
        Some(CompileError::Name(NameError::Duplicate { name }))
    );
}
