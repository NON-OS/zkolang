/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Recognise an include directive.

/// The path of an include directive on its own line, `include "path";`, or `None`
/// for an ordinary line. A trailing comment is allowed. Keeping the directive to one
/// line makes it a plain text step that runs before the lexer sees the source.
pub(super) fn include_path(line: &str) -> Option<&str> {
    let rest = line.trim().strip_prefix("include")?;
    if !rest.starts_with(|c: char| c.is_whitespace()) {
        return None;
    }
    let rest = rest.trim_start().strip_prefix('"')?;
    let end = rest.find('"')?;
    let path = &rest[..end];
    let after = rest[end + 1..].trim_start();
    let after = after.strip_prefix(';').unwrap_or(after).trim_start();
    if after.is_empty() || after.starts_with("//") {
        Some(path)
    } else {
        None
    }
}
