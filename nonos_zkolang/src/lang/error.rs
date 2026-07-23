/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Turning source into a program can fail; the front-end never panics but returns
//! one of these, with a byte offset where a token error can point at the source and the
//! offending name where a name error can quote it.

use alloc::string::String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CompileError {
    /// A character that starts no token, at this byte offset.
    UnexpectedChar { at: usize },
    /// A numeric literal too large for the field's 64-bit representative.
    NumberTooLarge { at: usize },
    /// The token stream ended mid-statement or mid-expression, at this byte offset.
    UnexpectedEof { at: usize },
    /// A token that does not fit the grammar at this point, at this byte offset.
    UnexpectedToken { at: usize },
    /// A reference to a name that was never bound.
    UnknownVariable { name: String },
    /// The program needs more live values than the register file holds.
    TooManyRegisters,
    /// A loop whose range would unroll to too many iterations.
    LoopTooLarge,
    /// A call to a function that was never defined.
    UnknownFunction { name: String },
    /// A call whose argument count does not match the parameters.
    ArityMismatch {
        name: String,
        expected: usize,
        got: usize,
    },
    /// Function inlining nested too deep, which a recursive call would cause.
    RecursionTooDeep,
    /// An index into a name that is neither a constant table nor an array, at this offset.
    NotIndexable { at: usize },
    /// A reference to a constant table that was never declared.
    UnknownConst { name: String },
    /// A table or array index that is not a compile-time constant.
    NonConstantIndex,
    /// A table or array index outside its elements, at this offset.
    IndexOutOfBounds { at: usize },
    /// An array used where a single value is required.
    ArrayNotScalar,
    /// A tuple used where a single value is required.
    TupleNotScalar,
    /// A destructuring binding whose name count does not match the value's arity.
    TupleArity { names: usize, values: usize },
    /// An included file the resolver could not find.
    IncludeNotFound,
    /// An include chain nested past the depth bound, which a cycle would cause.
    IncludeTooDeep,
}
