/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Recovery costs time linear in the text it skips, however many brackets an error leaves
 * open, and reports a bounded number of errors for them.
 */

use std::time::{Duration, Instant};

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

/** The number of diagnostics for `src`. */
fn reports(src: &str) -> usize {
    let mut map = SourceMap::new();
    let id = map.add(String::from("f.zkl"), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    parse_file(id, src, &lexed, &mut diags, &mut 0);
    diags.items().len()
}

#[test]
fn recovery_is_linear_in_unclosed_brackets() {
    let inputs = [
        (
            format!("fn f() {{ let x = (1 1 {}1; }}", "(1,".repeat(100_000)),
            1,
        ),
        ("mod m {\n".repeat(50_000), 130),
        (
            format!("fn f() {{\n{}}}", "    if x {\n".repeat(50_000)),
            130,
        ),
        ("#[\n".repeat(100_000), 100_000),
    ];
    for (src, most) in inputs {
        let t = Instant::now();
        let n = reports(&src);
        let spent = t.elapsed();
        assert!(
            spent < Duration::from_secs(20),
            "{spent:?} for {}",
            &src[..20]
        );
        assert!(n <= most, "{n} reports for {}", &src[..20]);
    }
}
