/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The built-in methods and associated functions of section 18.3 that compute at run time.
 * The receiver or the source value is the first operand; the result type is the call's.
 * `len`, `MIN`, `MAX` and `BITS` are constants and are folded by the checker.
 */

/** A built-in operation. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Builtin {
    WrappingAdd,
    WrappingSub,
    WrappingMul,
    WrappingNeg,
    /** `a.pow(k)`: checked for an integer, `k: u32`; modular for `field`, `k: u64`. */
    Pow,
    Min,
    Max,
    /** `a.inv()` on `field`: fails for zero. */
    Inv,
    /** `a.to_le_bits()`: the bits least significant first, as `[bool; N]`. */
    ToLeBits,
    /** `T::from_le_bits(bits)`, `T` the result type. */
    FromLeBits,
    /** `T::checked_from(e)`: fails when the value is not a value of `T`. */
    CheckedFrom,
    /** `T::wrapping_from(e)`: the value modulo `2^N`, as `T`. */
    WrappingFrom,
}
