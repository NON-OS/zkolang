/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Import trees, the body of a `use` item. */

use super::parser::{PResult, Parser};
use super::types::PathMode;
use crate::compiler::syntax::ast::UseTree;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * `use tree;`, the `use` current. After an error in the tree the rest of the item is
     * skipped through its `;`, so a word after `::` that begins an item, as in `use a::fn;`,
     * is not taken for the next item.
     */
    pub(super) fn use_item(&mut self) -> PResult<UseTree> {
        self.bump();
        let tree = match self.use_tree() {
            Ok(tree) => tree,
            Err(e) => {
                self.skip_elem(TokenKind::Semi);
                self.eat(TokenKind::Semi);
                return Err(e);
            }
        };
        self.expect(TokenKind::Semi)?;
        Ok(tree)
    }

    /** A `use` tree: `a::b`, `a::b as c`, `a::*`, or `a::{b, c::d}`. */
    pub(super) fn use_tree(&mut self) -> PResult<UseTree> {
        let start = self.span();
        if !self.at_path_start() {
            return Err(self.unexpected("a path"));
        }
        let prefix = self.path(PathMode::Use)?;
        if self.at(TokenKind::ColonColon) {
            self.bump();
            if self.eat(TokenKind::Star) {
                return Ok(UseTree::Glob {
                    prefix,
                    span: start.to(self.prev_span()),
                });
            }
            self.expect(TokenKind::LBrace)?;
            let trees = self.nested(|p| p.decl_list(TokenKind::RBrace, |p| p.use_tree()))?;
            return Ok(UseTree::Nested {
                prefix,
                trees,
                span: start.to(self.prev_span()),
            });
        }
        let alias = if self.eat_kw(Keyword::As) {
            Some(self.ident()?)
        } else {
            None
        };
        Ok(UseTree::Single {
            path: prefix,
            alias,
            span: start.to(self.prev_span()),
        })
    }
}
