/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Includes: composing a program from library files. An `include "path";` line is
//! replaced by the contents of that file, resolved through a caller-supplied lookup
//! so the core stays free of any filesystem. A file is included once however many
//! times it is named, which lets several modules share a dependency without
//! redefining it, and a depth bound turns an include cycle into an error rather than
//! a hang. Because functions and constant tables are top-level items, splicing a
//! file's text in is all a standard library needs.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::CompileError;

// The deepest an include chain may nest. A cycle would recurse without end, so this
// bound reports it rather than hanging. Ordinary library depth is far below it.
const MAX_INCLUDE_DEPTH: usize = 64;

/// Expand every `include` in a source, resolving each path through `resolve`, into a
/// single include-free source ready for the lexer. A path that `resolve` cannot find
/// is an error, and a file already included is skipped so it appears once.
pub fn expand_includes<F>(src: &str, resolve: &mut F) -> Result<String, CompileError>
where
    F: FnMut(&str) -> Option<String>,
{
    let mut out = String::new();
    let mut seen: Vec<String> = Vec::new();
    expand(src, resolve, &mut seen, &mut out, 0)?;
    Ok(out)
}

fn expand<F>(
    src: &str,
    resolve: &mut F,
    seen: &mut Vec<String>,
    out: &mut String,
    depth: usize,
) -> Result<(), CompileError>
where
    F: FnMut(&str) -> Option<String>,
{
    if depth > MAX_INCLUDE_DEPTH {
        return Err(CompileError::IncludeTooDeep);
    }
    for line in src.lines() {
        match include_path(line) {
            Some(path) => {
                if seen.iter().any(|s| s == path) {
                    continue;
                }
                seen.push(path.to_string());
                let content = resolve(path).ok_or(CompileError::IncludeNotFound)?;
                expand(&content, resolve, seen, out, depth + 1)?;
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    Ok(())
}

// The path of an include directive on its own line, `include "path";`, or `None` for
// an ordinary line. A trailing comment is allowed. Keeping the directive to one line
// makes it a plain text step that runs before the lexer sees the source.
fn include_path(line: &str) -> Option<&str> {
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
