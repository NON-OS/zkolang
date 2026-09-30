/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Literal patterns, and the ranges `lo..=hi` between two of them. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Lit, PatKind, Pattern};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A literal pattern, or a range `lo..=hi` of two, starting at `start`. */
    pub(super) fn pattern_lit_or_range(&mut self, start: Span) -> PResult<Pattern> {
        let lo = self.literal_pattern(true)?;
        if !self.at(TokenKind::DotDotEq) {
            return Ok(lo);
        }
        if matches!(
            lo.kind,
            PatKind::Lit {
                lit: Lit::Bool { .. },
                ..
            }
        ) {
            /* A range runs between integers (spec section 3, `literal_pat`). */
            return Err(self.unexpected("`=>` or `|`: a range pattern runs between integers"));
        }
        self.bump();
        let hi = self.literal_pattern(false)?;
        let kind = PatKind::Range {
            lo: Box::new(lo),
            hi: Box::new(hi),
        };
        let span = start.to(self.prev_span());
        Ok(Pattern {
            id: self.id(),
            kind,
            span,
        })
    }
}
