/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The binary operators, methods and comparisons of the model (sections 7.2 to 7.5). */

use crate::prop_model::Ty;

/** Binary operators, methods of two operands, and comparisons. */
#[derive(Clone, Copy, Debug)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    WrapAdd,
    WrapSub,
    WrapMul,
    Min,
    Max,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

/** Why a run fails, as far as the model tells. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fail {
    Overflow,
    DivideByZero,
    ShiftTooFar,
    InverseOfZero,
}

/** `x op y` of type `t`; a comparison gives 1 for true and 0 for false. */
pub(crate) fn binary(op: Op, x: i128, y: i128, t: Ty) -> Result<i128, Fail> {
    Ok(match op {
        Op::Add => t.fit(x.checked_add(y))?,
        Op::Sub => t.fit(x.checked_sub(y))?,
        Op::Mul => t.fit(x.checked_mul(y))?,
        Op::Div | Op::Rem if y == 0 => return Err(Fail::DivideByZero),
        Op::Div => t.fit(x.checked_div(y))?,
        Op::Rem => t.fit(if t.signed && x == t.min() && y == -1 {
            None
        } else {
            x.checked_rem(y)
        })?,
        Op::And => x & y,
        Op::Or => x | y,
        Op::Xor => x ^ y,
        Op::WrapAdd => t.wrap(x.wrapping_add(y)),
        Op::WrapSub => t.wrap(x.wrapping_sub(y)),
        Op::WrapMul => t.wrap(x.wrapping_mul(y)),
        Op::Min => x.min(y),
        Op::Max => x.max(y),
        Op::Lt => i128::from(x < y),
        Op::Le => i128::from(x <= y),
        Op::Gt => i128::from(x > y),
        Op::Ge => i128::from(x >= y),
        Op::Eq => i128::from(x == y),
        Op::Ne => i128::from(x != y),
    })
}
