/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The reference interpreter: the semantics of sections 7 and 8 run over the typed IR. It
 * is the oracle the compiled program is tested against, and the constant evaluator.
 * Every failure of section 14.1 is reported with the span of the operation that failed.
 */

mod arith;
mod arith_field;
mod arith_pow;
mod bits;
mod budget;
mod call_frame;
mod cast;
mod env;
mod eval;
mod eval_block;
mod eval_builtin;
mod eval_call;
mod eval_chain;
mod eval_loop;
mod eval_parts;
mod eval_place;
mod eval_unary;
mod eval_while;
mod failure;
mod machine;
mod place_access;
mod run;
mod test_run;
mod value;

pub use env::Env;
pub use failure::{FailKind, Failure};
pub use machine::Interp;
pub use test_run::{run_tests, TestRun};
pub use value::Value;
