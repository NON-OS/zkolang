/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The standard library embedded as source, so a host with no filesystem, the kernel terminal
//! and editor, can resolve `include` directives against it. Each module is compiled into the
//! crate with `include_str!`, and `expand_with_stdlib` runs the include step against them, so a
//! program that includes a standard gadget compiles with no file access at all.

use alloc::string::String;

use crate::lang::{expand_includes, CompileError};

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
