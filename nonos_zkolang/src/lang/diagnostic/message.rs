/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The one-line message for each error. */

use alloc::format;
use alloc::string::String;

use crate::lang::CompileError;

/** A one-line human description of an error. */
pub fn message(err: &CompileError) -> String {
    match err {
        CompileError::UnexpectedChar { .. } => "unexpected character".into(),
        CompileError::NumberTooLarge { .. } => "number too large for the field".into(),
        CompileError::UnexpectedEof { .. } => "unexpected end of input".into(),
        CompileError::UnexpectedToken { .. } => "unexpected token".into(),
        CompileError::UnknownVariable { name } => format!("unknown variable `{name}`"),
        CompileError::TooManyRegisters => "too many live values for the register file".into(),
        CompileError::LoopTooLarge => "loop range unrolls too far".into(),
        CompileError::ProgramTooLong => "program unrolls past the largest provable size".into(),
        CompileError::UnknownFunction { name } => {
            format!("call to an undefined function `{name}`")
        }
        CompileError::ArityMismatch {
            name,
            expected,
            got,
        } => {
            format!("function `{name}` takes {expected} arguments but got {got}")
        }
        CompileError::RecursionTooDeep => "function inlining nested too deep".into(),
        CompileError::NotIndexable { .. } => "indexing a value that is not a table or array".into(),
        CompileError::UnknownConst { name } => format!("unknown constant `{name}`"),
        CompileError::NonConstantIndex => "index is not a compile-time constant".into(),
        CompileError::IndexOutOfBounds { .. } => "index out of bounds".into(),
        CompileError::ArrayNotScalar => "array used where a single value is required".into(),
        CompileError::TupleNotScalar => "tuple used where a single value is required".into(),
        CompileError::TupleArity { names, values } => {
            format!("this binding names {names} values but the right side has {values}")
        }
        CompileError::IncludeNotFound => "included file not found".into(),
        CompileError::IncludeTooDeep => "include nested too deep".into(),
        CompileError::IoLimit => {
            "more inputs, outputs or comparison bits than the machine can index".into()
        }
        CompileError::NestingTooDeep { .. } => "nested too deep".into(),
        CompileError::ExpressionTooLarge { .. } => "expression grows too large".into(),
    }
}
