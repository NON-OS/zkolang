/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Arrays are scoped like every other binding. A function body used to see the caller's
 * arrays, ahead of its own scalar parameter of the same name, and a block's scalar local
 * did not hide an outer array: `v[0]` read an array the code in scope never bound.
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate, CompileError};

/** The compile result of both builds, which must agree on whether it is an error. */
fn build(src: &str) -> Result<Vec<nonos_zkolang::Op>, CompileError> {
    let opt = compile_source(src);
    let plain = compile_source_unoptimized(src);
    assert_eq!(opt.is_ok(), plain.is_ok(), "the builds disagree on `{src}`");
    opt
}

#[test]
fn a_function_sees_only_its_own_arrays() {
    let shadowed = "fn first(v) = v[0];\nlet v = [10, 20];\ninput x;\noutput first(x);";
    assert!(matches!(
        build(shadowed),
        Err(CompileError::NotIndexable { .. })
    ));
    let free_name = "fn f(z) = a[0] + z;\nlet a = [1, 2, 3];\noutput f(0);";
    assert!(
        build(free_name).is_err(),
        "a function read an array it was not given"
    );
    let param = "fn f(v) = v[0];\ninput a;\nlet v = [a + 7, 8];\noutput f(5);";
    assert!(matches!(
        build(param),
        Err(CompileError::NotIndexable { .. })
    ));
    /* An array passed in is still read through its parameter. */
    let passed = "fn first(v) = v[0];\nlet a = [10, 20];\noutput first(a);";
    let ops = build(passed).expect("compile");
    assert_eq!(evaluate(&ops, &[], &[]).expect("run"), vec![10]);
}

#[test]
fn a_block_local_hides_an_outer_array() {
    let src = "let v = [10, 20];\ninput x;\noutput v[1];\noutput { let v = x; v[0] };";
    assert!(matches!(build(src), Err(CompileError::NotIndexable { .. })));
    /* After the block the array is visible again, and a local aliasing it keeps it alive. */
    let after = "let v = [10, 20];\ninput x;\noutput { let t = v[0]; let v = x; t + v };\n\
                 let y = x * 2;\noutput v[1] + y;";
    let ops = build(after).expect("compile");
    assert_eq!(evaluate(&ops, &[7], &[]).expect("run"), vec![17, 34]);
}
