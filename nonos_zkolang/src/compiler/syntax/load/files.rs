/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Where a compilation reads its files (section 4.2): the command line reads the
 * filesystem, a kernel its own store, and a test a table. Paths are `/`-separated and
 * relative to where the reader starts.
 */

use alloc::string::{String, ToString};

/** A source of files, each named by its path. */
pub trait Files {
    /** The text of the file at `path`, if there is one. */
    fn read(&self, path: &str) -> Option<String>;
}

/** No files: a program of one file, whose `mod name;` items find nothing. */
#[derive(Clone, Copy, Debug, Default)]
pub struct NoFiles;

impl Files for NoFiles {
    fn read(&self, _path: &str) -> Option<String> {
        None
    }
}

/** The directory of the file at `path`: what comes before its last `/`, or nothing. */
pub(super) fn dir_of(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/** The path of `name` in the directory `dir`. */
pub(super) fn join(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        alloc::format!("{dir}/{name}")
    }
}
