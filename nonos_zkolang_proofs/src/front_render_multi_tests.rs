/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A label over lines with other labels inside it shows each line once and in order, lines
 * left out inside it keep its connector, and a second file is introduced at its first line.
 */

use nonos_zkolang::compiler::diag::{render, Code, Diagnostic};
use nonos_zkolang::compiler::source::{SourceMap, Span};

use crate::front_render_check::show;

const CALL: &str = "fn main() {\n    let t = f(\n        a,\n        b,\n        c,\n    );\n}\n";

#[test]
fn labels_inside_a_label_over_lines_are_drawn_inside_it() {
    let (lo, hi) = (
        CALL.find("f(").unwrap() as u32,
        CALL.find(");").unwrap() as u32 + 2,
    );
    let b = CALL.find('b').unwrap() as u32;
    let got = show(
        CALL,
        (lo, hi, "call"),
        &[(b, b + 1, "inside"), (hi - 1, hi, "end")],
    );
    let want = "error[E0100]: m\n --> t.zkl:2:13\n  |\n2 |       let t = f(\n  |  _____________^\n3 | |         a,\n4 | |         b,\n  | |         - inside\n5 | |         c,\n6 | |     );\n  | |      - end\n  | |______^ call\n";
    assert_eq!(got, want);
}

#[test]
fn lines_left_out_inside_a_label_keep_its_connector() {
    let (lo, hi) = (
        CALL.find("f(").unwrap() as u32,
        CALL.find(");").unwrap() as u32 + 2,
    );
    let got = show(CALL, (lo, hi, "call"), &[]);
    let want = "error[E0100]: m\n --> t.zkl:2:13\n  |\n2 |       let t = f(\n  |  _____________^\n... |\n6 | |     );\n  | |______^ call\n";
    assert_eq!(got, want);
}

#[test]
fn a_second_file_is_introduced_at_its_first_shown_line() {
    let mut map = SourceMap::new();
    let main = map.add(String::from("main.zkl"), String::from("g(1, 2);\n"));
    let util = map.add(
        String::from("util.zkl"),
        String::from("fn g(a: u8) {}\nconst Q: u8 = 1;\n"),
    );
    let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, "m", Span::new(main, 0, 7), "call")
        .with_label(Span::new(util, 15, 20), "later")
        .with_label(Span::new(util, 0, 11), "first");
    let got = render(&map, &d);
    let want = "error[E0100]: m\n --> main.zkl:1:1\n  |\n1 | g(1, 2);\n  | ^^^^^^^ call\n  |\n ::: util.zkl:1:1\n  |\n1 | fn g(a: u8) {}\n  | ----------- first\n2 | const Q: u8 = 1;\n  | ----- later\n";
    assert_eq!(got, want);
}

#[test]
fn a_label_before_a_label_over_lines_on_its_line_shares_the_line() {
    let (lo, hi) = (
        CALL.find("f(").unwrap() as u32,
        CALL.find(");").unwrap() as u32 + 2,
    );
    let t = CALL.find("t =").unwrap() as u32;
    let got = show(CALL, (lo, hi, "call"), &[(t, t + 1, "bound")]);
    let want = "error[E0100]: m\n --> t.zkl:2:13\n  |\n2 |       let t = f(\n  |           - bound\n  |  _____________^\n... |\n6 | |     );\n  | |______^ call\n";
    assert_eq!(got, want);
}
