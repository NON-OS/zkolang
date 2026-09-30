/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The arguments of an instance of a generic item (section 5.5): a type, or a `usize`. */

use super::TyId;

/** One argument of a generic item's instance. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GenArg {
    Type(TyId),
    Const(u32),
}
