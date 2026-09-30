/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Widths and columns for text rendering: gutter digits, tab expansion, display columns. */

use alloc::string::String;

/** The number of decimal digits of `n`, which sets the gutter width. */
pub(super) fn digits(mut n: usize) -> usize {
    let mut d = 1;
    while n >= 10 {
        n /= 10;
        d += 1;
    }
    d
}

/** Tabs render as four spaces, so the underline lines up with the text above it. */
pub(super) fn expand_tabs(text: &str) -> String {
    text.replace('\t', "    ")
}

/**
 * The zero-based display column of one-based character column `col`, counting a tab as
 * four columns.
 */
pub(super) fn display_col(text: &str, col: usize) -> usize {
    text.chars()
        .take(col.saturating_sub(1))
        .map(|c| if c == '\t' { 4 } else { 1 })
        .sum()
}
