/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The arguments a type gives a generic `impl` block, by matching the type the block is written for. */

use alloc::vec;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{GenArg, TyId, Types};
use crate::compiler::tir::FnId;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The arguments `ty` gives the generic `impl` block of the template `t`, if it matches. */
    pub(super) fn impl_args(&mut self, t: FnId, ty: TyId) -> Option<Vec<GenArg>> {
        let info = self.sema.fns.get(t.0 as usize)?;
        let (m, pat, params) = (info.module, info.impl_self?, info.impl_generics);
        let names: Vec<&str> = params.iter().map(|p| p.name().name.as_str()).collect();
        let mut out = vec![None; names.len()];
        if !self.match_ty(pat, ty, &mut (m, &names, &mut out)) {
            return None;
        }
        Some(
            out.into_iter()
                .map(|g| g.unwrap_or(GenArg::Type(Types::ERROR)))
                .collect(),
        )
    }
}
