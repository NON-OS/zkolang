/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Characters take the columns the Unicode tables give them, and a character of no width
 * that a diagnostic is about is shown, and named, by its code.
 */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;

use crate::front_render_check::{rows, show};

#[test]
fn characters_take_the_columns_unicode_gives_them() {
    for (text, cols) in [("🚀", 2), ("✅ ❌", 5), ("नमस्ते", 4), ("กิน", 2), ("🪐", 2)]
    {
        let text = format!("/* {text} */ @\n");
        let at = text.find('@').unwrap() as u32;
        let (_, marks) = rows(&show(&text, (at, at + 1, "here"), &[]));
        assert_eq!(
            marks,
            format!("  | {}^ here", " ".repeat(cols + 7)),
            "{text}"
        );
    }
}

#[test]
fn a_mark_on_a_character_of_no_width_shows_its_code() {
    let text = "let y\u{fe0f} = 2; e\u{301}\n";
    let got = show(text, (5, 8, "here"), &[]);
    let (source, marks) = rows(&got);
    assert_eq!(source, "1 | let y<U+FE0F> = 2; e\u{301}");
    assert_eq!(marks, "  |      ^^^^^^^^ here");
    let at = text.find('\u{301}').unwrap() as u32;
    let (source, marks) = rows(&show(text, (at, at + 2, "here"), &[]));
    assert!(
        source.ends_with("e<U+0301>") && marks.ends_with("^^^^^^^^ here"),
        "{source}"
    );
}

#[test]
fn a_character_of_no_width_is_named_by_its_code_alone() {
    for (src, want) in [
        ("y\u{fe0f}", "U+FE0F"),
        ("e\u{301}", "U+0301"),
        ("e\u{e9}", "`é` (U+00E9)"),
    ] {
        let mut map = SourceMap::new();
        let id = map.add(String::from("t.zkl"), String::from(src));
        let mut diags = Diagnostics::new();
        lex(id, src, &mut diags);
        assert_eq!(
            diags.items()[0].message,
            format!("unexpected character {want}")
        );
    }
}

#[test]
fn every_default_ignorable_character_shows_its_code() {
    for c in ['\u{1d173}', '\u{1bca0}', '\u{fff0}', '\u{e0080}'] {
        let text = format!("x {c} y\n");
        let got = show(&text, (2, 2 + c.len_utf8() as u32, "here"), &[]);
        let (source, _) = rows(&got);
        assert_eq!(source, format!("1 | x <U+{:04X}> y", c as u32));
    }
}
