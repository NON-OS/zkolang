/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The field prelude in x86_64 assembly: Goldilocks add, subtract, and multiply.
//! Each takes its operands in rdi and rsi, forms the 128-bit intermediate in rdx:rax,
//! and reduces once by dividing by the prime, so the result returned in rax is
//! canonical. Inputs are assumed already reduced, which keeps every quotient inside a
//! single machine word.

pub(super) const FIELD: &str = "\
.text
fadd:
    movq %rdi, %rax
    xorq %rdx, %rdx
    addq %rsi, %rax
    adcq $0, %rdx
    movabsq $0xFFFFFFFF00000001, %rcx
    divq %rcx
    movq %rdx, %rax
    ret
fsub:
    movabsq $0xFFFFFFFF00000001, %rcx
    subq %rsi, %rcx
    movq %rdi, %rax
    xorq %rdx, %rdx
    addq %rcx, %rax
    adcq $0, %rdx
    movabsq $0xFFFFFFFF00000001, %rcx
    divq %rcx
    movq %rdx, %rax
    ret
fmul:
    movq %rdi, %rax
    mulq %rsi
    movabsq $0xFFFFFFFF00000001, %rcx
    divq %rcx
    movq %rdx, %rax
    ret
";
