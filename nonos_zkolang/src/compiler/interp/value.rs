/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Values (section 6): what an expression evaluates to. */

use alloc::vec::Vec;

/** A value of the language. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
    Unit,
    Bool(bool),
    /** An integer's mathematical value, which its type bounds. */
    Int(i128),
    /** A `field` element, canonical in `[0, p)`. */
    Field(u64),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    /** An enum value: its variant's tag and its fields' values. */
    Variant(u32, Vec<Value>),
}

impl Value {
    /** The integer a value holds, or 0 for a value of another kind, which checking excludes. */
    pub fn int(&self) -> i128 {
        match self {
            Value::Int(v) => *v,
            Value::Field(v) => i128::from(*v),
            Value::Bool(b) => i128::from(*b),
            _ => 0,
        }
    }

    /** The bool a value holds, or `false` for a value of another kind. */
    pub fn bool(&self) -> bool {
        matches!(self, Value::Bool(true))
    }

    /** The elements of a tuple or array value; none for another kind. */
    pub fn parts(&self) -> &[Value] {
        match self {
            Value::Tuple(v) | Value::Array(v) => v,
            _ => &[],
        }
    }
}
