/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The built-in methods and associated functions of section 18.3, dispatched by kind. */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::tir::{Builtin, TExpr};

impl<'p> Lower<'p> {
    /** The built-in `b` on `args`, the expression `e`. */
    pub(super) fn builtin(&mut self, b: Builtin, args: &[TExpr], e: &TExpr) -> L<Vec<V>> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.expr(a)?);
        }
        let src = args.first().map_or(e.ty, |a| a.ty);
        let types = &self.p.types;
        let (from, to) = (types.kind(src).clone(), types.kind(e.ty).clone());
        let wide = |k: &TyKind| matches!(k, TyKind::Int(i) if i.bits() > 32);
        if wide(&from) || wide(&to) {
            return self.int64_builtin(b, &vals, args, (&from, &to), e);
        }
        let x = vals
            .first()
            .and_then(|v| v.first().copied())
            .unwrap_or(V(0));
        let y = vals.get(1).and_then(|v| v.first().copied()).unwrap_or(V(0));
        Ok(match (b, from.clone()) {
            (Builtin::Inv, _) => {
                let d = self.guarded(x, 1);
                alloc::vec![self.b.emit(Inst::Inv(d))]
            }
            (Builtin::Min | Builtin::Max, TyKind::Int(t)) => {
                let lt = self.less(x, y, t);
                let (a, c) = if b == Builtin::Min { (x, y) } else { (y, x) };
                alloc::vec![self.b.sel(lt, a, c)]
            }
            (
                Builtin::WrappingAdd
                | Builtin::WrappingSub
                | Builtin::WrappingMul
                | Builtin::WrappingNeg,
                TyKind::Int(t),
            ) => {
                alloc::vec![self.wrapping(b, x, y, t)]
            }
            (Builtin::Pow, _) => alloc::vec![self.pow(x, args.get(1), &from, e)?],
            (Builtin::ToLeBits, _) => self.bits_of(x, &from),
            (Builtin::FromLeBits, _) => {
                alloc::vec![self.value_of_bits(vals.first().map_or(&[][..], |v| v), &to)]
            }
            (Builtin::CheckedFrom, _) => alloc::vec![self.checked_from(x, &from, &to)],
            (Builtin::WrappingFrom, _) => alloc::vec![self.wrapping_from(x, &from, &to)],
            _ => return Err(LowerError::Unsupported("this built-in", e.span)),
        })
    }
}
