/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The labels written on a type (section 5.4): each `public` or `secret`, with the path to
 * the part of the value it qualifies. A path step is a tuple field; an array element is
 * the step `ELEMENT`. An empty path qualifies the whole value.
 */

use alloc::vec::Vec;

use crate::compiler::syntax::ast::Label;

/** The labelled parts of a type, outermost first. */
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Labels(pub Vec<(Vec<u32>, Label)>);

impl Labels {
    /** The path step that enters an array's elements. */
    pub const ELEMENT: u32 = u32::MAX;

    /** Whether no label is written. */
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /** The label written on the whole value, if any. */
    pub fn whole(&self) -> Option<Label> {
        self.0.iter().find(|(p, _)| p.is_empty()).map(|(_, l)| *l)
    }
}
