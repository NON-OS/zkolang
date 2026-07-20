/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Turning source into a program can fail; the front-end never panics but returns
//! one of these, with a byte offset where the lexer can point at the character.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CompileError {
    /// A character that starts no token, at this byte offset.
    UnexpectedChar { at: usize },
    /// A numeric literal too large for the field's 64-bit representative.
    NumberTooLarge { at: usize },
    /// The token stream ended mid-statement or mid-expression.
    UnexpectedEof,
    /// A token that does not fit the grammar at this point.
    UnexpectedToken,
    /// A reference to a name that was never bound.
    UnknownVariable,
    /// The program needs more live values than the register file holds.
    TooManyRegisters,
    /// A loop whose range would unroll to too many iterations.
    LoopTooLarge,
    /// A call to a function that was never defined.
    UnknownFunction,
    /// A call whose argument count does not match the parameters.
    ArityMismatch,
    /// Function inlining nested too deep, which a recursive call would cause.
    RecursionTooDeep,
    /// An index into a name that is neither a constant table nor an array.
    NotIndexable,
    /// A reference to a constant table that was never declared.
    UnknownConst,
    /// A table or array index that is not a compile-time constant.
    NonConstantIndex,
    /// A table or array index outside its elements.
    IndexOutOfBounds,
    /// An array used where a single value is required.
    ArrayNotScalar,
    /// An included file the resolver could not find.
    IncludeNotFound,
    /// An include chain nested past the depth bound, which a cycle would cause.
    IncludeTooDeep,
}
