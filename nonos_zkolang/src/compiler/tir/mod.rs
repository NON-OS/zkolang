/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The typed IR: a checked program with every name resolved and every expression typed.
 * The reference interpreter runs it, and the compiler lowers it to SSA. Operators of one
 * precedence stay a flat chain, as in the syntax tree, so no stage recurses per link.
 */

mod builtin;
mod expr;
mod ids;
mod item;
mod labels;
mod lit;
mod place;
mod stmt;
mod typed;
mod visit;
mod visit_block;

pub use builtin::Builtin;
pub use expr::TExprKind;
pub use ids::{ConstId, FnId, LocalId};
pub use item::{TConst, TFn, TLocal, TParam, TProgram};
pub use labels::Labels;
pub use lit::TLit;
pub use place::{Proj, TArg, TPlace};
pub use stmt::{TBlock, TPat, TStmt};
pub use typed::TExpr;
