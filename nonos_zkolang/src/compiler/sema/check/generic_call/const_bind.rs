/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The constant parameters an argument's type gives a generic function (section 10.5): the
 * parameter's written type walked beside the argument's type, each constant parameter
 * taking the array length or constant argument it stands at, the first it meets. Nothing
 * is lowered or unified; the call checks each argument against the instance after.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::const_names::{bind_one, Consts};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId, TyKind};
use crate::compiler::syntax::ast::{ConstArg, GenericArg, Type, TypeKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Bind the constant parameters that the type `pat`, written in module `m`, places in `ty`. */
    pub(super) fn bind_consts(&mut self, m: DefId, pat: &Type, ty: TyId, c: &mut Consts<'_>) {
        let ty = self.resolve(ty);
        match (&pat.kind, self.kind(ty)) {
            (TypeKind::Labelled(_, e) | TypeKind::RefMut(e), _) => self.bind_consts(m, e, ty, c),
            (TypeKind::Tuple(ps), TyKind::Tuple(ts)) => ps
                .iter()
                .zip(ts)
                .for_each(|(p, t)| self.bind_consts(m, p, t, c)),
            (TypeKind::Array(e, n), TyKind::Array(el, len)) => {
                self.bind_consts(m, e, el, c);
                if let ConstArg::Path(p) = n {
                    bind_one(p, len, c);
                }
            }
            (TypeKind::Path(p), TyKind::Adt(_)) => {
                let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
                let def = self.sema.defs.resolve(m, p.root, &names).ok();
                let adt = self
                    .sema
                    .types
                    .adt(ty)
                    .filter(|a| Some(DefId(a.def)) == def);
                let Some(args) = adt.map(|a| a.args.clone()) else {
                    return;
                };
                let given = p.segments.last().and_then(|s| s.generics.as_deref());
                for (g, a) in given.unwrap_or(&[]).iter().zip(args) {
                    match (g, a) {
                        (GenericArg::Type(t), GenArg::Type(x)) => self.bind_consts(m, t, x, c),
                        (GenericArg::Const(ConstArg::Path(q)), GenArg::Const(n)) => {
                            bind_one(q, n, c)
                        }
                        (GenericArg::Type(t), GenArg::Const(n)) => {
                            if let TypeKind::Path(q) = &t.kind {
                                bind_one(q, n, c);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
