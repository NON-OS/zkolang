/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `S { f }`: the field `f` given the variable or constant of the same name. */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefKind;
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{Ident, PathRoot};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The value `name` stands for in a field shorthand, of type `want` if given. */
    pub(super) fn shorthand(&mut self, name: &Ident, want: Option<TyId>) -> TExpr {
        let at = name.span;
        let found = match self.lookup(&name.name) {
            Some(l) => {
                self.read_local(l);
                let ty = self.locals.get(l.0 as usize).map(|x| x.ty);
                ty.map(|ty| TExpr {
                    kind: TExprKind::Local(l),
                    ty,
                    span: at,
                })
            }
            None => {
                let def =
                    self.sema
                        .defs
                        .resolve(self.module, PathRoot::Plain, &[name.name.as_str()]);
                let c = def
                    .ok()
                    .filter(|&d| self.sema.defs.get(d).map(|x| x.kind) == Some(DefKind::Const));
                c.and_then(|d| self.sema.const_of.get(&d).copied())
                    .map(|c| {
                        let ty = self.sema.const_ty(c);
                        TExpr {
                            kind: TExprKind::Const(c),
                            ty,
                            span: at,
                        }
                    })
            }
        };
        let Some(e) = found else {
            let what = format!(
                "no variable or constant `{}` for the field of that name",
                name.name
            );
            let d = Diagnostic::error(Code::UNRESOLVED_NAME, what, at, "not found");
            self.sema
                .diags
                .push(d.with_help(format!("write `{}: value`", name.name)));
            return self.error(at);
        };
        if let Some(w) = want {
            self.coerce(&e, w);
        }
        e
    }
}
