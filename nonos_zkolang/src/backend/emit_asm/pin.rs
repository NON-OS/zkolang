/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Read one input: the string at rdi as a decimal number, returned in rax. An empty
 * string, a character that is not a digit, or a value at or above the prime exits with
 * status 1 rather than being reduced, so the native program reads the same numbers the
 * prover binds. The exit path realigns the stack before calling into the C library.
 */

pub(super) const PIN: &str = "\
pin:
    xorl %eax, %eax
    cmpb $0, (%rdi)
    je 9f
1:
    movzbl (%rdi), %ecx
    testl %ecx, %ecx
    jz 2f
    subl $48, %ecx
    cmpl $9, %ecx
    ja 9f
    movl $10, %edx
    mulq %rdx
    jo 9f
    addq %rcx, %rax
    jc 9f
    incq %rdi
    jmp 1b
2:
    movabsq $0xFFFFFFFF00000001, %rcx
    cmpq %rcx, %rax
    jae 9f
    ret
9:
    andq $-16, %rsp
    movl $1, %edi
    call SYM(exit)
";
