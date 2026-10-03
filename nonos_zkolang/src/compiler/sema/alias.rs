/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Type aliases (section 5.3): each instance, for one list of generic arguments, lowered
 * once, and a cycle of them reported (E0206).
 */

use alloc::format;

use super::cx::{Sema, State};
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{GenArg, TyId, Types};
use crate::compiler::syntax::ast::ItemKind;
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type the alias `def`, which takes no generic arguments, stands for. */
    pub(crate) fn alias(&mut self, def: DefId) -> (TyId, Labels) {
        self.alias_ty(def, &[])
    }

    /**
     * The type the alias `def` stands for with `args`, and the labels it writes. The
     * caller has checked the arguments' number.
     */
    pub(crate) fn alias_ty(&mut self, def: DefId, args: &[GenArg]) -> (TyId, Labels) {
        let key = (def, args.to_vec());
        match self.aliases.get(&key) {
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
                self.aliases.insert(key, State::Failed);
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
        let Some(outer) = self.bind_generics(&a.generics, args) else {
            return (Types::ERROR, Labels::default());
        };
        self.aliases.insert(key.clone(), State::Checking);
        let out = self.lower_ty(module, &a.ty);
        self.generics = outer;
        if matches!(self.aliases.get(&key), Some(State::Checking)) {
            self.aliases.insert(key, State::Done(out.clone()));
        }
        out
    }
}
