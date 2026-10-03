/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a native target needs of each leaf of `main`'s inputs and result: the range of its
 * values, whether it takes two 32-bit slots, and whether its encoding is signed.
 */

use nonos_stark::field::P;

use crate::compiler::driver::abi::Leaf;

/** The values a leaf takes, inclusive, and how it is laid out in slots. */
pub(super) struct Bound {
    pub(super) lo: i128,
    pub(super) hi: i128,
    pub(super) wide: bool,
    pub(super) signed: bool,
}

/** The bound of `leaf`, as `zkolang run` checks its inputs (section 12.2). */
pub(super) fn bound(leaf: Leaf) -> Bound {
    let p = i128::from(P);
    let (lo, hi, signed) = match leaf {
        Leaf::Bool => (0, 1, false),
        Leaf::Field | Leaf::Slot => (0, p - 1, false),
        Leaf::Int(t) => (t.min(), t.max(), t.signed()),
        Leaf::Tag(n) => (0, i128::from(n) - 1, false),
    };
    Bound {
        lo,
        hi,
        wide: leaf.width() == 2,
        signed,
    }
}
