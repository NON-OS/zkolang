/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Positions as the protocol counts them: a line, and the UTF-16 code units before a point
 * on its line. A line ends at a line feed, a carriage return, or the two together.
 */

use super::json::Json;

/** The position of byte `at` of `text`, clamped to its end. */
pub(crate) fn position(text: &str, at: usize) -> Json {
    let (mut line, mut col) = (0, 0);
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if i >= at {
            break;
        }
        match c {
            '\n' => (line, col) = (line + 1, 0),
            '\r' => {
                if chars.peek().is_some_and(|(_, n)| *n == '\n') {
                    chars.next();
                }
                (line, col) = (line + 1, 0);
            }
            c => col += c.len_utf16(),
        }
    }
    Json::obj(vec![
        ("line", Json::num(line)),
        ("character", Json::num(col)),
    ])
}

/** The range from byte `lo` to byte `hi` of `text`. */
pub(crate) fn range(text: &str, lo: usize, hi: usize) -> Json {
    Json::obj(vec![
        ("start", position(text, lo)),
        ("end", position(text, hi)),
    ])
}
