/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Random integer expressions of one type. */

use crate::prop_binary::Op;
use crate::prop_expr::E;
use crate::prop_gen::Rng;
use crate::prop_model::Ty;

const BINARY: [Op; 13] = [
    Op::Add,
    Op::Sub,
    Op::Mul,
    Op::Div,
    Op::Rem,
    Op::And,
    Op::Or,
    Op::Xor,
    Op::WrapAdd,
    Op::WrapSub,
    Op::WrapMul,
    Op::Min,
    Op::Max,
];
const COMPARE: [Op; 6] = [Op::Lt, Op::Le, Op::Gt, Op::Ge, Op::Eq, Op::Ne];

/** A random expression of type `t`, at most `depth` operators deep. */
pub(crate) fn expr(r: &mut Rng, t: Ty, depth: u32) -> E {
    let leaf = depth == 0 || r.below(4) == 0;
    let sub = |r: &mut Rng| Box::new(expr(r, t, depth.saturating_sub(1)));
    if leaf {
        return match r.below(3) {
            0 => E::A,
            1 => E::B,
            _ => E::Lit(r.value(t)),
        };
    }
    match r.below(10) {
        0 if t.signed => E::Neg(sub(r)),
        1 => E::Not(sub(r)),
        2 => E::WrapNeg(sub(r)),
        3 => E::Shift(
            r.below(2) == 0,
            sub(r),
            (r.below(2) == 0).then(|| r.below(u64::from(t.bits)) as u32),
        ),
        4 => E::Pow(sub(r), r.below(5) as u32),
        5 => E::If(COMPARE[r.below(6) as usize], sub(r), sub(r), sub(r), sub(r)),
        _ => E::Bin(BINARY[r.below(13) as usize], sub(r), sub(r)),
    }
}
