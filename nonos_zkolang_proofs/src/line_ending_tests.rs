/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every line ending an editor displays ends a line for the compiler. A comment used to
 * run to the next `\n` only, so in a file with lone carriage returns it swallowed the
 * rest of the program, a false assertion included, and the empty program proved. A
 * byte-order mark at the start of a file was refused as an unexpected character.
 */

use nonos_zkolang::{compile_source, evaluate, expand_includes};

#[test]
fn a_lone_carriage_return_ends_a_comment() {
    let src = "// header\routput 1;\rassert 1 == 2;\r";
    let ops = compile_source(src).expect("compile");
    assert!(
        evaluate(&ops, &[], &[]).is_err(),
        "the false assertion was dropped"
    );
    let expanded = expand_includes(src, &mut |_| None).expect("expand");
    let ops = compile_source(&expanded).expect("compile");
    assert!(evaluate(&ops, &[], &[]).is_err());
    let honest = "// header\rinput x;\routput x + 1;\r";
    let ops = compile_source(honest).expect("compile");
    assert_eq!(evaluate(&ops, &[4], &[]).expect("run"), vec![5]);
}

#[test]
fn an_include_on_a_carriage_return_line_is_spliced() {
    let lib = |p: &str| (p == "lib.zkl").then(|| String::from("fn inc(a) = a + 1;"));
    let src = "include \"lib.zkl\";\rinput x;\routput inc(x);\r";
    let expanded = expand_includes(src, &mut |p| lib(p)).expect("expand");
    let ops = compile_source(&expanded).expect("compile");
    assert_eq!(evaluate(&ops, &[4], &[]).expect("run"), vec![5]);
}

#[test]
fn a_leading_byte_order_mark_is_skipped() {
    let ops = compile_source("\u{feff}input x;\noutput x;\n").expect("compile");
    assert_eq!(evaluate(&ops, &[9], &[]).expect("run"), vec![9]);
    /* An included file saved with a mark splices in without it. */
    let lib = |p: &str| (p == "lib.zkl").then(|| String::from("\u{feff}fn inc(a) = a + 1;\n"));
    let src = "\u{feff}include \"lib.zkl\";\ninput x;\noutput inc(x);\n";
    let expanded = expand_includes(src, &mut |p| lib(p)).expect("expand");
    let ops = compile_source(&expanded).expect("compile");
    assert_eq!(evaluate(&ops, &[4], &[]).expect("run"), vec![5]);
    /* A mark anywhere but the start is still refused. */
    assert!(compile_source("input x;\n\u{feff}output x;\n").is_err());
}
