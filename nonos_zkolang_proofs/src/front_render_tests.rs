/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Rendered diagnostics: labels on one line share it, a label over several lines is drawn
 * with connectors, and lines between labels are elided.
 */

use crate::front_render_check::show;

#[test]
fn labels_on_one_line_share_it() {
    let text = "let c = 1 +* 2;\n";
    let got = show(text, (11, 12, "here"), &[(10, 11, "after this")]);
    let want = "error[E0100]: m\n --> t.zkl:1:12\n  |\n1 | let c = 1 +* 2;\n  |           - after this\n  |            ^ here\n";
    assert_eq!(got, want);
}

#[test]
fn a_label_over_lines_is_drawn_with_connectors() {
    let text = "fn f() {\n    let x = 1 + (return\n        5);\n}\n";
    let lo = text.find("return").unwrap() as u32;
    let hi = text.find('5').unwrap() as u32 + 1;
    let got = show(text, (lo, hi, "inside"), &[]);
    let want = "error[E0100]: m\n --> t.zkl:2:18\n  |\n2 |       let x = 1 + (return\n  |  __________________^\n3 | |         5);\n  | |_________^ inside\n";
    assert_eq!(got, want);
}

#[test]
fn lines_between_labels_are_elided() {
    let text = "a\nb\nc\nd\ne\n";
    let got = show(text, (8, 9, "p"), &[(0, 1, "s")]);
    assert_eq!(
        got,
        "error[E0100]: m\n --> t.zkl:5:1\n  |\n1 | a\n  | - s\n...\n5 | e\n  | ^ p\n"
    );
    let got = show(text, (4, 5, "p"), &[(0, 1, "s")]);
    assert_eq!(
        got,
        "error[E0100]: m\n --> t.zkl:3:1\n  |\n1 | a\n  | - s\n2 | b\n3 | c\n  | ^ p\n"
    );
}
