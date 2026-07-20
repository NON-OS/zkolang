/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bit decomposition, the primitive under ordered comparison. A value is materialized
//! so its written row exposes it to the driver, its bits are read from the advice
//! suffix of the witness and each constrained boolean, and their weighted sum is
//! constrained to equal the value. This proves the value lies in the bit range and is
//! sound whatever the prover supplies, because the constraints, not the prover, decide.
//! The top bit is returned, which is the sign of a difference; the rest are freed.

use alloc::vec::Vec;

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::compile::compiled::Advice;
use crate::lang::CompileError;
use nonos_stark::field::Fp;

impl Compiler {
    pub(crate) fn decompose(&mut self, value: u8, nbits: u8) -> Result<u8, CompileError> {
        let zero = self.emit_num(0)?;
        let m = self.alloc()?;
        self.ops.push(Op::Add {
            d: m,
            a: value,
            b: zero.reg,
        });
        let value_op = (self.ops.len() - 1) as u32;
        self.free.push(zero.reg);

        let start = self.next_advice;
        let base = self.n_public + self.n_secret;
        let mut bits: Vec<u8> = Vec::with_capacity(nbits as usize);
        for _ in 0..nbits {
            let bit = self.alloc()?;
            let idx = base + self.next_advice;
            self.next_advice += 1;
            self.ops.push(Op::Inp { d: bit, idx });
            self.ops.push(Op::Bool { a: bit });
            bits.push(bit);
        }
        self.advice.push(Advice {
            value_op,
            start,
            width: nbits,
        });

        let mut acc = self.alloc()?;
        self.ops.push(Op::Imm {
            d: acc,
            v: Fp::ZERO,
        });
        for (k, &bit) in bits.iter().enumerate() {
            let pk = self.alloc()?;
            self.ops.push(Op::Imm {
                d: pk,
                v: Fp::from_u64(1u64 << (k as u32)),
            });
            let term = self.alloc()?;
            self.ops.push(Op::Mul {
                d: term,
                a: bit,
                b: pk,
            });
            let nacc = self.alloc()?;
            self.ops.push(Op::Add {
                d: nacc,
                a: acc,
                b: term,
            });
            self.free.push(acc);
            self.free.push(pk);
            self.free.push(term);
            acc = nacc;
        }

        let diff = self.alloc()?;
        self.ops.push(Op::Sub {
            d: diff,
            a: acc,
            b: m,
        });
        self.ops.push(Op::Assert { a: diff });
        self.free.push(acc);
        self.free.push(m);
        self.free.push(diff);
        for &bit in &bits[..bits.len() - 1] {
            self.free.push(bit);
        }
        Ok(bits[bits.len() - 1])
    }
}
