/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Includes: composing a program from library files. An `include "path";` line is
 * replaced by the contents of that file, resolved through a caller-supplied lookup
 * so the core stays free of any filesystem. A file is included once however many
 * times it is named, and a depth bound turns a cycle into an error. Splicing a
 * file's text in is all a standard library needs, since items are top-level.
 */

mod directive;
mod entry;
mod expand;
mod included;

pub use entry::{expand_includes, expand_includes_from};
pub use included::Included;
