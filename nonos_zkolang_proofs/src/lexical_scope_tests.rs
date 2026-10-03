/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Names resolve innermost first. A parameter or block local shadows a program constant
 * of its name, so a user constant no longer collides with a library function's parameter,
 * and a scalar in an inner scope, a loop variable included, hides a table or array.
 */

use nonos_zkolang::{
    compile_source, compile_source_unoptimized, evaluate, expand_with_stdlib, CompileError,
};

/** The outputs of both builds, which must agree, or the error both report. */
fn outputs(src: &str, public: &[u64]) -> Result<Vec<u64>, CompileError> {
    let run = |ops: Vec<_>| evaluate(&ops, public, &[]).expect("run");
    let opt = compile_source(src).map(run);
    assert_eq!(opt, compile_source_unoptimized(src).map(run), "`{src}`");
    opt
}

#[test]
fn a_parameter_or_local_shadows_a_constant() {
    let param = "const N = 5;\nfn f(N) = N * 2;\ninput x;\noutput f(x) + N;";
    assert_eq!(outputs(param, &[7]), Ok(vec![19]));
    let local = "const N = 5;\ninput x;\noutput { let N = x; N } + N;";
    assert_eq!(outputs(local, &[7]), Ok(vec![12]));
    let arr = "const N = 5;\nfn f(N) = N[0];\nlet a = [3, 4];\noutput f(a) + N;";
    assert_eq!(outputs(arr, &[]), Ok(vec![8]));
    let bare = "const N = 5;\nfn f(N) = N;\nlet a = [3, 4];\noutput f(a);";
    assert_eq!(outputs(bare, &[]), Err(CompileError::ArrayNotScalar));
}

#[test]
fn a_user_constant_may_share_a_library_parameter_name() {
    let src = expand_with_stdlib("include \"math.zkl\";\nconst a = 5;\ninput y;\noutput y * a;")
        .expect("expand");
    assert_eq!(outputs(&src, &[2]), Ok(vec![10]));
}

#[test]
fn an_inner_scalar_hides_a_table_or_array() {
    let at = |src: &str, pat: &str| src.find(pat).expect("index") + 1;
    let param = "const T = [7, 8];\nfn f(T) = T[0];\ninput x;\noutput f(x);";
    let not_indexable = |src| Err(CompileError::NotIndexable { at: at(src, "T[") });
    assert_eq!(outputs(param, &[1]), not_indexable(param));
    let array_param = "const T = [7, 8];\nfn f(T) = T[1];\nlet a = [1, 2];\noutput f(a);";
    assert_eq!(outputs(array_param, &[]), Ok(vec![2]));
    let table = "const T = [7, 8];\nfor T in 0..2 { output T[0]; }";
    assert_eq!(outputs(table, &[]), not_indexable(table));
    let array = "input x;\nlet v = [x, x + 1];\nfor v in 0..2 { output v[1]; }";
    let v_at = array.rfind("v[").expect("index") + 1;
    assert_eq!(
        outputs(array, &[5]),
        Err(CompileError::NotIndexable { at: v_at })
    );
    let counter = "input x;\nlet v = [x, x + 1];\nfor v in 0..2 { output v; }";
    assert_eq!(outputs(counter, &[5]), Ok(vec![0, 1]));
}
