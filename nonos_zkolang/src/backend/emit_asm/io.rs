/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The runtime glue in x86_64 assembly. On entry argc is in r12 and argv in r13. Each
//! input is read from the matching argument with strtoull and reduced by the prime, or
//! left zero when the argument is absent, matching the C target. Each output is printed
//! with printf as an unsigned value, followed by a trailing newline.

use alloc::format;
use alloc::string::String;

pub(super) fn parse_inputs(n_in: usize) -> String {
    let mut s = String::new();
    for i in 0..n_in {
        s.push_str(&format!("    cmpq ${}, %r12\n    jle .Ldef{i}\n    movq {}(%r13), %rdi\n    xorl %esi, %esi\n    movl $10, %edx\n    call SYM(strtoull)\n    xorq %rdx, %rdx\n    movabsq $0xFFFFFFFF00000001, %rcx\n    divq %rcx\n    movq %rdx, in+{}(%rip)\n    jmp .Ldone{i}\n.Ldef{i}:\n    movq $0, in+{}(%rip)\n.Ldone{i}:\n", i + 1, (i + 1) * 8, i * 8, i * 8));
    }
    s
}

pub(super) fn print_outputs(n_out: usize) -> String {
    let mut s = String::new();
    for i in 0..n_out {
        s.push_str(&format!("    leaq fmt(%rip), %rdi\n    movq out+{}(%rip), %rsi\n    xorl %eax, %eax\n    call SYM(printf)\n", i * 8));
    }
    s.push_str("    leaq nl(%rip), %rdi\n    xorl %eax, %eax\n    call SYM(printf)\n");
    s
}
