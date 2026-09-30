/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Uses of an item marked `#[deprecated = "..."]` (W0005), each warned where it stands. */

use alloc::format;

use super::attr_query::deprecated;
use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Path;

impl<'a> Sema<'a> {
    /** Warn of the use of `def` through the path `p` if `def` is deprecated. */
    pub(crate) fn note_use(&mut self, def: DefId, p: &Path) {
        let item = self.defs.get(def).and_then(|d| d.item);
        let Some(message) = item.and_then(|i| deprecated(&i.attrs)) else {
            return;
        };
        let what = format!("`{}` is deprecated: {message}", p.last_name());
        let d = Diagnostic::warning(Code::DEPRECATED, what, p.span, "deprecated");
        self.diags.push(d);
    }
}
