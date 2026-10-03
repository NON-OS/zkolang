/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * W0102 (section 15.3): a function of the program's own crate whose rows, with those it
 * inlines, are more than half of what a proof holds is warned of, unless it allows `cost`.
 */

use crate::prove_2026_tests::built;

/** A program whose `heavy` unrolls to about 34 000 rows, under the attributes given. */
fn program(heavy: &str, main: &str) -> String {
    format!("{heavy}fn heavy(x: field) -> field {{\n    let mut y = x;\n    for _ in 0..17000 {{\n        y = y * y + x;\n    }}\n    y\n}}\n\n{main}fn main(a: public field) -> field {{\n    heavy(a)\n}}\n")
}

/** The W0102 warnings built `src` gives, by the line of each. */
fn warned(src: &str) -> Vec<usize> {
    let b = built(src);
    let lines = b.warnings.items().iter().filter(|d| d.code.0 == "W0102");
    lines
        .map(|d| src[..d.span().lo as usize].matches('\n').count() + 1)
        .collect()
}

#[test]
fn a_function_past_half_the_rows_is_warned_of() {
    /* The attribute is written in two parts, so no line holds it whole. */
    let allow = ["#", "[allow(cost)]\n"].concat();
    assert_eq!(warned(&program("", "")), [1, 9]);
    assert_eq!(warned(&program(&allow, "")), [10]);
    assert_eq!(warned(&program(&allow, &allow)), Vec::<usize>::new());
}
