/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Includes: composing a program from library files. An `include "path";` line is
//! replaced by the contents of that file, resolved through a caller-supplied lookup
//! so the core stays free of any filesystem. A file is included once however many
//! times it is named, and a depth bound turns a cycle into an error. Splicing a
//! file's text in is all a standard library needs, since items are top-level.

mod directive;
mod expand;

use alloc::string::String;
use alloc::vec::Vec;

use super::CompileError;

/// The deepest an include chain may nest, so a cycle is reported rather than hangs.
pub(super) const MAX_INCLUDE_DEPTH: usize = 64;

/// Expand every include in a source into a single include-free source ready for the
/// lexer, resolving each path through `resolve`. A path the resolver cannot find is
/// an error, and a file already included is skipped so it appears once.
pub fn expand_includes<F>(src: &str, resolve: &mut F) -> Result<String, CompileError>
where
    F: FnMut(&str) -> Option<String>,
{
    let mut out = String::new();
    let mut seen: Vec<String> = Vec::new();
    expand::expand(src, resolve, &mut seen, &mut out, 0)?;
    Ok(out)
}
