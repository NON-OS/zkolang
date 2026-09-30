/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A named item of the program. */

use alloc::string::String;

use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Item, Visibility};

/** An item, as an index into the program's table of items. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DefId(pub u32);

/** The kinds of named item. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DefKind {
    Mod,
    Fn,
    Const,
    Alias,
    Struct,
    Enum,
}

impl DefKind {
    /** How a message names an item of this kind. */
    pub fn describe(self) -> &'static str {
        match self {
            DefKind::Mod => "module",
            DefKind::Fn => "function",
            DefKind::Const => "constant",
            DefKind::Alias => "type alias",
            DefKind::Struct => "struct",
            DefKind::Enum => "enum",
        }
    }
}

/** A named item: what it is, its name, and the module that declares it. */
#[derive(Clone, Debug)]
pub struct Def<'a> {
    pub kind: DefKind,
    pub name: String,
    /** Where its name is written; the whole file for the root module. */
    pub span: Span,
    /** The declaring module; `None` for the root module, which is the crate. */
    pub parent: Option<DefId>,
    pub vis: Visibility,
    /** The declaration; `None` for the root module. */
    pub item: Option<&'a Item>,
}
