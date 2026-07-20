/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Emit a whole program as a standalone x86_64 assembly file.

use alloc::string::String;

use super::data::data_section;
use super::field::FIELD;
use super::header::HEADER;
use super::inv::INV;
use super::io::{parse_inputs, print_outputs};
use super::op::emit_op;
use crate::backend::{n_inputs, n_outputs};
use crate::isa::{Op, REGS};

/// Emit a program as System V x86_64 assembly. Assembled and linked against the C
/// runtime with `cc file.s`, it runs as native code and produces the proven outputs.
pub fn to_asm(program: &[Op]) -> String {
    let n_in = n_inputs(program);
    let n_out = n_outputs(program);

    let mut s = String::from(HEADER);
    s.push_str(FIELD);
    s.push_str(INV);
    s.push_str("    .globl SYM(main)\nSYM(main):\n    pushq %rbx\n    pushq %r12\n    pushq %r13\n    movq %rdi, %r12\n    movq %rsi, %r13\n");
    s.push_str(&parse_inputs(n_in));
    for (i, op) in program.iter().enumerate() {
        s.push_str(&emit_op(op, i));
    }
    s.push_str(&print_outputs(n_out));
    s.push_str("    xorl %eax, %eax\n    popq %r13\n    popq %r12\n    popq %rbx\n    ret\n");
    s.push_str(&data_section(n_in, n_out, REGS));
    s
}
