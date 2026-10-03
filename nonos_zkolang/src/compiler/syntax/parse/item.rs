/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One module-level item: its documentation, attributes, visibility and declaration. */

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Item, ItemKind, Visibility};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** One item, or `None` for a construct that was reported and skipped. */
    pub(super) fn item(&mut self) -> PResult<Option<Item>> {
        let saved = self.item_indent;
        self.item_indent = self.line_indent(self.span().lo);
        self.unclosed_reported = false;
        /* A nesting error is reported once per top-level item. */
        if self.depth == 0 {
            self.nesting_reported = false;
        }
        let item = self.item_inner();
        self.item_indent = saved;
        item
    }

    fn item_inner(&mut self) -> PResult<Option<Item>> {
        let start = self.span();
        let (doc, attrs) = self.doc_and_attrs();
        let vis = if self.eat_kw(Keyword::Pub) {
            Visibility::Public
        } else {
            Visibility::Private
        };
        let kind = match self.kind() {
            TokenKind::Kw(Keyword::Fn) => ItemKind::Fn(self.fn_decl(false)?),
            TokenKind::Kw(Keyword::Const) if self.peek(1) == TokenKind::Kw(Keyword::Fn) => {
                self.bump();
                ItemKind::Fn(self.fn_decl(true)?)
            }
            TokenKind::Kw(Keyword::Const) => ItemKind::Const(self.const_decl()?),
            TokenKind::Kw(Keyword::Struct) => ItemKind::Struct(self.struct_decl()?),
            TokenKind::Kw(Keyword::Enum) => ItemKind::Enum(self.enum_decl()?),
            TokenKind::Kw(Keyword::Type) => ItemKind::TypeAlias(self.type_alias()?),
            TokenKind::Kw(Keyword::Mod) => ItemKind::Mod(self.mod_decl()?),
            TokenKind::Kw(Keyword::Use) => ItemKind::Use(self.use_item()?),
            TokenKind::Kw(Keyword::Impl) => ItemKind::Impl(self.impl_decl()?),
            _ if self.at_include() => {
                self.skip_include();
                return Ok(None);
            }
            _ => return Err(self.no_item()),
        };
        let span = start.to(self.prev_span());
        Ok(Some(Item {
            id: self.id(),
            attrs,
            vis,
            doc,
            kind,
            span,
        }))
    }
}
