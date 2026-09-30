/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Codes `E06xx` to `E99xx`: secret flow, generics and recursion, limits and
 * registers, programs and packages, and internal errors.
 */

use super::codes::codes;

codes! {
    /* Secret flow. */
    SECRET_LEAK = "E0600", "A value derived from a secret reaches a public position: the result of `main`, or a binding, parameter, field or result typed `public`. Use `declassify(e)` if revealing it is intended.";
    MAIN_UNLABELLED = "E0601", "Every parameter of `main` must be typed `public T` or `secret T`.";
    /* Generics and recursion. */
    RECURSION_UNBOUNDED = "E0700", "A recursion the compiler could not bound. Every call is expanded at compile time, so a recursion must be controlled by a constant condition, usually on a constant generic parameter.";
    WRONG_GENERICS = "E0701", "The wrong number or kind of generic arguments.";
    INSTANTIATION_FAILED = "E0702", "A generic item did not type-check for these arguments. The note points at the template line.";
    /* Limits and registers. */
    TOO_MANY_ROWS = "E0800", "The program compiles to more than 2^16 rows, the largest trace the prover sizes.";
    REGISTER_PRESSURE = "E0801", "More values are needed at once than the machine's 32 registers hold, and none of them can be written again or re-read: a secret input or advice value is read once and held until its last use. Restructure the computation so fewer values are needed at the same time.";
    TOO_MANY_IO = "E0802", "More inputs, outputs or advice values than the machine's sixteen-bit indices name.";
    /* Programs and packages. */
    NO_MAIN = "E0900", "Running a program needs `fn main` in the root module.";
    BAD_MAIN = "E0901", "`main` may not be generic and must return a value the public output can hold.";
    MANIFEST = "E0902", "The package manifest `zkolang.toml` is malformed or names something that does not exist.";
    BAD_ATTRIBUTE = "E0903", "An unknown attribute, or a known one used where it does not apply.";
    UNSUPPORTED = "E0904", "A form this build of the compiler does not check or compile yet. Structs, enums, `match`, generics, methods, and modules in their own files are checked from a later build on; a form that is checked but not compiled yet is named in the message.";
    RUN_FAILED = "E0905", "The program fails on these inputs where its run reaches the marked operation: an `assert` of `false`, an overflow, a division by zero, or another failure of section 14.1. A failing run has no proof.";
    BAD_INPUTS = "E0906", "The values given do not fit `main`: the wrong number of values for its public or secret parameters, or a value outside its type. Each scalar of a parameter, in order, takes one value.";
    /* Internal. */
    INTERNAL = "E9999", "The compiler failed an internal check and refused to emit a program. This is a compiler bug; please report it with the input.";
}
