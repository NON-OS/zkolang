/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Help for a path that does not resolve: the names close to the one at fault (section 4.4). */

use alloc::string::String;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::did_you_mean;
use crate::compiler::syntax::ast::{Path, PathRoot};
use crate::compiler::syntax::IntTy;

/** The primitive types by name, which a misspelt type may have meant. */
pub(crate) fn primitive_types() -> Vec<&'static str> {
    let ints = IntTy::ALL.iter().map(|t| t.name());
    ints.chain(["bool", "field"]).collect()
}

impl<'a> Sema<'a> {
    /**
     * Help naming what segment `i` of `p`, written in module `m`, may have meant; `more`
     * are names the place adds to the items and generic parameters in scope.
     */
    pub(crate) fn path_near(&self, m: DefId, p: &Path, i: usize, more: &[&str]) -> Option<String> {
        let segs: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let name = segs.get(i)?;
        let mut names = self.defs.names_near(m, p.root, &segs, i);
        if i == 0 && p.root == PathRoot::Plain {
            names.extend(self.generics.iter().map(|g| g.0.as_str()));
            names.extend(more.iter().copied());
        }
        did_you_mean(name, names)
    }
}
