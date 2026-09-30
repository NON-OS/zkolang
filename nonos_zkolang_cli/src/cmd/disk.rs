/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The filesystem, from which an edition 2026 crate's module files are read (section 4.2). */

use std::fs;

use nonos_zkolang::compiler::syntax::load::Files;

/** The files of the filesystem, each path relative to the working directory or absolute. */
pub(super) struct Disk;

impl Files for Disk {
    fn read(&self, path: &str) -> Option<String> {
        fs::read_to_string(path).ok()
    }
}
