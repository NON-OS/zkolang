/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a program is built from: its root file, and the manifest of its package when one
 * governs it (section 4.1), whose dependencies are loaded as crates beside it.
 */

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::diag::Diagnostics;
use crate::compiler::package::{load_package, CrateSrc};
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::load::{load, Files};

/** A program's root file, a path and its text, and its package's manifest, if it has one. */
#[derive(Clone, Debug)]
pub struct Source<'s> {
    pub root: (&'s str, String),
    pub manifest: Option<(&'s str, String)>,
}

impl<'s> Source<'s> {
    /** The program of the one file `path`, of text `text`, and no manifest. */
    pub fn file(path: &'s str, text: String) -> Source<'s> {
        Source {
            root: (path, text),
            manifest: None,
        }
    }
}

/** The crates of `src`, read from `files` into `map`, its own first; problems are in `diags`. */
pub(super) fn crates_of(
    files: &dyn Files,
    src: Source<'_>,
    map: &mut SourceMap,
    diags: &mut Diagnostics,
) -> Vec<CrateSrc> {
    match src.manifest {
        Some(m) => load_package(files, m, Some(src.root), map, diags),
        None => vec![CrateSrc {
            name: String::from("crate"),
            ast: load(files, src.root, map, diags),
            deps: Vec::new(),
        }],
    }
}
