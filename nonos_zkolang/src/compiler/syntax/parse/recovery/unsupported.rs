/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Forms a Rust programmer writes that the language does not have, or writes without a
 * part it needs: `if let`, and `limit` with no bound. Each is reported once, with what to
 * write instead, and skipped whole.
 */

use super::super::parser::{Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Report and skip `if let pat = e { .. } else ..`, the `let` current. */
    pub(in crate::compiler::syntax::parse) fn if_let(&mut self) -> Reported {
        let d = Diagnostic::error(
            Code::UNEXPECTED_TOKEN,
            "`if let` is not part of the language",
            self.span(),
            "a pattern test",
        )
        .with_help("test the value with `match`, which checks that every case is handled");
        self.diags.push(d);
        loop {
            if !self.skip_to_brace() {
                return Reported;
            }
            self.skip_until(&[TokenKind::RBrace]);
            if !self.eat_kw(Keyword::Else) {
                return Reported;
            }
        }
    }

    /**
     * Whether the `{` after `limit` opens the loop's body rather than a bound: its group is
     * not followed by another `{`.
     */
    pub(in crate::compiler::syntax::parse) fn limit_without_bound(&self) -> bool {
        if !self.at(TokenKind::LBrace) {
            return false;
        }
        let mut depth: usize = 0;
        for (i, t) in self.tokens.iter().enumerate().skip(self.pos) {
            match t.kind {
                TokenKind::LBrace => depth += 1,
                TokenKind::RBrace if depth <= 1 => {
                    let next = self.tokens.get(i + 1).map(|t| t.kind);
                    return next != Some(TokenKind::LBrace);
                }
                TokenKind::RBrace => depth -= 1,
                TokenKind::Eof => return false,
                _ => {}
            }
        }
        false
    }
}
