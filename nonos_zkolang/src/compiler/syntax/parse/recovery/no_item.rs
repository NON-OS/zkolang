/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The report of a missing item after what began one, such as `pub` or attributes. */

use super::super::parser::{Parser, Reported};
use crate::compiler::syntax::keyword::Keyword;

impl<'a> Parser<'a> {
    /**
     * Report that no item follows what an item began with, such as `pub` or attributes. A
     * stray token found there is reported here, and not again by the run it begins; a run
     * of `pub` is one mistake, and is skipped whole.
     */
    pub(in crate::compiler::syntax::parse) fn no_item(&mut self) -> Reported {
        let e = self
            .unexpected("an item: `fn`, `struct`, `enum`, `type`, `const`, `mod`, `use` or `impl`");
        if self.at_stray_between_items() {
            self.stray_reported = Some(self.pos);
        }
        while self.at_kw(Keyword::Pub) {
            self.bump();
        }
        e
    }
}
