/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Naming a character in a message: as written when that is unambiguous, else by its code
 * point and, for the characters source text most often carries by accident, its name and
 * the ASCII character it is mistaken for.
 */

use alloc::format;
use alloc::string::String;

use super::describe_names::NAMED;
use crate::compiler::diag::has_form;

/** How a message names `c`. */
pub(super) fn describe(c: char) -> String {
    if c.is_ascii_graphic() && c != '`' {
        return format!("`{c}`");
    }
    let code = format!("U+{:04X}", c as u32);
    match NAMED.iter().find(|n| n.0 == c) {
        Some((_, name, _)) => format!("{code} {name}"),
        None if has_form(c) && !c.is_whitespace() => format!("`{c}` ({code})"),
        None => code,
    }
}

/**
 * The ASCII character `c` is easily mistaken for, if any. Every whitespace character the
 * lexer does not take as whitespace looks like a space.
 */
pub(super) fn looks_like(c: char) -> Option<char> {
    let named = NAMED.iter().find(|n| n.0 == c).and_then(|n| n.2);
    named.or_else(|| c.is_whitespace().then_some(' '))
}

/** Whether every character of `s` looks like a space: text a reader takes for whitespace. */
pub(super) fn space_like(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| looks_like(c) == Some(' '))
}
