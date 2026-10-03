/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Attributes and their arguments. */

use alloc::vec::Vec;

use super::{Ident, Lit};
use crate::compiler::source::Span;

/** An attribute argument. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AttrArg {
    /** `name`, or `name = literal`. */
    Named { name: Ident, value: Option<Lit> },
    /** A bare literal. */
    Lit(Lit),
}

/** An attribute, `#[name]`, `#[name(args)]` or `#[name = literal]`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Attr {
    pub name: Ident,
    pub args: Option<Vec<AttrArg>>,
    pub value: Option<Lit>,
    /** Whether it was written `#![...]`, applying to the enclosing module. */
    pub inner: bool,
    pub span: Span,
}
