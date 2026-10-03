/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Why lowering a checked program stopped. */

use crate::compiler::source::Span;

/** A reason lowering stopped. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowerError {
    /** The program has no `main` in its root module. */
    NoMain,
    /** A form this build does not compile yet, at its place in the source. */
    Unsupported(&'static str, Span),
    /** The program unrolls to more instructions than the builder holds. */
    TooLarge,
}

/** A lowering result. */
pub type L<T> = Result<T, LowerError>;
