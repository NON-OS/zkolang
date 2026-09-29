/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Recursively expand includes into the output. */

use alloc::collections::BTreeSet;
use alloc::string::String;

use super::directive::include_path;
use super::included::Included;
use crate::lang::lex::BOM;
use crate::lang::CompileError;

/** The deepest an include chain may nest, so a cycle is reported rather than hangs. */
const MAX_INCLUDE_DEPTH: usize = 64;

/** Splice the file `key` names, whose text is `src`, into `out`. */
pub(super) fn expand<F>(
    key: &str,
    src: &str,
    resolve: &mut F,
    seen: &mut BTreeSet<String>,
    out: &mut String,
    depth: usize,
) -> Result<(), CompileError>
where
    F: FnMut(&str, &str) -> Option<Included>,
{
    if depth > MAX_INCLUDE_DEPTH {
        return Err(CompileError::IncludeTooDeep);
    }
    /*
     * Each file may start with a byte-order mark, and a lone carriage return ends a line
     * as `\n` and `\r\n` do, so an include on such a line is still recognized.
     */
    let src = src.strip_prefix(BOM).unwrap_or(src);
    for line in src.lines().flat_map(|l| l.split('\r')) {
        match include_path(line) {
            Some(path) => {
                let file = resolve(key, path).ok_or(CompileError::IncludeNotFound)?;
                if seen.insert(file.key.clone()) {
                    expand(&file.key, &file.text, resolve, seen, out, depth + 1)?;
                }
            }
            None => {
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    Ok(())
}
