/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Compiling on a small stack, the size a test thread or an embedded host gives. */

use nonos_zkolang::{compile_source, CompileError, Op};

/**
 * Compile on a thread with a two megabyte stack, so input nested deeper than the compiler
 * can walk shows up here as a stack overflow rather than only on a small host.
 */
pub(crate) fn compiles_on_a_small_stack(src: String) -> Result<Vec<Op>, CompileError> {
    std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(move || compile_source(&src))
        .expect("spawn")
        .join()
        .expect("the compiler must return, not overflow its stack")
}
