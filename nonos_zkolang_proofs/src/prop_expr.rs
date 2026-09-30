/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Random integer expressions over the parameters `a`, `b` and `k`, and their model value. */

use crate::prop_binary::{binary, Fail, Op};
use crate::prop_model::Ty;

/** An expression of one integer type. */
#[derive(Clone, Debug)]
pub(crate) enum E {
    A,
    B,
    Lit(i128),
    Neg(Box<E>),
    Not(Box<E>),
    WrapNeg(Box<E>),
    Bin(Op, Box<E>, Box<E>),
    /** A shift left when true, by a literal count or by `k` when `None`. */
    Shift(bool, Box<E>, Option<u32>),
    Pow(Box<E>, u32),
    /** `if l op r { t } else { f }`, `op` a comparison. */
    If(Op, Box<E>, Box<E>, Box<E>, Box<E>),
}

/** The value of `e` of type `t` for the arguments `(a, b, k)`, or why the run fails. */
pub(crate) fn eval(e: &E, t: Ty, args: (i128, i128, u32)) -> Result<i128, Fail> {
    let go = |x: &E| eval(x, t, args);
    Ok(match e {
        E::A => args.0,
        E::B => args.1,
        E::Lit(v) => *v,
        E::Neg(x) => t.fit(go(x)?.checked_neg())?,
        E::Not(x) => {
            if t.signed {
                !go(x)?
            } else {
                t.max() ^ go(x)?
            }
        }
        E::WrapNeg(x) => t.wrap(-go(x)?),
        E::Bin(op, l, r) => {
            let (x, y) = (go(l)?, go(r)?);
            binary(*op, x, y, t)?
        }
        E::Shift(left, x, count) => {
            let x = go(x)?;
            let c = count.unwrap_or(args.2);
            if c >= t.bits {
                return Err(Fail::ShiftTooFar);
            }
            if *left {
                t.wrap(x << c)
            } else {
                x >> c
            }
        }
        E::Pow(x, n) => t.fit(go(x)?.checked_pow(*n))?,
        E::If(op, l, r, yes, no) => {
            let (x, y) = (go(l)?, go(r)?);
            if binary(*op, x, y, t)? == 1 {
                go(yes)?
            } else {
                go(no)?
            }
        }
    })
}
