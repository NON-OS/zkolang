/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The pieces every part of the tree shares: node ids, names and visibility. */

use alloc::string::String;

use crate::compiler::source::Span;

/**
 * A node's identity, unique across one compilation, so later stages can attach facts to a
 * node in a side table.
 */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(pub u32);

/** A name as written. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

/** Whether an item or field is exported. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Visibility {
    Private,
    Public,
}
