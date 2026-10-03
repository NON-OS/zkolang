/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Conversions (section 7.8): `as`, which the checker allows only where every value fits,
 * and `checked_from` and `wrapping_from`, which may lose or refuse a value.
 */

use super::arith_field::to_field;
use super::bits::wrap;
use super::machine::Interp;
use super::{FailKind, Value};
use crate::compiler::sema::ty::{TyId, TyKind};

impl<'e> Interp<'e> {
    /** `v as to`; the checker has made sure `to` holds every value `v` may have. */
    pub(super) fn cast(&self, v: Value, to: TyId) -> Value {
        match self.env.types().kind(to) {
            TyKind::Field => Value::Field(to_field(v.int())),
            TyKind::Int(_) => Value::Int(v.int()),
            _ => v,
        }
    }

    /** `to::checked_from(v)`: the same integer, if `to` holds it. */
    pub(super) fn checked_from(&self, v: &Value, to: TyId) -> Result<Value, FailKind> {
        let x = v.int();
        match self.env.types().kind(to) {
            TyKind::Int(t) if t.contains(x) => Ok(Value::Int(x)),
            TyKind::Bool if x == 0 || x == 1 => Ok(Value::Bool(x == 1)),
            _ => Err(FailKind::CheckedFrom),
        }
    }

    /** `to::wrapping_from(v)`: the integer modulo `2^N`, as a value of `to`. */
    pub(super) fn wrapping_from(&self, v: &Value, to: TyId) -> Value {
        match self.env.types().kind(to) {
            TyKind::Int(t) => Value::Int(wrap(v.int(), *t)),
            _ => v.clone(),
        }
    }
}
