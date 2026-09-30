/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! SSA instructions. */

use super::{Hint, V};

/**
 * An instruction. The machine-level ones map one to one onto the machine's; the gadget
 * ones are expanded into machine-level ones before registers are allocated.
 */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Inst {
    /** A field element, as its canonical representative. */
    Const(u64),
    /** Slot `i` of the inputs: the public slots, then the secret ones (section 12.2). */
    Input(u16),
    /** A witness the prover supplies, found by the hint. */
    Advice(Hint),
    Add(V, V),
    Sub(V, V),
    Mul(V, V),
    /** The inverse; a zero operand makes the run fail. */
    Inv(V),
    /** `c ? a : b`, `c` constrained to be 0 or 1. */
    Sel(V, V, V),
    /** 1 if the operands are equal, else 0. */
    Eq(V, V),
    AssertBool(V),
    AssertZero(V),
    /** Output slot `i` is the value. */
    Output(u16, V),
    /** Gadget: fail unless the canonical representative is below `2^bits`, `bits < 64`. */
    RangeCheck(V, u8),
    /** Gadget: bit `k` of the value, which must be below `2^bits`, `bits < 64`. */
    Bit(V, u8, u8),
    /** Gadget: bit `k` of the canonical representative, of 64. */
    FieldBit(V, u8),
    /** Gadget: `a / b` rounded down, operands below `2^bits`, `bits <= 32`, `b` not 0. */
    Quot(V, V, u8),
    /** Gadget: `a mod b`, with the same conditions. */
    Rem(V, V, u8),
}
