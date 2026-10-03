/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the lexer produces for one file. */

use alloc::vec::Vec;

use super::comment::Comment;
use super::describe::space_like;
use crate::compiler::source::Span;
use crate::compiler::syntax::token::{Token, TokenKind};

/** A file's tokens and comments. */
#[derive(Clone, Debug, Default)]
pub struct Lexed {
    pub tokens: Vec<Token>,
    pub comments: Vec<Comment>,
    /** Text reported and left out of the token stream, in order. */
    pub strays: Vec<Span>,
}

impl Lexed {
    /**
     * Keep what was scanned at `span`: a token, or, for reported text that stands for
     * nothing, its span. A space look-alike stands where whitespace would, so it is not
     * kept: it hides no later error.
     */
    pub(super) fn keep(&mut self, kind: Option<TokenKind>, span: Span, text: &str) {
        match kind {
            Some(kind) => self.tokens.push(Token { kind, span }),
            None if space_like(text) => {}
            None => self.strays.push(span),
        }
    }
}
