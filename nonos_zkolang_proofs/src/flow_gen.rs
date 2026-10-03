/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Random zKølang programs with control flow, as source text: `main` over two `u32` inputs
 * and a `[u32; 4]`, with assignments, `if`, `for`, `while`, `break`, `continue`, an
 * early `return`, writes through run-time indices, `&mut` calls and `assert`s.
 */

use crate::prop_gen::Rng;

/** The variables in scope and whether a loop encloses the point. */
pub(crate) struct Scope {
    pub vars: Vec<String>,
    pub in_loop: bool,
    pub depth: u32,
}

/** A random program. */
pub(crate) fn program(r: &mut Rng) -> String {
    let mut s =
        String::from("fn bump(x: &mut u32, by: u32) -> u32 { x = x.wrapping_add(by); x ^ by }\n");
    s.push_str("fn main(a: public u32, b: public u32, c: public [u32; 4]) -> public u32 {\n");
    s.push_str("    let mut v0: u32 = a;\n    let mut v1: u32 = b;\n    let mut arr = c;\n");
    let mut scope = Scope {
        vars: vec!["v0".into(), "v1".into()],
        in_loop: false,
        depth: 0,
    };
    for _ in 0..1 + r.below(6) {
        s.push_str(&crate::flow_gen_stmt::stmt(r, &mut scope, 1));
    }
    s.push_str("    v0.wrapping_add(v1).wrapping_add(arr[0]) ^ arr[3]\n}\n");
    s
}

/** A random `u32` expression over the variables in scope. */
pub(crate) fn expr(r: &mut Rng, scope: &Scope, depth: u32) -> String {
    if depth == 0 || r.below(3) == 0 {
        return match r.below(5) {
            0 => format!("{}u32", r.below(20)),
            1 => ["a", "b"][r.below(2) as usize].to_string(),
            2 => format!("arr[({} % 4) as usize]", expr(r, scope, 0)),
            _ => scope.vars[r.below(scope.vars.len() as u64) as usize].clone(),
        };
    }
    let (x, y) = (expr(r, scope, depth - 1), expr(r, scope, depth - 1));
    match r.below(10) {
        0 => format!("({x}).wrapping_mul({y})"),
        1 => format!("({x} ^ {y})"),
        2 => format!("({x} & {y})"),
        3 => format!("({x} >> {}u32)", r.below(32)),
        4 => format!("({x} / ({y} | 1u32))"),
        5 => format!("({x} + {y})"),
        6 => format!("({x}).min({y})"),
        _ => format!("({x}).wrapping_add({y})"),
    }
}
