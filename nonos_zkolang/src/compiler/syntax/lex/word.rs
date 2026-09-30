/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Words: the wildcard, keywords, reserved words and identifiers. */

use alloc::format;

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
    let textual_include = w == "include" && text[end..].trim_start().starts_with('"');
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

/** What to write instead of a reserved word that names a Rust feature. */
fn instead(w: &str) -> Option<&'static str> {
    Some(match w {
        "loop" => "every loop is bounded: write `while cond limit N { ... }` or `for i in a..b`",
        "static" => "write a constant with `const`",
        "trait" => "functions on a type go in an `impl Type { ... }` block",
        "ref" => "a binding pattern binds by value; drop `ref`",
        "where" => "there are no bounds to state: generic functions are checked per use",
        _ => return None,
    })
}
