/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Words: the wildcard, keywords, reserved words and identifiers. */

use alloc::format;

use super::reserved_help::instead;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::Span;
use crate::compiler::syntax::keyword::{Keyword, RESERVED};
use crate::compiler::syntax::token::TokenKind;

/**
 * Classify the word at bytes `start..end` of `text`: the wildcard, a keyword, a reserved
 * word (an error, lexed as an identifier so parsing continues), or an identifier.
 */
pub(super) fn word(
    text: &str,
    start: usize,
    end: usize,
    span: Span,
    diags: &mut Diagnostics,
) -> TokenKind {
    let w = &text[start..end];
    if w == "_" {
        return TokenKind::Wildcard;
    }
    if let Some(k) = Keyword::from_word(w) {
        return TokenKind::Kw(k);
    }
    /*
     * `include "file";` is the edition 2025 textual include, which the parser reports with
     * its replacement; every other use of the word is a reserved word used as a name.
     */
    let textual_include = w == "include" && next_is_string(&text[end..]);
    if RESERVED.contains(&w) && !textual_include {
        let d = Diagnostic::error(
            Code::RESERVED_WORD,
            format!("`{w}` is reserved"),
            span,
            "reserved for a future edition",
        );
        diags.push(match instead(w) {
            Some(h) => d.with_help(h),
            None => d,
        });
    }
    TokenKind::Ident
}

/** Whether the next token in `rest`, after whitespace and comments, is a string literal. */
fn next_is_string(mut rest: &str) -> bool {
    loop {
        rest = rest.trim_start();
        if let Some(r) = rest.strip_prefix("//") {
            rest = r.find(['\n', '\r']).map_or("", |i| &r[i..]);
        } else if let Some(r) = rest.strip_prefix("/*") {
            rest = r.find("*/").map_or("", |i| &r[i + 2..]);
        } else {
            return rest.starts_with('"');
        }
    }
}
