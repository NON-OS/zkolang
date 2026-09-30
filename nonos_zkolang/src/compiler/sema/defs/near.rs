/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The names a path may have meant at a segment that does not resolve (section 4.4): the
 * names of the module the path has reached there that are visible from where it is
 * written, and for the first name of a plain path, the crates and the prelude as well.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::{DefId, Defs};
use crate::compiler::syntax::ast::PathRoot;

impl<'a> Defs<'a> {
    /** The names that may stand at segment `i` of the path `segs` after `root`, written in `from`. */
    pub fn names_near(&self, from: DefId, root: PathRoot, segs: &[&str], i: usize) -> Vec<&str> {
        let plain = i == 0 && root == PathRoot::Plain;
        let module = match plain {
            true => Some(from),
            false => segs.get(..i).and_then(|s| self.resolve(from, root, s).ok()),
        };
        let mut out: Vec<&str> = Vec::new();
        if let Some((m, names)) = module.and_then(|m| Some((m, self.modules.get(&m)?))) {
            for (n, b) in &names.names {
                if plain || self.visible(b, m, from) {
                    out.push(n.as_str());
                }
            }
        }
        if plain {
            if let Some(d) = self.deps.get(&self.crate_of(from)) {
                out.extend(d.keys().map(String::as_str));
            }
            if self.std.is_some() {
                out.push("std");
            }
            if let Some(p) = self.prelude() {
                out.extend(p.names.keys().map(String::as_str));
            }
        }
        out
    }
}
