/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Unifying what inference leaves open: a general variable binds to any type that does
 * not contain it, and two instances of one struct or enum are one type when their
 * generic arguments are.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{GenArg, TyId, TyKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Bind the general variable `x` to `t`, unless `t` contains it; `!` fits it unbound. */
    pub(super) fn bind_general(&mut self, x: u32, t: TyId) -> bool {
        if self.kind(t) == TyKind::Never {
            return true;
        }
        if self.occurs(x, t) {
            return false;
        }
        self.vars.bind(x, t);
        true
    }

    /** Make the instances `a` and `b` one type: one item, and each argument one. */
    pub(super) fn unify_adts(&mut self, a: TyId, b: TyId) -> bool {
        let types = &self.sema.types;
        let (Some(x), Some(y)) = (types.adt(a), types.adt(b)) else {
            return false;
        };
        if x.def != y.def || x.args.len() != y.args.len() {
            return false;
        }
        let pairs: Vec<(GenArg, GenArg)> =
            x.args.iter().copied().zip(y.args.iter().copied()).collect();
        pairs.into_iter().fold(true, |ok, pair| {
            let same = match pair {
                (GenArg::Type(p), GenArg::Type(q)) => self.unify(p, q),
                (GenArg::Const(m), GenArg::Const(n)) => m == n,
                _ => false,
            };
            same && ok
        })
    }

    /** The type arguments of the instance `t`. */
    pub(super) fn type_args(&self, t: TyId) -> Vec<TyId> {
        let args = self
            .sema
            .types
            .adt(t)
            .map(|a| a.args.as_slice())
            .unwrap_or(&[]);
        let ty = |g: &GenArg| match g {
            GenArg::Type(t) => Some(*t),
            GenArg::Const(_) => None,
        };
        args.iter().filter_map(ty).collect()
    }
}
