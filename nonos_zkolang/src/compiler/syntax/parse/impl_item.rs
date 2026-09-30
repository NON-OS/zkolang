/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One function inside an impl block, with its documentation, attributes and visibility. */

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Item, ItemKind, Visibility};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A function inside an impl block. */
    pub(super) fn impl_item(&mut self) -> PResult<Item> {
        let saved = self.item_indent;
        self.item_indent = self.line_indent(self.span().lo);
        self.unclosed_reported = false;
        let item = self.impl_item_inner();
        self.item_indent = saved;
        item
    }

    fn impl_item_inner(&mut self) -> PResult<Item> {
        let start = self.span();
        let (doc, attrs) = self.doc_and_attrs()?;
        let vis = if self.eat_kw(Keyword::Pub) {
            Visibility::Public
        } else {
            Visibility::Private
        };
        let decl = match self.kind() {
            TokenKind::Kw(Keyword::Fn) => self.fn_decl(false)?,
            TokenKind::Kw(Keyword::Const) if self.peek(1) == TokenKind::Kw(Keyword::Fn) => {
                self.bump();
                self.fn_decl(true)?
            }
            _ => return Err(self.unexpected("`fn`: an impl block holds functions")),
        };
        let span: Span = start.to(self.prev_span());
        Ok(Item {
            id: self.id(),
            attrs,
            vis,
            doc,
            kind: ItemKind::Fn(decl),
            span,
        })
    }
}
