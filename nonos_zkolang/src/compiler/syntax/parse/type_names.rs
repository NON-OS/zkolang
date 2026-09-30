/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The types keywords name, and the target of `as`, which takes only a type's name. */

use super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::syntax::ast::{Type, TypeKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;
use crate::compiler::syntax::IntTy;

impl<'a> Parser<'a> {
    /** The target of `as`: `field`, `bool`, an integer type, or a path without arguments. */
    pub(super) fn cast_ty(&mut self) -> PResult<Type> {
        let start = self.span();
        let kind = match self.kind() {
            TokenKind::Kw(Keyword::Field | Keyword::Bool) => self.ty_kind(false)?,
            TokenKind::Kw(k) if int_keyword(k).is_some() => self.ty_kind(false)?,
            _ if self.at_path_start() => TypeKind::Path(self.path(PathMode::Plain)?),
            _ => return Err(self.unexpected("a type to convert to")),
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
