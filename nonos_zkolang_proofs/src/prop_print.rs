/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Writing a random expression as zKølang source, every operand in parentheses. */

use crate::prop_binary::Op;
use crate::prop_expr::E;
use crate::prop_model::Ty;

/** The source of `e`, an expression of type `t`. */
pub(crate) fn print(e: &E, t: Ty) -> String {
    let p = |x: &E| print(x, t);
    match e {
        E::A => String::from("a"),
        E::B => String::from("b"),
        E::Lit(v) if *v < 0 => format!("(-{}{})", v.unsigned_abs(), t.name),
        E::Lit(v) => format!("{v}{}", t.name),
        E::Neg(x) => format!("(-{})", p(x)),
        E::Not(x) => format!("(!{})", p(x)),
        E::WrapNeg(x) => format!("({}).wrapping_neg()", p(x)),
        E::Bin(op, l, r) => match method(*op) {
            Some(m) => format!("({}).{m}({})", p(l), p(r)),
            None => format!("({} {} {})", p(l), sym(*op), p(r)),
        },
        E::Shift(left, x, count) => {
            let by = count.map_or(String::from("k"), |c| format!("{c}u32"));
            format!("({} {} {by})", p(x), if *left { "<<" } else { ">>" })
        }
        E::Pow(x, n) => format!("({}).pow({n})", p(x)),
        E::If(op, l, r, yes, no) => {
            let (l, r, yes, no) = (p(l), p(r), p(yes), p(no));
            format!("(if {l} {} {r} {{ {yes} }} else {{ {no} }})", sym(*op))
        }
    }
}

/** The method an operator is written as, if it is one. */
fn method(op: Op) -> Option<&'static str> {
    Some(match op {
        Op::WrapAdd => "wrapping_add",
        Op::WrapSub => "wrapping_sub",
        Op::WrapMul => "wrapping_mul",
        Op::Min => "min",
        Op::Max => "max",
        _ => return None,
    })
}

/** How an operator is written. */
fn sym(op: Op) -> &'static str {
    match op {
        Op::Add => "+",
        Op::Sub => "-",
        Op::Mul => "*",
        Op::Div => "/",
        Op::Rem => "%",
        Op::And => "&",
        Op::Or => "|",
        Op::Xor => "^",
        Op::Lt => "<",
        Op::Le => "<=",
        Op::Gt => ">",
        Op::Ge => ">=",
        Op::Eq => "==",
        _ => "!=",
    }
}
