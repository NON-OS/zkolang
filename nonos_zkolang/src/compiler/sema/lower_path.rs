/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A type written as a path: an alias, replaced by what it stands for, its labels placed
 * where the path stands. A struct or enum is reported where it is declared (E0904).
 */

use alloc::format;
use alloc::vec::Vec;

use super::cx::Sema;
use super::defs::{DefId, DefKind};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::Path;
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type the path `p` names: an alias's. */
    pub(super) fn lower_path(
        &mut self,
        m: DefId,
        p: &Path,
        path: &[u32],
        labels: &mut Labels,
    ) -> TyId {
        self.no_generics(p);
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let def = match self.defs.resolve(m, p.root, &names) {
            Ok(d) => d,
            Err(e) => {
                self.report_path(p, e);
                return Types::ERROR;
            }
        };
        match self.defs.get(def).map(|d| d.kind) {
            Some(DefKind::Alias) => {
                let (ty, own) = self.alias(def);
                for (q, l) in own.0 {
                    labels.0.push(([path, &q].concat(), l));
                }
                ty
            }
            Some(DefKind::Struct | DefKind::Enum) => Types::ERROR,
            Some(k) => {
                let d = Diagnostic::error(
                    Code::WRONG_KIND,
                    format!("`{}` is a {}, not a type", p.last_name(), k.describe()),
                    p.span,
                    "not a type",
                );
                self.diags.push(d);
                Types::ERROR
            }
            None => Types::ERROR,
        }
    }
}
