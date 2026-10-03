/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constants as slots (section 6): a scalar as its literal is, a tuple, struct or array its
 * parts' in order, and an enum value its tag, its variant's fields, then zeros.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::layout::slots;
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::V;

impl<'p> Lower<'p> {
    /** The slots of the constant value `v` of type `t`. */
    pub(super) fn value_slots(&mut self, v: &Value, t: TyId) -> Vec<V> {
        match v {
            Value::Unit => Vec::new(),
            Value::Bool(b) => alloc::vec![self.b.konst(i128::from(*b))],
            Value::Int(x) => self.int_slots(*x, t),
            Value::Field(x) => alloc::vec![self.b.konst(i128::from(*x))],
            Value::Tuple(parts) | Value::Array(parts) => {
                let tys = self.p.types.parts(t, None);
                let each = parts.iter().zip(tys);
                each.flat_map(|(p, e)| self.value_slots(p, e)).collect()
            }
            Value::Variant(tag, parts) => {
                let mut out = alloc::vec![self.b.konst(i128::from(*tag))];
                for (p, e) in parts.iter().zip(self.p.types.parts(t, Some(*tag))) {
                    out.extend(self.value_slots(p, e));
                }
                let zero = self.b.konst(0);
                out.resize(slots(&self.p.types, t), zero);
                out
            }
        }
    }
}
