/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Attributes: `#[...]` before an item or field, and `#![...]` at the start of a module. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::Attr;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `#[name]`, `#[name = lit]`, `#[name(args)]`, or the same with `#!`. */
    pub(super) fn attr(&mut self, inner: bool) -> PResult<Attr> {
        let start = self.expect(TokenKind::Pound)?.span;
        if inner {
            self.expect(TokenKind::Bang)?;
        }
        if !self.at(TokenKind::LBracket) {
            let e = self.unexpected("`[`");
            /* A run of `#`, as `###`, is one mistake. */
            while self.at(TokenKind::Pound) {
                self.bump();
            }
            return Err(e);
        }
        self.bump();
        let name = self.ident()?;
        let mut args = None;
        let mut value = None;
        if self.eat(TokenKind::Eq) {
            value = Some(self.attr_lit()?);
        } else if self.eat(TokenKind::LParen) {
            args = Some(self.decl_list(TokenKind::RParen, |p| p.attr_arg())?);
        }
        self.expect(TokenKind::RBracket)?;
        Ok(Attr {
            name,
            args,
            value,
            inner,
            span: start.to(self.prev_span()),
        })
    }

    /**
     * An attribute, or `None` after reporting an error in it and skipping to its `]`, so
     * that the attributes and doc comments around it and the item after it are kept.
     */
    pub(super) fn attr_or_skip(&mut self, inner: bool) -> Option<Attr> {
        let from = self.pos;
        match self.attr(inner) {
            Ok(a) => Some(a),
            Err(_) => {
                self.pay_owed(from);
                None
            }
        }
    }
}
