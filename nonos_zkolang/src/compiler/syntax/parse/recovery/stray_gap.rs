/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Text the lexer reported and left out, as the parser sees it: a gap between tokens. */

use super::super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     * Whether the lexer left out reported text between the previous token and the current
     * one. A token that is unexpected there is most likely unexpected because of that text,
     * so it is not reported a second time.
     */
    pub(in crate::compiler::syntax::parse) fn after_stray(&self) -> bool {
        let prev = self.pos.checked_sub(1).and_then(|i| self.tokens.get(i));
        let lo = prev.map_or(0, |t| t.span.hi);
        let hi = self.span().lo;
        let i = self.strays.partition_point(|s| s.lo < lo);
        self.strays.get(i).is_some_and(|s| s.hi <= hi)
    }
}
