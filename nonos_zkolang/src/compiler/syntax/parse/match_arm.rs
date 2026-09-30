/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One arm of a `match`, and recovery from an arm that fails to parse. */

use super::block_like::block_like;
use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::Arm;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** One arm, `pattern [if guard] => body`, and the comma after it. */
    pub(super) fn arm(&mut self) -> PResult<Arm> {
        {
            let start = self.span();
            let pat = self.pattern()?;
            let guard = if self.eat_kw(Keyword::If) {
                Some(self.expr()?)
            } else {
                None
            };
            self.expect(TokenKind::FatArrow)?;
            /* A block-like body ends as a block-like statement does. */
            let body = if self.at_block_like() {
                let e = self.nested(|p| p.primary())?;
                self.after_block_like(e)?
            } else {
                self.expr()?
            };
            /* After a block-like body, as after a block statement, the comma may be left out. */
            let braced = block_like(&body);
            let span = start.to(body.span);
            if !self.eat(TokenKind::Comma) && !braced && !self.at(TokenKind::RBrace) {
                /* Reported; the next arm is read from here. */
                self.unexpected("`,` between match arms");
            }
            Ok(Arm {
                pat,
                guard,
                span,
                body,
            })
        }
    }

    /** Skip the rest of an arm that failed to parse, through the `,` after it. */
    pub(super) fn recover_arm(&mut self, from: usize) {
        self.pay_owed(from);
        self.skip_until(&[TokenKind::Comma]);
        if !self.eat(TokenKind::Comma) && self.pos == from && !self.at(TokenKind::RBrace) {
            self.bump();
        }
    }
}
