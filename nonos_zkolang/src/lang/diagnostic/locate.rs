/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Find where an undefined name is used, for errors that carry only the name. */

use crate::lang::lex::{lex, Tok};
use crate::lang::CompileError;

/**
 * The offending name for an error whose name is undefined, so every occurrence of it in
 * the source is a use and the first is a correct place to point. An arity mismatch is left
 * out, because its name is defined and the first occurrence is the definition, not the call.
 */
pub(super) fn unknown_name(err: &CompileError) -> Option<&str> {
    match err {
        CompileError::UnknownVariable { name }
        | CompileError::UnknownFunction { name }
        | CompileError::UnknownConst { name } => Some(name.as_str()),
        _ => None,
    }
}

/**
 * The byte offset of the first identifier token equal to `name`, found by lexing, so the
 * location is a real token and never a match inside a comment or a string.
 */
pub(super) fn locate_name(src: &str, name: &str) -> Option<usize> {
    let (toks, spans) = lex(src).ok()?;
    toks.iter().zip(spans).find_map(|(t, at)| match t {
        Tok::Ident(n) if n.as_str() == name => Some(at),
        _ => None,
    })
}
