/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One opcode as x86_64 assembly over the register file in .bss. Arithmetic delegates
 * to the field prelude; a selection is a conditional move. Wherever the proof would be
 * unprovable the program exits with the status the C target returns: 2 for a failed
 * boolean constraint, 3 for a failed assertion, 4 for an inverse of zero, and 5 for a
 * select whose condition is not a bit. The index makes the guard labels unique.
 */

use alloc::format;
use alloc::string::String;

use crate::isa::Op;

fn call2(d: u8, a: u8, b: u8, f: &str) -> String {
    let (d, a, b) = (d as usize * 8, a as usize * 8, b as usize * 8);
    format!("    movq rf+{a}(%rip), %rdi\n    movq rf+{b}(%rip), %rsi\n    call {f}\n    movq %rax, rf+{d}(%rip)\n")
}

pub(super) fn emit_op(op: &Op, i: usize) -> String {
    let off = |r: &u8| *r as usize * 8;
    match op {
        Op::Imm { d, v } => format!("    movabsq ${}, %rax\n    movq %rax, rf+{}(%rip)\n", v.value(), off(d)),
        Op::Add { d, a, b } => call2(*d, *a, *b, "fadd"),
        Op::Sub { d, a, b } => call2(*d, *a, *b, "fsub"),
        Op::Mul { d, a, b } => call2(*d, *a, *b, "fmul"),
        Op::Inv { d, a } => format!("    movq rf+{}(%rip), %rdi\n    testq %rdi, %rdi\n    jnz .Lok{i}\n    movl $4, %edi\n    call SYM(exit)\n.Lok{i}:\n    call finv\n    movq %rax, rf+{}(%rip)\n", off(a), off(d)),
        Op::Sel { d, c, a, b } => format!("    movq rf+{}(%rip), %rdx\n    cmpq $1, %rdx\n    jbe .Lok{i}\n    movl $5, %edi\n    call SYM(exit)\n.Lok{i}:\n    movq rf+{}(%rip), %rax\n    movq rf+{}(%rip), %rcx\n    testq %rdx, %rdx\n    cmovnz %rcx, %rax\n    movq %rax, rf+{}(%rip)\n", off(c), off(b), off(a), off(d)),
        Op::Eq { d, a, b } => format!("    movq rf+{}(%rip), %rax\n    xorl %ecx, %ecx\n    cmpq rf+{}(%rip), %rax\n    sete %cl\n    movq %rcx, rf+{}(%rip)\n", off(a), off(b), off(d)),
        Op::Bool { a } => format!("    movq rf+{}(%rip), %rax\n    cmpq $0, %rax\n    je .Lok{i}\n    cmpq $1, %rax\n    je .Lok{i}\n    movl $2, %edi\n    call SYM(exit)\n.Lok{i}:\n", off(a)),
        Op::Assert { a } => format!("    movq rf+{}(%rip), %rax\n    testq %rax, %rax\n    jz .Lok{i}\n    movl $3, %edi\n    call SYM(exit)\n.Lok{i}:\n", off(a)),
        Op::Inp { d, idx } => format!("    movq in+{}(%rip), %rax\n    movq %rax, rf+{}(%rip)\n", *idx as usize * 8, off(d)),
        Op::Out { a, idx } => format!("    movq rf+{}(%rip), %rax\n    movq %rax, out+{}(%rip)\n", off(a), *idx as usize * 8),
        Op::Halt => String::from("    /* halt */\n"),
    }
}
