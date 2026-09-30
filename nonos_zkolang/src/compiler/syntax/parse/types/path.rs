/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A path, with the generic arguments its segments take in the given mode. */

use alloc::vec::Vec;

use super::super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::syntax::ast::{Path, PathRoot, PathSegment};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * A path. The first segment may be `crate`, `super`, `self` or `Self`. Generic
     * arguments, where the mode takes them, follow the whole path and are kept on its last
     * segment.
     */
    pub(in crate::compiler::syntax::parse) fn path(&mut self, mode: PathMode) -> PResult<Path> {
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
            segments.push(PathSegment {
                ident,
                generics: None,
            });
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
            segments.push(PathSegment {
                ident,
                generics: None,
            });
        }
        if self.at(TokenKind::ColonColon) {
            self.after_colons(mode)?;
        }
        self.path_generics(&mut segments, mode)?;
        let span = start.to(self.prev_span());
        Ok(Path {
            root,
            segments,
            span,
        })
    }
}
