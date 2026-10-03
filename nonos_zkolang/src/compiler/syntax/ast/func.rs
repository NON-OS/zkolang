/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Functions: their generic parameters, their parameters and the declaration itself. */

use alloc::vec::Vec;

use super::{Block, Ident, Pattern, Type};
use crate::compiler::source::Span;

/** A generic parameter. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GenericParam {
    /** A type parameter, `T`. */
    Type(Ident),
    /** A constant parameter, `const N: usize`. */
    Const { name: Ident, ty: Type },
}

impl GenericParam {
    /** The parameter's name. */
    pub fn name(&self) -> &Ident {
        match self {
            GenericParam::Type(i) | GenericParam::Const { name: i, .. } => i,
        }
    }
}

/** A function parameter. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Param {
    /** `self` or `&mut self`. */
    SelfParam { by_ref_mut: bool, span: Span },
    /** `pattern: Type`. */
    Typed { pat: Pattern, ty: Type },
}

/** `const? fn name<generics>(params) -> ret { body }`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FnDecl {
    pub is_const: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Block,
}
