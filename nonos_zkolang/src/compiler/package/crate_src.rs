/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A crate of a program, as loading gives it to the checker. */

use alloc::string::String;
use alloc::vec::Vec;

use super::limits::Limits;
use crate::compiler::syntax::ast::SourceAst;

/** A crate of a program: its name, its syntax, and what it names its dependencies. */
#[derive(Clone, Debug)]
pub struct CrateSrc {
    pub name: String,
    pub ast: SourceAst,
    /** Each dependency, by the name this crate uses and its index among the crates. */
    pub deps: Vec<(String, usize)>,
    /** The cost thresholds its manifest sets; the program's own crate's govern. */
    pub limits: Limits,
}
