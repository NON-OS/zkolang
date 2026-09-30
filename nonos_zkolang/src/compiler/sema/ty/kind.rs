/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a type is. Labels are not part of a type: section 13 tracks them apart. */

use alloc::vec::Vec;

use crate::compiler::syntax::IntTy;

/** A type, as an index into the table of one compilation's types. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TyId(pub u32);

/** The kinds of type. */
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TyKind {
    /**
     * The type of what failed to check. It agrees with every type, so that one mistake
     * is reported once and not again wherever its value goes.
     */
    Error,
    /** The type of `return`, `break` and `continue`, which yield no value: it fits any. */
    Never,
    Unit,
    Bool,
    Field,
    Int(IntTy),
    Tuple(Vec<TyId>),
    /** `[T; N]`: the element type and the length. */
    Array(TyId, u32),
    /** The type of an unsuffixed integer literal while it is not yet known (section 5.6). */
    Var(u32),
}
