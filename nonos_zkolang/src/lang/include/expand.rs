/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Recursively expand includes into the output.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::directive::include_path;
use super::MAX_INCLUDE_DEPTH;
use crate::lang::CompileError;

pub(super) fn expand<F>(
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
