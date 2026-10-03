/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The structs and enums of the type table. */

use alloc::vec::Vec;

use super::{Adt, AdtId, TyId, TyKind, Types};

impl Types {
    /** Add the struct or enum `a` and return its type. */
    pub fn add_adt(&mut self, a: Adt) -> TyId {
        let id = AdtId(u32::try_from(self.adts.len()).unwrap_or(u32::MAX));
        self.adts.push(a);
        self.intern(TyKind::Adt(id))
    }

    /** The struct or enum that type `t` is, if it is one. */
    pub fn adt(&self, t: TyId) -> Option<&Adt> {
        match self.kind(t) {
            TyKind::Adt(id) => self.adts.get(id.0 as usize),
            _ => None,
        }
    }

    /**
     * The types of the parts of a tuple or struct, in order, which are laid out, labelled
     * and matched alike; `None` for any other type.
     */
    pub fn record(&self, t: TyId) -> Option<Vec<TyId>> {
        match self.kind(t) {
            TyKind::Tuple(ts) => Some(ts.clone()),
            _ => {
                let a = self.adt(t).filter(|a| !a.is_enum)?;
                Some(a.variants.first()?.fields.iter().map(|f| f.ty).collect())
            }
        }
    }

    /**
     * The types of the parts a pattern of type `t` takes apart, in order: the elements of
     * a tuple or array, the fields of a struct, or the fields of the enum variant `tag`.
     */
    pub fn parts(&self, t: TyId, tag: Option<u32>) -> Vec<TyId> {
        if let (TyKind::Array(el, n), _) = (self.kind(t), tag) {
            return alloc::vec![*el; *n as usize];
        }
        match tag {
            Some(tag) => {
                let a = self.adt(t).filter(|a| a.is_enum);
                let v = a.and_then(|a| a.variants.get(tag as usize));
                v.map(|v| v.fields.iter().map(|f| f.ty).collect())
                    .unwrap_or_default()
            }
            None => self.record(t).unwrap_or_default(),
        }
    }
}
