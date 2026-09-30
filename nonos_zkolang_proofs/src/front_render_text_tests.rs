/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Rendered source text: nothing reaches the terminal as a control sequence, a caret stays
 * under the character it marks whatever comes before it, and a long line is cut to a
 * window around the mark.
 */

use crate::front_render_check::show;

#[test]
fn source_text_cannot_drive_the_terminal() {
    let text = "x @ // \u{1b}[2J \u{202e}\n";
    let got = show(text, (2, 3, "here"), &[]);
    assert!(
        !got.contains('\u{1b}') && !got.contains('\u{202e}'),
        "{got:?}"
    );
    assert!(got.contains("\u{241b}[2J <U+202E>"), "{got:?}");
}

#[test]
fn carets_sit_under_wide_and_combining_characters() {
    let text = "\"日本\" e\u{301} @\n";
    let at = text.find('@').unwrap() as u32;
    let got = show(text, (at, at + 1, "here"), &[]);
    let lines: Vec<&str> = got.lines().collect();
    assert_eq!(lines[3], "1 | \"日本\" e\u{301} @");
    assert_eq!(lines[4], format!("  | {}^ here", " ".repeat(9)));
}

#[test]
fn a_long_line_is_cut_to_a_window() {
    let text = format!("{}@{}\n", "a ".repeat(5000), " b".repeat(5000));
    let at = text.find('@').unwrap() as u32;
    let got = show(&text, (at, at + 1, "here"), &[]);
    assert!(got.len() < 400, "{} bytes", got.len());
    let lines: Vec<&str> = got.lines().collect();
    assert!(
        lines[3].starts_with("1 | ...a a") && lines[3].ends_with("b b..."),
        "{got}"
    );
    assert_eq!(lines[4].find('^'), lines[3].find('@'));
}
