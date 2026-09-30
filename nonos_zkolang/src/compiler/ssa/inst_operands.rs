/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The operands of an instruction: the values it reads, and the values its hint reads. A
 * machine instruction reads its operands from registers; a hint runs only in the witness
 * generator, which has every value, so its operands need no register.
 */

use super::{Hint, Inst, V};

impl Inst {
    /** The values the instruction reads, not counting its hint's. */
    pub fn operands(&self) -> impl Iterator<Item = V> {
        let (a, b, c) = match *self {
            Inst::Const(_) | Inst::Input(_) | Inst::Advice(_) => (None, None, None),
            Inst::Inv(a) | Inst::AssertBool(a) | Inst::AssertZero(a) | Inst::Output(_, a) => {
                (Some(a), None, None)
            }
            Inst::RangeCheck(a, _) | Inst::Bit(a, _, _) | Inst::FieldBit(a, _) => {
                (Some(a), None, None)
            }
            Inst::Add(a, b) | Inst::Sub(a, b) | Inst::Mul(a, b) | Inst::Eq(a, b) => {
                (Some(a), Some(b), None)
            }
            Inst::Quot(a, b, _) | Inst::Rem(a, b, _) => (Some(a), Some(b), None),
            Inst::Sel(c, a, b) => (Some(c), Some(a), Some(b)),
        };
        [a, b, c].into_iter().flatten()
    }

    /** The values the instruction's hint reads, if it has one. */
    pub fn hint_operands(&self) -> impl Iterator<Item = V> {
        let vs = match *self {
            Inst::Advice(Hint::Bit(a, _)) => [Some(a), None, None, None],
            Inst::Advice(Hint::Quot(a, b) | Hint::Rem(a, b)) => [Some(a), Some(b), None, None],
            Inst::Advice(Hint::Div64 { a, b, .. }) => [Some(a.0), Some(a.1), Some(b.0), Some(b.1)],
            _ => [None, None, None, None],
        };
        vs.into_iter().flatten()
    }

    /** Whether it constrains or outputs rather than defining a value others read. */
    pub fn is_effect(&self) -> bool {
        matches!(
            self,
            Inst::AssertBool(_) | Inst::AssertZero(_) | Inst::Output(..) | Inst::RangeCheck(..)
        )
    }

    /** Whether the gadget expander writes it out as machine instructions. */
    pub fn is_gadget(&self) -> bool {
        matches!(
            self,
            Inst::RangeCheck(..)
                | Inst::Bit(..)
                | Inst::FieldBit(..)
                | Inst::Quot(..)
                | Inst::Rem(..)
        )
    }
}
