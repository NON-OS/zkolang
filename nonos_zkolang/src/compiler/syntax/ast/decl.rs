/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The other declarations: type aliases, constants, modules, `use` trees and impl blocks. */

use alloc::string::String;
use alloc::vec::Vec;

use super::{Attr, Expr, GenericParam, Ident, Item, Path, Type};
use crate::compiler::source::Span;

/** `type Name<generics> = Type;`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeAliasDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub ty: Type,
}

/** `const NAME: Type = value;`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConstDecl {
    pub name: Ident,
    pub ty: Type,
    pub value: Expr,
}

/** `mod name;` or `mod name { items }`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModDecl {
    pub name: Ident,
    /** The inline body, or `None` for a module in its own file. */
    pub body: Option<Vec<Item>>,
    /** Inner attributes of an inline module, or of the file of a loaded one. */
    pub inner_attrs: Vec<Attr>,
    pub inner_doc: Option<String>,
    /** The whole of the module's own file, once loaded (`syntax::load`). */
    pub file: Option<Span>,
}

/** A `use` tree. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UseTree {
    /** `path` or `path as name`. */
    Single {
        path: Path,
        alias: Option<Ident>,
        span: Span,
    },
    /** `path::*` */
    Glob { prefix: Path, span: Span },
    /** `path::{a, b::c, ...}` */
    Nested {
        prefix: Path,
        trees: Vec<UseTree>,
        span: Span,
    },
}

/** `impl<generics> Type { fns }`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImplDecl {
    pub generics: Vec<GenericParam>,
    pub self_ty: Type,
    pub items: Vec<Item>,
}
