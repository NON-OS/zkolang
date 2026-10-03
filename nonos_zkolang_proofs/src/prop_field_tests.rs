/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The reference interpreter against the `field` model: random expressions over edge and
 * random elements must give the model's value, or fail where it fails and why.
 */

use nonos_zkolang::compiler::interp::{FailKind, Value};

use crate::prop_binary::Fail;
use crate::prop_field::eval;
use crate::prop_field_gen::{expr, print, value};
use crate::prop_gen::Rng;
use crate::sema_check::{call, checked};

#[test]
fn the_interpreter_agrees_with_the_model_on_fields() {
    let mut r = Rng(0xD1B5_4A32_D192_ED03);
    let mut problems = Vec::new();
    for _ in 0..600 {
        let e = expr(&mut r, 4);
        let src = format!(
            "fn f(a: field, b: field) -> field {{ let z: field = 0; {} }}\n",
            print(&e)
        );
        let (program, codes) = checked(&src);
        if codes.iter().any(|c| c.starts_with('E')) {
            problems.push(format!("{src}does not check: {codes:?}"));
            continue;
        }
        for _ in 0..8 {
            let args = (value(&mut r), value(&mut r));
            let want = eval(&e, args)
                .map(|v| Value::Field(v as u64))
                .map_err(|f| match f {
                    Fail::DivideByZero => FailKind::DivideByZero,
                    Fail::InverseOfZero => FailKind::InverseOfZero,
                    _ => FailKind::Overflow,
                });
            let argv = vec![Value::Field(args.0), Value::Field(args.1)];
            let got = call(&program, "f", argv).map_err(|f| f.kind);
            if got != want {
                problems.push(format!(
                    "{src}on {args:?}: model {want:?}, interpreter {got:?}"
                ));
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
