/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Function parameters: `self`, `&mut self`, or a pattern and its type. */

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Param;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn param(&mut self) -> PResult<Param> {
        let start = self.span();
        if self.at_kw(Keyword::SelfValue) {
            self.bump();
            return Ok(Param::SelfParam {
                by_ref_mut: false,
                span: start,
            });
        }
        if self.at(TokenKind::Amp)
            && self.peek(1) == TokenKind::Kw(Keyword::Mut)
            && self.peek(2) == TokenKind::Kw(Keyword::SelfValue)
        {
            self.bump();
            self.bump();
            self.bump();
            return Ok(Param::SelfParam {
                by_ref_mut: true,
                span: start.to(self.prev_span()),
            });
        }
        if self.at(TokenKind::Amp) && self.peek(1) == TokenKind::Kw(Keyword::SelfValue) {
            self.bump();
            let at = start.to(self.bump().span);
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "`&self` is not a receiver",
                at,
                "a shared reference",
            )
            .with_help("take `self` to read the value, or `&mut self` to change it");
            self.diags.push(d);
            return Ok(Param::SelfParam {
                by_ref_mut: false,
                span: at,
            });
        }
        let pat = self.pattern_no_alt()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.param_ty()?;
        Ok(Param::Typed { pat, ty })
    }
}
