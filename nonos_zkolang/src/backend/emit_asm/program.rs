/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Emit a whole program as a standalone x86_64 assembly file.

use alloc::string::String;

use super::data::data_section;
use super::field::FIELD;
use super::header::{HEADER, STACK_NOTE};
use super::inv::INV;
use super::io::{parse_inputs, print_outputs};
use super::op::emit_op;
use super::pin::PIN;
use crate::backend::{n_outputs, Plan};
use crate::isa::REGS;
use crate::lang::Compiled;

/**
 * Emit a program as System V x86_64 assembly. Saved as a `.S` file, so the C
 * preprocessor runs over its header, and assembled and linked with `cc file.S`, it runs
 * as native code and produces the proven outputs. It takes one argument per public input
 * and secret, and computes the bits of each ordered comparison itself.
 */
pub fn to_asm(compiled: &Compiled) -> String {
    let program = &compiled.ops;
    let plan = Plan::of(compiled);
    let n_in = plan.n_user;
    let n_out = n_outputs(program);

    let mut s = String::from(HEADER);
    s.push_str(FIELD);
    s.push_str(INV);
    s.push_str(PIN);
    s.push_str("    .globl SYM(main)\nSYM(main):\n    pushq %rbx\n    pushq %r12\n    pushq %r13\n    movq %rdi, %r12\n    movq %rsi, %r13\n");
    s.push_str(&parse_inputs(n_in));
    for (i, op) in program.iter().enumerate() {
        s.push_str(&emit_op(op, i, &plan));
    }
    s.push_str(&print_outputs(n_out));
    s.push_str("    xorl %eax, %eax\n    popq %r13\n    popq %r12\n    popq %rbx\n    ret\n");
    s.push_str(&data_section(n_in, n_out, REGS));
    s.push_str(STACK_NOTE);
    s
}
