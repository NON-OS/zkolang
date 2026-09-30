/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Impl blocks: a self type and the functions defined on it. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::ImplDecl;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn impl_decl(&mut self) -> PResult<ImplDecl> {
        self.bump();
        let generics = self.generic_params()?;
        let self_ty = self.ty()?;
        let open = self.expect(TokenKind::LBrace)?;
        let mut items = Vec::new();
        while !self.at(TokenKind::RBrace) {
            if self.at(TokenKind::Eof) {
                self.report_unclosed(open.span, "impl block");
                return Err(super::parser::Reported);
            }
            if self.at_outer_item() {
                self.report_unclosed(open.span, "impl block");
                return Ok(ImplDecl {
                    generics,
                    self_ty,
                    items,
                });
            }
            if self.at_include() {
                self.skip_include();
                continue;
            }
            if self.at_non_fn_item() {
                continue;
            }
            let before = self.pos;
            match self.impl_item() {
                Ok(item) => items.push(item),
                Err(_) => {
                    self.recover_item(before);
                    if self.pos == before {
                        self.bump();
                    }
                }
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(ImplDecl {
            generics,
            self_ty,
            items,
        })
    }
}
