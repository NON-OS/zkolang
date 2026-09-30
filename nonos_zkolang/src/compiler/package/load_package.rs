/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Loading the package a manifest describes, with its dependencies. */

use alloc::string::String;
use alloc::vec::Vec;

use super::crate_src::CrateSrc;
use super::graph::Graph;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::load::Files;

/**
 * The crates of the package whose manifest is `manifest`, a path and its text, and of its
 * dependencies, the package's first; its crate's root file is `root` if given, else the
 * manifest's entry. Problems are in `diags`.
 */
pub fn load_package(
    files: &dyn Files,
    manifest_file: (&str, String),
    root: Option<(&str, String)>,
    map: &mut SourceMap,
    diags: &mut Diagnostics,
) -> Vec<CrateSrc> {
    let mut g = Graph {
        files,
        map,
        diags,
        crates: Vec::new(),
        loaded: Vec::new(),
        open: Vec::new(),
    };
    g.package(manifest_file, root, None);
    g.crates
}
