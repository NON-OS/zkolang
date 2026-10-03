/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One path dependency of a package: its directory, relative to the manifest that names it,
 * holds its own manifest. A package being loaded that is named again closes a cycle.
 */

use alloc::format;
use alloc::string::String;

use super::graph::Graph;
use super::manifest::Dep;
use super::paths::normalize;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::load::join;

/** How many packages deep dependencies may nest. */
const MAX_DEPTH: usize = 64;

impl Graph<'_> {
    /** The crate of the dependency `d` of the package in `dir`, loaded once. */
    pub(super) fn dependency(&mut self, dir: &str, d: &Dep) -> Option<usize> {
        let at = normalize(&join(dir, &d.path));
        if self.open.contains(&at) {
            let what = format!("`{}` closes a dependency cycle", d.name);
            return self.refuse(what, d.span);
        }
        if let Some(&(_, i)) = self.loaded.iter().find(|(p, _)| *p == at) {
            return Some(i);
        }
        if self.open.len() >= MAX_DEPTH {
            let what = format!("dependencies nest more than {MAX_DEPTH} packages deep");
            return self.refuse(what, d.span);
        }
        let path = join(&at, "zkolang.toml");
        let Some(text) = self.files.read(&path) else {
            return self.refuse(format!("no manifest `{path}` for `{}`", d.name), d.span);
        };
        self.package((&path, text), None, Some(d.span))
    }

    /** Report `what` at `at` (E0902); nothing is loaded. */
    pub(super) fn refuse<T>(&mut self, what: String, at: Span) -> Option<T> {
        self.diags
            .push(Diagnostic::error(Code::MANIFEST, what, at, "here"));
        None
    }

    /** Report the package `name`, which is not of edition 2026, at `at`. */
    pub(super) fn not_2026<T>(&mut self, name: &str, at: Span) -> Option<T> {
        self.refuse(format!("`{name}` is not an edition 2026 package"), at)
    }
}
