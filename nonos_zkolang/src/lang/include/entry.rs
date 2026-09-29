/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The include entry points. */

use alloc::collections::BTreeSet;
use alloc::string::String;

use super::expand::expand;
use super::included::Included;
use crate::lang::CompileError;

/**
 * Expand every include in a source into a single include-free source ready for the
 * lexer, resolving each path through `resolve`. A path the resolver cannot find is
 * an error, and a path already included is skipped so it appears once. The path as
 * written is the file's identity; `expand_includes_from` lets a resolver say which
 * spellings name one file.
 */
pub fn expand_includes<F>(src: &str, resolve: &mut F) -> Result<String, CompileError>
where
    F: FnMut(&str) -> Option<String>,
{
    expand_includes_from("", src, &mut |_, path| {
        let text = resolve(path)?;
        let key = String::from(path);
        Some(Included { key, text })
    })
}

/**
 * Expand the includes of the file `root` names, whose text is `src`. The resolver is
 * given the key of the including file and the path it wrote, and returns the file with
 * its own key, so a path can resolve relative to the file that wrote it. A file is
 * spliced in once per key, and the root counts as included, so a library that names
 * the program back is not spliced into it again.
 */
pub fn expand_includes_from<F>(
    root: &str,
    src: &str,
    resolve: &mut F,
) -> Result<String, CompileError>
where
    F: FnMut(&str, &str) -> Option<Included>,
{
    let mut out = String::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    seen.insert(String::from(root));
    expand(root, src, resolve, &mut seen, &mut out, 0)?;
    Ok(out)
}
