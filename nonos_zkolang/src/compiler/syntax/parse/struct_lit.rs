/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Path expressions and struct literals, which both start with a path. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::types::PathMode;
use crate::compiler::syntax::ast::{Expr, ExprKind, FieldInit, Path};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A path expression, a struct literal, or a call's callee. */
    pub(super) fn path_expr(&mut self) -> PResult<Expr> {
        let path = self.path(PathMode::Expr)?;
        if self.at(TokenKind::LBrace) && (!self.no_struct || self.struct_literal_ahead()) {
            let in_head = self.no_struct;
            let e = self.struct_literal(path)?;
            if in_head {
                self.struct_in_head(e.span);
            }
            return Ok(e);
        }
        let span = path.span;
        Ok(self.mk(ExprKind::Path(path), span))
    }

    /** `Path { field: value, field, ... }`, the `{` current. */
    pub(super) fn struct_literal(&mut self, path: Path) -> PResult<Expr> {
        self.bump();
        let fields = self.restricted(false, |p| {
            let mut fields = Vec::new();
            while !p.at(TokenKind::RBrace) {
                if p.at(TokenKind::Eof) {
                    return Err(p.unexpected("`}`"));
                }
                let fstart = p.span();
                let name = p.ident()?;
                let value = if p.eat(TokenKind::Colon) {
                    Some(p.expr()?)
                } else {
                    None
                };
                fields.push(FieldInit {
                    name,
                    value,
                    span: fstart.to(p.prev_span()),
                });
                if !p.eat(TokenKind::Comma) {
                    break;
                }
            }
            p.expect_list_end(TokenKind::RBrace)?;
            Ok(fields)
        })?;
        let span = path.span.to(self.prev_span());
        Ok(self.mk(ExprKind::Struct { path, fields }, span))
    }
}
