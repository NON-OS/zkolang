/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The error for an `as` that not every value survives, naming the explicit forms. */

use alloc::format;
use alloc::string::String;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report that `from as to` at `at` is not allowed. */
    pub(super) fn bad_cast(&mut self, from: TyId, to: TyId, at: Span) {
        let (shown_from, shown) = (self.show(from), self.show(to));
        let help = match self.kind(to) {
            TyKind::Int(_) => format!("`{shown}::checked_from(e)` fails on a value `{shown}` does not hold; `{shown}::wrapping_from(e)` keeps the low bits"),
            TyKind::Bool => String::from("`bool::checked_from(e)` fails unless the value is 0 or 1"),
            _ => String::from("no conversion between these types is defined"),
        };
        let d = Diagnostic::error(
            Code::INVALID_CAST,
            format!("`{shown_from}` does not convert to `{shown}` with `as`"),
            at,
            "not every value fits",
        )
        .with_help(help);
        self.sema.diags.push(d);
    }
}
