/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The `for` loop over a range, an array, or an enumerated array. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::syntax::ast::{Expr, ExprKind, ForIter};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * `for pat in lo..hi { .. }`, `for pat in array { .. }`, or
     * `for (i, x) in array.enumerate() { .. }`.
     */
    pub(super) fn for_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        let pat = self.pattern_no_alt()?;
        if !self.eat_kw(Keyword::In) {
            return Err(self.unexpected("`in`"));
        }
        let iter = self.restricted(true, |p| {
            let first = p.expr()?;
            let inclusive = match p.kind() {
                TokenKind::DotDot => false,
                TokenKind::DotDotEq => true,
                _ => {
                    let enumerate = matches!(
                        &first.kind,
                        ExprKind::MethodCall { method, args, generics: None, .. }
                            if method.name == "enumerate" && args.is_empty()
                    );
                    if enumerate {
                        if let ExprKind::MethodCall { receiver, .. } = first.kind {
                            return Ok(ForIter::Enumerate(receiver));
                        }
                        return Err(Reported);
                    }
                    return Ok(ForIter::Array(Box::new(first)));
                }
            };
            p.bump();
            let hi = p.expr()?;
            Ok(ForIter::Range {
                lo: Box::new(first),
                hi: Box::new(hi),
                inclusive,
            })
        })?;
        let body = self.block()?;
        let span = start.to(self.prev_span());
        Ok(self.mk(
            ExprKind::For {
                pat,
                iter,
                body: Box::new(body),
            },
            span,
        ))
    }
}
