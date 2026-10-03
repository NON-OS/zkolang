/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The formatter's guard: a layout is kept only if it has the tokens of the text it came
 * from, each spelled the same, and its comments, each with the same lines once their
 * leading and trailing whitespace is left out.
 */

use alloc::vec::Vec;

use crate::compiler::source::Span;
use crate::compiler::syntax::lex::Lexed;

/** The text of `span` in `s`. */
fn text(s: &str, span: Span) -> &str {
    s.get(span.lo as usize..span.hi as usize).unwrap_or("")
}

/** The lines of `s`, each without its leading and trailing whitespace. */
fn lines(s: &str) -> Vec<&str> {
    s.lines().map(str::trim).collect()
}

/** Whether `b`, lexed as `lb`, has the tokens and comments of `a`, lexed as `la`. */
pub(super) fn same(a: &str, la: &Lexed, b: &str, lb: &Lexed) -> bool {
    let tokens = la.tokens.len() == lb.tokens.len()
        && (la.tokens.iter().zip(&lb.tokens))
            .all(|(x, y)| x.kind == y.kind && text(a, x.span) == text(b, y.span));
    let comments = la.comments.len() == lb.comments.len()
        && (la.comments.iter().zip(&lb.comments))
            .all(|(x, y)| x.kind == y.kind && lines(text(a, x.span)) == lines(text(b, y.span)));
    tokens && comments && lb.strays.is_empty()
}
