/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constant arguments: an array length, a repeat count or a `limit` (section 3). Each is a
 * `usize` literal, a constant of type `usize`, or `{ expr }` over constants.
 */

use alloc::format;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::Types;
use crate::compiler::syntax::ast::ConstArg;
use crate::compiler::syntax::IntTy;

impl<'a> Sema<'a> {
    /** The value of the constant argument `a` in module `m`, a `usize`. */
    pub(crate) fn const_usize(&mut self, m: DefId, a: &'a ConstArg) -> Option<u32> {
        let usize_ty = Types::int(IntTy::Usize);
        let v = match a {
            ConstArg::Lit {
                value,
                suffix,
                span,
            } => {
                if let Some(s) = suffix.filter(|s| *s != IntTy::Usize) {
                    let d = Diagnostic::error(
                        Code::MISMATCHED_TYPES,
                        "mismatched types",
                        *span,
                        format!("expected `usize`, found `{}`", s.name()),
                    );
                    self.diags.push(d);
                    return None;
                }
                i128::from(*value)
            }
            ConstArg::Path(p) => self.const_path(m, p)?,
            ConstArg::Expr(e) => self.eval_expr(m, e, usize_ty)?.int(),
        };
        match u32::try_from(v) {
            Ok(v) => Some(v),
            Err(_) => {
                let d = Diagnostic::error(
                    Code::LITERAL_OUT_OF_RANGE,
                    "the value does not fit `usize`",
                    a.span(),
                    format!("{v}"),
                );
                self.diags.push(d);
                None
            }
        }
    }
}
