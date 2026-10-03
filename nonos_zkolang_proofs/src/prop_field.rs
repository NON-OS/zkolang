/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A model of `field` arithmetic (section 7.1) in plain `u128` arithmetic modulo p, apart
 * from the prover's field type, and random `field` expressions to test it on.
 */

use crate::prop_binary::Fail;

/** The modulus, 2^64 - 2^32 + 1. */
pub(crate) const P: u128 = 0xFFFF_FFFF_0000_0001;

/** A `field` expression over the parameters `a` and `b`. */
#[derive(Clone, Debug)]
pub(crate) enum F {
    A,
    B,
    Lit(u64),
    Neg(Box<F>),
    /** `+`, `-`, `*` or `/`, by its symbol. */
    Bin(char, Box<F>, Box<F>),
    Inv(Box<F>),
    Pow(Box<F>, u64),
    /** `if l == r { t } else { f }`. */
    IfEq(Box<F>, Box<F>, Box<F>, Box<F>),
}

/** `x^n` modulo p. */
pub(crate) fn pow(x: u128, mut n: u64) -> u128 {
    let (mut acc, mut base) = (1u128, x % P);
    while n > 0 {
        if n & 1 == 1 {
            acc = acc * base % P;
        }
        base = base * base % P;
        n >>= 1;
    }
    acc
}

/** The value of `e` for the arguments `(a, b)`, or why the run fails. */
pub(crate) fn eval(e: &F, args: (u64, u64)) -> Result<u128, Fail> {
    let go = |x: &F| eval(x, args);
    Ok(match e {
        F::A => u128::from(args.0),
        F::B => u128::from(args.1),
        F::Lit(v) => u128::from(*v),
        F::Neg(x) => (P - go(x)?) % P,
        F::Bin(op, l, r) => {
            let (x, y) = (go(l)?, go(r)?);
            match op {
                '+' => (x + y) % P,
                '-' => (x + P - y) % P,
                '*' => x * y % P,
                _ if y == 0 => return Err(Fail::DivideByZero),
                _ => x * pow(y, (P - 2) as u64) % P,
            }
        }
        F::Inv(x) => match go(x)? {
            0 => return Err(Fail::InverseOfZero),
            y => pow(y, (P - 2) as u64),
        },
        F::Pow(x, n) => pow(go(x)?, *n),
        F::IfEq(l, r, yes, no) => match go(l)? == go(r)? {
            true => go(yes)?,
            false => go(no)?,
        },
    })
}
