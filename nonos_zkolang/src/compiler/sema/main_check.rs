/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `main` (section 12.1): its parameters are the program's inputs, so each is a value,
 * not a `&mut` place (E0901). Their labels are checked with secret flow (E0601).
 */

use super::cx::Sema;
use super::defs::Defs;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Param, TypeKind};

impl<'a> Sema<'a> {
    /** Report each `&mut` parameter of the root module's `main`. */
    pub(super) fn check_main(&mut self) {
        let root = self.defs.modules.get(&Defs::ROOT);
        let Some(def) = root.and_then(|m| m.names.get("main")).map(|b| b.def) else {
            return;
        };
        let Some(info) = self
            .fn_of
            .get(&def)
            .and_then(|f| self.fns.get(f.0 as usize))
        else {
            return;
        };
        for p in &info.decl.params {
            let at = match p {
                Param::Typed { ty, .. } if matches!(ty.kind, TypeKind::RefMut(_)) => ty.span,
                Param::SelfParam {
                    by_ref_mut: true,
                    span,
                } => *span,
                _ => continue,
            };
            let d = Diagnostic::error(
                Code::BAD_MAIN,
                "`main` takes its inputs by value",
                at,
                "a `&mut` parameter",
            )
            .with_help("an input is a value the prover supplies; return what `main` computes");
            self.diags.push(d);
        }
    }
}
