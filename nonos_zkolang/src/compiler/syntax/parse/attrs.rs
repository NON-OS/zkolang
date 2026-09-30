/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Attributes: `#[...]` before an item or field, and `#![...]` at the start of a module. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Attr, AttrArg};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `#[name]`, `#[name = lit]`, `#[name(args)]`, or the same with `#!`. */
    pub(super) fn attr(&mut self, inner: bool) -> PResult<Attr> {
        let start = self.expect(TokenKind::Pound)?.span;
        if inner {
            self.expect(TokenKind::Bang)?;
        }
        self.expect(TokenKind::LBracket)?;
        let name = self.ident()?;
        let mut args = None;
        let mut value = None;
        if self.eat(TokenKind::Eq) {
            value = Some(self.attr_lit()?);
        } else if self.eat(TokenKind::LParen) {
            let mut list = Vec::new();
            while !self.at(TokenKind::RParen) {
                if self.at(TokenKind::Ident) {
                    let n = self.ident()?;
                    let v = if self.eat(TokenKind::Eq) {
                        Some(self.attr_lit()?)
                    } else {
                        None
                    };
                    list.push(AttrArg::Named { name: n, value: v });
                } else {
                    list.push(AttrArg::Lit(self.attr_lit()?));
                }
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen)?;
            args = Some(list);
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
}
