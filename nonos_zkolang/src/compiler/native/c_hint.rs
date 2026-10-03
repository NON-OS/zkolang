/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A value and an advice hint as C expressions. */

use alloc::format;
use alloc::string::String;

use crate::compiler::ssa::{Hint, V};

/** Value `x` as its C local. */
pub(super) fn v(x: V) -> String {
    format!("v{}", x.0)
}

/** The C expression that finds the advice value of hint `h`. */
pub(super) fn hint(h: Hint) -> String {
    match h {
        Hint::Bit(a, k) => format!("bit({}, {k})", v(a)),
        Hint::Quot(a, b) => format!("{1} ? {0} / {1} : 0", v(a), v(b)),
        Hint::Rem(a, b) => format!("{1} ? {0} % {1} : {0}", v(a), v(b)),
        Hint::Div64 { a, b, part } => {
            let (a, b) = ((v(a.0), v(a.1)), (v(b.0), v(b.1)));
            format!("div64({}, {}, {}, {}, {part})", a.0, a.1, b.0, b.1)
        }
    }
}
