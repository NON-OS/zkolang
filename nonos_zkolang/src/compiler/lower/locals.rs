/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The locals of the call being lowered. */

use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::ssa::V;
use crate::compiler::tir::LocalId;

impl<'p> Lower<'p> {
    /** Set local `l` of the current call to `vals`. */
    pub(super) fn set_local(&mut self, l: LocalId, vals: Vec<V>) {
        if let Some(slot) = self.frame().and_then(|f| f.locals.get_mut(l.0 as usize)) {
            *slot = vals;
        }
    }

    /** The slots of local `l` of the current call. */
    pub(super) fn local(&mut self, l: LocalId) -> Vec<V> {
        self.frame()
            .and_then(|f| f.locals.get(l.0 as usize))
            .cloned()
            .unwrap_or_default()
    }
}
