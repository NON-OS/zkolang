/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The generic arguments a path in a body gives an item (section 5.5): those written on
 * the path, or else a variable per type parameter, which the rest of the body settles. A
 * constant parameter is not inferred: it is written.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{GenericArg, GenericParam};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The instance of the struct or enum `def`, with `given` if written, at `at`. */
    pub(crate) fn instance(
        &mut self,
        def: DefId,
        given: Option<&'a [GenericArg]>,
        at: Span,
    ) -> TyId {
        match self.generic_args(def, given, at) {
            Some(args) => self.sema.adt_ty(def, &args).0,
            None => Types::ERROR,
        }
    }

    /**
     * The generic arguments of the item `def` at `at`: `given` if written, else a variable
     * for each type parameter; `None` once reported, or for a constant parameter unwritten.
     */
    pub(crate) fn generic_args(
        &mut self,
        def: DefId,
        given: Option<&'a [GenericArg]>,
        at: Span,
    ) -> Option<Vec<GenArg>> {
        let params = self.sema.params_of(def);
        let name = self
            .sema
            .defs
            .get(def)
            .map_or_else(String::new, |d| d.name.clone());
        if let Some(g) = given {
            return self.sema.type_args(self.module, (params, &name), g, at);
        }
        let mut out = Vec::with_capacity(params.len());
        for p in params {
            let what = format!("the type `{}` of `{name}`", p.name().name);
            match p {
                GenericParam::Type(_) => {
                    let v = self.vars.fresh_general(&mut self.sema.types, at, what);
                    out.push(GenArg::Type(v));
                }
                GenericParam::Const { name: c, .. } => {
                    let what = format!("cannot infer the constant `{}` of `{name}`", c.name);
                    let d = Diagnostic::error(Code::CANNOT_INFER, what, at, "not written")
                        .with_help(format!("write it: `{name}::<..>`"));
                    self.sema.diags.push(d);
                    return None;
                }
            }
        }
        Some(out)
    }
}
