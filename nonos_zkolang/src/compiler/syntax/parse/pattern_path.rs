/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns that start with a path: tuple structs, structs, bindings and plain paths. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::syntax::ast::{FieldPat, PatKind};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A pattern that starts with a path: `P(..)`, `P { .. }`, a binding, or a path. */
    pub(super) fn pattern_path(&mut self) -> PResult<PatKind> {
        let path = self.path(PathMode::Expr)?;
        if self.at(TokenKind::LParen) {
            self.bump();
            let mut elems = Vec::new();
            while !self.at(TokenKind::RParen) {
                elems.push(self.pattern()?);
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::RParen)?;
            Ok(PatKind::TupleStruct(path, elems))
        } else if self.at(TokenKind::LBrace) {
            self.bump();
            let (fields, rest) = self.field_patterns()?;
            Ok(PatKind::Struct { path, fields, rest })
        } else if let Some(name) = path.as_ident() {
            Ok(PatKind::Bind {
                name: name.clone(),
                mutable: false,
            })
        } else {
            Ok(PatKind::Path(path))
        }
    }

    /** The fields of a struct pattern after its `{`, and whether it ends in `..`. */
    fn field_patterns(&mut self) -> PResult<(Vec<FieldPat>, bool)> {
        let mut fields = Vec::new();
        let mut rest = false;
        while !self.at(TokenKind::RBrace) {
            if self.eat(TokenKind::DotDot) {
                rest = true;
                break;
            }
            let start = self.span();
            let name = self.ident()?;
            let pat = if self.eat(TokenKind::Colon) {
                Some(self.pattern()?)
            } else {
                None
            };
            fields.push(FieldPat {
                name,
                pat,
                span: start.to(self.prev_span()),
            });
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok((fields, rest))
    }
}
