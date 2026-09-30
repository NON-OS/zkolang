/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Module declarations: `mod name;` for a file, or an inline body in braces. */

use alloc::string::String;
use alloc::vec::Vec;

use super::parser::{PResult, Parser, Reported};
use super::recovery::starts_item;
use crate::compiler::syntax::ast::{Attr, Item, ModDecl};
use crate::compiler::syntax::token::TokenKind;

/** An inline module's documentation, inner attributes and items. */
type ModBody = (Option<String>, Vec<Attr>, Vec<Item>);

impl<'a> Parser<'a> {
    pub(super) fn mod_decl(&mut self) -> PResult<ModDecl> {
        self.bump();
        let name = match self.ident() {
            Ok(name) => name,
            Err(e) => {
                /* The items' own errors are independent of the name's: report them. */
                if self.skip_to_brace() {
                    let _ = self.mod_body();
                }
                return Err(e);
            }
        };
        if !self.at(TokenKind::Semi) && !self.at(TokenKind::LBrace) {
            /* An item after the name is the next one: the `;` before it is missing. */
            let e = self.report_unexpected("`;` or `{`", true);
            if !starts_item(self.kind()) && self.skip_to_brace() {
                let _ = self.mod_body();
            }
            return Err(e);
        }
        if self.eat(TokenKind::Semi) {
            return Ok(ModDecl {
                name,
                body: None,
                inner_attrs: Vec::new(),
                inner_doc: None,
                file: None,
            });
        }
        let (inner_doc, inner_attrs, items) = self.mod_body()?;
        Ok(ModDecl {
            name,
            body: Some(items),
            inner_attrs,
            inner_doc,
            file: None,
        })
    }

    /** An inline module's body, from its `{`, which is current, through its `}`. */
    fn mod_body(&mut self) -> PResult<ModBody> {
        let open = self.bump();
        let (inner_doc, inner_attrs) = self.inner_doc_and_attrs(open.span.hi);
        let items = self.nested(|p| Ok(p.items(true)))?;
        if !self.at(TokenKind::RBrace) {
            self.report_unclosed(open.span, "module");
            return Err(Reported);
        }
        self.bump();
        Ok((inner_doc, inner_attrs, items))
    }
}
