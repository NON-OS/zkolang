/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The documents the editor has open, read in place of the files on disk. */

use std::collections::BTreeMap;
use std::fs;

use nonos_zkolang::compiler::syntax::load::Files;

/** The text of each open document, by its path. */
#[derive(Default)]
pub(crate) struct Docs(pub(crate) BTreeMap<String, String>);

impl Files for Docs {
    fn read(&self, path: &str) -> Option<String> {
        self.0
            .get(path)
            .cloned()
            .or_else(|| fs::read_to_string(path).ok())
    }
}
