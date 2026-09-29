/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Names whose meaning the edition leaves ambiguous are refused rather than resolved one
 * way by the lowering and another by the optimizer. A scalar constant used to win over a
 * top-level `let` or input of the same name, a loop variable over a `let` in its own body,
 * and the first of two definitions of one function or constant was silently the one used.
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, CompileError, NameError};

/** The error both builds report, which must agree. */
fn error(src: &str) -> Option<CompileError> {
    let opt = compile_source(src).err();
    assert_eq!(
        opt,
        compile_source_unoptimized(src).err(),
        "builds disagree on `{src}`"
    );
    opt
}

fn named(kind: fn(String) -> NameError, name: &str) -> Option<CompileError> {
    Some(CompileError::Name(kind(String::from(name))))
}

#[test]
fn ambiguous_names_are_refused() {
    let dup = |n| NameError::Duplicate { name: n };
    let shadow = |n| NameError::ShadowsConstant { name: n };
    let loop_var = |n| NameError::ShadowsLoopVariable { name: n };
    let src = "fn f(x) = x + 1;\nfn f(x) = x + 100;\ninput x;\noutput f(x);";
    assert_eq!(error(src), named(dup, "f"));
    assert_eq!(
        error("const K = 1;\nconst K = 2;\noutput K;"),
        named(dup, "K")
    );
    assert_eq!(
        error("const N = 5;\ninput N;\noutput N;"),
        named(shadow, "N")
    );
    assert_eq!(
        error("const T = [1, 2];\nlet T = 3;\noutput T;"),
        named(shadow, "T")
    );
    let body = "input x;\nfor i in 0..2 { let i = x; output i; }";
    assert_eq!(error(body), named(loop_var, "i"));
    let nested = "input x;\nfor i in 0..2 { output { let i = x; i }; }";
    assert_eq!(error(nested), named(loop_var, "i"));
    /* A repeat identical to the first, as two includes of one library give, is harmless. */
    assert!(error("fn f(x) = x + 1;\nfn f(x) = x + 1;\ninput x;\noutput f(x);").is_none());
    /* Names that do not collide still compile, and a loop variable may reuse an outer name. */
    assert!(
        error("const N = 5;\nlet i = 7;\nfor i in 0..2 { output i + N; }\noutput i;").is_none()
    );
}
