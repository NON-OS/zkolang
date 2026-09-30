/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Paths over a primitive type name, as `u8::MAX` and `field::from_le_bits`. */

use super::super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::syntax::ast::{Ident, Path, PathRoot, PathSegment};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;
use alloc::string::String;

impl<'a> Parser<'a> {
    /**
     * A one-segment path over a primitive type keyword, for `u8::MAX` and
     * `field::from_le_bits`.
     */
    pub(in crate::compiler::syntax::parse) fn primitive_path(&mut self) -> PResult<Path> {
        let t = self.bump();
        let ident = Ident {
            name: String::from(self.text_of(t)),
            span: t.span,
        };
        let mut segments = alloc::vec![PathSegment {
            ident,
            generics: None
        }];
        while self.at(TokenKind::ColonColon) && self.peek(1) == TokenKind::Ident {
            self.bump();
            let ident = self.ident()?;
            segments.push(PathSegment {
                ident,
                generics: None,
            });
        }
        self.path_generics(&mut segments, PathMode::Expr)?;
        Ok(Path {
            root: PathRoot::Plain,
            segments,
            span: t.span.to(self.prev_span()),
        })
    }

    /** Whether a path over a primitive type name starts here, as `u8::MAX`. */
    pub(in crate::compiler::syntax::parse) fn at_primitive_path(&self) -> bool {
        let TokenKind::Kw(k) = self.kind() else {
            return false;
        };
        let primitive = matches!(
            k,
            Keyword::Field
                | Keyword::Bool
                | Keyword::U8
                | Keyword::U16
                | Keyword::U32
                | Keyword::U64
                | Keyword::I8
                | Keyword::I16
                | Keyword::I32
                | Keyword::I64
                | Keyword::Usize
        );
        primitive && self.peek(1) == TokenKind::ColonColon
    }
}
