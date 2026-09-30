/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Source text: files, the spans that point into them, the map that owns them, and the
 * provider the compiler reads module files through, so the core never touches a
 * filesystem.
 */

mod file;
mod file_lines;
mod map;
mod provider;
mod span;

pub use file::SourceFile;
pub use map::SourceMap;
pub use provider::{MemoryProvider, SourceProvider};
pub use span::{FileId, Span};
