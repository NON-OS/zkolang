/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The cost report (section 15.2): every row is a function's own or outside all, each
 * function's inclusive rows hold its own and its callees', and rows land on the function
 * and line whose code they are.
 */

use nonos_zkolang::compiler::driver::{cost, Built, Cost, FnCost};

use crate::prove_2026_tests::built;

pub(crate) const PROGRAM: &str = "fn heavy(x: field) -> field {
    let mut y = x;
    for _ in 0..16 {
        y = y * y + x;
    }
    y
}

fn light(x: field) -> field {
    x + 1
}

fn both(x: field) -> field {
    heavy(x) + light(x)
}

fn main(a: public field, b: public field) -> field {
    both(a) + heavy(b) + light(b)
}
";

/** The cost of the function `name` of `b`. */
fn of(b: &Built, c: &Cost, name: &str) -> FnCost {
    let f = b.program.fns.iter().position(|f| f.name == name);
    let f = f.and_then(|f| c.fns.iter().find(|(id, _)| id.0 as usize == f));
    f.map_or(FnCost::default(), |(_, k)| *k)
}

#[test]
fn each_row_is_counted_once_and_inclusive_rows_hold_the_callees() {
    let b = built(PROGRAM);
    let c = cost(&b);
    let own: usize = c.fns.values().map(|k| k.exclusive).sum();
    assert_eq!(own + c.overhead, c.rows);
    assert_eq!(of(&b, &c, "main").inclusive + c.overhead, c.rows);
    let (heavy, light, both) = (of(&b, &c, "heavy"), of(&b, &c, "light"), of(&b, &c, "both"));
    assert_eq!(heavy.inclusive, heavy.exclusive);
    assert!(
        heavy.exclusive > 10 * light.exclusive && light.exclusive > 0,
        "{c:?}"
    );
    assert!(
        both.inclusive >= both.exclusive + heavy.exclusive / 2,
        "{c:?}"
    );
    assert!(c
        .fns
        .values()
        .all(|k| k.inclusive >= k.exclusive && k.peak <= c.peak));
    assert!(c.peak > 0 && c.peak <= 32);
}
