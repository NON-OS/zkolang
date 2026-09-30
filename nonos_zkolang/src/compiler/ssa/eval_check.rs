/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The constraints and gadgets of the SSA semantics: each holds or fails the run, and a
 * gadget that defines a value gives it.
 */

use nonos_stark::field::Fp;

use super::{Inst, V};

/** The value of the constraint or gadget `inst`, or `None` if it does not hold. */
pub(super) fn check(inst: Inst, values: &[Fp]) -> Option<Fp> {
    let get = |v: V| values.get(v.index()).map(|x| x.value());
    let below = |x: u64, bits: u8| bits < 64 && x >> bits == 0;
    let bit = |x: u64, k: u8| Fp::from_u64(x.checked_shr(u32::from(k)).unwrap_or(0) & 1);
    match inst {
        Inst::AssertBool(a) => (get(a)? <= 1).then_some(Fp::ZERO),
        Inst::AssertZero(a) => (get(a)? == 0).then_some(Fp::ZERO),
        Inst::RangeCheck(a, n) => below(get(a)?, n).then_some(Fp::ZERO),
        Inst::Bit(a, k, n) => {
            let x = get(a)?;
            below(x, n).then(|| bit(x, k))
        }
        Inst::FieldBit(a, k) => Some(bit(get(a)?, k)),
        Inst::Quot(a, b, n) | Inst::Rem(a, b, n) => {
            let (x, y) = (get(a)?, get(b)?);
            if n > 32 || !below(x, n) || !below(y, n) || y == 0 {
                return None;
            }
            let q = matches!(inst, Inst::Quot(..));
            Some(Fp::from_u64(if q { x / y } else { x % y }))
        }
        _ => None,
    }
}
