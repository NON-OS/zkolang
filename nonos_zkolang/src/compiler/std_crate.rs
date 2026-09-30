/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The standard library (section 18.2): written in zKølang, built into the compiler, and
 * loaded beside every crate as the crate `std`.
 */

use alloc::string::{String, ToString};

use super::syntax::load::{load, Files};
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::ast::SourceAst;

/** The root file of the standard library. */
const ROOT: &str = "std/lib.zkl";

/** The files of the standard library. */
#[derive(Clone, Copy, Debug, Default)]
pub struct Std;

impl Files for Std {
    fn read(&self, path: &str) -> Option<String> {
        let text = match path {
            "std/lib.zkl" => include_str!("../../../std/lib.zkl"),
            "std/hash.zkl" => include_str!("../../../std/hash.zkl"),
            "std/merkle.zkl" => include_str!("../../../std/merkle.zkl"),
            "std/option.zkl" => include_str!("../../../std/option.zkl"),
            "std/prelude.zkl" => include_str!("../../../std/prelude.zkl"),
            "std/result.zkl" => include_str!("../../../std/result.zkl"),
            _ => return None,
        };
        Some(text.to_string())
    }
}

/** The standard library, loaded into `map`; problems in it are in `diags`. */
pub fn load_std(map: &mut SourceMap, diags: &mut Diagnostics) -> SourceAst {
    let text = Std.read(ROOT).unwrap_or_default();
    load(&Std, (ROOT, text), map, diags)
}
