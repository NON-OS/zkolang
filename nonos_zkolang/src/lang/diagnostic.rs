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

use super::lex::{lex, Tok};
use super::CompileError;

/// The byte offset an error points at, when it carries one.
pub fn span_of(err: &CompileError) -> Option<usize> {
    match err {
        CompileError::UnexpectedChar { at }
        | CompileError::NumberTooLarge { at }
        | CompileError::UnexpectedEof { at }
        | CompileError::UnexpectedToken { at }
        | CompileError::NotIndexable { at }
        | CompileError::IndexOutOfBounds { at } => Some(*at),
        _ => None,
    }
}

/// A one-line human description of an error.
pub fn message(err: &CompileError) -> String {
    match err {
        CompileError::UnexpectedChar { .. } => "unexpected character".into(),
        CompileError::NumberTooLarge { .. } => "number too large for the field".into(),
        CompileError::UnexpectedEof { .. } => "unexpected end of input".into(),
        CompileError::UnexpectedToken { .. } => "unexpected token".into(),
        CompileError::UnknownVariable { name } => format!("unknown variable `{name}`"),
        CompileError::TooManyRegisters => "too many live values for the register file".into(),
        CompileError::LoopTooLarge => "loop range unrolls too far".into(),
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
    }
}

/// The offending name for an error whose name is undefined, so every occurrence of it in
/// the source is a use and the first is a correct place to point. An arity mismatch is left
/// out, because its name is defined and the first occurrence is the definition, not the call.
fn unknown_name(err: &CompileError) -> Option<&str> {
    match err {
        CompileError::UnknownVariable { name }
        | CompileError::UnknownFunction { name }
        | CompileError::UnknownConst { name } => Some(name.as_str()),
        _ => None,
    }
}

/// The byte offset of the first identifier token equal to `name`, found by lexing, so the
/// location is a real token and never a match inside a comment or a string.
fn locate_name(src: &str, name: &str) -> Option<usize> {
    let (toks, spans) = lex(src).ok()?;
    toks.iter().zip(spans).find_map(|(t, at)| match t {
        Tok::Ident(n) if n.as_str() == name => Some(at),
        _ => None,
    })
}

/// Render an error as a diagnostic over its source.
pub fn render(src: &str, err: &CompileError) -> String {
    let msg = message(err);
    let locate = || unknown_name(err).and_then(|n| locate_name(src, n));
    let Some(at) = span_of(err).or_else(locate) else {
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
