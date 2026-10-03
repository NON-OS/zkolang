/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One SSA instruction as Python: value `i` is the local `v{i}`. */

use alloc::format;
use alloc::string::String;

use crate::compiler::ssa::{Hint, Inst, V};

fn hint(h: Hint) -> String {
    match h {
        Hint::Bit(a, k) => format!("_bit(v{}, {k})", a.0),
        Hint::Quot(a, b) => format!("v{0} // v{1} if v{1} else 0", a.0, b.0),
        Hint::Rem(a, b) => format!("v{0} % v{1} if v{1} else v{0}", a.0, b.0),
        Hint::Div64 { a, b, part } => {
            format!(
                "_div64(v{}, v{}, v{}, v{}, {part})",
                a.0 .0, a.1 .0, b.0 .0, b.1 .0
            )
        }
    }
}

/** The Python statement that computes instruction `i`, `inst`. */
pub(super) fn inst(i: usize, inst: Inst) -> String {
    let v = |x: V| format!("v{}", x.0);
    let check = |ok: String| format!("_check({ok}, \"instruction {i}\")");
    match inst {
        Inst::Const(c) => format!("v{i} = {c}"),
        Inst::Input(k) => format!("v{i} = inp[{k}]"),
        Inst::Advice(h) => format!("v{i} = {}", hint(h)),
        Inst::Add(a, b) => format!("v{i} = _add({}, {})", v(a), v(b)),
        Inst::Sub(a, b) => format!("v{i} = _sub({}, {})", v(a), v(b)),
        Inst::Mul(a, b) => format!("v{i} = _mul({}, {})", v(a), v(b)),
        Inst::Inv(a) => format!("v{i} = _inv({})", v(a)),
        Inst::Sel(c, a, b) => format!("v{i} = _sel({}, {}, {})", v(c), v(a), v(b)),
        Inst::Eq(a, b) => format!("v{i} = int({} == {})", v(a), v(b)),
        Inst::AssertBool(a) => check(format!("{} <= 1", v(a))),
        Inst::AssertZero(a) => check(format!("{} == 0", v(a))),
        Inst::Output(k, a) => format!("out[{k}] = {}", v(a)),
        Inst::RangeCheck(a, n) => check(format!("_below({}, {n})", v(a))),
        Inst::Bit(a, k, n) => format!(
            "{}; v{i} = _bit({}, {k})",
            check(format!("_below({}, {n})", v(a))),
            v(a)
        ),
        Inst::FieldBit(a, k) => format!("v{i} = _bit({}, {k})", v(a)),
        Inst::Quot(a, b, n) | Inst::Rem(a, b, n) => {
            let op = if matches!(inst, Inst::Quot(..)) {
                "//"
            } else {
                "%"
            };
            let (a, b) = (v(a), v(b));
            let ok = format!("{n} <= 32 and _below({a}, {n}) and _below({b}, {n}) and {b} != 0");
            format!("{}; v{i} = {a} {op} {b}", check(ok))
        }
    }
}
