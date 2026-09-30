/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Codes `E03xx` to `E05xx`: types, patterns and constants. */

use super::codes::codes;

codes! {
    /* Types. */
    MISMATCHED_TYPES = "E0300", "An expression's type differs from the type this position requires. zKølang has no implicit conversions.";
    NO_OPERATOR = "E0301", "An operator applied to types it is not defined for, such as `<` on `field` or `+` on `bool`.";
    LITERAL_OUT_OF_RANGE = "E0302", "An integer literal that does not fit its type. A `field` literal must be below the modulus p = 2^64 - 2^32 + 1.";
    CANNOT_INFER = "E0303", "The type of this expression could not be determined; add an annotation.";
    INVALID_CAST = "E0304", "`as` converts only when every value of the source type is a value of the target. Use `T::checked_from(e)` to fail on out-of-range values or `T::wrapping_from(e)` to truncate.";
    NOT_A_PLACE = "E0305", "Only a place rooted in a `let mut` variable or a `&mut` parameter can be assigned.";
    IMMUTABLE = "E0306", "Assignment to a variable not declared `let mut`.";
    WRONG_ARITY = "E0307", "A call with the wrong number of arguments.";
    NO_FIELD = "E0308", "A field or method that the type does not have.";
    INDEX_OUT_OF_BOUNDS = "E0309", "A constant index outside the array.";
    ALIASED_MUT = "E0310", "Two `&mut` arguments of one call that may refer to the same place.";
    OUTSIDE_LOOP = "E0311", "`break` and `continue` stand only inside a loop.";
    SHIFT_TOO_FAR = "E0312", "A shift by a constant count at least the width of the shifted type.";
    RECURSIVE_TYPE = "E0313", "A struct or enum that contains itself, directly or through other types. Its values would take no finite number of slots.";
    DUPLICATE_FIELD = "E0314", "A struct or variant declares a field twice, or a literal or pattern names a field twice.";
    MISSING_FIELD = "E0315", "A struct literal or pattern leaves out a field. A literal gives every field exactly once; a pattern that ignores fields ends with `..`.";
    /* Patterns. */
    NON_EXHAUSTIVE = "E0400", "A `match` that does not cover every value of its scrutinee. The message names a missing pattern.";
    REFUTABLE_PATTERN = "E0401", "A `let` or a parameter takes only patterns that always match.";
    PATTERN_MISMATCH = "E0402", "A pattern whose shape does not fit the scrutinee's type.";
    /* Constants. */
    NOT_CONSTANT = "E0500", "This position needs a compile-time constant: a literal, a `const`, a constant generic parameter, or an expression over them.";
    CONST_EVAL_FAILED = "E0501", "Evaluating a constant failed: an overflow, a failed assertion, a division by zero or an out-of-bounds index.";
    CONST_CYCLE = "E0502", "Constants that depend on each other in a cycle.";
    CONST_BUDGET = "E0503", "Evaluating a constant took more than ten million steps.";
    NOT_CONST_FN = "E0504", "A `const fn` may call only `const fn`s and may not use secret or public types or `declassify`.";
    CONST_DEPTH = "E0505", "Evaluating a constant needs constants and functions that need others in turn, more than 64 deep.";
}
