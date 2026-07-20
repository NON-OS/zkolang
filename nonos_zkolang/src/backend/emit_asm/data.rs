/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The constant strings and the zeroed storage a program needs: the print format and
//! newline in the data section, and the register file, input vector, and output vector
//! as common symbols. The data section and common symbols assemble on both ELF and
//! Mach-O without a format-specific section name. The input and output vectors are
//! emitted only when the program has any, so a program with none carries no unused
//! storage.

use alloc::format;
use alloc::string::String;

pub(super) fn data_section(n_in: usize, n_out: usize, regs: usize) -> String {
    let mut s = String::from("    .data\nfmt:\n    .string \"%llu \"\nnl:\n    .string \"\\n\"\n");
    s.push_str(&format!("    .comm rf, {}, 8\n", regs * 8));
    if n_in > 0 {
        s.push_str(&format!("    .comm in, {}, 8\n", n_in * 8));
    }
    if n_out > 0 {
        s.push_str(&format!("    .comm out, {}, 8\n", n_out * 8));
    }
    s
}
