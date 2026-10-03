/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Structs and enums: their fields and variants. */

use alloc::string::String;
use alloc::vec::Vec;

use super::{Attr, GenericParam, Ident, NodeId, Type, Visibility};
use crate::compiler::source::Span;

/** A struct field. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldDecl {
    pub attrs: Vec<Attr>,
    pub vis: Visibility,
    pub doc: Option<String>,
    /** `None` for a tuple-struct or tuple-variant field. */
    pub name: Option<Ident>,
    pub ty: Type,
    pub span: Span,
}

/** The shape of a struct or variant's fields. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Fields {
    /** No fields: `struct S;` or `Variant`. */
    Unit,
    /** `(T, U)` */
    Tuple(Vec<FieldDecl>),
    /** `{ a: T, b: U }` */
    Named(Vec<FieldDecl>),
}

/** `struct Name<generics> fields`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StructDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Fields,
}

/** An enum variant. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Variant {
    pub id: NodeId,
    pub attrs: Vec<Attr>,
    pub doc: Option<String>,
    pub name: Ident,
    pub fields: Fields,
    pub span: Span,
}

/** `enum Name<generics> { variants }`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<Variant>,
}
