/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types as written. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{ConstArg, NodeId, Path};
use crate::compiler::source::Span;
use crate::compiler::syntax::IntTy;

/** A secrecy label written on a type. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Label {
    Public,
    Secret,
}

/** A type as written. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Type {
    pub id: NodeId,
    pub kind: TypeKind,
    pub span: Span,
}

/** The shapes a written type takes. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeKind {
    Field,
    Bool,
    Int(IntTy),
    /** `()` */
    Unit,
    /** `(T1, T2, ...)`, at least one element and a comma. */
    Tuple(Vec<Type>),
    /** `[T; N]` */
    Array(Box<Type>, ConstArg),
    /** A named type: a struct, an enum, an alias or a generic parameter. */
    Path(Path),
    /** `Self` inside an impl block. */
    SelfType,
    /** `&mut T`, a parameter type only. */
    RefMut(Box<Type>),
    /** `public T` or `secret T`. */
    Labelled(Label, Box<Type>),
    /** A type that failed to parse; the parser has reported it. */
    Error,
}
