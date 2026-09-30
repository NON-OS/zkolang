/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The failures of a run (section 14.1), and the limits of the interpreter itself. */

use alloc::string::String;

use crate::compiler::source::Span;

/** Why a run failed. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FailKind {
    /** `assert` of `false`, with its message. */
    Assert(Option<String>),
    /** A checked arithmetic result outside its type. */
    Overflow,
    DivideByZero,
    /** The inverse of a zero `field`. */
    InverseOfZero,
    IndexOutOfBounds,
    /** A shift by at least the width. */
    ShiftTooFar,
    /** `checked_from` of a value outside the target type. */
    CheckedFrom,
    /** `from_le_bits` of bits that are not a canonical `field` element. */
    NotCanonical,
    /** A `while` still running after its limit. */
    LimitExceeded,
    /** More steps than the budget allows (section 11), or evaluation nested too deep. */
    Budget,
    /** Code the checker should have rejected; reaching it is a compiler bug. */
    Internal,
}

impl FailKind {
    /** What a message says happened. */
    pub fn describe(&self) -> &str {
        match self {
            FailKind::Assert(Some(m)) => m,
            FailKind::Assert(None) => "an assertion failed",
            FailKind::Overflow => "arithmetic overflow",
            FailKind::DivideByZero => "division by zero",
            FailKind::InverseOfZero => "the inverse of zero",
            FailKind::IndexOutOfBounds => "index out of bounds",
            FailKind::ShiftTooFar => "a shift by at least the width",
            FailKind::CheckedFrom => "a value outside the target type",
            FailKind::NotCanonical => "bits that are not a field element",
            FailKind::LimitExceeded => "a `while` loop ran past its limit",
            FailKind::Budget => "evaluation took more steps than allowed",
            FailKind::Internal => "the interpreter met code the checker should have rejected",
        }
    }
}

/** A failed run: why, and at which operation. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Failure {
    pub kind: FailKind,
    pub span: Span,
}
