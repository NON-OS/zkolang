/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Generic argument lists, `<arg, ...>`, including the split of a `>>`, `>=` or `>>=` whose
 * first `>` closes the list.
 */

use alloc::vec::Vec;

use super::super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::GenericArg;
use crate::compiler::syntax::token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /** `<arg, ...>`, the opening `<` current. */
    pub(in crate::compiler::syntax::parse) fn generic_args(&mut self) -> PResult<Vec<GenericArg>> {
        let open = self.expect(TokenKind::Lt)?;
        if self.at_generic_close() && !self.after_stray() {
            self.empty_generics(open.span.to(self.span()));
        }
        /* Each argument is a type or constant, which spends its own nesting level. */
        let mut args = Vec::new();
        while !self.at_generic_close() {
            args.push(self.generic_arg()?);
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.close_generics()?;
        Ok(args)
    }

    fn generic_arg(&mut self) -> PResult<GenericArg> {
        if self.at(TokenKind::Int) || self.at(TokenKind::LBrace) || self.at(TokenKind::Minus) {
            return Ok(GenericArg::Const(self.const_arg()?));
        }
        Ok(GenericArg::Type(self.ty()?))
    }

    /**
     * Whether the current token closes a generic argument list, possibly as the first half
     * of a `>>`, `>=` or `>>=`.
     */
    fn at_generic_close(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::Gt | TokenKind::Shr | TokenKind::Ge | TokenKind::ShrEq
        )
    }

    /** Consume one `>`, splitting a `>>`, `>=` or `>>=` and leaving its rest current. */
    fn close_generics(&mut self) -> PResult<()> {
        let t = self.tok();
        let rest = match t.kind {
            TokenKind::Gt => {
                self.bump();
                return Ok(());
            }
            TokenKind::Shr => TokenKind::Gt,
            TokenKind::Ge => TokenKind::Eq,
            TokenKind::ShrEq => TokenKind::Ge,
            _ => return Err(self.unexpected("`>`")),
        };
        self.bump();
        let lo = t.span.lo.saturating_add(1);
        let mut span = t.span;
        span.lo = lo.min(t.span.hi);
        self.split = Some(Token { kind: rest, span });
        Ok(())
    }
}
