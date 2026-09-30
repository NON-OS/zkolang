/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The item a type path names, its generic arguments written only on its last name. */

use alloc::format;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Path;

impl<'a> Sema<'a> {
    /** The item the type path `p` names, generic arguments only on its last name. */
    pub(super) fn type_def(&mut self, m: DefId, p: &'a Path) -> Option<DefId> {
        if let Some((_, prefix)) = p.segments.split_last() {
            if let Some(s) = prefix.iter().find(|s| s.generics.is_some()) {
                let what = format!("`{}` takes no generic arguments", s.ident.name);
                let d = Diagnostic::error(Code::WRONG_GENERICS, what, p.span, "given here");
                self.diags.push(d);
            }
        }
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        match self.defs.resolve(m, p.root, &names) {
            Ok(d) => {
                self.note_use(d, p);
                Some(d)
            }
            Err(e) => {
                self.report_path(p, e);
                None
            }
        }
    }
}
