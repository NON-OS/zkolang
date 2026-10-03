/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Checked functions and constants, and the program that holds them. */

use alloc::string::String;
use alloc::vec::Vec;

use super::{Labels, LocalId, TBlock, TExpr, TPat};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;

/** A local variable or parameter. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TLocal {
    pub name: String,
    pub ty: TyId,
    pub mutable: bool,
    /** The labels its declaration writes. */
    pub labels: Labels,
    pub span: Span,
}

/** A parameter: the local that holds the argument, and the pattern that takes it apart. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TParam {
    pub local: LocalId,
    pub pat: TPat,
    /** Whether it is a `&mut` parameter, whose final value goes back to the caller. */
    pub by_ref: bool,
}

/** A checked function. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TFn {
    pub def: DefId,
    pub name: String,
    pub is_const: bool,
    pub params: Vec<TParam>,
    pub ret: TyId,
    pub ret_labels: Labels,
    pub body: TBlock,
    pub locals: Vec<TLocal>,
    pub span: Span,
}

/** A checked constant item and its initializer. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TConst {
    pub def: DefId,
    pub name: String,
    pub ty: TyId,
    pub init: TExpr,
    pub locals: Vec<TLocal>,
    pub span: Span,
}

/** A checked program. */
#[derive(Clone, Debug, Default)]
pub struct TProgram {
    pub types: Types,
    pub fns: Vec<TFn>,
    pub consts: Vec<TConst>,
    /** The value each constant evaluated to, in the order of `consts`. */
    pub values: Vec<crate::compiler::interp::Value>,
    /** The root module's `fn main`, if it declares one. */
    pub main: Option<super::FnId>,
    /** Each `#[test]` function, and whether it is marked `#[should_fail]`. */
    pub tests: Vec<(super::FnId, bool)>,
    /** Whether the cost warnings are silent for each function (section 15.3). */
    pub cost_quiet: Vec<bool>,
}
