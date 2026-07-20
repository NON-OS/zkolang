/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The opcode selector, one tag per instruction. The AIR turns this into the one-hot
//! selector columns that gate each opcode's transition constraints.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpTag {
    Imm,
    Add,
    Sub,
    Mul,
    Inv,
    Sel,
    Eq,
    Bool,
    Assert,
    Inp,
    Out,
    Halt,
}
