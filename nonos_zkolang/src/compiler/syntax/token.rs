/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Tokens. A token is a kind and a span; an identifier's name and a literal's digits are
 * read back from the source through the span, so the token stream stays small.
 */

use crate::compiler::source::Span;

pub use super::token_kind::TokenKind;

/** A token and where it is. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
