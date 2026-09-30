/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The tests of a program (section 16): each function marked `#[test]`, and whether it is
 * marked `#[should_fail]`. A test's attributes are checked with the others.
 */

use alloc::vec::Vec;

use super::attr_query::has;
use super::cx::Sema;
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /** The program's tests, in the order of their functions. */
    pub(super) fn tests(&self) -> Vec<(FnId, bool)> {
        let mut out = Vec::new();
        for (i, info) in self.fns.iter().enumerate() {
            let Some(item) = self.defs.get(info.def).and_then(|d| d.item) else {
                continue;
            };
            if has(&item.attrs, "test") {
                let f = FnId(u32::try_from(i).unwrap_or(u32::MAX));
                out.push((f, has(&item.attrs, "should_fail")));
            }
        }
        out
    }
}
