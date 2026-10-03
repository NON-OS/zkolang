/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The source map ends a line at a line feed, a carriage return or the two together, does
 * not count a leading byte-order mark as a column, and counts columns in characters
 * wherever they fall in a line.
 */

use nonos_zkolang::compiler::source::{FileId, SourceFile};

fn file(text: &str) -> SourceFile {
    SourceFile::new(FileId(0), String::from("t.zkl"), String::from(text))
}

#[test]
fn every_line_ending_starts_a_line() {
    let f = file("a\rb\r\nc\nd");
    let at = |c: char| f.line_col(f.text.find(c).unwrap() as u32);
    assert_eq!(
        [at('a'), at('b'), at('c'), at('d')],
        [(1, 1), (2, 1), (3, 1), (4, 1)]
    );
    assert_eq!(f.line_count(), 4);
    assert_eq!(
        [f.line_text(1), f.line_text(2), f.line_text(3)],
        ["a", "b", "c"]
    );
}

#[test]
fn a_byte_order_mark_is_not_a_column() {
    let f = file("\u{feff}ab\nc");
    assert_eq!(f.line_col(4), (1, 2));
    assert_eq!(f.line_col(0), (1, 1));
    assert_eq!(f.line_text(1), "ab");
    assert_eq!(f.line_offset(1), 3);
}

#[test]
fn columns_count_characters_across_long_lines() {
    let mut text = String::new();
    for i in 0..3000 {
        text.push(if i % 7 == 0 {
            'é'
        } else if i % 11 == 0 {
            '日'
        } else {
            'x'
        });
        if i % 1000 == 999 {
            text.push('\n');
        }
    }
    let f = file(&text);
    let mut line = 1;
    let mut col = 1;
    for (i, c) in text.char_indices() {
        assert_eq!(f.line_col(i as u32), (line, col), "offset {i}");
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    assert_eq!(f.line_col(text.len() as u32), (line, col));
}
