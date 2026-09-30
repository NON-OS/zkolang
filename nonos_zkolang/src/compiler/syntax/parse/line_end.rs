/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where a token missing at the end of a line belongs. */

use super::parser::Parser;
use crate::compiler::source::Span;

impl<'a> Parser<'a> {
    /**
     * The empty span just after the previous token, if a line ends between it and `at`.
     * A token missing there, such as a `;`, is reported at the end of the line it belongs
     * to rather than at the start of the next one.
     */
    pub(super) fn line_end_before(&self, at: Span) -> Option<Span> {
        let prev = self.tokens.get(self.pos.checked_sub(1)?)?.span;
        let gap = self.text.get(prev.hi as usize..at.lo as usize)?;
        gap.contains(['\n', '\r'])
            .then(|| Span::new(prev.file, prev.hi, prev.hi))
    }
}
