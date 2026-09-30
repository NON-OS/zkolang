/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The built-in methods and associated functions of section 18.3. */

use alloc::vec::Vec;

use super::arith_field::{field_inv, field_pow};
use super::arith_pow::int_pow;
use super::bits::{from_bits, to_bits, wrap};
use super::machine::{fail, Eval, Interp};
use super::{FailKind, Value};
use crate::compiler::sema::ty::TyKind;
use crate::compiler::tir::{Builtin, TExpr};

impl<'e> Interp<'e> {
    /** Run the built-in `b` on `args`, for the call `e`. */
    pub(super) fn builtin(&mut self, b: Builtin, args: &[TExpr], e: &TExpr) -> Eval {
        let values = self.eval_all(args)?;
        let first = values.first().cloned().unwrap_or(Value::Unit);
        let second = values.get(1).map_or(0, Value::int);
        let types = self.env.types();
        let src = args.first().map_or(&TyKind::Error, |a| types.kind(a.ty));
        let out = match (b, src) {
            (Builtin::WrappingAdd, TyKind::Int(t)) => {
                Ok(Value::Int(wrap(first.int() + second, *t)))
            }
            (Builtin::WrappingSub, TyKind::Int(t)) => {
                Ok(Value::Int(wrap(first.int() - second, *t)))
            }
            (Builtin::WrappingMul, TyKind::Int(t)) => {
                Ok(Value::Int(wrap(first.int().wrapping_mul(second), *t)))
            }
            (Builtin::WrappingNeg, TyKind::Int(t)) => Ok(Value::Int(wrap(-first.int(), *t))),
            (Builtin::Pow, TyKind::Int(t)) => int_pow(first.int(), second, *t),
            (Builtin::Pow, TyKind::Field) => Ok(field_pow(first.int() as u64, second as u64)),
            (Builtin::Min, TyKind::Int(_)) => Ok(Value::Int(first.int().min(second))),
            (Builtin::Max, TyKind::Int(_)) => Ok(Value::Int(first.int().max(second))),
            (Builtin::Inv, TyKind::Field) => field_inv(first.int() as u64),
            (Builtin::ToLeBits, TyKind::Int(t)) => Ok(bools(to_bits(first.int(), t.bits()))),
            (Builtin::ToLeBits, TyKind::Field) => Ok(bools(to_bits(first.int(), 64))),
            (Builtin::FromLeBits, _) => self.value_of_bits(&first, e),
            (Builtin::CheckedFrom, _) => self.checked_from(&first, e.ty),
            (Builtin::WrappingFrom, _) => Ok(self.wrapping_from(&first, e.ty)),
            _ => Err(FailKind::Internal),
        };
        out.map_err(|k| fail(k, e.span))
    }

    /** `T::from_le_bits(bits)`, `T` the type of `e`. */
    fn value_of_bits(&self, bits: &Value, e: &TExpr) -> Result<Value, FailKind> {
        let bits: Vec<bool> = bits.parts().iter().map(Value::bool).collect();
        let v = from_bits(&bits);
        match self.env.types().kind(e.ty) {
            TyKind::Int(t) => Ok(Value::Int(wrap(v, *t))),
            TyKind::Field if v < i128::from(nonos_stark::field::P) => Ok(Value::Field(v as u64)),
            TyKind::Field => Err(FailKind::NotCanonical),
            _ => Err(FailKind::Internal),
        }
    }
}

/** An array of bools. */
fn bools(bits: Vec<bool>) -> Value {
    Value::Array(bits.into_iter().map(Value::Bool).collect())
}
