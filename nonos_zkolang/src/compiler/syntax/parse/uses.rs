/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Import trees, the body of a `use` item. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::types::PathMode;
use crate::compiler::syntax::ast::UseTree;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A `use` tree: `a::b`, `a::b as c`, `a::*`, or `a::{b, c::d}`. */
    pub(super) fn use_tree(&mut self) -> PResult<UseTree> {
        let start = self.span();
        if !self.at_path_start() {
            return Err(self.unexpected("a path"));
        }
        let prefix = self.path(PathMode::Plain)?;
        if self.at(TokenKind::ColonColon) {
            self.bump();
            if self.eat(TokenKind::Star) {
                return Ok(UseTree::Glob {
                    prefix,
                    span: start.to(self.prev_span()),
                });
            }
            self.expect(TokenKind::LBrace)?;
            let trees = self.nested(|p| {
                let mut trees = Vec::new();
                while !p.at(TokenKind::RBrace) {
                    trees.push(p.use_tree()?);
                    if !p.eat(TokenKind::Comma) {
                        break;
                    }
                }
                Ok(trees)
            })?;
            self.expect_list_end(TokenKind::RBrace)?;
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
