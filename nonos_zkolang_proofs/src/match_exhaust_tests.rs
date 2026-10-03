/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Exhaustiveness (section 9.3) against listing every value: for random `match`es over
 * `(E, u8)`, the checker reports E0400 exactly when some value matches no arm without a
 * guard, naming a pattern that such a value matches, and W0004 exactly on each arm that
 * every value it matches reaches only after an earlier arm without a guard.
 */

use crate::match_gen::{Pat, Val};
use crate::match_pat::every_value;
use crate::match_rng::Rng;
use crate::match_witness::parse;
use crate::ui_run::report;

/** The arms before the first arm's line, and the line of `match`. */
const HEAD: &str = "#![allow(unused)]\nenum E { A, B(bool), C(u8) }\nfn f(x: (E, u8), c: bool) -> u8 {\n    match x {\n";

#[test]
fn exhaustiveness_agrees_with_every_value() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let values: Vec<Val> = every_value().collect();
    for round in 0..120 {
        let n = 1 + rng.below(5) as usize;
        let arms: Vec<(Pat, bool)> = (0..n)
            .map(|_| {
                let p = match rng.below(8) {
                    0 => Pat::Wild,
                    _ => Pat::Pair(Box::new(rng.pat(0, 0)), Box::new(rng.pat(2, 0))),
                };
                (p, rng.below(5) == 0)
            })
            .collect();
        let mut src = String::from(HEAD);
        for (p, guarded) in &arms {
            let guard = if *guarded { " if c" } else { "" };
            src.push_str(&format!("        {}{guard} => 0,\n", p.text()));
        }
        src.push_str("    }\n}\n");
        let mut want: Vec<(usize, String)> = Vec::new();
        let taken = |i: usize, v: Val| arms[..i].iter().any(|(q, g)| !g && q.matches(v));
        let missing: Vec<Val> = values.iter().copied().filter(|&v| !taken(n, v)).collect();
        if !missing.is_empty() {
            want.push((4, String::from("E0400")));
        }
        for (i, (p, _)) in arms.iter().enumerate() {
            if !values.iter().any(|&v| p.matches(v) && !taken(i, v)) {
                want.push((5 + i, String::from("W0004")));
            }
        }
        want.sort();
        let got = report("sema/gen.zkl", &src);
        assert_eq!(got.lines, want, "round {round}\n{src}\n{}", got.rendered);
        if let Some(w) = got.rendered.split("does not cover `").nth(1) {
            let w = parse(w.split('`').next().unwrap_or(""));
            let w = w.unwrap_or_else(|| panic!("witness in {}", got.rendered));
            assert!(
                missing.iter().any(|&v| w.matches(v)),
                "{src}\n{}",
                got.rendered
            );
        }
    }
}
