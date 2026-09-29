/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where in the source a diagnostic points, and on which line. */

/** Whether a character ends a line: `\n`, or a carriage return alone or before `\n`. */
pub(super) fn is_break(c: char) -> bool {
    c == '\n' || c == '\r'
}

/**
 * The offset a caret goes at. An offset past the end, or inside a character, as one from
 * the include-expanded text rendered under a caller's own source can be, moves back to
 * the nearest character start. One at the very end of a file that ends in line breaks
 * moves back to the end of the last line, rather than onto a line the file does not have.
 */
pub(super) fn place(src: &str, at: usize) -> usize {
    let mut at = at.min(src.len());
    while !src.is_char_boundary(at) {
        at -= 1;
    }
    if at == src.len() {
        at = src.trim_end_matches(is_break).len();
    }
    at
}

/** The line `at` is on, counting from 1, with `\r\n` one break and a lone `\r` another. */
pub(super) fn line_number(src: &str, at: usize) -> usize {
    let b = src.as_bytes();
    let breaks = b[..at]
        .iter()
        .enumerate()
        .filter(|&(i, &c)| c == b'\n' || (c == b'\r' && b.get(i + 1) != Some(&b'\n')))
        .count();
    breaks + 1
}
