/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `main`'s parameters read from the input slots (section 12.2): the public ones first,
 * then the secret ones, each slot checked against its type's invariant (section 12.3).
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use super::layout::slots;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::ast::Label;
use crate::compiler::tir::TFn;

impl<'p> Lower<'p> {
    /**
     * The slots of each parameter of `f`, read from the inputs; and how many slots are
     * public, and how many there are in all.
     */
    pub(super) fn read_params(&mut self, f: &TFn) -> (Vec<Vec<V>>, u16, u16) {
        let mut args: Vec<Vec<V>> = vec![Vec::new(); f.params.len()];
        let mut next: u16 = 0;
        let mut n_public = 0;
        for label in [Label::Public, Label::Secret] {
            for (i, param) in f.params.iter().enumerate() {
                let Some(local) = f.locals.get(param.local.0 as usize) else {
                    continue;
                };
                if local.labels.whole().unwrap_or(Label::Public) != label {
                    continue;
                }
                let n = slots(&self.p.types, local.ty);
                let vals: Vec<V> = (0..n)
                    .map(|k| self.b.emit(Inst::Input(next.saturating_add(k as u16))))
                    .collect();
                next = next.saturating_add(n as u16);
                self.check_input(local.ty, &vals);
                args[i] = vals;
            }
            if label == Label::Public {
                n_public = next;
            }
        }
        (args, n_public, next)
    }
}
