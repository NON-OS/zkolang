/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A parameter bound by name: a local, mutable if `&mut`, whose final value a `&mut` caller reads. */

use super::cx::FnCx;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::tir::{Labels, TParam, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The parameter `name`, of type, labels and passing `param`, bound as a local. */
    pub(super) fn named_param(
        &mut self,
        name: &str,
        param: (TyId, Labels, bool),
        mutable: bool,
        at: Span,
    ) -> TParam {
        let (ty, labels, by_ref) = param;
        let local = self.declare(name, ty, mutable, labels, at);
        if by_ref {
            self.read_local(local);
        }
        TParam {
            local,
            pat: TPat::Bind(local),
            by_ref,
        }
    }
}
