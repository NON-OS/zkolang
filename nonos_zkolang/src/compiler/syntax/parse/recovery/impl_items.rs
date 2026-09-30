/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! An item other than a function in an impl block: reported once and skipped whole. */

use super::super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Report and skip an item other than a function, which an impl block cannot hold. */
    pub(in crate::compiler::syntax::parse) fn at_non_fn_item(&mut self) -> bool {
        let pubbed = self.at_kw(Keyword::Pub);
        let (k, next) = if pubbed {
            (self.peek(1), self.peek(2))
        } else {
            (self.kind(), self.peek(1))
        };
        let non_fn = match k {
            TokenKind::Kw(Keyword::Const) => next != TokenKind::Kw(Keyword::Fn),
            TokenKind::Kw(
                Keyword::Type
                | Keyword::Struct
                | Keyword::Enum
                | Keyword::Mod
                | Keyword::Use
                | Keyword::Impl,
            ) => true,
            _ => false,
        };
        if non_fn {
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "an impl block holds only functions",
                self.span(),
                "not a function",
            )
            .with_help("declare it at module level");
            self.diags.push(d);
            self.skip_item();
        }
        non_fn
    }
}
