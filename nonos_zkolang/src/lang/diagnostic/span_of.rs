/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where an error points, when it carries a byte offset. */

use crate::lang::CompileError;

/** The byte offset an error points at, when it carries one. */
pub fn span_of(err: &CompileError) -> Option<usize> {
    match err {
        CompileError::UnexpectedChar { at }
        | CompileError::NumberTooLarge { at }
        | CompileError::UnexpectedEof { at }
        | CompileError::UnexpectedToken { at }
        | CompileError::NotIndexable { at }
        | CompileError::IndexOutOfBounds { at }
        | CompileError::NestingTooDeep { at }
        | CompileError::ExpressionTooLarge { at } => Some(*at),
        _ => None,
    }
}
