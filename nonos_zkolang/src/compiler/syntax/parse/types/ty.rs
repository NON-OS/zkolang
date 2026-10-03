/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types: at most one qualifier, `public` or `secret`, then a bare type (spec section 3). */

use alloc::boxed::Box;

use super::super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Label, Type, TypeKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A type, possibly qualified. */
    pub(in crate::compiler::syntax::parse) fn ty(&mut self) -> PResult<Type> {
        self.nested(|p| p.ty_inner())
    }

    /** The type of a function parameter: the one place `&mut T` may stand. */
    pub(in crate::compiler::syntax::parse) fn param_ty(&mut self) -> PResult<Type> {
        self.ref_mut_ok = true;
        let ty = self.ty();
        self.ref_mut_ok = false;
        ty
    }

    fn ty_inner(&mut self) -> PResult<Type> {
        let start = self.span();
        let ref_mut_ok = core::mem::take(&mut self.ref_mut_ok);
        let Some(label) = self.label() else {
            return self.bare_ty(ref_mut_ok);
        };
        self.bump();
        if self.label().is_some() {
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "a type takes one qualifier",
                self.span(),
                "a second qualifier",
            )
            .with_help("write `public T` or `secret T`");
            self.diags.push(d);
            self.bump();
        }
        let inner = self.bare_ty(ref_mut_ok)?;
        let span = start.to(self.prev_span());
        let kind = TypeKind::Labelled(label, Box::new(inner));
        Ok(Type {
            id: self.id(),
            kind,
            span,
        })
    }

    /** The qualifier the current token is, if it is one. */
    fn label(&self) -> Option<Label> {
        match self.kind() {
            TokenKind::Kw(Keyword::Public) => Some(Label::Public),
            TokenKind::Kw(Keyword::Secret) => Some(Label::Secret),
            _ => None,
        }
    }
}
