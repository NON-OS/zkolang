/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Literals and constants as slots: an integer of at most 32 bits is `x mod p`, a 64-bit
 * one the two 32-bit halves of its two's-complement pattern, a `field` element itself.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::layout::{elements, field_at};
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::ssa::V;
use crate::compiler::tir::TLit;

impl<'p> Lower<'p> {
    /** The slots of the literal `l` of type `t`. */
    pub(super) fn lit(&mut self, l: TLit, t: TyId) -> Vec<V> {
        match l {
            TLit::Unit => Vec::new(),
            TLit::Bool(b) => alloc::vec![self.b.konst(i128::from(b))],
            TLit::Int(v) => self.int_slots(v, t),
        }
    }

    /** The slots of the integer or `field` value `v` of type `t`. */
    pub(super) fn int_slots(&mut self, v: i128, t: TyId) -> Vec<V> {
        match self.p.types.kind(t) {
            TyKind::Int(i) if i.bits() > 32 => {
                let pattern = v.rem_euclid(1i128 << 64);
                let (lo, hi) = (pattern & 0xFFFF_FFFF, pattern >> 32);
                alloc::vec![self.b.konst(lo), self.b.konst(hi)]
            }
            _ => alloc::vec![self.b.konst(v)],
        }
    }

    /** The slots of the constant value `v` of type `t`. */
    pub(super) fn value_slots(&mut self, v: &Value, t: TyId) -> Vec<V> {
        let types = &self.p.types;
        match (v, types.kind(t).clone()) {
            (Value::Bool(b), _) => alloc::vec![self.b.konst(i128::from(*b))],
            (Value::Int(x), _) => self.int_slots(*x, t),
            (Value::Field(x), _) => alloc::vec![self.b.konst(i128::from(*x))],
            (Value::Tuple(parts), _) if types.record(t).is_some() => {
                let tys: Vec<TyId> = (0..parts.len())
                    .filter_map(|k| field_at(types, t, k as u32).map(|(_, e)| e))
                    .collect();
                parts
                    .iter()
                    .zip(tys)
                    .flat_map(|(p, e)| self.value_slots(p, e))
                    .collect()
            }
            (Value::Array(parts), TyKind::Array(..)) => {
                let e = elements(types, t).map(|(e, _, _)| e);
                match e {
                    Some(e) => parts.iter().flat_map(|p| self.value_slots(p, e)).collect(),
                    None => Vec::new(),
                }
            }
            _ => Vec::new(),
        }
    }
}
