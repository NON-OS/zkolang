/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Binding a function's parameters. A parameter that is a name is that name's local; any
 * other pattern takes the argument in a local of its own and is taken apart from it. A
 * `&mut` parameter is a name, whose final value the caller reads.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::cx::Sig;
use crate::compiler::sema::ty::Types;
use crate::compiler::syntax::ast::{Param, PatKind};
use crate::compiler::tir::{Labels, TParam};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Bind the parameters `params`, whose types `sig` gives. */
    pub(crate) fn params(&mut self, params: &'a [Param], sig: &Sig) -> Vec<TParam> {
        let mut out = Vec::with_capacity(params.len());
        for (i, p) in params.iter().enumerate() {
            let (ty, labels, by_ref) =
                sig.params
                    .get(i)
                    .cloned()
                    .unwrap_or((Types::ERROR, Labels::default(), false));
            let Param::Typed { pat, .. } = p else {
                if let Param::SelfParam { span, .. } = p {
                    out.push(self.named_param("self", (ty, labels, by_ref), by_ref, *span));
                }
                continue;
            };
            if let PatKind::Bind { name, mutable } = &pat.kind {
                let mutable = *mutable || by_ref;
                out.push(self.named_param(&name.name, (ty, labels, by_ref), mutable, name.span));
                continue;
            }
            if by_ref {
                let d = Diagnostic::error(
                    Code::PATTERN_MISMATCH,
                    "a `&mut` parameter is a name",
                    pat.span,
                    "a pattern for a place",
                )
                .with_help("name the parameter, and take it apart inside the body");
                self.sema.diags.push(d);
            }
            let local = self.declare("_", ty, false, labels.clone(), pat.span);
            let tpat = self.bind_pat(pat, ty, &labels, &mut Vec::new());
            out.push(TParam {
                local,
                pat: tpat,
                by_ref: false,
            });
        }
        out
    }
}
