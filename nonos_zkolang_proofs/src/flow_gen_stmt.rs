/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A random statement of a generated program, indented for its depth. */

use crate::flow_gen::{expr, Scope};
use crate::flow_gen_cond::cond;
use crate::prop_gen::Rng;

/** A random statement at nesting `depth`. */
pub(crate) fn stmt(r: &mut Rng, scope: &mut Scope, depth: u32) -> String {
    let pad = "    ".repeat(depth as usize);
    let target = scope.vars[r.below(scope.vars.len() as u64) as usize].clone();
    let nested = depth < 3 && scope.depth < 2;
    let body = |r: &mut Rng, scope: &mut Scope| {
        let n = 1 + r.below(3);
        (0..n)
            .map(|_| stmt(r, scope, depth + 1))
            .collect::<String>()
    };
    match r.below(12) {
        0 | 1 if nested => {
            let (c, t) = (cond(r, scope), body(r, scope));
            let e = body(r, scope);
            format!("{pad}if {c} {{\n{t}{pad}}} else {{\n{e}{pad}}}\n")
        }
        2 if nested => {
            let v = format!("i{depth}");
            let (was, depth_was) = (scope.in_loop, scope.depth);
            scope.vars.push(v.clone());
            scope.in_loop = true;
            scope.depth += 1;
            let b = body(r, scope);
            scope.vars.pop();
            scope.in_loop = was;
            scope.depth = depth_was;
            format!("{pad}for {v} in 0u32..{} {{\n{b}{pad}}}\n", 1 + r.below(3))
        }
        3 if nested && !scope.vars.iter().any(|v| v.starts_with('i')) => {
            let (was, depth_was) = (scope.in_loop, scope.depth);
            scope.in_loop = true;
            scope.depth += 1;
            let b = body(r, scope);
            scope.in_loop = was;
            scope.depth = depth_was;
            format!("{pad}while {target} < {} limit 4 {{\n{pad}    {target} = {target} + 1;\n{b}{pad}}}\n", r.below(5))
        }
        4 if scope.in_loop => format!(
            "{pad}if {} {{ {} }}\n",
            cond(r, scope),
            ["break;", "continue;"][r.below(2) as usize]
        ),
        5 => format!(
            "{pad}if {} {{ return {}; }}\n",
            cond(r, scope),
            expr(r, scope, 1)
        ),
        6 => format!(
            "{pad}arr[({} % 4) as usize] = {};\n",
            expr(r, scope, 1),
            expr(r, scope, 2)
        ),
        7 if !target.starts_with('i') => format!(
            "{pad}{target} = bump(&mut {target}, {});\n",
            expr(r, scope, 1)
        ),
        8 if r.below(3) == 0 => format!("{pad}assert {};\n", cond(r, scope)),
        _ if target.starts_with('i') => format!("{pad}v0 = {};\n", expr(r, scope, 2)),
        _ => format!("{pad}{target} = {};\n", expr(r, scope, 2)),
    }
}
