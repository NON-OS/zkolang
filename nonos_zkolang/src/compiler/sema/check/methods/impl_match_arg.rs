/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Matching the generic arguments and constants written in an `impl` block's type. */

use super::super::cx::FnCx;
use super::impl_match::Binds;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::syntax::ast::{ConstArg, GenericArg, Path, TypeKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Whether the written argument `g` matches the argument `a`. */
    pub(super) fn match_arg(&mut self, g: &'a GenericArg, a: GenArg, b: &mut Binds<'_>) -> bool {
        match (g, a) {
            (GenericArg::Type(t), GenArg::Type(ty)) => self.match_ty(t, ty, b),
            (GenericArg::Const(c), GenArg::Const(n)) => self.match_const(c, n, b),
            (GenericArg::Type(t), GenArg::Const(n)) => match &t.kind {
                TypeKind::Path(p) => self.match_const_path(p, n, b),
                _ => false,
            },
            _ => false,
        }
    }

    /** Whether the written constant `c` matches the value `n`. */
    pub(super) fn match_const(&mut self, c: &'a ConstArg, n: u32, b: &mut Binds<'_>) -> bool {
        match c {
            ConstArg::Path(p) => self.match_const_path(p, n, b),
            _ => self.sema.const_usize(b.0, c) == Some(n),
        }
    }

    /** Whether the constant path `p`, a parameter or a constant, matches the value `n`. */
    fn match_const_path(&mut self, p: &'a Path, n: u32, b: &mut Binds<'_>) -> bool {
        if let Some(k) = p
            .as_ident()
            .and_then(|i| b.1.iter().position(|x| *x == i.name))
        {
            return self.bind_param(k, GenArg::Const(n), b);
        }
        let v = self.sema.const_path(b.0, p);
        v.and_then(|v| u32::try_from(v).ok()) == Some(n)
    }

    /** Bind parameter `k` to `arg`, or check it agrees with what it has. */
    pub(super) fn bind_param(&mut self, k: usize, arg: GenArg, b: &mut Binds<'_>) -> bool {
        match (b.2.get(k).copied().flatten(), arg) {
            (None, _) => b.2.get_mut(k).map(|s| *s = Some(arg)).is_some(),
            (Some(GenArg::Type(x)), GenArg::Type(y)) => self.unify(x, y),
            (Some(GenArg::Const(x)), GenArg::Const(y)) => x == y,
            _ => false,
        }
    }
}
