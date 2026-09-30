/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The type forms other than parenthesized ones: labels, primitives, arrays, references, paths. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use super::paths::PathMode;
use super::types::int_keyword;
use crate::compiler::syntax::ast::{Label, TypeKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;
use crate::compiler::syntax::IntTy;

impl<'a> Parser<'a> {
    /** The kind of a type that does not open with `(`. */
    pub(super) fn ty_kind(&mut self) -> PResult<TypeKind> {
        let kind = match self.kind() {
            TokenKind::Kw(Keyword::Public) | TokenKind::Kw(Keyword::Secret) => {
                let label = if self.at_kw(Keyword::Public) {
                    Label::Public
                } else {
                    Label::Secret
                };
                self.bump();
                TypeKind::Labelled(label, Box::new(self.ty()?))
            }
            TokenKind::Kw(Keyword::Field) => {
                self.bump();
                TypeKind::Field
            }
            TokenKind::Kw(Keyword::Bool) => {
                self.bump();
                TypeKind::Bool
            }
            TokenKind::Kw(k) if int_keyword(k).is_some() => {
                self.bump();
                TypeKind::Int(int_keyword(k).unwrap_or(IntTy::U8))
            }
            TokenKind::LBracket => {
                self.bump();
                let elem = self.ty()?;
                self.expect(TokenKind::Semi)?;
                let len = self.const_arg()?;
                self.expect(TokenKind::RBracket)?;
                TypeKind::Array(Box::new(elem), len)
            }
            TokenKind::Amp => {
                self.bump();
                if !self.eat_kw(Keyword::Mut) {
                    return Err(self.unexpected("`mut`: the only reference type is `&mut T`"));
                }
                TypeKind::RefMut(Box::new(self.ty()?))
            }
            TokenKind::AmpAmp => return Err(self.unexpected("a type")),
            TokenKind::Kw(Keyword::SelfType) if self.peek(1) != TokenKind::ColonColon => {
                self.bump();
                TypeKind::SelfType
            }
            _ if self.at_path_start() => TypeKind::Path(self.path(PathMode::Type)?),
            _ => return Err(self.unexpected("a type")),
        };
        Ok(kind)
    }
}
