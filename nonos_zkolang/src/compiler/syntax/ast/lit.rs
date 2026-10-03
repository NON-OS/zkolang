/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Literals as written. */

use alloc::string::String;

use crate::compiler::source::Span;

/** A literal. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Lit {
    Int {
        value: u64,
        suffix: Option<crate::compiler::syntax::IntTy>,
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    Str {
        value: String,
        span: Span,
    },
}

impl Lit {
    /** Where the literal is. */
    pub fn span(&self) -> Span {
        match self {
            Lit::Int { span, .. } | Lit::Bool { span, .. } | Lit::Str { span, .. } => *span,
        }
    }
}
