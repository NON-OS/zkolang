/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Type, TypeKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;
use crate::compiler::syntax::IntTy;

impl<'a> Parser<'a> {
    /** A type, possibly labelled `public` or `secret`. */
    pub(super) fn ty(&mut self) -> PResult<Type> {
        self.nested(|p| p.ty_inner())
    }

    fn ty_inner(&mut self) -> PResult<Type> {
        let start = self.span();
        let kind = match self.kind() {
            TokenKind::LParen => {
                self.bump();
                if self.eat(TokenKind::RParen) {
                    TypeKind::Unit
                } else {
                    let first = self.ty()?;
                    if self.eat(TokenKind::RParen) {
                        /* `(T)` is `T`. */
                        return Ok(first);
                    }
                    let mut elems = alloc::vec![first];
                    while self.eat(TokenKind::Comma) {
                        if self.at(TokenKind::RParen) {
                            break;
                        }
                        elems.push(self.ty()?);
                    }
                    self.expect_list_end(TokenKind::RParen)?;
                    TypeKind::Tuple(elems)
                }
            }
            _ => self.ty_kind()?,
        };
        let span = start.to(self.prev_span());
        Ok(Type {
            id: self.id(),
            kind,
            span,
        })
    }
}

/** The integer type an integer keyword names. */
pub(super) fn int_keyword(k: Keyword) -> Option<IntTy> {
    Some(match k {
        Keyword::U8 => IntTy::U8,
        Keyword::U16 => IntTy::U16,
        Keyword::U32 => IntTy::U32,
        Keyword::U64 => IntTy::U64,
        Keyword::I8 => IntTy::I8,
        Keyword::I16 => IntTy::I16,
        Keyword::I32 => IntTy::I32,
        Keyword::I64 => IntTy::I64,
        Keyword::Usize => IntTy::Usize,
        _ => return None,
    })
}
