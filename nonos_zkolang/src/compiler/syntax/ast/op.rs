/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Operators: binary, unary and assignment. */

/** A binary operator. */
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/** A unary operator. */
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnOp {
    /** `-x` */
    Neg,
    /** `!x`, logical not on `bool` and bitwise complement on integers. */
    Not,
}

/** An assignment operator: `=` or a compound form. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssignOp {
    Assign,
    Compound(BinOp),
}
