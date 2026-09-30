/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `T::f` and `Self::f` (section 7.10): a function of an `impl` block, found through its
 * type. Through a generic struct or enum named without arguments, `G::f`, the function is
 * one of a generic `impl` block of `G`, whose parameters are inferred.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::defs::DefKind;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::syntax::ast::{Path, PathRoot};
use crate::compiler::tir::FnId;

/** A function of an `impl` block, and the arguments its type gives the block. */
pub(crate) type Member = (FnId, Vec<GenArg>);

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The function `p` names through a struct or enum, if `p` names one: `Some(Some(f))`
     * found, `Some(None)` reported; `None` if `p` does not go through one.
     */
    pub(crate) fn assoc_fn(&mut self, p: &'a Path) -> Option<Option<Member>> {
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let (last, prefix) = names.split_last()?;
        let ty = match (p.root, prefix.is_empty()) {
            (PathRoot::SelfType, true) => self.sema.self_type(p.span).0,
            (_, false) => {
                let def = self.sema.defs.resolve(self.module, p.root, prefix).ok()?;
                let kind = self.sema.defs.get(def).map(|d| d.kind);
                if !matches!(kind, Some(DefKind::Struct | DefKind::Enum)) {
                    return None;
                }
                self.sema.note_use(def, p);
                if !self.sema.params_of(def).is_empty() {
                    return Some(self.generic_member(def, last, p.span));
                }
                self.sema.struct_ty(def).0
            }
            _ => return None,
        };
        Some(self.member_fn(ty, last, p.span))
    }
}
