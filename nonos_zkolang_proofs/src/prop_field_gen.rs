/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Random `field` expressions and arguments, and their zKølang source. */

use crate::prop_field::{F, P};
use crate::prop_gen::Rng;

/** A `field` value: often at an edge, else any. */
pub(crate) fn value(r: &mut Rng) -> u64 {
    let edges = [
        0,
        1,
        2,
        (P - 1) as u64,
        (P - 2) as u64,
        1 << 32,
        (1 << 32) - 1,
    ];
    match r.below(2) {
        0 => edges[r.below(7) as usize],
        _ => (u128::from(r.next()) % P) as u64,
    }
}

/** A random `field` expression, at most `depth` operators deep. */
pub(crate) fn expr(r: &mut Rng, depth: u32) -> F {
    let sub = |r: &mut Rng| Box::new(expr(r, depth.saturating_sub(1)));
    if depth == 0 || r.below(4) == 0 {
        return match r.below(3) {
            0 => F::A,
            1 => F::B,
            _ => F::Lit(value(r)),
        };
    }
    match r.below(8) {
        0 => F::Neg(sub(r)),
        1 => F::Inv(sub(r)),
        2 => F::Pow(sub(r), r.below(70)),
        3 => F::IfEq(sub(r), sub(r), sub(r), sub(r)),
        _ => F::Bin(['+', '-', '*', '/'][r.below(4) as usize], sub(r), sub(r)),
    }
}

/** The source of `e`; a method's receiver adds `z`, a zero of type `field`, so its type is known. */
pub(crate) fn print(e: &F) -> String {
    match e {
        F::A => String::from("a"),
        F::B => String::from("b"),
        F::Lit(v) => format!("{v}"),
        F::Neg(x) => format!("(-{})", print(x)),
        F::Bin(op, l, r) => format!("({} {op} {})", print(l), print(r)),
        F::Inv(x) => format!("(z + {}).inv()", print(x)),
        F::Pow(x, n) => format!("(z + {}).pow({n})", print(x)),
        F::IfEq(l, r, t, f) => {
            let (l, r, t, f) = (print(l), print(r), print(t), print(f));
            format!("(if {l} == {r} {{ {t} }} else {{ {f} }})")
        }
    }
}
