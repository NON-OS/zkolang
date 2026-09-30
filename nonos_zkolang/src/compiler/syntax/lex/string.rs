/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * String literals, which appear only as `assert` messages and attribute values. A string
 * ends at its closing quote and may not span lines.
 */

use alloc::string::String;

use crate::compiler::diag::Code;

/**
 * Scan a string literal whose opening quote is at `i`. Returns the offset after it and,
 * when it is malformed, the code, message and offset of the first problem.
 */
pub(super) fn scan_string(
    b: &[u8],
    i: usize,
    len: usize,
) -> (usize, Option<(Code, &'static str, usize)>) {
    let mut j = i + 1;
    let mut problem = None;
    while j < len {
        match b[j] {
            b'"' => return (j + 1, problem),
            b'\n' => break,
            b'\\' => {
                if !matches!(b.get(j + 1), Some(b'"' | b'\\' | b'n' | b't' | b'0'))
                    && problem.is_none()
                {
                    problem = Some((Code::BAD_ESCAPE, "unknown escape in a string literal", j));
                }
                j += 2;
            }
            _ => j += 1,
        }
    }
    let end = j.min(len);
    (
        end,
        Some((Code::UNTERMINATED_STRING, "unterminated string literal", i)),
    )
}

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
