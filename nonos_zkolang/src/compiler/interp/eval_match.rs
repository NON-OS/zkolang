/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `match` (section 8.5): the scrutinee evaluated once, then the first arm whose pattern
 * matches and whose guard holds. Exhaustiveness is checked, so some arm is taken.
 */

use alloc::vec;

use super::machine::{fail, Eval, Interp};
use super::{FailKind, Value};
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::tir::{TArm, TExpr, TLit, TPat};

impl<'e> Interp<'e> {
    /** `match scrut { arms }`. */
    pub(super) fn eval_match(&mut self, scrut: &TExpr, arms: &[TArm], at: Span) -> Eval {
        let v = self.eval(scrut)?;
        for arm in arms {
            if !matches(&arm.pat, &v) {
                continue;
            }
            self.bind(&arm.pat, v.clone());
            if let Some(g) = &arm.guard {
                if !self.eval(g)?.bool() {
                    continue;
                }
            }
            return self.eval(&arm.body);
        }
        Err(fail(FailKind::Internal, at))
    }

    /** The variant `tag` of the enum `ty`, its fields evaluated in the order written. */
    pub(super) fn eval_variant(&mut self, tag: u32, fs: &[(u32, TExpr)], ty: TyId) -> Eval {
        let adt = self.env.types().adt(ty);
        let n = adt
            .and_then(|a| a.variants.get(tag as usize))
            .map_or(0, |v| v.fields.len());
        let mut parts = vec![Value::Unit; n];
        for (i, x) in fs {
            let v = self.eval(x)?;
            if let Some(p) = parts.get_mut(*i as usize) {
                *p = v;
            }
        }
        Ok(Value::Variant(tag, parts))
    }
}

/** Whether the value `v` matches the pattern `p`. */
pub(super) fn matches(p: &TPat, v: &Value) -> bool {
    match (p, v) {
        (TPat::Bind(_) | TPat::Wild, _) => true,
        (TPat::Tuple(ps), Value::Tuple(vs) | Value::Array(vs)) => {
            ps.iter().zip(vs).all(|(p, v)| matches(p, v))
        }
        (TPat::Lit(TLit::Bool(b), _), Value::Bool(c)) => b == c,
        (TPat::Lit(TLit::Int(i), _), Value::Int(x)) => i == x,
        (TPat::Lit(TLit::Int(i), _), Value::Field(x)) => *i == i128::from(*x),
        (TPat::Lit(TLit::Unit, _), Value::Unit) => true,
        (TPat::Range(lo, hi, _), Value::Int(x)) => lo <= x && x <= hi,
        (TPat::Variant(t, ps), Value::Variant(u, vs)) => {
            t == u && ps.iter().zip(vs).all(|(p, v)| matches(p, v))
        }
        (TPat::Or(alts), v) => alts.iter().any(|a| matches(a, v)),
        _ => false,
    }
}
