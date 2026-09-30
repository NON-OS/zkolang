/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The reference interpreter against a model written apart from it: random expressions of
 * every integer type, over parameters given edge and random values, must give the model's
 * value, or fail where the model fails and for the same reason.
 */

use nonos_zkolang::compiler::interp::{FailKind, Value};

use crate::prop_binary::Fail;
use crate::prop_expr::eval;
use crate::prop_gen::Rng;
use crate::prop_gen_expr::expr;
use crate::prop_model::TYPES;
use crate::prop_print::print;
use crate::sema_check::{call, checked, int};

/** Programs per type, and runs per program. */
const PROGRAMS: usize = 250;
const RUNS: usize = 12;

/** The failure kind of the interpreter that stands for `f`. */
fn kind(f: Fail) -> FailKind {
    match f {
        Fail::Overflow => FailKind::Overflow,
        Fail::DivideByZero => FailKind::DivideByZero,
        Fail::ShiftTooFar => FailKind::ShiftTooFar,
        Fail::InverseOfZero => FailKind::InverseOfZero,
    }
}

#[test]
fn the_interpreter_agrees_with_the_model_on_integers() {
    let mut r = Rng(0x9E37_79B9_7F4A_7C15);
    let mut problems = Vec::new();
    for t in TYPES {
        for _ in 0..PROGRAMS {
            let e = expr(&mut r, t, 4);
            let body = print(&e, t);
            let src = format!("fn f(a: {0}, b: {0}, k: u32) -> {0} {{ {body} }}\n", t.name);
            let (program, codes) = checked(&src);
            if codes.iter().any(|c| c.starts_with('E')) {
                problems.push(format!("{src}does not check: {codes:?}"));
                continue;
            }
            for _ in 0..RUNS {
                let args = (
                    r.value(t),
                    r.value(t),
                    r.below(u64::from(t.bits) + 2) as u32,
                );
                let want = eval(&e, t, args);
                let argv = vec![int(args.0), int(args.1), int(i128::from(args.2))];
                let got = call(&program, "f", argv).map_err(|f| f.kind);
                if got != want.map(Value::Int).map_err(kind) {
                    problems.push(format!(
                        "{src}on {args:?}: model {want:?}, interpreter {got:?}"
                    ));
                }
            }
        }
    }
    let shown: Vec<&String> = problems.iter().take(8).collect();
    assert!(
        problems.is_empty(),
        "{} disagreements, first {shown:#?}",
        problems.len()
    );
}
