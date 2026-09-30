/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bare types: every form of type but the qualifier. */

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Type, TypeKind};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A bare type; `&mut T` is allowed when `ref_mut_ok`. */
    pub(super) fn bare_ty(&mut self, ref_mut_ok: bool) -> PResult<Type> {
        let start = self.span();
        let kind = if self.at(TokenKind::LParen) {
            self.paren_ty()?
        } else {
            self.ty_kind(ref_mut_ok)?
        };
        let span = start.to(self.prev_span());
        Ok(Type {
            id: self.id(),
            kind,
            span,
        })
    }

    /** `()`, or a tuple type. The grammar has no parenthesized type, so `(T)` is reported. */
    fn paren_ty(&mut self) -> PResult<TypeKind> {
        let open = self.bump().span;
        if self.eat(TokenKind::RParen) {
            return Ok(TypeKind::Unit);
        }
        let first = self.ty()?;
        if self.at(TokenKind::RParen) {
            let close = self.bump().span;
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "parentheses around a type",
                open.to(close),
                "a type in parentheses",
            )
            .with_help("write `(T,)` for a one-element tuple, or leave the parentheses out");
            self.diags.push(d);
            return Ok(first.kind);
        }
        let mut elems = alloc::vec![first];
        while self.eat(TokenKind::Comma) {
            if self.at(TokenKind::RParen) {
                break;
            }
            elems.push(self.ty()?);
        }
        self.expect_list_end(TokenKind::RParen)?;
        Ok(TypeKind::Tuple(elems))
    }
}
