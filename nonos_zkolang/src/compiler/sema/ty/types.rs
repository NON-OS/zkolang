/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The table of types: each distinct type is stored once and named by its id. */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::{TyId, TyKind};
use crate::compiler::syntax::IntTy;

/** Every type of one compilation. */
#[derive(Clone, Debug)]
pub struct Types {
    kinds: Vec<TyKind>,
    ids: BTreeMap<TyKind, TyId>,
}

impl Types {
    /** A table holding the primitive types, at the ids the constants above name. */
    pub fn new() -> Types {
        let mut t = Types {
            kinds: Vec::new(),
            ids: BTreeMap::new(),
        };
        for k in [
            TyKind::Error,
            TyKind::Never,
            TyKind::Unit,
            TyKind::Bool,
            TyKind::Field,
        ] {
            t.intern(k);
        }
        IntTy::ALL.iter().for_each(|&i| {
            t.intern(TyKind::Int(i));
        });
        t
    }

    /** The id of the type `k`, adding it if it is new. */
    pub fn intern(&mut self, k: TyKind) -> TyId {
        if let Some(&id) = self.ids.get(&k) {
            return id;
        }
        let id = TyId(u32::try_from(self.kinds.len()).unwrap_or(u32::MAX));
        self.kinds.push(k.clone());
        self.ids.insert(k, id);
        id
    }

    /** What type `t` is. */
    pub fn kind(&self, t: TyId) -> &TyKind {
        self.kinds.get(t.0 as usize).unwrap_or(&TyKind::Error)
    }
}

impl Default for Types {
    fn default() -> Types {
        Types::new()
    }
}
