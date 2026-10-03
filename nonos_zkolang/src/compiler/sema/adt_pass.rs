/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The pass that lowers every struct and enum that takes no generic arguments. */

use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::{DefId, DefKind};

impl<'a> Sema<'a> {
    /**
     * Lower every struct and enum of the program that takes no generic arguments, so each
     * declaration is checked; a generic one is checked per instance, where it is used.
     */
    pub(super) fn structs(&mut self) {
        let kinds: Vec<_> = self.defs.defs.iter().map(|d| d.kind).collect();
        for (i, k) in kinds.into_iter().enumerate() {
            let def = DefId(u32::try_from(i).unwrap_or(u32::MAX));
            if matches!(k, DefKind::Struct | DefKind::Enum) && self.params_of(def).is_empty() {
                self.struct_ty(def);
            }
        }
    }
}
