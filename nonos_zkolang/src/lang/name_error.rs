/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Names the edition leaves ambiguous, refused rather than resolved one way or another. */

use alloc::string::String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NameError {
    /** Two functions, two constants, or two parameters of one function, of one name. */
    Duplicate { name: String },
    /** A parameter, binding, input or block local named like a constant or table. */
    ShadowsConstant { name: String },
    /** A binding inside a loop body named like the loop's variable. */
    ShadowsLoopVariable { name: String },
    /** A function that calls itself, directly or through others. */
    Recursive { name: String },
}
