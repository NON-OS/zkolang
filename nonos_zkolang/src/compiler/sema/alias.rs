/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Type aliases (section 5.3): each lowered once, and a cycle of them reported (E0206). */

use alloc::format;

use super::cx::{Sema, State};
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::ItemKind;
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type the alias `def` stands for, and the labels it writes. */
    pub(crate) fn alias(&mut self, def: DefId) -> (TyId, Labels) {
        match self.aliases.get(&def) {
            Some(State::Done(x)) => return x.clone(),
            Some(State::Checking) => {
                if let Some(d) = self.defs.get(def) {
                    let message = format!("the type alias `{}` stands for itself", d.name);
                    self.diags.push(Diagnostic::error(
                        Code::ALIAS_CYCLE,
                        message,
                        d.span,
                        "defined in terms of itself",
                    ));
                }
                self.aliases.insert(def, State::Failed);
                return (Types::ERROR, Labels::default());
            }
            Some(State::Failed) => return (Types::ERROR, Labels::default()),
            _ => {}
        }
        let found = self.defs.get(def).and_then(|d| Some((d.parent?, d.item?)));
        let Some((module, item)) = found else {
            return (Types::ERROR, Labels::default());
        };
        let ItemKind::TypeAlias(a) = &item.kind else {
            return (Types::ERROR, Labels::default());
        };
        self.aliases.insert(def, State::Checking);
        let out = self.lower_ty(module, &a.ty);
        if matches!(self.aliases.get(&def), Some(State::Checking)) {
            self.aliases.insert(def, State::Done(out.clone()));
        }
        out
    }
}
