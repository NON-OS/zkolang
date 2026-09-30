/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Text the lexer reported and left out, as the parser sees it: a gap between tokens. */

use super::super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Whether the lexer left out reported text between the previous token and the current
     * one. A token that is unexpected there is most likely unexpected because of that text,
     * so it is not reported a second time.
     */
    pub(in crate::compiler::syntax::parse) fn after_stray(&self) -> bool {
        let prev = self.pos.checked_sub(1).and_then(|i| self.tokens.get(i));
        let lo = prev.map_or(0, |t| t.span.hi);
        let hi = self.span().lo;
        let i = self.strays.partition_point(|s| s.lo < lo);
        self.strays.get(i).is_some_and(|s| s.hi <= hi)
    }
}

impl<'a> Parser<'a> {
    /**
     * Whether text the lexer found unterminated swallowed what the current token is missing:
     * a block comment ran to the end of the file, or the previous token is a string that
     * took the rest of its line and the current token is on a later line.
     */
    pub(in crate::compiler::syntax::parse) fn after_open_literal(&self) -> bool {
        if self.at(TokenKind::Eof) && self.comment_to_eof().is_some() {
            return true;
        }
        let Some(prev) = self.pos.checked_sub(1).and_then(|i| self.tokens.get(i)) else {
            return false;
        };
        prev.kind == TokenKind::Error
            && unterminated(self.text_of(*prev))
            && self.line_end_before(self.span()).is_some()
    }
}

/** Whether `s`, an error token's text, is a string, raw or plain, with no closing quote. */
fn unterminated(s: &str) -> bool {
    let body = s.trim_start_matches(['r', 'b']).trim_start_matches('#');
    let closed = body.len() >= 2 && body.trim_end_matches('#').ends_with('"');
    body.starts_with('"') && !closed
}
