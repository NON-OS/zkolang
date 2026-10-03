/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The primitive types, which every table holds at fixed ids. */

use super::{TyId, Types};
use crate::compiler::syntax::IntTy;

impl Types {
    pub const ERROR: TyId = TyId(0);
    pub const NEVER: TyId = TyId(1);
    pub const UNIT: TyId = TyId(2);
    pub const BOOL: TyId = TyId(3);
    pub const FIELD: TyId = TyId(4);

    /** The id of an integer type, which `new` has interned. */
    pub fn int(i: IntTy) -> TyId {
        let at = IntTy::ALL.iter().position(|&x| x == i).unwrap_or(0);
        TyId(5 + at as u32)
    }
}
