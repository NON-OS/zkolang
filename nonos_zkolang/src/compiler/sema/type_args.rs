/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Generic arguments written on a path (section 5.5): one per parameter, a type or a
 * constant `usize` as it is (E0701), writing no `secret` or `public` (section 5.4).
 */

use alloc::format;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{GenericArg, GenericParam, TypeKind};

impl<'a> Sema<'a> {
    /**
     * The arguments `given` for `params`, written in module `m` at `at` on the item named
     * `name`; `None` once a mistake is reported.
     */
    pub(crate) fn type_args(
        &mut self,
        m: DefId,
        (params, name): (&'a [GenericParam], &str),
        given: &'a [GenericArg],
        at: Span,
    ) -> Option<Vec<GenArg>> {
        if params.len() != given.len() {
            let (n, got) = (params.len(), given.len());
            let what = match n {
                0 => format!("`{name}` takes no generic arguments"),
                1 => format!("`{name}` takes 1 generic argument, not {got}"),
                _ => format!("`{name}` takes {n} generic arguments, not {got}"),
            };
            let d = Diagnostic::error(Code::WRONG_GENERICS, what, at, "wrong number");
            self.diags.push(d);
            return None;
        }
        let mut out = Vec::with_capacity(given.len());
        for (p, g) in params.iter().zip(given) {
            let arg = match (p, g) {
                (GenericParam::Type(_), GenericArg::Type(t)) => {
                    let (ty, labels) = self.lower_ty(m, t);
                    if !labels.0.is_empty() {
                        self.wrong_arg::<()>(
                            t.span,
                            "a generic argument writes no `secret` or `public`",
                        );
                    }
                    GenArg::Type(ty)
                }
                (GenericParam::Const { .. }, GenericArg::Const(c)) => {
                    GenArg::Const(self.const_usize(m, c)?)
                }
                (GenericParam::Const { .. }, GenericArg::Type(t)) => match &t.kind {
                    TypeKind::Path(path) => {
                        let v = self.const_path(m, path)?;
                        GenArg::Const(u32::try_from(v).ok()?)
                    }
                    _ => return self.wrong_arg(t.span, "a constant `usize` stands here"),
                },
                (GenericParam::Type(_), GenericArg::Const(c)) => {
                    return self.wrong_arg(c.span(), "a type stands here")
                }
            };
            out.push(arg);
        }
        Some(out)
    }
}
