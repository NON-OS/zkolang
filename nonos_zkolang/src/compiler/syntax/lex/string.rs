/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The value of a string literal. */

use alloc::string::String;

/**
 * The value of a string literal's text, quotes included, with escapes resolved. The lexer
 * has already refused a malformed one; an unknown escape here reads as the escaped byte.
 */
pub fn str_literal(text: &str) -> String {
    let inner = text
        .strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .unwrap_or(text);
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('0') => out.push('\0'),
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}
