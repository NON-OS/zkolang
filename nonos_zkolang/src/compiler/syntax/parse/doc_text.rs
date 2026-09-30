/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The text of one doc comment, without its delimiters. */

use alloc::string::String;
use alloc::vec::Vec;

/**
 * The text of a doc comment. A line comment loses its three leading marks and one space.
 * A block comment loses its three opening and two closing marks, and each of its lines the
 * indentation and one star of gutter with a space after it; blank first and last lines
 * are dropped.
 */
pub(super) fn doc_text(text: &str) -> String {
    if let Some(body) = text.strip_prefix("/*") {
        let body = body.get(1..).unwrap_or("");
        let body = body.strip_suffix("*/").unwrap_or(body);
        /* Every line ending an editor shows ends a line of the text (spec section 2.1). */
        let body = body.replace("\r\n", "\n").replace('\r', "\n");
        let mut lines: Vec<&str> = body
            .lines()
            .map(|l| {
                let l = l.trim_start();
                let l = l.strip_prefix('*').unwrap_or(l);
                l.strip_prefix(' ').unwrap_or(l).trim_end()
            })
            .collect();
        while lines.first().is_some_and(|l| l.is_empty()) {
            lines.remove(0);
        }
        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        return lines.join("\n");
    }
    let body = text.get(3..).unwrap_or("");
    String::from(body.strip_prefix(' ').unwrap_or(body).trim_end())
}
