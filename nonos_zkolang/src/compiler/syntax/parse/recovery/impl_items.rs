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
    /**
     * Report and skip an item other than a function, which an impl block cannot hold,
     * with the attributes and `pub` before it.
     */
    pub(in crate::compiler::syntax::parse) fn at_non_fn_item(&mut self) -> bool {
        let Some((i, k, next)) = self.item_keyword_at() else {
            return false;
        };
        let non_fn = match k {
            TokenKind::Kw(Keyword::Const) => next != TokenKind::Kw(Keyword::Fn),
            TokenKind::Kw(Keyword::Fn) => false,
            _ => true,
        };
        if non_fn {
            let at = self.tokens.get(i).map_or(self.span(), |t| t.span);
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "an impl block holds only functions",
                at,
                "not a function",
            )
            .with_help("declare it at module level");
            self.diags.push(d);
            self.skip_item();
        }
        non_fn
    }
}
