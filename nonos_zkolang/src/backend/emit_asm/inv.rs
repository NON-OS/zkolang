/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Field inverse in x86_64 assembly by Fermat exponentiation. The base is raised to
//! the prime less two through square-and-multiply, reusing fmul for every product, so
//! the inverse costs no separate reduction logic. The loop keeps its running result,
//! base, and exponent in callee-saved registers across the calls.

pub(super) const INV: &str = "\
finv:
    pushq %rbx
    pushq %r12
    pushq %r13
    movq $1, %rbx
    movq %rdi, %r12
    movabsq $0xFFFFFFFF00000001, %r13
    subq $2, %r13
1:
    testq %r13, %r13
    jz 2f
    testq $1, %r13
    jz 3f
    movq %rbx, %rdi
    movq %r12, %rsi
    call fmul
    movq %rax, %rbx
3:
    movq %r12, %rdi
    movq %r12, %rsi
    call fmul
    movq %rax, %r12
    shrq $1, %r13
    jmp 1b
2:
    movq %rbx, %rax
    popq %r13
    popq %r12
    popq %rbx
    ret
";
