/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `abi`: the layout of an edition 2026 program's inputs and result (section 12.2), each
 * of `main`'s parameters with its type and the slots it takes, in the order they are given.
 */

use nonos_zkolang::compiler::driver::abi::{leaves, slots, Leaf};
use nonos_zkolang::compiler::syntax::ast::Label;
use nonos_zkolang::compiler::tir::TProgram;

use super::abi_words::{count, shown};
use super::modern::built;
use crate::line::Line;

const USAGE: &str = "usage: zkolang abi <file> [--edition 2026]";

/** Print the layout of the program `args` names. */
pub(crate) fn abi(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--edition"], USAGE)?;
    let (_, b) = built(&line)?;
    for (side, label) in [("public", false), ("secret", true)] {
        let params = side_params(&b.program, label);
        let n: usize = params.iter().map(|(_, _, l)| slots(l)).sum();
        println!("{side} inputs, {}", count(n));
        for (name, ty, l) in &params {
            println!("  {name}: {ty}  {}", shown(l));
        }
    }
    let n = slots(&b.output);
    println!("output, {}\n  {}", count(n), shown(&b.output));
    Ok(())
}

/** `main`'s parameters labelled secret if `secret`, else public: name, type and leaves. */
fn side_params(p: &TProgram, secret: bool) -> Vec<(String, String, Vec<Leaf>)> {
    let Some(f) = p.main.and_then(|m| p.fns.get(m.0 as usize)) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for param in &f.params {
        let Some(l) = f.locals.get(param.local.0 as usize) else {
            continue;
        };
        if (l.labels.whole() == Some(Label::Secret)) != secret {
            continue;
        }
        let mut ls = Vec::new();
        leaves(&p.types, l.ty, &mut ls);
        out.push((l.name.clone(), p.types.display(l.ty), ls));
    }
    out
}
