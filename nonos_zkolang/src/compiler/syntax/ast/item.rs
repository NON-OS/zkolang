/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Items: the declarations a module holds, and a parsed file. */

use alloc::string::String;
use alloc::vec::Vec;

use super::{Attr, ConstDecl, EnumDecl, FnDecl, Ident, ImplDecl, ModDecl, NodeId, StructDecl};
use super::{TypeAliasDecl, UseTree, Visibility};
use crate::compiler::source::{FileId, Span};

/** An item with the attributes, visibility and documentation written on it. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Item {
    pub id: NodeId,
    pub attrs: Vec<Attr>,
    pub vis: Visibility,
    /** The text of the item's outer doc comments, markers stripped, lines joined by `\n`. */
    pub doc: Option<String>,
    pub kind: ItemKind,
    pub span: Span,
}

/** The kinds of item. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemKind {
    Fn(FnDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    TypeAlias(TypeAliasDecl),
    Const(ConstDecl),
    /** `mod name;` (`body` is `None`) or `mod name { ... }`. */
    Mod(ModDecl),
    Use(UseTree),
    Impl(ImplDecl),
}

impl Item {
    /** The item's declared name, if it has one. */
    pub fn name(&self) -> Option<&Ident> {
        match &self.kind {
            ItemKind::Fn(f) => Some(&f.name),
            ItemKind::Struct(s) => Some(&s.name),
            ItemKind::Enum(e) => Some(&e.name),
            ItemKind::TypeAlias(t) => Some(&t.name),
            ItemKind::Const(c) => Some(&c.name),
            ItemKind::Mod(m) => Some(&m.name),
            ItemKind::Use(_) | ItemKind::Impl(_) => None,
        }
    }
}

/** A parsed source file: its inner attributes, module documentation and items. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceAst {
    pub file: FileId,
    pub inner_attrs: Vec<Attr>,
    pub inner_doc: Option<String>,
    pub items: Vec<Item>,
    pub span: Span,
}
