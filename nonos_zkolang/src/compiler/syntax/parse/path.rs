/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A path, with the generic arguments its segments take in the given mode. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::syntax::ast::{GenericArg, Path, PathRoot, PathSegment};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * A path. The first segment may be `crate`, `super`, `self` or `Self`; a primitive type
     * name may start an expression path such as `u8::MAX`, which the caller passes as
     * `first`.
     */
    pub(super) fn path(&mut self, mode: PathMode) -> PResult<Path> {
        let start = self.span();
        let mut segments = Vec::new();
        let root = match self.kind() {
            TokenKind::Kw(Keyword::Crate) => PathRoot::Crate,
            TokenKind::Kw(Keyword::Super) => PathRoot::Super,
            TokenKind::Kw(Keyword::SelfValue) => PathRoot::SelfModule,
            TokenKind::Kw(Keyword::SelfType) => PathRoot::SelfType,
            _ => PathRoot::Plain,
        };
        if root == PathRoot::Plain {
            let ident = self.ident()?;
            let generics = self.segment_generics(mode)?;
            segments.push(PathSegment { ident, generics });
        } else {
            self.bump();
        }
        /*
         * A rooted path may stand alone (`Self`, or `crate` before `::{...}` in a use); the
         * caller decides whether that is allowed where it is.
         */
        while self.at(TokenKind::ColonColon) && self.peek(1) == TokenKind::Ident {
            self.bump();
            let ident = self.ident()?;
            let generics = self.segment_generics(mode)?;
            segments.push(PathSegment { ident, generics });
        }
        let span = start.to(self.prev_span());
        Ok(Path {
            root,
            segments,
            span,
        })
    }

    /** The generic arguments after a segment, if the mode allows them here. */
    pub(super) fn segment_generics(&mut self, mode: PathMode) -> PResult<Option<Vec<GenericArg>>> {
        match mode {
            PathMode::Type if self.at(TokenKind::Lt) => Ok(Some(self.generic_args()?)),
            PathMode::Expr if self.at(TokenKind::ColonColon) && self.peek(1) == TokenKind::Lt => {
                self.bump();
                Ok(Some(self.generic_args()?))
            }
            _ => Ok(None),
        }
    }
}
