/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Function signatures (section 10.1): parameter and result types, lowered on first use. */

use alloc::vec::Vec;

use super::cx::{Sema, Sig};
use crate::compiler::sema::ty::Types;
use crate::compiler::syntax::ast::{Param, TypeKind};
use crate::compiler::tir::{FnId, Labels};

impl<'a> Sema<'a> {
    /** The signature of function `f`. */
    pub(crate) fn sig(&mut self, f: FnId) -> Sig {
        let Some(info) = self.fns.get(f.0 as usize) else {
            return Sig {
                params: Vec::new(),
                ret: Types::ERROR,
                ret_labels: Labels::default(),
            };
        };
        if let Some(s) = &info.sig {
            return s.clone();
        }
        let (m, decl) = (info.module, info.decl);
        let mut params = Vec::with_capacity(decl.params.len());
        for p in &decl.params {
            params.push(match p {
                Param::SelfParam { span, .. } => {
                    self.not_checked("methods", *span);
                    (Types::ERROR, Labels::default(), false)
                }
                Param::Typed { ty, .. } => {
                    let by_ref = matches!(ty.kind, TypeKind::RefMut(_));
                    let (t, l) = self.lower_ty(m, ty);
                    (t, l, by_ref)
                }
            });
        }
        let (ret, ret_labels) = match &decl.ret {
            Some(t) => self.lower_ty(m, t),
            None => (Types::UNIT, Labels::default()),
        };
        let sig = Sig {
            params,
            ret,
            ret_labels,
        };
        if let Some(info) = self.fns.get_mut(f.0 as usize) {
            info.sig = Some(sig.clone());
        }
        sig
    }
}
