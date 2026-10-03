/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The invariants of section 6, checked on the slots of an input: a `bool` is 0 or 1, an
 * integer is in its type's range, a 64-bit integer's halves are each below `2^32`, and an
 * enum's tag names a variant whose fields hold theirs.
 */

use super::cx::Lower;
use super::layout::{elements, field_at, slots};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** Check that `vals`, the slots of a value of type `t`, hold its invariant. */
    pub(super) fn check_input(&mut self, t: TyId, vals: &[V]) {
        let types = &self.p.types;
        match types.kind(t).clone() {
            TyKind::Bool => {
                if let Some(&v) = vals.first() {
                    self.b.emit(Inst::AssertBool(v));
                }
            }
            TyKind::Int(i) if i.bits() > 32 => {
                for &v in vals {
                    self.require_below(v, 32);
                }
            }
            TyKind::Int(i) => {
                if let Some(&v) = vals.first() {
                    self.check_int(v, i);
                }
            }
            _ if types.record(t).is_some() => {
                let n = types.record(t).map_or(0, |ts| ts.len());
                for k in 0..n {
                    if let Some((at, e)) = field_at(types, t, k as u32) {
                        let n = slots(types, e);
                        self.check_input(e, vals.get(at..at + n).unwrap_or(&[]));
                    }
                }
            }
            TyKind::Array(..) => {
                let Some((e, size, n)) = elements(types, t) else {
                    return;
                };
                for k in 0..n {
                    self.check_input(e, vals.get(k * size..(k + 1) * size).unwrap_or(&[]));
                }
            }
            TyKind::Adt(_) if types.adt(t).is_some_and(|a| a.is_enum) => self.check_enum(t, vals),
            _ => {}
        }
    }
}
