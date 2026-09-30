/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns that start with a path: tuple structs, structs, bindings and plain paths. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::types::PathMode;
use crate::compiler::syntax::ast::PatKind;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A pattern that starts with a path: `P(..)`, `P { .. }`, a binding, or a path. */
    pub(super) fn pattern_path(&mut self) -> PResult<PatKind> {
        let path = if self.at_primitive_path() {
            self.primitive_path()?
        } else {
            self.path(PathMode::Plain)?
        };
        if self.at(TokenKind::LParen) {
            self.bump();
            let mut elems = Vec::new();
            while !self.at(TokenKind::RParen) {
                elems.push(self.pattern()?);
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.expect_list_end(TokenKind::RParen)?;
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
}
