/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Indices into a checked program: its functions, its constants, a function's locals. */

/** A function, as an index into the program's functions. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FnId(pub u32);

/** A constant item, as an index into the program's constants. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ConstId(pub u32);

/** A local variable or parameter, as an index into its function's locals. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LocalId(pub u32);
