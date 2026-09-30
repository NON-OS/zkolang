/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Impl blocks: a self type and the functions defined on it. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{GenericParam, ImplDecl, Type};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn impl_decl(&mut self) -> PResult<ImplDecl> {
        self.bump();
        let (generics, self_ty) = match self.impl_header() {
            Ok(h) => h,
            Err(e) => {
                /* The functions' own errors are independent of the header's: report them. */
                if self.skip_to_brace() {
                    let open = self.bump().span;
                    let _ = self.impl_members(open);
                }
                return Err(e);
            }
        };
        let open = self.bump().span;
        let items = self.impl_members(open)?;
        Ok(ImplDecl {
            generics,
            self_ty,
            items,
        })
    }

    /** The generic parameters and the self type, up to the `{`, which is left current. */
    fn impl_header(&mut self) -> PResult<(Vec<GenericParam>, Type)> {
        let generics = self.generic_params()?;
        let self_ty = self.ty()?;
        if self.at(TokenKind::LBrace) {
            return Ok((generics, self_ty));
        }
        if self.at_kw(Keyword::For) {
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "`impl .. for` is not part of the language",
                self.span(),
                "a trait implementation",
            )
            .with_help("the language has no traits: an impl block defines functions on its type");
            self.diags.push(d);
            return Err(Reported);
        }
        Err(self.unexpected("`{`"))
    }
}
