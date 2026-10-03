/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The runtime glue in x86_64 assembly. On entry argc is in r12 and argv in r13. The
 * program takes exactly one argument per input, each read by `pin` as a decimal number
 * below the prime, and exits with status 1 for any other command line, as the C target
 * returns 1. Each output is printed with printf as an unsigned value, followed by a
 * trailing newline.
 */

use alloc::format;
use alloc::string::String;

pub(super) fn parse_inputs(n_in: usize) -> String {
    let mut s = format!(
        "    cmpq ${}, %r12\n    je .Largc\n    movl $1, %edi\n    call SYM(exit)\n.Largc:\n",
        n_in + 1
    );
    for i in 0..n_in {
        let (arg, slot) = ((i + 1) * 8, i * 8);
        s.push_str(&format!(
            "    movq {arg}(%r13), %rdi\n    call pin\n    movq %rax, in+{slot}(%rip)\n"
        ));
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
