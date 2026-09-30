/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Where module files come from. The compiler asks a provider for a path relative to a
 * package root; the command-line tool reads the filesystem, the kernel its own store, and
 * tests an in-memory table. The standard library is always embedded and never asked for.
 */

use alloc::string::String;
use alloc::vec::Vec;

/** A source of module files. Paths use `/` and are relative to the package root. */
pub trait SourceProvider {
    /** The text of the file at `path`, or `None` if there is none. */
    fn read(&mut self, path: &str) -> Option<String>;
}

/** A provider over an in-memory list of files, for tests and single-file hosts. */
#[derive(Clone, Debug, Default)]
pub struct MemoryProvider {
    files: Vec<(String, String)>,
}

impl MemoryProvider {
    /** An empty provider. */
    pub fn new() -> MemoryProvider {
        MemoryProvider { files: Vec::new() }
    }

    /** Add or replace the file at `path`. */
    pub fn insert(&mut self, path: &str, text: &str) {
        if let Some(e) = self.files.iter_mut().find(|(p, _)| p == path) {
            e.1 = String::from(text);
        } else {
            self.files.push((String::from(path), String::from(text)));
        }
    }
}

impl SourceProvider for MemoryProvider {
    fn read(&mut self, path: &str) -> Option<String> {
        self.files
            .iter()
            .find(|(p, _)| p == path)
            .map(|(_, t)| t.clone())
    }
}
