/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Render one diagnostic over a text, for the rendering tests. */

use nonos_zkolang::compiler::diag::{render, Code, Diagnostic};
use nonos_zkolang::compiler::source::{SourceMap, Span};

/** Render one error over `text`, with a primary label and optional secondary labels. */
pub(crate) fn show(text: &str, primary: (u32, u32, &str), others: &[(u32, u32, &str)]) -> String {
    let mut map = SourceMap::new();
    let id = map.add(String::from("t.zkl"), String::from(text));
    let at = |(lo, hi, _): (u32, u32, &str)| Span::new(id, lo, hi);
    let mut d = Diagnostic::error(Code::UNEXPECTED_TOKEN, "m", at(primary), primary.2);
    for &o in others {
        d = d.with_label(at(o), o.2);
    }
    render(&map, &d)
}

/** The source row and the mark row of a rendered snippet with one line. */
pub(crate) fn rows(got: &str) -> (String, String) {
    let lines: Vec<&str> = got.lines().collect();
    (String::from(lines[3]), String::from(lines[4]))
}
