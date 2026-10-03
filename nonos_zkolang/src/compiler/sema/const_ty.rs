/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The type of a constant item, lowered on first use, and the record of its state. */

use super::cx::{Sema, State};
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::tir::{ConstId, TConst};

impl<'a> Sema<'a> {
    /** The type of constant `c`. */
    pub(crate) fn const_ty(&mut self, c: ConstId) -> TyId {
        let Some(info) = self.consts.get(c.0 as usize) else {
            return Types::ERROR;
        };
        if let Some(t) = info.ty {
            return t;
        }
        let (m, decl) = (info.module, info.decl);
        let (t, _) = self.lower_ty(m, &decl.ty);
        if let Some(info) = self.consts.get_mut(c.0 as usize) {
            info.ty = Some(t);
        }
        t
    }

    /** Record constant `c`'s state; a failure stays one. */
    pub(super) fn set_const(&mut self, c: ConstId, s: State<(TConst, Value)>) {
        if let Some(info) = self.consts.get_mut(c.0 as usize) {
            if !matches!((&info.state, &s), (State::Failed, State::Done(_))) {
                info.state = s;
            }
        }
    }
}
