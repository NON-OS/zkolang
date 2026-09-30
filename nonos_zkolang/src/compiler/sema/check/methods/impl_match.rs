/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The arguments a type gives a generic `impl` block (section 10.2): its written type
 * matched against the type, each of its parameters taking the part it stands at. A
 * parameter written twice takes parts that are one type.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::impl_mentions::mentions;
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId, TyKind};
use crate::compiler::syntax::ast::{Type, TypeKind};

/** Where a match writes: the module, the parameters' names, and what each takes. */
pub(super) type Binds<'n> = (DefId, &'n [&'n str], &'n mut [Option<GenArg>]);

impl<'s, 'a> FnCx<'s, 'a> {
    /** Whether the written type `pat` matches the type `ty`, binding parameters in `b`. */
    pub(super) fn match_ty(&mut self, pat: &'a Type, ty: TyId, b: &mut Binds<'_>) -> bool {
        let ty = self.resolve(ty);
        match (&pat.kind, self.kind(ty)) {
            (TypeKind::Path(p), _)
                if p.as_ident().is_some_and(|i| b.1.contains(&i.name.as_str())) =>
            {
                let k = b.1.iter().position(|n| p.last_name() == *n).unwrap_or(0);
                self.bind_param(k, GenArg::Type(ty), b)
            }
            (TypeKind::Path(p), TyKind::Adt(_)) => {
                let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
                let def = self.sema.defs.resolve(b.0, p.root, &names).ok();
                let adt = self
                    .sema
                    .types
                    .adt(ty)
                    .filter(|a| Some(DefId(a.def)) == def);
                let Some(args) = adt.map(|a| a.args.clone()) else {
                    return false;
                };
                let given = p
                    .segments
                    .last()
                    .and_then(|s| s.generics.as_deref())
                    .unwrap_or(&[]);
                given.len() == args.len()
                    && given.iter().zip(args).all(|(g, a)| self.match_arg(g, a, b))
            }
            (TypeKind::Tuple(ps), TyKind::Tuple(ts)) if ps.len() == ts.len() => {
                ps.iter().zip(ts).all(|(p, t)| self.match_ty(p, t, b))
            }
            (TypeKind::Array(e, n), TyKind::Array(el, len)) => {
                self.match_ty(e, el, b) && self.match_const(n, len, b)
            }
            _ if mentions(pat, b.1) => false,
            _ => {
                let t = self.sema.lower_ty(b.0, pat).0;
                self.unify(t, ty)
            }
        }
    }
}
