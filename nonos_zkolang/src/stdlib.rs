/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The standard library embedded as source, so a host with no filesystem, the kernel terminal
//! and editor, can resolve `include` directives against it. Each module is compiled into the
//! crate with `include_str!`, and `expand_with_stdlib` runs the include step against them, so a
//! program that includes a standard gadget compiles with no file access at all.

use alloc::string::String;
use alloc::vec::Vec;

use crate::driver::{prove_source_with_witness, Report, RunError};
use crate::isa::Op;
use crate::lang::{compile_source, expand_includes, CompileError};

/// The source of a standard-library module by its include name, or `None` if there is no such
/// module. This is the whole standard library, baked into the binary.
pub fn stdlib_source(name: &str) -> Option<&'static str> {
    Some(match name {
        "bits.zkl" => include_str!("../../stdlib/bits.zkl"),
        "cmp.zkl" => include_str!("../../stdlib/cmp.zkl"),
        "curve.zkl" => include_str!("../../stdlib/curve.zkl"),
        "encode.zkl" => include_str!("../../stdlib/encode.zkl"),
        "field.zkl" => include_str!("../../stdlib/field.zkl"),
        "gate.zkl" => include_str!("../../stdlib/gate.zkl"),
        "hash.zkl" => include_str!("../../stdlib/hash.zkl"),
        "logic.zkl" => include_str!("../../stdlib/logic.zkl"),
        "math.zkl" => include_str!("../../stdlib/math.zkl"),
        "merkle.zkl" => include_str!("../../stdlib/merkle.zkl"),
        "order.zkl" => include_str!("../../stdlib/order.zkl"),
        "poly.zkl" => include_str!("../../stdlib/poly.zkl"),
        "select.zkl" => include_str!("../../stdlib/select.zkl"),
        "vm.zkl" => include_str!("../../stdlib/vm.zkl"),
        _ => return None,
    })
}

/// Expand a program's includes against the embedded standard library, so it compiles with no
/// filesystem. An include the standard library does not name is reported, not dropped; a host
/// with its own files layers its own resolver over `stdlib_source`.
pub fn expand_with_stdlib(src: &str) -> Result<String, CompileError> {
    expand_includes(src, &mut |name| stdlib_source(name).map(String::from))
}

/// Compile a program against the embedded standard library, the one call an editor makes to
/// check a file: it expands the includes and compiles, returning the op list or the diagnostic
/// error to render under the source.
pub fn check(src: &str) -> Result<Vec<Op>, CompileError> {
    compile_source(&expand_with_stdlib(src)?)
}

/// Compile and prove a program against the embedded standard library, the one call a terminal
/// makes to run a file. The public inputs are the prefix the statement binds; the rest is the
/// private witness. The report says whether it verified and what it output.
pub fn run(src: &str, public: &[u64], secret: &[u64]) -> Result<Report, RunError> {
    let expanded = expand_with_stdlib(src).map_err(RunError::Compile)?;
    prove_source_with_witness(&expanded, public, secret)
}
