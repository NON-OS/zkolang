/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Checked programs run by the reference interpreter give the results sections 7 and 8 define. */

use nonos_zkolang::compiler::interp::{FailKind, Value};

use crate::sema_check::{int, run};

#[test]
fn integer_arithmetic_is_checked() {
    let src = "fn add(a: u8, b: u8) -> u8 { a + b }\nfn div(a: i8, b: i8) -> i8 { a / b }\nfn rem(a: i8, b: i8) -> i8 { a % b }\n";
    assert_eq!(run(src, "add", vec![int(200), int(55)]), Ok(int(255)));
    assert_eq!(
        run(src, "add", vec![int(200), int(56)]).unwrap_err().kind,
        FailKind::Overflow
    );
    assert_eq!(run(src, "div", vec![int(-7), int(2)]), Ok(int(-3)));
    assert_eq!(run(src, "rem", vec![int(-7), int(2)]), Ok(int(-1)));
    assert_eq!(
        run(src, "div", vec![int(-128), int(-1)]).unwrap_err().kind,
        FailKind::Overflow
    );
    assert_eq!(
        run(src, "rem", vec![int(-128), int(-1)]).unwrap_err().kind,
        FailKind::Overflow
    );
    assert_eq!(
        run(src, "div", vec![int(1), int(0)]).unwrap_err().kind,
        FailKind::DivideByZero
    );
}

#[test]
fn loops_and_mutation_compute_sums() {
    let src = "fn sum(n: u32) -> u32 {\n    let mut s = 0;\n    for i in 0..10 {\n        if i < n { s += i; }\n    }\n    s\n}\nfn count(x: u32) -> u32 {\n    let mut c = 0;\n    let mut v = x;\n    while v > 0 limit 32 {\n        v = v / 2;\n        c += 1;\n    }\n    c\n}\n";
    assert_eq!(run(src, "sum", vec![int(4)]), Ok(int(6)));
    assert_eq!(run(src, "count", vec![int(1000)]), Ok(int(10)));
}

#[test]
fn field_arithmetic_is_modular() {
    let src = "fn f(a: field) -> field { a * a.inv() }\nfn g(a: field) -> field { -a + 1 }\nfn h() -> field { 3.pow(2) }\n";
    assert_eq!(
        run(src, "f", vec![Value::Field(12345)]),
        Ok(Value::Field(1))
    );
    assert_eq!(run(src, "g", vec![Value::Field(1)]), Ok(Value::Field(0)));
    assert_eq!(
        run(src, "f", vec![Value::Field(0)]).unwrap_err().kind,
        FailKind::InverseOfZero
    );
    assert_eq!(run(src, "h", vec![]), Ok(Value::Field(9)));
}
