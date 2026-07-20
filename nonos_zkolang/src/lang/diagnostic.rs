/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Rendering a compile error as a diagnostic. An error that carries a byte offset is
//! shown with its line and column and a caret under the offending place in the source,
//! the way a compiler points at a mistake; an error without a location is shown as its
//! message alone.

use alloc::format;
use alloc::string::String;

use super::CompileError;

/// The byte offset an error points at, when it carries one.
pub fn span_of(err: &CompileError) -> Option<usize> {
    match err {
        CompileError::UnexpectedChar { at }
        | CompileError::NumberTooLarge { at }
        | CompileError::UnexpectedEof { at }
        | CompileError::UnexpectedToken { at } => Some(*at),
        _ => None,
    }
}

/// A one-line human description of an error.
pub fn message(err: &CompileError) -> &'static str {
    match err {
        CompileError::UnexpectedChar { .. } => "unexpected character",
        CompileError::NumberTooLarge { .. } => "number too large for the field",
        CompileError::UnexpectedEof { .. } => "unexpected end of input",
        CompileError::UnexpectedToken { .. } => "unexpected token",
        CompileError::UnknownVariable => "unknown variable",
        CompileError::TooManyRegisters => "too many live values for the register file",
        CompileError::LoopTooLarge => "loop range unrolls too far",
        CompileError::UnknownFunction => "call to an undefined function",
        CompileError::ArityMismatch => "wrong number of arguments",
        CompileError::RecursionTooDeep => "function inlining nested too deep",
        CompileError::NotIndexable => "indexing a value that is not a table or array",
        CompileError::UnknownConst => "unknown constant",
        CompileError::NonConstantIndex => "index is not a compile-time constant",
        CompileError::IndexOutOfBounds => "index out of bounds",
        CompileError::ArrayNotScalar => "array used where a single value is required",
        CompileError::IncludeNotFound => "included file not found",
        CompileError::IncludeTooDeep => "include nested too deep",
    }
}

/// Render an error as a diagnostic over its source.
pub fn render(src: &str, err: &CompileError) -> String {
    let msg = message(err);
    let Some(at) = span_of(err) else {
        return format!("error: {msg}");
    };
    let at = at.min(src.len());
    let line_start = src[..at].rfind('\n').map(|p| p + 1).unwrap_or(0);
    let line_end = src[at..].find('\n').map(|p| at + p).unwrap_or(src.len());
    let line = src[..at].matches('\n').count() + 1;
    let col = src[line_start..at].chars().count() + 1;
    let text = &src[line_start..line_end];
    let mut caret = String::new();
    for _ in 1..col {
        caret.push(' ');
    }
    caret.push('^');
    format!("error: {msg}\n  --> {line}:{col}\n   |\n   | {text}\n   | {caret}")
}
