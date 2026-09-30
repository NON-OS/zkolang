/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Loading a package and its path dependencies (section 4.1): each package's manifest
 * checked, its crate loaded from its entry, and each dependency loaded once however many
 * packages depend on it. A dependency on a package still being loaded is a cycle, and a
 * dependency of another edition than 2026 is refused (E0902).
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::crate_src::CrateSrc;
use super::manifest::manifest;
use super::paths::normalize;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{SourceMap, Span};
use crate::compiler::syntax::load::{dir_of, Files};

/** What loading a package graph reads and writes. */
pub(super) struct Graph<'g> {
    pub files: &'g dyn Files,
    pub map: &'g mut SourceMap,
    pub diags: &'g mut Diagnostics,
    pub crates: Vec<CrateSrc>,
    /** Each package loaded, by its directory, and its crate's index. */
    pub loaded: Vec<(String, usize)>,
    /** The directories of the packages being loaded, outermost first. */
    pub open: Vec<String>,
}

impl Graph<'_> {
    /**
     * Load the package whose manifest is `file`, named as a dependency at `named` if it is
     * one; the index of its crate, if it loads.
     */
    pub(super) fn package(
        &mut self,
        file: (&str, String),
        root: Option<(&str, String)>,
        named: Option<Span>,
    ) -> Option<usize> {
        let dir = normalize(dir_of(file.0));
        let id = self.map.add(String::from(file.0), file.1.clone());
        let m = manifest(id, &file.1, self.diags)?;
        if m.edition != 2026 {
            return self.not_2026(&m.name, named.unwrap_or(Span::new(id, 0, 0)));
        }
        let ast = self.entry(&dir, (id, &m), root)?;
        let index = self.crates.len();
        self.crates.push(CrateSrc {
            name: m.name.clone(),
            ast,
            deps: Vec::new(),
        });
        self.loaded.push((dir.clone(), index));
        self.open.push(dir.clone());
        for d in &m.deps {
            if let Some(i) = self.dependency(&dir, d) {
                if let Some(c) = self.crates.get_mut(index) {
                    c.deps.push((d.name.clone(), i));
                }
            }
        }
        self.open.pop();
        Some(index)
    }
}
