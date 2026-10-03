/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One SSA instruction as C: value `i` is the local `v{i}`. */

use alloc::format;
use alloc::string::String;

use super::c_hint::{hint, v};
use crate::compiler::ssa::Inst;

/** What fails the run at `inst`, if anything can, and the value it defines, if any. */
fn parts(inst: Inst) -> (Option<String>, Option<String>) {
    let (a, b) = (Option::<String>::None, Option::<String>::None);
    match inst {
        Inst::Const(c) => (a, Some(format!("{c}ULL"))),
        Inst::Input(k) => (a, Some(format!("in[{k}]"))),
        Inst::Advice(h) => (a, Some(hint(h))),
        Inst::Add(x, y) => (a, Some(format!("fadd({}, {})", v(x), v(y)))),
        Inst::Sub(x, y) => (a, Some(format!("fsub({}, {})", v(x), v(y)))),
        Inst::Mul(x, y) => (a, Some(format!("fmul({}, {})", v(x), v(y)))),
        Inst::Inv(x) => (
            Some(format!("{} == 0", v(x))),
            Some(format!("finv({})", v(x))),
        ),
        Inst::Sel(c, x, y) => {
            let pick = format!("{} ? {} : {}", v(c), v(x), v(y));
            (Some(format!("{} > 1", v(c))), Some(pick))
        }
        Inst::Eq(x, y) => (a, Some(format!("(u64)({} == {})", v(x), v(y)))),
        Inst::AssertBool(x) => (Some(format!("{} > 1", v(x))), b),
        Inst::AssertZero(x) => (Some(format!("{} != 0", v(x))), b),
        Inst::Output(..) => (a, b),
        Inst::RangeCheck(x, n) => (Some(format!("!below({}, {n})", v(x))), b),
        Inst::Bit(x, k, n) => (
            Some(format!("!below({}, {n})", v(x))),
            Some(format!("bit({}, {k})", v(x))),
        ),
        Inst::FieldBit(x, k) => (a, Some(format!("bit({}, {k})", v(x)))),
        Inst::Quot(x, y, n) | Inst::Rem(x, y, n) => {
            let op = if matches!(inst, Inst::Quot(..)) {
                '/'
            } else {
                '%'
            };
            let (x, y) = (v(x), v(y));
            let check = format!("{n} > 32 || !below({x}, {n}) || !below({y}, {n}) || {y} == 0");
            (Some(check), Some(format!("{x} {op} {y}")))
        }
    }
}

/** Whether `inst` checks something that can fail the run. */
pub(super) fn fails(inst: Inst) -> bool {
    parts(inst).0.is_some()
}

/**
 * The C statements of instruction `i`, `inst`, one per line: the check that fails the
 * run, then the value, declared only when it is `live`.
 */
pub(super) fn inst(i: usize, inst: Inst, live: bool) -> alloc::vec::Vec<String> {
    if let Inst::Output(k, x) = inst {
        return alloc::vec![format!("out[{k}] = {};", v(x))];
    }
    let (check, value) = parts(inst);
    let check = check.map(|c| format!("if ({c}) FAIL({i});"));
    let value = value.filter(|_| live).map(|e| format!("u64 v{i} = {e};"));
    check.into_iter().chain(value).collect()
}
