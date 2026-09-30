/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The value E0400 names as not covered is written as a pattern a program can use: added
 * as an arm, it is covered, and a `match` built that way ends up covering every value.
 */

use crate::ui_run::report;

const PROGRAM: &str = "#![allow(unused)]
enum Shape<T> {
    Dot,
    Line(T),
    Area { w: T, h: bool },
}

struct Pair<T> {
    a: T,
    b: bool,
}

fn f(x: TYPE) -> u32 {
    match x {
        ARMS
    }
}
";

/** The pattern the first E0400 of `rendered` names as not covered. */
fn missing(rendered: &str) -> Option<String> {
    let rest = rendered.split_once("the `match` does not cover `")?.1;
    Some(String::from(rest.split_once('`')?.0))
}

/** The arms a `match` on `ty` after the arm `first` takes, each one what the one before left. */
fn cover(ty: &str, first: &str) -> Vec<String> {
    let mut arms: Vec<String> = Vec::new();
    for _ in 0..8 {
        let written: String = arms.iter().map(|a| format!("{a} => 0,\n")).collect();
        let src = PROGRAM.replace("TYPE", ty);
        let got = report(
            "sema/witness.zkl",
            &src.replace("ARMS", &(first.to_owned() + &written)),
        );
        let Some(m) = missing(&got.rendered) else {
            assert!(got.lines.is_empty(), "{}", got.rendered);
            return arms;
        };
        arms.push(m);
    }
    panic!("no cover for {ty} after 8 arms: {arms:?}");
}

#[test]
fn a_missing_value_is_named_by_a_pattern_that_covers_it() {
    let shapes = ["Shape::Dot", "Shape::Line(_)", "Shape::Area { .. }"];
    assert_eq!(cover("Shape<u8>", ""), shapes);
    let pairs = ["Pair { a: _, b: false }"];
    assert_eq!(cover("Pair<u8>", "Pair { a: _, b: true } => 1,\n"), pairs);
    assert_eq!(cover("Option<u8>", ""), ["Option::None", "Option::Some(_)"]);
}
