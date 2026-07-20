/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A single VM instruction. `d` is the destination register; `a`, `b`, `c` are
//! source registers. Every opcode is enforced by the step AIR.

use nonos_stark::field::Fp;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    /// r_d = literal.
    Imm { d: u8, v: Fp },
    /// r_d = r_a + r_b.
    Add { d: u8, a: u8, b: u8 },
    /// r_d = r_a - r_b.
    Sub { d: u8, a: u8, b: u8 },
    /// r_d = r_a * r_b.
    Mul { d: u8, a: u8, b: u8 },
    /// r_d = r_a inverse; an inverse of zero makes the trace unprovable.
    Inv { d: u8, a: u8 },
    /// r_d = r_c ? r_a : r_b, with r_c constrained boolean. Branchless.
    Sel { d: u8, c: u8, a: u8, b: u8 },
    /// r_d = (r_a == r_b) as {0,1}.
    Eq { d: u8, a: u8, b: u8 },
    /// Constrain r_a to be boolean.
    Bool { a: u8 },
    /// Constrain r_a to be zero.
    Assert { a: u8 },
    /// r_d = input[idx] (public or secret).
    Inp { d: u8, idx: u16 },
    /// public_output[idx] = r_a.
    Out { a: u8, idx: u16 },
    /// End of program.
    Halt,
}
