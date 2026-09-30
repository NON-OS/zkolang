/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The crate of a package: its root file, given, or the manifest's entry, or `src/main.zkl`, else `src/lib.zkl`. */

use alloc::format;
use alloc::string::String;

use super::graph::Graph;
use super::manifest::Manifest;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::ast::SourceAst;
use crate::compiler::syntax::load::{join, load};

impl Graph<'_> {
    /** The syntax of the crate of the manifest `m`, the file `id`, of the package in `dir`. */
    pub(super) fn entry(
        &mut self,
        dir: &str,
        (id, m): (FileId, &Manifest),
        root: Option<(&str, String)>,
    ) -> Option<SourceAst> {
        let (path, text) = match root {
            Some((p, t)) => (String::from(p), t),
            None => {
                let candidates = match &m.entry {
                    Some(e) => alloc::vec![join(dir, e)],
                    None => alloc::vec![join(dir, "src/main.zkl"), join(dir, "src/lib.zkl")],
                };
                let read = candidates
                    .iter()
                    .find_map(|p| Some((p.clone(), self.files.read(p)?)));
                let Some(found) = read else {
                    let what = format!(
                        "no entry file for `{}`: {}",
                        m.name,
                        candidates.join(" or ")
                    );
                    return self.refuse(what, Span::new(id, 0, 0));
                };
                found
            }
        };
        Some(load(self.files, (&path, text), self.map, self.diags))
    }
}
