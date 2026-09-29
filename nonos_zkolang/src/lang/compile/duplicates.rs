/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A name defined twice: two functions, two constants, or two parameters of one function. */

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;

use super::same_expr::same;
use crate::lang::parse::Ast;
use crate::lang::{CompileError, NameError};

/** Refuse the first name a program defines twice. */
pub(super) fn check_duplicates(ast: &Ast) -> Result<(), CompileError> {
    let dup = |n: &String| CompileError::Name(NameError::Duplicate { name: n.clone() });
    let fn_defs = ast.fns.iter().map(|f| (&f.name, (&f.params, &f.body)));
    if let Some(n) = conflicting(fn_defs, |a, b| a.0 == b.0 && same(a.1, b.1)) {
        return Err(dup(n));
    }
    let const_defs = ast.consts.iter().map(|c| (&c.name, (&c.values, c.scalar)));
    if let Some(n) = conflicting(const_defs, |a, b| a == b) {
        return Err(dup(n));
    }
    for f in &ast.fns {
        let mut params = BTreeSet::new();
        if let Some(p) = f.params.iter().find(|p| !params.insert(p.as_str())) {
            return Err(dup(p));
        }
    }
    Ok(())
}

/**
 * The first name defined twice with definitions `same` tells apart. Including one library
 * through two paths repeats its items word for word, and an identical repeat is harmless.
 */
fn conflicting<'a, D>(
    defs: impl Iterator<Item = (&'a String, D)>,
    same: impl Fn(&D, &D) -> bool,
) -> Option<&'a String> {
    let mut seen: BTreeMap<&String, D> = BTreeMap::new();
    for (n, d) in defs {
        match seen.get(n) {
            Some(e) if !same(e, &d) => return Some(n),
            Some(_) => {}
            None => {
                seen.insert(n, d);
            }
        }
    }
    None
}
