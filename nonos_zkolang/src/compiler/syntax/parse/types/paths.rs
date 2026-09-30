/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Paths, generic argument lists and constant arguments.
 *
 * In a type a path takes its generic arguments directly, `Pair<u8>`; in an expression they
 * need the turbofish, `size_of::<u8>()`, because there `<` is less-than. A generic
 * argument that is a bare path may name a type or a constant; the parser records it as a
 * type and name resolution reads it as whichever it names.
 */

use super::super::parser::Parser;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

/** Where a path is being parsed, which decides how it takes generic arguments. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::compiler::syntax::parse) enum PathMode {
    /** In a type: `Name<args>`. */
    Type,
    /** In an expression or pattern: `Name::<args>`. */
    Expr,
    /** In a `use` or `mod` path: no generic arguments. */
    Plain,
}

impl<'a> Parser<'a> {
    /** Whether the current token can begin a path. */
    pub(in crate::compiler::syntax::parse) fn at_path_start(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::Ident
                | TokenKind::Kw(Keyword::Crate)
                | TokenKind::Kw(Keyword::Super)
                | TokenKind::Kw(Keyword::SelfValue)
                | TokenKind::Kw(Keyword::SelfType)
        )
    }
}
