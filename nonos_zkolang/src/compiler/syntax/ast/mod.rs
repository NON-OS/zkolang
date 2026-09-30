/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The syntax tree, as written. Every node has a span, and the nodes later stages attach
 * facts to have a node id.
 */

mod attr;
mod binop;
mod block;
mod common;
mod data;
mod decl;
mod expr;
mod expr_kind;
mod func;
mod generic;
mod item;
mod lit;
mod op;
mod parts;
mod pat;
mod path;
mod ty;

pub use attr::{Attr, AttrArg};
pub use block::{Block, Stmt, StmtKind};
pub use common::{Ident, NodeId, Visibility};
pub use data::{EnumDecl, FieldDecl, Fields, StructDecl, Variant};
pub use decl::{ConstDecl, ImplDecl, ModDecl, TypeAliasDecl, UseTree};
pub use expr::Expr;
pub use expr_kind::ExprKind;
pub use func::{FnDecl, GenericParam, Param};
pub use generic::{ConstArg, GenericArg};
pub use item::{Item, ItemKind, SourceAst};
pub use lit::Lit;
pub use op::{AssignOp, BinOp, UnOp};
pub use parts::{Arm, FieldInit, ForIter};
pub use pat::{FieldPat, PatKind, Pattern};
pub use path::{Path, PathRoot, PathSegment};
pub use ty::{Label, Type, TypeKind};
