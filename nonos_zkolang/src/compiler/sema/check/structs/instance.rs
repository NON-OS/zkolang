/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The instance of a generic struct or enum a path in a body names (section 5.5): for the
 * arguments written on the path, or else for a variable per type parameter, which the
 * rest of the body settles. A constant parameter is not inferred: it is written.
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
        let params = self.sema.params_of(def);
        let name = self
            .sema
            .defs
            .get(def)
            .map_or_else(String::new, |d| d.name.clone());
        let args = match given {
            Some(g) => match self.sema.type_args(self.module, (params, &name), g, at) {
                Some(a) => a,
                None => return Types::ERROR,
            },
            None => {
                let mut out = Vec::with_capacity(params.len());
                for p in params {
                    let what = format!("the type `{}` of `{name}`", p.name().name);
                    match p {
                        GenericParam::Type(_) => {
                            let v = self.vars.fresh_general(&mut self.sema.types, at, what);
                            out.push(GenArg::Type(v));
                        }
                        GenericParam::Const { name: c, .. } => {
                            let what =
                                format!("cannot infer the constant `{}` of `{name}`", c.name);
                            let d = Diagnostic::error(Code::CANNOT_INFER, what, at, "not written")
                                .with_help(format!("write it: `{name}::<..>`"));
                            self.sema.diags.push(d);
                            return Types::ERROR;
                        }
                    }
                }
                out
            }
        };
        self.sema.adt_ty(def, &args).0
    }
}
