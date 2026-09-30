/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The window a long line is cut to holds what it marks, whatever takes the columns before
 * it, and labels too far apart for one window each get their own.
 */

use crate::front_render_check::{rows, show};

#[test]
fn a_mark_after_wide_text_stays_in_the_window() {
    let tabs = format!(
        "{}let y = @; {}\n",
        "\t".repeat(40),
        "let z = 1; ".repeat(10)
    );
    let escapes = format!(
        "/* {} */ let y = @; {}\n",
        "\u{200b}".repeat(20),
        "z; ".repeat(30)
    );
    for text in [tabs, escapes] {
        let at = text.find('@').unwrap() as u32;
        let (source, marks) = rows(&show(&text, (at, at + 1, "here"), &[]));
        assert_eq!(marks.find('^'), source.find('@'), "{source}\n{marks}");
    }
}

#[test]
fn labels_far_apart_on_a_long_line_are_each_shown() {
    let text = format!(
        "{}a{}b{}\n",
        "x ".repeat(60),
        " ".repeat(56),
        " y".repeat(60)
    );
    let (a, b) = (
        text.find('a').unwrap() as u32,
        text.find('b').unwrap() as u32,
    );
    let got = show(&text, (b, b + 1, "second"), &[(a, a + 1, "first")]);
    let lines: Vec<&str> = got.lines().collect();
    let (first, second) = (lines[3], lines[5]);
    assert_eq!(lines[4].find('-'), first.find('a'), "{got}");
    assert_eq!(lines[6].find('^'), second.find('b'), "{got}");
    assert!(
        lines[4].ends_with("first") && lines[6].ends_with("second"),
        "{got}"
    );
}
