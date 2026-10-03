/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A program is valid or not whether or not it is optimized. Folding ran before names and
 * shapes were checked, so the optimized build accepted an undefined name dropped by
 * `* 0` or an untaken branch, an array or tuple let through scalar arithmetic by `* 1`,
 * `+ 0` or a constant select, and an index by a `let`-bound constant.
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, CompileError};

/** The error both builds report, which must be the same. */
fn error(src: &str) -> CompileError {
    let opt = compile_source(src).expect_err("the optimized build accepted it");
    assert_eq!(
        Some(opt.clone()),
        compile_source_unoptimized(src).err(),
        "`{src}`"
    );
    opt
}

#[test]
fn folding_admits_nothing_the_language_refuses() {
    let unknown = |n: &str| CompileError::UnknownVariable { name: n.into() };
    assert_eq!(error("input x;\noutput nosuch * 0;"), unknown("nosuch"));
    assert_eq!(
        error("input x;\noutput if 1 { x } else { nosuch };"),
        unknown("nosuch")
    );
    let array = "input x;\nlet v = [x, x] * 1;\noutput v[1];";
    assert_eq!(error(array), CompileError::ArrayNotScalar);
    assert_eq!(
        error("let w = [1, 2] + 0;\noutput w[0];"),
        CompileError::ArrayNotScalar
    );
    let tuple = "input x;\nlet (p, q) = if 1 { (x, 2) } else { 0 };\noutput p + q;";
    assert!(matches!(error(tuple), CompileError::TupleNotScalar));
    let index = "const T = [10, 20, 30];\nlet k = 1;\noutput T[k];";
    assert_eq!(error(index), CompileError::NonConstantIndex);
}

#[test]
fn the_optimizer_may_still_save_registers() {
    let names: Vec<String> = (0..40).map(|i| format!("a{i}")).collect();
    let lets: String = names.iter().map(|n| format!("let {n} = 1;\n")).collect();
    let src = format!("{lets}output {};", names.join(" + "));
    let unopt = compile_source_unoptimized(&src).err();
    assert_eq!(unopt, Some(CompileError::TooManyRegisters));
    assert!(compile_source(&src).is_ok());
}
