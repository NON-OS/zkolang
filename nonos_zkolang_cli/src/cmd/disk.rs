/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The filesystem, from which an edition 2026 program's files are read (section 4.2), and
 * the manifest that governs a file.
 */

use std::fs;
use std::path::Path;

use nonos_zkolang::compiler::syntax::load::Files;

/** The files of the filesystem, each path relative to the working directory or absolute. */
pub(super) struct Disk;

impl Files for Disk {
    fn read(&self, path: &str) -> Option<String> {
        fs::read_to_string(path).ok()
    }
}

/**
 * The manifest that governs the file `file` (section 4.1): `zkolang.toml` in the file's
 * directory or the nearest one above it, as the path names them; its path and text.
 */
pub(crate) fn manifest_for(file: &str) -> Option<(String, String)> {
    let dir = Path::new(file).parent()?;
    dir.ancestors().find_map(|d| {
        let path = d.join("zkolang.toml");
        let text = fs::read_to_string(&path).ok()?;
        Some((path.to_string_lossy().into_owned(), text))
    })
}
