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
 * Classify a word: the wildcard, a keyword, a reserved word (an error, lexed as an
 * identifier so parsing continues), or an identifier.
 */
pub(super) fn word(w: &str, span: Span, diags: &mut Diagnostics) -> TokenKind {
    if w == "_" {
        return TokenKind::Underscore;
    }
    if let Some(k) = Keyword::from_word(w) {
        return TokenKind::Kw(k);
    }
    /*
     * `include` is reserved too, but the parser reports it where it can say what replaces
     * it, so the lexer lets it through.
     */
    if w != "include" && RESERVED.contains(&w) {
        diags.push(Diagnostic::error(
            Code::RESERVED_WORD,
            format!("`{w}` is reserved"),
            span,
            "reserved for a future edition",
        ));
    }
    TokenKind::Ident
}
