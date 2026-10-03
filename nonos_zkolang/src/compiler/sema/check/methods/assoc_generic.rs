/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A function of a generic `impl` block named through its struct or enum, whose arguments are inferred. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::assoc_fn::Member;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The function `name` of a generic `impl` block of `def`, its parameters to infer. */
    pub(super) fn generic_member(&mut self, def: DefId, name: &str, at: Span) -> Option<Member> {
        let Some(&t) = self.sema.generic_assoc.get(&(def, String::from(name))) else {
            let item = self.sema.defs.get(def).map_or("", |d| d.name.as_str());
            let what = format!("`{item}` has no function `{name}` for every argument");
            let d = Diagnostic::error(Code::NO_FIELD, what, at, "not found").with_help(
                "give the type its arguments, as in `let x: G<u8> = ..`, and call the method on it",
            );
            self.sema.diags.push(d);
            return None;
        };
        let info = self.sema.fns.get(t.0 as usize)?;
        let item = self
            .sema
            .defs
            .get(def)
            .map_or_else(String::new, |d| d.name.clone());
        let params: Vec<String> = info
            .impl_generics
            .iter()
            .map(|g| g.name().name.clone())
            .collect();
        let mut args = Vec::with_capacity(params.len());
        for p in params {
            let what = format!("the type `{p}` of `{item}`");
            args.push(GenArg::Type(self.vars.fresh_general(
                &mut self.sema.types,
                at,
                what,
            )));
        }
        Some((t, args))
    }
}
