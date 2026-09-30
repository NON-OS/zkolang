/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Literal values in the typed IR. */

/** A literal value of the expression's type. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TLit {
    Unit,
    Bool(bool),
    /** An integer of the expression's integer type, or a `field` element in `[0, p)`. */
    Int(i128),
}
