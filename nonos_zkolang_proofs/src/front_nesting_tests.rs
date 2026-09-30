/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The nesting bound counts each level once, whatever the construct: 63 levels parse, 64
 * are an error, and neither overflows a two-megabyte stack in a debug build.
 */

use crate::front_lex_tests::parse;

/** Programs nesting one construct `n` levels deep. */
fn nests(n: usize) -> Vec<String> {
    vec![
        format!("fn f() {{ {}1{} }}", "{ ".repeat(n), " }".repeat(n)),
        format!("fn f() {{ {}1{} }}", "g(".repeat(n), ")".repeat(n)),
        format!("fn f() {{ a{}0{}; }}", "[b".repeat(n), "]".repeat(n)),
        format!(
            "fn f() {{ {}1{} }}",
            "if c { ".repeat(n),
            " } else { 0 }".repeat(n)
        ),
        format!(
            "fn f() {{ {}1{} }}",
            "match x { _ => ".repeat(n),
            " }".repeat(n)
        ),
        format!("fn f() {{ {}1 }}", "-".repeat(n)),
        format!("fn f(x: {}u8{}) {{}}", "A<".repeat(n), ">".repeat(n)),
        format!("fn f(x: {}u8{}) {{}}", "[".repeat(n), "; {1}]".repeat(n)),
        format!("fn f() {{ let {}a{} = x; }}", "(".repeat(n), ",)".repeat(n)),
    ]
}

#[test]
fn every_construct_nests_to_the_bound_and_no_further() {
    let run = std::thread::Builder::new().stack_size(2 << 20).spawn(|| {
        for src in nests(63) {
            assert!(parse(&src).1.is_empty(), "{src}");
        }
        for src in nests(64) {
            assert_eq!(parse(&src).1, vec!["E0102"], "{src}");
        }
    });
    run.expect("spawn").join().expect("no overflow");
}
