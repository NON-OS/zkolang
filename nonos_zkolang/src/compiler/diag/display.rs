/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * How source text and messages reach a terminal. A tab shows as four spaces, a control
 * character as its Unicode control picture, and a character that is invisible or reorders
 * the text around it as `<U+XXXX>`, so no source can clear the screen, recolour it, or make
 * a line read differently from what the compiler sees. Every other character is shown as
 * written, and its width is the number of columns a terminal gives it.
 */

use alloc::format;
use alloc::string::String;

use super::display_width::{invisible, width};

/** Append how `c` is shown and return the number of columns it takes. */
pub(super) fn cell(c: char, out: &mut String) -> usize {
    match c {
        '\t' => {
            out.push_str("    ");
            4
        }
        '\0'..='\x1f' => {
            out.push(char::from_u32(0x2400 + c as u32).unwrap_or('?'));
            1
        }
        '\x7f' => {
            out.push('\u{2421}');
            1
        }
        _ if invisible(c) => {
            let shown = format!("<U+{:04X}>", c as u32);
            out.push_str(&shown);
            shown.len()
        }
        _ => {
            out.push(c);
            width(c)
        }
    }
}

/** The text as shown, for messages, notes and file names. */
pub(super) fn visible(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        cell(c, &mut out);
    }
    out
}

/** The number of columns `s` takes when shown. */
pub(super) fn columns(s: &str) -> usize {
    let mut scratch = String::new();
    s.chars()
        .map(|c| {
            scratch.clear();
            cell(c, &mut scratch)
        })
        .sum()
}

/** The largest character boundary of `s` at or before `at`. */
pub(super) fn floor(s: &str, mut at: usize) -> usize {
    while at > 0 && !s.is_char_boundary(at) {
        at -= 1;
    }
    at
}
